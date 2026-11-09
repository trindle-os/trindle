//! What Discover, a kind and results list: Flathub's apps and AppImageHub's side by side, as the selector chooses.

use crate::store::catalogue::{App, POPULAR};
use crate::store::github::Found;
use crate::store::formats::{self, Show};
use crate::store::{browse, hub, Store, View, PAGE};
use crate::toolkit::widget::Tone;
use crate::toolkit::t;

pub(crate) enum Tile<'a> {
    Flat(&'a App),
    Image(&'a Found),
}

impl Tile<'_> {
    fn name(&self) -> &str {
        match self {
            Tile::Flat(a) => &a.name,
            Tile::Image(f) => &f.name,
        }
    }
}

fn words_of(text: &str) -> Vec<String> {
    text.to_lowercase().split_whitespace().map(String::from).collect()
}

pub(crate) fn tiles(s: &Store) -> Vec<Tile<'_>> {
    let listed: &[Found] = s.hub.as_deref().map_or(&[], |v| &v[..]);
    let (flat, images): (Vec<&App>, Vec<&Found>) = match &s.view {
        View::Home => (POPULAR.iter().filter_map(|i| s.apps.get(*i)).collect(), hub::popular(listed)),
        View::Category(code) => (s.apps.values().filter(|a| a.categories.contains(code)).collect(),
                                 listed.iter().filter(|f| hub::in_kind(f, code)).collect()),
        View::Search(text) => {
            let words = words_of(text);
            (s.apps.values().filter(|a| a.matches(&words)).collect(), hub::matching(listed, &words))
        }
        _ => return Vec::new(),
    };
    let flat = flat.into_iter().map(Tile::Flat).filter(|_| s.show.flatpaks());
    let images = images.into_iter().map(Tile::Image).filter(|_| s.show.appimages());
    let mut all: Vec<Tile> = if s.view == View::Home { take_turns(flat, images) } else { flat.chain(images).collect() };
    match &s.view {
        View::Category(_) => all.sort_by_cached_key(|t| t.name().to_lowercase()),
        View::Search(text) => {
            let first = words_of(text).into_iter().next().unwrap_or_default();
            all.sort_by_cached_key(|t| { let n = t.name().to_lowercase(); (!n.starts_with(&first), n) });
        }
        _ => {}
    }
    all
}

fn take_turns<'a>(a: impl Iterator<Item = Tile<'a>>, b: impl Iterator<Item = Tile<'a>>) -> Vec<Tile<'a>> {
    let (mut a, mut b) = (a.fuse(), b.fuse());
    let mut all = Vec::new();
    loop {
        let (x, y) = (a.next(), b.next());
        if x.is_none() && y.is_none() {
            return all;
        }
        all.extend(x.into_iter().chain(y));
    }
}

pub(crate) fn offered(s: &Store) -> (usize, usize) {
    let listed = s.hub.as_deref().map_or(0, Vec::len);
    (if s.show.flatpaks() { s.apps.len() } else { 0 }, if s.show.appimages() { listed } else { 0 })
}

pub(crate) fn icons(s: &Store) -> Vec<String> {
    tiles(s).into_iter().take(s.shown).filter_map(|t| match t {
        Tile::Image(f) if !f.icon.is_empty() => Some(f.icon.clone()),
        _ => None,
    }).take(ICONS_AT_ONCE).collect()
}

const ICONS_AT_ONCE: usize = 60;

pub(crate) fn why_none(s: &Store) -> Option<&'static str> {
    if !s.show.appimages() {
        return None;
    }
    match s.hub.as_deref().map(Vec::as_slice) {
        Some([]) if !s.online => Some(t!("store-browse-offline")),
        Some([]) => Some(t!("store-browse-failed")),
        _ => None,
    }
}

pub(crate) fn set_show(s: &mut Store, show: Show) {
    s.show = show;
    s.shown = PAGE;
    if let Err(e) = formats::set_show_in(&s.conf, show) {
        s.top = Some((e.to_string(), Tone::Problem));
    }
    if show.appimages() {
        browse::want_hub(s);
    }
    browse::fetch_icons(s);
}
