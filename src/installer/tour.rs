//! The short tour shown while Trindle is being written to the disk.

use crate::toolkit::colour::paint as ink;
use std::cell::RefCell;

use tiny_skia::{FillRule, PathBuilder, Pixmap, PixmapMut, PixmapPaint, Stroke, Transform};

use crate::toolkit::t;
use crate::toolkit::text::Style;
use crate::toolkit::widget::Pen;

struct Note {
    x: f32,
    y: f32,
    right: bool,
    text: &'static str,
}

struct Slide {
    picture: &'static str,
    title: &'static str,
    notes: Vec<Note>,
}

fn slides() -> Vec<Slide> {
    vec![
        Slide {
            picture: "ring.jpg",
            title: t!("installer-tour-ring"),
            notes: vec![
                Note { x: 0.05, y: 0.10, right: false, text: t!("installer-tour-ring-corner") },
                Note { x: 0.50, y: 0.45, right: true, text: t!("installer-tour-ring-time") },
                Note { x: 0.50, y: 0.35, right: true, text: t!("installer-tour-ring-programs") },
                Note { x: 0.50, y: 0.55, right: false, text: t!("installer-tour-ring-switches") },
                Note { x: 0.50, y: 0.96, right: false, text: t!("installer-tour-ring-foot") },
            ],
        },
        Slide {
            picture: "settings.jpg",
            title: t!("installer-tour-settings"),
            notes: vec![
                Note { x: 0.30, y: 0.18, right: false, text: t!("installer-tour-settings-search") },
                Note { x: 0.28, y: 0.44, right: false, text: t!("installer-tour-settings-network") },
                Note { x: 0.56, y: 0.33, right: true, text: t!("installer-tour-settings-dark") },
                Note { x: 0.60, y: 0.52, right: true, text: t!("installer-tour-settings-colours") },
            ],
        },
        Slide {
            picture: "store.jpg",
            title: t!("installer-tour-apps"),
            notes: vec![
                Note { x: 0.30, y: 0.20, right: false, text: t!("installer-tour-apps-search") },
                Note { x: 0.62, y: 0.40, right: true, text: t!("installer-tour-apps-many") },
                Note { x: 0.30, y: 0.70, right: false, text: t!("installer-tour-apps-updates") },
            ],
        },
        Slide {
            picture: "settings.jpg",
            title: t!("installer-tour-voice"),
            notes: vec![
                Note { x: 0.28, y: 0.235, right: false, text: t!("installer-tour-voice-where") },
                Note { x: 0.62, y: 0.30, right: true, text: t!("installer-tour-voice-add") },
                Note { x: 0.62, y: 0.58, right: true, text: t!("installer-tour-voice-say") },
                Note { x: 0.30, y: 0.72, right: false, text: t!("installer-tour-voice-private") },
            ],
        },
        Slide {
            picture: "snapshots.jpg",
            title: t!("installer-tour-snapshots"),
            notes: vec![
                Note { x: 0.35, y: 0.30, right: false, text: t!("installer-tour-snapshots-before") },
                Note { x: 0.62, y: 0.50, right: true, text: t!("installer-tour-snapshots-back") },
            ],
        },
    ]
}

fn wrap(text: &str, fits: impl Fn(&str) -> bool) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        if !line.is_empty() && !fits(&format!("{line} {word}")) {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

pub const COUNT: usize = 5;

pub fn title(slide: usize) -> &'static str {
    slides().swap_remove(slide % COUNT).title
}

pub type Cache = RefCell<Option<Kept>>;

pub struct Kept {
    slide: usize,
    full: (f32, f32),
    shown: Pixmap,
}

pub fn paint(canvas: &mut PixmapMut, pen: &Pen, slide: usize, cache: &Cache) {
    let slide_at = slide % COUNT;
    let all = slides();
    let slide = &all[slide_at];
    let (w, h) = (canvas.width() as f32, canvas.height() as f32);

    let margin = (w * 0.24).min(230.0 * pen.scale);
    let Some(shown) = photo(canvas, slide, slide_at, (w, h, margin), cache) else { return };
    write_notes(canvas, pen, slide, (w, h, margin), shown);
}

