// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

#[cfg(test)]
use crate::rig::kept_ring::{beat, there};
use crate::rig::*;

#[test]
fn a_ring_gone_quiet_is_ended_at_a_key_and_every_key_after_goes_to_the_window() {
    alone("rig::tests::ring_life::a_ring_gone_quiet_is_ended_at_a_key_and_every_key_after_goes_to_the_window", |dir| {
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();

        beat(&mut ring.pulse);
        rig.panel(ring.who, "trindle-ring");
        assert!(rig.has_keyboard(ring.who), "the ring's panel was not given the keyboard as it came out");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(ring.who), [KEY_A], "a healthy ring was not given its key");
        assert_eq!(rig.pressed(window), [] as [u32; 0]);
        assert_eq!(rig.state.ring.running_now(), Some((ring.number, ring.pid)), "a healthy ring was ended");

        std::thread::sleep(Duration::from_millis(crate::ring_pulse::SILENT_AFTER + 300));
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(ring.who), [KEY_A], "a ring that had stopped answering was given the key");
        assert_eq!(rig.pressed(window), [KEY_A], "the key did not go to the window instead");
        assert!(rig.has_keyboard(window), "the keyboard did not go back to the window");

        let ended = rig.until(Duration::from_secs(10), |rig| rig.state.ring.running_now().is_some_and(|r| r.0 != ring.number));
        assert!(ended, "no new ring was started after the old one was ended");
        assert!(!there(ring.pid), "the ring that stopped answering is still there");
        assert_eq!(rig.state.ring.counted(), 2, "the start after a ring was ended was not counted as one more");

        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(ring.who), [KEY_A], "the ended ring's connection was given a key");
        assert_eq!(rig.pressed(window), [KEY_A, KEY_A], "the second key did not go to the window");
    });
}

#[test]
fn a_ring_that_leaves_for_an_update_is_not_counted_as_one_that_fell_over() {
    alone("rig::tests::ring_life::a_ring_that_leaves_for_an_update_is_not_counted_as_one_that_fell_over", |dir| {

        let (program, leave) = (dir.join("trindle"), dir.join("leave"));
        let script = format!("#!/bin/sh\nwhile kill -0 $PPID 2>/dev/null; do if rm {} 2>/dev/null; then exit 0; fi; sleep 0.05; done\n", leave.display());
        let install = |n: usize| {

            let new = dir.join("trindle.dpkg-new");
            std::fs::write(&new, format!("{script}# {n}\n")).unwrap();
            std::fs::set_permissions(&new, std::os::unix::fs::PermissionsExt::from_mode(0o700)).unwrap();
            std::fs::rename(&new, &program).unwrap();
        };
        install(0);
        let mut rig = Rig::new(dir);
        let ring = rig.kept_ring();
        let mut number = ring.number;
        let mut next = |rig: &mut Rig, what: &str| {
            std::fs::write(&leave, "").unwrap();
            let started = rig.until(Duration::from_secs(10), |rig| rig.state.ring.running_now().is_some_and(|r| r.0 != number));
            assert!(started, "no ring was started after one left {what}: {} starts counted", rig.state.ring.counted());
            number = rig.state.ring.running_now().unwrap().0;
        };

        for n in 1..=10 {
            install(n);
            next(&mut rig, "for an update");
            assert_eq!(rig.state.ring.counted(), 1, "a ring that left for update {n} was counted as one that fell over");
        }

        for _ in 1..4 {
            next(&mut rig, "with no update");
        }
        assert_eq!(rig.state.ring.counted(), 4);
        next(&mut rig, "with no update");
        assert_eq!(rig.state.ring.counted(), 5);
        std::fs::write(&leave, "").unwrap();
        let held_back = rig.until(Duration::from_secs(10), |rig| rig.state.ring.running_now().is_none());
        assert!(held_back, "a ring that keeps leaving with no update was never held back");
        std::thread::sleep(Duration::from_millis(300));
        rig.pump();
        assert!(rig.state.ring.running_now().is_none(), "a ring held back was started all the same");
    });
}

#[test]
fn a_ring_given_up_on_is_given_no_key_on_any_of_its_panels() {
    alone("rig::tests::ring_life::a_ring_given_up_on_is_given_no_key_on_any_of_its_panels", |dir| {
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        rig.state.ring.spared = true;
        beat(&mut ring.pulse);
        rig.panel(ring.who, "trindle-ring");
        rig.panel(ring.who, "trindle-notes");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(ring.who), [KEY_A]);

        std::thread::sleep(Duration::from_millis(crate::ring_pulse::SILENT_AFTER + 300));
        for n in 1..=3 {
            rig.key_by_hand(KEY_A);
            assert_eq!(rig.pressed(ring.who), [KEY_A], "key {n} went to a panel of a ring that had stopped answering");
            assert_eq!(rig.pressed(window).len(), n, "key {n} did not go to the window");
            assert_eq!(rig.keys_with(), None, "the keyboard is with a panel of a ring given up on");
        }
    });
}

