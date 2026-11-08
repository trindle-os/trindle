//! What the installer says and does for the system it is installing.

use crate::common::files::read_keys;
use crate::toolkit::t;

pub struct Brand {

    pub name: String,

    pub accent: (f64, f64, f64),

    pub slides: Vec<(String, String)>,

    pub remove: Vec<String>,

    pub after: Option<String>,
}

const PLACES: &[&str] = &["/etc/trindle-installer.conf", "/usr/local/etc/trindle-installer.conf"];

impl Default for Brand {
    fn default() -> Brand {
        Brand {
            name: "Trindle".into(),
            accent: (0.984, 0.780, 0.067),
            slides: default_slides().iter().map(|(t, b)| (t.to_string(), b.to_string())).collect(),
            remove: ["live-boot", "live-boot-doc", "live-boot-initramfs-tools", "live-config",
                     "live-config-systemd", "live-tools"].iter().map(|s| s.to_string()).collect(),
            after: None,
        }
    }
}

pub fn brand() -> Brand {
    let Some(text) = PLACES.iter().find_map(|p| std::fs::read_to_string(p).ok()) else {
        return Brand::default();
    };
    read(&text)
}

pub fn read(text: &str) -> Brand {
    let keys = read_keys(text);
    let mut brand = Brand::default();
    if let Some(name) = keys.get("name").filter(|n| !n.is_empty()) {
        brand.name = name.clone();
    }
    if let Some(rgb) = keys.get("accent").and_then(|c| colour(c)) {
        brand.accent = rgb;
    }
    if let Some(list) = keys.get("remove") {
        brand.remove = list.split_whitespace().map(String::from).collect();
    }
    brand.after = keys.get("after").filter(|a| !a.is_empty()).cloned();
    let mut slides = Vec::new();
    for n in 1..=12 {
        let (title, body) = (keys.get(&format!("slide{n}_title")), keys.get(&format!("slide{n}_text")));
        if let (Some(t), Some(b)) = (title, body) {
            slides.push((t.clone(), b.clone()));
        }
    }
    if !slides.is_empty() {
        brand.slides = slides;
    }
    brand
}

fn colour(hex: &str) -> Option<(f64, f64, f64)> {
    let h = hex.trim().trim_start_matches('#');
    if h.len() != 6 || !h.is_ascii() {
        return None;
    }
    let part = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok().map(|v| v as f64 / 255.0);
    Some((part(0)?, part(2)?, part(4)?))
}

fn default_slides() -> [(&'static str, &'static str); 5] {
    [
        (t!("installer-slide-ring"), t!("installer-slide-ring-text")),
        (t!("installer-slide-snapshots"), t!("installer-slide-snapshots-text")),
        (t!("installer-slide-apps"), t!("installer-slide-apps-text")),
        (t!("installer-slide-yours"), t!("installer-slide-yours-text")),
        (t!("installer-slide-small"), t!("installer-slide-small-text")),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn six_bytes_that_are_not_hex_digits_are_no_colour() {
        assert_eq!(colour("\u{20ac}\u{20ac}"), None);
        assert_eq!(colour("#fbc71\u{e9}"), None);
        assert!(colour("#fbc711").is_some());
    }

    #[test]
    fn a_project_can_write_its_own() {
        let text = "name=Example Linux\naccent=#ACCENT\nremove=live-boot live-config\nafter=/usr/bin/example-setup\n\
                    slide1_title=One\nslide1_text=First\nslide2_title=Two\nslide2_text=Second\n";

        let b = read(&text.replace("ACCENT", "2295f1"));
        assert_eq!(b.name, "Example Linux");
        assert_eq!(b.slides.len(), 2);
        assert_eq!(b.remove, vec!["live-boot", "live-config"]);
        assert_eq!(b.after.as_deref(), Some("/usr/bin/example-setup"));
        assert!((b.accent.0 - 0.133).abs() < 0.01);
    }

    #[test]
    fn the_installer_follows_the_language_and_falls_back_word_by_word() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let french = "installer-slide-small = Petit et calme\ninstaller-user-needed = Un nom est nécessaire.\n\
                      installer-guess-minutes =\n    { $n ->\n        [one] Environ { $n } minute.\n       *[other] Environ { $n } minutes.\n    }\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            let slides = Brand::default().slides;
            assert_eq!(slides[4].0, "Petit et calme");
            assert_eq!(slides[0].0, "Everything in one circle", "a word French lacks is English");
            assert_eq!(super::super::plan::username_fault(""), Some("Un nom est nécessaire."));
            assert_eq!(crate::toolkit::t!("installer-guess-minutes", n = 1), "Environ 1 minute.");
            assert_eq!(crate::toolkit::t!("installer-guess-minutes", n = 5), "Environ 5 minutes.");
        });
    }

    #[test]
    fn nothing_written_means_trindle() {
        let b = read("");
        assert_eq!(b.name, "Trindle");
        assert!(b.slides.len() >= 5);
    }
}
