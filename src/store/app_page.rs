//! The page Apps shows for one app, with Install in the same place whatever the app.

use std::cell::Cell;
use std::rc::Rc;

use tiny_skia::{Paint, Rect, Transform};

use super::catalogue::App;
use super::look::{self, Dot};
use super::pages::{self, install::AppFacts, pictures::app_icon};
use super::screen::CARD_GAP;
use super::catalogue::open_app;
use super::side::page_room;
use super::{go, reviews, update_page, Msg, Store};
use crate::toolkit::picture;
use crate::toolkit::t;
use crate::toolkit::theme::{self, alpha};
use crate::toolkit::tiles::{standard_ring_with, tile_head};
use crate::toolkit::widget::{button, card, column, drawing, headline, label, layers, main_action, note, page_title, placed, row, scroll,
                             space, spinner, title, Length, Node, Tone};

pub enum Asked<T> {
    Waiting,
    Nothing,
    Here(T),
}

impl<T> Asked<T> {
    fn here(&self) -> Option<&T> {
        match self {
            Asked::Here(t) => Some(t),
            _ => None,
        }
    }
}

pub enum State {
    Absent,
    Fetching(f32),
    PuttingIn(f32),
    Removing,
    Updating(f32),
    Here { version: String, waiting: bool, newer: String, kept: bool },
}

pub enum Shot {
    Waiting,
    Missed,
    Here(Rc<image::DynamicImage>),
}

pub struct Shown {
    pub app: App,
    pub back: String,
    pub state: State,
    pub problem: Option<String>,
    pub problem_page: &'static str,
    pub held: Option<String>,
    pub busy: bool,
    pub facts: Asked<AppFacts>,
    pub ratings: Asked<reviews::Ratings>,
    pub said: Asked<Vec<reviews::Review>>,
    pub shots: Vec<Shot>,
    pub shot: usize,
    pub online: bool,
    pub ask: Option<pages::remove_ask::RemoveAsk>,
    pub across: f32,
    pub pulsing: Rc<Cell<bool>>,
    pub created: Asked<u64>,
    pub now: u64,
}

const REVIEWS: usize = 3;
const REVIEW_CHARS: usize = 220;
const ALMOST: f32 = 0.97;
use super::listing::BLANK;
fn from_service() -> &'static str {
    t!("store-app-rating-source")
}

pub const ACTION_PLACE: &str = "app-page-action";
pub const MAIN_ACTION: &str = "app-page-main";
pub const WEBSITE: &str = "app-page-website";
pub const OPEN: &str = "app-page-open";
pub const PICTURE_PLACE: &str = "app-page-picture";
pub const FACTS_PLACE: &str = "app-page-facts";
pub const RATING_PLACE: &str = "app-page-rating";

pub fn room(across: f32) -> f32 {
    let k = theme::sheet();
    (page_room(across) - 2.0 * k.page_padding - k.scrollbar_hover - 2.0 * k.scrollbar_inset).max(k.card_padding * 8.0).floor()
}

pub fn narrow(across: f32) -> bool {
    room(across) < theme::app_page().narrow
}

pub fn frame_height(room: f32) -> f32 {
    let k = theme::app_page();
    (room * k.picture_shape).clamp(k.picture_least, k.picture_most).round()
}

pub fn fit_whole(picture: (f32, f32), frame: (f32, f32)) -> (i32, i32, u32, u32) {
    let (iw, ih) = (picture.0.max(1.0), picture.1.max(1.0));
    let (fw, fh) = (frame.0.max(1.0), frame.1.max(1.0));
    let by = (fw / iw).min(fh / ih);
    let (w, h) = ((iw * by).floor().clamp(1.0, fw), (ih * by).floor().clamp(1.0, fh));
    (((fw - w) / 2.0).floor() as i32, ((fh - h) / 2.0).floor() as i32, w as u32, h as u32)
}

pub fn gather(s: &Store, a: &App) -> Shown {
    let id = &a.id;
    let state = if let Some(d) = s.dials.get(id) {
        if s.busy.get(id) == Some(&"uninstall") {
            State::Removing
        } else if d.share() < ALMOST {
            State::Fetching(d.share())
        } else {
            State::PuttingIn(d.share())
        }
    } else if let Some(share) = update_page::app_share(s, id) {
        State::Updating(share)
    } else if let Some((_, version)) = s.installed.get(id) {
        let newer = update_page::newer_version(s, id).unwrap_or_default().to_string();
        State::Here { version: version.clone(), waiting: s.updatable.contains(id), newer, kept: s.held.app(id) }
    } else {
        State::Absent
    };
    let problem_at = [id.clone(), update_page::app_key(id)].into_iter()
        .find(|k| s.said.get(k).is_some_and(|(words, tone)| matches!(tone, Tone::Problem) && !words.is_empty()));
    let problem_page = problem_at.as_deref().map_or(super::help::INSTALL, |k| s.page_of(k));
    let problem = problem_at.and_then(|k| s.said.get(&k).map(|(words, _)| words.clone()));
    let facts = match s.facts.get(id) {
        Some(Some(f)) => Asked::Here(f.clone()),
        Some(None) if s.facts_heard.contains(id) => Asked::Nothing,
        _ => Asked::Waiting,
    };
    let ratings = match s.reviews.get(id) {
        Some(Some(r)) => Asked::Here(r.clone()),
        Some(None) if s.reviews_heard.contains(id) => Asked::Nothing,
        _ => Asked::Waiting,
    };
    let said = match s.said_about.get(id) {
        Some(list) if !list.is_empty() => Asked::Here(list.clone()),
        Some(_) => Asked::Nothing,
        None => Asked::Waiting,
    };
    let shots = a.screenshots.iter().map(|(url, _)| match s.shots.get(url) {
        Some(Some(img)) => Shot::Here(img.clone()),
        _ if s.shots_missed.contains(url) => Shot::Missed,
        _ => Shot::Waiting,
    }).collect();
    let created = match super::pages::github_search::project(s, &a.repo).and_then(|p| p.created) {
        Some(at) => Asked::Here(at),
        None if super::pages::github_search::looking_up(s, &a.repo) => Asked::Waiting,
        None => Asked::Nothing,
    };
    Shown {
        created, now: super::age::now(),
        app: a.clone(), back: pages::links::back_name(&s.view), state, problem, problem_page, facts, ratings, said, shots, shot: s.shot,
        held: s.other.map(|who| who.sentence().to_string()), busy: update_page::busy(s),
        online: s.online, ask: s.owned.ask.clone().filter(|k| k.id == a.id), across: s.across, pulsing: s.pulsing.clone(),
    }
}

