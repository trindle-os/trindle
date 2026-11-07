//! What the ring and the file chooser both need.

use std::collections::{HashMap, HashSet};
use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

pub use crate::toolkit::common::file_work::address::{file_uri, uri_path};

use crate::common::clock::Local;
use crate::common::files::data_home;
use crate::toolkit::t;

#[derive(Default)]
pub struct Mime {
    endings: HashMap<String, (u32, String)>,
    others: Vec<(u32, String, String, bool)>,
    parents: HashMap<String, Vec<String>>,
    aliases: HashMap<String, String>,
}

type Lists = [String; 3];

fn lists_in(mime: &Path) -> Lists {
    ["globs2", "subclasses", "aliases"].map(|name| crate::common::files::read_string(mime.join(name)).unwrap_or_default())
}

const ELSEWHERE_WAIT: std::time::Duration = std::time::Duration::from_millis(40);

static ELSEWHERE_READS: crate::common::run::Slots = crate::common::run::Slots::new(2);

impl Mime {
    pub fn load() -> Mime {
        let mut dirs = vec![data_home()];
        let shared = std::env::var("XDG_DATA_DIRS").ok().filter(|s| !s.is_empty())
            .unwrap_or_else(|| "/usr/local/share:/usr/share".into());
        dirs.extend(shared.split(':').map(PathBuf::from));
        Mime::load_from(dirs, crate::common::files::own_place, lists_in, ELSEWHERE_WAIT)
    }

    fn load_from(dirs: Vec<PathBuf>, own: fn(&Path) -> bool, read: fn(&Path) -> Lists, wait: std::time::Duration) -> Mime {
        let dirs: Vec<PathBuf> = dirs.iter().map(|d| d.join("mime")).collect();
        let here = |mime: &PathBuf| ["globs2", "subclasses", "aliases"].iter().all(|name| own(&mime.join(name)));
        let mut lists: Vec<Option<Lists>> = dirs.iter().map(|mime| here(mime).then(|| read(mime))).collect();
        let elsewhere: Vec<(usize, PathBuf)> = dirs.iter().enumerate().filter(|(n, _)| lists[*n].is_none()).map(|(n, d)| (n, d.clone())).collect();
        if !elsewhere.is_empty() {
            if let Some(place) = ELSEWHERE_READS.take() {
                let (back, came) = std::sync::mpsc::channel();
                std::thread::spawn(move || {
                    let _place = place;
                    let found: Vec<(usize, Lists)> = elsewhere.into_iter().map(|(n, mime)| (n, read(&mime))).collect();
                    let _ = back.send(found);
                });
                for (n, read) in came.recv_timeout(wait).unwrap_or_default() {
                    lists[n] = Some(read);
                }
            }
        }
        let mut mime = Mime::default();
        for [globs, subclasses, aliases] in lists.iter().rev().flatten() {
            mime.add(globs, subclasses, aliases);
        }
        mime
    }

    pub(crate) fn add(&mut self, globs: &str, subclasses: &str, aliases: &str) {
        for line in globs.lines().filter(|l| !l.starts_with('#')) {
            let mut bits = line.splitn(4, ':');
            let (Some(weight), Some(kind), Some(glob)) = (bits.next(), bits.next(), bits.next()) else { continue };
            let Ok(weight) = weight.parse::<u32>() else { continue };
            let cs = bits.next().is_some_and(|f| f.split(',').any(|f| f == "cs"));
            match glob.strip_prefix("*.") {
                Some(end) if !cs && !end.contains(['*', '?', '[']) => {
                    let end = end.to_lowercase();
                    if self.endings.get(&end).is_none_or(|(w, _)| *w < weight) {
                        self.endings.insert(end, (weight, kind.to_string()));
                    }
                }
                _ => self.others.push((weight, glob.to_string(), kind.to_string(), cs)),
            }
        }
        for line in subclasses.lines() {
            if let Some((kind, parent)) = line.split_once(' ') {
                self.parents.entry(kind.to_string()).or_default().push(parent.to_string());
            }
        }
        for line in aliases.lines() {
            if let Some((alias, kind)) = line.split_once(' ') {
                self.aliases.insert(alias.to_string(), kind.to_string());
            }
        }
    }

    pub fn of(&self, name: &str) -> Option<String> {
        let lower = name.to_lowercase();
        let mut best: Option<(u32, usize, &str)> = None;
        for (at, _) in lower.match_indices('.') {
            if let Some((w, kind)) = self.endings.get(&lower[at + 1..]) {
                let len = lower.len() - at + 1;
                if best.is_none_or(|(bw, bl, _)| (*w, len) > (bw, bl)) {
                    best = Some((*w, len, kind));
                }
            }
        }
        for (w, glob, kind, cs) in &self.others {
            let hit = if *cs { glob_matches(glob, name) } else { glob_matches(&glob.to_lowercase(), &lower) };
            if hit && best.is_none_or(|(bw, bl, _)| (*w, glob.len()) > (bw, bl)) {
                best = Some((*w, glob.len(), kind));
            }
        }
        best.map(|(_, _, k)| k.to_string())
    }

    pub fn is_a(&self, kind: &str, wanted: &str) -> bool {
        let wanted = self.aliases.get(wanted).map_or(wanted, |s| s.as_str());
        let kind = self.aliases.get(kind).map_or(kind, |s| s.as_str());
        if let Some(group) = wanted.strip_suffix("/*") {
            return kind.split('/').next() == Some(group);
        }
        if wanted == "text/plain" && kind.starts_with("text/") {
            return true;
        }
        let (mut seen, mut todo) = (HashSet::new(), vec![kind.to_string()]);
        while let Some(k) = todo.pop() {
            if k == wanted {
                return true;
            }
            if seen.insert(k.clone()) {
                todo.extend(self.parents.get(&k).into_iter().flatten().cloned());
            }
        }
        false
    }
}

