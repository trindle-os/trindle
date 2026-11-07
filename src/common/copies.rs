//! Whole copies of one file, each made the one way.

use std::fs;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

#[cfg(test)]
use std::io::Write;

#[cfg(test)]
use super::files::FULL;
use super::files::{open_file, rename_new, spare_name};
use super::gentle;

fn copy_spare(from: &Path, dir: &Path, name: &std::ffi::OsStr) -> std::io::Result<PathBuf> {
    use std::io::{Error, ErrorKind};
    use std::sync::atomic::{AtomicU64, Ordering};
    static COPIES: AtomicU64 = AtomicU64::new(0);
    const TRIES: usize = 16;
    let mut first = open_file(from)?;
    let mode = first.metadata()?.mode() & 0o777;
    for _ in 0..TRIES {
        let mut spare_as = std::ffi::OsString::from("copy-");
        spare_as.push(name);
        let spare = dir.join(spare_name(&spare_as, COPIES.fetch_add(1, Ordering::Relaxed)));
        let made = match fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open(&spare) {
            Ok(made) => made,
            Err(e) if e.kind() == ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        };
        let done = (|| {
            #[cfg(test)]
            if FULL.get() {
                (&made).write_all(&[0])?;
                return Err(Error::from_raw_os_error(libc::ENOSPC));
            }
            let made = gentle::copy_into(&mut first, made)?;
            made.set_permissions(fs::Permissions::from_mode(mode))?;
            made.sync_all()
        })();
        return match done {
            Ok(()) => Ok(spare),
            Err(e) => {
                let _ = fs::remove_file(&spare);
                Err(e)
            }
        };
    }
    Err(Error::new(ErrorKind::AlreadyExists, "no spare name is free"))
}

fn same_bytes(a: &Path, b: &Path) -> bool {
    use std::io::Read;
    let (Ok(x), Ok(y)) = (open_file(a), open_file(b)) else { return false };
    if !matches!((x.metadata(), y.metadata()), (Ok(m), Ok(n)) if m.len() == n.len()) {
        return false;
    }
    let (mut x, mut y) = (std::io::BufReader::new(x), std::io::BufReader::new(y));
    let (mut p, mut q) = (vec![0u8; 1 << 16], vec![0u8; 1 << 16]);
    loop {
        let Ok(n) = x.read(&mut p) else { return false };
        if n == 0 {
            return y.read(&mut q).is_ok_and(|m| m == 0);
        }
        if y.read_exact(&mut q[..n]).is_err() || p[..n] != q[..n] {
            return false;
        }
    }
}

pub fn copy_new(from: &Path, to: &Path) -> std::io::Result<()> {
    use std::io::{Error, ErrorKind};
    let (Some(dir), Some(name)) = (to.parent(), to.file_name()) else {
        return Err(Error::new(ErrorKind::InvalidInput, "it has no name to be kept under"));
    };
    let spare = copy_spare(from, dir, name)?;
    rename_new(&spare, to).inspect_err(|_| {
        let _ = fs::remove_file(&spare);
    })
}

