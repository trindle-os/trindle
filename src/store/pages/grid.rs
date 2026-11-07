//! Discover, a kind, or what a search found, as a grid of apps.

use crate::store::formats::Show;
use crate::store::{go, listing, update_page, Msg, Store, View, PAGE};
use crate::toolkit::t;
use crate::toolkit::widget::{card, choice, column, main_action, note, row, section, Length, Node};
use crate::toolkit::theme;
use crate::toolkit::tiles::empty_state;
use crate::store::screen::{screen, title_row};
use crate::store::look::{self};
use super::links::{more_about, red_about};
use super::mixed;

pub fn grid(s: &Store, head: &str) -> Node<Msg> {
    let home = s.view == View::Home;
    let title = title_row(if home { t!("store-grid-discover") } else { head }, vec![selector(s)]);
    let tiles = mixed::tiles(s);
    let (why, only) = (mixed::why_none(s), s.show == Show::AppImages);
    let foot = why.filter(|_| !only).map(note);
    if tiles.is_empty() {
        return none_page(s, title, foot);
    }
    let words = sentence(s, tiles.len());
    let all = tiles.len();
    let shown: Vec<mixed::Tile> = tiles.into_iter().take(s.shown).collect();
    let mut body = Vec::new();
    if home {
        body.push(section(t!("store-grid-popular")));
    }
    body.push(listing::app_grid(s, &shown));
    if all > shown.len() {
        let next = PAGE.min(all - shown.len());
        body.push(row(vec![note(t!("store-grid-showing", shown = shown.len(), apps = count(all))).width(Length::Fill),
                           look::link(&t!("store-grid-show-more", next = next), go(|s| { s.shown += PAGE; crate::store::browse::fetch_icons(s); }))])
                  .gap(theme::sheet().gap).centred());
    }
    body.extend(foot);
    body.extend(chatgpt_line(s));
    screen(title, words, body)
}

fn count(n: usize) -> String {
    t!("store-grid-apps-count", n = n, count = crate::common::thousands(n as u64))
}

fn sentence(s: &Store, listed: usize) -> String {
    if s.view != View::Home {
        return t!("store-grid-line", apps = count(listed));
    }
    let (flat, images) = mixed::offered(s);
    if flat == 0 {
        return t!("store-grid-home-images", apps = count(images));
    }
    let more = match s.waiting_updates() {
        0 if update_page::unchecked(s) => t!("store-grid-home-unchecked").to_string(),
        0 => t!("store-grid-home-uptodate").to_string(),
        n => t!("store-grid-updates-waiting", n = n, count = crate::common::thousands(n as u64)),
    };
    if images == 0 {
        t!("store-grid-home-line", apps = count(flat), more = more)
    } else {
        t!("store-grid-home-both", apps = count(flat + images), more = more)
    }
}

fn none_page(s: &Store, title: Node<Msg>, foot: Option<Node<Msg>>) -> Node<Msg> {
    let (words, next) = match &s.view {
        View::Search(text) => (t!("store-grid-none-found", text = text), t!("store-grid-try-shorter")),
        _ => (t!("store-grid-none-here").to_string(), t!("store-grid-search-or-look")),
    };
    let only = s.show == Show::AppImages;
    let (words, advice) = match (only, mixed::why_none(s)) {
        (true, Some(why)) => (why.to_string(), false),
        (true, None) if s.hub.is_none() => (t!("store-browse-fetching").to_string(), false),
        _ => (words, true),
    };
    let mut body: Vec<Node<Msg>> = advice.then(|| card(empty_state(next, None))).into_iter().collect();
    body.extend(foot);
    body.extend(chatgpt_line(s));
    screen(title, words, body)
}

fn selector(s: &Store) -> Node<Msg> {
    let names = Show::ALL.iter().map(|x| x.name().to_string()).collect();
    let at = Show::ALL.iter().position(|x| *x == s.show).unwrap_or(0);
    choice(names, at, t!("store-tabs-show"), |i| go(move |s| mixed::set_show(s, Show::ALL[i]))).width(Length::Shrink).key(SELECTOR)
}

pub(crate) const SELECTOR: &str = "apps-show";

pub(crate) fn asks_for_chatgpt(text: &str) -> bool {
    let text = text.to_lowercase();
    ["chatgpt", "openai", "codex"].iter().any(|w| text.contains(w))
}

fn chatgpt_line(s: &Store) -> Option<Node<Msg>> {
    asks_for_chatgpt(search_text(s)).then(|| note(t!("store-chatgpt-returns")))
}

fn search_text(s: &Store) -> &str {
    match &s.view {
        View::Search(text) => text,
        _ => "",
    }
}

pub(crate) const GITHUB_HUB: usize = 30;

pub fn open_without() -> [&'static str; 5] {
    [t!("store-nav-installed"), t!("store-nav-updates"), t!("store-nav-plugins"), t!("store-nav-sources"), crate::store::side::move_apps()]
}

fn still_work(without: Without) -> String {
    let pages = open_without().join(", ");
    match without {
        Without::It => t!("store-still-work-it", pages = pages),
        Without::Flathub => t!("store-still-work-flathub", pages = pages),
        Without::Internet => t!("store-still-work-internet", pages = pages),
    }
}

#[derive(Clone, Copy)]
enum Without {
    It,
    Flathub,
    Internet,
}

pub fn waiting_for_flathub(title: &str, failed: Option<&str>, online: bool) -> Node<Msg> {
    let Some(err) = failed else {
        let ring = crate::toolkit::tiles::loading_state(theme::apps_list().tile_icon * 2.0);
        return screen(title_row(title, Vec::new()), t!("store-wait-fetching"),
                      vec![card(column(vec![ring, note(still_work(Without::It))]).gap(theme::sheet().gap))]);
    };
    let again = main_action("view-refresh-symbolic", t!("store-wait-try-again")).on_press(go(|s| s.load_catalogue()));
    let (words, why) = if online {
        (t!("store-wait-no-reach"), vec![red_about(err.to_string(), crate::store::help::OFFLINE), note(still_work(Without::Flathub))])
    } else {
        (t!("store-wait-offline"),
         vec![crate::store::help::beside(note(t!("store-wait-comes-from")), more_about(crate::store::help::OFFLINE)), note(still_work(Without::Internet))])
    };
    screen(title_row(title, vec![again]), words, vec![card(column(why).gap(theme::sheet().gap))])
}
