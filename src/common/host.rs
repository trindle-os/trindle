//! Trindle's answers to what the toolkit asks of the program using it.

use crate::toolkit::host::{Act, Host};
use crate::toolkit::ui::Labels;

pub struct Trindle;

impl Host for Trindle {
    fn labels(&self) -> Labels {
        super::tip::chosen()
    }

    fn moving(&self) -> bool {
        super::graphics::moving()
    }

    fn drawing_thread(&self) {
        super::refused::drawing_thread();
    }

    fn missing_icon(&self, id: &str, shown: &str) {
        super::missing_icons::note(id, shown);
    }

    fn own_icon_set(&self) -> Option<&'static str> {
        Some(crate::icons::CIRCLES)
    }

    fn end_trial_if_over(&self) {
        super::themes::end_if_over();
    }

    fn asks_only_the_person(&self, name: &str) -> bool {
        super::person_only::asks(name)
    }

    fn act(&self, act: &str) -> Act {
        use super::person_only::{ACTS, WAITING};
        if WAITING.iter().any(|waiting| waiting.0 == act) {
            Act::Waiting
        } else if ACTS.iter().any(|listed| listed.0 == act) {
            Act::PersonsOwn
        } else {
            Act::Unknown
        }
    }

    fn reader_switched_on(&self) -> bool {
        use super::own_helpers::{on, Helper};
        on(Helper::Reader)
    }

    fn words(&self) -> &'static [(&'static str, &'static str)] {
        include!(concat!(env!("OUT_DIR"), "/english.rs"))
    }

    fn language(&self) -> Option<String> {
        Some(super::files::setting("language")).filter(|s| !s.trim().is_empty())
    }
}

pub extern "C" fn set() {
    crate::toolkit::host::set(&Trindle);
}

#[cfg(test)]
mod tests {
    use crate::toolkit::window::{by_reader, persons_own, reader_may_ask, with_named, with_press};

    #[test]
    fn the_toolkit_has_trindles_answers_from_the_start() {
        let host = crate::toolkit::host::host();
        assert_eq!(host.own_icon_set(), Some(crate::icons::CIRCLES));
        assert!(host.asks_only_the_person(super::super::person_only::PASSWORD));
        assert_eq!(host.act(super::super::person_only::STOP), crate::toolkit::host::Act::PersonsOwn);
    }

    fn named_presses() {
        use crate::common::person_only::STOP;
        let stop = || persons_own(STOP);
        assert!(with_named(true, Some(7), || with_press(Some(7), stop)), "the person's own press was refused");
        assert!(!with_named(true, Some(7), || with_press(Some(8), stop)), "a press the compositor never named did the act");
        assert!(!with_named(true, Some(7), stop));
        assert!(with_named(false, None, || with_press(Some(7), stop)), "under a compositor that names no presses, the person's own press was refused");
        assert!(with_named(false, None, stop));
        assert!(!with_named(true, None, || with_press(Some(7), stop)), "a press the compositor offered to name, and did not, did the act");
        assert!(!with_named(true, None, || by_reader(stop)), "a press asked over the accessibility bus did the act, with the screen reader not switched on by the person");
        assert!(!with_named(true, Some(7), || by_reader(stop)));
    }

    fn the_screen_reader_switch() {
        use crate::common::person_only::{ACTS, STOP};
        let stop = || persons_own(STOP);
        let dir = std::env::temp_dir().join(format!("trindle-window-reader-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        crate::common::own_helpers::kept_at(&dir.join("own-helpers"), || {
            use crate::common::own_helpers::Helper;
            use crate::common::own_helpers_set::switch;
            switch(Helper::Reader, true, false).unwrap();
            assert!(!with_named(true, None, || by_reader(stop)), "a screen reader switched on by a made-up press did the act");
            switch(Helper::Keyboard, true, true).unwrap();
            assert!(!with_named(true, None, || by_reader(stop)));
            switch(Helper::Reader, true, true).unwrap();
            assert!(with_named(true, None, || by_reader(stop)) && with_named(false, None, || by_reader(stop)), "a screen reader the person switched on was shut out");
            assert!(!with_named(true, None, stop));
            for (act, _, _) in ACTS {
                assert!(with_named(true, None, || by_reader(|| persons_own(act))), "{act}");
            }
            switch(Helper::Reader, false, false).unwrap();
            assert!(!with_named(true, None, || by_reader(stop)), "a screen reader switched off did the act");
        });
        let _ = std::fs::remove_dir_all(&dir);
        assert!(with_named(false, None, || by_reader(stop)));
        assert!(!with_named(true, None, stop));
    }

