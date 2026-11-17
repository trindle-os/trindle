//! The side bar: its places and how high its lines are for the window's height.

use std::rc::Rc;

use crate::toolkit::t;
use crate::toolkit::widget::{column, item, list, row, side_heading, space, Length, Node};
use super::catalogue::categories;
use super::{go, Msg, Store};

pub(super) const SIDE: f32 = 230.0;

pub(super) fn side_room() -> f32 {
    let s = crate::toolkit::theme::sheet();
    2.0 * s.entry_padding + s.icon + s.gap
}

pub(super) fn side_list() -> f32 {
    thread_local! {
        static MEASURED: crate::common::widest::Measured = const { std::cell::RefCell::new(None) };
    }
    crate::common::widest::per_language(&MEASURED, || {
        let mut names: Vec<String> = ["store-nav-discover", "store-nav-github", "store-nav-installed", "store-nav-sources", "store-nav-plugins"]
            .map(|k| crate::toolkit::lang::t(k).to_string()).to_vec();
        names.extend([move_apps().to_string(), t!("store-nav-updates-waiting", n = 999)]);
        names.extend(categories().iter().map(|c| c.1.to_string()));
        names.extend(crate::settings::apps_titles().into_iter().map(str::to_string));
        let widest = names.iter().map(|n| crate::common::widest::words(n)).fold(0.0, f32::max);
        (widest + side_room()).clamp(SIDE, 1.5 * SIDE).ceil()
    })
}

pub(super) fn side_width() -> f32 {
    side_list() + 2.0 * crate::toolkit::theme::sheet().snug_padding
}

pub(super) fn page_room(across: f32) -> f32 {
    (across - side_width() - crate::toolkit::theme::sheet().gap).max(0.0)
}

pub(super) const SIDE_ROW: f32 = 32.0;

pub(super) const SIDE_ROW_LEAST: f32 = 18.0;

const SIDE_HEAD: f32 = 18.0;
pub(super) const SIDE_HEAD_GAP: f32 = 4.0;

const SIDE_HEAD_GAP_LEAST: f32 = 1.0;

pub(super) const SIDE_GAP_LEAST: f32 = 2.0;

const SIDE_SLACK: f32 = 4.0;

pub(super) const SIDE_GAP: f32 = 6.0;
const SIDE_HEAD_INDENT: f32 = 12.0;

pub(crate) fn with_rules<M: 'static>(sections: Vec<Node<M>>) -> Vec<Node<M>> {
    let last = sections.len().saturating_sub(1);
    let mut out = Vec::new();
    for (i, s) in sections.into_iter().enumerate() {
        out.push(s);
        if i < last {
            out.push(crate::toolkit::widget::section_rule());
        }
    }
    out
}

pub(super) fn side_fit(tall: f32, rows: usize, groups: usize, room: f32) -> (f32, f32, f32) {
    if rows == 0 || tall <= 0.0 {
        return (SIDE_ROW, SIDE_GAP, SIDE_HEAD_GAP);
    }
    let (rows, groups) = (rows as f32, groups as f32);

    let rule = crate::toolkit::theme::sheet().section_line_width;
    let left = |gap: f32, under: f32| tall - 2.0 * room - groups * (SIDE_HEAD + under) - (2.0 * gap + rule) * (groups - 1.0).max(0.0) - SIDE_SLACK;
    if left(SIDE_GAP, SIDE_HEAD_GAP) >= rows * SIDE_ROW {
        return (SIDE_ROW, SIDE_GAP, SIDE_HEAD_GAP);
    }
    let under = SIDE_HEAD_GAP_LEAST;
    let heads = groups * (SIDE_HEAD + under);
    let row = (left(SIDE_GAP_LEAST, under) / rows).floor().clamp(SIDE_ROW_LEAST, SIDE_ROW);
    let gap = if groups > 1.0 { (((tall - 2.0 * room - heads - SIDE_SLACK - rows * row) / (groups - 1.0) - rule) / 2.0).floor().clamp(SIDE_GAP_LEAST, SIDE_GAP) } else { SIDE_GAP };
    (row, gap, under)
}

pub(crate) fn move_apps() -> &'static str {
    t!("store-move")
}

pub(super) type Place = (String, String, &'static str);

impl Store {

    pub(super) fn settings_places(&self) -> Vec<Place> {
        self.hosted.as_ref().map(|h| h.pages()).unwrap_or_default().into_iter()
            .map(|(id, title, icon)| (format!("set:{id}"), title.to_string(), icon)).collect()
    }

    pub(super) fn side(&self, settings: Vec<Place>) -> Node<Msg> {
        let n = self.waiting_updates();
        let here = self.view.nav_key();
        let updates = if n == 0 { t!("store-nav-updates").to_string() } else { t!("store-nav-updates-waiting", n = n) };
        let mine: Vec<(String, String, &str)> = vec![
            ("home".into(), t!("store-nav-discover").into(), "starred-symbolic"),
            ("github".into(), t!("store-nav-github").into(), "system-search-symbolic"),
            ("installed".into(), t!("store-nav-installed").into(), crate::settings::ui::INSTALL_ICON),
            ("updates".into(), updates, crate::settings::ui::UPDATE_ICON),
            ("sources".into(), t!("store-nav-sources").into(), "network-server-symbolic"),
            ("plugins".into(), t!("store-nav-plugins").into(), "application-x-addon-symbolic"),
            ("backup".into(), move_apps().into(), crate::settings::ui::SAVE_ICON),
        ];
        let kinds: Vec<(String, String, &str)> = categories().iter().map(|(code, name, icon)| (format!("cat:{code}"), name.to_string(), *icon)).collect();
        let rows = mine.len() + kinds.len() + settings.len();

        let sheet = crate::toolkit::theme::sheet();
        let (line, between, under) = side_fit(self.size.tall, rows, if settings.is_empty() { 2 } else { 3 }, sheet.sidebar_room + sheet.snug_padding);
        let group = |name: &str, places: Vec<(String, String, &str)>| {

            let chosen = places.iter().position(|(k, _, _)| *k == here);
            let rows = places.iter().map(|(_, text, icon)| item(text.clone()).icon(*icon)).collect();
            let keys: Rc<Vec<String>> = Rc::new(places.into_iter().map(|(k, _, _)| k).collect());
            let pick = list(rows, chosen, move |i| {
                let keys = keys.clone();
                go(move |s| s.select_nav(&keys[i]))
            }).called(name).row_height(line).here().width(Length::Fixed(side_list()));
            let head = row(vec![space().width(Length::Fixed(SIDE_HEAD_INDENT)), side_heading(name)]).height(Length::Fixed(SIDE_HEAD));
            column(vec![head, pick]).gap(under)
        };

        let mut groups = vec![group(t!("store-group-apps"), mine)];
        if !settings.is_empty() {
            groups.push(group(t!("store-group-settings"), settings));
        }
        groups.push(group(t!("store-group-categories"), kinds));
        column(with_rules(groups)).gap(between)
    }
}
