//! The files a plugin allowed "copied" may be given: exactly the ones the
//! person has just copied, found by Trindle itself and never named by the
//! plugin. This file is the judging: which addresses count, which paths may
//! be opened, how a folder is walked and how often and how long after a
//! copy a plugin may ask. The socket that carries the answer is in
//! src/store/copied_guard.rs. Every limit here is one constant, tried by the
//! tests.

use std::ffi::{CStr, CString, OsStr, OsString};
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

pub const MAX_ENTRIES: usize = 5000;

pub const MAX_DEPTH: usize = 16;

pub const MAX_BYTES: u64 = 64 << 30;

pub const GAP: Duration = Duration::from_secs(2);

pub const RECENT: Duration = Duration::from_secs(120);

pub const LIST_MAX: u64 = 1 << 20;

pub const URI_MAX: usize = 4096;
pub const REL_MAX: usize = 1024;
pub const NAME_MAX: usize = 255;

pub const KINDS: [&str; 2] = ["x-special/gnome-copied-files", "text/uri-list"];

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    pub entries: usize,
    pub depth: usize,
    pub bytes: u64,
}

impl Default for Limits {
    fn default() -> Limits {
        Limits { entries: MAX_ENTRIES, depth: MAX_DEPTH, bytes: MAX_BYTES }
    }
}

pub fn uri_path(uri: &str) -> Result<PathBuf, &'static str> {
    if uri.len() > URI_MAX {
        return Err("too long");
    }
    let rest = match uri.get(..7) {
        Some(head) if head.eq_ignore_ascii_case("file://") => &uri[7..],
        _ => return Err("not the address of a file"),
    };
    let slash = rest.find('/').ok_or("no path")?;
    let (host, path) = rest.split_at(slash);
    if !(host.is_empty() || host.eq_ignore_ascii_case("localhost")) {
        return Err("a file on another computer");
    }
    if path.contains('?') || path.contains('#') {
        return Err("a query or fragment");
    }
    let bytes = percent_decode(path)?;
    let text = String::from_utf8(bytes).map_err(|_| "not UTF-8")?;
    if text.len() > URI_MAX {
        return Err("too long");
    }
    if text.chars().any(|c| c.is_control() || matches!(c, '\u{2028}' | '\u{2029}')) {
        return Err("a control character");
    }
    let inner = text.strip_prefix('/').ok_or("not an absolute path")?;
    let inner = inner.strip_suffix('/').unwrap_or(inner);
    if inner.is_empty() {
        return Err("the whole computer");
    }
    let mut out = PathBuf::from("/");
    for part in inner.split('/') {
        if part.is_empty() {
            return Err("an empty part");
        }
        if part == "." || part == ".." {
            return Err("it climbs");
        }
        if part.len() > NAME_MAX {
            return Err("a name that is too long");
        }
        out.push(part);
    }
    Ok(out)
}

fn percent_decode(s: &str) -> Result<Vec<u8>, &'static str> {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' {
            let hex = |c: u8| (c as char).to_digit(16).map(|d| d as u8);
            match (b.get(i + 1).copied().and_then(hex), b.get(i + 2).copied().and_then(hex)) {
                (Some(h), Some(l)) => out.push(h * 16 + l),
                _ => return Err("a bad percent sign"),
            }
            i += 3;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    Ok(out)
}

pub struct Roots {

    places: Vec<(PathBuf, PathBuf)>,
    home: PathBuf,
}

impl Roots {
    pub fn for_person() -> Roots {
        let home = crate::common::files::home();
        Roots::new(&home, &[Path::new("/media"), Path::new("/run/media"), Path::new("/mnt")])
    }

    pub fn new(home: &Path, others: &[&Path]) -> Roots {
        let real = |p: &Path| std::fs::canonicalize(p).ok().map(|c| (p.to_path_buf(), c));
        let home_real = std::fs::canonicalize(home).unwrap_or_else(|_| home.to_path_buf());
        let places = std::iter::once(home).chain(others.iter().copied()).filter_map(real).collect();
        Roots { places, home: home_real }
    }

