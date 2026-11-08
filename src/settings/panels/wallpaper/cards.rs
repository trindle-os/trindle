//! Wallpaper: the cards of the page and the pictures drawn in them.

use std::path::Path;

use super::{NEXT, PICTURES, PREVIEW_STEP, Slot, TILE_STEP, Wallpaper, every_choices, fits, no_changer, orders, tell_changer};
use crate::session::moving;
use crate::session::wallpaper::own_pictures;
use crate::settings::ui::{dropdown, group, line, tiles, Kit, Msg, on, ROW_GAP};
use crate::toolkit::colour::Colour;
use crate::toolkit::t;
use crate::toolkit::text::Style;
use crate::toolkit::tiles::picture;
use crate::toolkit::widget::{self as w, button, drawing, entry, switch, Length, Node, Pen, Pointer, Rect};
use tiny_skia::{FilterQuality, Paint, Pattern, Pixmap, PixmapMut, SpreadMode, Stroke, Transform};

impl Wallpaper {
    pub(super) fn preview(&self) -> Node<Msg> {
        let path = self.now.showing.clone();
        let px = self.frame(Slot::Preview, &path).or_else(|| self.thumb(&path, PREVIEW_STEP));

        drawing((360.0, 280.0), self.now.showing_words.clone(), move |canvas, pen| {
            paint_cover(canvas, pen, px.as_deref(), 1.0, false);
        })
        .width(Length::Fill)
    }

    pub(super) fn tile(&self, path: &Path) -> Node<Msg> {
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        let lit = if self.now.rotating { !self.now.skip.contains(&name) } else { path == self.now.src };
        let animated = moving::is_moving(path);
        let px = self.frame(Slot::Tile, path).or_else(|| self.thumb(path, TILE_STEP));
        let mut pic = drawing((150.0, 85.0), name, move |canvas, pen| {
            paint_cover(canvas, pen, px.as_deref(), if lit { 1.0 } else { 0.35 }, lit);
            if animated {
                paint_animated(canvas, pen);
            }
        })
        .width(Length::Fill)
        .height(Length::Fill);
        if animated {

            let p = path.to_path_buf();
            pic = pic.on_pointer(move |at| {
                let p = p.clone();
                match at {
                    Pointer::Hover(..) => on::<Wallpaper>(move |me, k| {
                        if me.tile.as_ref().is_none_or(|t| t.path != p) {
                            me.play(k, Slot::Tile, &p, TILE_STEP);
                        }
                    }),
                    Pointer::Leave => on::<Wallpaper>(move |me, _| {
                        if me.tile.as_ref().is_some_and(|t| t.path == p) {
                            me.tile = None;
                        }
                    }),
                    _ => on::<Wallpaper>(|_, _| {}),
                }
            }).on_hover();
        }
        let p = path.to_path_buf();
        w::card(pic).on_press(on::<Wallpaper>(move |me, k| me.press_tile(k, &p))).key(format!("tile:{}", path.display()))
    }

    pub(super) fn picture_card(&self, shade: Option<Colour>) -> Node<Msg> {
        let now = &self.now;

        let art = if now.plain {
            drawing((360.0, 220.0), t!("settings-wallpaper-plain-name"), move |canvas, pen| {
                let r = Rect { x: 0.0, y: 0.0, w: canvas.width() as f32, h: canvas.height() as f32 };
                let Some(shape) = crate::toolkit::ui::rounded(r, crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Picture, r.w.min(r.h), pen.scale)) else { return };
                let mut paint = Paint { anti_alias: true, ..Paint::default() };
                paint.set_color(shade.unwrap_or(pen.theme.bg).to_skia());
                canvas.fill_path(&shape, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
            }).width(Length::Fill)
        } else {
            self.preview()
        };

        picture("preferences-desktop-wallpaper-symbolic", t!("settings-wallpaper-on-screen"), art).span(HALF, HALF, HALF_WORDS)
    }

