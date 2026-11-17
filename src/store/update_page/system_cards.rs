//! The Debian, security and firmware cards.

use crate::toolkit::t;
use crate::store::look::{self, dot_line, Dot};
use crate::store::{go, updates, Msg, Store};
use crate::common::held::Thing;
use crate::common::run;
use crate::toolkit::widget::{note, row, Length, Node};
use crate::store::screen::{rows_tile, small, tile};
use super::install::{update_debian, update_device, update_trindle};
use super::rows::{Own, red_words, update_line};
use super::view::{checking};
use super::words::{debian_why, held_words, never_held, not_held, size_words, status};
use super::{DEBIAN_ROW, SECURITY_ROW, FIRMWARE, FIRST_ROWS, MORE_ROWS, SYSTEM, SYSTEM_ROW, TRINDLE_ROW, debian_name, going_under, job_ring, ring_under};

pub(super) fn system_card(s: &Store) -> Node<Msg> {
    let mut names: Vec<&str> = s.debian.iter().map(String::as_str).collect();
    let debian_bytes: u64 = names.iter().filter_map(|n| s.upd.sizes.get(*n)).sum();
    if s.trindle_waiting.is_some() {
        names.push("trindle");
    }
    let bytes: u64 = names.iter().filter_map(|n| s.upd.sizes.get(*n)).sum();
    let waiting = s.debian.len() + s.trindle_waiting.is_some() as usize;
    let (d, words) = status(s, SYSTEM, going_under(s, &[SYSTEM_ROW, TRINDLE_ROW, DEBIAN_ROW]), waiting, bytes, 0);
    let mut items = vec![look::status_line(s, d, words, crate::store::help::UPDATE)];

    let going = |own: &str| ring_under(s, &[SYSTEM_ROW, own]);
    let current = updates::shown(updates::trindle_version());
    if let Some(latest) = &s.trindle_waiting {
        items.push(update_line(s, None, t!("store-upd-trindle").into(), t!("store-upd-version-to", from = &current, to = updates::shown(latest)), String::new(),
                               size_words(s.upd.sizes.get("trindle").copied()), Own::button(s, TRINDLE_ROW.into(), going(TRINDLE_ROW), update_trindle)));
    }
    if !s.debian.is_empty() {

        let n = t!("store-upd-debian-count", n = s.debian.len());
        items.push(update_line(s, None, debian_name().into(), n, String::new(), size_words(Some(debian_bytes)),
                               Own::button(s, DEBIAN_ROW.into(), going(DEBIAN_ROW), update_debian)));
    }
    let shown = if s.upd.shown == 0 { FIRST_ROWS } else { s.upd.shown };
    for p in s.debian.iter().take(shown) {
        items.push(update_line(s, None, p.clone(), t!("store-upd-from-debian").into(), String::new(), size_words(s.upd.sizes.get(p).copied()), Own::WithGroup));
    }
    if s.debian.len() > shown {
        items.push(row(vec![note(t!("store-upd-debian-showing", shown = shown, total = s.debian.len())).width(Length::Fill),
                            look::link(&t!("store-grid-show-more", next = MORE_ROWS.min(s.debian.len() - shown)),
                                       go(move |s| s.upd.shown = shown + MORE_ROWS))]).gap(12.0).centred());
    }
    if !s.debian.is_empty() {
        items.push(note(debian_why()));
    }
    if waiting > 0 {
        items.push(note(not_held()));
    }
    items.push(note(t!("store-upd-has-trindle", version = &current)));
    items.extend(red_words(s, SYSTEM_ROW));
    tile("computer-symbolic", t!("store-upd-card-system"), checking(s, SYSTEM), items, small())
}

