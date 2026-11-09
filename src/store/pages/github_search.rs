//! Searching GitHub and AppImageHub, looking up a project, and installing what it holds.

use crate::store::came_from::CameFrom;
use crate::store::{hub, github, GITHUB_CHECKS, Store, View};
use crate::toolkit::t;
use crate::toolkit::widget::Tone;
use super::add_file::{add_appimage_file};
use super::appimage_warning::before;
use super::github_key::{close_key_box, look_for_key};
use super::job_rows::{begin, settle};

pub(super) fn github_typed(s: &mut Store, text: String) {
    if text == s.github.text {
        return;
    }
    let g = &mut s.github;
    g.text = text;
    g.turn += 1;
    g.asked = false;
    g.found.clear();
    let words: Vec<String> = g.text.to_lowercase().split_whitespace().map(String::from).collect();
    g.hub_found = hub::search(&s.hub.clone().unwrap_or_default(), &words);
    crate::store::browse::fetch_icons(s);
}

pub(super) fn ask_github(s: &mut Store) {
    let words: Vec<String> = s.github.text.to_lowercase().split_whitespace().map(String::from).collect();
    if words.is_empty() {
        return;
    }
    let g = &mut s.github;
    g.turn += 1;
    let turn = g.turn;
    (g.asked, g.waiting, g.error, g.problem, g.checking, g.left) = (true, true, None, None, 0, 0);
    (g.half, g.unread, g.unread_why) = (None, 0, String::new());
    g.found.clear();
    let already: Vec<String> = g.hub_found.iter().map(|f| f.repo.clone()).collect();

    look_for_key(s);
    github::search_github(&words, s.then(move |s: &mut Store, res: Result<github::Searched, String>| {
        if s.github.turn != turn {
            return;
        }
        s.github.waiting = false;
        let list = match res {
            Err(e) => return s.github.error = Some(e),
            Ok(searched) => {

                s.github.half = searched.missed;
                searched.found
            }
        };
        let list: Vec<_> = list.into_iter().filter(|f| !already.contains(&f.repo)).take(GITHUB_CHECKS).collect();
        (s.github.checking, s.github.left) = (list.len(), list.len());
        for f in list {
            let repo = f.repo.clone();
            let back = s.then(move |s: &mut Store, res: Result<github::Release, String>| {
                if s.github.turn != turn {
                    return;
                }
                let g = &mut s.github;
                g.left = g.left.saturating_sub(1);
                match res {
                    Ok(rel) => {
                        g.releases.insert(f.repo.clone(), rel);
                        g.found.push(f);
                    }
                    Err(e) => g.release_failed(e),
                }
            });
            github::release(&repo, back);
        }
    }));
}

pub(super) fn with_release(s: &mut Store, repo: &str, then: impl FnOnce(&mut Store, Result<github::Release, String>) + 'static) {
    if let Some(rel) = s.github.releases.get(repo).cloned() {
        return then(s, Ok(rel));
    }
    let r = repo.to_string();
    github::release(repo, s.then(move |s: &mut Store, res: Result<github::Release, String>| {
        if let Ok(rel) = &res {
            s.github.releases.insert(r, rel.clone());
        }
        then(s, res);
    }));
}

pub(super) fn see_files(s: &mut Store, f: &github::Found) {
    let repo = f.repo.as_str();
    if s.github.open.remove(repo) {
        return;
    }
    if f.listed {
        look_up(s, repo);
    }
    let (r, key) = (repo.to_string(), format!("gh:{repo}"));
    s.github.fetching.insert(r.clone());
    with_release(s, repo, move |s, res| {
        s.github.fetching.remove(&r);
        match res {
            Ok(_) => {
                s.github.open.insert(r);
            }
            Err(e) => s.say(&key, e, Tone::Problem),
        }
    });
}

pub(crate) fn look_up(s: &mut Store, repo: &str) {
    if repo.is_empty() || !s.github.looked_up.insert(repo.to_string()) {
        return;
    }
    let r = repo.to_string();
    github::project(repo, s.then(move |s: &mut Store, res: Result<github::Project, String>| match res {
        Ok(p) => { s.github.projects.insert(r, p); }

        Err(_) => { s.github.looked_up.remove(&r); }
    }));
}

