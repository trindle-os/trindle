// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Keys and buttons as the person changed them (common/remap.rs), put in
//! the compositor's two doors, so each press is changed before anything
//! sees it: the shortcuts, the ring, the lock screen, every program. And
//! the keyboards and mice a program made in the kernel (uinput), which
//! type and play but are not the person's own unless the person allowed
//! them by name.
//!
//! A changed press goes on as a made-up key or click would (virtual_input.rs),
//! with the same standing it came with: a real device's change is still the
//! person's own press. Before it is passed on, a Trindle window is told
//! which key was really pressed (`trindle_own_press_v1`'s `was`), so
//! Settings can choose a key by its being pressed.

use std::fmt::Write as _;
use std::collections::HashMap;
use std::path::PathBuf;

use smithay::backend::input::{ButtonState, Device, Event, InputBackend, InputEvent, KeyState, KeyboardKeyEvent, PointerButtonEvent};

use crate::remap::{Changer, Out, Table};
use crate::state::{AnvilState, Backend};
use crate::virtual_input::{Button, Key};

const MADE_BY_A_PROGRAM: &str = "/sys/devices/virtual/";

pub fn made_by_a_program(syspath: Option<&std::path::Path>) -> bool {
    syspath.is_some_and(|p| p.starts_with(MADE_BY_A_PROGRAM))
}

#[derive(Clone, Debug, PartialEq)]
struct Seen {
    name: String,
    made: bool,
}

#[derive(Debug, Default)]
pub struct KeysChanged {
    changer: Changer,
    devices: HashMap<String, Seen>,
    names: Vec<Seen>,
    allowed: Vec<String>,
    pub was: Option<u32>,
    pub from_a_program: bool,
}

fn table() -> Table {
    if crate::session::launch().greeter {
        crate::remap::for_login()
    } else {
        crate::remap::load().unwrap_or_default()
    }
}

pub fn sources() -> Vec<PathBuf> {
    let mut out = vec![crate::remap::file()];
    out.extend(crate::own_helpers::file());
    out
}

impl KeysChanged {
    pub fn load() -> KeysChanged {
        KeysChanged { changer: Changer::new(table()), allowed: crate::own_helpers::allowed_devices(), ..KeysChanged::default() }
    }

    pub fn arrived(&mut self, id: String, name: String, made: bool) {
        let seen = Seen { name: crate::remap::clean_name(&name), made };
        if !self.names.contains(&seen) {
            self.names.push(seen.clone());
            self.write_list();
        }
        self.devices.insert(id, seen);
    }

    pub fn left(&mut self, id: &str) {
        self.devices.remove(id);
    }

    fn write_list(&self) {
        let text: String = self.names.iter().fold(String::new(), |mut out, s| { let _ = writeln!(out, "{}\t{}", u8::from(s.made), s.name); out });
        if let Err(e) = crate::files::write_whole(&crate::remap::devices_file(), text.as_bytes()) {
            tracing::warn!("the list of keyboards and mice for Settings was not written: {e}");
        }
    }

    pub(crate) fn not_allowed(&self, id: &str) -> bool {
        self.devices.get(id).is_some_and(|s| s.made && !self.allowed.contains(&s.name))
    }

    pub fn plugged_in(&self, id: &str) -> bool {
        self.devices.get(id).is_some_and(|s| !s.made)
    }

    fn name(&self, id: &str, fallback: String) -> String {
        self.devices.get(id).map(|s| s.name.clone()).unwrap_or_else(|| crate::remap::clean_name(&fallback))
    }
}

enum Came {
    Key { code: u32, down: bool, time: u32 },
    Button { code: u32, down: bool, time: u32 },
}

fn came<B: InputBackend>(event: &InputEvent<B>) -> Option<(Came, B::Device)> {
    Some(match event {
        InputEvent::Keyboard { event } => {
            let code = event.key_code().raw().checked_sub(8)?;
            (Came::Key { code, down: event.state() == KeyState::Pressed, time: event.time_msec() }, event.device())
        }
        InputEvent::PointerButton { event } => {
            let down = event.state() == ButtonState::Pressed;
            (Came::Button { code: event.button_code(), down, time: event.time_msec() }, event.device())
        }
        _ => return None,
    })
}

pub type Console = Option<i32>;

impl<BackendData: Backend> AnvilState<BackendData> {
    pub fn keys_reread(&mut self) {
        self.keys_changed.allowed = crate::own_helpers::allowed_devices();
        let table = table();
        if self.keys_changed.changer.table() != &table {
            let outs = self.keys_changed.changer.set_table(table);
            self.play(outs);
        }
    }

