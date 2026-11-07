//! Fonts people add themselves.

use std::path::{Path, PathBuf};

use super::copies::{Put, Taken};
use super::files::data_home;
use super::run::text;

pub fn fonts_dir() -> PathBuf {
    data_home().join("fonts")
}

pub fn is_archive(name: &str) -> bool {
    let n = name.to_lowercase();
    [".zip", ".tar.gz", ".tgz", ".tar.xz", ".tar.bz2", ".tar"].iter().any(|e| n.ends_with(e))
}

pub fn is_font(name: &str) -> bool {
    let n = name.to_lowercase();
    [".ttf", ".otf", ".ttc", ".woff2"].iter().any(|e| n.ends_with(e))
}

fn gently(program: &str, args: &[&str]) -> std::io::Result<std::process::ExitStatus> {
    super::gentle::program(&mut std::process::Command::new(program)).args(args).status()
}

fn unpack(file: &Path, into: &Path) -> Result<(), String> {
    let name = file.to_string_lossy().to_lowercase();
    let out = into.to_string_lossy().to_string();
    let file = file.to_string_lossy().to_string();
    let done = if name.ends_with(".zip") {
        gently("unzip", &["-q", "-o", &file, "-d", &out])
    } else {
        gently("tar", &["-xf", &file, "-C", &out])
    };
    match done {
        Ok(s) if s.success() => Ok(()),
        Ok(_) => Err("The file could not be unpacked. It may be damaged, or not the kind it claims.".into()),
        Err(e) => Err(format!("The file could not be unpacked: {e}")),
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
        let put = super::copies::copy_in(file, &dest, std::ffi::OsStr::new(&name)).map_err(|e| format!("{name} could not be copied: {e}"))?;
        count(&mut added, put);
        if added.made > 0 {
            rebuild_font_list();
        }
        return Ok(added);
    }
    if !is_archive(&name) {
        return Err(format!("{name} is not a font or an archive of fonts."));
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
            return Err("There are no fonts in that file. See Help, adding fonts.".to_string());
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
    if taken.from.starts_with(waiting_dir()) {
        let _ = std::fs::remove_file(&taken.from);
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
        assert_eq!(code.matches("Command::new(").count(), 1, "a program is started outside `gently`");
        assert_eq!(code.matches("gently(\"").count(), 2);
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
