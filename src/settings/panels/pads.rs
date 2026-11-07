//! Game controllers.

use std::collections::HashMap;
use std::time::Duration;

use crate::common::pads::{self, Change, Family, From, Live, Pad, Profiles, To};
use crate::common::steam_files;
use super::games::Games;
use crate::settings::pad_picture::{self, Shot};
use crate::settings::ui::{dropdown, group, line, not_saved, on, Kit, Msg};
use crate::toolkit::t;
use crate::toolkit::widget::{button, card, drawing, entry, label, remove, row as row_of, switch, Length, Node};

const ESCAPE: u32 = 1;
const MOUSE_LEFT: u32 = 0x110;
const MOUSE_RIGHT: u32 = 0x111;
const MOUSE_MIDDLE: u32 = 0x112;
const LOOK: Duration = Duration::from_millis(40);
const MOST_RECENT: usize = 8;

const KEY_WIDTH: f32 = 72.0;

fn row_key(from: From) -> String {
    format!("pad-row:{}", from.name())
}

fn swap_key(swap: usize) -> String {
    format!("pad-swap:{swap}")
}

fn other_key(i: usize) -> String {
    format!("pad-other:{i}")
}

fn on_pads(f: impl Fn(&mut Pads, &mut Kit) + 'static) -> Msg {
    on::<Games>(move |g, k| f(&mut g.pads, k))
}

fn after(kit: &Kit, wait: Duration, f: impl FnOnce(&mut Pads, &mut Kit) + 'static) {
    kit.after::<Games>(wait, move |g, k| f(&mut g.pads, k));
}

#[derive(Default)]
pub struct Pads {
    profiles: Profiles,
    pads: Vec<Pad>,
    chosen: usize,
    live: Option<Live>,
    round: u32,
    picked: Option<From>,
    recent: Vec<From>,
    typed: HashMap<usize, String>,
    type_round: u32,
    game: Option<u32>,
    steam: Vec<(u32, String)>,
    running: Vec<u32>,
    naming: Option<String>,
    name_round: u32,
}

impl Pads {
    fn load(&mut self) {
        self.profiles = pads::load().unwrap_or_default();
        let here = pads::plugged_in();
        let mut all = here.clone();
        for p in &self.profiles.pads {
            if !all.contains(&p.pad) {
                all.push(p.pad.clone());
            }
        }
        let was = self.current().cloned();
        self.chosen = was.and_then(|p| all.iter().position(|q| *q == p)).unwrap_or(0);
        self.pads = all;
        self.typed.clear();
        self.naming = None;
    }

    fn profile(&self) -> Option<String> {
        let pad = self.current()?;
        self.profiles.chosen(self.game).filter(|n| self.profiles.index_of(pad, n).is_some()).map(str::to_string)
    }