pub fn copy_whole(from: &Path, to: &Path) -> std::io::Result<()> {
    use std::io::{Error, ErrorKind};
    let (Some(dir), Some(name)) = (to.parent(), to.file_name()) else {
        return Err(Error::new(ErrorKind::InvalidInput, "it has no name to be kept under"));
    };
    let spare = copy_spare(from, dir, name)?;
    fs::rename(&spare, to).inspect_err(|_| {
        let _ = fs::remove_file(&spare);
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Taken {
    pub name: String,
    pub from: PathBuf,
    pub there: PathBuf,
    pub new_size: u64,
    pub old_size: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Put {
    Made(PathBuf),
    Already(PathBuf),
    Taken(Taken),
}

pub fn copy_in(from: &Path, dir: &Path, name: &std::ffi::OsStr) -> std::io::Result<Put> {
    const TRIES: usize = 3;
    let to = dir.join(name);
    for _ in 0..TRIES {
        if let Ok(old) = fs::symlink_metadata(&to) {
            if same_bytes(from, &to) {
                return Ok(Put::Already(to));
            }
            let new_size = open_file(from)?.metadata()?.len();
            return Ok(Put::Taken(Taken { name: name.to_string_lossy().into_owned(), from: from.to_path_buf(), there: to, new_size, old_size: old.len() }));
        }
        match copy_new(from, &to) {
            Ok(()) => return Ok(Put::Made(to)),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(e) => return Err(e),
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "the name keeps being taken"))
}

pub fn replace(taken: &Taken) -> std::io::Result<()> {
    copy_whole(&taken.from, &taken.there)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::ffi::OsStrExt;

    fn names_in(d: &Path) -> Vec<String> {
        let mut n: Vec<String> = fs::read_dir(d).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
        n.sort();
        n
    }

    fn a_taken_name_is_asked_about(own: &Path, downloads: &Path) {
        let name = std::ffi::OsStr::new("chime.ogg");
        let asked = copy_in(&downloads.join("chime.ogg"), own, name).unwrap();
        let taken = Taken { name: "chime.ogg".into(), from: downloads.join("chime.ogg"), there: own.join("chime.ogg"), new_size: 24, old_size: 22 };
        assert_eq!(asked, Put::Taken(taken.clone()));
        assert_eq!(fs::read(own.join("chime.ogg")).unwrap(), b"the person's own chime", "the person's own sound was written over");
        assert_eq!(names_in(own), ["chime.ogg"], "a second name was made");
        assert_eq!(copy_in(&own.join("chime.ogg"), own, name).unwrap(), Put::Already(own.join("chime.ogg")));
        replace(&taken).unwrap();
        assert_eq!(fs::read(own.join("chime.ogg")).unwrap(), b"another chime altogether");
        assert_eq!(fs::metadata(own.join("chime.ogg")).unwrap().mode() & 0o777, 0o640, "the copy keeps the first's mode");
        assert_eq!(copy_in(&downloads.join("chime.ogg"), own, name).unwrap(), Put::Already(own.join("chime.ogg")));
        assert_eq!(names_in(own), ["chime.ogg"], "a spare file was left");
    }

    fn a_link_under_the_name_is_not_written_through(dir: &Path, own: &Path, downloads: &Path) {
        std::os::unix::fs::symlink(dir.join("elsewhere.ogg"), own.join("bell.ogg")).unwrap();
        fs::write(downloads.join("bell.ogg"), b"a bell").unwrap();
        let linked = copy_in(&downloads.join("bell.ogg"), own, std::ffi::OsStr::new("bell.ogg")).unwrap();
        assert!(matches!(linked, Put::Taken(_)), "{linked:?}");
        assert!(!dir.join("elsewhere.ogg").exists(), "the copy was written through a link");
        if let Put::Taken(t) = linked {
            replace(&t).unwrap();
        }
        assert!(!dir.join("elsewhere.ogg").exists(), "a replace was written through a link");
        assert_eq!(fs::read(own.join("bell.ogg")).unwrap(), b"a bell");
        assert!(!fs::symlink_metadata(own.join("bell.ogg")).unwrap().file_type().is_symlink());
    }

    fn a_full_drive_leaves_nothing(own: &Path, downloads: &Path) {
        fs::write(downloads.join("ping.ogg"), b"a ping").unwrap();
        assert_eq!(copy_in(&downloads.join("ping.ogg"), own, std::ffi::OsStr::new("ping.ogg")).unwrap(), Put::Made(own.join("ping.ogg")));
        fs::write(downloads.join("long.ogg"), vec![7u8; 4096]).unwrap();
        let before = names_in(own);
        FULL.set(true);
        let failed = (copy_in(&downloads.join("long.ogg"), own, std::ffi::OsStr::new("long.ogg")).is_err(),
                      copy_new(&downloads.join("long.ogg"), &own.join("long.ogg")).is_err(),
                      copy_whole(&downloads.join("long.ogg"), &own.join("chime.ogg")).is_err());
        FULL.set(false);
        assert_eq!(failed, (true, true, true));
        assert_eq!(names_in(own), before, "something half made was left");
        assert_eq!(fs::read(own.join("chime.ogg")).unwrap(), b"another chime altogether", "a copy that failed lost the old one");
        let taken = copy_new(&downloads.join("long.ogg"), &own.join("chime.ogg"));
        assert_eq!(taken.err().map(|e| e.kind()), Some(std::io::ErrorKind::AlreadyExists));
        assert_eq!(fs::read(own.join("chime.ogg")).unwrap(), b"another chime altogether");
        copy_new(&downloads.join("long.ogg"), &own.join("long.ogg")).unwrap();
        assert_eq!(fs::read(own.join("long.ogg")).unwrap().len(), 4096);
        copy_whole(&downloads.join("ping.ogg"), &own.join("long.ogg")).unwrap();
        assert_eq!(fs::read(own.join("long.ogg")).unwrap(), b"a ping");
        assert_eq!(names_in(own).len(), before.len() + 1, "no spare file is left");
    }

    fn a_pipe_is_not_waited_on(dir: &Path, own: &Path, downloads: &Path) {
        let pipe = downloads.join("pipe.ogg");
        let c = std::ffi::CString::new(pipe.as_os_str().as_bytes()).unwrap();
        // SAFETY: a path that ends in a nought, for a pipe made in this test's own folder.
        assert_eq!(unsafe { libc::mkfifo(c.as_ptr(), 0o600) }, 0);
        let (tell, told) = std::sync::mpsc::channel();
        let (asked, into) = (pipe.clone(), own.to_path_buf());
        std::thread::spawn(move || tell.send((copy_in(&asked, &into, std::ffi::OsStr::new("pipe.ogg")).is_err(), open_file(&asked).is_err())));
        let got = told.recv_timeout(std::time::Duration::from_secs(5));
        let _ = fs::OpenOptions::new().write(true).custom_flags(libc::O_NONBLOCK).open(&pipe);
        assert_eq!(got, Ok((true, true)), "a pipe was waited on, or copied");
        assert!(open_file(dir).is_err(), "a folder is no file");
    }

    #[test]
    fn a_file_copied_into_a_folder_never_writes_over_what_has_its_name() {
        let dir = std::env::temp_dir().join(format!("trindle-copy-in-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let (own, downloads) = (dir.join("sounds"), dir.join("Downloads"));
        fs::create_dir_all(&own).unwrap();
        fs::create_dir_all(&downloads).unwrap();
        fs::write(own.join("chime.ogg"), b"the person's own chime").unwrap();
        fs::write(downloads.join("chime.ogg"), b"another chime altogether").unwrap();
        fs::set_permissions(downloads.join("chime.ogg"), fs::Permissions::from_mode(0o640)).unwrap();
        a_taken_name_is_asked_about(&own, &downloads);
        a_link_under_the_name_is_not_written_through(&dir, &own, &downloads);
        a_full_drive_leaves_nothing(&own, &downloads);
        a_pipe_is_not_waited_on(&dir, &own, &downloads);
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_copy_longer_than_one_gentle_step_is_whole_and_keeps_its_mode() {
        let dir = std::env::temp_dir().join(format!("trindle-copy-steps-{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let (from, to) = (dir.join("Made-up.AppImage"), dir.join("kept.AppImage"));
        let long: Vec<u8> = (0..gentle::STEP as usize * 2 + 4321).map(|i| (i % 251) as u8).collect();
        fs::write(&from, &long).unwrap();
        fs::set_permissions(&from, fs::Permissions::from_mode(0o750)).unwrap();
        copy_new(&from, &to).unwrap();
        assert!(fs::read(&to).unwrap() == long, "the copy is not the first, byte for byte");
        assert_eq!(fs::metadata(&to).unwrap().mode() & 0o777, 0o750);
        fs::write(&from, &long[..long.len() - 7]).unwrap();
        copy_whole(&from, &to).unwrap();
        assert_eq!(fs::metadata(&to).unwrap().len() as usize, long.len() - 7);
        assert_eq!(fs::read_dir(&dir).unwrap().count(), 2, "a spare file was left");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn every_shared_way_a_file_of_any_size_is_written_is_gentle() {
        use crate::common::no_writing_in_place::{all_code, code};
        let files = all_code();
        let plain = |text: &str| code(text).into_iter().map(|line| line.split("//").next().unwrap_or(line)).collect::<Vec<_>>().join("\n");
        let of = |name: &str| plain(&files.iter().find(|(file, _)| file == name).unwrap_or_else(|| panic!("{name} is gone")).1);
        let here = of("common/copies.rs");
        assert_eq!(here.matches("gentle::copy_into(").count(), 1, "the one place a copy's bytes go across");
        for plain in ["io::copy(", "fs::copy(", "write_all(", "copy_file_range("] {
            assert!(!here.contains(plain), "a copy is made with {plain} and not by the gentle rule");
        }
        let callers: Vec<&str> = files.iter().filter(|(file, text)| file != "toolkit/common/files.rs" && plain(text).contains("write_whole_by(")).map(|(file, _)| file.as_str()).collect();
        assert!(callers.is_empty(), "a file of any size is written whole from a new place: write it gently (gentle::Out), not through write_whole_by: {callers:?}");
    }
}
