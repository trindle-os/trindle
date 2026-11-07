//! Keyboard.

use std::collections::HashMap;

use super::{CAPS_KEY, Keyboard, clicker_on, keyboard_button_on};
use crate::common::pointer::{self, Pointer};
use crate::plugins::shortcuts::{self};
use crate::settings::session_env;
use crate::settings::ui::{dropdown, group, line, on, Kit, Msg};
use crate::toolkit::t;
use crate::toolkit::widget::Node;

impl Keyboard {
    pub(super) fn load(&mut self) {
        if !self.read {
            let (layouts, variants) = read_rules(RULES);
            let mut all: Vec<(String, String)> = layouts.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            all.sort_by_key(|(_, v)| v.to_lowercase());
            self.names = layouts;
            self.variants = variants;
            self.all = all;
            self.read = true;
        }
        self.chosen = Chosen::load();
        self.pointer = Pointer::load();
        self.touchpad = pointer::has_touchpad();
        self.button = keyboard_button_on();
        self.clicker = clicker_on();
        self.usb = crate::usbretry::read_log(std::path::Path::new(crate::usbretry::LOG));
        self.plugin_keys = shortcuts::bindings();
        self.load_keys();
    }

    pub(super) fn tell(&self) -> String {
        let c = &self.chosen;
        let name = |layout: &str, variant: &str| {
            self.variants.get(layout).and_then(|v| v.iter().find(|(n, _)| n == variant)).map(|(_, d)| d.clone())
                .or_else(|| self.names.get(layout).cloned())
                .unwrap_or_else(|| layout.to_uppercase())
        };
        let mut words = t!("settings-keyboard-top-typing", layout = name(&c.first, &c.first_var));
        if !c.second.is_empty() {
            words.push(' ');
            words.push_str(&t!("settings-keyboard-top-second", layout = name(&c.second, &c.second_var)));
        }
        let acts = match c.caps.as_str() {
            "caps:none" => Some(t!("settings-keyboard-top-caps-nothing").to_string()),
            "ctrl:nocaps" => Some(t!("settings-keyboard-top-caps-acts", key = t!("settings-keyboard-caps-as-ctrl"))),
            "caps:escape" => Some(t!("settings-keyboard-top-caps-acts", key = t!("settings-keyboard-caps-as-escape"))),
            "caps:backspace" => Some(t!("settings-keyboard-top-caps-acts", key = t!("settings-keyboard-caps-as-backspace"))),
            _ => None,
        };
        if let Some(acts) = acts {
            words.push(' ');
            words.push_str(&acts);
        }
        let mut changed: Vec<(u32, Option<&str>)> = self.table.changes.iter().map(|c| (c.from, c.device.as_deref())).collect();
        changed.sort();
        changed.dedup();
        if !changed.is_empty() {
            words.push(' ');
            words.push_str(&t!("settings-keyboard-top-changed", n = changed.len()));
        }
        words
    }

    fn set(&mut self, kit: &mut Kit, what: Change) {
        let c = &mut self.chosen;
        let key = match what {
            Change::First(v) => {
                c.first = v;
                c.first_var.clear();
                FIRST
            }
            Change::FirstVar(v) => {
                c.first_var = v;
                VARIANT
            }
            Change::Second(v) => {
                c.second = v;
                c.second_var.clear();
                SECOND
            }
            Change::Caps(v) => {
                c.caps = v;
                CAPS_KEY
            }
        };
        let mut layouts = vec![c.first.as_str()];
        let mut variants = vec![c.first_var.as_str()];
        if !c.second.is_empty() {
            layouts.push(&c.second);
            variants.push(&c.second_var);
        }
        let mut options = Vec::new();
        if !c.second.is_empty() {
            options.push(SWITCH);
        }
        if !c.caps.is_empty() {
            options.push(&c.caps);
        }
        let layout = layouts.join(",");
        let variant = variants.iter().any(|v| !v.is_empty()).then(|| variants.join(","));
        let option = (!options.is_empty()).then(|| options.join(","));
        let kept = session_env::update(
            &[
                ("XKB_DEFAULT_LAYOUT", Some(&layout)),
                ("XKB_DEFAULT_VARIANT", variant.as_deref()),
                ("XKB_DEFAULT_OPTIONS", option.as_deref()),
            ],
            kit,
            key,
            t!("settings-keyboard-in-use"),
        );
        if !kept {
            self.chosen = Chosen::load();
        }
    }

    pub(super) fn pick(kit: &Kit, key: &str, options: &[(String, String)], chosen: &str, tip: &str,
            change: fn(String) -> Change) -> Node<Msg> {
        let words: Vec<String> = options.iter().map(|o| o.1.clone()).collect();
        let values: Vec<String> = options.iter().map(|o| o.0.clone()).collect();
        let at = values.iter().position(|v| v == chosen);
        dropdown(kit, key, words, at, tip, move |i| {
            let v = values[i].clone();
            on::<Keyboard>(move |me, k| me.set(k, change(v.clone())))
        })
    }

