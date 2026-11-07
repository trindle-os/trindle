use super::*;
use std::process::Command;
use crate::plugins::fixtures::*;

#[test]
fn a_background_part_is_started_outside_the_session_it_was_started_from() {
    let words = super::keeper_command("org.example.flow", true);
    assert_eq!(words, ["systemd-run", "--user", "--scope", "--quiet", "--collect", "--", "trindle-plugin", "run", "org.example.flow", "background"]);
    assert_eq!(super::keeper_command("org.example.flow", false), ["trindle-plugin", "run", "org.example.flow", "background"]);
    let code = plugin_code();
    let plain = ["\"trindle-plugin\", \"run\"", ", \"background\"]"];
    let started: Vec<&str> = code.lines().filter(|l| l.contains(plain[0]) && l.contains(plain[1])).collect();
    assert_eq!(started.len(), 1, "a background part is started other than by start_keeper: {started:#?}");
    assert!(started[0].contains("words.extend"), "{started:?}");
    assert_eq!(code.matches("start_keeper(").count(), 3, "start_background and begin_background each start it by start_keeper");
}

#[test]
fn a_lock_let_go_while_a_program_is_being_started_is_free_a_moment_later() {
    use std::os::unix::process::CommandExt;
    let dir = kept_dir("handed");
    let held = lock_taken(&dir);
    let mut starting = Command::new("true");
    // SAFETY: the closure runs in the new process before the program
    // begins and only sleeps, which is safe there; it touches no memory
    // and takes no lock.
    unsafe {
        starting.pre_exec(|| {
            libc::usleep(300_000);
            Ok(())
        })
    };
    let slow = std::thread::spawn(move || starting.status());
    std::thread::sleep(Duration::from_millis(100));
    drop(held);
    assert!(!lock_free(&dir), "the program being started still has its copy");
    assert!(comes_free(&dir), "and it is free once that program has begun");
    let _ = slow.join();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_new_keeper_waits_for_a_lock_the_last_of_the_old_box_still_has() {
    let dir = kept_dir("going");
    let id = format!("org.example.Going{}", std::process::id());
    let left_over = lock_taken(&dir);
    assert!(!lock_free(&dir));
    let going = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(400));
        drop(left_over);
    });
    let mine = only_copy_at(&dir, &id, Duration::from_secs(5));
    assert!(mine.is_some(), "the new keeper has the lock once the old box lets go");
    assert_eq!(read_text(dir.join(PID)), std::process::id().to_string());
    going.join().unwrap();
    drop(mine);
    assert!(comes_free(&dir));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_lock_held_by_nothing_that_keeps_the_plugin_does_not_stop_it_starting() {
    let dir = kept_dir("stale");
    let id = format!("org.example.Stale{}", std::process::id());
    std::fs::write(dir.join(LOCK), "").unwrap();
    assert!(comes_free(&dir));
    drop(only_copy_at(&dir, &id, Duration::ZERO).expect("a lock file left behind is no lock"));
    let left_over = lock_taken(&dir);
    let mine = only_copy_at(&dir, &id, Duration::from_millis(300));
    assert!(mine.is_some(), "a new lock is begun");
    assert_eq!(read_text(dir.join(PID)), std::process::id().to_string());
    drop(left_over);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_second_copy_still_does_not_start_while_a_keeper_runs() {
    let dir = kept_dir("second");
    let id = format!("org.example.Second{}", std::process::id());
    let held = lock_taken(&dir);
    let mut keeper = made_up_keeper(&dir, &id);
    assert_eq!(keepers_of(&id), [keeper.id() as i32]);
    let began = Instant::now();
    assert!(only_copy_at(&dir, &id, Duration::from_secs(5)).is_none());
    assert!(began.elapsed() < Duration::from_secs(2), "and it does not wait to find that out");
    assert!(!dir.join(PID).exists());
    assert!(keepers_of("org.example.Other").is_empty());
    let _ = keeper.kill();
    let _ = keeper.wait();
    drop(held);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn stopping_ends_a_keeper_whose_number_was_lost_and_waits_for_its_lock() {
    let dir = kept_dir("stop");
    let id = format!("org.example.Stop{}", std::process::id());
    let mut keeper = made_up_keeper(&dir, &id);
    assert!(keeper_in(&dir, &id).is_none());
    let left_over = lock_taken(&dir);
    let going = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(400));
        drop(left_over);
    });
    let began = Instant::now();
    stop_background_at(&dir, &id);
    assert!(began.elapsed() >= Duration::from_millis(300), "stopping came back only once the lock was let go");
    assert!(comes_free(&dir));
    assert!(keepers_of(&id).is_empty());
    assert!(keeper.try_wait().unwrap().is_some() || !is_background_of(keeper.id() as i32, &id), "the keeper was ended");
    let _ = keeper.kill();
    let _ = keeper.wait();
    going.join().unwrap();
    let mut named = made_up_keeper(&dir, &id);
    std::fs::write(dir.join(PID), named.id().to_string()).unwrap();
    assert_eq!(keeper_in(&dir, &id), Some(named.id() as i32));
    stop_background_at(&dir, &id);
    assert!(keepers_of(&id).is_empty() && !dir.join(PID).exists());
    let _ = named.kill();
    let _ = named.wait();
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_keeper_goes_on_in_a_session_that_is_left_and_a_failed_swap_starts_it_again() {
    use std::os::unix::net::UnixListener;
    let dir = kept_dir("screens");
    let first = UnixListener::bind(dir.join("wayland-0")).unwrap();
    let second = UnixListener::bind(dir.join("wayland-1")).unwrap();
    assert_eq!(living_screen(&dir, Some("wayland-1")).as_deref(), Some("wayland-1"));
    drop(second);
    assert!(dir.join("wayland-1").exists());
    let comes = |want: Option<&str>| {
        let began = Instant::now();
        while living_screen(&dir, Some("wayland-1")).as_deref() != want && began.elapsed() < LET_GO_WAIT {
            std::thread::sleep(Duration::from_millis(10));
        }
        living_screen(&dir, Some("wayland-1")).as_deref() == want
    };
    assert!(comes(Some("wayland-0")), "the session still open was not found");
    assert_eq!(living_screen(&dir, None).as_deref(), Some("wayland-0"));
    let other = UnixListener::bind(dir.join("pipewire-0")).unwrap();
    drop(first);
    assert!(comes(None), "a socket that is no screen's was joined, or one that had closed");
    drop(other);
    let _ = std::fs::remove_dir_all(&dir);
    let source = plugin_code();
    let body = |name: &str| source.split_once(&format!("fn {name}(")).unwrap().1.split_once("\n}\n").unwrap().0;
    let keeper = body("keep_running");
    assert_eq!((keeper.matches("screen_to_go_on_with()").count(), keeper.matches("screen_there()").count()), (2, 0));
    let install = body("put_in");
    let (stop, start) = (install.find("stop_background(").unwrap(), install.rfind("begin_background(").unwrap());
    let between = &install[stop..start];
    assert!(!between.contains("?;"), "a failure between the stop and the start leaves the plugin stopped");
    assert_eq!(between.matches("return Err(").count(), 1);
    assert_eq!(between.matches("return Err(not_swapped(").count(), 1, "a failed swap leaves the plugin stopped");
    assert!(between.contains("match if live { back_on(&p.id) } else { None } {"));
}

