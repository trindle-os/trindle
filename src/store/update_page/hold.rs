//! Holding an update back and releasing it, and forgetting what is removed.

use crate::toolkit::t;
use crate::store::plugin_page;
use crate::store::Store;
use crate::common::held::Thing;
use crate::common::run;
use super::words::not_told;
use super::{app_key, busy};

pub(super) fn held_rows_follow(s: &mut Store) {
    let new: Vec<Thing> = s.held.things().filter(|t| !s.upd.held_rows.contains(t)).cloned().collect();
    s.upd.held_rows.extend(new);
}

pub(super) fn keep_held(s: &mut Store, thing: &Thing, hold: bool, key: &str) -> bool {
    let done = if hold { s.held.hold(thing.clone()) } else { s.held.release(thing) };
    let plugin = key.strip_prefix("pl:").map(String::from);
    match done {
        Ok(()) => {
            s.said.remove(key);
            held_rows_follow(s);
            true
        }
        Err(e) => {
            let words = if hold { t!("store-upd-not-held-list", why = e.to_string()) } else { t!("store-upd-not-released-list", why = e.to_string()) };
            match plugin {
                Some(id) => plugin_page::say(s, &id, words),
                None => s.say_on(key, words, crate::store::help::HELD),
            }
            false
        }
    }
}

pub(crate) fn set_held(s: &mut Store, thing: &Thing, hold: bool, key: &str) {
    if busy(s) || !keep_held(s, thing, hold, key) {
        return;
    }
    let Some(pattern) = thing.mask() else { return };
    let mut argv = vec!["flatpak", "mask", "--user"];
    if !hold {
        argv.push("--remove");
    }
    argv.push(&pattern);
    let row = key.to_string();
    let back = s.then(move |s: &mut Store, (ok, err): (bool, String)| {
        if ok || (!hold && err.contains("No current mask")) {
            return;
        }
        s.say_on(&row, not_told(hold, &err), crate::store::help::HELD);
    });
    run::capture(&argv, None, move |ok, _, err| back((ok, err)));
}

pub(super) fn remind_flatpak(s: &Store) {
    let masks = s.held.masks();
    if masks.is_empty() {
        return;
    }
    let mut argv = vec!["flatpak", "mask", "--user"];
    argv.extend(masks.iter().map(String::as_str));
    run::capture(&argv, None, |_, _, _| {});
}

pub(crate) fn removed(s: &mut Store, id: &str) {
    if let Some(thing) = Thing::app(id).filter(|t| s.held.has(t)) {
        set_held(s, &thing, false, &app_key(id));
    }
}
