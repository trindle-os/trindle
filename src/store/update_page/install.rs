//! Installing updates.

use crate::toolkit::t;
use std::rc::Rc;
use std::cell::RefCell;
use crate::store::pages::job_rows::{begin, settle};
use crate::store::plugin_page;
use crate::store::reason::why_not_updated;
use crate::store::{firmware, progress, updates, Store};
use crate::common::run;
use crate::common::stopping::Stop;
use crate::common::updating::{self, Hold, Part, Who};
use crate::toolkit::widget::Tone;
use super::read::{read_flatpaks, read_system};
use super::rows::{app_name};
use super::stopping::{start_stoppable, stopped};
use super::{APPS_ROW, DEBIAN_ROW, Flat, NEEDS_RESTART, REBOOT_REQUIRED, SYSTEM_ROW, TRINDLE_ROW, app_key, busy, debian_name, flat_held};

pub fn install_all(s: &mut Store) {
    install_all_in(s);
}

pub(super) fn install_all_in(s: &mut Store) {
    if busy(s) {
        return;
    }
    let hold = match updating::begin(Who::Apps) {
        Ok(hold) => hold,
        Err(other) => {
            s.other = Some(other);
            return;
        }
    };
    s.upd.wants_restart = false;
    s.upd.firmware_done = false;
    s.upd.restart = false;
    s.upd.one = None;
    forget_failures(s);
    let firmware: Vec<firmware::Firmware> = s.firmware.iter().filter(|f| !s.held.firmware(&f.id)).cloned().collect();
    for (mine, offer) in plugin_page::waiting(s) {
        let asks_more = !crate::plugins::catalogue::said_to_ask_more(&mine, &offer).is_empty();
        if !s.held.plugin(&mine.id) && !asks_more {
            plugin_page::install_unseen(s, &offer);
        }
    }
    if let Some(names) = flatpaks_for_all(s) {
        s.upd.jobs += 1;
        update_apps(s, hold.clone(), names);
    }
    if updating::needs_password(s.debian.len() + usize::from(s.trindle_waiting.is_some())) {
        s.upd.jobs += 1;
        upgrade_system(s, hold, SYSTEM_ROW, "apt-upgrade", firmware);
    } else if !firmware.is_empty() {
        s.upd.jobs += 1;
        update_firmware(s, hold, firmware);
    }
}

pub(super) fn flatpaks_for_all(s: &Store) -> Option<Vec<String>> {
    if !s.held.any_flatpak() {
        return (!s.upd.flats.is_empty() || !s.updatable.is_empty()).then(Vec::new);
    }
    let names: Vec<String> = s.upd.flats.iter().filter(|f| !flat_held(s, f)).map(Flat::named).collect();
    (!names.is_empty()).then_some(names)
}

pub(super) fn alone(s: &mut Store, name: &str, take: impl FnOnce() -> Result<Hold, Who>) -> Option<Hold> {
    if busy(s) {
        return None;
    }
    match take() {
        Ok(hold) => {
            s.upd.wants_restart = s.upd.wants_restart || s.upd.restart;
            s.upd.one = Some(name.to_string());
            s.upd.jobs += 1;
            Some(hold)
        }
        Err(other) => {
            s.other = Some(other);
            None
        }
    }
}

pub fn update_app(s: &mut Store, id: &str) {
    let name = app_name(s, id);
    let Some(hold) = alone(s, &name, || updating::begin(Who::Apps)) else { return };
    let named = s.upd.flats.iter().find(|f| f.app && f.id == id).map_or_else(|| id.to_string(), Flat::named);
    update_flatpak(s, hold, app_key(id), "--app", named);
}

pub(super) fn update_part(s: &mut Store, key: &str) {
    let Some(part) = s.upd.flats.iter().find(|f| !f.app && f.key() == key).cloned() else { return };
    let Some(hold) = alone(s, &part.plain_name(), || updating::begin(Who::Apps)) else { return };
    update_flatpak(s, hold, part.key(), "--runtime", part.named());
}

pub(super) fn update_trindle(s: &mut Store) {
    let Some(hold) = alone(s, t!("store-upd-trindle"), || updating::begin(Who::Apps)) else { return };
    upgrade_system(s, hold, TRINDLE_ROW, "apt-upgrade-trindle", Vec::new());
}

pub(super) fn update_debian(s: &mut Store) {
    let Some(hold) = alone(s, debian_name(), || updating::begin(Who::Apps)) else { return };
    upgrade_system(s, hold, DEBIAN_ROW, "apt-upgrade-debian", Vec::new());
}