    pub fn locate(&self, path: &Path) -> Result<(PathBuf, Vec<OsString>), &'static str> {
        for (spelled, real) in &self.places {
            for base in [spelled, real] {
                let Ok(rest) = path.strip_prefix(base) else { continue };
                let parts: Vec<OsString> = rest.components().map(|c| c.as_os_str().to_owned()).collect();
                if parts.is_empty() {
                    continue;
                }
                if parts.iter().any(|p| p.as_bytes().first() == Some(&b'.')) {
                    return Err("a hidden file or folder");
                }
                let whole = parts.iter().fold(real.clone(), |acc, p| acc.join(p));
                if self.home.starts_with(&whole) {
                    return Err("it holds the home folder");
                }
                return Ok((real.clone(), parts));
            }
        }
        Err("outside your own folders")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Top {

    pub name: String,
    pub root: PathBuf,
    pub rest: Vec<OsString>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Plan {
    pub tops: Vec<Top>,

    pub refused: usize,

    pub cut: bool,
}

pub fn plan(clip: &[(String, Vec<u8>)], roots: &Roots) -> Plan {
    let mut plan = Plan::default();
    let pick = KINDS.iter().find_map(|k| clip.iter().find(|(n, _)| n == k));
    let Some((kind, data)) = pick else { return plan };
    let Ok(text) = std::str::from_utf8(data) else {
        plan.refused = 1;
        return plan;
    };
    let mut lines = text.lines();
    if *kind == KINDS[0] {

        match lines.next().map(str::trim) {
            Some("cut") => plan.cut = true,
            Some("copy") => {}
            _ => {
                plan.refused = 1;
                return plan;
            }
        }
    }
    let mut seen: Vec<PathBuf> = Vec::new();
    for (n, line) in lines.map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).enumerate() {
        if n >= MAX_ENTRIES {
            plan.refused += 1;
            break;
        }
        let located = uri_path(line).and_then(|p| roots.locate(&p).map(|(root, rest)| (p, root, rest)));
        let Ok((path, root, rest)) = located else {
            plan.refused += 1;
            continue;
        };
        if seen.contains(&path) {
            continue;
        }
        seen.push(path);
        let base = rest.last().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
        let name = unique(&base, &plan.tops);
        plan.tops.push(Top { name, root, rest });
    }
    plan
}

fn unique(base: &str, taken: &[Top]) -> String {
    let free = |n: &str| !taken.iter().any(|t| t.name == n);
    if free(base) {
        return base.to_string();
    }
    (2..).map(|i| format!("{base} ({i})")).find(|n| free(n)).unwrap_or_default()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Stop {

    TooBig,

    Gone,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub files: usize,
    pub dirs: usize,
    pub bytes: u64,

    pub skipped: usize,
}

pub enum Entry {
    Dir { rel: String },
    File { rel: String, size: u64, fd: OwnedFd },
}

fn cstr(name: &OsStr) -> io::Result<CString> {
    CString::new(name.as_bytes()).map_err(|_| io::ErrorKind::InvalidInput.into())
}

fn owned(fd: RawFd) -> io::Result<OwnedFd> {
    if fd < 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: the call that made this number has just returned it as a new

    Ok(unsafe { OwnedFd::from_raw_fd(fd) })
}

fn open_at(dir: RawFd, name: &OsStr, flags: i32) -> io::Result<OwnedFd> {
    let c = cstr(name)?;
    // SAFETY: `c` is a NUL-terminated string that lives through the call and

    owned(unsafe { libc::openat(dir, c.as_ptr(), flags | libc::O_CLOEXEC | libc::O_NOFOLLOW | libc::O_NOCTTY) })
}

fn open_folder_at(dir: RawFd, name: &OsStr) -> io::Result<OwnedFd> {
    open_at(dir, name, libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NONBLOCK)
}

fn open_file_at(dir: RawFd, name: &OsStr) -> io::Result<OwnedFd> {

    open_at(dir, name, libc::O_RDONLY | libc::O_NONBLOCK)
}

fn stat_at(dir: RawFd, name: &OsStr) -> io::Result<libc::stat> {
    let c = cstr(name)?;
    let mut st = std::mem::MaybeUninit::<libc::stat>::zeroed();
    // SAFETY: `c` is a NUL-terminated string that lives through the call,

    let r = unsafe { libc::fstatat(dir, c.as_ptr(), st.as_mut_ptr(), libc::AT_SYMLINK_NOFOLLOW) };
    if r != 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: fstatat succeeded, so it filled the struct in.
    Ok(unsafe { st.assume_init() })
}

fn fstat(fd: RawFd) -> io::Result<libc::stat> {
    let mut st = std::mem::MaybeUninit::<libc::stat>::zeroed();
    // SAFETY: `st` has room for one stat and `fd` is held open by the caller.
    if unsafe { libc::fstat(fd, st.as_mut_ptr()) } != 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: fstat succeeded, so it filled the struct in.
    Ok(unsafe { st.assume_init() })
}

fn names_in(fd: &OwnedFd, limit: usize) -> io::Result<Vec<OsString>> {
    // SAFETY: dup only copies a descriptor the caller holds open.
    let copy = unsafe { libc::dup(fd.as_raw_fd()) };
    if copy < 0 {
        return Err(io::Error::last_os_error());
    }
    // SAFETY: `copy` is a folder descriptor made just above; on success the

    let dir = unsafe { libc::fdopendir(copy) };
    if dir.is_null() {
        let e = io::Error::last_os_error();
        // SAFETY: fdopendir failed, so `copy` is still ours to close.
        unsafe { libc::close(copy) };
        return Err(e);
    }
    let mut out = Vec::new();
    // SAFETY: `dir` is the open stream made above, used on this thread only.
    unsafe { libc::rewinddir(dir) };
    loop {
        // SAFETY: `dir` is open; readdir gives null at the end.
        let e = unsafe { libc::readdir(dir) };
        if e.is_null() {
            break;
        }
        // SAFETY: a non-null entry's d_name is a NUL-terminated string that

        let name = unsafe { CStr::from_ptr((*e).d_name.as_ptr()) }.to_bytes();
        if name == b"." || name == b".." {
            continue;
        }
        out.push(OsString::from_vec(name.to_vec()));
        if out.len() >= limit {
            break;
        }
    }
    // SAFETY: `dir` is open and not used again.
    unsafe { libc::closedir(dir) };
    out.sort();
    Ok(out)
}

fn plain_name(name: &OsStr) -> Option<&str> {
    let s = std::str::from_utf8(name.as_bytes()).ok()?;
    let bad = s.is_empty() || s.len() > NAME_MAX || s == "." || s == ".."
        || s.chars().any(|c| c.is_control() || matches!(c, '\u{2028}' | '\u{2029}'));
    (!bad).then_some(s)
}

struct Walk<'a> {
    limits: &'a Limits,
    tally: Tally,
    entries: usize,
    sink: &'a mut dyn FnMut(Entry) -> Result<(), Stop>,
}

impl Walk<'_> {
    fn skip(&mut self) {
        self.tally.skipped += 1;
    }

