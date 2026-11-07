//! Which kinds of app Discover, a kind and results list, and whether the AppImage warning is still wanted.

use std::path::Path;

use crate::common::files::{read_env, update_env};
use crate::toolkit::t;

const SHOW: &str = "apps_show";
const WARNING_OFF: &str = "appimage_warning_off";

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Show {
    #[default]
    Both,
    Flatpaks,
    AppImages,
}

impl Show {
    pub const ALL: [Show; 3] = [Show::Both, Show::Flatpaks, Show::AppImages];

    fn word(self) -> &'static str {
        match self {
            Show::Both => "both",
            Show::Flatpaks => "flatpaks",
            Show::AppImages => "appimages",
        }
    }

    fn read(word: &str) -> Show {
        Show::ALL.into_iter().find(|s| s.word() == word.trim()).unwrap_or_default()
    }

    pub fn flatpaks(self) -> bool {
        self != Show::AppImages
    }

    pub fn appimages(self) -> bool {
        self != Show::Flatpaks
    }

    pub fn name(self) -> &'static str {
        match self {
            Show::Both => t!("store-show-both"),
            Show::Flatpaks => t!("store-show-flatpaks"),
            Show::AppImages => t!("store-show-appimages"),
        }
    }
}

pub fn show_in(conf: &Path) -> Show {
    Show::read(read_env(conf).get(SHOW).map_or("", String::as_str))
}

pub fn set_show_in(conf: &Path, show: Show) -> std::io::Result<()> {
    update_env(conf, &[(SHOW, Some(show.word()))])
}

pub fn warning_off_in(conf: &Path) -> bool {
    read_env(conf).get(WARNING_OFF).is_some_and(|v| v.trim() == "yes")
}

pub fn set_warning_off_in(conf: &Path) -> std::io::Result<()> {
    update_env(conf, &[(WARNING_OFF, Some("yes"))])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-formats-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir.join("trindle.conf")
    }

    #[test]
    fn the_choice_is_kept_and_starts_on_both() {
        let conf = scratch("choice");
        assert_eq!(show_in(&conf), Show::Both);
        for show in Show::ALL {
            set_show_in(&conf, show).unwrap();
            assert_eq!(show_in(&conf), show);
        }
        update_env(&conf, &[("clock_format", Some("%H:%M"))]).unwrap();
        assert_eq!(show_in(&conf), Show::AppImages, "another setting leaves it be");
        update_env(&conf, &[(SHOW, Some("everything"))]).unwrap();
        assert_eq!(show_in(&conf), Show::Both);
    }

    #[test]
    fn each_choice_lists_what_it_names() {
        assert!(Show::Both.flatpaks() && Show::Both.appimages());
        assert!(Show::Flatpaks.flatpaks() && !Show::Flatpaks.appimages());
        assert!(!Show::AppImages.flatpaks() && Show::AppImages.appimages());
    }

    #[test]
    fn the_warning_is_wanted_until_it_is_ticked_away() {
        let conf = scratch("warning");
        assert!(!warning_off_in(&conf));
        set_warning_off_in(&conf).unwrap();
        assert!(warning_off_in(&conf));
    }
}