fn photo(canvas: &mut PixmapMut, slide: &Slide, slide_at: usize, (w, h, margin): (f32, f32, f32), cache: &Cache) -> Option<(f32, f32, f32, f32)> {
    let fit = |(iw, ih): (f32, f32)| {
        let scale = ((w - margin * 2.0) / iw).min(h / ih).max(0.0);
        ((iw * scale).round().max(1.0), (ih * scale).round().max(1.0))
    };
    let mut kept = cache.borrow_mut();
    let fresh = kept.as_ref().is_some_and(|k| {
        let (dw, dh) = fit(k.full);
        k.slide == slide_at && (k.shown.width(), k.shown.height()) == (dw as u32, dh as u32)
    });
    if !fresh {
        *kept = None;
        let path = crate::common::files::share("tour").join(slide.picture);
        if let Some(img) = crate::toolkit::picture::load(&path) {
            let full = (img.width() as f32, img.height() as f32);
            let (dw, dh) = fit(full);
            if let Some(mut shown) = Pixmap::new(dw as u32, dh as u32) {
                crate::toolkit::picture::draw(&mut shown.as_mut(), &img, 0, 0, dw as u32, dh as u32);
                *kept = Some(Kept { slide: slide_at, full, shown });
            }
        }
    }
    let k = kept.as_ref()?;
    let (dw, dh) = (k.shown.width() as f32, k.shown.height() as f32);
    let (dx, dy) = (((w - dw) / 2.0).round(), ((h - dh) / 2.0).round());
    canvas.draw_pixmap(dx as i32, dy as i32, k.shown.as_ref(), &PixmapPaint::default(), Transform::identity(), None);
    Some((dx, dy, dw, dh))
}

fn write_notes(canvas: &mut PixmapMut, pen: &Pen, slide: &Slide, (w, h, margin): (f32, f32, f32), (dx, dy, dw, dh): (f32, f32, f32, f32)) {
    let s = pen.scale;
    let (words, line) = (pen.theme.fg, pen.theme.accent);
    let style = Style::new(13.0 * s);
    let stroke = Stroke { width: 2.0 * s, line_cap: tiny_skia::LineCap::Round, ..Stroke::default() };
    for side in [false, true] {
        let mine: Vec<&Note> = slide.notes.iter().filter(|n| n.right == side).collect();
        for (i, note) in mine.iter().enumerate() {
            let spot = (dx + note.x * dw, dy + note.y * dh);
            let ly = h * (i as f32 + 1.0) / (mine.len() as f32 + 1.0);
            let lx = if side { w - margin + 10.0 * s } else { margin - 10.0 * s };

            if let Some(dot) = PathBuilder::from_circle(spot.0, spot.1, 3.0 * s) {
                canvas.fill_path(&dot, &ink(line), FillRule::Winding, Transform::identity(), None);
            }
            let mut pb = PathBuilder::new();
            pb.move_to(spot.0, spot.1);
            pb.line_to(lx, spot.1);
            pb.line_to(lx, ly);
            if let Some(path) = pb.finish() {
                canvas.stroke_path(&path, &ink(line), &stroke, Transform::identity(), None);
            }

            let room = margin - 26.0 * s;
            let lines = wrap(note.text, |l| pen.fonts.layout(l, &style, None).width <= room);
            let start = ly - (lines.len() as f32 - 1.0) * 8.0 * s;
            for (n, line) in lines.iter().enumerate() {
                let layout = pen.fonts.layout(line, &style, None);
                let base = layout.lines.first().map_or(layout.height, |l| l.baseline);
                let y = start + n as f32 * 16.0 * s - base;
                let x = if side { lx + 8.0 * s } else { lx - 8.0 * s - layout.width };
                pen.fonts.draw(canvas, &layout, x.round(), y.round(), words);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::wrap;

    #[test]
    fn lines_as_long_as_they_fit() {
        let fits = |l: &str| l.chars().count() <= 12;
        assert_eq!(wrap("Put the pointer in this corner", fits), vec!["Put the", "pointer in", "this corner"]);
        assert_eq!(wrap("A wordtoolongtofitanywhere here", fits), vec!["A", "wordtoolongtofitanywhere", "here"]);
        assert!(wrap("", fits).is_empty());
    }
}