pub fn glob_matches(glob: &str, name: &str) -> bool {
    let (Ok(g), Ok(n)) = (CString::new(glob), CString::new(name)) else { return false };
    // SAFETY: both are NUL-terminated strings that live through the call.
    unsafe { libc::fnmatch(g.as_ptr(), n.as_ptr(), 0) == 0 }
}

pub fn icon_for(kind: Option<&str>) -> &'static str {
    let Some(kind) = kind else { return "text-x-generic-symbolic" };
    match kind.split('/').next().unwrap_or("") {
        "image" => "image-x-generic-symbolic",
        "audio" => "audio-x-generic-symbolic",
        "video" => "video-x-generic-symbolic",
        _ if kind == "application/pdf" || kind.contains("document") || kind.contains("opendocument") => "x-office-document-symbolic",
        _ if ["zip", "tar", "compress", "archive", "x-7z", "rar", "xz", "bzip", "gzip", "zstd", "debian-package", "efi.iso", "cd-image", "iso9660-image", "x-cpio", "x-rpm", "x-lha", "x-xar", "x-lz"]
            .iter().any(|w| kind.contains(w)) => {
            "package-x-generic-symbolic"
        }
        _ => "text-x-generic-symbolic",
    }
}

pub fn recent(xbel: &str) -> Vec<PathBuf> {
    let mut found: Vec<(String, PathBuf)> = Vec::new();
    for tag in xbel.split("<bookmark ").skip(1) {
        let tag = &tag[..tag.find('>').unwrap_or(tag.len())];
        let attr = |name: &str| {
            let at = tag.find(&format!(" {name}=\"")).or_else(|| tag.starts_with(&format!("{name}=\"")).then_some(0))?;
            let from = tag[at..].find('"')? + at + 1;
            let len = tag[from..].find('"')?;
            Some(tag[from..from + len].replace("&amp;", "&").replace("&apos;", "'").replace("&quot;", "\""))
        };
        let Some(path) = attr("href").as_deref().and_then(uri_path) else { continue };
        let when = attr("modified").or_else(|| attr("visited")).unwrap_or_default();
        found.retain(|(_, p)| *p != path);
        found.push((when, path));
    }
    found.sort_by(|a, b| b.0.cmp(&a.0));
    found.into_iter().map(|(_, p)| p).collect()
}

pub fn root_name() -> &'static str {
    t!("chosen-root-name")
}

pub fn shown_name(path: &Path) -> String {
    path.file_name().map_or_else(|| path.to_string_lossy().into_owned(), |n| n.to_string_lossy().into_owned())
}

pub fn parts(folder: &Path, home: &Path) -> Vec<(String, PathBuf)> {
    let (mut out, rest) = match folder.strip_prefix(home) {
        Ok(rest) if home != Path::new("/") => (vec![(t!("chosen-home").to_string(), home.to_path_buf())], rest),
        _ => (vec![(root_name().to_string(), PathBuf::from("/"))], folder.strip_prefix("/").unwrap_or(folder)),
    };
    let mut at = out[0].1.clone();
    for part in rest.components() {
        at.push(part);
        out.push((part.as_os_str().to_string_lossy().into_owned(), at.clone()));
    }
    out
}

pub fn when_text(secs: i64) -> String {
    if secs <= 0 {
        return String::new();
    }
    let at = Local::at(secs);
    let now = Local::at(std::time::SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs() as i64));
    if at.format("%F") == now.format("%F") { at.line(trindle_toolkit::lang::format::Pattern::Clock) } else { at.line(trindle_toolkit::lang::format::Pattern::DayShortYear) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn made_up(mime: &Path) -> Lists {
        let dir = mime.parent().unwrap().file_name().unwrap().to_string_lossy().into_owned();
        if dir.contains("slow") {
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
        [format!("50:application/x-{dir}:*.{dir}\n"), String::new(), String::new()]
    }

    fn own(path: &Path) -> bool {
        path.starts_with("/here")
    }

    #[test]
    fn the_kinds_of_files_are_never_waited_for_long_from_a_folder_kept_elsewhere() {
        let wait = std::time::Duration::from_millis(500);
        let began = std::time::Instant::now();
        let all = Mime::load_from(vec!["/far/quick".into(), "/here/own".into()], own, made_up, wait);
        assert!(began.elapsed() < wait, "a folder that answered at once was waited for");
        assert_eq!(all.of("a.quick").as_deref(), Some("application/x-quick"));
        assert_eq!(all.of("a.own").as_deref(), Some("application/x-own"));
        let began = std::time::Instant::now();
        let some = Mime::load_from(vec!["/far/slow".into(), "/here/own".into()], own, made_up, std::time::Duration::from_millis(60));
        assert!(began.elapsed() < std::time::Duration::from_secs(2), "the start waited on a folder that had stopped answering");
        assert_eq!(some.of("a.own").as_deref(), Some("application/x-own"));
        assert_eq!(some.of("a.slow"), None);
        let code = crate::common::source::code(include_str!("kinds.rs"));
        let code = code.split("\n#[cfg(test)]").next().unwrap();
        assert!(code.contains("here(mime).then(|| read(mime))"));
        assert_eq!(code.matches("read(mime)").count(), 1);
        assert_eq!(code.matches("read(&mime)").count(), 1, "the lists kept elsewhere are read in one place, on the thread");
        assert!(code.contains("Mime::load_from(dirs, crate::common::files::own_place, lists_in, ELSEWHERE_WAIT)"));
    }
}
