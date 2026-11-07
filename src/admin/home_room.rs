//! The room only the snapshots of the home folders hold.

use std::ffi::CStr;
use std::fs::File;
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use crate::common::home_snapshots as home;

const BATCH: usize = 128;

const FS_IOC_FIEMAP: libc::c_ulong = 0xC020_660B;
const LAST: u32 = 0x1;
const UNKNOWN: u32 = 0x2;
const NOT_WRITTEN: u32 = 0x4;
const PACKED: u32 = 0x8;
const INLINE: u32 = 0x200;
const SHARED: u32 = 0x2000;
const MOUNT_ROOT: u64 = 0x2000;

#[repr(C)]
#[derive(Clone, Copy)]
struct Extent {
    logical: u64,
    physical: u64,
    length: u64,
    reserved64: [u64; 2],
    flags: u32,
    reserved: [u32; 3],
}

#[repr(C)]
struct Map {
    start: u64,
    length: u64,
    flags: u32,
    mapped: u32,
    count: u32,
    reserved: u32,
    extents: [Extent; BATCH],
}

fn new_map() -> Box<Map> {
    // SAFETY: both structs are plain numbers, for which all-zeroes is valid.
    Box::new(unsafe { std::mem::zeroed() })
}

fn extents(file: &File, map: &mut Map, each: &mut dyn FnMut(u64, u64, u32)) {
    let mut start = 0u64;
    loop {
        map.start = start;
        map.length = u64::MAX - start;
        map.flags = 0;
        map.mapped = 0;
        map.count = BATCH as u32;
        // SAFETY: an open file, and a struct laid out as the kernel's with
        // room for the `count` extents it is told it may fill in.
        if unsafe { libc::ioctl(file.as_raw_fd(), FS_IOC_FIEMAP as _, map as *mut Map) } != 0 {
            return;
        }
        let got = (map.mapped as usize).min(BATCH);
        if got == 0 {
            return;
        }
        for e in &map.extents[..got] {
            if e.flags & (INLINE | UNKNOWN | NOT_WRITTEN) == 0 && e.length > 0 {
                each(e.physical, e.length, e.flags);
            }
            let next = e.logical.saturating_add(e.length);
            if e.flags & LAST != 0 || next <= start {
                return;
            }
            start = next;
        }
    }
}

fn open_in(dir: RawFd, name: &CStr, flags: i32) -> Option<File> {
    // SAFETY: a NUL-terminated name that outlives the call.
    let fd = unsafe { libc::openat(dir, name.as_ptr(), flags | libc::O_NOFOLLOW | libc::O_CLOEXEC | libc::O_NOCTTY) };
    // SAFETY: a descriptor just opened, which nothing else owns.
    (fd >= 0).then(|| unsafe { File::from_raw_fd(fd) })
}

fn mounted(dir: RawFd, name: &CStr) -> bool {
    // SAFETY: statx's struct is plain numbers, so all-zeroes is valid.
    let mut st: libc::statx = unsafe { std::mem::zeroed() };
    // SAFETY: a NUL-terminated name and a local struct the kernel fills in.
    let ok = unsafe {
        libc::statx(dir, name.as_ptr(), libc::AT_SYMLINK_NOFOLLOW | libc::AT_NO_AUTOMOUNT, libc::STATX_TYPE, &mut st)
    } == 0;
    !ok || st.stx_attributes & MOUNT_ROOT != 0
}

fn walk(folder: File, skip: Option<&CStr>, each: &mut dyn FnMut(&File)) {
    let fd = folder.into_raw_fd();
    // SAFETY: an open descriptor, which the stream owns from here on.
    let stream = unsafe { libc::fdopendir(fd) };
    if stream.is_null() {
        // SAFETY: the stream was not made, so the descriptor is still ours.
        unsafe { libc::close(fd) };
        return;
    }
    loop {
        // SAFETY: a stream that is open and read by nothing else.
        let entry = unsafe { libc::readdir64(stream) };
        if entry.is_null() {
            break;
        }
        // SAFETY: the entry is valid until this stream is read again, and
        // its name is NUL-terminated; a folder gone into has its own stream.
        let (name, kind) = unsafe { (CStr::from_ptr((*entry).d_name.as_ptr()), (*entry).d_type) };
        if matches!(name.to_bytes(), b"." | b"..") || skip == Some(name) {
            continue;
        }
        match kind {
            libc::DT_DIR if !mounted(fd, name) => {
                if let Some(inner) = open_in(fd, name, libc::O_RDONLY | libc::O_DIRECTORY) {
                    walk(inner, None, each);
                }
            }
            libc::DT_REG => {
                let quiet = libc::O_RDONLY | libc::O_NONBLOCK;
                if let Some(file) = open_in(fd, name, quiet | libc::O_NOATIME).or_else(|| open_in(fd, name, quiet)) {
                    each(&file);
                }
            }
            _ => {}
        }
    }
    // SAFETY: the stream made above, closed once; it closes the descriptor.
    unsafe { libc::closedir(stream) };
}