pub(super) fn security_card(s: &Store) -> Node<Msg> {
    let on = !s.upd.security_off;
    let switch = crate::toolkit::widget::switch(on, crate::common::security_updates::tip(on), move |now| go(move |s| set_security(s, now)));
    tile("security-high-symbolic", t!("store-upd-card-security"), vec![switch], vec![
        if on { dot_line(s, Dot::UpToDate, t!("store-upd-security-on")) } else { note(t!("settings-about-security-off")) },
        note(never_held()),
    ].into_iter().chain(red_words(s, SECURITY_ROW)).collect(), small())
}

pub(super) fn firmware_card(s: &Store) -> Node<Msg> {
    let have = run::have("fwupdmgr");
    let free = || s.firmware.iter().filter(|f| !s.held.firmware(&f.id));
    let bytes: u64 = free().map(|f| f.size).sum();
    let running = going_under(s, &["fw:"]);
    let (d, words) = if !have {
        (Dot::Quiet, t!("store-upd-fw-none").to_string())
    } else if s.upd.firmware_done && s.upd.restart && running.is_none() {
        (Dot::Ready, t!("store-upd-fw-restart").into())
    } else {
        status(s, FIRMWARE, running, free().count(), bytes, s.firmware.len() - free().count())
    };
    let mut items = vec![look::status_line(s, d, words, crate::store::help::FIRMWARE)];
    for f in &s.firmware {
        let key = format!("fw:{}", f.id);
        let going = job_ring(s, &key);
        let from = if f.current.is_empty() { t!("store-upd-fw-version", version = &f.newest) } else { t!("store-upd-version-to", from = &f.current, to = &f.newest) };
        let device = f.id.clone();
        let kept = held_words(Some(&f.current), Some(&f.newest));
        items.push(update_line(s, None, f.name.clone(), from, kept, size_words(Some(f.size)),
                               Own::button(s, key, going, move |s| update_device(s, &device)).holding(Thing::firmware(&f.id))));
    }
    items.push(note(if have {
        t!("store-upd-fw-note")
    } else {
        t!("store-upd-fw-install")
    }));
    rows_tile("drive-harddisk-symbolic", t!("store-upd-card-firmware"), checking(s, FIRMWARE), items, small())
}

fn set_security(s: &mut Store, now: bool) {
    use crate::toolkit::widget::Tone;
    s.say(SECURITY_ROW, "", Tone::Plain);
    if crate::common::security_updates::set(now).is_err() {
        s.say(SECURITY_ROW, crate::settings::ui::not_saved(), Tone::Problem);
        return;
    }
    s.upd.security_off = !now;
    let helper = run::admin_helper();
    let back = s.then::<(bool, String)>(move |s, (ok, err)| {
        if !ok {
            s.upd.security_off = now;
            let words = refused_words(crate::common::security_updates::set, now, run::admin_error(&err));
            s.say(SECURITY_ROW, words, Tone::Problem);
        }
    });
    run::capture(&["pkexec", &helper, "security-updates", if now { "on" } else { "off" }], None, move |ok, _, err| back((ok, err)));
}

fn refused_words(set: impl Fn(bool) -> std::io::Result<()>, now: bool, why: String) -> String {
    if set(!now).is_err() { crate::settings::ui::not_saved().to_string() } else { why }
}

#[cfg(test)]
mod security_tests {
    #[test]
    fn a_refused_administrator_step_puts_the_setting_back_and_says_why() {
        let put = std::cell::RefCell::new(Vec::new());
        let words = super::refused_words(|v| { put.borrow_mut().push(v); Ok(()) }, true, "Not done.".into());
        assert_eq!((words.as_str(), put.into_inner()), ("Not done.", vec![false]));
        let words = super::refused_words(|_| Err(std::io::Error::other("full")), true, "Not done.".into());
        assert_eq!(words, crate::settings::ui::not_saved());
    }

    #[test]
    fn a_closed_password_box_puts_the_switch_back_and_says_nothing() {
        let closed = crate::common::run::admin_error("Error executing command as another user: Request dismissed");
        let words = super::refused_words(|_| Ok(()), false, closed);
        assert!(crate::common::refused::closed_box(&words));
        assert!(!crate::common::refused::closed_box("Not done."));
    }
}
