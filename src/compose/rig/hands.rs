// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use smithay::backend::input::InputEvent;

use crate::virtual_input::{Absolute, Button, Did, Key, Maker, Scroll, Virtual};

use super::*;

impl Rig {

    pub fn press(&mut self, key: Option<u32>) -> Did {
        let time = self.next_time();
        match key {
            Some(code) => Did::Key(Key { time, code, pressed: true }),
            None => Did::Button(Button { time, button: BTN_LEFT, pressed: true }),
        }
    }

    pub(super) fn next_time(&mut self) -> u32 {
        self.time += 20;
        self.time
    }

    pub fn key_by_hand(&mut self, code: u32) {
        for pressed in [true, false] {
            let event = Key { time: self.next_time(), code, pressed };
            self.state.process_input_event_windowed(InputEvent::<Virtual>::Keyboard { event }, SCREEN);
            self.pump();
        }
    }

    pub fn key_held_by_hand(&mut self, code: u32, held: u32) {
        let time = self.next_time();
        for (pressed, at) in [(true, time), (false, time + held)] {
            let event = Key { time: at, code, pressed };
            self.state.process_input_event_windowed(InputEvent::<Virtual>::Keyboard { event }, SCREEN);
            self.pump();
        }
        self.time = time + held;
    }

    pub fn key_by_hand_only(&mut self, code: u32, pressed: bool) {
        let event = Key { time: self.next_time(), code, pressed };
        self.state.process_input_event_windowed(InputEvent::<Virtual>::Keyboard { event }, SCREEN);
        self.pump();
    }

    pub fn button_by_hand(&mut self, button: u32) {
        for pressed in [true, false] {
            let event = Button { time: self.next_time(), button, pressed };
            self.state.process_input_event_windowed(InputEvent::<Virtual>::PointerButton { event }, SCREEN);
            self.pump();
        }
    }

    pub fn key_by_a_program(&mut self, code: u32) {
        for pressed in [true, false] {
            let event = Key { time: self.next_time(), code, pressed };
            self.state.by_a_program(|s| s.windowed_event(InputEvent::<Virtual>::Keyboard { event }, SCREEN));
            self.pump();
        }
    }

    pub fn click_by_a_program(&mut self) {
        for pressed in [true, false] {
            let event = Button { time: self.next_time(), button: BTN_LEFT, pressed };
            self.state.by_a_program(|s| s.windowed_event(InputEvent::<Virtual>::PointerButton { event }, SCREEN));
            self.pump();
        }
    }

    pub fn change_keys(&mut self, lines: &str) {
        let file = self.dir.join("home/.config/trindle/remap.conf");
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(file, lines).unwrap();
        self.state.keys_reread();
        self.pump();
    }

    pub fn made_up_key(&mut self, code: u32) {
        for pressed in [true, false] {
            let event = Key { time: self.next_time(), code, pressed };
            self.state.virtual_input(Maker { from_box: true, plugin: None }, Did::Key(event));
            self.pump();
        }
    }

    pub fn move_by_hand(&mut self, x: f64, y: f64) {
        let event = Absolute { time: self.next_time(), x, y };
        self.state.process_input_event_windowed(InputEvent::<Virtual>::PointerMotionAbsolute { event }, SCREEN);
        self.pump();
    }

    pub fn made_up_move(&mut self, x: f64, y: f64) {
        let event = Absolute { time: self.next_time(), x, y };
        self.state.virtual_input(Maker::default(), Did::Absolute(event));
        self.pump();
    }

    pub fn click_by_hand(&mut self) {
        for pressed in [true, false] {
            let event = Button { time: self.next_time(), button: BTN_LEFT, pressed };
            self.state.process_input_event_windowed(InputEvent::<Virtual>::PointerButton { event }, SCREEN);
            self.pump();
        }
    }

    pub fn scroll_by_hand(&mut self) {
        let event = Scroll { time: self.next_time(), source: smithay::backend::input::AxisSource::Wheel, amount: [0.0, 15.0], v120: [None, Some(120.0)], stopped: [false; 2] };
        self.state.process_input_event_windowed(InputEvent::<Virtual>::PointerAxis { event }, SCREEN);
        self.pump();
    }

    pub fn made_up_click(&mut self) {
        for pressed in [true, false] {
            let event = Button { time: self.next_time(), button: BTN_LEFT, pressed };
            self.state.virtual_input(Maker::default(), Did::Button(event));
            self.pump();
        }
    }

    pub fn move_by_hand_on(&mut self, screen: &str, x: f64, y: f64) {
        let event = Absolute { time: self.next_time(), x, y };
        self.state.process_input_event_windowed(InputEvent::<Virtual>::PointerMotionAbsolute { event }, screen);
        self.pump();
    }

    pub fn click_through_no_door(&mut self) {
        for pressed in [true, false] {
            let event = Button { time: self.next_time(), button: BTN_LEFT, pressed };
            self.state.windowed_event(InputEvent::<Virtual>::PointerButton { event }, SCREEN);
            self.pump();
        }
    }

    pub fn made_key(&mut self, maker: &Maker, code: u32) {
        for pressed in [true, false] {
            let event = Key { time: self.next_time(), code, pressed };
            self.state.virtual_input(maker.clone(), Did::Key(event));
            self.pump();
        }
    }

    pub fn made_move(&mut self, maker: &Maker, x: f64, y: f64) {
        let event = Absolute { time: self.next_time(), x, y };
        self.state.virtual_input(maker.clone(), Did::Absolute(event));
        self.pump();
    }

    pub fn made_click(&mut self, maker: &Maker) {
        for pressed in [true, false] {
            let event = Button { time: self.next_time(), button: BTN_LEFT, pressed };
            self.state.virtual_input(maker.clone(), Did::Button(event));
            self.pump();
        }
    }
}
