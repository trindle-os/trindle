//! The Installed page.

use crate::store::came_from::{self};
use crate::store::{go, listing, plugin_page, update_page, updates, Msg, Store};
use crate::common::run;
use crate::toolkit::t;
use crate::toolkit::widget::{card, column, main_action, note, row, Node, Tone};
use crate::toolkit::theme;
use crate::toolkit::tiles::empty_state;
use crate::store::screen::{screen, title_row};
use crate::store::look::{self};
use std::collections::HashMap;
use super::add_file::{add_appimage, replace_card};
use super::appimage_warning::before;
use super::installed_row::{installed_row};
use super::links::{red_about};
use super::remove_ask::{remove_card};

pub(super) fn tabs() -> [&'static str; updates::KINDS + 1] {
    [t!("store-tab-all"), "Flathub", t!("store-tab-flatpak-files"), t!("store-tab-appimages"), t!("store-tab-deb-files"), t!("store-tab-plugins")]
}

pub(crate) const INSTALLED_TABS: &str = "installed-kinds";

pub(super) fn installed_sentence(total: usize, [flathub, files, appimages, debs, plugins]: [usize; updates::KINDS], waiting: usize) -> String {
    if total == 0 {
        return t!("store-installed-nothing").into();
    }
    let only = |n: usize| n == total;
    let said = if only(plugins) {
        t!("store-installed-only-plugins", n = total)
    } else {
        let start = if plugins > 0 { t!("store-installed-things", n = total) } else { t!("store-installed-programs", n = total) };
        if only(flathub) {
            t!("store-installed-all-flathub", n = total, start = start)
        } else if only(files) {
            t!("store-installed-all-files", n = total, start = start)
        } else if only(appimages) {
            t!("store-installed-all-appimages", n = total, start = start)
        } else if only(debs) {
            t!("store-installed-all-debs", n = total, start = start)
        } else if debs + plugins == 0 {
            let mut parts = Vec::new();
            if flathub > 0 {
                parts.push(t!("store-installed-part-flathub", n = flathub));
            }
            if files > 0 {
                parts.push(t!("store-installed-part-files", n = files));
            }
            if appimages > 0 {
                parts.push(t!("store-installed-part-appimages", n = appimages));
            }
            t!("store-installed-mixed-list", start = start, parts = crate::settings::ui::in_words(&parts))
        } else {
            let mut parts = Vec::new();
            if flathub + files + appimages > 0 {
                parts.push(t!("store-installed-part-apps", n = flathub + files + appimages));
            }
            if debs > 0 {
                parts.push(t!("store-installed-part-debs", n = debs));
            }
            if plugins > 0 {
                parts.push(t!("store-installed-part-plugins", n = plugins));
            }
            t!("store-installed-mixed-kinds", start = start, parts = crate::settings::ui::in_words(&parts))
        }
    };
    let state = match waiting {
        0 if total == 1 => t!("store-installed-state-one").to_string(),
        0 => t!("store-installed-state-all").to_string(),
        n => t!("store-installed-state-waiting", n = n),
    };
    t!("store-installed-line", said = said, state = state)
}

pub(super) fn under_tab(tab: usize, item: &updates::Installed) -> bool {
    tab == 0 || item.kind() + 1 == tab
}

fn everything(s: &Store) -> Option<Vec<updates::Installed>> {
    let p = &s.owned;
    let mut all = p.list.clone()?;
    all.extend(p.debs.as_ref()?.iter().map(updates::Installed::of_deb));
    all.extend(s.plugins.installed.iter().map(|i| {
        updates::Installed::of_plugin(i, plugin_page::picture_of(&s.plugins, &i.id), p.plugin_sizes.get(&i.id).copied().unwrap_or(0))
    }));
    all.sort_by_key(|i| i.name.to_lowercase());
    Some(all)
}

fn update_waits(s: &Store, item: &updates::Installed) -> bool {
    match &item.source {
        updates::Source::Flatpak { .. } | updates::Source::AppImage(_) => s.updatable.contains(&item.id) && !s.held.app(&item.id),
        updates::Source::Plugin { .. } => plugin_offer(s, &item.id).is_some() && !s.held.plugin(&item.id),
        updates::Source::Deb { .. } => false,
    }
}

