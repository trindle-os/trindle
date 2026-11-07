//! Game controller profiles.

use std::path::PathBuf;

use super::files;
use crate::toolkit::t;
use super::remap;

#[path = "pads/family.rs"]
mod family;
#[path = "pads/live.rs"]
mod live;
#[allow(unused_imports)]
pub use family::Family;
#[allow(unused_imports)]
pub use live::{Layout, Live};

const KEY: &str = "Key";

pub const EV_KEY: u16 = 1;
pub const EV_ABS: u16 = 3;
const MOST_LINES: usize = 512;
pub const MOST_BYTES: u64 = 64 * 1024;
const MOST_DEAD: u8 = 50;
const LONGEST_NAME: usize = 64;

const BUTTONS: &[(u16, &str, &[&str])] = &[
    (0x130, "South", &["A", "Cross"]),
    (0x131, "East", &["B", "Circle"]),
    (0x133, "North", &["X", "Triangle"]),
    (0x134, "West", &["Y", "Square"]),
    (0x136, "Left Shoulder", &["LB", "L1"]),
    (0x137, "Right Shoulder", &["RB", "R1"]),
    (0x138, "Left Trigger Button", &["L2 Button"]),
    (0x139, "Right Trigger Button", &["R2 Button"]),
    (0x13a, "Select", &["Back", "Share", "View"]),
    (0x13b, "Start", &["Options", "Menu"]),
    (0x13c, "Home", &["Guide", "PS"]),
    (0x13d, "Left Stick Press", &["L3"]),
    (0x13e, "Right Stick Press", &["R3"]),
    (0x220, "D-pad Up", &[]),
    (0x221, "D-pad Down", &[]),
    (0x222, "D-pad Left", &[]),
    (0x223, "D-pad Right", &[]),
];

const AXES: &[(u16, &str, &[&str])] = &[
    (0, "Left Stick X", &[]),
    (1, "Left Stick Y", &[]),
    (2, "Left Trigger", &["L2", "LT"]),
    (3, "Right Stick X", &[]),
    (4, "Right Stick Y", &[]),
    (5, "Right Trigger", &["R2", "RT"]),
    (16, "D-pad X", &[]),
    (17, "D-pad Y", &[]),
];
const LEFT_STICK: [u16; 2] = [0, 1];
const RIGHT_STICK: [u16; 2] = [3, 4];
const TRIGGERS: [u16; 2] = [2, 5];

pub const NOT_A_BUTTON: &str = "Not a button Trindle knows. Try South, Left Trigger or Escape.";
pub const NO_EQUALS: &str = "A change needs what is pressed, an equals sign, and what it becomes.";
pub const NOT_A_ZONE: &str = "A dead zone is from 0% to 50%.";

fn find(list: &[(u16, &str, &[&str])], word: &str) -> Option<u16> {
    let word = word.trim();
    list.iter().find(|n| n.1.eq_ignore_ascii_case(word) || n.2.iter().any(|a| a.eq_ignore_ascii_case(word))).map(|n| n.0)
}

fn named(list: &[(u16, &str, &[&str])], code: u16) -> String {
    list.iter().find(|n| n.0 == code).map(|n| n.1.to_string()).unwrap_or_else(|| format!("Code {code}"))
}

pub fn button_name(code: u16) -> String {
    named(BUTTONS, code)
}

