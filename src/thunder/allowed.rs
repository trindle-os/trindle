//! The devices the person said may always connect, kept by the guard in a
//! file only root can read, `/var/lib/trindle-thunder/allowed`. One line a
//! device, its fields parted by tabs: its unique id, its key at the secure
//! level ("-" when it has none), its maker and its name, the last two only
//! for showing in Settings.

use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use super::sysfs::{clean, sane_id};

pub const FILE: &str = "allowed";

#[derive(Clone, Debug, PartialEq)]
pub struct Allowed {
    pub id: String,
    pub key: Option<String>,
    pub vendor: String,
    pub name: String,
}

pub fn load(state: &Path) -> Vec<Allowed> {
    let Ok(text) = fs::read_to_string(state.join(FILE)) else { return Vec::new() };
    text.lines().filter_map(|l| {
        let mut f = l.split('\t');
        let id = f.next()?.to_string();
        let key = f.next()?;
        let key = (key != "-" && key.len() == 64 && key.chars().all(|c| c.is_ascii_hexdigit())).then(|| key.to_string());
        let vendor = clean(f.next().unwrap_or(""));
        let name = clean(f.next().unwrap_or(""));
        sane_id(&id).then_some(Allowed { id, key, vendor, name })
    }).collect()
}

pub fn save(state: &Path, all: &[Allowed]) -> io::Result<()> {
    fs::create_dir_all(state)?;
    let mut text = String::new();
    for a in all {
        let key = a.key.as_deref().unwrap_or("-");
        text.push_str(&format!("{}\t{key}\t{}\t{}\n", a.id, clean(&a.vendor), clean(&a.name)));
    }
    let new = state.join(format!(".{FILE}.new"));
    let _ = fs::remove_file(&new);
    let mut f = fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open(&new)?;
    f.write_all(text.as_bytes())?;
    f.sync_all()?;
    fs::rename(&new, state.join(FILE))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn the_list_keeps_and_reads_back() {
        let t = crate::common::files::private_temp("thunder-allowed").unwrap();
        let key = "ab".repeat(32);
        let all = vec![
            Allowed { id: "aaaa-1".into(), key: None, vendor: "Dock Makers".into(), name: "Big Dock".into() },
            Allowed { id: "bbbb-2".into(), key: Some(key.clone()), vendor: "Disk\tCo".into(), name: "Disk".into() },
        ];
        save(&t, &all).unwrap();
        let back = load(&t);
        assert_eq!(back.len(), 2);
        assert_eq!(back[1].key.as_deref(), Some(key.as_str()));
        assert_eq!(back[1].vendor, "Disk Co");
        let mode = fs::metadata(t.join(FILE)).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "keys are for root's eyes only");

        fs::write(t.join(FILE), "not\tan id!\t-\tx\ty\ncccc-3\t-\tA\tB\n").unwrap();
        assert_eq!(load(&t).iter().map(|a| a.id.as_str()).collect::<Vec<_>>(), ["cccc-3"]);
        let _ = fs::remove_dir_all(&t);
    }
}
