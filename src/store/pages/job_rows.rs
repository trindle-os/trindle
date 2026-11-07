//! Rows with a ring for a job, and the start and end of that ring.

use crate::store::{go, listing, Msg, Store};
use crate::common::filling::Dial;
use crate::toolkit::t;
use crate::toolkit::widget::{button, note, row, Length, Node, Tone};
use crate::toolkit::theme;

pub(crate) fn updating(s: &Store) -> bool {
    s.dials.iter().any(|(k, d)| (k.starts_with("up:") || k.starts_with("fw:")) && d.running())
}

pub(crate) fn no_reason() -> &'static str {
    t!("store-job-no-reason")
}

pub(crate) fn settle(s: &mut Store, key: &str, ok: bool, words: &str) {
    if let Some(d) = s.dials.get_mut(key) {
        d.finish(ok, words);
    }
    if !ok {
        let row = key.strip_suffix("-look").or_else(|| key.strip_suffix("-go")).unwrap_or(key);
        s.say(row, no_reason(), Tone::Problem);
    }
    let k = key.to_string();
    let back = s.then(move |s: &mut Store, ()| {
        if s.dials.get(&k).is_some_and(|d| !d.running()) {
            s.dials.remove(&k);
        }
    });
    s.events.after(std::time::Duration::from_secs(3), move || back(()));
}

pub(super) fn job_button(s: &Store, key: &str, icon: &str, tip: &str, press: impl Fn(&mut Store) + 'static) -> Node<Msg> {
    let b = button(icon, tip).key(key);
    let b = match s.dials.get(key) {
        Some(d) if d.running() => d.round(b),
        _ if updating(s) => b,
        _ => b.on_press(go(press)),
    };
    b
}

pub(super) fn update_row(s: &Store, key: &str, head: String, sub: String, buttons: Vec<Node<Msg>>) -> Node<Msg> {
    pictured_row(s, None, key, head, sub, buttons)
}

pub(super) fn pictured_row(s: &Store, picture: Option<Node<Msg>>, key: &str, head: String, sub: String, buttons: Vec<Node<Msg>>) -> Node<Msg> {
    pictured_row_with(s, picture, key, head, sub, Vec::new(), buttons)
}

pub(super) fn pictured_row_with(s: &Store, picture: Option<Node<Msg>>, key: &str, head: String, sub: String, lines: Vec<Node<Msg>>,
                     buttons: Vec<Node<Msg>>) -> Node<Msg> {
    let mut under = lines;
    if let Some((w, t)) = s.said.get(key).filter(|(w, _)| !w.is_empty()) {
        let more = (*t == Tone::Problem).then(|| crate::common::more::button(s.more(key)));
        match s.behind.get(key).filter(|b| !b.is_empty()) {
            Some(behind) => {
                let own = key.to_string();
                let mut beside = vec![note(w.clone()).tone(*t).width(Length::Fill)];
                beside.extend(more);
                beside.push(button("dialog-information-symbolic", t!("store-show-behind")).size(theme::sheet().info)
                    .on_press(go(move |s| s.toggle_behind(&own))));
                under.push(row(beside).gap(8.0).centred());
                if s.behind_open.contains(key) {
                    under.extend(behind.iter().map(|b| note(b.clone())));
                }
            }
            None => under.push(match more {
                Some(more) => row(vec![note(w.clone()).tone(*t).width(Length::Fill), more]).gap(8.0).centred(),
                None => note(w.clone()).tone(*t),
            }),
        }
    }
    listing::listing(picture, head, note(sub), under, buttons)
}

pub(crate) fn begin(s: &mut Store, key: &str, set: impl FnOnce(&mut Dial)) {
    let mut d = Dial::default();
    set(&mut d);
    s.dials.insert(key.to_string(), d);
}

pub(super) fn banded(rows: Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    listing::rows(rows)
}
