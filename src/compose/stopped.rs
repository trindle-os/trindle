//! The pill shown when a part the desktop relies on has stopped.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use smithay::{
    backend::{
        renderer::{ImportAll, ImportMem, Renderer},
    },
    output::Output,
    utils::{Logical, Point, Rectangle, Size},
};

use crate::indicators::{kept, pixel_size, pixels, Overlay};
use crate::render::CustomRenderElements;
use crate::shell::ssd::{self, Paint};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    Ring,
    Shot,
}

impl Stopped {

    pub fn words(self) -> &'static str {
        match self {
            Stopped::Ring => trindle_toolkit::t!("compose-stopped-ring"),
            Stopped::Shot => trindle_toolkit::t!("compose-stopped-shot"),
        }
    }

    pub fn try_again() -> &'static str {
        trindle_toolkit::t!("compose-stopped-try-again")
    }
}

#[derive(Default)]
struct List(Vec<Stopped>);

impl List {
    fn say(&mut self, what: Stopped) {
        self.0.retain(|s| *s != what);
        self.0.push(what);
    }

    fn clear(&mut self, what: Stopped) {
        self.0.retain(|s| *s != what);
    }

    fn newest(&self) -> Option<Stopped> {
        self.0.last().copied()
    }
}

static SAID: Mutex<List> = Mutex::new(List(Vec::new()));

pub fn say(what: Stopped) {
    if let Ok(mut said) = SAID.lock() {
        said.say(what);
    }
}

pub fn clear(what: Stopped) {
    if let Ok(mut said) = SAID.lock() {
        said.clear(what);
    }
}

pub fn now() -> Option<Stopped> {
    SAID.lock().ok().and_then(|s| s.newest())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metrics {

    pub pad: i32,

    pub icon: i32,

    pub gap: i32,

    pub button: i32,
    pub height: i32,

    pub again_icon: i32,

    pub tint: f32,
    pub tint_on: f32,
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics { pad: 16, icon: 22, gap: 12, button: 34, height: 52, again_icon: 18, tint: 0.14, tint_on: 0.30 }
    }
}

impl Metrics {

    pub fn now() -> Metrics {
        let layers = crate::common::sheet::layers();
        let light = crate::common::sheet::asked_light();
        let d = Metrics::default();
        let n = |name: &str, was: f32, least: f32, most: f32| layers.number(name, light, least, most).unwrap_or(was);
        let height = n("trindle-stopped-height", d.height as f32, 40.0, 72.0).round() as i32;
        let button = (n("trindle-stopped-button", d.button as f32, 24.0, 44.0).round() as i32).min(height);
        Metrics {
            pad: n("trindle-stopped-padding", d.pad as f32, 8.0, 32.0).round() as i32,
            icon: (n("trindle-stopped-icon", d.icon as f32, 16.0, 32.0).round() as i32).min(height),
            gap: n("trindle-stopped-gap", d.gap as f32, 6.0, 24.0).round() as i32,
            button,
            height,
            again_icon: (n("trindle-stopped-again-icon", d.again_icon as f32, 12.0, 28.0).round() as i32).min(button),
            tint: n("trindle-stopped-tint", d.tint, 0.0, 0.5),
            tint_on: n("trindle-stopped-tint-on", d.tint_on, 0.0, 0.6),
        }
    }
}

const ALERT: &str = "dialog-warning-symbolic";
const AGAIN: &str = "view-refresh-symbolic";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Shape {
    pub w: i32,
    pub h: i32,

    pub text: i32,

    pub button: (f32, f32, f32),

    pub look: Metrics,
}

pub fn shape(text: f32) -> Shape {
    shape_with(text, Metrics::now())
}

fn shape_with(text: f32, m: Metrics) -> Shape {
    let text_x = m.pad + m.icon + m.gap;
    let after = text_x + text.ceil() as i32 + m.gap;
    let side = (m.height - m.button) / 2;
    Shape {
        w: after + m.button + side,
        h: m.height,
        text: text_x,
        button: ((after + m.button / 2) as f32, m.height as f32 / 2.0, m.button as f32 / 2.0),
        look: m,
    }
}

fn width_of(words: &str) -> f32 {
    ssd::face(400.0).map_or(words.chars().count() as f32 * 7.0, |font| {
        let size = ab_glyph::PxScale::from(ssd::FONT_PX);
        ssd::chars_width(&font, size, &words.chars().collect::<Vec<_>>())
    })
}

