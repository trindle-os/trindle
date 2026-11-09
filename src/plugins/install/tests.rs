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

    assert!(swap_in(&at, &fresh).is_err());
    assert_eq!(std::fs::read_to_string(app.join("old")).unwrap(), "old", "the old program is back");
    assert!(!fresh.exists() && !at.join("app.old").exists());

    std::fs::create_dir_all(&fresh).unwrap();
    std::fs::write(fresh.join("new"), "new").unwrap();
    assert!(swap_in(&at, &fresh).is_ok());
    assert!(app.join("new").exists() && !app.join("old").exists() && !fresh.exists() && !at.join("app.old").exists());
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
