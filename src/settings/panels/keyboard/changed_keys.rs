//! Keyboard.

use super::{KEYS_PAUSE, Keyboard, PICK_WAIT, not_told, nothing_pressed};
use crate::common::remap::{self, Change as KeyChange, Table, To, SWAPS};
use crate::plugins::shortcuts::{self};
use crate::settings::ui::{dropdown, group, line, on, Kit, Msg, not_saved};
use crate::toolkit::t;
use crate::toolkit::widget::button;
use crate::toolkit::widget::{entry, switch, Length, Node};

impl Keyboard {
    pub(super) fn load_keys(&mut self) {
        match remap::load() {
            Ok(table) => {
                self.table = table;
                self.table_unread = None;
            }
            Err(e) => {
                self.table = Table::default();
                self.table_unread = Some(e.to_string());
            }
        }
        self.typed.clear();
        self.devices = remap::read_devices();
        self.allowed = crate::common::own_helpers::allowed_devices();
    }

    fn keep_keys(&mut self, kit: &mut Kit, key: &str) {
        if kit.settle(key, remap::save(&self.table), not_saved()).is_none() {
            self.load_keys();
        }
    }

    pub(super) fn swap(&mut self, kit: &mut Kit, mark: &str, on: bool) {
        self.table.set_swap(mark, on);
        self.typed.clear();
        self.keep_keys(kit, &swap_key(mark));
    }

    fn change_typed(&mut self, kit: &mut Kit, i: usize, text: String) {
        self.typed.insert(i, text);
        self.typed_round = self.typed_round.wrapping_add(1);
        let mine = self.typed_round;
        kit.after::<Keyboard>(KEYS_PAUSE, move |me, k| {
            if me.typed_round != mine {
                return;
            }
            let Some(text) = me.typed.get(&i).cloned() else { return };
            match To::parse(&text) {
                Err(why) => k.say(&change_key(i), why, false, true),
                Ok(to) => {
                    if let Some(change) = me.table.changes.get_mut(i) {
                        change.to = to;
                        me.typed.remove(&i);
                        me.keep_keys(k, &change_key(i));
                    }
                }
            }
        });
    }

    fn change_device(&mut self, kit: &mut Kit, i: usize, device: Option<String>) {
        if let Some(change) = self.table.changes.get_mut(i) {
            change.device = device;
            self.keep_keys(kit, &change_key(i));
        }
    }

    fn take_away(&mut self, kit: &mut Kit, i: usize) {
        if i < self.table.changes.len() {
            self.table.changes.remove(i);
            self.typed.clear();
            self.keep_keys(kit, KEYS_ADD);
        }
    }

    fn wait_for_press(&mut self, kit: &mut Kit) {
        self.picking = true;
        self.pick_round = self.pick_round.wrapping_add(1);
        let mine = self.pick_round;
        kit.say(KEYS_ADD, press_now(), true, true);
        kit.after::<Keyboard>(PICK_WAIT, move |me, k| {
            if me.picking && me.pick_round == mine {
                me.picking = false;
                k.say(KEYS_ADD, nothing_pressed(), true, true);
            }
        });
    }

    pub(super) fn picked(&mut self, kit: &mut Kit, code: Option<u32>) {
        self.picking = false;
        let Some(code) = code else {
            kit.say(KEYS_ADD, not_told(), true, true);
            return;
        };
        if let Some(i) = self.table.changes.iter().position(|c| c.swap.is_none() && c.from == code && c.device.is_none()) {
            kit.say(KEYS_ADD, &t!("settings-keyboard-already", key = remap::name(code)), true, true);
            kit.say(&change_key(i), type_it(), true, true);
            return;
        }
        self.table.changes.push(KeyChange { swap: None, from: code, device: None, to: To::Held(vec![code]) });
        let i = self.table.changes.len() - 1;
        self.keep_keys(kit, KEYS_ADD);
        kit.say(&change_key(i), type_it(), true, true);
    }

