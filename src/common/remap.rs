//! Keys and buttons, changed.

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use super::files;
use crate::toolkit::t;

const KEY_MAX: u32 = 0x2ff;
pub const BUTTONS: std::ops::RangeInclusive<u32> = 0x110..=0x117;
pub const TAP_MS: u32 = 200;
pub const MOST_IN_A_RUN: usize = 16;
const MOST_HELD: usize = 8;
pub const MOST_LINES: usize = 256;
pub const MOST_BYTES: u64 = 64 * 1024;
const LONGEST_NAME: usize = 64;
pub const LOGIN_FOLDER: &str = "/var/lib/trindle/login-remaps";
const LOGIN_FOLDER_ENV: &str = "TRINDLE_LOGIN_REMAPS";

const NAMES: &[(u32, &str, &[&str])] = &[
    (1, "Escape", &["Esc"]),
    (2, "1", &[]), (3, "2", &[]), (4, "3", &[]), (5, "4", &[]), (6, "5", &[]),
    (7, "6", &[]), (8, "7", &[]), (9, "8", &[]), (10, "9", &[]), (11, "0", &[]),
    (12, "Minus", &["-"]), (13, "Equals", &[]), (14, "Backspace", &[]), (15, "Tab", &[]),
    (16, "Q", &[]), (17, "W", &[]), (18, "E", &[]), (19, "R", &[]), (20, "T", &[]), (21, "Y", &[]),
    (22, "U", &[]), (23, "I", &[]), (24, "O", &[]), (25, "P", &[]),
    (26, "Left Bracket", &[]), (27, "Right Bracket", &[]), (28, "Enter", &["Return"]),
    (29, "Ctrl", &["Left Ctrl", "Control"]),
    (30, "A", &[]), (31, "S", &[]), (32, "D", &[]), (33, "F", &[]), (34, "G", &[]), (35, "H", &[]),
    (36, "J", &[]), (37, "K", &[]), (38, "L", &[]),
    (39, "Semicolon", &[";"]), (40, "Apostrophe", &["'"]), (41, "Grave", &["`"]),
    (42, "Shift", &["Left Shift"]), (43, "Backslash", &["\\"]),
    (44, "Z", &[]), (45, "X", &[]), (46, "C", &[]), (47, "V", &[]), (48, "B", &[]), (49, "N", &[]), (50, "M", &[]),
    (51, "Comma", &[]), (52, "Full Stop", &[".", "Period", "Dot"]), (53, "Slash", &["/"]),
    (54, "Right Shift", &[]), (55, "Keypad Star", &["Keypad *"]), (56, "Alt", &["Left Alt"]),
    (57, "Space", &[]), (58, "Caps Lock", &["Caps", "CapsLock"]),
    (59, "F1", &[]), (60, "F2", &[]), (61, "F3", &[]), (62, "F4", &[]), (63, "F5", &[]), (64, "F6", &[]),
    (65, "F7", &[]), (66, "F8", &[]), (67, "F9", &[]), (68, "F10", &[]),
    (69, "Num Lock", &["NumLock"]), (70, "Scroll Lock", &[]),
    (71, "Keypad 7", &[]), (72, "Keypad 8", &[]), (73, "Keypad 9", &[]), (74, "Keypad Minus", &["Keypad -"]),
    (75, "Keypad 4", &[]), (76, "Keypad 5", &[]), (77, "Keypad 6", &[]), (78, "Keypad Plus", &[]),
    (79, "Keypad 1", &[]), (80, "Keypad 2", &[]), (81, "Keypad 3", &[]), (82, "Keypad 0", &[]),
    (83, "Keypad Full Stop", &["Keypad ."]), (86, "Extra Backslash", &["102nd"]),
    (87, "F11", &[]), (88, "F12", &[]), (96, "Keypad Enter", &[]), (97, "Right Ctrl", &[]),
    (98, "Keypad Slash", &["Keypad /"]), (99, "Print Screen", &["Print", "PrtSc", "SysRq"]),
    (100, "Right Alt", &["AltGr", "Alt Gr"]),
    (102, "Home", &[]), (103, "Up", &["Up Arrow"]), (104, "Page Up", &["PgUp"]), (105, "Left", &["Left Arrow"]),
    (106, "Right", &["Right Arrow"]), (107, "End", &[]), (108, "Down", &["Down Arrow"]),
    (109, "Page Down", &["PgDn"]), (110, "Insert", &["Ins"]), (111, "Delete", &["Del"]),
    (113, "Mute", &[]), (114, "Volume Down", &[]), (115, "Volume Up", &[]), (116, "Power", &[]),
    (117, "Keypad Equals", &[]), (119, "Pause", &["Break"]),
    (125, "Super", &["Left Super", "Windows", "Win", "Logo", "Command", "Cmd"]),
    (126, "Right Super", &["Right Windows", "Right Command"]), (127, "Menu", &["Compose", "Context Menu"]),
    (128, "Stop", &[]), (131, "Undo", &[]), (133, "Copy", &[]), (135, "Paste", &[]),
    (136, "Find", &[]), (137, "Cut", &[]), (138, "Help", &[]), (140, "Calculator", &[]), (142, "Sleep", &[]),
    (155, "Mail", &[]), (158, "Back", &[]), (159, "Forward", &[]),
    (163, "Next Track", &[]), (164, "Play Pause", &["Play"]), (165, "Previous Track", &[]),
    (166, "Stop Playing", &[]), (172, "Home Page", &[]),
    (183, "F13", &[]), (184, "F14", &[]), (185, "F15", &[]), (186, "F16", &[]), (187, "F17", &[]),
    (188, "F18", &[]), (189, "F19", &[]), (190, "F20", &[]), (191, "F21", &[]), (192, "F22", &[]),
    (193, "F23", &[]), (194, "F24", &[]),
    (224, "Brightness Down", &[]), (225, "Brightness Up", &[]),
    (0x110, "Left Click", &["Left Button", "Click"]), (0x111, "Right Click", &["Right Button"]),
    (0x112, "Middle Click", &["Middle Button", "Wheel Button"]),
    (0x113, "Back Button", &["Side Button"]), (0x114, "Forward Button", &["Extra Button"]),
    (0x115, "Button 6", &[]), (0x116, "Button 7", &[]), (0x117, "Button 8", &[]),
];