pub(super) fn plugin_offer(s: &Store, id: &str) -> Option<crate::plugins::catalogue::Offer> {
    let mine = s.plugins.installed.iter().find(|i| i.id == id)?;
    plugin_page::newer_offer(&s.plugins, mine).cloned()
}

pub(crate) fn list_room(across: f32) -> f32 {
    let k = theme::sheet();
    (crate::store::side::page_room(across) - 2.0 * (k.page_padding + k.card_padding)).max(0.0)
}

#[cfg(test)]
pub(crate) fn installed_tabs(s: &Store) -> Vec<(&'static str, usize)> {
    tab_counts(&everything(s).unwrap_or_default())
}

fn tab_counts(list: &[updates::Installed]) -> Vec<(&'static str, usize)> {
    let mut numbers = [list.len(); updates::KINDS + 1];
    numbers[1..].copy_from_slice(&updates::installed_counts(list));
    tabs().into_iter().zip(numbers).collect()
}

pub fn installed(s: &Store) -> Node<Msg> {
    let p = &s.owned;
    let all = everything(s);
    let list = all.as_deref().unwrap_or_default();
    let counts = updates::installed_counts(list);
    let waiting = list.iter().filter(|i| update_waits(s, i)).count();
    let add = main_action("list-add-symbolic", t!("store-installed-add-appimage")).on_press(go(|s| before(s, add_appimage)));
    let named = tab_counts(list);
    let tab = if named[p.tab.min(updates::KINDS)].1 == 0 { 0 } else { p.tab.min(updates::KINDS) };
    let words_in: Vec<String> = p.find.to_lowercase().split_whitespace().map(String::from).collect();
    let shown: Vec<&updates::Installed> = list.iter().filter(|i| under_tab(tab, i))
        .filter(|i| words_in.iter().all(|w| i.name.to_lowercase().contains(w.as_str()) || i.id.to_lowercase().contains(w.as_str())))
        .collect();
    let words = headline(s, all.as_deref(), &shown, !words_in.is_empty(), counts, waiting);
    let mut rows = vec![tabs_and_search(s, &named, tab)];
    rows.extend(cards_above(s));
    rows.extend(list_rows(s, all.as_deref(), &shown));
    screen(title_row(t!("store-nav-installed"), vec![add]), words, vec![card(column(rows).gap(theme::sheet().gap))])
}

