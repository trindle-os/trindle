//! Fonts people add themselves: unpacked into their own folder, checked for
//! shape, and listed for Settings.

use std::path::{Path, PathBuf};

use super::copies::{Put, Taken};
use super::files::data_home;
use super::run::text;
use crate::toolkit::t;

pub fn fonts_dir() -> PathBuf {
    data_home().join("fonts")
}

pub fn is_archive(name: &str) -> bool {
    let n = name.to_lowercase();
    [".zip", ".tar.gz", ".tgz", ".tar.xz", ".tar.bz2", ".tar"].iter().any(|e| n.ends_with(e))
}

pub fn is_font(name: &str) -> bool {
    let n = name.to_lowercase();
    FONT_KINDS.iter().any(|e| n.ends_with(e))
}

const FONT_KINDS: [&str; 4] = [".ttf", ".otf", ".ttc", ".woff2"];

fn gently(program: &str, args: &[&str]) -> std::io::Result<std::process::ExitStatus> {
    super::gentle::program(&mut std::process::Command::new(program)).args(args).stdin(std::process::Stdio::null()).status()
}

const MOST_FILES: u64 = 5000;
const MOST_UNPACKED: u64 = 1 << 30;

const MOST_LISTING: usize = 4 << 20;

fn zip_totals(listing: &str) -> Option<(u64, u64)> {
    let line = listing.lines().rev().find(|l| l.contains("bytes uncompressed"))?;
    let mut numbers = line.split(|c: char| !c.is_ascii_digit()).filter(|w| !w.is_empty()).map(|w| w.parse::<u64>());
    Some((numbers.next()?.ok()?, numbers.next()?.ok()?))
}

fn tar_totals(listing: &str) -> Option<(u64, u64)> {
    let (mut files, mut bytes) = (0u64, 0u64);
    for line in listing.lines() {
        files += 1;
        bytes = bytes.saturating_add(line.split_whitespace().nth(2)?.parse::<u64>().ok()?);
    }
    Some((files, bytes))
}

fn listing(argv: &[&str]) -> Option<String> {
    use std::io::Read;
    let mut command = std::process::Command::new(argv.first()?);
    let mut child = super::run::parsed(super::gentle::program(&mut command).args(&argv[1..]))
        .stdin(std::process::Stdio::null()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::null()).spawn().ok()?;
    let mut out = Vec::new();
    let read = child.stdout.take()?.take(MOST_LISTING as u64 + 1).read_to_end(&mut out);
    let too_much = out.len() > MOST_LISTING;
    if too_much {
        let _ = child.kill();
    }
    let ok = child.wait().is_ok_and(|s| s.success());
    (read.is_ok() && ok && !too_much).then(|| String::from_utf8_lossy(&out).into_owned())
}

fn modest(files: u64, bytes: u64) -> bool {
    files <= MOST_FILES && bytes <= MOST_UNPACKED
}

