//! The page of one AppImage, laid out as an app's page is.

use std::cell::Cell;
use std::rc::Rc;

use crate::store::app_page::{action_places, fact_line, head_of, narrow, part, room, scrolled, Asked, FACTS_PLACE, MAIN_ACTION, WEBSITE};
use crate::store::came_from::CameFrom;
use crate::store::github::{self, Found, Project, Release};
use crate::store::look::{self, Dot};
use crate::store::{age, go, help, Msg, Store};
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::widget::{column, headline, label, main_action, note, page_title, row, space, spinner, Length, Node, Tone};
use super::github_page::fetched_icon;
use super::github_search::{install_repo, look_up, project, with_release};
use super::image_tile::{key_of, open_file, standing, Standing};
use super::install::pressed_install;
use super::links;

pub const MAIN_LINK: &str = "image-page-link";

pub struct Shown {
    pub found: Found,
    pub back: String,
    pub standing: Standing,
    pub problem: Option<String>,
    pub problem_page: &'static str,
    pub release: Asked<Release>,
    pub project: Asked<Project>,
    pub icon: Option<Rc<image::DynamicImage>>,
    pub now: u64,
    pub across: f32,
    pub pulsing: Rc<Cell<bool>>,
}

pub(crate) fn open(s: &mut Store, f: Found) {
    s.headline = None;
    s.detail = None;
    if !f.repo.is_empty() {
        ask_about(s, &f.repo);
    }
    s.image = Some(f);
}

fn ask_about(s: &mut Store, repo: &str) {
    look_up(s, repo);
    if s.github.releases.contains_key(repo) || s.github.unreleased.contains(repo) || !s.github.fetching.insert(repo.to_string()) {
        return;
    }
    let (r, key) = (repo.to_string(), format!("gh:{repo}"));
    with_release(s, repo, move |s, res| {
        s.github.fetching.remove(&r);
        let Err(e) = res else { return };
        if matches!(github::failure(&e), github::Failure::Nothing) {
            s.github.unreleased.insert(r);
        } else {
            s.say(&key, e, Tone::Problem);
        }
    });
}

pub fn gather(s: &Store, f: &Found) -> Shown {
    let key = key_of(f);
    let problem_at = Some(&key).filter(|k| s.said.get(*k).is_some_and(|(words, tone)| *tone == Tone::Problem && !words.is_empty()));
    let release = match s.github.releases.get(&f.repo) {
        Some(r) => Asked::Here(r.clone()),
        None if f.repo.is_empty() || s.github.unreleased.contains(&f.repo) || problem_at.is_some() => Asked::Nothing,
        None => Asked::Waiting,
    };
    let project = match project(s, &f.repo) {
        Some(p) => Asked::Here(p.clone()),
        None if super::github_search::looking_up(s, &f.repo) => Asked::Waiting,
        None => Asked::Nothing,
    };
    Shown {
        found: f.clone(), back: links::back_name(&s.view), standing: standing(s, f), release, project,
        problem: problem_at.and_then(|k| s.said.get(k)).map(|(words, _)| words.clone()),
        problem_page: problem_at.map_or(help::INSTALL, |k| s.page_of(k)),
        icon: s.shots.get(&f.icon).cloned().flatten(),
        now: age::now(), across: s.across, pulsing: s.pulsing.clone(),
    }
}

pub fn laid(v: &Shown) -> Node<Msg> {
    let k = theme::sheet();
    let (wide, narrow) = (room(v.across), narrow(v.across));
    let failure = match &v.problem {
        Some(w) => links::red_cut_about(w.clone(), v.problem_page),
        None => help::red_blank(),
    };
    let back = look::back(&v.back, go(|s| s.image = None));
    let top = column(vec![back, age::updated(updated(v), v.now), head(v, narrow), failure]).gap(k.gap);
    let (facts, about) = (facts(v), about(v));
    let pair = if narrow {
        column(vec![facts, about]).gap(theme::bento().page_gap)
    } else {
        row(vec![facts.height(Length::Fill), about.height(Length::Fill)]).gap(theme::bento().page_gap)
    };
    let cards = column(vec![pair, files(v)]).gap(theme::bento().page_gap);
    scrolled(format!("image-page:{}", key_of(&v.found)), wide, vec![top, cards])
}

fn updated(v: &Shown) -> Option<u64> {
    let release = match &v.release {
        Asked::Here(r) => r.published,
        _ => None,
    };
    release.or(v.found.pushed).or(match &v.project {
        Asked::Here(p) => p.pushed,
        _ => None,
    })
}

fn stars(v: &Shown) -> Option<u64> {
    v.found.stars.or(match &v.project {
        Asked::Here(p) => p.stars,
        _ => None,
    })
}

fn file_of(v: &Shown) -> Option<&github::Download> {
    match &v.release {
        Asked::Here(r) => r.files.iter().find(|f| f.kind == github::Kind::AppImage).or_else(|| r.files.first()),
        _ => None,
    }
}

fn head(v: &Shown, narrow: bool) -> Node<Msg> {
    let f = &v.found;
    let (dot, words) = state_words(v);
    let state = row(vec![look::dot_marking(&v.pulsing, dot), note(words).cut()]).gap(8.0);
    let line = if f.summary.trim().is_empty() { t!("store-appimage-line-bare").to_string() } else { t!("store-appimage-line", summary = &f.summary) };
    let from = note(CameFrom::of(f).words(stars(v))).cut();
    let names = vec![page_title(f.name.clone()).cut(), headline(line).cut(), state, from];
    head_of(fetched_icon(v.icon.clone(), theme::app_page().icon), names, |first| actions(v, first), narrow)
}

