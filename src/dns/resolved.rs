//! What systemd-resolved is told while blocking is on behind it.

use super::LOCAL;

pub const TLDS_FILE: &str = "tlds";

const BUILT_IN: &str = "
    xyz com shop net de casa click cn org info top online site cfd ru live pro space digital sbs cyou
    fr cloud cc store vip website lol app qpon eu io br at uk pl in co icu club nl it jp ink rest biz
    lat help me us au ch vn se es asia life cz work id dev buzz ca tv fun quest link fit best be dk za
    hu kr world ai pics one wiki ro ua art tech ir fi cam tr page to cl su blog no sk ph mx win homes
    my beer guru ar today gr bet mom ws garden pt pk forum agency mobi finance il bond hair vu monster
    st autos media ae coupons gp pw africa network bid ng run nz lt is tw ie gg academy ltd cat
    business si lv pe news mk kz center surf sg tattoo support name bg rs hr baby ke ee love casino sh
    nu lc xn--p1ai games skin by study zone services care np host luxe xin th team report chat xxx
    global bio ma la hk cash trade je capital bd im bar yoga rocks company tc solutions stream fyi
    download works studio cm am so onl email beauty group tokyo mw sa ovh living ac vg lu land courses
    ge cx tools li edu boats uno systems money ba sale plus lk fm events ec cool wtf do cv ve uz
    delivery bz bi tz tn exchange zw re ly desi ag aero software";

pub fn most_domains(systemd: u32) -> usize {
    if systemd >= 258 {
        1024
    } else {
        256
    }
}

pub fn systemd_version() -> u32 {
    std::process::Command::new("systemctl").arg("--version").stderr(std::process::Stdio::null()).output().ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|t| t.split_whitespace().nth(1).and_then(|v| v.parse().ok()))
        .unwrap_or(0)
}

fn claimable(tld: &str) -> bool {
    !tld.is_empty() && tld != "arpa" && !LOCAL.contains(&tld)
        && tld.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

pub fn ranked(texts: &[String]) -> String {
    let mut counts: std::collections::HashMap<String, u64> = std::collections::HashMap::new();
    for t in texts {
        for name in super::list::names(t) {
            if let Some((_, tld)) = name.rsplit_once('.') {
                *counts.entry(tld.to_string()).or_default() += 1;
            }
        }
    }
    let mut all: Vec<(String, u64)> = counts.into_iter().filter(|(t, _)| claimable(t)).collect();
    all.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    all.into_iter().map(|(t, _)| t + "\n").collect()
}

pub fn drop_in(counted: &str, most: usize) -> String {
    let mut domains = vec!["~.".to_string()];
    for tld in counted.split_whitespace().chain(BUILT_IN.split_whitespace()) {
        if domains.len() >= most {
            break;
        }
        let d = format!("~{tld}");
        if claimable(tld) && !domains.contains(&d) {
            domains.push(d);
        }
    }
    format!("# Written by trindle-dns while blocking is on in Settings.\n\
             [Resolve]\nDNS=127.0.2.53\n\
             # Every name, and each top-level name blocked sites sit under, so a VPN\n\
             # claiming every name for itself never answers those before trindle-dns.\n\
             Domains={}\n\
             # trindle-dns answers only plain lookups; it makes them private itself.\n\
             DNSOverTLS=no\n", domains.join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chosen<'a>(name: &str, scopes: &[(&'a str, Vec<String>)]) -> Vec<&'a str> {
        let labels = |d: &str| if d == "." { 0 } else { d.split('.').count() };
        let matches = |d: &str| d == "." || name == d || name.ends_with(&format!(".{d}"));
        let best: Vec<(usize, &str)> = scopes.iter().filter_map(|(who, ds)| {
            ds.iter().map(|d| d.trim_start_matches('~')).filter(|d| matches(d)).map(labels).max().map(|n| (n, *who))
        }).collect();
        let top = best.iter().map(|b| b.0).max().unwrap_or(0);
        best.into_iter().filter(|b| b.0 == top).map(|b| b.1).collect()
    }

    fn domains(text: &str) -> Vec<String> {
        text.lines().find_map(|l| l.strip_prefix("Domains=")).unwrap().split(' ').map(String::from).collect()
    }

    #[test]
    fn a_full_tunnel_vpn_never_answers_first() {
        let lists = ["ads.example.xyz\nx.y.xyz\nbad.shop\ntrack.example.com\nprinter.lan\nodd.example.newtld\n".to_string()];
        let counted = ranked(&lists);
        assert_eq!(counted, "xyz\ncom\nnewtld\nshop\n");
        let text = drop_in(&counted, most_domains(257));
        assert!(text.contains("[Resolve]\nDNS=127.0.2.53\n") && text.contains("DNSOverTLS=no\n"));
        let ours = domains(&text);
        assert_eq!(ours.len(), 256);
        assert_eq!(&ours[..5], ["~.", "~xyz", "~com", "~newtld", "~shop"]);
        assert!(!ours.iter().any(|d| ["~lan", "~local", "~arpa", "~internal"].contains(&d.as_str())));
        assert_eq!(domains(&drop_in("", most_domains(258))).len(), 1 + BUILT_IN.split_whitespace().count());
        assert_eq!(BUILT_IN.split_whitespace().count(), 255);

        let scopes = [("trindle-dns", ours), ("wg0", vec!["~.".to_string(), "~corp.example".to_string()]),
                      ("wlan0", vec!["~home.example".to_string()])];
        assert_eq!(chosen("ads.example.xyz", &scopes), ["trindle-dns"]);
        assert_eq!(chosen("odd.example.newtld", &scopes), ["trindle-dns"]);
        assert_eq!(chosen("intranet.corp.example", &scopes), ["wg0"]);
        assert_eq!(chosen("nas.home.example", &scopes), ["wlan0"]);

        assert_eq!(chosen("x.unlisted", &scopes), ["trindle-dns", "wg0"]);
        assert_eq!(most_domains(0), 256);
    }
}
