//! The level pill shown after a volume, microphone or brightness key.

use std::cell::RefCell;
use std::process::Child;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use smithay::{
    backend::{
        renderer::{ImportAll, ImportMem, Renderer},
    },
    output::Output,
};

use crate::indicators::{kept, pixel_size, pixels, Overlay};
use crate::levels::{self, Sound};
use crate::render::CustomRenderElements;
use crate::shell::ssd::{self, Paint};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Speakers,
    Microphone,
    Brightness,
}

pub fn kind_of(command: &str) -> Option<Kind> {
    let words: Vec<&str> = command.split_whitespace().collect();
    match words.as_slice() {
        ["wpctl", "set-volume", rest @ ..] | ["wpctl", "set-mute", rest @ ..] => match rest.iter().find(|w| w.starts_with('@')) {
            Some(&levels::SPEAKERS) => Some(Kind::Speakers),
            Some(&levels::MICROPHONE) => Some(Kind::Microphone),
            _ => None,
        },
        ["trindle-brightness", "up" | "down"] => Some(Kind::Brightness),
        _ => None,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Shown {
    pub icon: &'static str,
    pub share: f32,
    pub words: Option<&'static str>,
}

pub fn shown(kind: Kind, sound: Sound) -> Shown {
    let share = if sound.muted { 0.0 } else { levels::bar_share(sound.share) };
    match kind {
        Kind::Speakers => Shown {
            icon: match share {
                s if sound.muted || s <= 0.0 => "audio-volume-muted-symbolic",
                s if s < 1.0 / 3.0 => "audio-volume-low-symbolic",
                s if s < 2.0 / 3.0 => "audio-volume-medium-symbolic",
                _ => "audio-volume-high-symbolic",
            },
            share,
            words: sound.muted.then_some(trindle_toolkit::t!("compose-level-muted")),
        },
        Kind::Microphone => Shown {
            icon: if sound.muted { "microphone-sensitivity-muted-symbolic" } else { "audio-input-microphone-symbolic" },
            share,
            words: sound.muted.then_some(trindle_toolkit::t!("compose-level-mic-off")),
        },
        Kind::Brightness => Shown { icon: BRIGHTNESS, share, words: None },
    }
}

fn read(kind: Kind) -> Option<Sound> {
    match kind {
        Kind::Speakers => levels::volume(levels::SPEAKERS),
        Kind::Microphone => levels::volume(levels::MICROPHONE),
        Kind::Brightness => levels::brightness().map(|share| Sound { share, muted: false }),
    }
}

static NOW: Mutex<Option<(Shown, Instant)>> = Mutex::new(None);

fn put(what: Shown) {
    if let Ok(mut now) = NOW.lock() {
        *now = Some((what, Instant::now()));
    }
}

pub fn show_after(mut child: Child, kind: Kind) {
    std::thread::spawn(move || {
        let _ = child.wait();
        if let Some(sound) = read(kind) {
            put(shown(kind, sound));
        }
    });
}

fn still(since: Instant, now: Instant, stays: Duration) -> bool {
    now.saturating_duration_since(since) < stays
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Metrics {
    pub pad: i32,
    pub icon: i32,
    pub gap: i32,
    pub bar_w: i32,
    pub bar_h: i32,
    pub height: i32,
    pub bottom: i32,
    pub stays: Duration,
    pub track: f32,
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics { pad: 16, icon: 22, gap: 12, bar_w: 160, bar_h: 6, height: 48, bottom: 64, stays: Duration::from_millis(1500), track: 0.30 }
    }
}

impl Metrics {
    pub fn now() -> Metrics {
        let layers = crate::common::sheet::layers();
        let light = crate::common::sheet::asked_light();
        let d = Metrics::default();
        let n = |name: &str, was: f32, least: f32, most: f32| layers.number(name, light, least, most).unwrap_or(was);
        let height = n("trindle-level-height", d.height as f32, 36.0, 72.0).round() as i32;
        Metrics {
            pad: n("trindle-level-padding", d.pad as f32, 8.0, 32.0).round() as i32,
            icon: (n("trindle-level-icon", d.icon as f32, 16.0, 32.0).round() as i32).min(height),
            gap: n("trindle-level-gap", d.gap as f32, 6.0, 24.0).round() as i32,
            bar_w: n("trindle-level-bar-width", d.bar_w as f32, 80.0, 320.0).round() as i32,
            bar_h: (n("trindle-level-bar-height", d.bar_h as f32, 3.0, 12.0).round() as i32).min(height),
            height,
            bottom: n("trindle-level-bottom", d.bottom as f32, 0.0, 200.0).round() as i32,
            stays: Duration::from_millis(n("trindle-level-stays", d.stays.as_millis() as f32, 500.0, 5000.0) as u64),
            track: n("trindle-level-track", d.track, 0.0, 0.5),
        }
    }
}

fn width_of(words: &str) -> f32 {
    ssd::face(400.0).map_or(words.chars().count() as f32 * 7.0, |font| {
        let size = ab_glyph::PxScale::from(ssd::FONT_PX);
        ssd::chars_width(&font, size, &words.chars().collect::<Vec<_>>())
    })
}

fn width(m: &Metrics, text: f32) -> i32 {
    let words = if text > 0.0 { m.gap + text.ceil() as i32 } else { 0 };
    m.pad + m.icon + m.gap + m.bar_w + words + m.pad
}

pub fn filled(m: &Metrics, share: f32) -> i32 {
    (levels::bar_share(share) * m.bar_w as f32).round() as i32
}

type Key = (Shown, u64, [[f32; 4]; 3], i32, Metrics, i32);

thread_local! {
    static PAINTED: RefCell<Option<(Key, Overlay)>> = const { RefCell::new(None) };
}

fn lay_sun(px: &mut [Paint], pw: usize, at: (i32, i32), side: f32, ink: [f32; 4]) {
    let (cx, cy) = (at.0 as f32 + side / 2.0, at.1 as f32 + side / 2.0);
    let (disc, near, far, half) = (side * 0.2, side * 0.33, side * 0.48, side * 0.075);
    for y in at.1.max(0)..(at.1 + side.ceil() as i32) {
        for x in at.0.max(0)..(at.0 + side.ceil() as i32) {
            let (dx, dy) = (x as f32 + 0.5 - cx, y as f32 + 0.5 - cy);
            let d = (dx * dx + dy * dy).sqrt();
            let mut cover = (0.5 - (d - disc)).clamp(0.0, 1.0);
            for k in 0..8 {
                let a = k as f32 * std::f32::consts::FRAC_PI_4;
                let (along, across) = (dx * a.cos() + dy * a.sin(), (-dx * a.sin() + dy * a.cos()).abs());
                let from_ends = (near - along).max(along - far).max(0.0);
                let gap = (from_ends * from_ends + (across - half).max(0.0).powi(2)).sqrt();
                cover = cover.max((0.5 - gap).clamp(0.0, 1.0));
            }
            let i = y as usize * pw + x as usize;
            if cover > 0.0 && x >= 0 && i < px.len() {
                px[i] = Paint::of(ink, cover).over(px[i]);
            }
        }
    }
}

const BRIGHTNESS: &str = "display-brightness-symbolic";

fn paint(what: &Shown, m: &Metrics, w_logical: i32, scale: f64) -> (Vec<u8>, i32, i32) {
    let (theme, radius) = ssd::theme();
    let c = theme.active;
    let (w, h) = pixel_size((w_logical, m.height).into(), scale);
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
    let side = (m.icon as f32 * s).round() as u32;
    let at = ((m.pad as f32 * s).round() as i32, ((h as u32).saturating_sub(side) / 2) as i32);
    match crate::stopped::icon_in(&crate::stopped::bases(), &crate::stopped::sets(), what.icon, side) {
        Some(sign) => crate::stopped::lay_icon(&mut px, pw, &sign, at, c.text),

        None if what.icon == BRIGHTNESS => lay_sun(&mut px, pw, at, side as f32, c.text),
        None => {}
    }

    let bx = (m.pad + m.icon + m.gap) as f32 * s;
    let (bw, bh) = (m.bar_w as f32 * s, m.bar_h as f32 * s);
    let by = (h as f32 - bh) / 2.0;
    let fill = filled(m, what.share) as f32 * s;
    let round = bh / 2.0;
    for y in 0..ph {
        for x in 0..pw {
            let (fx, fy) = (x as f32 + 0.5 - bx, y as f32 + 0.5 - by);
            if fx < -1.0 || fx > bw + 1.0 || fy < -1.0 || fy > bh + 1.0 {
                continue;
            }
            let track = (0.5 - crate::indicators::outside_round_box(fx, fy, bw, bh, round)).clamp(0.0, 1.0);
            if track > 0.0 {
                px[y * pw + x] = Paint::of([c.text[0], c.text[1], c.text[2], m.track], track).over(px[y * pw + x]);
            }
            if fill > 0.0 {
                let part = (0.5 - crate::indicators::outside_round_box(fx, fy, fill.max(bh), bh, round)).clamp(0.0, 1.0);
                let part = if fx > fill { 0.0 } else { part };
                if part > 0.0 {
                    px[y * pw + x] = Paint::of(c.text, part).over(px[y * pw + x]);
                }
            }
        }
    }
    if let (Some(words), Some(font)) = (what.words, ssd::face(400.0)) {
        let size = ab_glyph::PxScale::from(ssd::FONT_PX * s);
        let text: Vec<char> = words.chars().collect();
        let baseline = (h as f32 / 2.0 + 0.36 * size.y).round();
        ssd::draw_chars(&mut px, pw, ph, &font, size, &text, c.text, bx + bw + m.gap as f32 * s, baseline);
    }
    (ssd::to_bytes(&px), w, h)
}

pub fn elements<R>(renderer: &mut R, output: &Output) -> Vec<CustomRenderElements<R>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Send + 'static,
{
    let m = Metrics::now();
    let what = match NOW.lock() {
        Ok(mut now) => match now.as_ref() {
            Some((what, since)) if still(*since, Instant::now(), m.stays) => what.clone(),
            Some(_) => {
                *now = None;
                return Vec::new();
            }
            None => return Vec::new(),
        },
        Err(_) => return Vec::new(),
    };
    let Some(mode) = output.current_mode() else { return Vec::new() };
    let screen = output.current_transform().transform_size(mode.size);
    let scale = output.current_scale().fractional_scale();
    let w_logical = width(&m, what.words.map_or(0.0, width_of));
    let (theme, radius) = ssd::theme();
    let c = theme.active;
    let key: Key = (what, scale.to_bits(), [c.background, c.border, c.text], radius, m, w_logical);
    let pill = PAINTED.with(|p| kept(p, key, |k| Overlay::new(&paint(&k.0, &m, w_logical, scale).0, (w_logical, m.height).into(), scale)));
    let (w, h) = (pixels(w_logical, scale) as f64, pixels(m.height, scale) as f64);
    let at = (((screen.w as f64 - w) / 2.0).round().max(0.0), (screen.h as f64 - h - m.bottom as f64 * scale).round().max(0.0));
    pill.element(renderer, at).into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEYS: [(&str, Kind); 6] = [
        ("XF86AudioRaiseVolume", Kind::Speakers),
        ("XF86AudioLowerVolume", Kind::Speakers),
        ("XF86AudioMute", Kind::Speakers),
        ("XF86AudioMicMute", Kind::Microphone),
        ("XF86MonBrightnessUp", Kind::Brightness),
        ("XF86MonBrightnessDown", Kind::Brightness),
    ];

    fn bound(key: &str) -> String {
        let file = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/config/desktop/shortcuts.xml")).unwrap();
        let line = file.lines().find(|l| l.contains(&format!("key=\"{key}\""))).unwrap_or_else(|| panic!("{key} is not bound"));
        let from = line.find("command=\"").unwrap() + "command=\"".len();
        line[from..].split('"').next().unwrap().to_string()
    }

    #[test]
    fn each_of_the_six_keys_shows_its_display() {
        for (key, kind) in KEYS {
            assert_eq!(kind_of(&bound(key)), Some(kind), "{key}");
        }
    }

    #[test]
    fn other_commands_show_nothing() {
        for c in ["foot", "wpctl status", "wpctl set-volume 55 5%+", "trindle-brightness 40%", "trindle-ring", ""] {
            assert_eq!(kind_of(c), None, "{c}");
        }
    }

    #[test]
    fn the_level_maps_to_the_bar_and_is_clamped() {
        let m = Metrics::default();
        assert_eq!(filled(&m, 0.0), 0);
        assert_eq!(filled(&m, 0.5), 80);
        assert_eq!(filled(&m, 1.0), 160);
        assert_eq!(filled(&m, 1.5), 160);
        assert_eq!(filled(&m, -1.0), 0);
        assert_eq!(filled(&m, f32::NAN), 0);
        let loud = shown(Kind::Speakers, Sound { share: 1.4, muted: false });
        assert_eq!(loud.share, 1.0);
        assert_eq!(shown(Kind::Brightness, Sound { share: -0.3, muted: false }).share, 0.0);
    }

    #[test]
    fn the_icon_and_words_follow_what_changed() {
        let s = |share, muted| Sound { share, muted };
        assert_eq!(shown(Kind::Speakers, s(0.7, false)), Shown { icon: "audio-volume-high-symbolic", share: 0.7, words: None });
        assert_eq!(shown(Kind::Speakers, s(0.5, false)).icon, "audio-volume-medium-symbolic");
        assert_eq!(shown(Kind::Speakers, s(0.2, false)).icon, "audio-volume-low-symbolic");
        assert_eq!(shown(Kind::Speakers, s(0.0, false)).icon, "audio-volume-muted-symbolic");
        assert_eq!(shown(Kind::Speakers, s(0.7, true)), Shown { icon: "audio-volume-muted-symbolic", share: 0.0, words: Some("Muted") });
        assert_eq!(shown(Kind::Microphone, s(0.7, true)), Shown { icon: "microphone-sensitivity-muted-symbolic", share: 0.0, words: Some("Microphone off") });
        assert_eq!(shown(Kind::Microphone, s(0.7, false)).words, None);
        assert_eq!(shown(Kind::Brightness, s(0.3, false)), Shown { icon: BRIGHTNESS, share: 0.3, words: None });
    }

    #[test]
    fn the_display_goes_by_itself_after_its_time() {
        let (t, stays) = (Instant::now(), Duration::from_millis(1500));
        assert!(still(t, t, stays));
        assert!(still(t, t + Duration::from_millis(1499), stays));
        assert!(!still(t, t + stays, stays));
    }

    #[test]
    fn the_pill_grows_only_with_words() {
        let m = Metrics::default();
        assert_eq!(width(&m, 0.0), 16 + 22 + 12 + 160 + 16);
        assert_eq!(width(&m, 40.0), 16 + 22 + 12 + 160 + 12 + 40 + 16);
    }

    #[test]
    fn the_pill_is_whole_at_every_scale() {
        let m = Metrics::default();
        let s = |share, muted| Sound { share, muted };
        let cases = [(Kind::Brightness, s(0.5, false)), (Kind::Brightness, s(1.0, false)), (Kind::Speakers, s(0.8, true)), (Kind::Microphone, s(0.8, true))];
        for (kind, sound) in cases {
            let what = shown(kind, sound);
            for scale in [1.0f64, 1.25, 1.5, 2.0, 0.9, 1.65, 1.925] {
                let wl = width(&m, what.words.map_or(0.0, width_of));
                let (bytes, w, h) = paint(&what, &m, wl, scale);
                let (w, h) = (w as usize, h as usize);
                let alpha = |x: usize, y: usize| bytes[(y * w + x) * 4 + 3];
                let r = h as f32 / 2.0;
                for y in 0..h {
                    for x in 0..w {
                        let out = crate::indicators::outside_round_box(x as f32 + 0.5, y as f32 + 0.5, w as f32, h as f32, r);
                        if out > 1.0 {
                            assert_eq!(alpha(x, y), 0, "{kind:?} x{scale}: opaque outside the box at {x},{y}");
                        }
                        if out < -1.5 {
                            assert!(alpha(x, y) > 0, "{kind:?} x{scale}: hole inside the box at {x},{y}");
                        }
                    }
                }
                for (x, y) in [(0, 0), (w - 1, 0), (0, h - 1), (w - 1, h - 1)] {
                    assert_eq!(alpha(x, y), 0, "{kind:?} x{scale}: square corner at {x},{y}");
                }
                let pad = (m.pad as f64 * scale).floor() as usize;
                for y in 0..h {
                    for x in w - pad + 1..w {
                        let pixel = &bytes[(y * w + x) * 4..][..4];
                        let (b, g, rd) = (pixel[0], pixel[1], pixel[2]);
                        assert!(b.max(g).max(rd) < 0xb0 || alpha(x, y) < 0xff || out_of_pill(x, y, w, h), "{kind:?} x{scale}: ink in the right padding at {x},{y}");
                    }
                }
            }
        }
    }

    fn out_of_pill(x: usize, y: usize, w: usize, h: usize) -> bool {
        crate::indicators::outside_round_box(x as f32 + 0.5, y as f32 + 0.5, w as f32, h as f32, h as f32 / 2.0) > 0.0
    }

    #[test]
    #[ignore]
    fn shoot_the_level() {
        let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-level".into()));
        std::fs::create_dir_all(&out).unwrap();
        let m = Metrics::default();
        let s = |share, muted| Sound { share, muted };
        let cases = [
            ("volume-20", Kind::Speakers, s(0.2, false)),
            ("volume-80", Kind::Speakers, s(0.8, false)),
            ("volume-muted", Kind::Speakers, s(0.8, true)),
            ("microphone-off", Kind::Microphone, s(0.8, true)),
            ("brightness-30", Kind::Brightness, s(0.3, false)),
        ];
        for (light, look, desk) in [(false, "dark", (0x2a, 0x2d, 0x33)), (true, "light", (0xd8, 0xda, 0xdf))] {
            ssd::set_theme(crate::config::Theme::of(light), 10);
            for (name, kind, sound) in cases {
                let what = shown(kind, sound);
                for scale in [1.0f64, 1.5, 2.0] {
                let wl = width(&m, what.words.map_or(0.0, width_of));
                let (bytes, w, h) = paint(&what, &m, wl, scale);
                let (sw, sh) = (w + 240, h + 240);
                let mut screen = tiny_skia::Pixmap::new(sw as u32, sh as u32).unwrap();
                screen.fill(tiny_skia::Color::from_rgba8(desk.0, desk.1, desk.2, 0xff));
                let mut pill = tiny_skia::Pixmap::new(w as u32, h as u32).unwrap();
                for (d, b) in pill.data_mut().chunks_mut(4).zip(bytes.chunks(4)) {
                    d.copy_from_slice(&[b[2], b[1], b[0], b[3]]);
                }
                screen.draw_pixmap(120, 120, pill.as_ref(), &tiny_skia::PixmapPaint::default(), tiny_skia::Transform::identity(), None);
                screen.save_png(out.join(format!("level-{name}-{look}-x{scale}.png"))).unwrap();
                }
            }
        }
    }

    #[test]
    fn the_pill_is_painted_its_declared_size_at_every_scale() {
        let m = Metrics::default();
        let what = shown(Kind::Speakers, Sound { share: 0.5, muted: false });
        for scale in crate::indicators::every_scale() {
            for wl in [205, 215, 220, 230, 245, 300, 333] {
                let (bytes, w, h) = paint(&what, &m, wl, scale);
                assert_eq!((w, h), pixel_size((wl, m.height).into(), scale), "x{scale} w{wl}");
                assert_eq!(bytes.len(), (w * h * 4) as usize, "x{scale} w{wl}");
            }
        }
    }
}

