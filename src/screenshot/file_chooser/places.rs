//! The places down the left of the chooser: the usual folders, bookmarks and drives.

use std::path::{Path, PathBuf};

use crate::common::kinds::{root_name, shown_name, uri_path};
use crate::toolkit::t;

use super::{Place, Where};

pub(super) fn once_each(mut places: Vec<Place>) -> Vec<Place> {
    let leads_to = |p: &Place| match &p.at {
        Where::Recent => None,
        Where::Folder(f) => {
            let written = f.to_string_lossy();
            let plain = written.trim_end_matches('/');
            Some(if plain.is_empty() { "/".to_string() } else { plain.to_string() })
        }
    };
    let last = places.pop();
    let mut listed: Vec<Option<String>> = last.iter().map(leads_to).collect();
    let mut once: Vec<Place> = places.into_iter().filter(|p| {
        let to = leads_to(p);
        let new = !listed.contains(&to);
        if new {
            listed.push(to);
        }
        new
    }).collect();
    once.extend(last);
    once
}

pub(super) fn first_places(home: &Path) -> Vec<Place> {
    vec![Place { label: t!("screenshot-chooser-recent").into(), icon: "document-open-recent-symbolic", folder: false, at: Where::Recent },
         Place { label: t!("screenshot-chooser-home").into(), icon: "user-home-symbolic", folder: true, at: Where::Folder(home.to_path_buf()) },
         Place { label: root_name().into(), icon: "drive-harddisk-symbolic", folder: false, at: Where::Folder(PathBuf::from("/")) }]
}

pub(super) fn bookmarks(text: &str) -> Vec<(String, PathBuf)> {
    text.lines().filter_map(|line| {
        let (uri, name) = line.split_once(' ').unwrap_or((line, ""));
        let path = uri_path(uri)?;
        let name = if name.trim().is_empty() { shown_name(&path) } else { name.trim().to_string() };
        Some((name, path))
    }).collect()
}

pub(super) fn usual(text: &str, home: &Path) -> Vec<(&'static str, PathBuf)> {
    let keys = [("DOCUMENTS", "folder-documents-symbolic"), ("DOWNLOAD", "folder-download-symbolic"), ("MUSIC", "folder-music-symbolic"),
        ("PICTURES", "folder-pictures-symbolic"), ("VIDEOS", "folder-videos-symbolic")];
    keys.iter().filter_map(|(key, icon)| {
        let path = crate::common::files::user_folder_for(text, key, home)?;
        (path != home).then_some((*icon, path))
    }).collect()
}

pub(super) fn drives(mounts: &str) -> Vec<PathBuf> {
    mounts.lines().filter_map(|line| {
        let at = line.split(' ').nth(1)?;
        let at = at.replace("\\040", " ").replace("\\011", "\t").replace("\\134", "\\");
        ["/media/", "/run/media/", "/mnt/"].iter().any(|p| at.starts_with(p)).then(|| PathBuf::from(at))
    }).collect()
}
