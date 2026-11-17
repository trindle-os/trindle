//! Backing up and putting back from the command line, with no window.

use std::cell::Cell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::common::run;
use crate::toolkit::event::Loop;
use crate::toolkit::t;
use super::opts::Opts;
use super::{backup, flat, refusal};

pub(super) fn command_line_work(opts: &Opts, events: &Loop) -> Option<i32> {
    if let Some(file) = opts.backup.clone() {
        return Some(without_window(events, |done| {
            let parts = backup::Parts { apps: true, data: true, appimages: true, settings: true };
            backup::backup(parts, PathBuf::from(file), move |res| {
                match &res {
                    Ok(size) => println!("trindle-apps: {}", t!("store-cli-backed-up", size = size)),
                    Err(e) => eprintln!("trindle-apps: {}", refusal::plain(e)),
                }
                done(res.is_ok());
            });
        }));
    }
    if let Some(file) = opts.restore.clone() {
        return Some(without_window(events, |done| restore(PathBuf::from(file), done)));
    }
    None
}

fn without_window(events: &Loop, work: impl FnOnce(Done)) -> i32 {
    run::hand_back_to(events);
    until_done(events, work)
}

fn until_done(events: &Loop, work: impl FnOnce(Done)) -> i32 {
    let ev = events.clone();
    let code = Rc::new(Cell::new(1));
    let c = code.clone();
    work(Box::new(move |ok| {
        c.set(if ok { 0 } else { 1 });
        ev.quit();
    }));
    events.run();
    code.get()
}

type Done = Box<dyn FnOnce(bool)>;

fn restore(path: PathBuf, done: Done) {
    let f = match flat::read(&path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("trindle-apps: {}", refusal::plain(&e));
            return done(false);
        }
    };
    let out = flat::restore(&f, false, |line, _| println!("trindle-apps: {line}"));
    println!("trindle-apps: {}", refusal::plain(&flat::summary(&out)));
    let whole = out.failed.is_empty();
    if !f.trindle {
        return done(whole);
    }
    backup::restore_trindle_part(path, |step| println!("trindle-apps: {step}"), move |res| {
        match &res {
            Ok(()) => println!("trindle-apps: {}", t!("store-cli-settings-back")),
            Err(e) => eprintln!("trindle-apps: {}", refusal::plain(e)),
        }
        done(whole && res.is_ok());
    });
}

#[cfg(test)]
mod tests {

    #[test]
    fn command_line_work_ends_and_says_whether_it_was_done() {
        use crate::toolkit::event::Loop;
        use std::time::Duration;
        for (ok, code) in [(false, 1), (true, 0)] {

            assert_eq!(super::until_done(&Loop::new(), |done| done(ok)), code);

            let lp = Loop::new();
            let later = lp.clone();
            assert_eq!(super::until_done(&lp, move |done| later.after(Duration::from_millis(5), move || done(ok))), code);
        }
    }

    #[test]
    fn a_restore_of_a_file_that_is_no_backup_ends_with_a_failing_code() {
        let dir = std::env::temp_dir().join(format!("trindle-cli-restore-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("not-a-backup.fmpack");
        std::fs::write(&file, b"no backup here").unwrap();
        assert_eq!(super::until_done(&crate::toolkit::event::Loop::new(), |done| super::restore(file, done)), 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_restore_hands_on_its_step_as_numbers_and_not_in_words() {
        let source = crate::common::source::code(include_str!("flat.rs"));
        let restore = source.split("pub fn restore(").nth(1).and_then(|r| r.split("#[cfg(test)]").next()).unwrap_or_default();
        assert!(restore.contains("Some((at, total))"), "the step goes with the words");
        let pages = crate::common::source::every_file().into_iter().filter(|(name, _)| name.starts_with("store/pages/"));
        assert!(pages.into_iter().all(|(_, text)| !text.contains("fn counted")), "no step is read back out of the words");
    }
}
