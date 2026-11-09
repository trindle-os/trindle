//! The screen reader: the one place Trindle's own reader will plug in.

use super::files::{set_setting, setting};

pub const KEY: &str = "screen_reader";

pub const NONE_YET: &str = "No screen reader is installed.";

pub trait Backend: Sync {
    fn is_installed(&self) -> bool;
    fn is_running(&self) -> bool;
    fn start(&self);
    fn stop(&self);
}

const EXE: &str = "/usr/lib/trindle/plugins/org.trindle.ScreenReader/bin/trindle-screen-reader";

struct Reader;

impl Backend for Reader {
    fn is_installed(&self) -> bool {
        std::path::Path::new(EXE).is_file()
    }
    fn is_running(&self) -> bool {
        std::process::Command::new(EXE).arg("status").stdout(std::process::Stdio::null()).status().is_ok_and(|s| s.success())
    }
    fn start(&self) {
        super::run::spawn(&[EXE, "start"]);
    }
    fn stop(&self) {
        super::run::spawn(&[EXE, "stop"]);
    }
}

const BACKEND: Option<&dyn Backend> = Some(&Reader);

pub fn on() -> bool {
    setting(KEY) == "on"
}

pub fn installed() -> bool {
    BACKEND.is_some_and(|b| b.is_installed())
}

pub fn running() -> bool {
    BACKEND.is_some_and(|b| b.is_running())
}

pub fn start() {
    if let Some(b) = BACKEND {
        b.start();
    }
}

pub fn start_for_login() {
    if installed() {
        super::run::spawn(&login_command());
    }
}

fn login_command() -> [&'static str; 5] {
    ["env", "TRINDLE_SR_CONTEXT=greeter", "TRINDLE_SR_CONFIG=/etc/trindle/screen-reader.toml", EXE, "start"]
}

pub fn stop() {
    if let Some(b) = BACKEND {
        b.stop();
    }
}

pub fn set(on: bool, own: bool) -> std::io::Result<()> {
    use super::own_helpers::Helper;
    use super::own_helpers_set;
    turn(
        on,
        |on| own_helpers_set::switch(Helper::Reader, on, own),
        |on| set_setting(KEY, if on { "on" } else { "off" }),
        |on| {
            if on && installed() {
                start();
            } else {
                stop();
            }
        },
    )
}

fn turn(on: bool, mark: impl Fn(bool) -> std::io::Result<()>, save: impl Fn(bool) -> std::io::Result<()>, run: impl FnOnce(bool)) -> std::io::Result<()> {
    if on {
        mark(true)?;
        if let Err(e) = save(true) {
            return Err(match mark(false) {
                Err(undo) => std::io::Error::new(e.kind(), format!("{e}; and its mark could not be taken away again: {undo}")),
                Ok(()) => e,
            });
        }
    } else {
        let marked = mark(false);
        let saved = save(false);
        run(false);
        super::run::spawn(&["trindle-ring", "--restyle"]);
        return marked.and(saved);
    }
    run(true);

    super::run::spawn(&["trindle-ring", "--restyle"]);
    Ok(())
}

pub fn toggle(own: bool) -> std::io::Result<bool> {
    toggle_with(installed(), running(), own, |on| set(on, own))
}

fn toggle_with(installed: bool, running: bool, own: bool, set: impl FnOnce(bool) -> std::io::Result<()>) -> std::io::Result<bool> {
    if !installed {
        return Ok(false);
    }
    if let Some(on) = toggled(running, own) {
        set(on)?;
    }
    Ok(true)
}

fn toggled(running: bool, own: bool) -> Option<bool> {
    (!running || own).then_some(!running)
}

#[cfg(test)]
mod tests {

    #[test]
    fn nothing_installs_or_starts_an_outside_reader() {
        let word = ["or", "ca"].concat();
        let speech = ["speech-", "dispatcher"].concat();
        let voice = ["e", "speak"].concat();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut found = Vec::new();
        let mut dirs = vec![root.join("src"), root.join("config")];
        dirs.extend(["packages.txt", "README.md"].map(|f| root.join(f)));
        while let Some(p) = dirs.pop() {
            if p.is_dir() {
                dirs.extend(std::fs::read_dir(&p).unwrap().flatten().map(|e| e.path()));
            } else if let Ok(text) = std::fs::read_to_string(&p) {
                for (n, line) in text.lines().enumerate() {
                    let low = line.to_lowercase();
                    if low.contains(&word) || low.contains(&speech) || low.contains(&voice) {
                        found.push(format!("{}:{}", p.display(), n + 1));
                    }
                }
            }
        }

        assert!(found.is_empty(), "a program for reading the screen is named: {found:?}");
    }

    #[test]
    fn the_words_say_none_is_installed_and_installed_follows_the_file() {
        assert!(super::NONE_YET.contains("No screen reader"));
        assert_eq!(super::installed(), std::path::Path::new(super::EXE).is_file());
    }

    #[test]
    fn the_login_reader_gets_its_context_and_config() {
        let c = super::login_command();
        assert_eq!(c[0], "env");
        assert!(c.contains(&"TRINDLE_SR_CONTEXT=greeter"));
        assert!(c.contains(&"TRINDLE_SR_CONFIG=/etc/trindle/screen-reader.toml"));
        assert_eq!(&c[3..], [super::EXE, "start"]);
    }

    #[test]
    fn turning_off_goes_on_past_a_failed_mark_and_says_so() {
        use std::cell::RefCell;
        use std::io::Error;
        let did = RefCell::new(Vec::new());
        let fail = |what: &'static str| Error::other(what);
        let r = super::turn(false, |on| { did.borrow_mut().push(format!("mark {on}")); Err(fail("mark")) },
            |on| { did.borrow_mut().push(format!("save {on}")); Ok(()) }, |on| did.borrow_mut().push(format!("run {on}")));
        assert_eq!(r.unwrap_err().to_string(), "mark");
        assert_eq!(*did.borrow(), ["mark false", "save false", "run false"]);

        did.borrow_mut().clear();
        let r = super::turn(false, |_| Ok(()), |_| Err(fail("save")), |on| did.borrow_mut().push(format!("run {on}")));
        assert_eq!(r.unwrap_err().to_string(), "save");
        assert_eq!(*did.borrow(), ["run false"]);

        did.borrow_mut().clear();
        let r = super::turn(true, |on| { did.borrow_mut().push(format!("mark {on}")); Ok(()) }, |_| Err(fail("save")), |on| did.borrow_mut().push(format!("run {on}")));
        assert_eq!(r.unwrap_err().to_string(), "save");
        assert_eq!(*did.borrow(), ["mark true", "mark false"]);
    }

    #[test]
    fn the_keys_say_what_failed() {
        use std::io::Error;
        let broken = |_| Err(Error::other("no file"));
        assert!(!super::toggle_with(false, false, true, broken).unwrap());
        assert_eq!(super::toggle_with(true, true, true, broken).unwrap_err().to_string(), "no file");
        assert_eq!(super::toggle_with(true, false, false, broken).unwrap_err().to_string(), "no file");

        assert!(super::toggle_with(true, true, false, broken).unwrap());
    }

    #[test]
    fn made_up_keys_never_turn_the_screen_reader_off() {
        assert_eq!((super::toggled(false, false), super::toggled(true, false)), (Some(true), None));
        assert_eq!((super::toggled(false, true), super::toggled(true, true)), (Some(true), Some(false)));
    }
}
