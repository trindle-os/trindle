//! A tray icon's menu as a card: the entries the program lists, their lines and the Close that ends the program.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use zbus::zvariant::Value;

use crate::ring::chosen;
use crate::ring::ctx::{Msg, msg};
use crate::toolkit::t;
use crate::toolkit::widget::{self, Length, Node};

use super::{Entry, MENU, Target, WAIT};
use super::look::GLYPH;

const CARD_PADDING: f32 = 8.0;

const CARD_WIDTH: f32 = 224.0;

const LINE_GAP: f32 = 8.0;

const SUB_INDENT: f32 = 16.0;

const GROUP_GAP: f32 = 8.0;

pub(super) fn kids(node: &Value) -> Vec<Entry> {
    let Some(Value::Array(children)) = part(node, 2) else { return Vec::new() };
    children.inner().iter().filter_map(|c| {
        let child = unbox(c);
        let id = part(child, 0).and_then(number).unwrap_or(0) as i32;
        let Some(Value::Dict(d)) = part(child, 1) else { return None };
        let get = |k: &str| d.iter().find(|(n, _)| text(n).as_deref() == Some(k)).map(|(_, v)| unbox(v).clone());
        let words = |k: &str| get(k).and_then(|v| text(&v)).unwrap_or_default();

        let label = words("label").replace("__", "\u{0}").replace('_', "").replace('\u{0}', "_");
        let on = get("toggle-state").and_then(|v| number(&v)).unwrap_or(0) == 1;
        let submenu = words("children-display") == "submenu";
        Some(Entry {
            id,
            label,
            separator: words("type") == "separator",
            visible: get("visible").and_then(|v| truth(&v)) != Some(false),
            enabled: get("enabled").and_then(|v| truth(&v)).unwrap_or(true),
            ticked: !words("toggle-type").is_empty() && on,
            submenu,
            kids: if submenu { kids(child) } else { Vec::new() },
        })
    }).collect()
}

pub(super) fn menu_card(entries: &[Entry], open: &Rc<RefCell<HashSet<i32>>>, to: &Rc<Target>, id: &str) -> Node<Msg> {
    let mut lines = entry_lines(entries, open, to, 0.0);
    if !lines.is_empty() {
        lines.push(Part::Gap);
    }

    let room = !entries.is_empty();
    lines.push(chosen::menu_part(id, room));
    let pid = to.pid.clone();
    lines.push(menu_line_in(t!("ring-close").into(), room, msg(move |f| {
        close_process(pid.get());
        f.close();
    })));
    menu_frame(lines)
}

pub(super) fn may_close(pid: u32) -> bool {
    pid > 1 && pid != std::process::id()
}

fn close_process(pid: u32) {
    if may_close(pid) {
        // SAFETY: kill only sends a signal.
        unsafe { libc::kill(pid as libc::pid_t, libc::SIGTERM) };
    }
}

pub(in crate::ring) enum Part {

    Line(Node<Msg>),

    Words(Node<Msg>),

    Gap,
}

impl Part {

    pub(in crate::ring) fn grab(self, turn: u32) -> Part {
        match self {
            Part::Line(n) => Part::Line(n.grab(turn)),
            other => other,
        }
    }
}

pub(in crate::ring) fn menu_frame(parts: Vec<Part>) -> Node<Msg> {
    let mut lines: Vec<Node<Msg>> = Vec::new();
    let mut run = 0;
    for part in parts {
        match part {
            Part::Gap => {
                run = 0;
                lines.push(widget::space().width(Length::Fixed(0.0)).height(Length::Fixed(GROUP_GAP)));
            }
            Part::Words(n) => {
                run = 0;
                lines.push(widget::card(n).snug().flat());
            }
            Part::Line(n) => {
                lines.push(if run % 2 == 1 { n.banded() } else { widget::card(n).snug().flat() });
                run += 1;
            }
        }
    }
    lines.push(widget::space().width(Length::Fixed(CARD_WIDTH)).height(Length::Fixed(0.0)));
    widget::column(lines).padding(CARD_PADDING).gap(0.0).width(Length::Shrink)
}