pub const NOT_A_KEY: &str = "Not a key Trindle knows. Try Escape, Ctrl+C or Left Click.";
pub const TOO_LONG: &str = "That is more keys than one change can hold.";
pub const NO_EQUALS: &str = "A change needs what is pressed, an equals sign, and what it becomes.";
pub const TAPPED_AND_HELD: &str = "Say what it is tapped and what it is held, such as Escape tapped, Ctrl held.";

pub fn name(code: u32) -> String {
    NAMES.iter().find(|n| n.0 == code).map(|n| n.1.to_string()).unwrap_or_else(|| format!("Code {code}"))
}

pub fn named() -> impl Iterator<Item = (u32, &'static str)> {
    NAMES.iter().map(|n| (n.0, n.1))
}

pub fn code(word: &str) -> Option<u32> {
    let word = word.trim();
    if word.is_empty() {
        return None;
    }
    if let Some(n) = word.strip_prefix("Code ").or_else(|| word.strip_prefix("code ")) {
        return n.trim().parse().ok().filter(|c| (1..=KEY_MAX).contains(c));
    }
    NAMES.iter().find(|n| n.1.eq_ignore_ascii_case(word) || n.2.iter().any(|a| a.eq_ignore_ascii_case(word))).map(|n| n.0)
}

pub fn is_button(code: u32) -> bool {
    BUTTONS.contains(&code)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum To {
    Nothing,
    Held(Vec<u32>),
    Run(Vec<Vec<u32>>),
    TapHold { tap: Vec<u32>, hold: Vec<u32> },
}

fn combo_words(codes: &[u32]) -> String {
    codes.iter().map(|c| name(*c)).collect::<Vec<_>>().join("+")
}

fn parse_combo(words: &str) -> Result<Vec<u32>, &'static str> {
    let words = words.trim();
    if let Some(one) = code(words) {
        return Ok(vec![one]);
    }
    let codes: Option<Vec<u32>> = words.split('+').map(code).collect();
    let codes = codes.ok_or(NOT_A_KEY)?;
    if codes.is_empty() {
        return Err(NOT_A_KEY);
    }
    if codes.len() > MOST_HELD {
        return Err(TOO_LONG);
    }
    Ok(codes)
}

impl To {
    pub fn words(&self) -> String {
        match self {
            To::Nothing => "Nothing".into(),
            To::Held(codes) => combo_words(codes),
            To::Run(run) => run.iter().map(|c| combo_words(c)).collect::<Vec<_>>().join(", "),
            To::TapHold { tap, hold } => format!("{} tapped, {} held", combo_words(tap), combo_words(hold)),
        }
    }