pub fn pill_on(screen: Size<i32, Logical>, shape: Shape) -> Rectangle<i32, Logical> {
    Rectangle::new(((screen.w - shape.w) / 2, (screen.h - shape.h) / 2).into(), (shape.w, shape.h).into())
}

pub fn on_button(place: Rectangle<i32, Logical>, at: Point<f64, Logical>) -> bool {
    on_button_of(now(), place, at)
}

fn on_button_of(shown: Option<Stopped>, place: Rectangle<i32, Logical>, at: Point<f64, Logical>) -> bool {
    let Some(stopped) = shown else { return false };
    let shape = shape(width_of(stopped.words()));
    let pill = pill_on(place.size, shape);
    let (cx, cy, r) = shape.button;
    let (x, y) = (at.x - place.loc.x as f64 - pill.loc.x as f64, at.y - place.loc.y as f64 - pill.loc.y as f64);

    let reach = r as f64 + 4.0;
    place.to_f64().contains(at) && (x - cx as f64).powi(2) + (y - cy as f64).powi(2) <= reach * reach
}

pub(crate) fn bases() -> Vec<PathBuf> {
    let mut bases = vec![crate::files::data_home().join("icons")];
    let sys = std::env::var("XDG_DATA_DIRS").ok().filter(|d| !d.is_empty()).unwrap_or_else(|| "/usr/local/share:/usr/share".into());
    bases.extend(sys.split(':').filter(|s| !s.is_empty()).map(|s| Path::new(s).join("icons")));
    bases
}

pub(crate) fn chosen() -> Option<String> {
    let ini = std::fs::read_to_string(crate::files::config_home().join("gtk-4.0").join("settings.ini")).ok()?;
    ini.lines()
        .find_map(|l| l.trim().strip_prefix("gtk-icon-theme-name").map(|v| v.trim_start().trim_start_matches('=').trim().to_string()))
        .filter(|v| !v.is_empty())
}

pub(crate) fn icon_in(bases: &[PathBuf], sets: &[String], name: &str, px: u32) -> Option<tiny_skia::Pixmap> {
    use std::io::Read as _;
    use std::os::unix::fs::OpenOptionsExt as _;
    for base in bases {
        for set in sets {
            for kind in ["status", "actions"] {
                let file = base.join(set).join("symbolic").join(kind).join(format!("{name}.svg"));
                let Ok(opened) = std::fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC).open(&file) else { continue };
                let Ok(meta) = opened.metadata() else { continue };
                if !meta.is_file() || meta.len() > 64 * 1024 {
                    continue;
                }
                let mut data = Vec::new();
                if opened.take(64 * 1024).read_to_end(&mut data).is_err() {
                    continue;
                }
                let Some(tree) = crate::picture::svg_tree(&data) else { continue };
                let mut out = tiny_skia::Pixmap::new(px.max(1), px.max(1))?;
                let size = tree.size();
                let k = px as f32 / size.width().max(size.height());
                resvg::render(&tree, tiny_skia::Transform::from_scale(k, k), &mut out.as_mut());
                return Some(out);
            }
        }
    }
    None
}

pub(crate) fn sets() -> Vec<String> {
    chosen().into_iter().chain(["Trindle-Circle".to_string()]).collect()
}

pub(crate) fn lay_icon(px: &mut [Paint], w: usize, icon: &tiny_skia::Pixmap, (x, y): (i32, i32), ink: [f32; 4]) {
    for iy in 0..icon.height() as i32 {
        for ix in 0..icon.width() as i32 {
            let (tx, ty) = (x + ix, y + iy);
            if tx < 0 || ty < 0 || tx as usize >= w {
                continue;
            }
            let at = ty as usize * w + tx as usize;
            let Some(pixel) = icon.pixel(ix as u32, iy as u32) else { continue };
            if at < px.len() {
                px[at] = Paint::of(ink, pixel.alpha() as f32 / 255.0).over(px[at]);
            }
        }
    }
}

fn paint(stopped: Stopped, shape: Shape, scale: f64, on: bool) -> (Vec<u8>, i32, i32) {
    paint_with(stopped, shape, scale, on, &bases(), &sets())
}

