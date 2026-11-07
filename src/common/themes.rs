//! Themes.

use std::path::Path;

use super::files;
use super::sheet::{self, Values, LOOK_KEY, NAMING, NAMING_KEY, THEME_KEY, THEME_MOST, TRIAL_FOR, TRIAL_KEY};

const MOST: usize = 40;
const LOOKED_AT: usize = 400;
pub const TOO_BIG: &str = "This file is too big to be a theme, so it is not used. A theme is at most 64 KB.";
pub const NOT_A_SHEET: &str = "This is not an ordinary file of words, so it cannot be a theme and is not used.";
pub const GONE: &str = "Its file is no longer there.";
pub const TOO_SLOW: &str = "Its file took too long to read, so it is not used.";
pub const LIST_SLOW: &str = "The folders of themes took too long to read, so only Trindle's own look is listed.";
pub const OWN_TOO_BIG: &str = "Your own style sheet, style.css, is over 256 KB, so it is not used.";
pub const OWN_NOT_A_SHEET: &str = "Your own style sheet, style.css, is not an ordinary file of words, so it is not used.";
pub const OWN_TOO_SLOW: &str = "Your own style sheet, style.css, took too long to read, so it is not used.";
pub const OWN_HARD_TO_READ: &str = "Some colours are hard to read.";
pub const OWN_NO_TARGET: &str = "Your own style sheet, style.css, is a link to a file that is not there, so it is not used.";
const LIST_WITHIN: std::time::Duration = std::time::Duration::from_secs(2);

pub fn why(unread: sheet::Unread) -> &'static str {
    match unread {
        sheet::Unread::TooBig => TOO_BIG,
        sheet::Unread::NotASheet | sheet::Unread::NoTarget => NOT_A_SHEET,
        sheet::Unread::TooSlow => TOO_SLOW,
        sheet::Unread::Gone => GONE,
    }
}

pub fn own_sheet_trouble() -> Option<&'static str> {
    sheet::refused().own.map(|unread| match unread {
        sheet::Unread::TooBig => OWN_TOO_BIG,
        sheet::Unread::TooSlow => OWN_TOO_SLOW,
        sheet::Unread::NotASheet | sheet::Unread::Gone => OWN_NOT_A_SHEET,
        sheet::Unread::NoTarget => OWN_NO_TARGET,
    })
}

pub const NOT_ALL: &str = "A folder of themes may hold more than are listed: only the first forty themes in a folder are shown, and only its first 400 files are looked at.";

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub id: String,
    pub name: String,
    pub about: String,
    pub author: String,
    pub version: String,
    pub mine: bool,
    pub css: String,
    pub only: Option<bool>,
    pub refused: Option<&'static str>,
}

fn look_word(light: bool) -> &'static str {
    if light { "light" } else { "dark" }
}

impl Theme {
    pub fn looks(&self) -> &'static [bool] {
        match self.only {
            None => &[false, true],
            Some(false) => &[false],
            Some(true) => &[true],
        }
    }

    pub fn light(&self, asked: bool) -> bool {
        self.only.unwrap_or(asked)
    }
}

fn word(values: &Values, key: &str, most: usize) -> String {
    let said = values.get(key, false).unwrap_or("").trim();
    let said = said.strip_prefix('"').and_then(|s| s.strip_suffix('"')).or_else(|| said.strip_prefix('\'').and_then(|s| s.strip_suffix('\''))).unwrap_or(said);
    said.chars().filter(|c| !c.is_control()).take(most).collect::<String>().trim().to_string()
}

fn theme(id: &str, css: &str, mine: bool) -> Theme {
    let v = Values::parse(css);
    let name = word(&v, "trindle-theme-name", 40);
    let name = if !name.is_empty() {
        name
    } else {
        let mut c = id.replace('-', " ");
        if let Some(first) = c.get_mut(0..1) {
            first.make_ascii_uppercase();
        }
        c
    };
    Theme { id: id.to_string(), name, about: word(&v, "trindle-theme-about", 120), author: word(&v, "trindle-theme-author", 60),
            version: word(&v, "trindle-theme-version", 20), mine, css: if id.is_empty() { String::new() } else { css.to_string() },
            only: if id.is_empty() { None } else { sheet::one_look(css) }, refused: None }
}

fn refused(id: &str, mine: bool, why: &'static str) -> Theme {
    Theme { refused: Some(why), ..theme(id, "", mine) }
}

#[cfg(test)]
pub(crate) fn from_sheet(id: &str, css: &str) -> Theme {
    theme(id, css, false)
}

fn ids_in(folder: &Path) -> (Vec<String>, bool) {
    ids_among(std::fs::read_dir(folder).into_iter().flatten().flatten().map(|e| e.file_name().to_string_lossy().into_owned()))
}