    fn entry(&mut self, dir: RawFd, name: &OsStr, rel: String, depth: usize, dev: Option<u64>) -> Result<(), Stop> {
        if rel.len() > REL_MAX {
            self.skip();
            return Ok(());
        }
        self.entries += 1;
        if self.entries > self.limits.entries {
            return Err(Stop::TooBig);
        }
        let Ok(st) = stat_at(dir, name) else {
            self.skip();
            return Ok(());
        };
        let same_disk = dev.is_none_or(|d| d == st.st_dev);
        match st.st_mode & libc::S_IFMT {
            libc::S_IFREG if same_disk => self.regular(dir, name, rel),
            libc::S_IFDIR if same_disk && depth <= self.limits.depth => self.folder(dir, name, rel, depth, dev),
            _ => {
                self.skip();
                Ok(())
            }
        }
    }

    fn regular(&mut self, dir: RawFd, name: &OsStr, rel: String) -> Result<(), Stop> {
        let Ok(fd) = open_file_at(dir, name) else {
            self.skip();
            return Ok(());
        };

        let Ok(now) = fstat(fd.as_raw_fd()) else {
            self.skip();
            return Ok(());
        };
        if now.st_mode & libc::S_IFMT != libc::S_IFREG {
            self.skip();
            return Ok(());
        }
        let size = now.st_size.max(0) as u64;
        self.tally.bytes = self.tally.bytes.saturating_add(size);
        if self.tally.bytes > self.limits.bytes {
            return Err(Stop::TooBig);
        }
        self.tally.files += 1;
        (self.sink)(Entry::File { rel, size, fd })
    }

    fn folder(&mut self, dir: RawFd, name: &OsStr, rel: String, depth: usize, dev: Option<u64>) -> Result<(), Stop> {
        let Ok(fd) = open_folder_at(dir, name) else {
            self.skip();
            return Ok(());
        };
        let Ok(now) = fstat(fd.as_raw_fd()) else {
            self.skip();
            return Ok(());
        };
        if now.st_mode & libc::S_IFMT != libc::S_IFDIR || dev.is_some_and(|d| d != now.st_dev) {
            self.skip();
            return Ok(());
        }
        self.tally.dirs += 1;
        (self.sink)(Entry::Dir { rel: rel.clone() })?;
        let room = self.limits.entries.saturating_sub(self.entries) + 1;
        let Ok(names) = names_in(&fd, room) else {
            self.skip();
            return Ok(());
        };
        for n in names {
            match plain_name(&n) {
                Some(s) => {
                    let child = format!("{rel}/{s}");
                    self.entry(fd.as_raw_fd(), &n, child, depth + 1, Some(now.st_dev))?;
                }
                None => {

                    self.entries += 1;
                    if self.entries > self.limits.entries {
                        return Err(Stop::TooBig);
                    }
                    self.skip();
                }
            }
        }
        Ok(())
    }
}

