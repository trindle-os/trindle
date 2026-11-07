//! Why a backup was refused, in plain words, and what lies behind the words.

use crate::toolkit::t;

pub fn in_the_way_words() -> String {
    nothing(in_the_way_because())
}

pub fn in_the_way_because() -> &'static str {
    t!("store-because-in-the-way")
}

pub fn nothing(why: &str) -> String {
    t!("store-refusal-nothing", why = why)
}

pub(super) fn lead_of(message: String) -> Option<String> {
    let lead = message.split('\u{0}').next().unwrap_or("").to_string();
    (lead.chars().count() >= 8).then_some(lead)
}

fn said_already(words: &str) -> bool {
    let nul = "\u{0}";
    [
        lead_of(t!("store-refusal-nothing", why = nul)),
        lead_of(t!("store-refusal-part", why = nul)),
        lead_of(t!("store-not-installed-failed", failed = nul)),
        lead_of(t!("store-not-installed-refused", refused = nul)),
    ].into_iter().flatten().any(|lead| words.starts_with(&lead))
}

const BEHIND: char = '\u{1f}';
const WORDS: char = '\u{1e}';

pub fn with_behind(words: &str, behind: &[String]) -> String {
    let mut out = String::new();
    for b in behind.iter().filter(|b| !b.is_empty()) {
        out.push(BEHIND);
        out.push_str(&b.replace([BEHIND, WORDS], " "));
    }
    if !out.is_empty() {
        out.push(WORDS);
    }
    out + words
}

pub fn in_the_way(paths: &[String]) -> String {
    with_behind(&in_the_way_words(), paths)
}

pub fn apart(text: &str) -> (String, Vec<String>) {
    match text.strip_prefix(BEHIND).and_then(|rest| rest.split_once(WORDS)) {
        Some((behind, words)) => (words.to_string(), behind.split(BEHIND).map(String::from).collect()),
        None => (text.to_string(), Vec::new()),
    }
}

pub fn after(before: &str, text: &str) -> String {
    let (first, mut behind) = apart(before);
    let (words, more) = apart(text);
    behind.extend(more);
    with_behind(&format!("{first} {words}"), &behind)
}

pub fn last_said(stderr: &str) -> String {
    stderr.trim().lines().map(str::trim).rfind(|l| !l.is_empty() && !l.contains("Exiting with failure status"))
        .unwrap_or(t!("store-because-failed")).to_string()
}

pub fn explained(e: String) -> String {
    let (words, _) = apart(&e);
    if said_already(&words) {
        return e;
    }
    with_behind(&t!("store-refusal-part", why = because(&words)), &[words])
}

fn known(reason: &str) -> bool {
    ["Permission denied", "No space left on device", "Read-only file system", "Disk quota exceeded",
     "No such file or directory"].contains(&reason)
}

pub fn because(reason: &str) -> String {
    let (reason, _) = apart(reason);
    let whole = reason.trim().trim_end_matches('.').split(" (os error").next().unwrap_or("").trim();
    let r = whole.rsplit_once(": ").map(|(_, end)| end).filter(|end| known(end)).unwrap_or(whole);
    match r {
        "" => t!("store-because-none").into(),
        "Permission denied" => t!("store-because-permission").into(),
        "No space left on device" => t!("store-because-space").into(),
        "Read-only file system" => t!("store-because-read-only").into(),
        "Disk quota exceeded" => t!("store-because-quota").into(),
        "No such file or directory" => t!("store-because-missing").into(),
        _ if format!("{r}.") == in_the_way_words() => in_the_way_because().into(),
        _ => {
            let mut c = r.chars();
            c.next().map(|f| f.to_lowercase().collect::<String>() + c.as_str()).unwrap_or_default()
        }
    }
}

#[cfg(test)]
pub fn words(text: &str) -> String {
    apart(text).0
}

pub fn plain(text: &str) -> String {
    let (words, behind) = apart(text);
    behind.iter().fold(words, |all, b| format!("{all}\n  {b}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_exact_words_are_kept() {
        assert_eq!(in_the_way_words(), "Nothing was put back, because something on this computer is already where part of the backup needs to go.");
        assert_eq!(in_the_way_words(), format!("Nothing was put back, because {}.", in_the_way_because()));
    }

    #[test]
    fn words_and_what_is_behind_them_come_apart_again() {
        let paths = vec!["/home/a/.config/trindle".to_string(), "/home/a/.appimage/x y".to_string()];
        let text = in_the_way(&paths);
        let in_way = in_the_way_words();
        assert_eq!(apart(&text), (in_way.clone(), paths));
        let longer = format!("{text} Your account picture was not put back either.");
        let (w, b) = apart(&longer);
        assert_eq!(w, format!("{in_way} Your account picture was not put back either."));
        assert_eq!(b.len(), 2);
        assert_eq!(apart("plain words"), ("plain words".to_string(), Vec::<String>::new()));
        assert_eq!(with_behind("plain", &[]), "plain");
        assert_eq!(words(&text), in_way);
        assert_eq!(apart(&after("Installed 2.", &text)), (format!("Installed 2. {in_way}"), vec!["/home/a/.config/trindle".to_string(), "/home/a/.appimage/x y".to_string()]));
    }

    #[test]
    fn a_bare_reason_is_made_into_words_that_say_why() {
        let e = explained("tar: a/b: Cannot open: Permission denied".into());
        let (w, b) = apart(&e);
        assert_eq!(w, "Part of the backup was not put back, because this computer would not let Trindle write there.");
        assert_eq!(b, vec!["tar: a/b: Cannot open: Permission denied".to_string()]);
        let (w, _) = apart(&explained("No space left on device".into()));
        assert_eq!(w, "Part of the backup was not put back, because there is no room left on this computer.");
        let kept = in_the_way(&["/x".into()]);
        assert_eq!(explained(kept.clone()), kept);
        assert_eq!(last_said("tar: x: Cannot open: File exists\ntar: Exiting with failure status due to previous errors"),
                   "tar: x: Cannot open: File exists");
    }

    #[test]
    fn two_refusals_joined_keep_every_path_and_the_command_line_prints_them() {
        let a = in_the_way(&["/a".into()]);
        let b = in_the_way(&["/b".into()]);
        let (w, behind) = apart(&after(&a, &b));
        assert!(w.starts_with("Nothing was put back") && !w.contains('\u{1f}') && !w.contains('\u{1e}'));
        assert_eq!(behind, vec!["/a".to_string(), "/b".to_string()]);
        assert_eq!(plain(&a), format!("{}\n  /a", in_the_way_words()));
    }

    #[test]
    fn a_reason_follows_because_in_plain_words() {
        assert_eq!(because("Permission denied"), "this computer would not let Trindle write there");
        assert_eq!(because("No space left on device (os error 28)"), "there is no room left on this computer");
        assert_eq!(because("Unexpected EOF in archive."), "unexpected EOF in archive");
        assert_eq!(because(""), "it gave no reason");
        assert_eq!(because(&in_the_way(&["/x".into()])), in_the_way_because());
    }
}
