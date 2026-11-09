//! The page that lists the keyboard shortcuts.

use crate::ring::ctx::Msg;
use crate::toolkit::t;
use crate::toolkit::widget::{column, Node};

use super::{BOLD, line_of};

pub(super) fn settings_page(panel: &'static str) -> &'static str {
    match panel {
        "clipboard" | "notes" => "ring",
        "shortcuts" => "keyboard",
        "brightness" => "display",
        other => other,
    }
}

pub(super) fn shortcut_lines(rows: &[crate::common::key_list::Row]) -> Vec<Vec<String>> {
    use crate::common::key_list::Group;
    let mut out = Vec::new();
    let mut group = Group::Trindle;
    for r in rows {
        if r.group != group {
            group = r.group;
            out.push(vec![(if group == Group::Plugin { t!("ring-shortcuts-heading-plugins") } else { t!("ring-shortcuts-heading-yours") }).to_string()]);
        }
        out.push(vec![r.words.clone(), r.keys.clone()]);
    }
    out
}

pub(super) fn shortcut_rows(rows: &[crate::common::key_list::Row]) -> Vec<Node<Msg>> {
    shortcut_lines(rows).into_iter().map(|mut lines| match lines.len() {
        1 => line_of(lines.remove(0), true, BOLD).named(true),
        _ => column(vec![line_of(lines.remove(0), false, BOLD).named(true), line_of(lines.remove(0), true, 400).named(true)]).gap(0.0),
    }).collect()
}
