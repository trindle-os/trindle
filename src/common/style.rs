//! The look as Trindle hands it on: told to GTK programs, Flatpaks and
//! GNOME's settings, earlier versions' icon themes moved onto Trindle's
//! sets, the ring's own numbers and the palette of highlights. The look
//! itself, the accent and the style sheet, is the toolkit's
//! (its common/style.rs and common/sheet.rs).

pub use crate::toolkit::common::style::*;

pub struct Palette {

    pub dim: f64,
}

pub fn palette() -> Palette {
    Palette { dim: layers().number("trindle-ring-dim", light(), 0.0, 0.8).unwrap_or(0.0) as f64 }
}

pub fn ring_page_margin() -> f32 {
    layers().number("trindle-ring-page-margin", light(), 4.0, 24.0).unwrap_or(10.0)
}

pub fn ring_words_width() -> f32 {
    layers().number("trindle-ring-words-width", light(), 240.0, 800.0).unwrap_or(480.0)
}

const OLD_ICON_THEMES: &[&str] = &["Numix-Circle", "Numix-Circle-Light"];

fn icon_theme_instead(now: &str, made: bool, set: &str) -> Option<String> {
    use crate::icons::THEME;
    match (OLD_ICON_THEMES.contains(&now), now == THEME, made) {
        (true, _, true) => Some(THEME.to_string()),
        (true, _, false) | (_, true, false) => Some(set.to_string()),
        _ => None,
    }
}

fn icon_source_instead(saved: Option<&str>) -> Option<Option<&'static str>> {
    match saved? {
        v if crate::icons::is_a_set(v) => None,
        _ => Some(None),
    }
}

fn move_onto_trindle_sets() {
    let conf = super::files::trindle_conf();
    if let Some(want) = icon_source_instead(super::files::read_env(&conf).get("icon_source").map(String::as_str)) {
        let _ = super::files::update_env(&conf, &[("icon_source", want)]);
    }
    let set = crate::icons::chosen_set();
    let made = crate::icons::theme_made();
    for dir in ["gtk-3.0", "gtk-4.0"] {
        let file = super::files::config_home().join(dir).join("settings.ini");
        let text = super::files::read_text(&file);
        let Some(now) = text.lines().find_map(|l| l.strip_prefix("gtk-icon-theme-name=")) else { continue };
        if let Some(want) = icon_theme_instead(now.trim(), made, set) {
            let _ = super::files::write_text(&file, &text.replace(&format!("gtk-icon-theme-name={now}"),
                                                                  &format!("gtk-icon-theme-name={want}")));
        }
    }
}

pub fn apply_system_theme() {
    move_onto_trindle_sets();
    use std::process::{Command, Stdio};
    let light = light();
    let quiet = |args: &[&str]| {
        let _ = Command::new(args[0]).args(&args[1..]).stdout(Stdio::null()).stderr(Stdio::null()).status();
    };
    super::app_theme::write();

    quiet(&["flatpak", "override", "--user", "--unset-env=GTK_THEME",
            "--filesystem=xdg-config/gtk-3.0:ro", "--filesystem=xdg-config/gtk-4.0:ro"]);
    for dir in ["gtk-3.0", "gtk-4.0"] {
        let file = super::files::config_home().join(dir).join("settings.ini");
        let text = super::files::read_text(&file);
        if text.is_empty() {
            continue;
        }
        let mut lines: Vec<String> = text.lines().filter(|l| !(l.starts_with("gtk-application-prefer-dark-theme")
            || dir == "gtk-3.0" && l.starts_with("gtk-theme-name"))).map(font_now).collect();
        let at = lines.iter().position(|l| l.trim() == "[Settings]").map(|i| i + 1).unwrap_or(0);
        lines.insert(at, format!("gtk-application-prefer-dark-theme={}", !light));
        if dir == "gtk-3.0" {
            lines.insert(at, "gtk-theme-name=Adwaita".to_string());
        }
        let new = lines.join("\n") + "\n";
        if new != text {
            let _ = super::files::write_text(&file, &new);
        }
    }

    super::dconf::copy_look();
}

fn font_now(line: &str) -> String {
    match line.trim().strip_prefix("gtk-font-name=Manrope ") {
        Some(size) if size.parse::<f32>().is_ok() => format!("gtk-font-name={} {size}", crate::toolkit::text::FAMILY),
        _ => line.to_string(),
    }
}

pub const ACCENTS: &[(&str, &str)] = &[
    ("#fbc711", "Yellow"),
    ("#fe9700", "Orange"),
    ("#f34236", "Red"),
    ("#e8207e", "Pink"),
    ("#9b26ae", "Purple"),
    ("#3d51b4", "Indigo"),
    ("#2295f1", "Blue"),
    ("#00bcd3", "Cyan"),
    ("#019587", "Teal"),
    ("#4bae4f", "Green"),
];

#[cfg(test)]
mod tests {
    #[test]
    fn the_old_default_font_becomes_trindles_own() {
        assert_eq!(super::font_now("gtk-font-name=Manrope 11"), "gtk-font-name=Trindle Sans 11");
        assert_eq!(super::font_now("gtk-font-name=Manrope Bold 11"), "gtk-font-name=Manrope Bold 11");
        assert_eq!(super::font_now("gtk-font-name=Noto Sans 12"), "gtk-font-name=Noto Sans 12");
    }

    #[test]
    fn earlier_icons_move_onto_the_trindle_sets() {
        let instead = super::icon_theme_instead;

        assert_eq!(instead("Numix-Circle", true, "Trindle-Circle").as_deref(), Some("Trindle"));
        assert_eq!(instead("Numix-Circle-Light", false, "Trindle-Circle").as_deref(), Some("Trindle-Circle"));

        assert_eq!(instead("Trindle", true, "Trindle-Circle"), None);
        assert_eq!(instead("Trindle", false, "Trindle-Circle").as_deref(), Some("Trindle-Circle"));

        assert_eq!(instead("Trindle-Circle", false, "Trindle-Circle"), None);
        assert_eq!(instead("Adwaita", true, "Trindle-Circle"), None);
    }

    #[test]
    fn only_a_set_this_version_offers_is_kept_as_the_icon_source() {
        let instead = super::icon_source_instead;
        assert_eq!(instead(Some("Papirus")), Some(None));
        assert_eq!(instead(Some("Numix-Circle")), Some(None));
        assert_eq!(instead(Some("")), Some(None));
        assert_eq!(instead(Some("Trindle-Square")), Some(None));
        assert_eq!(instead(Some("Trindle-Circle")), None);
        assert_eq!(instead(None), None);
    }
}
