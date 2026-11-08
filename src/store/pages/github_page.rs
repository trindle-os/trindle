//! The From GitHub page: the search, and a row for each project found.

use crate::store::came_from::CameFrom;
use crate::store::{age, github, go, Msg, Store};
use crate::toolkit::picture;
use crate::toolkit::t;
use crate::toolkit::widget::{button, column, drawing, entry, note, Node};
use crate::toolkit::theme;
use crate::store::screen::{list_card, list_card_with, screen, title_row};
use super::github_key::{key_button};
use super::github_search::{ask_github, github_typed, install_file, install_repo, see_files};
use super::grid::{GITHUB_HUB};
use super::install::pressed_install;
use super::job_rows::{banded, job_button, pictured_row_with, update_row};
use super::links::{link_button, more_about, red_about};
use super::pictures::{icon};
use super::{GitHubPage};

impl GitHubPage {

    pub(super) fn release_failed(&mut self, error: String) {
        match github::failure(&error) {
            github::Failure::Nothing => {}
            github::Failure::Limit => self.problem = Some(error),
            github::Failure::Unread => {
                self.unread += 1;
                self.unread_why = error;
            }
        }
    }

    pub(super) fn problems(&self) -> Vec<String> {
        let mut said: Vec<String> = self.half.iter().map(|why| github::half_words(why)).collect();
        said.extend(self.problem.clone());
        if self.unread > 0 {
            said.push(github::unread_words(self.unread, &self.unread_why));
        }
        said
    }
}

pub fn github(s: &Store) -> Node<Msg> {
    let g = &s.github;
    let mut items = vec![
        entry(g.text.clone(), t!("store-gh-search-placeholder"), |t| go(move |s| github_typed(s, t.clone())))
            .on_submit(go(ask_github)),
    ];
    let mut tiles = Vec::new();
    if !g.text.trim().is_empty() {
        if !g.hub_found.is_empty() {
            let mut rows = vec![note(t!("store-hub-warning")), note(t!("store-from-hint"))];
            rows.extend(banded(g.hub_found.iter().take(GITHUB_HUB).map(|f| column(github_row(s, f)).gap(8.0)).collect()));
            tiles.push(list_card("system-software-install-symbolic", &t!("store-gh-hub", n = g.hub_found.len()), rows));
        }
        if !g.asked {
            items.push(note(t!("store-gh-press-enter")));
        } else if g.error.is_some() && !s.online {

            items.push(crate::store::help::beside(note(t!("store-gh-offline-note")), more_about(crate::store::help::OFFLINE)));
        } else if let Some(e) = &g.error {
            items.push(red_about(e.clone(), github_page(e)));
        } else if g.waiting {
            items.push(note(t!("store-gh-asking")));
        } else if g.checking == 0 {
            items.push(note(t!("store-gh-nothing-more")));
            items.extend(g.problems().into_iter().map(|p| { let page = github_page(&p); red_about(p, page) }));
        } else {
            let problems = g.problems();
            if g.left > 0 {
                items.push(note(t!("store-gh-looking", n = g.checking)));
            } else if problems.is_empty() && g.found.is_empty() {
                items.push(note(t!("store-gh-none-fit")));
            } else {
                items.extend(problems.into_iter().map(|p| { let page = github_page(&p); red_about(p, page) }));
            }
            if !g.found.is_empty() {
                tiles.push(list_card("github-symbolic", t!("store-gh-on"),
                                banded(g.found.iter().map(|f| column(github_row(s, f)).gap(8.0)).collect())));
            }
        }
    }

    if g.asked && !g.waiting {
        items.extend(g.key_is.trouble().map(|why| red_about(why, crate::store::help::GITHUB_KEY)));
    }

    if github::says_unchecked(tiles.len()) {
        items.push(note(github::unchecked()));
    }
    let mut first = vec![list_card_with("system-search-symbolic", t!("store-gh-search-card"), key_button(s), items)];
    first.extend(tiles);

    let words = if s.online { t!("store-gh-line") } else { github_offline() };
    screen(title_row(t!("store-nav-github"), Vec::new()), words, first)
}

pub(crate) fn github_offline() -> &'static str {
    t!("store-gh-offline")
}

