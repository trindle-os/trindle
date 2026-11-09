//! Every look the chooser takes at the disk, each on a thread of its own.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use crate::common::files::{config_home, data_home};
use crate::common::kinds::{recent, shown_name};
use crate::toolkit::ui::App;

use super::{Chooser, Entry, Found, LATE, Msg, Place, Waited, Where};
use super::entries::sort;
use super::kept::folders_read;
use super::places::{bookmarks, drives, first_places, once_each, usual};

fn begun(named: Option<PathBuf>, kept: &Path, app_id: &str, home: &Path) -> PathBuf {
    named.filter(|f| f.is_dir()).or_else(|| last_folder(kept, app_id).filter(|f| f.is_dir())).unwrap_or_else(|| home.to_path_buf())
}

fn named_places(home: &Path) -> Vec<Place> {
    let mut places = Vec::new();
    let dirs = std::fs::read_to_string(config_home().join("user-dirs.dirs")).unwrap_or_default();
    for (icon, path) in usual(&dirs, home) {
        places.push(Place { label: shown_name(&path), icon, folder: true, at: Where::Folder(path) });
    }
    let marks = std::fs::read_to_string(config_home().join("gtk-3.0/bookmarks")).unwrap_or_default();
    for (label, path) in bookmarks(&marks) {
        places.push(Place { label, icon: "folder-symbolic", folder: true, at: Where::Folder(path) });
    }
    for path in drives(&std::fs::read_to_string("/proc/self/mounts").unwrap_or_default()) {
        places.push(Place { label: shown_name(&path), icon: "drive-removable-media-symbolic", folder: false, at: Where::Folder(path) });
    }
    places
}

fn place_looked_at(rank: usize, place: Place) -> Msg {
    match &place.at {
        Where::Folder(f) if f.is_dir() => Msg::Checked(rank, place),
        _ => Msg::Checked(usize::MAX, place),
    }
}

fn entry_of(path: PathBuf) -> Option<Entry> {
    let name = shown_name(&path);
    let meta = std::fs::metadata(&path).or_else(|_| std::fs::symlink_metadata(&path)).ok()?;
    let when = meta.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map_or(0, |d| d.as_secs() as i64);
    Some(Entry { name, path, folder: meta.is_dir(), size: meta.len(), when })
}

fn read_place(at: &Where, hidden: bool) -> Vec<Entry> {
    match at {
        Where::Recent => {
            let text = std::fs::read_to_string(data_home().join("recently-used.xbel")).unwrap_or_default();
            recent(&text).into_iter().filter(|p| p.exists()).take(50).filter_map(entry_of).collect()
        }
        Where::Folder(f) => std::fs::read_dir(f).map(|rd| {
            rd.flatten().filter(|d| hidden || !d.file_name().as_encoded_bytes().starts_with(b"."))
                .filter(|d| !d.file_name().as_encoded_bytes().ends_with(crate::common::files::SPARE_END.as_bytes()))
                .filter_map(|d| entry_of(d.path())).collect()
        }).unwrap_or_default(),
    }
}

fn listed_look(ear: Option<std::os::fd::OwnedFd>, at: Where, hidden: bool) -> Msg {
    let mark = watch_added(ear, &at);
    Msg::Listed(read_place(&at, hidden), mark, at, hidden)
}

fn watch_added(ear: Option<std::os::fd::OwnedFd>, at: &Where) -> Option<i32> {
    match at {
        Where::Folder(f) => crate::toolkit::watch::add(&ear?, f).ok(),
        Where::Recent => None,
    }
}

fn is_taken(target: &Path) -> bool {
    std::fs::symlink_metadata(target).is_ok() && !target.is_dir()
}

fn target_found(target: &Path) -> Found {
    Found { folder: target.is_dir(), parent: target.parent().is_some_and(|p| p.is_dir()), there: std::fs::symlink_metadata(target).is_ok() }
}

