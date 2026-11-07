// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! trindle-compose: Trindle's own compositor. It draws every window, the
//! panel, the ring and the wallpaper, reads the keyboard, mouse and touchpad,
//! and runs the screens.
//!
//! It began as anvil, Smithay's example compositor (MIT, Victor Berger and
//! Victoria Brekenfeld), and is being turned into Trindle's own from there.
//!
//!   trindle-compose                 a Trindle desktop on the computer's own screens
//!   trindle-compose --greeter CMD   the login screen: CMD filling the screen, ending with it
//!   trindle-compose --window        inside a window on another desktop, for trying it
//!   trindle-compose --start CMD     start CMD once ready, instead of Trindle's autostart

extern "C" fn set_words() {
    trindle_toolkit::host::set(&trindle_shared::Words);
}

#[used]
#[link_section = ".init_array"]
static TOOLKIT_HOST: extern "C" fn() = set_words;

#[macro_use]
#[path = "../said.rs"]
#[allow(dead_code)]
mod said;
pub(crate) use trindle_shared::files;
pub(crate) use trindle_shared::pointer;
pub(crate) use trindle_shared::combos;
pub(crate) use trindle_shared::grants;
#[path = "../common/memory.rs"]
mod memory;
pub(crate) use trindle_shared::crash;
pub(crate) use trindle_shared::in_use;
pub(crate) use trindle_shared::screen_scale;
pub(crate) use trindle_shared::levels;
pub(crate) use trindle_shared::locked;
#[cfg(test)]
pub(crate) use trindle_toolkit::common::source;
pub(crate) use trindle_shared::summon;
#[path = "../common/expand.rs"]
mod expand;
pub(crate) use trindle_shared::sandbox;
pub(crate) use trindle_shared::ring_pulse;
#[path = "../common/flow_unlock.rs"]
mod flow_unlock;
pub(crate) use trindle_shared::pin;
pub(crate) use trindle_shared::clock;
pub(crate) use trindle_shared::own_hands;
#[path = "../common/own_helpers.rs"]
mod own_helpers;
#[cfg(test)]
#[path = "../common/own_helpers_set.rs"]
mod own_helpers_set;
pub(crate) use trindle_toolkit::common::own_press;
pub(crate) use trindle_shared::person_only;
pub(crate) use trindle_shared::restart;
pub(crate) use trindle_shared::remap;
pub(crate) use trindle_shared::pads;
pub(crate) use trindle_shared::controller_wire;
pub(crate) use trindle_shared::ring_said;
pub use trindle_shared::x11_rooms;
pub(crate) use trindle_toolkit::common::sheet;
pub(crate) use trindle_toolkit::picture;
mod common {
    pub(crate) use super::{combos, files, pointer, sheet};
}

#[cfg(test)]
#[path = "../common/wire.rs"]
mod wire;

mod config;
mod cursor;
mod dragging;
mod expander;
mod reader_keys;
mod drawing;
mod drm_extras;
mod animate;
mod capture;
mod capture_scene;
mod focus;
mod idle;
mod limits;
mod indicators;
mod input_handler;
mod keys_changed;
mod level;
mod lock;
mod newcomers;
mod own_ring;
mod panel;
mod protocols;
mod clean_start;
mod modal;
mod modeset_recovery;
mod render;
mod rest;
#[cfg(test)]
mod games;
mod game_pads;
#[cfg(test)]
mod rig;
mod screen_edge;
mod scroll;
mod session;
mod shell;
mod trusted;
mod virtual_input;
mod virtual_keyboard;
mod state;
mod stopped;
mod udev;
mod winit;
mod wm;
mod xdnd;
mod xrooms;
mod xwayland_later;

pub use state::{AnvilState, ClientState};

fn kept_in_log(rest: bool) -> String {
    let rest = if rest { format!(",{}=info", rest::LOG) } else { String::new() };
    format!("warn,{}=info,{}=info{rest}", capture::LOG, own_ring::LOG)
}

fn main() {
    xwayland_later::run_if_asked();
    memory::few_heaps();
    session::more_files();
    crash::watch("trindle-compose");
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(kept_in_log(rest::told())));
    tracing_subscriber::fmt().compact().with_env_filter(filter).init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let after = |flag: &str| args.iter().position(|a| a == flag).and_then(|i| args.get(i + 1)).cloned();
    let greeter = after("--greeter");
    let startup = greeter.clone().or_else(|| after("--start"))
        .or_else(|| (!args.iter().any(|a| a == "--window")).then(|| session::AUTOSTART.to_string()));
    let launch = session::Launch {
        greeter: greeter.is_some(),
        ring: (greeter.is_none() && startup.as_deref() == Some(session::AUTOSTART)).then(own_ring::program).flatten(),
        startup,
    };
    if !launch.greeter {
        config::apply_environment();
    }
    std::env::set_var("XDG_SESSION_TYPE", "wayland");
    let _ = session::LAUNCH.set(launch);
    if args.iter().any(|a| a == "--window") {
        winit::run_winit();
    } else {
        udev::run::run_udev();
    }
}
