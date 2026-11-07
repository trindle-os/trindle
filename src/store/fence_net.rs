//! The optional limit on the network for AI programs (an advanced switch on Settings, AI programs, off by default).

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{IpAddr, SocketAddr, TcpStream, ToSocketAddrs};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub const PROXY_PORT: u16 = 3128;
const MOST: usize = 64;
const HEAD_MAX: usize = 8192;

#[derive(Clone, Debug)]
pub struct Policy {
    pub hosts: Vec<String>,
    pub ports: Vec<u16>,
    pub allow_local: bool,
}

pub fn host_pattern_ok(h: &str) -> bool {
    let name = h.strip_prefix("*.").unwrap_or(h);
    super::fence::host_ok(name) && name.contains('.') && !name.ends_with(".local") && !name.ends_with(".localhost") && !name.ends_with(".internal")
        && !name.ends_with(".lan") && !name.ends_with(".home")
}

pub fn host_allowed(list: &[String], host: &str) -> bool {
    let host = host.to_ascii_lowercase();
    if host.parse::<IpAddr>().is_ok() || !super::fence::host_ok(&host) {
        return false;
    }
    list.iter().any(|p| match p.strip_prefix("*.") {
        Some(suffix) => host.len() > suffix.len() + 1 && host.ends_with(suffix) && host[..host.len() - suffix.len()].ends_with('.'),
        None => p.eq_ignore_ascii_case(&host),
    })
}

pub fn is_local(ip: &IpAddr) -> bool {
    match ip {
        IpAddr::V4(a) => a.is_loopback() || a.is_private() || a.is_link_local() || a.is_unspecified() || a.is_broadcast() || a.is_multicast()
            || (a.octets()[0] == 100 && (64..128).contains(&a.octets()[1])) || a.octets()[0] == 0,
        IpAddr::V6(a) => match a.to_ipv4_mapped() {
            Some(v4) => is_local(&IpAddr::V4(v4)),
            None => a.is_loopback() || a.is_unspecified() || a.is_multicast() || (a.segments()[0] & 0xfe00) == 0xfc00 || (a.segments()[0] & 0xffc0) == 0xfe80,
        },
    }
}

pub type Resolver = Arc<dyn Fn(&str, u16) -> Vec<SocketAddr> + Send + Sync>;

pub fn serve(listener: UnixListener, policy: Policy, resolver: Option<Resolver>) {
    let resolver: Resolver = resolver.unwrap_or_else(|| Arc::new(|h, p| (h, p).to_socket_addrs().map(|i| i.collect()).unwrap_or_default()));
    let open = Arc::new(AtomicUsize::new(0));
    for conn in listener.incoming().flatten() {
        if open.fetch_add(1, Ordering::SeqCst) >= MOST {
            open.fetch_sub(1, Ordering::SeqCst);
            continue;
        }
        let (policy, resolver, open) = (policy.clone(), resolver.clone(), open.clone());
        std::thread::spawn(move || {
            let _ = one(conn, &policy, &resolver);
            open.fetch_sub(1, Ordering::SeqCst);
        });
    }
}

