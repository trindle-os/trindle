//! The HTTP side.

use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};

use super::{HTTP_MOST, HTTP_WAIT, REQUEST_MOST};

pub(super) fn read_request(r: &mut impl Read) -> Option<(String, Vec<u8>)> {
    let mut all = Vec::new();
    let mut buf = [0u8; 4096];
    let head_end = loop {
        if let Some(i) = all.windows(4).position(|w| w == b"\r\n\r\n") {
            break i;
        }
        let n = r.read(&mut buf).ok().filter(|n| *n > 0)?;
        all.extend_from_slice(&buf[..n]);
        if all.len() > REQUEST_MOST {
            return None;
        }
    };
    let head = String::from_utf8_lossy(&all[..head_end]).into_owned();
    let mut first = head.lines().next()?.split_whitespace();
    if first.next()? != "POST" {
        return None;
    }
    let path = first.next()?.to_string();
    let length: usize = head.lines().find_map(|l| {
        let (k, v) = l.split_once(':')?;
        k.trim().eq_ignore_ascii_case("content-length").then(|| v.trim().parse().ok()).flatten()
    })?;
    if length > REQUEST_MOST {
        return None;
    }
    let mut body = all[head_end + 4..].to_vec();
    while body.len() < length {
        let n = r.read(&mut buf).ok().filter(|n| *n > 0)?;
        body.extend_from_slice(&buf[..n]);
    }
    body.truncate(length);
    Some((path, body))
}

pub(super) fn http_reply(body: Option<&str>) -> String {
    match body {
        Some(b) => format!("HTTP/1.1 200 OK\r\nContent-Type: application/soap+xml\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{b}", b.len()),
        None => "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".into(),
    }
}

pub(super) fn url_host(url: &str) -> Option<String> {
    let rest = url.strip_prefix("http://")?;
    let hostport = rest.split('/').next()?;
    let host = hostport.rsplit_once(':').map_or(hostport, |(h, _)| h);
    host.parse::<Ipv4Addr>().ok().map(|a| a.to_string())
}

pub(super) fn post(url: &str, body: &str) -> Option<Vec<u8>> {
    let rest = url.strip_prefix("http://")?;
    let (hostport, path) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    let addr: SocketAddr = match hostport.rsplit_once(':') {
        Some((h, p)) => SocketAddrV4::new(h.parse().ok()?, p.parse().ok()?).into(),
        None => SocketAddrV4::new(hostport.parse().ok()?, 80).into(),
    };
    let mut s = TcpStream::connect_timeout(&addr, HTTP_WAIT).ok()?;
    s.set_read_timeout(Some(HTTP_WAIT)).ok()?;
    s.set_write_timeout(Some(HTTP_WAIT)).ok()?;
    let head = format!(
        "POST {path} HTTP/1.1\r\nHost: {hostport}\r\nContent-Type: application/soap+xml\r\n\
         User-Agent: wsdd\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    s.write_all(head.as_bytes()).ok()?;
    s.write_all(body.as_bytes()).ok()?;
    let mut all = Vec::new();
    let _ = s.take(HTTP_MOST).read_to_end(&mut all);
    let split = all.windows(4).position(|w| w == b"\r\n\r\n")?;
    let head = String::from_utf8_lossy(&all[..split]).to_ascii_lowercase();
    let body = &all[split + 4..];
    if !head.lines().next()?.split_whitespace().nth(1)?.starts_with('2') {
        return None;
    }
    if head.lines().any(|l| l.starts_with("transfer-encoding:") && l.contains("chunked")) {
        return Some(dechunk(body));
    }
    Some(body.to_vec())
}

fn dechunk(mut data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    while let Some(end) = data.windows(2).position(|w| w == b"\r\n") {
        let size = String::from_utf8_lossy(&data[..end]);
        let size = size.split(';').next().unwrap_or("").trim();
        let Ok(size) = usize::from_str_radix(size, 16) else { break };
        data = &data[end + 2..];
        if size == 0 || data.len() < size {
            out.extend_from_slice(&data[..size.min(data.len())]);
            break;
        }
        out.extend_from_slice(&data[..size]);
        data = data.get(size + 2..).unwrap_or(&[]);
    }
    out
}