    pub(super) fn wallpaper_card(&self, kit: &Kit, shade: Option<Colour>) -> Node<Msg> {
        let now = &self.now;
        let swatch = drawing((32.0, 32.0), "", move |canvas, pen| {
            let r = Rect { x: 0.0, y: 0.0, w: canvas.width() as f32, h: canvas.height() as f32 };
            let Some(shape) = crate::toolkit::ui::rounded(r.inset(pen.scale), crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Picture, r.w.min(r.h), pen.scale)) else { return };
            let mut paint = Paint { anti_alias: true, ..Paint::default() };
            paint.set_color(shade.unwrap_or(pen.theme.raised).to_skia());
            canvas.fill_path(&shape, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
            paint.set_color(pen.theme.card_edge.to_skia());
            canvas.stroke_path(&shape, &paint, &Stroke { width: pen.scale, ..Stroke::default() }, Transform::identity(), None);
        });

        let fit_at = fits().iter().position(|(v, _)| *v == now.fit);

        let colour_shows = now.plain || matches!(now.fit.as_str(), "fit" | "center");
        let mut top = vec![
            line(t!("settings-wallpaper-wallpaper")).sub(t!("settings-wallpaper-wallpaper-about")).key("switch")
                .control(switch(!now.plain, t!("settings-wallpaper-wallpaper"), |v| on::<Wallpaper>(move |me, k| me.set_wallpaper(k, v))))
                .node(kit),

            line(t!("settings-wallpaper-picture")).sub(t!("settings-wallpaper-picture-about")).key("source")
                .controls(vec![
                    button("image-x-generic-symbolic", t!("settings-wallpaper-choose-picture-button")).on_press(on::<Wallpaper>(|me, k| me.pick_file(k))),
                    button("folder-pictures-symbolic", t!("settings-wallpaper-choose-folder")).on_press(on::<Wallpaper>(|me, k| me.pick_folder(k))),
                    button("user-desktop-symbolic", t!("settings-wallpaper-own-pictures")).on_press(on::<Wallpaper>(|me, k| {
                        me.set(k, "wallpaper", &own_pictures().to_string_lossy(), None);
                    })),
                ])
                .node(kit).off(now.plain),
            line(t!("settings-wallpaper-fit")).sub(t!("settings-wallpaper-fit-about")).key("fit")
                .control(dropdown(kit, "fit", fits().iter().map(|(_, l)| l.to_string()).collect(), fit_at, t!("settings-wallpaper-fit-name"),
                                  |i| on::<Wallpaper>(move |me, k| me.set(k, "wallpaper_fit", fits()[i].0, Some("fit")))))
                .node(kit).off(now.plain),
            line(t!("settings-wallpaper-colour"))
                .sub(if colour_shows { t!("settings-wallpaper-colour-shown") } else { t!("settings-wallpaper-colour-hidden") })
                .key("colour")
                .controls(vec![
                    swatch,
                    entry(self.colour.clone(), t!("settings-wallpaper-colour-hex"),
                          |t| on::<Wallpaper>(move |me, k| me.typed(k, t.clone())))
                        .width(Length::Fixed(110.0)),
                ])
                .node(kit).off(!colour_shows),
        ];

        if !now.plain {
            top.extend(now.memory.iter().map(|words| crate::toolkit::widget::note(words.to_string())));
        }
        group(t!("settings-wallpaper-card-title")).icon("preferences-desktop-wallpaper-symbolic")
            .about(t!("settings-wallpaper-card-about"))
            .rows(vec![spread(top)])
            .span(HALF, HALF)
            .node(kit)
    }

    pub(super) fn changing_card(&self, kit: &Kit) -> Node<Msg> {
        let now = &self.now;
        let waits = now.plain.then_some(t!("settings-wallpaper-waits"));
        let every_at = every_choices().iter().position(|(v, _)| *v == now.every);
        let order_at = orders().iter().position(|(v, _)| *v == now.order);

        let often = vec![
            line(t!("settings-wallpaper-rotate")).sub(t!("settings-wallpaper-rotate-about"))
                .control(switch(now.rotating, t!("settings-wallpaper-rotate"),
                                |v| on::<Wallpaper>(move |me, k| me.set_rotating(k, v))))
                .node(kit),
            line(t!("settings-wallpaper-every")).sub(t!("settings-wallpaper-every-about")).key("every")
                .control(dropdown(kit, "every", every_choices().iter().map(|(_, l)| l.to_string()).collect(), every_at, t!("settings-wallpaper-every"),
                                  |i| on::<Wallpaper>(move |me, k| me.set(k, "wallpaper_every", &every_choices()[i].0.to_string(), Some("every")))))
                .node(kit).off(!now.rotating),
        ];
        let which = vec![
            line(t!("settings-wallpaper-order")).sub(t!("settings-wallpaper-order-about")).key("order")
                .control(dropdown(kit, "order", orders().iter().map(|(_, l)| l.to_string()).collect(), order_at, t!("settings-wallpaper-order"),
                                  |i| on::<Wallpaper>(move |me, k| me.set(k, "wallpaper_order", orders()[i].0, Some("order")))))
                .node(kit).off(!now.rotating),
            line(t!("settings-wallpaper-next")).sub(t!("settings-wallpaper-next-about")).key("next")
                .control(button("media-skip-forward-symbolic", t!("settings-wallpaper-next")).key(NEXT).on_press(on::<Wallpaper>(|_, k| {
                    let told = tell_changer(true);
                    k.done("next", (!told).then_some(no_changer()), t!("settings-wallpaper-changed"));
                })))
                .node(kit).off(!now.rotating),
        ];
        let side = |lines: Vec<Node<Msg>>| w::column(lines).gap(ROW_GAP);
        group(t!("settings-wallpaper-changing-title")).icon("media-playlist-shuffle-symbolic")
            .about(t!("settings-wallpaper-changing-about"))
            .waits_for(waits)
            .rows(vec![w::even_grid(COLUMN_LEAST, vec![side(often), side(which)]).gap(COLUMN_GAP)])
            .span(WHOLE, WHOLE)
            .wide_by_choice()
            .node(kit)
    }