pub(super) fn update_device(s: &mut Store, id: &str) {
    let Some(dev) = s.firmware.iter().find(|f| f.id == id).cloned() else { return };
    let Some(hold) = alone(s, &dev.name, || updating::begin(Who::Apps)) else { return };
    update_firmware(s, hold, vec![dev]);
}

pub(super) fn update_plugin(s: &mut Store, offer: &crate::plugins::catalogue::Offer) {
    if busy(s) {
        return;
    }
    plugin_page::install(s, offer);
}

pub(super) fn forget_failures(s: &mut Store) {
    s.said.retain(|k, _| !(k.starts_with("up:") || k.starts_with("fw:")));
    s.upd.stopped.clear();
}

pub(super) fn job_done(s: &mut Store) {
    s.upd.jobs = s.upd.jobs.saturating_sub(1);
    if s.upd.jobs > 0 {
        return;
    }
    s.upd.one = None;
    s.upd.restart = s.upd.wants_restart || std::path::Path::new(REBOOT_REQUIRED).exists();
    s.upd.waits = crate::common::renewed::said();
    s.refresh_installed(|_| {});
}

fn update_flatpak(s: &mut Store, hold: Hold, key: String, kind: &'static str, named: String) {
    begin(s, &key, |_| {});
    s.said.remove(&key);
    s.working += 1;
    let said: Rc<RefCell<Vec<String>>> = Rc::default();
    let heard = said.clone();
    let job = RefCell::new(progress::Job::default());
    let dial = key.clone();
    let on_line = s.heard(move |s, line| {
        if let Some(at) = job.borrow_mut().heard(line) {
            if let Some(d) = s.dials.get_mut(&dial) {
                d.within(at.share);
            }
        }
        heard.borrow_mut().push(line.to_string());
    });
    let id = named.split('/').next().unwrap_or_default().to_string();
    let stop = Stop::default();
    s.upd.stops.insert(key.clone(), stop.clone());
    s.upd.stopped.remove(&key);
    let was = stop.clone();
    start_stoppable(&stop, &run::line_by_line(&["flatpak", "update", "--user", "-y", kind, &named]), on_line, s.then(move |s: &mut Store, ok: bool| {
        s.job_over();
        if !stopped(s, &key, &was, ok) {
            settle(s, &key, ok, "");
            if ok {
                s.said.remove(&key);
                went_in(s, &key);
                renew_steam(s, &[id]);
            } else {
                s.say(&key, why_not_updated(&said.borrow()), Tone::Problem);
            }
        }
        read_flatpaks(s, true, move |s| {
            job_done(s);
            drop(hold);
        });
    }));
}

fn renew_steam(s: &mut Store, ids: &[String]) {
    if crate::store::steam::touched(ids) && crate::store::steam::running() && run::spawn(&["trindle-apps", "--renew-steam"]) {
        s.upd.steam_waits = true;
    }
}

pub(super) fn went_in(s: &mut Store, key: &str) {
    s.updatable.retain(|id| app_key(id) != key);
    s.upd.flats.retain(|f| f.key() != key);
}

fn update_apps(s: &mut Store, hold: Hold, names: Vec<String>) {
    begin(s, APPS_ROW, |_| {});
    s.working += 1;
    s.upd.app_order.clear();
    s.upd.app_rows = s.updatable.iter().filter(|id| !s.held.app(id))
        .chain(s.upd.flats.iter().filter(|f| !flat_held(s, f)).map(|f| &f.id)).map(|i| (i.clone(), 0.0)).collect();
    let said: Rc<RefCell<Vec<String>>> = Rc::default();
    let heard = said.clone();
    let job = RefCell::new(progress::Job::default());
    let on_line = s.heard(move |s, line| {
        apps_line(s, line, &job);
        heard.borrow_mut().push(line.to_string());
    });
    let going: Vec<String> = s.upd.app_rows.keys().cloned().collect();
    let mut argv = vec!["flatpak", "update", "--user", "-y"];
    argv.extend(names.iter().map(String::as_str));
    let stop = Stop::default();
    s.upd.stops.insert(APPS_ROW.to_string(), stop.clone());
    s.upd.stopped.remove(APPS_ROW);
    let end = AppsEnd { stop: stop.clone(), going, said, hold };
    start_stoppable(&stop, &run::line_by_line(&argv), on_line, s.then(move |s: &mut Store, ok: bool| apps_end(s, ok, end)));
}

