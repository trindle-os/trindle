//! Trindle's look as other programs' toolkits ask for it: dark or light, the
//! icons, pointer, fonts, animations and the rest GTK and libadwaita read.
//! It comes from Trindle's settings file and the GTK settings files Trindle
//! writes; what Trindle has no setting for is GNOME's own default, or the
//! choice Trindle's packaging already makes.

use std::path::PathBuf;

use super::files;
use crate::toolkit::colour::Colour;
use crate::toolkit::theme::{mix, Theme};

pub const INTERFACE: &str = "org.gnome.desktop.interface";
pub const APPEARANCE: &str = "org.freedesktop.appearance";
const A11Y: &str = "org.gnome.desktop.a11y";
const A11Y_INTERFACE: &str = "org.gnome.desktop.a11y.interface";
const CALENDAR: &str = "org.gnome.desktop.calendar";
const FONTCONFIG: &str = "org.gnome.fontconfig";
const MOUSE: &str = "org.gnome.desktop.peripherals.mouse";
const PRIVACY: &str = "org.gnome.desktop.privacy";
const SOUND: &str = "org.gnome.desktop.sound";
const WM: &str = "org.gnome.desktop.wm.preferences";

pub const KDE_GENERAL: &str = "org.kde.kdeglobals.General";
const KDE_KDE: &str = "org.kde.kdeglobals.KDE";

const ICONS: &str = "Trindle-Circle";
const CURSOR: &str = "Adwaita";
const CURSOR_SIZE: i32 = 24;
const FONT: &str = "Trindle Sans 11";
const MONOSPACE: &str = "Monospace 11";
const DOUBLE_CLICK: i32 = super::pointer::DOUBLE_CLICK_MS as i32;
const DRAG: i32 = super::pointer::DOUBLE_CLICK_SLOP as i32;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    S(String),
    B(bool),
    I(i32),
    U(u32),
    D(f64),
}

impl Value {
    fn s(text: &str) -> Self {
        Value::S(text.to_string())
    }

    pub fn sig(&self) -> &'static str {
        match self {
            Value::S(_) => "s",
            Value::B(_) => "b",
            Value::I(_) => "i",
            Value::U(_) => "u",
            Value::D(_) => "d",
        }
    }

    fn gvariant(&self) -> Vec<u8> {
        match self {
            Value::S(s) => [s.as_bytes(), &[0]].concat(),
            Value::B(b) => vec![u8::from(*b)],
            Value::I(i) => i.to_le_bytes().to_vec(),
            Value::U(u) => u.to_le_bytes().to_vec(),
            Value::D(d) => d.to_le_bytes().to_vec(),
        }
    }
}

pub type Entry = (&'static str, &'static str, Value);

pub fn gtk_files() -> [PathBuf; 2] {
    let home = files::config_home();
    [home.join("gtk-4.0/settings.ini"), home.join("gtk-3.0/settings.ini")]
}

pub fn now() -> Vec<Entry> {
    let light = super::style::kept_light();
    let ini = files::read_text(&gtk_files()[0]);
    table(light, &ini, super::graphics::moving())
}

pub fn ini_value<'a>(ini: &'a str, key: &str) -> Option<&'a str> {
    ini.lines().filter_map(|l| l.split_once('='))
        .find(|(k, _)| k.trim() == key)
        .map(|(_, v)| v.trim())
        .filter(|v| !v.is_empty())
}

