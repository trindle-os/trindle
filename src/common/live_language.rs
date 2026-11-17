//! Changes the language a running program's words are in, to the nearest one Trindle has.

use std::path::PathBuf;
#[cfg(test)]
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use crate::toolkit::lang;

const LOCALE_MOST: usize = 64;

pub fn folders() -> Vec<PathBuf> {
    #[cfg(not(test))]
    return lang::folders();
    #[cfg(test)]
    vec![std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("share/lang")]
}

fn shipped(tags: Vec<String>) -> Vec<String> {
    let dirs = folders();
    tags.into_iter().filter(|t| dirs.iter().any(|d| d.join(t).is_dir())).collect()
}

pub fn for_locale(locale: &str) -> Vec<String> {
    if locale.len() > LOCALE_MOST {
        return Vec::new();
    }
    shipped(lang::tags_of(locale))
}

pub fn for_choice(own: &str, lang: &str) -> Vec<String> {
    let env = |k: &str| std::env::var(k).ok();
    let own = Some(own.to_string()).filter(|o| o.len() <= LOCALE_MOST);
    let lang = Some(lang.to_string()).filter(|l| l.len() <= LOCALE_MOST);
    shipped(lang::choice_from(own, env("LC_ALL"), env("LC_MESSAGES"), lang))
}

pub fn differs(tags: &[String]) -> bool {
    tags.first().map_or("en", String::as_str) != lang::chosen_tag()
}

pub fn switch(tags: Vec<String>) {
    #[cfg(not(test))]
    {

        lang::switch_to(&tags);
    }
    #[cfg(test)]
    {
        SWITCHED.with(|s| *s.borrow_mut() = Some(tags));
        GENERATION.with(|g| g.set(FRESH.fetch_add(1, Ordering::Relaxed)));
    }
}

#[cfg(not(test))]
fn generation() -> u64 {
    lang::generation()
}

#[cfg(test)]
static FRESH: AtomicU64 = AtomicU64::new(1);

#[cfg(test)]
thread_local! {
    static SWITCHED: std::cell::RefCell<Option<Vec<String>>> = const { std::cell::RefCell::new(None) };
    static GENERATION: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
fn generation() -> u64 {
    GENERATION.with(std::cell::Cell::get)
}

pub fn words_in_use() -> (String, u64) {
    (lang::chosen_tag(), generation())
}

#[cfg(test)]
pub fn switched() -> Option<Vec<String>> {
    SWITCHED.with(|s| s.borrow_mut().take())
}

pub type Kept<T> = Mutex<Vec<((String, u64), &'static T)>>;

pub fn per_language<T: Sync>(kept: &'static Kept<T>, make: impl FnOnce() -> T) -> &'static T {
    let now = words_in_use();
    let mut kept = kept.lock().unwrap_or_else(|e| e.into_inner());
    if let Some((_, made)) = kept.iter().find(|(had, _)| *had == now) {
        return made;
    }

    let made: &'static T = Box::leak(Box::new(make()));
    kept.push((now, made));
    made
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_locale_finds_the_nearest_language_trindle_has() {
        for (locale, want) in [("fr_FR.UTF-8", vec!["fr"]), ("pt_BR", vec!["pt-BR"]), ("pt_BR.UTF-8@euro", vec!["pt-BR"]),
                               ("zh_CN.UTF-8", vec!["zh-Hans"]), ("en_US.UTF-8", vec!["en-US", "en"]), ("ar_EG.UTF-8", vec!["ar"])] {
            assert_eq!(for_locale(locale), want, "{locale}");
        }
        for english in ["en_GB.UTF-8", "en"] {
            assert_eq!(for_locale(english), ["en"], "{english}: English, with any words of the person's own");
        }
        for none in ["C.UTF-8", "POSIX", "", "xx_YY.UTF-8", "tlh"] {
            assert!(for_locale(none).is_empty(), "{none} should be the built-in English");
        }
    }

    #[test]
    fn a_hostile_locale_names_no_folder() {
        for bad in ["../x", "../../etc", "fr/../../x", "fr_FR/..", "..", "/fr", "fr\\..", "fr\0"] {
            assert!(for_locale(bad).is_empty(), "{bad:?}");
        }
        assert!(for_locale(&format!("fr_FR{}", "-a".repeat(40))).is_empty(), "too long");
        assert_eq!(for_choice(&"x".repeat(100_000), "fr_FR.UTF-8"), for_choice("", "fr_FR.UTF-8"), "a huge setting is ignored");
    }

    #[test]
    fn the_own_setting_wins_over_the_system_and_auto_follows_it() {
        if std::env::var("LC_ALL").is_ok_and(|v| !v.is_empty()) || std::env::var("LC_MESSAGES").is_ok_and(|v| !v.is_empty()) {
            return;
        }
        assert_eq!(for_choice("de", "fr_FR.UTF-8"), vec!["de"]);
        assert_eq!(for_choice("auto", "fr_FR.UTF-8"), vec!["fr"]);
        assert_eq!(for_choice("", "zh_CN.UTF-8"), vec!["zh-Hans"]);
        assert_eq!(for_choice("en-GB", "fr_FR.UTF-8"), ["en"]);
    }

    #[test]
    fn per_language_makes_one_for_each_language() {
        static KEPT: Kept<String> = Mutex::new(Vec::new());
        let english = per_language(&KEPT, || lang::t("settings-window-title").to_string());
        let french = crate::common::shipped_languages::in_language("fr", || per_language(&KEPT, || lang::t("settings-window-title").to_string()));
        assert_ne!(english, french, "French is made in French, not handed the English");
        assert!(std::ptr::eq(english, per_language(&KEPT, || unreachable!())), "English is made once");
        assert!(differs(&["fr".to_string()]) && !differs(&[]) && !differs(&["en".to_string()]));
    }
}
