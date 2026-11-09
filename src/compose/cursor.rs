//! Origin: began as a file of anvil, Smithay's example compositor (MIT, Victor Berger and
//! Victoria Brekenfeld). About 17% of this file's lines still read as
//! they did there; the rest is Trindle's own.
//! The fallback pointer picture (resources/cursor.png) is anvil's cursor.rgba saved as a PNG.

use std::{collections::HashMap, path::PathBuf, time::Duration};

use smithay::input::pointer::CursorIcon;
use tracing::warn;
use xcursor::parser::{parse_xcursor, Image};

static FALLBACK_CURSOR_PNG: &[u8] = include_bytes!("resources/cursor.png");

fn fallback_pixels() -> Vec<u8> {
    image::load_from_memory_with_format(FALLBACK_CURSOR_PNG, image::ImageFormat::Png)
        .map(|picture| picture.to_rgba8().into_raw())
        .unwrap_or_else(|_| vec![0; 64 * 64 * 4])
}

pub struct Cursor {

    name: String,

    paths: Vec<PathBuf>,

    shapes: HashMap<CursorIcon, Vec<Image>>,
    size: u32,
}

impl Cursor {

    fn asked() -> (String, u32) {
        let name = std::env::var("XCURSOR_THEME")
            .ok()
            .unwrap_or_else(|| "default".into());
        let size = std::env::var("XCURSOR_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(24);
        (name, size)
    }

    pub fn stale(&self) -> bool {
        Cursor::asked() != (self.name.clone(), self.size)
    }

    pub fn load() -> Cursor {
        let (name, size) = Cursor::asked();
        Cursor::load_from(name, size, search_paths())
    }

    fn load_from(name: String, size: u32, paths: Vec<PathBuf>) -> Cursor {
        let mut cursor = Cursor { name, paths, shapes: HashMap::new(), size };
        let icons = cursor.read(&["default"]).unwrap_or_else(|| {
            warn!("Unable to load xcursor, using fallback cursor");
            vec![Image {
                size: 32,
                width: 64,
                height: 64,
                xhot: 1,
                yhot: 1,
                delay: 1,
                pixels_rgba: fallback_pixels(),
                pixels_argb: vec![],
            }]
        });
        cursor.shapes.insert(CursorIcon::Default, icons);
        cursor
    }

    fn read(&mut self, names: &[&str]) -> Option<Vec<Image>> {
        let (set, paths) = (self.name.clone(), self.paths.clone());
        let names: Vec<String> = names.iter().map(|n| n.to_string()).collect();
        let wanted = names.clone();
        if let Some(found) = crate::files::gently_over(&self.paths, crate::files::START_WAIT, move || first_of(&set, &wanted, &paths)) {
            return found;
        }
        warn!("a folder of pointer sets did not answer; only those on this computer are used");
        self.paths.retain(|p| crate::files::here(p));
        let (set, paths) = (self.name.clone(), self.paths.clone());
        crate::files::gently_over(&self.paths, crate::files::START_WAIT, move || first_of(&set, &names, &paths)).flatten()
    }

    pub fn get_image(&mut self, scale: u32, time: Duration) -> Image {
        self.get_named(CursorIcon::Default, scale, time)
    }

    pub fn get_named(&mut self, icon: CursorIcon, scale: u32, time: Duration) -> Image {
        if !self.shapes.contains_key(&icon) {
            let names: Vec<&str> = std::iter::once(icon.name()).chain(icon.alt_names().iter().copied()).collect();
            if let Some(images) = self.read(&names) {
                self.shapes.entry(icon).or_insert(images);
            }
        }

        let images = &self.shapes[if self.shapes.contains_key(&icon) { &icon } else { &CursorIcon::Default }];
        frame(time.as_millis() as u32, self.size * scale, images)
    }
}

pub fn picture_scale(screen_scale: f64) -> u32 {
    if !screen_scale.is_finite() {
        return 1;
    }
    (screen_scale - 1e-6).ceil().clamp(1.0, 4.0) as u32
}

pub fn hotspot_points(xhot: u32, yhot: u32, scale: u32) -> smithay::utils::Point<i32, smithay::utils::Logical> {
    let scale = scale.max(1);
    ((xhot / scale) as i32, (yhot / scale) as i32).into()
}

fn search_paths() -> Vec<PathBuf> {
    let var = |name: &str| std::env::var(name).ok().filter(|v| !v.is_empty());
    let home = var("HOME").map(PathBuf::from);
    let expand = |entry: &str| -> Option<PathBuf> {
        match entry.strip_prefix("~/") {
            Some(rest) => home.as_ref().map(|h| h.join(rest)),
            None if entry == "~" => home.clone(),
            None => Some(PathBuf::from(entry)),
        }
    };
    if let Some(listed) = var("XCURSOR_PATH") {
        return listed.split(':').filter(|e| !e.is_empty()).filter_map(expand).collect();
    }
    let mut paths = Vec::new();
    match var("XDG_DATA_HOME") {
        Some(data) => paths.extend(expand(&data).map(|d| d.join("icons"))),
        None => paths.extend(home.as_ref().map(|h| h.join(".local/share/icons"))),
    }
    paths.extend(home.as_ref().map(|h| h.join(".icons")));
    match var("XDG_DATA_DIRS") {
        Some(dirs) => paths.extend(dirs.split(':').filter(|e| !e.is_empty()).filter_map(expand).map(|d| d.join("icons"))),
        None => paths.extend([PathBuf::from("/usr/local/share/icons"), PathBuf::from("/usr/share/icons")]),
    }
    paths.push(PathBuf::from("/usr/share/pixmaps"));
    paths.extend(home.as_ref().map(|h| h.join(".cursors")));
    paths.push(PathBuf::from("/usr/share/cursors/xorg-x11"));
    paths
}

fn first_of(set: &str, names: &[String], paths: &[PathBuf]) -> Option<Vec<Image>> {
    names.iter().filter_map(|n| file_of(set, n, paths, &mut Vec::new()))
        .filter_map(|f| crate::files::read_file(f).ok().and_then(|bytes| parse_xcursor(&bytes)))
        .find(|images| !images.is_empty())
}

fn file_of(set: &str, name: &str, paths: &[PathBuf], walked: &mut Vec<String>) -> Option<PathBuf> {
    let found: Vec<PathBuf> = paths.iter().map(|p| p.join(set)).filter(|d| d.is_dir()).collect();
    if let Some(file) = found.iter().map(|d| d.join("cursors").join(name)).find(|f| f.is_file()) {
        return Some(file);
    }
    walked.push(set.to_string());
    for dir in &found {
        let from = crate::files::read_string(dir.join("index.theme")).ok().and_then(|text| inherits(&text))
            .or_else(|| (set != "default").then(|| "default".to_string()));
        let Some(from) = from.filter(|f| !walked.contains(f)) else { continue };
        if let Some(file) = file_of(&from, name, paths, walked) {
            return Some(file);
        }
    }
    None
}

fn inherits(index: &str) -> Option<String> {
    let apart = |c: char| c.is_whitespace() || c == ';' || c == ',';
    index.lines().filter_map(|line| line.strip_prefix("Inherits")?.trim_start().strip_prefix('='))
        .map(|rest| rest.trim_start_matches(apart).split(apart).next().unwrap_or("").to_string())
        .find(|set| !set.is_empty())
}

fn nearest_images(size: u32, images: &[Image]) -> impl Iterator<Item = &Image> {

    let nearest_image = images
        .iter()
        .min_by_key(|image| (size as i32 - image.size as i32).abs())
        .unwrap();

    images
        .iter()
        .filter(move |image| image.width == nearest_image.width && image.height == nearest_image.height)
}

fn frame(mut millis: u32, size: u32, images: &[Image]) -> Image {
    let total = nearest_images(size, images).fold(0, |acc, image| acc + image.delay);
    if total == 0 {
        return nearest_images(size, images).next().unwrap().clone();
    }
    millis %= total;

    for img in nearest_images(size, images) {
        if millis < img.delay {
            return img.clone();
        }
        millis -= img.delay;
    }

    unreachable!()
}

#[cfg(test)]
mod tests {

