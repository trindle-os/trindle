//! What counts as a game on screen, and the hold taken and given back on a bus of the test's own.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use super::which::{is_game, parent_of, steam_game, Places, DEEPEST};
use super::{hold_while_playing, mpsc, Duration, Filling, Program, ASK_AGAIN, PROFILES, PROFILES_PATH};
use crate::rig::*;
use crate::state::ClientState;

struct Made<'a> {
    pid: u32,
    parent: u32,
    comm: &'a str,
    args: &'a [&'a str],
    env: &'a [&'a str],
    exe: &'a str,
}

const fn made<'a>(pid: u32, parent: u32, comm: &'a str, args: &'a [&'a str]) -> Made<'a> {
    Made { pid, parent, comm, args, env: &[], exe: "/usr/bin/program" }
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-game-power-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn processes(name: &str, list: &[Made]) -> PathBuf {
    let proc = scratch(name);
    for p in list {
        let dir = proc.join(p.pid.to_string());
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("stat"), format!("{} ({}) S {} 1 1 0 -1", p.pid, p.comm, p.parent)).unwrap();
        std::fs::write(dir.join("cmdline"), p.args.join("\0") + "\0").unwrap();
        std::fs::write(dir.join("environ"), p.env.join("\0") + "\0").unwrap();
        std::os::unix::fs::symlink(p.exe, dir.join("exe")).unwrap();
    }
    proc
}

fn launchers(name: &str, list: &[(&str, &str)]) -> PathBuf {
    let dir = scratch(name);
    for (file, text) in list {
        std::fs::write(dir.join(file), text).unwrap();
    }
    dir
}

const STEAM: u32 = 100;
const REAPER: u32 = 200;
const GAME: u32 = 300;
const SECOND_GAME: u32 = 310;
const FILM: u32 = 400;
const BOTTLE: u32 = 600;
const PROTON_BY_HAND: u32 = 610;
const LUTRIS_GAME: u32 = 620;
const HEROIC_GAME: u32 = 630;
const BROWSER: u32 = 640;

fn desktop(name: &str) -> PathBuf {
    processes(name, &[
        made(STEAM, 1, "steam", &["/app/bin/steam", "-silent"]),
        made(REAPER, STEAM, "reaper", &["reaper", "SteamLaunch", "AppId=570", "--", "proton", "waitforexitandrun"]),
        made(250, REAPER, "wine (x) )", &["sh", "game.sh"]),
        made(GAME, 250, "game", &["./game"]),
        made(SECOND_GAME, REAPER, "other", &["other"]),
        made(FILM, 1, "mpv", &["mpv", "--fs", "film.mkv"]),
        made(500, 501, "loop", &["a"]),
        made(501, 500, "loop", &["b"]),
        Made { exe: "/app/share/bottles/runners/soda/bin/wine64-preloader", ..made(BOTTLE, 1, "game.exe", &["C:\\game.exe"]) },
        Made { exe: "/home/john/proton/files/bin/wine", ..made(PROTON_BY_HAND, 1, "game.exe", &["Z:\\game.exe"]) },
        Made { env: &["HOME=/home/john", "LUTRIS_GAME_UUID=3f1c"], ..made(LUTRIS_GAME, 1, "game", &["./game"]) },
        Made { env: &["HEROIC_APP_NAME=Example", "HEROIC_APP_RUNNER=gog"], ..made(HEROIC_GAME, 1, "game", &["./game"]) },
        Made { env: &["NOT_LUTRIS_GAME_UUID=1", "LUTRIS_GAME_UUIDX"], exe: "/usr/lib/wine-viewer", ..made(BROWSER, 1, "firefox", &["firefox"]) },
    ])
}

fn shown(pid: u32) -> Filling {
    Filling { program: Program::Pid(pid), app_id: String::new() }
}

fn places(proc: PathBuf, apps: PathBuf) -> Places {
    Places { proc, apps: vec![apps, PathBuf::from("/nonexistent")] }
}

#[test]
fn only_steam_s_reaper_and_what_runs_under_it_is_a_steam_game() {
    let proc = desktop("which");
    assert!(steam_game(&proc, GAME), "a game under the reaper");
    assert!(steam_game(&proc, REAPER));
    assert!(!steam_game(&proc, STEAM), "Steam and Big Picture are not a game");
    assert!(!steam_game(&proc, FILM), "a film filling the screen is not a game");
    assert!(!steam_game(&proc, 500), "parents in a ring end");
    assert!(!steam_game(&proc, 999), "a process gone");
    assert!(!steam_game(&proc, 0) && !steam_game(&proc, 1));
    let _ = std::fs::remove_dir_all(proc);
}

