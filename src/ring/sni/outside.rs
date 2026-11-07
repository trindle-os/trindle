//! Icons for programs that have no tray icon of their own.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use tiny_skia::Pixmap;
use zbus::zvariant::Value;

use crate::toolkit::picture;
use crate::toolkit::icon::Icons;

use super::{MAX_PIXMAP, Outside, Want};
use super::look::{Look, SIZE};
use super::menu::{number, part, unbox};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Step {
    Nothing,
    Look(Want),
}

#[derive(Debug, PartialEq)]
pub(super) enum Seen {
    Named(String),
    Shape(String),
    Pixels(Pixmap),
}

impl Seen {
    pub(super) fn look(self) -> Look {
        match self {
            Seen::Named(name) => Look::Named(name),
            Seen::Shape(name) => Look::shape(Rc::new(Icons::new()), name),
            Seen::Pixels(px) => Look::Pixels { px, cache: RefCell::new(None) },
        }
    }
}

impl Outside {
    pub(super) fn keeps(&mut self, want: &Option<Want>, stand_in: Rc<Look>) -> bool {
        self.stand_in = Some(stand_in);
        let kept = want.is_some() && self.shown == *want;
        if !kept {
            self.shown = None;
        }
        kept
    }

    pub(super) fn asks(&mut self, want: Option<Want>) -> Step {
        self.wanted = want;
        match (&self.out, &self.wanted) {
            (None, Some(want)) => {
                self.out = Some(want.clone());
                Step::Look(want.clone())
            }
            _ => Step::Nothing,
        }
    }

    pub(super) fn back(&mut self) -> (bool, Step) {
        let went_for = self.out.take();
        let ours = went_for.is_some() && went_for == self.wanted;
        match self.wanted.clone().filter(|_| !ours) {
            Some(want) => {
                self.out = Some(want.clone());
                (ours, Step::Look(want))
            }
            None => (ours, Step::Nothing),
        }
    }

    pub(super) fn shows(&mut self, found: Option<Look>) -> Option<Rc<Look>> {
        match found {
            Some(look) => {
                self.shown = self.wanted.clone();
                Some(Rc::new(look))
            }
            None => self.shown.take().and(self.stand_in.clone()),
        }
    }
}

pub(super) fn wanted_outside(name: &str, folder: &str, in_set: bool, pixels: bool) -> Option<Want> {
    let whole = name.starts_with('/');
    let wanted = !name.is_empty() && !in_set && if whole { !pixels } else { !folder.is_empty() };
    wanted.then(|| Want { name: name.to_string(), folder: if whole { String::new() } else { folder.to_string() } })
}

fn seen_outside(name: &str, folder: &str) -> Option<Seen> {
    if name.starts_with('/') {
        let file = Path::new(name);
        return file.is_file().then(|| file_seen(file, false)).flatten();
    }
    own_folder(Path::new(folder), name)
}

pub(super) fn look_aside(want: Want, done: impl FnOnce(Option<Seen>) + 'static) {
    crate::common::run::on_thread(move || seen_outside(&want.name, &want.folder), done);
}

fn own_folder(dir: &Path, name: &str) -> Option<Seen> {
    let symbolic = format!("{name}-symbolic");
    for kind in ["svg", "png"] {
        let f = dir.join(format!("{symbolic}.{kind}"));
        if f.is_file() {
            return Some(Seen::Named(f.to_string_lossy().into_owned()));
        }
    }
    for kind in ["png", "svg", "xpm"] {
        let f = dir.join(format!("{name}.{kind}"));
        if f.is_file() {
            return file_seen(&f, false);
        }
    }
    let mut found: Vec<(u32, bool, PathBuf)> = Vec::new();
    for set in subfolders(dir) {
        for size in subfolders(&set) {
            let n = size.file_name().map(|s| s.to_string_lossy().into_owned()).unwrap_or_default();
            let px = if n == "scalable" || n == "symbolic" { u32::MAX } else {
                n.split('x').next().and_then(|d| d.parse().ok()).unwrap_or(0)
            };
            for kind in subfolders(&size) {
                for (stem, is_symbolic) in [(symbolic.as_str(), true), (name, false)] {
                    for ext in ["png", "svg", "xpm"] {
                        let f = kind.join(format!("{stem}.{ext}"));
                        if f.is_file() {
                            found.push((px, is_symbolic, f));
                        }
                    }
                }
            }
        }
    }
    let best = found.iter().filter(|f| f.0 >= SIZE).min_by_key(|f| f.0)
        .or_else(|| found.iter().max_by_key(|f| f.0))?;
    file_seen(&best.2, best.1)
}