    pub(super) fn pictures_card(&self, kit: &Kit) -> Node<Msg> {
        let now = &self.now;
        let waits = now.plain.then_some(t!("settings-wallpaper-waits"));
        let about = if now.rotating {
            t!("settings-wallpaper-pictures-rotating")
        } else {
            t!("settings-wallpaper-pictures-one")
        };
        let pictures = group(t!("settings-wallpaper-pictures-title")).icon("folder-pictures-symbolic").about(about).span(WHOLE, WHOLE).waits_for(waits);
        let card = if now.choices.is_empty() {

            let frame = drawing((150.0, 85.0), t!("settings-wallpaper-no-pictures-name"), |canvas, pen| paint_cover(canvas, pen, None, 0.35, false))
                .width(Length::Fill).height(Length::Fill);
            pictures.rows(vec![line(t!("settings-wallpaper-no-pictures")).sub(t!("settings-wallpaper-no-pictures-about")).node(kit), frame])
        } else {

            let failed = kit.said(PICTURES).filter(|(_, problem)| *problem)
                .map(|(words, _)| crate::toolkit::widget::note(words.to_string()).tone(crate::toolkit::widget::Tone::Problem));
            pictures.vis(tiles(150.0, now.choices.iter().map(|p| self.tile(p)).collect())).rows(failed.into_iter().collect())
        };
        card.node(kit)
    }
}

pub(super) fn paint_cover(canvas: &mut PixmapMut, pen: &Pen, px: Option<&Pixmap>, opacity: f32, lit: bool) {
    let (cw, ch) = (canvas.width() as f32, canvas.height() as f32);
    let r = Rect { x: 0.0, y: 0.0, w: cw, h: ch };
    let radius = 8.0 * pen.scale;
    let Some(shape) = crate::toolkit::ui::rounded(r, radius) else { return };
    let mut paint = Paint { anti_alias: true, ..Paint::default() };
    match px {
        Some(px) => {
            let (iw, ih) = (px.width() as f32, px.height() as f32);
            let k = (cw / iw).max(ch / ih);
            let (dx, dy) = ((cw - iw * k) / 2.0, (ch - ih * k) / 2.0);
            paint.shader = Pattern::new(px.as_ref(), SpreadMode::Pad, FilterQuality::Bicubic, opacity,
                                        Transform::from_row(k, 0.0, 0.0, k, dx, dy));
        }
        None => paint.set_color(pen.theme.raised.to_skia()),
    }
    canvas.fill_path(&shape, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
    if lit {
        let half = pen.scale;
        if let Some(edge) = crate::toolkit::ui::rounded(r.inset(half), radius - half) {
            let mut ink = Paint { anti_alias: true, ..Paint::default() };
            ink.set_color(pen.theme.accent.to_skia());
            canvas.stroke_path(&edge, &ink, &Stroke { width: 2.0 * pen.scale, ..Stroke::default() }, Transform::identity(), None);
        }
    }
}

fn paint_animated(canvas: &mut PixmapMut, pen: &Pen) {
    let s = pen.scale;
    let layout = pen.fonts.layout(t!("settings-wallpaper-animated"), &Style::new((11.0 * s).round()), None);
    let (pad, h) = (5.0 * s, canvas.height() as f32);
    let back = Rect { x: 4.0 * s, y: h - layout.height - 2.0 * pad - 4.0 * s, w: layout.width + 2.0 * pad, h: layout.height + 2.0 * pad };
    if let Some(shape) = crate::toolkit::ui::rounded(back, crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Badge, back.w.min(back.h), 1.0)) {
        let mut paint = Paint { anti_alias: true, ..Paint::default() };
        paint.set_color(Colour { a: 0.7, ..Colour::rgb(0.0, 0.0, 0.0) }.to_skia());
        canvas.fill_path(&shape, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
    }
    pen.fonts.draw(canvas, &layout, (back.x + pad).round(), (back.y + pad).round(), Colour::rgb(1.0, 1.0, 1.0));
}

pub(super) const HALF: u8 = 6;
pub(super) const WHOLE: u8 = 12;

pub(super) const HALF_WORDS: f32 = 300.0;
pub(super) const COLUMN_LEAST: f32 = 320.0;

pub(super) const COLUMN_GAP: f32 = 32.0;

pub(super) fn spread(lines: Vec<Node<Msg>>) -> Node<Msg> {
    let mut all = Vec::new();
    for (i, l) in lines.into_iter().enumerate() {
        if i > 0 {
            all.push(w::space().height(Length::Fill));
        }
        all.push(l);
    }
    w::column(all).gap(ROW_GAP / 2.0).height(Length::Fill)
}
