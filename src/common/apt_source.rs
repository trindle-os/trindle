//! Which archive a waiting package comes from, as apt itself names it (`apt-cache policy`).

use std::collections::HashMap;

const LONGEST: usize = 32;

type Archive = (String, Option<String>, Option<String>, Option<String>);

pub fn archives(policy: &str) -> HashMap<String, String> {
    let mut found = HashMap::new();
    let mut at: Option<Archive> = None;
    let close = |at: &mut Option<Archive>, found: &mut HashMap<String, String>| {
        if let Some((key, origin, label, host)) = at.take() {
            let name = [origin, label, host].into_iter().flatten().find_map(|n| clean(&n)).or_else(|| clean(&host_of(&key)));
            found.extend(name.map(|n| (key, n)));
        }
    };
    for line in policy.lines() {
        let words = line.trim();
        if let Some(key) = archive_line(line) {
            close(&mut at, &mut found);
            at = Some((key, None, None, None));
        } else if let Some((_, origin, label, host)) = at.as_mut() {
            if let Some(release) = words.strip_prefix("release ") {
                let field = |k: &str| release.split(',').find_map(|f| f.strip_prefix(k)).map(str::to_string);
                (*origin, *label) = (field("o="), field("l="));
            } else if let Some(h) = words.strip_prefix("origin ") {
                *host = Some(h.to_string());
            }
        }
    }
    close(&mut at, &mut found);
    found
}

pub fn candidates(policy: &str, archives: &HashMap<String, String>) -> HashMap<String, String> {
    let (mut found, mut name, mut want, mut inside) = (HashMap::new(), None::<&str>, None::<&str>, false);
    for line in policy.lines() {
        if !line.starts_with(' ') {
            (name, want, inside) = (line.strip_suffix(':').filter(|n| !n.contains(' ')), None, false);
        } else if let Some(v) = line.trim().strip_prefix("Candidate:") {
            want = Some(v.trim());
        } else if let Some(key) = archive_line(line) {
            if let Some(n) = name.filter(|n| inside && !found.contains_key(*n)) {
                let label = archives.get(&key).cloned().or_else(|| clean(&host_of(&key)));
                found.extend(label.map(|l| (n.to_string(), l)));
            }
        } else {

            let words: Vec<&str> = line.split_whitespace().filter(|w| *w != "***").collect();
            if let [version, priority] = words[..] {
                inside = priority.parse::<i32>().is_ok() && want == Some(version);
            }
        }
    }
    found
}

fn archive_line(line: &str) -> Option<String> {
    let mut words = line.split_whitespace();
    let priority = words.next()?;
    let rest: Vec<&str> = words.collect();
    (priority.parse::<i32>().is_ok() && rest.len() >= 2 && rest[0].contains("://")).then(|| rest.join(" "))
}

fn host_of(text: &str) -> String {
    let address = text.split_whitespace().next().unwrap_or("");
    let after = address.split_once("://").map_or(address, |(_, r)| r);
    let authority = after.split('/').next().unwrap_or("");
    let host = authority.rsplit('@').next().unwrap_or("");
    host.split(':').next().unwrap_or("").to_string()
}

fn clean(text: &str) -> Option<String> {
    let kept: String = text.chars().filter(|c| c.is_alphanumeric() || " .-+_".contains(*c)).collect();
    let name: String = kept.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(LONGEST).collect();
    let name = name.trim().to_string();
    (!name.is_empty()).then_some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: &str = "Package files:\n 100 /var/lib/dpkg/status\n     release a=now\n \
        500 https://downloads.claude.ai/claude-desktop/apt/stable stable/main amd64 Packages\n     release o=Anthropic,a=stable,n=stable,l=Anthropic,c=main,b=amd64\n     origin downloads.claude.ai\n \
        500 http://deb.debian.org/debian trixie/main amd64 Packages\n     release v=13.2,o=Debian,a=stable,n=trixie,l=Debian,c=main,b=amd64\n     origin deb.debian.org\n \
        500 https://user:secret@odd.example.org/repo stable/main amd64 Packages\n     release a=stable\n";
    const EACH: &str = "claude-code:\n  Installed: 2.0\n  Candidate: 2.1\n  Version table:\n     \
        2.1 500\n        500 https://downloads.claude.ai/claude-desktop/apt/stable stable/main amd64 Packages\n \
        *** 2.0 100\n        100 /var/lib/dpkg/status\n\
        bash:\n  Installed: 5.2\n  Candidate: 5.3\n  Version table:\n \
        5.3 500\n        500 http://deb.debian.org/debian trixie/main amd64 Packages\n \
        *** 5.2 100\n        100 /var/lib/dpkg/status\n\
        odd:\n  Installed: 1\n  Candidate: 2\n  Version table:\n     \
        2 500\n        500 https://user:secret@odd.example.org/repo stable/main amd64 Packages\n";

    #[test]
    fn each_package_is_named_by_the_archive_its_candidate_comes_from() {
        let found = candidates(EACH, &archives(ALL));
        assert_eq!(found["claude-code"], "Anthropic");
        assert_eq!(found["bash"], "Debian");
        assert_eq!(found["odd"], "odd.example.org", "no origin or label: the host, never the login");
    }

    #[test]
    fn hostile_names_are_cleaned_and_capped() {
        let all = format!(" 500 https://x.example/a stable/main amd64 Packages\n     release o=Bad\u{1b}[31m<b>{},l=L\n", "z".repeat(100));
        let name = &archives(&all)["https://x.example/a stable/main amd64 Packages"];
        assert!(name.len() <= LONGEST && name.starts_with("Bad31mbzzz") && !name.contains(['<', '[', '\u{1b}']), "{name}");
        assert_eq!(clean("\u{1b}\n"), None);
        assert_eq!(host_of("https://a:b@h.example:8443/p suite"), "h.example");
    }
}
