//! One line of a card: its words on the left, its controls on the right, and what came of the last change.

use super::{Kit, Msg, icon};
use crate::toolkit::widget::{column, form_label, note,
                             row, space, Length, Node, Tone};

pub const CONTROL_GAP: f32 = 12.0;

pub struct Line {
    title: String,
    sub: Option<String>,
    icon: Option<String>,
    controls: Vec<Node<Msg>>,
    below: Vec<Node<Msg>>,
    key: Option<String>,

    room: u8,
}

pub fn sub_key(line: &str) -> String {
    format!("{line}:sub")
}

pub fn line(heading: impl Into<String>) -> Line {
    Line { title: heading.into(), sub: None, icon: None, controls: Vec::new(), below: Vec::new(), key: None, room: 0 }
}

impl Line {

    pub fn keep_sub(mut self, words: impl Into<String>, lines: u8) -> Self {
        let words = words.into();
        self.sub = (!words.is_empty()).then_some(words);
        self.room = lines.max(1);
        self
    }

    pub fn sub(mut self, words: impl Into<String>) -> Self {
        let words = words.into();
        self.sub = (!words.is_empty()).then_some(words);
        self
    }

    pub fn icon(mut self, name: impl Into<String>) -> Self {
        self.icon = Some(name.into());
        self
    }

    pub fn control(mut self, c: Node<Msg>) -> Self {
        self.controls.push(c);
        self
    }

    pub fn controls(mut self, cs: Vec<Node<Msg>>) -> Self {
        self.controls.extend(cs);
        self
    }

    pub fn below(mut self, n: Node<Msg>) -> Self {
        self.below.push(n);
        self
    }

    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }

    pub fn node(mut self, kit: &Kit) -> Node<Msg> {

        if self.controls.len() == 1 {
            self.controls = self.controls.into_iter().map(Node::choosers_hushed).collect();
        }
        for c in &mut self.controls {
            if c.tip() == Some(self.title.as_str()) {
                *c = std::mem::replace(c, space()).hushed();
            }
        }
        let mut words = vec![form_label(self.title)];
        if self.room > 0 {
            let sub = note(self.sub.unwrap_or_default()).room_for(self.room);
            words.push(match &self.key {
                Some(k) => sub.key(sub_key(k)),
                None => sub,
            });
        } else if let Some(s) = self.sub {
            words.push(note(s));
        }
        if let Some(k) = &self.key {
            if let Some((said, problem)) = kit.said(k) {
                words.push(note(said).tone(if problem { Tone::Problem } else { Tone::Plain }));
            }

            let turn = kit.glow(k);
            if turn > 0 {
                let boxed = self.controls.iter().chain(&self.below).any(Node::holds_box);
                let buttons = !boxed && self.controls.iter().chain(&self.below).all(Node::only_buttons);
                self.controls = self.controls.into_iter().map(|c| c.glow_kept(turn, buttons)).collect();
                self.below = self.below.into_iter().map(|c| c.glow_kept(turn, buttons)).collect();
            }
        }
        let mut items = Vec::new();
        if let Some(i) = self.icon {
            items.push(icon(i, 24.0));
        }
        items.push(column(words).gap(4.0).width(Length::Fill));

        if self.controls.len() > 1 {
            items.push(row(self.controls).gap(CONTROL_GAP).width(Length::Shrink));
        } else {
            items.extend(self.controls);
        }
        let top = row(items).gap(CONTROL_GAP).stacks();
        if self.below.is_empty() {
            return top;
        }
        let mut all = vec![top];
        all.extend(self.below);
        column(all).gap(8.0)
    }
}