    pub fn keys_device<B: InputBackend>(&mut self, event: &InputEvent<B>) {
        match event {
            InputEvent::DeviceAdded { device } => {
                let made = made_by_a_program(device.syspath().as_deref());
                if made {
                    tracing::info!(target: crate::capture::LOG, name = %device.name(), "a keyboard or mouse made by a program arrived");
                }
                self.keys_changed.arrived(device.id(), device.name(), made);
            }
            InputEvent::DeviceRemoved { device } => self.keys_changed.left(&device.id()),
            _ => {}
        }
    }

    pub fn from_a_program_not_allowed<B: InputBackend>(&self, event: &InputEvent<B>) -> bool {
        crate::input_handler::sender(event).is_some_and(|d| self.keys_changed.not_allowed(&d.id()))
    }

    pub fn by_a_program(&mut self, handle: impl FnOnce(&mut Self)) {
        let before = std::mem::replace(&mut self.ring.by_hand, false);
        let was = std::mem::replace(&mut self.keys_changed.from_a_program, true);
        handle(self);
        self.ring.by_hand = before;
        self.keys_changed.from_a_program = was;
    }

    pub fn kept_from_a_program<B: InputBackend>(&self, event: &InputEvent<B>) -> bool {
        use smithay::backend::input::{TabletToolTipEvent, TabletToolTipState, TabletToolButtonEvent};
        if !self.keys_changed.from_a_program {
            return false;
        }
        let did = match event {
            InputEvent::Keyboard { event } if event.state() == KeyState::Pressed => {
                crate::virtual_input::Did::Key(Key { time: 0, code: 0, pressed: true })
            }
            InputEvent::PointerButton { event } if event.state() == ButtonState::Pressed => {
                crate::virtual_input::Did::Button(Button { time: 0, button: 0, pressed: true })
            }
            InputEvent::TouchDown { .. } => crate::virtual_input::Did::Button(Button { time: 0, button: 0, pressed: true }),
            InputEvent::TabletToolTip { event } if event.tip_state() == TabletToolTipState::Down => {
                crate::virtual_input::Did::Button(Button { time: 0, button: 0, pressed: true })
            }
            InputEvent::TabletToolButton { event } if event.button_state() == ButtonState::Pressed => {
                crate::virtual_input::Did::Button(Button { time: 0, button: 0, pressed: true })
            }
            _ => return false,
        };
        self.not_for_a_made_up_press(&did)
    }

    pub fn keys_before_scroll(&mut self) {
        let outs = self.keys_changed.changer.other();
        if !outs.is_empty() {
            self.play(outs);
        }
    }

    pub fn through_table<B: InputBackend>(&mut self, event: &InputEvent<B>) -> Option<Console> {
        let (came, device) = came(event)?;
        if self.keys_changed.changer.idle() {
            return None;
        }
        self.keys_changed.changer.guarded = self.is_locked() || crate::session::launch().greeter;
        let name = self.keys_changed.name(&device.id(), device.name());
        let (code, down, time) = match came {
            Came::Key { code, down, time } | Came::Button { code, down, time } => (code, down, time),
        };
        let outs = self.keys_changed.changer.press(&name, code, down, time);
        if outs == [Out::Itself] {
            return None;
        }
        let itself = match came {
            Came::Key { .. } => Pass::Key { code, down, time },
            Came::Button { .. } => Pass::Button { code, down, time },
        };
        Some(self.play_with(outs, Some(itself), time, code))
    }

    fn play(&mut self, outs: Vec<Out>) -> Console {
        let time = self.clock.now().as_millis();
        self.play_with(outs, None, time, 0)
    }

    fn play_with(&mut self, outs: Vec<Out>, itself: Option<Pass>, time: u32, real: u32) -> Console {
        let mut console = None;
        for out in outs {
            let pass = match out {
                Out::Itself => match itself {
                    Some(p) => p,
                    None => continue,
                },
                Out::Down(code) | Out::Up(code) => {
                    let down = matches!(out, Out::Down(_));
                    if crate::remap::is_button(code) { Pass::Button { code, down, time } } else { Pass::Key { code, down, time } }
                }
            };
            self.keys_changed.was = (out != Out::Itself && real != 0).then_some(real);
            match pass {
                Pass::Key { code, down, time } => {
                    if let Some(vt) = self.changed_key(Key { time, code, pressed: down }) {
                        console = Some(vt);
                    }
                }
                Pass::Button { code, down, time } => self.changed_button(Button { time, button: code, pressed: down }),
            }
            self.keys_changed.was = None;
        }
        console
    }
}