fn paint_with(stopped: Stopped, shape: Shape, scale: f64, on: bool, bases: &[PathBuf], sets: &[String]) -> (Vec<u8>, i32, i32) {
    let (theme, radius) = ssd::theme();
    let c = theme.active;
    let (w, h) = pixel_size((shape.w, shape.h).into(), scale);
    let s = scale as f32;
    let (pw, ph) = (w as usize, h as usize);
    let mut px = vec![Paint::CLEAR; pw * ph];
    let r = (radius as f32 * s).max(h as f32 / 2.0).min(h as f32 / 2.0);
    let edge = s.max(1.0);
    for y in 0..ph {
        for x in 0..pw {
            let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
            let outer = 0.5 - crate::indicators::outside_round_box(fx, fy, w as f32, h as f32, r);
            let inner = 0.5 - crate::indicators::outside_round_box(fx - edge, fy - edge, w as f32 - 2.0 * edge, h as f32 - 2.0 * edge, (r - edge).max(0.0));
            px[y * pw + x] = Paint::of(c.background, inner).over(Paint::of(c.border, outer));
        }
    }

    let (bx, by, br) = (shape.button.0 * s, shape.button.1 * s, shape.button.2 * s);
    let tint = if on { shape.look.tint_on } else { shape.look.tint };
    for y in 0..ph {
        for x in 0..pw {
            let d = ((x as f32 + 0.5 - bx).powi(2) + (y as f32 + 0.5 - by).powi(2)).sqrt();
            let cover = (0.5 - (d - br)).clamp(0.0, 1.0);
            if cover > 0.0 {
                px[y * pw + x] = Paint::of([c.text[0], c.text[1], c.text[2], tint], cover).over(px[y * pw + x]);
            }
        }
    }
    let side = (shape.look.icon as f32 * s).round() as u32;
    if let Some(alert) = icon_in(bases, sets, ALERT, side) {
        lay_icon(&mut px, pw, &alert, ((shape.look.pad as f32 * s).round() as i32, ((h as u32 - side) / 2) as i32), c.text);
    }
    let again = (shape.look.again_icon as f32 * s).round() as u32;
    if let Some(sign) = icon_in(bases, sets, AGAIN, again) {
        lay_icon(&mut px, pw, &sign, ((bx - again as f32 / 2.0).round() as i32, (by - again as f32 / 2.0).round() as i32), c.text);
    }
    if let Some(font) = ssd::face(400.0) {
        let size = ab_glyph::PxScale::from(ssd::FONT_PX * s);
        let text: Vec<char> = stopped.words().chars().collect();
        let baseline = (h as f32 / 2.0 + 0.36 * size.y).round();
        ssd::draw_chars(&mut px, pw, ph, &font, size, &text, c.text, shape.text as f32 * s, baseline);
    }
    (ssd::to_bytes(&px), w, h)
}

type Key = (Stopped, u64, bool, [[f32; 4]; 3], i32, Metrics);

thread_local! {
    static PAINTED: RefCell<Option<(Key, Overlay)>> = const { RefCell::new(None) };
}