#[test]
fn a_reaper_too_far_up_is_not_looked_for() {
    let mut list = vec![made(10, 1, "reaper", &["reaper", "SteamLaunch", "AppId=1"])];
    for pid in 11..(11 + DEEPEST as u32) {
        list.push(made(pid, pid - 1, "child", &["child"]));
    }
    let proc = processes("deep", &list);
    assert!(steam_game(&proc, 10 + DEEPEST as u32 - 1));
    assert!(!steam_game(&proc, 10 + DEEPEST as u32));
    let _ = std::fs::remove_dir_all(proc);
}

#[test]
fn the_parent_is_read_after_the_name_s_last_bracket() {
    assert_eq!(parent_of(b"300 (a) b) (c) S 250 1 1"), Some(250));
    assert_eq!(parent_of(b"300 (game) S"), None);
    assert_eq!(parent_of(b"no bracket"), None);
    assert_eq!(parent_of(b"300 (x) S -4 1"), None);
}

#[test]
fn wine_lutris_and_heroic_games_count_and_nothing_alike_does() {
    let at = places(desktop("others"), launchers("others-apps", &[]));
    for (pid, what) in [(BOTTLE, "Bottles"), (PROTON_BY_HAND, "Proton by hand"), (LUTRIS_GAME, "Lutris"), (HEROIC_GAME, "Heroic"), (GAME, "Steam")] {
        assert!(is_game(&at, Some(pid), ""), "a game from {what} did not count");
    }
    for (pid, what) in [(BROWSER, "a browser"), (FILM, "a film player"), (STEAM, "Steam's own window"), (999, "a process gone")] {
        assert!(!is_game(&at, Some(pid), ""), "{what} counted as a game");
    }
    assert!(!is_game(&at, None, ""));
    let _ = std::fs::remove_dir_all(at.proc);
    let _ = std::fs::remove_dir_all(&at.apps[0]);
}

#[test]
fn an_installed_app_counts_only_when_its_launcher_file_says_game() {
    let apps = launchers("apps", &[
        ("org.example.Game.desktop", "[Desktop Entry]\nType=Application\nCategories=Game;ActionGame;\n"),
        ("example-runner.desktop", "[Desktop Entry]\nStartupWMClass=RunnerX\nCategories=Game\nX-Trindle-AppImage=/home/john/Runner.AppImage\n"),
        ("org.example.Boxed.desktop", "[Desktop Entry]\nX-Flatpak=org.example.Puzzle\nCategories=Game;LogicGame;\n"),
        ("org.example.Films.desktop", "[Desktop Entry]\nCategories=AudioVideo;Video;Player;\n"),
        ("firefox.desktop", "[Desktop Entry]\nCategories=Network;WebBrowser;\n"),
        ("org.example.Office.desktop", "[Desktop Entry]\nCategories=Office;\n[Desktop Action play]\nCategories=Game;\n"),
        ("com.valvesoftware.Steam.desktop", "[Desktop Entry]\nStartupWMClass=Steam\nX-Flatpak=com.valvesoftware.Steam\nCategories=Network;FileTransfer;Game;\n"),
        ("net.lutris.Lutris.desktop", "[Desktop Entry]\nCategories=Game;\n"),
        ("notes.txt", "Categories=Game"),
    ]);
    let at = places(desktop("apps-proc"), apps);
    for id in ["org.example.Game", "org.example.game.desktop", "Game", "RunnerX", "org.example.Puzzle"] {
        assert!(is_game(&at, Some(FILM), id), "{id} did not count");
    }
    for id in ["org.example.Films", "firefox", "org.example.Office", "steam", "com.valvesoftware.Steam", "net.lutris.Lutris", "notes", "unknown", "", "  "] {
        assert!(!is_game(&at, Some(FILM), id), "{id:?} counted as a game");
    }
    let _ = std::fs::remove_dir_all(at.proc);
    let _ = std::fs::remove_dir_all(&at.apps[0]);
}

struct Daemon(Arc<Mutex<Vec<String>>>);

#[zbus::interface(name = "org.freedesktop.UPower.PowerProfiles")]
impl Daemon {
    fn hold_profile(&self, profile: &str, _reason: &str, application_id: &str) -> u32 {
        self.0.lock().unwrap().push(format!("hold {profile} {application_id}"));
        7
    }
    fn release_profile(&self, cookie: u32) {
        self.0.lock().unwrap().push(format!("release {cookie}"));
    }
}