fn tabs_and_search(s: &Store, named: &[(&'static str, usize)], tab: usize) -> Node<Msg> {
    row(vec![look::tabs(named, tab, list_room(s.across), |i| go(move |s| s.owned.tab = i)).key(INSTALLED_TABS),
             look::search(&s.owned.find, t!("store-installed-search"), |t| go(move |s| s.owned.find = t.clone()))])
        .gap(theme::sheet().gap).centred().stacks()
}

fn headline(s: &Store, all: Option<&[updates::Installed]>, shown: &[&updates::Installed], narrowed: bool, counts: [usize; updates::KINDS], waiting: usize) -> String {
    let p = &s.owned;
    match (&p.adding, all) {
        (Some((_, Tone::Problem)), _) => t!("store-installed-appimage-failed").to_string(),
        (Some((w, _)), _) => w.clone(),
        (None, _) if waiting > 0 && s.other.is_some() =>
            t!("store-installed-other-busy", who = s.other.map_or("", |who| who.sentence()), back = crate::store::plugin_page::come_back()),
        (None, Some(l)) if narrowed && !shown.is_empty() =>
            if counts[updates::KINDS - 1] == 0 {
                t!("store-installed-match-programs", shown = shown.len(), total = l.len(), find = p.find.trim())
            } else {
                t!("store-installed-match-things", shown = shown.len(), total = l.len(), find = p.find.trim())
            },
        (_, None) => t!("store-looking").to_string(),
        (_, Some(l)) => installed_sentence(l.len(), counts, waiting),
    }
}

fn cards_above(s: &Store) -> Vec<Node<Msg>> {
    let p = &s.owned;
    let mut rows = Vec::new();
    if let Some((t, _)) = &p.appimage_taken {
        rows.push(replace_card(t));
    }
    if let Some(ask) = &p.ask {
        rows.push(remove_card(ask));
    }
    if let Some((w, Tone::Problem)) = &p.adding {
        rows.push(red_about(w.clone(), crate::store::help::INSTALL));
    }
    rows
}

fn list_rows(s: &Store, all: Option<&[updates::Installed]>, shown: &[&updates::Installed]) -> Vec<Node<Msg>> {
    let mut rows = Vec::new();
    match all {
        None => rows.push(crate::settings::ui::loading()),
        Some([]) => rows.push(empty_state(t!("store-installed-empty"), None)),
        Some(_) if shown.is_empty() => rows.push(empty_state(t!("store-installed-no-match", find = s.owned.find.trim()), None)),
        Some(_) => {
            rows.extend(listing::rows(shown.iter().map(|item| installed_row(s, item)).collect()));
            rows.push(note(t!("store-installed-remove-twice")));
            if shown.iter().any(|i| matches!(i.source, updates::Source::AppImage(_))) {
                rows.push(note(t!("store-from-hint")));
            }
        }
    }
    rows
}

pub(crate) fn remove_installed(s: &mut Store, item: &updates::Installed, wipe: bool) {
    if matches!(item.source, updates::Source::Plugin { .. }) && !plugin_page::may_remove() {
        return;
    }
    #[cfg(test)]
    if let Some(noted) = s.owned.removals.clone() {
        noted.borrow_mut().push((item.removal().with_data(wipe), item.id.clone()));
        return removal_done(s, item, Ok(()), false);
    }
    s.owned.said.insert(item.key(), (t!("store-installed-removing").into(), Tone::Plain));
    let gone = item.clone();
    updates::remove(item, wipe, s.then(move |s: &mut Store, res: Result<(), String>| removal_done(s, &gone, res, true)));
}

fn removal_done(s: &mut Store, item: &updates::Installed, res: Result<(), String>, read: bool) {
    let key = item.key();
    match (res, &item.source) {
        (res, updates::Source::Plugin { .. }) => {
            s.owned.said.remove(&key);
            plugin_page::removed(s, &item.id, res, read);
        }
        (Ok(()), updates::Source::Deb { .. }) => {
            s.owned.said.remove(&key);
            if let Some(debs) = s.owned.debs.as_mut() {
                debs.retain(|d| d.package != item.id);
            }
        }
        (Ok(()), _) => {
            s.owned.said.remove(&key);
            update_page::hold::removed(s, &item.id);
            if let Some(list) = s.owned.list.as_mut() {
                list.retain(|i| i.id != item.id);
            }
            if read {
                installed_opened(s);
            }
        }
        (Err(e), _) => {
            s.owned.said.insert(key, (e, Tone::Problem));
        }
    }
}

pub(crate) fn installed_name(s: &Store, id: &str) -> Option<String> {
    s.owned.list.as_deref()?.iter().find(|i| i.id == id).map(|i| i.name.clone())
}

pub fn installed_opened(s: &mut Store) {
    s.refresh_installed(|_| {});
    updates::installed(s.then(|s: &mut Store, list: Vec<updates::Installed>| {
        let files: Vec<std::path::PathBuf> = list.iter().filter_map(|i| match &i.source {
            updates::Source::AppImage(p) => Some(p.clone()),
            _ => None,
        }).collect();
        s.owned.list = Some(list);
        run::on_thread(move || files.into_iter().map(|f| {
            let from = came_from::read(&f.file_name().unwrap_or_default().to_string_lossy());
            (f, from)
        }).collect(), s.then(|s: &mut Store, origins| s.owned.origins = origins));
    }));
    let back = s.then(|s: &mut Store, (debs, sizes): (Vec<crate::store::boxed::added::Program>, HashMap<String, u64>)| {
        s.owned.debs = Some(debs);
        s.owned.plugin_sizes = sizes;
    });
    run::on_thread(|| {
        let sizes = std::fs::read_dir(crate::plugins::folders()).into_iter().flatten().flatten()
            .map(|e| (e.file_name().to_string_lossy().into_owned(), crate::store::boxed::added::size_of_tree(&e.path()))).collect();
        (crate::store::boxed::added::added(), sizes)
    }, back);
}
