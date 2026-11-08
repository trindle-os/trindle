//! The code the three programs share, so a setting or a message means the same in each.

#[macro_use]
#[path = "said.rs"]
pub mod said;

pub(crate) use trindle_toolkit as toolkit;
pub(crate) use trindle_toolkit::common::sheet;

pub struct Words;

impl trindle_toolkit::host::Host for Words {
    fn words(&self) -> &'static [(&'static str, &'static str)] {
        include!(concat!(env!("OUT_DIR"), "/english.rs"))
    }

    fn language(&self) -> Option<String> {
        Some(files::setting("language")).filter(|s| !s.trim().is_empty())
    }
}

#[cfg(test)]
extern "C" fn set_words() {
    trindle_toolkit::host::set(&Words);
}

#[cfg(test)]
#[used]
#[link_section = ".init_array"]
static TOOLKIT_HOST: extern "C" fn() = set_words;

mod common {
    pub(crate) use super::{files, kit};
}

#[path = "common/boot.rs"]
pub mod boot;
#[path = "common/clock.rs"]
pub mod clock;
#[path = "common/combos.rs"]
pub mod combos;
#[path = "common/controller_wire.rs"]
pub mod controller_wire;
#[path = "common/crash.rs"]
pub mod crash;
#[path = "common/files.rs"]
pub mod files;
#[path = "common/grants.rs"]
pub mod grants;
#[path = "common/in_use.rs"]
pub mod in_use;
#[path = "common/kit.rs"]
pub mod kit;
#[path = "common/levels.rs"]
pub mod levels;
#[path = "common/locked.rs"]
pub mod locked;
#[path = "common/pads.rs"]
pub mod pads;
#[path = "common/person_only.rs"]
pub mod person_only;
#[path = "common/pin.rs"]
pub mod pin;
#[path = "common/pointer.rs"]
pub mod pointer;
#[path = "common/remap.rs"]
pub mod remap;
#[path = "common/restart.rs"]
pub mod restart;
#[path = "common/ring_pulse.rs"]
pub mod ring_pulse;
#[path = "common/ring_said.rs"]
pub mod ring_said;
#[path = "common/sandbox.rs"]
pub mod sandbox;
#[path = "common/screen_scale.rs"]
pub mod screen_scale;
#[path = "common/summon.rs"]
pub mod summon;
#[path = "common/own_hands.rs"]
pub mod own_hands;
#[path = "common/x11_rooms.rs"]
pub mod x11_rooms;
