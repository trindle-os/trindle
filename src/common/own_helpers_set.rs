//! Changing the marks `own_helpers` reads. Only Settings changes them, and the
//! compositor's tests, so the compositor proper never carries this.

use std::fmt::Write as _;
use super::files;
use super::own_helpers::{file, words, Helper, DEVICE, MOST};

pub fn switch(helper: Helper, on: bool, own: bool) -> std::io::Result<()> {
    let Some(file) = file() else {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "no file to keep it in"));
    };
    let on = on && own;
    let mut all = words(&file);
    if all.iter().any(|w| w == helper.word()) == on {
        return Ok(());
    }
    all.retain(|w| w != helper.word());
    if on {
        all.push(helper.word().to_string());
    }
    write(&file, &all)
}

fn write(file: &std::path::Path, lines: &[String]) -> std::io::Result<()> {
    if std::fs::symlink_metadata(file).is_ok_and(|m| !m.is_file()) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "not a file"));
    }
    let text: String = lines.iter().fold(String::new(), |mut s, w| { let _ = writeln!(s, "{w}"); s });
    if text.len() as u64 > MOST {
        return Err(std::io::Error::new(std::io::ErrorKind::StorageFull, "too many to keep"));
    }
    files::write_whole_private(file, text.as_bytes())
}

pub fn allow_device(name: &str, on: bool, own: bool) -> std::io::Result<()> {
    if !own {
        return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "not the person's own press"));
    }
    let Some(file) = file() else {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "no file to keep it in"));
    };
    let name: String = name.chars().filter(|c| !c.is_control()).collect();
    let line = format!("{DEVICE}{}", name.trim());
    let mut all = words(&file);
    if all.contains(&line) == on {
        return Ok(());
    }
    all.retain(|w| *w != line);
    if on {
        all.push(line);
    }
    write(&file, &all)
}

#[cfg(test)]
mod tests {
    #[test]
    fn marks_past_the_most_that_is_read_back_are_not_written_by_either_way() {
        let dir = std::env::temp_dir().join(format!("trindle-own-helpers-set-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("marks");
        let many: Vec<String> = (0..2000).map(|n| format!("word-{n}")).collect();
        assert_eq!(super::write(&file, &many).unwrap_err().kind(), std::io::ErrorKind::StorageFull);
        assert!(!file.exists());
        super::write(&file, &many[..3]).unwrap();
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "word-0\nword-1\nword-2\n");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