fn table(light: bool, ini: &str, moving: bool) -> Vec<Entry> {
    let from = |key, default| ini_value(ini, key).unwrap_or(default).to_string();
    let font = from("gtk-font-name", FONT);
    let size = from("gtk-cursor-theme-size", "").parse().unwrap_or(CURSOR_SIZE);
    vec![
        (INTERFACE, "color-scheme", Value::s(if light { "prefer-light" } else { "prefer-dark" })),

        (INTERFACE, "gtk-theme", Value::s("Adwaita")),
        (INTERFACE, "icon-theme", Value::S(from("gtk-icon-theme-name", ICONS))),
        (INTERFACE, "cursor-theme", Value::S(from("gtk-cursor-theme-name", CURSOR))),
        (INTERFACE, "cursor-size", Value::I(size)),
        (INTERFACE, "font-name", Value::S(font.clone())),
        (INTERFACE, "document-font-name", Value::S(font)),
        (INTERFACE, "monospace-font-name", Value::s(MONOSPACE)),
        (INTERFACE, "accent-color", Value::s("blue")),
        (INTERFACE, "text-scaling-factor", Value::D(1.0)),

        (INTERFACE, "enable-animations", Value::B(moving)),
        (INTERFACE, "font-antialiasing", Value::s("grayscale")),
        (INTERFACE, "font-hinting", Value::s("slight")),
        (INTERFACE, "font-rgba-order", Value::s("rgb")),
        (INTERFACE, "cursor-blink", Value::B(true)),
        (INTERFACE, "cursor-blink-time", Value::I(1200)),
        (INTERFACE, "cursor-blink-timeout", Value::I(10)),
        (INTERFACE, "gtk-im-module", Value::s("")),
        (INTERFACE, "gtk-enable-primary-paste", Value::B(true)),
        (INTERFACE, "overlay-scrolling", Value::B(true)),
        (APPEARANCE, "color-scheme", Value::U(if light { 2 } else { 1 })),
        (APPEARANCE, "contrast", Value::U(0)),
        (A11Y, "always-show-text-caret", Value::B(false)),
        (A11Y_INTERFACE, "high-contrast", Value::B(false)),
        (A11Y_INTERFACE, "show-status-shapes", Value::B(false)),
        (CALENDAR, "show-weekdate", Value::B(false)),
        (FONTCONFIG, "serial", Value::I(0)),
        (MOUSE, "double-click", Value::I(DOUBLE_CLICK)),
        (MOUSE, "drag-threshold", Value::I(DRAG)),
        (PRIVACY, "recent-files-max-age", Value::I(-1)),
        (PRIVACY, "remember-recent-files", Value::B(true)),
        (SOUND, "theme-name", Value::s("freedesktop")),
        (SOUND, "event-sounds", Value::B(true)),
        (SOUND, "input-feedback-sounds", Value::B(false)),

        (WM, "button-layout", Value::s(":minimize,maximize,close")),
        (WM, "action-double-click-titlebar", Value::s("toggle-maximize")),
        (WM, "action-middle-click-titlebar", Value::s("none")),
        (WM, "action-right-click-titlebar", Value::s("menu")),
        (KDE_GENERAL, "ColorScheme", Value::s(qt_scheme(light))),
        (KDE_KDE, "widgetStyle", Value::s(QT_STYLE)),
    ]
}

pub fn changed<'a>(before: &[Entry], after: &'a [Entry]) -> Vec<&'a Entry> {
    after.iter().filter(|e| !before.contains(e)).collect()
}

const QT_STYLE: &str = "Fusion";

fn qt_scheme(light: bool) -> &'static str {
    if light { "TrindleLight" } else { "TrindleDark" }
}

pub fn qt_folder() -> PathBuf {
    files::trindle_config().join("qt")
}

pub fn write_qt() -> String {
    let path = qt_folder().join("kdeglobals");
    let text = qt_globals(&Theme::chosen(), &files::read_text(&gtk_files()[0]));
    if files::read_text(&path) != text {
        let _ = files::write_text(&path, &text);
    }
    text
}

