//! A language of the person's own.

use std::path::{Path, PathBuf};

use super::{tag_ok, unfit, Why};
use crate::common::language_names::Tongue;
use crate::toolkit::{lang, t};

const NAME_FILE: &str = "name.txt";
const NAME_FILE_MOST: u64 = 256;
const NAME_MOST: usize = 30;
const NAME_NOT: &[char] = &['<', '>', '&', '/', '\\', '`', '$', '{', '}', '~', '|', '"', ':', ';', '='];
const STEM_MOST: usize = 8;

pub fn is_own(tag: &str) -> bool {
    tag.get(..2).is_some_and(|p| p.eq_ignore_ascii_case("x-"))
}

pub fn name_ok(name: &str) -> Result<String, Why> {
    let name = name.trim();
    if !(1..=NAME_MOST).contains(&name.chars().count()) {
        return Err(Why::NameLength);
    }
    if unfit(name, &[]) || name.contains(['\u{fffd}', '\n']) || name.contains(NAME_NOT) || name.contains("..") {
        return Err(Why::NamePlain);
    }
    Ok(name.to_string())
}

pub fn name_of(base: &Path, tag: &str) -> Option<String> {
    use std::io::Read;
    use std::os::unix::fs::OpenOptionsExt;
    if !tag_ok(tag) {
        return None;
    }
    let file = std::fs::OpenOptions::new().read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC).open(base.join(tag).join(NAME_FILE)).ok()?;
    if !file.metadata().ok()?.is_file() {
        return None;
    }
    let mut bytes = Vec::new();
    file.take(NAME_FILE_MOST + 1).read_to_end(&mut bytes).ok()?;
    if bytes.len() as u64 > NAME_FILE_MOST {
        return None;
    }
    name_ok(std::str::from_utf8(&bytes).ok()?).ok()
}

pub fn languages(base: &Path) -> Vec<(String, String)> {
    let Ok(read) = std::fs::read_dir(base) else { return Vec::new() };
    let mut out: Vec<(String, String)> = read.flatten()
        .filter_map(|e| e.file_name().to_str().map(str::to_string).filter(|n| is_own(n) && tag_ok(n) && e.path().is_dir()))
        .map(|tag| (tag.clone(), name_of(base, &tag).unwrap_or(tag)))
        .collect();
    out.sort_by_key(|(t, n)| (n.to_lowercase(), t.clone()));
    out
}

fn taken(dirs: &[PathBuf], tag: &str) -> bool {
    dirs.iter().filter_map(|d| std::fs::read_dir(d).ok()).flatten().flatten()
        .any(|e| e.file_name().to_str().is_some_and(|n| n.eq_ignore_ascii_case(tag)))
}

pub fn tag_for(name: &str, dirs: &[PathBuf]) -> Option<String> {
    let stem: String = name.chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_lowercase()).collect();
    let stem = if stem.is_empty() { "own".to_string() } else { stem };
    (1..1000).find_map(|n| {
        let num = if n == 1 { String::new() } else { n.to_string() };
        let cut: String = stem.chars().take(STEM_MOST - num.len()).collect();
        let tag = format!("x-{cut}{num}");
        (tag_ok(&tag) && lang::tags_of(&tag).first() == Some(&tag) && !taken(dirs, &tag)).then_some(tag)
    })
}

pub fn start(base: &Path, name: &str, dirs: &[PathBuf]) -> Result<String, Why> {
    let name = name_ok(name)?;
    if languages(base).iter().any(|(_, n)| n.to_lowercase() == name.to_lowercase()) {
        return Err(Why::NameTaken);
    }
    let tag = tag_for(&name, dirs).ok_or(Why::NotSaved)?;
    std::fs::create_dir_all(base).map_err(|_| Why::NotSaved)?;
    let dir = base.join(&tag);
    std::fs::create_dir(&dir).map_err(|_| Why::NotSaved)?;
    crate::common::files::write_whole_private(&dir.join(NAME_FILE), name.as_bytes()).map_err(|_| Why::NotSaved)?;
    Ok(tag)
}

pub fn remove(base: &Path, tag: &str) -> std::io::Result<()> {
    if !is_own(tag) || !tag_ok(tag) {
        return Err(std::io::Error::other("language tag"));
    }
    let (dir, bak) = (base.join(tag), base.join(format!("{tag}.bak")));
    if std::fs::symlink_metadata(&dir).is_err() {
        return Ok(());
    }
    match std::fs::symlink_metadata(&bak) {
        Ok(m) if m.is_dir() => std::fs::remove_dir_all(&bak)?,
        Ok(_) => std::fs::remove_file(&bak)?,
        Err(_) => {}
    }
    std::fs::rename(dir, bak)
}

pub fn with_own(found: Vec<Tongue>, base: &Path) -> Vec<Tongue> {
    let all = found.iter().map(|t| t.all).max().unwrap_or(0);
    let (mut real, mut mine): (Vec<Tongue>, Vec<Tongue>) = found.into_iter().partition(|t| !is_own(&t.code));
    for (tag, _) in languages(base) {
        if !mine.iter().any(|t| t.code == tag) {
            mine.push(Tongue { code: tag, name: String::new(), done: 0, all, machine: false });
        }
    }
    for t in &mut mine {
        t.machine = false;
        t.name = label(&name_of(base, &t.code).unwrap_or_else(|| t.code.clone()));
    }
    mine.sort_by_key(|t| (t.name.to_lowercase(), t.code.clone()));
    real.extend(mine);
    real
}

pub fn label(name: &str) -> String {
    t!("settings-words-own-label", name = name)
}
