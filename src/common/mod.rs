

pub use crate::toolkit::common::{eye, own_press, selection, sheet, shm, ui};
#[cfg(test)]
pub use crate::toolkit::common::source;

pub const ORDINARY_UIDS: std::ops::Range<u32> = 1000..60000;

pub mod host;
#[cfg(test)]
mod from_the_toolkit;
#[cfg(test)]
pub mod scratch_home;
#[cfg(test)]
pub mod help_pages;
#[cfg(test)]
pub mod info_popup_tests;
#[cfg(test)]
mod xdnd_test;
#[cfg(test)]
mod xwm_test;
#[cfg(test)]
mod painted_in_every_theme;
pub mod no_tick;
pub mod terms;
pub mod access;
pub mod language_names;
pub mod seccomp;
pub mod app_theme;
pub mod app_choice;
pub mod apps;
#[cfg(test)]
mod own_tests;
#[cfg(test)]
mod words_tests;
pub mod audio;
pub mod autostart;
pub mod capture;
pub use trindle_shared::boot;
pub mod charge;
pub mod games;
pub mod steam_files;
pub mod drive_leave;
pub mod firewall;
pub mod clipboard;
pub mod damaged;
pub use trindle_shared::clock;
pub use trindle_shared::combos;
pub mod key_list;
pub mod copied;
pub mod compositor;
pub use trindle_shared::crash;
pub mod dconf;
pub mod dns;
#[cfg(test)]
mod every_secret_has_an_eye;
pub mod faces;
pub mod favourites;
pub mod copies;
pub mod cups;
pub mod boxed_self;
pub mod kinds;
pub mod looks;
pub use trindle_shared::files;
pub mod filling;
pub use crate::toolkit::common::file_work::gentle;
#[cfg(test)]
mod gentle_guard;
pub mod hidden_drives;
pub mod keyfile;
pub mod graphics;
pub mod more;
pub mod no_driver;
pub use trindle_shared::grants;
pub use trindle_shared::summon;
pub mod expand;
pub mod held;

pub use trindle_shared::in_use;

pub mod kept_order;
pub mod keyring_said;
#[cfg(test)]
mod never_asks_twice;
pub use trindle_shared::kit;
pub use trindle_shared::levels;
pub use trindle_shared::locked;
pub mod look;
pub mod memory;
pub mod missing_icons;
#[cfg(test)]
mod folder_colour;
#[cfg(test)]
mod apt_store;
#[cfg(test)]
mod no_old_name;
#[cfg(test)]
mod no_plugin_named;
#[cfg(test)]
mod only_the_catalogue;
#[cfg(test)]
pub mod needs;
#[cfg(test)]
mod never_written_over;
#[cfg(test)]
mod unreadable_kept;
#[cfg(test)]
mod no_waiting_on_a_name;
#[cfg(test)]
mod one_window;
#[cfg(test)]
mod keys_panels;
#[cfg(test)]
mod no_waiting_on_a_share;
#[cfg(test)]
mod nothing_named_is_followed;
#[cfg(test)]
pub(crate) mod no_writing_in_place;
#[cfg(test)]
mod one_decoder;
#[cfg(test)]
mod redirects_stay_https;
#[cfg(test)]
mod save_is_a_word;
#[cfg(test)]
mod jobs_say_why;
#[cfg(test)]
mod red_for_good;
pub mod otheros;
pub mod outputs;
pub mod outside;
pub mod wire;
pub mod packs;
pub use trindle_shared::pin;
pub mod removal;
pub mod pocketkey;
pub use trindle_shared::own_hands;
pub mod own_helpers;
pub mod own_helpers_set;

pub use trindle_shared::person_only;
pub use trindle_shared::pointer;
pub mod refused;
pub mod renewed;
pub use trindle_shared::restart;

pub use trindle_shared::ring_pulse;
pub use trindle_shared::ring_said;

pub use trindle_shared::remap;

pub use trindle_shared::pads;
pub use trindle_shared::controller_wire;
pub mod plain;
pub mod bt_audio;
pub mod rfkill;
pub mod run;
pub use trindle_shared::sandbox;
pub mod sharing;
pub mod smb;
pub mod snapshots;
pub mod snapshot_title;
pub mod home_snapshots;
pub mod screen_reader;

pub use trindle_shared::screen_scale;
pub mod key_enrolment;
pub mod security_updates;
pub mod tailscale;
pub mod update_asked;
pub mod update_checks;
pub mod stopping;
pub mod updating;
pub mod sounds;
pub mod tip;
pub mod sources;
pub mod style;
pub mod themes;
pub mod volumes;
pub use trindle_shared::x11_rooms;

pub fn thousands(n: u64) -> String {
    trindle_toolkit::lang::format::number(n as f64, 0)
}

pub fn online() -> bool {
    let read = |file: &str| std::fs::read_to_string(file).unwrap_or_default();
    way_out(&read("/proc/net/route"), &read("/proc/net/ipv6_route"))
}

fn way_out(v4: &str, v6: &str) -> bool {
    let old = v4.lines().skip(1).any(|l| l.split_whitespace().nth(1) == Some("00000000"));
    old || v6.lines().any(|l| {
        let cols: Vec<&str> = l.split_whitespace().collect();
        cols.len() >= 10 && cols[0].bytes().all(|b| b == b'0') && cols[1] == "00" && cols[9] != "lo"
    })
}

#[cfg(test)]
mod online_tests {
    use super::way_out;

    const HEAD: &str = "Iface\tDestination\tGateway \tFlags\tRefCnt\tUse\tMetric\tMask\t\tMTU\tWindow\tIRTT\n";
    const LO6: &str = "00000000000000000000000000000000 00 00000000000000000000000000000000 00 00000000000000000000000000000000 ffffffff 00000001 00000000 00200200       lo\n";

    #[test]
    fn a_route_to_everywhere_of_either_kind_is_a_way_out() {
        let v4 = format!("{HEAD}wlan0\t00000000\t0101A8C0\t0003\t0\t0\t600\t00000000\t0\t0\t0\nwlan0\t0001A8C0\t00000000\t0001\t0\t0\t600\t00FFFFFF\t0\t0\t0\n");
        assert!(way_out(&v4, LO6));
        let local = format!("{HEAD}wlan0\t0001A8C0\t00000000\t0001\t0\t0\t600\t00FFFFFF\t0\t0\t0\n");
        assert!(!way_out(&local, LO6), "a network with no way beyond itself");
        assert!(!way_out(HEAD, LO6), "the kernel's own route on lo leads nowhere");
        assert!(!way_out("", ""));
        let v6 = format!("{LO6}00000000000000000000000000000000 00 00000000000000000000000000000000 00 fe800000000000000000000000000001 00000064 00000002 00000000 00450003    wlan0\n");
        assert!(way_out(HEAD, &v6), "a network that gives only the new kind of address");
    }
}
