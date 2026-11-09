//! The screens: what is plugged in, how each is set, and changing them, asked
//! of Trindle's compositor directly over trindle_screens_v1. Display settings, the start-up scale
//! and About this computer all come here, so no outside program is needed.

use serde::{Deserialize, Serialize};
use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_client::protocol::wl_registry;
use wayland_client::{event_created_child, Connection, Dispatch, EventQueue, QueueHandle, WEnum};

use crate::common::wire::screens::{
    trindle_screen_plan_v1::{self, TrindleScreenPlanV1},
    trindle_screen_v1::{self, TrindleScreenV1},
    trindle_screens_manager_v1::{self, TrindleScreensManagerV1},
};

use crate::common::files::{config_path, read_text, write_text};
use trindle_toolkit::t;

const SAVED: &str = "displays.conf";

const OLD_SAVED: &str = "display.sh";

const TRIES: usize = 3;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Turn {
    Normal,
    R90,
    R180,
    R270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}

const TURNS: [(Turn, &str, u32); 8] = [
    (Turn::Normal, "normal", 0),
    (Turn::R90, "90", 1),
    (Turn::R180, "180", 2),
    (Turn::R270, "270", 3),
    (Turn::Flipped, "flipped", 4),
    (Turn::Flipped90, "flipped-90", 5),
    (Turn::Flipped180, "flipped-180", 6),
    (Turn::Flipped270, "flipped-270", 7),
];

impl Turn {
    pub fn name(self) -> &'static str {
        TURNS.iter().find(|t| t.0 == self).map_or("normal", |t| t.1)
    }

    pub fn from_name(name: &str) -> Option<Turn> {
        TURNS.iter().find(|t| t.1 == name).map(|t| t.0)
    }

    fn number(self) -> u32 {
        TURNS.iter().find(|t| t.0 == self).map_or(0, |t| t.2)
    }

    fn from_number(n: u32) -> Turn {
        TURNS.iter().find(|x| x.2 == n).map_or(Turn::Normal, |x| x.0)
    }

    pub fn sideways(self) -> bool {
        matches!(self, Turn::R90 | Turn::R270 | Turn::Flipped90 | Turn::Flipped270)
    }
}

impl TryFrom<String> for Turn {
    type Error = String;
    fn try_from(name: String) -> Result<Self, Self::Error> {
        Turn::from_name(&name).ok_or_else(|| format!("no such way up: {name}"))
    }
}

