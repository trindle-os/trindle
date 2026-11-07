//! Trindle's plugins.

mod background;
pub mod catalogue;
mod fields;
pub mod help;
mod install;
pub mod launcher;
mod log;
mod manifest;
mod menu;
mod notes;
pub mod pin;
mod permissions;
mod ports;
pub mod rows;
pub mod shortcuts;

pub use background::start_background;
pub use fields::{Field, Kind, choices_of, field_shown, note_of};
pub use install::{Background, Installed, remove};
pub use manifest::{Plugin, folder, folders, home_of, pin_command};
pub(crate) use manifest::good_id;
pub use menu::{MenuItem, menu_of};
pub use notes::{DynItem, NOTES, RingNote, ring_note, set_value, unread_notes, values};
pub use permissions::{ACCESSIBILITY, ALLOW_ICON, HOME_NETWORK, PERMISSIONS, USB_DEVICES, allow, allow_word, allowed, ask_again, asked, find, installed, listen_refusal, mark_asked, needs_words, set_allowed, settled_words, standing};
pub use ports::ports_to_open;

use background::{become_new_keeper, Ending, keep_running, lock_handed_on, only_copy, reach_the_screen, restart_background};
use manifest::good_computer;
use notes::good_args;
use permissions::{admin, firewall_apply, firewall_said, open_ports};

use crate::store::boxed;

#[cfg(any(test, feature = "publisher"))]
use std::path::Path;

fn run(id: &str, part: &str, item: &str, rest: &[String]) -> i32 {
    let Some(p) = find(id) else {
        eprintln!("trindle-plugin: {id} is not an installed plugin that conforms");
        return 1;
    };
    let call = Call { id, part, item, rest };
    if matches!(part, "pin" | "unlock") {
        return run_pin(&p, &call);
    }
    permissions::preset_reader(&folders(), id);
    let allowed = allowed(&p);
    let mut lock: Option<i32> = None;
    let mut argv = match part_command(p, &call, &allowed, &mut lock) {
        Ok(argv) => argv,
        Err(code) => return code,
    };
    if matches!(part, "ring" | "menu" | "shortcut" | "args") {
        let _ = crate::common::grants::give(id);
    }
    argv[0] = format!("/app/{}", argv[0]);
    let log = (part == "background").then(|| log::keep(&log::path(id))).flatten();
    let ending = if part == "background" { keep_running(id, &allowed, &argv) } else { Ending::Code(boxed::run::run_boxed(&folder(id), id, &allowed, &argv)) };
    match (ending, lock) {
        (Ending::Renew(program), Some(lock)) => become_new_keeper(id, &program, lock, log),
        (Ending::Code(code), _) => {
            if let Some(log) = log {
                log.finish();
            }
            code
        }
        (Ending::Renew(_), None) => 1,
    }
}

struct Call<'a> {
    id: &'a str,
    part: &'a str,
    item: &'a str,
    rest: &'a [String],
}

fn run_pin(p: &Plugin, call: &Call) -> i32 {
    let Call { id, part, item, .. } = *call;
    let Some(mut argv) = pin_command(id).filter(|_| good_computer(item)) else {
        eprintln!("trindle-plugin: {id} may not unlock a computer");
        return 1;
    };
    if part == "pin" {
        return pin::main_for(id, item);
    }
    argv.push(item.to_string());
    argv[0] = format!("/app/{}", argv[0]);
    boxed::run::run_boxed(&folder(id), id, &allowed(p), &argv)
}

fn part_command(p: Plugin, call: &Call, allowed: &[String], lock: &mut Option<i32>) -> Result<Vec<String>, i32> {
    let Call { id, part, item, rest } = *call;
    let argv = match part {
        "ring" => p.ring.map(|r| r.2),
        "menu" => menu_of(&folder(id).join("app")).into_iter().find(|i| i.id == item).map(|i| i.run),
        "args" => {
            if !good_args(rest) {
                eprintln!("trindle-plugin: those are not arguments a menu item may give");
                return Err(1);
            }
            p.ring.as_ref().and_then(|r| r.2.first().cloned()).map(|program| {
                let mut argv = vec![program];
                argv.extend(rest.iter().cloned());
                argv
            })
        }
        "shortcut" => shortcuts::run_of(&folder(id).join("app"), item),
        "background" if allowed.iter().any(|a| a == "background") => {
            if let Err(why) = reach_the_screen() {
                eprintln!("trindle-plugin: {id}: {why}");
                return Err(1);
            }
            *lock = lock_handed_on(&folder(id), rest).or_else(|| only_copy(id));
            if lock.is_none() {
                return Err(0);
            }
            p.background
        }
        "background" => {
            eprintln!("trindle-plugin: {id} has not been allowed to run from login");
            return Err(1);
        }
        _ => None,
    };
    argv.ok_or_else(|| {
        eprintln!("trindle-plugin: {id} has no {part} part");
        1
    })
}

