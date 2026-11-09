//! Where a downloaded .deb came from. Browsers write the address a file was
//! downloaded from, and the page that led to it, into the file's extended
//! attributes (user.xdg.origin.url and user.xdg.referrer.url). Apps
//! shows the site, and checks it and its parent domains against HaGeZi's
//! Threat Intelligence list (medium), fetched only when a .deb is opened and
//! the copy kept is over a day old. Nothing about the download is sent
//! anywhere: the whole list is fetched and searched here.

use crate::toolkit::t;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

pub const LIST: &str = "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/wildcard/tif.medium-onlydomains.txt";

pub const HOME: &str = "https://github.com/hagezi/dns-blocklists";
pub fn credit() -> &'static str {
    t!("store-origin-credit")
}

const FRESH: Duration = Duration::from_secs(24 * 60 * 60);

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Origin {

    pub site: Option<String>,

    pub referrer: Option<String>,
}

impl Origin {
    pub fn hosts(&self) -> Vec<String> {
        self.site.iter().chain(self.referrer.iter()).cloned().collect()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Verdict {

    Unknown,

    Clear,

    Listed(String),

    NotChecked,
}

pub fn xattr(path: &Path, name: &str) -> Option<String> {
    use std::os::unix::ffi::OsStrExt;
    let p = std::ffi::CString::new(path.as_os_str().as_bytes()).ok()?;
    let n = std::ffi::CString::new(name).ok()?;
    let mut buf = vec![0u8; 4096];
    // SAFETY: both names are NUL-terminated CStrings that live across the

    let got = unsafe { libc::getxattr(p.as_ptr(), n.as_ptr(), buf.as_mut_ptr().cast(), buf.len()) };
    if got <= 0 {
        return None;
    }
    buf.truncate(got as usize);
    String::from_utf8(buf).ok().map(|s| s.trim_end_matches('\0').trim().to_string()).filter(|s| !s.is_empty())
}

pub fn host_of(url: &str) -> Option<String> {
    let (scheme, rest) = url.split_once("://")?;
    if !["http", "https", "ftp"].contains(&scheme.to_ascii_lowercase().as_str()) {
        return None;
    }
    let authority = rest.split(['/', '?', '#']).next()?;
    let authority = authority.rsplit_once('@').map_or(authority, |(_, h)| h);
    let host = if let Some(inner) = authority.strip_prefix('[') {
        inner.split(']').next()?.to_string()
    } else {
        authority.split(':').next()?.to_string()
    };
    let host = host.trim_end_matches('.').to_ascii_lowercase();
    (!host.is_empty() && host.chars().all(|c| c.is_ascii_alphanumeric() || "-.:".contains(c))).then_some(host)
}

pub fn of(path: &Path) -> Origin {
    let site = xattr(path, "user.xdg.origin.url").as_deref().and_then(host_of);
    let referrer = xattr(path, "user.xdg.referrer.url").as_deref().and_then(host_of).filter(|r| Some(r) != site.as_ref());
    Origin { site, referrer }
}

pub fn names(host: &str) -> Vec<String> {
    let labels: Vec<&str> = host.split('.').collect();
    if labels.len() < 2 || host.parse::<std::net::IpAddr>().is_ok() {
        return vec![host.to_string()];
    }
    (0..labels.len() - 1).map(|i| labels[i..].join(".")).collect()
}

pub fn search(list: &Path, hosts: &[String]) -> std::io::Result<Option<String>> {
    let wanted: Vec<String> = hosts.iter().flat_map(|h| names(h)).collect();
    let mut found: Option<String> = None;
    for line in BufReader::new(std::fs::File::open(list)?).lines() {
        let line = line?;
        let d = line.trim().trim_start_matches("*.");
        if d.is_empty() || d.starts_with('#') {
            continue;
        }
        if let Some(w) = wanted.iter().find(|w| w.as_str() == d) {

            if found.as_ref().is_none_or(|f| w.len() > f.len()) {
                found = Some(w.clone());
            }
        }
    }
    Ok(found)
}

pub fn kept() -> PathBuf {
    crate::common::files::cache_home().join("trindle/hagezi-tif-medium.txt")
}

fn stale(list: &Path) -> bool {
    std::fs::metadata(list).and_then(|m| m.modified()).ok()
        .and_then(|t| SystemTime::now().duration_since(t).ok())
        .is_none_or(|age| age > FRESH)
}

fn fetch(list: &Path) -> bool {
    let Some(dir) = list.parent() else { return false };
    if std::fs::create_dir_all(dir).is_err() {
        return false;
    }
    let part = list.with_extension("part");
    let ok = std::process::Command::new("curl")
        .args(["-fsSL", "--proto", "=https", "--proto-redir", "=https", "--max-time", "120", "--max-filesize", crate::dns::list::LARGEST, "-o"])
        .arg(&part).arg(LIST)
        .stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null())
        .status().is_ok_and(|s| s.success());
    if ok && accept(&part, list) && std::fs::rename(&part, list).is_ok() {
        return true;
    }
    let _ = std::fs::remove_file(&part);
    false
}

fn accept(new: &Path, old: &Path) -> bool {
    use crate::dns::list::{shrank, vet};
    let Some(n) = std::fs::read_to_string(new).ok().and_then(|t| vet(&t).ok()) else { return false };
    let o = std::fs::read_to_string(old).ok().and_then(|t| vet(&t).ok()).unwrap_or(0);
    !shrank(o as u64, n as u64)
}

pub fn check_with(origin: &Origin, list: &Path, fetch: impl FnOnce(&Path) -> bool) -> Verdict {
    let hosts = origin.hosts();
    if hosts.is_empty() {
        return Verdict::Unknown;
    }
    if stale(list) {

        fetch(list);
    }
    match search(list, &hosts) {
        Ok(Some(name)) => Verdict::Listed(name),
        Ok(None) => Verdict::Clear,
        Err(_) => Verdict::NotChecked,
    }
}

pub fn check(path: &Path) -> (Origin, Verdict) {
    let origin = of(path);
    let verdict = check_with(&origin, &kept(), fetch);
    (origin, verdict)
}

pub fn words(origin: &Origin, verdict: &Verdict) -> (String, bool) {
    let from = match (&origin.site, &origin.referrer) {
        (Some(s), Some(r)) => t!("store-origin-both", site = s, referrer = r),
        (Some(s), None) => t!("store-origin-site", site = s),
        (None, Some(r)) => t!("store-origin-referrer", referrer = r),
        (None, None) => return (t!("store-origin-unknown").into(), false),
    };
    match verdict {
        Verdict::Listed(name) => (t!("store-origin-listed", from = from, name = name), true),
        Verdict::Clear => (t!("store-origin-clear", from = from), false),
        Verdict::NotChecked => (t!("store-origin-not-checked", from = from), false),
        Verdict::Unknown => (from, false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("trindle-origin-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    fn origin(site: &str) -> Origin {
        Origin { site: host_of(site), referrer: None }
    }

    #[test]
    fn reads_the_host_from_an_address() {
        assert_eq!(host_of("https://Downloads.Example.org:8443/a/b.deb?x=1").as_deref(), Some("downloads.example.org"));
        assert_eq!(host_of("http://user:pw@example.com./x").as_deref(), Some("example.com"));
        assert_eq!(host_of("https://[2001:db8::1]:443/x").as_deref(), Some("2001:db8::1"));
        assert_eq!(host_of("file:///home/john/x.deb"), None);
        assert_eq!(host_of("not an address"), None);
        assert_eq!(names("a.b.example.co.uk"), ["a.b.example.co.uk", "b.example.co.uk", "example.co.uk", "co.uk"]);
        assert_eq!(names("192.0.2.1"), ["192.0.2.1"]);
    }

    #[test]
    fn a_listed_site_or_any_site_under_it_is_warned_of() {
        let d = scratch("listed");
        let list = d.join("list.txt");
        std::fs::write(&list, "# Title: test\nbad-downloads.example\n*.worse.example\n").unwrap();
        let never = |_: &Path| panic!("a fresh list is not fetched again");
        assert_eq!(check_with(&origin("https://cdn.files.bad-downloads.example/x.deb"), &list, never),
                   Verdict::Listed("bad-downloads.example".into()));
        assert_eq!(check_with(&origin("https://worse.example/x.deb"), &list, never), Verdict::Listed("worse.example".into()));

        let o = Origin { site: Some("mirror.example.net".into()), referrer: Some("www.bad-downloads.example".into()) };
        assert_eq!(check_with(&o, &list, never), Verdict::Listed("bad-downloads.example".into()));

        assert_eq!(check_with(&origin("https://notbad-downloads.example/x.deb"), &list, never), Verdict::Clear);
        let (said, warn) = words(&o, &check_with(&o, &list, never));
        assert!(warn && said.contains("bad-downloads.example is on HaGeZi"));
        let (said, warn) = words(&origin("https://example.org/x"), &Verdict::Clear);
        assert!(!warn && said.contains("not complete"));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn a_file_that_does_not_say_is_said_to_be_unknown() {
        let d = scratch("unknown");
        let deb = d.join("x.deb");
        std::fs::write(&deb, b"!<arch>\n").unwrap();
        assert_eq!(of(&deb), Origin::default());
        let fetched = std::cell::Cell::new(false);
        let v = check_with(&of(&deb), &d.join("list.txt"), |_| { fetched.set(true); true });
        assert_eq!(v, Verdict::Unknown);
        assert!(!fetched.get(), "nothing is fetched for a file that names no site");
        assert_eq!(words(&Origin::default(), &v), ("Trindle cannot tell where this came from.".to_string(), false));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn offline_with_no_copy_says_it_could_not_check() {
        let d = scratch("offline");
        let list = d.join("list.txt");
        let o = origin("https://example.org/x.deb");
        assert_eq!(check_with(&o, &list, |_| false), Verdict::NotChecked);
        assert!(words(&o, &Verdict::NotChecked).0.contains("could not be checked"));

        std::fs::write(&list, "example.org\n").unwrap();
        let old = SystemTime::now() - Duration::from_secs(3 * 24 * 60 * 60);
        std::fs::File::options().write(true).open(&list).unwrap().set_modified(old).unwrap();
        let tried = std::cell::Cell::new(false);
        assert_eq!(check_with(&o, &list, |_| { tried.set(true); false }), Verdict::Listed("example.org".into()));
        assert!(tried.get(), "a copy over a day old is fetched again");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn reads_what_the_browser_wrote() {
        let d = scratch("xattr");
        let deb = d.join("x.deb");
        std::fs::write(&deb, b"!<arch>\n").unwrap();
        let set = |name: &str, value: &str| {
            use std::os::unix::ffi::OsStrExt;
            let p = std::ffi::CString::new(deb.as_os_str().as_bytes()).unwrap();
            let n = std::ffi::CString::new(name).unwrap();
            // SAFETY: NUL-terminated names that outlive the call, and a value

            unsafe { libc::setxattr(p.as_ptr(), n.as_ptr(), value.as_ptr().cast(), value.len(), 0) }
        };

        assert_eq!(set("user.xdg.origin.url", "https://dl.example.com/pkg/x.deb"), 0,
                   "{} keeps no user attributes, so there is nothing for this test to read: set TMPDIR to a folder on a disk", d.display());
        set("user.xdg.referrer.url", "https://www.example.org/download");
        assert_eq!(of(&deb), Origin { site: Some("dl.example.com".into()), referrer: Some("www.example.org".into()) });
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn download_is_vetted() {
        let d = scratch("accept");
        let big = |n: usize| (0..n).fold(String::new(), |mut a, i| { a.push_str(&format!("s{i}.example.com\n")); a });
        let (old, new) = (d.join("old"), d.join("new"));
        std::fs::write(&old, big(30_000)).unwrap();
        std::fs::write(&new, big(29_000)).unwrap();
        assert!(accept(&new, &old));
        std::fs::write(&new, big(14_000)).unwrap();
        assert!(!accept(&new, &old));
        std::fs::write(&new, "<html>nope</html>\n").unwrap();
        assert!(!accept(&new, &old));
        std::fs::write(&new, big(20_000)).unwrap();
        assert!(accept(&new, &d.join("none")));
    }
}