    pub fn parse(words: &str) -> Result<To, &'static str> {
        let words = words.trim();
        if words.eq_ignore_ascii_case("nothing") {
            return Ok(To::Nothing);
        }
        let parts: Vec<&str> = words.split(',').map(str::trim).collect();
        let tapped = parts.iter().any(|p| ends_with_word(p, "tapped"));
        let held = parts.iter().any(|p| ends_with_word(p, "held"));
        if tapped || held {
            let [a, b] = parts.as_slice() else { return Err(TAPPED_AND_HELD) };
            let (tap, hold) = match (strip_word(a, "tapped"), strip_word(b, "held"), strip_word(a, "held"), strip_word(b, "tapped")) {
                (Some(t), Some(h), _, _) | (_, _, Some(h), Some(t)) => (t, h),
                _ => return Err(TAPPED_AND_HELD),
            };
            return Ok(To::TapHold { tap: parse_combo(tap)?, hold: parse_combo(hold)? });
        }
        if parts.len() > MOST_IN_A_RUN {
            return Err(TOO_LONG);
        }
        let mut run = parts.iter().map(|p| parse_combo(p)).collect::<Result<Vec<_>, _>>()?;
        Ok(if run.len() == 1 { To::Held(run.remove(0)) } else { To::Run(run) })
    }
}

fn ends_with_word(text: &str, word: &str) -> bool {
    strip_word(text, word).is_some()
}

fn strip_word<'a>(text: &'a str, word: &str) -> Option<&'a str> {
    let at = text.len().checked_sub(word.len())?;
    let (head, tail) = (text.get(..at)?, text.get(at..)?);
    (tail.eq_ignore_ascii_case(word) && head.ends_with(' ')).then(|| head.trim_end())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Change {
    pub swap: Option<String>,
    pub from: u32,
    pub device: Option<String>,
    pub to: To,
}

pub fn clean_name(name: &str) -> String {
    name.chars().filter(|c| !c.is_control() && *c != '"' && *c != '=').take(LONGEST_NAME).collect::<String>().trim().to_string()
}

impl Change {
    pub fn line(&self) -> String {
        let swap = self.swap.as_ref().map(|s| format!("[{s}] ")).unwrap_or_default();
        let device = self.device.as_ref().map(|d| format!(" on \"{d}\"")).unwrap_or_default();
        format!("{swap}{}{device} = {}", name(self.from), self.to.words())
    }

    pub fn parse(line: &str) -> Result<Change, &'static str> {
        let mut rest = line.trim();
        let mut swap = None;
        if let Some(after) = rest.strip_prefix('[') {
            let (mark, after) = after.split_once(']').ok_or(NO_EQUALS)?;
            swap = Some(mark.trim().to_string()).filter(|m| !m.is_empty());
            rest = after.trim_start();
        }
        let (left, right) = rest.split_once(" = ").or_else(|| rest.split_once('=')).ok_or(NO_EQUALS)?;
        let (key, device) = match left.split_once(" on \"") {
            Some((key, device)) => (key, Some(clean_name(device.trim_end().trim_end_matches('"')))),
            None => (left, None),
        };
        let from = code(key).ok_or(NOT_A_KEY)?;
        Ok(Change { swap, from, device: device.filter(|d| !d.is_empty()), to: To::parse(right)? })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Table {
    pub changes: Vec<Change>,
    pub unread: Vec<(usize, String, &'static str)>,
}

impl Table {
    pub fn read(text: &str) -> Table {
        let mut table = Table::default();
        for (i, line) in text.lines().enumerate().take(MOST_LINES) {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            match Change::parse(line) {
                Ok(change) => table.changes.push(change),
                Err(why) => table.unread.push((i + 1, line.to_string(), why)),
            }
        }
        table
    }

    pub fn text(&self) -> String {
        self.changes.iter().fold(String::new(), |mut out, c| {
            let _ = writeln!(out, "{}", c.line());
            out
        })
    }

    pub fn find(&self, device: &str, from: u32) -> Option<&Change> {
        let rank = |c: &Change| match (&c.device, &c.swap) {
            (Some(_), None) => 0,
            (Some(_), Some(_)) => 1,
            (None, None) => 2,
            (None, Some(_)) => 3,
        };
        self.changes.iter()
            .filter(|c| c.from == from && c.device.as_deref().is_none_or(|d| d == device))
            .min_by_key(|c| rank(c))
    }

    pub fn own_over(&self, from: u32, device: Option<&str>) -> bool {
        self.changes.iter().any(|c| c.swap.is_none() && c.from == from && c.device.as_deref() == device)
    }

    pub fn has_swap(&self, mark: &str) -> bool {
        self.changes.iter().any(|c| c.swap.as_deref() == Some(mark))
    }

    pub fn set_swap(&mut self, mark: &str, on: bool) {
        self.changes.retain(|c| c.swap.as_deref() != Some(mark));
        if on {
            if let Some(swap) = SWAPS.iter().find(|s| s.mark == mark) {
                self.changes.extend(swap.changes());
            }
        }
    }
}

pub struct Swap {
    pub mark: &'static str,
    pub lines: &'static [&'static str],
}

impl Swap {
    pub fn words(&self) -> &'static str {
        match self.mark {
            "ctrl-caps" => t!("settings-keyboard-swap-ctrl-caps"),
            "caps-tap" => t!("settings-keyboard-swap-caps-tap"),
            "esc-caps" => t!("settings-keyboard-swap-esc-caps"),
            "alt-super" => t!("settings-keyboard-swap-alt-super"),
            "side-copy" => t!("settings-keyboard-swap-side-copy"),
            _ => "",
        }
    }