    fn why_not(&self) -> Option<&'static str> {
        self.profile().is_none().then(|| t!("settings-pads-standard-why"))
    }

    pub(super) fn current(&self) -> Option<&Pad> {
        self.pads.get(self.chosen)
    }

    fn family(&self) -> Family {
        self.current().map_or(Family::Xbox, Family::of)
    }

    fn open(&mut self) {
        self.live = self.current().map(|p| {
            let nodes: Vec<String> = pads::listed().into_iter().filter(|(q, _)| q.vendor == p.vendor && q.product == p.product).filter_map(|(_, n)| n).collect();
            Live::open(&nodes)
        });
    }

    fn keep(&mut self, kit: &mut Kit, key: &str) {
        if kit.settle(key, pads::save(&self.profiles), not_saved()).is_none() {
            self.profiles = pads::load().unwrap_or_default();
        }
    }

    fn watch(&mut self, kit: &mut Kit) {
        let mine = self.round;
        after(kit, LOOK, move |me, k| {
            if me.round != mine {
                return;
            }
            me.step();
            me.watch(k);
        });
    }

    fn step(&mut self) {
        let profile = self.profile();
        let Some(live) = self.live.as_mut() else { return };
        let Some(pad) = self.pads.get(self.chosen) else { return };
        for from in live.poll() {
            let Some(name) = profile.as_deref() else { continue };
            if self.profiles.changed_to(pad, name, from).is_some() {
                continue;
            }
            self.recent.retain(|f| *f != from);
            self.recent.insert(0, from);
            self.recent.truncate(MOST_RECENT);
            self.picked = Some(from);
        }
    }

    fn pick(&mut self, from: From) {
        self.picked = Some(from);
    }

    fn key_pressed(&mut self, kit: &mut Kit, code: Option<u32>) {
        let (Some(from), Some(pad), Some(name)) = (self.picked, self.current().cloned(), self.profile()) else { return };
        match code {
            Some(ESCAPE) => self.picked = None,
            Some(code) => {
                self.picked = None;
                self.profiles.set_into(&pad, &name, from, To::Keys(vec![code]));
                self.keep(kit, &row_key(from));
            }
            None => kit.say(&row_key(from), t!("settings-keyboard-not-told"), false, true),
        }
    }

    fn become_button(&mut self, kit: &mut Kit, from: From, target: To) {
        let (Some(pad), Some(name)) = (self.current().cloned(), self.profile()) else { return };
        self.picked = None;
        self.profiles.set_into(&pad, &name, from, target);
        self.keep(kit, &row_key(from));
    }

    fn take_away(&mut self, kit: &mut Kit, from: From) {
        let (Some(pad), Some(name)) = (self.current().cloned(), self.profile()) else { return };
        self.profiles.take_into(&pad, &name, from);
        self.keep(kit, &row_key(from));
    }

    fn swap(&mut self, kit: &mut Kit, swap: usize, on: bool) {
        if let (Some(p), Some(name)) = (self.current().cloned(), self.profile()) {
            self.profiles.set_swap(&p, &name, swap, on);
            self.typed.clear();
            self.keep(kit, &swap_key(swap));
        }
    }

    fn typed(&mut self, kit: &mut Kit, i: usize, text: String) {
        self.typed.insert(i, text);
        self.type_round = self.type_round.wrapping_add(1);
        let mine = self.type_round;
        after(kit, Duration::from_millis(600), move |me, k| {
            if me.type_round != mine {
                return;
            }
            let (Some(text), Some(p), Some(name)) = (me.typed.get(&i).cloned(), me.current().cloned(), me.profile()) else { return };
            match Change::parse(&text) {
                Err(why) => k.say(&other_key(i), why, false, true),
                Ok(change) => {
                    if let Some(slot) = me.profiles.index_of(&p, &name).and_then(|at| me.profiles.pads[at].changes.get_mut(i)) {
                        *slot = change;
                        me.typed.remove(&i);
                        me.keep(k, &other_key(i));
                    }
                }
            }
        });
    }

    fn take_other(&mut self, kit: &mut Kit, i: usize) {
        let (Some(p), Some(name)) = (self.current().cloned(), self.profile()) else { return };
        if let Some(at) = self.profiles.index_of(&p, &name) {
            if i < self.profiles.pads[at].changes.len() {
                self.profiles.pads[at].changes.remove(i);
                self.typed.clear();
                self.keep(kit, &other_key(i));
            }
        }
    }

    fn pad_name(&self) -> String {
        match self.current() {
            Some(p) if !p.name.is_empty() => p.name.clone(),
            Some(p) => t!("settings-keyboard-pad-unnamed", vendor = format!("{:04x}", p.vendor), product = format!("{:04x}", p.product)),
            None => t!("settings-pads-title").to_string(),
        }
    }

    fn name_of(&self, from: From) -> String {
        if let Some(l) = self.family().label(from) {
            return l.to_string();
        }
        let way = |positive: bool, vertical: bool| match (vertical, positive) {
            (false, false) => t!("settings-pads-left"),
            (false, true) => t!("settings-pads-right"),
            (true, false) => t!("settings-pads-up"),
            (true, true) => t!("settings-pads-down"),
        };
        match from {
            From::Button(b @ 0x220..=0x223) => t!("settings-pads-dpad", way = way(b == 0x223 || b == 0x221, b < 0x222)),
            From::Direction(axis, positive) => {
                let stick = if axis < 2 { t!("settings-pads-stick-left") } else { t!("settings-pads-stick-right") };
                t!("settings-pads-stick-way", stick = stick, way = way(positive, axis % 3 == 1))
            }
            _ => from.name(),
        }
    }

    fn becomes(&self, to: &To) -> String {
        match to {
            To::Button(b) => self.family().label(From::Button(*b)).map_or_else(|| pads::button_name(*b), str::to_string),
            To::Trigger(a) => self.family().label(From::Trigger(*a)).map_or_else(|| pads::axis_name(*a), str::to_string),
            other => other.words().strip_prefix("Key ").map_or_else(|| other.words(), str::to_string),
        }
    }

    fn order(&self) -> Vec<From> {
        let mut all = self.recent.clone();
        all.extend(pads::inputs().into_iter().filter(|f| !self.recent.contains(f)));
        all
    }

    pub(super) fn tell(&self) -> String {
        match self.picked {
            None if self.profiles.off => t!("settings-pads-top-off").to_string(),
            None if self.profile().is_none() => t!("settings-pads-top-standard").to_string(),
            None => t!("settings-pads-top-idle").to_string(),
            Some(from) => t!("settings-pads-top-picked", name = self.name_of(from)),
        }
    }

    pub(super) fn picture_card(&self, kit: &Kit) -> Node<Msg> {
        let family = self.family();
        let profile = self.profile().unwrap_or_default();
        let changed: Vec<From> = self.current().map_or_else(Vec::new, |p| {
            pads::inputs().into_iter().chain(self.recent.iter().copied()).filter(|f| self.profiles.changed_to(p, &profile, *f).is_some()).collect()
        });
        let live = self.live.as_ref();
        let keys: Vec<(From, String)> = self.current().map_or_else(Vec::new, |p| {
            pads::inputs().into_iter().filter_map(|f| self.profiles.changed_to(p, &profile, f).filter(|to| matches!(to, To::Keys(_))).map(|to| (f, self.becomes(to)))).collect()
        });
        let shot = match live {
            Some(l) => Shot::of(l, family, self.picked, changed),
            None => Shot { family: Some(family), picked: self.picked, changed, ..Shot::default() },
        };
        let shot = Shot { keys, ..shot };
        let title = self.pad_name();
        let picture = drawing((pad_picture::WIDE, pad_picture::HIGH), t!("settings-pads-picture", title = &title), move |canvas, pen| pad_picture::paint(canvas, pen, &shot))
            .width(Length::Fill).height(Length::Fixed(400.0));
        let mut rows = Vec::new();
        if self.pads.len() > 1 {
            let words: Vec<String> = self.pads.iter().map(|p| if p.name.is_empty() { format!("{:04x}:{:04x}", p.vendor, p.product) } else { p.name.clone() }).collect();
            rows.push(line(t!("settings-pads-which")).control(dropdown(kit, "pad-which", words, Some(self.chosen), t!("settings-pads-which"),
                |c| on_pads(move |me, _| {
                    me.chosen = c;
                    me.picked = None;
                    me.recent.clear();
                    me.naming = None;
                    me.open();
                })))
                .node(kit));
        }
        rows.push(picture);
        group(title).icon("input-gaming-symbolic").rows(rows).node(kit)
    }

    pub(super) fn buttons_card(&self, kit: &Kit) -> Node<Msg> {
        let Some(pad) = self.current() else { return group(t!("settings-pads-buttons-title")).node(kit) };
        let profile = self.profile().unwrap_or_default();
        let why = self.why_not();
        let mut targets: Vec<String> = vec![
            t!("settings-pads-waiting").to_string(),
            t!("settings-pads-nothing").to_string(),
            t!("settings-pads-on-keyboard", what = t!("settings-pads-escape")),
            t!("settings-pads-on-mouse", what = t!("settings-pads-left-click")),
            t!("settings-pads-on-mouse", what = t!("settings-pads-right-click")),
            t!("settings-pads-on-mouse", what = t!("settings-pads-middle-click")),
        ];
        let names = pads::names();
        targets.extend(names.iter().map(|n| t!("settings-pads-on-pad", what = self.family().label(button_of(&n.0)).map_or_else(|| n.0.clone(), str::to_string))));
        let mut rows = Vec::new();
        for from in self.order() {
            let name = self.name_of(from);
            let waits = self.picked == Some(from);
            let mut row = line(name.clone()).key(row_key(from));
            if waits {
                let all = names.clone();
                row = row.control(dropdown(kit, &format!("pad-to:{}", from.name()), targets.clone(), Some(0), &t!("settings-pads-choose", name = &name), move |c| {
                    let all = all.clone();
                    on_pads(move |me, k| match c {
                        0 => {}
                        1 => me.become_button(k, from, To::Nothing),
                        2 => me.become_button(k, from, To::Keys(vec![ESCAPE])),
                        3 => me.become_button(k, from, To::Keys(vec![MOUSE_LEFT])),
                        4 => me.become_button(k, from, To::Keys(vec![MOUSE_RIGHT])),
                        5 => me.become_button(k, from, To::Keys(vec![MOUSE_MIDDLE])),
                        n => {
                            if let Some(to) = all.get(n - 6).and_then(|b| To::named(&b.0)) {
                                me.become_button(k, from, to);
                            }
                        }
                    })
                }));
                row = row.control(button("window-close-symbolic", t!("settings-pads-stop", name = &name)).on_press(on_pads(|me, _| me.picked = None)));
            } else {
                let change = t!("settings-pads-change", name = &name);
                match self.profiles.changed_to(pad, &profile, from) {
                    Some(to) => {
                        let said = self.becomes(to);
                        let shown = if matches!(to, To::Keys(_)) { card(label(said)).chip().width(Length::Shrink) } else { label(said) };
                        row = row.control(row_of(vec![shown.called(change).on_press(on_pads(move |me, _| me.pick(from)))]).width(Length::Fixed(KEY_WIDTH)));
                        row = row.control(button("user-trash-symbolic", t!("settings-keyboard-take-away")).on_press(on_pads(move |me, k| me.take_away(k, from))));
                    }
                    None => row = row.control(button("input-keyboard-symbolic", change).off_because(why).on_press(on_pads(move |me, _| me.pick(from)))),
                }
            }
            rows.push(row.node(kit));
        }
        group(t!("settings-pads-buttons-title")).icon("input-gaming-symbolic").about(t!("settings-pads-buttons-about")).rows(rows).node(kit)
    }

    pub(super) fn quick_card(&self, kit: &Kit) -> Node<Msg> {
        let Some(pad) = self.current() else { return group(t!("settings-pads-quick-title")).node(kit) };
        let title = self.pad_name();
        let profile = self.profile().unwrap_or_default();
        let mut rows = Vec::new();
        for i in 0..pads::SWAPS.len() {
            let name = pads::swap_name(i);
            let on_now = self.profiles.has_swap(pad, &profile, i);
            rows.push(line(name).key(swap_key(i))
                .control(switch(on_now, t!("settings-keyboard-pad-swap-switch", name = name, title = &title), move |b| on_pads(move |me, k| me.swap(k, i, b))).off_because(self.why_not()))
                .node(kit));
        }
        group(t!("settings-pads-quick-title")).icon("input-gaming-symbolic").span(5, 7).rows(rows).node(kit)
    }

    pub(super) fn other_card(&self, kit: &Kit) -> Option<Node<Msg>> {
        let pad = self.current()?;
        let profile = self.profile().unwrap_or_default();
        let mut rows = Vec::new();
        for (i, change) in self.profiles.own(pad, &profile) {
            if matches!(change, Change::Into { .. }) {
                continue;
            }
            let typed = self.typed.get(&i).cloned().unwrap_or_else(|| change.line());
            let box_ = entry(typed, t!("settings-keyboard-pad-does", title = self.pad_name()), move |text| on_pads(move |me, k| me.typed(k, i, text.clone())));
            let gone = button("user-trash-symbolic", t!("settings-keyboard-take-away")).on_press(on_pads(move |me, k| me.take_other(k, i)));
            rows.push(line(self.pad_name()).sub(t!("settings-keyboard-pad-type")).key(other_key(i)).control(box_).control(gone).node(kit));
        }
        if let Some((n, text, why)) = self.profiles.unread.first() {
            rows.push(line(text.clone()).sub(t!("settings-keyboard-line-unused", n = *n, why = *why)).node(kit));
        }
        (!rows.is_empty()).then(|| group(t!("settings-pads-other-title")).icon("input-gaming-symbolic").about(t!("settings-pads-other-about")).rows(rows).node(kit))
    }
}

