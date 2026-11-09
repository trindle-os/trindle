// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::fmt::Write as _;
use std::time::{Duration, Instant};

use smithay::desktop::layer_map_for_output;
use smithay::wayland::seat::WaylandFocus;

use wayland_protocols::wp::text_input::zv3::client::zwp_text_input_v3;

use super::*;

impl Rig {

    pub fn sent(&self, who: usize) -> &Program {
        &self.programs[who].program
    }

    pub fn allow(&self, id: &str, what: &[&str]) {
        let folder = self.dir.join("home/.local/share/trindle/plugins").join(id);
        std::fs::create_dir_all(&folder).unwrap();
        std::fs::write(folder.join("allowed"), what.iter().fold(String::new(), |mut s, w| { let _ = writeln!(s, "{w}"); s })).unwrap();
    }

    pub fn refused(&mut self, who: usize) -> usize {
        std::mem::take(&mut self.programs[who].program.refused)
    }

    pub fn flow_from_the_catalogue(&self) {
        use crate::own_hands::{FLOW, MARK};
        self.allow(FLOW, &["control"]);
        let mark = self.dir.join("home/.local/share/trindle/plugins").join(FLOW).join(MARK);
        std::fs::write(&mark, format!("{}\n", "ab".repeat(32))).unwrap();
        std::fs::set_permissions(&mark, std::os::unix::fs::PermissionsExt::from_mode(0o600)).unwrap();
    }

    pub fn a_moment(&mut self) {
        std::thread::sleep(Duration::from_millis(600));
        self.pump();
    }

    pub fn typed(&self, who: usize) -> &str {
        &self.programs[who].program.typed
    }

    pub fn password_box(&mut self, who: usize) {
        let p = &self.programs[who];
        let qh = p.queue.handle();
        let input = p.program.texts.as_ref().expect("the text input global")
            .get_text_input(p.program.seat.as_ref().expect("a seat"), &qh, ());
        input.enable();
        input.set_content_type(zwp_text_input_v3::ContentHint::None, zwp_text_input_v3::ContentPurpose::Password);
        input.commit();
        self.pump();
    }

    pub fn pressed(&self, who: usize) -> &[u32] {
        &self.programs[who].program.pressed
    }

    pub fn has_keyboard(&self, who: usize) -> bool {
        let focus = self.state.seat.get_keyboard().and_then(|k| k.current_focus());
        let whose = focus.and_then(|f| f.wl_surface().and_then(|s| smithay::reexports::wayland_server::Resource::client(&*s))).map(|c| c.id());
        whose.as_ref() == Some(&self.programs[who].id)
    }

    pub fn numbered(&mut self, who: usize) -> Vec<u32> {
        std::mem::take(&mut self.programs[who].program.numbered)
    }

    pub fn keys_with(&self) -> Option<String> {
        match self.state.seat.get_keyboard()?.current_focus()? {
            crate::focus::KeyboardFocusTarget::LayerSurface(panel) => Some(panel.namespace().to_string()),
            _ => None,
        }
    }

    pub fn standing(&self) -> Vec<String> {
        layer_map_for_output(&self.output).layers().map(|l| l.namespace().to_string()).collect()
    }

    pub fn hidden(&self, name: &str) -> Vec<bool> {
        let map = layer_map_for_output(&self.output);
        map.layers().filter(|l| l.namespace() == name).map(|l| crate::trusted::hidden_by_full_screen(&self.output, l)).collect()
    }

    pub fn ran(&mut self) -> usize {
        self.ran_as("ring")
    }

    pub fn shots_by_command(&mut self) -> usize {
        self.ran_as("shot")
    }

    pub(super) fn ran_as(&mut self, what: &str) -> usize {
        self.ran_lines(what).len()
    }

    pub(super) fn ran_lines(&mut self, what: &str) -> Vec<String> {
        let since = Instant::now();
        while since.elapsed() < Duration::from_millis(400) {
            self.pump();
            std::thread::sleep(Duration::from_millis(20));
        }
        std::fs::read_to_string(self.dir.join("ran")).map_or(Vec::new(), |t| t.lines().filter(|l| l.starts_with(what)).map(String::from).collect())
    }

    pub fn settings_ran(&mut self) -> Vec<String> {
        self.ran_lines("settings")
    }

    pub fn bind_the_screen_reader(&mut self) {
        use crate::config::{Action, Keybind, Mods};
        for (key, command) in [(smithay::input::keyboard::Keysym::F7, crate::wm::READER_COMMAND), (smithay::input::keyboard::Keysym::F6, "trindle-settings --panel power")] {
            self.state.config.keybinds.push(Keybind { mods: Mods::default(), key, on_release: false, action: Action::Execute(command.into()) });
        }
    }

    pub fn bind_the_ring(&mut self) {
        use crate::config::{Action, Keybind, Mods};
        self.state.config.keybinds.push(Keybind {
            mods: Mods::default(),
            key: smithay::input::keyboard::Keysym::F5,
            on_release: false,
            action: Action::Execute("trindle-ring".into()),
        });

        self.state.config.keybinds.push(Keybind {
            mods: Mods::default(),
            key: smithay::input::keyboard::Keysym::F6,
            on_release: false,
            action: Action::Execute("trindle-screenshot".into()),
        });
    }
}
