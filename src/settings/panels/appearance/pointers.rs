//! Appearance: the mouse pointer, its theme and its size, with a drawing of the pointer in use.

use std::collections::BTreeSet;
use std::path::PathBuf;

use super::{Appearance, write_ini};
use crate::common::files::home;
use crate::common::run;
use crate::settings::session_env;
use crate::settings::ui::{dropdown, group, line, on, Kit, Msg, Page, not_saved};
use crate::toolkit::t;
use crate::toolkit::widget::{self, drawing, note, row, Length, Node, Rect};
use tiny_skia::Transform;

impl Appearance {
    pub(super) fn cursor(&mut self, kit: &mut Kit, key: &'static str, theme: Option<String>, size: Option<String>) {
        if let Some(t) = &theme {
            self.cursor = t.clone();
            self.pointer_art = pointer_art(t);
        }
        if let Some(s) = &size {
            self.cursor_size = s.clone();
        }
        let back = kit.then::<Appearance, std::io::Result<()>>(move |me, k, written| {

            if k.settle_quietly(key, written, not_saved()).is_none() {
                me.shown(k);
                return;
            }
            let mut changes: Vec<(&str, Option<&str>)> = Vec::new();
            if let Some(t) = &theme {
                changes.push(("XCURSOR_THEME", Some(t)));
            }
            if let Some(s) = &size {
                changes.push(("XCURSOR_SIZE", Some(s)));
            }
            if !session_env::update(&changes, k, key, "Saved") {
                me.shown(k);
            }
        });
        let (t, s) = (self.cursor.clone(), self.cursor_size.clone());
        let (theme_set, size_set) = (key == CURSOR, key == CURSOR_SIZE);
        run::on_thread(move || {
            if theme_set {
                write_ini("gtk-cursor-theme-name", &t)?;
            }
            if size_set {
                write_ini("gtk-cursor-theme-size", &s)?;
            }
            Ok(())
        }, back);
    }

    pub(super) fn pointer_card(&self, kit: &Kit) -> Node<Msg> {

        let shown: Vec<String> = self.cursors.iter()
            .map(|t| if t == "Adwaita" { t!("settings-appearance-cursor-standard").to_string() } else { t.clone() })
            .collect();
        let at = self.cursors.iter().position(|t| *t == self.cursor);
        let themes = self.cursors.clone();

        let theme = if shown.len() < 2 {
            note(shown.first().cloned().unwrap_or_else(|| t!("settings-appearance-cursor-standard").to_string()))
        } else {
            dropdown(CURSOR, shown, at, t!("settings-appearance-cursor"), move |i| {
                let t = themes[i].clone();
                on::<Appearance>(move |me, k| me.cursor(k, CURSOR, Some(t.clone()), None))
            })
        };
        let size_at = CURSOR_SIZES.iter().position(|v| *v == self.cursor_size);
        let size = dropdown(CURSOR_SIZE, (0..CURSOR_SIZES.len()).map(|i| cursor_size_name(i).to_string()).collect(), size_at,
                            t!("settings-appearance-cursor-size"), |i| {
            let v = CURSOR_SIZES[i].to_string();
            on::<Appearance>(move |me, k| me.cursor(k, CURSOR_SIZE, None, Some(v.clone())))
        });
        group(t!("settings-appearance-pointer-title")).icon("input-mouse-symbolic")
            .vis(pointers(&self.pointer_art, size_at))
            .rows(vec![line(t!("settings-appearance-cursor")).key(CURSOR).control(theme).node(kit),
                       line(t!("settings-appearance-cursor-size")).key(CURSOR_SIZE).control(size).node(kit)])
            .node(kit)
    }
}

const CURSOR_SIZES: [&str; 3] = ["24", "32", "48"];

fn cursor_size_name(i: usize) -> &'static str {
    match i {
        0 => t!("settings-appearance-cursor-normal"),
        1 => t!("settings-appearance-cursor-large"),
        _ => t!("settings-appearance-cursor-larger"),
    }
}

const CURSOR: &str = "appearance:cursor";
const CURSOR_SIZE: &str = "appearance:cursor-size";

fn icon_dirs() -> [PathBuf; 3] {
    [home().join(".local/share/icons"), home().join(".icons"), PathBuf::from("/usr/share/icons")]
}

