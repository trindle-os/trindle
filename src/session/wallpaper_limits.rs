//! How big a wallpaper may be, in one place.

use crate::toolkit::t;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::SystemTime;

use super::moving;

pub const MOST_WIDE: u32 = 3840;
pub const MOST_HIGH: u32 = 2160;

pub const MOST_STILL_BYTES: u64 = 20 << 20;
pub const MOST_MOVING_BYTES: u64 = 200 << 20;

pub fn bigger_than_screen() -> &'static str {
    t!("session-wallpaper-bigger-than-screen")
}
pub fn moving_costs() -> &'static str {
    t!("session-wallpaper-moving-costs")
}

pub fn fits_screen(width: u32, height: u32, screen: Option<(u32, u32)>) -> bool {
    screen.is_none_or(|(w, h)| (width <= w && height <= h) || (width <= h && height <= w))
}

pub fn screen_biggest() -> Option<(u32, u32)> {
    std::fs::read_dir("/sys/class/drm").ok()?.flatten()
        .filter(|d| std::fs::read_to_string(d.path().join("status")).is_ok_and(|s| s.trim() == "connected"))
        .filter_map(|d| {
            let modes = std::fs::read_to_string(d.path().join("modes")).ok()?;
            let (w, h) = modes.lines().next()?.split_once('x')?;
            Some((w.parse().ok()?, h.trim_end_matches(|c: char| !c.is_ascii_digit()).parse().ok()?))
        })
        .max_by_key(|(w, h): &(u32, u32)| w * h)
}