    pub fn changes(&self) -> Vec<Change> {
        self.lines.iter().filter_map(|l| Change::parse(l).ok()).map(|c| Change { swap: Some(self.mark.into()), ..c }).collect()
    }
}

pub const SWAPS: &[Swap] = &[
    Swap { mark: "ctrl-caps", lines: &["Caps Lock = Ctrl", "Ctrl = Caps Lock"] },
    Swap { mark: "caps-tap", lines: &["Caps Lock = Escape tapped, Ctrl held"] },
    Swap { mark: "esc-caps", lines: &["Caps Lock = Escape", "Escape = Caps Lock"] },
    Swap { mark: "alt-super", lines: &["Alt = Super", "Super = Alt", "Right Alt = Right Super", "Right Super = Right Alt"] },
    Swap { mark: "side-copy", lines: &["Back Button = Ctrl+C", "Forward Button = Ctrl+V"] },
];

pub fn file() -> PathBuf {
    files::trindle_config().join("remap.conf")
}

pub fn load() -> Result<Table, std::io::Error> {
    files::read_within(&file(), MOST_BYTES).map(|text| Table::read(&text))
}

pub fn save(table: &Table) -> std::io::Result<()> {
    let text = table.text();
    files::write_whole(&file(), text.as_bytes())?;
    remember_for_login(&text);
    Ok(())
}

pub fn devices_file() -> PathBuf {
    files::scratch().join("trindle-input-devices")
}

pub fn read_devices() -> Vec<(bool, String)> {
    let text = files::read_within(&devices_file(), MOST_BYTES).unwrap_or_default();
    text.lines().filter_map(|l| l.split_once('\t')).map(|(made, name)| (made == "1", clean_name(name))).filter(|d| !d.1.is_empty()).collect()
}

fn login_folder() -> PathBuf {
    std::env::var_os(LOGIN_FOLDER_ENV).filter(|f| !f.is_empty()).map(PathBuf::from).unwrap_or_else(|| PathBuf::from(LOGIN_FOLDER))
}

pub fn remember_for_login(text: &str) {
    if let Err(e) = super::screen_scale::write_login_file(&login_folder(), text, MOST_BYTES) {
        eprintln!("trindle: could not keep the changed keys for the login screen: {e}");
    }
}

pub fn for_login() -> Table {
    for_login_in(&login_folder())
}

