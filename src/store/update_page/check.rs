//! Checking for updates.

use crate::toolkit::t;
use crate::store::catalogue::REMOTE;
use crate::store::plugin_page;
use crate::store::{firmware, updates, Store};
use crate::common::held::{self};
use crate::common::run;
use super::hold::{held_rows_follow, remind_flatpak};
use super::install::{forget_failures, install_all_in};
use super::read::{read_flatpaks, read_system};
use super::words::{CHECK_LIMIT, no_flathub, not_fetched, too_long};
use super::{APPS, FIRMWARE, Phase, SYSTEM, busy, own_busy, phase, refresh_off};

pub fn opened(s: &mut Store) {
    if !own_busy(s) {
        s.held = held::Held::read();
    }
    s.upd.held_card = !s.held.is_empty();
    s.upd.held_rows = s.held.things().cloned().collect();
    looked_again(s);
}

pub fn looked_again(s: &mut Store) {
    if own_busy(s) {
        s.upd.start_asked = false;
        return;
    }
    refresh_off(s);
    s.held = held::Held::read();
    held_rows_follow(s);
    remind_flatpak(s);
    crate::store::pages::installed::installed_opened(s);
    s.upd.reads_left = 2;
    read_flatpaks(s, s.upd.off, read_back);
    read_system(s, read_back);
    plugin_page::opened_by_itself(s);
}

fn read_back(s: &mut Store) {
    s.upd.reads_left = s.upd.reads_left.saturating_sub(1);
    if s.upd.reads_left == 0 {
        start_asked(s);
    }
}

pub(super) fn start_asked(s: &mut Store) {
    if !std::mem::take(&mut s.upd.start_asked) || s.view != crate::store::View::Updates {
        return;
    }
    if phase(s) == Phase::Found && s.firmware.iter().all(|f| s.held.firmware(&f.id)) {
        install_all_in(s);
    }
}

#[cfg(test)]
pub(crate) fn start_is_waiting(s: &Store) -> bool {
    s.upd.start_asked
}

pub(crate) fn ask_start(s: &mut Store) {
    s.upd.start_asked = crate::common::update_asked::take();
}

pub fn check_all(s: &mut Store) {
    if busy(s) {
        return;
    }
    s.online = crate::common::online();
    s.upd.failed.clear();
    s.upd.round += 1;
    let round = s.upd.round;
    let late = s.then(move |s: &mut Store, ()| gave_up(s, round));
    s.events.after(CHECK_LIMIT, move || late(()));
    s.upd.checking.extend([APPS, SYSTEM]);
    let fw = run::have("fwupdmgr");
    if fw {
        s.upd.checking.insert(FIRMWARE);
    }
    forget_failures(s);
    if s.upd.off {
        plugin_page::opened(s);
    }

    run::stream(&run::line_by_line(&["flatpak", "update", "--user", "--appstream", REMOTE]), |_| {},
                s.then(|s: &mut Store, ok: bool| read_flatpaks(s, false, move |s| {
                    if !ok {
                        could_not(s, APPS, no_flathub());
                    }
                    answered(s, APPS);
                })));

    updates::debian_watching("apt-refresh", |_| {}, s.then(|s: &mut Store, res: Result<(), String>| {
        lists_fetched(s, res, true);
        read_system(s, |s| answered(s, SYSTEM));
    }));

    if fw {
        let back = s.then(|s: &mut Store, ok: bool| {
            if ok {
                s.upd.failed.remove(FIRMWARE);
            } else {
                could_not(s, FIRMWARE, t!("store-upd-fw-no-list"));
            }
            firmware::waiting(s.then(|s: &mut Store, list| {
                s.firmware = list;
                answered(s, FIRMWARE);
            }));
        });
        run::capture(&["fwupdmgr", "refresh", "--force", "--no-authenticate"], None, move |ok, _, _| back(ok));
    }
}

pub(super) fn could_not(s: &mut Store, card: &'static str, why: &str) {
    if s.online {
        s.upd.failed.insert(card, why.to_string());
    }
}

pub(crate) fn lists_fetched(s: &mut Store, res: Result<(), String>, asked: bool) -> bool {
    match res {
        Ok(()) => {
            s.upd.failed.remove(SYSTEM);
            true
        }
        Err(e) if !asked && crate::common::refused::of_words(&e) == Some(crate::common::refused::Refused::Cancelled) => false,
        Err(e) => {
            could_not(s, SYSTEM, &not_fetched(&e));
            false
        }
    }
}

pub(super) fn gave_up(s: &mut Store, round: u32) {
    if s.upd.round != round {
        return;
    }
    for card in s.upd.checking.clone() {
        could_not(s, card, too_long());
        answered(s, card);
    }
}

pub(super) fn answered(s: &mut Store, card: &'static str) {
    if !s.upd.checking.remove(card) {
        return;
    }
    if s.upd.checking.is_empty() {
        let now = crate::common::clock::Local::now();
        s.upd.checked_at = Some(now.line(crate::toolkit::lang::format::Pattern::Clock));
        s.upd.waits = crate::common::renewed::said();
    }
}
