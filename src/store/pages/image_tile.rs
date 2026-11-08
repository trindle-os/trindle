//! An AppImage as a tile on Discover, a kind and results: the Flatpaks' design, its line saying what it is; and how an AppImage stands on this computer.

use std::path::{Path, PathBuf};

use crate::store::came_from::CameFrom;
use crate::store::github::Found;
use crate::store::{go, listing, loose, look, Msg, Store};
use crate::settings::ui::INSTALL_ICON;
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::widget::{button, card, space, Length, Node};
use crate::store::updates::Source;
use super::github_page::hub_icon;
use super::github_search::install_repo;
use super::install::pressed_install;
use super::links::link_button;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Standing {
    Absent,
    Going(f32, bool),
    Here(PathBuf),
}

pub(crate) fn key_of(f: &Found) -> String {
    if f.repo.is_empty() { format!("site:{}", f.site) } else { format!("gh:{}", f.repo) }
}

pub(crate) fn installed_path(s: &Store, f: &Found) -> Option<PathBuf> {
    let from = CameFrom::of(f);
    s.owned.list.as_deref()?.iter().find_map(|i| {
        let Source::AppImage(path) = &i.source else { return None };
        let same = match (&from, s.owned.origins.get(path)?) {
            (CameFrom::GitHub(a), CameFrom::GitHub(b)) => a.eq_ignore_ascii_case(b),
            (CameFrom::Site(a), CameFrom::Site(b)) => a == b,
            (_, CameFrom::NotKnown) => same_name(path, &f.name),
            _ => false,
        };
        same.then(|| path.clone())
    })
}

fn same_name(file: &Path, name: &str) -> bool {
    let file = file.file_name().unwrap_or_default().to_string_lossy();
    loose::id_for(crate::common::apps::appimage_stem(&file)) == loose::id_for(name)
}

pub(crate) fn standing(s: &Store, f: &Found) -> Standing {
    if let Some(path) = installed_path(s, f) {
        return Standing::Here(path);
    }
    match s.dials.get(&key_of(f)) {
        Some(d) if d.tone() != crate::toolkit::widget::Tone::Problem => Standing::Going(d.share(), d.running()),
        _ => Standing::Absent,
    }
}

pub(crate) fn open_file(path: &Path) {
    #[cfg(test)]
    STARTED.with(|a| a.borrow_mut().push(path.to_path_buf()));
    #[cfg(not(test))]
    crate::common::run::spawn(&[&path.display().to_string()]);
}

#[cfg(test)]
thread_local! {
    static STARTED: std::cell::RefCell<Vec<PathBuf>> = const { std::cell::RefCell::new(Vec::new()) };
}

#[cfg(test)]
pub(crate) fn started_so_far() -> Vec<PathBuf> {
    STARTED.with(|a| std::mem::take(&mut *a.borrow_mut()))
}

pub(crate) fn image_tile(s: &Store, f: &Found) -> Node<Msg> {
    let key = key_of(f);
    let words = if f.summary.trim().is_empty() { t!("store-appimage-line-bare").to_string() } else { t!("store-appimage-line", summary = &f.summary) };
    let picture = hub_icon(s, &f.icon, theme::apps_list().tile_icon);
    let end = vec![action(s, f).key(format!("{}:{key}", listing::TILE_ACTION))];
    let open = f.clone();
    card(listing::listing(Some(picture), f.name.clone(), listing::line(words), Vec::new(), end))
        .key(format!("{}:{key}", listing::TILE)).on_press(go(move |s| super::image_page::open(s, open.clone())))
}

fn action(s: &Store, f: &Found) -> Node<Msg> {
    let name = &f.name;
    match standing(s, f) {
        Standing::Here(path) => look::open_button(&t!("store-list-open", name = name)).on_press(go(move |_| open_file(&path))),
        Standing::Going(..) => match s.dials.get(&key_of(f)) {
            Some(d) => d.round(button(INSTALL_ICON, t!("store-list-installing", name = name))),
            None => space().width(Length::Fixed(theme::icon_button())),
        },
        Standing::Absent if f.repo.is_empty() => match CameFrom::of(f).link() {
            Some((url, tip)) => link_button(tip, &url),
            None => space().width(Length::Fixed(theme::icon_button())),
        },
        Standing::Absent => {
            let repo = f.clone();
            button(INSTALL_ICON, t!("store-list-install", name = name)).on_press(go(move |s| pressed_install(s, |s| install_repo(s, &repo))))
        }
    }
}