fn for_login_in(folder: &Path) -> Table {
    super::screen_scale::login_files(folder, MOST_BYTES).into_iter().next().map(|(_, text)| Table::read(&text)).unwrap_or_default()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out {
    Itself,
    Down(u32),
    Up(u32),
}

#[derive(Debug)]
struct Held {
    device: String,
    code: u32,
    itself: bool,
    out: Vec<u32>,
}

#[derive(Debug)]
struct Waiting {
    device: String,
    code: u32,
    since: u32,
    tap: Vec<u32>,
    hold: Vec<u32>,
}

#[derive(Debug, Default)]
pub struct Changer {
    table: Table,
    held: Vec<Held>,
    waiting: Option<Waiting>,
    gone: Vec<(String, u32)>,
    real_ctrl: Vec<u32>,
    real_alt: Vec<u32>,
    pub guarded: bool,
}

const GUARDED: [u32; 7] = [1, 14, 15, 28, 42, 54, 96];

const CTRLS: [u32; 2] = [29, 97];
const ALTS: [u32; 2] = [56, 100];
fn console_key(code: u32) -> bool {
    (59..=68).contains(&code) || code == 87 || code == 88
}

fn down_all(codes: &[u32]) -> Vec<Out> {
    codes.iter().map(|c| Out::Down(*c)).collect()
}

fn up_all(codes: &[u32]) -> Vec<Out> {
    codes.iter().rev().map(|c| Out::Up(*c)).collect()
}

impl Changer {
    pub fn new(table: Table) -> Changer {
        Changer { table, ..Changer::default() }
    }

    pub fn table(&self) -> &Table {
        &self.table
    }

    pub fn idle(&mut self) -> bool {
        let idle = self.table.changes.is_empty() && self.gone.is_empty() && self.waiting.is_none()
            && self.held.iter().all(|h| h.itself);
        if idle {
            self.held.clear();
        }
        idle
    }

    pub fn set_table(&mut self, table: Table) -> Vec<Out> {
        let mut outs = Vec::new();
        for held in std::mem::take(&mut self.held) {
            if held.itself {
                self.held.push(held);
            } else {
                outs.extend(up_all(&held.out));
                self.gone.push((held.device, held.code));
            }
        }
        if let Some(waiting) = self.waiting.take() {
            self.gone.push((waiting.device, waiting.code));
        }
        self.table = table;
        outs
    }

    pub fn other(&mut self) -> Vec<Out> {
        match self.waiting.take() {
            Some(w) => {
                let outs = down_all(&w.hold);
                self.held.push(Held { device: w.device, code: w.code, itself: false, out: w.hold });
                outs
            }
            None => Vec::new(),
        }
    }

    pub fn press(&mut self, device: &str, code: u32, down: bool, time: u32) -> Vec<Out> {
        let real = |list: &mut Vec<u32>, of: &[u32]| {
            if of.contains(&code) {
                list.retain(|c| *c != code);
                if down {
                    list.push(code);
                }
            }
        };
        real(&mut self.real_ctrl, &CTRLS);
        real(&mut self.real_alt, &ALTS);
        if down {
            self.went_down(device, code, time)
        } else {
            self.went_up(device, code, time)
        }
    }

    fn went_down(&mut self, device: &str, code: u32, time: u32) -> Vec<Out> {
        let mut outs = self.other();
        let console = console_key(code) && !self.real_ctrl.is_empty() && !self.real_alt.is_empty();
        let change = if console || (self.guarded && GUARDED.contains(&code)) { None } else { self.table.find(device, code).cloned() };
        let change = change.filter(|c| !(self.guarded && c.to == To::Nothing));
        let held = |itself, out| Held { device: device.to_string(), code, itself, out };
        match change.map(|c| c.to) {
            None => {
                outs.push(Out::Itself);
                self.held.push(held(true, Vec::new()));
            }
            Some(To::Nothing) => self.held.push(held(false, Vec::new())),
            Some(To::Held(codes)) => {
                outs.extend(down_all(&codes));
                self.held.push(held(false, codes));
            }
            Some(To::Run(run)) => {
                for combo in &run {
                    outs.extend(down_all(combo));
                    outs.extend(up_all(combo));
                }
                self.held.push(held(false, Vec::new()));
            }
            Some(To::TapHold { tap, hold }) => {
                self.waiting = Some(Waiting { device: device.to_string(), code, since: time, tap, hold });
            }
        }
        outs
    }

    fn went_up(&mut self, device: &str, code: u32, time: u32) -> Vec<Out> {
        if self.waiting.as_ref().is_some_and(|w| w.device == device && w.code == code) {
            let w = self.waiting.take().expect("just looked at");
            if time.wrapping_sub(w.since) <= TAP_MS {
                let mut outs = down_all(&w.tap);
                outs.extend(up_all(&w.tap));
                return outs;
            }
            return Vec::new();
        }
        if let Some(at) = self.gone.iter().position(|g| g.0 == device && g.1 == code) {
            self.gone.remove(at);
            return Vec::new();
        }
        match self.held.iter().position(|h| h.device == device && h.code == code) {
            Some(at) => {
                let held = self.held.remove(at);
                if held.itself { vec![Out::Itself] } else { up_all(&held.out) }
            }
            None => vec![Out::Itself],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CAPS: u32 = 58;
    const ESC: u32 = 1;
    const CTRL: u32 = 29;
    const ALT: u32 = 56;
    const A: u32 = 30;
    const B: u32 = 48;
    const C: u32 = 46;
    const V: u32 = 47;
    const LEFT: u32 = 0x110;
    const BACK: u32 = 0x113;

    fn table(lines: &str) -> Table {
        let t = Table::read(lines);
        assert!(t.unread.is_empty(), "{:?}", t.unread);
        t
    }

    fn changer(lines: &str) -> Changer {
        Changer::new(table(lines))
    }

    #[test]
    fn under_the_lock_screen_no_table_can_trap_a_password() {
        let mut c = changer("Enter = A\nBackspace = Nothing\nB = Nothing\nTab = Escape\nCaps Lock = Ctrl\nLeft Shift = Z\n");
        c.guarded = true;
        for key in GUARDED {
            assert_eq!(c.press("kb", key, true, 0), [Out::Itself], "key {key} was changed under the lock");
            assert_eq!(c.press("kb", key, false, 1), [Out::Itself]);
        }
        assert_eq!(c.press("kb", B, true, 0), [Out::Itself], "a key was turned into nothing under the lock");
        assert_eq!(c.press("kb", B, false, 1), [Out::Itself]);
        assert_eq!(c.press("kb", CAPS, true, 0), [Out::Down(CTRL)]);
        assert_eq!(c.press("kb", CAPS, false, 1), [Out::Up(CTRL)]);
        c.guarded = false;
        assert_eq!(c.press("kb", B, true, 5), []);
        c.guarded = true;
        assert_eq!(c.press("kb", B, false, 6), []);
    }

    #[test]
    fn every_name_reads_back_as_its_own_key() {
        for (code, name, others) in NAMES {
            assert_eq!(super::code(name), Some(*code), "{name}");
            assert_eq!(super::code(&name.to_lowercase()), Some(*code), "{name}");
            for other in *others {
                assert_eq!(super::code(other), Some(*code), "{other}");
            }
            for word in others.iter().chain([name]) {
                assert!(!word.contains(['=', ',', '+', '[', ']', '"']) && word.trim() == *word && !word.is_empty(), "{word:?}");
            }
            assert_eq!(super::name(*code), *name);
        }
        let mut codes: Vec<u32> = NAMES.iter().map(|n| n.0).collect();
        codes.sort();
        codes.dedup();
        assert_eq!(codes.len(), NAMES.len(), "a key is named twice");
        assert_eq!(name(240), "Code 240");
        assert_eq!(code("Code 240"), Some(240));
        assert_eq!(code("Code 0"), None);
        assert_eq!(code("Code 99999"), None);
        assert_eq!(code("Shift Lock"), None);
    }

    #[test]
    fn every_kind_of_change_reads_and_writes_the_same() {
        for line in [
            "Caps Lock = Escape",
            "Caps Lock = Ctrl+C",
            "A = Left Click",
            "Back Button = Ctrl+Shift+T",
            "F1 = H, E, L, L, O",
            "Insert = Nothing",
            "Caps Lock = Escape tapped, Ctrl held",
            "Caps Lock on \"Keychron K2\" = Escape",
            "[ctrl-caps] Caps Lock = Ctrl",
            "Code 240 = Code 241",
        ] {
            let change = Change::parse(line).unwrap_or_else(|e| panic!("{line}: {e}"));
            assert_eq!(change.line(), line);
        }
        let held = Change::parse("caps = ctrl held, esc tapped").unwrap();
        assert_eq!(held.to, To::TapHold { tap: vec![ESC], hold: vec![CTRL] });
        assert_eq!(Change::parse("Keypad Star = Keypad Plus").unwrap().to, To::Held(vec![78]));
    }

    #[test]
    fn what_cannot_be_read_says_why_and_the_rest_is_kept() {
        assert_eq!(Change::parse("Caps Lock"), Err(NO_EQUALS));
        assert_eq!(Change::parse("Shift Lock = A"), Err(NOT_A_KEY));
        assert_eq!(Change::parse("A = Hyper"), Err(NOT_A_KEY));
        assert_eq!(Change::parse("A = Escape tapped"), Err(TAPPED_AND_HELD));
        assert_eq!(Change::parse("A = Escape tapped, Ctrl tapped"), Err(TAPPED_AND_HELD));
        assert_eq!(Change::parse("A = A+B+C+D+E+F+G+H+I"), Err(TOO_LONG));
        let long = vec!["A"; MOST_IN_A_RUN + 1].join(", ");
        assert_eq!(Change::parse(&format!("B = {long}")), Err(TOO_LONG));
        assert_eq!(Change::parse("[] A = B").unwrap().swap, None);
        let t = Table::read("# mine\nA = B\nnonsense\n\nC = Nothing\nA = \n");
        assert_eq!(t.changes.len(), 2);
        assert_eq!(t.unread.iter().map(|u| u.0).collect::<Vec<_>>(), vec![3, 6]);
    }

    #[test]
    fn a_hostile_file_is_read_within_its_limits() {
        let c = Change::parse("A on \"Evil\u{7}\"\"Name that is far far far far far far far far far far far far far far far long\" = B").unwrap();
        let d = c.device.unwrap();
        assert!(d.len() <= LONGEST_NAME && !d.contains('"') && !d.chars().any(char::is_control), "{d}");
        let text = "A = B\n".repeat(MOST_LINES * 4);
        assert_eq!(Table::read(&text).changes.len(), MOST_LINES);
        for line in ["[", "]", "[x", "= =", " on \" = A", "A on \"", "A = ,", "A = +", "A = ++", "A = tapped, held",
                     "A = é tapped, ü held", "\u{0} = \u{0}"] {
            let _ = Change::parse(line);
        }
    }

    #[test]
    fn a_change_for_one_device_wins_and_the_persons_own_over_a_swap() {
        let t = table("[s] A = C\nA = B\nA on \"Pad\" = V\n[s] A on \"Pad\" = Escape\n");
        assert_eq!(t.find("Keyboard", A).unwrap().to, To::Held(vec![B]));
        assert_eq!(t.find("Pad", A).unwrap().to, To::Held(vec![V]));
        let swaps_only = table("[s] A = C\n[s] A on \"Pad\" = Escape\n");
        assert_eq!(swaps_only.find("Pad", A).unwrap().to, To::Held(vec![ESC]));
        assert_eq!(swaps_only.find("Other", A).unwrap().to, To::Held(vec![C]));
        assert!(t.find("Keyboard", B).is_none());
        assert!(t.own_over(A, None) && t.own_over(A, Some("Pad")) && !swaps_only.own_over(A, None));
    }

    #[test]
    fn a_swap_switched_off_takes_away_exactly_its_own_lines() {
        let mut t = table("A = B\n");
        for swap in SWAPS {
            assert_eq!(swap.changes().len(), swap.lines.len(), "{} has a line that cannot be read", swap.mark);
            t.set_swap(swap.mark, true);
            assert!(t.has_swap(swap.mark));
        }
        for swap in SWAPS {
            t.set_swap(swap.mark, false);
            assert!(!t.has_swap(swap.mark));
        }
        assert_eq!(t, table("A = B\n"));
        t.set_swap("ctrl-caps", true);
        assert_eq!(Table::read(&t.text()), t);
    }

    #[test]
    fn a_key_without_a_change_is_passed_on_as_it_came() {
        let mut c = changer("Caps Lock = Escape\n");
        assert_eq!(c.press("k", A, true, 0), vec![Out::Itself]);
        assert_eq!(c.press("k", A, false, 10), vec![Out::Itself]);
        assert_eq!(c.press("k", B, false, 20), vec![Out::Itself]);
    }

    #[test]
    fn a_key_becomes_another_key_a_click_or_keys_held_together() {
        let mut c = changer("Caps Lock = Escape\nA = Left Button\nBack Button = Ctrl+C\nLeft Button = Right Button\n");
        assert_eq!(c.press("k", CAPS, true, 0), vec![Out::Down(ESC)]);
        assert_eq!(c.press("k", CAPS, false, 5), vec![Out::Up(ESC)]);
        assert_eq!(c.press("k", A, true, 10), vec![Out::Down(LEFT)]);
        assert_eq!(c.press("k", A, false, 15), vec![Out::Up(LEFT)]);
        assert_eq!(c.press("m", BACK, true, 20), vec![Out::Down(CTRL), Out::Down(C)]);
        assert_eq!(c.press("m", BACK, false, 25), vec![Out::Up(C), Out::Up(CTRL)]);
        assert_eq!(c.press("m", LEFT, true, 30), vec![Out::Down(0x111)]);
        assert_eq!(c.press("m", LEFT, false, 35), vec![Out::Up(0x111)]);
    }

    #[test]
    fn a_run_is_pressed_and_let_go_in_turn_and_nothing_is_nothing() {
        let mut c = changer("F1 = Ctrl+C, V\nInsert = Nothing\n");
        assert_eq!(c.press("k", 59, true, 0), vec![Out::Down(CTRL), Out::Down(C), Out::Up(C), Out::Up(CTRL), Out::Down(V), Out::Up(V)]);
        assert_eq!(c.press("k", 59, false, 5), vec![]);
        assert_eq!(c.press("k", 110, true, 10), vec![]);
        assert_eq!(c.press("k", 110, false, 15), vec![]);
    }

    #[test]
    fn tapped_is_one_thing_and_held_another() {
        let lines = "Caps Lock = Escape tapped, Ctrl held\n";
        let mut c = changer(lines);
        assert_eq!(c.press("k", CAPS, true, 1000), vec![]);
        assert_eq!(c.press("k", CAPS, false, 1000 + TAP_MS), vec![Out::Down(ESC), Out::Up(ESC)]);
        assert_eq!(c.press("k", CAPS, true, 2000), vec![]);
        assert_eq!(c.press("k", A, true, 2010), vec![Out::Down(CTRL), Out::Itself]);
        assert_eq!(c.press("k", A, false, 2020), vec![Out::Itself]);
        assert_eq!(c.press("k", CAPS, false, 2030), vec![Out::Up(CTRL)]);
        assert_eq!(c.press("k", CAPS, true, 3000), vec![]);
        assert_eq!(c.press("m", LEFT, true, 3010), vec![Out::Down(CTRL), Out::Itself]);
        assert_eq!(c.press("m", LEFT, false, 3020), vec![Out::Itself]);
        assert_eq!(c.press("k", CAPS, false, 3030), vec![Out::Up(CTRL)]);
        assert_eq!(c.press("k", CAPS, true, 4000), vec![]);
        assert_eq!(c.other(), vec![Out::Down(CTRL)]);
        assert_eq!(c.other(), vec![]);
        assert_eq!(c.press("k", CAPS, false, 4030), vec![Out::Up(CTRL)]);
        assert_eq!(c.press("k", CAPS, true, 5000), vec![]);
        assert_eq!(c.press("k", CAPS, false, 5000 + TAP_MS + 1), vec![]);
        assert_eq!(c.press("k", CAPS, true, u32::MAX - 10), vec![]);
        assert_eq!(c.press("k", CAPS, false, 20), vec![Out::Down(ESC), Out::Up(ESC)]);
    }

    #[test]
    fn a_change_for_one_device_leaves_the_others() {
        let mut c = changer("A on \"Pad\" = B\n");
        assert_eq!(c.press("Pad", A, true, 0), vec![Out::Down(B)]);
        assert_eq!(c.press("Keyboard", A, true, 1), vec![Out::Itself]);
        assert_eq!(c.press("Keyboard", A, false, 2), vec![Out::Itself]);
        assert_eq!(c.press("Pad", A, false, 3), vec![Out::Up(B)]);
    }

    #[test]
    fn the_way_to_a_text_console_is_never_changed_away() {
        let mut c = changer("F2 = Nothing\nCtrl = A\nAlt = B\n");
        assert_eq!(c.press("k", CTRL, true, 0), vec![Out::Down(A)]);
        assert_eq!(c.press("k", ALT, true, 1), vec![Out::Down(B)]);
        assert_eq!(c.press("k", 60, true, 2), vec![Out::Itself]);
        assert_eq!(c.press("k", 60, false, 3), vec![Out::Itself]);
        c.press("k", ALT, false, 4);
        assert_eq!(c.press("k", 60, true, 5), vec![]);
    }

    #[test]
    fn a_new_table_lets_go_of_what_the_old_one_held() {
        let mut c = changer("A = Ctrl+B\nCaps Lock = Escape tapped, Ctrl held\n");
        assert_eq!(c.press("k", A, true, 0), vec![Out::Down(CTRL), Out::Down(B)]);
        assert_eq!(c.press("k", C, true, 1), vec![Out::Itself]);
        assert_eq!(c.press("k", CAPS, true, 2), vec![]);
        assert_eq!(c.set_table(table("A = V\n")), vec![Out::Up(B), Out::Up(CTRL)]);
        assert_eq!(c.press("k", A, false, 3), vec![]);
        assert_eq!(c.press("k", CAPS, false, 4), vec![]);
        assert_eq!(c.press("k", C, false, 5), vec![Out::Itself]);
        assert_eq!(c.press("k", A, true, 6), vec![Out::Down(V)]);
    }

    #[test]
    fn the_login_screen_takes_the_newest_file_it_trusts() {
        let d = std::env::temp_dir().join(format!("trindle-remaps-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        assert_eq!(for_login_in(&d), Table::default());
        super::super::screen_scale::write_login_file(&d, "Caps Lock = Escape\n", MOST_BYTES).unwrap();
        assert_eq!(for_login_in(&d).changes.len(), 1);
        std::fs::write(d.join("not-a-number"), "A = B\n").unwrap();
        std::os::unix::fs::symlink(d.join("not-a-number"), d.join("4000")).unwrap();
        assert_eq!(for_login_in(&d).changes[0].from, CAPS);
        let pipe = std::ffi::CString::new(d.join("5000").to_str().unwrap()).unwrap();
        // SAFETY: makes a pipe at a path of the test's own.
        unsafe { libc::mkfifo(pipe.as_ptr(), 0o644) };
        assert_eq!(for_login_in(&d).changes[0].from, CAPS);
        let _ = std::fs::remove_dir_all(&d);
    }
}
