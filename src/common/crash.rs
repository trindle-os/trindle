//! Crash notes: when one of Trindle's programs panics or is killed by a fault,
//! a short note is added to ~/.local/state/trindle/crash-notes.jsonl. The note
//! holds only what the crash-report add-on may send: the kind of crash, the
//! program, the version, and where in Trindle's own code it went wrong. Never
//! the panic message, which could hold a file name or something typed.
//! Trindle itself never sends a note anywhere; they stay on the computer
//! unless the separate add-on is installed. Each note also goes into
//! crashes.jsonl beside it, which the add-on leaves alone: the ring tells the
//! person from it, and an assistant reads it to work out what went wrong.

use std::ffi::CString;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

const MOST: u64 = 64 * 1024;

const FRAMES: usize = 12;

const SIGNALS: &[(libc::c_int, &str)] = &[
    (libc::SIGSEGV, "SIGSEGV"), (libc::SIGABRT, "SIGABRT"), (libc::SIGBUS, "SIGBUS"),
    (libc::SIGILL, "SIGILL"), (libc::SIGFPE, "SIGFPE"), (libc::SIGTRAP, "SIGTRAP"),
    (libc::SIGSYS, "SIGSYS"),
];

static PROGRAM: OnceLock<String> = OnceLock::new();

static READY: OnceLock<([CString; 2], Vec<Vec<u8>>)> = OnceLock::new();

static PANICKED: AtomicBool = AtomicBool::new(false);

fn notes() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").filter(|h| !h.is_empty())?;
    Some(PathBuf::from(home).join(".local/state/trindle/crash-notes.jsonl"))
}

pub fn history() -> Option<PathBuf> {
    notes().map(|n| n.with_file_name("crashes.jsonl"))
}

pub fn watch(program: &str) {
    // SAFETY: getuid cannot fail.
    if unsafe { libc::getuid() } == 0 {
        return;
    }
    let Some(path) = notes() else { return };
    let _ = PROGRAM.set(program.to_string());
    let _ = path.parent().map(fs::create_dir_all);
    let lines = SIGNALS.iter().map(|(_, name)| line("crash", name, None, &[])).collect();
    let Some(kept) = history() else { return };
    let c = |p: &PathBuf| CString::new(p.as_os_str().as_encoded_bytes()).ok();
    let (Some(c_path), Some(c_kept)) = (c(&path), c(&kept)) else { return };
    let _ = READY.set(([c_path, c_kept], lines));
    for (sig, _) in SIGNALS {
        // SAFETY: the action is zeroed, then given a handler that only calls

        unsafe {
            let mut act: libc::sigaction = std::mem::zeroed();
            act.sa_sigaction = on_signal as extern "C" fn(libc::c_int) as usize;
            act.sa_flags = libc::SA_ONSTACK | libc::SA_RESETHAND;
            libc::sigemptyset(&mut act.sa_mask);
            libc::sigaction(*sig, &act, std::ptr::null_mut());
        }
    }
    let before = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        if !PANICKED.swap(true, Ordering::SeqCst) {
            let place = info.location().map(|l| (l.file().to_string(), l.line()));
            add(&line("crash", "panic", place, &own_frames()));
        }
        before(info);
    }));
}

pub fn error(code: &'static str) {
    if READY.get().is_some() {
        add(&line("error", code, None, &own_frames()));
    }
}

extern "C" fn on_signal(sig: libc::c_int) {
    let Some((paths, lines)) = READY.get() else { return };
    let noted_panic = sig == libc::SIGABRT && PANICKED.load(Ordering::SeqCst);
    if let Some(i) = SIGNALS.iter().position(|(s, _)| *s == sig).filter(|_| !noted_panic) {
        for path in paths {
            // SAFETY: open, write and close are safe inside a signal handler,

            unsafe {
                let fd = libc::open(path.as_ptr(), libc::O_WRONLY | libc::O_APPEND | libc::O_CREAT | libc::O_CLOEXEC, 0o600);
                if fd >= 0 {
                    libc::write(fd, lines[i].as_ptr().cast(), lines[i].len());
                    libc::close(fd);
                }
            }
        }
    }

    // SAFETY: raise only sends a signal to this process.
    unsafe { libc::raise(sig) };
}

fn own_frames() -> Vec<String> {
    let trace = std::backtrace::Backtrace::force_capture().to_string();
    let mut frames = Vec::new();
    for l in trace.lines() {
        let Some((_, name)) = l.trim().split_once(": ") else { continue };
        let name = name.strip_suffix(|c: char| c.is_ascii_hexdigit()).map_or(name, |_| trim_hash(name));
        let Some(own) = ["trindle::", "trindle_compose::"].iter().find_map(|p| name.strip_prefix(p)) else { continue };

        let here = own.strip_prefix("common::").unwrap_or(own).strip_prefix("crash::").unwrap_or("");
        let noting = ["own_frames", "error", "watch"].contains(&here) || here.starts_with("watch::");
        if noting || frames.last().is_some_and(|f: &String| f == own) {
            continue;
        }
        frames.push(own.to_string());
        if frames.len() == FRAMES {
            break;
        }
    }
    frames
}

fn trim_hash(name: &str) -> &str {
    match name.rsplit_once("::h") {
        Some((head, hash)) if hash.len() == 16 && hash.chars().all(|c| c.is_ascii_hexdigit()) => head,
        _ => name,
    }
}

fn line(kind: &str, what: &str, place: Option<(String, u32)>, frames: &[String]) -> Vec<u8> {
    let (file, at) = place.map_or((None, None), |(f, l)| (Some(f), Some(l)));
    let mut out = serde_json::to_vec(&serde_json::json!({
        "kind": kind,
        "what": what,
        "program": PROGRAM.get().map_or("trindle", String::as_str),
        "version": env!("CARGO_PKG_VERSION"),
        "file": file,
        "line": at,
        "frames": frames,
    })).unwrap_or_default();
    out.push(b'\n');
    out
}

