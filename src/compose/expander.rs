//! Typing expansion: the person's own letters go
//! into one short line held here only, and a plugin's trigger at its end is
//! swapped for the plugin's words, typed by the compositor's own keyboard.
//! The line is never written, logged or sent anywhere.

use std::path::Path;
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::time::{Duration, Instant};

use smithay::input::keyboard::xkb;

use crate::expand::{Expansion, LONGEST_TRIGGER, WORD};

const LOOK_EVERY: Duration = Duration::from_secs(1);

const FIRST_CODE: u32 = 9;
const LAST_CODE: u32 = 255;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Typed {
    Letter(char),
    Back,

    Other,

    Nothing,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Swap {
    pub remove: usize,
    pub words: String,
}

type Seen = Vec<(String, Option<(u64, std::time::SystemTime)>)>;

type Scan = (Seen, Option<Vec<Expansion>>);

struct Entry {
    trigger: Vec<char>,
    e: Expansion,
}

fn entries(list: Vec<Expansion>) -> Vec<Entry> {
    list.into_iter().map(|e| Entry { trigger: e.trigger.chars().collect(), e }).collect()
}

#[derive(Default)]
pub struct Expander {
    line: Vec<char>,
    list: Vec<Entry>,
    seen: Seen,
    looked: Option<Instant>,

    scanning: Option<Receiver<Scan>>,
}

impl Expander {
    pub fn clear(&mut self) {
        self.line.clear();
    }

    pub fn refresh(&mut self, folders: &Path) {
        if let Some(scanning) = &self.scanning {
            match scanning.try_recv() {
                Ok((seen, list)) => {
                    self.scanning = None;
                    if let Some(list) = list {
                        self.list = entries(list);
                    }
                    self.seen = seen;
                }
                Err(TryRecvError::Empty) => return,
                Err(TryRecvError::Disconnected) => self.scanning = None,
            }
        }
        if self.looked.is_some_and(|t| t.elapsed() < LOOK_EVERY) {
            return;
        }
        self.looked = Some(Instant::now());
        let (folders, before) = (folders.to_path_buf(), self.seen.clone());
        let (tx, rx) = channel();
        if std::thread::Builder::new().name("expander-scan".into()).spawn(move || {
            let _ = tx.send(scan(&folders, &before));
        }).is_ok() {
            self.scanning = Some(rx);
        }
    }

    #[cfg(test)]
    pub fn with(list: Vec<Expansion>) -> Expander {
        Expander { list: entries(list), looked: Some(Instant::now()), ..Expander::default() }
    }

    pub fn key(&mut self, typed: Typed) -> Option<Swap> {
        match typed {
            Typed::Nothing => None,
            Typed::Other => {
                self.line.clear();
                None
            }
            Typed::Back => {
                self.line.pop();
                None
            }
            Typed::Letter(c) => {
                self.line.push(c);

                if self.line.len() > LONGEST_TRIGGER + 1 {
                    self.line.remove(0);
                }
                let found = matched(&self.line, &self.list)?;
                self.line.clear();
                Some(found)
            }
        }
    }
}

impl std::fmt::Debug for Expander {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Expander")
    }
}

fn scan(folders: &Path, before: &Seen) -> Scan {
    let mut ids: Vec<String> = std::fs::read_dir(folders).into_iter().flatten().flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|id| crate::summon::good(id) && crate::summon::record_says(folders, id, &[WORD]))
        .collect();
    ids.sort();
    let now: Seen = ids.into_iter().map(|id| { let s = crate::expand::stamp(&folders.join(&id)); (id, s) }).collect();
    let list = (&now != before).then(|| now.iter().flat_map(|(id, _)| crate::expand::read(&folders.join(id))).collect());
    (now, list)
}