fn open_folder(path: &Path) -> Option<File> {
    std::fs::OpenOptions::new().read(true).custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW).open(path).ok()
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Held {
    plain: Vec<(u64, u64)>,
    packed: Vec<(u64, u64)>,
}

impl Held {
    fn add(&mut self, at: u64, length: u64, flags: u32) {
        if flags & PACKED != 0 {
            self.packed.push((at, length));
        } else {
            self.plain.push((at, at.saturating_add(length)));
        }
    }

    fn tidy(mut self) -> Held {
        self.plain.sort_unstable();
        let mut plain: Vec<(u64, u64)> = Vec::with_capacity(self.plain.len());
        for (from, to) in self.plain {
            match plain.last_mut() {
                Some(last) if from <= last.1 => last.1 = last.1.max(to),
                _ => plain.push((from, to)),
            }
        }
        self.packed.sort_unstable();
        let mut packed: Vec<(u64, u64)> = Vec::with_capacity(self.packed.len());
        for (at, length) in self.packed {
            match packed.last_mut() {
                Some(last) if last.0 == at => last.1 = length,
                _ => packed.push((at, length)),
            }
        }
        Held { plain, packed }
    }

    fn without(&self, other: &Held) -> Held {
        let mut plain = Vec::new();
        let mut theirs = other.plain.iter().peekable();
        for &(mut from, to) in &self.plain {
            while from < to {
                while theirs.peek().is_some_and(|t| t.1 <= from) {
                    theirs.next();
                }
                match theirs.peek() {
                    Some(&&(their_from, their_to)) if their_from < to => {
                        if their_from > from {
                            plain.push((from, their_from));
                        }
                        from = their_to;
                    }
                    _ => {
                        plain.push((from, to));
                        break;
                    }
                }
            }
        }
        let packed = self.packed.iter().filter(|(at, _)| other.packed.binary_search_by_key(at, |p| p.0).is_err())
            .copied().collect();
        Held { plain, packed }
    }

    fn join(mut self, mut other: Held) -> Held {
        self.plain.append(&mut other.plain);
        self.packed.append(&mut other.packed);
        self.tidy()
    }

    fn bytes(&self) -> u64 {
        self.plain.iter().map(|(from, to)| to - from).chain(self.packed.iter().map(|p| p.1)).sum()
    }
}

