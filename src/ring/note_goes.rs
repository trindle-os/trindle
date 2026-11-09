//! Where a click on a notification circle takes the person: to the program
//! that sent it. The sender is untrusted, so this only ever looks at the
//! launcher it can prove is the sender's, never at the notification's words.

use crate::common::apps::{self, Entry};
use crate::toolkit::t;

pub struct Sender<'a> {

    pub app: &'a str,

    pub desktop: &'a str,

    pub has_default: bool,

    pub process: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub enum Goes {

    Default,

    Raise(String),

    Start(String),

    Why(&'static str),
}

pub fn no_program() -> &'static str {
    t!("ring-goes-no-program")
}
pub fn other_program() -> &'static str {
    t!("ring-goes-other-program")
}
pub fn not_started() -> &'static str {
    t!("ring-goes-not-started")
}

const GO_BETWEENS: [&str; 4] = ["xdg-dbus-proxy", "bwrap", "flatpak-portal", "snap-confine"];

fn find<'a>(all: &'a [Entry], desktop: &str, app: &str) -> Option<&'a Entry> {
    if !desktop.is_empty() {
        if let Some(e) = all.iter().find(|e| apps::same_name(e.key(), desktop)).or_else(|| apps::find_for_window(all, desktop)) {
            return Some(e);
        }
    }
    if app.is_empty() {
        return None;
    }
    all.iter().find(|e| apps::same_name(e.key(), app) || e.name.eq_ignore_ascii_case(app))
}

fn is_process_of(process: &str, e: &Entry) -> bool {
    let p = process.to_lowercase();
    let base = |s: &str| s.rsplit('/').next().unwrap_or(s).to_lowercase();
    [base(&e.program), e.key().to_lowercase(), e.wm_class.to_lowercase(), e.name.to_lowercase(), base(e.exec.split_whitespace().next().unwrap_or(""))]
        .iter()
        .filter(|c| !c.is_empty())
        .any(|c| *c == p || (p.len() >= 15 && c.starts_with(&p)))
}

pub fn launcher<'a>(all: &'a [Entry], s: &Sender) -> Result<&'a Entry, &'static str> {
    let process = s.process.filter(|p| !p.is_empty() && !GO_BETWEENS.contains(p));
    let named = find(all, s.desktop, s.app);
    match (named, process) {
        (Some(e), Some(p)) if !is_process_of(p, e) => {

            all.iter().find(|e| is_process_of(p, e)).ok_or_else(other_program)
        }
        (Some(e), _) => Ok(e),
        (None, Some(p)) => all.iter().find(|e| is_process_of(p, e)).ok_or_else(no_program),
        (None, None) => Err(no_program()),
    }
}

pub fn plan(all: &[Entry], s: &Sender, open: &[String]) -> Goes {
    if s.has_default {
        return Goes::Default;
    }
    match launcher(all, s) {
        Err(why) => Goes::Why(why),
        Ok(e) if open.iter().any(|w| apps::owns(all, e, w)) => Goes::Raise(e.id.clone()),
        Ok(e) => Goes::Start(e.id.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, name: &str, program: &str) -> Entry {
        Entry {
            id: id.into(), name: name.into(), icon: None, exec: program.into(), try_exec: None, wm_class: String::new(),
            program: program.into(), kinds: vec![], path: Default::default(), category: "Other", keywords: vec![],
            flatpak: None, appimage: None, terminal: false,
        }
    }

    fn all() -> Vec<Entry> {
        vec![entry("claude-desktop.desktop", "Claude", "claude"), entry("firefox.desktop", "Firefox", "firefox")]
    }

    fn sender<'a>(app: &'a str, desktop: &'a str, has_default: bool, process: Option<&'a str>) -> Sender<'a> {
        Sender { app, desktop, has_default, process }
    }

    fn open(ids: &[&str]) -> Vec<String> {
        ids.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn a_default_action_is_left_to_the_sender() {
        assert_eq!(plan(&all(), &sender("Claude", "claude-desktop", true, None), &open(&[])), Goes::Default);
    }

    #[test]
    fn an_entry_alone_finds_the_launcher_and_raises_or_starts() {
        let a = all();
        let s = sender("", "claude-desktop", false, None);
        assert_eq!(plan(&a, &s, &open(&["claude"])), Goes::Raise("claude-desktop.desktop".into()));
        assert_eq!(plan(&a, &s, &open(&["firefox"])), Goes::Start("claude-desktop.desktop".into()));
        assert_eq!(plan(&a, &s, &open(&[])), Goes::Start("claude-desktop.desktop".into()));
    }

    #[test]
    fn a_name_alone_finds_the_launcher() {
        let a = all();
        let s = sender("Firefox", "", false, None);
        assert_eq!(plan(&a, &s, &open(&["firefox"])), Goes::Raise("firefox.desktop".into()));
        assert_eq!(plan(&a, &s, &open(&[])), Goes::Start("firefox.desktop".into()));
    }

    #[test]
    fn an_unknown_sender_is_told_why_in_few_words() {
        let a = all();
        assert_eq!(plan(&a, &sender("Mystery", "", false, None), &open(&["claude"])), Goes::Why(no_program()));
        assert_eq!(plan(&a, &sender("", "nothing-like-it", false, None), &open(&[])), Goes::Why(no_program()));
        assert_eq!(plan(&a, &sender("", "", false, None), &open(&[])), Goes::Why(no_program()));
    }

    #[test]
    fn a_sender_naming_another_programs_entry_never_brings_that_program_forward() {
        let a = all();

        let s = sender("Firefox", "firefox", false, Some("evil"));
        assert_eq!(plan(&a, &s, &open(&["firefox"])), Goes::Why(other_program()));

        let s = sender("Firefox", "firefox", false, Some("claude"));
        assert_eq!(plan(&a, &s, &open(&["firefox"])), Goes::Start("claude-desktop.desktop".into()));
        assert_eq!(plan(&a, &s, &open(&["firefox", "claude"])), Goes::Raise("claude-desktop.desktop".into()));
    }

    #[test]
    fn the_process_alone_can_find_the_launcher_and_a_sandbox_proxy_says_nothing() {
        let a = all();
        assert_eq!(plan(&a, &sender("", "", false, Some("firefox")), &open(&["firefox"])), Goes::Raise("firefox.desktop".into()));
        assert_eq!(plan(&a, &sender("Firefox", "", false, Some("xdg-dbus-proxy")), &open(&["firefox"])), Goes::Raise("firefox.desktop".into()));
        assert_eq!(plan(&a, &sender("", "", false, Some("xdg-dbus-proxy")), &open(&[])), Goes::Why(no_program()));
    }
}
