//! One row of the Installed page, whatever kind of thing it is.

use crate::store::came_from::CameFrom;
use crate::store::catalogue::open_app;
use crate::store::{go, listing, plugin_page, update_page, updates, Msg, Store};
use crate::settings::ui::UPDATE_ICON;
use crate::common::apps::generic_icon_for;
use crate::toolkit::t;
use crate::toolkit::widget::{button, note, row, space, Length, Node, Tone};
use crate::toolkit::theme;
use crate::store::look::{self, Dot};
use super::installed::{plugin_offer, remove_installed};
use super::links::{link_button, red_about};
use super::pictures::{app_icon, file_icon, icon, icon_or, own_icon};
use super::remove_ask::{ask_removing};
use crate::store::catalogue::App;
use crate::store::updates::Source;

pub(super) fn installed_row(s: &Store, item: &updates::Installed) -> Node<Msg> {
    let size = theme::apps_list().row_icon;
    let known = s.apps.get(&item.id).filter(|_| matches!(item.source, Source::Flatpak { .. } | Source::AppImage(_)));
    let picture = row_picture(item, known, size);
    let from = match &item.source {
        Source::AppImage(path) => Some(s.owned.origins.get(path).cloned().unwrap_or(CameFrom::NotKnown)),
        _ => None,
    };
    let up = OwnUpdate::of(s, item);
    let facts = row_facts(s, item, &up, from.as_ref());
    let facts = match from.and_then(|f| f.link()) {
        Some((url, tip)) => row(vec![facts, link_button(tip, &url)]).gap(8.0).centred(),
        None => facts,
    };
    let row_key = item.key();
    let under = said_under(s, item, &up.key);
    let remove = remove_button(s, item, up.going);
    let about = about_button(&row_key, known);
    let open = open_button(item).key(format!("open:{row_key}"));
    let update = update_button(s, item, up);
    listing::row_of(Some(picture), item.name.clone(), facts, under, vec![remove, about, open, update])
}

struct OwnUpdate {
    key: String,
    offer: Option<crate::plugins::catalogue::Offer>,
    has_update: bool,
    newer: Option<String>,
    thing: Option<crate::common::held::Thing>,
    going: Option<f32>,
}

impl OwnUpdate {
    fn of(s: &Store, item: &updates::Installed) -> OwnUpdate {
        let plugin = matches!(item.source, Source::Plugin { .. });
        let flatpak = matches!(item.source, Source::Flatpak { .. });
        let key = if plugin { format!("pl:{}", item.id) } else { update_page::app_key(&item.id) };
        let offer = if plugin { plugin_offer(s, &item.id) } else { None };
        let (has_update, newer) = match &item.source {
            Source::Flatpak { .. } => (s.updatable.contains(&item.id), update_page::newer_version(s, &item.id).map(String::from)),
            Source::Plugin { .. } => (offer.is_some(), offer.as_ref().map(|o| o.entry.version.clone())),
            Source::AppImage(_) | Source::Deb { .. } => (false, None),
        };
        let thing = if plugin { crate::common::held::Thing::plugin(&item.id) } else { crate::common::held::Thing::app(&item.id).filter(|_| flatpak) };
        let thing = thing.filter(|t| s.held.has(t));
        let going = match &item.source {
            Source::Flatpak { .. } => update_page::app_share(s, &item.id),
            Source::Plugin { .. } => s.dials.get(&key).filter(|d| d.running()).map(|d| d.share()),
            Source::AppImage(_) | Source::Deb { .. } => None,
        };
        OwnUpdate { key, offer, has_update, newer, thing, going }
    }
}

fn row_picture(item: &updates::Installed, known: Option<&App>, size: f32) -> Node<Msg> {
    match (&item.source, known) {
        (_, Some(a)) => app_icon(a, size),
        (Source::Flatpak { .. }, None) => own_icon(item.id.clone(), size),
        (Source::AppImage(path), None) => {
            let launcher = crate::icons::appimages::launcher_for(path);
            let named = crate::icons::read_entry(&launcher).get("Icon").cloned().unwrap_or_default();
            if std::path::Path::new(&named).is_file() { file_icon(named.into(), size) } else { icon(generic_icon_for(&item.id, &item.name), size) }
        }
        (Source::Deb { icon, .. }, None) => icon_or(icon.clone().unwrap_or_default(), generic_icon_for(&item.id, &item.name), size),
        (Source::Plugin { icon }, None) => icon_or(icon.clone().unwrap_or_default(), plugin_page::PLAIN_ICON, size),
    }
}

