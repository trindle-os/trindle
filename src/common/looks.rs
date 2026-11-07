//! How a folder or a file is drawn outside Folders, as the ring shows what is pinned to it.

use std::ffi::OsString;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

use md5::{Digest, Md5};

use super::files::{cache_home, read_file, trindle_config};
use super::kinds::{icon_for, shown_name, Mime};

const COLOURS: &str = "files-colours";

pub struct Looks {
    pub icon: String,
    pub tint: Option<crate::toolkit::colour::Colour>,
    pub folder: bool,
}

pub fn looks(paths: &[PathBuf]) -> Vec<Looks> {
    looks_with(paths, &Mime::load(), &colours(), &cache_home().join("thumbnails"))
}

fn looks_with(paths: &[PathBuf], mime: &Mime, colours: &[(PathBuf, String)], thumbs: &Path) -> Vec<Looks> {
    paths.iter().map(|path| {
        if path.is_dir() {
            let tint = colours.iter().rev().find(|(p, _)| p == path).and_then(|(_, name)| colour_named(name));
            return Looks { icon: "folder-symbolic".into(), tint, folder: true };
        }
        if let Some(picture) = cached(thumbs, path) {
            return Looks { icon: picture.to_string_lossy().into_owned(), tint: None, folder: false };
        }
        let kind = mime.of(&shown_name(path));
        Looks { icon: icon_for(kind.as_deref()).into(), tint: None, folder: false }
    }).collect()
}

fn colours() -> Vec<(PathBuf, String)> {
    let bytes = read_file(trindle_config().join(COLOURS)).unwrap_or_default();
    bytes.split(|&b| b == b'\n').filter_map(|line| {
        let tab = line.iter().position(|&b| b == b'\t')?;
        let word = String::from_utf8(line[..tab].to_vec()).ok()?;
        let path = &line[tab + 1..];
        (!path.is_empty()).then(|| (PathBuf::from(OsString::from_vec(path.to_vec())), word))
    }).collect()
}

fn colour_named(name: &str) -> Option<crate::toolkit::colour::Colour> {
    let hex = super::style::ACCENTS.iter()
        .find(|(_, n)| n.eq_ignore_ascii_case(name)).map(|(h, _)| *h).unwrap_or(name);
    crate::toolkit::colour::Colour::parse(hex)
}

fn uri(path: &Path) -> String {
    let whole = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir().map(|d| d.join(path)).unwrap_or_else(|_| path.to_path_buf())
    };
    let mut out = String::from("file://");
    for &b in whole.as_os_str().as_bytes() {
        if b.is_ascii_alphanumeric() || b"!$&'()*+,-./:=@_~".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

fn cached(root: &Path, path: &Path) -> Option<PathBuf> {
    let thumb = root.join("normal").join(format!("{:x}.png", Md5::digest(uri(path).as_bytes())));
    fits(&thumb, path).then_some(thumb)
}

fn fits(thumb: &Path, path: &Path) -> bool {
    let Ok(meta) = std::fs::metadata(path) else { return false };
    let Ok(png) = read_file(thumb) else { return false };
    let texts = texts(&png);
    let get = |key: &str| texts.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str());
    let mtime = get("Thumb::MTime").and_then(|t| t.split('.').next()).and_then(|t| t.parse::<i64>().ok());
    let size_ok = get("Thumb::Size").is_none_or(|s| s.parse::<u64>().ok() == Some(meta.len()));
    get("Thumb::URI") == Some(uri(path).as_str()) && mtime == Some(meta.mtime()) && size_ok
}

fn texts(png: &[u8]) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if !png.starts_with(b"\x89PNG\r\n\x1a\n") {
        return out;
    }
    let mut at = 8;
    while at + 8 <= png.len() {
        let len = u32::from_be_bytes([png[at], png[at + 1], png[at + 2], png[at + 3]]) as usize;
        let kind = &png[at + 4..at + 8];
        let Some(data) = png.get(at + 8..at + 8 + len) else { break };
        if kind == b"IDAT" || kind == b"IEND" {
            break;
        }
        if kind == b"tEXt" {
            if let Some(nul) = data.iter().position(|&b| b == 0) {
                let latin = |b: &[u8]| b.iter().map(|&c| c as char).collect::<String>();
                out.push((latin(&data[..nul]), latin(&data[nul + 1..])));
            }
        }
        at += 12 + len;
    }
    out
}