pub fn fits(width: u32, height: u32) -> bool {
    (width <= MOST_WIDE && height <= MOST_HIGH) || (width <= MOST_HIGH && height <= MOST_WIDE)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Over {
    pub pixels: bool,
    pub film_bigger: bool,
    pub still_bytes: bool,
    pub moving_bytes: bool,
}

impl Over {
    pub fn any(&self) -> bool {
        self.pixels || self.film_bigger || self.still_bytes || self.moving_bytes
    }

    fn or(self, other: Over) -> Over {
        Over {
            pixels: self.pixels || other.pixels,
            film_bigger: self.film_bigger || other.film_bigger,
            still_bytes: self.still_bytes || other.still_bytes,
            moving_bytes: self.moving_bytes || other.moving_bytes,
        }
    }

    pub fn reasons(&self) -> String {
        let mut out = Vec::new();
        if self.pixels {
            out.push(t!("session-wallpaper-larger-4k"));
        } else if self.film_bigger {
            out.push(t!("session-wallpaper-larger-screen"));
        }
        match (self.still_bytes, self.moving_bytes) {
            (true, true) => out.push(t!("session-wallpaper-over-20-or-200")),
            (true, false) => out.push(t!("session-wallpaper-over-20")),
            (false, true) => out.push(t!("session-wallpaper-over-200")),
            (false, false) => {}
        }
        match out.as_slice() {
            [first, second] => t!("session-wallpaper-reasons-or", first = *first, second = *second),
            [only] => only.to_string(),
            _ => String::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Left {
    pub count: usize,
    pub why: Over,
}

impl Left {
    pub fn add(&mut self, over: Over) {
        if over.any() {
            self.count += 1;
            self.why = self.why.or(over);
        }
    }

    pub fn words(&self) -> Option<String> {
        (self.count > 0).then(|| t!("session-wallpaper-left-out", count = self.count, why = self.why.reasons()))
    }
}

pub fn refusal(over: Over) -> Option<String> {
    let mut left = Left::default();
    left.add(over);
    left.words()
}

fn moving_one(path: &Path) -> bool {
    moving::is_moving(path)
}

fn by_size(path: &Path) -> Option<(Over, u64, SystemTime)> {
    let about = std::fs::metadata(path).ok()?;
    let mut over = Over::default();
    if moving_one(path) {
        over.moving_bytes = about.len() > MOST_MOVING_BYTES;
    } else {
        over.still_bytes = about.len() > MOST_STILL_BYTES;
    }
    Some((over, about.len(), about.modified().unwrap_or(SystemTime::UNIX_EPOCH)))
}

fn size_of_still(path: &Path) -> Option<(u32, u32)> {
    crate::toolkit::picture::reader(path).ok()?.into_dimensions().ok()
}

fn size_of_film(path: &Path) -> Option<(u32, u32)> {
    moving::measure(path).map(|i| (i.width, i.height))
}

type Key = (std::path::PathBuf, SystemTime, u64);

fn kept() -> &'static Mutex<HashMap<Key, Option<(u32, u32)>>> {
    static KEPT: OnceLock<Mutex<HashMap<Key, Option<(u32, u32)>>>> = OnceLock::new();
    KEPT.get_or_init(Default::default)
}

const MOST_KEPT: usize = 50_000;

pub fn over(path: &Path) -> Over {
    over_on(path, if moving_one(path) { screen_biggest() } else { None })
}

pub fn over_on(path: &Path, screen: Option<(u32, u32)>) -> Over {
    let Some((mut over, len, when)) = by_size(path) else { return Over::default() };
    if over.any() {
        return over;
    }
    let key = (path.to_path_buf(), when, len);
    let film = moving_one(path);
    let found = kept().lock().ok().and_then(|k| k.get(&key).copied());
    let size = match found {
        Some(size) => size,
        None => {
            let size = if film { size_of_film(path) } else { size_of_still(path) };
            if let Ok(mut kept) = kept().lock() {
                if kept.len() >= MOST_KEPT {
                    kept.clear();
                }
                kept.insert(key, size);
            }
            size
        }
    };
    over.pixels = size.is_some_and(|(w, h)| !fits(w, h));
    over.film_bigger = film && size.is_some_and(|(w, h)| !fits_screen(w, h, screen));
    over
}

pub fn memory_notes(path: &Path, screen: Option<(u32, u32)>) -> Vec<&'static str> {
    let mut out = Vec::new();
    let film = moving_one(path);
    let size = if film { size_of_film(path) } else { size_of_still(path) };
    if let (Some((sw, sh)), Some((w, h))) = (screen, size) {
        if w > sw || h > sh {
            out.push(bigger_than_screen());
        }
    }
    if film {
        out.push(moving_costs());
    }
    out
}

#[cfg(test)]
pub fn over_quickly(path: &Path) -> Over {
    let Some((mut over, ..)) = by_size(path) else { return Over::default() };
    if !over.any() && !moving_one(path) {
        over.pixels = size_of_still(path).is_some_and(|(w, h)| !fits(w, h));
    }
    over
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png(dir: &Path, name: &str, w: u32, h: u32) -> std::path::PathBuf {
        let path = dir.join(name);
        image::RgbImage::new(w, h).save(&path).unwrap();
        path
    }

    #[test]
    fn four_k_is_the_limit_either_way_up() {
        assert!(fits(3840, 2160) && fits(2160, 3840) && fits(1, 1));
        assert!(!fits(3841, 2160) && !fits(3840, 2161) && !fits(2161, 3841) && !fits(7680, 4320));
    }

    #[test]
    fn a_wallpaper_is_over_by_its_picture_or_by_its_file() {
        let dir = std::env::temp_dir().join(format!("limits-a-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        assert!(!over(&png(&dir, "ok.png", 3840, 2160)).any());
        assert!(!over(&png(&dir, "tall.png", 2160, 3840)).any());
        let big = png(&dir, "big.png", 3841, 100);
        assert_eq!(over(&big), Over { pixels: true, ..Over::default() });
        assert_eq!(over_quickly(&big), over(&big));
        let heavy = dir.join("heavy.jpg");
        let file = std::fs::File::create(&heavy).unwrap();
        file.set_len(MOST_STILL_BYTES + 1).unwrap();
        assert_eq!(over(&heavy), Over { still_bytes: true, ..Over::default() });
        assert_eq!(over_quickly(&heavy), over(&heavy));
        let film = dir.join("heavy.webm");
        std::fs::File::create(&film).unwrap().set_len(MOST_STILL_BYTES + 1).unwrap();
        assert!(!over(&film).moving_bytes && !over(&film).still_bytes);
        std::fs::File::create(&film).unwrap().set_len(MOST_MOVING_BYTES + 1).unwrap();
        assert_eq!(over(&film), Over { moving_bytes: true, ..Over::default() });
        assert!(!over(&dir.join("gone.png")).any());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn the_label_says_how_many_and_why() {
        let mut left = Left::default();
        assert_eq!(left.words(), None);
        left.add(Over::default());
        assert_eq!(left.words(), None);
        left.add(Over { pixels: true, ..Over::default() });
        assert_eq!(left.words().as_deref(), Some("1 left out: larger than 4K."));
        left.add(Over { still_bytes: true, ..Over::default() });
        assert_eq!(left.words().as_deref(), Some("2 left out: larger than 4K or over 20 MB."));
        left.add(Over { moving_bytes: true, ..Over::default() });
        assert_eq!(left.words().as_deref(), Some("3 left out: larger than 4K or over 20 MB (200 MB if moving)."));
        assert_eq!(refusal(Over { moving_bytes: true, ..Over::default() }).as_deref(), Some("1 left out: over 200 MB."));
        assert_eq!(refusal(Over::default()), None);
        assert_eq!(refusal(Over { film_bigger: true, ..Over::default() }).as_deref(), Some("1 left out: larger than your screen."));
        assert_eq!(refusal(Over { film_bigger: true, pixels: true, ..Over::default() }).as_deref(), Some("1 left out: larger than 4K."));
    }

    #[test]
    fn a_film_is_capped_at_the_size_of_the_largest_screen() {
        let screen = Some((1920, 1080));
        assert!(fits_screen(1920, 1080, screen) && fits_screen(1280, 720, screen) && fits_screen(1080, 1920, screen));
        assert!(!fits_screen(1921, 1080, screen) && !fits_screen(3840, 2160, screen) && !fits_screen(1920, 1081, screen));
        assert!(fits_screen(3840, 2160, None));
        let dir = std::env::temp_dir().join(format!("limits-c-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let still = png(&dir, "s.png", 3000, 100);
        assert!(!over_on(&still, screen).any());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_bigger_picture_than_the_screen_and_a_moving_one_each_say_so() {
        let dir = std::env::temp_dir().join(format!("limits-b-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = png(&dir, "p.png", 3000, 100);
        assert_eq!(memory_notes(&p, Some((1920, 1080))), [bigger_than_screen()]);
        assert!(memory_notes(&p, Some((3840, 2160))).is_empty());
        assert!(memory_notes(&p, None).is_empty());
        let words = [bigger_than_screen(), moving_costs()];
        assert_eq!(words[0], "This picture is bigger than your screen and uses extra memory.");
        assert_eq!(words[1], "Moving wallpapers use more memory and battery.");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
