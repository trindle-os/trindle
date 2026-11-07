//! Tells the person when one of Trindle's programs has crashed.

use std::path::{Path, PathBuf};
use std::rc::Rc;

use serde_json::Value;

use crate::toolkit::event::Flow;
use crate::toolkit::watch::Watch;

use super::ctx::Ctx;
use super::notify;
use crate::toolkit::t;

const TRIM_AT: u64 = 32 * 1024;
const KEEP: usize = 40;

#[derive(Debug, PartialEq)]
pub enum Previous {
    Nothing,
    Mark(i64),
    Window(String, i64),
}

pub fn previous(on: bool, mark: Option<i64>, newest: Option<(String, i64)>, now: i64) -> Previous {
    if !on {
        return Previous::Nothing;
    }
    match (mark, newest) {
        (None, _) => Previous::Mark(now),
        (Some(m), Some((program, at))) if at > m && program != "trindle-report" => Previous::Window(program, at),
        _ => Previous::Nothing,
    }
}

fn mark_file() -> PathBuf {
    crate::report::pending_dir().with_file_name("reports-offered")
}

fn now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

fn write_mark(at: i64) {
    let file = mark_file();
    if let Some(dir) = file.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = crate::common::files::write_whole_private(&file, at.to_string().as_bytes());
}

fn after_start() {
    std::thread::spawn(start_work);
}

fn start_work() {
    crate::report::flush();
    let mark = std::fs::read_to_string(mark_file()).ok().and_then(|t| t.trim().parse().ok());
    let on = crate::report::offered();
    let newest = if on { crate::report::probe::newest_trindle() } else { None };
    match previous(on, mark, newest, now()) {
        Previous::Mark(at) => write_mark(at),
        Previous::Window(program, at) => {
            write_mark(at);
            crate::common::run::spawn(&["trindle-report", "--crash", &program]);
        }
        Previous::Nothing => {}
    }
}

fn made(folder: &Path) -> bool {
    std::fs::create_dir_all(folder).is_ok()
}

pub fn watch(ctx: &Rc<Ctx>) {
    after_start();
    ctx.lp.every(std::time::Duration::from_secs(600), || {
        std::thread::spawn(|| {
            crate::report::flush();
        });
        Flow::Continue
    });
    let Some(path) = crate::common::crash::history() else { return };
    let Some(folder) = path.parent().map(Path::to_path_buf) else { return };
    let lp = ctx.lp.clone();
    let there = folder.clone();
    crate::common::run::on_thread(move || made(&there), move |made| {
        if !made {
            return;
        }
        look(&path);
        let Ok(mut watch) = Watch::new() else { return };
        if watch.folder(&folder).is_err() {
            return;
        }
        lp.watch(watch.fd(), move |hung_up| {
            if hung_up {
                return Flow::Stop;
            }
            if watch.changed().contains(&path) {
                look(&path);
            }
            Flow::Continue
        });
    });
}

pub fn hand_off(text: &str) {
    let time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    crate::common::clipboard::copy(&crate::common::clipboard::Item {
        id: time, time, text: text.to_string(), picture: None, pinned: false,
    });
}

fn seen_file(history: &Path) -> PathBuf {
    history.with_file_name("crashes.seen")
}

fn look(path: &Path) {
    let mut asked = false;
    for line in crate::common::files::unread_lines(path, &seen_file(path), TRIM_AT, KEEP) {
        if let Some((summary, handoff)) = told(&line) {
            notify::say_crash(&summary, t!("ring-crash-click"), handoff);
        }
        if !asked && crate::report::offered() {
            if let Some(program) = report_program(&line) {
                asked = true;
                std::thread::spawn(move || { write_mark(now()); crate::common::run::spawn(&["trindle-report", "--crash", &program]); });
            }
        }
    }
}

fn report_program(line: &str) -> Option<String> {
    let v: Value = serde_json::from_str(line).ok()?;
    let program = v["program"].as_str()?;
    let own = v["kind"] == "crash" && v["app"] != true && crate::NAMES.contains(&program) && program != "trindle-report";
    own.then(|| program.to_string())
}

fn called(program: &str) -> String {
    match program.strip_prefix("trindle-").unwrap_or(program) {
        "trindle" | "" => t!("ring-crash-trindle").into(),
        "compose" => t!("ring-crash-desktop").into(),
        "greeter" => t!("ring-crash-greeter").into(),
        "lock" => t!("ring-crash-lock").into(),
        "settings" => t!("ring-crash-settings").into(),
        "apps" | "store" => t!("ring-crash-apps").into(),
        rest => t!("ring-crash-other", name = rest.replace('-', " ")),
    }
}