fn unique_all(into: &Path, names: &[String]) -> Vec<PathBuf> {
    let mut given = HashSet::new();
    names.iter().map(|n| {
        let p = unique(into, n, &given);
        given.insert(p.clone());
        p
    }).collect()
}

fn unique(folder: &Path, name: &str, given: &HashSet<PathBuf>) -> PathBuf {
    let first = folder.join(name);
    if !first.exists() && !given.contains(&first) {
        return first;
    }
    let (stem, ext) = match name.rfind('.') {
        Some(at) if at > 0 => (&name[..at], &name[at..]),
        _ => (name, ""),
    };
    (2..).map(|n| folder.join(format!("{stem} ({n}){ext}")))
        .find(|p| !p.exists() && !given.contains(p))
        .unwrap_or(first)
}

fn last_folder(file: &Path, app_id: &str) -> Option<PathBuf> {
    let text = folders_read(file);
    text.lines().find_map(|l| l.split_once('\t').filter(|(a, _)| *a == app_id).map(|(_, p)| PathBuf::from(p)))
}

impl Chooser {

    pub(super) fn aside(&mut self, look: impl FnOnce() -> Msg + Send + 'static) {
        match self.post.clone() {
            Some(post) => {
                let _ = std::thread::Builder::new().name("chooser-look".into()).spawn(move || {
                    post.send(look());
                });
            }
            None => {
                let found = look();
                self.update(found);
            }
        }
    }

    pub(super) fn begin(&mut self) {
        if std::mem::replace(&mut self.began, true) {
            return;
        }
        let (named, kept, app_id, home) = (self.ask.folder.clone(), self.kept.clone(), self.ask.app_id.clone(), self.home.clone());

        self.start_out = true;
        self.wait_for(Waited::Start);
        self.aside(move || Msg::Began(begun(named, &kept, &app_id, &home)));
        let home = self.home.clone();
        self.aside(move || Msg::Candidates(named_places(&home)));
    }

    pub(super) fn has_begun(&mut self, start: PathBuf) {
        self.start_out = false;
        if !self.moved {
            self.late_place = false;
            self.at = Where::Folder(start);
            self.list();
        }
    }

    pub(super) fn candidates(&mut self, named: Vec<Place>) {
        for (rank, place) in named.into_iter().enumerate() {
            if !place.folder {
                self.found.push((rank, place));
                continue;
            }
            self.aside(move || place_looked_at(rank, place));
        }
        self.show_places();
    }

    pub(super) fn checked(&mut self, rank: usize, place: Place) {
        if rank != usize::MAX {
            self.found.push((rank, place));
            self.show_places();
        }
    }

    fn show_places(&mut self) {
        let mut all = first_places(&self.home);
        let computer = all.pop();
        let mut found = self.found.clone();
        found.sort_by_key(|(rank, _)| *rank);
        all.extend(found.into_iter().map(|(_, place)| place));
        all.extend(computer);
        self.places = once_each(all);
    }

    pub(super) fn list(&mut self) {
        let (at, hidden) = (self.at.clone(), self.hidden);
        if self.looks.contains(&(at.clone(), hidden)) {
            self.again = true;
            return;
        }
        self.looks.push((at.clone(), hidden));

        let unwatched = matches!(&at, Where::Folder(f) if !self.watched.contains(f));
        let ear = self.watching.as_ref().filter(|_| unwatched).and_then(|w| w.lend());
        self.wait_for(Waited::Place(at.clone(), hidden));

        self.aside(move || listed_look(ear, at, hidden));
    }

    fn wait_for(&mut self, waited: Waited) {
        if let Some(post) = self.post.clone() {
            let _ = std::thread::Builder::new().name("chooser-wait".into()).spawn(move || {
                std::thread::sleep(LATE);
                post.send(Msg::Late(waited));
            });
        }
    }