    #[test]
    fn the_pointer_picture_follows_the_screen_scale() {
        assert_eq!(super::picture_scale(1.0), 1);
        assert_eq!(super::picture_scale(1.25), 2);
        assert_eq!(super::picture_scale(1.5), 2);
        assert_eq!(super::picture_scale(2.0), 2);
        assert_eq!(super::picture_scale(0.5), 1);
        assert_eq!(super::picture_scale(f64::NAN), 1);
        assert_eq!(super::picture_scale(9.0), 4);
        assert_eq!(super::hotspot_points(20, 10, 2), (10, 5).into());
        let big = |size: u32| xcursor::parser::Image {
            size, width: size, height: size, xhot: 0, yhot: 0, delay: 0, pixels_rgba: vec![0; (size * size * 4) as usize], pixels_argb: vec![],
        };
        assert_eq!(super::frame(0, 24 * super::picture_scale(1.5), &[big(24), big(48)]).width, 48);
        assert_eq!(super::frame(0, 24 * super::picture_scale(1.0), &[big(24), big(48)]).width, 24);
    }

    #[test]
    fn the_fallback_arrow_is_64_by_64_and_not_blank() {
        let pixels = super::fallback_pixels();
        assert_eq!(pixels.len(), 64 * 64 * 4);
        assert!(pixels.iter().any(|&b| b != 0));
    }

