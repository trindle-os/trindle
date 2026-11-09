//! Appearance: the highlight colour, chosen from the palette or typed, and the light or dark look.

use super::{Appearance, THEME, saved_accent};
use super::looks::{look_name, look_name_lower};
use crate::common::files::set_setting;
use crate::common::{run, style, themes};
use crate::settings::ui::{group, headline, line, on, Kit, Msg, not_saved};
use crate::toolkit::colour::Colour;
use crate::toolkit::t;
use crate::toolkit::theme::Palette;
use crate::toolkit::widget::{self, button, drawing, entry, row, Length, Node, Rect};
use tiny_skia::{FillRule, Paint, PixmapMut, Transform};

impl Appearance {

    pub(super) fn set_look(&mut self, kit: &mut Kit, light: bool) {
        self.off(kit, move || themes::set_look(light).inspect(|_| crate::toolkit::theme::reread()), move |me, kit, set| {
            if kit.settle_quietly(THEME, set, not_saved()).is_none() {
                return;
            }
            me.light = light;
            kit.aloud(&t!("settings-appearance-aloud-look", look = look_name_lower(light)));
            std::thread::spawn(|| {
                style::apply_system_theme();
                run::spawn(&["trindle-ring", "--restyle"]);
            });
        });
    }

    pub(super) fn typed(&mut self, kit: &mut Kit, text: String) {
        self.hex = text.trim().chars().take(9).collect();
        if let Some(code) = hex_code(&self.hex) {
            if code != self.accent {
                self.choose(kit, &code, true);
            }
        }
    }

    fn check_hex(&mut self, kit: &mut Kit, text: &str) {
        match hex_code(text) {
            Some(code) => self.choose(kit, &code, true),
            None if !text.trim().is_empty() => {
                kit.say(ACCENT, t!("settings-appearance-hex-bad"), false, false);
            }
            None => {}
        }
    }

    fn choose(&mut self, kit: &mut Kit, hex: &str, typed: bool) {
        if kit.settle(ACCENT, set_setting("accent", hex), not_saved()).is_none() {
            return;
        }
        if !typed {
            self.hex.clear();
        }
        self.accent = hex.to_string();
        std::thread::spawn(crate::common::app_theme::write);
    }

    fn accent_name(&self) -> String {
        let accent = if self.accent.is_empty() { saved_accent() } else { self.accent.clone() };
        match highlights().into_iter().find(|(h, _)| *h == accent) {
            Some((h, n)) if h == style::DEFAULT_ACCENT => n,
            Some((_, n)) => n.to_lowercase(),
            None => accent,
        }
    }

    pub(super) fn top(&self, kit: &Kit) -> Node<Msg> {
        headline(kit, self.top_words())
    }

    pub(super) fn top_words(&self) -> String {
        let shape = t!("settings-appearance-shape-round");
        let theme = self.themes.iter().find(|t| t.id == self.theme && !t.id.is_empty());

        if let Some((name, why)) = &self.unusable {
            return t!("settings-appearance-top-unusable", theme = name, why = *why);
        }
        match theme {

            Some(t) if t.looks().len() == 2 => t!("settings-appearance-top-theme-look", theme = &t.name, look = look_name_lower(self.light), accent = self.accent_name(), shape = shape),
            Some(t) => t!("settings-appearance-top-theme", theme = &t.name, accent = self.accent_name(), shape = shape),
            None => t!("settings-appearance-top-look", look = look_name(self.light), accent = self.accent_name(), shape = shape),
        }
    }

    pub(super) fn highlight_card(&self, kit: &Kit) -> Node<Msg> {
        let all = highlights();
        let picker = row(all.iter().map(|(hex, name)| {
            let fill = Colour::parse(hex).unwrap_or(Colour::rgb(1.0, 1.0, 1.0));
            let chosen = self.accent == *hex;
            let code = hex.clone();
            button("", name.clone())
                .face(move |canvas, r, _, pen| swatch(canvas, r, fill, pen, chosen))
                .on(chosen)
                .on_press(on::<Appearance>(move |me, k| me.choose(k, &code, false)))
        }).collect()).gap(8.0).width(Length::Fill);
        let shown = hex_code(&self.hex).or_else(|| Some(self.accent.clone())).and_then(|h| Colour::parse(&h));
        let dot = drawing((34.0, 34.0), t!("settings-appearance-dot"), move |canvas, pen| {
            let r = Rect { x: 0.0, y: 0.0, w: canvas.width() as f32, h: canvas.height() as f32 };
            disc(canvas, r, 0.5, shown.unwrap_or(pen.theme.accent), pen.scale);
        });
        let typed = self.hex.clone();
        let code = entry(self.hex.clone(), t!("settings-appearance-hex-code"), |t| on::<Appearance>(move |me, k| me.typed(k, t.clone())))
            .spoken(t!("settings-appearance-hex-spoken"))
            .on_submit(on::<Appearance>(move |me, k| me.check_hex(k, &typed)))
            .width(Length::Fixed(120.0));
        let own = line(t!("settings-appearance-any-colour"))
            .sub(t!("settings-appearance-any-colour-about"))
            .key(ACCENT)
            .controls(vec![code, dot])
            .node(kit);

        group(t!("settings-appearance-colour-title")).icon("color-select-symbolic")
            .about(t!("settings-appearance-colour-about"))
            .rows(vec![line(t!("settings-appearance-highlight")).sub(t!("settings-appearance-highlight-about")).below(picker).node(kit), own])
            .span(6, 6)
            .node(kit)
    }

}

const ACCENT: &str = "appearance:accent";

pub(crate) fn highlights() -> Vec<(String, String)> {
    let mut all = vec![(style::DEFAULT_ACCENT.to_string(), t!("settings-appearance-trindle-yellow").to_string())];
    all.extend(Palette::ALL.iter().map(|p| {
        let name = p.name().replace('-', " ");
        let mut c = name.chars();
        let name = c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default();
        (p.swatch().css(), name)
    }));
    all
}

fn hex_code(text: &str) -> Option<String> {
    let digits = text.trim().trim_start_matches('#');
    ((digits.len() == 6 || digits.len() == 8) && digits.chars().all(|c| c.is_ascii_hexdigit())).then(|| format!("#{}", digits.to_lowercase()))
}

fn disc(canvas: &mut PixmapMut, r: Rect, share: f32, c: Colour, scale: f32) {
    let side = 2.0 * r.w * share;
    let whole = Rect { x: r.x + r.w / 2.0 - side / 2.0, y: r.y + r.h / 2.0 - side / 2.0, w: side, h: side };
    let Some(path) = crate::toolkit::ui::rounded(whole, crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Picture, side, scale)) else { return };
    let mut paint = Paint { anti_alias: true, ..Default::default() };
    paint.set_color(c.to_skia());
    canvas.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
}

pub(super) fn swatch(canvas: &mut PixmapMut, r: Rect, fill: Colour, pen: &widget::Pen, chosen: bool) {
    if chosen {
        disc(canvas, r, 0.82, pen.theme.panel, pen.scale);
        disc(canvas, r, 0.58, fill, pen.scale);
    } else {
        disc(canvas, r, 0.82, fill, pen.scale);
    }
}