#[derive(Clone, Copy)]
enum Pass {
    Key { code: u32, down: bool, time: u32 },
    Button { code: u32, down: bool, time: u32 },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rig::{alone, kept_ring::beat, Rig, KEY_A, KEY_F5};
    use crate::state::ClientState;

    const CAPS: u32 = 58;
    const ESC: u32 = 1;
    const CTRL: u32 = 29;
    const C: u32 = 46;
    const B: u32 = 48;
    const LEFT: u32 = 0x110;
    const RIGHT: u32 = 0x111;

    #[test]
    fn a_device_is_made_by_a_program_only_under_the_kernels_own_folder_for_them() {
        use std::path::Path;
        assert!(made_by_a_program(Some(Path::new("/sys/devices/virtual/input/input23/event20"))));
        assert!(!made_by_a_program(Some(Path::new("/sys/devices/pci0000:00/0000:00:14.0/usb1/1-2/input/input5/event5"))));
        assert!(!made_by_a_program(Some(Path::new("/sys/devices/virtualx/input/input1"))));
        assert!(!made_by_a_program(None));
        let mut keys = KeysChanged::default();
        keys.arrived("made".into(), "keyd virtual keyboard".into(), true);
        keys.arrived("real".into(), "AT Translated Set 2 keyboard".into(), false);
        assert!(keys.not_allowed("made") && !keys.not_allowed("real") && !keys.not_allowed("unknown"));
        keys.allowed = vec!["keyd virtual keyboard".into()];
        assert!(!keys.not_allowed("made"), "a device allowed by name was still not the person's");
        keys.left("made");
        assert!(keys.names.iter().any(|s| s.made && s.name == "keyd virtual keyboard"));
    }

    #[test]
    fn a_changed_key_reaches_the_window_as_its_new_key_still_the_persons_own_with_the_real_key_said() {
        alone("keys_changed::tests::a_changed_key_reaches_the_window_as_its_new_key_still_the_persons_own_with_the_real_key_said", |dir| {
            let mut rig = Rig::new(dir);
            let app = rig.join(ClientState::default());
            rig.window_called(app, "org.example.Notes");
            rig.onto_the_window_in_front();
            rig.named_first(app);
            rig.change_keys("Caps Lock = Escape\nA = Ctrl+C\nF5 = Left Button\nRight Button = B\nInsert = Nothing\n");
            rig.key_by_hand(CAPS);
            assert_eq!(rig.pressed(app).last(), Some(&ESC), "Caps Lock did not become Escape");
            assert_eq!(rig.sent(app).pressed_was.last(), Some(&Some(CAPS)), "the real key was not said");
            assert_eq!(rig.named_first(app), [true, true], "a changed key of a real keyboard was not the person's own");
            let before = rig.pressed(app).len();
            rig.key_by_hand(KEY_A);
            assert_eq!(&rig.pressed(app)[before..], [CTRL, C]);
            assert_eq!(&rig.sent(app).pressed_was[before..], [Some(KEY_A), Some(KEY_A)]);
            assert!(rig.state.seat.get_keyboard().unwrap().pressed_keys().is_empty(), "a key was left held");
            let clicks = rig.sent(app).clicked.len();
            rig.key_by_hand(KEY_F5);
            assert_eq!(&rig.sent(app).clicked[clicks..], [LEFT]);
            assert_eq!(rig.sent(app).clicked_was.last(), Some(&Some(KEY_F5)));
            rig.button_by_hand(RIGHT);
            assert_eq!(rig.pressed(app).last(), Some(&B), "the right button did not become B");
            assert_eq!(rig.sent(app).pressed_was.last(), Some(&Some(RIGHT)));
            let before = rig.pressed(app).len();
            rig.key_by_hand(110);
            assert_eq!(rig.pressed(app).len(), before, "a key changed to nothing was passed on");
            rig.key_by_hand(B);
            assert_eq!(rig.pressed(app).last(), Some(&B));
            assert_eq!(rig.sent(app).pressed_was.last(), Some(&None));
            rig.change_keys("F5 = H, I\nCaps Lock = Escape tapped, Ctrl held\n");
            let before = rig.pressed(app).len();
            rig.key_by_hand(KEY_F5);
            assert_eq!(&rig.pressed(app)[before..], [35, 23]);
            rig.key_by_hand(CAPS);
            assert_eq!(rig.pressed(app).last(), Some(&ESC), "Caps Lock tapped was not Escape");
            let before = rig.pressed(app).len();
            rig.key_by_hand_only(CAPS, true);
            rig.key_by_hand(KEY_A);
            rig.key_by_hand_only(CAPS, false);
            assert_eq!(&rig.pressed(app)[before..], [CTRL, KEY_A], "Caps Lock held with A was not Ctrl and A");
            let before = rig.pressed(app).len();
            rig.key_held_by_hand(CAPS, 900);
            assert_eq!(rig.pressed(app).len(), before, "Caps Lock held alone a long while did something");
            rig.change_keys("A = Ctrl+C\n");
            rig.key_by_hand_only(KEY_A, true);
            assert_eq!(rig.state.seat.get_keyboard().unwrap().pressed_keys().len(), 2);
            rig.change_keys("");
            assert!(rig.state.seat.get_keyboard().unwrap().pressed_keys().is_empty(), "Ctrl and C were left held when the table changed");
            rig.key_by_hand_only(KEY_A, false);
            rig.key_by_hand(KEY_A);
            assert_eq!(rig.pressed(app).last(), Some(&KEY_A), "with no changes A was not A");
        });
    }

