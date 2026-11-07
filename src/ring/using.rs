//! Who is using the microphone, the camera, the screen or the mouse and keyboard, as the ring shows it.

use std::time::{Duration, Instant};

use crate::common::apps::{self, Entry};
use crate::common::in_use::{Kind, User};

pub const HOLD: Duration = Duration::from_secs(3);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Known {
    pub name: String,
    pub icon: Option<String>,
}

pub fn known_plugin(id: &str) -> Option<Known> {
    let p = crate::plugins::find(id)?;
    Some(Known { name: p.name, icon: p.ring.map(|r| r.0.to_string_lossy().into_owned()) })
}

#[derive(Default)]
pub struct Hold {
    seen: Vec<(User, Instant)>,
}

impl Hold {
    pub fn settle(&mut self, now: Instant, fresh: Vec<User>) -> Vec<User> {
        for u in &fresh {
            match self.seen.iter_mut().find(|(s, _)| s == u) {
                Some((_, at)) => *at = now,
                None => self.seen.push((u.clone(), now)),
            }
        }
        self.seen.retain(|(_, at)| now.duration_since(*at) < HOLD);
        self.seen.iter().map(|(u, _)| u.clone()).collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub kind: Kind,
    pub who: Vec<String>,
    pub icons: Vec<String>,
}

impl Item {
    pub fn words(&self) -> String {
        let who: Vec<&str> = self.who.iter().map(String::as_str).collect();
        self.kind.spoken(&who)
    }
}

fn name_of(u: &User) -> String {
    let n = u.name.trim();
    if n.is_empty() { u.app_ids.first().cloned().unwrap_or_default() } else { n.to_string() }
}

fn plugin_of(u: &User, plugin: &impl Fn(&str) -> Option<Known>) -> Option<(String, Known)> {
    std::iter::once(u.name.trim()).chain(u.app_ids.iter().map(|a| a.trim())).filter(|n| !n.is_empty())
        .find_map(|n| plugin(n).map(|k| (n.to_string(), k)))
}

fn shown_name(u: &User, apps: &[Entry], plugin: &impl Fn(&str) -> Option<Known>) -> String {
    if let Some((_, k)) = plugin_of(u, plugin) {
        return k.name;
    }
    let id = u.app_ids.first().map(String::as_str).unwrap_or(&u.name);
    apps::find_for_window(apps, id).map(|e| e.name.clone()).unwrap_or_else(|| name_of(u))
}

pub const ON_TABS: [Kind; 3] = [Kind::Microphone, Kind::Camera, Kind::Screen];

pub fn window_uses(users: &[User], apps: &[Entry], plugin: impl Fn(&str) -> Option<Known>, app_id: &str) -> Vec<Item> {
    if app_id.is_empty() {
        return Vec::new();
    }
    ON_TABS.into_iter().filter_map(|kind| {
        let mut who: Vec<String> = Vec::new();
        for u in users.iter().filter(|u| u.kind == kind && u.app_ids.iter().any(|a| a == app_id)) {
            let name = shown_name(u, apps, &plugin);
            if !name.is_empty() && !who.contains(&name) {
                who.push(name);
            }
        }
        let used = users.iter().any(|u| u.kind == kind && u.app_ids.iter().any(|a| a == app_id));
        used.then(|| Item { kind, who, icons: Vec::new() })
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user(kind: Kind, name: &str) -> User {
        User { kind, pid: Some(7), name: name.into(), app_ids: vec![] }
    }

    #[test]
    fn a_program_that_lets_go_stays_for_the_hold_and_then_goes() {
        let sample = user(Kind::Input, "Sample");
        let t0 = Instant::now();
        let mut hold = Hold::default();
        assert_eq!(hold.settle(t0, vec![sample.clone()]), std::slice::from_ref(&sample));
        let short = t0 + HOLD / 2;
        assert_eq!(hold.settle(short, vec![]), std::slice::from_ref(&sample), "it flickers if it goes at once");
        assert_eq!(hold.settle(short + Duration::from_millis(100), vec![sample.clone()]), std::slice::from_ref(&sample));
        let later = short + HOLD + Duration::from_millis(200);
        assert!(hold.settle(later, vec![]).is_empty());
        assert!(hold.settle(later + HOLD, vec![]).is_empty());
    }

    fn on(kind: Kind, name: &str, ids: &[&str]) -> User {
        User { kind, pid: Some(7), name: name.into(), app_ids: ids.iter().map(|s| s.to_string()).collect() }
    }

    #[test]
    fn a_window_with_nothing_in_use_has_no_uses() {
        assert!(window_uses(&[], &[], |_| None, "firefox").is_empty());
        assert!(window_uses(&[on(Kind::Microphone, "Zoom", &["zoom"])], &[], |_| None, "firefox").is_empty());
        assert!(window_uses(&[on(Kind::Microphone, "Zoom", &["zoom"])], &[], |_| None, "").is_empty());
    }

    #[test]
    fn a_window_has_one_use_in_the_corner_labels_words() {
        let users = [on(Kind::Microphone, "Firefox", &["firefox"])];
        let got = window_uses(&users, &[], |_| None, "firefox");
        assert_eq!(got.len(), 1);
        assert_eq!(got[0].kind, Kind::Microphone);
        assert_eq!(got[0].words(), "Microphone in use by Firefox");
        assert_eq!(crate::common::in_use::lines(&users, false), [got[0].words()]);
    }

    #[test]
    fn two_uses_come_in_the_corner_dots_order_and_a_repeat_is_one() {
        let users = [on(Kind::Screen, "Firefox", &["firefox"]), on(Kind::Camera, "Firefox", &["firefox"]),
                     on(Kind::Camera, "Firefox", &["firefox"]), on(Kind::Input, "Firefox", &["firefox"])];
        let got = window_uses(&users, &[], |_| None, "firefox");
        let kinds: Vec<Kind> = got.iter().map(|i| i.kind).collect();
        assert_eq!(kinds, [Kind::Camera, Kind::Screen], "the keyboard and mouse have no dot on a tab");
        assert_eq!(got[1].words(), "Screen being recorded by Firefox");
    }

    #[test]
    fn with_many_windows_each_gets_only_its_own_programs_uses() {
        let users = [on(Kind::Microphone, "Zoom", &["zoom"]), on(Kind::Camera, "Cheese", &["org.gnome.Cheese"])];
        let ids = ["firefox", "zoom", "org.gnome.Cheese", "gimp", "zoom"];
        let kinds: Vec<Vec<Kind>> = ids.iter().map(|id| window_uses(&users, &[], |_| None, id).iter().map(|i| i.kind).collect()).collect();
        assert_eq!(kinds, [vec![], vec![Kind::Microphone], vec![Kind::Camera], vec![], vec![Kind::Microphone]]);
    }

    #[test]
    fn a_program_with_no_window_is_on_no_tab_but_stays_in_the_corner() {
        let users = [on(Kind::Microphone, "Voice", &[])];
        for id in ["firefox", "voice", "Voice"] {
            assert!(window_uses(&users, &[], |_| None, id).is_empty());
        }
        assert_eq!(crate::common::in_use::lines(&users, false), ["Microphone in use by Voice"]);
    }
}