pub(crate) fn github_hub_found(s: &Store) -> &[github::Found] {
    &s.github.hub_found
}

pub(crate) fn project<'a>(s: &'a Store, repo: &str) -> Option<&'a github::Project> {
    s.github.projects.get(repo)
}

pub(crate) fn looking_up(s: &Store, repo: &str) -> bool {
    s.github.looked_up.contains(repo) && !s.github.projects.contains_key(repo)
}

pub(crate) fn install_repo(s: &mut Store, f: &github::Found) {
    if warns(s.github.releases.get(&f.repo), f.listed) {
        let f = f.clone();
        return before(s, move |s| fetch_repo(s, &f));
    }
    fetch_repo(s, f);
}

pub(super) fn warns(release: Option<&github::Release>, listed: bool) -> bool {
    release.and_then(|rel| chosen(rel, listed)).is_none_or(|d| d.kind == github::Kind::AppImage)
}

fn chosen(rel: &github::Release, listed: bool) -> Option<&github::Download> {
    let first = if listed { github::Kind::AppImage } else { github::Kind::Flatpak };
    rel.files.iter().find(|f| f.kind == first).or_else(|| rel.files.first())
}

fn failed(s: &mut Store, key: &str, e: String) {
    if matches!(s.view, View::Home | View::Category(_) | View::Search(_)) && s.image.is_none() {
        s.top = Some((e.clone(), Tone::Problem));
    }
    s.say(key, e, Tone::Problem);
}

fn fetch_repo(s: &mut Store, f: &github::Found) {
    let (repo, listed) = (f.repo.as_str(), f.listed);
    let key = format!("gh:{repo}");
    if listed {
        look_up(s, repo);
    }
    let from = CameFrom::of(f);
    begin(s, &key, |d| d.by_steps());
    s.said.remove(&key);
    with_release(s, repo, move |s, res| {
        let file = res.and_then(|rel| chosen(&rel, listed).cloned().ok_or_else(|| t!("store-gh-nothing-installs").to_string()));
        match file {
            Err(e) => {
                settle(s, &key, false, "");
                failed(s, &key, e);
            }
            Ok(file) => fetch_file(s, &key, &file, from),
        }
    });
}

pub(super) fn install_file(s: &mut Store, key: &str, file: &github::Download, from: CameFrom) {
    if file.kind != github::Kind::AppImage {
        return fetch_file(s, key, file, from);
    }
    let (key, file) = (key.to_string(), file.clone());
    before(s, move |s| fetch_file(s, &key, &file, from));
}

fn fetch_file(s: &mut Store, key: &str, file: &github::Download, from: CameFrom) {
    if !s.dials.contains_key(key) {
        begin(s, key, |d| d.by_steps());
    }
    s.said.remove(key);
    s.working += 1;
    let (k, k2) = (key.to_string(), key.to_string());
    let heard = s.heard(move |s, line| {
        if let Some(d) = s.dials.get_mut(&k) {
            d.heard(line);
        }
    });
    github::install(file, heard, s.then(move |s: &mut Store, res: Result<Option<std::path::PathBuf>, String>| {
        s.job_over();
        settle(s, &k2, res.is_ok(), "");
        match res {
            Ok(fetched) => {
                s.say(&k2, t!("store-added"), Tone::Plain);
                if let Some(file) = fetched {
                    add_appimage_file(s, file, false, from);
                }
            }
            Err(e) => failed(s, &k2, e),
        }
    }));
}

pub fn github_opened(s: &mut Store) {

    close_key_box(s);
    look_for_key(s);
    let pending = s.pending_github.take();
    let search = move |s: &mut Store| {
        if let Some(words) = pending {
            github_typed(s, words);
            ask_github(s);
        }
    };
    if s.hub.is_some() {
        return search(s);
    }
    hub::fetch(s.then(move |s: &mut Store, found: Vec<github::Found>| {
        crate::store::browse::hub_here(s, found);
        let text = std::mem::take(&mut s.github.text);
        github_typed(s, text);
        search(s);
    }));
}