fn file_seen(file: &Path, symbolic: bool) -> Option<Seen> {
    if file.extension().is_some_and(|x| x == "xpm") {
        return picture::from_xpm(&std::fs::read_to_string(file).ok()?).map(Seen::Pixels);
    }
    let name = file.to_string_lossy().into_owned();
    Some(if symbolic { Seen::Named(name) } else { Seen::Shape(name) })
}

fn subfolders(dir: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(dir).into_iter().flatten().flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect()
}

pub(super) fn largest(v: &Value) -> Option<Pixmap> {
    let Value::Array(list) = unbox(v) else { return None };
    let mut best: Option<(i64, i64, Vec<u8>)> = None;
    for p in list.inner() {
        let (Some(w), Some(h), Some(Value::Array(bytes))) = (part(p, 0).and_then(number), part(p, 1).and_then(number), part(p, 2)) else { continue };
        if w <= 0 || h <= 0 || w > MAX_PIXMAP || h > MAX_PIXMAP {
            continue;
        }
        if (bytes.inner().len() as i64) < w * h * 4 {
            continue;
        }
        if best.as_ref().is_none_or(|b| w > b.0) {
            let data = bytes.inner().iter().map(|b| if let Value::U8(b) = b { *b } else { 0 }).collect();
            best = Some((w, h, data));
        }
    }
    let (w, h, data) = best?;
    picture::from_argb32(w as u32, h as u32, &data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_look_outside_the_set_finds_the_items_own_icon() {
        let dir = std::env::temp_dir().join(format!("trindle-sni-own-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let sized = dir.join("hicolor/24x24/apps");
        std::fs::create_dir_all(&sized).unwrap();
        std::fs::create_dir_all(dir.join("hicolor/16x16/apps")).unwrap();
        for file in [dir.join("loose.png"), dir.join("tinted-symbolic.svg"), sized.join("player.png"), dir.join("hicolor/16x16/apps/player.png")] {
            std::fs::write(file, b"").unwrap();
        }
        std::fs::write(dir.join("old.xpm"), "/* XPM */\nstatic char *x[] = {\n\"1 1 1 1\",\n\"a c #ff0000\",\n\"a\"\n};\n").unwrap();
        let folder = dir.to_string_lossy().into_owned();
        let path = |p: std::path::PathBuf| p.to_string_lossy().into_owned();
        assert_eq!(seen_outside("loose", &folder), Some(Seen::Shape(path(dir.join("loose.png")))));
        assert_eq!(seen_outside("tinted", &folder), Some(Seen::Named(path(dir.join("tinted-symbolic.svg")))));
        assert_eq!(seen_outside("player", &folder), Some(Seen::Shape(path(sized.join("player.png")))));
        assert!(matches!(seen_outside("old", &folder), Some(Seen::Pixels(_))));
        assert_eq!(seen_outside("nothing", &folder), None);
        assert_eq!(seen_outside(&path(dir.join("loose.png")), ""), Some(Seen::Shape(path(dir.join("loose.png")))));
        assert_eq!(seen_outside(&path(dir.join("gone.png")), ""), None);
        assert_eq!(seen_outside(&folder, ""), None);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