const OFF_KEY: &str = "pad-profiles:off";
const GAME_KEY: &str = "pad-profiles:game";
const WHICH_KEY: &str = "pad-profiles:which";
const NAME_KEY: &str = "pad-profiles:name";

impl Pads {
    fn set_off(&mut self, kit: &mut Kit, off: bool) {
        self.profiles.off = off;
        self.keep(kit, OFF_KEY);
    }

    fn show_game(&mut self, game: Option<u32>) {
        self.game = game;
        self.picked = None;
        self.naming = None;
        self.typed.clear();
    }

    fn choose(&mut self, kit: &mut Kit, name: Option<String>) {
        self.profiles.choose(self.game, name);
        self.picked = None;
        self.naming = None;
        self.typed.clear();
        self.keep(kit, WHICH_KEY);
    }

    fn add(&mut self, kit: &mut Kit) {
        let Some(pad) = self.current().cloned() else { return };
        let taken = self.profiles.names_of(&pad);
        let name = (1..).map(|n| t!("settings-pads-profile-new", n = n)).find(|n| !taken.contains(n)).unwrap_or_default();
        match self.profiles.add(&pad, &name) {
            Ok(name) => self.choose(kit, Some(name)),
            Err(why) => kit.say(WHICH_KEY, why, false, true),
        }
    }