fn qt_globals(t: &Theme, ini: &str) -> String {
    let rgb = |c: Colour| {
        let c = mix(t.bg, Colour { a: 1.0, ..c }, c.a);
        let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
        format!("{},{},{}", byte(c.r), byte(c.g), byte(c.b))
    };
    let font = qt_font(ini_value(ini, "gtk-font-name").unwrap_or(FONT));
    let mut text = format!(
        "[General]\nColorScheme={}\nfont={font}\nmenuFont={font}\ntoolBarFont={font}\nfixed={}\n\n\
         [Icons]\nTheme={}\n\n\
         [KDE]\nwidgetStyle={QT_STYLE}\nSingleClick=false\nDoubleClickInterval={DOUBLE_CLICK}\nStartDragDistance={DRAG}\n",
        qt_scheme(t.light), qt_font(MONOSPACE), ini_value(ini, "gtk-icon-theme-name").unwrap_or(ICONS));
    let groups = [
        ("Window", t.bg, t.panel, t.fg),
        ("Header", t.bg, t.panel, t.fg),

        ("View", t.field, t.bg, t.fg),
        ("Button", t.control, t.raised, t.fg),
        ("Selection", t.accent, t.accent, t.on_accent),
        ("Tooltip", crate::toolkit::theme::sheet().label_back, crate::toolkit::theme::sheet().label_back,
         crate::toolkit::theme::sheet().label_ink),
    ];
    for (group, back, alternate, fore) in groups {
        let lines = [
            ("BackgroundNormal", back), ("BackgroundAlternate", alternate),
            ("ForegroundNormal", fore), ("ForegroundInactive", t.muted),
            ("ForegroundLink", t.accent_text), ("ForegroundVisited", t.accent_text),
            ("ForegroundNegative", t.danger), ("ForegroundPositive", t.fg),
            ("DecorationFocus", t.accent), ("DecorationHover", t.accent),
        ];
        text.push_str(&format!("\n[Colors:{group}]\n"));
        for (key, colour) in lines {
            text.push_str(&format!("{key}={}\n", rgb(colour)));
        }
    }
    text
}

fn qt_font(gtk: &str) -> String {
    let (family, size) = match gtk.rsplit_once(' ') {
        Some((family, size)) if size.parse::<f32>().is_ok() => (family, size),
        _ => (gtk, "11"),
    };
    format!("{family},{size},-1,5,50,0,0,0,0,0")
}

pub fn dconf_change(values: &[(String, &Value)]) -> Vec<u8> {
    let mut array = Vec::new();
    let mut ends = Vec::new();
    for (path, value) in values {
        pad(&mut array, 8);

        let mut entry = [path.as_bytes(), &[0]].concat();
        let path_end = entry.len();
        pad(&mut entry, 8);
        entry.extend(value.gvariant());
        entry.push(0);
        entry.extend(value.sig().as_bytes());
        entry.push(0);
        array.extend(framed(entry, &[path_end]));
        ends.push(array.len());
    }
    framed(array, &ends)
}

fn pad(bytes: &mut Vec<u8>, to: usize) {
    bytes.resize(bytes.len().next_multiple_of(to), 0);
}

fn framed(mut body: Vec<u8>, ends: &[usize]) -> Vec<u8> {
    if ends.is_empty() {
        return body;
    }
    let width = [1usize, 2, 4, 8].into_iter()
        .find(|w| *w == 8 || body.len() + w * ends.len() < 1 << (8 * w))
        .unwrap_or(8);
    for end in ends {
        body.extend(&end.to_le_bytes()[..width]);
    }
    body
}

#[cfg(test)]
mod tests {
    use super::*;

    fn offset_size(n: usize) -> usize {
        match n {
            0 => 0,
            1..=0xff => 1,
            0x100..=0xffff => 2,
            0x1_0000..=0xffff_ffff => 4,
            _ => 8,
        }
    }

    fn end_at(bytes: &[u8], at: usize, size: usize) -> usize {
        bytes[at..at + size].iter().rev().fold(0, |n, b| (n << 8) | *b as usize)
    }

    fn parse(bytes: &[u8]) -> Vec<(String, Value)> {
        let n = bytes.len();
        if n == 0 {
            return Vec::new();
        }
        let size = offset_size(n);
        let table = end_at(bytes, n - size, size);
        let count = (n - table) / size;
        let mut start = 0;
        (0..count).map(|i| {
            let end = end_at(bytes, table + i * size, size);
            let entry = &bytes[start..end];
            start = (end + 7) & !7;
            let size = offset_size(entry.len());
            let key_end = end_at(entry, entry.len() - size, size);
            let key = std::str::from_utf8(&entry[..key_end - 1]).unwrap().to_string();

            let maybe = &entry[(key_end + 7) & !7..entry.len() - size];
            assert_eq!(maybe.last(), Some(&0));
            let variant = &maybe[..maybe.len() - 1];
            let split = variant.iter().rposition(|b| *b == 0).unwrap();
            let (data, sig) = (&variant[..split], std::str::from_utf8(&variant[split + 1..]).unwrap());
            let value = match sig {
                "s" => Value::S(std::str::from_utf8(&data[..data.len() - 1]).unwrap().to_string()),
                "b" => Value::B(data == [1]),
                "i" => Value::I(i32::from_le_bytes(data.try_into().unwrap())),
                "u" => Value::U(u32::from_le_bytes(data.try_into().unwrap())),
                "d" => Value::D(f64::from_le_bytes(data.try_into().unwrap())),
                other => panic!("unexpected type {other}"),
            };
            (key, value)
        }).collect()
    }