pub(super) fn github_page(words: &str) -> &'static str {
    if words == t!("store-gh-key-refused") { crate::store::help::GITHUB_KEY } else { crate::store::help::GITHUB }
}

pub(crate) fn github_row(s: &Store, f: &github::Found) -> Vec<Node<Msg>> {
    let from = CameFrom::of(f);
    let rel = s.github.releases.get(&f.repo);
    let project = s.github.projects.get(&f.repo);
    let mut sub = from.words(f.stars.or(project.and_then(|p| p.stars)));
    if let Some(rel) = rel {
        let has = |kind: github::Kind| rel.files.iter().any(|f| f.kind == kind);
        let kinds = match (has(github::Kind::Flatpak), has(github::Kind::AppImage)) {
            (true, true) => t!("store-gh-both-kinds"),
            (true, false) => "Flatpak",
            (false, true) => "AppImage",
            (false, false) => "",
        };
        sub.push_str(&format!("  {kinds}"));
    }
    if !f.summary.is_empty() {
        sub.push_str(&format!("\n{}", f.summary));
    }
    let ages = if f.pushed.is_some() || f.created.is_some() || rel.is_some() || project.is_some() {
        let updated = rel.and_then(|r| r.published).or(f.pushed).or(project.and_then(|p| p.pushed));
        vec![age::updated(updated, age::now()), note(age::created_words(f.created.or(project.and_then(|p| p.created))))]
    } else {
        Vec::new()
    };
    let key = if f.repo.is_empty() { format!("site:{}", f.site) } else { format!("gh:{}", f.repo) };
    let mut buttons: Vec<Node<Msg>> = from.link().map(|(url, tip)| link_button(tip, &url)).into_iter().collect();
    if !f.repo.is_empty() {
        let open = s.github.open.contains(&f.repo);
        let (found, fetching) = (f.clone(), s.github.fetching.contains(&f.repo));
        let tip = if f.listed { t!("store-browse-details") } else { t!("store-gh-see-files") };
        let look = button("view-reveal-symbolic", tip).on(open);
        buttons.push(if fetching { look } else { look.on_press(go(move |s| see_files(s, &found))) });
        let repo = f.clone();
        let tip = if f.listed { t!("store-browse-add") } else { t!("store-gh-install") };
        buttons.push(job_button(s, &key, crate::settings::ui::INSTALL_ICON, tip, move |s| pressed_install(s, |s| install_repo(s, &repo))));
    }
    let picture = (!f.icon.is_empty()).then(|| hub_icon(s, &f.icon, theme::apps_list().row_icon));
    let mut rows = vec![pictured_row_with(s, picture, &key, f.name.clone(), sub, ages, buttons)];
    if let Some(rel) = rel.filter(|_| s.github.open.contains(&f.repo)) {
        for file in &rel.files {
            let kind = match file.kind { github::Kind::AppImage => "AppImage", github::Kind::Flatpak => "Flatpak" };
            let key = format!("gh:{}/{}", f.repo, file.name);
            let (k, d, repo) = (key.clone(), file.clone(), f.repo.clone());
            let get = job_button(s, &key, crate::settings::ui::INSTALL_ICON, t!("store-gh-install"),
                                 move |s| pressed_install(s, |s| install_file(s, &k, &d, CameFrom::GitHub(repo.clone()))));
            rows.push(update_row(s, &key, file.name.clone(),
                                 t!("store-gh-file-line", kind = kind, size = github::size_text(file.size), tag = &rel.tag), vec![get])
                      .padding(0.0));
        }
    }
    rows
}

pub(crate) fn hub_icon(s: &Store, url: &str, size: f32) -> Node<Msg> {
    fetched_icon(s.shots.get(url).cloned().flatten(), size)
}

pub(crate) fn fetched_icon(img: Option<std::rc::Rc<image::DynamicImage>>, size: f32) -> Node<Msg> {
    match img {
        Some(img) => drawing((size, size), "", move |canvas, _pen| {
            let side = canvas.width().min(canvas.height());
            picture::draw(canvas, &img, 0, 0, side, side);
        }),
        None => icon("application-x-executable-symbolic".into(), size),
    }
}