pub fn elements<R>(renderer: &mut R, output: &Output, place: Option<Rectangle<i32, Logical>>) -> Vec<CustomRenderElements<R>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Send + 'static,
{
    let Some(stopped) = now() else { return Vec::new() };
    let Some(mode) = output.current_mode() else { return Vec::new() };
    let screen = output.current_transform().transform_size(mode.size);
    let scale = output.current_scale().fractional_scale();
    let shape = shape(width_of(stopped.words()));
    let on = place.zip(crate::indicators::pointer()).is_some_and(|(p, at)| on_button(p, at));
    let (theme, radius) = ssd::theme();
    let c = theme.active;
    let key: Key = (stopped, scale.to_bits(), on, [c.background, c.border, c.text], radius, shape.look);
    let pill = PAINTED.with(|p| kept(p, key, |_| Overlay::new(&paint(stopped, shape, scale, on).0, (shape.w, shape.h).into(), scale)));
    let (w, h) = (pixels(shape.w, scale) as f64, pixels(shape.h, scale) as f64);
    let at = (((screen.w as f64 - w) / 2.0).round().max(0.0), ((screen.h as f64 - h) / 2.0).round().max(0.0));
    let mut out: Vec<_> = pill.element(renderer, at).into_iter().collect();

    if on {
        let label = crate::indicators::label(vec![Stopped::try_again().to_string()], scale);
        let (cx, cy, r) = shape.button;
        let x = at.0 + (cx as f64 * scale) - pixels(label.logical.w, scale) as f64 / 2.0;
        let y = at.1 + ((cy + r + 8.0) as f64 * scale);
        out.extend(label.element(renderer, (x.round().max(0.0), y.round())));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_compositor_follows_the_language_and_falls_back_word_by_word() {
        use trindle_toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let french = "compose-stopped-ring = L'anneau s'est arrete.\ncompose-level-muted = Muet\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            assert_eq!(Stopped::Ring.words(), "L'anneau s'est arrete.");
            assert_eq!(crate::level::shown(crate::level::Kind::Speakers, crate::levels::Sound { share: 0.5, muted: true }).words, Some("Muet"));
            assert_eq!(Stopped::Shot.words(), "Screenshot stopped and was closed.");
            assert_eq!(Stopped::try_again(), "Try again");
        });
    }

    #[test]
    fn the_button_is_round_and_inside_the_pill() {
        for words in [Stopped::Ring.words(), Stopped::Shot.words()] {
            let s = shape(width_of(words));
            let (cx, cy, r) = s.button;
            assert!(cx - r >= (s.text as f32) && cx + r <= s.w as f32 && cy - r >= 0.0 && cy + r <= s.h as f32, "{s:?}");
            assert!(s.w > s.text + 40 && s.h == Metrics::default().height);

            assert_eq!(cy - r, s.h as f32 - (cy + r));
        }
    }

    #[test]
    fn the_words_are_as_given() {
        assert_eq!(Stopped::Ring.words(), "The ring stopped. Press Super to restart it.");
        assert_eq!(Stopped::Shot.words(), "Screenshot stopped and was closed.");
        assert_eq!(Stopped::try_again(), "Try again");
        for s in [Stopped::Ring, Stopped::Shot] {
            assert!(!s.words().contains('\u{2014}') && s.words().len() < 70);
        }
    }

    #[test]
    fn what_is_said_stays_until_it_runs_again() {
        let mut said = List::default();
        assert_eq!(said.newest(), None);
        said.say(Stopped::Ring);
        said.say(Stopped::Ring);
        assert_eq!(said.newest(), Some(Stopped::Ring));
        said.say(Stopped::Shot);
        assert_eq!(said.newest(), Some(Stopped::Shot));
        said.clear(Stopped::Shot);
        assert_eq!(said.newest(), Some(Stopped::Ring));
        said.clear(Stopped::Ring);
        assert_eq!(said.newest(), None);
    }

    #[test]
    fn the_pill_is_painted_with_the_icons_from_the_set() {
        let dir = std::env::temp_dir().join(format!("trindle-stopped-{}", std::process::id()));
        let place = dir.join("Test-Set/symbolic/status");
        std::fs::create_dir_all(&place).unwrap();
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><rect x="2" y="2" width="12" height="12" fill="#bebebe"/></svg>"##;
        std::fs::write(place.join("dialog-warning-symbolic.svg"), svg).unwrap();
        let got = icon_in(std::slice::from_ref(&dir), &["Test-Set".to_string()], ALERT, 22).expect("the icon is read");
        assert!(got.pixels().iter().any(|p| p.alpha() == 255) && got.pixels().iter().any(|p| p.alpha() == 0));

        assert!(icon_in(std::slice::from_ref(&dir), &["No-Set".to_string()], ALERT, 22).is_none());
        assert!(icon_in(std::slice::from_ref(&dir), &["Test-Set".to_string()], "no-such-icon", 22).is_none());
        std::os::unix::fs::symlink(place.join("dialog-warning-symbolic.svg"), place.join("linked-symbolic.svg")).unwrap();
        assert!(icon_in(std::slice::from_ref(&dir), &["Test-Set".to_string()], "linked-symbolic", 22).is_none());
        std::fs::remove_dir_all(&dir).unwrap();
        let s = shape(width_of(Stopped::Ring.words()));
        let (rest, w, h) = paint(Stopped::Ring, s, 1.0, false);
        let (on, ..) = paint(Stopped::Ring, s, 1.0, true);
        assert_eq!((w, h), (s.w, s.h));
        assert_eq!(rest.len(), (w * h * 4) as usize);
        assert!(rest.chunks(4).any(|p| p[3] == 255) && rest.chunks(4).any(|p| p[3] == 0));
        assert_ne!(rest, on);

        let (cx, _, r) = s.button;
        let first_column = ((cx - r - 1.0).floor() as i32).max(0);
        for (i, (a, b)) in rest.chunks(4).zip(on.chunks(4)).enumerate() {
            if a != b {
                assert!((i as i32 % w) >= first_column, "pixel {i} changed outside the button");
            }
        }
    }

    #[test]
    fn only_the_round_button_takes_a_press() {
        let place = Rectangle::new((1920, 0).into(), (1920, 1080).into());
        let s = shape(width_of(Stopped::Ring.words()));
        let pill = pill_on(place.size, s);
        let on = |shown: Option<Stopped>, x: f64, y: f64| on_button_of(shown, place, (x, y).into());
        let (cx, cy, _) = s.button;
        let (mx, my) = (1920.0 + pill.loc.x as f64 + cx as f64, pill.loc.y as f64 + cy as f64);
        assert!(!on(None, mx, my), "a press counted with nothing said");
        let ring = Some(Stopped::Ring);
        assert!(on(ring, mx, my));
        assert!(!on(ring, mx - 80.0, my), "the words took the press");
        assert!(!on(ring, mx, my + 60.0));

        assert!(!on(ring, pill.loc.x as f64 + cx as f64, pill.loc.y as f64 + cy as f64));
    }

    #[test]
    #[ignore]
    fn shoot_the_pill() {
        let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-stopped".into()));
        std::fs::create_dir_all(&out).unwrap();
        let icons = PathBuf::from(std::env::var("TRINDLE_ICONS").unwrap_or_else(|_| "../trindle-icons".into()));
        let sets = ["Trindle-Circle".to_string()];
        for (stopped, name) in [(Stopped::Ring, "ring"), (Stopped::Shot, "screenshot")] {
            let shape = shape(width_of(stopped.words()));
            for (on, when) in [(false, "rest"), (true, "pointer-on-button")] {
                for (scale, size) in [(1.0f64, "1x"), (2.0, "2x")] {
                    let (bytes, w, h) = paint_with(stopped, shape, scale, on, std::slice::from_ref(&icons), &sets);

                    let (sw, sh) = (w + 160, h + 120);
                    let mut screen = tiny_skia::Pixmap::new(sw as u32, sh as u32).unwrap();
                    screen.fill(tiny_skia::Color::from_rgba8(0x3a, 0x3d, 0x44, 0xff));
                    let mut pill = tiny_skia::Pixmap::new(w as u32, h as u32).unwrap();
                    for (d, b) in pill.data_mut().chunks_mut(4).zip(bytes.chunks(4)) {
                        d.copy_from_slice(&[b[2], b[1], b[0], b[3]]);
                    }
                    screen.draw_pixmap(80, 60, pill.as_ref(), &tiny_skia::PixmapPaint::default(), tiny_skia::Transform::identity(), None);
                    screen.save_png(out.join(format!("pill-{name}-{when}-{size}.png"))).unwrap();
                }
            }
        }
    }

    #[test]
    fn the_style_sheet_gives_the_pill_its_sizes() {
        use crate::common::sheet::{Layers, CSS};
        let d = Metrics::default();
        let l = Layers::new(CSS, "");
        let n = |name: &str| l.number(name, false, -1000.0, 1000.0).unwrap();
        assert_eq!(n("trindle-stopped-padding"), d.pad as f32);
        assert_eq!(n("trindle-stopped-icon"), d.icon as f32);
        assert_eq!(n("trindle-stopped-gap"), d.gap as f32);
        assert_eq!(n("trindle-stopped-button"), d.button as f32);
        assert_eq!(n("trindle-stopped-height"), d.height as f32);
        assert_eq!(n("trindle-stopped-again-icon"), d.again_icon as f32);
        assert_eq!(n("trindle-stopped-tint"), d.tint);
        assert_eq!(n("trindle-stopped-tint-on"), d.tint_on);
        assert_eq!(Metrics::now(), d);
        let s = shape_with(100.0, d);
        assert_eq!((s.h, s.text, s.look), (52, 50, d));
    }

    #[test]
    fn the_stopped_pill_is_painted_its_declared_size_at_every_scale() {
        for scale in crate::indicators::every_scale() {
            for text in [95.0, 105.0, 120.5, 151.0, 200.0] {
                let shape = shape_with(text, Metrics::default());
                let (bytes, w, h) = paint_with(Stopped::Ring, shape, scale, false, &[], &[]);
                assert_eq!((w, h), pixel_size((shape.w, shape.h).into(), scale), "x{scale} t{text}");
                assert_eq!(bytes.len(), (w * h * 4) as usize, "x{scale} t{text}");
            }
        }
    }
}