fn ids_among(names: impl Iterator<Item = String>) -> (Vec<String>, bool) {
    let mut looked = 0;
    let mut ids: Vec<String> = names
        .take(LOOKED_AT)
        .inspect(|_| looked += 1)
        .filter_map(|n| n.strip_suffix(".css").map(str::to_string))
        .filter(|id| sheet::theme_id(id))
        .collect();
    let more = looked >= LOOKED_AT || ids.len() > MOST;
    ids.sort();
    ids.truncate(MOST);
    (ids, more)
}

pub fn all() -> Vec<Theme> {
    listed().themes
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Listed {
    pub themes: Vec<Theme>,
    pub said: Vec<&'static str>,
}

pub fn listed() -> Listed {
    let (system, own) = (sheet::system_themes(), sheet::own_themes());
    sheet::within(LIST_WITHIN, move || listed_in(&system, &own))
        .unwrap_or_else(|| Listed { themes: vec![theme("", sheet::CSS, false)], said: vec![LIST_SLOW] })
}

fn listed_in(system: &Path, own: &Path) -> Listed {
    let (themes, more) = list_of(system, own);
    Listed { themes, said: if more { vec![NOT_ALL] } else { Vec::new() } }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Seen {
    pub listed: Listed,
    pub in_use: String,
    pub kept: String,
    pub unusable: Option<(String, &'static str)>,
    pub own: Option<&'static str>,
    pub hard: bool,
}

pub fn seen() -> Seen {
    let listed = listed();
    let (using, kept) = (sheet::theme_in_use(), sheet::chosen_theme());
    let words = sheet::theme_words(&using);
    let kept_reads = if kept == using { words.is_ok() } else { sheet::theme_words(&kept).is_ok() };
    Seen { listed, in_use: if words.is_ok() { using.clone() } else { String::new() }, kept: if kept_reads { kept } else { String::new() },
           unusable: unusable_as(&using, words), own: own_sheet_trouble(),
           hard: crate::toolkit::theme::own_colours_fall_short(&sheet::layers(), crate::common::style::DEFAULT_ACCENT) }
}

fn unusable_as(id: &str, words: Result<String, sheet::Unread>) -> Option<(String, &'static str)> {
    if !sheet::theme_id(id) {
        return None;
    }
    Some((theme(id, "", false).name, why(words.err()?)))
}

#[cfg(test)]
fn unusable_in(id: &str, own: &Path, system: &Path) -> Option<(String, &'static str)> {
    unusable_as(id, sheet::theme_words_in(id, own, system))
}

#[cfg(test)]
fn list(system: &Path, own: &Path) -> Vec<Theme> {
    list_of(system, own).0
}

fn list_of(system: &Path, own: &Path) -> (Vec<Theme>, bool) {
    let mut out = vec![theme("", sheet::CSS, false)];
    let mut more = false;
    for (folder, is_mine) in [(system, false), (own, true)] {
        let (ids, cut) = ids_in(folder);
        more |= cut;
        for id in ids {
            if sheet::theme_in(folder, &id).is_none() || (!is_mine && sheet::theme_in(own, &id).is_some()) {
                continue;
            }
            match sheet::read_sheet(&folder.join(format!("{id}.css")), THEME_MOST) {
                Ok(css) => out.push(theme(&id, &css, is_mine)),
                Err(unread) => out.push(refused(&id, is_mine, why(unread))),
            }
        }
    }
    (out, more)
}

pub fn chosen() -> String {
    let id = sheet::chosen_theme();
    if sheet::theme_words(&id).is_ok() { id } else { String::new() }
}

fn no_such() -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::NotFound, "There is no theme of that name.")
}

fn usable(id: &str) -> std::io::Result<()> {
    if id.is_empty() {
        return Ok(());
    }
    usable_as(sheet::theme_words(id))
}

#[cfg(test)]
fn usable_in(id: &str, file: Option<&Path>) -> std::io::Result<()> {
    if id.is_empty() {
        return Ok(());
    }
    usable_as(file.map_or(Err(sheet::Unread::Gone), |f| if f.symlink_metadata().is_ok() { sheet::read_sheet(f, THEME_MOST) } else { Err(sheet::Unread::Gone) }))
}

fn usable_as(words: Result<String, sheet::Unread>) -> std::io::Result<()> {
    match words {
        Ok(_) => Ok(()),
        Err(sheet::Unread::Gone) => Err(no_such()),
        Err(unread) => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, why(unread))),
    }
}

fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

fn trial_line(id: &str, now: u64, look: Option<bool>) -> String {
    let line = format!("{}@{}", if id.is_empty() { "default" } else { id }, now + TRIAL_FOR);
    match look {
        Some(light) => format!("{line}@{}", look_word(light)),
        None => line,
    }
}