#[test]
fn the_kept_rings_own_error_words_are_in_the_compositors_log() {
    alone("rig::tests::ring_life::the_kept_rings_own_error_words_are_in_the_compositors_log", |dir| {
        std::fs::write(dir.join("trindle"), "#!/bin/sh\necho 'the ring says its pulse was refused' >&2\nwhile kill -0 $PPID 2>/dev/null; do sleep 0.2; done\n").unwrap();
        let mut rig = Rig::new(dir);
        rig.kept_ring();

        let log = dir.join("out");
        let said = rig.until(Duration::from_secs(5), |_| std::fs::read_to_string(&log).is_ok_and(|t| t.contains("the ring says its pulse was refused")));
        assert!(said, "what the kept ring wrote to its error output is not in the compositor's log");
    });
}

#[test]
fn a_ring_that_does_not_come_out_when_the_person_asks_is_ended_and_the_next_one_opened() {
    alone("rig::tests::ring_life::a_ring_that_does_not_come_out_when_the_person_asks_is_ended_and_the_next_one_opened", |dir| {

        let started = dir.join("started");
        std::fs::write(dir.join("trindle"), format!("#!/bin/sh\necho \"with $*\" >> {}\nwhile kill -0 $PPID 2>/dev/null; do sleep 0.2; done\n", started.display())).unwrap();
        let mut rig = Rig::new(dir);
        rig.bind_the_ring();
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        let wait = crate::own_ring::UNANSWERED + Duration::from_millis(700);
        let same = |rig: &mut Rig| rig.state.ring.running_now().map(|r| r.0) == Some(ring.number);

        rig.key_by_hand(KEY_F5);
        beat(&mut ring.pulse);
        let panel = rig.panel(ring.who, "trindle-ring");
        rig.until(wait, |_| false);
        assert!(same(&mut rig), "a ring that came out when asked for was ended");
        rig.put_away(&panel);

        rig.key_by_hand(KEY_F5);
        beat(&mut ring.pulse);
        rig.until(wait, |_| false);
        assert!(same(&mut rig), "a ring that gave a sign of life after it was asked for was ended");

        rig.made_up_key(KEY_F5);
        rig.until(wait, |_| false);
        assert!(same(&mut rig), "a ring was ended for not answering a made-up key");

        rig.key_by_hand(KEY_F5);
        let ended = rig.until(wait + Duration::from_secs(8), |rig| rig.state.ring.running_now().is_some_and(|r| r.0 != ring.number));
        assert!(ended, "a ring that neither came out nor gave a sign of life when the person asked for it was left as it was");
        assert!(!there(ring.pid), "the ring that did not answer is still there");
        assert_eq!(rig.state.ring.counted(), 2);

        let opened = rig.until(Duration::from_secs(5), |_| std::fs::read_to_string(&started).is_ok_and(|t| t.lines().count() == 2));
        assert!(opened, "no second ring was started");
        assert_eq!(std::fs::read_to_string(&started).unwrap(), "with --background\nwith \n", "the ring started in its place was not opened as the person asked");
        assert!(rig.state.ring.wished.is_some(), "the ring started in its place does not stand as asked for");
    });
}

#[test]
fn a_ring_that_outlives_its_ending_never_has_the_keyboard_again() {
    alone("rig::tests::ring_life::a_ring_that_outlives_its_ending_never_has_the_keyboard_again", |dir| {
        let mut rig = Rig::new(dir);
        let window = rig.join(ClientState::default());
        rig.window(window, false);
        let mut ring = rig.kept_ring();
        rig.state.ring.spared = true;
        beat(&mut ring.pulse);
        rig.panel(ring.who, "trindle-ring");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(ring.who), [KEY_A]);
        std::thread::sleep(Duration::from_millis(crate::ring_pulse::SILENT_AFTER + 300));

        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(window), [KEY_A]);
        assert!(there(ring.pid));

        for n in 2..=4 {
            rig.key_by_hand(KEY_A);
            assert_eq!(rig.pressed(ring.who), [KEY_A], "a ring given up on was handed the keyboard back at key {n}");
            assert_eq!(rig.pressed(window).len(), n, "key {n} did not go to the window");
            assert!(!rig.has_keyboard(ring.who));
        }

        assert_eq!(rig.state.ring.running_now(), Some((ring.number, ring.pid)));
        assert_eq!(rig.state.ring.counted(), 1);

        let said = rig.until(Duration::from_secs(9), |_| crate::ring_said::read() == Some(crate::ring_said::Said::NotEnded));
        assert!(said, "a ring that could not be ended was never said to be one");
        assert_eq!(rig.state.ring.running_now(), Some((ring.number, ring.pid)), "a ring was started beside one that had not ended");
        rig.key_by_hand(KEY_A);
        assert_eq!(rig.pressed(window).len(), 5);
        assert_eq!(rig.pressed(ring.who), [KEY_A]);

        // SAFETY: kill only sends a signal, to the stand-in this test's

        unsafe { libc::kill(ring.pid as libc::pid_t, libc::SIGKILL) };
        let started = rig.until(Duration::from_secs(10), |rig| rig.state.ring.running_now().is_some_and(|r| r.0 != ring.number));
        assert!(started, "no ring was started once the old one had gone");
        assert_eq!(rig.state.ring.counted(), 2);
    });
}