fn matched(line: &[char], list: &[Entry]) -> Option<Swap> {
    let mut best: Option<(usize, &Expansion)> = None;
    for Entry { trigger: t, e } in list {
        if t.len() > line.len() || best.is_some_and(|(n, _)| n >= t.len()) {
            continue;
        }
        let tail = &line[line.len() - t.len()..];
        let same = if e.case {
            tail.iter().flat_map(|c| c.to_lowercase()).eq(t.iter().flat_map(|c| c.to_lowercase()))
        } else {
            tail == t.as_slice()
        };
        let starts = line.len() == t.len() || !line[line.len() - t.len() - 1].is_alphanumeric();
        if same && (!e.word || starts) {
            best = Some((t.len(), e));
        }
    }
    let (n, e) = best?;
    let typed = &line[line.len() - n..];
    let words = if e.case { capitals_of(typed, &e.words) } else { e.words.clone() };
    Some(Swap { remove: n - 1, words })
}

fn capitals_of(typed: &[char], words: &str) -> String {
    let letters: Vec<char> = typed.iter().copied().filter(|c| c.is_alphabetic()).collect();
    if letters.len() > 1 && letters.iter().all(|c| c.is_uppercase()) {
        return words.to_uppercase();
    }
    if letters.first().is_some_and(|c| c.is_uppercase()) {
        let mut cs = words.chars();
        if let Some(f) = cs.next() {
            return f.to_uppercase().chain(cs).collect();
        }
    }
    words.to_string()
}

pub fn typed(sym: xkb::Keysym, ctrl_alt_super: bool) -> Typed {
    if sym.is_modifier_key() {
        return Typed::Nothing;
    }
    if ctrl_alt_super {
        return Typed::Other;
    }
    if sym == xkb::Keysym::BackSpace {
        return Typed::Back;
    }
    match sym.key_char() {
        Some(c) if !c.is_control() => Typed::Letter(c),
        _ => Typed::Other,
    }
}

pub fn keymaps(swap: &Swap) -> Vec<(xkb::Keymap, Vec<u32>)> {
    let mut out = Vec::new();
    let mut syms: Vec<xkb::Keysym> = Vec::new();
    let mut keys: Vec<u32> = Vec::new();
    let push = |sym: xkb::Keysym, syms: &mut Vec<xkb::Keysym>, keys: &mut Vec<u32>, out: &mut Vec<(xkb::Keymap, Vec<u32>)>| {
        let at = match syms.iter().position(|s| *s == sym) {
            Some(at) => at,
            None => {
                if syms.len() as u32 == LAST_CODE - FIRST_CODE + 1 {
                    if let Some(map) = keymap(syms) {
                        out.push((map, std::mem::take(keys)));
                    }
                    syms.clear();
                }
                syms.push(sym);
                syms.len() - 1
            }
        };
        keys.push(FIRST_CODE + at as u32);
    };
    for _ in 0..swap.remove {
        push(xkb::Keysym::BackSpace, &mut syms, &mut keys, &mut out);
    }
    for c in swap.words.chars() {
        push(xkb::utf32_to_keysym(c as u32), &mut syms, &mut keys, &mut out);
    }
    if !keys.is_empty() {
        if let Some(map) = keymap(&syms) {
            out.push((map, keys));
        }
    }
    out
}

