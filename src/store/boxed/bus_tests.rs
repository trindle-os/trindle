//! The filtered bus: the proxy's rules, its address, its end with the box, and a real proxy in front of a bus of the test's own.

use super::*;
use crate::store::boxed::walls::{BUS_AT, Gates, walls};
use crate::toolkit::bus::testing::own_daemon;
use std::process::Command;
use std::time::Duration;
use zbus::blocking::{connection::Builder, Connection};

fn connect(address: &str) -> Connection {
    Builder::address(address).unwrap().method_timeout(Duration::from_secs(5)).build().unwrap()
}

fn ping(c: &Connection, name: &str) -> zbus::Result<()> {
    c.call_method(Some(name), "/", Some("org.freedesktop.DBus.Peer"), "Ping", &()).map(|_| ())
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-bus-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn scratch_band(dir: &Path) -> Band {
    Band::take(&dir.join("bands")).unwrap()
}

struct Notes;

#[zbus::interface(name = "org.freedesktop.Notifications")]
impl Notes {
    #[allow(clippy::too_many_arguments)]
    fn notify(&self, _app: &str, _replaces: u32, _icon: &str, _summary: &str, _body: &str, _actions: Vec<String>,
              _hints: std::collections::HashMap<String, zbus::zvariant::OwnedValue>, _timeout: i32) -> u32 {
        7
    }
    fn close_notification(&self, _id: u32) {}
    fn get_capabilities(&self) -> Vec<String> {
        vec!["body".into()]
    }
}

fn services(address: &str, more: &[&str]) -> Connection {
    let server = Builder::address(address).unwrap().serve_at(NOTIFY_PATH, Notes).unwrap().build().unwrap();
    for name in [NOTIFICATIONS, WATCHER].iter().chain(more) {
        server.request_name(*name).unwrap();
    }
    server
}

fn call_notes(c: &Connection, member: &str) -> zbus::Result<()> {
    let r = match member {
        "Notify" => c.call_method(Some(NOTIFICATIONS), NOTIFY_PATH, Some(NOTIFICATIONS), member,
                                  &("app", 0u32, "", "s", "b", Vec::<String>::new(), std::collections::HashMap::<String, zbus::zvariant::OwnedValue>::new(), -1i32)),
        "CloseNotification" => c.call_method(Some(NOTIFICATIONS), NOTIFY_PATH, Some(NOTIFICATIONS), member, &(7u32,)),
        _ => c.call_method(Some(NOTIFICATIONS), NOTIFY_PATH, Some(NOTIFICATIONS), member, &()),
    };
    r.map(|_| ())
}

#[test]
fn the_proxy_is_told_exactly_the_allowed_names_and_nothing_broader() {
    let dir = scratch("args");
    let band = scratch_band(&dir);
    let args = proxy_args("unix:path=/run/user/1000/bus", Path::new("/run/user/1000/trindle-boxed/x/bus"), 7, Some(band.pids()));
    assert_eq!(args[..10], ["--fd=7", "unix:path=/run/user/1000/bus", "/run/user/1000/trindle-boxed/x/bus", "--filter",
                           "--call=org.freedesktop.Notifications=org.freedesktop.Notifications.Notify@/org/freedesktop/Notifications",
                           "--call=org.freedesktop.Notifications=org.freedesktop.Notifications.CloseNotification@/org/freedesktop/Notifications",
                           "--call=org.freedesktop.Notifications=org.freedesktop.Notifications.GetCapabilities@/org/freedesktop/Notifications",
                           "--call=org.freedesktop.Notifications=org.freedesktop.Notifications.GetServerInformation@/org/freedesktop/Notifications",
                           "--broadcast=org.freedesktop.Notifications=org.freedesktop.Notifications.*@/org/freedesktop/Notifications",
                           "--talk=org.kde.StatusNotifierWatcher"]);
    let owns = &args[10..];
    assert_eq!(owns.len() as u32, BAND_PIDS * ITEM_NUMBERS * 2);
    for o in owns {
        let name = o.strip_prefix("--own=org.kde.StatusNotifierItem-").or_else(|| o.strip_prefix("--own=org.freedesktop.StatusNotifierItem-"))
            .unwrap_or_else(|| panic!("{o}"));
        let (pid, n) = name.split_once('-').unwrap();
        assert!(pid.parse::<u32>().is_ok_and(|p| band.pids().contains(&p)) && n.parse::<u32>().is_ok_and(|n| (1..=ITEM_NUMBERS).contains(&n)), "{o}");
    }
    let all = args.join(" ");
    assert!(!all.contains("--see"), "{all}");
    assert_eq!(args.iter().filter(|a| a.starts_with("--broadcast=") || a.starts_with("--call=")).count(), 5);
    assert!(!args.iter().any(|a| (a.starts_with("--own=") || a.starts_with("--talk=")) && a.contains('*')), "no wildcard name");
    assert_eq!(args.iter().filter(|a| a.starts_with("--talk=")).count(), 1);
    let none = proxy_args("unix:path=/x", Path::new("/y"), 7, None);
    assert!(!none.iter().any(|a| a.starts_with("--own=")), "no band: no tray name at all");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn every_box_running_at_once_has_a_band_of_its_own() {
    let dir = scratch("bands");
    let held: Vec<Band> = (0..BANDS).map(|_| scratch_band(&dir)).collect();
    assert!(Band::take(&dir.join("bands")).is_none(), "every band is held");
    for (i, a) in held.iter().enumerate() {
        assert!(*a.pids().start() >= BAND_FIRST && a.pids().count() as u32 == BAND_PIDS, "{:?}", a.pids());
        for b in &held[i + 1..] {
            assert!(!a.pids().any(|p| b.pids().contains(&p)), "{:?} and {:?} share none", a.pids(), b.pids());
        }
    }
    let freed = held[3].pids();
    drop(held);

    let again = (0..200).find_map(|_| Band::take(&dir.join("bands")).filter(|b| *b.pids().start() == BAND_FIRST)
        .or_else(|| { std::thread::sleep(Duration::from_millis(10)); None })).unwrap();
    assert_eq!(*again.pids().start(), BAND_FIRST, "all given back: {freed:?} among them");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn only_the_sessions_own_socket_file_is_put_in_front() {
    let dir = scratch("address");
    let sock = dir.join("bus");
    let _listener = std::os::unix::net::UnixListener::bind(&sock).unwrap();
    let path = sock.to_string_lossy().into_owned();
    assert_eq!(session_address(Some(&format!("unix:path={path}"))), Ok(format!("unix:path={path}")));
    assert_eq!(session_address(Some(&format!("unix:path={path},guid=abc"))), Ok(format!("unix:path={path}")), "extra keys are dropped");
    for bad in [None, Some(""), Some("unix:abstract=/tmp/dbus-x"), Some("tcp:host=localhost,port=1"), Some("unix:path=relative/bus"),
                Some("unix:path=/nowhere/at/all"), Some(&*format!("unix:path={path};unix:abstract=x")), Some("unix:path=/run/user/%31/bus"),
                Some(&*format!("unix:path={}", dir.display()))] {
        assert!(session_address(bad).is_err(), "{bad:?}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_missing_or_failing_proxy_is_an_error_not_a_hang_and_leaves_no_process() {
    let dir = scratch("missing");
    let missing = launch("trindle-no-such-proxy", "unix:path=/x", &dir.join("bus"), None).err().unwrap();
    assert!(missing.contains("is not installed"), "{missing}");
    let ended = launch("true", "unix:path=/x", &dir.join("bus"), None).err().unwrap();
    assert!(ended.contains("ended as it started"), "{ended}");
    let none = Bus::start_at(None, &dir, None).err().unwrap();
    assert!(none.contains("not on a socket file"), "{none}");
    assert!(!dir.join("bus").exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_box_is_given_the_filtered_socket_inside_its_own_runtime_folder_or_no_bus() {
    let on = Gates { bus: Some("/run/user/1000/trindle-boxed/a-1/bus".into()), ..Gates::default() };
    let a = walls(Path::new("/data/x"), &[], &on, &["true".to_string()]);
    let at = a.windows(3).find(|w| w[0] == "--ro-bind" && w[1] == "/run/user/1000/trindle-boxed/a-1/bus").map(|w| w[2].clone()).unwrap();
    assert!(at.ends_with(&format!("/{BUS_AT}")) && !at.starts_with("/etc") && !at.starts_with("/usr"), "{at}");
    assert!(a.windows(3).any(|w| w == ["--setenv", "DBUS_SESSION_BUS_ADDRESS", &format!("unix:path={at}")]), "{a:?}");
    assert!(!a.windows(2).any(|w| w == ["--unsetenv", "DBUS_SESSION_BUS_ADDRESS"]));
    let off = walls(Path::new("/data/x"), &[], &Gates::default(), &["true".to_string()]);
    assert!(off.windows(2).any(|w| w == ["--unsetenv", "DBUS_SESSION_BUS_ADDRESS"]), "no bus given: none is set");
    assert!(!off.iter().any(|x| x.ends_with("/bus") || x.contains("dbus")), "{off:?}");
}

#[test]
fn the_proxy_ends_with_whoever_holds_it_even_if_that_is_killed() {
    crate::common::needs::programs(&["xdg-dbus-proxy", "dbus-daemon"]);
    let d = own_daemon();
    let dir = scratch("ends");
    let address = session_address(Some(&d.1)).unwrap();
    let (mut child, alive) = launch("xdg-dbus-proxy", &address, &dir.join("bus"), None).unwrap();
    assert!(child.try_wait().unwrap().is_none(), "running while held");
    drop(alive);
    let ended = (0..200).any(|_| { std::thread::sleep(Duration::from_millis(20)); child.try_wait().unwrap().is_some() });
    assert!(ended, "the closing of the holder's end stopped it");
    let bus = Bus::start_at(Some(&d.1), &dir, None).unwrap();
    let pid = bus.pid();
    assert!(bus.socket().starts_with(dir.to_str().unwrap()));
    drop(bus);
    assert!(!Path::new(&format!("/proc/{pid}")).exists(), "dropped with the box: gone and collected");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_box_reaches_notifications_and_the_tray_and_no_other_name_nor_the_other_box() {
    crate::common::needs::programs(&["xdg-dbus-proxy", "dbus-daemon"]);
    let d = own_daemon();
    assert!(d.1.starts_with("unix:path=/tmp/dbus-"), "the test's own bus: {}", d.1);
    assert_ne!(std::env::var("DBUS_SESSION_BUS_ADDRESS").ok().as_deref(), Some(d.1.as_str()), "never the real bus");
    let dir = scratch("e2e");
    let server = services(&d.1, &["org.example.Secret", "org.freedesktop.secrets"]);
    let (one, two) = (dir.join("one"), dir.join("two"));
    std::fs::create_dir_all(&one).unwrap();
    std::fs::create_dir_all(&two).unwrap();
    let (band1, band2) = (scratch_band(&dir), scratch_band(&dir));
    let (bus1, bus2) = (Bus::start_at(Some(&d.1), &one, Some(&band1)).unwrap(), Bus::start_at(Some(&d.1), &two, Some(&band2)).unwrap());
    let (app, other) = (connect(&format!("unix:path={}", bus1.socket())), connect(&format!("unix:path={}", bus2.socket())));
    let _ = (app.object_server(), other.object_server());

    call_notes(&app, "Notify").expect("a note gets through");
    call_notes(&app, "GetCapabilities").expect("so does asking what is shown");
    call_notes(&app, "CloseNotification").expect("so does closing one: the ring closes only the sender's own (ring/notify.rs)");
    ping(&app, WATCHER).expect("the tray's watcher gets through");
    for refused in ["org.example.Secret", "org.freedesktop.secrets", "org.freedesktop.portal.Desktop"] {
        assert!(ping(&app, refused).is_err(), "{refused} is not reachable");
    }
    let (first1, first2) = (*band1.pids().start(), *band2.pids().start());
    for refused in ["org.example.Evil".to_string(), "org.kde.Evil".into(), format!("org.kde.StatusNotifierItem-{first1}-5"),
                    "org.kde.StatusNotifierItem-2-1".into(), format!("org.kde.StatusNotifierItem-{first2}-1"),
                    format!("org.freedesktop.StatusNotifierItem-{first2}-1"), "org.freedesktop.Notifications.Evil".into()] {
        assert!(app.request_name(refused.as_str()).is_err(), "{refused} cannot be owned");
    }
    for family in ["org.kde.StatusNotifierItem", "org.freedesktop.StatusNotifierItem"] {
        let (mine, theirs) = (format!("{family}-{first1}-1"), format!("{family}-{first2}-1"));
        app.request_name(mine.as_str()).expect("its own tray name, in either family");
        other.request_name(theirs.as_str()).expect("the other box's, at the same time");
        ping(&server, &mine).expect("the host reaches the item by its name");
        ping(&server, &theirs).expect("and the other box's");
        assert!(ping(&app, &theirs).is_err(), "one box cannot call another box's tray icon");
        assert!(ping(&other, &mine).is_err(), "nor the other way round");
    }
    ping(&server, app.unique_name().unwrap().as_str()).expect("and by its number");
    let names: Vec<String> = app.call_method(Some("org.freedesktop.DBus"), "/org/freedesktop/DBus", Some("org.freedesktop.DBus"), "ListNames", &())
        .unwrap().body().deserialize().unwrap();
    assert!(!names.iter().any(|n| n == "org.example.Secret" || n == "org.freedesktop.secrets" || n.contains(&first2.to_string())), "{names:?}");

    let rule = zbus::MatchRule::builder().msg_type(zbus::message::Type::Signal).interface(NOTIFICATIONS).unwrap().build();
    let mut heard = zbus::blocking::MessageIterator::for_match_rule(rule, &app, Some(4)).unwrap();
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        if let Some(Ok(m)) = heard.next() {
            let _ = tx.send(m.header().member().map(|m| m.to_string()));
        }
    });
    std::thread::sleep(Duration::from_millis(200));
    server.emit_signal(None::<&str>, NOTIFY_PATH, NOTIFICATIONS, "NotificationClosed", &(7u32, 2u32)).unwrap();
    assert_eq!(rx.recv_timeout(Duration::from_secs(5)).ok().flatten().as_deref(), Some("NotificationClosed"));
    drop((app, other, bus1, bus2));
    let _ = std::fs::remove_dir_all(&dir);
}

const INSIDE: &str = "TRINDLE_BUS_TEST_INSIDE";

#[test]
fn a_real_deb_box_and_a_real_appimage_box_get_the_filtered_bus_and_a_plugin_box_gets_none() {
    crate::common::needs::programs(&["bwrap", "xdg-dbus-proxy", "dbus-daemon", "gdbus"]);
    let call = |name: &str, path: &str, method: &str, args: &str| format!("gdbus call --session --dest {name} --object-path {path} --method {method} {args}");
    let own = |name: &str| format!("{} 2>/dev/null | grep -q 'uint32 1,'", call("org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus.RequestName", &format!("{name} 0")));
    let notes = |method: &str, args: &str| format!("{} >/dev/null 2>&1", call(NOTIFICATIONS, NOTIFY_PATH, &format!("{NOTIFICATIONS}.{method}"), args));
    let script = format!(
        "test \"$DBUS_SESSION_BUS_ADDRESS\" = \"unix:path=$XDG_RUNTIME_DIR/bus\" || exit 10; \
         {} || exit 11; if {}; then exit 12; fi; ls /tmp | grep -q dbus && exit 13; \
         {} || exit 14; {} || exit 15; if {}; then exit 16; fi; {} || exit 17; \
         echo $$ > \"$HOME/started-at\"; sleep 1; exit 0",
        notes("GetCapabilities", ""), call("org.example.Secret", "/", "org.freedesktop.DBus.Peer.Ping", ">/dev/null 2>&1"),
        own("org.kde.StatusNotifierItem-$$-1"), own("org.freedesktop.StatusNotifierItem-$$-2"), own("org.kde.StatusNotifierItem-2-1"),
        notes("CloseNotification", "'uint32 7'"));
    let data = std::env::temp_dir().join(format!("trindle-bus-box-{}", std::process::id()));
    if std::env::var_os(INSIDE).is_none() {
        let d = own_daemon();
        assert!(d.1.starts_with("unix:path=/tmp/dbus-"), "the test's own bus: {}", d.1);
        let _server = services(&d.1, &["org.example.Secret"]);
        let _ = std::fs::remove_dir_all(&data);
        let out = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "store::boxed::bus::bus_tests::a_real_deb_box_and_a_real_appimage_box_get_the_filtered_bus_and_a_plugin_box_gets_none", "--nocapture"])
            .env(INSIDE, "1").env("DBUS_SESSION_BUS_ADDRESS", &d.1).env("XDG_DATA_HOME", &data).env_remove("WAYLAND_DISPLAY").output().unwrap();
        let _ = std::fs::remove_dir_all(&data);
        let said = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
        assert!(out.status.success() && said.contains("1 passed"), "{said}");
        return;
    }
    let data = PathBuf::from(std::env::var_os("XDG_DATA_HOME").unwrap());
    assert!(data.to_string_lossy().contains("trindle-bus-box") && crate::common::files::data_home() == data, "only ever in the scratch folder");
    let sh = |code: &str| ["sh", "-c", code].map(String::from).to_vec();

    let deb = crate::store::boxed::folder("probe-app");
    std::fs::create_dir_all(deb.join("home")).unwrap();
    std::fs::write(deb.join("version"), "1.0\n").unwrap();
    let mut args = vec!["trindle-boxed".to_string(), "probe-app".to_string()];
    args.extend(sh(&script));

    let image = data.join("probe.AppImage");
    std::fs::write(&image, format!("#!/bin/sh\n{script}\n")).unwrap();
    std::fs::set_permissions(&image, std::os::unix::fs::PermissionsExt::from_mode(0o700)).unwrap();
    let at = crate::store::loose::folder("appimage", "probe");
    crate::store::loose::prepare(&at, &[]).unwrap();
    let both = std::thread::spawn(move || crate::store::loose::appimage(&image, &[]));
    assert_eq!(crate::store::boxed::first_apps::main(args), 0, "a deb in its folder has the filtered bus");
    assert_eq!(both.join().unwrap(), 0, "an AppImage, running at the same time, has it too");
    let started = |home: PathBuf| std::fs::read_to_string(home.join("started-at")).unwrap().trim().parse::<u32>().unwrap();
    let (a, b) = (started(deb.join("home")), started(at.join("home")));
    assert_ne!(a, b, "two boxes at once: two bands");
    for p in [a, b] {
        assert!(p >= BAND_FIRST && (p - BAND_FIRST) % BAND_PIDS == 0, "the app starts at its band's first number: {p}");
    }

    let none = "test -z \"$DBUS_SESSION_BUS_ADDRESS\" && ! test -e \"$XDG_RUNTIME_DIR/bus\" && test $$ -lt 64";
    assert_eq!(crate::store::boxed::run::run_boxed(&deb, "probe-app", &[], &sh(none)), 0, "a box not asked for one has none, nor a band");
}
