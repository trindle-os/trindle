//! The two notices about ordinary programs outside a box: one reached the
//! network, one has the person's files open. Looked for every few seconds
//! on a thread of its own (`common::outside`); said once for each program
//! per login, even when the ring restarts after an update (the list of what
//! was said lives in the login's own runtime folder), and never again once
//! the person pressed Allow. It tells; it stops nothing.

use std::fmt::Write as _;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::time::Duration;

use super::notify;
use crate::common::outside::{self, Kind, Seen, Tables};
use crate::toolkit::event::Loop;
use crate::toolkit::t;

const LOOK: Duration = Duration::from_secs(5);
const FIRST: Duration = Duration::from_secs(10);

const NAMED: usize = 3;

const PREFIX: &str = "outside:";

struct Batch {
    kind: Kind,
    programs: Vec<(String, String)>,
    note: u32,
}

#[derive(Default)]
struct State {
    said: HashSet<(Kind, String)>,
    batches: HashMap<u32, Batch>,
    next: u32,
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

pub fn start(lp: &Loop) {
    let post = lp.post(|seen: Vec<Seen>| heard(&seen));
    std::thread::spawn(move || {
        std::thread::sleep(FIRST);
        loop {
            let (net, files) = (outside::network_on(), outside::files_on());
            if let (true, Some(mine)) = (net || files, outside::my_room(Path::new("/proc"))) {
                let sockets = if net { outside::reaching(&Tables::read(Path::new("/proc/net"))) } else { HashSet::new() };
                let folders = outside::personal_folders(&crate::common::files::home());
                let seen = outside::scan(Path::new("/proc"), &sockets, &folders, &mine, net, files);
                if !seen.is_empty() && !post.send(seen) {
                    return;
                }
            }
            std::thread::sleep(LOOK);
        }
    });
}

fn heard(seen: &[Seen]) {
    let fine = outside::fine();
    let kept = said_path();
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        if s.said.is_empty() {
            if let Some(path) = &kept {
                s.said = said_in(path);
            }
        }
    });
    for kind in [Kind::Network, Kind::Files] {
        let fresh = STATE.with(|s| outside::to_say(seen, kind, &fine, &s.borrow().said));
        if fresh.is_empty() {
            continue;
        }
        STATE.with(|s| s.borrow_mut().said.extend(fresh.iter().map(|f| (kind, f.key.clone()))));
        if let Some(path) = &kept {
            STATE.with(|s| keep_said(path, &s.borrow().said));
        }
        say(kind, &fresh);
    }
}

fn said_path() -> Option<std::path::PathBuf> {
    let dir = std::env::var_os("XDG_RUNTIME_DIR").filter(|d| !d.is_empty())?;
    Some(Path::new(&dir).join("trindle-outside-said"))
}

fn said_in(path: &Path) -> HashSet<(Kind, String)> {
    std::fs::read_to_string(path).unwrap_or_default().lines()
        .filter_map(|l| l.split_once('\t').and_then(|(k, key)| Kind::from_word(k).map(|k| (k, key.to_string()))))
        .collect()
}

fn keep_said(path: &Path, said: &HashSet<(Kind, String)>) {

    let text: String = said.iter().filter(|(_, key)| !key.contains(['\t', '\n']))
        .fold(String::new(), |mut s, (k, key)| { let _ = writeln!(s, "{}\t{key}", k.word()); s });
    let _ = crate::common::files::write_text(path, &text);
}

fn shown(seen: &Seen, launchers: &[crate::common::apps::Entry]) -> String {
    launchers
        .iter()
        .find(|e| !e.program.is_empty() && Path::new(&e.program).file_name().is_some_and(|n| n.to_string_lossy() == seen.exe))
        .map_or_else(|| seen.name.clone(), |e| e.name.clone())
}

pub fn body(names: &[String]) -> String {
    let shown = names.iter().take(NAMED).cloned().collect::<Vec<_>>().join(", ");
    t!("ring-outside-body", names = shown, more = names.len().saturating_sub(NAMED) as i64)
}