#[test]
fn a_renewed_keeper_trusts_only_the_lock_it_really_holds() {
    use std::os::fd::AsRawFd;
    let dir = kept_dir("handed");
    let held = take_lock(&dir).expect("the lock is free");
    let say = |n: i32| vec![format!("{KEEPING}{n}")];
    assert_eq!(lock_handed_on(&dir, &say(held.as_raw_fd())), Some(held.as_raw_fd()));
    for rest in [vec![], vec!["--keeping=x".to_string()], say(1), say(-1), say(9999), vec![format!("{}", held.as_raw_fd())]] {
        assert_eq!(lock_handed_on(&dir, &rest), None, "{rest:?}");
    }
    let other = std::fs::File::open(dir.join(LOCK)).unwrap();
    assert_eq!(lock_handed_on(&dir, &say(other.as_raw_fd())), None);
    let elsewhere = std::fs::File::create(dir.join("other")).unwrap();
    assert_eq!(lock_handed_on(&dir, &say(elsewhere.as_raw_fd())), None);
    assert!(take_lock(&dir).is_none());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_keeper_renews_only_between_runs_of_the_part() {
    let source = plugin_code();
    let body = |name: &str| source.split_once(&format!("fn {name}(")).unwrap().1.split_once("\n}\n").unwrap().0;
    let keeper = body("keep_running");
    let (run, due) = (keeper.find("run_boxed(").unwrap(), keeper.find("due_program(\"trindle-plugin\")").unwrap());
    assert!(run < due, "the keeper asks about renewing before the part has run");
    assert_eq!(keeper.matches("due_program(").count(), 1);
    let renew = body("become_new_keeper");
    assert!(renew.find("log.finish()").unwrap() < renew.find("become_new(").unwrap());
    assert!(renew.contains("KEEPING"));
}

#[test]
fn every_start_of_a_background_part_after_a_stop_is_the_one_that_looks() {
    let source = plugin_code();
    let body = |name: &str| source.split_once(&format!("fn {name}(")).unwrap().1.split_once("\n}\n").unwrap().0;
    for name in ["put_in", "restart_background"] {
        let b = body(name);
        let (stop, start) = (b.find("stop_background(").expect(name), b.find("begin_background(").expect(name));
        assert!(stop < start, "{name} stops, then starts and looks");
        assert!(!b.contains("\"background\"]"), "{name} starts it only through begin_background");
    }
    let login = body("start_background");
    assert!(login.find("stop_background(").unwrap() < login.find("start_keeper(").unwrap());
    assert_eq!(source.matches("libc::flock(").count(), 3);
    assert!(body("lock_handed_on").contains("libc::LOCK_NB"));
}

#[test]
fn only_its_own_background_part_is_stopped() {
    assert!(!is_background_of(std::process::id() as i32, "org.example.Weather"));
    assert!(!is_background_of(1, "org.example.Weather"));
}

#[test]
fn only_real_screen_sockets_are_found() {
    let dir = std::env::temp_dir().join(format!("trindle-screens-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for name in ["wayland-1", "wayland-12", "wayland-x", "bus"] {
        std::os::unix::net::UnixListener::bind(dir.join(name)).unwrap();
    }
    std::fs::write(dir.join("wayland-1.lock"), "").unwrap();
    std::fs::write(dir.join("wayland-3"), "a plain file").unwrap();
    assert_eq!(screens_in(&dir), ["wayland-1", "wayland-12"]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_wait_before_starting_again_grows_and_is_capped() {
    let secs: Vec<u64> = (0..8).map(|n| wait_before(n).as_secs()).collect();
    assert_eq!(secs, [2, 4, 8, 16, 32, 60, 60, 60]);
    assert_eq!(wait_before(u32::MAX).as_secs(), WAIT_MOST);
}
