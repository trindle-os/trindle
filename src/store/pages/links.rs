//! Links out of Apps, the way back from an app's page, and the More beside red words.

use crate::store::catalogue::{self};
use crate::store::{go, Msg, Store, View};
use crate::common::run;
use crate::toolkit::t;
use crate::toolkit::widget::{Node, Tone};
use super::Pipe3;

fn s_title(id: &str) -> String {
    crate::settings::panels::all().iter().find(|p| p.id() == id).map(|p| p.title().to_string()).unwrap_or_else(|| t!("store-title").into())
}

pub(crate) fn back_name(view: &View) -> String {
    match view {
        View::Home => t!("store-nav-discover").into(),
        View::Installed => t!("store-nav-installed").into(),
        View::Updates => t!("store-nav-updates").into(),
        View::Plugins => t!("store-nav-plugins").into(),
        View::Browse => t!("store-nav-browse").into(),
        View::GitHub => t!("store-nav-github").into(),
        View::Backup => crate::store::side::move_apps().into(),
        View::Sources => t!("store-nav-sources").into(),
        View::Category(code) => catalogue::categories().iter().find(|(c, _, _)| c == code).map(|(_, n, _)| n.to_string())
            .unwrap_or_else(|| t!("store-nav-discover").into()),
        View::Search(_) => t!("store-back-results").into(),
        View::Hosted(id) => s_title(id),
    }
}

pub(crate) fn link_button(tip: &str, url: &str) -> Node<Msg> {
    let url = url.to_string();
    let button = if tip == t!("store-app-website") { crate::toolkit::widget::link(&url) } else { crate::toolkit::widget::link_named(tip, &url) };
    button.on_press(go(move |s| open_link(s, &url)))
}

pub fn open_link(s: &mut Store, url: &str) {
    if !crate::toolkit::link::is_web(url) {
        return;
    }
    let u = url.to_string();
    run::capture(&["xdg-open", url], None, s.then(move |s: &mut Store, (ok, _o, _e): (bool, String, String)| {
        if !ok {
            s.top = Some((t!("store-no-browser", url = u), Tone::Problem));
        }
    }).pipe3());
}

pub(crate) fn more_about(page: &'static str) -> Msg {
    go(move |_| crate::store::help::open(page))
}

pub(crate) fn more_about_text(page: &'static str, text: String) -> Msg {
    go(move |_| crate::store::help::open_with(page, &text))
}

pub(crate) fn red_about(words: String, page: &'static str) -> Node<Msg> {
    let press = more_about_text(page, words.clone());
    crate::store::help::red(words, press)
}

pub(crate) fn red_cut_about(words: String, page: &'static str) -> Node<Msg> {
    let press = more_about_text(page, words.clone());
    crate::store::help::red_cut(words, press)
}
