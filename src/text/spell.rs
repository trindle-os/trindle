//! Spelling: a Hunspell dictionary installed on the computer, read by
//! spellbook, and the words a person has said are right. Text brings no
//! dictionaries of its own.

use crate::toolkit::t;
use std::borrow::Cow;
use std::collections::HashSet;
use std::path::Path;

use encoding_rs::{Encoding, UTF_8, WINDOWS_1252};
use spellbook::Dictionary;

use crate::common::files;

pub const FOLDER: &str = "/usr/share/hunspell";

const SUGGESTIONS: usize = 6;

pub fn pick(installed: &[String], chosen: &str, system: Option<&str>) -> Option<String> {
    let has = |name: &str| installed.iter().find(|l| *l == name).cloned();
    if !chosen.is_empty() {
        if let Some(l) = has(chosen) {
            return Some(l);
        }
    }
    let system = system.map(|s| s.split(['.', '@']).next().unwrap_or("")).filter(|s| !s.is_empty() && *s != "C" && *s != "POSIX");
    let by_system = || {
        let s = system?;
        let lang = s.split('_').next().unwrap_or(s);
        has(s).or_else(|| installed.iter().find(|l| l.split('_').next() == Some(lang)).cloned())
    };
    has("en_GB").or_else(by_system).or_else(|| has("en_US")).or_else(|| installed.first().cloned())
}

pub fn system_lang() -> Option<String> {
    ["LC_ALL", "LC_MESSAGES", "LANG"].iter().find_map(|k| std::env::var(k).ok().filter(|v| !v.is_empty()))
}

