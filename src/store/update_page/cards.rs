//! The Apps, Plugins and Held back cards.

use crate::toolkit::t;
use crate::store::listing;
use crate::store::look::{self, dot_line, Dot};
use crate::store::plugin_page;
use crate::store::{Msg, Store};
use crate::common::held::{Kind, Thing};
use crate::common::updating::{self};
use crate::toolkit::theme;
use crate::toolkit::widget::{note, Node};
use crate::store::screen::{rows_tile, small};
use super::install::{update_app, update_part, update_plugin};
use super::rows::{Own, app_name, held_row_key, hold_button, red_words, update_line};
use super::view::checking;
use super::words::{from_words, held_with_nothing_newer, held_words, hold_why, nothing_held, release_why, released, size_words, status, with_held};
use super::{APPS, APPS_ROW, Flat, HOLD_ICON, PART_ICON, app_key, app_share, flat_held, going_under, newer_version, part_name, unchecked};

pub(super) fn apps_card(s: &Store) -> Node<Msg> {
    let parts: Vec<&Flat> = s.upd.flats.iter().filter(|f| !f.app).collect();
    let apps: Vec<&String> = s.updatable.iter().filter(|id| !s.held.app(id)).collect();
    let free: Vec<&&Flat> = parts.iter().filter(|f| !flat_held(s, f)).collect();
    let bytes: u64 = apps.iter().filter_map(|i| s.upd.sizes.get(*i)).sum::<u64>() + free.iter().map(|f| f.size).sum::<u64>();
    let waiting = apps.len() + free.len();
    let kept = s.updatable.len() + parts.len() - waiting;
    let (d, words) = status(s, APPS, going_under(s, &["up:app", "up:part:"]), waiting, bytes, kept);
    let mut items = vec![look::status_line(s, d, words, crate::store::help::UPDATE)];
    let all = s.dials.get(APPS_ROW).is_some_and(|d| d.running());
    let size = theme::apps_list().row_icon;
    for id in &s.updatable {
        items.push(app_line(s, id, all, size));
    }
    for f in parts {
        items.push(part_line(s, f, all, size));
    }
    if s.upd.steam_waits {
        items.push(note(t!("store-upd-steam-restarts")));
    }
    items.push(note(t!("store-upd-yours-alone")));
    items.push(note(hold_why()));
    if s.upd.stopped.contains(APPS_ROW) {
        items.push(note(t!("store-upd-stopped-all")));
    }
    items.extend(red_words(s, APPS_ROW));
    let mut head = checking(s, APPS);
    head.extend(s.upd.stop_button(APPS_ROW));
    rows_tile("view-app-grid-symbolic", t!("store-upd-card-apps"), head, items, small())
}

fn app_line(s: &Store, id: &String, all: bool, size: f32) -> Node<Msg> {
    let flat = s.upd.flats.iter().find(|f| f.app && f.id == *id);
    let name = app_name(s, id);
    let from = from_words(flat.map_or("", |f| f.origin.as_str()));
    let have = s.installed.get(id).map(|(_, v)| v.clone()).filter(|v| !v.is_empty());
    let to = newer_version(s, id);
    let detail = match updating::change_words(have.as_deref(), to) {
        Some(change) => t!("store-upd-from-change", from = from, change = change),
        None => t!("store-upd-from", from = from),
    };
    let share = if s.held.app(id) { None } else { app_share(s, id).or(all.then_some(0.0)) };
    let picture = s.apps.get(id).map_or_else(|| crate::store::pages::pictures::own_icon(id.clone(), size), |a| crate::store::pages::pictures::app_icon(a, size));
    let app = id.clone();
    update_line(s, Some(picture), name, detail, held_words(have.as_deref(), to), size_words(s.upd.sizes.get(id).copied()),
                Own::button(s, app_key(id), share, move |s| update_app(s, &app)).holding(Thing::app(id)))
}

fn part_line(s: &Store, f: &Flat, all: bool, size: f32) -> Node<Msg> {
    let key = f.key();
    let share = if flat_held(s, f) {
        None
    } else {
        s.dials.get(&key).filter(|d| d.running()).map(|d| d.share())
            .or_else(|| all.then(|| s.upd.app_rows.get(&f.id).copied().unwrap_or(0.0)))
    };
    let detail = t!("store-upd-shared-part", from = from_words(&f.origin));
    let part = key.clone();
    let name = f.plain_name();
    let called = format!("{name} ({})", f.id);
    update_line(s, Some(crate::store::pages::pictures::icon(PART_ICON.into(), size)), name, detail, held_words(None, Some(&f.version)), size_words(Some(f.size)),
                Own::button(s, key, share, move |s| update_part(s, &part)).holding(f.thing()).called(called))
}

