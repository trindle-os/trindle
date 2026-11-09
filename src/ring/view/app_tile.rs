//! One app in the ring: its button, the pointer on it and how its icon and name are drawn.

use std::cell::Cell;
use std::rc::Rc;

use tiny_skia::{Pixmap, PixmapMut};

use crate::common::favourites;
use crate::ring::{layout, store};
use crate::ring::ctx::{msg, Msg, Program};
use crate::toolkit::colour::Colour;
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::text::Style;
use crate::toolkit::widget::{drawing, Node, Pen, Pointer};

use super::Face;
use super::pinned::Ink;

const DRAG_FROM: f32 = 10.0;

pub(in crate::ring) const LABEL_SIZE: f32 = 12.0;

pub(in crate::ring) const NAME_EDGE: f32 = 2.0;

pub(super) fn opens_menu(sym: Keysym, shift: bool) -> bool {
    matches!(sym, Keysym::Menu) || (matches!(sym, Keysym::F10) && shift)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn app_button(app: &Program, i: usize, icon: i32, names: bool, best: bool, lit: bool, at: [f32; 4], ink: Ink) -> Node<Msg> {

    let icons = if favourites::is_path(&app.id) { app.icon.clone().into_iter().collect() } else { app.icons() };
    let name = app.name.clone();
    let under = name.clone();
    let cut = Rc::new(Cell::new(false));
    let was_cut = cut.clone();
    drawing((at[2], at[3]), name, move |pm, pen| {
        paint_app(pm, pen, &icons, &under, icon, names, best, lit, ink);
        was_cut.set(names && name_is_cut(pen, &under, pm.width() as f32));
    })

    .named(names)
    .tip_if_cut(cut)
    .pressable(msg(move |f| {
        f.pressed_at = at;
        f.activate(i)
    }))
    .on_hover()
    .on_keys(move |input| match input {
        Input::Key { sym: Keysym::Return | Keysym::space, .. } => Some(msg(move |f| {
            f.pressed_at = at;
            f.activate(i)
        })),
        Input::Key { sym, mods, .. } if opens_menu(*sym, mods.shift) => Some(msg(move |f| f.menu = Some((i, at)))),
        _ => None,
    })
    .on_pointer(move |p| match p {
        Pointer::Hover(..) => Msg::unless(move |f| f.pointer_on_app(i, at, p), move |f| f.hover_app == Some(i)),
        Pointer::Leave => Msg::unless(move |f| f.pointer_on_app(i, at, p), move |f| f.hover_app.is_none()),
        _ => msg(move |f| f.pointer_on_app(i, at, p)),
    })
}

#[allow(clippy::too_many_arguments)]
pub(super) fn paint_app(pm: &mut PixmapMut, pen: &Pen, icons: &[String], name: &str, icon: i32, names: bool, best: bool, lit: bool, ink: Ink) {
    let s = pen.scale;
    let x = ((pm.width() as f32 - icon as f32 * s) / 2.0) as i32;
    let y = (5.0 * s) as i32;
    let scale = s.round().max(1.0) as u32;
    let size = ((icon as f32 * s) / scale as f32).round() as u32;
    let ink = match ink {
        Ink::Usual => pen.theme.fg,

        Ink::Accent => pen.theme.folder_ink(None, &[pen.theme.bg, pen.theme.panel]),
        Ink::Own(own) => own,
    };
    let draw = |pm: &mut PixmapMut| icons.iter().any(|n| pen.icons.draw(pm, n, x, y, size, scale, ink).is_some());
    if best || lit {
        let edge = (crate::toolkit::theme::sheet().ring_app_edge * s).round().max(1.0) as i32;
        if let Some(mut alone) = Pixmap::new(pm.width(), pm.height()) {
            if draw(&mut alone.as_mut()) {
                edge_round(pm, &alone, pen.theme.accent, edge);
            }
        }
    }
    draw(pm);
    if names {
        paint_name(pm, pen, name, (icon as f32 + 10.0) * s, pen.theme.fg);
    }
}

pub(in crate::ring) fn paint_name(pm: &mut PixmapMut, pen: &Pen, name: &str, y: f32, ink: Colour) {
    let style = Style::new(LABEL_SIZE * pen.scale);
    let dots = pen.fonts.layout("\u{2026}", &style, None);
    let room = name_room(pen, pm.width() as f32);
    let l = pen.fonts.layout(name, &style, None).cut(room, &dots);
    pen.fonts.draw(pm, &l, ((pm.width() as f32 - l.width) / 2.0).max(NAME_EDGE * pen.scale), y, ink);
}

fn name_room(pen: &Pen, across: f32) -> f32 {
    (across - 2.0 * NAME_EDGE * pen.scale).max(0.0)
}

pub(in crate::ring) fn name_is_cut(pen: &Pen, name: &str, across: f32) -> bool {
    pen.fonts.layout(name, &Style::new(LABEL_SIZE * pen.scale), None).width > name_room(pen, across)
}

fn edge_round(pm: &mut PixmapMut, shape: &Pixmap, ink: Colour, edge: i32) {
    let (w, h) = (shape.width() as i32, shape.height() as i32);
    let alpha: Vec<u8> = shape.pixels().iter().map(|p| p.alpha()).collect();
    let reach: Vec<(i32, i32)> = (-edge..=edge).flat_map(|dy| (-edge..=edge).map(move |dx| (dx, dy)))
        .filter(|(dx, dy)| dx * dx + dy * dy <= edge * edge).collect();
    let ink = ink.to_skia();
    let (r, g, b) = (ink.red(), ink.green(), ink.blue());
    let out = pm.pixels_mut();
    for py in 0..h {
        for px in 0..w {
            let most = reach.iter().filter_map(|(dx, dy)| {
                let (qx, qy) = (px + dx, py + dy);
                (qx >= 0 && qy >= 0 && qx < w && qy < h).then(|| alpha[(qy * w + qx) as usize])
            }).max().unwrap_or(0);
            if most == 0 {
                continue;
            }

            let k = most as f32 / 255.0;
            let under = out[(py * w + px) as usize];
            let mix = |ink: f32, was: u8| ((ink * k + was as f32 / 255.0 * (1.0 - k)) * 255.0).round().clamp(0.0, 255.0) as u8;
            let a = mix(1.0, under.alpha());
            let (nr, ng, nb) = (mix(r, under.red()).min(a), mix(g, under.green()).min(a), mix(b, under.blue()).min(a));
            if let Some(c) = tiny_skia::PremultipliedColorU8::from_rgba(nr, ng, nb, a) {
                out[(py * w + px) as usize] = c;
            }
        }
    }
}

impl Face {
    fn pointer_on_app(&mut self, i: usize, at: [f32; 4], p: Pointer) {
        match p {
            Pointer::Down(x, y) => {
                self.pressed_at = at;
                self.press = Some((i, x, y, false));
            }
            Pointer::Move(x, y) => {
                if let Some((j, sx, sy, moved)) = self.press {
                    if j == i && !moved && ((x - sx).powi(2) + (y - sy).powi(2)).sqrt() > DRAG_FROM {
                        self.press = Some((j, sx, sy, true));
                    }
                }
            }
            Pointer::Up(x, y) => match self.press.take() {
                Some((j, _, _, false)) if j == i => self.activate(i),
                Some((j, _, _, true)) if j == i => self.drop_favourite(i, at[0] + x, at[1] + y),
                _ => {}
            },
            Pointer::Menu(_, _) => {
                self.press = None;
                self.gone = None;
                self.open_menu(i, at);
            }
            Pointer::Hover(_, _) => self.hover_app = Some(i),
            Pointer::Leave => self.hover_app = None,
            _ => {}
        }
    }

    fn drop_favourite(&mut self, i: usize, x: f32, y: f32) {
        let all = self.all();
        let Some(id) = all.get(i).map(|a| a.id.clone()) else { return };
        if !self.favourites.contains(&id) {
            return;
        }
        let near = self.lay.placed[..self.first_len.min(self.lay.placed.len())]
            .iter()
            .min_by(|a, b| {
                let d = |p: &layout::Placed<usize>| (p.x - x as f64).powi(2) + (p.y - y as f64).powi(2);
                d(a).total_cmp(&d(b))
            })
            .filter(|p| ((p.x - x as f64).powi(2) + (p.y - y as f64).powi(2)).sqrt() <= self.lay.pitch)
            .and_then(|p| all.get(p.item))
            .map(|a| a.id.clone());
        if let Some(to) = near {
            if store::move_favourite(&mut self.favourites, &id, &to) {
                store::save_favourites(&self.favourites);
                self.fav_stamp = store::favourites_changed_at();
                self.relayout();
            }
        }
    }
}