pub fn gather(plan: &Plan, limits: &Limits, sink: &mut dyn FnMut(Entry) -> Result<(), Stop>) -> Result<Tally, Stop> {
    let mut walk = Walk { limits, tally: Tally::default(), entries: 0, sink };
    for top in &plan.tops {
        let Some((last, above)) = top.rest.split_last() else { continue };
        let Ok(mut dir) = open_at(libc::AT_FDCWD, top.root.as_os_str(), libc::O_RDONLY | libc::O_DIRECTORY) else {
            walk.skip();
            continue;
        };
        let mut lost = false;
        for part in above {
            match open_folder_at(dir.as_raw_fd(), part) {
                Ok(next) => dir = next,
                Err(_) => {
                    lost = true;
                    break;
                }
            }
        }
        if lost {
            walk.skip();
            continue;
        }
        walk.entry(dir.as_raw_fd(), last, top.name.clone(), 0, None)?;
    }
    Ok(walk.tally)
}

#[derive(Default)]
pub struct Gate {
    last: Option<Instant>,
}

impl Gate {
    pub fn new() -> Gate {
        Gate::default()
    }

    #[cfg(test)]
    pub fn admit(&mut self, now: Instant, copied_at: Option<Instant>) -> Result<(), &'static str> {
        self.rate(now)?;
        Gate::recent(now, copied_at)
    }

    pub fn rate(&mut self, now: Instant) -> Result<(), &'static str> {
        if self.last.is_some_and(|l| now.saturating_duration_since(l) < GAP) {
            return Err("slow_down");
        }
        self.last = Some(now);
        Ok(())
    }

    pub fn recent(now: Instant, copied_at: Option<Instant>) -> Result<(), &'static str> {
        match copied_at {
            Some(at) if now.saturating_duration_since(at) <= RECENT => Ok(()),
            _ => Err("too_old"),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Write as _;
    use super::*;
    use std::os::unix::fs::{symlink, PermissionsExt};

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-copied-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::canonicalize(&dir).unwrap()
    }

    fn uri(p: &Path) -> String {
        format!("file://{}", p.display())
    }

    fn roots_for(home: &Path) -> Roots {
        Roots::new(home, &[])
    }

    fn found(plan: &Plan, limits: &Limits) -> (Result<Tally, Stop>, Vec<(String, Option<u64>)>) {
        let mut all = Vec::new();
        let mut sink = |e: Entry| {
            match e {
                Entry::Dir { rel } => all.push((rel, None)),
                Entry::File { rel, size, .. } => all.push((rel, Some(size))),
            }
            Ok(())
        };
        let t = gather(plan, limits, &mut sink);
        (t, all)
    }

    #[test]
    fn plain_addresses_are_read() {
        assert_eq!(uri_path("file:///home/a/b.txt").unwrap(), PathBuf::from("/home/a/b.txt"));
        assert_eq!(uri_path("file:///home/a/b%20c.txt").unwrap(), PathBuf::from("/home/a/b c.txt"));
        assert_eq!(uri_path("file://localhost/home/a").unwrap(), PathBuf::from("/home/a"));
        assert_eq!(uri_path("FILE:///home/a/").unwrap(), PathBuf::from("/home/a"));
        assert_eq!(uri_path("file:///home/a/%C3%A9.txt").unwrap(), PathBuf::from("/home/a/\u{e9}.txt"));
        assert_eq!(uri_path("file:///home/a/caf\u{e9}").unwrap(), PathBuf::from("/home/a/caf\u{e9}"));
    }

    #[test]
    fn hostile_addresses_are_refused() {
        let refused = [

            "http://example.com/a", "ftp:///a", "/home/a/b", "home/a", "file:home/a", "file:/home/a", "",
            "file://host/home/a", "file://192.168.1.2/share/a", "file://localhost.evil/home/a", "file://",
            "file:///home/a?x=1", "file:///home/a#frag",

            "file:///home/a/../b", "file:///home/a/%2e%2e/b", "file:///home/a/%2E%2e/b", "file:///home/a/./b",
            "file:///home/a/%2e/b", "file:///home/a/..", "file:///../etc/passwd", "file:///home/a/%2e%2E",
            "file:///home/a/..%2fb", "file:///home/a%2f..%2fb",

            "file:///", "file:////", "file:///home//a", "file:///home/a//", "file:///%2f",

            "file:///home/a/b%00c", "file:///home/a/b%0Ac", "file:///home/a/b%0d%0ac", "file:///home/a/b%09c",
            "file:///home/a/b%1bc", "file:///home/a/b%7fc", "file:///home/a/b\nc", "file:///home/a/b\tc",
            "file:///home/a/b%c2%85c", "file:///home/a/b%e2%80%a8c",

            "file:///home/a/%ff%fe", "file:///home/a/%c3", "file:///home/a/%", "file:///home/a/%g0", "file:///home/a/%0",
        ];
        for r in refused {
            assert!(uri_path(r).is_err(), "{r:?} was taken as {:?}", uri_path(r));
        }

        assert!(uri_path(&format!("file:///{}", "a/".repeat(3000))).is_err());
        assert!(uri_path(&format!("file:///home/{}", "a".repeat(256))).is_err());
        assert!(uri_path(&format!("file:///home/{}", "%41".repeat(2000))).is_err());
        assert!(uri_path(&format!("file:///home/{}", "a".repeat(255))).is_ok());
    }

    #[test]
    fn only_the_persons_own_visible_places_are_let_in() {
        let home = scratch("roots");
        let media = scratch("media");
        std::fs::create_dir_all(home.join("Documents")).unwrap();
        std::fs::create_dir_all(home.join(".ssh")).unwrap();
        let roots = Roots::new(&home, &[&media]);
        let ok = |p: PathBuf| roots.locate(&p);
        assert!(ok(home.join("Documents/a.txt")).is_ok());
        assert!(ok(media.join("disk/a.txt")).is_ok());
        let (root, parts) = ok(home.join("Documents/a.txt")).unwrap();
        assert_eq!((root, parts), (home.clone(), vec![OsString::from("Documents"), OsString::from("a.txt")]));

        assert_eq!(ok(home.join(".ssh/id_ed25519")), Err("a hidden file or folder"));
        assert_eq!(ok(home.join("Documents/.hidden")), Err("a hidden file or folder"));
        assert_eq!(ok(home.join(".local/share/trindle/plugins")), Err("a hidden file or folder"));
        assert!(ok(home.clone()).is_err());
        assert!(ok(home.parent().unwrap().to_path_buf()).is_err());
        for p in ["/etc/shadow", "/proc/self/environ", "/dev/mem", "/sys/kernel", "/usr/bin/sudo", "/root/x", "/tmp/x"] {
            assert_eq!(ok(PathBuf::from(p)), Err("outside your own folders"), "{p}");
        }

        std::fs::create_dir_all(media.join("home/me")).unwrap();
        let inner = Roots::new(&media.join("home/me"), &[&media]);
        assert_eq!(inner.locate(&media.join("home/me")), Err("it holds the home folder"));
        assert_eq!(inner.locate(&media.join("home")), Err("it holds the home folder"));
        assert!(inner.locate(&media.join("home/me/Documents")).is_ok());
        let _ = std::fs::remove_dir_all(&home);
        let _ = std::fs::remove_dir_all(&media);
    }

    #[test]
    fn a_home_that_is_a_link_still_works() {
        let real = scratch("realhome");
        let link = std::env::temp_dir().join(format!("trindle-copied-link-{}", std::process::id()));
        let _ = std::fs::remove_file(&link);
        symlink(&real, &link).unwrap();
        std::fs::write(real.join("a.txt"), b"x").unwrap();
        let roots = Roots::new(&link, &[]);

        for p in [link.join("a.txt"), real.join("a.txt")] {
            let (root, parts) = roots.locate(&p).unwrap();
            assert_eq!(root, real);
            assert_eq!(parts, vec![OsString::from("a.txt")]);
        }
        let _ = std::fs::remove_file(&link);
        let _ = std::fs::remove_dir_all(&real);
    }

    fn clip(kind: &str, text: &str) -> Vec<(String, Vec<u8>)> {
        vec![(kind.to_string(), text.as_bytes().to_vec())]
    }

    #[test]
    fn the_clipboard_list_is_judged_line_by_line() {
        let home = scratch("plan");
        std::fs::write(home.join("a.txt"), b"x").unwrap();
        std::fs::create_dir_all(home.join("Folder")).unwrap();
        let roots = roots_for(&home);
        let gnome = format!("copy\n{}\n{}\nfile:///etc/shadow\nfile://host/x", uri(&home.join("a.txt")), uri(&home.join("Folder")));
        let p = plan(&clip(KINDS[0], &gnome), &roots);
        assert_eq!(p.tops.iter().map(|t| t.name.as_str()).collect::<Vec<_>>(), ["a.txt", "Folder"]);
        assert_eq!((p.refused, p.cut), (2, false));
        let cut = plan(&clip(KINDS[0], &format!("cut\n{}", uri(&home.join("a.txt")))), &roots);
        assert!(cut.cut && cut.tops.len() == 1);

        let list = plan(&clip(KINDS[1], &format!("# a note\r\n{}\r\n\r\n", uri(&home.join("a.txt")))), &roots);
        assert_eq!((list.tops.len(), list.refused), (1, 0));

        assert!(plan(&clip(KINDS[0], &uri(&home.join("a.txt"))), &roots).tops.is_empty());
        assert!(plan(&clip("text/plain", &uri(&home.join("a.txt"))), &roots).tops.is_empty());
        assert!(plan(&[], &roots).tops.is_empty());

        let bad = vec![(KINDS[1].to_string(), vec![0xff, 0xfe, b'\n'])];
        assert_eq!(plan(&bad, &roots), Plan { tops: vec![], refused: 1, cut: false });

        std::fs::create_dir_all(home.join("B")).unwrap();
        std::fs::write(home.join("B/a.txt"), b"y").unwrap();
        let twice = format!("{}\n{}\n{}", uri(&home.join("a.txt")), uri(&home.join("a.txt")), uri(&home.join("B/a.txt")));
        let p = plan(&clip(KINDS[1], &twice), &roots);
        assert_eq!(p.tops.iter().map(|t| t.name.as_str()).collect::<Vec<_>>(), ["a.txt", "a.txt (2)"]);
        // Thousands of lines are read to a limit.
        let many: String = (0..MAX_ENTRIES + 50).fold(String::new(), |mut s, i| { let _ = writeln!(s, "{}", uri(&home.join(format!("n{i}")))); s });
        let p = plan(&clip(KINDS[1], &many), &roots);
        assert_eq!(p.tops.len(), MAX_ENTRIES);
        assert!(p.refused >= 1);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn a_file_and_a_folder_are_walked_with_their_sizes() {
        let home = scratch("walk");
        std::fs::write(home.join("a.txt"), b"hello").unwrap();
        std::fs::create_dir_all(home.join("Folder/sub")).unwrap();
        std::fs::write(home.join("Folder/one"), b"12").unwrap();
        std::fs::write(home.join("Folder/sub/two"), b"123").unwrap();
        std::fs::write(home.join("Folder/.hidden"), b"1234").unwrap();
        std::fs::create_dir_all(home.join("Folder/empty")).unwrap();
        let roots = roots_for(&home);
        let text = format!("{}\n{}", uri(&home.join("a.txt")), uri(&home.join("Folder")));
        let p = plan(&clip(KINDS[1], &text), &roots);
        let (t, all) = found(&p, &Limits::default());
        let t = t.unwrap();
        assert_eq!((t.files, t.dirs, t.bytes, t.skipped), (4, 3, 14, 0));
        assert_eq!(all, vec![
            ("a.txt".to_string(), Some(5)),
            ("Folder".to_string(), None),
            ("Folder/.hidden".to_string(), Some(4)),
            ("Folder/empty".to_string(), None),
            ("Folder/one".to_string(), Some(2)),
            ("Folder/sub".to_string(), None),
            ("Folder/sub/two".to_string(), Some(3)),
        ]);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn links_are_never_followed() {
        let home = scratch("links");
        let outside = scratch("outside");
        std::fs::write(outside.join("secret"), b"top secret").unwrap();
        std::fs::create_dir_all(home.join("Folder")).unwrap();
        std::fs::write(home.join("Folder/real"), b"ok").unwrap();

        symlink(outside.join("secret"), home.join("Folder/to-file")).unwrap();
        symlink(&outside, home.join("Folder/to-dir")).unwrap();
        symlink(&home, home.join("Folder/loop")).unwrap();
        symlink("real", home.join("Folder/relative")).unwrap();
        symlink("/etc/shadow", home.join("link.txt")).unwrap();
        symlink(&outside, home.join("Link")).unwrap();
        let roots = roots_for(&home);

        let (t, all) = found(&plan(&clip(KINDS[1], &uri(&home.join("Folder"))), &roots), &Limits::default());
        assert_eq!(all.iter().map(|(r, _)| r.as_str()).collect::<Vec<_>>(), ["Folder", "Folder/real"]);
        assert_eq!(t.unwrap().skipped, 4);

        for p in ["link.txt", "Link"] {
            let (t, all) = found(&plan(&clip(KINDS[1], &uri(&home.join(p))), &roots), &Limits::default());
            assert!(all.is_empty(), "{p}: {all:?}");
            assert_eq!(t.unwrap().skipped, 1);
        }

        let through = plan(&clip(KINDS[1], &uri(&home.join("Link/secret"))), &roots);
        let (t, all) = found(&through, &Limits::default());
        assert!(all.is_empty());
        assert_eq!(t.unwrap().skipped, 1);
        let _ = std::fs::remove_dir_all(&home);
        let _ = std::fs::remove_dir_all(&outside);
    }

    #[test]
    fn pipes_and_devices_and_sockets_are_left_out() {
        let home = scratch("special");
        std::fs::create_dir_all(home.join("Folder")).unwrap();
        std::fs::write(home.join("Folder/real"), b"ok").unwrap();
        let fifo = CString::new(home.join("Folder/pipe").as_os_str().as_bytes()).unwrap();
        // SAFETY: a NUL-terminated path that lives through the call.
        assert_eq!(unsafe { libc::mkfifo(fifo.as_ptr(), 0o600) }, 0);
        let sock = std::os::unix::net::UnixListener::bind(home.join("Folder/sock")).unwrap();
        let roots = roots_for(&home);
        let (t, all) = found(&plan(&clip(KINDS[1], &uri(&home.join("Folder"))), &roots), &Limits::default());
        assert_eq!(all.iter().map(|(r, _)| r.as_str()).collect::<Vec<_>>(), ["Folder", "Folder/real"]);
        assert_eq!(t.unwrap().skipped, 2);

        let (t, all) = found(&plan(&clip(KINDS[1], &uri(&home.join("Folder/pipe"))), &roots), &Limits::default());
        assert!(all.is_empty());
        assert_eq!(t.unwrap().skipped, 1);

        let p = Plan { tops: vec![Top { name: "null".into(), root: PathBuf::from("/dev"), rest: vec!["null".into()] }], ..Plan::default() };
        let (t, all) = found(&p, &Limits::default());
        assert!(all.is_empty());
        assert_eq!(t.unwrap().skipped, 1);
        drop(sock);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn the_walk_stops_at_its_limits() {
        let home = scratch("limits");
        let deep = (0..20).fold(home.join("Deep"), |p, i| p.join(format!("d{i}")));
        std::fs::create_dir_all(&deep).unwrap();
        std::fs::write(deep.join("far"), b"x").unwrap();
        std::fs::create_dir_all(home.join("Wide")).unwrap();
        for i in 0..30 {
            std::fs::write(home.join(format!("Wide/f{i:02}")), vec![b'x'; 100]).unwrap();
        }
        let roots = roots_for(&home);

        let (t, all) = found(&plan(&clip(KINDS[1], &uri(&home.join("Deep"))), &roots), &Limits::default());
        let t = t.unwrap();
        assert_eq!(all.len(), 1 + MAX_DEPTH, "{all:?}");
        assert!(all.iter().all(|(_, size)| size.is_none()), "no file is as deep as the twentieth folder");
        assert_eq!(t.skipped, 1);

        let wide = plan(&clip(KINDS[1], &uri(&home.join("Wide"))), &roots);
        assert!(found(&wide, &Limits { entries: 31, ..Limits::default() }).0.is_ok());
        assert_eq!(found(&wide, &Limits { entries: 30, ..Limits::default() }).0.unwrap_err(), Stop::TooBig);

        assert!(found(&wide, &Limits { bytes: 3000, ..Limits::default() }).0.is_ok());
        assert_eq!(found(&wide, &Limits { bytes: 2999, ..Limits::default() }).0.unwrap_err(), Stop::TooBig);

        let d = Limits::default();
        assert_eq!((d.entries, d.depth, d.bytes), (5000, 16, 64 * 1024 * 1024 * 1024));
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn a_huge_folder_is_not_read_to_its_end() {
        let home = scratch("huge");
        std::fs::create_dir_all(home.join("Many")).unwrap();
        for i in 0..400 {
            std::fs::write(home.join(format!("Many/n{i}")), b"").unwrap();
        }
        let roots = roots_for(&home);
        let p = plan(&clip(KINDS[1], &uri(&home.join("Many"))), &roots);
        assert_eq!(found(&p, &Limits { entries: 50, ..Limits::default() }).0.unwrap_err(), Stop::TooBig);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn odd_names_inside_a_folder_are_left_out() {
        let home = scratch("names");
        std::fs::create_dir_all(home.join("Folder")).unwrap();
        std::fs::write(home.join("Folder/fine"), b"x").unwrap();
        std::fs::write(home.join("Folder/new\nline"), b"x").unwrap();
        std::fs::write(home.join("Folder/tab\there"), b"x").unwrap();
        std::fs::write(home.join(OsStr::from_bytes(b"Folder/bad\xffutf8")), b"x").unwrap();
        let roots = roots_for(&home);
        let (t, all) = found(&plan(&clip(KINDS[1], &uri(&home.join("Folder"))), &roots), &Limits::default());
        assert_eq!(all.iter().map(|(r, _)| r.as_str()).collect::<Vec<_>>(), ["Folder", "Folder/fine"]);
        assert_eq!(t.unwrap().skipped, 3);
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn a_file_swapped_for_a_link_after_the_list_was_made_is_not_followed() {
        let home = scratch("swap");
        let outside = scratch("swapout");
        std::fs::write(outside.join("secret"), b"top secret").unwrap();
        std::fs::write(home.join("a.txt"), b"mine").unwrap();
        let roots = roots_for(&home);
        let p = plan(&clip(KINDS[1], &uri(&home.join("a.txt"))), &roots);
        std::fs::remove_file(home.join("a.txt")).unwrap();
        symlink(outside.join("secret"), home.join("a.txt")).unwrap();
        let (t, all) = found(&p, &Limits::default());
        assert!(all.is_empty());
        assert_eq!(t.unwrap().skipped, 1);
        let _ = std::fs::remove_dir_all(&home);
        let _ = std::fs::remove_dir_all(&outside);
    }

    #[test]
    fn a_file_that_cannot_be_read_is_left_out_not_failed() {
        let home = scratch("unreadable");
        std::fs::write(home.join("locked"), b"x").unwrap();
        std::fs::set_permissions(home.join("locked"), std::fs::Permissions::from_mode(0o000)).unwrap();
        let roots = roots_for(&home);
        let (t, all) = found(&plan(&clip(KINDS[1], &uri(&home.join("locked"))), &roots), &Limits::default());

        // SAFETY: geteuid only reads this process's own user number.
        if unsafe { libc::geteuid() } != 0 {
            assert!(all.is_empty());
            assert_eq!(t.unwrap().skipped, 1);
        }
        let _ = std::fs::remove_dir_all(&home);
    }

    #[test]
    fn the_rate_limit_and_the_recent_copy_window() {
        let t0 = Instant::now();
        let mut gate = Gate::new();
        let copied = t0;
        assert_eq!(gate.admit(t0 + Duration::from_millis(500), Some(copied)), Ok(()));

        assert_eq!(gate.admit(t0 + Duration::from_millis(1500), Some(copied)), Err("slow_down"));
        assert_eq!(gate.admit(t0 + Duration::from_millis(2400), Some(copied)), Err("slow_down"));
        assert_eq!(gate.admit(t0 + Duration::from_millis(2500), Some(copied)), Ok(()));

        let mut let_in = 0;
        let mut gate = Gate::new();
        for ms in (0..20_000).step_by(100) {
            if gate.admit(t0 + Duration::from_millis(ms), Some(copied)).is_ok() {
                let_in += 1;
            }
        }
        assert_eq!(let_in, 10);

        let mut gate = Gate::new();
        assert_eq!(gate.admit(copied + Duration::from_secs(120), Some(copied)), Ok(()));
        let mut gate = Gate::new();
        assert_eq!(gate.admit(copied + Duration::from_millis(120_001), Some(copied)), Err("too_old"));

        let mut gate = Gate::new();
        assert_eq!(gate.admit(copied + Duration::from_secs(500), Some(copied + Duration::from_secs(450))), Ok(()));
        assert_eq!(Gate::new().admit(copied, None), Err("too_old"));

        assert_eq!(Gate::new().admit(copied, Some(copied + Duration::from_secs(5))), Ok(()));
    }
}
