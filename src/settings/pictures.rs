//! The main picture of what each Settings page is about (one main picture per screen, where colour lives), drawn in a box of its own units in the greys, the highlight and the palette.

use crate::toolkit::t;
use crate::toolkit::theme::{alpha, Palette};
use crate::toolkit::tiles::{sketch, Sketch};
use crate::toolkit::widget::Node;

fn screen(s: &mut Sketch, r: (f32, f32, f32, f32)) {
    let page = s.page();
    s.rect(r, 10.0, page);
}

fn pointer(s: &mut Sketch, at: (f32, f32)) {
    let (x, y) = at;
    let c = s.words();
    s.shape(&[(x, y), (x, y + 15.0), (x + 4.0, y + 11.0), (x + 7.0, y + 17.0), (x + 10.0, y + 16.0), (x + 7.0, y + 10.0), (x + 12.0, y + 10.0)], c);
}

pub fn ring<M>() -> Node<M> {
    let tip = t!("settings-common-ring-picture").to_string();
    sketch(tip, (360.0, 200.0), crate::toolkit::theme::bento().picture_least, move |s| {
        screen(s, (8.0, 8.0, 344.0, 184.0));
        let words = s.words();
        let centre = (180.0, 100.0);
        let track = alpha(Palette::Blue.colour(), 0.35);
        s.ring(centre, 46.0, 16.0, track);
        let hues = [Palette::Blue, Palette::Orange, Palette::Blue, Palette::Orange, Palette::Blue, Palette::Orange];
        for (i, h) in hues.iter().enumerate() {
            let a = std::f32::consts::TAU * i as f32 / 6.0 - std::f32::consts::FRAC_PI_2;
            s.circle((centre.0 + 46.0 * a.cos(), centre.1 + 46.0 * a.sin()), 7.0, h.colour());
        }
        s.text(t!("settings-common-ring-name"), centre, 12.0, words);
        pointer(s, (330.0, 30.0));
    })
}