    #[test]
    fn dconf_can_read_the_change() {
        let (name, size, moving, scale, scheme) =
            (Value::s("Trindle"), Value::I(24), Value::B(false), Value::D(1.5), Value::U(2));
        let read = parse(&dconf_change(&[
            ("/org/gnome/desktop/interface/icon-theme".into(), &name),
            ("/org/gnome/desktop/interface/cursor-size".into(), &size),
            ("/org/gnome/desktop/interface/enable-animations".into(), &moving),
            ("/org/gnome/desktop/interface/text-scaling-factor".into(), &scale),
            ("/a".into(), &scheme),
        ]));
        assert_eq!(read[0].0, "/org/gnome/desktop/interface/icon-theme");
        assert_eq!(read[0].1, Value::s("Trindle"));
        assert_eq!(read[1].1, Value::I(24));
        assert_eq!(read[2].1, Value::B(false));
        assert_eq!(read[3].1, Value::D(1.5));
        assert_eq!((read[4].0.as_str(), &read[4].1), ("/a", &Value::U(2)));
    }

    #[test]
    fn a_long_change_takes_wider_ends() {
        let long = Value::S("x".repeat(70_000));
        let read = parse(&dconf_change(&[("/k".into(), &long), ("/l".into(), &Value::B(true))]));
        assert_eq!(read[0].1, long);
        assert_eq!(read[1].1, Value::B(true));
        assert!(parse(&dconf_change(&[])).is_empty());
    }

    #[test]
    fn the_look_comes_from_the_files() {
        let ini = "[Settings]\ngtk-icon-theme-name=Trindle\ngtk-cursor-theme-size=32\ngtk-font-name=Inter 12\n";
        let look = table(true, ini, false);
        let get = |ns: &str, key: &str| look.iter().find(|e| e.0 == ns && e.1 == key).unwrap().2.clone();
        assert_eq!(get(INTERFACE, "icon-theme"), Value::s("Trindle"));
        assert_eq!(get(INTERFACE, "cursor-size"), Value::I(32));
        assert_eq!(get(INTERFACE, "cursor-theme"), Value::s(CURSOR));
        assert_eq!(get(INTERFACE, "document-font-name"), Value::s("Inter 12"));
        assert_eq!(get(INTERFACE, "color-scheme"), Value::s("prefer-light"));
        assert_eq!(get(APPEARANCE, "color-scheme"), Value::U(2));
        assert_eq!(get(INTERFACE, "enable-animations"), Value::B(false));
        let dark = table(false, "", true);
        assert_eq!(get(KDE_GENERAL, "ColorScheme"), Value::s("TrindleLight"));
        assert_eq!(changed(&look, &dark).len(), 8);
        assert!(changed(&dark, &dark).is_empty());
    }