pub fn laid(v: &Shown) -> Node<Msg> {
    let k = theme::sheet();
    let (wide, narrow) = (room(v.across), narrow(v.across));
    let back = look::back(&v.back, go(|s| s.detail = None));
    let failure = match &v.problem {
        Some(w) => super::pages::links::red_cut_about(w.clone(), v.problem_page),
        None => super::help::red_blank(),
    };
    let mut top = vec![back, super::age::updated(v.app.released, v.now), head(v, narrow), failure];
    top.extend(v.ask.as_ref().map(pages::remove_ask::remove_card));
    let top = column(top).gap(k.gap);
    let pictures = column(vec![picture(v, wide), picture_buttons(v)]).gap(k.gap);
    let (facts, rating) = (facts(v), rating(v));
    let pair = if narrow {
        column(vec![facts, rating]).gap(theme::bento().page_gap)
    } else {
        row(vec![facts.height(Length::Fill), rating.height(Length::Fill)]).gap(theme::bento().page_gap)
    };
    let cards = column(vec![pair, about(v), people(v)]).gap(theme::bento().page_gap);
    scrolled(format!("app-page:{}", v.app.id), wide, vec![top, pictures, cards])
}

pub(super) fn scrolled(name: String, wide: f32, parts: Vec<Node<Msg>>) -> Node<Msg> {
    let k = theme::sheet();
    scroll(name, column(vec![column(parts).gap(k.gap).width(Length::Fixed(wide))]).padding(k.page_padding)).height(Length::Fill)
}

fn one_line(a: &App) -> String {
    let said = a.summary.trim().trim_end_matches('.');
    let by = a.developer.trim().trim_end_matches('.');
    match (said.is_empty(), by.is_empty()) {
        (true, true) => BLANK.into(),
        (false, true) => t!("store-app-line-said", said = said),
        (true, false) => t!("store-app-line-by", by = by),
        (false, false) => t!("store-app-line-both", said = said, by = by),
    }
}

fn head(v: &Shown, narrow: bool) -> Node<Msg> {
    let a = &v.app;
    let (dot, words) = state_words(v);
    let state = row(vec![look::dot_marking(&v.pulsing, dot), note(words).cut()]).gap(8.0);
    let names = vec![page_title(a.name.clone()).cut(), headline(one_line(a)).cut(), state];
    head_of(app_icon(a, theme::app_page().icon), names, |first| actions(v, first), narrow)
}

pub(super) fn head_of(icon: Node<Msg>, names: Vec<Node<Msg>>, actions: impl Fn(bool) -> Node<Msg>, narrow: bool) -> Node<Msg> {
    let names = column(names).gap(4.0);
    if narrow {
        column(vec![row(vec![icon, names]).gap(CARD_GAP), row(vec![actions(true), space()])]).gap(theme::sheet().gap)
    } else {
        row(vec![icon, names, actions(false)]).gap(CARD_GAP)
    }
}

fn state_words(v: &Shown) -> (Dot, String) {
    match &v.state {
        State::Removing => (Dot::Checking, t!("store-app-state-removing").into()),
        State::Fetching(share) => (Dot::Filling(*share), t!("store-app-state-downloading").into()),
        State::PuttingIn(share) => (Dot::Filling(*share), t!("store-app-state-putting-in").into()),
        State::Updating(share) => (Dot::Filling(*share), t!("store-app-state-updating").into()),
        State::Here { version, waiting: true, newer, kept: true } => (Dot::Quiet, update_page::words::held_words(Some(version), Some(newer))),
        State::Here { version, waiting: false, kept: true, .. } if version.is_empty() => (Dot::Quiet, t!("store-app-state-held").into()),
        State::Here { version, waiting: false, kept: true, .. } => (Dot::Quiet, t!("store-app-state-held-version", version = version)),
        State::Here { version, waiting, newer, .. } => {
            let waits = crate::common::updating::waiting_words(Some(version.as_str()), Some(newer.as_str()));
            match (&v.held, *waiting) {
                (Some(who), true) => (Dot::Checking, who.clone()),
                (_, true) if version.is_empty() => (Dot::Waiting, t!("store-app-state-waiting", waits = waits)),
                (_, true) => (Dot::Waiting, t!("store-app-state-waiting-version", version = version, waits = waits)),
                (_, false) if version.is_empty() => (Dot::UpToDate, t!("store-app-state-current").into()),
                (_, false) => (Dot::UpToDate, t!("store-app-state-current-version", version = version)),
            }
        }
        State::Absent => {
            match v.facts.here().map(|f| f.download.clone()).filter(|d| !d.is_empty()) {
                Some(size) => (Dot::Quiet, t!("store-app-state-absent-size", size = size)),
                None => (Dot::Quiet, t!("store-app-state-absent").into()),
            }
        }
    }
}