fn told(line: &str) -> Option<(String, String)> {
    let v: Value = serde_json::from_str(line).ok()?;
    if v["kind"] != "crash" {
        return None;
    }
    let program = v["program"].as_str()?;
    let what = v["what"].as_str().unwrap_or("a fault");
    let name = v["name"].as_str().map_or_else(|| called(program), String::from);
    let mut handoff = match v["version"].as_str() {
        Some(version) => format!("{name} ({program}, version {version}) crashed: {what}"),
        None => format!("The app {name} ({program}) crashed: {what}"),
    };
    if let (Some(file), Some(at)) = (v["file"].as_str(), v["line"].as_u64()) {
        handoff.push_str(&format!(", at {file} line {at}"));
    }
    let frames: Vec<&str> = v["frames"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
    if !frames.is_empty() {
        handoff.push_str(&format!(". Where in Trindle's code, nearest first: {}", frames.join(", ")));
    }
    handoff.push_str(". Please find out why. If you are connected to Trindle's MCP server (trindle-mcp), its \
                      list_crashes and read_logs tools hold the detail. Tell me in plain words what went wrong and what I can do.");
    Some((t!("ring-crash-stopped", name = name), handoff))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_crash_from_before_this_start_is_offered_once_and_never_with_the_switch_off() {
        let newest = || Some(("trindle-compose".to_string(), 500));
        assert_eq!(previous(false, Some(100), newest(), 900), Previous::Nothing, "switch off");
        assert_eq!(previous(false, None, newest(), 900), Previous::Nothing, "switch off, first start");
        assert_eq!(previous(true, None, newest(), 900), Previous::Mark(900), "nothing from before this was asked about");
        assert_eq!(previous(true, Some(100), newest(), 900), Previous::Window("trindle-compose".into(), 500));
        assert_eq!(previous(true, Some(500), newest(), 900), Previous::Nothing, "already offered");
        assert_eq!(previous(true, Some(100), None, 900), Previous::Nothing);
    }

    #[test]
    fn a_report_is_offered_only_for_a_crash_of_trindles_own_program() {
        let own = r#"{"kind":"crash","what":"panic","program":"trindle-ring"}"#;
        assert_eq!(report_program(own).as_deref(), Some("trindle-ring"));
        for no in [r#"{"kind":"crash","program":"org.example.Draw","app":true}"#,
                   r#"{"kind":"crash","program":"trindle-report"}"#,
                   r#"{"kind":"error","program":"trindle-ring"}"#,
                   r#"{"kind":"crash","program":"/bin/evil"}"#, "not json"] {
            assert_eq!(report_program(no), None, "{no}");
        }
    }

    #[test]
    fn a_crash_is_told_and_an_error_is_not() {
        let crash = r#"{"kind":"crash","what":"panic","program":"trindle-ring","version":"0.9.0","file":"src/ring/view.rs","line":12,"frames":["ring::view::open"]}"#;
        let (summary, handoff) = told(crash).unwrap();
        assert_eq!(summary, "Trindle's ring stopped unexpectedly");
        assert!(handoff.contains("src/ring/view.rs line 12") && handoff.contains("ring::view::open"), "{handoff}");
        assert!(told(r#"{"kind":"error","what":"E_SAVE_FAILED","program":"trindle-ring"}"#).is_none());
        assert!(told("not a note").is_none());
        let app = r#"{"kind":"crash","what":"SIGSEGV","program":"org.example.Draw","name":"Draw","app":true}"#;
        let (summary, handoff) = told(app).unwrap();
        assert_eq!(summary, "Draw stopped unexpectedly");
        assert!(handoff.starts_with("The app Draw (org.example.Draw) crashed: SIGSEGV"), "{handoff}");
    }

    #[test]
    fn each_crash_is_told_once() {
        let dir = std::env::temp_dir().join(format!("trindle-crashes-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("crashes.jsonl");
        std::fs::write(&path, "{}\n{}\n").unwrap();
        look(&path);
        assert_eq!(std::fs::read_to_string(seen_file(&path)).unwrap(), "2");
        std::fs::write(&path, "{}\n").unwrap();
        look(&path);
        assert_eq!(std::fs::read_to_string(seen_file(&path)).unwrap(), "1");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
