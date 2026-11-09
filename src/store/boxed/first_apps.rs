//! The apps the image carries, put in their own folders at login, and the command that does it.

use crate::toolkit::t;
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use crate::store::parts;
use crate::common::files::data_home;
use super::install::{install_with};
use super::launchers::{is_terminal, put_launchers, take_launchers};
use super::needs::{answered, candidate, debs_in, download, providers};
use super::run::{plain_command, run_boxed};
use super::x11::{run_own_x11};
use super::{FIRST_APPS, allow, allowed, categories, folder, lines_at, mark_asked, version, write_kept_lines};

pub fn to_offer(available: &[(String, PathBuf)], offered: &[String]) -> Vec<(String, PathBuf)> {
    available.iter().filter(|(n, _)| !offered.contains(n)).cloned().collect()
}

pub(super) fn offered_file() -> PathBuf {
    data_home().join("trindle/first-apps")
}

fn let_terminal_out(name: &str, offered: &mut Vec<String>) {
    let note = format!("{name}:terminal");
    if offered.contains(&note) || version(name).is_none() || !is_terminal(&categories(name)) {
        return;
    }
    offered.push(note);
    let _ = write_kept_lines(&offered_file(), t!("common-unreadable-name-box-offered"), offered);
    let _ = allow(name, "computer", true);
    let _ = mark_asked(name);
    take_launchers(name);
    let _ = put_launchers(name, &folder(name).join("files"));
}

pub fn first_apps() -> i32 {
    let available: Vec<(String, PathBuf)> = debs_in(Path::new(FIRST_APPS)).into_iter()
        .map(|d| (parts::field_of(&parts::control_of(&d), "Package"), d))
        .filter(|(n, _)| !n.is_empty())
        .collect();
    let mut offered = lines_at(&offered_file());
    for (name, deb) in to_offer(&available, &offered) {

        offered.push(name.clone());
        let _ = write_kept_lines(&offered_file(), t!("common-unreadable-name-box-offered"), &offered);
        if version(&name).is_some() {
            continue;
        }
        match install_with(&deb, Some(&Path::new(FIRST_APPS).join("parts")), |_| {}) {
            Ok(done) => {
                keep_settings(&name);
                for m in &done.missing {
                    eprintln!("trindle-boxed: {name} is missing {m}");
                }
                let_terminal_out(&name, &mut offered);
                if done.ask && !offered.contains(&format!("{name}:terminal")) {
                    crate::common::run::spawn(&["trindle-boxed", "--ask", &name]);
                }
            }
            Err(e) => eprintln!("trindle-boxed: {name}: {e}"),
        }
    }
    for (name, _) in &available {
        let_terminal_out(name, &mut offered);
    }
    0
}

fn keep_settings(name: &str) {
    let from = crate::common::files::config_home().join(name);
    let to = folder(name).join("home/.config").join(name);
    for e in std::fs::read_dir(&from).into_iter().flatten().flatten() {
        let target = to.join(e.file_name());
        if e.file_type().is_ok_and(|t| t.is_file()) && !target.exists() {
            let _ = std::fs::create_dir_all(&to);
            let _ = crate::common::copies::copy_new(&e.path(), &target);
        }
    }
}

pub fn gather(dir: &Path, names: &[String]) -> i32 {
    let parts_dir = dir.join("parts");
    if std::fs::create_dir_all(&parts_dir).is_err() {
        eprintln!("trindle-boxed: cannot write to {}", dir.display());
        return 1;
    }
    let mut have = parts::base_here();
    let mut waiting: VecDeque<Vec<String>> = VecDeque::new();
    for name in names {
        let Some(c) = candidate(name) else {
            eprintln!("trindle-boxed: {name} is not in this computer's sources");
            return 1;
        };
        if download(name, dir).is_none() {
            eprintln!("trindle-boxed: {name} could not be fetched");
            return 1;
        }
        have.insert(name.clone());
        waiting.extend(parts::needs(&c));
    }
    while let Some(group) = waiting.pop_front() {
        if answered(&group, &have) {
            continue;
        }
        let got = group.iter().find_map(|p| candidate(p).map(|c| (p.clone(), c)))
            .or_else(|| group.iter().flat_map(|p| providers(p)).find_map(|p| candidate(&p).map(|c| (p, c))));
        let Some((part, c)) = got else {
            eprintln!("trindle-boxed: nothing in the sources gives {}", group.join(" or "));
            have.extend(group);
            continue;
        };
        if download(&part, &parts_dir).is_none() {
            eprintln!("trindle-boxed: {part} could not be fetched");
            return 1;
        }
        have.insert(part);
        have.extend(parts::groups(&parts::field_of(&c, "Provides")).into_iter().flatten());
        waiting.extend(parts::needs(&c));
    }
    0
}

pub(super) fn told() -> (&'static str, &'static str) {
    (t!("store-boxed-terminal"), t!("store-boxed-terminal-told"))
}

fn tell_once(at: &Path) {
    let note = at.join("told");
    if note.exists() || std::fs::write(&note, "").is_err() {
        return;
    }
    let Ok(bus) = zbus::blocking::Connection::session() else { return };
    let hints: std::collections::HashMap<&str, zbus::zvariant::Value> = std::collections::HashMap::new();
    let _ = bus.call_method(Some("org.freedesktop.Notifications"), "/org/freedesktop/Notifications",
                            Some("org.freedesktop.Notifications"), "Notify",
                            &("Terminal", 0u32, "utilities-terminal", told().0, told().1, Vec::<&str>::new(), hints, -1i32));
}

pub fn main(args: Vec<String>) -> i32 {
    use std::os::unix::process::CommandExt;
    let arg = |i: usize| args.get(i).map(String::as_str).unwrap_or("");
    match arg(1) {
        "--first" => return first_apps(),
        "--patch" => return crate::store::patch::main(),
        "--ask" if !arg(2).is_empty() => return crate::store::asking::main(arg(2)),
        "--gather" if !arg(2).is_empty() => return gather(Path::new(arg(2)), &args[3..]),
        "--own-x11" if args.len() > 3 => return run_own_x11(arg(2), &args[3..]),
        "--appimage" if !arg(2).is_empty() => return crate::store::loose::appimage(Path::new(arg(2)), &args[3..]),
        "--ai" if !arg(2).is_empty() => return crate::store::fence::main_for(arg(2), &args[3..]),
        "--ai-share" if !arg(2).is_empty() => return crate::store::fence::share_main(arg(2)),
        "--ai-approve" if args.len() > 3 => return crate::store::fence_window::main(arg(2), arg(3)),
        "--restart-parts-after-update" => return crate::session::restart_ask::after_update(),
        "--restart-ask" => return crate::session::restart_ask::main(),
        "--loose" if args.len() > 3 => return crate::store::loose::program(arg(2), &args[3..]),
        _ => {}
    }
    let (Some(name), Some(_)) = (args.get(1), args.get(2)) else {
        eprintln!("trindle-boxed: give a package installed in its own folder, then the command to run");
        return 2;
    };
    if version(name).is_none() {
        eprintln!("trindle-boxed: {name} is not installed in its own folder");
        return 1;
    }
    let allowed = allowed(name);
    if allowed.iter().any(|a| a == "computer") {
        tell_once(&folder(name));
        let e = plain_command(&folder(name), &args[2..]).exec();
        eprintln!("trindle-boxed: could not start {}: {e}", args[2]);
        return 1;
    }
    run_boxed(&folder(name), name, &allowed, &args[2..])
}
