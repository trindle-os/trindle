//! The one way anything out of a backup is put on the disk.

use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::os::fd::AsRawFd;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::toolkit::t;
use std::process::{Command, Stdio};

use crate::common::files::{open_in, open_named, private_in, through, Named};

pub(super) fn outside() -> &'static str {
    t!("store-because-outside-own")
}

#[derive(Clone, Copy, PartialEq)]
pub(super) enum Holds {

    Anything,

    FilesAndFolders,
}

fn gentle_tar() -> Command {
    let mut tar = Command::new("tar");
    crate::common::gentle::program(&mut tar);
    tar
}

fn tar(args: &[&OsStr]) -> Result<String, String> {
    let out = gentle_tar().args(args).stdin(Stdio::null()).output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(super::refusal::last_said(&String::from_utf8_lossy(&out.stderr)))
    }
}

pub(super) fn listed(archive: &Path, how: &str) -> Result<String, String> {
    tar(&[how.as_ref(), "-tf".as_ref(), archive.as_os_str()])
}

pub(super) fn list_of(pack: &Path, how: Option<&str>) -> Result<String, String> {
    let mut args: Vec<&OsStr> = how.iter().map(OsStr::new).collect();
    args.extend([OsStr::new("-tf"), pack.as_os_str()]);
    tar(&args)
}

pub(super) fn holds_in(listing: &str, member: &str) -> bool {
    has(listing, member)
}

pub(super) fn too_big() -> &'static str {
    t!("store-because-too-big")
}

pub(super) fn read_out(pack: &Path, how: Option<&str>, member: &str, most: u64) -> Result<Vec<u8>, String> {
    use std::io::Read;
    let mut child = gentle_tar().args(how).arg("-xOf").arg(pack).arg("--").arg(member)
        .stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().map_err(|e| e.to_string())?;
    let (out, err) = (child.stdout.take(), child.stderr.take());
    let (bytes, said) = std::thread::scope(|both| {

        let said = both.spawn(move || {
            let (mut last, mut some) = (Vec::new(), [0u8; 4096]);
            let Some(mut err) = err else { return last };
            while let Ok(n @ 1..) = err.read(&mut some) {
                last.extend_from_slice(&some[..n]);
                let extra = last.len().saturating_sub(some.len());
                last.drain(..extra);
            }
            last
        });
        let mut bytes = Vec::new();
        let read = match out {

            Some(out) => out.take(most + 1).read_to_end(&mut bytes).map(|_| ()),
            None => Ok(()),
        };
        (read.map(|()| bytes), said.join().unwrap_or_default())
    });
    let ended = child.wait().map_err(|e| e.to_string())?;
    let bytes = bytes.map_err(|e| e.to_string())?;
    if bytes.len() as u64 > most {
        return Err(too_big().to_string());
    }
    if !ended.success() {
        return Err(String::from_utf8_lossy(&said).trim().lines().last().unwrap_or("failed").to_string());
    }
    Ok(bytes)
}

fn has(listing: &str, member: &str) -> bool {
    listing.lines().any(|l| l == member || l.trim_end_matches('/') == member || l.starts_with(&format!("{member}/")))
}

pub(super) fn taken_out(pack: &Path, member: &str, work: &Path) -> Result<PathBuf, String> {
    tar(&["-xf".as_ref(), pack.as_os_str(), "-C".as_ref(), work.as_os_str(), "--".as_ref(), member.as_ref()])?;
    let at = work.join(member);
    match open_named(&at, None, None) {
        Ok(Named::File(..)) => Ok(at),
        _ => Err(outside().to_string()),
    }
}

