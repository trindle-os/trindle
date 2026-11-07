//! The helpers a person presses through, and whether they switched each on themselves.

use std::path::{Path, PathBuf};

use crate::common::files;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Helper {
    Reader,
    Keyboard,
    Clicker,
}

impl Helper {
    pub(crate) const fn word(self) -> &'static str {
        match self {
            Helper::Reader => "screen-reader",
            Helper::Keyboard => "on-screen-keyboard",
            Helper::Clicker => "clicking-helper",
        }
    }
}

pub const OWN_KEY: &str = "TRINDLE_OWN_KEY";

pub(crate) const MOST: u64 = 4096;

#[cfg(test)]
thread_local! {
    static AT: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
pub fn kept_at<R>(file: &Path, during: impl FnOnce() -> R) -> R {
    let before = AT.with(|at| at.replace(Some(file.to_path_buf())));
    let out = during();
    AT.with(|at| *at.borrow_mut() = before);
    out
}

#[cfg(test)]
pub(crate) fn file() -> Option<PathBuf> {
    AT.with(|at| at.borrow().clone())
}

#[cfg(not(test))]
pub(crate) fn file() -> Option<PathBuf> {
    Some(files::home().join(".local/state/trindle/own-helpers"))
}

pub fn on(helper: Helper) -> bool {
    file().is_some_and(|file| words(&file).iter().any(|w| w == helper.word()))
}

pub(crate) const DEVICE: &str = "device ";

pub fn allowed_devices() -> Vec<String> {
    file().map(|file| words(&file).iter().filter_map(|w| w.strip_prefix(DEVICE)).map(String::from).collect()).unwrap_or_default()
}

pub(crate) fn words(file: &Path) -> Vec<String> {
    use std::os::unix::fs::MetadataExt;
    if !std::fs::symlink_metadata(file).is_ok_and(|m| m.is_file()) {
        return Vec::new();
    }
    let Ok(opened) = files::open_file(file) else { return Vec::new() };
    let Ok(meta) = opened.metadata() else { return Vec::new() };
    // SAFETY: asks the kernel for this process's own user id. It reads
    // nothing and cannot fail.
    let me = unsafe { libc::getuid() };
    if !meta.is_file() || meta.uid() != me || meta.mode() & 0o022 != 0 || meta.len() > MOST {
        return Vec::new();
    }
    let mut text = String::new();
    if std::io::Read::read_to_string(&mut std::io::Read::take(opened, MOST), &mut text).is_err() {
        return Vec::new();
    }
    text.lines().map(str::trim).filter(|l| !l.is_empty()).map(String::from).collect()
}

#[cfg(test)]
pub const WRITTEN_IN: [(&str, &str); 4] = [
    ("common/screen_reader.rs", "own_helpers_set::switch(Helper::Reader, on, own)"),
    ("settings/panels/keyboard/changed_keys.rs", "own_helpers_set::allow_device(&name, on, own)"),
    ("settings/panels/keyboard/devices.rs", "own_helpers_set::switch(Helper::Keyboard, on, own)"),
    ("settings/panels/keyboard/devices.rs", "own_helpers_set::switch(Helper::Clicker, on, own)"),
];

#[cfg(test)]
mod tests {
    use super::super::own_helpers_set::{allow_device, switch};
    use super::*;

    fn folder(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-own-helpers-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn a_helper_is_on_only_once_the_persons_own_press_switched_it_on() {
        let dir = folder("switch");
        let file = dir.join("state/own-helpers");
        kept_at(&file, || {
            assert!(!on(Helper::Reader) && !on(Helper::Keyboard), "a helper was on with nothing written");
            switch(Helper::Reader, true, false).unwrap();
            assert!(!on(Helper::Reader), "a press that was not the person's switched the screen reader on");
            switch(Helper::Reader, true, true).unwrap();
            assert!(on(Helper::Reader) && !on(Helper::Keyboard));
            switch(Helper::Keyboard, true, true).unwrap();
            assert!(on(Helper::Reader) && on(Helper::Keyboard));
            switch(Helper::Keyboard, true, false).unwrap();
            assert!(on(Helper::Reader) && !on(Helper::Keyboard), "a helper stayed on for a press that was not the person's");
            switch(Helper::Reader, false, false).unwrap();
            assert!(!on(Helper::Reader));
            use std::os::unix::fs::PermissionsExt;
            assert_eq!(std::fs::metadata(&file).unwrap().permissions().mode() & 0o777, 0o600);
        });
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn only_an_ordinary_file_nobody_else_may_write_reached_through_no_link_counts() {
        use std::os::unix::fs::PermissionsExt;
        let dir = folder("file");
        let file = dir.join("own-helpers");
        let write = |mode: u32| {
            std::fs::write(&file, "screen-reader\non-screen-keyboard\n").unwrap();
            std::fs::set_permissions(&file, std::fs::Permissions::from_mode(mode)).unwrap();
        };
        kept_at(&file, || {
            write(0o600);
            assert!(on(Helper::Reader) && on(Helper::Keyboard));
            for mode in [0o620, 0o602, 0o666] {
                write(mode);
                assert!(!on(Helper::Reader) && !on(Helper::Keyboard), "a file others may write ({mode:o}) counted");
            }
            write(0o600);
            let real = dir.join("real");
            std::fs::rename(&file, &real).unwrap();
            std::os::unix::fs::symlink(&real, &file).unwrap();
            assert!(!on(Helper::Reader), "a link counted");
            assert!(switch(Helper::Reader, true, true).is_err(), "the mark was written through a link");
            assert_eq!(std::fs::read_to_string(&real).unwrap(), "screen-reader\non-screen-keyboard\n");
            std::fs::remove_file(&file).unwrap();
            std::fs::create_dir(&file).unwrap();
            assert!(!on(Helper::Reader));
            std::fs::remove_dir(&file).unwrap();
            std::fs::write(&file, format!("screen-reader\n{}", "x".repeat(5000))).unwrap();
            std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600)).unwrap();
            assert!(!on(Helper::Reader), "a file too long to be the mark counted");
            std::fs::write(&file, "screen-reader-not\nscreen\n").unwrap();
            assert!(!on(Helper::Reader));
        });
        assert!(!on(Helper::Reader) && switch(Helper::Reader, true, true).is_err());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_device_counts_as_the_persons_only_once_their_own_press_allowed_it() {
        let dir = folder("devices");
        let file = dir.join("own-helpers");
        kept_at(&file, || {
            assert!(allowed_devices().is_empty());
            assert!(allow_device("keyd virtual keyboard", true, false).is_err(), "a press not the person's allowed a device");
            assert!(allowed_devices().is_empty());
            allow_device("keyd virtual keyboard", true, true).unwrap();
            switch(Helper::Reader, true, true).unwrap();
            assert_eq!(allowed_devices(), ["keyd virtual keyboard"]);
            assert!(on(Helper::Reader), "allowing a device lost a helper's mark");
            allow_device("ydotoold virtual device", true, true).unwrap();
            assert!(allow_device("keyd virtual keyboard", false, false).is_err());
            assert_eq!(allowed_devices().len(), 2);
            allow_device("keyd virtual keyboard", false, true).unwrap();
            assert_eq!(allowed_devices(), ["ydotoold virtual device"]);
            allow_device("screen-reader\nscreen-reader", true, true).unwrap();
            switch(Helper::Reader, false, false).unwrap();
            assert!(!on(Helper::Reader));
        });
        let _ = std::fs::remove_dir_all(dir);
    }

    fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for e in entries.flatten() {
            let p = e.path();
            if p.is_dir() {
                sources(&p, out);
            } else if p.extension().is_some_and(|x| x == "rs") {
                out.push(p);
            }
        }
    }

    #[test]
    fn the_mark_is_written_only_where_the_press_was_judged() {
        let mut all: Vec<(String, PathBuf)> = Vec::new();
        for (src, prefix) in trindle_toolkit::common::source::trees() {
            let mut found = Vec::new();
            sources(&src, &mut found);
            all.extend(found.into_iter().map(|f| (format!("{prefix}{}", f.strip_prefix(&src).unwrap().display()), f)));
        }
        all.sort();
        assert!(all.len() > 100, "the source was not found");
        let (write, said, heard) = (["own_helpers_set::", "switch("].concat(), ["crate::own_helpers::", "OWN_KEY"].concat(), ["common::own_helpers::", "OWN_KEY"].concat());
        let allow = ["own_helpers_set::", "allow_device("].concat();
        let (mut writes, mut says, mut hears) = (Vec::new(), Vec::new(), Vec::new());
        for (name, file) in all {
            if name == "common/own_helpers.rs" {
                continue;
            }
            let text = std::fs::read_to_string(&file).unwrap();
            for (word, found) in [(&write, &mut writes), (&said, &mut says), (&heard, &mut hears)] {
                found.extend(std::iter::repeat_n(name.clone(), text.matches(word.as_str()).count()));
            }
            writes.extend(std::iter::repeat_n(name.clone(), text.matches(allow.as_str()).count()));
        }
        let listed: Vec<String> = WRITTEN_IN.iter().map(|w| w.0.to_string()).collect();
        assert_eq!(writes, listed, "the mark is written somewhere the list does not have, or no longer where it has");
        for (file, words) in WRITTEN_IN {
            assert!(std::fs::read_to_string(trindle_toolkit::common::source::path_of(file)).unwrap().contains(words), "{file} no longer writes the mark as {words}");
        }
        assert_eq!(says, ["compose/wm.rs", "compose/wm.rs"], "the compositor's word is said, or taken away, somewhere else");
        assert_eq!(hears, ["settings/mod.rs"], "the compositor's word is heard somewhere else");
    }
}