pub fn main(args: Vec<String>) -> i32 {
    let arg = |i: usize| args.get(i).map(String::as_str).unwrap_or("");
    match (arg(1), arg(2)) {
        ("remove", id) if !id.is_empty() => match remove(id) {
            Ok(()) => 0,
            Err(e) => {
                eprintln!("trindle-plugin: {id}: {e}");
                1
            }
        },
        ("shortcuts", _) => list_shortcuts(),
        ("list", _) => list_plugins(),
        ("run", id) if !id.is_empty() => run(id, arg(3), arg(4), args.get(4..).unwrap_or_default()),
        #[cfg(any(test, feature = "publisher"))]
        ("catalogue", f) if !f.is_empty() => catalogue::publisher::main(Path::new(f)),
        #[cfg(any(test, feature = "publisher"))]
        ("new-key", to) if !to.is_empty() && arg(3).is_empty() => catalogue::publisher::new_key_main(Path::new(to)),
        #[cfg(any(test, feature = "publisher"))]
        ("sign", f) if !f.is_empty() && !arg(3).is_empty() && arg(4).is_empty() => catalogue::publisher::sign_main(Path::new(f), Path::new(arg(3))),
        #[cfg(any(test, feature = "publisher"))]
        ("verify", f) if !f.is_empty() && arg(4).is_empty() => catalogue::publisher::check_main(Path::new(f), Some(arg(3)).filter(|k| !k.is_empty())),
        ("ask", id) if !id.is_empty() => crate::store::asking::plugin_main(id, arg(3) == "again"),
        ("restart", id) if !id.is_empty() => restart_background(id),
        ("firewall", id) if !id.is_empty() => fit_firewall(id),
        ("close", id) if good_id(id) => match admin(&["firewall-plugin-close", id]) {
            Ok(()) => 0,
            Err(e) => {
                eprintln!("trindle-plugin: {id}: its ports could not be closed: {e}");
                1
            }
        },
        _ => {
            eprintln!("trindle-plugin: remove ID, list, shortcuts, run ID ring|background|menu ITEM|shortcut ITEM|pin COMPUTER|unlock COMPUTER, \
                       ask ID [again], restart ID, firewall ID or close ID");
            2
        }
    }
}

fn list_shortcuts() -> i32 {
    let _ = shortcuts::refresh();
    for b in shortcuts::bindings() {
        let keys = b.keys.as_ref().map_or("off".to_string(), |k| k.shown());
        println!("{}\t{}\t{keys}\t{}", b.plugin, b.id, b.refused.as_deref().unwrap_or(""));
    }
    0
}

fn list_plugins() -> i32 {
    for p in installed() {
        println!("{}\t{}\t{}", p.id, p.version, p.name);
    }
    0
}

fn fit_firewall(id: &str) -> i32 {
    let opening = find(id).is_some_and(|p| !open_ports(&p, &allowed(&p)).is_empty());
    match firewall_apply(id) {
        Ok(()) => 0,
        Err(e) => {
            let put_off = if opening { allow(id, "listen", false).map_err(|e| e.to_string()) } else { Ok(()) };
            eprintln!("trindle-plugin: {id}: {}", firewall_said(opening, &e, put_off));
            1
        }
    }
}

#[cfg(test)]
pub use fields::{CHOICES_MAX, CHOICE_TEXT_MAX};
#[cfg(test)]
pub use manifest::NAME_MAX;
#[cfg(test)]
pub(crate) use permissions::mark_asked_in;

#[cfg(test)]
mod fixtures;