pub fn axis_name(code: u16) -> String {
    named(AXES, code)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum To {
    Nothing,
    Button(u16),
    Trigger(u16),
    Keys(Vec<u32>),
}

impl To {
    pub fn words(&self) -> String {
        match self {
            To::Nothing => "Nothing".into(),
            To::Button(b) => button_name(*b),
            To::Trigger(a) => axis_name(*a),
            To::Keys(keys) => {
                let words = remap::To::Held(keys.clone()).words();
                if find(BUTTONS, &words).is_some() || find(AXES, &words).is_some_and(|a| TRIGGERS.contains(&a)) {
                    format!("{KEY} {words}")
                } else {
                    words
                }
            }
        }
    }

    fn parse(words: &str) -> Result<To, &'static str> {
        let words = words.trim();
        if words.eq_ignore_ascii_case("nothing") {
            return Ok(To::Nothing);
        }
        if let Some(key) = words.split_once(' ').filter(|(w, _)| w.eq_ignore_ascii_case(KEY)).map(|(_, rest)| rest.trim()) {
            return match remap::To::parse(key) {
                Ok(remap::To::Held(keys)) => Ok(To::Keys(keys)),
                _ => Err(NOT_A_BUTTON),
            };
        }
        if let Some(b) = find(BUTTONS, words) {
            return Ok(To::Button(b));
        }
        if let Some(a) = find(AXES, words).filter(|a| TRIGGERS.contains(a)) {
            return Ok(To::Trigger(a));
        }
        match remap::To::parse(words) {
            Ok(remap::To::Held(keys)) => Ok(To::Keys(keys)),
            _ => Err(NOT_A_BUTTON),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Change {
    Into { from: From, to: To },
    Inverted(u16),
    SticksSwapped,
    DeadZone { stick: u16, share: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum From {
    Button(u16),
    Trigger(u16),
    Direction(u16, bool),
}

const DIRECTIONS: &[(u16, bool, &str)] = &[
    (0, false, "Left Stick Left"), (0, true, "Left Stick Right"), (1, false, "Left Stick Up"), (1, true, "Left Stick Down"),
    (3, false, "Right Stick Left"), (3, true, "Right Stick Right"), (4, false, "Right Stick Up"), (4, true, "Right Stick Down"),
];

fn direction_name(axis: u16, positive: bool) -> String {
    DIRECTIONS.iter().find(|d| d.0 == axis && d.1 == positive).map(|d| d.2.to_string()).unwrap_or_else(|| format!("Code {axis}"))
}

fn hat_buttons(axis: u16) -> Option<(u16, u16)> {
    match axis {
        16 => Some((0x222, 0x223)),
        17 => Some((0x220, 0x221)),
        _ => None,
    }
}

impl Change {
    pub fn line(&self) -> String {
        match self {
            Change::Into { from: From::Button(b), to } => format!("{} = {}", button_name(*b), to.words()),
            Change::Into { from: From::Trigger(a), to } => format!("{} = {}", axis_name(*a), to.words()),
            Change::Into { from: From::Direction(a, up), to } => format!("{} = {}", direction_name(*a, *up), to.words()),
            Change::Inverted(a) => format!("{} = inverted", axis_name(*a)),
            Change::SticksSwapped => "Sticks = swapped".into(),
            Change::DeadZone { stick, share } => {
                let side = if *stick == LEFT_STICK[0] { "Left" } else { "Right" };
                format!("{side} Stick dead zone = {share}%")
            }
        }
    }

    pub fn parse(line: &str) -> Result<Change, &'static str> {
        let (left, right) = line.split_once('=').ok_or(NO_EQUALS)?;
        let (left, right) = (left.trim(), right.trim());
        if left.eq_ignore_ascii_case("sticks") && right.eq_ignore_ascii_case("swapped") {
            return Ok(Change::SticksSwapped);
        }
        for (side, stick) in [("left", LEFT_STICK[0]), ("right", RIGHT_STICK[0])] {
            if left.eq_ignore_ascii_case(&format!("{side} stick dead zone")) {
                let share: u8 = right.trim_end_matches('%').trim().parse().map_err(|_| NOT_A_ZONE)?;
                if share > MOST_DEAD {
                    return Err(NOT_A_ZONE);
                }
                return Ok(Change::DeadZone { stick, share });
            }
        }
        if let Some(d) = DIRECTIONS.iter().find(|d| d.2.eq_ignore_ascii_case(left)) {
            return Ok(Change::Into { from: From::Direction(d.0, d.1), to: To::parse(right)? });
        }
        if let Some(axis) = find(AXES, left) {
            if right.eq_ignore_ascii_case("inverted") {
                return Ok(Change::Inverted(axis));
            }
            if TRIGGERS.contains(&axis) {
                return Ok(Change::Into { from: From::Trigger(axis), to: To::parse(right)? });
            }
            return Err(NOT_A_BUTTON);
        }
        let from = find(BUTTONS, left).ok_or(NOT_A_BUTTON)?;
        Ok(Change::Into { from: From::Button(from), to: To::parse(right)? })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pad {
    pub vendor: u16,
    pub product: u16,
    pub name: String,
}

impl Pad {
    fn heading(&self) -> String {
        format!("[{:04x}:{:04x} {}]", self.vendor, self.product, self.name)
    }

    fn parse(line: &str) -> Option<Pad> {
        let inner = line.trim().strip_prefix('[')?.strip_suffix(']')?;
        let (ids, name) = inner.split_once(' ').unwrap_or((inner, ""));
        let (v, p) = ids.split_once(':')?;
        Some(Pad { vendor: u16::from_str_radix(v, 16).ok()?, product: u16::from_str_radix(p, 16).ok()?, name: clean(name) })
    }
}

fn clean(name: &str) -> String {
    name.chars().filter(|c| !c.is_control() && !matches!(c, '[' | ']')).take(LONGEST_NAME).collect::<String>().trim().to_string()
}

pub const STANDARD: &str = "Standard";
pub const FIRST: &str = "My keys";
const PROFILE: &str = "Profile";
const GAMES: &str = "[Games]";
const CHANGES: &str = "Changes";
const OTHER_GAME: &str = "Any other game";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Profile {
    pub pad: Pad,
    pub name: String,
    pub changes: Vec<Change>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Profiles {
    pub pads: Vec<Profile>,
    pub games: Vec<(u32, String)>,
    pub other: Option<String>,
    pub off: bool,
    pub unread: Vec<(usize, String, &'static str)>,
}

pub fn profile_name(name: &str) -> Result<String, &'static str> {
    let name = clean(name);
    if name.is_empty() {
        Err(t!("settings-pads-profile-empty"))
    } else if name.eq_ignore_ascii_case(STANDARD) {
        Err(t!("settings-pads-profile-standard"))
    } else {
        Ok(name)
    }
}

impl Profiles {
    pub fn read(text: &str) -> Profiles {
        let mut out = Profiles::default();
        let (mut in_games, mut games_seen) = (false, false);
        for (i, line) in text.lines().enumerate().take(MOST_LINES) {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.eq_ignore_ascii_case(GAMES) {
                (in_games, games_seen) = (true, true);
                continue;
            }
            if line.starts_with('[') {
                in_games = false;
                match Pad::parse(line) {
                    Some(pad) => out.pads.push(Profile { pad, name: FIRST.into(), changes: Vec::new() }),
                    None => out.unread.push((i + 1, line.to_string(), "Not a controller's heading.")),
                }
                continue;
            }
            if in_games {
                if let Err(why) = out.game_line(line) {
                    out.unread.push((i + 1, line.to_string(), why));
                }
                continue;
            }
            let named = line.split_once('=').filter(|(l, _)| l.trim().eq_ignore_ascii_case(PROFILE)).map(|(_, r)| profile_name(r));
            match (out.pads.last_mut(), named) {
                (None, _) => out.unread.push((i + 1, line.to_string(), "It is under no controller's heading.")),
                (Some(p), Some(Ok(name))) => p.name = name,
                (Some(_), Some(Err(why))) => out.unread.push((i + 1, line.to_string(), why)),
                (Some(p), None) => match Change::parse(line) {
                    Ok(change) => p.changes.push(change),
                    Err(why) => out.unread.push((i + 1, line.to_string(), why)),
                },
            }
        }
        if !games_seen && out.pads.iter().any(|p| p.name == FIRST) {
            out.other = Some(FIRST.into());
        }
        out
    }

    fn game_line(&mut self, line: &str) -> Result<(), &'static str> {
        let (left, right) = line.split_once('=').ok_or(NO_EQUALS)?;
        let (left, right) = (left.trim(), right.trim());
        let named = profile_name(right).ok();
        if left.eq_ignore_ascii_case(CHANGES) {
            self.off = right.eq_ignore_ascii_case("off");
        } else if left.eq_ignore_ascii_case(OTHER_GAME) {
            self.other = named;
        } else {
            let id: u32 = left.parse().map_err(|_| t!("settings-pads-not-a-game"))?;
            self.choose(Some(id), named);
        }
        Ok(())
    }

    pub fn text(&self) -> String {
        let mut text = String::new();
        for p in &self.pads {
            text.push_str(&format!("{}\n{PROFILE} = {}\n", p.pad.heading(), p.name));
            for c in &p.changes {
                text.push_str(&c.line());
                text.push('\n');
            }
        }
        if !self.pads.is_empty() || self.off || self.other.is_some() || !self.games.is_empty() {
            text.push_str(&format!("{GAMES}\n"));
            if self.off {
                text.push_str(&format!("{CHANGES} = off\n"));
            }
            text.push_str(&format!("{OTHER_GAME} = {}\n", self.other.as_deref().unwrap_or(STANDARD)));
            for (id, name) in &self.games {
                text.push_str(&format!("{id} = {name}\n"));
            }
        }
        text
    }

    pub fn chosen(&self, game: Option<u32>) -> Option<&str> {
        match game {
            Some(id) => self.games.iter().find(|g| g.0 == id).map(|g| g.1.as_str()),
            None => self.other.as_deref(),
        }
    }

    pub fn choose(&mut self, game: Option<u32>, name: Option<String>) {
        match game {
            None => self.other = name,
            Some(id) => {
                self.games.retain(|g| g.0 != id);
                if let Some(name) = name {
                    self.games.push((id, name));
                    self.games.sort_unstable_by_key(|g| g.0);
                }
            }
        }
    }

    pub fn game_now(&self, running: &[u32]) -> Option<u32> {
        running.iter().copied().find(|id| self.chosen(Some(*id)).is_some()).or_else(|| running.first().copied())
    }

    pub fn in_force(&self, vendor: u16, product: u16, name: &str, running: &[u32]) -> Option<&[Change]> {
        if self.off {
            return None;
        }
        let profile = self.chosen(self.game_now(running))?;
        let mine = |p: &&Profile| p.name == profile && p.pad.vendor == vendor && p.pad.product == product;
        self.pads.iter().filter(mine).find(|p| p.pad.name == clean(name))
            .or_else(|| self.pads.iter().filter(mine).find(|p| p.pad.name.is_empty()))
            .map(|p| p.changes.as_slice())
    }

    pub fn names_of(&self, pad: &Pad) -> Vec<String> {
        self.pads.iter().filter(|p| p.pad == *pad).map(|p| p.name.clone()).collect()
    }

    pub fn add(&mut self, pad: &Pad, name: &str) -> Result<String, &'static str> {
        let name = profile_name(name)?;
        if self.index_of(pad, &name).is_some() {
            return Err(t!("settings-pads-profile-taken"));
        }
        self.pads.push(Profile { pad: pad.clone(), name: name.clone(), changes: Vec::new() });
        Ok(name)
    }

    fn rechoose(&mut self, old: &str, new: Option<&str>) {
        if self.pads.iter().any(|p| p.name == old) {
            return;
        }
        if self.other.as_deref() == Some(old) {
            self.other = new.map(str::to_string);
        }
        match new {
            Some(n) => self.games.iter_mut().filter(|g| g.1 == old).for_each(|g| g.1 = n.to_string()),
            None => self.games.retain(|g| g.1 != old),
        }
    }

    pub fn rename(&mut self, pad: &Pad, old: &str, new: &str) -> Result<String, &'static str> {
        let new = profile_name(new)?;
        if new == old {
            return Ok(new);
        }
        if self.index_of(pad, &new).is_some() {
            return Err(t!("settings-pads-profile-taken"));
        }
        let at = self.index_of(pad, old).ok_or(t!("settings-pads-profile-gone"))?;
        self.pads[at].name = new.clone();
        self.rechoose(old, Some(&new));
        Ok(new)
    }

    pub fn remove(&mut self, pad: &Pad, name: &str) {
        if let Some(at) = self.index_of(pad, name) {
            self.pads.remove(at);
            self.rechoose(name, None);
        }
    }
}

pub fn steam_games_now(proc: &std::path::Path) -> Vec<u32> {
    let Ok(all) = std::fs::read_dir(proc) else { return Vec::new() };
    let mut found: Vec<u32> = all.flatten().take(1 << 16)
        .filter(|e| e.file_name().to_string_lossy().bytes().all(|b| b.is_ascii_digit()))
        .filter_map(|e| {
            let line = std::fs::read(e.path().join("cmdline")).ok()?;
            let args: Vec<&[u8]> = line.split(|&b| b == 0).collect();
            if !args.contains(&&b"SteamLaunch"[..]) {
                return None;
            }
            args.iter().find_map(|a| std::str::from_utf8(a.strip_prefix(b"AppId=")?).ok()?.parse().ok())
        })
        .collect();
    found.sort_unstable();
    found.dedup();
    found
}

pub fn file() -> PathBuf {
    files::trindle_config().join("controllers.conf")
}

pub fn load() -> std::io::Result<Profiles> {
    files::read_within(&file(), MOST_BYTES).map(|t| Profiles::read(&t))
}

pub fn save(profiles: &Profiles) -> std::io::Result<()> {
    files::write_whole(&file(), profiles.text().as_bytes())
}

pub const SWAPS: &[&[&str]] = &[
    &["South = East", "East = South"],
    &["North = West", "West = North"],
    &["Right Stick Y = inverted"],
];

pub fn swap_name(i: usize) -> &'static str {
    match i {
        0 => t!("settings-keyboard-pad-swap-a-b"),
        1 => t!("settings-keyboard-pad-swap-x-y"),
        _ => t!("settings-keyboard-pad-swap-stick"),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out {
    Pad(u16, u16, i32),
    Key(u32, bool),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Range {
    pub min: i32,
    pub max: i32,
}

impl Range {
    fn middle(self) -> i32 {
        ((i64::from(self.min) + i64::from(self.max)) / 2) as i32
    }
    fn turned(self, v: i32) -> i32 {
        (i64::from(self.max) + i64::from(self.min) - i64::from(v)).clamp(i64::from(self.min), i64::from(self.max)) as i32
    }
    fn pulled(self, v: i32) -> bool {
        i64::from(v) * 2 >= i64::from(self.min) + i64::from(self.max) && v > self.min
    }
}

#[derive(Debug, Default)]
pub struct Changer {
    changes: Vec<Change>,
    ranges: Vec<(u16, Range)>,
    pulled: Vec<(u16, bool)>,
    lit: Vec<(From, bool)>,
}

impl Changer {
    pub fn new(changes: &[Change], ranges: &[(u16, Range)]) -> Changer {
        Changer { changes: changes.to_vec(), ranges: ranges.to_vec(), pulled: Vec::new(), lit: Vec::new() }
    }

    fn range(&self, axis: u16) -> Range {
        self.ranges.iter().find(|r| r.0 == axis).map(|r| r.1).unwrap_or(Range { min: -32768, max: 32767 })
    }

    fn target_of(&self, from: From) -> Option<&To> {
        self.changes.iter().find_map(|c| match c {
            Change::Into { from: f, to } if *f == from => Some(to),
            _ => None,
        })
    }

    fn to_out(&self, to: &To, down: bool) -> Vec<Out> {
        match to {
            To::Nothing => Vec::new(),
            To::Button(b) => vec![Out::Pad(EV_KEY, *b, i32::from(down))],
            To::Trigger(a) => {
                let r = self.range(*a);
                vec![Out::Pad(EV_ABS, *a, if down { r.max } else { r.min })]
            }
            To::Keys(keys) if down => keys.iter().map(|k| Out::Key(*k, true)).collect(),
            To::Keys(keys) => keys.iter().rev().map(|k| Out::Key(*k, false)).collect(),
        }
    }

    pub fn event(&mut self, kind: u16, code: u16, value: i32) -> Vec<Out> {
        match kind {
            EV_KEY => match self.target_of(From::Button(code)) {
                Some(to) => self.to_out(&to.clone(), value != 0),
                None => vec![Out::Pad(kind, code, value)],
            },
            EV_ABS => self.axis(code, value),
            _ => vec![Out::Pad(kind, code, value)],
        }
    }

    fn watch(&mut self, from: From, now: bool) -> Option<Vec<Out>> {
        let to = self.target_of(from)?.clone();
        let was = self.lit.iter().find(|l| l.0 == from).map(|l| l.1).unwrap_or(false);
        self.lit.retain(|l| l.0 != from);
        self.lit.push((from, now));
        Some(if was != now { self.to_out(&to, now) } else { Vec::new() })
    }

    fn ways(&mut self, code: u16, value: i32) -> (Vec<Out>, bool) {
        let r = self.range(code);
        let off = i64::from(value) - i64::from(r.middle());
        let half = (i64::from(r.max) - i64::from(r.min)) / 2;
        let mut outs = Vec::new();
        let mut changed = [false; 2];
        let mut take = |me: &mut Changer, at: usize, from: From, now: bool| {
            if let Some(o) = me.watch(from, now) {
                changed[at] = true;
                outs.extend(o);
            }
        };
        let mut ways = [(From::Direction(code, false), off * 2 < -half), (From::Direction(code, true), off * 2 > half)];
        if let Some((lo, hi)) = hat_buttons(code) {
            ways = [(From::Button(lo), off < 0), (From::Button(hi), off > 0)];
        }
        for (at, (from, now)) in ways.into_iter().enumerate() {
            take(self, at, from, now);
        }
        outs.sort_by_key(|o| matches!(o, Out::Key(_, true)));
        (outs, changed == [true, true])
    }

    fn axis(&mut self, code: u16, value: i32) -> Vec<Out> {
        let (mut outs, whole) = self.ways(code, value);
        if !whole {
            outs.extend(self.plain_axis(code, value));
        }
        outs
    }

    fn plain_axis(&mut self, code: u16, value: i32) -> Vec<Out> {
        if TRIGGERS.contains(&code) {
            if let Some(to) = self.target_of(From::Trigger(code)).cloned() {
                let held = self.range(code).pulled(value);
                let was = self.pulled.iter().find(|p| p.0 == code).map(|p| p.1).unwrap_or(false);
                self.pulled.retain(|p| p.0 != code);
                self.pulled.push((code, held));
                return if held != was { self.to_out(&to, held) } else { Vec::new() };
            }
        }
        let swapped = self.changes.contains(&Change::SticksSwapped);
        let (stick, other) = if LEFT_STICK.contains(&code) {
            (LEFT_STICK, RIGHT_STICK)
        } else if RIGHT_STICK.contains(&code) {
            (RIGHT_STICK, LEFT_STICK)
        } else {
            ([code, code], [code, code])
        };
        let r = self.range(code);
        let mut v = value;
        let dead = self.changes.iter().find_map(|c| match c {
            Change::DeadZone { stick: s, share } if *s == stick[0] => Some(*share),
            _ => None,
        });
        if let Some(share) = dead {
            let half = (i64::from(r.max) - i64::from(r.min)) / 2;
            if (i64::from(v) - i64::from(r.middle())).abs() * 100 <= half * i64::from(share) {
                v = r.middle();
            }
        }
        if self.changes.contains(&Change::Inverted(code)) {
            v = r.turned(v);
        }
        let to = if swapped && stick != other { other[if code == stick[0] { 0 } else { 1 }] } else { code };
        vec![Out::Pad(EV_ABS, to, v)]
    }
}

impl Profiles {
    pub fn index_of(&self, pad: &Pad, name: &str) -> Option<usize> {
        self.pads.iter().position(|p| p.pad == *pad && p.name == name)
    }

    pub fn has_swap(&self, pad: &Pad, name: &str, i: usize) -> bool {
        let Some(at) = self.index_of(pad, name) else { return false };
        SWAPS[i].iter().filter_map(|l| Change::parse(l).ok()).all(|c| self.pads[at].changes.contains(&c))
    }

    fn changes_mut(&mut self, pad: &Pad, name: &str) -> &mut Vec<Change> {
        let at = self.index_of(pad, name).unwrap_or_else(|| {
            self.pads.push(Profile { pad: pad.clone(), name: name.to_string(), changes: Vec::new() });
            self.pads.len() - 1
        });
        &mut self.pads[at].changes
    }

    pub fn set_swap(&mut self, pad: &Pad, name: &str, i: usize, on: bool) {
        let lines: Vec<Change> = SWAPS[i].iter().filter_map(|l| Change::parse(l).ok()).collect();
        let changes = self.changes_mut(pad, name);
        changes.retain(|c| !lines.contains(c));
        if on {
            changes.extend(lines);
        }
    }

    pub fn own(&self, pad: &Pad, name: &str) -> Vec<(usize, &Change)> {
        let Some(at) = self.index_of(pad, name) else { return Vec::new() };
        let swapped: Vec<Change> = (0..SWAPS.len()).filter(|i| self.has_swap(pad, name, *i))
            .flat_map(|i| SWAPS[i].iter().filter_map(|l| Change::parse(l).ok())).collect();
        self.pads[at].changes.iter().enumerate().filter(|(_, c)| !swapped.contains(c)).collect()
    }
}

impl From {
    pub fn name(self) -> String {
        match self {
            From::Button(b) => button_name(b),
            From::Trigger(a) => axis_name(a),
            From::Direction(a, positive) => direction_name(a, positive),
        }
    }
}

impl To {
    pub fn named(name: &str) -> Option<To> {
        To::parse(name).ok()
    }
}

pub fn inputs() -> Vec<From> {
    const ORDER: [u16; 15] = [0x130, 0x131, 0x133, 0x134, 0x136, 0x137, 0x13a, 0x13b, 0x13c, 0x13d, 0x13e, 0x220, 0x221, 0x222, 0x223];
    let mut out: Vec<From> = ORDER.iter().map(|b| From::Button(*b)).collect();
    out.splice(4..4, TRIGGERS.iter().map(|a| From::Trigger(*a)));
    out.extend(DIRECTIONS.iter().map(|d| From::Direction(d.0, d.1)));
    out
}

impl Profiles {
    pub fn changed_to(&self, pad: &Pad, name: &str, from: From) -> Option<&To> {
        let at = self.index_of(pad, name)?;
        self.pads[at].changes.iter().find_map(|c| match c {
            Change::Into { from: f, to } if *f == from => Some(to),
            _ => None,
        })
    }

    pub fn set_into(&mut self, pad: &Pad, name: &str, from: From, to: To) {
        let changes = self.changes_mut(pad, name);
        changes.retain(|c| !matches!(c, Change::Into { from: f, .. } if *f == from));
        changes.push(Change::Into { from, to });
    }

    pub fn take_into(&mut self, pad: &Pad, name: &str, from: From) {
        if let Some(at) = self.index_of(pad, name) {
            self.pads[at].changes.retain(|c| !matches!(c, Change::Into { from: f, .. } if *f == from));
        }
    }
}

pub fn names() -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = BUTTONS.iter().map(|b| (b.1.to_string(), format!("{} = {}", b.1, b.1))).collect();
    out.extend(TRIGGERS.iter().map(|a| (axis_name(*a), format!("{} = {}", axis_name(*a), axis_name(*a)))));
    out
}

pub fn listed_in(text: &str) -> Vec<(Pad, Option<String>)> {
    let mut out = Vec::new();
    for block in text.split("\n\n").take(256) {
        let (mut vendor, mut product, mut name, mut stick, mut node) = (None, None, String::new(), false, None);
        for line in block.lines() {
            if line.starts_with("I:") {
                for word in line.split_whitespace() {
                    if let Some(v) = word.strip_prefix("Vendor=") {
                        vendor = u16::from_str_radix(v, 16).ok();
                    } else if let Some(v) = word.strip_prefix("Product=") {
                        product = u16::from_str_radix(v, 16).ok();
                    }
                }
            } else if let Some(n) = line.strip_prefix("N: Name=") {
                name = clean(n.trim().trim_matches('"'));
            } else if line.starts_with("H:") {
                let handlers = line.trim_start_matches("H:").trim_start().trim_start_matches("Handlers=");
                stick = handlers.split_whitespace().any(|w| w.len() > 2 && w.starts_with("js") && w[2..].chars().all(|c| c.is_ascii_digit()));
                node = handlers.split_whitespace().find(|w| w.strip_prefix("event").is_some_and(|n| !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()))).map(str::to_string);
            }
        }
        if let (true, Some(vendor), Some(product)) = (stick, vendor, product) {
            out.push((Pad { vendor, product, name }, node));
        }
    }
    out
}

pub fn plugged_in_in(text: &str) -> Vec<Pad> {
    let mut out: Vec<Pad> = Vec::new();
    for (pad, _) in listed_in(text) {
        if !out.contains(&pad) {
            out.push(pad);
        }
    }
    out
}

pub fn listed() -> Vec<(Pad, Option<String>)> {
    files::read_within(std::path::Path::new("/proc/bus/input/devices"), MOST_BYTES).map(|t| listed_in(&t)).unwrap_or_default()
}

pub fn plugged_in() -> Vec<Pad> {
    files::read_within(std::path::Path::new("/proc/bus/input/devices"), MOST_BYTES).map(|t| plugged_in_in(&t)).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOUTH: u16 = 0x130;
    const EAST: u16 = 0x131;
    const RANGES: [(u16, Range); 6] = [
        (0, Range { min: -32768, max: 32767 }), (1, Range { min: -32768, max: 32767 }), (2, Range { min: 0, max: 255 }),
        (3, Range { min: -32768, max: 32767 }), (4, Range { min: -32768, max: 32767 }), (5, Range { min: 0, max: 255 }),
    ];

    fn changer(text: &str) -> Changer {
        let p = Profiles::read(&format!("[054c:09cc Wireless Controller]\n{text}"));
        assert!(p.unread.is_empty(), "{:?}", p.unread);
        Changer::new(&p.pads[0].changes, &RANGES)
    }

    #[test]
    fn every_kind_of_line_reads_and_writes_the_same() {
        let text = "[054c:09cc Wireless Controller]\nSouth = East\nRight Stick Y = inverted\nSticks = swapped\n\
                    Left Stick dead zone = 10%\nLeft Trigger = Right Shoulder\nNorth = Right Trigger\nSelect = Escape\n\
                    Start = Ctrl+S\nHome = Nothing\n[045e:028e ]\nEast = South\n";
        let p = Profiles::read(text);
        assert!(p.unread.is_empty(), "{:?}", p.unread);
        assert_eq!(Profiles::read(&p.text()), p);
        assert_eq!(p.pads[0].changes.len(), 9);
        assert_eq!(Change::parse("a = b"), Ok(Change::Into { from: From::Button(SOUTH), to: To::Button(EAST) }));
        for lines in SWAPS {
            for l in *lines {
                assert!(Change::parse(l).is_ok(), "{l}");
            }
        }
    }

    #[test]
    fn what_cannot_be_read_says_why() {
        let p = Profiles::read("South = East\n[zz:09cc]\n[054c:09cc Pad]\nFlux = East\nSouth\nLeft Stick dead zone = 90%\nLeft Stick X = South\n");
        let whys: Vec<_> = p.unread.iter().map(|u| (u.0, u.2)).collect();
        assert_eq!(whys, [(1, "It is under no controller's heading."), (2, "Not a controller's heading."), (4, NOT_A_BUTTON),
                          (5, NO_EQUALS), (6, NOT_A_ZONE), (7, NOT_A_BUTTON)]);
        for line in ["[", "[]", "[:]", "[ffff:ffff \u{0}]", "= =", "Left Stick dead zone = -1", "South = ++"] {
            let _ = Profiles::read(&format!("[0001:0002]\n{line}\n"));
        }
        assert_eq!(Profiles::read(&"[0001:0002]\nSouth = East\n".repeat(400)).pads.len(), MOST_LINES / 2);
    }

    #[test]
    fn a_stick_pushed_makes_the_keys_it_is_changed_into_and_says_nothing_else() {
        let mut c = changer("Left Stick Up = Key W\nLeft Stick Down = Key S");
        assert_eq!(c.event(EV_ABS, 1, -30000), [Out::Key(17, true)], "W is key 17");
        assert_eq!(c.event(EV_ABS, 1, -29000), [], "still held, nothing new");
        assert_eq!(c.event(EV_ABS, 1, 0), [Out::Key(17, false)]);
        assert_eq!(c.event(EV_ABS, 1, 30000), [Out::Key(31, true)], "S is key 31");
        assert_eq!(c.event(EV_ABS, 1, -30000), [Out::Key(31, false), Out::Key(17, true)]);
        let mut one = changer("Left Stick Up = Key W");
        assert_eq!(one.event(EV_ABS, 1, -30000), [Out::Key(17, true), Out::Pad(EV_ABS, 1, -30000)]);
        assert_eq!(one.event(EV_ABS, 1, 30000), [Out::Key(17, false), Out::Pad(EV_ABS, 1, 30000)]);
    }

    #[test]
    fn a_dpad_hat_stands_for_the_dpad_buttons() {
        let mut c = changer("D-pad Left = Key A\nD-pad Right = Key D");
        assert_eq!(c.event(EV_ABS, 16, -1), [Out::Key(30, true)], "A is key 30");
        assert_eq!(c.event(EV_ABS, 16, 0), [Out::Key(30, false)]);
        assert_eq!(c.event(EV_ABS, 16, 1), [Out::Key(32, true)], "D is key 32");
        let p = Profiles::read("[0001:0002]\nLeft Stick Up = Key W\nRight Stick Left = Key A\n");
        assert_eq!(Profiles::read(&p.text()), p);
    }

    #[test]
    fn a_letter_that_is_a_buttons_name_too_is_written_with_key_before_it() {
        let p = Profiles::read("[0001:0002]\nSelect = A\nStart = Key A\nHome = Escape\nWest = key b\n");
        assert!(p.unread.is_empty(), "{:?}", p.unread);
        let lines: Vec<String> = p.pads[0].changes.iter().map(Change::line).collect();
        assert_eq!(lines, ["Select = South", "Start = Key A", "Home = Escape", "West = Key B"]);
        assert_eq!(Profiles::read(&p.text()), p, "what is written is read back the same");
    }

    #[test]
    fn a_profile_is_chosen_by_its_controllers_numbers_and_name() {
        let p = Profiles::read("[054c:09cc]\nSouth = East\n[054c:09cc Wireless Controller]\nSouth = Nothing\n");
        assert_eq!(p.in_force(0x054c, 0x09cc, "Wireless Controller", &[]).unwrap()[0].line(), "South = Nothing");
        assert_eq!(p.in_force(0x054c, 0x09cc, "Other", &[]).unwrap()[0].line(), "South = East");
        assert!(p.in_force(0x045e, 0x028e, "Xbox", &[]).is_none());
    }

    #[test]
    fn buttons_become_buttons_triggers_keys_or_nothing() {
        let mut c = changer("South = East\nNorth = Right Trigger\nSelect = Escape\nHome = Nothing\n");
        assert_eq!(c.event(EV_KEY, SOUTH, 1), [Out::Pad(EV_KEY, EAST, 1)]);
        assert_eq!(c.event(EV_KEY, SOUTH, 0), [Out::Pad(EV_KEY, EAST, 0)]);
        assert_eq!(c.event(EV_KEY, 0x133, 1), [Out::Pad(EV_ABS, 5, 255)]);
        assert_eq!(c.event(EV_KEY, 0x133, 0), [Out::Pad(EV_ABS, 5, 0)]);
        assert_eq!(c.event(EV_KEY, 0x13a, 1), [Out::Key(1, true)]);
        assert_eq!(c.event(EV_KEY, 0x13a, 0), [Out::Key(1, false)]);
        assert_eq!(c.event(EV_KEY, 0x13c, 1), []);
        assert_eq!(c.event(EV_KEY, EAST, 1), [Out::Pad(EV_KEY, EAST, 1)]);
        assert_eq!(c.event(0, 0, 0), [Out::Pad(0, 0, 0)]);
    }

    #[test]
    fn a_trigger_as_a_button_goes_down_once_half_pulled_and_up_once_let_back() {
        let mut c = changer("Left Trigger = Right Shoulder\n");
        assert_eq!(c.event(EV_ABS, 2, 40), []);
        assert_eq!(c.event(EV_ABS, 2, 130), [Out::Pad(EV_KEY, 0x137, 1)]);
        assert_eq!(c.event(EV_ABS, 2, 255), []);
        assert_eq!(c.event(EV_ABS, 2, 100), [Out::Pad(EV_KEY, 0x137, 0)]);
        assert_eq!(c.event(EV_ABS, 2, 0), []);
    }

    #[test]
    fn sticks_turn_swap_and_keep_still_inside_their_dead_zone() {
        let mut c = changer("Left Stick Y = inverted\nLeft Stick dead zone = 10%\n");
        assert_eq!(c.event(EV_ABS, 1, 32767), [Out::Pad(EV_ABS, 1, -32768)]);
        assert_eq!(c.event(EV_ABS, 1, -32768), [Out::Pad(EV_ABS, 1, 32767)]);
        assert_eq!(c.event(EV_ABS, 0, 3000), [Out::Pad(EV_ABS, 0, 0)], "inside the dead zone the stick was not still");
        assert_eq!(c.event(EV_ABS, 0, 4000), [Out::Pad(EV_ABS, 0, 4000)]);
        assert_eq!(c.event(EV_ABS, 3, 3000), [Out::Pad(EV_ABS, 3, 3000)], "the other stick's dead zone was changed");
        let mut c = changer("Sticks = swapped\n");
        assert_eq!(c.event(EV_ABS, 0, 5), [Out::Pad(EV_ABS, 3, 5)]);
        assert_eq!(c.event(EV_ABS, 4, 6), [Out::Pad(EV_ABS, 1, 6)]);
        assert_eq!(c.event(EV_ABS, 16, 1), [Out::Pad(EV_ABS, 16, 1)]);
    }

    #[test]
    fn swaps_are_put_in_and_taken_out_alone_and_the_rest_is_the_persons_own() {
        let pad = Pad { vendor: 0x054c, product: 0x09cc, name: "Wireless Controller".into() };
        let mut p = Profiles::read("[054c:09cc Wireless Controller]\nSelect = Escape\n");
        assert!(!p.has_swap(&pad, FIRST, 0));
        p.set_swap(&pad, FIRST, 0, true);
        assert!(p.has_swap(&pad, FIRST, 0) && !p.has_swap(&pad, FIRST, 1));
        assert_eq!(p.own(&pad, FIRST).len(), 1);
        p.set_swap(&pad, FIRST, 0, false);
        assert_eq!(p.pads[0].changes.len(), 1, "taking a swap away took another line");
        let mut q = Profiles::default();
        q.set_swap(&pad, "Racing", 1, true);
        q.set_swap(&pad, "Racing", 1, false);
        assert_eq!(q.names_of(&pad), ["Racing"], "a named profile left empty was dropped");
        assert_eq!(Profiles::read(&p.text()), p);
    }

    const GOD_OF_WAR: u32 = 1593500;
    const SANDS: u32 = 13600;

    #[test]
    fn a_file_from_before_profiles_had_names_reads_as_one_profile_still_in_force() {
        let p = Profiles::read("[054c:09cc Wireless Controller]\nSelect = Key A\n");
        assert!(p.unread.is_empty(), "{:?}", p.unread);
        assert_eq!(p.names_of(&p.pads[0].pad), [FIRST]);
        assert_eq!(p.chosen(None), Some(FIRST), "the old profile was no longer used outside Steam games");
        assert_eq!(p.in_force(0x054c, 0x09cc, "Wireless Controller", &[]).unwrap()[0].line(), "Select = Key A");
        assert!(p.in_force(0x054c, 0x09cc, "Wireless Controller", &[GOD_OF_WAR]).is_none(), "a Steam game with no choice is not Standard");
        assert_eq!(Profiles::read(&p.text()), p, "what is written is read back the same");
    }

    #[test]
    fn each_steam_game_uses_standard_unless_a_profile_is_chosen_for_it() {
        let text = "[054c:09cc Pad]\nProfile = Sands\nSelect = Key A\n[054c:09cc Pad]\nProfile = Racing\nSouth = East\n\
                    [Games]\nAny other game = Standard\n13600 = Sands\n";
        let mut p = Profiles::read(text);
        assert!(p.unread.is_empty(), "{:?}", p.unread);
        let force = |p: &Profiles, running: &[u32]| p.in_force(0x054c, 0x09cc, "Pad", running).map(|c| c[0].line());
        assert_eq!(force(&p, &[SANDS]).as_deref(), Some("Select = Key A"));
        assert_eq!(force(&p, &[GOD_OF_WAR]), None, "God of War did not get Standard");
        assert_eq!(force(&p, &[]), None);
        assert_eq!(force(&p, &[GOD_OF_WAR, SANDS]).as_deref(), Some("Select = Key A"), "the game with a choice wins");
        p.choose(Some(GOD_OF_WAR), Some("Racing".into()));
        assert_eq!(force(&p, &[GOD_OF_WAR]).as_deref(), Some("South = East"));
        assert_eq!(force(&p, &[SANDS]).as_deref(), Some("Select = Key A"));
        assert_eq!(Profiles::read(&p.text()), p);
        p.off = true;
        assert_eq!(force(&p, &[SANDS]), None, "the one switch left changes on");
        assert!(p.text().contains("Changes = off"));
        assert_eq!(Profiles::read(&p.text()), p);
        p.off = false;
        assert!(p.in_force(0x045e, 0x028e, "Xbox", &[SANDS]).is_none());
    }

    #[test]
    fn profiles_are_added_renamed_and_removed_and_the_games_follow() {
        let pad = Pad { vendor: 1, product: 2, name: "Pad".into() };
        let mut p = Profiles::default();
        assert_eq!(p.add(&pad, "Sands"), Ok("Sands".to_string()));
        assert!(p.add(&pad, "Sands").is_err(), "two profiles of one name");
        assert!(p.add(&pad, " standard ").is_err() && p.add(&pad, "[]").is_err());
        p.set_into(&pad, "Sands", From::Button(0x13a), To::Keys(vec![30]));
        p.choose(Some(SANDS), Some("Sands".into()));
        p.choose(None, Some("Sands".into()));
        assert_eq!(p.rename(&pad, "Sands", "Sands of Time"), Ok("Sands of Time".to_string()));
        assert_eq!((p.chosen(Some(SANDS)), p.chosen(None)), (Some("Sands of Time"), Some("Sands of Time")));
        assert!(p.rename(&pad, "Sands of Time", "Standard").is_err());
        p.remove(&pad, "Sands of Time");
        assert!(p.pads.is_empty());
        assert_eq!((p.chosen(Some(SANDS)), p.chosen(None)), (None, None), "a removed profile was still chosen");
    }

    #[test]
    fn game_lines_that_cannot_be_read_say_why_and_a_profile_called_standard_is_refused() {
        let p = Profiles::read("[0001:0002]\nProfile = Standard\n[Games]\nGod of War = Sands\nnothing\n");
        let lines: Vec<usize> = p.unread.iter().map(|u| u.0).collect();
        assert_eq!(lines, [2, 4, 5]);
    }

    #[test]
    fn a_running_steam_game_is_found_by_its_reaper() {
        let proc = std::env::temp_dir().join(format!("trindle-pads-proc-{}", std::process::id()));
        let put = |at: &str, what: &str| {
            std::fs::create_dir_all(proc.join(at)).unwrap();
            std::fs::write(proc.join(at).join("cmdline"), what).unwrap();
        };
        put("40", "reaper\0SteamLaunch\0AppId=1593500\0--\0game.exe\0");
        put("41", "vim\0AppId=77\0");
        put("42", "reaper\0SteamLaunch\0AppId=1593500\0");
        put("self", "reaper\0SteamLaunch\0AppId=5\0");
        assert_eq!(steam_games_now(&proc), [GOD_OF_WAR]);
        assert!(steam_games_now(&proc.join("none")).is_empty());
        let _ = std::fs::remove_dir_all(&proc);
    }

    #[test]
    fn controllers_are_found_by_the_kernels_list_and_only_those_with_a_stick_handler() {
        let text = "I: Bus=0003 Vendor=054c Product=09cc Version=8111\nN: Name=\"Wireless Controller\"\nH: Handlers=event5 js0 \n\n\
                    I: Bus=0011 Vendor=0001 Product=0001 Version=ab41\nN: Name=\"AT keyboard\"\nH: Handlers=sysrq kbd event3 leds \n\n\
                    I: Bus=0003 Vendor=045e Product=028e Version=0114\nN: Name=\"Xbox\"\nH: Handlers=event6 js1\n";
        let pads = plugged_in_in(text);
        assert_eq!(pads.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(), ["Wireless Controller", "Xbox"]);
        assert_eq!((pads[0].vendor, pads[0].product), (0x054c, 0x09cc));
    }

    #[test]
    fn the_device_number_is_found_whichever_handler_the_kernel_lists_first() {
        let text = "I: Bus=0003 Vendor=11c0 Product=5500 Version=0111\nN: Name=\"Pad\"\nH: Handlers=event2 js0 \n\n\
                    I: Bus=0003 Vendor=054c Product=09cc Version=8111\nN: Name=\"Other\"\nH: Handlers=js1 event7 \n";
        let nodes: Vec<Option<String>> = listed_in(text).into_iter().map(|(_, n)| n).collect();
        assert_eq!(nodes, [Some("event2".to_string()), Some("event7".to_string())]);
    }
}
