//! Typing pauses, counted for each box on its own, so a pause in one box never cancels the save of another.

use std::collections::HashMap;

#[derive(Default)]
pub struct Rounds(HashMap<String, u32>);

impl Rounds {

    pub fn next(&mut self, box_: &str) -> u32 {
        let n = self.0.entry(box_.to_string()).or_insert(0);
        *n = n.wrapping_add(1);
        *n
    }

    pub fn latest(&self, box_: &str, round: u32) -> bool {
        self.0.get(box_) == Some(&round)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typing_in_a_second_box_leaves_the_first_boxs_save_standing() {
        let mut r = Rounds::default();
        let first = r.next("one");
        let other = r.next("two");
        assert!(r.latest("one", first) && r.latest("two", other));
        let again = r.next("one");
        assert!(!r.latest("one", first) && r.latest("one", again) && r.latest("two", other));
    }
}
