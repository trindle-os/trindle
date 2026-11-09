//! Tests of the boxed packages.

use super::*;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::store::parts;
use crate::common::files::{data_home, home};
use crate::common::x11_rooms::{self};
use super::added::{came_with_trindle, size_of_tree};
use super::first_apps::to_offer;
use super::install::{install_with, remove};
use super::launchers::{is_terminal, launcher, launchers_of};
use super::needs::{debs_in, providers_in, wants_from};
use super::run::{loader, plain_command, run_boxed};
use super::walls::{Gates, OWN_FOLDERS, engine_for, sys_args_for, walls};
use super::x11::{XHold, XNext, X_WORD_WAIT, x_next};

#[test]
fn a_box_whose_xwayland_ended_is_started_once_more_and_says_why_when_it_will_not_start() {
    use std::io::Write;
    use x11_rooms::Told;
    assert_eq!(x_next(None, 1), XNext::Done);
    assert_eq!(x_next(Some(Told::Lost), 1), XNext::Again);
    assert_eq!(x_next(Some(Told::Lost), 2), XNext::Say, "started again only once");
    assert_eq!(x_next(Some(Told::Failed), 1), XNext::Say);

    let (mut compositor, boxed) = std::os::unix::net::UnixStream::pair().unwrap();
    let hold = XHold::new(boxed);
    assert_eq!(hold.told(0), None, "a program that ended well waited");
    compositor.write_all(Told::Lost.line().as_bytes()).unwrap();

    assert_eq!(hold.told(1), Some(Told::Lost));
    compositor.write_all(Told::Failed.line().as_bytes()).unwrap();
    compositor.write_all(Told::Lost.line().as_bytes()).unwrap();
    let end = std::time::Instant::now() + X_WORD_WAIT;
    while hold.told(1) != Some(Told::Failed) && std::time::Instant::now() < end {}
    assert_eq!(hold.told(1), Some(Told::Failed), "would not start was outweighed");

    drop(hold);
    let mut rest = Vec::new();
    use std::io::Read;
    compositor.set_read_timeout(Some(std::time::Duration::from_secs(2))).unwrap();
    assert_eq!(compositor.read_to_end(&mut rest).unwrap(), 0);
}

fn set(v: &[&str]) -> HashSet<String> {
    v.iter().map(|s| s.to_string()).collect()
}

#[test]
fn a_name_no_package_has_is_answered_by_the_packages_that_provide_it() {
    let shown = "Package: dbus-session-bus\nVersions: \n\nReverse Depends: \n  zoom,dbus-session-bus\n\
                 Dependencies: \nProvides: \nReverse Provides: \ndbus-x11 1.16.2-2 (= )\n\
                 dbus-user-session 1.16.2-2 (= )\ndbus-x11 1.16.2-1 (= )\n";
    assert_eq!(providers_in(shown), vec!["dbus-x11", "dbus-user-session"]);
    assert!(providers_in("Package: zoom\nReverse Provides: \n").is_empty());
    assert!(providers_in("").is_empty());
}

#[test]
fn a_launcher_starts_it_boxed() {
    let text = "[Desktop Entry]\nName=Foot\nExec=foot %U\nTryExec=foot\nIcon=foot\n[Desktop Action new]\nExec=foot --new\n";
    let out = launcher("foot", text, false, |i| (i == "foot").then(|| PathBuf::from("/f/foot.svg")));
    assert_eq!(out, "[Desktop Entry]\nX-Trindle-Boxed=foot\nName=Foot\nExec=trindle-boxed foot foot %U\n\
                     Icon=/f/foot.svg\n[Desktop Action new]\nExec=trindle-boxed foot foot --new\n");
}

#[test]
fn what_came_with_trindle_is_told_from_what_the_person_added() {
    let offered = vec!["example-terminal".to_string(), "example-terminal:terminal".to_string(), "example-notes".to_string()];
    assert!(came_with_trindle("example-terminal", false, &offered));
    assert!(came_with_trindle("example-notes", false, &offered));
    assert!(!came_with_trindle("example-notes", true, &offered), "put in again from a file of their own");
    assert!(!came_with_trindle("example-editor", false, &offered), "opened as a .deb file before the mark was kept");
    assert!(!came_with_trindle("example-editor", true, &[]));
}