fn unpack(file: &Path, into: &Path) -> Result<(), String> {
    let name = file.to_string_lossy().to_lowercase();
    let out = into.to_string_lossy().to_string();
    let file = file.to_string_lossy().to_string();
    let zip = name.ends_with(".zip");
    let totals = if zip { listing(&["unzip", "-Zt", &file]).as_deref().and_then(zip_totals) } else { listing(&["tar", "-tvf", &file]).as_deref().and_then(tar_totals) };
    match totals {
        Some((files, bytes)) if modest(files, bytes) => {}
        Some(_) => return Err(t!("common-fonts-too-big").to_string()),
        None => return Err(t!("common-fonts-unpack-failed").to_string()),
    }

    let done = if zip {
        gently("unzip", &["-q", "-o", &file, "-d", &out])
    } else {
        gently("tar", &["-xf", &file, "-C", &out])
    };
    match done {
        Ok(s) if s.success() => Ok(()),
        Ok(_) => Err(t!("common-fonts-unpack-failed").to_string()),
        Err(e) => Err(t!("common-fonts-unpack-error", error = e.to_string())),
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Added {
    pub made: usize,
    pub already: usize,
    pub taken: Vec<Taken>,
}

fn waiting_dir() -> PathBuf {
    data_home().join("fonts-waiting")
}

pub fn add_fonts(file: &Path) -> Result<Added, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static ADDS: AtomicU64 = AtomicU64::new(0);
    let name = file.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let dest = fonts_dir();
    std::fs::create_dir_all(&dest).map_err(|e| e.to_string())?;
    let mut added = Added::default();
    if is_font(&name) {
        let put = super::copies::copy_in(file, &dest, std::ffi::OsStr::new(&name)).map_err(|e| t!("common-pack-not-copied", name = &name, error = e.to_string()).to_string())?;
        count(&mut added, put);
        if added.made > 0 {
            rebuild_font_list();
        }
        return Ok(added);
    }
    if !is_archive(&name) {
        return Err(t!("common-pack-not-fonts", name = &name).to_string());
    }
    let own = format!("{}-{}", std::process::id(), ADDS.fetch_add(1, Ordering::Relaxed));
    let work = waiting_dir().join(format!("unpacking-{own}"));
    let keep = waiting_dir().join(format!("kept-{own}"));
    let _ = std::fs::remove_dir_all(&work);
    std::fs::create_dir_all(&work).map_err(|e| e.to_string())?;
    let outcome = (|| {
        unpack(file, &work)?;
        fonts_in(&work, &dest, &mut added);
        if added.made + added.already + added.taken.len() == 0 {
            return Err(t!("common-fonts-none").to_string());
        }

        for (n, t) in added.taken.iter_mut().enumerate() {
            let at = keep.join(n.to_string());
            std::fs::create_dir_all(&at).map_err(|e| e.to_string())?;
            let to = at.join(&t.name);
            std::fs::rename(&t.from, &to).map_err(|e| e.to_string())?;
            t.from = to;
        }
        if added.made > 0 {
            rebuild_font_list();
        }
        Ok(())
    })();
    let _ = std::fs::remove_dir_all(&work);
    if outcome.is_err() {
        let _ = std::fs::remove_dir_all(&keep);
    }
    outcome.map(|()| added)
}

fn count(added: &mut Added, put: Put) {
    match put {
        Put::Made(_) => added.made += 1,
        Put::Already(_) => added.already += 1,
        Put::Taken(t) => added.taken.push(t),
    }
}

pub fn replace_font(taken: &Taken) -> std::io::Result<()> {
    super::copies::replace(taken)?;
    forget_waiting(taken);
    rebuild_font_list();
    Ok(())
}

pub fn forget_waiting(taken: &Taken) {
    forget_waiting_in(taken, &waiting_dir());
}

fn forget_waiting_in(taken: &Taken, waiting: &Path) {
    if !taken.from.starts_with(waiting) {
        return;
    }
    let _ = std::fs::remove_file(&taken.from);
    let mut at = taken.from.parent();
    while let Some(folder) = at.filter(|f| *f != waiting && f.starts_with(waiting)) {
        if std::fs::remove_dir(folder).is_err() {
            break;
        }
        at = folder.parent();
    }
}

fn fonts_in(work: &Path, dest: &Path, added: &mut Added) {
    let mut stack = vec![work.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            let path = entry.path();
            let Ok(kind) = entry.file_type() else { continue };
            if kind.is_dir() {
                stack.push(path);
                continue;
            }
            if !kind.is_file() {
                continue;
            }
            let Some(name) = path.file_name().map(|n| n.to_string_lossy().to_string()) else { continue };
            if is_font(&name) {
                if let Ok(put) = super::copies::copy_in(&path, dest, std::ffi::OsStr::new(&name)) {
                    count(added, put);
                }
            }
        }
    }
}