    fn remove(&mut self, kit: &mut Kit) {
        let (Some(pad), Some(name)) = (self.current().cloned(), self.profile()) else { return };
        self.profiles.remove(&pad, &name);
        self.naming = None;
        self.typed.clear();
        self.keep(kit, WHICH_KEY);
    }

    fn named(&mut self, kit: &mut Kit, text: String) {
        self.naming = Some(text);
        self.name_round = self.name_round.wrapping_add(1);
        let mine = self.name_round;
        after(kit, Duration::from_millis(600), move |me, k| {
            if me.name_round != mine {
                return;
            }
            let (Some(text), Some(pad), Some(old)) = (me.naming.clone(), me.current().cloned(), me.profile()) else { return };
            match me.profiles.rename(&pad, &old, &text) {
                Err(why) => k.say(NAME_KEY, why, false, true),
                Ok(_) => {
                    me.naming = None;
                    me.keep(k, NAME_KEY);
                }
            }
        });
    }

    pub(super) fn profiles_card(&self, kit: &Kit) -> Node<Msg> {
        let Some(pad) = self.current() else { return group(t!("settings-pads-profiles-title")).node(kit) };
        let on_now = !self.profiles.off;
        let mut rows = vec![line(t!("settings-pads-changes")).sub(t!("settings-pads-changes-sub")).key(OFF_KEY)
            .control(switch(on_now, t!("settings-pads-changes"), |b| on_pads(move |me, k| me.set_off(k, !b))))
            .node(kit)];
        let mut games: Vec<(Option<u32>, String)> = vec![(None, t!("settings-pads-other-game").to_string())];
        games.extend(self.steam.iter().map(|(id, name)| {
            let said = if self.running.contains(id) { t!("settings-pads-game-running", name = name.as_str()) } else { name.clone() };
            (Some(*id), said)
        }));
        if let Some(id) = self.game.filter(|g| !games.iter().any(|w| w.0 == Some(*g))) {
            games.push((Some(id), t!("settings-pads-game-number", n = id)));
        }
        let at = games.iter().position(|g| g.0 == self.game);
        let ids: Vec<Option<u32>> = games.iter().map(|g| g.0).collect();
        rows.push(line(t!("settings-pads-game")).key(GAME_KEY)
            .control(dropdown(kit, GAME_KEY, games.into_iter().map(|g| g.1).collect(), at, t!("settings-pads-game"),
                move |c| { let game = ids.get(c).copied().flatten(); on_pads(move |me, _| me.show_game(game)) }))
            .node(kit));
        let names = self.profiles.names_of(pad);
        let profile = self.profile();
        let mut words = vec![t!("settings-pads-standard").to_string()];
        words.extend(names.iter().cloned());
        let at = profile.as_ref().and_then(|p| names.iter().position(|n| n == p)).map_or(0, |i| i + 1);
        let mut which = line(t!("settings-pads-profile")).key(WHICH_KEY);
        if profile.is_none() {
            which = which.sub(t!("settings-pads-standard-sub"));
        }
        which = which.control(dropdown(kit, WHICH_KEY, words, Some(at), t!("settings-pads-profile"),
                move |c| { let name = c.checked_sub(1).and_then(|i| names.get(i).cloned()); on_pads(move |me, k| me.choose(k, name.clone())) }))
            .control(button("list-add-symbolic", t!("settings-pads-profile-add")).on_press(on_pads(|me, k| me.add(k))));
        if let Some(name) = &profile {
            which = which.control(remove("user-trash-symbolic", t!("settings-pads-profile-remove", name = name.as_str())).on_press(on_pads(|me, k| me.remove(k))));
        }
        rows.push(which.node(kit));
        if let Some(name) = profile {
            let typed = self.naming.clone().unwrap_or(name);
            rows.push(line(t!("settings-pads-profile-name")).key(NAME_KEY)
                .control(entry(typed, t!("settings-pads-profile-name"), |text| on_pads(move |me, k| me.named(k, text.clone()))))
                .node(kit));
        }
        group(t!("settings-pads-profiles-title")).icon("input-gaming-symbolic").about(t!("settings-pads-profiles-about")).rows(rows).node(kit)
    }
}

