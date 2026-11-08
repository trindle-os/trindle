//! The DNS message format, only as far as trindle-dns needs it: the one
//! question, the client's answer size, building the few answers it makes
//! itself, and the time-to-live of each record for the cache.


pub const HEADER: usize = 12;
const OPT: u16 = 41;
pub const SERVFAIL: u8 = 2;
const FORMERR: u8 = 1;
const NXDOMAIN: u8 = 3;
pub const REFUSED: u8 = 5;
pub const A: u16 = 1;
pub const AAAA: u16 = 28;
const CNAME: u16 = 5;

const UDP_MOST: usize = 4096;

#[derive(Debug, Clone, PartialEq)]
pub struct Question {

    pub name: String,
    pub kind: u16,
    pub class: u16,

    pub end: usize,
}

fn u16_at(m: &[u8], i: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*m.get(i)?, *m.get(i + 1)?]))
}

fn count(m: &[u8], which: usize) -> usize {
    u16_at(m, 4 + 2 * which).unwrap_or(0) as usize
}

pub fn question(m: &[u8]) -> Option<Question> {
    if m.len() < HEADER || m[2] & 0x80 != 0 || (m[2] >> 3) & 0x0f != 0 || count(m, 0) != 1 {
        return None;
    }
    let mut i = HEADER;
    let mut name = String::new();
    loop {
        let len = *m.get(i)? as usize;
        i += 1;
        if len == 0 {
            break;
        }

        if len > 63 {
            return None;
        }
        let label = m.get(i..i + len)?;
        if !name.is_empty() {
            name.push('.');
        }
        name.extend(label.iter().map(|b| b.to_ascii_lowercase() as char));
        i += len;
        if name.len() > crate::common::dns::NAME_MOST {
            return None;
        }
    }
    let kind = u16_at(m, i)?;
    let class = u16_at(m, i + 2)?;
    Some(Question { name, kind, class, end: i + 4 })
}

fn skip_name(m: &[u8], mut i: usize) -> Option<usize> {
    loop {
        let len = *m.get(i)?;
        match len {
            0 => return Some(i + 1),
            l if l & 0xc0 == 0xc0 => return m.get(i + 1).map(|_| i + 2),
            l if l & 0xc0 != 0 => return None,
            l => i += 1 + l as usize,
        }
    }
}

fn records(m: &[u8], from: usize) -> Option<Vec<(usize, u16)>> {
    let n = count(m, 1) + count(m, 2) + count(m, 3);
    let mut out = Vec::with_capacity(n);
    let mut i = from;
    for _ in 0..n {
        i = skip_name(m, i)?;
        let kind = u16_at(m, i)?;
        let len = u16_at(m, i + 8)? as usize;
        if m.len() < i + 10 + len {
            return None;
        }
        out.push((i, kind));
        i += 10 + len;
    }
    Some(out)
}

fn read_name(m: &[u8], mut i: usize) -> Option<String> {
    let mut name = String::new();
    for _ in 0..128 {
        let len = *m.get(i)? as usize;
        if len & 0xc0 == 0xc0 {
            i = ((len & 0x3f) << 8) | *m.get(i + 1)? as usize;
            continue;
        }
        if len == 0 {
            return Some(name);
        }
        let label = m.get(i + 1..i + 1 + len)?;
        if !name.is_empty() {
            name.push('.');
        }
        name.extend(label.iter().map(|b| b.to_ascii_lowercase() as char));
        i += 1 + len;
    }
    None
}

fn put_name(m: &mut Vec<u8>, name: &str) {
    for label in name.split('.').filter(|l| !l.is_empty()) {
        m.push(label.len().min(63) as u8);
        m.extend_from_slice(&label.as_bytes()[..label.len().min(63)]);
    }
    m.push(0);
}

pub fn ask(name: &str, kind: u16, id: [u8; 2]) -> Vec<u8> {
    let mut m = vec![id[0], id[1], 0x01, 0x00, 0, 1, 0, 0, 0, 0, 0, 0];
    put_name(&mut m, name);
    m.extend_from_slice(&kind.to_be_bytes());
    m.extend_from_slice(&[0, 1]);
    m
}

