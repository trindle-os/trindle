//! Ring buttons whose state needs the person's eye are drawn filled: the highlight for no internet and keep awake, the sheet's green for an update waiting. The icon is dark or light, by WCAG contrast on the fill.

use crate::toolkit::colour::Colour;
use crate::toolkit::theme::{self, Part, Theme};
use crate::toolkit::widget::{Node, Pen, Rect};

use super::ctx::Msg;

const GREEN: &str = "trindle-green";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Need {
    NoInternet,
    KeepAwake,
    Update,
}

pub fn fill_for(need: Need, t: &Theme, green: Colour) -> (Colour, Colour) {
    let fill = if need == Need::Update { green } else { t.accent };
    (fill, theme::on(fill))
}

pub fn paint(canvas: &mut tiny_skia::PixmapMut, icon_at: Rect, pen: &Pen, icon: &str, need: Need, green: [Colour; 2], across: f32) {
    let (fill, ink) = fill_for(need, pen.theme, green[usize::from(pen.theme.light)]);
    let side = (across * pen.scale).round();
    let grow = (side - icon_at.w) / 2.0;
    let square = Rect { x: icon_at.x - grow, y: icon_at.y - grow, w: side, h: side };
    let corner = (theme::sheet().corner_of(Part::RingButton) * pen.scale).round();
    crate::toolkit::ui::fill(canvas, square, corner, fill);
    pen.icons.draw(canvas, icon, icon_at.x as i32, icon_at.y as i32, icon_at.w as u32, 1, ink);
}

pub fn filled(node: Node<Msg>, icon: &str, need: Option<Need>, size: f32) -> Node<Msg> {
    let Some(need) = need else { return node };
    let green = if need == Need::Update { [theme::named(GREEN, false), theme::named(GREEN, true)] } else { [Colour::rgb(0.0, 0.0, 0.0); 2] };
    let icon = icon.to_string();
    node.face(move |canvas, at, _, pen| paint(canvas, at, pen, &icon, need, green, size))
}

pub fn net_need(icon: &str) -> Option<Need> {
    matches!(icon, "network-offline-symbolic" | "network-wireless-offline-symbolic" | "network-wired-disconnected-symbolic").then_some(Need::NoInternet)
}

pub fn when(on: bool, need: Need) -> Option<Need> {
    on.then_some(need)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::text::Fonts;
    use crate::toolkit::icon::Icons;

    const NEEDS: [Need; 3] = [Need::NoInternet, Need::KeepAwake, Need::Update];
    const ACCENTS: [&str; 4] = ["#fbc711", "#ffee88", "#1a237e", "#2196f3"];

    fn green(light: bool) -> Colour {
        theme::named(GREEN, light)
    }

    #[test]
    fn each_need_is_filled_in_its_colour_and_the_icon_reads_on_it_in_every_look() {
        for light in [false, true] {
            for accent in ACCENTS {
                let t = Theme::new(light, accent);
                for need in NEEDS {
                    let (fill, ink) = fill_for(need, &t, green(light));
                    let want = if need == Need::Update { green(light) } else { t.accent };
                    assert_eq!(fill, want, "{need:?} light {light} {accent}");
                    assert!(theme::contrast(ink, fill) >= theme::WORDS, "{need:?} light {light} {accent}: {}", theme::contrast(ink, fill));
                }
            }
        }
    }

    #[test]
    fn the_square_is_painted_in_the_fill_and_nothing_else_is_filled() {
        let (fonts, icons) = (Fonts::new(), Icons::new());
        for light in [false, true] {
            let t = Theme::new(light, "#fbc711");
            let pen = Pen { theme: &t, fonts: &fonts, icons: &icons, scale: 1.0 };
            for need in NEEDS {
                let mut pix = tiny_skia::Pixmap::new(48, 48).unwrap();
                paint(&mut pix.as_mut(), Rect { x: 12.0, y: 12.0, w: 24.0, h: 24.0 }, &pen, "no-such-icon", need, [green(false), green(true)], 36.0);
                let (fill, _) = fill_for(need, &t, green(light));
                let p = pix.pixel(7, 24).unwrap();
                let got = Colour::rgb(p.red() as f32 / 255.0, p.green() as f32 / 255.0, p.blue() as f32 / 255.0);
                assert!((got.r - fill.r).abs() < 0.01 && (got.g - fill.g).abs() < 0.01 && (got.b - fill.b).abs() < 0.01, "{need:?} light {light}");
                assert_eq!(pix.pixel(2, 2).unwrap().alpha(), 0, "outside the square stays clear");
            }
        }
    }

    #[test]
    fn normal_states_are_not_filled_and_only_offline_icons_count() {
        assert_eq!(net_need("network-offline-symbolic"), Some(Need::NoInternet));
        assert_eq!(net_need("network-wireless-offline-symbolic"), Some(Need::NoInternet));
        assert_eq!(net_need("network-wired-disconnected-symbolic"), Some(Need::NoInternet));
        for icon in ["network-wired-symbolic", "network-wireless-signal-good-symbolic", "network-vpn-symbolic"] {
            assert_eq!(net_need(icon), None, "{icon}");
        }
        assert_eq!(when(false, Need::KeepAwake), None);
        assert_eq!(when(true, Need::KeepAwake), Some(Need::KeepAwake));
    }
}