    fn allow_made(&mut self, kit: &mut Kit, name: String, on: bool) {
        if !crate::toolkit::window::persons_own(crate::common::person_only::INPUT_MADE) {
            kit.say(&made_key(&name), only_yours(), true, true);
            return;
        }
        let own = true;
        if kit.settle(&made_key(&name), crate::common::own_helpers_set::allow_device(&name, on, own), not_saved()).is_some() {
            self.allowed = crate::common::own_helpers::allowed_devices();
        }
    }

    pub(super) fn keys_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows = Vec::new();
        for swap in SWAPS {
            let mark = swap.mark;
            let on_now = self.table.has_swap(mark);
            let over = on_now && swap.changes().iter().any(|c| self.table.own_over(c.from, c.device.as_deref()));
            let sub = if over { t!("settings-keyboard-swap-over") } else { "" };
            rows.push(line(swap.words()).sub(sub).key(swap_key(mark))
                .control(switch(on_now, swap.words(), move |b| on::<Keyboard>(move |me, k| me.swap(k, mark, b))))
                .node(kit));
        }
        let mut names: Vec<String> = vec![every_device().to_string()];
        names.extend(self.devices.iter().map(|d| d.1.clone()).filter(|n| !n.is_empty()));
        for (i, change) in self.table.changes.iter().enumerate().filter(|(_, c)| c.swap.is_none()) {
            let mut devices = names.clone();
            if let Some(d) = &change.device {
                if !devices.contains(d) {
                    devices.push(d.clone());
                }
            }
            let at = Some(change.device.as_ref().and_then(|d| devices.iter().position(|n| n == d)).unwrap_or(0));
            let chosen = devices.clone();
            let title = remap::name(change.from);
            let typed = self.typed.get(&i).cloned().unwrap_or_else(|| change.to.words());
            let device = dropdown(kit, &format!("keys-device:{i}"), devices, at, &t!("settings-keyboard-which-device", key = &title), move |n| {
                let device = (n > 0).then(|| chosen[n].clone());
                on::<Keyboard>(move |me, k| me.change_device(k, i, device.clone()))
            });
            let becomes = entry(typed, t!("settings-keyboard-becomes", key = &title), move |text| {
                on::<Keyboard>(move |me, k| me.change_typed(k, i, text.clone()))
            }).width(Length::Fixed(220.0));
            let gone = button("user-trash-symbolic", t!("settings-keyboard-take-away")).on_press(on::<Keyboard>(move |me, k| me.take_away(k, i)));
            rows.push(line(title).sub(change.device.clone().unwrap_or_else(|| t!("settings-keyboard-every-device-about").into()))
                .key(change_key(i)).control(becomes).control(device).control(gone).node(kit));
        }
        for (n, text, why) in &self.table.unread {
            rows.push(line(text.clone()).sub(t!("settings-keyboard-line-unused", n = *n, why = *why)).node(kit));
        }
        if let Some(why) = &self.table_unread {
            rows.push(line(t!("settings-keyboard-keys-unread")).sub(t!("settings-keyboard-keys-unread-about", why = why)).key(KEYS_FILE).node(kit));
        }
        let all: Vec<(u32, &str)> = remap::named().collect();
        let words: Vec<String> = all.iter().map(|(_, n)| n.to_string()).collect();
        let codes: Vec<u32> = all.iter().map(|(c, _)| *c).collect();
        let list = dropdown(kit, "keys-list", words, None, t!("settings-keyboard-keys-list"), move |n| {
            let code = codes.get(n).copied();
            on::<Keyboard>(move |me, k| me.picked(k, code))
        });
        let press = button("list-add-symbolic", t!("settings-keyboard-keys-press")).on_press(on::<Keyboard>(|me, k| me.wait_for_press(k)));
        let sub = if self.picking { press_now() } else { t!("settings-keyboard-keys-add-about") };
        rows.push(line(t!("settings-keyboard-keys-add")).sub(sub).key(KEYS_ADD).control(press).control(list).node(kit));
        group(t!("settings-keyboard-keys-title")).icon("input-keyboard-symbolic")
            .about(t!("settings-keyboard-keys-about"))
            .rows(rows)
            .node(kit)
    }

    pub(super) fn made_card(&self, kit: &Kit) -> Node<Msg> {
        let made: Vec<&String> = self.devices.iter().filter(|d| d.0).map(|d| &d.1).collect();
        let mut rows: Vec<Node<Msg>> = made.iter().map(|name| {
            let allowed = self.allowed.contains(name);
            let sub = if allowed { t!("settings-keyboard-made-yes") } else { t!("settings-keyboard-made-no") };
            let named = (*name).clone();
            line((*name).clone()).sub(sub).key(made_key(name))
                .control(switch(allowed, t!("settings-keyboard-made-switch", name = *name), move |b| {
                    let named = named.clone();
                    on::<Keyboard>(move |me, k| me.allow_made(k, named.clone(), b))
                }))
                .node(kit)
        }).collect();
        if rows.is_empty() {
            rows.push(line(t!("settings-keyboard-made-none")).sub(t!("settings-keyboard-made-none-about")).node(kit));
        }
        group(t!("settings-keyboard-made-title")).icon("input-mouse-symbolic")
            .about(t!("settings-keyboard-made-about"))
            .rows(rows)
            .node(kit)
    }

    pub(super) fn plugin_keys_card(&self, kit: &Kit) -> Node<Msg> {
        group(t!("settings-keyboard-plugin-title")).icon("input-keyboard-symbolic").rows(self.plugin_keys.iter().map(|b| {
            let key = format!("pluginkeys:{}:{}", b.plugin, b.id);
            let (plugin, id) = (b.plugin.clone(), b.id.clone());
            let sub = match &b.refused {
                Some(why) => t!("settings-keyboard-plugin-refused", plugin = &b.plugin_name, why = why),
                None => t!("settings-keyboard-plugin-about", plugin = &b.plugin_name),
            };
            line(b.label.clone()).sub(sub).key(key)
                .control(entry(b.shown(), t!("settings-keyboard-plugin-keys-name", label = &b.label), move |text| {
                    let (plugin, id) = (plugin.clone(), id.clone());
                    on::<Keyboard>(move |me, k| me.keys_typed(k, &plugin, &id, text.clone()))
                }).width(Length::Fixed(200.0)))
                .node(kit)
        }).collect()).node(kit)
    }

    fn keys_typed(&mut self, kit: &mut Kit, plugin: &str, id: &str, text: String) {
        if let Some(b) = self.plugin_keys.iter_mut().find(|b| b.plugin == plugin && b.id == id) {
            b.typed = Some(text);
        }
        self.keys_round = self.keys_round.wrapping_add(1);
        let (mine, plugin, id) = (self.keys_round, plugin.to_string(), id.to_string());
        kit.after::<Keyboard>(KEYS_PAUSE, move |me, k| {
            if me.keys_round != mine {
                return;
            }
            let text = me.plugin_keys.iter().find(|b| b.plugin == plugin && b.id == id)
                .and_then(|b| b.typed.clone()).unwrap_or_default();
            let said = shortcuts::choose(&plugin, &id, &text).err();
            k.done(&format!("pluginkeys:{plugin}:{id}"), said.as_deref(), t!("settings-keyboard-plugin-saved"));
        });
    }

}

const KEYS_ADD: &str = "keys-add";
const KEYS_FILE: &str = "keys-file";
fn swap_key(mark: &str) -> String {
    format!("keys-swap:{mark}")
}
fn change_key(i: usize) -> String {
    format!("keys:{i}")
}

fn made_key(name: &str) -> String {
    format!("keys-made:{name}")
}

fn only_yours() -> &'static str {
    t!("settings-keyboard-only-yours")
}
fn press_now() -> &'static str {
    t!("settings-keyboard-press-now")
}

fn type_it() -> &'static str {
    t!("settings-keyboard-type-it")
}

fn every_device() -> &'static str {
    t!("settings-keyboard-every-device")
}
