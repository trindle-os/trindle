//! Blanking of a report: the home folder, the person's names, Wi-Fi names,
//! network and hardware addresses and every email address are taken out of
//! each line before it is shown or kept.

use crate::common::files::home;

#[derive(Clone, Debug, Default)]
pub struct Who {
    pub home: String,

    pub names: Vec<String>,

    pub ssids: Vec<String>,
}

impl Who {
    pub fn here() -> Who {
        let user = std::env::var("USER").or_else(|_| std::env::var("LOGNAME")).unwrap_or_default();
        let mut names = vec![user.clone()];
        let passwd = std::fs::read_to_string("/etc/passwd").unwrap_or_default();

        for line in passwd.lines() {
            let mut f = line.split(':');
            let name = f.next().unwrap_or("");
            let uid = f.nth(1).and_then(|u| u.parse::<u32>().ok()).unwrap_or(0);
            if crate::common::ORDINARY_UIDS.contains(&uid) {
                names.push(name.to_string());
            }
        }
        if let Some(line) = passwd.lines().find(|l| l.split(':').next() == Some(user.as_str())) {
            if let Some(full) = line.split(':').nth(4) {
                names.extend(full.split(',').next().unwrap_or("").split_whitespace().map(str::to_string));
            }
        }
        names.push(std::fs::read_to_string("/etc/hostname").unwrap_or_default().trim().to_string());
        Who { home: home().to_string_lossy().into_owned(), names, ssids: wifi_names() }
    }
}

fn wifi_names() -> Vec<String> {
    let saved = crate::common::run::text(&["nmcli", "-t", "-f", "NAME,TYPE", "connection", "show"]);
    let seen = crate::common::run::text(&["nmcli", "-t", "-f", "SSID", "device", "wifi", "list"]);
    let mut all = Vec::new();
    for l in saved.lines() {
        if let Some((name, kind)) = l.rsplit_once(':') {
            if kind.contains("wireless") {
                all.push(unescape(name));
            }
        }
    }
    all.extend(seen.lines().map(unescape));
    all.retain(|n| n.chars().count() >= SHORTEST);
    all.sort();
    all.dedup();
    all
}

fn unescape(s: &str) -> String {
    s.replace("\\:", ":").replace("\\\\", "\\")
}

const SHORTEST: usize = 3;

pub fn scrub(line: &str, who: &Who) -> String {
    let mut s = emails(line);
    s = addresses(&s);
    s = ssid_values(&s);
    let mut ssids: Vec<&String> = who.ssids.iter().filter(|n| n.chars().count() >= SHORTEST).collect();
    ssids.sort_by_key(|n| std::cmp::Reverse(n.len()));
    for n in ssids {
        s = s.replace(n.as_str(), "[wifi]");
    }
    if who.home.len() > 1 && who.home != "/" {
        s = s.replace(who.home.trim_end_matches('/'), "~");
    }
    s = home_paths(&s);
    for name in &who.names {
        if name.chars().count() >= SHORTEST {
            s = names(&s, name);
        }
    }
    s
}

fn local(c: char) -> bool {
    c.is_ascii_alphanumeric() || "._%+-".contains(c)
}

fn domain(c: char) -> bool {
    c.is_ascii_alphanumeric() || ".-".contains(c)
}