pub fn flavour(m: &[u8], q: &Question) -> u8 {
    let opt = records(m, q.end).and_then(|rs| rs.into_iter().find(|(_, k)| *k == OPT));
    let mut f = 0;
    if let Some((at, _)) = opt {
        f |= 1;
        if m.get(at + 6).is_some_and(|b| b & 0x80 != 0) {
            f |= 2;
        }
    }
    if m.get(3).is_some_and(|b| b & 0x10 != 0) {
        f |= 4;
    }
    f
}

fn answer_records(m: &[u8], q: &Question) -> Vec<(usize, usize, u16)> {
    let mut out = Vec::new();
    let mut i = q.end;
    for _ in 0..count(m, 1) {
        let Some(at) = skip_name(m, i) else { break };
        let (Some(kind), Some(len)) = (u16_at(m, at), u16_at(m, at + 8)) else { break };
        if m.len() < at + 10 + len as usize {
            break;
        }
        out.push((i, at, kind));
        i = at + 10 + len as usize;
    }
    out
}

pub fn has(m: &[u8], q: &Question, kind: u16) -> bool {
    answer_records(m, q).iter().any(|(_, _, k)| *k == kind)
}

const SPOTS: [(u8, [usize; 4]); 6] = [
    (96, [12, 13, 14, 15]),
    (64, [9, 10, 11, 12]),
    (56, [7, 9, 10, 11]),
    (48, [6, 7, 9, 10]),
    (40, [5, 6, 7, 9]),
    (32, [4, 5, 6, 7]),
];

pub fn nat64_prefix(m: &[u8], q: &Question) -> Option<([u8; 16], u8)> {
    for (_, at, kind) in answer_records(m, q) {
        if kind != AAAA || u16_at(m, at + 8) != Some(16) {
            continue;
        }
        let mut ip = [0u8; 16];
        ip.copy_from_slice(&m[at + 10..at + 26]);
        for (len, spots) in SPOTS {
            let v4 = spots.map(|i| ip[i]);
            if v4 == [192, 0, 0, 170] || v4 == [192, 0, 0, 171] {
                let mut prefix = [0u8; 16];
                prefix[..len as usize / 8].copy_from_slice(&ip[..len as usize / 8]);
                return Some((prefix, len));
            }
        }
    }
    None
}

pub fn embed(prefix: &[u8; 16], len: u8, v4: [u8; 4]) -> [u8; 16] {
    let mut ip = *prefix;
    if let Some((_, spots)) = SPOTS.iter().find(|(l, _)| *l == len) {
        for (i, b) in spots.iter().zip(v4) {
            ip[*i] = b;
        }
    }
    ip
}

pub fn synthesise(query: &[u8], q: &Question, four: &[u8], four_q: &Question, prefix: &[u8; 16], len: u8) -> Option<Vec<u8>> {
    let mut body = Vec::new();
    let (mut n, mut addresses) = (0u16, 0);
    for (name_at, at, kind) in answer_records(four, four_q) {
        let owner = read_name(four, name_at)?;
        let ttl = four.get(at + 4..at + 8)?;
        let data = four.get(at + 10..at + 10 + u16_at(four, at + 8)? as usize)?;
        let rdata = match kind {
            A if data.len() == 4 => {
                addresses += 1;
                embed(prefix, len, [data[0], data[1], data[2], data[3]]).to_vec()
            }
            CNAME => {
                let mut t = Vec::new();
                put_name(&mut t, &read_name(four, at + 10)?);
                t
            }
            _ => continue,
        };
        put_name(&mut body, &owner);
        body.extend_from_slice(&if kind == A { AAAA } else { kind }.to_be_bytes());
        body.extend_from_slice(&[0, 1]);
        body.extend_from_slice(ttl);
        body.extend_from_slice(&(rdata.len() as u16).to_be_bytes());
        body.extend_from_slice(&rdata);
        n += 1;
    }
    if addresses == 0 {
        return None;
    }
    let mut r = reply(query, q, 0, false, n);
    r.extend_from_slice(&body);
    Some(r)
}

pub fn first_ptr(m: &[u8], q: &Question) -> Option<String> {
    let (at, _) = records(m, q.end)?.into_iter().take(count(m, 1)).find(|(_, k)| *k == 12)?;
    read_name(m, at + 10)
}

pub fn udp_size(m: &[u8], q: &Question) -> usize {
    let said = records(m, q.end).and_then(|rs| rs.into_iter().find(|(_, k)| *k == OPT))
        .and_then(|(at, _)| u16_at(m, at + 2));
    said.map_or(512, |s| (s as usize).clamp(512, UDP_MOST))
}

