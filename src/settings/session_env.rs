//! This person's own session environment, such as the keyboard layout, in ~/.config/trindle/environment.

use std::path::PathBuf;

use crate::common::files::{read_env, update_env, user_environment};

use super::ui::{Kit, not_saved};

pub fn path() -> PathBuf {
    user_environment()
}

pub fn get(key: &str) -> Option<String> {
    read_env(&path()).get(key).cloned()
}

pub fn update(changes: &[(&str, Option<&str>)], kit: &mut Kit, key: &str, success: &str) -> bool {
    let kept = kit.settle(key, update_env(&path(), changes), not_saved()).is_some();
    if kept {
        kit.say(key, success, true, false);
    }
    kept
}
