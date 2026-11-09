//! Keyboard: the keyboard button on the ring, the helper that clicks, and the keyboards and mice made by programs.

use super::{Keyboard, CAPS_KEY, CLICKER, said};
use super::layout::{caps, Change};
use crate::common::files::set_setting;
use crate::common::pointer::{self};
use crate::settings::ui::{group, line, on, Kit, Msg, not_saved};
use crate::toolkit::t;
use crate::toolkit::widget::{switch, Node};

impl Keyboard {
    pub(super) fn switch_line(kit: &Kit, key: &'static str, on_now: bool, title: &str, sub: &str) -> Node<Msg> {
        line(title).sub(sub).key(key)
            .control(switch(on_now, title, move |b| {
                on::<Keyboard>(move |me, k| {
                    me.flag(key, b);

                    if !apply(k, key, if b { "yes" } else { "no" }) {
                        me.flag(key, !b);
                    }
                })
            }))
            .node(kit)
    }

    fn flag(&mut self, key: &str, b: bool) {
        let p = &mut self.pointer;
        match key {
            pointer::MOUSE_NATURAL => p.mouse_natural = b,
            pointer::LEFT_HANDED => p.left_handed = b,
            pointer::TAP => p.tap = b,
            pointer::TOUCHPAD_NATURAL => p.touchpad_natural = b,
            pointer::QUIET_TYPING => p.quiet_typing = b,
            _ => {}
        }
    }

    pub(super) fn usb_card(&self, kit: &Kit) -> Node<Msg> {
        let rows = self.usb.iter().map(|e| line(e.words()).node(kit)).collect();
        group(t!("settings-keyboard-usb-title")).icon("input-keyboard-symbolic")
            .about(t!("settings-keyboard-usb-about"))
            .rows(rows)
            .node(kit)
    }

    pub(super) fn keyboard_button(&mut self, kit: &mut Kit, b: bool) {
        if !crate::toolkit::window::persons_own(crate::common::person_only::KEYBOARD) {
            return;
        }
        if kit.settle(BUTTON, set_keyboard_button(b, true), not_saved()).is_some() {
            self.button = b;
        }
    }

    pub(super) fn clicking_helper(&mut self, kit: &mut Kit, b: bool) {
        if !crate::toolkit::window::persons_own(crate::common::person_only::CLICKER) {
            return;
        }
        if kit.settle(CLICKER, set_clicker(b, true), not_saved()).is_some() {
            self.clicker = b;
        }
    }

    pub(super) fn ring_card(&self, kit: &Kit) -> Node<Msg> {
        let c = &self.chosen;
        let caps: Vec<(String, String)> = caps().iter().map(|(v, l)| (v.to_string(), l.to_string())).collect();
        group(t!("settings-keyboard-other-title")).icon("input-keyboard-symbolic")
            .about(t!("settings-keyboard-other-about"))
            .rows(vec![
                line(t!("settings-keyboard-caps-acts-as")).sub(t!("settings-keyboard-caps-acts-as-about")).key(CAPS_KEY)
                    .control(Self::pick(kit, CAPS_KEY, &caps, &c.caps, t!("settings-keyboard-caps-acts-as"), Change::Caps)).node(kit),
                line(t!("settings-keyboard-ring-button")).sub(t!("settings-keyboard-ring-button-about")).key(BUTTON)
                    .control(switch(self.button, t!("settings-keyboard-ring-button"), |b| on::<Keyboard>(move |me, k| me.keyboard_button(k, b))))
                    .node(kit),
            ])
            .node(kit)
    }

}

const BUTTON: &str = "keyboard-button";

fn set_keyboard_button(on: bool, own: bool) -> std::io::Result<()> {
    use crate::common::own_helpers::Helper;
    use crate::common::own_helpers_set;
    let mark = |on: bool| own_helpers_set::switch(Helper::Keyboard, on, own);

    mark(on)?;
    if let Err(e) = set_setting("keyboard_button", if on { "on" } else { "off" }) {
        return Err(match (on, mark(false)) {
            (true, Err(undo)) => std::io::Error::new(e.kind(), t!("settings-keyboard-button-and-mark", error = e.to_string(), undo = undo.to_string())),
            _ => e,
        });
    }
    Ok(())
}

fn set_clicker(on: bool, own: bool) -> std::io::Result<()> {
    use crate::common::own_helpers::Helper;
    use crate::common::own_helpers_set;
    own_helpers_set::switch(Helper::Clicker, on, own)
}

fn apply(kit: &mut Kit, key: &str, value: &str) -> bool {
    said(kit, key, set_setting(key, value))
}