fn actions(v: &Shown, first: bool) -> Node<Msg> {
    let a = &v.app;
    let id = a.id.clone();
    let k = theme::app_page();
    let site = if a.homepage.is_empty() { format!("https://flathub.org/apps/{}", a.id) } else { a.homepage.clone() };
    let pill = |icon: &str, words: &str| main_action(icon, words).width(Length::Fixed(k.action)).key(MAIN_ACTION);
    let busy = |words: &str| row(vec![spinner(), label(words.to_string()).cut()]).gap(8.0);
    let mut open = None;
    let (second, main) = match &v.state {
        State::Removing => (None, busy(t!("store-app-busy-removing"))),
        State::Updating(share) => (None, row(vec![look::ring_with_figure(*share, theme::sheet().action_height)])),
        State::Fetching(share) => {
            let i = id.clone();
            (Some(button("process-stop-symbolic", t!("store-app-stop")).on_press(go(move |s| pages::install::give_up(s, &i)))),
             row(vec![look::ring_with_figure(*share, theme::sheet().action_height)]))
        }
        State::PuttingIn(_) => (None, busy(t!("store-app-busy-installing"))),
        State::Here { waiting, kept, .. } => {
            let (gone, o) = (a.clone(), id.clone());
            let remove = button(look::REMOVE_ICON, t!("store-app-remove")).key(format!("remove:{id}")).spoken(t!("store-app-remove"))
                .on_press(go(move |s| pages::remove_ask::ask_removing(s, Some(gone.clone()), None)));
            let main = if let Some(thing) = crate::common::held::Thing::app(&id).filter(|_| *kept) {
                open = Some(look::open_button(t!("store-app-open-round")).key(OPEN).on_press(go(move |_| open_app(&o))));
                let row = update_page::app_key(&id);
                pill(update_page::HOLD_ICON, t!("store-app-release")).on_press(go(move |s| update_page::hold::set_held(s, &thing, false, &row)))
                    .off_because(crate::common::ui::wait_while(v.busy || v.held.is_some()))
            } else if *waiting {
                let i = id.clone();
                open = Some(look::open_button(t!("store-app-open-round")).key(OPEN).on_press(go(move |_| open_app(&o))));
                pill(crate::settings::ui::UPDATE_ICON, t!("store-app-update")).on_press(go(move |s| update_page::install::update_app(s, &i)))
                    .off_because(crate::common::ui::wait_while(v.busy || v.held.is_some()))
            } else {
                pill("media-playback-start-symbolic", t!("store-app-open")).on_press(go(move |_| open_app(&o)))
            };
            (Some(remove), main)
        }
        State::Absent => {
            let i = a.clone();
            (None, pill(crate::settings::ui::INSTALL_ICON, t!("store-app-install")).on_press(go(move |s| pages::install::pressed_install(s, |s| pages::install::install(s, &i)))))
        }
    };
    action_places(open, second, pages::links::link_button(t!("store-app-website"), &site).key(WEBSITE), main, first)
}

pub(super) fn action_places(open: Option<Node<Msg>>, second: Option<Node<Msg>>, website: Node<Msg>, main: Node<Msg>, first: bool) -> Node<Msg> {
    let k = theme::app_page();
    let round = Length::Fixed(theme::icon_button());
    let in_place = |b: Option<Node<Msg>>| match b {
        Some(b) => row(vec![b]).width(round),
        None => space().width(round),
    };
    let place = drawing((k.action, theme::sheet().action_height), "", |_, _| {}).key(ACTION_PLACE);
    let main = layers(vec![place, main]).width(Length::Fixed(k.action)).height(Length::Fixed(theme::sheet().action_height));
    let mut all = vec![in_place(open), in_place(second), website, main];
    if first {
        all.reverse();
    }
    row(all).gap(theme::sheet().gap).width(Length::Shrink)
}