pub fn try_out(id: &str, look: Option<bool>) -> std::io::Result<()> {
    usable(id)?;
    files::update_env(&files::trindle_conf(), &[(TRIAL_KEY, Some(&trial_line(id, now(), look)))])
}

pub fn keep() -> std::io::Result<()> {
    let Some((id, _)) = sheet::trial() else { return Err(no_such()) };
    files::update_env(&files::trindle_conf(), &kept(&id, sheet::trial_look()))
}

pub fn set_look(light: bool) -> std::io::Result<()> {
    files::update_env(&files::trindle_conf(), &[(LOOK_KEY, Some(look_word(light)))])
}

pub fn go_back() -> std::io::Result<()> {
    files::update_env(&files::trindle_conf(), &[(TRIAL_KEY, None)])
}

pub fn end_if_over() -> bool {
    let said = files::setting(TRIAL_KEY);
    if said.is_empty() || sheet::trial_of(&said, now()).is_some() {
        return false;
    }
    go_back().is_ok()
}

pub fn choose(id: &str) -> std::io::Result<()> {
    usable(id)?;
    files::update_env(&files::trindle_conf(), &kept(id, None))
}

fn kept(id: &str, look: Option<bool>) -> Vec<(&'static str, Option<&str>)> {
    let mut all = vec![(THEME_KEY, (!id.is_empty()).then_some(id)), (NAMING_KEY, Some(NAMING)), (TRIAL_KEY, None)];
    if let Some(light) = look {
        all.push((LOOK_KEY, Some(look_word(light))));
    }
    all
}

fn id_of(file: &Path) -> String {
    let stem = file.file_stem().map(|s| s.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
    let mut id = String::new();
    for c in stem.chars() {
        if c.is_ascii_lowercase() || c.is_ascii_digit() {
            id.push(c);
        } else if !id.ends_with('-') && !id.is_empty() {
            id.push('-');
        }
    }
    let id: String = id.trim_end_matches('-').chars().take(36).collect();
    let id = id.trim_end_matches('-').to_string();
    if sheet::theme_id(&id) { id } else { "theme".to_string() }
}

pub fn add(file: &Path) -> Result<String, String> {
    add_to(file, &sheet::own_themes(), &sheet::system_themes())
}

fn add_to(file: &Path, own: &Path, system: &Path) -> Result<String, String> {
    let name = file.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    if !name.to_ascii_lowercase().ends_with(".css") {
        return Err(format!("{name} is not a theme. A theme is one file whose name ends in .css."));
    }
    let size = std::fs::metadata(file).map_err(|_| format!("{name} could not be read."))?.len();
    if size > THEME_MOST {
        return Err(format!("{name} is too big to be a theme. A theme is at most 64 KB."));
    }
    let css = sheet::read_sheet(file, THEME_MOST).map_err(|why| match why {
        sheet::Unread::TooBig => format!("{name} is too big to be a theme. A theme is at most 64 KB."),
        sheet::Unread::NotASheet | sheet::Unread::Gone | sheet::Unread::NoTarget => format!("{name} is not a style sheet: it holds more than words."),
        sheet::Unread::TooSlow => format!("{name} took too long to read."),
    })?;
    crate::toolkit::theme::check(&css).map_err(|f| f.words())?;
    let base = id_of(file);
    let mut id = base.clone();
    let mut n = 2;
    while system.join(format!("{id}.css")).exists() {
        id = format!("{base}-{n}");
        n += 1;
    }
    keep_file(&own.join(format!("{id}.css")), &css).map_err(|e| format!("The theme could not be kept: {e}."))?;
    Ok(id)
}

fn keep_file(to: &Path, css: &str) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    let Some(folder) = to.parent() else { return Err(no_such()) };
    std::fs::create_dir_all(folder)?;
    let tmp = to.with_extension("css-new");
    let _ = std::fs::remove_file(&tmp);
    let mut f = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o644).custom_flags(libc::O_NOFOLLOW).open(&tmp)?;
    f.write_all(css.as_bytes())?;
    drop(f);
    std::fs::rename(&tmp, to)
}