fn refuse(conn: &mut UnixStream, code: &str, why: &str) {
    let body = format!("{why}\n");
    let _ = write!(conn, "HTTP/1.1 {code}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len());
}

fn one(mut conn: UnixStream, policy: &Policy, resolve: &Resolver) -> std::io::Result<()> {
    conn.set_read_timeout(Some(Duration::from_secs(15)))?;
    let mut reader = BufReader::new(conn.try_clone()?);
    let mut first = String::new();
    reader.by_ref().take(HEAD_MAX as u64).read_line(&mut first)?;
    let mut seen = first.len();
    loop {
        let mut l = String::new();
        let n = reader.by_ref().take(HEAD_MAX as u64).read_line(&mut l)?;
        seen += n;
        if n == 0 || l == "\r\n" || l == "\n" || seen > HEAD_MAX {
            break;
        }
    }
    let mut words = first.split_whitespace();
    let (Some("CONNECT"), Some(target), Some(version)) = (words.next(), words.next(), words.next()) else {
        refuse(&mut conn, "405 Method Not Allowed", "This computer's fence lets AI programs use secure (https) connections to listed hosts only.");
        return Ok(());
    };
    let Some((host, port)) = target.rsplit_once(':').and_then(|(h, p)| Some((h, p.parse::<u16>().ok()?))) else {
        refuse(&mut conn, "400 Bad Request", "That is not a host and port.");
        return Ok(());
    };
    if !version.starts_with("HTTP/1.") || !host_allowed(&policy.hosts, host) || !policy.ports.contains(&port) {
        refuse(&mut conn, "403 Forbidden", &format!("{host} is not on the list of hosts AI programs may reach. Add it in Settings, AI programs."));
        return Ok(());
    }
    let addrs: Vec<SocketAddr> = resolve(host, port).into_iter().filter(|a| policy.allow_local || !is_local(&a.ip())).collect();
    let Some(out) = addrs.iter().find_map(|a| TcpStream::connect_timeout(a, Duration::from_secs(10)).ok()) else {
        refuse(&mut conn, "502 Bad Gateway", &format!("{host} could not be reached."));
        return Ok(());
    };
    conn.set_read_timeout(None)?;
    conn.write_all(b"HTTP/1.1 200 Connection established\r\n\r\n")?;
    let buffered = reader.buffer().to_vec();
    let mut out_w = out.try_clone()?;
    out_w.write_all(&buffered)?;
    let mut conn_r = conn.try_clone()?;
    let up = std::thread::spawn(move || {
        let _ = std::io::copy(&mut conn_r, &mut out_w);
        let _ = out_w.shutdown(std::net::Shutdown::Write);
    });
    let (mut out_r, mut conn_w) = (out, conn);
    let _ = std::io::copy(&mut out_r, &mut conn_w);
    let _ = conn_w.shutdown(std::net::Shutdown::Write);
    let _ = up.join();
    Ok(())
}

pub fn bridge(socket: &Path) -> ! {
    let socket = socket.to_path_buf();
    let Ok(listener) = std::net::TcpListener::bind(("127.0.0.1", PROXY_PORT)) else { std::process::exit(1) };
    for conn in listener.incoming().flatten() {
        let socket = socket.clone();
        std::thread::spawn(move || {
            let Ok(up) = UnixStream::connect(&socket) else { return };
            let (mut a, mut b) = (conn, up);
            let (mut a2, mut b2) = (a.try_clone().unwrap(), b.try_clone().unwrap());
            let t = std::thread::spawn(move || {
                let _ = std::io::copy(&mut a2, &mut b2);
                let _ = b2.shutdown(std::net::Shutdown::Write);
            });
            let _ = std::io::copy(&mut b, &mut a);
            let _ = a.shutdown(std::net::Shutdown::Write);
            let _ = t.join();
        });
    }
    std::process::exit(0)
}

pub fn proxy_env() -> Vec<(String, String)> {
    let url = format!("http://127.0.0.1:{PROXY_PORT}");
    ["HTTPS_PROXY", "https_proxy", "HTTP_PROXY", "http_proxy", "ALL_PROXY", "all_proxy"].iter().map(|k| (k.to_string(), url.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    #[test]
    fn only_named_hosts_match_and_never_an_address() {
        let list: Vec<String> = ["api.anthropic.com", "*.pythonhosted.org"].iter().map(|s| s.to_string()).collect();
        for ok in ["api.anthropic.com", "API.Anthropic.com", "files.pythonhosted.org", "a.b.pythonhosted.org"] {
            assert!(host_allowed(&list, ok), "{ok}");
        }
        for bad in ["anthropic.com", "evil-api.anthropic.com", "api.anthropic.com.evil.example", "pythonhosted.org", "xpythonhosted.org", "127.0.0.1",
                    "[::1]", "10.0.0.5", "localhost", "", "a b.example", "api.anthropic.com@evil.example"] {
            assert!(!host_allowed(&list, bad), "{bad}");
        }
        for ok in ["api.anthropic.com", "*.crates.io"] {
            assert!(host_pattern_ok(ok), "{ok}");
        }
        for bad in ["127.0.0.1", "localhost", "nas.local", "*.local", "*", "*.", "a", "x.internal", "*.com.", "-x.com", "10.1.2.3"] {
            assert!(!host_pattern_ok(bad), "{bad}");
        }
    }

    #[test]
    fn addresses_inside_this_computer_or_its_network_are_local() {
        for a in ["127.0.0.1", "10.1.2.3", "192.168.1.1", "172.16.0.1", "169.254.1.1", "0.0.0.0", "100.64.0.1", "::1", "fe80::1", "fc00::1", "::ffff:127.0.0.1"] {
            assert!(is_local(&a.parse().unwrap()), "{a}");
        }
        for a in ["8.8.8.8", "1.1.1.1", "2606:4700::1111"] {
            assert!(!is_local(&a.parse().unwrap()), "{a}");
        }
    }

    fn ask(socket: &Path, request: &str) -> String {
        let mut c = UnixStream::connect(socket).unwrap();
        c.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        c.write_all(request.as_bytes()).unwrap();
        let mut got = String::new();
        let mut buf = [0u8; 512];
        while let Ok(n) = c.read(&mut buf) {
            if n == 0 {
                break;
            }
            got.push_str(&String::from_utf8_lossy(&buf[..n]));
            if got.contains("\r\n\r\n") && !got.starts_with("HTTP/1.1 200") {
                break;
            }
            if got.contains("HELLO") {
                break;
            }
        }
        got
    }

    #[test]
    fn the_proxy_lets_through_listed_hosts_only() {
        let server = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let port = server.local_addr().unwrap().port();
        std::thread::spawn(move || {
            for mut c in server.incoming().flatten() {
                let _ = c.write_all(b"HELLO");
            }
        });
        let dir = std::env::temp_dir().join(format!("trindle-fence-net-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let resolver: Resolver = Arc::new(move |h, p| if h == "listed.example.com" || h == "other.example.com" { vec![SocketAddr::from(([127, 0, 0, 1], p))] } else { Vec::new() });
        let mut policy = Policy { hosts: vec!["listed.example.com".into()], ports: vec![port], allow_local: true };
        let (a, b) = (dir.join("a.sock"), dir.join("b.sock"));
        std::thread::spawn({ let (l, p, r) = (UnixListener::bind(&a).unwrap(), policy.clone(), resolver.clone()); move || serve(l, p, Some(r)) });
        let connect = |host: &str, port: u16| format!("CONNECT {host}:{port} HTTP/1.1\r\nHost: {host}:{port}\r\n\r\n");
        assert!(ask(&a, &connect("listed.example.com", port)).starts_with("HTTP/1.1 200"), "a listed host");
        assert!(ask(&a, &connect("listed.example.com", port)).contains("HELLO"), "and its bytes come back");
        for (what, req) in [("an unlisted host", connect("other.example.com", port)), ("an address number", connect("127.0.0.1", port)),
                            ("another port", connect("listed.example.com", port + 1)), ("plain http", format!("GET http://listed.example.com:{port}/ HTTP/1.1\r\n\r\n")),
                            ("a made-up word", "HELLO\r\n\r\n".to_string()), ("a name with a trick", connect("listed.example.com@evil.example", port))] {
            let got = ask(&a, &req);
            assert!(got.starts_with("HTTP/1.1 4") || got.starts_with("HTTP/1.1 5"), "{what}: {got}");
        }
        policy.allow_local = false;
        std::thread::spawn({ let (l, p, r) = (UnixListener::bind(&b).unwrap(), policy, resolver); move || serve(l, p, Some(r)) });
        let got = ask(&b, &connect("listed.example.com", port));
        assert!(got.starts_with("HTTP/1.1 502") && !got.contains("HELLO"), "{got}");
        let _ = std::fs::remove_dir_all(dir);
    }
}
