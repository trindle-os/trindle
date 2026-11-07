//! Security fixes for the Debian packages carried inside boxes.

use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::Path;
use std::process::{Command, Stdio};

use super::{boxed, parts};
use crate::common::updating;

const WAIT_FOR_TURN: std::time::Duration = std::time::Duration::from_secs(3600);

#[derive(Debug, PartialEq)]
pub struct Fix {
    pub package: String,
    pub old: String,
    pub new: String,
}

fn order(c: Option<u8>) -> i32 {
    match c {
        Some(b'~') => -1,
        None => 0,
        Some(c) if c.is_ascii_alphabetic() => c as i32,
        Some(c) => c as i32 + 256,
    }
}

fn compare_part(a: &str, b: &str) -> Ordering {
    let (mut a, mut b) = (a.as_bytes(), b.as_bytes());
    while !a.is_empty() || !b.is_empty() {
        loop {
            let (x, y) = (a.first().copied().filter(|c| !c.is_ascii_digit()), b.first().copied().filter(|c| !c.is_ascii_digit()));
            if x.is_none() && y.is_none() {
                break;
            }
            let o = order(x).cmp(&order(y));
            if o != Ordering::Equal {
                return o;
            }
            a = &a[1..];
            b = &b[1..];
        }
        let digits = |s: &[u8]| s.iter().take_while(|c| c.is_ascii_digit()).count();
        let (da, db) = (digits(a), digits(b));
        let num = |s: &[u8]| s.iter().skip_while(|c| **c == b'0').copied().collect::<Vec<u8>>();
        let (na, nb) = (num(&a[..da]), num(&b[..db]));
        let o = na.len().cmp(&nb.len()).then_with(|| na.cmp(&nb));
        if o != Ordering::Equal {
            return o;
        }
        a = &a[da..];
        b = &b[db..];
    }
    Ordering::Equal
}

pub fn compare(a: &str, b: &str) -> Ordering {
    let split = |v: &str| {
        let (epoch, rest) = match v.split_once(':') {
            Some((e, r)) if e.chars().all(|c| c.is_ascii_digit()) => (e.parse::<u64>().unwrap_or(0), r.to_string()),
            _ => (0, v.to_string()),
        };
        let (up, rev) = match rest.rsplit_once('-') {
            Some((u, r)) => (u.to_string(), r.to_string()),
            None => (rest, String::new()),
        };
        (epoch, up, rev)
    };
    let (ea, ua, ra) = split(a);
    let (eb, ub, rb) = split(b);
    ea.cmp(&eb).then_with(|| compare_part(&ua, &ub)).then_with(|| compare_part(&ra, &rb))
}

pub fn security_versions(policy: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut current: Option<String> = None;
    for line in policy.lines() {
        let t = line.trim_start_matches([' ', '*']).trim();
        let words: Vec<&str> = t.split_whitespace().collect();
        if words.len() == 2 && words[1].parse::<i32>().is_ok() && !words[0].contains("://") {
            current = Some(words[0].to_string());
        } else if words.len() >= 3 && words[0].parse::<i32>().is_ok() && words[2].contains("-security") {
            if let Some(v) = &current {
                if !out.contains(v) {
                    out.push(v.clone());
                }
            }
        }
    }
    out
}

pub fn to_refresh(carried: &[(String, String, String)], security: &HashMap<String, Vec<String>>) -> Vec<Fix> {
    carried.iter().filter_map(|(package, old, _)| {
        let newest = security.get(package)?.iter().filter(|v| compare(v, old) == Ordering::Greater)
            .max_by(|a, b| compare(a, b))?;
        Some(Fix { package: package.clone(), old: old.clone(), new: newest.clone() })
    }).collect()
}

fn policy(package: &str) -> Vec<String> {
    let out = Command::new("apt-cache").args(["policy", "--", package]).stderr(Stdio::null()).output();
    out.map(|o| security_versions(&String::from_utf8_lossy(&o.stdout))).unwrap_or_default()
}

fn fetch(fix: &Fix, into: &Path) -> Result<String, String> {
    std::fs::create_dir_all(into).map_err(|e| e.to_string())?;
    let ok = Command::new("apt-get").args(["download", "-q", "--", &format!("{}={}", fix.package, fix.new)])
        .current_dir(into).stdout(Stdio::null()).stderr(Stdio::null()).status().is_ok_and(|s| s.success());
    if !ok {
        return Err(format!("{} {} could not be fetched", fix.package, fix.new));
    }
    let deb = std::fs::read_dir(into).map_err(|e| e.to_string())?.flatten().map(|e| e.path())
        .find(|p| p.extension().is_some_and(|x| x == "deb") && parts::field_of(&parts::control_of(p), "Package") == fix.package)
        .ok_or("the fetched package was not found")?;
    let key = parts::make(&deb);
    let _ = std::fs::remove_file(&deb);
    key
}