pub(super) fn cursor_themes() -> Vec<String> {
    let mut found = BTreeSet::new();
    for base in icon_dirs() {
        let Ok(entries) = std::fs::read_dir(&base) else { continue };
        for e in entries.flatten() {
            if e.path().join("cursors").is_dir() {
                found.insert(e.file_name().to_string_lossy().to_string());
            }
        }
    }
    let mut out: Vec<String> = found.into_iter().collect();
    out.sort_by_key(|s| s.to_lowercase());
    out
}

pub(super) type PointerArt = std::rc::Rc<Vec<Arrow>>;

pub(super) struct Arrow {
    size: u32,
    pic: tiny_skia::Pixmap,
}

const MOST_FILE: u64 = 4 << 20;
const MOST_SIDE: u32 = 512;

pub(super) fn pointer_art(name: &str) -> PointerArt {
    let read = || {
        let path = xcursor::CursorTheme::load(name).load_icon("default")?;
        let mut bytes = Vec::new();
        std::io::Read::read_to_end(&mut std::io::Read::take(std::fs::File::open(path).ok()?, MOST_FILE + 1), &mut bytes).ok()?;
        if bytes.len() as u64 > MOST_FILE {
            return None;
        }
        xcursor::parser::parse_xcursor(&bytes)
    };
    std::rc::Rc::new(read().unwrap_or_default().iter().filter_map(arrow).collect())
}

fn arrow(img: &xcursor::parser::Image) -> Option<Arrow> {
    if img.width == 0 || img.height == 0 || img.width > MOST_SIDE || img.height > MOST_SIDE {
        return None;
    }
    let mut pic = tiny_skia::Pixmap::new(img.width, img.height)?;
    if img.pixels_rgba.len() < pic.pixels().len() * 4 {
        return None;
    }
    for (out, px) in pic.pixels_mut().iter_mut().zip(img.pixels_rgba.chunks_exact(4)) {

        let a = px[3];
        if let Some(c) = tiny_skia::PremultipliedColorU8::from_rgba(px[0].min(a), px[1].min(a), px[2].min(a), a) {
            *out = c;
        }
    }
    Some(Arrow { size: img.size.max(1), pic })
}

fn nearest(art: &[Arrow], size: u32) -> Option<&Arrow> {
    art.iter().min_by_key(|i| i.size.abs_diff(size))
}

const POINTER_TILE: f32 = 64.0;

pub(super) fn pointers(art: &PointerArt, chosen: Option<usize>) -> Node<Msg> {
    let tiles = CURSOR_SIZES.iter().enumerate().map(|(i, value)| {
        let name = cursor_size_name(i);
        let (art, size, lit) = (art.clone(), value.parse::<u32>().unwrap_or(24), chosen == Some(i));
        let v = value.to_string();
        let press = move || {
            let v = v.clone();
            on::<Appearance>(move |me, k| me.cursor(k, CURSOR_SIZE, None, Some(v.clone())))
        };
        let keys = press.clone();
        drawing((POINTER_TILE, POINTER_TILE), t!("settings-appearance-pointer-spoken", size = name), move |canvas, pen| {
            let s = pen.scale;
            let side = POINTER_TILE * s;
            if lit {
                let sheet = crate::toolkit::theme::sheet();
                crate::toolkit::ui::fill(canvas, Rect { x: 0.0, y: 0.0, w: side, h: side }, sheet.corner_of(crate::toolkit::theme::Part::Button) * s, pen.theme.raised);
            }
            let want = (size as f32 * s).round().max(1.0);
            let Some(img) = nearest(&art, want as u32) else { return };
            let k = want / img.size as f32;
            let (w, h) = (img.pic.width() as f32 * k, img.pic.height() as f32 * k);
            let at = Transform::from_scale(k, k).post_translate(((side - w) / 2.0).round(), ((side - h) / 2.0).round());
            let paint = tiny_skia::PixmapPaint { quality: tiny_skia::FilterQuality::Bilinear, ..Default::default() };
            canvas.draw_pixmap(0, 0, img.pic.as_ref(), &paint, at, None);
        })
        .pressable(press())
        .on_pointer(move |p| match p {
            widget::Pointer::Up(..) => press(),
            _ => on::<Appearance>(|_, _| {}),
        })
        .on_keys(move |input| match input {
            crate::toolkit::input::Input::Key { sym: crate::toolkit::input::Keysym::Return | crate::toolkit::input::Keysym::space, .. } => Some(keys()),
            _ => None,
        })
        .key(format!("pointer-size:{value}"))
    }).collect();
    row(tiles).gap(8.0).width(Length::Shrink)
}