    pub(super) fn keyboard_cards(&self, kit: &Kit) -> Vec<Node<Msg>> {
        let c = &self.chosen;
        let mut variants = vec![(String::new(), t!("settings-keyboard-standard").to_string())];
        variants.extend(self.variants.get(&c.first).cloned().unwrap_or_default());
        let mut seconds = vec![(String::new(), t!("settings-keyboard-none").to_string())];
        seconds.extend(self.all.iter().cloned());
        vec![group(t!("settings-keyboard-layout-title")).icon("input-keyboard-symbolic")
            .about(t!("settings-keyboard-layout-about"))
            .rows(vec![
                line(t!("settings-keyboard-layout")).key(FIRST).control(Self::pick(kit, FIRST, &self.all, &c.first, t!("settings-keyboard-layout"), Change::First)).node(kit),
                line(t!("settings-keyboard-variant")).key(VARIANT).control(Self::pick(kit, VARIANT, &variants, &c.first_var, t!("settings-keyboard-variant"), Change::FirstVar)).node(kit),
                line(t!("settings-keyboard-switch-to")).sub(t!("settings-keyboard-switch-to-about")).key(SECOND)
                    .control(Self::pick(kit, SECOND, &seconds, &c.second, t!("settings-keyboard-switch-to"), Change::Second)).node(kit),
            ])
            .node(kit)]
    }

}

const RULES: &str = "/usr/share/X11/xkb/rules/evdev.lst";

pub(super) fn caps() -> [(&'static str, &'static str); 5] {
    [
        ("", t!("settings-keyboard-caps-key")),
        ("ctrl:nocaps", t!("settings-keyboard-caps-ctrl")),
        ("caps:escape", t!("settings-keyboard-caps-escape")),
        ("caps:backspace", t!("settings-keyboard-caps-backspace")),
        ("caps:none", t!("settings-keyboard-caps-nothing")),
    ]
}
const SWITCH: &str = "grp:alt_shift_toggle";

const FIRST: &str = "keyboard-first";
const VARIANT: &str = "keyboard-variant";
const SECOND: &str = "keyboard-second";

pub(super) type Variants = HashMap<String, Vec<(String, String)>>;

fn read_rules(path: &str) -> (HashMap<String, String>, Variants) {
    let mut layouts = HashMap::new();
    let mut variants: Variants = HashMap::new();
    let mut section = String::new();
    let text = std::fs::read_to_string(path).unwrap_or_default();
    for line in text.lines() {
        if let Some(name) = line.strip_prefix("! ") {
            section = name.trim().to_string();
            continue;
        }
        let Some((name, desc)) = line.trim().split_once(char::is_whitespace) else { continue };
        let desc = desc.trim_start();
        if desc.is_empty() {
            continue;
        }
        if section == "layout" {
            layouts.insert(name.to_string(), desc.to_string());
        } else if section == "variant" {
            if let Some((layout, d)) = desc.split_once(':') {
                variants.entry(layout.to_string()).or_default().push((name.to_string(), d.trim().to_string()));
            }
        }
    }
    (layouts, variants)
}

#[derive(Default)]
pub(super) struct Chosen {
    pub(super) first: String,
    pub(super) first_var: String,
    pub(super) second: String,
    pub(super) second_var: String,
    pub(super) caps: String,
}

impl Chosen {
    pub(super) fn load() -> Chosen {
        let installed = crate::common::files::read_env(std::path::Path::new("/etc/default/keyboard"));
        let get = |k: &str| {
            let from_install = k.strip_prefix("XKB_DEFAULT_").map(|rest| format!("XKB{rest}"));
            session_env::get(k).filter(|v| !v.is_empty())
                .or_else(|| std::env::var(k).ok().filter(|v| !v.is_empty()))
                .or_else(|| from_install.and_then(|f| installed.get(&f).map(|v| v.trim_matches('"').to_string())))
                .unwrap_or_default()
        };
        let layout = get("XKB_DEFAULT_LAYOUT");
        let layout = if layout.is_empty() { "us".to_string() } else { layout };
        let chosen: Vec<String> = layout.split(',').map(str::to_string).collect();
        let chosen_var: Vec<String> = get("XKB_DEFAULT_VARIANT").split(',').map(str::to_string).collect();
        let options = get("XKB_DEFAULT_OPTIONS");
        Chosen {
            first: chosen[0].clone(),
            first_var: chosen_var.first().cloned().unwrap_or_default(),
            second: chosen.get(1).cloned().unwrap_or_default(),
            second_var: chosen_var.get(1).cloned().unwrap_or_default(),
            caps: options
                .split(',')
                .find(|o| o.starts_with("ctrl:nocaps") || o.starts_with("caps:"))
                .unwrap_or("")
                .to_string(),
        }
    }
}

pub(super) enum Change {
    First(String),
    FirstVar(String),
    Second(String),
    Caps(String),
}
