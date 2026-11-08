//! A press on the empty part of any round cover in the ring reaches nothing under it.

use std::rc::Rc;
use std::time::Instant;

use crate::ring::ctx::Msg;
use crate::ring::layout::{button_size, layout, CENTRE};
use crate::toolkit::input::{Button, Input, Keysym, Mods};
use crate::toolkit::ui::Ui;
use crate::toolkit::widget::{button, drawing, layers, placed, row, Length, Node};

use super::app::{look, uncovered, MIDDLE_COVER};
use super::arrange::quick_side;
use super::middle::tray_cover;
use super::paint::disc;

const SIZE: (f32, f32) = (1280.0, 800.0);

fn live(size: (f32, f32), tip: &'static str, m: &Msg) -> Node<Msg> {
    let (a, b, c, d) = (m.clone(), m.clone(), m.clone(), m.clone());
    drawing(size, tip, |_, _| {})
        .pressable(a)
        .on_hover()
        .on_keys(move |_| Some(b.clone()))
        .on_pointer(move |_| c.clone())
        .on_scroll(move |_| d.clone())
}

fn middle(n: Node<Msg>) -> Node<Msg> {
    placed(n, |(w, h), (aw, ah)| ((aw - w) / 2.0, (ah - h) / 2.0))
}

fn ring_layout() -> crate::ring::layout::Layout<usize> {
    layout::<usize>(&(0..8).collect::<Vec<_>>(), &[], SIZE.0 as f64, SIZE.1 as f64)
}

fn under(hidden: &Msg, seen: &Msg, reach: f32) -> Vec<((f32, f32), Node<Msg>)> {
    let mid = (SIZE.0 / 2.0, SIZE.1 / 2.0);
    let lay = ring_layout();
    let tools: Vec<Node<Msg>> = (0..4).map(|_| button("drive-harddisk-symbolic", "Disks").on_press(hidden.clone())).collect();
    let centre = layers(vec![live((CENTRE as f32, CENTRE as f32), "Clock", hidden), middle(row(tools))])
        .width(Length::Fixed(CENTRE as f32)).height(Length::Fixed(CENTRE as f32));
    let mut all = vec![(mid, middle(centre))];
    let (bw, bh) = button_size(lay.icon, lay.names);
    for p in lay.placed.iter() {
        let (x, y) = (p.x as f32, p.y as f32);
        let m = if (x - mid.0).hypot(y - mid.1) < reach { hidden } else { seen };
        let at = (x - bw as f32 / 2.0, y - bh as f32 / 2.0);
        all.push(((x, y), placed(live((bw as f32, bh as f32), "Disks", m), move |_, _| at)));
    }
    all
}

fn covers(disc_radius: f64) -> Vec<(String, f32, Vec<Node<Msg>>)> {
    let across = quick_side(disc_radius) as f32;
    let mut all: Vec<(String, f32, Vec<Node<Msg>>)> = crate::ring::quick::tests::pages().into_iter().map(|(name, mut q)| {
        q.set_size(across as i32);
        let mut parts = vec![middle(disc(across, |t| t.raised))];
        if q.asking() {
            parts.push(middle(disc(q.pop_size(across as i32) as f32, |t| t.raised_more)));
        }
        parts.push(middle(q.view()));
        (name.to_string(), across / 2.0, parts)
    }).collect();
    all.push(("tray".into(), MIDDLE_COVER / 2.0, vec![middle(disc(MIDDLE_COVER, |t| t.raised)), middle(tray_cover())]));
    all.push(("power".into(), MIDDLE_COVER / 2.0, vec![middle(disc(MIDDLE_COVER, |t| t.raised))]));
    all
}

fn key(sym: Keysym, shift: bool) -> Input {
    Input::Key { sym, text: None, mods: Mods { shift, ..Default::default() }, repeat: false }
}

#[test]
fn nothing_under_a_cover_takes_a_press() {
    let (mark, seen) = (crate::ring::ctx::msg(|_| {}), crate::ring::ctx::msg(|_| {}));
    let from_under = |got: &[Msg]| got.iter().any(|g| Rc::ptr_eq(&g.0, &mark.0));
    let (w, h) = SIZE;
    let mid = (w / 2.0, h / 2.0);
    let mut faults = Vec::new();
    for (name, reach, parts) in covers(ring_layout().disc_radius) {
        let beneath = under(&mark, &seen, reach);
        let mut all: Vec<Node<Msg>> = uncovered(Some(reach), mid, beneath).collect();
        all.extend(parts);
        let mut ui = Ui::new(layers(all).width(Length::Fixed(w)).height(Length::Fixed(h)), look());
        ui.lay_at(w as u32, h as u32);
        let now = Instant::now();
        let step = 10.0;
        let mut y = mid.1 - reach;
        while y <= mid.1 + reach {
            let mut x = mid.0 - reach;
            while x <= mid.0 + reach {
                if (x - mid.0).hypot(y - mid.1) < reach - 1.0 {
                    let mut got = ui.input(&Input::PointerMove { x, y }, now).messages;
                    for b in [Button::Left, Button::Right, Button::Middle] {
                        got.extend(ui.input(&Input::Press { button: b, x, y }, now).messages);
                        got.extend(ui.input(&Input::Release { button: b, x, y }, now).messages);
                        got.extend(ui.input(&key(Keysym::Escape, false), now).messages);
                    }
                    got.extend(ui.input(&Input::Scroll { x: 0.0, y: 1.0, steps: true }, now).messages);
                    got.extend(ui.input(&Input::PointerOut, now).messages);
                    got.extend(ui.input(&Input::TouchDown { id: 1, x, y }, now).messages);
                    got.extend(ui.input(&Input::TouchUp { id: 1 }, now).messages);
                    if from_under(&got) {
                        faults.push(format!("{name}: at {x:.0}, {y:.0}"));
                    }
                }
                x += step;
            }
            y += step;
        }
        let mut got = Vec::new();
        for _ in 0..80 {
            got.extend(ui.input(&key(Keysym::Tab, false), now).messages);
            got.extend(ui.input(&key(Keysym::Return, false), now).messages);
        }
        if from_under(&got) {
            faults.push(format!("{name}: by the keyboard"));
        }

        let reached = got.iter().any(|g| Rc::ptr_eq(&g.0, &seen.0));
        if reached != matches!(name.as_str(), "tray" | "power") {
            faults.push(format!("{name}: the favourites round it answer: {reached}"));
        }
    }
    let shown: Vec<&String> = faults.iter().take(12).collect();
    assert!(faults.is_empty(), "{} presses reached what is under a cover, among them {shown:?}", faults.len());
}

#[test]
fn the_view_puts_the_middle_and_every_favourite_through_the_rule() {
    let code = super::tests::view_code();
    let view = &code[code.find("fn view(&self)").unwrap()..];
    let view = &view[..view.find("\n    }\n").unwrap()];
    assert!(view.contains("uncovered(cover, mid, under)"), "the view does not use the rule");
    for part in ["self.centre()", "self.app_layers(&mut under)", "self.segment(", "self.top_segment("] {
        let at = view.find(part).unwrap_or_else(|| panic!("{part} is not in the view"));
        assert!(at < view.find("uncovered(").unwrap(), "{part} is put in the ring after the rule, not through it");
    }
}
