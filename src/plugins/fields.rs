//! A plugin's settings fields.

use std::collections::BTreeMap;
use std::path::Path;

use crate::toolkit::t;

use super::manifest::{good_name, home_of, Plugin};
use super::notes::{plain_words, read_plain};

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    Switch,
    Text,
    Choice(Vec<String>),
    Note(String),
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub label: String,
    pub kind: Kind,
    pub default: String,
    pub choices_from: Option<String>,
    pub show_when: Option<(String, String)>,
}

pub const SHOW_WHEN_MAX: usize = 128;
pub const CHOICES_LEAST: usize = 2;

pub(super) fn show_when(group: &str, name: &str, at: usize, names: &[String], earlier: &[Field], said: &str,
             problems: &mut Vec<String>) -> Option<(String, String)> {
    if said.is_empty() {
        return None;
    }
    let mut refuse = |words: String| {
        problems.push(words);
        None
    };
    if said.chars().count() > SHOW_WHEN_MAX {
        return refuse(t!("plugins-showwhen-long", group = group, max = SHOW_WHEN_MAX));
    }
    let Some((other, want)) = said.split_once('=') else {
        return refuse(t!("plugins-showwhen-form", group = group));
    };
    let (other, want) = (other.trim(), want.trim());
    if !good_name(other) {
        return refuse(t!("plugins-showwhen-field-name", group = group));
    }
    if want.is_empty() || !is_plain(want, CHOICE_TEXT_MAX) {
        return refuse(t!("plugins-showwhen-value", group = group, max = CHOICE_TEXT_MAX));
    }
    if other == name {
        return refuse(t!("plugins-showwhen-own", group = group));
    }
    match names.iter().position(|n| n == other) {
        None => return refuse(t!("plugins-showwhen-unknown", group = group, other = other)),
        Some(i) if i > at => {
            return refuse(t!("plugins-showwhen-later", group = group, other = other));
        }
        Some(_) => {}
    }
    let o = earlier.iter().find(|f| f.name == other)?;
    if o.show_when.is_some() {
        return refuse(t!("plugins-showwhen-chained", group = group, other = other));
    }
    match &o.kind {
        Kind::Switch if !matches!(want, "true" | "false") => {
            return refuse(t!("plugins-showwhen-switch", group = group, other = other));
        }
        Kind::Choice(c) if o.choices_from.is_none() && !c.iter().any(|c| c == want) => {
            return refuse(t!("plugins-showwhen-not-choice", group = group, value = format!("{want:?}"), other = other));
        }
        Kind::Note(_) => {
            return refuse(t!("plugins-showwhen-note", group = group, other = other));
        }
        _ => {}
    }
    Some((other.to_string(), want.to_string()))
}

pub fn field_shown(p: &Plugin, f: &Field, values: &BTreeMap<String, String>, offered: &[(String, String)]) -> bool {
    if let Some((other, want)) = &f.show_when {
        let Some(o) = p.fields.iter().find(|o| o.name == *other) else { return false };
        let now = values.get(other).map(String::as_str).unwrap_or(&o.default);
        let met = match o.kind {
            Kind::Switch => (now == "true") == (want == "true"),
            _ => now == want,
        };
        if !met {
            return false;
        }
    }
    !(matches!(f.kind, Kind::Choice(_)) && f.choices_from.is_some() && offered.len() < CHOICES_LEAST)
}

pub const CHOICES_MAX: usize = 32;
pub const CHOICE_TEXT_MAX: usize = 48;
const CHOICES_FILE_MAX: u64 = 8192;

pub fn is_plain(text: &str, max: usize) -> bool {
    text.chars().count() <= max && plain_words(text, usize::MAX) == text
}

pub fn good_choices_file(name: &str) -> bool {
    !name.is_empty() && name.len() <= 64 && !name.starts_with('.')
        && name.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'))
}

fn plain_value(text: &str, max: usize) -> String {
    let whole = plain_words(text, usize::MAX);
    whole.chars().take(max).collect::<String>().trim_end().to_string()
}

pub fn choices_in(home: &Path, file: &str) -> Vec<(String, String)> {
    if !good_choices_file(file) {
        return Vec::new();
    }
    let Some(bytes) = read_plain(&home.join(file), CHOICES_FILE_MAX) else { return Vec::new() };
    let mut out: Vec<(String, String)> = Vec::new();
    for line in String::from_utf8_lossy(&bytes).lines() {
        let (value, label) = line.split_once('|').unwrap_or((line, line));
        let value = plain_value(value, CHOICE_TEXT_MAX);
        if value.is_empty() || out.iter().any(|(v, _)| *v == value) {
            continue;
        }
        let label = plain_words(label, CHOICE_TEXT_MAX);
        let label = if label.is_empty() { value.clone() } else { label };
        out.push((value, label));
        if out.len() == CHOICES_MAX {
            break;
        }
    }
    out
}

pub const NOTE_MAX: usize = 120;

pub fn note_of(id: &str, f: &Field) -> String {
    note_in(&home_of(id), f)
}

pub fn note_in(home: &Path, f: &Field) -> String {
    let Kind::Note(file) = &f.kind else { return String::new() };
    if !good_choices_file(file) {
        return String::new();
    }
    let Some(bytes) = read_plain(&home.join(file), CHOICES_FILE_MAX) else { return String::new() };
    String::from_utf8_lossy(&bytes).lines().map(|l| plain_words(l, NOTE_MAX)).find(|l| !l.is_empty()).unwrap_or_default()
}

pub fn choices_of(id: &str, f: &Field) -> Vec<(String, String)> {
    choices_of_in(&home_of(id), f)
}

pub fn choices_of_in(home: &Path, f: &Field) -> Vec<(String, String)> {
    let own = || match &f.kind {
        Kind::Choice(c) => c.iter().map(|c| (c.clone(), c.clone())).collect(),
        _ => Vec::new(),
    };
    match &f.choices_from {
        Some(file) => {
            let got = choices_in(home, file);
            if got.is_empty() { own() } else { got }
        }
        None => own(),
    }
}

#[cfg(test)]
mod tests;