pub fn app_ended(id: &str, name: &str, status: i32) {
    let Some(signal) = fault(status) else { return };
    let Some(path) = history() else { return };
    if fs::metadata(&path).is_ok_and(|m| m.len() > MOST) {
        return;
    }
    let mut out = serde_json::to_vec(&serde_json::json!({
        "kind": "crash", "what": signal, "program": id, "name": name, "app": true,
    })).unwrap_or_default();
    out.push(b'\n');
    if let Ok(mut f) = OpenOptions::new().append(true).create(true).mode(0o600).open(&path) {
        let _ = f.write_all(&out);
    }
}

pub fn is_fault(status: i32) -> bool {
    fault(status).is_some()
}

fn fault(status: i32) -> Option<&'static str> {
    let signal = if libc::WIFSIGNALED(status) {
        libc::WTERMSIG(status)
    } else if libc::WIFEXITED(status) && libc::WEXITSTATUS(status) > 128 {
        libc::WEXITSTATUS(status) - 128
    } else {
        return None;
    };
    Some(match signal {
        libc::SIGSEGV => "SIGSEGV",
        libc::SIGABRT => "SIGABRT",
        libc::SIGBUS => "SIGBUS",
        libc::SIGILL => "SIGILL",
        libc::SIGFPE => "SIGFPE",
        libc::SIGTRAP => "SIGTRAP",
        libc::SIGSYS => "SIGSYS",

        _ => return None,
    })
}

fn add(line: &[u8]) {
    for path in [notes(), history()].into_iter().flatten() {
        if fs::metadata(&path).is_ok_and(|m| m.len() > MOST) {
            continue;
        }
        if let Ok(mut f) = OpenOptions::new().append(true).create(true).mode(0o600).open(&path) {
            let _ = f.write_all(line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_faults_are_crashes() {
        assert_eq!(fault(libc::SIGSEGV), Some("SIGSEGV"));
        assert_eq!(fault((128 + libc::SIGABRT) << 8), Some("SIGABRT"));
        assert_eq!(fault(libc::SIGTERM), None);
        assert_eq!(fault(libc::SIGKILL), None);
        assert_eq!(fault(0), None);
        assert_eq!(fault(1 << 8), None);
    }

    #[test]
    fn hashes_come_off_names() {
        assert_eq!(trim_hash("trindle::ring::open::h0123456789abcdef"), "trindle::ring::open");
        assert_eq!(trim_hash("trindle::ring::open"), "trindle::ring::open");
    }

    #[test]
    fn a_note_holds_only_the_allowed_parts() {
        let l = line("crash", "panic", Some(("src/ring/mod.rs".into(), 12)), &["ring::open".into()]);
        let v: serde_json::Value = serde_json::from_slice(&l).unwrap();
        let mut keys: Vec<&str> = v.as_object().unwrap().keys().map(String::as_str).collect();
        keys.sort();
        assert_eq!(keys, ["file", "frames", "kind", "line", "program", "version", "what"]);
        assert_eq!(v["file"], "src/ring/mod.rs");
    }

    fn note_from_child(how: &str) -> serde_json::Value {
        let home = std::env::temp_dir().join(format!("trindle-crash-{how}-{}", std::process::id()));
        let _ = fs::remove_dir_all(&home);
        fs::create_dir_all(&home).unwrap();
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "--nocapture", "--test-threads=1"])
            .arg(format!("{}::child_falls_over", module_path!().split_once("::").map_or("", |(_, rest)| rest)))
            .env("HOME", &home).env("TRINDLE_CRASH_CHILD", how)
            .output().unwrap().status;
        assert!(!status.success());
        let text = fs::read_to_string(home.join(".local/state/trindle/crash-notes.jsonl")).unwrap_or_default();
        let kept = fs::read_to_string(home.join(".local/state/trindle/crashes.jsonl")).unwrap_or_default();
        let _ = fs::remove_dir_all(&home);
        assert_eq!(text, kept, "the history holds the same note");
        assert_eq!(text.lines().count(), 1, "{text}");
        serde_json::from_str(text.trim()).unwrap()
    }

    #[test]
    fn child_falls_over() {
        match std::env::var("TRINDLE_CRASH_CHILD").as_deref() {
            Ok("signal") => {
                watch("trindle-test");
                // SAFETY: the test child is meant to end here.
                unsafe { libc::raise(libc::SIGSEGV) };
            }
            Ok("panic") => {
                watch("trindle-test");
                panic!("a message that must never be noted");
            }
            _ => {}
        }
    }

    #[test]
    fn a_fault_leaves_one_note() {
        let v = note_from_child("signal");
        assert_eq!((v["kind"].as_str(), v["what"].as_str(), v["program"].as_str()), (Some("crash"), Some("SIGSEGV"), Some("trindle-test")));
    }

    #[test]
    fn a_panic_leaves_one_note_without_its_message() {
        let v = note_from_child("panic");
        assert_eq!(v["what"], "panic");
        assert!(v["file"].as_str().is_some_and(|f| f.ends_with("crash.rs")));
        assert!(!v.to_string().contains("never be noted"));
    }

    #[inline(never)]
    fn a_named_place() -> Vec<String> {
        own_frames()
    }

    #[test]
    fn frames_name_trindles_own_functions() {
        let frames = a_named_place();

        if cfg!(debug_assertions) {
            assert!(frames.iter().any(|f| f.contains("crash::tests::")), "{frames:?}");
        }
        assert!(frames.iter().all(|f| !f.contains("::h") && !f.starts_with("std::")), "{frames:?}");
    }
}
