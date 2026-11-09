//! The lists of advert and bad sites, from HaGeZi. Kept on disk as sorted
//! 64-bit hashes of the names and mapped into memory, so nearly a million
//! sites cost a few megabytes of disk cache rather than of the program's own
//! memory. A site on the list is blocked with every name under it.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub const SOURCES: [&str; 2] = [
    "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/wildcard/multi-onlydomains.txt",
    "https://cdn.jsdelivr.net/gh/hagezi/dns-blocklists@latest/wildcard/tif.medium-onlydomains.txt",
];
const MAGIC: &[u8; 8] = b"TRDNSL1\0";
const HEAD: usize = 24;
pub const FILE: &str = "lists.bin";

pub fn folder() -> PathBuf {
    std::env::var_os("CACHE_DIRECTORY").map_or_else(|| PathBuf::from("/var/cache/trindle-dns"), PathBuf::from)
}

pub fn hash(name: &str) -> u64 {
    name.bytes().fold(0xcbf2_9ce4_8422_2325u64, |h, b| (h ^ b as u64).wrapping_mul(0x0100_0000_01b3))
}

pub fn names(text: &str) -> impl Iterator<Item = String> + '_ {
    text.lines().filter_map(|l| {
        let l = l.split('#').next().unwrap_or("").trim();
        if l.is_empty() {
            None
        } else {
            crate::common::dns::site(l)
        }
    })
}

pub const FLOOR: usize = 10_000;
pub const CEILING: usize = 5_000_000;

pub const LARGEST: &str = "100M";

pub fn vet(text: &str) -> Result<usize, String> {
    let lines = text.lines().filter(|l| !l.split('#').next().unwrap_or("").trim().is_empty()).count();
    let valid = names(text).count();
    if lines - valid > lines / 100 {
        return Err("not plain domain lines".into());
    }
    if !(FLOOR..=CEILING).contains(&valid) {
        return Err(format!("{valid} names is not believable"));
    }
    Ok(valid)
}

pub fn shrank(old: u64, new: u64) -> bool {
    new.saturating_mul(2) < old
}

pub fn build(texts: &[String], fetched: u64) -> Vec<u8> {
    let mut all: Vec<u64> = texts.iter().flat_map(|t| names(t).map(|n| hash(&n))).collect();
    all.sort_unstable();
    all.dedup();
    let mut out = Vec::with_capacity(HEAD + all.len() * 8);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&(all.len() as u64).to_le_bytes());
    out.extend_from_slice(&fetched.to_le_bytes());
    for h in all {
        out.extend_from_slice(&h.to_le_bytes());
    }
    out
}

pub struct List {
    at: *const u8,
    len: usize,
    pub count: u64,
    pub fetched: u64,
}

// SAFETY: the mapping is read-only and never changes while it is held; the

unsafe impl Send for List {}
// SAFETY: as above; reads alone are made, from any thread.
unsafe impl Sync for List {}

impl List {
    pub fn open(path: &Path) -> Option<List> {
        use std::os::fd::AsRawFd;
        let file = std::fs::File::open(path).ok()?;
        let len = file.metadata().ok()?.len() as usize;
        if len < HEAD {
            return None;
        }
        // SAFETY: a fresh read-only private mapping of a file we opened; it is

        let at = unsafe { libc::mmap(std::ptr::null_mut(), len, libc::PROT_READ, libc::MAP_PRIVATE, file.as_raw_fd(), 0) };
        if at == libc::MAP_FAILED {
            return None;
        }
        let mut list = List { at: at as *const u8, len, count: 0, fetched: 0 };
        let head = list.bytes();
        let word = |i: usize| u64::from_le_bytes(head[i..i + 8].try_into().unwrap_or([0; 8]));
        let (magic, count, fetched) = (&head[..8] == MAGIC, word(8), word(16));
        if !magic || (len - HEAD) as u64 != count.saturating_mul(8) {
            return None;
        }
        list.count = count;
        list.fetched = fetched;
        Some(list)
    }

    fn bytes(&self) -> &[u8] {
        // SAFETY: `at` is a live mapping of `len` bytes until Drop.
        unsafe { std::slice::from_raw_parts(self.at, self.len) }
    }

    fn get(&self, i: usize) -> u64 {
        let at = HEAD + i * 8;
        u64::from_le_bytes(self.bytes()[at..at + 8].try_into().unwrap_or([0; 8]))
    }