fn picture(v: &Shown, room: f32) -> Node<Msg> {
    let tall = frame_height(room);
    let at = v.shot.min(v.shots.len().saturating_sub(1));
    let shot = v.shots.get(at);
    let img = match shot {
        Some(Shot::Here(img)) => Some(img.clone()),
        _ => None,
    };
    let caption = v.app.screenshots.get(at).map(|(_, c)| c.clone()).unwrap_or_default();
    let frame = drawing((room, tall), caption, move |canvas, pen| {
        let (cw, ch) = (canvas.width() as f32, canvas.height() as f32);
        if let Some(r) = Rect::from_xywh(0.0, 0.0, cw, ch) {
            let mut paint = Paint::default();
            paint.set_color(pen.theme.panel.to_skia());
            canvas.fill_rect(r, &paint, Transform::identity(), None);
        }
        if let Some(img) = &img {
            let (x, y, w, h) = fit_whole((img.width() as f32, img.height() as f32), (cw, ch));
            picture::draw(canvas, img, x, y, w, h);
        }
    }).key(PICTURE_PLACE);
    let words = match shot {
        None => Some(t!("store-app-no-pictures")),
        Some(Shot::Here(_)) => None,
        Some(Shot::Missed) => Some(t!("store-app-picture-missed")),
        Some(Shot::Waiting) if v.online => Some(t!("store-app-picture-fetching")),
        Some(Shot::Waiting) => Some(t!("store-app-picture-offline")),
    };
    let mut over = vec![frame];
    if let Some(words) = words {
        let words = if matches!(shot, Some(Shot::Missed)) || (matches!(shot, Some(Shot::Waiting)) && !v.online) {
            super::help::beside(note(words), super::pages::links::more_about(super::help::OFFLINE))
        } else {
            note(words)
        };
        over.push(placed(words, |(w, h), (aw, ah)| (((aw - w) / 2.0).max(0.0), ((ah - h) / 2.0).max(0.0))));
    }
    layers(over).height(Length::Fixed(tall))
}

fn picture_buttons(v: &Shown) -> Node<Msg> {
    let n = v.shots.len();
    let at = v.shot.min(n.saturating_sub(1));
    let words = match v.app.screenshots.get(at) {
        None => BLANK.to_string(),
        Some((_, caption)) if caption.trim().is_empty() => t!("store-app-picture-of", at = at + 1, n = n),
        Some((_, caption)) => t!("store-app-picture-of-caption", at = at + 1, n = n, caption = caption.trim()),
    };
    let to = |icon: &str, tip: &str, there: Option<usize>| {
        let b = button(icon, tip).off(there.is_none());
        match there {
            Some(i) => b.on_press(go(move |s| s.shot = i)),
            None => b,
        }
    };
    row(vec![to("go-previous-symbolic", t!("store-app-picture-earlier"), at.checked_sub(1)), space(), note(words).cut(), space(),
             to("go-next-symbolic", t!("store-app-picture-later"), (at + 1 < n).then_some(at + 1))])
}

pub(super) fn part<M>(icon: &str, name: &str, body: Vec<Node<M>>) -> Node<M> {
    let mut all = vec![tile_head(icon, name, Vec::new())];
    all.extend(body);
    card(column(all).gap(CARD_GAP))
}

pub(super) fn fact_line<M>(name: &str, value: String) -> Node<M> {
    row(vec![note(name.to_string()).cut().width(Length::Fill), label(value).cut()])
}

fn facts(v: &Shown) -> Node<Msg> {
    let a = &v.app;
    let known = v.facts.here();
    let blank = if matches!(v.facts, Asked::Waiting) { "\u{2026}" } else { "\u{2013}" };
    let or_dash = |value: Option<String>| value.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| blank.to_string());
    let version = match &v.state {
        State::Here { version, .. } if !version.is_empty() => Some(version.clone()),
        _ => known.map(|f| f.version.clone()),
    };
    let free = !a.licence.contains("LicenseRef-proprietary");
    let licence = if a.licence.trim().is_empty() { "\u{2013}".to_string() } else if free { a.licence.clone() } else { t!("store-app-proprietary").into() };
    let made_by = if a.developer.trim().is_empty() { "\u{2013}".to_string() } else { a.developer.trim().to_string() };
    let line = fact_line;
    part("dialog-information-symbolic", t!("store-app-facts"), vec![
        line(t!("store-app-fact-download"), or_dash(known.map(|f| f.download.clone()))),
        line(t!("store-app-fact-source"), "Flathub".into()),
        line(t!("store-app-fact-version"), or_dash(version)),
        line(if free { t!("store-app-fact-licence") } else { t!("store-app-fact-licence-closed") }, licence),
        line(t!("store-app-fact-made-by"), made_by),
        line(t!("store-app-fact-created"), match v.created {
            Asked::Waiting => "\u{2026}".into(),
            Asked::Here(at) => super::age::day(at),
            Asked::Nothing => super::age::day_or_not_known(None),
        }),
    ]).key(FACTS_PLACE)
}

fn bar(filled: f32) -> Node<Msg> {
    let k = theme::app_page();
    drawing((k.bar_name, k.bar), "", move |canvas, pen| {
        let (w, h) = (canvas.width() as f32, canvas.height() as f32);
        for (wide, c) in [(w, alpha(pen.theme.fg, 0.14)), (w * filled.clamp(0.0, 1.0), pen.theme.soft)] {
            if let Some(r) = Rect::from_xywh(0.0, 0.0, wide, h).filter(|_| wide > 0.0) {
                let mut paint = Paint::default();
                paint.set_color(c.to_skia());
                canvas.fill_rect(r, &paint, Transform::identity(), None);
            }
        }
    }).width(Length::Fill)
}