pub fn remove(id: &str) -> std::io::Result<()> {
    if !sheet::theme_id(id) {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "There is no theme of that name."));
    }
    if sheet::chosen_theme() == id {
        choose("")?;
    } else if sheet::trial().is_some_and(|(on_trial, _)| on_trial == id) {
        go_back()?;
    }
    std::fs::remove_file(sheet::own_themes().join(format!("{id}.css")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn room(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-themes-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("own")).unwrap();
        std::fs::create_dir_all(dir.join("system")).unwrap();
        dir
    }

    const HARBOUR: &str = ":root {\n  --trindle-theme-name: \"Harbour\";\n  --trindle-theme-about: \"Cool greys.\";\n  \
                           --trindle-theme-author: \"John Smith\";\n  --trindle-theme-version: \"1.0\";\n  --trindle-page: #101820;\n}\n";

    #[test]
    fn trindles_own_look_is_a_theme_like_any_other_and_always_first() {
        let dir = room("first");
        let all = list(&dir.join("system"), &dir.join("own"));
        assert_eq!(all.len(), 1);
        assert_eq!((all[0].id.as_str(), all[0].name.as_str(), all[0].mine), ("", "Trindle", false));
        assert!(!all[0].about.is_empty() && !all[0].version.is_empty());
        assert_eq!(list(&dir.join("none"), &dir.join("nor-this")).len(), 1);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn themes_are_found_in_both_folders_and_say_who_they_are() {
        let dir = room("found");
        std::fs::write(dir.join("system/harbour.css"), HARBOUR).unwrap();
        std::fs::write(dir.join("own/my-own.css"), ":root { --trindle-card: #223344; }").unwrap();
        std::fs::write(dir.join("own/notes.txt"), HARBOUR).unwrap();
        std::fs::write(dir.join("own/Bad Name.css"), HARBOUR).unwrap();
        std::fs::create_dir_all(dir.join("own/folder.css")).unwrap();
        let all = list(&dir.join("system"), &dir.join("own"));
        let seen: Vec<(&str, &str, bool)> = all.iter().map(|t| (t.id.as_str(), t.name.as_str(), t.mine)).collect();
        assert_eq!(seen, [("", "Trindle", false), ("harbour", "Harbour", false), ("my-own", "My own", true)]);
        assert_eq!((all[1].about.as_str(), all[1].author.as_str(), all[1].version.as_str()), ("Cool greys.", "John Smith", "1.0"));
        std::fs::write(dir.join("own/harbour.css"), ":root { --trindle-theme-name: 'Mine'; --trindle-page: #000000; }").unwrap();
        let all = list(&dir.join("system"), &dir.join("own"));
        assert_eq!(all.iter().filter(|t| t.id == "harbour").map(|t| (t.name.as_str(), t.mine)).collect::<Vec<_>>(), [("Mine", true)]);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn what_a_theme_says_about_itself_is_made_safe_to_show() {
        let long = "x".repeat(500);
        let css = format!(":root {{ --trindle-theme-name: \"{long}\"; --trindle-theme-about: one\u{7}two\u{1b}[31m; --trindle-page: #000; }}");
        let t = theme("odd", &css, true);
        assert_eq!(t.name.chars().count(), 40);
        assert!(!t.about.chars().any(char::is_control), "{:?}", t.about);
        assert_eq!(theme("plain-file", ":root { --trindle-page: #000; }", true).name, "Plain file");
    }

    #[test]
    fn a_good_sheet_is_added_and_a_bad_one_refused_in_plain_words() {
        let dir = room("add");
        let (own, system) = (dir.join("own"), dir.join("system"));
        let from = dir.join("My Harbour (2).css");
        std::fs::write(&from, HARBOUR).unwrap();
        assert_eq!(add_to(&from, &own, &system).as_deref(), Ok("my-harbour-2"));
        assert_eq!(std::fs::read_to_string(own.join("my-harbour-2.css")).unwrap(), HARBOUR);
        assert_eq!(add_to(&from, &own, &system).as_deref(), Ok("my-harbour-2"));
        std::fs::write(system.join("harbour.css"), HARBOUR).unwrap();
        let same = dir.join("harbour.css");
        std::fs::write(&same, HARBOUR).unwrap();
        assert_eq!(add_to(&same, &own, &system).as_deref(), Ok("harbour-2"));
        for name in ["default.css", "Trindle.css", "---.css"] {
            let f = dir.join(name);
            std::fs::write(&f, HARBOUR).unwrap();
            assert_eq!(add_to(&f, &own, &system).as_deref(), Ok("theme"), "{name}");
        }
        let refused = |name: &str, bytes: &[u8]| {
            let f = dir.join(name);
            std::fs::write(&f, bytes).unwrap();
            add_to(&f, &own, &system).unwrap_err()
        };
        assert!(refused("picture.png", HARBOUR.as_bytes()).contains("ends in .css"));
        assert!(refused("big.css", &vec![b' '; 70 * 1024]).contains("too big"));
        assert!(refused("binary.css", &[0xff, 0xfe, 0x00, 0x41]).contains("not a style sheet"));
        assert_eq!(refused("wrong.css", b":root {\n  --trindle-page: #101820;\n  --trindle-card: porridge;\n}\n"),
                   "Line 3: --trindle-card is not a colour Trindle can read.");
        assert!(refused("empty.css", b"body { color: red; }").contains("Nothing in the file"));
        assert!(add_to(&dir.join("not-there.css"), &own, &system).unwrap_err().contains("could not be read"));
        assert_eq!(ids_in(&own).0, ["harbour-2", "my-harbour-2", "theme"]);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_theme_is_kept_as_a_plain_file_and_never_through_a_link() {
        use std::os::unix::fs::PermissionsExt;
        let dir = room("links");
        let (own, system) = (dir.join("own"), dir.join("system"));
        let theirs = dir.join("precious.txt");
        std::fs::write(&theirs, "keep me").unwrap();
        std::os::unix::fs::symlink(&theirs, own.join("harbour.css")).unwrap();
        assert_eq!(sheet::read_sheet(&own.join("harbour.css"), THEME_MOST), Err(sheet::Unread::NotASheet));
        let all = list(&system, &own);
        assert_eq!(all.len(), 2);
        assert_eq!((all[1].id.as_str(), all[1].refused, all[1].css.as_str(), all[1].name.as_str()), ("harbour", Some(NOT_A_SHEET), "", "Harbour"));
        assert!(!format!("{:?}", all[1]).contains("keep me"));
        let from = dir.join("harbour.css");
        std::fs::write(&from, HARBOUR).unwrap();
        std::fs::set_permissions(&from, std::fs::Permissions::from_mode(0o777)).unwrap();
        assert_eq!(add_to(&from, &own, &system).as_deref(), Ok("harbour"));
        assert_eq!(std::fs::read_to_string(&theirs).unwrap(), "keep me");
        let kept = std::fs::symlink_metadata(own.join("harbour.css")).unwrap();
        assert!(kept.is_file() && !kept.file_type().is_symlink());
        assert_eq!(kept.permissions().mode() & 0o133, 0, "{:o}", kept.permissions().mode());
        let link = dir.join("offered.css");
        std::os::unix::fs::symlink("/etc/passwd", &link).unwrap();
        assert!(add_to(&link, &own, &system).is_err());
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_trial_ends_by_the_clock_alone() {
        let now = 1_800_000_000;
        let line = trial_line("harbour", now, None);
        assert_eq!(line, format!("harbour@{}", now + TRIAL_FOR));
        assert_eq!(sheet::trial_of(&line, now), Some(("harbour".into(), now + TRIAL_FOR)));
        assert_eq!(sheet::trial_of(&line, now + TRIAL_FOR - 1), Some(("harbour".into(), now + TRIAL_FOR)));
        assert_eq!(sheet::trial_of(&line, now + TRIAL_FOR), None);
        assert_eq!(sheet::trial_of(&line, now + 86_400), None);
        assert_eq!(sheet::trial_of(&trial_line("", now, None), now + 5), Some((String::new(), now + TRIAL_FOR)));
        assert_eq!(sheet::trial_of(&line, now - 600), None);
        for bad in ["", "harbour", "harbour@", "@5", "harbour@soon", "../x@1800000010", "Harbour@1800000010", "a@-1", "a@1e9",
                    "a@99999999999999999999999"] {
            assert_eq!(sheet::trial_of(bad, now), None, "{bad}");
        }
    }

    #[test]
    fn a_trial_names_its_look_and_only_keep_saves_it() {
        let now = 1_800_000_000;
        for (id, look, line) in [("harbour", Some(true), "harbour@1800000030@light"), ("", Some(false), "default@1800000030@dark"),
                                 ("harbour", None, "harbour@1800000030")] {
            assert_eq!(trial_line(id, now, look), line);
            assert_eq!(sheet::trial_of(line, now), Some((id.to_string(), now + TRIAL_FOR)), "{line}");
            assert_eq!(sheet::trial_look_of(line, now + 5), look, "{line}");
            assert_eq!(sheet::trial_look_of(line, now + TRIAL_FOR), None, "{line}");
        }
        for bad in ["harbour@1800000030@", "harbour@1800000030@Light", "harbour@1800000030@both", "harbour@1800000030@light@x",
                    "harbour@light", "@1800000030@light"] {
            assert_eq!((sheet::trial_of(bad, now), sheet::trial_look_of(bad, now)), (None, None), "{bad}");
        }
        assert_eq!(LOOK_KEY, "theme");
        assert_eq!(kept("harbour", Some(true)).last(), Some(&(LOOK_KEY, Some("light"))));
        assert_eq!(kept("", Some(false)).last(), Some(&(LOOK_KEY, Some("dark"))));
        assert!(kept("harbour", None).iter().all(|(key, _)| *key != LOOK_KEY));
    }

    #[test]
    fn a_theme_says_which_looks_it_has() {
        let both = ":root { --trindle-page: #101010; } @media (prefers-color-scheme: light) { :root { --trindle-page: #fafafa; } }";
        for (css, looks) in [
            (HARBOUR, &[false][..]),
            (":root { --trindle-theme-look: dark; --trindle-page: #101820; }", &[false]),
            (":root { --trindle-theme-look: light; --trindle-page: #ecebe6; }", &[true]),
            (":root { --trindle-theme-look: \"Light\"; --trindle-page: #ecebe6; }", &[true]),
            (both, &[false, true]),
            (":root { --trindle-theme-look: both; --trindle-card-corner: 20; }", &[false, true]),
            (&format!("{both} :root {{ --trindle-theme-look: dark; }}"), &[false]),
            (&format!("{both} :root {{ --trindle-theme-look: sideways; }}"), &[false, true]),
        ] {
            let t = theme("some", css, true);
            assert_eq!(t.looks(), looks, "{css}");
            for asked in [false, true] {
                let drawn = t.light(asked);
                assert!(t.looks().contains(&drawn), "{css}");
                assert_eq!(drawn, sheet::Layers::themed(sheet::CSS, css, "").light(asked), "{css}");
                if t.looks().len() == 2 {
                    assert_eq!(drawn, asked, "{css}");
                }
            }
        }
        assert_eq!(theme("", sheet::CSS, false).looks(), [false, true]);
        assert_eq!(sheet::one_look(""), None);
    }

    #[test]
    fn a_choice_of_compact_kept_before_the_rename_is_snug() {
        assert_eq!(sheet::kept_theme("compact", ""), "snug");
        assert_eq!(sheet::kept_theme("compact", NAMING), "snug");
        assert_eq!(sheet::kept_theme("snug", NAMING), "snug");
        let css = include_str!("../../share/themes/snug.css");
        assert!(css.contains("--trindle-theme-name: \"Snug\""), "{css}");
        assert!(!css.contains("Compact:"));
    }

    #[test]
    fn a_choice_of_midnight_kept_before_the_looks_changed_places_is_trindles_own() {
        assert_eq!(sheet::kept_theme("midnight", ""), "");
        assert_eq!(sheet::kept_theme("midnight", NAMING), "midnight");
        for id in ["paper", "bold", "snug", "harbour", ""] {
            assert_eq!(sheet::kept_theme(id, ""), id);
            assert_eq!(sheet::kept_theme(id, NAMING), id);
        }
        assert_eq!(sheet::kept_theme("../x", NAMING), "");
        assert_eq!(kept("midnight", None), [(THEME_KEY, Some("midnight")), (NAMING_KEY, Some(NAMING)), (TRIAL_KEY, None)]);
        assert_eq!(kept("", None)[0], (THEME_KEY, None));
    }

    #[test]
    fn a_folder_stuffed_with_files_is_read_only_so_far() {
        let dir = room("many");
        for i in 0..60 {
            std::fs::write(dir.join(format!("own/t{i:02}.css")), ":root { --trindle-page: #000; }").unwrap();
        }
        assert_eq!(list(&dir.join("system"), &dir.join("own")).len(), 1 + MOST);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn an_id_can_never_name_a_file_outside_the_two_folders() {
        for bad in ["", "../x", "a/b", "a.b", "A", "x y", "..", &"a".repeat(41), "a\0b"] {
            assert!(!sheet::theme_id(bad), "{bad:?}");
            assert_eq!(sheet::theme_file(bad), None);
        }
        assert!(sheet::theme_id("my-theme-2"));
        let dir = room("kinds");
        let not = Err(sheet::Unread::NotASheet);
        assert_eq!(sheet::read_sheet(&dir, THEME_MOST), not);
        assert_eq!(sheet::read_sheet(Path::new("/dev/zero"), THEME_MOST), not);
        assert_eq!(sheet::read_sheet(&dir.join("gone.css"), THEME_MOST), not);
        std::fs::write(dir.join("binary.css"), [0xff, 0xfe, 0x00, 0x41]).unwrap();
        assert_eq!(sheet::read_sheet(&dir.join("binary.css"), THEME_MOST), not);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_file_that_cannot_be_a_theme_is_a_tile_that_says_so_and_hides_nothing_quietly() {
        let dir = room("not-sheets");
        let (own, system) = (dir.join("own"), dir.join("system"));
        std::fs::write(system.join("bold.css"), HARBOUR).unwrap();
        std::fs::write(system.join("paper.css"), HARBOUR).unwrap();
        std::fs::write(system.join("snug.css"), HARBOUR).unwrap();
        std::os::unix::fs::symlink(dir.join("nowhere.css"), own.join("bold.css")).unwrap();
        std::os::unix::fs::symlink(system.join("paper.css"), own.join("linked.css")).unwrap();
        let pipe = std::ffi::CString::new(own.join("pipe.css").to_str().unwrap()).unwrap();
        // SAFETY: mkfifo only reads the path it is handed, which ends in a nul.
        assert_eq!(unsafe { libc::mkfifo(pipe.as_ptr(), 0o600) }, 0);
        std::fs::write(own.join("binary.css"), [0xff, 0xfe, 0x00, 0x41]).unwrap();
        std::fs::create_dir_all(own.join("snug.css")).unwrap();
        let all = list(&system, &own);
        let seen: Vec<(&str, bool, Option<&str>)> = all.iter().map(|t| (t.id.as_str(), t.mine, t.refused)).collect();
        assert_eq!(seen, [("", false, None), ("paper", false, None), ("snug", false, None), ("binary", true, Some(NOT_A_SHEET)),
                          ("bold", true, Some(NOT_A_SHEET)), ("linked", true, Some(NOT_A_SHEET)), ("pipe", true, Some(NOT_A_SHEET))]);
        assert_eq!(sheet::theme_file_in("bold", &own, &system), Some(own.join("bold.css")));
        assert_eq!(sheet::theme_file_in("pipe", &own, &system), Some(own.join("pipe.css")));
        assert_eq!(sheet::theme_file_in("snug", &own, &system), Some(system.join("snug.css")));
        assert_eq!(sheet::theme_file_in("paper", &own, &system), Some(system.join("paper.css")));
        assert_eq!(sheet::theme_file_in("none", &own, &system), None);
        for id in ["bold", "linked", "pipe", "binary"] {
            let no = usable_in(id, sheet::theme_file_in(id, &own, &system).as_deref()).unwrap_err();
            assert_eq!(no.to_string(), NOT_A_SHEET, "{id}");
        }
        assert!(!NOT_A_SHEET.contains('\u{2014}') && NOT_A_SHEET.ends_with('.'));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_theme_in_use_that_cannot_be_used_is_said_with_the_reason() {
        let dir = room("unusable");
        let (own, system) = (dir.join("own"), dir.join("system"));
        std::fs::write(system.join("harbour.css"), HARBOUR).unwrap();
        std::fs::write(own.join("grown.css"), vec![b' '; THEME_MOST as usize + 1]).unwrap();
        std::os::unix::fs::symlink(dir.join("nowhere.css"), own.join("harbour-2.css")).unwrap();
        assert_eq!(unusable_in("grown", &own, &system), Some(("Grown".to_string(), TOO_BIG)));
        assert_eq!(unusable_in("harbour-2", &own, &system), Some(("Harbour 2".to_string(), NOT_A_SHEET)));
        assert_eq!(unusable_in("taken-away", &own, &system), Some(("Taken away".to_string(), GONE)));
        assert_eq!(unusable_in("harbour", &own, &system), None);
        assert_eq!(unusable_in("", &own, &system), None);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_folder_with_more_themes_than_are_shown_says_so() {
        let dir = room("more-than-shown");
        let (own, system) = (dir.join("own"), dir.join("system"));
        for n in 0..MOST {
            std::fs::write(own.join(format!("theme-{n:03}.css")), HARBOUR).unwrap();
        }
        let all = listed_in(&system, &own);
        assert_eq!((all.themes.len(), all.said.as_slice()), (1 + MOST, &[][..]), "forty are all shown");
        std::fs::write(own.join("theme-999.css"), HARBOUR).unwrap();
        let all = listed_in(&system, &own);
        assert_eq!((all.themes.len(), all.said.as_slice()), (1 + MOST, &[NOT_ALL][..]));
        let shown = all.themes.iter().filter(|t| t.mine).count();
        let names = |theme_at: usize| (0..1000).map(move |n| if n == theme_at { "harbour.css".to_string() } else { format!("note-{n}.txt") });
        let looked = (0..1000).take_while(|at| ids_among(names(*at)).0 == ["harbour"]).count();
        assert_eq!((shown, looked), (40, 400));
        assert!(NOT_ALL.contains("first forty themes") && NOT_ALL.contains(&format!("first {looked} files")), "{NOT_ALL}");
        let _ = std::fs::remove_dir_all(dir);
        let dir = room("overfull");
        let (own, system) = (dir.join("own"), dir.join("system"));
        for n in 0..=LOOKED_AT {
            std::fs::write(own.join(format!("note-{n}.txt")), "").unwrap();
        }
        assert_eq!(listed_in(&system, &own).said, [NOT_ALL]);
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_sheet_over_the_size_is_refused_whole_and_its_tile_says_why() {
        let dir = room("big");
        let (own, system) = (dir.join("own"), dir.join("system"));
        let fill = |to: usize, end: &str| {
            let start = ":root { --trindle-theme-name: \"Harbour\"; --trindle-card: #223344; } /* ";
            format!("{start}{} */ {end}", "x".repeat(to - start.len() - end.len() - 4))
        };
        let end = "@media (prefers-color-scheme: light) { :root { --trindle-page: #f0f4f8; } }";
        let whole = fill(THEME_MOST as usize, end);
        assert_eq!(whole.len() as u64, THEME_MOST);
        std::fs::write(own.join("just-fits.css"), &whole).unwrap();
        assert_eq!(sheet::read_sheet(&own.join("just-fits.css"), THEME_MOST).as_deref(), Ok(whole.as_str()));
        assert_eq!(sheet::one_look(&whole), None, "it has both looks");
        let over = fill(THEME_MOST as usize + 40, end);
        assert_eq!(sheet::one_look(&over), None);
        assert_eq!(sheet::one_look(&over[..THEME_MOST as usize]), Some(false));
        std::fs::write(own.join("too-big.css"), &over).unwrap();
        std::fs::write(system.join("huge.css"), vec![b' '; 3 * THEME_MOST as usize]).unwrap();
        for file in [own.join("too-big.css"), system.join("huge.css")] {
            assert_eq!(sheet::read_sheet(&file, THEME_MOST), Err(sheet::Unread::TooBig), "{}", file.display());
        }
        let all = list(&system, &own);
        let seen: Vec<(&str, &str, Option<&str>, &str)> = all.iter().map(|t| (t.id.as_str(), t.name.as_str(), t.refused, t.css.as_str())).collect();
        assert_eq!(seen[0], ("", "Trindle", None, ""));
        assert_eq!(seen[1], ("huge", "Huge", Some(TOO_BIG), ""));
        assert_eq!((seen[2].0, seen[2].1, seen[2].2), ("just-fits", "Harbour", None));
        assert_eq!(seen[3], ("too-big", "Too big", Some(TOO_BIG), ""));
        assert!(all[3].mine && !all[1].mine);
        let no = usable_in("too-big", Some(&own.join("too-big.css"))).unwrap_err();
        assert_eq!(no.to_string(), TOO_BIG);
        assert!(usable_in("just-fits", Some(&own.join("just-fits.css"))).is_ok());
        assert!(usable_in("gone", None).is_err() && usable_in("gone", Some(&own.join("gone.css"))).is_err());
        assert!(usable_in("", None).is_ok(), "Trindle's own look needs no file");
        assert!(!TOO_BIG.contains('\u{2014}') && TOO_BIG.ends_with('.'));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_file_named_as_trindles_own_look_is_not_a_theme() {
        let dir = room("own-names");
        let (own, system) = (dir.join("own"), dir.join("system"));
        for name in sheet::OWN_NAMES {
            assert!(!sheet::theme_id(name), "{name}");
            assert_eq!(sheet::theme_file(name), None, "{name}");
            std::fs::write(own.join(format!("{name}.css")), HARBOUR).unwrap();
            std::fs::write(system.join(format!("{name}.css")), HARBOUR).unwrap();
        }
        std::fs::write(own.join("default-2.css"), HARBOUR).unwrap();
        let all = list(&system, &own);
        assert_eq!(all.iter().map(|t| t.id.as_str()).collect::<Vec<_>>(), ["", "default-2"]);
        let now = 1_800_000_000;
        assert_eq!(sheet::trial_of(&trial_line("", now, None), now), Some((String::new(), now + TRIAL_FOR)));
        assert_eq!(sheet::trial_of("trindle@1800000030", now), None);
        assert_eq!(sheet::kept_theme("default", NAMING), "");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn a_stuffed_folder_is_left_after_so_many_names() {
        let taken = std::cell::Cell::new(0usize);
        let names = (0..1_000_000).map(|i| {
            taken.set(taken.get() + 1);
            if i % 2 == 0 { format!("t{i:07}.css") } else { format!("notes{i}.txt") }
        });
        let (ids, more) = ids_among(names);
        assert!(more, "and the list says it is not the whole folder");
        assert_eq!(taken.get(), LOOKED_AT);
        assert_eq!(ids.len(), MOST);
        assert_eq!(ids[0], "t0000000");
        assert_eq!(ids_among(["b.css", "a.css", "c.txt", "Bad Name.css", "default.css"].into_iter().map(String::from)), (vec!["a".to_string(), "b".to_string()], false));
    }

    #[test]
    fn a_theme_makes_every_shape_square_or_round_from_its_own_named_values() {
        use crate::toolkit::theme::{sample, Part, FULL_ROUND};
        for p in Part::ALL {
            assert!(sheet::CSS.contains(&format!("--{}:", p.name())), "{p:?} is named in Trindle's sheet");
        }
        for (own, want) in [("0", 0.0), ("6", 6.0), ("999", FULL_ROUND)] {
            let css = format!(":root {{ --trindle-card-corner: {own}; --trindle-box-corner: {own}; }}");
            let s = sample(&css, false, "#2196f3");
            assert_eq!((s.card_corner, s.box_corner), (want, want), "corner {own}");
        }
    }
}
