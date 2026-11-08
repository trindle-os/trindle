//! When the apps' icons are made again, and how long the ring waits.

use std::time::Duration;

pub const WAIT: Duration = Duration::from_secs(5);

pub const MOST: usize = 2;

#[derive(Default)]
pub struct Runs {
    last: u64,

    out: Vec<(u64, bool)>,
    again: bool,
}

impl Runs {

    pub fn changed(&mut self) -> Option<u64> {
        if self.out.len() >= MOST || self.out.iter().any(|(_, late)| !late) {
            self.again = true;
            return None;
        }
        self.again = false;
        self.last += 1;
        self.out.push((self.last, false));
        Some(self.last)
    }

    pub fn late(&mut self, id: u64) -> (bool, Option<u64>) {
        match self.out.iter_mut().find(|(n, late)| *n == id && !late) {
            Some(run) => {
                run.1 = true;
                (true, self.waiting())
            }
            None => (false, None),
        }
    }

    pub fn back(&mut self, id: u64) -> Option<u64> {
        self.out.retain(|(n, _)| *n != id);
        self.waiting()
    }

    fn waiting(&mut self) -> Option<u64> {
        if self.again { self.changed() } else { None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_run_that_never_comes_back_holds_up_nothing_and_no_more_pile_up_behind_it() {
        let mut runs = Runs::default();

        let first = runs.changed().expect("the first change starts a run");
        assert_eq!(runs.back(first), None);

        let stuck = runs.changed().unwrap();
        assert_eq!((runs.changed(), runs.changed()), (None, None), "a second run was started beside one still out");

        let (read, next) = runs.late(stuck);
        assert!(read, "the apps were not read when the run was late");
        let second = next.expect("the change that waited was forgotten");

        assert_eq!(runs.changed(), None);
        assert_eq!(runs.late(second), (true, None), "a third run was started with two already out");
        for _ in 0..50 {
            assert_eq!(runs.changed(), None);
        }
        assert_eq!(runs.out.len(), MOST);

        assert_eq!(runs.late(second), (false, None));
        assert_eq!(runs.late(first), (false, None));

        let third = runs.back(stuck).expect("the changes that waited got no run once one could be made");
        assert_eq!(runs.changed(), None, "one is out and not late");
        assert_eq!(runs.back(second), None);
        assert_eq!(runs.back(third).map(|_| ()), Some(()), "the change made while the third was out was forgotten");
    }

    #[test]
    fn the_ring_starts_the_icons_program_in_one_place() {
        for (file, code) in [("mod.rs", crate::common::source::code(include_str!("mod.rs"))), ("view", crate::ring::view::tests::view_code())] {
            let code = code.split("\n#[cfg(test)]").next().unwrap();
            let most = if file == "mod.rs" { 1 } else { 0 };
            assert_eq!(code.matches("\"trindle-icons\"").count(), most, "{file} starts the icons program for itself");
        }
        let code = crate::common::source::code(include_str!("mod.rs"));
        assert!(code.contains("self.icons.borrow_mut().changed()") && code.contains("self.icons.borrow_mut().late(id)") && code.contains("self.icons.borrow_mut().back(id)"));
    }
}