fn serve(address: &str, heard: &Arc<Mutex<Vec<String>>>) -> zbus::blocking::Connection {
    zbus::blocking::connection::Builder::address(address).unwrap()
        .name(PROFILES).unwrap()
        .serve_at(PROFILES_PATH, Daemon(heard.clone())).unwrap()
        .build().unwrap()
}

fn heard_until(heard: &Arc<Mutex<Vec<String>>>, count: usize) -> Vec<String> {
    let start = Instant::now();
    while heard.lock().unwrap().len() < count && start.elapsed() < Duration::from_secs(10) {
        std::thread::sleep(Duration::from_millis(10));
    }
    heard.lock().unwrap().clone()
}

fn holder(address: &str, proc: PathBuf, ask_again: Duration) -> (mpsc::Sender<Vec<Filling>>, std::thread::JoinHandle<()>) {
    assert!(address.starts_with("unix:"), "{address}");
    let (tx, rx) = mpsc::channel();
    let address = address.to_string();
    let thread = std::thread::spawn(move || {
        let at = Places { proc, apps: Vec::new() };
        hold_while_playing(&rx, &at, ask_again, || zbus::blocking::connection::Builder::address(address.as_str())?.build());
    });
    (tx, thread)
}

#[test]
fn one_hold_while_any_game_fills_a_screen_and_given_back_after() {
    let daemon = trindle_toolkit::bus::testing::own_daemon();
    let heard = Arc::new(Mutex::new(Vec::new()));
    let _service = serve(&daemon.1, &heard);
    let (tx, thread) = holder(&daemon.1, desktop("hold"), ASK_AGAIN);
    tx.send(vec![shown(FILM)]).unwrap();
    tx.send(vec![shown(GAME)]).unwrap();
    assert_eq!(heard_until(&heard, 1), ["hold performance trindle"]);
    tx.send(vec![shown(GAME), shown(LUTRIS_GAME)]).unwrap();
    tx.send(vec![shown(SECOND_GAME)]).unwrap();
    tx.send(vec![shown(FILM)]).unwrap();
    assert_eq!(heard_until(&heard, 2)[1..], ["release 7"], "two games still made one hold");
    tx.send(vec![shown(BOTTLE)]).unwrap();
    assert_eq!(heard_until(&heard, 3)[2..], ["hold performance trindle"]);
    drop(tx);
    thread.join().unwrap();
    assert_eq!(heard_until(&heard, 4)[3..], ["release 7"], "a hold was kept after the compositor stopped looking");
}

#[test]
fn with_no_daemon_the_hold_is_asked_for_again_later() {
    let daemon = trindle_toolkit::bus::testing::own_daemon();
    let heard = Arc::new(Mutex::new(Vec::new()));
    let (tx, thread) = holder(&daemon.1, desktop("later"), Duration::from_millis(50));
    tx.send(vec![shown(GAME)]).unwrap();
    std::thread::sleep(Duration::from_millis(200));
    let _service = serve(&daemon.1, &heard);
    assert_eq!(heard_until(&heard, 1), ["hold performance trindle"]);
    std::thread::sleep(Duration::from_millis(200));
    assert_eq!(heard.lock().unwrap().len(), 1, "a hold was asked for twice");
    drop(tx);
    thread.join().unwrap();
}

#[test]
fn only_a_window_filling_a_screen_that_shows_is_looked_at() {
    alone("game_power::tests::only_a_window_filling_a_screen_that_shows_is_looked_at", |dir| {
        let mut rig = Rig::new(dir);
        let programs = |rig: &Rig| rig.state.filling_screens().into_iter().map(|f| f.program).collect::<Vec<_>>();
        let me = Program::Pid(std::process::id());
        let who = rig.join(ClientState::default());
        rig.window(who, false);
        assert_eq!(programs(&rig), []);
        rig.window(who, true);
        assert_eq!(programs(&rig), std::slice::from_ref(&me));
        rig.state.set_workspace_count(2);
        rig.state.go_to_workspace(1);
        rig.state.tidy_after_a_turn();
        assert_eq!(programs(&rig), [], "a game on a workspace not showing");
        rig.state.go_to_workspace(0);
        rig.state.tidy_after_a_turn();
        assert_eq!(programs(&rig), [me]);
        let locker = rig.join(ClientState::default());
        rig.lock_by(locker);
        assert_eq!(programs(&rig), [], "a game behind the lock");
    });
}