#[test]
fn a_folder_s_size_is_its_plain_files_and_follows_no_link() {
    let dir = std::env::temp_dir().join(format!("trindle-size-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("files/usr/bin")).unwrap();
    std::fs::write(dir.join("version"), "1.0").unwrap();
    std::fs::write(dir.join("files/usr/bin/example"), [0u8; 4096]).unwrap();
    std::os::unix::fs::symlink("/usr", dir.join("files/away")).unwrap();
    assert_eq!(size_of_tree(&dir), 4099);
    assert_eq!(size_of_tree(&dir.join("nothing-here")), 0);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn our_terminal_is_called_terminal() {
    let text = "[Desktop Entry]\nName=Alacritty\nName[de]=Alacritty\nIcon=Alacritty\nExec=alacritty\n\
                Categories=System;TerminalEmulator;\n[Desktop Action New]\nName=New Terminal\nExec=alacritty\n";
    let out = launcher("alacritty", text, true, |_| Some(PathBuf::from("/a/Alacritty.svg")));
    assert_eq!(out, "[Desktop Entry]\nX-Trindle-Boxed=alacritty\nName=Terminal\nIcon=utilities-terminal\n\
                     Exec=trindle-boxed alacritty alacritty\nCategories=System;TerminalEmulator;\n\
                     [Desktop Action New]\nName=New Terminal\nExec=trindle-boxed alacritty alacritty\n");
    assert!(is_terminal("System;TerminalEmulator;"));
    assert!(!is_terminal("Utility;Calculator;"));
}

#[test]
fn only_a_plugin_allowed_control_gets_the_control_socket() {
    let v = |l: &[&str]| l.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    assert_eq!(engine_for(&v(&["network", "background"])), "org.trindle.boxed");
    assert_eq!(engine_for(&v(&["network", "control"])), "org.trindle.boxed.control");
    assert_eq!(engine_for(&v(&["capture"])), "org.trindle.boxed.capture");
    assert_eq!(engine_for(&v(&["capture", "network", "control"])), "org.trindle.boxed.control.capture");
    assert!(!ALLOWED.contains(&"control") && !ALLOWED.contains(&"capture"));
}

#[test]
fn a_plugin_that_listens_shares_this_computers_network_only_with_the_internet_allowed() {

    let both = walls(&folder("sync"), &["network".into(), "listen".into()], &Gates::default(), &["sync".into()]).join(" ");
    assert!(both.contains("--share-net"), "{both}");
    let only = walls(&folder("sync"), &["listen".into()], &Gates::default(), &["sync".into()]).join(" ");
    assert!(!only.contains("--share-net"), "{only}");
    assert!(!ALLOWED.contains(&"listen"), "a package never has it");
}

#[test]
fn the_copied_files_socket_is_the_only_thing_a_plugin_allowed_copied_is_given() {
    let gates = Gates { copied: Some("/run/user/1000/trindle-boxed/x-1/copied".into()), ..Gates::default() };
    let line = |allowed: &[&str]| {
        let a: Vec<String> = allowed.iter().map(|s| s.to_string()).collect();
        walls(&folder("sync"), &a, &gates, &["sync".into()])
    };

    let shut = line(&["network", "files"]).join(" ");
    assert!(!shut.contains("copied") && shut.contains("--unsetenv TRINDLE_COPIED_SOCKET"), "{shut}");

    let open = line(&["copied"]);
    let at = open.windows(3).position(|w| w[0] == "--ro-bind" && w[2].ends_with("/trindle-copied")).expect("bound");
    assert_eq!(open[at + 1], "/run/user/1000/trindle-boxed/x-1/copied");
    assert!(open.join(" ").contains(&format!("--setenv TRINDLE_COPIED_SOCKET {}", open[at + 2])));
    let without_files = open.join(" ");
    assert!(!without_files.contains("--bind-try") && !without_files.contains("--share-net"), "{without_files}");
    assert!(!ALLOWED.contains(&"copied"), "a package never has it");
}

#[test]
fn nothing_is_let_through_until_allowed() {
    let shut = walls(&folder("foot"), &[], &Gates::default(), &["foot".into()]).join(" ");
    assert!(!shut.contains("--share-net") && !shut.contains("pipewire") && !shut.contains("pulse"), "{shut}");
    assert!(shut.contains("--unsetenv DBUS_SESSION_BUS_ADDRESS"), "{shut}");
    assert!(shut.contains("--unsetenv WAYLAND_DISPLAY"), "no socket given, none let through: {shut}");
    assert!(shut.ends_with("-- foot"), "{shut}");
    let gates = Gates { pulse: Some("/run/guard".into()), ..Gates::default() };
    let open = walls(&folder("foot"), &["network".into(), "sound".into()], &gates, &["foot".into()]).join(" ");
    assert!(open.contains("--share-net"), "{open}");

    assert!(open.contains("--ro-bind /run/guard ") && !open.contains("pipewire-0"), "{open}");
    let tuned = Gates { pulse: Some("/run/guard".into()), alsa: Some("/run/asound.conf".into()), ..Gates::default() };
    let alsa = walls(&folder("foot"), &["sound".into()], &tuned, &["foot".into()]).join(" ");
    assert!(alsa.contains("--ro-bind /run/asound.conf /etc/trindle-asound.conf --setenv ALSA_CONFIG_PATH /etc/trindle-asound.conf"), "{alsa}");
    let mic = walls(&folder("foot"), &["sound".into(), "microphone".into()], &gates, &["foot".into()]).join(" ");
    assert!(mic.contains("pipewire-0") && !mic.contains("/run/guard"), "{mic}");
}

#[test]
fn the_older_way_of_drawing_only_when_allowed() {

    std::env::set_var("DISPLAY", ":7");
    let own = Gates { x11: Some(12), ..Gates::default() };
    let shut = walls(&folder("xterm"), &[], &own, &["xterm".into()]).join(" ");
    assert!(!shut.contains(".X11-unix") && shut.contains("--unsetenv DISPLAY"), "{shut}");
    let none = walls(&folder("xterm"), &["x11".into()], &Gates::default(), &["xterm".into()]).join(" ");
    assert!(!none.contains(".X11-unix") && none.contains("--unsetenv DISPLAY"), "{none}");
    let open = walls(&folder("xterm"), &["x11".into()], &own, &["xterm".into()]).join(" ");
    assert!(open.contains("--ro-bind /tmp/.X11-unix/X12 /tmp/.X11-unix/X12 --setenv DISPLAY :12"), "{open}");
    assert!(!open.contains(":7") && !open.contains("/tmp/.X11-unix /tmp/.X11-unix"), "{open}");
    assert!(!open.contains("--unsetenv DISPLAY"), "{open}");
}

#[test]
fn files_are_the_own_folders_never_the_hidden_ones() {
    let at = folder("foot");
    let a = walls(&at, &["files".into()], &Gates::default(), &["foot".into()]);
    let h = home();
    let binds: Vec<(&str, &str)> = a.windows(3).filter(|w| w[0] == "--bind" || w[0] == "--bind-try").map(|w| (w[1].as_str(), w[2].as_str())).collect();
    assert!(binds.contains(&(at.join("home").to_str().unwrap(), h.to_str().unwrap())), "its own home: {binds:?}");
    for (from, to) in &binds {
        assert!(*from != h.to_str().unwrap(), "the whole home is never let in: {binds:?}");
        let rel = Path::new(to).strip_prefix(&h).ok();
        if let Some(rel) = rel.filter(|r| !r.as_os_str().is_empty()) {
            let top = rel.components().next().unwrap().as_os_str().to_string_lossy().into_owned();
            assert!(OWN_FOLDERS.contains(&top.as_str()), "{to} is not one of the person's own folders");
        }
    }
}

#[test]
fn every_folder_lent_is_bound_for_files() {
    let at = folder("foot");
    let a = walls(&at, &["files".into()], &Gates::default(), &["foot".into()]);
    let h = home();
    for name in ["Documents", "Downloads", "Pictures", "Music", "Videos", "Desktop"] {
        let p = h.join(name).to_string_lossy().into_owned();
        assert!(a.windows(3).any(|w| w[0] == "--bind-try" && w[1] == p && w[2] == p), "{name} is not bound: {}", a.join(" "));
    }
    assert_eq!(OWN_FOLDERS.len(), 6, "six folders are lent");
    let shut = walls(&at, &[], &Gates::default(), &["foot".into()]);
    assert!(!shut.iter().any(|x| x.ends_with("/Desktop")), "nothing without files: {}", shut.join(" "));
}

#[test]
fn nothing_but_the_exact_word_files_lets_the_folders_in_and_never_the_hidden_ones() {

    let at = folder("foot");
    let h = home().to_string_lossy().into_owned();
    for words in [&["file"][..], &["Files"], &["files "], &["files;network"], &["fileS"], &["*"], &["home"], &["~"], &["/"]] {
        let words: Vec<String> = words.iter().map(|w| w.to_string()).collect();
        let a = walls(&at, &words, &Gates::default(), &["foot".into()]);
        assert!(!a.iter().any(|x| x.ends_with("/Desktop") || x.ends_with("/Documents")), "{words:?}: {}", a.join(" "));
    }
    let a = walls(&at, &["files".into()], &Gates::default(), &["foot".into()]);

    for w in a.windows(3).filter(|w| w[0] == "--bind-try") {
        assert_eq!(w[1], w[2]);
        let rel = Path::new(&w[1]).strip_prefix(&h).expect("inside the home");
        assert!(OWN_FOLDERS.iter().any(|f| Path::new(f) == rel), "{rel:?}");
    }
    assert!(!a.iter().any(|x| x.contains("/.ssh") || x.contains("/.gnupg") || x.contains("/.config")), "{}", a.join(" "));
}

#[test]
fn only_the_graphics_part_of_sys() {
    let real = PathBuf::from("/sys/devices/pci0000:00/0000:00:02.0/drm/card1");
    let render = PathBuf::from("/sys/devices/pci0000:00/0000:00:02.0/drm/renderD128");
    let a = sys_args_for(&[("card1".into(), "226:1".into(), real), ("renderD128".into(), "226:128".into(), render)]);
    let text = a.join(" ");
    assert!(!a.windows(3).any(|w| w[0] == "--ro-bind" && w[1] == "/sys"), "{text}");
    assert_eq!(a.windows(2).filter(|w| w[0] == "--ro-bind" && w[1] == "/sys/devices/pci0000:00/0000:00:02.0").count(), 1, "{text}");
    assert!(text.contains("--symlink /sys/devices/pci0000:00/0000:00:02.0/drm/renderD128 /sys/dev/char/226:128"), "{text}");
    assert!(text.contains("/sys/class/drm/card1") && text.contains("/sys/bus/pci/devices/0000:00:02.0"), "{text}");
    assert!(text.contains("/sys/devices/system/cpu") && !text.contains("/sys/class/net"), "{text}");
    let none = walls(&folder("foot"), &[], &Gates::default(), &["foot".into()]);
    assert!(!none.windows(2).any(|w| w[1] == "/sys"), "{}", none.join(" "));
}

#[test]
fn the_system_call_filter_shuts_new_user_namespaces() {
    let gates = Gates { seccomp: Some(9), sync: Some(11), ..Gates::default() };
    let a = walls(&folder("foot"), &[], &gates, &["foot".into()]).join(" ");
    assert!(a.contains("--unshare-user --disable-userns --seccomp 9") && a.contains("--sync-fd 11"), "{a}");
}

#[test]
fn each_app_the_image_carries_is_offered_once() {
    let available = vec![("alacritty".to_string(), PathBuf::from("/a.deb")), ("foot".into(), PathBuf::from("/f.deb"))];
    assert_eq!(to_offer(&available, &[]).len(), 2);
    let left = to_offer(&available, &["alacritty".into()]);
    assert_eq!(left, vec![("foot".to_string(), PathBuf::from("/f.deb"))]);
    assert!(to_offer(&available, &["alacritty".into(), "foot".into()]).is_empty(), "taken away, never back");
}

#[test]
fn what_a_package_wants_is_worked_out_from_itself() {
    let none = HashSet::new();

    assert_eq!(wants_from("System;TerminalEmulator;", &set(&["libfontconfig.so.1"]), &set(&["libfontconfig.so.1"]), true, ""),
               ["computer"]);

    let gtk = set(&["libgtk-3.so.0", "libwayland-client.so.0", "libX11.so.6"]);
    assert!(wants_from("Utility;Calculator;", &set(&["libgtk-3.so.0"]), &gtk, true, "").is_empty());

    assert_eq!(wants_from("Graphics;", &set(&["libX11.so.6"]), &set(&["libX11.so.6"]), true, ""), ["files", "x11"]);

    assert_eq!(wants_from("", &none, &none, false, "python3, python3-tk"), ["x11"]);
    assert!(wants_from("", &none, &none, false, "python3").is_empty());
    let browser = wants_from("Network;WebBrowser;", &set(&["libgtk-3.so.0", "libpulse.so.0"]), &gtk, true, "");
    assert_eq!(browser, ["network", "sound", "files"]);
    assert_eq!(wants_from("AudioVideo;Audio;Recorder;", &none, &gtk, true, ""), ["sound", "microphone", "files"]);
}

#[test]
fn a_terminal_outside_its_box_finds_its_own_programs() {
    let at = std::env::temp_dir().join(format!("trindle-plain-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&at);
    std::fs::create_dir_all(at.join("files/usr/bin")).unwrap();
    std::fs::copy("/bin/true", at.join("files/usr/bin/term")).unwrap();
    let c = plain_command(&at, &["term".into(), "-e".into(), "htop".into()]);
    let args: Vec<String> = c.get_args().map(|a| a.to_string_lossy().into_owned()).collect();
    assert_eq!(c.get_program(), loader());
    assert_eq!(args[0], "--library-path");
    assert!(args[1].starts_with(&at.join("files/usr/lib").to_string_lossy().into_owned()));
    assert_eq!(&args[2..], ["--argv0", "term", &at.join("files/usr/bin/term").to_string_lossy(), "-e", "htop"]);
    let env: Vec<(String, String)> = c.get_envs().filter_map(|(k, v)| Some((k.to_string_lossy().into_owned(), v?.to_string_lossy().into_owned()))).collect();
    let path = &env.iter().find(|(k, _)| k == "PATH").unwrap().1;
    assert!(path.ends_with(&format!("{}/files/usr/sbin", at.display())), "{path}");
    assert!(!env.iter().any(|(k, _)| k == "LD_LIBRARY_PATH"), "nothing passed on to what it starts");

    let mut c = plain_command(&at, &["term".into()]);
    assert!(c.status().unwrap().success());
    let _ = std::fs::remove_dir_all(&at);
}

#[test]
#[ignore = "needs real packages named by TRINDLE_BOX_CYCLE: run by hand"]
fn real_packages_go_in_run_and_come_out() {
    let Ok(dirs) = std::env::var("TRINDLE_BOX_CYCLE") else { return };
    let home = crate::common::files::home();
    assert!(home.to_string_lossy().contains("boxcycle"), "only ever in a scratch home");
    assert!(data_home().starts_with(&home), "XDG_DATA_HOME must be inside the scratch home");
    let run: Vec<String> = std::env::var("TRINDLE_BOX_CYCLE_RUN").unwrap_or_default()
        .split(' ').filter(|s| !s.is_empty()).map(String::from).collect();
    let du = |p: &[PathBuf]| {
        let out = Command::new("du").arg("-sc").arg("--block-size=1M").args(p).output().unwrap();
        String::from_utf8_lossy(&out.stdout).lines().last().unwrap_or("").split_whitespace().next().unwrap_or("").to_string()
    };
    let mut names = Vec::new();
    for dir in dirs.split(':') {
        let dir = Path::new(dir);
        for deb in debs_in(dir) {
            let started = std::time::Instant::now();
            let done = install_with(&deb, Some(&dir.join("parts")), |_| {}).unwrap();
            let parts = lines_at(&folder(&done.name).join("parts")).len();
            println!("{}: installed in {:?}, {parts} parts, {} MB, missing {:?}, wants {:?}",
                done.name, started.elapsed(), du(&[folder(&done.name)]), done.missing, wants(&done.name));
            assert!(!launchers_of(&done.name).is_empty() || done.name != "alacritty");
            names.push(done.name);
        }
    }
    println!("all together: {} MB", du(&[folders()]));
    if let Some(program) = run.first() {
        let name = names.iter().find(|n| folder(n).join("files/usr/bin").join(program).is_file()).unwrap();
        let started = std::time::Instant::now();
        let code = run_boxed(&folder(name), name, &[], &run);
        println!("{name} boxed: {code} in {:?}", started.elapsed());
        assert_eq!(code, 0);
        let mut plain = plain_command(&folder(name), &run);
        assert!(plain.status().unwrap().success(), "and outside its box");
    }
    if std::env::var_os("TRINDLE_BOX_CYCLE_KEEP").is_some() {
        return;
    }
    for (i, name) in names.iter().enumerate() {
        remove(name).unwrap();
        assert!(!folder(name).exists() && launchers_of(name).is_empty());
        let left = std::fs::read_dir(parts::store()).map(|d| d.count()).unwrap_or(0);
        println!("{name} removed, {left} parts left, {} MB", du(&[folders()]));
        if i + 1 == names.len() {
            assert_eq!(left, 0, "the last one out takes every part with it");
        }
    }
}

#[test]
fn a_qt_program_gets_its_wayland_add_on_in_its_own_folder_and_is_told_to_use_wayland() {
    let names = |l: &[&str]| l.iter().map(|s| s.to_string()).collect::<HashSet<String>>();
    let control = "Package: example-viewer\nVersion: 1.0\nDepends: libqt6widgets6, libc6\nRecommends: example-viewer-data, qt6-gtk-platformtheme | qt6-style\n";
    let all = |g: Vec<Vec<String>>| g.into_iter().flatten().collect::<Vec<_>>();

    let six = all(parts::recommended_extras(control, &names(&["libqt6widgets6", "libqt6core6t64", "libc6"])));
    assert_eq!(six, ["example-viewer-data", "qt6-gtk-platformtheme", "qt6-style", "qt6-wayland"]);

    let five = all(parts::recommended_extras("Package: old\nVersion: 1\n", &names(&["libqt5core5t64"])));
    assert_eq!(five, ["qtwayland5"]);
    let both = all(parts::recommended_extras("Package: x\nVersion: 1\n", &names(&["libqt5gui5t64", "qt6-base-dev"])));
    assert_eq!(both, ["qt6-wayland", "qtwayland5"]);

    assert!(parts::recommended_extras("Package: x\nVersion: 1\n", &names(&["libgtk-3-0", "libc6"])).is_empty());
    assert!(parts::recommended_extras("Package: x\nVersion: 1\n", &names(&["libqt6core6t64", "qt6-wayland"])).is_empty());
    let named = all(parts::recommended_extras("Package: x\nRecommends: qt6-wayland\n", &names(&["libqt6core6t64"])));
    assert_eq!(named, ["qt6-wayland"]);

    let gates = Gates { wayland: Some("/run/user/1000/w".into()), ..Gates::default() };
    let line = walls(&folder("viewer"), &[], &gates, &["viewer".into()]).join(" ");
    assert!(line.contains("--setenv QT_QPA_PLATFORM wayland;xcb"), "{line}");
    let none = walls(&folder("viewer"), &[], &Gates::default(), &["viewer".into()]).join(" ");
    assert!(!none.contains("QT_QPA_PLATFORM"), "{none}");
}

#[test]
fn a_box_reaches_the_accessibility_bus_only_when_allowed() {
    let at = std::env::temp_dir().join(format!("trindle-a11y-walls-{}", std::process::id()));
    let gates = Gates { a11y: Some(("/run/user/1000/at-spi/bus_0".into(), "/run/user/1000/trindle-screen-reader".into(), "/run/user/1000/trindle-voice".into())), ..Gates::default() };
    let run = |allowed: &[&str], gates: &Gates| {
        let allowed: Vec<String> = allowed.iter().map(|s| s.to_string()).collect();
        walls(&at, &allowed, gates, &["x".to_string()]).join(" ")
    };
    let on = run(&["sound", "accessibility"], &gates);
    assert!(on.contains("--ro-bind /run/user/1000/at-spi/bus_0 /run/user/1000/at-spi/bus_0"), "{on}");
    assert!(on.contains("--setenv AT_SPI_BUS_ADDRESS unix:path=/run/user/1000/at-spi/bus_0"), "{on}");
    assert!(on.contains("--bind /run/user/1000/trindle-screen-reader /run/user/1000/trindle-screen-reader"), "{on}");
    assert!(on.contains("--ro-bind /run/user/1000/trindle-voice /run/user/1000/trindle-voice"), "{on}");
    assert!(on.contains("--unsetenv DBUS_SESSION_BUS_ADDRESS"));
    for (what, allowed, gates) in [("not allowed", &["sound", "control", "x11", "files"][..], &gates), ("no gates", &["accessibility"], &Gates::default())] {
        let off = run(allowed, gates);
        assert!(!off.contains("at-spi") && !off.contains("trindle-screen-reader") && !off.contains("trindle-voice") && !off.contains("--setenv AT_SPI"), "{what}: {off}");
        assert!(off.contains("--unsetenv AT_SPI_BUS_ADDRESS"), "{what}: {off}");
    }
    for spelled in ["Accessibility", "accessibilty", "accessibility ", "a11y", "read-screen", "accessibility;", "access"] {
        assert!(!run(&[spelled], &gates).contains("at-spi"), "{spelled}");
    }
}
