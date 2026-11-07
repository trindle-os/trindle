//! The Browse AppImages page.

use std::rc::Rc;

use super::catalogue::categories;
use super::pages::{self, github_page::github_row, grid::GITHUB_HUB};
use super::screen::{list_card, screen, title_row};
use super::{github, go, hub, listing, look, Msg, Store, View};
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::widget::{column, note, row, space, Length, Node};

const ROWS: usize = 20;

pub struct Page {
    kind: usize,
    shown: usize,
}

impl Default for Page {
    fn default() -> Page {
        Page { kind: 0, shown: ROWS }
    }
}

fn kinds() -> Vec<(&'static str, &'static str)> {
    let mut all = vec![("", t!("store-tab-all"))];
    all.extend(categories().iter().map(|(code, name, _)| (*code, *name)));
    all
}

fn of_kind<'a>(listed: &'a [github::Found], code: &str) -> Vec<&'a github::Found> {
    listed.iter().filter(|f| code.is_empty() || hub::in_kind(f, code)).collect()
}

pub fn want_hub(s: &mut Store) {
    if s.hub_asked && s.hub.as_ref().is_none_or(|l| !l.is_empty()) {
        return;
    }
    s.hub_asked = true;
    hub::fetch(s.then(hub_here));
}

pub fn hub_here(s: &mut Store, found: Vec<github::Found>) {
    s.hub = Some(Rc::new(found));
    s.hub_asked = true;
    fetch_icons(s);
}

pub fn opened(s: &mut Store) {
    s.browse.shown = ROWS;
    want_hub(s);
    fetch_icons(s);
}

pub fn fetch_icons(s: &mut Store) {
    let Some(listed) = s.hub.clone() else { return };
    let (urls, size): (Vec<String>, f32) = match &s.view {
        View::Browse => (of_kind(&listed, kinds()[s.browse.kind.min(kinds().len() - 1)].0).into_iter().take(s.browse.shown)
            .map(|f| f.icon.clone()).collect(), theme::apps_list().row_icon),
        View::Home | View::Category(_) | View::Search(_) => (pages::mixed::icons(s), theme::apps_list().tile_icon),
        View::GitHub => (pages::github_search::github_hub_found(s).iter().take(GITHUB_HUB).map(|f| f.icon.clone()).collect(), theme::apps_list().row_icon),
        _ => (Vec::new(), 0.0),
    };
    let px = (size * 2.0) as u32;
    for url in urls.into_iter().filter(|u| !u.is_empty()) {
        pages::pictures::fetch_picture(s, &url, Some(px));
    }
}

fn pick(s: &mut Store, kind: usize) {
    s.browse.kind = kind;
    s.browse.shown = ROWS;
    fetch_icons(s);
}

pub fn page(s: &Store) -> Node<Msg> {
    let listed = s.hub.as_deref().map(|l| &l[..]);
    let words = match listed {
        None => t!("store-browse-fetching").to_string(),
        Some([]) if !s.online => t!("store-browse-offline").to_string(),
        Some([]) => t!("store-browse-failed").to_string(),
        Some(l) => t!("store-browse-line", n = l.len(), count = crate::common::thousands(l.len() as u64)),
    };
    let about = list_card("dialog-information-symbolic", t!("store-browse-about"),
                          vec![note(t!("store-hub-warning")), note(t!("store-from-hint"))]);
    let mut tiles = vec![about];
    if let Some(l) = listed.filter(|l| !l.is_empty()) {
        let all = kinds();
        let kind = s.browse.kind.min(all.len() - 1);
        let named: Vec<(&str, usize)> = all.iter().map(|(code, name)| (*name, of_kind(l, code).len())).collect();
        let tabs = look::tabs(&named, kind, pages::installed::list_room(s.across), |i| go(move |s| pick(s, i)));
        let these = of_kind(l, all[kind].0);
        let mut rows = vec![row(vec![tabs, space()]).centred()];
        if these.is_empty() {
            rows.push(note(t!("store-browse-none")));
        }
        rows.extend(listing::rows(these.iter().take(s.browse.shown).map(|f| column(github_row(s, f)).gap(8.0)).collect()));
        if these.len() > s.browse.shown {
            let next = (these.len() - s.browse.shown).min(ROWS);
            rows.push(row(vec![note(t!("store-browse-showing", shown = s.browse.shown, n = these.len())).width(Length::Fill),
                               look::link(&t!("store-grid-show-more", next = next), go(|s| { s.browse.shown += ROWS; fetch_icons(s); }))])
                .gap(theme::sheet().gap).centred());
        }
        tiles.push(list_card("system-software-install-symbolic", t!("store-browse-list"), rows));
    }
    screen(title_row(t!("store-nav-browse"), Vec::new()), words, tiles)
}

#[cfg(test)]
pub fn made_up(kind: usize) -> Page {
    Page { kind, shown: ROWS }
}