pub(super) fn plugins_card(s: &Store) -> Node<Msg> {
    let waiting = plugin_page::waiting(s);
    let kept = waiting.iter().filter(|(mine, _)| s.held.plugin(&mine.id)).count();
    let ready = waiting.len() - kept;
    let words = match (ready, kept) {
        (0, 0) if unchecked(s) => t!("store-upd-not-looked").to_string(),
        (0, 0) => t!("store-upd-plugins-current").to_string(),
        (0, n) => t!("store-upd-plugins-held", n = n),
        (n, _) => t!("store-upd-sentence", text = with_held(t!("store-upd-plugins-have", n = n), kept)),
    };
    let d = if ready > 0 { Dot::Waiting } else if kept > 0 || unchecked(s) { Dot::Quiet } else { Dot::UpToDate };
    let mut items = vec![dot_line(s, d, words)];
    for (mine, offer) in waiting {
        let key = format!("pl:{}", mine.id);
        let detail = plugin_page::waiting_detail(&mine, &offer);
        let kept = held_words(Some(&mine.version), Some(&offer.entry.version));
        let going = s.dials.get(&key).filter(|d| d.running()).map(|d| d.share());
        let own = Own::Button { key, problem: plugin_page::problem(s, &mine.id), going, press: Box::new(move |s| update_plugin(s, &offer)),
                                hold: Thing::plugin(&mine.id), called: None };
        items.push(update_line(s, None, mine.name.clone(), detail, kept, String::new(), own));
    }
    rows_tile("application-x-addon-symbolic", t!("store-upd-card-plugins"), Vec::new(), items, small())
}

const DEVICE_ICON: &str = "drive-harddisk-symbolic";

const PLUGIN_ICON: &str = "application-x-addon-symbolic";

pub(super) fn held_card(s: &Store) -> Node<Msg> {
    let words = match s.held.len() {
        0 => nothing_held().to_string(),
        n => t!("store-upd-held-back-n", n = n),
    };
    let mut items = vec![dot_line(s, Dot::Quiet, words)];
    let size = theme::apps_list().row_icon;
    for thing in &s.upd.held_rows {
        let id = thing.id();
        let (picture, name, called, have, waits): (Node<Msg>, String, Option<String>, Option<String>, Option<Option<String>>) = match thing.kind() {
            Kind::App => {
                let picture = s.apps.get(id).map_or_else(|| crate::store::pages::pictures::own_icon(id.to_string(), size), |a| crate::store::pages::pictures::app_icon(a, size));
                let have = s.installed.get(id).map(|(_, v)| v.clone());
                let waits = s.updatable.iter().any(|u| u == id).then(|| newer_version(s, id).map(String::from));
                (picture, app_name(s, id), None, have, waits)
            }
            Kind::Part => {
                let flat = s.upd.flats.iter().find(|f| !f.app && f.id == id && f.branch == thing.branch());
                let name = flat.map_or_else(|| part_name("", id, thing.branch()), Flat::plain_name);
                let called = format!("{name} ({id})");
                (crate::store::pages::pictures::icon(PART_ICON.into(), size), name, Some(called), None, flat.map(|f| Some(f.version.clone())))
            }
            Kind::Firmware => {
                let device = s.firmware.iter().find(|f| f.id == id);
                let name = device.map_or_else(|| t!("store-upd-device-firmware").to_string(), |f| f.name.clone());
                let called = device.is_none().then(|| t!("store-upd-device-called", id = id));
                (crate::store::pages::pictures::icon(DEVICE_ICON.into(), size), name, called, device.map(|f| f.current.clone()), device.map(|f| Some(f.newest.clone())))
            }
            Kind::Plugin => {
                let mine = s.plugins.installed.iter().find(|p| p.id == id);
                let waits = mine.and_then(|m| plugin_page::newer_offer(&s.plugins, m)).map(|o| Some(o.entry.version.clone()));
                (crate::store::pages::pictures::icon(PLUGIN_ICON.into(), size), mine.map_or_else(|| id.to_string(), |m| m.name.clone()), None,
                 mine.map(|m| m.version.clone()), waits)
            }
        };
        let line = match (s.held.has(thing), &waits) {
            (false, _) => released().to_string(),
            (true, Some(to)) => held_words(have.as_deref(), to.as_deref()),
            (true, None) => held_with_nothing_newer(have.as_deref()),
        };
        let key = held_row_key(thing);
        let end = vec![hold_button(s, thing, called.as_deref().unwrap_or(&name), &key)];
        items.push(listing::row_of(Some(picture), name, listing::row_line(line), red_words(s, &key).into_iter().collect(), end));
    }
    items.push(note(release_why()));
    rows_tile(HOLD_ICON, t!("store-upd-card-held"), Vec::new(), items, small())
}