    fn one_pixel() -> Vec<u8> {
        let words: [u32; 17] = [0x7275_6358, 16, 0x1_0000, 1, 0xfffd_0002, 24, 28, 36, 0xfffd_0002, 24, 1, 1, 1, 0, 0, 0, 0xff00_00ff];
        words.iter().flat_map(|w| w.to_le_bytes()).collect()
    }

    #[test]
    fn a_pointer_set_is_found_through_what_it_takes_from_and_never_round_for_ever() {
        use std::path::PathBuf;
        let room = std::env::temp_dir().join(format!("trindle-cursor-sets-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&room);
        let (a, b) = (room.join("a"), room.join("b"));
        for dir in [a.join("far-set"), b.join("near-set/cursors"), a.join("round"), b.join("about")] {
            std::fs::create_dir_all(dir).unwrap();
        }
        std::fs::write(a.join("far-set/index.theme"), "[Icon Theme]\nInherits = ,near-set;other\n").unwrap();
        std::fs::write(b.join("near-set/cursors/default"), one_pixel()).unwrap();
        std::fs::write(a.join("round/index.theme"), "Inherits=about\n").unwrap();
        std::fs::write(b.join("about/index.theme"), "Inherits=round\n").unwrap();
        let paths = vec![a.clone(), b.clone()];
        let names = |n: &[&str]| n.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let found = super::first_of("far-set", &names(&["no-such-shape", "default"]), &paths).expect("the set it takes from was not looked in");
        assert_eq!((found.len(), found[0].width), (1, 1));
        assert!(super::first_of("round", &names(&["default"]), &paths).is_none());
        assert_eq!(super::inherits("x\nInherits= ; ,Adwaita, y\n").as_deref(), Some("Adwaita"));
        let plain = super::Cursor::load_from("round".into(), 24, vec![PathBuf::from(&a)]);
        assert_eq!(plain.shapes[&smithay::input::pointer::CursorIcon::Default][0].width, 64, "the plain arrow was not drawn");
        let mut set = super::Cursor::load_from("far-set".into(), 24, paths);
        assert_eq!(set.get_image(1, std::time::Duration::ZERO).width, 1);
        std::fs::remove_dir_all(&room).unwrap();
    }
}