fn reply(query: &[u8], q: &Question, rcode: u8, truncated: bool, answers: u16) -> Vec<u8> {
    let mut r = Vec::with_capacity(q.end + 32);
    r.extend_from_slice(&query[..2]);
    r.push(0x80 | (query[2] & 0x01) | if truncated { 0x02 } else { 0 });
    r.push(0x80 | rcode);
    r.extend_from_slice(&[0, 1]);
    r.extend_from_slice(&answers.to_be_bytes());
    r.extend_from_slice(&[0, 0, 0, 0]);
    r.extend_from_slice(&query[HEADER..q.end]);
    r
}

pub fn blocked(query: &[u8], q: &Question) -> Vec<u8> {
    let data: &[u8] = match (q.kind, q.class) {
        (1, 1) => &[0; 4],
        (28, 1) => &[0; 16],
        _ => &[],
    };
    let mut r = reply(query, q, 0, false, if data.is_empty() { 0 } else { 1 });
    if !data.is_empty() {

        r.extend_from_slice(&[0xc0, 0x0c]);
        r.extend_from_slice(&q.kind.to_be_bytes());
        r.extend_from_slice(&q.class.to_be_bytes());
        r.extend_from_slice(&300u32.to_be_bytes());
        r.extend_from_slice(&(data.len() as u16).to_be_bytes());
        r.extend_from_slice(data);
    }
    r
}

pub fn nxdomain(query: &[u8], q: &Question) -> Vec<u8> {
    reply(query, q, NXDOMAIN, false, 0)
}

pub fn servfail(query: &[u8], q: &Question) -> Vec<u8> {
    reply(query, q, SERVFAIL, false, 0)
}

pub fn refused(query: &[u8], q: &Question) -> Vec<u8> {
    reply(query, q, REFUSED, false, 0)
}

pub fn truncated(query: &[u8], q: &Question) -> Vec<u8> {
    reply(query, q, 0, true, 0)
}

pub fn formerr(query: &[u8]) -> Option<Vec<u8>> {
    if query.len() < HEADER || query[2] & 0x80 != 0 {
        return None;
    }
    let mut r = query[..HEADER].to_vec();
    r[2] = 0x80 | (query[2] & 0x79);
    r[3] = 0x80 | FORMERR;
    r[4..].fill(0);
    Some(r)
}

pub fn rcode(m: &[u8]) -> u8 {
    m.get(3).map_or(SERVFAIL, |b| b & 0x0f)
}

pub fn not_checked(m: &mut [u8]) {
    if let Some(b) = m.get_mut(3) {
        *b &= !0x20;
    }
}

pub fn is_truncated(m: &[u8]) -> bool {
    m.get(2).is_some_and(|b| b & 0x02 != 0)
}

pub fn answers(reply: &[u8], query: &[u8], q: &Question) -> bool {
    reply.len() >= q.end && reply[..2] == query[..2] && reply[2] & 0x80 != 0
        && reply[HEADER..q.end].eq_ignore_ascii_case(&query[HEADER..q.end])
}

pub fn ttl(m: &[u8], q: &Question) -> Option<u32> {
    records(m, q.end)?.into_iter().filter(|(_, k)| *k != OPT)
        .filter_map(|(at, _)| m.get(at + 4..at + 8).map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]])))
        .min()
}