pub fn main() -> i32 {
    if !crate::common::security_updates::on() {
        return 0;
    }
    let Some(_hold) = updating::begin_waiting(updating::Who::Boxes, WAIT_FOR_TURN) else {
        eprintln!("trindle-boxed: other updates were still going in, so the boxes are left for the next run");
        return 0;
    };
    let boxes: Vec<String> = boxed::installed();
    let mut security: HashMap<String, Vec<String>> = HashMap::new();
    for name in &boxes {
        for (package, _, _) in boxed::carried(name) {
            if let std::collections::hash_map::Entry::Vacant(e) = security.entry(package) {
                let v = policy(e.key());
                e.insert(v);
            }
        }
    }
    let downloads = parts::store().join(".downloads.new");
    let mut made: HashMap<(String, String), String> = HashMap::new();
    for name in &boxes {
        refresh_box(name, &security, &downloads, &mut made);
    }
    let _ = std::fs::remove_dir_all(&downloads);
    parts::sweep();
    0
}

fn refresh_box(name: &str, security: &HashMap<String, Vec<String>>, downloads: &Path, made: &mut HashMap<(String, String), String>) {
    let fixes = to_refresh(&boxed::carried(name), security);
    if fixes.is_empty() {
        return;
    }
    let at = boxed::folder(name);
    let mut keys: Vec<String> = std::fs::read_to_string(at.join("parts")).unwrap_or_default()
        .lines().map(String::from).filter(|l| !l.is_empty()).collect();
    let mut changed = false;
    for fix in &fixes {
        let id = (fix.package.clone(), fix.new.clone());
        let key = match made.get(&id) {
            Some(k) => k.clone(),
            None => match fetch(fix, downloads) {
                Ok(k) => {
                    made.insert(id, k.clone());
                    k
                }
                Err(e) => {
                    eprintln!("trindle-boxed: {name}: {e}");
                    continue;
                }
            },
        };
        for k in keys.iter_mut() {
            if parts::field_of(&parts::control(k), "Package") == fix.package {
                *k = key.clone();
                changed = true;
            }
        }
        eprintln!("trindle-boxed: {name}: {} {} replaced by security fix {}", fix.package, fix.old, fix.new);
    }
    if changed {
        if let Err(e) = boxed::install::relink(&at, &keys) {
            eprintln!("trindle-boxed: {name}: could not be rebuilt: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_compare_as_dpkg_does() {
        let lt = |a: &str, b: &str| assert_eq!(compare(a, b), Ordering::Less, "{a} < {b}");
        lt("1.0", "1.1");
        lt("1.9", "1.10");
        lt("1.0~rc1", "1.0");
        lt("1.0-1", "1.0-1+deb13u1");
        lt("0.15.1-3", "0.15.1-3+deb13u1");
        lt("2.0", "1:1.0");
        lt("1.0a", "1.0b");
        lt("1.0", "1.0a");
        lt("3.0.15-1~deb13u1", "3.0.15-1");
        assert_eq!(compare("1.01", "1.1"), Ordering::Equal);
        assert_eq!(compare("0:1.2-3", "1.2-3"), Ordering::Equal);
    }

    #[test]
    fn only_newer_security_versions_are_refreshed() {
        let policy = "libssl3t64:\n  Installed: 3.5.1-1\n  Candidate: 3.5.4-1~deb13u1\n  Version table:\n \
                      3.5.4-1~deb13u1 500\n        500 http://security.debian.org/debian-security trixie-security/main amd64 Packages\n \
                      *** 3.5.1-1 500\n        500 http://deb.debian.org/debian trixie/main amd64 Packages\n        100 /var/lib/dpkg/status\n";
        assert_eq!(security_versions(policy), ["3.5.4-1~deb13u1"]);
        let carried = vec![
            ("libssl3t64".to_string(), "3.5.1-1".to_string(), "amd64".to_string()),
            ("libfreetype6".into(), "2.13.3+dfsg-1".into(), "amd64".into()),
            ("libpng16-16t64".into(), "1.6.48-1+deb13u1".into(), "amd64".into()),
        ];
        let mut security = HashMap::new();
        security.insert("libssl3t64".to_string(), security_versions(policy));
        security.insert("libfreetype6".to_string(), vec![]);
        security.insert("libpng16-16t64".to_string(), vec!["1.6.48-1+deb13u1".into()]);
        assert_eq!(to_refresh(&carried, &security),
                   vec![Fix { package: "libssl3t64".into(), old: "3.5.1-1".into(), new: "3.5.4-1~deb13u1".into() }]);
    }
}