    pub(super) fn late(&mut self, waited: Waited) {
        match waited {
            Waited::Place(at, hidden) => {
                if (&at, hidden) == (&self.at, self.hidden) && self.looks.contains(&(at, hidden)) {
                    self.late_place = true;
                }
            }
            Waited::Start => {
                if self.start_out && !self.moved {
                    self.late_place = true;
                }
            }
            Waited::Save(target) => {
                if self.accepting.contains(&target) {
                    self.late_save = true;
                }
            }
        }
    }

    pub(super) fn listed(&mut self, mut entries: Vec<Entry>, mark: Option<i32>, at: Where, hidden: bool) {
        self.looks.retain(|look| *look != (at.clone(), hidden));
        if let (Where::Folder(f), Some(mark), Some(w)) = (&at, mark, &mut self.watching) {
            w.known(mark, f);
            self.watched.insert(f.clone());
        }
        if (&at, hidden) != (&self.at, self.hidden) {
            return;
        }
        self.late_place = false;
        let keep: Vec<PathBuf> = self.picked.iter().filter_map(|&i| self.entries.get(i)).map(|e| e.path.clone()).collect();
        let chosen = self.chosen.and_then(|i| self.entries.get(i)).map(|e| e.path.clone());
        entries.retain(|e| self.shows(e));
        if let Where::Folder(_) = self.at {
            sort(&mut entries);
        }
        self.entries = entries;
        self.listed = true;
        self.picked = self.entries.iter().enumerate().filter(|(_, e)| keep.contains(&e.path)).map(|(i, _)| i).collect();
        self.chosen = chosen.and_then(|c| self.entries.iter().position(|e| e.path == c));
        if let Some(next) = self.pick_next.take() {
            self.chosen = self.entries.iter().position(|e| e.path == next);
            self.picked = self.chosen.into_iter().collect();
        }

        self.seen_old = true;
        if std::mem::take(&mut self.again) {
            self.list();
        }
    }

    pub(super) fn look_taken(&mut self) {
        let Some(target) = self.target() else { return };
        let Some(folder) = target.parent().map(Path::to_path_buf) else { return };
        let known = !self.seen_old && self.seen.as_ref().is_some_and(|(seen, _)| *seen == target);
        if known || self.seen_out.contains(&folder) {
            return;
        }
        self.seen_old = false;
        self.seen_out.push(folder);
        self.aside(move || Msg::Seen(is_taken(&target), target));
    }

    pub(super) fn seen_taken(&mut self, taken: bool, target: PathBuf) {
        self.seen_out.retain(|folder| Some(folder.as_path()) != target.parent());
        self.seen = Some((target, taken));
    }

    pub(super) fn look_for_free_names(&mut self, into: PathBuf, names: Vec<String>) {
        if self.accepting.contains(&into) {
            return;
        }
        self.accepting.push(into.clone());
        self.wait_for(Waited::Save(into.clone()));
        self.aside(move || Msg::Named(unique_all(&into, &names), into));
    }

    pub(super) fn look_at_target(&mut self, pressed: bool) {
        let Some(target) = self.target() else { return };
        if self.accepting.contains(&target) {
            return;
        }
        self.accepting.push(target.clone());
        self.wait_for(Waited::Save(target.clone()));
        self.aside(move || Msg::Target(target_found(&target), target, pressed));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_taken_name_gets_a_number() {
        let dir = std::env::temp_dir().join(format!("trindle-chooser-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("photo.png"), b"").unwrap();
        let mut given = HashSet::new();
        let one = unique(&dir, "photo.png", &given);
        assert_eq!(one, dir.join("photo (2).png"));
        given.insert(one);
        assert_eq!(unique(&dir, "photo.png", &given), dir.join("photo (3).png"));
        assert_eq!(unique(&dir, "notes", &given), dir.join("notes"));
        std::fs::write(dir.join(".bashrc"), b"").unwrap();
        assert_eq!(unique(&dir, ".bashrc", &given), dir.join(".bashrc (2)"));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