pub fn age(m: &mut [u8], q: &Question, by: u32) {
    for (at, kind) in records(m, q.end).unwrap_or_default() {
        if kind == OPT {
            continue;
        }
        let t = u32::from_be_bytes([m[at + 4], m[at + 5], m[at + 6], m[at + 7]]);
        m[at + 4..at + 8].copy_from_slice(&t.saturating_sub(by).to_be_bytes());
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn query(name: &str, kind: u16, size: Option<u16>) -> Vec<u8> {
        let mut m = vec![0x12, 0x34, 0x01, 0x00, 0, 1, 0, 0, 0, 0, 0, if size.is_some() { 1 } else { 0 }];
        for label in name.split('.').filter(|l| !l.is_empty()) {
            m.push(label.len() as u8);
            m.extend_from_slice(label.as_bytes());
        }
        m.push(0);
        m.extend_from_slice(&kind.to_be_bytes());
        m.extend_from_slice(&[0, 1]);
        if let Some(s) = size {
            m.extend_from_slice(&[0, 0, 41]);
            m.extend_from_slice(&s.to_be_bytes());
            m.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
        }
        m
    }

    pub fn answer(q: &[u8], ttls: &[u32]) -> Vec<u8> {
        let qn = question(q).unwrap();
        let mut r = q[..qn.end].to_vec();
        r[2] |= 0x80;
        r[3] = 0x80;
        r[6..8].copy_from_slice(&(ttls.len() as u16).to_be_bytes());
        r[10..12].copy_from_slice(&[0, 0]);
        for t in ttls {
            r.extend_from_slice(&[0xc0, 0x0c, 0, 1, 0, 1]);
            r.extend_from_slice(&t.to_be_bytes());
            r.extend_from_slice(&[0, 4, 93, 184, 215, 14]);
        }
        r
    }

    #[test]
    fn reads_the_question() {
        let mut m = query("Ads.Example.COM", 1, Some(1232));
        let q = question(&m).unwrap();
        assert_eq!((q.name.as_str(), q.kind, q.class), ("ads.example.com", 1, 1));
        assert_eq!(udp_size(&m, &q), 1232);
        assert_eq!(udp_size(&query("a.b", 1, None), &q_of("a.b")), 512);
        assert_eq!(question(&query("", 2, None)).unwrap().name, "");

        m[2] |= 0x80;
        assert_eq!(question(&m), None);
        let mut two = query("a.b", 1, None);
        two[5] = 2;
        assert_eq!(question(&two), None);
        let mut pointer = query("a.b", 1, None);
        pointer[12] = 0xc0;
        assert_eq!(question(&pointer), None);
        assert_eq!(question(&[0; 5]), None);
    }

    fn q_of(name: &str) -> Question {
        question(&query(name, 1, None)).unwrap()
    }

    #[test]
    fn blocked_answers() {
        let m = query("ads.example.com", 1, None);
        let q = question(&m).unwrap();
        let r = blocked(&m, &q);
        assert_eq!(&r[..4], &[0x12, 0x34, 0x81, 0x80]);
        assert_eq!(&r[6..8], &[0, 1]);
        assert_eq!(&r[r.len() - 4..], &[0, 0, 0, 0]);
        assert_eq!(ttl(&r, &q), Some(300));
        let m6 = query("ads.example.com", 28, None);
        let r6 = blocked(&m6, &question(&m6).unwrap());
        assert_eq!(r6.len(), m6.len() + 12 + 16);
        let mx = query("ads.example.com", 15, None);
        let rx = blocked(&mx, &question(&mx).unwrap());
        assert_eq!((rx.len(), rcode(&rx), &rx[6..8]), (mx.len(), 0, &[0u8, 0][..]));
        let f = servfail(&m, &q);
        assert_eq!((rcode(&f), is_truncated(&f)), (SERVFAIL, false));
        assert!(is_truncated(&truncated(&m, &q)));
        assert_eq!(rcode(&formerr(&m).unwrap()), 1);
    }

    #[test]
    fn ttls_for_the_cache() {
        let m = query("example.com", 1, None);
        let q = question(&m).unwrap();
        let mut r = answer(&m, &[600, 120, 3600]);
        assert!(answers(&r, &m, &q));
        assert_eq!(ttl(&r, &q), Some(120));
        age(&mut r, &q, 100);
        assert_eq!(ttl(&r, &q), Some(20));
        age(&mut r, &q, 1000);
        assert_eq!(ttl(&r, &q), Some(0));
        assert_eq!(ttl(&answer(&m, &[]), &q), None);

        let mut other = answer(&m, &[60]);
        other[1] ^= 1;
        assert!(!answers(&other, &m, &q));

        let mut short = answer(&m, &[60]);
        short.pop();
        assert_eq!(ttl(&short, &q), None);
    }

    #[test]
    fn flavours_kept_apart() {
        let plain = query("example.com", 1, None);
        assert_eq!(flavour(&plain, &question(&plain).unwrap()), 0);
        let edns = query("example.com", 1, Some(1232));
        let q = question(&edns).unwrap();
        assert_eq!(flavour(&edns, &q), 1);
        let mut dnssec = edns.clone();
        let at = dnssec.len() - 4;
        dnssec[at] = 0x80;
        assert_eq!(flavour(&dnssec, &q), 3);
        dnssec[3] |= 0x10;
        assert_eq!(flavour(&dnssec, &q), 7);
        assert_eq!(rcode(&nxdomain(&plain, &question(&plain).unwrap())), 3);
    }

    pub fn with(q: &[u8], recs: &[(u16, &[u8])]) -> Vec<u8> {
        let mut r = answer(q, &[]);
        r[6..8].copy_from_slice(&(recs.len() as u16).to_be_bytes());
        for (kind, data) in recs {
            r.extend_from_slice(&[0xc0, 0x0c]);
            r.extend_from_slice(&kind.to_be_bytes());
            r.extend_from_slice(&[0, 1, 0, 0, 1, 44]);
            r.extend_from_slice(&(data.len() as u16).to_be_bytes());
            r.extend_from_slice(data);
        }
        r
    }

    #[test]
    fn dns64() {

        let m = ask("ipv4only.arpa", AAAA, [1, 1]);
        let q = question(&m).unwrap();
        let mut wk = [0u8; 16];
        wk[..4].copy_from_slice(&[0, 0x64, 0xff, 0x9b]);
        wk[12..].copy_from_slice(&[192, 0, 0, 170]);
        let (prefix, len) = nat64_prefix(&with(&m, &[(AAAA, &wk)]), &q).unwrap();
        assert_eq!((len, &prefix[..4], &prefix[4..]), (96, &[0, 0x64, 0xff, 0x9b][..], &[0u8; 12][..]));
        let mut p56 = [0x20, 0x01, 0x0d, 0xb8, 0x12, 0x34, 0x56, 192, 0, 0, 0, 171, 0, 0, 0, 0];
        p56[8] = 0;
        let (prefix56, len56) = nat64_prefix(&with(&m, &[(AAAA, &p56)]), &q).unwrap();
        assert_eq!(len56, 56);
        assert_eq!(embed(&prefix56, 56, [192, 0, 0, 171]), p56);
        assert_eq!(nat64_prefix(&with(&m, &[]), &q), None);

        let six = query("www.example.com", AAAA, None);
        let sq = question(&six).unwrap();
        let four = ask("www.example.com", A, [2, 2]);
        let fq = question(&four).unwrap();
        let cname = [&[3u8][..], b"cdn", &[7], b"example", &[3], b"net", &[0]].concat();
        let mut fr = with(&four, &[(CNAME, &cname)]);

        let target = (fq.end + 12) as u8;
        fr[7] = 2;
        fr.extend_from_slice(&[0xc0, target, 0, 1, 0, 1, 0, 0, 0, 60, 0, 4, 93, 184, 215, 14]);
        let r = synthesise(&six, &sq, &fr, &fq, &prefix, len).unwrap();
        assert!(answers(&r, &six, &sq) && has(&r, &sq, AAAA));
        let recs = answer_records(&r, &sq);
        assert_eq!(recs.len(), 2);
        assert_eq!(read_name(&r, recs[1].0).as_deref(), Some("cdn.example.net"));
        let at = recs[1].1;
        assert_eq!(&r[at + 10..at + 26], &embed(&prefix, 96, [93, 184, 215, 14]));
        assert_eq!(ttl(&r, &sq), Some(60));
        assert!(synthesise(&six, &sq, &with(&four, &[]), &fq, &prefix, len).is_none());
    }

    #[test]
    fn reads_a_ptr() {
        let m = ask("5.0.64.100.in-addr.arpa", 12, [1, 2]);
        let q = question(&m).unwrap();
        assert_eq!((q.name.as_str(), q.kind), ("5.0.64.100.in-addr.arpa", 12));
        let mut r = m.clone();
        r[2] = 0x81;
        r[3] = 0x80;
        r[7] = 1;

        let target = r.len() + 12 + 9;
        r.extend_from_slice(&[0xc0, 0x0c, 0, 12, 0, 1, 0, 0, 0, 60]);
        let data = [&[6u8][..], b"laptop", &[0xc0, target as u8]].concat();
        let rest = [&[5u8][..], b"tail1", &[2], b"ts", &[3], b"net", &[0]].concat();
        let len = data.len() + rest.len();
        r.extend_from_slice(&(len as u16).to_be_bytes());
        r.extend_from_slice(&data);
        assert_eq!(r.len(), target);
        r.extend_from_slice(&rest);
        assert_eq!(first_ptr(&r, &q).as_deref(), Some("laptop.tail1.ts.net"));
    }
}
