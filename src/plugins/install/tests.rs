use super::*;
use crate::plugins::fixtures::*;
use crate::plugins::fields::choices_of;
use crate::plugins::manifest::check;

#[test]
fn what_a_choice_offers_is_the_files_else_its_own() {
    let mut p = check(&folder_with("offers", &FOUND.replace("ChoicesFrom=found.txt", "Choices=a;b\nChoicesFrom=found.txt"))).unwrap();
    let f = p.fields.remove(0);

    assert_eq!(choices_of("no.such.plugin.id", &f), [("a".to_string(), "a".to_string()), ("b".to_string(), "b".to_string())]);
}

#[test]
fn a_failed_swap_leaves_the_old_program_whole_and_no_new_folder() {
    let at = kept_dir("swap");
    let (app, fresh) = (at.join("app"), at.join("app.new"));
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(app.join("old"), "old").unwrap();

    assert!(swap_in_with(&at, &fresh, &|a, b| std::fs::rename(a, b)).is_err());
    assert_eq!(std::fs::read_to_string(app.join("old")).unwrap(), "old", "the old program is back");
    assert!(!fresh.exists() && !at.join("app.old").exists());

    std::fs::create_dir_all(&fresh).unwrap();
    std::fs::write(fresh.join("new"), "new").unwrap();
    assert!(swap_in_with(&at, &fresh, &|a, b| std::fs::rename(a, b)).is_ok());
    assert!(app.join("new").exists() && !app.join("old").exists() && !fresh.exists() && !at.join("app.old").exists());
    let _ = std::fs::remove_dir_all(&at);
}

#[test]
fn a_swap_that_cannot_go_back_keeps_the_new_program_and_tells_both_reasons() {
    use std::cell::Cell;
    let at = kept_dir("swap-back");
    let (app, fresh, old_app) = (at.join("app"), at.join("app.new"), at.join("app.old"));
    std::fs::create_dir_all(&app).unwrap();
    std::fs::create_dir_all(&fresh).unwrap();
    std::fs::write(fresh.join("new"), "new").unwrap();
    let step = Cell::new(0);
    let rename = |from: &Path, to: &Path| {
        step.set(step.get() + 1);
        if step.get() == 1 { std::fs::rename(from, to) } else { Err(std::io::Error::other(format!("step {}", step.get()))) }
    };
    let err = swap_in_with(&at, &fresh, &rename).unwrap_err();
    assert!(err.contains("step 2") && err.contains("step 3"), "{err}");
    assert!(fresh.join("new").exists(), "the new program is kept");
    assert!(old_app.exists(), "so is the old one");
    let _ = std::fs::remove_dir_all(&at);
}

#[test]
fn a_second_install_of_a_plugin_waits_for_the_first() {
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    let at = kept_dir("lock");
    std::fs::create_dir_all(&at).unwrap();
    let first = install_lock(&at).unwrap();
    let got = Arc::new(AtomicBool::new(false));
    let (flag, dir) = (got.clone(), at.clone());
    let second = std::thread::spawn(move || {
        let _held = install_lock(&dir).unwrap();
        flag.store(true, Ordering::SeqCst);
    });
    std::thread::sleep(std::time::Duration::from_millis(300));
    assert!(!got.load(Ordering::SeqCst), "the second went ahead while the first held it");
    drop(first);
    second.join().unwrap();
    assert!(got.load(Ordering::SeqCst));
    let _ = std::fs::remove_dir_all(&at);
}

#[test]
fn a_failed_swap_keeps_the_old_versions_allowances() {
    let at = kept_dir("swap-allow");
    let (app, fresh) = (at.join("app"), at.join("app.new"));
    std::fs::create_dir_all(&app).unwrap();
    std::fs::create_dir_all(&fresh).unwrap();
    std::fs::write(at.join("asked"), "listen\n").unwrap();
    let p = check(&folder_with("swap-allow-new", FOUND)).unwrap();
    let failing = |_: &Path, _: &Path| Err(std::io::Error::other("no"));
    assert!(swap_then_revoke(&at, &fresh, &p, true, &failing).is_err());
    assert_eq!(std::fs::read_to_string(at.join("asked")).unwrap(), "listen\n", "nothing was taken away");
    std::fs::create_dir_all(&fresh).unwrap();
    assert!(swap_then_revoke(&at, &fresh, &p, true, &|a, b| std::fs::rename(a, b)).is_ok());
    assert_eq!(std::fs::read_to_string(at.join("asked")).unwrap(), "", "taken away once it is in");
    let _ = std::fs::remove_dir_all(&at);
}