    fn every_act_on_the_list() {
        use crate::common::person_only::{ACTS, WAITING};
        for (act, _, _) in ACTS {
            assert!(!with_named(true, Some(7), || with_press(Some(8), || persons_own(act))), "{act}");
            assert!(with_named(true, Some(7), || with_press(Some(7), || persons_own(act))), "{act}");
        }
        for (act, _, _) in WAITING {
            assert!(with_named(true, None, || persons_own(act)), "{act} is waiting for the owner and was refused");
            assert!(!ACTS.iter().any(|a| a.0 == act), "{act} is on both lists");
        }
        assert!(!with_named(true, Some(7), || with_press(Some(7), || persons_own("no-such-act"))));
    }

    fn questions_only_the_person_may_answer() {
        use crate::common::person_only::{ASKED_BY_NAME, ASKED_BY_TRINDLE, PANELS_BY_NAME, WINDOWS};
        let dir = std::env::temp_dir().join(format!("trindle-window-asks-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let questions: Vec<&str> = ASKED_BY_TRINDLE.iter().chain(&ASKED_BY_NAME).chain(&WINDOWS).copied().collect();
        assert_eq!(questions.len(), 8);
        for name in &questions {
            assert!(!with_named(true, None, || reader_may_ask(name)), "the accessibility bus was let press in {name}, with the screen reader not switched on by the person");
            assert!(with_named(false, None, || reader_may_ask(name)), "{name}: under a compositor that names no presses it is left as it was");
        }
        for name in ["org.trindle.Settings", "org.example.Notes", "", PANELS_BY_NAME[0]] {
            assert!(with_named(true, None, || reader_may_ask(name)), "{name}");
        }
        crate::common::own_helpers::kept_at(&dir.join("own-helpers"), || {
            use crate::common::own_helpers::Helper;
            use crate::common::own_helpers_set::switch;
            switch(Helper::Reader, true, false).unwrap();
            switch(Helper::Keyboard, true, true).unwrap();
            assert!(questions.iter().all(|name| !with_named(true, None, || reader_may_ask(name))));
            switch(Helper::Reader, true, true).unwrap();
            assert!(questions.iter().all(|name| with_named(true, None, || reader_may_ask(name))), "a screen reader the person switched on was shut out of a question");
        });
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn the_one_place_the_bus_asks_and_the_newest_presses() {
        let ui = std::fs::read_to_string(crate::common::source::toolkit_root().join("src/ui/shown.rs")).unwrap();
        let ui = crate::common::source::code(Box::leak(ui.into_boxed_str()));
        assert!(ui.contains("if !super::window::reader_may_ask(&name) {") && ui.matches("super::window::by_reader(").count() == 1 && ui.matches(".ui.act(&req)").count() == 1,
                "what the accessibility bus asks is no longer done in one place, behind both rules");
        let mut seen = crate::common::own_press::Seen::default();
        (0..1000).for_each(|serial| seen.note(serial));
        assert!(seen.has(999) && !seen.has(0));
    }

    #[test]
    fn only_the_persons_own_press_does_an_act_on_the_list() {
        named_presses();
        the_screen_reader_switch();
        every_act_on_the_list();
        questions_only_the_person_may_answer();
        the_one_place_the_bus_asks_and_the_newest_presses();
    }
}
