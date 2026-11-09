//! Files another program lets go on the Apps window, and which of them it
//! takes: Debian packages, AppImages and Flatpak references. Everything
//! that arrives is from a stranger, so each is looked at before anything
//! is done with it.

use std::io::Read;
use std::path::{Path, PathBuf};

use crate::toolkit::dnd::Given;
use crate::toolkit::t;
use crate::toolkit::widget::Tone;
use super::{came_from, pages, Store};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {

    Deb,

    AppImage,

    FlatpakRef,
}

const REF_MOST: u64 = 64 * 1024;

pub fn only() -> &'static str {
    t!("store-dropped-only")
}

fn ending(file: &Path, want: &str) -> bool {
    file.extension().is_some_and(|e| e.eq_ignore_ascii_case(want))
}

fn head(file: &Path, n: usize) -> Vec<u8> {
    let mut got = Vec::new();
    if let Ok(f) = std::fs::File::open(file) {
        let _ = f.take(n as u64).read_to_end(&mut got);
    }
    got
}

pub fn kind_of(file: &Path) -> Option<Kind> {
    let meta = std::fs::metadata(file).ok().filter(|m| m.is_file())?;
    if ending(file, "deb") {
        return Some(Kind::Deb);
    }
    if ending(file, "appimage") {
        return head(file, 4).starts_with(b"\x7fELF").then_some(Kind::AppImage);
    }
    if ending(file, "flatpakref") {
        return (meta.len() <= REF_MOST && reference(file).is_some()).then_some(Kind::FlatpakRef);
    }
    None
}

pub fn taken(files: &[PathBuf], most: usize) -> Vec<(Kind, PathBuf)> {
    files.iter().filter_map(|f| kind_of(f).map(|k| (k, f.clone()))).take(most).collect()
}

#[derive(Debug, PartialEq, Eq)]
pub struct Reference {
    pub name: String,
    pub host: String,
}

pub fn reference(file: &Path) -> Option<Reference> {
    let text = String::from_utf8(head(file, REF_MOST as usize)).ok()?;
    parse(&text)
}

fn parse(text: &str) -> Option<Reference> {
    let mut lines = text.trim_start_matches('\u{feff}').lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#'));
    if lines.next()? != "[Flatpak Ref]" {
        return None;
    }
    let (mut name, mut url, mut runtime) = (None, None, false);
    for line in lines {
        if line.starts_with('[') {
            break;
        }
        let Some((key, value)) = line.split_once('=') else { continue };
        match key.trim() {
            "Name" => name = Some(value.trim().to_string()),
            "Url" => url = Some(value.trim().to_string()),
            "IsRuntime" => runtime = value.trim().eq_ignore_ascii_case("true"),
            _ => {}
        }
    }
    let name = name.filter(|n| !n.is_empty() && n.len() <= 255 && n.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '_' || c == '-'))?;
    let host = url?.strip_prefix("https://")?.split(['/', '?', '#']).next()?.to_string();
    (!runtime && !host.is_empty() && host.chars().all(|c| c.is_ascii_graphic())).then_some(Reference { name, host })
}

pub fn install_argv(file: &Path) -> Vec<String> {
    let from = format!("--from={}", file.display());
    crate::common::gentle::argv(&["flatpak", "install", "--user", "--noninteractive", "-y", &from]).into_iter().map(String::from).collect()
}

const DROPPED_MOST: usize = 4;

impl Store {

    pub(super) fn dropped(&mut self, given: Given) {
        let files = match given {
            Given::Files(files) => files,
            Given::Text(_) | Given::Typed(..) => return,
            Given::Nothing(why) => return self.top = Some((why.words().into(), Tone::Problem)),
        };
        let files = taken(&files, DROPPED_MOST);
        if files.is_empty() {
            self.top = Some((only().into(), Tone::Plain));
            return;
        }
        self.top = None;
        for (kind, file) in files {
            match kind {
                Kind::Deb => pages::add_file::open_deb(self, file),
                Kind::AppImage => pages::appimage_warning::before(self, move |s| pages::add_file::add_appimage_file(s, file, true, came_from::CameFrom::NotKnown)),
                Kind::FlatpakRef => pages::add_file::install_flatpakref(self, file),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str, bytes: &[u8]) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-dropped-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join(name);
        std::fs::write(&p, bytes).unwrap();
        p
    }

    #[test]
    fn a_reference_is_read_and_a_strangers_odd_one_is_turned_away() {
        let good = "[Flatpak Ref]\nName=org.example.App\nBranch=stable\nUrl=https://dl.example.com/repo/\n";
        assert_eq!(parse(good), Some(Reference { name: "org.example.App".into(), host: "dl.example.com".into() }));
        assert!(parse("[Flatpak Ref]\nName=org.example.App\nUrl=http://dl.example.com/\n").is_none(), "not https");
        assert!(parse("[Flatpak Ref]\nName=org.example.App\nUrl=https://x.example.com/\nIsRuntime=true\n").is_none(), "a runtime");
        assert!(parse("[Flatpak Ref]\nName=a b\nUrl=https://x.example.com/\n").is_none(), "an odd name");
        assert!(parse("[Flatpak Ref]\nName=org.example.App\n").is_none(), "no address");
        assert!(parse("[Desktop Entry]\nName=org.example.App\nUrl=https://x.example.com/\n").is_none(), "wrong group");
        assert!(parse("").is_none());
    }

    #[test]
    fn a_name_alone_is_not_believed() {
        let elf = scratch("a.AppImage", b"\x7fELF....");
        let text = scratch("b.AppImage", b"#!/bin/sh\nrm -rf x\n");
        let good = scratch("c.flatpakref", b"[Flatpak Ref]\nName=org.example.App\nUrl=https://x.example.com/\n");
        let bad = scratch("d.flatpakref", b"echo hi");
        let big = scratch("e.flatpakref", &vec![b'#'; REF_MOST as usize + 1]);
        let deb = scratch("f.DEB", b"x");
        let other = scratch("g.txt", b"x");
        assert_eq!(kind_of(&elf), Some(Kind::AppImage));
        assert_eq!(kind_of(&text), None);
        assert_eq!(kind_of(&good), Some(Kind::FlatpakRef));
        assert_eq!(kind_of(&bad), None);
        assert_eq!(kind_of(&big), None);
        assert_eq!(kind_of(&deb), Some(Kind::Deb));
        assert_eq!(kind_of(&other), None);
        assert_eq!(kind_of(Path::new("/nonexistent/x.deb")), None);
        let all = [elf.clone(), text, good.clone(), bad, deb.clone(), other];
        assert_eq!(taken(&all, 2), vec![(Kind::AppImage, elf), (Kind::FlatpakRef, good)]);
        assert_eq!(taken(&all, 9).last().map(|t| t.1.clone()), Some(deb));
    }

    #[test]
    fn the_path_is_never_read_as_an_option() {
        let argv = install_argv(Path::new("/tmp/--user.flatpakref"));
        assert_eq!(argv.last().unwrap(), "--from=/tmp/--user.flatpakref");
        assert_eq!(argv.iter().filter(|a| a.starts_with("--from")).count(), 1);
    }
}
