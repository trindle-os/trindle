//! Copies Trindle's look to GNOME's settings, which other people's GTK programs read for themselves, through dconf's own service on D-Bus.

use std::sync::OnceLock;

use zbus::blocking::Connection;

use super::look;

const NAME: &str = "ca.desrt.dconf";
const PATH: &str = "/ca/desrt/dconf/Writer/user";
const WRITER: &str = "ca.desrt.dconf.Writer";

const CHOSEN: &[&str] = &["color-scheme", "gtk-theme", "icon-theme", "cursor-theme", "cursor-size",
                          "font-name", "enable-animations"];

fn bus() -> Option<&'static Connection> {
    static BUS: OnceLock<Option<Connection>> = OnceLock::new();
    BUS.get_or_init(|| Connection::session().ok()).as_ref()
}

pub fn copy_look() {
    look::write_qt();
    let Some(bus) = bus() else { return };
    let change = look::dconf_change(&paths(&look::now()));
    let _ = bus.call_method(Some(NAME), PATH, Some(WRITER), "Change", &(change,));
}

fn paths(now: &[look::Entry]) -> Vec<(String, &look::Value)> {
    let mut paths = Vec::new();
    for (_, key, value) in now.iter().filter(|e| e.0 == look::INTERFACE && CHOSEN.contains(&e.1)) {
        paths.push((format!("/org/gnome/desktop/interface/{key}"), value));
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_chosen_look_goes_to_gnomes_settings_only() {
        let now = look::now();
        let paths = paths(&now);
        assert_eq!(paths.len(), CHOSEN.len());
        assert!(paths.iter().any(|(p, _)| p == "/org/gnome/desktop/interface/color-scheme"));
        assert!(paths.iter().all(|(p, _)| p.starts_with("/org/gnome/")));
    }

    #[test]
    #[ignore = "needs dbus-daemon, dconf and gsettings and changes this process's surroundings: run by hand"]
    fn dconf_keeps_the_copy() {
        use std::io::BufRead;
        crate::common::needs::programs(&["dbus-daemon", "gsettings"]);
        let home = std::env::temp_dir().join(format!("trindle-dconf-{}", std::process::id()));
        let runtime = home.join("run");
        std::fs::create_dir_all(&runtime).unwrap();
        std::fs::create_dir_all(home.join("gtk-4.0")).unwrap();
        std::fs::write(home.join("gtk-4.0/settings.ini"), "[Settings]\ngtk-font-name=Example Sans 13\n").unwrap();
        let mut daemon = std::process::Command::new("dbus-daemon")
            .args(["--session", "--print-address=1", "--nofork", "--nopidfile"])
            .env("XDG_CONFIG_HOME", &home).env("XDG_RUNTIME_DIR", &runtime)
            .env_remove("DISPLAY").env_remove("WAYLAND_DISPLAY")
            .stdout(std::process::Stdio::piped()).spawn().unwrap();
        let mut address = String::new();
        std::io::BufReader::new(daemon.stdout.take().unwrap()).read_line(&mut address).unwrap();
        std::env::set_var("DBUS_SESSION_BUS_ADDRESS", address.trim());
        std::env::set_var("XDG_CONFIG_HOME", &home);
        std::env::set_var("XDG_RUNTIME_DIR", &runtime);
        std::env::set_var("GSETTINGS_BACKEND", "dconf");
        copy_look();
        let get = |key: &str| {
            let out = std::process::Command::new("gsettings").args(["get", "org.gnome.desktop.interface", key])
                .output().unwrap();
            String::from_utf8_lossy(&out.stdout).trim().trim_matches('\'').to_string()
        };
        let (icons, scheme) = (get("icon-theme"), get("color-scheme"));
        let (size, font) = (get("cursor-size").parse::<i32>().unwrap_or(0), get("font-name"));
        let _ = daemon.kill();
        let _ = daemon.wait();
        let _ = std::fs::remove_dir_all(&home);
        assert_eq!((icons.as_str(), scheme.as_str(), size), ("Trindle-Circle", "prefer-dark", 24));
        assert_eq!(font.as_str(), "Example Sans 13");
    }
}