pub(super) fn put_back(archive: &Path, how: &str, listing: &str, wanted: &[&str], into: &Path, holds: Holds) -> Result<(), String> {
    let present: Vec<&str> = wanted.iter().copied().filter(|m| has(listing, m)).collect();
    if present.is_empty() {
        return Ok(());
    }
    std::fs::create_dir_all(into).map_err(|e| e.to_string())?;
    let stage = private_in(into, ".trindle-putting-back").map_err(|e| e.to_string())?;
    let done = (|| {
        let mut args: Vec<&OsStr> = vec![how.as_ref(), "-xf".as_ref(), archive.as_os_str(), "-C".as_ref(), stage.as_os_str(), "--no-same-owner".as_ref()];
        if holds == Holds::Anything {
            args.push("-p".as_ref());
        }
        args.push("--".as_ref());
        args.extend(present.iter().map(|m| OsStr::new(*m)));
        tar(&args)?;
        let from = File::open(&stage).map_err(|e| e.to_string())?;
        let to = File::open(into).map_err(|e| e.to_string())?;
        if holds == Holds::FilesAndFolders {
            only_files(&from)?;
        }

        let mut in_the_way = Vec::new();
        walk(&from, &to, Step::Look, into, &mut in_the_way)?;
        if !in_the_way.is_empty() {
            return Err(super::refusal::in_the_way(&in_the_way));
        }
        walk(&from, &to, Step::Move, into, &mut in_the_way)?;
        if in_the_way.is_empty() { Ok(()) } else { Err(super::refusal::in_the_way(&in_the_way)) }
    })();
    // Only what was not moved is left in it.
    let _ = std::fs::remove_dir_all(&stage);
    done
}

fn names(dir: &File) -> Result<Vec<OsString>, String> {
    std::fs::read_dir(through(dir)).and_then(|list| list.map(|e| e.map(|e| e.file_name())).collect()).map_err(|e| e.to_string())
}

fn only_files(dir: &File) -> Result<(), String> {
    for name in names(dir)? {
        match open_named(Path::new(&name), Some(dir), None) {
            Ok(Named::File(..)) => {}
            Ok(Named::Folder(inside, _)) => only_files(&inside)?,
            _ => return Err(outside().to_string()),
        }
    }
    Ok(())
}

#[derive(Clone, Copy, PartialEq)]
enum Step {

    Look,

    Move,
}

