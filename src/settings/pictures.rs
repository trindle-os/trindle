//! The main picture of what each Settings page is about (one main picture
//! per screen, where colour lives), drawn in a box of its own units in the
//! greys, the highlight and the palette.

use crate::toolkit::t;
use crate::toolkit::theme::{alpha, Palette};
use crate::toolkit::tiles::sketch;
use crate::toolkit::widget::Node;

pub fn ring<M>() -> Node<M> {
    let tip = t!("settings-common-ring-name").to_string();
    sketch(tip, (200.0, 200.0), crate::toolkit::theme::bento().picture_least, move |s| {
        let centre = (100.0, 100.0);
        let track = alpha(Palette::Blue.colour(), 0.35);
        s.ring(centre, 70.0, 22.0, track);

        let hues = [Palette::Blue, Palette::Orange, Palette::Blue, Palette::Orange, Palette::Blue, Palette::Orange];
        for (i, h) in hues.iter().enumerate() {
            let a = std::f32::consts::TAU * i as f32 / 6.0 - std::f32::consts::FRAC_PI_2;
            s.circle((centre.0 + 70.0 * a.cos(), centre.1 + 70.0 * a.sin()), 10.0, h.colour());
        }
    })
}