impl From<Turn> for String {
    fn from(t: Turn) -> String {
        t.name().to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mode {
    pub width: i32,
    pub height: i32,

    pub refresh: i32,
    pub preferred: bool,
}

impl Mode {

    pub fn hz(&self) -> Option<f64> {
        (self.refresh > 0).then(|| self.refresh as f64 / 1000.0)
    }

    pub fn resolution(&self) -> Resolution {
        Resolution { width: self.width, height: self.height, refresh: self.refresh }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Resolution {
    pub width: i32,
    pub height: i32,

    #[serde(default)]
    pub refresh: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Head {

    pub name: String,

    pub description: String,
    pub make: String,
    pub model: String,
    pub serial: String,

    pub physical_size: (i32, i32),
    pub modes: Vec<Mode>,

    pub current: Option<usize>,
    pub enabled: bool,

    pub position: (i32, i32),
    pub turn: Turn,
    pub scale: f64,

    pub varies_rate: bool,
}

impl Head {
    pub fn current_mode(&self) -> Option<&Mode> {
        self.current.and_then(|i| self.modes.get(i))
    }

    pub fn logical_size(&self) -> (i32, i32) {
        let Some(m) = self.current_mode() else { return (0, 0) };
        let (w, h) = if self.turn.sideways() { (m.height, m.width) } else { (m.width, m.height) };
        let s = if self.scale > 0.0 { self.scale } else { 1.0 };
        ((w as f64 / s).round() as i32, (h as f64 / s).round() as i32)
    }

    pub fn screen(&self) -> crate::common::screen_scale::Screen {
        let pixels = self.current_mode().or_else(|| self.modes.iter().find(|m| m.preferred))
            .map(|m| (m.width, m.height)).unwrap_or((0, 0));
        crate::common::screen_scale::Screen {
            connector: self.name.clone(), make: self.make.clone(), model: self.model.clone(), serial: self.serial.clone(),
            pixels, millimetres: self.physical_size,
        }
    }

    pub fn setting(&self) -> Setting {
        if !self.enabled {
            return Setting::off(&self.name);
        }
        Setting {
            name: self.name.clone(),
            enabled: true,
            mode: self.current_mode().map(Mode::resolution),
            position: Some(self.position),
            turn: Some(self.turn),
            scale: Some(self.scale),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Setting {
    pub name: String,
    pub enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<Resolution>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<(i32, i32)>,
    #[serde(default, rename = "transform", skip_serializing_if = "Option::is_none")]
    pub turn: Option<Turn>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<f64>,
}

impl Setting {

    pub fn on(name: &str) -> Setting {
        Setting { name: name.to_string(), enabled: true, mode: None, position: None, turn: None, scale: None }
    }

    pub fn off(name: &str) -> Setting {
        Setting { enabled: false, ..Setting::on(name) }
    }
}

pub fn heads() -> Result<Vec<Head>, String> {
    Ok(Link::open()?.state.heads())
}

pub fn apply(settings: &[Setting]) -> Result<(), String> {
    let mut link = Link::open()?;
    for _ in 0..TRIES {
        match link.apply(settings)? {
            (Outcome::Applied, _) => return Ok(()),
            (Outcome::Refused, why) if why.is_empty() => {
                return Err("The desktop could not set the displays that way.".into())
            }
            (Outcome::Refused, why) => {
                return Err(format!("The desktop could not set the displays that way: {why}."))
            }

            (Outcome::Stale, _) => link.settle()?,
        }
    }
    Err("The displays kept changing, so nothing was changed.".into())
}

pub fn saved() -> Option<Vec<Setting>> {
    saved_in(&config_path(SAVED), &config_path(OLD_SAVED))
}

pub(crate) fn saved_in(path: &std::path::Path, old: &std::path::Path) -> Option<Vec<Setting>> {
    if path.is_file() {
        return serde_json::from_str(&read_text(path)).ok();
    }
    if !old.is_file() {
        return None;
    }

    let text = match crate::common::files::read_small(old) {
        Ok(text) => text,
        Err(_) => {
            let _ = crate::common::damaged::may_write(old, t!("common-unreadable-name-old-display"));
            return None;
        }
    };
    let settings = from_wlr_randr(&text);
    match save_in(path, &settings) {

        Ok(()) => {
            let _ = std::fs::remove_file(old);
        }
        Err(_) => eprintln!("trindle: could not move {} across to {SAVED}", old.display()),
    }
    Some(settings)
}

pub fn save(settings: &[Setting]) -> std::io::Result<()> {
    save_in(&config_path(SAVED), settings)?;

    let _ = std::fs::remove_file(config_path(OLD_SAVED));
    Ok(())
}

fn save_in(path: &std::path::Path, settings: &[Setting]) -> std::io::Result<()> {
    let text = serde_json::to_string_pretty(settings).map_err(std::io::Error::other)?;
    write_text(path, &(text + "\n"))
}

pub fn apply_saved(settings: &[Setting]) {
    if apply(settings).is_ok() {
        return;
    }
    for s in settings {
        if let Err(e) = apply(std::slice::from_ref(s)) {
            eprintln!("trindle: display {}: {e}", s.name);
        }
    }
}

fn turn_for(saved: Option<Turn>, now: Turn) -> Turn {
    saved.unwrap_or(now)
}

pub fn overlapping(heads: &[Head]) -> bool {
    let on: Vec<_> = heads.iter().filter(|h| h.enabled).map(|h| (h.position, h.logical_size())).collect();
    on.iter().enumerate().any(|(i, ((ax, ay), (aw, ah)))| on[i + 1..].iter().any(|((bx, by), (bw, bh))| {
        *aw > 0 && *bw > 0 && ax < &(bx + bw) && bx < &(ax + aw) && ay < &(by + bh) && by < &(ay + ah)
    }))
}

fn clear_unnamed(heads: &[Head], settings: &[Setting], mut wanted: Vec<Setting>) -> Vec<Setting> {
    let rects: Vec<(i32, i32, i32, i32)> = heads.iter().zip(&wanted).map(|(h, w)| {
        let res = w.mode.map(|r| (r.width, r.height)).or_else(|| h.current_mode().map(|m| (m.width, m.height))).unwrap_or((0, 0));
        let turn = turn_for(w.turn, h.turn);
        let (pw, ph) = if turn.sideways() { (res.1, res.0) } else { res };
        let scale = w.scale.unwrap_or(h.scale);
        let scale = if scale > 0.0 { scale } else { 1.0 };
        let at = w.position.unwrap_or(h.position);
        (at.0, at.1, if w.enabled { (pw as f64 / scale).round() as i32 } else { 0 }, if w.enabled { (ph as f64 / scale).round() as i32 } else { 0 })
    }).collect();
    let movable: Vec<bool> = heads.iter().map(|h| !settings.iter().any(|s| s.name == h.name)).collect();
    for (i, to) in move_clear(&rects, &movable).into_iter().enumerate() {
        if let Some(to) = to {
            wanted[i].position = Some(to);
        }
    }
    wanted
}

fn move_clear(rects: &[(i32, i32, i32, i32)], movable: &[bool]) -> Vec<Option<(i32, i32)>> {
    let mut rects = rects.to_vec();
    let mut out = vec![None; rects.len()];
    let overlaps = |a: (i32, i32, i32, i32), b: (i32, i32, i32, i32)| {
        a.2 > 0 && b.2 > 0 && a.0 < b.0 + b.2 && b.0 < a.0 + a.2 && a.1 < b.1 + b.3 && b.1 < a.1 + a.3
    };
    for i in 0..rects.len() {
        if !movable[i] || rects[i].2 == 0 {
            continue;
        }
        if (0..rects.len()).any(|j| j != i && overlaps(rects[i], rects[j])) {
            let right = rects.iter().enumerate().filter(|(j, r)| *j != i && r.2 > 0).map(|(_, r)| r.0 + r.2).max().unwrap_or(0);
            rects[i].0 = right;
            rects[i].1 = 0;
            out[i] = Some((right, 0));
        }
    }
    out
}

pub fn remember_for_login(heads: &[Head]) {
    let on: Vec<_> = heads.iter().filter(|h| h.enabled).map(|h| (h.screen(), h.scale)).collect();
    if !on.is_empty() {
        crate::common::screen_scale::remember(&on);
    }
}

pub fn from_wlr_randr(text: &str) -> Vec<Setting> {
    let mut settings: Vec<Setting> = Vec::new();
    for line in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
        let words = shell_words(line.trim_end_matches("2>/dev/null").trim());
        if words.first().is_none_or(|w| w != "wlr-randr") {
            continue;
        }
        let mut words = words.into_iter().skip(1);
        while let Some(word) = words.next() {
            if word == "--output" {
                if let Some(name) = words.next() {
                    settings.push(Setting::on(&name));
                }
                continue;
            }
            let Some(s) = settings.last_mut() else { continue };
            match word.as_str() {
                "--on" => s.enabled = true,
                "--off" => s.enabled = false,
                "--mode" | "--custom-mode" => s.mode = words.next().and_then(|m| resolution(&m)),
                "--pos" => s.position = words.next().and_then(|p| {
                    let (x, y) = p.split_once(',')?;
                    Some((x.trim().parse().ok()?, y.trim().parse().ok()?))
                }),
                "--transform" => s.turn = words.next().and_then(|t| Turn::from_name(&t)),
                "--scale" => s.scale = words.next().and_then(|v| v.parse().ok()).filter(|v: &f64| *v > 0.0),
                _ => {}
            }
        }
    }
    settings
}

fn resolution(text: &str) -> Option<Resolution> {
    let (size, rate) = match text.split_once('@') {
        Some((s, r)) => (s, Some(r.trim_end_matches("Hz"))),
        None => (text, None),
    };
    let (w, h) = size.split_once('x')?;
    let refresh = match rate {
        Some(r) => (r.parse::<f64>().ok()? * 1000.0).round() as i32,
        None => 0,
    };
    Some(Resolution { width: w.parse().ok()?, height: h.parse().ok()?, refresh })
}

fn shell_words(line: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut word = String::new();
    let (mut quoted, mut any) = (false, false);
    for c in line.chars() {
        match c {
            '\'' => {
                quoted = !quoted;
                any = true;
            }
            c if c.is_whitespace() && !quoted => {
                if any {
                    words.push(std::mem::take(&mut word));
                    any = false;
                }
            }
            c => {
                word.push(c);
                any = true;
            }
        }
    }
    if any {
        words.push(word);
    }
    words
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Outcome {
    Applied,
    Refused,

    Stale,
}

struct Entry {
    proxy: TrindleScreenV1,
    head: Head,
}

#[derive(Default)]
struct State {
    manager: Option<TrindleScreensManagerV1>,
    entries: Vec<Entry>,

    serial: Option<u32>,

    outcome: Option<(Outcome, String)>,
}

impl State {
    fn heads(&self) -> Vec<Head> {
        self.entries.iter().map(|e| e.head.clone()).collect()
    }
}

struct Link {
    conn: Connection,
    queue: EventQueue<State>,
    state: State,
}

impl Link {
    fn open() -> Result<Link, String> {
        let conn = Connection::connect_to_env().map_err(|_| "There is no desktop to ask.".to_string())?;
        let (globals, queue) = registry_queue_init::<State>(&conn).map_err(|e| e.to_string())?;
        let qh = queue.handle();
        let manager: TrindleScreensManagerV1 = globals
            .bind(&qh, 1..=1, ())
            .map_err(|_| "This desktop cannot say what displays there are.".to_string())?;
        let mut link = Link { conn, queue, state: State { manager: Some(manager), ..State::default() } };
        link.settle()?;
        Ok(link)
    }

    fn settle(&mut self) -> Result<(), String> {
        let before = self.state.serial.take();

        for _ in 0..2 {
            self.queue.roundtrip(&mut self.state).map_err(|e| e.to_string())?;
            if self.state.serial.is_some_and(|now| Some(now) != before) {
                return Ok(());
            }
        }

        self.state.serial = before;
        if before.is_some() {
            return Ok(());
        }
        Err("The desktop did not say what displays there are.".into())
    }

    fn apply(&mut self, settings: &[Setting]) -> Result<(Outcome, String), String> {
        let (Some(manager), Some(serial)) = (self.state.manager.clone(), self.state.serial) else {
            return Err("The desktop did not say what displays there are.".into());
        };
        let heads = self.state.heads();

        let wanted: Vec<Setting> = heads
            .iter()
            .map(|h| settings.iter().find(|s| s.name == h.name).cloned().unwrap_or_else(|| h.setting()))
            .collect();
        if !heads.is_empty() && !wanted.iter().any(|s| s.enabled) {
            return Err("That would leave no display on.".into());
        }
        let wanted = clear_unnamed(&heads, settings, wanted);
        let qh = self.queue.handle();
        let plan = manager.plan(serial, &qh, ());
        for (want, (head, entry)) in wanted.iter().zip(heads.iter().zip(self.state.entries.iter())) {
            let (width, height, millihertz) = want.mode.map_or((0, 0, 0), |r| (r.width, r.height, r.refresh));

            let position = want.position.unwrap_or(head.position);
            let turn = turn_for(want.turn, head.turn).number();
            let scale = ((want.scale.unwrap_or(head.scale)) * 120.0).round().max(0.0) as u32;
            plan.screen(&entry.proxy, u32::from(want.enabled), width, height, millihertz, position.0, position.1, turn,
                        scale, u32::from(head.varies_rate));
        }
        self.state.outcome = None;
        plan.apply();
        while self.state.outcome.is_none() {
            if self.queue.blocking_dispatch(&mut self.state).is_err() {
                plan.destroy();
                return Err("The desktop stopped answering.".into());
            }
        }
        plan.destroy();
        let _ = self.conn.flush();
        Ok(self.state.outcome.take().unwrap_or((Outcome::Refused, String::new())))
    }
}

impl Dispatch<TrindleScreensManagerV1, ()> for State {
    fn event(state: &mut Self, _: &TrindleScreensManagerV1, event: trindle_screens_manager_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        match event {
            trindle_screens_manager_v1::Event::Screen { screen } => state.entries.push(Entry {
                proxy: screen,
                head: Head {
                    name: String::new(),
                    description: String::new(),
                    make: String::new(),
                    model: String::new(),
                    serial: String::new(),
                    physical_size: (0, 0),
                    modes: Vec::new(),
                    current: None,
                    enabled: false,
                    position: (0, 0),
                    turn: Turn::Normal,
                    scale: 1.0,
                    varies_rate: false,
                },
            }),
            trindle_screens_manager_v1::Event::Settled { serial } => state.serial = Some(serial),
        }
    }

    event_created_child!(State, TrindleScreensManagerV1, [
        trindle_screens_manager_v1::EVT_SCREEN_OPCODE => (TrindleScreenV1, ()),
    ]);
}

impl Dispatch<TrindleScreenV1, ()> for State {
    fn event(state: &mut Self, proxy: &TrindleScreenV1, event: trindle_screen_v1::Event, _: &(), _: &Connection,
             _: &QueueHandle<Self>) {
        use trindle_screen_v1::Event;
        if let Event::Unplugged = event {
            state.entries.retain(|e| e.proxy != *proxy);
            proxy.destroy();
            return;
        }
        let Some(e) = state.entries.iter_mut().find(|e| e.proxy == *proxy) else { return };
        match event {

            Event::Clear => {
                e.head.modes.clear();
                e.head.current = None;
            }
            Event::Identity { name, description, make, model, serial_number } => {
                e.head.name = name;
                e.head.description = description;
                e.head.make = make;
                e.head.model = model;
                e.head.serial = serial_number;
            }
            Event::SizeMm { width, height } => e.head.physical_size = (width, height),
            Event::Mode { width, height, millihertz, flags } => {
                e.head.modes.push(Mode { width, height, refresh: millihertz, preferred: flags & 1 != 0 })
            }
            Event::Layout { on, mode, x, y, turn, scale_120ths, varies_rate } => {
                e.head.enabled = on != 0;
                e.head.current = usize::try_from(mode).ok().filter(|&i| on != 0 && i < e.head.modes.len());
                e.head.position = (x, y);
                e.head.turn = Turn::from_number(turn);
                e.head.scale = f64::from(scale_120ths) / 120.0;
                e.head.varies_rate = varies_rate != 0;
            }
            Event::Unplugged => {}
        }
    }
}

impl Dispatch<TrindleScreenPlanV1, ()> for State {
    fn event(state: &mut Self, _: &TrindleScreenPlanV1, event: trindle_screen_plan_v1::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {
        let trindle_screen_plan_v1::Event::Outcome { outcome: result, why } = event;
        let outcome = match result {
            WEnum::Value(trindle_screen_plan_v1::Outcome::Applied) => Outcome::Applied,
            WEnum::Value(trindle_screen_plan_v1::Outcome::Stale) => Outcome::Stale,
            _ => Outcome::Refused,
        };
        state.outcome = Some((outcome, why));
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(_: &mut Self, _: &wl_registry::WlRegistry, _: wl_registry::Event, _: &GlobalListContents,
             _: &Connection, _: &QueueHandle<Self>) {}
}

#[cfg(test)]
mod harness {
    use super::*;

    #[test]
    #[ignore]
    fn set_harness_screen() {
        let mode = std::env::var("TRINDLE_HARNESS_MODE").ok().and_then(|m| resolution(&m));
        let scale = std::env::var("TRINDLE_HARNESS_SCALE").ok().and_then(|v| v.parse::<f64>().ok());
        if let Ok(w) = std::env::var("TRINDLE_HARNESS_WAYLAND") {
            std::env::set_var("WAYLAND_DISPLAY", w);
        }
        let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_default();
        assert!(runtime.contains("rtb"), "only the harness's own compositor, not {runtime}");
        let changes: Vec<Setting> = heads().unwrap().iter().map(|h| Setting { mode, scale, ..Setting::on(&h.name) }).collect();
        apply(&changes).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mode(width: i32, height: i32, refresh: i32, preferred: bool) -> Mode {
        Mode { width, height, refresh, preferred }
    }

    #[test]
    fn a_screen_not_in_the_saved_layout_is_moved_clear_of_the_ones_that_are() {

        let rects = [(3210, 0, 1289, 859), (2256, 0, 1920, 1080)];
        assert_eq!(move_clear(&rects, &[false, true]), vec![None, Some((4499, 0))]);

        assert_eq!(move_clear(&[(0, 0, 100, 100), (100, 0, 100, 100)], &[true, true]), vec![None, None]);
        assert_eq!(move_clear(&[(0, 0, 100, 100), (50, 0, 100, 100)], &[false, false]), vec![None, None]);

        assert_eq!(move_clear(&[(0, 0, 100, 100), (0, 0, 0, 0)], &[true, true]), vec![None, None]);
    }

    #[test]
    fn old_display_sh_reads_across() {
        let text = "#!/bin/sh\n# Written by Trindle Settings.\n\
            wlr-randr --output eDP-1 --on --mode 1920x1080@59.951Hz --scale 1.25 --transform normal --pos 0,0 2>/dev/null\n\
            wlr-randr --output 'HDMI A 1' --off 2>/dev/null\n\
            wlr-randr --output DP-2 --on --scale 1 --transform 90 --pos 1536,-10 2>/dev/null\n";
        let got = from_wlr_randr(text);
        assert_eq!(got.len(), 3);
        assert_eq!(got[0], Setting {
            name: "eDP-1".into(),
            enabled: true,
            mode: Some(Resolution { width: 1920, height: 1080, refresh: 59951 }),
            position: Some((0, 0)),
            turn: Some(Turn::Normal),
            scale: Some(1.25),
        });
        assert_eq!(got[1], Setting::off("HDMI A 1"));
        assert_eq!(got[2].turn, Some(Turn::R90));
        assert_eq!(got[2].position, Some((1536, -10)));
        assert_eq!(got[2].mode, None);
    }

    #[test]
    fn splits_display_lines() {
        assert_eq!(shell_words("wlr-randr --output 'HDMI A 1' --scale 1.25"),
                   vec!["wlr-randr", "--output", "HDMI A 1", "--scale", "1.25"]);
        assert_eq!(shell_words("a '' b"), vec!["a", "", "b"]);
    }

    #[test]
    fn kept_settings_survive_the_file() {
        let s = vec![Setting { scale: Some(1.5), turn: Some(Turn::Flipped270), ..Setting::on("DP-1") }, Setting::off("eDP-1")];
        let text = serde_json::to_string(&s).unwrap();
        assert!(text.contains("\"flipped-270\""));
        assert_eq!(serde_json::from_str::<Vec<Setting>>(&text).unwrap(), s);
    }

    #[test]
    fn every_way_up_has_a_number_and_comes_back() {
        for (turn, name, number) in TURNS {
            assert_eq!(turn.number(), number);
            assert_eq!(Turn::from_number(number), turn);
            assert_eq!(Turn::from_name(name), Some(turn));
        }
        assert_eq!(Turn::from_number(99), Turn::Normal);
    }

    #[test]
    fn a_saved_turn_beats_the_panels_own_and_none_keeps_it() {
        assert_eq!(turn_for(None, Turn::R270), Turn::R270);
        assert_eq!(turn_for(Some(Turn::Normal), Turn::R270), Turn::Normal);
        assert_eq!(turn_for(Some(Turn::R90), Turn::R270), Turn::R90);
    }

    #[test]
    fn a_turned_screen_is_taller_than_wide() {
        let mut h = Head {
            name: "DP-1".into(), description: String::new(), make: String::new(), model: String::new(),
            serial: String::new(), physical_size: (0, 0), modes: vec![mode(1920, 1080, 60000, true)],
            current: Some(0), enabled: true, position: (0, 0), turn: Turn::Normal, scale: 1.25, varies_rate: false,
        };
        assert_eq!(h.logical_size(), (1536, 864));
        h.turn = Turn::R270;
        assert_eq!(h.logical_size(), (864, 1536));
    }
}
