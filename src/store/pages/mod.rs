//! The Store's pages.

pub(super) mod add_file;
pub(super) mod appimage_warning;
pub(super) mod backup_page;
pub(super) mod github_key;
pub(super) mod github_page;
pub(super) mod github_search;
pub(super) mod grid;
pub(super) mod install;
pub(super) mod image_page;
pub(super) mod image_tile;
pub(super) mod installed;
pub(super) mod installed_row;
pub(super) mod job_rows;
pub(super) mod links;
pub(super) mod mixed;
pub(super) mod pictures;
pub(super) mod remove_ask;
pub(super) mod sources_page;
#[cfg(test)]
pub(super) mod shots_support;
#[cfg(test)]
mod tests;

use crate::store::came_from::CameFrom;
use crate::store::{github, updates};
use crate::common::sources::{self};
use crate::toolkit::widget::Tone;
use std::collections::{HashMap, HashSet};
use remove_ask::RemoveAsk;
#[cfg(test)]
use std::{cell::RefCell, rc::Rc};

#[derive(Default)]
pub struct GitHubPage {
    text: String,

    hub_found: Vec<github::Found>,

    turn: u64,

    asked: bool,
    waiting: bool,
    error: Option<String>,

    checking: usize,
    left: usize,
    problem: Option<String>,

    half: Option<String>,

    unread: usize,
    unread_why: String,

    found: Vec<github::Found>,
    releases: HashMap<String, github::Release>,

    projects: HashMap<String, github::Project>,
    looked_up: HashSet<String>,

    open: HashSet<String>,
    fetching: HashSet<String>,

    unreleased: HashSet<String>,
    key: String,

    key_is: github::KeyIs,

    key_at: Option<crate::toolkit::widget::Rect>,
}

#[derive(Default)]
pub struct BackupPage {
    no_apps: bool,
    no_data: bool,
    no_appimages: bool,
    no_settings: bool,
    data_size: Option<String>,
    appimages_size: Option<String>,

    waiting: Option<Waiting>,
}

pub struct Waiting {
    path: std::path::PathBuf,
    ticked: bool,
}

#[derive(Default)]
pub struct SourcesPage {

    loaded: bool,

    known: Vec<(&'static sources::Known, bool, bool)>,

    others: Vec<(String, String)>,
    form: SourceForm,
}

#[derive(Clone)]
struct SourceForm {
    uri: String,
    suite: String,
    comps: String,
    key: String,
    package: String,
}

impl Default for SourceForm {
    fn default() -> Self {
        SourceForm { uri: String::new(), suite: String::new(), comps: "main".into(), key: String::new(), package: String::new() }
    }
}

trait Pipe3<A, B, C> {
    fn pipe3(self) -> impl FnOnce(A, B, C) + 'static;
}

impl<A: 'static, B: 'static, C: 'static, F: FnOnce((A, B, C)) + 'static> Pipe3<A, B, C> for F {

    fn pipe3(self) -> impl FnOnce(A, B, C) + 'static {
        move |a, b, c| self((a, b, c))
    }
}

#[derive(Default)]
pub struct InstalledPage {

    list: Option<Vec<updates::Installed>>,

    debs: Option<Vec<super::boxed::added::Program>>,

    plugin_sizes: HashMap<String, u64>,

    tab: usize,

    find: String,

    said: HashMap<String, (String, Tone)>,
    adding: Option<(String, Tone)>,

    appimage_taken: Option<(crate::common::copies::Taken, CameFrom)>,

    origins: HashMap<std::path::PathBuf, CameFrom>,

    pub(super) ask: Option<RemoveAsk>,

    #[cfg(test)]
    pub(super) removals: Option<Rc<RefCell<Vec<(updates::Removal, String)>>>>,
}

#[cfg(test)]
pub(super) mod made_up {
    use super::*;
    use crate::common::sources::KNOWN;

    pub fn installed(list: Option<Vec<updates::Installed>>, tab: usize, find: &str) -> InstalledPage {
        InstalledPage { list, debs: Some(Vec::new()), tab, find: find.into(), ..InstalledPage::default() }
    }

    pub fn said(mut p: InstalledPage, key: &str, words: &str) -> InstalledPage {
        p.said.insert(key.into(), (words.into(), Tone::Problem));
        p
    }

    pub fn with_debs(mut p: InstalledPage, debs: Vec<crate::store::boxed::added::Program>, plugin_sizes: &[(&str, u64)]) -> InstalledPage {
        p.debs = Some(debs);
        p.plugin_sizes = plugin_sizes.iter().map(|(id, n)| (id.to_string(), *n)).collect();
        p
    }

    pub fn from(mut p: InstalledPage, origins: Vec<(std::path::PathBuf, CameFrom)>) -> InstalledPage {
        p.origins = origins.into_iter().collect();
        p
    }

    pub fn looked_at(mut g: GitHubPage, repo: &str, project: github::Project, release: Option<github::Release>) -> GitHubPage {
        g.projects.insert(repo.into(), project);
        g.looked_up.insert(repo.into());
        g.releases.extend(release.map(|r| (repo.to_string(), r)));
        g
    }

    pub fn unreleased(mut g: GitHubPage, repo: &str) -> GitHubPage {
        g.unreleased.insert(repo.into());
        g
    }

    pub fn github(text: &str, hub_found: Vec<github::Found>, found: Vec<github::Found>, asked: bool) -> GitHubPage {
        GitHubPage { text: text.into(), hub_found, checking: found.len(), found, asked, ..GitHubPage::default() }
    }

    pub fn github_key(mut g: GitHubPage, at: Option<crate::toolkit::widget::Rect>, stored: bool, problem: Option<&str>) -> GitHubPage {
        let is = if stored { github::KeyIs::Stored } else { github::KeyIs::None };
        (g.key_at, g.key_is, g.problem) = (at, is, problem.map(String::from));
        g
    }

    pub fn sources() -> SourcesPage {
        SourcesPage {
            loaded: true,
            known: KNOWN.iter().take(5).enumerate().map(|(i, k)| (k, i == 0, i < 2)).collect(),
            others: vec![("example.list".into(), "https://apt.example.org/debian".into())],
            form: SourceForm::default(),
        }
    }

    pub fn backup() -> BackupPage {
        BackupPage { data_size: Some("about 1.2 GB".into()), appimages_size: Some("about 310 MB".into()), ..BackupPage::default() }
    }
}