fn walk(from: &File, to: &File, step: Step, at: &Path, in_the_way: &mut Vec<String>) -> Result<(), String> {
    for name in names(from)? {
        let (thing, meta) = open_in(from, &name, None).map_err(|e| e.to_string())?;
        match open_in(to, &name, None) {
            Ok((there, m)) if m.is_dir() && meta.is_dir() => {

                // is opened up first: it is the stage's own copy.
                let _ = std::fs::set_permissions(through(&thing), std::fs::Permissions::from_mode(meta.permissions().mode() & 0o7777 | 0o700));
                walk(&thing, &there, step, &at.join(&name), in_the_way)?;
                continue;
            }
            Ok((_, m)) if m.is_dir() || meta.is_dir() => {
                in_the_way.push(at.join(&name).display().to_string());
                continue;
            }
            Ok(_) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.to_string()),
        }
        if step == Step::Move {
            moved(from, to, &name).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn moved(from: &File, to: &File, name: &OsStr) -> std::io::Result<()> {
    let name = std::ffi::CString::new(name.as_bytes()).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "a name cannot hold a nought"))?;
    // SAFETY: both folders are kept open by `from` and `to` until the call

    if unsafe { libc::renameat(from.as_raw_fd(), name.as_ptr(), to.as_raw_fd(), name.as_ptr()) } == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

#[cfg(test)]
pub(super) fn holds(pack: &Path, how: Option<&str>, member: &str) -> Result<bool, String> {
    Ok(holds_in(&list_of(pack, how)?, member))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::needs;

    #[test]
    fn what_is_in_the_way_is_said_with_every_path_and_nothing_is_put_back() {
        let tar = needs::program("tar");
        let top = std::env::temp_dir().join(format!("trindle-in-the-way-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        std::fs::create_dir_all(top.join("src/.config/trindle/sub")).unwrap();
        std::fs::create_dir_all(top.join("src/.appimage")).unwrap();
        std::fs::write(top.join("src/.config/trindle/sub/a"), "a").unwrap();
        std::fs::write(top.join("src/.appimage/App.AppImage"), "x").unwrap();
        run(&tar, &["-czf", "../b.tar.gz", ".config", ".appimage"], &top.join("src"));
        let pack = top.join("b.tar.gz");
        let listing = listed(&pack, "--gzip").unwrap();
        let wanted = [".config/trindle", ".appimage"];

        let home = top.join("home");
        std::fs::create_dir_all(home.join(".config")).unwrap();
        std::fs::write(home.join(".config/trindle"), "mine").unwrap();
        std::fs::create_dir_all(&home).unwrap();
        std::os::unix::fs::symlink(top.join("elsewhere"), home.join(".appimage")).unwrap();
        let e = put_back(&pack, "--gzip", &listing, &wanted, &home, Holds::FilesAndFolders).unwrap_err();
        let (words, behind) = super::super::refusal::apart(&e);
        assert_eq!(words, "Nothing was put back, because something on this computer is already where part of the backup needs to go.");
        let mut both = vec![home.join(".config/trindle").display().to_string(), home.join(".appimage").display().to_string()];
        both.sort();
        let mut behind = behind;
        behind.sort();
        assert_eq!(behind, both);

        assert_eq!(std::fs::read_to_string(home.join(".config/trindle")).unwrap(), "mine");
        assert!(std::fs::symlink_metadata(home.join(".appimage")).unwrap().file_type().is_symlink());
        assert!(!top.join("elsewhere").exists());

        let home2 = top.join("home2");
        std::fs::create_dir_all(home2.join(".config/trindle/sub/a/inside")).unwrap();
        let e = put_back(&pack, "--gzip", &listing, &wanted, &home2, Holds::FilesAndFolders).unwrap_err();
        let (words, behind) = super::super::refusal::apart(&e);
        assert_eq!(words, super::super::refusal::in_the_way_words());
        assert_eq!(behind, vec![home2.join(".config/trindle/sub/a").display().to_string()]);
        assert!(!home2.join(".appimage").exists());

        let home3 = top.join("home3");
        put_back(&pack, "--gzip", &listing, &wanted, &home3, Holds::FilesAndFolders).unwrap();
        assert!(home3.join(".config/trindle/sub/a").is_file());
        let _ = std::fs::remove_dir_all(&top);
    }

    const APP: &str = "org.example.Notes";

    fn run(tar: &Path, args: &[&str], at: &Path) {
        let done = Command::new(tar).args(args).current_dir(at).output().unwrap();
        assert!(done.status.success(), "tar {args:?}: {}", String::from_utf8_lossy(&done.stderr));
    }

    struct Door {
        name: &'static str,
        under: &'static str,
        links_kept: bool,

        clears: bool,
        pack: fn(&Path, &Path, &Path) -> PathBuf,
        open: fn(&Path, &Path, &Path) -> Result<(), String>,
    }

    fn flat_pack(tar: &Path, inner: &Path, top: &Path) -> PathBuf {
        std::fs::create_dir_all(top.join("flat/apps")).unwrap();
        std::fs::copy(inner, top.join(format!("flat/apps/{APP}.tar.gz"))).unwrap();
        std::fs::write(top.join("flat/manifest.json"), serde_json::json!({
            "format_version": 1,
            "apps": [{"id": APP, "name": "Notes", "origin": "flathub", "blob": format!("apps/{APP}.tar.gz")}],
        }).to_string()).unwrap();
        run(tar, &["-cf", "../flat.fmpack", "manifest.json", "apps"], &top.join("flat"));
        top.join("flat.fmpack")
    }

    fn flat_door(pack: &Path, work: &Path, into: &Path) -> Result<(), String> {
        let file = super::super::flat::read(pack)?;
        let app = &file.apps[0];
        super::super::flat::settings_into(&file, app, app.settings.as_ref().unwrap(), work, into)
    }

    fn home_pack(tar: &Path, inner: &Path, top: &Path) -> PathBuf {
        std::fs::create_dir_all(top.join("part/trindle")).unwrap();
        std::fs::copy(inner, top.join("part/trindle/home.tar.gz")).unwrap();
        run(tar, &["-cf", "../part.fmpack", "trindle"], &top.join("part"));
        top.join("part.fmpack")
    }

    fn home_door(pack: &Path, work: &Path, into: &Path) -> Result<(), String> {
        super::super::backup::home_part_into(pack, work, into)
    }

    fn old_pack(_: &Path, inner: &Path, _: &Path) -> PathBuf {
        inner.to_path_buf()
    }

    fn old_door(pack: &Path, _: &Path, into: &Path) -> Result<(), String> {
        super::super::backup::old_into(pack, into)
    }

    const DOORS: [Door; 5] = [
        Door { name: "an app's folder in a Flat backup", under: APP, links_kept: true, clears: true, pack: flat_pack, open: flat_door },
        Door { name: "Trindle's settings in a Flat backup", under: ".config/trindle", links_kept: false, clears: false, pack: home_pack, open: home_door },
        Door { name: "the AppImages in a Flat backup", under: ".appimage", links_kept: false, clears: false, pack: home_pack, open: home_door },
        Door { name: "Trindle's settings in an earlier backup", under: ".config/trindle", links_kept: false, clears: false, pack: old_pack, open: old_door },
        Door { name: "an app's folder in an earlier backup", under: ".var/app/org.example.Notes", links_kept: true, clears: false, pack: old_pack, open: old_door },
    ];

    struct Try {
        tar: PathBuf,
        top: PathBuf,
        into: PathBuf,
        victim: PathBuf,
    }

    impl Try {
        fn new(n: usize, what: &str) -> Try {
            let top = std::env::temp_dir().join(format!("trindle-put-back-{}-{n}-{what}", std::process::id()));
            let _ = std::fs::remove_dir_all(&top);
            let t = Try { tar: needs::program("tar"), into: top.join("into"), victim: top.join("victim"), top };
            for dir in [&t.into, &t.victim, &t.top.join("work"), &t.top.join("packed")] {
                std::fs::create_dir_all(dir).unwrap();
            }
            t
        }

        fn packed(&self, under: &str) -> PathBuf {
            let at = self.top.join("packed").join(under);
            std::fs::create_dir_all(&at).unwrap();
            at
        }

        fn restore(&self, door: &Door, members: &[String]) -> Result<(), String> {
            let plain = self.top.join("inner.tar");
            let _ = std::fs::remove_file(&plain);
            for (i, member) in members.iter().enumerate() {
                let how = if i == 0 { "-cf" } else { "-rf" };
                run(&self.tar, &[how, "../inner.tar", "--no-recursion", member], &self.top.join("packed"));
            }
            let packed = Command::new("gzip").args(["-kf", "inner.tar"]).current_dir(&self.top).status().unwrap();
            assert!(packed.success());
            let pack = (door.pack)(&self.tar, &self.top.join("inner.tar.gz"), &self.top);
            (door.open)(&pack, &self.top.join("work"), &self.into)
        }

        fn untouched(&self, door: &Door, what: &str) {
            let left: Vec<_> = std::fs::read_dir(&self.victim).unwrap().flatten().map(|e| e.file_name()).collect();
            assert!(left.is_empty(), "{}, {what}: written outside the folder it was given: {left:?}", door.name);
        }
    }

    #[test]
    fn nothing_out_of_a_backup_is_written_through_a_link_or_outside_its_folder() {
        needs::program("gzip");
        for (n, door) in DOORS.iter().enumerate() {
            let under = door.under;
            a_link_left_in_the_folder(n, door, under);
            the_folder_itself_is_a_link(n, door, under);
            the_backup_brings_a_link_then_a_file(n, door, under);
            the_backup_brings_a_link_alone(n, door, under);
            a_plain_backup_goes_back(n, door, under);
        }
    }

    fn a_link_left_in_the_folder(n: usize, door: &Door, under: &str) {

        let t = Try::new(n, "left");
        std::fs::create_dir_all(t.into.join(under)).unwrap();
        std::os::unix::fs::symlink(&t.victim, t.into.join(under).join("link")).unwrap();
        std::fs::create_dir_all(t.packed(under).join("link")).unwrap();
        std::fs::write(t.packed(under).join("link/run.desktop"), "x").unwrap();
        let _ = t.restore(door, &[format!("{under}/link/run.desktop")]);
        t.untouched(door, "a link left in the folder");
    }

    fn the_folder_itself_is_a_link(n: usize, door: &Door, under: &str) {

        let t = Try::new(n, "top");
        if let Some(up) = t.into.join(under).parent() {
            std::fs::create_dir_all(up).unwrap();
        }
        std::os::unix::fs::symlink(&t.victim, t.into.join(under)).unwrap();
        std::fs::write(t.packed(under).join("run.desktop"), "x").unwrap();
        let _ = t.restore(door, &[format!("{under}/run.desktop")]);
        t.untouched(door, "the folder is a link");
    }

    fn the_backup_brings_a_link_then_a_file(n: usize, door: &Door, under: &str) {

        let t = Try::new(n, "brought");
        std::os::unix::fs::symlink(&t.victim, t.packed(under).join("link")).unwrap();
        let first = t.top.join("packed-two").join(under).join("link");
        std::fs::create_dir_all(&first).unwrap();
        std::fs::write(first.join("run.desktop"), "x").unwrap();
        run(&t.tar, &["-cf", "../both.tar", "--no-recursion", &format!("{under}/link")], &t.top.join("packed"));
        run(&t.tar, &["-rf", "../both.tar", "--no-recursion", &format!("{under}/link/run.desktop")], &t.top.join("packed-two"));
        assert!(Command::new("gzip").args(["-kf", "both.tar"]).current_dir(&t.top).status().unwrap().success());
        let pack = (door.pack)(&t.tar, &t.top.join("both.tar.gz"), &t.top);
        let _ = (door.open)(&pack, &t.top.join("work"), &t.into);
        t.untouched(door, "the backup brings a link and a file through it");
    }

    fn the_backup_brings_a_link_alone(n: usize, door: &Door, under: &str) {

        let t = Try::new(n, "link");
        std::os::unix::fs::symlink(&t.victim, t.packed(under).join("link")).unwrap();
        std::fs::write(t.packed(under).join("kept"), "x").unwrap();
        let done = t.restore(door, &[format!("{under}/kept"), format!("{under}/link")]);
        let link = t.into.join(under).join("link");
        if door.links_kept {
            assert_eq!(done, Ok(()), "{}", door.name);
            assert!(std::fs::symlink_metadata(&link).unwrap().file_type().is_symlink(), "{}", door.name);
            assert!(t.into.join(under).join("kept").is_file(), "{}", door.name);
        } else {
            assert!(done.is_err(), "{}: a link among Trindle's settings is refused", door.name);
            assert!(std::fs::symlink_metadata(&link).is_err(), "{}: the link was put back", door.name);
            assert!(!t.into.join(under).join("kept").exists(), "{}: refused whole, before anything is moved", door.name);
        }
        t.untouched(door, "the backup brings a link");
    }

    fn a_plain_backup_goes_back(n: usize, door: &Door, under: &str) {

        let t = Try::new(n, "plain");
        std::fs::create_dir_all(t.into.join(under).join("sub")).unwrap();
        std::fs::write(t.into.join(under).join("sub/old"), "old").unwrap();
        std::fs::write(t.into.join(under).join("sub/both"), "old").unwrap();
        std::fs::create_dir_all(t.packed(under).join("sub")).unwrap();
        std::fs::write(t.packed(under).join("sub/both"), "new").unwrap();
        std::fs::write(t.packed(under).join("top"), "new").unwrap();
        assert_eq!(t.restore(door, &[format!("{under}/sub/both"), format!("{under}/top")]), Ok(()), "{}", door.name);
        let read = |name: &str| std::fs::read_to_string(t.into.join(under).join(name)).ok();
        assert_eq!((read("sub/both").as_deref(), read("top").as_deref()), (Some("new"), Some("new")), "{}", door.name);

        if !door.clears {
            assert_eq!(read("sub/old").as_deref(), Some("old"), "{}", door.name);
        }
        let left: Vec<_> = std::fs::read_dir(&t.into).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.starts_with(".trindle-putting-back")).collect();
        assert!(left.is_empty(), "{}: the folder it was unpacked in is gone: {left:?}", door.name);
    }

    #[test]
    fn a_part_of_a_backup_that_is_no_ordinary_file_is_not_read() {
        let tar = needs::program("tar");
        let top = std::env::temp_dir().join(format!("trindle-put-back-part-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        std::fs::create_dir_all(top.join("stage/apps")).unwrap();
        std::fs::create_dir_all(top.join("work")).unwrap();
        std::fs::write(top.join("secret"), "mine").unwrap();
        std::os::unix::fs::symlink(top.join("secret"), top.join("stage/apps/linked.tar.gz")).unwrap();
        let pipe = std::ffi::CString::new(top.join("stage/apps/pipe.tar.gz").as_os_str().as_bytes()).unwrap();
        // SAFETY: the name is a string that ends in a nought and lives until the call returns.
        assert_eq!(unsafe { libc::mkfifo(pipe.as_ptr(), 0o600) }, 0);
        std::fs::write(top.join("stage/apps/plain.tar.gz"), "x").unwrap();
        run(&tar, &["-cf", "../pack.tar", "apps"], &top.join("stage"));
        let pack = top.join("pack.tar");
        assert_eq!(taken_out(&pack, "apps/plain.tar.gz", &top.join("work")), Ok(top.join("work/apps/plain.tar.gz")));
        assert_eq!(taken_out(&pack, "apps/linked.tar.gz", &top.join("work")), Err(outside().to_string()));
        assert_eq!(taken_out(&pack, "apps/pipe.tar.gz", &top.join("work")), Err(outside().to_string()));
        assert!(taken_out(&pack, "apps/absent.tar.gz", &top.join("work")).is_err());
        let _ = std::fs::remove_dir_all(&top);
    }

    #[test]
    fn a_backup_that_cannot_all_go_back_moves_nothing() {
        needs::program("gzip");
        for (n, door) in DOORS.iter().enumerate().filter(|(_, d)| !d.clears) {
            let under = door.under;
            let beside: Vec<String> = (0..40).map(|i| format!("beside-{i:02}")).collect();
            let pack = |t: &Try, amid: &str| {
                for name in &beside {
                    std::fs::write(t.packed(under).join(name), "new").unwrap();
                }
                let mut members: Vec<String> = beside.iter().map(|name| format!("{under}/{name}")).collect();

                members.insert(beside.len() / 2, format!("{under}/{amid}"));
                members
            };
            let nothing_moved = |t: &Try, what: &str| {
                let there: Vec<&String> = beside.iter().filter(|name| t.into.join(under).join(name).exists()).collect();
                assert!(there.is_empty(), "{}, {what}: refused, with {} of {} things already moved", door.name, there.len(), beside.len());
                let left: Vec<_> = std::fs::read_dir(&t.into).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned())
                    .filter(|n| n.starts_with(".trindle-putting-back")).collect();
                assert!(left.is_empty(), "{}, {what}: the folder it was unpacked in is gone: {left:?}", door.name);
            };

            let t = Try::new(n, "link-in-the-way");
            std::fs::create_dir_all(t.into.join(under)).unwrap();
            std::os::unix::fs::symlink(&t.victim, t.into.join(under).join("sub")).unwrap();
            std::fs::create_dir_all(t.packed(under).join("sub")).unwrap();
            std::fs::write(t.packed(under).join("sub/inside"), "new").unwrap();
            let members = pack(&t, "sub/inside");
            assert!(t.restore(door, &members).is_err(), "{}: a link where a folder is to be filled", door.name);
            nothing_moved(&t, "a link where a folder is to be filled");
            t.untouched(door, "a link where a folder is to be filled");

            let t = Try::new(n, "file-in-the-way");
            std::fs::create_dir_all(t.into.join(under)).unwrap();
            std::fs::write(t.into.join(under).join("sub"), "old").unwrap();
            std::fs::create_dir_all(t.packed(under).join("sub")).unwrap();
            std::fs::write(t.packed(under).join("sub/inside"), "new").unwrap();
            let members = pack(&t, "sub/inside");
            assert!(t.restore(door, &members).is_err(), "{}: a file where a folder is to be filled", door.name);
            nothing_moved(&t, "a file where a folder is to be filled");
            assert_eq!(std::fs::read_to_string(t.into.join(under).join("sub")).unwrap(), "old", "{}", door.name);

            let t = Try::new(n, "folder-in-the-way");
            std::fs::create_dir_all(t.into.join(under).join("thing")).unwrap();
            std::fs::write(t.into.join(under).join("thing/kept"), "old").unwrap();
            std::fs::write(t.packed(under).join("thing"), "new").unwrap();
            let members = pack(&t, "thing");
            assert!(t.restore(door, &members).is_err(), "{}: a folder where a file is to go", door.name);
            nothing_moved(&t, "a folder where a file is to go");
            assert_eq!(std::fs::read_to_string(t.into.join(under).join("thing/kept")).unwrap(), "old", "{}", door.name);
        }
    }

    fn noise(len: usize) -> Vec<u8> {
        let mut n = 0x2545_f491_4f6c_dd1du64;
        (0..len).map(|_| {
            n ^= n << 13;
            n ^= n >> 7;
            n ^= n << 17;
            (n >> 24) as u8
        }).collect()
    }

    #[test]
    fn a_part_that_cannot_be_taken_out_fails_the_restore() {
        needs::program("gzip");
        let door = &DOORS[1];
        let t = Try::new(1, "unread");
        std::fs::write(t.packed(door.under).join("settings.conf"), noise(96 << 10)).unwrap();

        assert_eq!(t.restore(door, &[format!("{}/settings.conf", door.under)]), Ok(()));
        let back = t.into.join(door.under).join("settings.conf");
        assert!(back.is_file());
        std::fs::remove_file(&back).unwrap();

        std::fs::create_dir_all(t.top.join("none/apps")).unwrap();
        run(&t.tar, &["-cf", "../none.fmpack", "apps"], &t.top.join("none"));
        assert_eq!(home_door(&t.top.join("none.fmpack"), &t.top.join("work"), &t.into), Ok(()));

        let whole = std::fs::read(t.top.join("part.fmpack")).unwrap();
        assert!(whole.len() > 96 << 10);
        std::fs::write(t.top.join("cut.fmpack"), &whole[..whole.len() / 2]).unwrap();
        let _ = std::fs::remove_dir_all(t.top.join("work/trindle"));
        assert!(home_door(&t.top.join("cut.fmpack"), &t.top.join("work"), &t.into).is_err(), "a backup cut short was said to be put back");
        assert!(!back.exists());

        std::fs::write(t.top.join("none-at-all.fmpack"), b"not a backup\n".repeat(400)).unwrap();
        let _ = std::fs::remove_dir_all(t.top.join("work/trindle"));
        assert!(home_door(&t.top.join("none-at-all.fmpack"), &t.top.join("work"), &t.into).is_err(), "a file that is no backup was said to be put back");
        assert!(!back.exists());
    }

    #[test]
    fn what_is_read_out_of_a_backup_is_read_so_far_and_no_further() {
        let tar = needs::program("tar");
        let top = std::env::temp_dir().join(format!("trindle-put-back-read-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        std::fs::create_dir_all(top.join("stage")).unwrap();
        std::fs::write(top.join("stage/small"), "hello").unwrap();
        std::fs::write(top.join("stage/big"), noise(300 << 10)).unwrap();
        run(&tar, &["-cf", "../pack.tar", "small", "big"], &top.join("stage"));
        let pack = top.join("pack.tar");
        assert_eq!(read_out(&pack, None, "small", 100), Ok(b"hello".to_vec()));
        assert_eq!(read_out(&pack, None, "small", 5), Ok(b"hello".to_vec()));
        assert_eq!(read_out(&pack, None, "small", 4), Err(too_big().to_string()));
        assert_eq!(read_out(&pack, None, "big", 1000), Err(too_big().to_string()));
        assert_eq!(read_out(&pack, None, "big", 300 << 10).map(|b| b.len()), Ok(300 << 10));
        assert!(read_out(&pack, None, "absent", 100).is_err_and(|e| e != too_big()));
        assert_eq!((holds(&pack, None, "small"), holds(&pack, None, "absent")), (Ok(true), Ok(false)));
        std::fs::write(top.join("none.tar"), b"not a backup\n".repeat(400)).unwrap();
        assert!(holds(&top.join("none.tar"), None, "small").is_err());
        assert!(holds(&pack, Some("--gzip"), "small").is_err(), "a backup not packed the way it is said to be");
        let _ = std::fs::remove_dir_all(&top);
    }

    fn unpacks(word: &str) -> bool {
        let long = ["--extract", "--get"];
        let short = word.strip_prefix('-').filter(|rest| !rest.starts_with('-') && !rest.is_empty() && rest.chars().all(|c| c.is_ascii_alphabetic()));
        long.iter().any(|l| word == *l || word.starts_with(&format!("{l}="))) || short.is_some_and(|s| s.contains('x') && !s.contains('O'))
    }

    fn quoted<'a>(lines: &[&'a str]) -> Vec<&'a str> {
        lines.iter().flat_map(|l| l.split('"').skip(1).step_by(2)).collect()
    }

    const NOT_A_BACKUP: &[(&str, usize, &str)] = &[
        ("store/parts.rs", 1, "dpkg-deb unpacks a Debian package the person chose to install, into a folder made for it"),
    ];

    #[test]
    fn a_restore_unpacks_onto_the_disk_only_here() {
        use crate::common::no_writing_in_place::{all_code, code};
        let files = all_code();
        let mut seen = 0;
        for (file, text) in files.iter().filter(|(f, _)| f.starts_with("store/") && f != "store/put_back.rs") {
            seen += 1;
            let found: Vec<&str> = quoted(&code(text)).into_iter().filter(|w| unpacks(w)).collect();
            let listed = NOT_A_BACKUP.iter().find(|(f, ..)| f == file).map_or(0, |(_, n, _)| *n);
            assert_eq!(found.len(), listed, "{file} has a program unpack onto the disk itself ({found:?}), and {listed} such are listed in NOT_A_BACKUP: a backup goes through store::put_back");
        }
        assert!(seen > 20, "Apps' code was not found");
        assert!(NOT_A_BACKUP.iter().all(|(f, n, why)| *n > 0 && !why.is_empty() && files.iter().any(|(name, _)| name == f)), "each one listed is there, and says how many and why");

        for word in ["-xf", "-xpf", "-xzf", "-x", "-xJf", "-xvf", "-zxf", "--extract", "--get", "--extract=x"] {
            assert!(unpacks(word), "{word}");
        }
        for word in ["-xOf", "-xzOf", "-tf", "-czf", "--exclude=.config/trindle/github-key", "x", "-", "--", "--gzip", "-C"] {
            assert!(!unpacks(word), "{word}");
        }
        assert_eq!(quoted(&["tar(&[\"-xf\".as_ref(), a, \"-C\".as_ref()])"]), ["-xf", "-C"]);
    }
}
