//! Keyboard layout, alternate layout and Caps Lock behavior, keys and buttons changed, the keyboards and mice programs made, and the mouse and touchpad.

mod changed_keys;
mod devices;
mod layout;
mod mouse;
mod restart_keys;

use std::collections::HashMap;
use std::time::Duration;

use layout::{Chosen, Variants};

use crate::common::files::setting;
use crate::common::pointer::Pointer;
use crate::common::remap::Table;
use crate::plugins::shortcuts::Binding;
use crate::settings::ui::{group, headline, line, on, page, Kit, Msg, Page, not_saved};
use crate::toolkit::t;
use crate::toolkit::widget::Node;

const KEYS_PAUSE: Duration = Duration::from_millis(900);

const CAPS_KEY: &str = "keyboard-caps";
const CLICKER: &str = "keyboard-clicker";
const PICK_WAIT: Duration = Duration::from_secs(10);
fn nothing_pressed() -> &'static str {
    t!("settings-keyboard-nothing-pressed")
}
fn not_told() -> &'static str {
    t!("settings-keyboard-not-told")
}

fn keyboard_button_on() -> bool {
    setting("keyboard_button") != "off"
}

fn clicker_on() -> bool {
    crate::common::own_helpers::on(crate::common::own_helpers::Helper::Clicker)
}

fn said(kit: &mut Kit, key: &str, saved: std::io::Result<()>) -> bool {
    kit.settle(key, saved, not_saved()).is_some()
}

pub struct Keyboard {
    chosen: Chosen,
    variants: Variants,
    names: HashMap<String, String>,
    all: Vec<(String, String)>,
    read: bool,
    pointer: Pointer,
    touchpad: bool,
    button: bool,
    clicker: bool,
    rounds: [u32; 3],
    plugin_keys: Vec<Binding>,
    restart_typed: Vec<(&'static str, String)>,
    keys_round: u32,
    table: Table,
    table_unread: Option<String>,
    typed: HashMap<usize, String>,
    typed_round: u32,
    picking: bool,
    pick_round: u32,
    devices: Vec<(bool, String)>,
    allowed: Vec<String>,
    usb: Vec<crate::usbretry::Entry>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard {
            chosen: Chosen::default(),
            variants: Variants::new(),
            names: HashMap::new(),
            all: Vec::new(),
            read: false,
            pointer: Pointer::load(),
            touchpad: false,
            button: false,
            clicker: false,
            rounds: [0; 3],
            plugin_keys: Vec::new(),
            restart_typed: Vec::new(),
            keys_round: 0,
            table: Table::default(),
            table_unread: None,
            typed: HashMap::new(),
            typed_round: 0,
            picking: false,
            pick_round: 0,
            devices: Vec::new(),
            allowed: Vec::new(),
            usb: Vec::new(),
        }
    }
}

impl Keyboard {

}

impl Page for Keyboard {
    fn id(&self) -> &'static str {
        "keyboard"
    }
    fn title(&self) -> &'static str {
        t!("settings-keyboard-title")
    }
    fn icon(&self) -> &'static str {
        "input-keyboard-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-keyboard-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-keyboard-help")
    }

    fn shown(&mut self, _kit: &mut Kit) {
        self.load();
    }

    fn conf_changed(&mut self, _kit: &mut Kit) {
        self.button = keyboard_button_on();
        self.clicker = clicker_on();
        self.pointer = Pointer::load();
    }

    fn intercept(&self, input: &crate::toolkit::input::Input) -> Option<Msg> {
        use crate::toolkit::input::Input;
        if !self.picking {
            return None;
        }
        match input {
            Input::Key { repeat: false, .. } | Input::Press { .. } => {
                let code = crate::toolkit::window::code_now();
                Some(on::<Keyboard>(move |me, k| me.picked(k, code)))
            }
            _ => None,
        }
    }

    fn view(&self, kit: &Kit) -> Node<Msg> {
        let mut items = Vec::new();
        if self.names.is_empty() {
            items.push(headline(kit, t!("settings-keyboard-no-list")));
            items.push(group(t!("settings-keyboard-no-list-title")).icon("input-keyboard-symbolic")
                .rows(vec![line(t!("settings-keyboard-no-list-heading"))
                    .sub(t!("settings-keyboard-no-list-about")).node(kit)])
                .node(kit));
        } else {
            items.push(headline(kit, self.tell()));
            items.extend(self.keyboard_cards(kit));
        }
        items.push(self.ring_card(kit));
        items.push(self.keys_card(kit));
        items.push(self.made_card(kit));
        items.extend(self.mouse_cards(kit));
        if !self.usb.is_empty() {
            items.push(self.usb_card(kit));
        }
        items.push(self.shortcuts_card(kit));
        items.push(self.restart_card(kit));
        if !self.plugin_keys.is_empty() {
            items.push(self.plugin_keys_card(kit));
        }
        page(kit, self.title(), self.help(), items)
    }
}

#[cfg(test)]
impl Keyboard {
    pub(crate) fn made_up() -> Keyboard {
        let all: Vec<(String, String)> = [("gb", "English (UK)"), ("fr", "French"), ("de", "German")]
            .iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
        let mut table = Table::read("Caps Lock = Escape tapped, Ctrl held\nBack Button on \"Logitech MX Master 3\" = Ctrl+C\nF1 = H, E, L, P\n");
        table.set_swap("alt-super", true);
        let devices = vec![(false, "AT Translated Set 2 keyboard".into()), (false, "Logitech MX Master 3".into()),
                           (true, "keyd virtual keyboard".into()), (true, "ydotoold virtual device".into())];
        Keyboard { chosen: Chosen { first: "gb".into(), second: "fr".into(), ..Chosen::default() },
                   names: all.iter().cloned().collect(), all, read: true, touchpad: true, table, devices,
                   allowed: vec!["keyd virtual keyboard".into()], ..Keyboard::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::mouse::times;
    use crate::common::pointer;
    use super::*;

    #[test]
    fn a_speed_is_written_with_the_languages_decimal_mark() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let paces = [0.5, 1.0, 2.0];
        assert_eq!((times(1.5, &paces), times(1.0, &paces), times(0.75, &paces)), ("1.5\u{d7}".to_string(), "1\u{d7}".to_string(), "0.75\u{d7}".to_string()));
        with_catalogue(Catalogue::new(Bundle::from_text("xx", "kit-number-decimal = ,\n"), english()), || {
            assert_eq!((times(1.5, &paces), times(1.0, &paces), times(0.75, &paces)), ("1,5\u{d7}".to_string(), "1\u{d7}".to_string(), "0,75\u{d7}".to_string()));
        });
    }

    #[test]
    fn a_choice_that_could_not_be_kept_says_why() {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: crate::settings::ui::Do| {});
        let mut kit = Kit::new(events.clone(), asks, None, None);
        assert!(!said(&mut kit, pointer::TAP, Err(std::io::Error::from_raw_os_error(28))));
        assert_eq!(kit.said(pointer::TAP), Some(("It could not be saved. The disk is full.", true)));
        assert!(said(&mut kit, pointer::TAP, Ok(())));
        assert_eq!(kit.said(pointer::TAP), None);
    }
}
