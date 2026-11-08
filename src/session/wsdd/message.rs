//! The messages sent and the names and times written into them.

use super::{MEX, PNPX, PUB, SOAP, WSA, WSD, WSDP};

pub(super) fn same_type(a: &str, b: &str) -> bool {
    let local = |s: &str| s.rsplit(':').next().unwrap_or(s).to_string();
    local(a) == local(b)
}

pub(super) fn clean(s: &str) -> String {
    s.replace(['\t', '\n', '\r'], " ")
}

pub(super) fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub(super) fn envelope(to: &str, action: &str, id: &str, extra: &str, body: &str) -> String {
    format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\
         <soap:Envelope xmlns:soap=\"{SOAP}\" xmlns:wsa=\"{WSA}\" xmlns:wsd=\"{WSD}\" xmlns:wsdp=\"{WSDP}\" \
         xmlns:pub=\"{PUB}\" xmlns:wsx=\"{MEX}\" xmlns:pnpx=\"{PNPX}\">\
         <soap:Header><wsa:To>{to}</wsa:To><wsa:Action>{action}</wsa:Action>\
         <wsa:MessageID>{id}</wsa:MessageID>{extra}</soap:Header>\
         <soap:Body>{body}</soap:Body></soap:Envelope>"
    )
}

pub(super) fn random_byte() -> u8 {
    let mut b = [0u8; 1];
    let _ = getrandom::fill(&mut b);
    b[0]
}

pub(super) fn new_urn() -> String {
    let mut b = [0u8; 16];
    let _ = getrandom::fill(&mut b);
    urn_of(b)
}

pub(super) fn urn_of(mut b: [u8; 16]) -> String {
    b[6] = (b[6] & 0x0f) | 0x40;
    b[8] = (b[8] & 0x3f) | 0x80;
    let h = b.iter().fold(String::new(), |mut s, x| {
        let _ = std::fmt::Write::write_fmt(&mut s, format_args!("{x:02x}"));
        s
    });
    format!("urn:uuid:{}-{}-{}-{}-{}", &h[0..8], &h[8..12], &h[12..16], &h[16..20], &h[20..32])
}

pub(super) fn local_time(secs: i64) -> String {
    let t = secs as libc::time_t;
    // SAFETY: tm is plain data, so all zeroes is a valid value to fill in.
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    // SAFETY: both pointers are to live locals for the length of the call.
    if unsafe { libc::localtime_r(&t, &mut tm) }.is_null() {
        return String::new();
    }
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
        tm.tm_year + 1900,
        tm.tm_mon + 1,
        tm.tm_mday,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec
    )
}