fn say(kind: Kind, fresh: &[Seen]) {
    let launchers = crate::common::apps::entries();
    let names: Vec<String> = fresh.iter().map(|s| shown(s, &launchers)).collect();
    let (summary, icon) = match kind {
        Kind::Network => (t!("ring-outside-network"), "network-workgroup-symbolic"),
        Kind::Files => (t!("ring-outside-files"), "folder-symbolic"),
    };
    let token = STATE.with(|s| {
        let mut s = s.borrow_mut();
        s.next += 1;
        s.next
    });
    let note = notify::say_allow("Trindle", icon, summary, &body(&names), &format!("{PREFIX}{token}"), 0);
    let programs = fresh.iter().map(|s| (s.key.clone(), s.name.clone())).collect();
    STATE.with(|s| s.borrow_mut().batches.insert(token, Batch { kind, programs, note }));
}

pub fn allow_pressed(target: &str) {
    allow_to(target, &outside::fine_path());
}

fn allow_to(target: &str, list: &Path) {
    let Some(token) = target.strip_prefix(PREFIX) else {
        crate::plugins::ask_again(target);
        return;
    };
    let Some(batch) = token.parse::<u32>().ok().and_then(|t| STATE.with(|s| s.borrow_mut().batches.remove(&t))) else { return };
    if outside::make_fine_in(list, batch.kind, &batch.programs).is_ok() {
        notify::take_back(batch.note);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seen(name: &str, exe: &str) -> Seen {
        Seen { key: format!("/usr/bin/{exe}"), name: name.into(), exe: exe.into(), network: true, files: false }
    }

    #[test]
    fn what_was_said_is_remembered_past_a_restart_of_the_ring() {
        let path = std::env::temp_dir().join(format!("trindle-outside-said-{}", std::process::id()));
        let said: HashSet<(Kind, String)> = [(Kind::Network, "/usr/bin/node /x/sync.mjs".to_string()), (Kind::Files, "/usr/bin/vlc".to_string())].into();
        keep_said(&path, &said);
        assert_eq!(said_in(&path), said);
        let _ = std::fs::remove_file(&path);
        assert!(said_in(&path).is_empty());
    }

    #[test]
    fn a_notice_names_three_and_counts_the_rest() {
        let names: Vec<String> = ["Firefox", "Steam", "Vlc", "Gimp", "Krita"].iter().map(|s| s.to_string()).collect();
        assert_eq!(body(&names[..1]), "Firefox");
        assert_eq!(body(&names[..3]), "Firefox, Steam, Vlc");
        assert_eq!(body(&names), "Firefox, Steam, Vlc and 2 more");
    }

    #[test]
    fn a_program_is_shown_by_its_launchers_name() {
        let mut launcher = crate::common::favourites::path_entry(Path::new("/usr/share/applications/x.desktop"), None);
        launcher.name = "Firefox".into();
        launcher.program = "/usr/bin/firefox-esr".into();
        assert_eq!(shown(&seen("firefox-esr", "firefox-esr"), std::slice::from_ref(&launcher)), "Firefox");
        assert_eq!(shown(&seen("thing", "thing"), &[launcher]), "thing");
    }

    #[test]
    fn allow_makes_the_programs_fine_once_and_a_stale_press_does_nothing() {
        let top = std::env::temp_dir().join(format!("trindle-outside-ring-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        let list = top.join("outside-fine");
        let batch = Batch { kind: Kind::Network, programs: vec![("/usr/bin/vlc".into(), "vlc".into())], note: 0 };
        STATE.with(|s| s.borrow_mut().batches.insert(7, batch));
        allow_to("outside:7", &list);
        assert_eq!(outside::fine_in(&list).iter().map(|f| f.key.as_str()).collect::<Vec<_>>(), ["/usr/bin/vlc"]);
        allow_to("outside:7", &list);
        allow_to("outside:nonsense", &list);
        assert_eq!(outside::fine_in(&list).len(), 1);
        let _ = std::fs::remove_dir_all(&top);
    }
}