fn button_of(name: &str) -> From {
    To::named(name).and_then(|to| if let To::Button(b) = to { Some(From::Button(b)) } else if let To::Trigger(a) = to { Some(From::Trigger(a)) } else { None }).unwrap_or(From::Button(0))
}

impl Pads {
    pub(super) fn none_card(&self, kit: &Kit) -> Node<Msg> {
        group(t!("settings-pads-title")).icon("input-gaming-symbolic")
            .rows(vec![line(t!("settings-pads-none")).sub(t!("settings-pads-none-about")).node(kit)])
            .node(kit)
    }

    pub(super) fn shown(&mut self, kit: &mut Kit) {
        self.round = self.round.wrapping_add(1);
        self.picked = None;
        self.load();
        self.steam = steam_files::games(&steam_files::libraries(&steam_files::root())).into_iter().map(|g| (g.id, g.name)).collect();
        self.running = pads::steam_games_now(std::path::Path::new("/proc"));
        self.game = self.profiles.game_now(&self.running);
        self.open();
        self.watch(kit);
    }

    pub(super) fn hidden(&mut self, _kit: &mut Kit) {
        self.round = self.round.wrapping_add(1);
        self.live = None;
        self.picked = None;
    }

    pub(super) fn second(&mut self, _kit: &mut Kit) -> bool {
        let here = pads::plugged_in();
        let known: Vec<&Pad> = self.pads.iter().filter(|p| here.contains(p) || self.profiles.names_of(p).is_empty()).collect();
        if known.len() == here.len() && here.iter().all(|p| known.contains(&p)) && self.live.as_ref().is_some_and(Live::is_open) != here.is_empty() {
            return false;
        }
        self.load();
        self.open();
        true
    }

    pub(super) fn intercept(&self, input: &crate::toolkit::input::Input) -> Option<Msg> {
        use crate::toolkit::input::Input;
        self.picked?;
        match input {
            Input::Key { repeat: false, .. } => {
                let code = crate::toolkit::window::code_now();
                Some(on_pads(move |me, k| me.key_pressed(k, code)))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
impl Pads {
    pub(super) fn made_up() -> Pads {
        let pad = Pad { vendor: 0x11c0, product: 0x5500, name: "Test pad".into() };
        let mut profiles = Profiles::default();
        profiles.set_into(&pad, pads::FIRST, From::Button(0x130), To::Keys(vec![17]));
        profiles.choose(None, Some(pads::FIRST.into()));
        Pads { profiles, pads: vec![pad], steam: vec![(1593500, "God of War".into())], live: Some(Live::made_up(pads::Layout::STANDARD)), recent: vec![From::Button(0x131)], picked: Some(From::Button(0x131)), ..Pads::default() }
    }
}