fn row_facts(s: &Store, item: &updates::Installed, up: &OwnUpdate, from: Option<&CameFrom>) -> Node<Msg> {
    let flatpak = matches!(item.source, Source::Flatpak { .. });
    let plugin = matches!(item.source, Source::Plugin { .. });
    let mut bits = vec![item.came_from()];
    bits.extend(from.map(|f| f.words(None)));
    if !item.size.is_empty() {
        bits.push(item.size.clone());
    }
    if !item.version.is_empty() {
        bits.push(t!("store-installed-version", version = &item.version));
    }
    let waits = crate::common::updating::waiting_words(Some(item.version.as_str()), up.newer.as_deref());
    let dot = match (up.going, up.thing.is_some(), up.has_update) {
        (Some(share), _, _) => Dot::Filling(share),
        (None, true, _) => {
            bits.push(if up.has_update { t!("store-installed-held-waits", waits = &waits) } else { t!("store-installed-held").into() });
            Dot::Quiet
        }
        (None, false, true) => {
            bits.push(waits);
            Dot::Waiting
        }
        (None, false, false) if flatpak || (plugin && plugin_page::offered(&s.plugins, &item.id)) => Dot::UpToDate,
        (None, false, false) => Dot::Quiet,
    };
    listing::dotted_row_line(s, dot, bits.join(" \u{00b7} "))
}

fn said_under(s: &Store, item: &updates::Installed, key: &str) -> Vec<Node<Msg>> {
    let plugin = matches!(item.source, Source::Plugin { .. });
    let row_key = item.key();
    let failed = if plugin {
        plugin_page::problem(s, &item.id).map(|w| (w, Tone::Problem))
    } else {
        s.said.get(key).filter(|(w, t)| !w.is_empty() && *t == Tone::Problem).cloned()
    };
    let own_said = s.owned.said.get(&row_key).cloned();
    let page = if own_said.is_some() {
        s.said_on.get(&row_key).copied().unwrap_or_else(|| crate::store::help::of_key(&row_key))
    } else if plugin {
        failed.as_ref().map_or(crate::store::help::PLUGIN, |(w, _)| plugin_page::page_of(w))
    } else {
        s.said_on.get(key).copied().unwrap_or_else(|| crate::store::help::of_key(key))
    };
    own_said.or(failed).map(|(w, t)| if t == Tone::Problem { red_about(w, page) } else { note(w).tone(t) }).into_iter().collect()
}

fn update_button(s: &Store, item: &updates::Installed, up: OwnUpdate) -> Node<Msg> {
    let OwnUpdate { key, offer, has_update, thing, going, .. } = up;
    let room = || space().width(Length::Fixed(theme::icon_button()));
    match (going, thing, &item.source) {
        (Some(share), _, Source::Flatpak { .. }) => button(UPDATE_ICON, t!("store-installed-updating", name = &item.name)).key(key).working(share, Tone::Accent),
        (_, Some(thing), _) => update_page::rows::hold_button(s, &thing, &item.name, &key),
        (None, None, Source::Flatpak { .. }) if has_update => {
            let id = item.id.clone();
            button(UPDATE_ICON, t!("store-installed-update", name = &item.name)).key(key)
                .on_press(go(move |s| update_page::install::update_app(s, &id))).off_because(crate::common::ui::wait_while(update_page::busy(s)))
        }
        (_, None, Source::Plugin { .. }) => match offer {
            Some(o) => plugin_page::job_in(&s.dials, update_page::busy(s), &key, UPDATE_ICON, &t!("store-installed-update", name = &item.name),
                                           move |s| plugin_page::install(s, &o)),
            None => room(),
        },
        _ => room(),
    }
}

fn remove_button(s: &Store, item: &updates::Installed, going: Option<f32>) -> Node<Msg> {
    let row_key = item.key();
    let gone = item.clone();
    if matches!(item.source, Source::Flatpak { .. }) {
        button(look::REMOVE_ICON, t!("store-app-remove")).key(format!("remove:{row_key}")).spoken(t!("store-app-remove"))
            .on_press(go(move |s| ask_removing(s, None, Some(gone.clone()))))
    } else {
        look::remove(s, &format!("remove:{row_key}"), t!("store-app-remove"), move |s| remove_installed(s, &gone, false))
            .off_because(crate::common::ui::wait_while(matches!(item.source, Source::Plugin { .. }) && going.is_some()))
    }
}

fn about_button(row_key: &str, known: Option<&App>) -> Node<Msg> {
    let about = look::about_button(t!("store-installed-about")).key(format!("about:{row_key}"));
    match known.cloned() {
        Some(a) => about.on_press(go(move |s| s.open_detail(a.clone()))),
        None => about.off(true),
    }
}

fn open_button(item: &updates::Installed) -> Node<Msg> {
    let run = item.clone();
    match &item.source {
        Source::Plugin { .. } => button(plugin_page::SETTINGS_ICON, plugin_page::open_settings_tip()).on_press(go(|_| plugin_page::open_settings())),
        Source::Deb { launcher: None, .. } => look::open_button(t!("store-app-open-round")).off(true),
        _ => look::open_button(t!("store-app-open-round")).on_press(go(move |_| match &run.source {
            Source::Flatpak { .. } => open_app(&run.id),
            Source::AppImage(p) => super::image_tile::open_file(p),
            Source::Deb { launcher, .. } => {
                let entry = launcher.as_ref().and_then(|at| crate::common::apps::entries().into_iter().find(|e| e.path == *at));
                if let Some(entry) = entry {
                    crate::common::apps::launch(&entry, &[]);
                }
            }
            Source::Plugin { .. } => {}
        })),
    }
}