fn rating(v: &Shown) -> Node<Msg> {
    let k = theme::app_page();
    let (share, figure, words) = match &v.ratings {
        Asked::Here(r) => {
            let avg = r.average();
            ((avg / 5.0) as f32, crate::toolkit::lang::format::decimal(avg, 1), t!("store-app-rating-from", stars = reviews::stars_text(avg), n = r.total, count = crate::common::thousands(r.total)))
        }
        Asked::Nothing if v.online => (0.0, "\u{2013}".to_string(), t!("store-app-rating-none").to_string()),
        Asked::Nothing => (0.0, "\u{2013}".to_string(), t!("store-app-rating-none-offline").to_string()),
        Asked::Waiting => (0.0, BLANK.to_string(), t!("store-app-rating-looking").to_string()),
    };
    let counts = v.ratings.here();
    let most = counts.map_or(1, |r| r.total.max(1)) as f32;
    let mut side = vec![note(words).cut()];
    for star in (0..5).rev() {
        let n = counts.map(|r| r.stars[star]);
        side.push(row(vec![note(format!("{} \u{2605}", star + 1)).cut().width(Length::Fixed(k.bar_name)),
                           bar(n.unwrap_or(0) as f32 / most),
                           note(n.map_or_else(|| BLANK.to_string(), crate::common::thousands)).cut().width(Length::Fixed(k.bar_count))]).gap(8.0));
    }
    let ring = row(vec![title(figure).cut().width(Length::Fixed(k.figure)), standard_ring_with(Some(share), None, k.ring)]).width(Length::Shrink);
    let foot = if matches!(v.ratings, Asked::Nothing) && !v.online { t!("store-app-rating-offline") } else { from_service() };
    part("starred-symbolic", t!("store-app-rating"), vec![row(vec![ring, column(side).gap(4.0)]).gap(CARD_GAP), if matches!(v.ratings, Asked::Nothing) && !v.online { super::help::beside(note(foot).cut(), super::pages::links::more_about(super::help::OFFLINE)) } else { super::help::beside_room(note(foot).cut()) }]).key(RATING_PLACE)
}

fn about(v: &Shown) -> Node<Msg> {
    let said = v.app.description.trim();
    let body = if said.is_empty() { note(t!("store-app-no-description")) } else { label(said.to_string()) };
    part("text-x-generic-symbolic", t!("store-app-about"), vec![body])
}