pub(in crate::ring) fn menu_line(words: String, press: Msg) -> Part {
    menu_line_in(words, false, press)
}

pub(in crate::ring) fn menu_line_in(words: String, room: bool, press: Msg) -> Part {
    let label = widget::label(words).width(Length::Fill);
    if !room {
        return Part::Line(widget::card(label).flat().on_press(press));
    }
    let line = widget::row(vec![glyph(crate::common::no_tick::MARK_ICON, false), label]).gap(LINE_GAP).width(Length::Fill);
    Part::Line(widget::card(line).flat().on_press(press))
}

pub(in crate::ring) fn menu_words(words: String) -> Part {
    Part::Words(widget::card(widget::note(words).width(Length::Fill)).flat())
}

pub(in crate::ring) fn menu_gap() -> Part {
    Part::Gap
}

fn entry_lines(entries: &[Entry], open: &Rc<RefCell<HashSet<i32>>>, to: &Rc<Target>, indent: f32) -> Vec<Part> {
    let mut lines = Vec::new();
    for e in entries.iter().filter(|e| e.visible) {
        if e.separator {
            lines.push(Part::Gap);
            continue;
        }
        let id = e.id;
        let press = if e.submenu {

            let open = open.clone();
            msg(move |_| {
                let mut o = open.borrow_mut();
                if !o.remove(&id) {
                    o.insert(id);
                }
            })
        } else {
            let to = to.clone();
            msg(move |f| {
                let body = (id, "clicked", Value::from(0i32), 0u32);
                to.bus.call(&to.service, &to.path, MENU).method("Event", &body, false, Some(WAIT), &to.ctx.lp, |_| {});
                f.close();
            })
        };

        let mut parts = vec![glyph(crate::common::no_tick::MARK_ICON, e.ticked), widget::label(e.label.clone()).width(Length::Fill)];
        if e.submenu {
            parts.push(glyph("pan-down-symbolic", true));
        }
        let line = widget::row(parts).gap(LINE_GAP).width(Length::Fill);
        let line = widget::card(line).flat().on_press(press).off(!e.enabled);

        lines.push(Part::Line(if indent > 0.0 {
            widget::row(vec![widget::space().width(Length::Fixed(indent)), line]).width(Length::Fill)
        } else {
            line
        }));
        if e.submenu && open.borrow().contains(&id) {
            lines.extend(entry_lines(&e.kids, open, to, indent + SUB_INDENT));
        }
    }
    lines
}

fn glyph(name: &'static str, shown: bool) -> Node<Msg> {
    widget::drawing((GLYPH, GLYPH), "", move |canvas, pen| {
        if shown {
            let size = (GLYPH * pen.scale).round() as u32;
            pen.icons.draw(canvas, name, 0, 0, size, 1, pen.theme.fg);
        }
    })
}

pub(super) fn part<'a>(v: &'a Value<'a>, i: usize) -> Option<&'a Value<'a>> {
    match unbox(v) {
        Value::Structure(s) => s.fields().get(i).map(unbox),
        _ => None,
    }
}

pub(super) fn unbox<'a>(v: &'a Value<'a>) -> &'a Value<'a> {
    match v {
        Value::Value(inner) => unbox(inner),
        _ => v,
    }
}

pub(super) fn text(v: &Value) -> Option<String> {
    match unbox(v) {
        Value::Str(s) => Some(s.to_string()),
        Value::ObjectPath(p) => Some(p.to_string()),
        Value::Signature(s) => Some(s.to_string()),
        _ => None,
    }
}

pub(super) fn truth(v: &Value) -> Option<bool> {
    match unbox(v) {
        Value::Bool(b) => Some(*b),
        _ => None,
    }
}

pub(super) fn number(v: &Value) -> Option<i64> {
    match unbox(v) {
        Value::I32(n) => Some(*n as i64),
        Value::U32(n) => Some(*n as i64),
        Value::I64(n) => Some(*n),
        Value::U8(n) => Some(*n as i64),
        Value::I16(n) => Some(*n as i64),
        Value::U16(n) => Some(*n as i64),
        _ => None,
    }
}
