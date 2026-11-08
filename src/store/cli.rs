//! Backing up and putting back from the command line, with no window.

use std::cell::Cell;
use std::path::PathBuf;
use std::rc::Rc;

use crate::common::run;
use crate::toolkit::event::Loop;
use super::opts::Opts;
use super::{backup, flat, refusal};

pub(super) fn command_line_work(opts: &Opts, events: &Loop) -> Option<i32> {
    if let Some(file) = opts.backup.clone() {
        return Some(without_window(events, |done| {
            let parts = backup::Parts { apps: true, data: true, appimages: true, settings: true };
            backup::backup(parts, PathBuf::from(file), move |res| {
                match &res {
                    Ok(size) => println!("trindle-apps: backed up ({size})"),
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
    if flat::is_flat(&path) {
        match flat::read(&path) {
            Ok(f) => {
                let out = flat::restore(&f, false, |line, _| println!("trindle-apps: {line}"));
                println!("trindle-apps: {}", refusal::plain(&flat::summary(&out)));
                let whole = out.failed.is_empty();
                if f.trindle {
                    backup::restore_trindle_part(path, |step| println!("trindle-apps: {step}"), move |res| {
                        match &res {
                            Ok(()) => println!("trindle-apps: Trindle's settings put back"),
                            Err(e) => eprintln!("trindle-apps: {}", refusal::plain(e)),
                        }
                        done(whole && res.is_ok());
                    });
                    return;
                }
                done(whole);
            }
            Err(e) => {
                eprintln!("trindle-apps: {}", refusal::plain(&e));
                done(false);
            }
        }
        return;
    }
    let reading = path.clone();
    backup::read_manifest(&reading, move |m| match m {
        Err(e) => {
            eprintln!("trindle-apps: {}", refusal::plain(&e));
            done(false);
        }
        Ok(manifest) => backup::restore(path, manifest, false, |step| println!("trindle-apps: {step}"), move |res| {
            match &res {
                Ok(()) => println!("trindle-apps: put back"),
                Err(e) => eprintln!("trindle-apps: {}", refusal::plain(e)),
            }
            done(res.is_ok());
        }),
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

        let code = crate::common::source::code(include_str!("cli.rs"));
        let restore = code.split_once("\nfn restore(").unwrap().1.split_once("\n}\n").unwrap().0;
        assert_eq!(restore.matches("eprintln!").count(), 4);
        assert_eq!(restore.matches("done(false)").count() + restore.matches("is_ok())").count() + restore.matches("done(whole)").count(), 5);
        assert!(!restore.contains("done()"));
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