fn people(v: &Shown) -> Node<Msg> {
    let body = match &v.said {
        Asked::Here(said) => column(said.iter().take(REVIEWS).map(|r| {
            let head = format!("{}  {}", reviews::stars_text(r.stars as f64), if r.summary.is_empty() { t!("store-app-review-no-title") } else { &r.summary });
            let mut text: String = r.text.chars().take(REVIEW_CHARS).collect();
            if r.text.chars().count() > REVIEW_CHARS {
                text = format!("{}\u{2026}", text.trim_end());
            }
            column(vec![label(head).cut(), note(text), note(format!("{}, {}", r.who, r.when))]).gap(4.0)
        }).collect()).gap(CARD_GAP),
        Asked::Nothing if v.online => note(t!("store-app-people-none")),
        Asked::Nothing => super::help::beside(note(t!("store-app-people-offline")), super::pages::links::more_about(super::help::OFFLINE)),
        Asked::Waiting => note(t!("store-app-people-looking")),
    };
    part("user-available-symbolic", t!("store-app-people"), vec![body])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    use crate::toolkit::widget::Rect as Place;

    const NAMES: [&str; 3] = ["Photos", "Notes", "A Very Long Name For An App That Goes On And On Past Any Window"];
    const LINES: [&str; 3] = ["Look after your pictures", "",
                              "Does a great many things for a great many people, all at once and all day long, and then some more after that"];

    const NOW: u64 = 1_800_000_000;

    fn made_up(n: usize, shots: usize, words: usize) -> App {
        App {
            id: format!("org.example.App{n}"),
            name: NAMES[n % 3].to_string(),
            summary: LINES[n % 3].to_string(),
            description: "It keeps what you make in one tidy place. ".repeat(words),
            developer: if n % 2 == 0 { "John Smith".into() } else { String::new() },
            licence: ["GPL-3.0-or-later", "", "LicenseRef-proprietary=https://example.com/terms"][n % 3].to_string(),
            categories: Vec::new(),
            icon: None,
            homepage: if n % 2 == 0 { "https://example.com".into() } else { String::new() },
            keywords: String::new(),
            screenshots: (0..shots).map(|i| (format!("https://example.com/{n}/{i}.png"), if i % 2 == 0 { format!("The window, {i}") } else { String::new() })).collect(),
            released: [Some(NOW - 86_400), Some(NOW - 400 * 86_400), None][n % 3],
            repo: String::new(),
        }
    }

    const EDGE: [u8; 3] = [230, 90, 60];
    fn painted(w: u32, h: u32) -> Rc<image::DynamicImage> {
        thread_local! {
            static MADE: std::cell::RefCell<std::collections::HashMap<(u32, u32), Rc<image::DynamicImage>>> = Default::default();
        }
        MADE.with(|made| made.borrow_mut().entry((w, h)).or_insert_with(|| {
            let img = image::RgbaImage::from_fn(w, h, |x, y| {
                let edge = x < 24 || y < 24 || x + 24 >= w || y + 24 >= h;
                if edge { image::Rgba([EDGE[0], EDGE[1], EDGE[2], 255]) } else { image::Rgba([(40 + x * 150 / w) as u8, (70 + y * 120 / h) as u8, 160, 255]) }
            });
            Rc::new(image::DynamicImage::ImageRgba8(img))
        }).clone())
    }

    fn stars() -> reviews::Ratings {
        reviews::Ratings { stars: [3, 5, 12, 40, 90], total: 150 }
    }

    fn written() -> Vec<reviews::Review> {
        vec![reviews::Review { summary: "Does the job".into(), text: "It opened my pictures and kept them in order. ".repeat(9), stars: 4,
                               who: "John Smith".into(), when: "1 October 2026".into() }]
    }

    fn state(n: usize) -> State {
        match n % 10 {
            0 => State::Absent,
            1 => State::Fetching(0.0),
            2 => State::Fetching(0.42),
            3 => State::PuttingIn(0.98),
            4 => State::Removing,
            5 => State::Here { version: "3.2.1".into(), waiting: false, newer: String::new(), kept: false },
            6 => State::Here { version: "2026.10.1-beta.3+build.20261001".into(), waiting: true, newer: "2026.11.1-beta.1+build.20261101".into(), kept: false },
            7 => State::Here { version: "3.2.1".into(), waiting: true, newer: String::new(), kept: false },
            8 => State::Updating(0.42),
            _ => State::Here { version: String::new(), waiting: false, newer: String::new(), kept: false },
        }
    }

    fn shots(n: usize) -> Vec<Shot> {
        match n % 7 {
            0 => Vec::new(),
            1 => vec![Shot::Here(painted(752, 423))],
            2 => vec![Shot::Here(painted(300, 640)), Shot::Waiting, Shot::Here(painted(752, 423))],
            3 => vec![Shot::Here(painted(500, 500)), Shot::Here(painted(752, 423))],
            4 => vec![Shot::Waiting, Shot::Waiting, Shot::Waiting, Shot::Waiting, Shot::Waiting, Shot::Waiting, Shot::Waiting, Shot::Waiting],
            5 => vec![Shot::Missed, Shot::Here(painted(752, 423))],
            _ => vec![Shot::Here(painted(1600, 300))],
        }
    }

    fn case(i: usize, across: f32) -> Shown {
        let shots = shots(i / 3);
        let shot = if shots.is_empty() { 0 } else { (i / 5) % shots.len() };
        Shown {
            app: made_up(i, shots.len(), [0, 2, 12, 300][(i / 2) % 4]),
            back: ["Discover", "Installed", "Results", "Sound and video"][i % 4].into(),
            state: state(i),
            ask: None,
            problem_page: super::super::help::INSTALL,
            problem: (i % 5 == 3).then(|| "error: Failed to install org.example.App: While pulling from the remote: it could not be reached, and so on at some length. ".repeat(1 + i % 3)),
            facts: match (i / 4) % 3 {
                0 => Asked::Here(AppFacts { download: "45 MB".into(), version: "3.2.1".into() }),
                1 => Asked::Waiting,
                _ => Asked::Nothing,
            },
            ratings: match (i / 7) % 3 {
                0 => Asked::Here(stars()),
                1 => Asked::Waiting,
                _ => Asked::Nothing,
            },
            said: match (i / 11) % 3 {
                0 => Asked::Here(written()),
                1 => Asked::Waiting,
                _ => Asked::Nothing,
            },
            held: (i % 9 == 4).then(|| "Updates are being installed from the ring.".to_string()), busy: i % 13 == 6,
            shots, shot, online: i % 6 != 5, across, pulsing: Rc::default(),
            created: match i % 3 { 0 => Asked::Here(0), 1 => Asked::Waiting, _ => Asked::Nothing }, now: NOW,
        }
    }

    fn places(v: &Shown, down: f32) -> Vec<(String, Place)> {
        thread_local! {
            static UI: std::cell::RefCell<Ui<Msg>> = std::cell::RefCell::new(Ui::new(space(), Theme::new(false, "#fbc711")));
        }
        UI.with(|ui| {
            let mut ui = ui.borrow_mut();
            ui.set_view(laid(v));
            ui.places_at((page_room(v.across) as u32, down as u32))
        })
    }

    fn place(all: &[(String, Place)], key: &str) -> Place {
        all.iter().find(|(id, _)| id == key).unwrap_or_else(|| panic!("{key} is on the page")).1
    }

    const SIZES: [(f32, f32); 3] = [(780.0, 560.0), (1100.0, 760.0), (1720.0, 1100.0)];
    const HELD: [&str; 5] = [ACTION_PLACE, WEBSITE, PICTURE_PLACE, FACTS_PLACE, RATING_PLACE];

    #[test]
    fn every_app_and_every_state_keeps_the_same_places() {
        for (across, down) in SIZES {
            let first = places(&case(0, across), down);
            let want: Vec<Place> = HELD.iter().map(|k| place(&first, k)).collect();
            let (action, shot, facts, rating) = (want[0], want[2], want[3], want[4]);
            assert!(action.w > 0.0 && action.h > 0.0 && action.y + action.h < down / 2.0, "{across}: {action:?}");
            assert_eq!((shot.w, shot.h), (room(across), frame_height(room(across))), "{across}");
            assert!(shot.y > action.y + action.h && facts.y > shot.y + shot.h, "{across}: the head, the picture, then the facts");
            if narrow(across) {
                assert!((facts.x, facts.w) == (rating.x, rating.w) && rating.y > facts.y + facts.h, "{across}: one above the other");
            } else {
                assert!((facts.y, facts.h, facts.w) == (rating.y, rating.h, rating.w) && rating.x > facts.x + facts.w, "{across}: side by side");
                assert_eq!(action.x + action.w, shot.x + shot.w, "{across}: the action ends where the page does");
            }
            for i in 0..240 {
                let v = case(i, across);
                let all = places(&v, down);
                for (key, want) in HELD.iter().zip(&want) {
                    assert_eq!(place(&all, key), *want, "{across} wide, case {i}: {key}");
                }
                if let Some((_, pill)) = all.iter().find(|(id, _)| id == MAIN_ACTION) {
                    assert_eq!(*pill, action, "{across} wide, case {i}");
                }
                assert_eq!(matches!(v.state, State::Absent | State::Here { .. }), all.iter().any(|(id, _)| id == MAIN_ACTION), "case {i}");
                assert_eq!(matches!(v.state, State::Here { waiting: true, .. } | State::Here { kept: true, .. }), all.iter().any(|(id, _)| id == OPEN), "case {i}");
                let end = shot.x + shot.w;
                for (id, r) in all.iter().filter(|(id, _)| !id.starts_with("app-page:")) {
                    assert!(r.x >= shot.x - 0.5 && r.x + r.w <= end + 0.5, "{across} wide, case {i}: {id} at {r:?} runs past {end}");
                }
            }
        }
    }

    #[test]
    fn the_arrangement_changes_with_the_windows_width_alone() {
        assert!(narrow(SIZES[0].0) && !narrow(SIZES[1].0) && !narrow(SIZES[2].0));
        let (short, tall) = (places(&case(9, 1100.0), 400.0), places(&case(9, 1100.0), 1400.0));
        for key in HELD {
            assert_eq!(place(&short, key), place(&tall, key), "{key}");
        }
        let k = theme::app_page();
        assert_eq!(frame_height(100.0), k.picture_least);
        assert_eq!(frame_height(10_000.0), k.picture_most);
        assert!(frame_height(room(1100.0)) > k.picture_least && frame_height(room(1100.0)) <= k.picture_most);
    }

    #[test]
    fn a_picture_fits_whole_inside_its_frame() {
        for frame in [(794.0, 397.0), (430.0, 215.0), (1400.0, 420.0), (300.0, 300.0)] {
            for picture in [(752.0, 423.0), (300.0, 640.0), (500.0, 500.0), (1600.0, 300.0), (64.0, 48.0), (1.0, 900.0)] {
                let (x, y, w, h) = fit_whole(picture, frame);
                let (x, y, w, h) = (x as f32, y as f32, w as f32, h as f32);
                assert!(x >= 0.0 && y >= 0.0 && x + w <= frame.0 && y + h <= frame.1, "{picture:?} in {frame:?}: all of it is inside");
                assert!(frame.0 - w < 1.5 || frame.1 - h < 1.5, "{picture:?} in {frame:?}");
                assert!((x - (frame.0 - w - x)).abs() <= 1.0 && (y - (frame.1 - h - y)).abs() <= 1.0, "{picture:?} in {frame:?}");
                assert!((w - h * picture.0 / picture.1).abs() <= 1.5 || (h - w * picture.1 / picture.0).abs() <= 1.5, "{picture:?} in {frame:?}");
            }
        }
    }

    #[test]
    fn a_wide_a_tall_and_a_square_picture_are_each_drawn_whole() {
        for (iw, ih) in [(752, 423), (300, 640), (500, 500), (1600, 300)] {
            for (across, down) in SIZES {
                let mut v = case(0, across);
                v.app = made_up(0, 1, 2);
                v.shots = vec![Shot::Here(painted(iw, ih))];
                let mut ui = Ui::new(laid(&v), Theme::new(false, "#fbc711"));
                let size = (page_room(across) as u32, down as u32 + 400);
                let frame = place(&ui.places_at(size), PICTURE_PLACE);
                let mut pix = tiny_skia::Pixmap::new(size.0, size.1).unwrap();
                ui.paint(&mut pix.as_mut(), 1.0);
                let (x, y, w, h) = fit_whole((iw as f32, ih as f32), (frame.w, frame.h));
                let (x, y, w, h) = (frame.x + x as f32, frame.y + y as f32, w as f32, h as f32);
                let seen = |px: f32, py: f32| {
                    let c = pix.pixel(px as u32, py as u32).unwrap();
                    (c.red(), c.green(), c.blue())
                };
                let near = |c: (u8, u8, u8)| (c.0 as i32 - EDGE[0] as i32).abs() < 40 && (c.1 as i32 - EDGE[1] as i32).abs() < 40 && (c.2 as i32 - EDGE[2] as i32).abs() < 40;
                let (mx, my) = (x + w / 2.0, y + h / 2.0);
                for (side, px, py) in [("left", x + 3.0, my), ("right", x + w - 4.0, my), ("top", mx, y + 3.0), ("bottom", mx, y + h - 4.0)] {
                    assert!(near(seen(px, py)), "{iw}x{ih} at {across}: its {side} edge is drawn, saw {:?}", seen(px, py));
                }
                assert!(!near(seen(mx, my)), "{iw}x{ih} at {across}: its middle is not its edge");
                if w + 12.0 < frame.w {
                    assert!(!near(seen(frame.x + 3.0, my)) && !near(seen(frame.x + frame.w - 4.0, my)), "{iw}x{ih} at {across}: the frame shows either side");
                }
            }
        }
    }

    #[test]
    fn a_part_with_nothing_to_show_says_so_in_its_place() {
        let words = |v: &Shown| {
            fn walk(n: &Node<Msg>, out: &mut Vec<String>) {
                if let crate::toolkit::widget::What::Label { text, .. } = &n.what {
                    out.push(text.clone());
                }
                for item in n.items() {
                    walk(item, out);
                }
            }
            let mut out = Vec::new();
            walk(&laid(v), &mut out);
            out
        };
        let mut v = case(0, 1100.0);
        v.shots = Vec::new();
        v.app = made_up(1, 0, 0);
        v.ratings = Asked::Nothing;
        v.said = Asked::Nothing;
        v.facts = Asked::Nothing;
        let said = words(&v);
        for want in ["This app has no pictures.", "No ratings yet.", "This app has no description.", "Nobody has written about this app yet."] {
            assert!(said.iter().any(|w| w == want), "{want}: {said:?}");
        }
        assert_eq!(said.iter().filter(|w| *w == "\u{2013}").count(), 5, "{said:?}");
        v.online = false;
        v.app = made_up(1, 1, 0);
        v.shots = vec![Shot::Waiting];
        let said = words(&v);
        assert!(said.iter().filter(|w| w.contains("not connected to the internet")).count() == 3, "{said:?}");
        assert!(said.iter().any(|w| w == "No ratings to show."));
        v.shots = vec![Shot::Missed];
        assert!(words(&v).iter().any(|w| w == "This picture could not be fetched."));
        v.online = true;
        v.shots = vec![Shot::Waiting];
        v.ratings = Asked::Waiting;
        v.said = Asked::Waiting;
        let said = words(&v);
        for want in ["Fetching the picture\u{2026}", "Looking for ratings\u{2026}", "Looking for what people say\u{2026}"] {
            assert!(said.iter().any(|w| w == want), "{want}: {said:?}");
        }
    }

    #[test]
    fn the_one_line_names_who_made_it() {
        let mut a = made_up(0, 0, 0);
        assert_eq!(one_line(&a), "Look after your pictures, by John Smith.");
        a.developer.clear();
        assert_eq!(one_line(&a), "Look after your pictures.");
        a.summary.clear();
        assert_eq!(one_line(&a), BLANK);
    }

    #[test]
    #[ignore]
    fn shoot_the_app_page() {
        let dir = std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into());
        std::fs::create_dir_all(&dir).unwrap();
        let with = |mut v: Shown, n: usize, shots: Vec<Shot>, words: usize, state: State, ratings: Asked<reviews::Ratings>| {
            v.app = made_up(n, shots.len(), words);
            v.said = if matches!(ratings, Asked::Here(_)) { Asked::Here(written()) } else { Asked::Nothing };
            v.facts = Asked::Here(AppFacts { download: "45 MB".into(), version: "3.2.1".into() });
            (v.shots, v.shot, v.state, v.ratings, v.problem, v.online) = (shots, 0, state, ratings, None, true);
            (v.held, v.busy) = (None, false);
            v
        };
        let three = || vec![Shot::Here(painted(752, 423)), Shot::Waiting, Shot::Waiting];
        let mut failed = with(case(0, 1100.0), 0, three(), 12, State::Absent, Asked::Here(stars()));
        failed.problem = Some("error: Failed to install org.example.App0: the remote could not be reached".into());
        let mut offline = with(case(0, 1100.0), 1, vec![Shot::Waiting], 2, State::Absent, Asked::Nothing);
        offline.online = false;
        offline.facts = Asked::Nothing;
        let cases: Vec<(&str, f32, f32, Shown)> = vec![
            ("not-installed", 1100.0, 760.0, with(case(0, 1100.0), 0, three(), 12, State::Absent, Asked::Here(stars()))),
            ("installing", 1100.0, 760.0, with(case(0, 1100.0), 0, three(), 12, State::Fetching(0.42), Asked::Here(stars()))),
            ("installed", 1100.0, 760.0, with(case(0, 1100.0), 0, three(), 12, State::Here { version: "3.2.1".into(), waiting: false, newer: String::new(), kept: false }, Asked::Here(stars()))),
            ("failed", 1100.0, 760.0, failed),
            ("installed-tall-picture", 1100.0, 760.0, with(case(0, 1100.0), 1, vec![Shot::Here(painted(300, 640))], 2, State::Here { version: "3.2.1".into(), waiting: true, newer: "3.3".into(), kept: false }, Asked::Nothing)),
            ("no-pictures-long-words", 1100.0, 760.0, with(case(0, 1100.0), 2, Vec::new(), 60, State::PuttingIn(0.98), Asked::Waiting)),
            ("offline", 1100.0, 760.0, offline),
            ("narrow", 780.0, 560.0, with(case(0, 780.0), 0, vec![Shot::Here(painted(500, 500)), Shot::Waiting], 12, State::Absent, Asked::Here(stars()))),
            ("narrow-installed", 780.0, 560.0, with(case(0, 780.0), 2, Vec::new(), 2, State::Here { version: "3.2.1".into(), waiting: true, newer: "3.3".into(), kept: false }, Asked::Nothing)),
            ("wide", 1720.0, 1100.0, with(case(0, 1720.0), 0, three(), 12, State::Here { version: "3.2.1".into(), waiting: false, newer: String::new(), kept: false }, Asked::Here(stars()))),
        ];
        for (name, across, down, v) in cases {
            let w = page_room(across);
            let mut ui = Ui::new(laid(&v), Theme::new(false, "#fbc711"));
            let h = ui.natural_height(w).ceil().max(down) as u32;
            let mut pix = tiny_skia::Pixmap::new(w as u32, h).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            pix.save_png(std::path::Path::new(&dir).join(format!("app-page-{name}.png"))).unwrap();
        }
    }
}