fn apps_line(s: &mut Store, line: &str, job: &RefCell<progress::Job>) {
    if let Some(id) = table_row(line) {
        s.upd.app_order.push(id);
    }
    if let Some(at) = job.borrow_mut().heard(line) {
        if let Some(d) = s.dials.get_mut(APPS_ROW) {
            d.within(at.share);
        }
    }
    if let Some((step, within)) = fetching(line) {
        for (i, id) in s.upd.app_order.iter().enumerate() {
            if let Some(share) = s.upd.app_rows.get_mut(id) {
                let now = match (i + 1).cmp(&step) {
                    std::cmp::Ordering::Less => 1.0,
                    std::cmp::Ordering::Equal => within,
                    std::cmp::Ordering::Greater => 0.0,
                };
                *share = share.max(now);
            }
        }
    }
}

struct AppsEnd {
    stop: Stop,
    going: Vec<String>,
    said: Rc<RefCell<Vec<String>>>,
    hold: Hold,
}

fn apps_end(s: &mut Store, ok: bool, end: AppsEnd) {
    let AppsEnd { stop, going, said, hold } = end;
    s.job_over();
    let cut_short = stopped(s, APPS_ROW, &stop, ok);
    if ok {
        renew_steam(s, &going);
    }
    read_flatpaks(s, cut_short, move |s| {
        let still = s.updatable.iter().any(|id| !s.held.app(id)) || s.upd.flats.iter().any(|f| !flat_held(s, f));
        if !cut_short {
            settle(s, APPS_ROW, ok && !still, "");
            s.said.remove(APPS_ROW);
            if !ok || still {
                s.say(APPS_ROW, why_not_updated(&said.borrow()), Tone::Problem);
            }
        }
        s.upd.app_rows.clear();
        job_done(s);
        drop(hold);
    });
}

pub(super) fn table_row(line: &str) -> Option<String> {
    let line = line.trim();
    let (n, rest) = line.split_once('.')?;
    if n.is_empty() || !n.bytes().all(|c| c.is_ascii_digit()) || !rest.contains('<') {
        return None;
    }
    rest.split_whitespace().find(|w| !w.starts_with('[')).map(String::from)
}

pub(super) fn fetching(line: &str) -> Option<(usize, f32)> {
    let (verb, rest) = line.trim().split_once(' ')?;
    if !verb.ends_with("ing") {
        return None;
    }
    let (n, rest) = rest.split_once('/')?;
    let n: usize = n.trim().parse().ok()?;
    let percent = rest.rsplit('%').nth(1).and_then(|before| {
        let digits: String = before.chars().rev().take_while(char::is_ascii_digit).collect::<Vec<_>>().into_iter().rev().collect();
        digits.parse::<f32>().ok()
    });
    Some((n, percent.unwrap_or(0.0) / 100.0))
}

fn upgrade_system(s: &mut Store, hold: Hold, key: &'static str, step: &'static str, firmware: Vec<firmware::Firmware>) {
    begin(s, key, |d| d.by_steps());
    s.said.remove(key);
    s.working += 1;
    if key != TRINDLE_ROW && s.debian.iter().any(|p| NEEDS_RESTART.iter().any(|n| p.starts_with(n))) {
        s.upd.wants_restart = true;
    }
    let on_line = s.heard(move |s, line| {
        if let Some(d) = s.dials.get_mut(key) {
            d.heard(line);
        }
    });
    updates::debian_watching(step, on_line, s.then(move |s: &mut Store, res: Result<(), String>| {
        s.job_over();
        let part = Part::of_admin(res);
        settle(s, key, part == Part::Done, "");
        s.said.remove(key);
        if let Some(why) = part.why() {
            s.say(key, why, Tone::Problem);
        }
        read_system(s, move |s| {
            if firmware.is_empty() {
                job_done(s);
                drop(hold);
            } else {
                update_firmware(s, hold, firmware);
            }
        });
    }));
}

fn update_firmware(s: &mut Store, hold: Hold, mut left: Vec<firmware::Firmware>) {
    if left.is_empty() {
        job_done(s);
        drop(hold);
        return;
    }
    let dev = left.remove(0);
    let key = format!("fw:{}", dev.id);
    begin(s, &key, |d| d.expect(60.0));
    s.said.remove(&key);
    s.working += 1;
    let device = dev.id.clone();
    updates::admin_watching(&["firmware-update", &device], |_| {}, s.then(move |s: &mut Store, res: Result<(), String>| {
        s.job_over();
        let part = Part::of_admin(res);
        settle(s, &key, part == Part::Done, "");
        s.said.remove(&key);
        match part.why() {
            Some(why) => s.say(&key, why, Tone::Problem),
            None => {
                s.upd.wants_restart = true;
                s.upd.firmware_done = true;
                s.firmware.retain(|f| f.id != dev.id);
            }
        }
        update_firmware(s, hold, left);
    }));
}

pub(super) fn restart() {
    if !crate::toolkit::window::persons_own(crate::common::person_only::APPS_RESTART) {
        return;
    }
    run::spawn(&["systemctl", "reboot"]);
}
