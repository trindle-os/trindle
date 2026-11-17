//! Answers asked for elsewhere by key, so "none" can be told from "not yet".

use super::app_page::Asked;
use std::collections::{HashMap, HashSet};

pub struct Heard<T> {
    got: HashMap<String, Option<T>>,
    heard: HashSet<String>,
}

impl<T> Default for Heard<T> {
    fn default() -> Self {
        Heard { got: HashMap::new(), heard: HashSet::new() }
    }
}

impl<T: Clone> Heard<T> {

    pub fn ask(&mut self, key: &str) -> bool {
        if self.got.contains_key(key) {
            return false;
        }
        self.got.insert(key.to_string(), None);
        true
    }

    pub fn answer(&mut self, key: String, got: Option<T>) {
        self.heard.insert(key.clone());
        self.got.insert(key, got);
    }

    pub fn asked(&self, key: &str) -> Asked<T> {
        match self.got.get(key) {
            Some(Some(t)) => Asked::Here(t.clone()),
            Some(None) if self.heard.contains(key) => Asked::Nothing,
            _ => Asked::Waiting,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn not_asked_waiting_nothing_and_here_are_told_apart() {
        let mut h: Heard<u8> = Heard::default();
        assert!(matches!(h.asked("a"), Asked::Waiting));
        assert!(h.ask("a") && !h.ask("a"), "asked once");
        assert!(matches!(h.asked("a"), Asked::Waiting));
        h.answer("a".into(), None);
        assert!(matches!(h.asked("a"), Asked::Nothing));
        assert!(!h.ask("a"), "an answer of none is not asked again");
        h.answer("b".into(), Some(7));
        assert!(matches!(h.asked("b"), Asked::Here(7)));
    }
}