pub fn installed(folder: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(folder)
        .map(|d| {
            d.flatten()
                .filter_map(|e| e.file_name().to_str().and_then(|n| n.strip_suffix(".dic")).map(String::from))
                .filter(|n| folder.join(format!("{n}.aff")).is_file())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

fn encoding(aff: &[u8]) -> &'static Encoding {
    aff.split(|b| *b == b'\n')
        .find_map(|l| l.strip_prefix(b"SET "))
        .and_then(|l| Encoding::for_label(l.trim_ascii()))
        .unwrap_or(UTF_8)
}

fn decode(bytes: &[u8], enc: &'static Encoding) -> String {
    let enc = if enc == UTF_8 && std::str::from_utf8(bytes).is_err() { WINDOWS_1252 } else { enc };
    enc.decode_with_bom_removal(bytes).0.into_owned()
}

pub struct Speller {
    pub lang: String,
    dict: Dictionary,
    own: HashSet<String>,
}

impl Speller {

    pub fn load(folder: &Path, lang: &str, own: &Path) -> Result<Speller, String> {
        if lang.is_empty() || !lang.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') {
            return Err(t!("text-no-dictionary", lang = lang));
        }
        let read = |ext: &str| {
            let path = folder.join(format!("{lang}.{ext}"));
            crate::common::files::read_file(&path).map_err(|e| t!("text-dictionary-unreadable", lang = lang, reason = e.to_string()))
        };
        let (aff, dic) = (read("aff")?, read("dic")?);
        let enc = encoding(&aff);
        let dict = Dictionary::new(&decode(&aff, enc), &decode(&dic, enc))
            .map_err(|e| t!("text-dictionary-unreadable", lang = lang, reason = e.to_string()))?;
        Ok(Speller { lang: lang.to_string(), dict, own: own_words(own) })
    }

    pub fn right(&self, word: &str) -> bool {

        let word: Cow<str> = if word.contains('\u{2019}') { Cow::Owned(word.replace('\u{2019}', "'")) } else { Cow::Borrowed(word) };
        self.own.contains(word.as_ref()) || self.own.contains(&word.to_lowercase()) || self.dict.check(&word)
    }

    pub fn suggest(&self, word: &str) -> Vec<String> {
        let mut out = Vec::new();
        self.dict.suggest(&word.replace('\u{2019}', "'"), &mut out);
        out.truncate(SUGGESTIONS);
        out
    }

    pub fn add(&mut self, word: &str, file: &Path) -> std::io::Result<()> {
        add_word(file, word)?;
        self.own.insert(word.to_string());
        Ok(())
    }
}

pub fn own_words(file: &Path) -> HashSet<String> {
    files::read_text(file).lines().map(str::trim).filter(|w| !w.is_empty()).map(String::from).collect()
}

pub(crate) fn add_word(file: &Path, word: &str) -> std::io::Result<()> {
    let mut text = files::read_small(file)?;
    if text.lines().any(|l| l.trim() == word) {
        return Ok(());
    }
    if !text.is_empty() && !text.ends_with('\n') {
        text.push('\n');
    }
    text.push_str(word);
    text.push('\n');
    files::write_text(file, &text)
}

#[cfg(test)]
mod tests {
    #[test]
    fn language_order() {
        let have = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let all = have(&["de_DE", "en_GB", "en_US", "fr_FR"]);
        assert_eq!(super::pick(&all, "", Some("fr_FR.UTF-8")).as_deref(), Some("en_GB"));
        assert_eq!(super::pick(&all, "fr_FR", Some("de_DE.UTF-8")).as_deref(), Some("fr_FR"));
        assert_eq!(super::pick(&all, "nl_NL", None).as_deref(), Some("en_GB"));
        let no_gb = have(&["de_AT", "en_US", "fr_FR"]);
        assert_eq!(super::pick(&no_gb, "", Some("de_DE.UTF-8")).as_deref(), Some("de_AT"));
        assert_eq!(super::pick(&no_gb, "", Some("C.UTF-8")).as_deref(), Some("en_US"));
        assert_eq!(super::pick(&have(&["fr_FR"]), "", None).as_deref(), Some("fr_FR"));
        assert_eq!(super::pick(&[], "en_GB", None), None);
    }

    use super::*;
    use std::path::PathBuf;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-text-spell-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn a_small_dictionary_of_its_own() {
        let dir = scratch("small");

        std::fs::write(dir.join("xx.aff"), b"SET ISO8859-1\nTRY esianrtolcdugmfphbyvkw\n\nSFX S Y 1\nSFX S 0 s .\n").unwrap();
        std::fs::write(dir.join("xx.dic"), b"3\ncaf\xe9/S\nhouse/S\nit's\n").unwrap();
        std::fs::write(dir.join("broken.dic"), b"1\nword\n").unwrap();
        assert_eq!(installed(&dir), vec!["xx".to_string()], "a dictionary needs both its files");
        let own = dir.join("words");
        let mut s = Speller::load(&dir, "xx", &own).unwrap();
        assert!(s.right("café") && s.right("cafés") && s.right("houses"));
        assert!(s.right("it\u{2019}s"), "a curly apostrophe is read as a straight one");
        assert!(!s.right("hous"));
        assert!(s.suggest("hous").contains(&"house".to_string()));
        s.add("gubbins", &own).unwrap();
        s.add("gubbins", &own).unwrap();
        assert!(s.right("gubbins") && s.right("Gubbins"), "a word added in small letters may start a sentence");
        s.add("Trindle", &own).unwrap();
        assert!(s.right("Trindle") && !s.right("trindle"), "a name added keeps its capital");
        assert_eq!(files::read_text(&own), "gubbins\nTrindle\n", "a word is kept once");
        assert!(Speller::load(&dir, "../xx", &own).is_err());
        assert!(Speller::load(&dir, "yy", &own).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_installed_english_dictionary() {

        let folder = Path::new(FOLDER);
        crate::common::needs::file(&folder.join("en_GB.dic"), "the British English dictionary (hunspell-en-gb)");
        let s = Speller::load(folder, "en_GB", Path::new("/nonexistent/words")).unwrap();
        assert!(s.right("colour") && !s.right("color"));
        assert!(s.right("isn't") && s.right("Paris"));
        assert!(!s.right("wrod"));
        assert!(s.suggest("wrod").contains(&"word".to_string()));
    }
}
