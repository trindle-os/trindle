//! Putting a package in its own folder with the parts it needs, and taking it off again.

use crate::toolkit::t;
use std::collections::{HashSet, VecDeque};
use std::path::{Path, PathBuf};
use crate::store::parts;
use super::added::{ADDED};
use super::launchers::{launchers_of, put_launchers, take_launchers};
use super::needs::{answered, debs_in, find_part, work_out_wants};
use super::{MOST_PARTS, asked, folder, write_lines};

pub struct Installed {
    pub name: String,

    pub missing: Vec<String>,

    pub ask: bool,
}

pub fn relink(at: &Path, keys: &[String]) -> Result<(), String> {
    let fresh = at.join("files.new");
    let _ = std::fs::remove_dir_all(&fresh);
    std::fs::create_dir_all(&fresh).map_err(|e| e.to_string())?;
    for key in keys {
        parts::link_tree(&parts::tree(key), &fresh).map_err(|e| format!("{key}: {e}"))?;
    }
    let files = at.join("files");
    let _ = std::fs::remove_dir_all(&files);
    std::fs::rename(&fresh, &files).map_err(|e| e.to_string())?;
    write_lines(&at.join("parts"), keys).map_err(|e| e.to_string())?;
    let carried: Vec<String> = keys.iter().map(|k| {
        let c = parts::control(k);
        format!("{} {} {}", parts::field_of(&c, "Package"), parts::field_of(&c, "Version"), parts::field_of(&c, "Architecture"))
    }).collect();
    write_lines(&at.join("packages"), &carried).map_err(|e| e.to_string())
}

pub fn install(deb: &Path, heard: impl Fn(&str)) -> Result<Installed, String> {
    let done = install_with(deb, None, heard)?;
    // The person opened this file themselves, so it is theirs: see `added`.
    let _ = std::fs::write(folder(&done.name).join(ADDED), "");
    Ok(done)
}

pub fn install_with(deb: &Path, local: Option<&Path>, heard: impl Fn(&str)) -> Result<Installed, String> {
    let control = parts::control_of(deb);
    let name = parts::field_of(&control, "Package");
    if name.is_empty() || name.contains('/') || name.starts_with('.') {
        return Err(t!("store-boxed-not-deb").into());
    }
    let at = folder(&name);
    let was_there = at.join("version").is_file();
    std::fs::create_dir_all(at.join("home")).map_err(|e| e.to_string())?;
    let attempt = Attempt { name: &name, control: &control, pending: at.join("parts.new"), at, was_there };
    heard(&t!("store-boxed-unpacking", name = &name));
    let main = parts::make(deb)?;
    let mut found = vec![main.clone()];
    let _ = write_lines(&attempt.pending, &found);
    if parts::cannot_box(&parts::walk(&parts::tree(&main))) {
        return attempt.give_up(format!("{name} is a driver or a background service. Those cannot work from their own \
                                folder, so it was not installed."));
    }
    let missing = attempt.gather_needs(local, &mut found, &heard);
    attempt.finish(&main, &found, missing)
}

struct Attempt<'a> {
    name: &'a str,
    control: &'a str,
    at: PathBuf,
    pending: PathBuf,
    was_there: bool,
}

struct Needs {
    have: HashSet<String>,
    waiting: VecDeque<(Vec<String>, bool)>,
    missing: Vec<String>,
}

impl Needs {

    fn miss(&mut self, group: &[String], optional: bool) {
        if !optional {
            self.missing.push(group.join(" or "));
        }
    }
}

impl Attempt<'_> {

    fn give_up(&self, why: String) -> Result<Installed, String> {
        let _ = std::fs::remove_file(&self.pending);
        if !self.was_there {
            let _ = std::fs::remove_dir_all(&self.at);
        }
        parts::sweep();
        Err(why)
    }

    fn gather_needs(&self, local: Option<&Path>, found: &mut Vec<String>, heard: &impl Fn(&str)) -> Vec<String> {
        let local: Vec<(PathBuf, String)> = local.map(debs_in).unwrap_or_default().into_iter()
            .map(|d| { let c = parts::control_of(&d); (d, c) }).collect();
        let downloads = self.at.join("downloads");
        let _ = std::fs::remove_dir_all(&downloads);
        let mut have = parts::base_here();
        have.insert(self.name.to_string());
        have.extend(parts::groups(&parts::field_of(self.control, "Provides")).into_iter().flatten());
        let mut needs = Needs {
            have,
            waiting: parts::needs(self.control).into_iter().map(|g| (g, false)).collect(),
            missing: Vec::new(),
        };
        let mut extras_added = false;
        loop {
            let Some((group, optional)) = needs.waiting.pop_front() else {

                if extras_added {
                    break;
                }
                extras_added = true;
                let extras = parts::recommended_extras(self.control, &needs.have);
                needs.waiting.extend(extras.into_iter().map(|g| (g, true)));
                continue;
            };
            if answered(&group, &needs.have) {
                continue;
            }
            if found.len() > MOST_PARTS {
                needs.miss(&group, optional);
                continue;
            }
            match group.iter().find_map(|p| find_part(p, &local, &downloads).map(|k| (p.clone(), k))) {
                Some((part, key)) => {
                    heard(&t!("store-boxed-adding", part = &part));
                    self.take_part(&mut needs, found, (part, key), optional);
                }
                None => {
                    needs.miss(&group, optional);
                    needs.have.extend(group);
                }
            }
        }
        let _ = std::fs::remove_dir_all(&downloads);
        needs.missing
    }

    fn take_part(&self, needs: &mut Needs, found: &mut Vec<String>, (part, key): (String, String), optional: bool) {
        let c = parts::control(&key);
        needs.have.insert(part);
        needs.have.insert(parts::field_of(&c, "Package"));
        needs.have.extend(parts::groups(&parts::field_of(&c, "Provides")).into_iter().flatten());
        needs.waiting.extend(parts::needs(&c).into_iter().map(|g| (g, optional)));
        if !found.contains(&key) {
            found.push(key);
            let _ = write_lines(&self.pending, found);
        }
    }

    fn finish(&self, main: &str, found: &[String], missing: Vec<String>) -> Result<Installed, String> {

        let mut keys: Vec<String> = found[1..].iter().rev().cloned().collect();
        keys.push(main.to_string());
        if let Err(e) = relink(&self.at, &keys) {
            return self.give_up(e);
        }
        let files = self.at.join("files");
        take_launchers(self.name);
        put_launchers(self.name, &files)?;
        crate::common::files::write_whole(&self.at.join("version"), parts::field_of(self.control, "Version").as_bytes()).map_err(|e| e.to_string())?;
        let _ = std::fs::remove_file(&self.pending);
        let wants = work_out_wants(&parts::tree(main), &files, self.control);
        write_lines(&self.at.join("wants"), &wants).map_err(|e| e.to_string())?;
        parts::sweep();
        let asked = asked(self.name);
        let ask = !wants.iter().all(|w| asked.contains(w));
        Ok(Installed { name: self.name.to_string(), missing, ask })
    }
}

pub fn remove(name: &str) -> std::io::Result<()> {
    if name.is_empty() || name.contains('/') || name.starts_with('.') {
        return Err(std::io::Error::other("not a package's name"));
    }
    crate::common::removal::stop_boxed(name, None);
    for path in launchers_of(name) {
        if let Some(id) = path.file_name() {
            let _ = crate::common::favourites::unpin_app(&crate::common::favourites::file(), &id.to_string_lossy());
        }
    }
    take_launchers(name);
    match std::fs::remove_dir_all(folder(name)) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(e),
        _ => {}
    }
    parts::sweep();
    Ok(())
}
