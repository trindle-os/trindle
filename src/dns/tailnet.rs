//! Tailscale's split DNS.

use std::io::{Read, Write};
use std::net::{IpAddr, SocketAddr};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::{Duration, Instant};

use serde_json::Value;

pub const SOCKETS: [&str; 2] = ["/run/tailscale/tailscaled.sock", "/var/run/tailscale/tailscaled.sock"];
const WAIT: Duration = Duration::from_secs(1);
const MOST: usize = 16 * 1024 * 1024;
const WATCH: &str = "/localapi/v0/watch-ipn-bus?mask=24";
const CONFIG: &str = "/localapi/v0/dns-config";

#[derive(Debug, Clone, PartialEq)]
pub struct Split {
    pub suffix: String,
    pub servers: Vec<SocketAddr>,
}

pub fn routes() -> Option<Vec<Split>> {
    SOCKETS.iter().map(Path::new).filter(|p| p.exists()).find_map(routes_at)
}

pub fn routes_at(socket: &Path) -> Option<Vec<Split>> {
    let (status, body) = get(socket, CONFIG, false)?;
    let dns = if status == 200 {
        serde_json::from_slice::<Value>(&body).ok()?
    } else if status == 404 {
        let (status, body) = get(socket, WATCH, true)?;
        if status != 200 {
            return None;
        }
        let first: Value = serde_json::from_slice(&body).ok()?;
        first.get("NetMap")?.get("DNS")?.clone()
    } else {
        return None;
    };
    Some(parse(&dns))
}

pub fn parse(dns: &Value) -> Vec<Split> {
    let mut out = Vec::new();
    let Some(routes) = dns.get("Routes").and_then(Value::as_object) else { return out };
    for (name, list) in routes {
        let Some(suffix) = crate::common::dns::site(name) else { continue };
        let servers: Vec<SocketAddr> = list.as_array().map(|l| l.iter()
            .filter_map(|r| r.get("Addr").and_then(Value::as_str))
            .filter_map(|a| a.parse::<SocketAddr>().ok().or_else(|| a.parse::<IpAddr>().ok().map(|ip| SocketAddr::new(ip, 53))))
            .collect()).unwrap_or_default();
        out.push(Split { suffix, servers });
    }
    out.sort_by(|a, b| b.suffix.len().cmp(&a.suffix.len()).then_with(|| a.suffix.cmp(&b.suffix)));
    out
}

pub fn find<'a>(name: &str, routes: &'a [Split]) -> Option<&'a Split> {
    routes.iter().find(|r| name.strip_suffix(r.suffix.as_str()).is_some_and(|rest| rest.is_empty() || rest.ends_with('.')))
}

fn get(socket: &Path, path: &str, first_line: bool) -> Option<(u16, Vec<u8>)> {
    let mut s = UnixStream::connect(socket).ok()?;
    s.set_read_timeout(Some(WAIT)).ok()?;
    s.set_write_timeout(Some(WAIT)).ok()?;
    s.write_all(format!("GET {path} HTTP/1.0\r\nHost: local-tailscaled.sock\r\n\r\n").as_bytes()).ok()?;
    let end = Instant::now() + WAIT * 3;
    let mut got = Vec::new();
    let mut buf = [0u8; 16 * 1024];
    loop {
        if let Some(h) = got.windows(4).position(|w| w == b"\r\n\r\n") {
            let body = &got[h + 4..];
            if first_line {
                if let Some(nl) = body.iter().position(|&b| b == b'\n') {
                    return Some((status(&got[..h])?, body[..nl].to_vec()));
                }
            }
        }
        if got.len() > MOST || Instant::now() > end {
            return None;
        }
        match s.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => got.extend_from_slice(&buf[..n]),
            Err(_) => return None,
        }
    }
    let h = got.windows(4).position(|w| w == b"\r\n\r\n")?;
    Some((status(&got[..h])?, got[h + 4..].to_vec()))
}