    #[test]
    fn under_the_screen_lock_the_keys_that_send_a_password_are_left_as_they_are() {
        use crate::virtual_input::{Key, Virtual};
        alone("keys_changed::tests::under_the_screen_lock_the_keys_that_send_a_password_are_left_as_they_are", |dir| {
            let mut rig = Rig::new(dir);
            rig.change_keys("Enter = A\nBackspace = Nothing\nB = Nothing\nCaps Lock = Escape\n");
            let press = |rig: &mut Rig, code: u32, pressed: bool| {
                let event = InputEvent::<Virtual>::Keyboard { event: Key { time: 1, code, pressed } };
                rig.state.through_table(&event).is_some()
            };
            assert!(press(&mut rig, 28, true) && press(&mut rig, 28, false));
            assert!(press(&mut rig, B, true) && press(&mut rig, B, false));
            rig.state.lock_for_rig();
            for code in [28, 14, B] {
                assert!(!press(&mut rig, code, true), "key {code} was changed under the lock");
                assert!(!press(&mut rig, code, false));
            }
            assert!(press(&mut rig, CAPS, true) && press(&mut rig, CAPS, false));
        });
    }

    #[test]
    fn a_keyboard_a_program_made_types_and_clicks_but_never_into_a_question() {
        alone("keys_changed::tests::a_keyboard_a_program_made_types_and_clicks_but_never_into_a_question", |dir| {
            use crate::person_only as only;
            let mut rig = Rig::new(dir);
            let app = rig.join(ClientState::default());
            rig.window_called(app, "org.example.Notes");
            rig.onto_the_window_in_front();
            rig.named_first(app);
            rig.key_by_a_program(KEY_A);
            assert_eq!(rig.pressed(app).last(), Some(&KEY_A), "a program's keyboard could not type");
            let clicks = rig.sent(app).clicked.len();
            rig.click_by_a_program();
            assert_eq!(rig.sent(app).clicked.len(), clicks + 1, "a program's mouse could not click");
            assert_eq!(rig.named_first(app), [false; 4], "a program's key or click was named as the person's own");
            rig.change_keys("A = B\n");
            rig.key_by_a_program(KEY_A);
            assert_eq!(rig.pressed(app).last(), Some(&B));
            assert_eq!(rig.named_first(app), [false; 2]);
            rig.change_keys("");
            let mut ring = rig.kept_ring();
            for name in only::ASKED_BY_TRINDLE {
                beat(&mut ring.pulse);
                let question = rig.panel(ring.who, name);
                let before = rig.pressed(ring.who).len();
                rig.key_by_a_program(KEY_A);
                assert_eq!(rig.pressed(ring.who).len(), before, "{name}: a program's keyboard typed into it");
                rig.key_by_hand(KEY_A);
                assert_eq!(rig.pressed(ring.who).len(), before + 1, "{name}: the person's own key did not reach it");
                rig.put_away(&question);
            }
            for name in only::WINDOWS {
                let asker = rig.join(ClientState::default());
                rig.window_called(asker, name);
                rig.onto_the_window_in_front();
                let (keys, clicks) = (rig.pressed(asker).len(), rig.sent(asker).clicked.len());
                rig.key_by_a_program(KEY_A);
                rig.click_by_a_program();
                assert_eq!(rig.pressed(asker).len(), keys, "{name}: a program's keyboard typed into it");
                assert_eq!(rig.sent(asker).clicked.len(), clicks, "{name}: a program's mouse clicked in it");
                rig.key_by_hand(KEY_A);
                assert_eq!(rig.pressed(asker).len(), keys + 1, "{name}: the person's own key did not reach it");
            }
        });
    }
}