    #[test]
    fn other_programs_are_told_the_look_the_theme_really_is() {
        use crate::common::style::{told_light, CSS};
        let both = ":root { --trindle-page: #101010; } @media (prefers-color-scheme: light) { :root { --trindle-page: #fafafa; } }";
        let themes = [("trindle", "", None), ("midnight", include_str!("../../share/themes/midnight.css"), Some(false)),
                      ("paper", include_str!("../../share/themes/paper.css"), Some(true)), ("bold", include_str!("../../share/themes/bold.css"), Some(false)),
                      ("snug", include_str!("../../share/themes/snug.css"), None), ("both", both, None)];
        for (name, css, only) in themes {
            for saved in ["dark", "light", "", "nonsense"] {
                let light = told_light(css, saved);
                assert_eq!(light, only.unwrap_or(saved == "light"), "{name} with {saved:?} saved");

                let layers = crate::common::sheet::Layers::themed(CSS, css, "");
                assert_eq!(light, layers.light(saved == "light"), "{name} with {saved:?} saved");
                assert_eq!(crate::toolkit::theme::sample(css, saved == "light", "#2196f3").theme.light, light, "{name} with {saved:?} saved");
                let told = table(light, "", true);
                let get = |ns: &str, key: &str| told.iter().find(|e| e.0 == ns && e.1 == key).unwrap().2.clone();
                assert_eq!(get(INTERFACE, "color-scheme"), Value::s(if light { "prefer-light" } else { "prefer-dark" }), "{name} with {saved:?} saved");
                assert_eq!(get(APPEARANCE, "color-scheme"), Value::U(if light { 2 } else { 1 }), "{name} with {saved:?} saved");
                assert_eq!(get(KDE_GENERAL, "ColorScheme"), Value::s(if light { "TrindleLight" } else { "TrindleDark" }), "{name} with {saved:?} saved");
            }
        }
    }

    #[test]
    fn qt_is_told_the_look_in_kdes_file() {
        let ini = "[Settings]\ngtk-icon-theme-name=Trindle\ngtk-font-name=Inter Bold 12.5\n";
        let dark = qt_globals(&Theme::new(false, "#2196f3"), ini);
        assert!(dark.contains("ColorScheme=TrindleDark\n"));
        assert!(dark.contains("font=Inter Bold,12.5,-1,5,50,0,0,0,0,0\n"));
        assert!(dark.contains("fixed=Monospace,11,-1,5,50,0,0,0,0,0\n"));
        assert!(dark.contains("[Icons]\nTheme=Trindle\n"));
        assert!(dark.contains("widgetStyle=Fusion\n"));
        assert!(dark.contains("[Colors:Selection]\nBackgroundNormal=33,150,243\n"));
        assert!(dark.contains("[Colors:Tooltip]\nBackgroundNormal=46,68,104\n"), "{dark}");

        for line in dark.lines().filter(|l| l.contains("ground") || l.starts_with("Decoration")) {
            let value = line.split_once('=').unwrap().1;
            let parts: Vec<&str> = value.split(',').collect();
            assert_eq!(parts.len(), 3, "{line}");
            assert!(parts.iter().all(|p| p.parse::<u8>().is_ok()), "{line}");
        }

        let lightness = |text: &str, key: &str| -> u32 {
            let group = text.split("[Colors:View]").nth(1).unwrap();
            let line = group.lines().find(|l| l.starts_with(key)).unwrap();
            line.split_once('=').unwrap().1.split(',').map(|p| p.parse::<u32>().unwrap()).sum()
        };
        assert!(lightness(&dark, "ForegroundNormal") > lightness(&dark, "BackgroundNormal"));
        let light = qt_globals(&Theme::new(true, "#2196f3"), "");
        assert!(light.contains("ColorScheme=TrindleLight\n"));
        assert!(light.contains("font=Trindle Sans,11,-1,5,50,0,0,0,0,0\n"));
        assert!(lightness(&light, "ForegroundNormal") < lightness(&light, "BackgroundNormal"));
    }

    #[test]
    fn qt_is_given_every_group_and_key_its_kde_theme_reads() {
        for light in [false, true] {
            let text = qt_globals(&Theme::new(light, "#2196f3"), "");
            for group in ["Window", "View", "Button", "Selection", "Tooltip"] {
                let body = text.split(&format!("[Colors:{group}]\n")).nth(1).unwrap_or_else(|| panic!("no {group} group"));
                let body = body.split("\n[").next().unwrap();
                for key in ["BackgroundNormal", "BackgroundAlternate", "ForegroundNormal", "ForegroundInactive", "DecorationFocus", "DecorationHover"] {
                    assert!(body.lines().any(|l| l.starts_with(&format!("{key}="))), "light {light}: {group} has no {key}");
                }
            }
        }
    }
}