    fn has(&self, h: u64) -> bool {
        let (mut lo, mut hi) = (0usize, self.count as usize);
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match self.get(mid).cmp(&h) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => lo = mid + 1,
                std::cmp::Ordering::Greater => hi = mid,
            }
        }
        false
    }

    pub fn blocks(&self, name: &str) -> bool {
        under(name).any(|n| self.has(hash(n)))
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // SAFETY: unmaps exactly the mapping made in open, once.
        unsafe { libc::munmap(self.at as *mut libc::c_void, self.len) };
    }
}

pub fn under(name: &str) -> impl Iterator<Item = &str> {
    let mut next = Some(name);
    std::iter::from_fn(move || {
        let n = next?;
        next = n.split_once('.').map(|(_, rest)| rest).filter(|r| r.contains('.'));
        n.contains('.').then_some(n)
    })
}

pub fn refresh(dir: &Path) -> Result<u64, String> {
    let mut texts = Vec::new();
    for (i, url) in SOURCES.iter().enumerate() {
        let part = dir.join(format!("{i}.part"));
        let ok = Command::new("curl")
            .args(["-fsSL", "--proto", "=https", "--proto-redir", "=https", "--max-time", "300", "--max-filesize", LARGEST, "--retry", "2", "-o"])
            .arg(&part).arg(url).stdout(Stdio::null()).stderr(Stdio::null()).status().is_ok_and(|s| s.success());
        let text = if ok { std::fs::read_to_string(&part).ok() } else { None };
        let _ = std::fs::remove_file(&part);
        let text = text.ok_or_else(|| format!("{url} could not be fetched"))?;
        vet(&text).map_err(|e| format!("{url}: {e}"))?;
        texts.push(text);
    }
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    let data = build(&texts, now);

    let tlds = super::resolved::ranked(&texts);
    drop(texts);
    let count = (data.len() - HEAD) as u64 / 8;
    if let Some(old) = List::open(&dir.join(FILE)) {
        if shrank(old.count, count) {
            return Err(format!("the lists shrank from {} to {count} sites", old.count));
        }
    }
    let new = dir.join(format!("{FILE}.new"));
    std::fs::write(&new, &data).map_err(|e| e.to_string())?;
    std::fs::rename(&new, dir.join(FILE)).map_err(|e| e.to_string())?;
    let new = dir.join(format!("{}.new", super::resolved::TLDS_FILE));
    if std::fs::write(&new, tlds).is_ok() {
        let _ = std::fs::rename(&new, dir.join(super::resolved::TLDS_FILE));
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_and_parents() {
        let text = "# Title: HaGeZi\n# comment\nads.example.com\nTrack.Example.org # note\n\nnot valid!\n";
        assert_eq!(names(text).collect::<Vec<_>>(), ["ads.example.com", "track.example.org"]);
        assert_eq!(under("a.b.example.com").collect::<Vec<_>>(), ["a.b.example.com", "b.example.com", "example.com"]);
        assert_eq!(under("com").count(), 0);
        assert_eq!(under("").count(), 0);
    }

    #[test]
    fn vetting() {
        let good: String = (0..FLOOR).fold(String::new(), |mut a, i| { a.push_str(&format!("site{i}.example.com\n")); a });
        assert_eq!(vet(&format!("# Title\n{good}")), Ok(FLOOR));
        assert!(vet("ads.example.com\n").is_err());
        assert!(vet(&format!("{good}<html>\n")).is_ok());
        let html: String = (0..FLOOR).map(|_| "<div class=x>\n").collect();
        assert!(vet(&format!("{html}{good}")).is_err());
        assert!(shrank(1000, 499) && !shrank(1000, 500) && !shrank(0, 0));
    }

    #[test]
    fn lists_on_disk() {
        let dir = std::env::temp_dir().join(format!("trindle-dns-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join(FILE);
        let lists = ["ads.example.com\ntracker.test\n".to_string(), "tracker.test\nbad.example.net\n".to_string()];
        std::fs::write(&path, build(&lists, 1_791_100_000)).unwrap();
        let list = List::open(&path).unwrap();
        assert_eq!((list.count, list.fetched), (3, 1_791_100_000));
        assert!(list.blocks("ads.example.com"));
        assert!(list.blocks("x.y.ads.example.com"));
        assert!(list.blocks("tracker.test"));
        assert!(!list.blocks("example.com"));
        assert!(!list.blocks("good.example.net"));
        assert!(!list.blocks("test"));

        std::fs::write(&path, b"TRDNSL1\0\x05\0\0\0\0\0\0\0").unwrap();
        assert!(List::open(&path).is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
