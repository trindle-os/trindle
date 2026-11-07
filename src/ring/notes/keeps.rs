//! Pinned and snoozed circles kept across a restart.

use crate::common::files::{config_path, read_text, write_text};

const FILE: &str = "notes-kept";

#[derive(Clone, Debug, PartialEq)]
pub struct Kept {
    pub app: String,
    pub icon: String,
    pub desktop: String,
    pub summary: String,
    pub body: String,
    pub put: Option<(f32, f32)>,
    pub wake: Option<u64>,
}

fn clean(s: &str) -> String {
    s.replace(['\t', '\n', '\r'], " ")
}

pub fn render(all: &[Kept]) -> String {
    all.iter().map(|k| {
        let (x, y) = k.put.map_or((String::new(), String::new()), |p| (p.0.to_string(), p.1.to_string()));
        let wake = k.wake.map_or(String::new(), |w| w.to_string());
        [clean(&k.app), clean(&k.icon), clean(&k.desktop), clean(&k.summary), clean(&k.body), x, y, wake].join("\t") + "\n"
    }).collect()
}

pub fn parse(text: &str) -> Vec<Kept> {
    text.lines().filter_map(|l| {
        let f: Vec<&str> = l.split('\t').collect();
        if f.len() != 8 {
            return None;
        }
        let put = match (f[5].parse::<f32>(), f[6].parse::<f32>()) {
            (Ok(x), Ok(y)) if x.is_finite() && y.is_finite() => Some((x, y)),
            _ => None,
        };
        Some(Kept {
            app: f[0].into(), icon: f[1].into(), desktop: f[2].into(), summary: f[3].into(), body: f[4].into(),
            put, wake: f[7].parse().ok(),
        })
    }).collect()
}

pub fn load() -> Vec<Kept> {
    parse(&read_text(config_path(FILE)))
}

pub fn save(text: &str) {
    if let Err(e) = write_text(&config_path(FILE), text) {
        eprintln!("trindle-ring: could not keep the circles: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pinned_circles_place_and_a_snoozed_ones_time_come_back_as_they_went() {
        let all = vec![
            Kept { app: "Mail".into(), icon: "mail".into(), desktop: "mail.desktop".into(), summary: "Hello\tthere".into(), body: "Two\nlines".into(), put: Some((120.0, 40.5)), wake: None },
            Kept { app: "Clock".into(), icon: String::new(), desktop: String::new(), summary: "Wake".into(), body: String::new(), put: None, wake: Some(1_800_000_000_000) },
        ];
        let back = parse(&render(&all));
        assert_eq!(back.len(), 2);
        assert_eq!(back[0].put, Some((120.0, 40.5)));
        assert_eq!(back[0].summary, "Hello there");
        assert_eq!(back[0].body, "Two lines");
        assert_eq!((back[1].put, back[1].wake), (None, Some(1_800_000_000_000)));
    }

    #[test]
    fn a_line_that_does_not_read_is_left_out() {
        assert!(parse("one\ttwo\nnot even this\n").is_empty());
        assert_eq!(parse("a\tb\tc\td\te\tNaN\t3\t\n")[0].put, None);
    }
}
