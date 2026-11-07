//! Markdown.

use std::collections::HashMap;
use blocks::{fence, reference};

mod blocks;
pub(super) mod finished;
pub(super) mod html;
mod marks;
pub(super) mod layout;
pub(super) mod paint;

#[derive(Clone, Debug, PartialEq)]
pub enum Span {
    Text(String),
    Bold(Vec<Span>),
    Italic(Vec<Span>),
    Strike(Vec<Span>),
    Code(String),
    Link(Vec<Span>, String),
    Image(String, String),
    Break,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Align {
    Start,
    Left,
    Centre,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub task: Option<bool>,
    pub blocks: Vec<Block>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Block {
    Heading(u8, Vec<Span>),
    Para(Vec<Span>),
    Quote(Vec<Block>),
    List(Option<u64>, Vec<Item>),
    Code(String, String),
    Rule,
    Table(Vec<Align>, Vec<Vec<Span>>, Vec<Vec<Vec<Span>>>),
}

type Refs = HashMap<String, String>;

pub fn parse(text: &str) -> Vec<Block> {
    let text = text.replace('\t', "    ");
    let mut refs = Refs::new();
    let mut lines = Vec::new();
    let mut fenced: Option<(char, usize)> = None;
    for line in text.lines() {
        if let Some((c, n, info)) = fence(line) {
            match fenced {
                None => fenced = Some((c, n)),
                Some((fc, fnum)) if fc == c && n >= fnum && info.is_empty() => fenced = None,
                _ => {}
            }
        } else if fenced.is_none() {
            if let Some((name, url)) = reference(line) {
                refs.insert(name, url);
                continue;
            }
        }
        lines.push(line);
    }
    blocks::blocks(&lines, &refs)
}

pub fn plain(spans: &[Span]) -> String {
    let mut s = String::new();
    for span in spans {
        match span {
            Span::Text(t) | Span::Code(t) | Span::Image(t, _) => s.push_str(t),
            Span::Bold(x) | Span::Italic(x) | Span::Strike(x) | Span::Link(x, _) => s.push_str(&plain(x)),
            Span::Break => s.push(' '),
        }
    }
    s
}

pub fn title(blocks: &[Block]) -> Option<String> {
    blocks.iter().find_map(|b| match b {
        Block::Heading(_, s) => Some(plain(s)).filter(|t| !t.trim().is_empty()),
        _ => None,
    })
}

#[cfg(test)]
mod tests;