fn rebuild_font_list() {
    let _ = text(&["fc-cache", "-f"]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_archive_of_either_kind_is_unpacked_gently() {
        let sh = crate::common::needs::program("sh");
        crate::common::needs::program("nice");
        assert!(gently(&sh.to_string_lossy(), &["-c", "test \"$(nice)\" = 19"]).unwrap().success());

        let code = crate::common::source::code(include_str!("packs.rs")).split_once("#[cfg(test)]").unwrap().0;
        assert_eq!(code.matches("Command::new(").count(), 2, "a program is started outside `gently` and the listing");
        assert_eq!(code.matches("gently(\"").count(), 2);
    }

    #[test]
    fn an_archive_is_measured_before_it_is_unpacked() {
        let zip = "Archive:  x.zip\n-rw-r--r--  3.0 unx    100 tx 40 defX 26-Jan-01 00:00 a.ttf\n2 files, 6000000000 bytes uncompressed, 1000 bytes compressed:  99.9%\n";
        assert_eq!(zip_totals(zip), Some((2, 6_000_000_000)));
        assert_eq!(zip_totals("nothing"), None);
        let tar = "-rw-r--r-- me/me  1234 2026-01-01 00:00 a b.ttf\ndrwxr-xr-x me/me  0 2026-01-01 00:00 dir/\n";
        assert_eq!(tar_totals(tar), Some((2, 1234)));
        assert_eq!(tar_totals("garbled\n"), None);
        assert!(modest(10, 1000) && !modest(MOST_FILES + 1, 1) && !modest(1, MOST_UNPACKED + 1));
    }

    #[test]
    fn a_big_archive_is_refused_and_a_normal_one_is_unpacked() {
        let tar = crate::common::needs::program("tar");
        let dir = std::env::temp_dir().join(format!("trindle-packs-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("src")).unwrap();
        std::fs::write(dir.join("src/Inter.ttf"), "font").unwrap();
        std::fs::write(dir.join("src/INTER-B.TTF"), "font").unwrap();
        std::fs::write(dir.join("src/readme.txt"), "words").unwrap();
        let run = |args: &[&str]| std::process::Command::new(&tar).current_dir(dir.join("src")).args(args).output().unwrap();
        run(&["-cf", "../fonts.tar", "Inter.ttf", "INTER-B.TTF", "readme.txt"]);
        std::fs::create_dir_all(dir.join("out")).unwrap();
        unpack(&dir.join("fonts.tar"), &dir.join("out")).unwrap();
        assert!(dir.join("out/Inter.ttf").is_file() && dir.join("out/INTER-B.TTF").is_file());
        let many: Vec<String> = (0..MOST_FILES + 1).map(|n| format!("f{n}.ttf")).collect();
        for name in &many {
            std::fs::write(dir.join("src").join(name), "").unwrap();
        }
        let mut args = vec!["-cf", "../many.tar"];
        args.extend(many.iter().map(String::as_str));
        run(&args);
        assert!(unpack(&dir.join("many.tar"), &dir.join("out")).is_err());
        assert!(!dir.join("out/f0.ttf").exists(), "nothing of it was unpacked");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_font_that_waited_takes_its_folders_with_it() {
        let waiting = std::env::temp_dir().join(format!("trindle-fonts-waiting-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&waiting);
        let deep = waiting.join("kept-1-0/0");
        std::fs::create_dir_all(&deep).unwrap();
        std::fs::write(deep.join("A.ttf"), "font").unwrap();
        std::fs::create_dir_all(waiting.join("kept-1-1/0")).unwrap();
        let taken = Taken { name: "A.ttf".into(), from: deep.join("A.ttf"), there: PathBuf::new(), new_size: 0, old_size: 0 };
        forget_waiting_in(&taken, &waiting);
        assert!(!waiting.join("kept-1-0").exists(), "its folders went");
        assert!(waiting.join("kept-1-1/0").exists() && waiting.exists(), "another's did not");
        let _ = std::fs::remove_dir_all(&waiting);
    }

    #[test]
    fn only_the_right_kinds_are_taken() {
        assert!(is_archive("Inter.tar.gz"));
        assert!(is_archive("fonts.zip"));
        assert!(!is_archive("icon.png"));
        assert!(is_font("Inter-Regular.ttf"));
        assert!(is_font("Inter.otf"));
        assert!(!is_font("readme.txt"));
    }

    #[test]
    fn a_link_in_an_archive_of_fonts_is_not_followed() {
        let top = std::env::temp_dir().join(format!("trindle-packs-links-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        let (work, dest, elsewhere) = (top.join("work"), top.join("fonts"), top.join("elsewhere"));
        for dir in [&work.join("inside"), &dest, &elsewhere] {
            std::fs::create_dir_all(dir).unwrap();
        }
        std::fs::write(work.join("inside/Own.ttf"), "own").unwrap();
        std::fs::write(elsewhere.join("Other.ttf"), "other").unwrap();
        std::os::unix::fs::symlink(&elsewhere, work.join("folder")).unwrap();
        std::os::unix::fs::symlink(elsewhere.join("Other.ttf"), work.join("Linked.ttf")).unwrap();
        let mut counted = Added::default();
        fonts_in(&work, &dest, &mut counted);
        assert_eq!((counted.made, counted.already, counted.taken.len()), (1, 0, 0));

        fonts_in(&work, &dest, &mut counted);
        assert_eq!((counted.made, counted.already, counted.taken.len()), (1, 1, 0));
        let mut added: Vec<String> = std::fs::read_dir(&dest).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect();
        added.sort();
        assert_eq!(added, ["Own.ttf"]);
        let _ = std::fs::remove_dir_all(&top);
    }
}