pub fn only_in_snapshots(live: &Path, snapshots: &Path, numbers: &[u32]) -> Option<u64> {
    let mut map = new_map();
    let skip = snapshots.strip_prefix(live).ok().and_then(|p| std::ffi::CString::new(p.to_str()?).ok());
    let mut now = Held::default();
    walk(open_folder(live)?, skip.as_deref(), &mut |file| {
        extents(file, &mut map, &mut |at, length, flags| {
            if flags & SHARED != 0 {
                now.add(at, length, flags);
            }
        });
    });
    let now = now.tidy();
    let mut alone = 0u64;
    let mut theirs = Held::default();
    for number in numbers {
        let mut this = Held::default();
        walk(open_folder(&home::tree(snapshots, *number))?, None, &mut |file| {
            extents(file, &mut map, &mut |at, length, flags| {
                if flags & SHARED != 0 {
                    this.add(at, length, flags);
                } else {
                    alone = alone.saturating_add(length);
                }
            });
        });
        theirs = theirs.join(this.tidy().without(&now));
    }
    Some(alone.saturating_add(theirs.bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn held(plain: &[(u64, u64)], packed: &[(u64, u64)]) -> Held {
        Held { plain: plain.to_vec(), packed: packed.to_vec() }.tidy()
    }

    #[test]
    fn nothing_on_the_disk_is_counted_twice() {
        let mut h = Held::default();
        for (at, length) in [(100, 50), (100, 50), (120, 60), (180, 20), (500, 10)] {
            h.add(at, length, SHARED);
        }
        for (at, length) in [(1000, 4096), (1000, 4096), (1001, 8192), (1000, 128)] {
            h.add(at, length, SHARED | PACKED);
        }
        let h = h.tidy();
        assert_eq!(h.plain, [(100, 200), (500, 510)]);
        assert_eq!(h.packed, [(1000, 4096), (1001, 8192)]);
        assert_eq!(h.bytes(), 100 + 10 + 4096 + 8192);
        assert_eq!(Held::default().tidy().bytes(), 0);
    }

    #[test]
    fn what_the_home_folders_still_hold_is_taken_off() {
        let snapshot = held(&[(0, 100), (200, 300), (400, 500), (600, 700)], &[(1000, 50), (2000, 60)]);
        let now = held(&[(0, 100), (220, 240), (390, 410), (490, 600)], &[(2000, 60), (3000, 9)]);
        let only = snapshot.without(&now);
        assert_eq!(only.plain, [(200, 220), (240, 300), (410, 490), (600, 700)]);
        assert_eq!(only.packed, [(1000, 50)]);
        assert_eq!(only.bytes(), 20 + 60 + 80 + 100 + 50);
        assert_eq!(snapshot.without(&Held::default()), snapshot);
        assert_eq!(snapshot.without(&snapshot).bytes(), 0);
        assert_eq!(snapshot.without(&held(&[(0, 650)], &[])).plain, [(650, 700)]);
        let both = only.clone().join(only.clone());
        assert_eq!(both, only);
    }

    #[test]
    fn the_walk_stays_inside_the_folder_it_was_given() {
        use std::io::Read;
        let root = std::env::temp_dir().join(format!("trindle-home-room-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let top = root.join("home");
        std::fs::create_dir_all(top.join("john/deep/deeper")).unwrap();
        std::fs::create_dir_all(top.join(".snapshots/1")).unwrap();
        std::fs::create_dir_all(root.join("outside")).unwrap();
        std::fs::write(top.join("john/a.txt"), "a").unwrap();
        std::fs::write(top.join("john/deep/deeper/b.txt"), "b").unwrap();
        std::fs::write(top.join(".snapshots/1/kept.txt"), "kept").unwrap();
        std::fs::write(root.join("outside/secret.txt"), "secret").unwrap();
        std::os::unix::fs::symlink(root.join("outside"), top.join("john/way-out")).unwrap();
        std::os::unix::fs::symlink(root.join("outside/secret.txt"), top.join("john/out.txt")).unwrap();
        std::fs::create_dir_all(top.join("john/.snapshots")).unwrap();
        std::fs::write(top.join("john/.snapshots/c.txt"), "c").unwrap();
        let mut seen = Vec::new();
        let skip = std::ffi::CString::new(".snapshots").unwrap();
        walk(open_folder(&top).unwrap(), Some(&skip), &mut |mut file: &File| {
            let mut words = String::new();
            file.read_to_string(&mut words).unwrap();
            seen.push(words);
        });
        seen.sort();
        assert_eq!(seen, ["a", "b", "c"]);
        std::os::unix::fs::symlink(&top, root.join("link")).unwrap();
        assert!(open_folder(&root.join("link")).is_none());
        assert_eq!(only_in_snapshots(&root.join("link"), &top.join(".snapshots"), &[1]), None);
        assert_eq!(only_in_snapshots(&top, &top.join(".snapshots"), &[7]), None);
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    #[ignore = "needs root and real snapshots of /home"]
    fn on_real_snapshots_the_room_is_measured() {
        let folder = Path::new(home::SNAPSHOTS);
        let numbers: Vec<u32> = home::kept_in(folder).iter().map(|k| k.number).collect();
        let started = std::time::Instant::now();
        let room = only_in_snapshots(Path::new(home::HOME), folder, &numbers);
        println!("snapshots {numbers:?}: {room:?} bytes held only by them, found in {:?}", started.elapsed());
        let disk = home::room_on(Path::new(home::HOME)).unwrap();
        println!("disk: {} bytes, {} free, nearly full: {}", disk.total, disk.free, home::nearly_full(disk));
        assert!(room.is_some());
        if let Some(least) = std::env::var("TRINDLE_ROOM_AT_LEAST").ok().and_then(|n| n.parse::<u64>().ok()) {
            assert!(room.unwrap() >= least, "{room:?} is under {least}");
        }
        if let Some(most) = std::env::var("TRINDLE_ROOM_AT_MOST").ok().and_then(|n| n.parse::<u64>().ok()) {
            assert!(room.unwrap() <= most, "{room:?} is over {most}");
        }
    }
}