fn keymap(syms: &[xkb::Keysym]) -> Option<xkb::Keymap> {
    let mut codes = String::new();
    let mut symbols = String::new();
    for (i, sym) in syms.iter().enumerate() {
        let code = FIRST_CODE + i as u32;
        codes.push_str(&format!("<T{code}> = {code}; "));
        symbols.push_str(&format!("key <T{code}> {{ [ {} ] }}; ", xkb::keysym_get_name(*sym)));
    }
    let text = format!(
        "xkb_keymap {{ xkb_keycodes \"trindle\" {{ minimum = 8; maximum = {LAST_CODE}; {codes}}}; \
         xkb_types \"trindle\" {{ include \"complete\" }}; \
         xkb_compatibility \"trindle\" {{ include \"complete\" }}; \
         xkb_symbols \"trindle\" {{ {symbols}}}; }};"
    );
    let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
    xkb::Keymap::new_from_string(&context, text, xkb::KEYMAP_FORMAT_TEXT_V1, xkb::KEYMAP_COMPILE_NO_FLAGS)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(trigger: &str, words: &str, word: bool, case: bool) -> Expansion {
        Expansion { trigger: trigger.into(), words: words.into(), word, case }
    }

    fn type_in(x: &mut Expander, text: &str) -> Option<Swap> {
        let mut last = None;
        for c in text.chars() {
            last = x.key(Typed::Letter(c));
        }
        last
    }

    #[test]
    fn a_trigger_at_the_end_of_the_line_is_swapped_and_the_line_emptied() {
        let mut x = Expander::with(vec![e("tn#", "Trindle", false, false)]);
        assert_eq!(type_in(&mut x, "hello tn#"), Some(Swap { remove: 2, words: "Trindle".into() }));
        assert!(x.line.is_empty(), "nothing typed is kept past a match");
        assert_eq!(type_in(&mut x, "tn"), None);
        x.key(Typed::Back);
        assert_eq!(type_in(&mut x, "n#"), Some(Swap { remove: 2, words: "Trindle".into() }), "Backspace takes a letter off");
        type_in(&mut x, "tn");
        x.key(Typed::Other);
        assert_eq!(type_in(&mut x, "#"), None, "a key that makes no letter empties the line");
        type_in(&mut x, "tn");
        x.key(Typed::Nothing);
        assert!(type_in(&mut x, "#").is_some(), "Shift going down changes nothing");
        type_in(&mut x, "tn");
        x.clear();
        assert_eq!(type_in(&mut x, "#"), None, "emptied on a change of window");
        type_in(&mut x, &"a".repeat(100));
        assert_eq!(x.line.len(), LONGEST_TRIGGER + 1, "the line is never longer than a trigger and a letter before it");
    }

    #[test]
    fn the_longest_trigger_wins_and_word_and_case_are_kept_to() {
        let mut x = Expander::with(vec![e("n#", "short", false, false), e("tn#", "long", false, false), e("tn#", "second", false, false)]);
        assert_eq!(type_in(&mut x, "tn#").unwrap().words, "long");
        assert_eq!(type_in(&mut x, "xn#").unwrap().words, "short");
        let mut x = Expander::with(vec![e("sig#", "Regards", true, false)]);
        assert_eq!(type_in(&mut x, "asig#"), None, "only at the start of a word");
        assert!(type_in(&mut x, " sig#").is_some() && type_in(&mut x, "(sig#").is_some() && type_in(&mut x, "sig#").is_some());
        let mut x = Expander::with(vec![e("tn#", "trindle desktop", false, true)]);
        assert_eq!(type_in(&mut x, "TN#").unwrap().words, "TRINDLE DESKTOP");
        assert_eq!(type_in(&mut x, "Tn#").unwrap().words, "Trindle desktop");
        assert_eq!(type_in(&mut x, "tn#").unwrap().words, "trindle desktop");
        let mut x = Expander::with(vec![e("tn#", "x", false, false)]);
        assert_eq!(type_in(&mut x, "TN#"), None, "without case, capitals must match");
    }

    #[test]
    fn keys_make_letters_only_with_nothing_but_shift_held() {
        assert_eq!(typed(xkb::Keysym::a, false), Typed::Letter('a'));
        assert_eq!(typed(xkb::Keysym::numbersign, false), Typed::Letter('#'));
        assert_eq!(typed(xkb::Keysym::a, true), Typed::Other);
        assert_eq!(typed(xkb::Keysym::BackSpace, false), Typed::Back);
        for other in [xkb::Keysym::Return, xkb::Keysym::Tab, xkb::Keysym::Escape, xkb::Keysym::Left, xkb::Keysym::dead_acute] {
            assert_eq!(typed(other, false), Typed::Other, "{other:?}");
        }
        assert_eq!(typed(xkb::Keysym::Shift_L, false), Typed::Nothing);
    }

    #[test]
    fn a_whole_word_trigger_of_full_length_does_not_fire_in_the_middle_of_a_word() {
        let long: String = "a".repeat(LONGEST_TRIGGER - 1) + ";";
        let mut x = Expander::with(vec![e(&long, "words", true, false)]);
        assert_eq!(type_in(&mut x, &format!("q{long}")), None, "a letter before it makes it the middle of a word");
        assert_eq!(x.line.len(), LONGEST_TRIGGER + 1);
        assert!(type_in(&mut x, &format!(" {long}")).is_some(), "after a space it is a word of its own");
        assert!(type_in(&mut x, &long).is_some(), "alone it is a word");
    }

    #[test]
    fn the_plugins_are_looked_at_off_the_key_thread() {
        let dir = std::env::temp_dir().join(format!("trd-ex-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let home = dir.join("org.example.snip/home");
        std::fs::create_dir_all(&home).unwrap();
        std::fs::write(dir.join("org.example.snip/allowed"), "expand\n").unwrap();
        std::fs::write(home.join(crate::expand::FILE), r#"{"version":1,"snippets":[{"trigger":"tn;","replace":"Trindle","word":false,"case":false}]}"#).unwrap();
        let mut x = Expander::default();
        x.refresh(&dir);
        assert!(x.scanning.is_some() && x.list.is_empty(), "the look was made on the key's own thread");
        let start = Instant::now();
        while x.list.is_empty() && start.elapsed() < Duration::from_secs(5) {
            std::thread::sleep(Duration::from_millis(20));
            x.refresh(&dir);
        }
        assert_eq!(type_in(&mut x, "tn;").map(|s| s.words), Some("Trindle".to_string()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_made_up_keymap_types_any_letter_and_backspace() {
        let swap = Swap { remove: 2, words: "Ünïcode € 漢字 ok".into() };
        let maps = keymaps(&swap);
        assert_eq!(maps.len(), 1);
        let (map, keys) = &maps[0];
        let state = xkb::State::new(map);
        let typed: Vec<xkb::Keysym> = keys.iter().map(|k| state.key_get_one_sym((*k).into())).collect();
        assert_eq!(&typed[..2], &[xkb::Keysym::BackSpace, xkb::Keysym::BackSpace]);
        let text: String = typed[2..].iter().filter_map(|s| s.key_char()).collect();
        assert_eq!(text, swap.words);

        let many: String = (0..600u32).filter_map(|i| char::from_u32(0x4E00 + i)).collect();
        let maps = keymaps(&Swap { remove: 0, words: many.clone() });
        assert!(maps.len() > 1);
        let back: String = maps.iter().flat_map(|(m, ks)| {
            let s = xkb::State::new(m);
            ks.iter().filter_map(move |k| s.key_get_one_sym((*k).into()).key_char()).collect::<Vec<_>>()
        }).collect();
        assert_eq!(back, many);
    }

    #[test]
    fn a_short_code_typed_by_hand_becomes_the_words_in_the_window() {
        use crate::rig::{alone, Rig};
        use crate::state::ClientState;
        alone("expander::tests::a_short_code_typed_by_hand_becomes_the_words_in_the_window", |dir| {
            const T: u32 = 20;
            const N: u32 = 49;
            const SPACE: u32 = 57;
            const A: u32 = 30;
            const Q: u32 = 16;
            const SEMICOLON: u32 = 39;
            const SHIFT: u32 = 42;
            let id = "org.example.snippets";
            let home = dir.join("home/.local/share/trindle/plugins").join(id).join("home");
            std::fs::create_dir_all(&home).unwrap();
            std::fs::write(home.join(crate::expand::FILE), r#"{"version":1,"snippets":[
                {"trigger":"tn;","replace":"Trindle \u20ac \u00fcber","word":false,"case":false},
                {"trigger":"qq;","replace":"made","word":false,"case":false}]}"#).unwrap();
            let mut rig = Rig::new(dir);
            let app = rig.join(ClientState::default());
            rig.window(app, false);
            rig.onto_the_window_in_front();
            let tn = |rig: &mut Rig| {
                rig.key_by_hand(T);
                rig.key_by_hand(N);
                rig.key_by_hand(SEMICOLON);
            };
            tn(&mut rig);
            assert_eq!(rig.typed(app), "tn;", "a plugin not allowed expand had its code swapped");
            rig.allow(id, &["expand"]);
            std::thread::sleep(Duration::from_millis(1100));

            rig.key_by_hand(SPACE);
            std::thread::sleep(Duration::from_millis(300));
            tn(&mut rig);
            assert_eq!(rig.typed(app), "tn; Trindle \u{20ac} \u{fc}ber", "the code was not swapped for the words");
            rig.key_by_hand(A);
            rig.key_by_hand_only(SHIFT, true);
            rig.key_by_hand(A);
            rig.key_by_hand_only(SHIFT, false);
            assert_eq!(rig.typed(app), "tn; Trindle \u{20ac} \u{fc}beraA", "the person's own keymap and Shift were not given back");

            for k in [SPACE, Q, Q, SEMICOLON] {
                rig.made_up_key(k);
            }
            assert!(rig.typed(app).ends_with(" qq;"), "a made-up key was watched: {}", rig.typed(app));

            rig.password_box(app);
            rig.key_by_hand(SPACE);
            tn(&mut rig);
            assert!(rig.typed(app).ends_with(" tn;"), "a password box was watched: {}", rig.typed(app));

            rig.allow(id, &[]);
            std::thread::sleep(Duration::from_millis(1100));
            let app2 = rig.join(ClientState::default());
            rig.window(app2, false);
            rig.onto_the_window_in_front();
            rig.key_by_hand(SPACE);
            std::thread::sleep(Duration::from_millis(300));
            tn(&mut rig);
            assert_eq!(rig.typed(app2), " tn;", "withdrawn, the code was still swapped");
        });
    }

    const T: u32 = 20;
    const N: u32 = 49;
    const SEMICOLON: u32 = 39;
    const SPACE: u32 = 57;
    const ALT: u32 = 56;
    const SUPER: u32 = 125;

    fn tn(rig: &mut crate::rig::Rig) {
        rig.key_by_hand(T);
        rig.key_by_hand(N);
        rig.key_by_hand(SEMICOLON);
    }

    fn ends(rig: &crate::rig::Rig, who: usize, with: &str) -> bool {
        rig.typed(who).ends_with(with)
    }

    fn held_modifiers_and_the_lock_screen_stop_the_watching(rig: &mut crate::rig::Rig, app: usize) {

        for held in [ALT, SUPER] {
            rig.key_by_hand_only(held, true);
            tn(rig);
            rig.key_by_hand_only(held, false);
            assert!(!ends(rig, app, "Trindle"), "a code typed with {held} held was swapped: {}", rig.typed(app));
        }

        rig.key_by_hand(SPACE);
        rig.key_by_hand(T);
        rig.key_by_hand(N);
        rig.state.lock_for_rig();
        rig.key_by_hand(SPACE);
        rig.state.unlock_for_rig();
        rig.onto_the_window_in_front();
        rig.key_by_hand(SEMICOLON);
        assert!(!ends(rig, app, "Trindle"), "what was typed before the lock was still remembered: {}", rig.typed(app));
    }

    fn questions_and_panels_that_hold_the_keyboard_stop_the_watching(rig: &mut crate::rig::Rig, app: usize) {
        use crate::state::ClientState;
        use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_shell_v1::Layer as On;
        use wayland_protocols_wlr::layer_shell::v1::client::zwlr_layer_surface_v1::KeyboardInteractivity;

        let root = rig.join(ClientState { from_root: true, ..ClientState::default() });
        let question = rig.panel_sized(root, "org.example.question", On::Overlay, 400, 200, KeyboardInteractivity::None);
        rig.key_by_hand(SPACE);
        tn(rig);
        assert!(!ends(rig, app, "Trindle"), "typing was watched under a question: {}", rig.typed(app));
        rig.put_away(&question);
        rig.key_by_hand(SPACE);
        tn(rig);
        assert!(ends(rig, app, "Trindle"), "typing was not watched again once the question went: {}", rig.typed(app));

        let other = rig.join(ClientState::default());
        rig.key_by_hand(SPACE);
        rig.key_by_hand(T);
        rig.key_by_hand(N);
        let panel = rig.panel_sized(other, "org.example.keys", On::Top, 200, 40, KeyboardInteractivity::Exclusive);
        let before = rig.typed(app).to_string();
        rig.key_by_hand(SEMICOLON);
        assert_eq!(rig.typed(app), before, "a panel holding the keyboard let a key through");
        rig.put_away(&panel);
        rig.crash(other);
        rig.key_by_hand(SEMICOLON);
        assert!(!ends(rig, app, "Trindle"), "what was typed before a panel took the keyboard was remembered: {}", rig.typed(app));
    }

    fn a_change_of_window_a_click_and_a_scroll_forget_what_was_typed(rig: &mut crate::rig::Rig) {
        use crate::state::ClientState;

        rig.key_by_hand(SPACE);
        rig.key_by_hand(T);
        rig.key_by_hand(N);
        let app2 = rig.join(ClientState::default());
        rig.window(app2, false);
        rig.onto_the_window_in_front();
        rig.key_by_hand(SEMICOLON);
        assert!(!rig.typed(app2).contains("Trindle"), "a code begun in one window was finished in another: {}", rig.typed(app2));
        rig.move_by_hand(0.5, 0.5);
        rig.key_by_hand(SPACE);
        rig.key_by_hand(T);
        rig.key_by_hand(N);
        rig.click_by_hand();
        rig.key_by_hand(SEMICOLON);
        assert!(!rig.typed(app2).contains("Trindle"), "a code begun before a click was finished after it: {}", rig.typed(app2));
        rig.key_by_hand(SPACE);
        rig.key_by_hand(T);
        rig.key_by_hand(N);
        rig.scroll_by_hand();
        rig.key_by_hand(SEMICOLON);
        assert!(!rig.typed(app2).contains("Trindle"), "a code begun before a scroll was finished after it: {}", rig.typed(app2));
        rig.key_by_hand(SPACE);
        tn(rig);
        assert!(rig.typed(app2).ends_with("Trindle"), "a code typed whole after all that was not swapped: {}", rig.typed(app2));
    }

    #[test]
    fn typing_is_watched_only_where_it_should_be() {
        use crate::rig::{alone, Rig};
        use crate::state::ClientState;
        alone("expander::tests::typing_is_watched_only_where_it_should_be", |dir| {
            let id = "org.example.snippets";
            let home = dir.join("home/.local/share/trindle/plugins").join(id).join("home");
            std::fs::create_dir_all(&home).unwrap();
            std::fs::write(home.join(crate::expand::FILE), r#"{"version":1,"snippets":[
                {"trigger":"tn;","replace":"Trindle","word":false,"case":false}]}"#).unwrap();
            let mut rig = Rig::new(dir);
            rig.allow(id, &["expand"]);
            let app = rig.join(ClientState::default());
            rig.window(app, false);
            rig.onto_the_window_in_front();
            std::thread::sleep(Duration::from_millis(1100));
            rig.key_by_hand(SPACE);
            std::thread::sleep(Duration::from_millis(300));
            tn(&mut rig);
            assert!(ends(&rig, app, "Trindle"), "the code was not swapped: {}", rig.typed(app));
            held_modifiers_and_the_lock_screen_stop_the_watching(&mut rig, app);
            questions_and_panels_that_hold_the_keyboard_stop_the_watching(&mut rig, app);
            a_change_of_window_a_click_and_a_scroll_forget_what_was_typed(&mut rig);
        });
    }
}