fn state_words(v: &Shown) -> (Dot, String) {
    match &v.standing {
        Standing::Going(share, true) => (Dot::Filling(*share), t!("store-app-state-downloading").into()),
        Standing::Going(share, false) => (Dot::Filling(*share), t!("store-app-state-putting-in").into()),
        Standing::Here(_) => (Dot::Quiet, t!("store-list-installed").into()),
        Standing::Absent => match file_of(v).map(|d| d.size).filter(|n| *n > 0) {
            Some(size) => (Dot::Quiet, t!("store-app-state-absent-size", size = github::size_text(size))),
            None => (Dot::Quiet, t!("store-app-state-absent").into()),
        },
    }
}

fn actions(v: &Shown, first: bool) -> Node<Msg> {
    let f = &v.found;
    let k = theme::app_page();
    let round = Length::Fixed(theme::icon_button());
    let link = |key: &str| CameFrom::of(f).link().map(|(url, tip)| links::link_button(tip, &url).key(key.to_string()));
    let pill = |icon: &str, words: &str| main_action(icon, words).width(Length::Fixed(k.action)).key(MAIN_ACTION);
    let (website, main) = match &v.standing {
        Standing::Going(share, true) => (link(WEBSITE), row(vec![look::ring_with_figure(*share, theme::sheet().action_height)])),
        Standing::Going(_, false) => (link(WEBSITE), row(vec![spinner(), label(t!("store-app-busy-installing")).cut()]).gap(8.0)),
        Standing::Here(path) => {
            let path = path.clone();
            (link(WEBSITE), pill("media-playback-start-symbolic", t!("store-app-open")).on_press(go(move |_| open_file(&path))))
        }
        Standing::Absent if !f.repo.is_empty() => {
            let repo = f.clone();
            (link(WEBSITE), pill(crate::settings::ui::INSTALL_ICON, t!("store-app-install")).on_press(go(move |s| pressed_install(s, |s| install_repo(s, &repo)))))
        }
        Standing::Absent => {
            let beside = link(MAIN_LINK).map(|b| row(if first { vec![b, space()] } else { vec![space(), b] }).width(Length::Fixed(k.action)));
            (None, beside.unwrap_or_else(|| space().width(Length::Fixed(k.action))))
        }
    };
    action_places(None, None, website.unwrap_or_else(|| space().width(round)), main, first)
}

fn facts(v: &Shown) -> Node<Msg> {
    let f = &v.found;
    let blank = if matches!(v.release, Asked::Waiting) { "\u{2026}" } else { "\u{2013}" };
    let release = match &v.release {
        Asked::Here(r) => Some(r),
        _ => None,
    };
    let or_dash = |value: Option<String>| value.map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| blank.to_string());
    let created = match &v.project {
        Asked::Waiting => "\u{2026}".to_string(),
        Asked::Here(p) => age::day_or_not_known(p.created.or(f.created)),
        Asked::Nothing => age::day_or_not_known(f.created),
    };
    let stars = match (&v.project, stars(v)) {
        (_, Some(n)) => crate::common::thousands(n),
        (Asked::Waiting, None) => "\u{2026}".to_string(),
        _ => "\u{2013}".to_string(),
    };
    part("dialog-information-symbolic", t!("store-app-facts"), vec![
        fact_line(t!("store-app-fact-download"), or_dash(file_of(v).map(|d| github::size_text(d.size)).filter(|_| release.is_some()))),
        fact_line(t!("store-app-fact-source"), if f.listed { "AppImageHub" } else { "GitHub" }.into()),
        fact_line(t!("store-app-fact-version"), or_dash(release.map(|r| r.tag.clone()))),
        fact_line(t!("store-image-fact-stars"), stars),
        fact_line(t!("store-image-fact-files"), or_dash(release.map(|r| r.files.len().to_string()))),
        fact_line(t!("store-app-fact-created"), created),
    ]).key(FACTS_PLACE)
}

fn about(v: &Shown) -> Node<Msg> {
    let said = v.found.summary.trim();
    let body = if said.is_empty() { note(t!("store-app-no-description")) } else { label(said.to_string()) };
    part("text-x-generic-symbolic", t!("store-app-about"), vec![body, note(t!("store-from-hint"))])
}

fn files(v: &Shown) -> Node<Msg> {
    let body: Vec<Node<Msg>> = match &v.release {
        Asked::Here(r) => r.files.iter().map(|d| {
            let kind = match d.kind { github::Kind::AppImage => "AppImage", github::Kind::Flatpak => "Flatpak" };
            fact_line(&d.name, t!("store-gh-file-line", kind = kind, size = github::size_text(d.size), tag = &r.tag))
        }).collect(),
        Asked::Waiting => vec![note(t!("store-image-files-looking"))],
        Asked::Nothing if v.found.repo.is_empty() => vec![note(t!("store-image-files-site"))],
        Asked::Nothing => vec![note(t!("store-image-files-none"))],
    };
    part("system-software-install-symbolic", t!("store-image-files"), body)
}