fn status(head: &[u8]) -> Option<u16> {
    std::str::from_utf8(head).ok()?.split_whitespace().nth(1)?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::net::UnixListener;

    fn addr(s: &str) -> SocketAddr {
        s.parse().unwrap()
    }

    #[test]
    fn routes_from_settings() {
        let dns: Value = serde_json::from_str(r#"{"Resolvers":[{"Addr":"9.9.9.9"}],
            "Routes":{"corp.example.":[{"Addr":"10.0.0.53"},{"Addr":"[fd00::53]:5353"}],
                      "lab.corp.example":[{"Addr":"100.101.102.103:53"}],
                      "records.example":[],
                      "doh.example":[{"Addr":"https://dns.example/dns-query"}],
                      "not a name":[{"Addr":"10.0.0.1"}]},
            "Domains":["tail1234.ts.net"]}"#).unwrap();
        let r = parse(&dns);
        assert_eq!(r.iter().map(|s| s.suffix.as_str()).collect::<Vec<_>>(),
                   ["lab.corp.example", "records.example", "corp.example", "doh.example"]);
        assert_eq!(r[0].servers, [addr("100.101.102.103:53")]);
        assert_eq!(r[2].servers, [addr("10.0.0.53:53"), addr("[fd00::53]:5353")]);
        assert!(r[1].servers.is_empty() && r[3].servers.is_empty());
        assert_eq!(find("wiki.lab.corp.example", &r).map(|s| s.suffix.as_str()), Some("lab.corp.example"));
        assert_eq!(find("mail.corp.example", &r).map(|s| s.suffix.as_str()), Some("corp.example"));
        assert_eq!(find("corp.example", &r).map(|s| s.suffix.as_str()), Some("corp.example"));
        assert!(find("notcorp.example", &r).is_none());
        assert!(parse(&Value::Null).is_empty());
    }

    fn fake_tailscaled(dir: &Path, newer: bool) -> std::path::PathBuf {
        let path = dir.join(if newer { "new.sock" } else { "old.sock" });
        let _ = std::fs::remove_file(&path);
        let l = UnixListener::bind(&path).unwrap();
        std::thread::spawn(move || {
            for c in l.incoming() {
                let Ok(mut c) = c else { continue };
                let mut req = Vec::new();
                let mut b = [0u8; 1024];
                while !req.windows(4).any(|w| w == b"\r\n\r\n") {
                    match c.read(&mut b) {
                        Ok(0) | Err(_) => break,
                        Ok(n) => req.extend_from_slice(&b[..n]),
                    }
                }
                let req = String::from_utf8_lossy(&req).to_string();
                assert!(req.contains("Host: local-tailscaled.sock\r\n"));
                let routes = r#"{"Routes":{"corp.example.":[{"Addr":"10.0.0.53"}]}}"#;
                if req.starts_with(&format!("GET {CONFIG} ")) && newer {
                    let _ = write!(c, "HTTP/1.0 200 OK\r\nContent-Type: application/json\r\n\r\n{routes}\n");
                } else if req.starts_with(&format!("GET {WATCH} ")) && !newer {
                    let _ = write!(c, "HTTP/1.0 200 OK\r\n\r\n{{\"Version\":\"1.90.0\",\"NetMap\":{{\"DNS\":{routes}}}}}\n");
                    std::thread::sleep(Duration::from_secs(5));
                } else {
                    let _ = write!(c, "HTTP/1.0 404 Not Found\r\n\r\n404 page not found\n");
                }
            }
        });
        path
    }

    #[test]
    fn asks_tailscaled() {
        let dir = std::env::temp_dir().join(format!("trindle-dns-tailnet-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let want = vec![Split { suffix: "corp.example".into(), servers: vec![addr("10.0.0.53:53")] }];
        assert_eq!(routes_at(&fake_tailscaled(&dir, true)), Some(want.clone()));
        let t = Instant::now();
        assert_eq!(routes_at(&fake_tailscaled(&dir, false)), Some(want));
        assert!(t.elapsed() < Duration::from_secs(2), "the watch is not waited out");
        assert_eq!(routes_at(&dir.join("missing.sock")), None);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
