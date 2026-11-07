//! Answers kept for as long as they say they are good for, so a page that asks for the same names again and again is answered at once.

use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::wire::{self, Question};

const MOST: usize = 4096;
const EMPTY: u32 = 60;
const LONGEST: u32 = 86_400;

struct Kept {
    answer: Vec<u8>,
    at: Instant,
    good_for: u32,
}

#[derive(Default)]
pub struct Cache {
    kept: HashMap<(String, u16, u16, u8), Kept>,
}

impl Cache {
    pub fn get(&self, query: &[u8], q: &Question, now: Instant) -> Option<Vec<u8>> {
        let k = self.kept.get(&(q.name.clone(), q.kind, q.class, wire::flavour(query, q)))?;
        let age = now.saturating_duration_since(k.at).as_secs() as u32;
        if age >= k.good_for {
            return None;
        }
        let mut r = k.answer.clone();
        r[..2].copy_from_slice(&query[..2]);
        r[wire::HEADER..q.end].copy_from_slice(&query[wire::HEADER..q.end]);
        wire::age(&mut r, q, age);
        Some(r)
    }

    pub fn put(&mut self, query: &[u8], q: &Question, answer: &[u8], now: Instant) {
        if answer.len() < q.end || wire::rcode(answer) == wire::SERVFAIL || wire::is_truncated(answer) {
            return;
        }
        let good_for = wire::ttl(answer, q).unwrap_or(EMPTY).min(LONGEST);
        if good_for == 0 {
            return;
        }
        if self.kept.len() >= MOST {
            self.kept.retain(|_, k| now.saturating_duration_since(k.at) < Duration::from_secs(k.good_for as u64));
            if self.kept.len() >= MOST {
                let mut ends: Vec<Instant> = self.kept.values().map(|k| k.at + Duration::from_secs(k.good_for as u64)).collect();
                ends.sort_unstable();
                let cut = ends[ends.len() / 2];
                self.kept.retain(|_, k| k.at + Duration::from_secs(k.good_for as u64) > cut);
            }
        }
        self.kept.insert((q.name.clone(), q.kind, q.class, wire::flavour(query, q)), Kept { answer: answer.to_vec(), at: now, good_for });
    }

    pub fn clear(&mut self) {
        self.kept = HashMap::new();
    }

    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.kept.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dns::wire::tests::{answer, query};
    use crate::dns::wire::{question, ttl};

    #[test]
    fn kept_and_aged() {
        let mut c = Cache::default();
        let t = Instant::now();
        let m = query("Example.com", 1, None);
        let q = question(&m).unwrap();
        c.put(&m, &q, &answer(&m, &[120]), t);
        let mut again = query("EXAMPLE.com", 1, None);
        again[0] = 0x77;
        let qa = question(&again).unwrap();
        let r = c.get(&again, &qa, t + Duration::from_secs(20)).unwrap();
        assert_eq!(r[0], 0x77);
        assert_eq!(&r[12..qa.end], &again[12..qa.end]);
        assert_eq!(ttl(&r, &qa), Some(100));
        assert!(c.get(&again, &qa, t + Duration::from_secs(120)).is_none());
        let six = query("example.com", 28, None);
        assert!(c.get(&six, &question(&six).unwrap(), t).is_none());
        let mut signed = query("example.com", 1, Some(1232));
        let at = signed.len() - 4;
        signed[at] = 0x80;
        assert!(c.get(&signed, &question(&signed).unwrap(), t).is_none());
        let mut fail = answer(&m, &[]);
        fail[3] = 0x82;
        let mut cut = answer(&m, &[60]);
        cut[2] |= 0x02;
        let mut c2 = Cache::default();
        c2.put(&m, &q, &fail, t);
        c2.put(&m, &q, &cut, t);
        assert_eq!(c2.len(), 0);
    }

    #[test]
    fn never_too_big() {
        let mut c = Cache::default();
        let t = Instant::now();
        for i in 0..MOST + 10 {
            let m = query(&format!("n{i}.example.com"), 1, None);
            c.put(&m, &question(&m).unwrap(), &answer(&m, &[60 + i as u32]), t);
        }
        assert!(c.len() <= MOST);
        assert!(c.len() > MOST / 4);
    }
}