fn emails(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '@' {
            let a = out.chars().rev().take_while(|c| local(*c)).count();
            let mut b = i + 1;
            while b < chars.len() && domain(chars[b]) {
                b += 1;
            }
            while b > i + 1 && chars[b - 1] == '.' {
                b -= 1;
            }
            let host: String = chars[i + 1..b].iter().collect();
            if a > 0 && host.contains('.') && !host.starts_with('.') {
                let keep = out.chars().count() - a;
                out = out.chars().take(keep).collect();
                out.push_str("[email]");
                i = b;
                continue;
            }
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

fn token(c: char) -> bool {
    c.is_ascii_alphanumeric() || ":.%-_".contains(c)
}

fn addresses(s: &str) -> String {
    let mut out = String::new();
    let mut run = String::new();
    for c in s.chars() {
        if token(c) {
            run.push(c);
        } else {
            out.push_str(&one_token(&run));
            run.clear();
            out.push(c);
        }
    }
    out.push_str(&one_token(&run));
    out
}

fn one_token(t: &str) -> String {
    let core = t.trim_end_matches(['.', ':', '-']);
    let tail = &t[core.len()..];
    if is_mac(core) {
        return format!("[mac]{tail}");
    }
    if is_ip(core) {
        return format!("[ip]{tail}");
    }

    if let Some((a, port)) = core.rsplit_once(':') {
        if is_v4(a) && !port.is_empty() && port.chars().all(|c| c.is_ascii_digit()) {
            return format!("[ip]:{port}{tail}");
        }
    }

    if let Some(at) = core.find([':', '=']) {
        let (label, rest) = (&core[..at], &core[at + 1..]);
        if !label.is_empty() && label.chars().all(|c| c.is_ascii_alphabetic()) {
            let inner = one_token(rest);
            if inner != rest {
                return format!("{label}{}{inner}{tail}", &core[at..=at]);
            }
        }
    }
    t.to_string()
}

fn is_v4(t: &str) -> bool {
    let parts: Vec<&str> = t.split('.').collect();
    parts.len() == 4
        && parts.iter().all(|p| !p.is_empty() && p.len() <= 3 && p.chars().all(|c| c.is_ascii_digit()) && p.parse::<u16>().is_ok_and(|n| n <= 255))
}

fn is_ip(t: &str) -> bool {
    let bare = t.split('%').next().unwrap_or(t);
    is_v4(bare) || (bare.contains(':') && bare.parse::<std::net::Ipv6Addr>().is_ok())
}

fn is_mac(t: &str) -> bool {
    [':', '-'].iter().any(|sep| {
        let parts: Vec<&str> = t.split(*sep).collect();
        parts.len() == 6 && parts.iter().all(|p| p.len() == 2 && p.chars().all(|c| c.is_ascii_hexdigit()))
    })
}

fn ssid_values(s: &str) -> String {
    let lower = s.to_ascii_lowercase();
    let mut out = String::new();
    let mut from = 0;
    while let Some(found) = lower[from..].find("ssid") {
        let end = from + found + 4;
        out.push_str(&s[from..end]);
        from = end;
        let rest = &s[end..];
        let after = rest.trim_start_matches(' ');
        let mut skip = rest.len() - after.len();
        let value = match after.chars().next() {
            Some('=' | ':') => {
                let v = after[1..].trim_start_matches(' ');
                skip += after.len() - v.len();
                v
            }
            Some('"' | '\'') => after,
            _ => continue,
        };
        let len = match value.chars().next() {
            Some(q @ ('"' | '\'')) => match value[1..].find(q) {
                Some(close) => {
                    skip += 1;
                    close
                }
                None => continue,
            },
            Some(_) => value.find(|c: char| c.is_whitespace() || c == ',' || c == ';').unwrap_or(value.len()),
            None => continue,
        };
        out.push_str(&s[from..from + skip]);
        if len > 0 {
            out.push_str("[wifi]");
        }
        from += skip + len;
    }
    out.push_str(&s[from..]);
    out
}

fn home_paths(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(at) = rest.find("/home/") {
        let (before, after) = rest.split_at(at + 6);
        out.push_str(before);
        let end = after.find(|c: char| c == '/' || c.is_whitespace() || "\"'`:,;)]}".contains(c)).unwrap_or(after.len());
        if end > 0 {
            out.push_str("[name]");
        }
        rest = &after[end..];
    }
    out.push_str(rest);
    out
}

fn word(c: char) -> bool {
    c.is_alphanumeric()
}

fn names(s: &str, name: &str) -> String {
    let (hay, needle) = (s.to_ascii_lowercase(), name.to_ascii_lowercase());
    let mut out = String::new();
    let mut from = 0;
    while let Some(found) = hay[from..].find(&needle) {
        let (a, b) = (from + found, from + found + needle.len());
        let before = s[..a].chars().next_back().is_some_and(word);
        let after = s[b..].chars().next().is_some_and(word);
        out.push_str(&s[from..a]);
        out.push_str(if before || after { &s[a..b] } else { "[name]" });
        from = b;
    }
    out.push_str(&s[from..]);
    out
}
