//! The languages Trindle can be shown in, each by its own name, and how many of the English words each has.

use std::collections::BTreeSet;

use crate::toolkit::lang::{self, Bundle};

pub const VARIANTS: [&str; 3] = ["en-GB", "en-GB-oxendict", "en-US"];

const OWN_NAMES: &[(&str, &str)] = &[
    ("en", "English"), ("en-GB", "English (British)"), ("en-GB-oxendict", "English (Oxford)"), ("en-US", "English (American)"),
    ("ar", "العربية"), ("bg", "Български"), ("ca", "Català"), ("cs", "Čeština"), ("cy", "Cymraeg"), ("da", "Dansk"),
    ("de", "Deutsch"), ("el", "Ελληνικά"), ("es", "Español"), ("et", "Eesti"), ("eu", "Euskara"), ("fa", "فارسی"),
    ("fi", "Suomi"), ("fr", "Français"), ("ga", "Gaeilge"), ("gd", "Gàidhlig"), ("gl", "Galego"), ("he", "עברית"),
    ("hi", "हिन्दी"), ("hr", "Hrvatski"), ("hu", "Magyar"), ("id", "Bahasa Indonesia"), ("is", "Íslenska"),
    ("it", "Italiano"), ("ja", "日本語"), ("ko", "한국어"), ("lt", "Lietuvių"), ("lv", "Latviešu"), ("nb", "Norsk bokmål"),
    ("nl", "Nederlands"), ("nn", "Norsk nynorsk"), ("pl", "Polski"), ("pt", "Português"), ("pt-BR", "Português (Brasil)"),
    ("ro", "Română"), ("ru", "Русский"), ("sk", "Slovenčina"), ("sl", "Slovenščina"), ("sr", "Српски"), ("sv", "Svenska"),
    ("th", "ไทย"), ("tr", "Türkçe"), ("uk", "Українська"), ("vi", "Tiếng Việt"), ("zh", "中文"),
    ("zh-Hans", "简体中文"), ("zh-Hant", "繁體中文"),
];

pub fn own_name(code: &str) -> String {
    OWN_NAMES.iter().find(|(c, _)| c.eq_ignore_ascii_case(code)).map(|(_, n)| n.to_string()).unwrap_or_else(|| code.to_string())
}

#[derive(Clone, Debug, PartialEq)]
pub struct Tongue {
    pub code: String,
    pub name: String,
    pub done: usize,
    pub all: usize,
    pub machine: bool,
}

impl Tongue {
    pub fn share(&self) -> f64 {
        if self.all == 0 { 1.0 } else { (self.done as f64 / self.all as f64).min(1.0) }
    }
}

fn is_code(name: &str) -> bool {
    let tags = lang::tags_of(name);
    tags.first().is_some_and(|t| t.eq_ignore_ascii_case(name))
}

pub fn codes_in(dirs: &[std::path::PathBuf]) -> Vec<String> {
    let mut found: BTreeSet<String> = ["en"].iter().chain(VARIANTS.iter()).map(|s| s.to_string()).collect();
    for dir in dirs {
        let Ok(read) = std::fs::read_dir(dir) else { continue };
        for entry in read.flatten() {
            let Some(name) = entry.file_name().to_str().map(str::to_string) else { continue };
            if is_code(&name) && entry.path().is_dir() && !lang::folder_files(&entry.path()).is_empty() {
                found.insert(lang::tags_of(&name).remove(0));
            }
        }
    }
    let mut codes: Vec<String> = found.into_iter().collect();
    codes.sort_by_key(|c| (own_name(c).to_lowercase(), c.clone()));
    codes
}

fn is_machine(code: &str, dirs: &[std::path::PathBuf]) -> bool {
    dirs.iter().any(|d| lang::folder_files(&d.join(code)).iter().any(|(_, text)| text.lines().take(4).any(|l| l.starts_with("# machine translated"))))
}

pub fn found(dirs: &[std::path::PathBuf], english: &Bundle) -> Vec<Tongue> {
    let all = english.len();
    codes_in(dirs)
        .into_iter()
        .map(|code| {
            let whole = code == "en" || VARIANTS.contains(&code.as_str());
            let done = if whole {
                all
            } else {
                let have = lang::load(&lang::tags_of(&code), dirs);
                english.keys().filter(|k| have.get(k).is_some()).count()
            };
            let machine = !whole && is_machine(&code, dirs);
            Tongue { name: own_name(&code), code, done, all, machine }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-tongues-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn languages_are_found_by_folder_and_shown_by_their_own_name_with_how_complete_they_are() {
        let dir = scratch("found");
        std::fs::create_dir_all(dir.join("de")).unwrap();
        std::fs::create_dir_all(dir.join("xx")).unwrap();
        std::fs::create_dir_all(dir.join("empty")).unwrap();
        std::fs::create_dir_all(dir.join("not a language")).unwrap();
        std::fs::write(dir.join("de/a.ftl"), "one = Eins\ntwo = Zwei\n").unwrap();
        std::fs::write(dir.join("xx/a.ftl"), "one = X\n").unwrap();
        std::fs::write(dir.join("not a language/a.ftl"), "one = X\n").unwrap();
        let english = Bundle::from_text("en", "one = One\ntwo = Two\nthree = Three\nfour = Four\n");
        let all = found(&[dir], &english);
        let get = |c: &str| all.iter().find(|t| t.code == c).cloned();
        let de = get("de").unwrap();
        assert_eq!((de.name.as_str(), de.done, de.all), ("Deutsch", 2, 4));
        assert_eq!(de.share(), 0.5);
        assert_eq!(get("xx").unwrap().name, "xx");
        assert!(get("empty").is_none() && get("not a language").is_none());
        for code in ["en", "en-GB", "en-GB-oxendict", "en-US"] {
            assert_eq!(get(code).unwrap().share(), 1.0, "{code}");
        }
        assert_eq!(get("en-US").unwrap().name, "English (American)");
    }
}
