//! Wallpaper: everything that looks at pictures on the disk, always off the page's own thread, and the previews and small pictures made from them.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

use super::{NO_PICTURE, PREVIEW_LOOPS, PREVIEW_STEP, Play, STILL_WIDTH, Slot, TILE_STEP, Thumb, Wallpaper, every_choices, expand_user, or_default, screen_size, shown_name, typed_colour};
use crate::common::files::setting;
use crate::common::run::on_thread;
use crate::session::moving;
use crate::session::wallpaper::{default_picture, looked_in, Found, own_pictures, showing, skipped};
use crate::session::wallpaper_limits::{self as limits, Over};
use crate::settings::ui::{Kit};
use crate::toolkit::picture;
use crate::toolkit::t;
use tiny_skia::Pixmap;

impl Wallpaper {
    pub(super) fn refresh(&mut self, kit: &mut Kit) {
        self.look_again(kit, false);
    }

    pub(super) fn look_again(&mut self, kit: &mut Kit, counted: bool) {
        let raw = setting("wallpaper");
        self.now.plain = raw == NO_PICTURE;
        self.now.skip = skipped();
        self.now.every = setting("wallpaper_every").parse().unwrap_or(300);
        self.now.order = or_default(setting("wallpaper_order"), "shuffle");
        self.now.fit = or_default(setting("wallpaper_fit"), "fill");
        if self.colour.is_empty() || typed_colour(&self.colour).is_some() {
            self.colour = or_default(setting("wallpaper_color"), "#09090b");
        }
        self.looks += 1;
        let (turn, every, skip) = (self.looks, self.now.every, self.now.skip.clone());
        let back = kit.then::<Wallpaper, Seen>(move |me, k, seen| me.looked(k, turn, seen));
        on_thread(move || look(&raw, every, &skip, counted), back);
    }

    pub(super) fn looked(&mut self, kit: &mut Kit, turn: u64, seen: Seen) {
        if turn != self.looks || self.away {
            return;
        }
        let old_showing = std::mem::replace(&mut self.now.showing, seen.showing);
        self.now.src = seen.src;
        self.now.is_dir = seen.is_dir;
        self.now.rotating = seen.rotating;
        self.now.showing_there = seen.showing_there;
        self.now.showing_words = seen.showing_words;
        self.now.choices = seen.choices;
        self.now.said = seen.said;
        self.now.memory = seen.memory;
        self.want_thumbs(kit);
        if old_showing != self.now.showing {
            self.preview = None;
            let p = self.now.showing.clone();
            self.play(kit, Slot::Preview, &p, PREVIEW_STEP);
        }
    }

    fn want_thumbs(&mut self, kit: &mut Kit) {
        let mut wanted = Vec::new();
        if self.now.showing_there {
            wanted.push((self.now.showing.clone(), PREVIEW_STEP));
        }
        wanted.extend(self.now.choices.iter().map(|p| (p.clone(), TILE_STEP)));
        for key in wanted {
            if !self.thumbs.contains_key(&key) {
                self.thumbs.insert(key.clone(), Thumb::Waiting);
                self.queue.push_back(key);
            }
        }
        self.next_thumb(kit);
    }

    fn next_thumb(&mut self, kit: &mut Kit) {
        if self.decoding {
            return;
        }
        let Some((path, step)) = self.queue.pop_front() else { return };
        self.decoding = true;
        let key = (path.clone(), step);
        let back = kit.then::<Wallpaper, Option<Pixmap>>(move |me, k, px| {
            me.decoding = false;
            if me.away {
                return;
            }
            me.thumbs.insert(key, px.map_or(Thumb::Failed, |p| Thumb::Ready(Rc::new(p))));
            me.next_thumb(k);
        });
        on_thread(move || decode(&path, step), back);
    }

    pub(super) fn thumb(&self, path: &Path, step: usize) -> Option<Rc<Pixmap>> {
        match self.thumbs.get(&(path.to_path_buf(), step)) {
            Some(Thumb::Ready(p)) => Some(p.clone()),
            Some(Thumb::Waiting | Thumb::Failed) | None => None,
        }
    }

    fn slot(&mut self, which: Slot) -> &mut Option<Play> {
        match which {
            Slot::Preview => &mut self.preview,
            Slot::Tile => &mut self.tile,
        }
    }

    pub(super) fn play(&mut self, kit: &mut Kit, which: Slot, path: &Path, step: usize) {
        if !moving::is_moving(path) || !crate::common::graphics::moving() {
            return;
        }
        self.gen += 1;
        let gen = self.gen;
        *self.slot(which) = Some(Play { gen, path: path.to_path_buf(), step, film: None, left: 0, gap: Duration::ZERO, frame: None });
        let back = kit.then::<Wallpaper, Option<(Film, usize, Duration)>>(move |me, k, opened| me.opened(k, which, gen, opened));
        let path = path.to_path_buf();
        on_thread(move || open_film(&path, step), back);
    }

    pub(super) fn opened(&mut self, kit: &mut Kit, which: Slot, gen: u64, opened: Option<(Film, usize, Duration)>) {
        let slot = self.slot(which);
        let Some(p) = slot.as_mut().filter(|p| p.gen == gen) else { return };
        let Some((film, left, gap)) = opened else {
            *slot = None;
            return;
        };
        (p.film, p.left, p.gap) = (Some(film), left, gap);
        kit.after::<Wallpaper>(gap, move |me, k| me.advance(k, which, gen));
    }

    fn advance(&mut self, kit: &mut Kit, which: Slot, gen: u64) {
        let slot = self.slot(which);
        let Some(p) = slot.as_mut().filter(|p| p.gen == gen) else { return };
        if p.left == 0 {
            *slot = None;
            return;
        }

        let Some(film) = p.film.take() else { return };
        p.left -= 1;
        let (step, since) = (p.step, std::time::Instant::now());
        let back = kit.then::<Wallpaper, (Film, Option<Pixmap>)>(move |me, k, (film, px)| me.stepped(k, which, gen, film, px, since));
        on_thread(move || step_film(film, step), back);
    }

    fn stepped(&mut self, kit: &mut Kit, which: Slot, gen: u64, film: Film, px: Option<Pixmap>, since: std::time::Instant) {
        let Some(p) = self.slot(which).as_mut().filter(|p| p.gen == gen) else { return };
        p.film = Some(film);
        if let Some(px) = px {
            p.frame = Some(Rc::new(px));
        }

        let wait = p.gap.saturating_sub(since.elapsed());
        kit.after::<Wallpaper>(wait, move |me, k| me.advance(k, which, gen));
    }

    pub(super) fn frame(&self, which: Slot, path: &Path) -> Option<Rc<Pixmap>> {
        let p = match which {
            Slot::Preview => self.preview.as_ref(),
            Slot::Tile => self.tile.as_ref(),
        };
        p.filter(|p| p.path == path).and_then(|p| p.frame.clone())
    }

    pub(super) fn set_rotating(&mut self, kit: &mut Kit, on: bool) {
        let raw = setting("wallpaper");
        let back = kit.then::<Wallpaper, PathBuf>(|me, k, chosen| me.set(k, "wallpaper", &chosen.to_string_lossy(), None));
        on_thread(move || rotating_source(&raw, on), back);
    }

    pub(super) fn choose(&mut self, kit: &mut Kit, path: PathBuf) {
        let back = kit.then::<Wallpaper, (PathBuf, Over)>(|me, k, (path, over)| me.chosen(k, path, over));
        on_thread(move || vet(path), back);
    }

    pub(super) fn chosen(&mut self, kit: &mut Kit, path: PathBuf, over: Over) {
        match limits::refusal(over) {
            Some(why) => {
                kit.say("source", &why, false, false);
            }
            None => self.set(kit, "wallpaper", &path.to_string_lossy(), None),
        }
    }

}

fn decode(path: &Path, step: usize) -> Option<Pixmap> {
    if moving::is_moving(path) {
        if let Some(img) = moving::first_frame(path, step).and_then(|(w, h, rgb)| picture::from_rgb(w, h, rgb)) {
            return picture::to_pixmap(&img);
        }
    }
    let img = picture::load(path)?;
    let most = STILL_WIDTH / step as u32;
    let (w, h) = (img.width(), img.height());
    let img = if w > most && h > 0 { img.thumbnail(most, (most as u64 * h as u64 / w as u64).max(1) as u32) } else { img };
    picture::to_pixmap(&img)
}

pub(super) struct Film {
    player: moving::Player,
    buf: Vec<u8>,
    stride: usize,
}

fn open_film(path: &Path, step: usize) -> Option<(Film, usize, Duration)> {
    let player = moving::Player::open(path)?;
    let info = player.info();
    let (w, h) = (info.width as usize / step, info.height as usize / step);
    let gap = Duration::from_secs_f64(1.0 / info.fps.max(1.0));
    let left = player.frames() * PREVIEW_LOOPS;
    Some((Film { player, buf: vec![0u8; w * h * 4], stride: w * 4 }, left, gap))
}

fn step_film(mut film: Film, step: usize) -> (Film, Option<Pixmap>) {
    let px = next_frame(&mut film, step);
    (film, px)
}

fn next_frame(film: &mut Film, step: usize) -> Option<Pixmap> {
    let (fw, fh) = film.player.next_small(&mut film.buf, film.stride, step)?;
    let mut px = Pixmap::new(fw, fh)?;

    let row = fw as usize * 4;
    for (y, out) in px.data_mut().chunks_exact_mut(row).enumerate() {
        let src = film.buf.get(y * film.stride..y * film.stride + row)?;
        for (d, s) in out.chunks_exact_mut(4).zip(src.chunks_exact(4)) {
            d.copy_from_slice(&[s[2], s[1], s[0], 255]);
        }
    }
    Some(px)
}

pub(super) struct Seen {
    pub(super) src: PathBuf,
    pub(super) is_dir: bool,
    pub(super) rotating: bool,
    pub(super) showing: PathBuf,
    pub(super) showing_there: bool,
    pub(super) showing_words: String,
    pub(super) choices: Vec<PathBuf>,
    pub(super) said: String,
    pub(super) memory: Vec<&'static str>,
}

fn look(raw: &str, every: u64, skip: &HashSet<String>, counted: bool) -> Seen {
    let plain = raw == NO_PICTURE;
    let named = if raw.is_empty() { default_picture() } else { expand_user(raw) };
    let src = if plain { PathBuf::new() } else { named.clone() };
    let is_dir = src.is_dir();
    let rotating = named.is_dir();
    let is_used = |p: &PathBuf| !skip.contains(&p.file_name().unwrap_or_default().to_string_lossy().to_string());

    let found = is_dir.then(|| looked_in(&src));
    let used = found.as_ref().map_or(0, |f| f.pictures.iter().filter(|p| is_used(p)).count());
    let is_file = !is_dir && src.is_file();

    let refused = if is_file { limits::refusal(limits::over(&src)) } else { None };
    let named_now = Named { raw, plain, src: &src, found: found.as_ref(), is_file, refused: refused.as_ref(), used };
    let (showing, showing_words) = showing_words(&named_now, counted);
    let said = said_words(&named_now, every);
    let showing_there = showing.is_file();
    let memory = if showing_there { limits::memory_notes(&showing, screen_size()) } else { Vec::new() };

    let choices = match found {
        Some(found) => found.pictures,
        None => {
            let folder = src.parent().map(Path::to_path_buf).unwrap_or_default();
            if folder.is_dir() { looked_in(&folder).pictures } else { Vec::new() }
        }
    };
    Seen { src, is_dir, rotating, showing, showing_there, showing_words, choices, said, memory }
}

#[derive(Clone, Copy)]
struct Named<'a> {
    raw: &'a str,
    plain: bool,
    src: &'a Path,
    found: Option<&'a Found>,
    is_file: bool,
    refused: Option<&'a String>,
    used: usize,
}

fn showing_words(n: &Named, counted: bool) -> (PathBuf, String) {
    let Named { raw, plain, src, found, is_file, refused, used } = *n;
    if let Some(found) = found {
        let pics = &found.pictures;
        let words = if counted {
            t!("settings-wallpaper-count", used = used, all = pics.len())
        } else if pics.is_empty() {
            t!("settings-wallpaper-empty-folder", folder = src.display().to_string())
        } else if src == own_pictures() {
            t!("settings-wallpaper-count-own", used = used, all = pics.len())
        } else {
            t!("settings-wallpaper-count-folder", used = used, all = pics.len(), folder = src.display().to_string())
        };
        (pics.first().cloned().unwrap_or_default(), words)
    } else if is_file {
        let words = if raw.is_empty() {
            t!("settings-wallpaper-file-own", name = shown_name(src))
        } else {
            src.to_string_lossy().to_string()
        };
        (if refused.is_some() { default_picture() } else { src.to_path_buf() }, words)
    } else if plain {
        (PathBuf::new(), t!("settings-wallpaper-plain-below").to_string())
    } else {
        (PathBuf::new(), t!("settings-wallpaper-gone-below").to_string())
    }
}

fn said_words(n: &Named, every: u64) -> String {
    let Named { raw, plain, src, found, is_file, refused, used } = *n;
    if plain {
        t!("settings-wallpaper-plain").to_string()
    } else if let Some(found) = found {
        let all = found.pictures.len();
        let every = every_choices().iter().find(|(v, _)| *v == every).map(|(_, l)| *l).unwrap_or(t!("settings-wallpaper-every-5m"));
        if all == 0 {
            t!("settings-wallpaper-empty-folder-choose", folder = src.display().to_string())
        } else {

            let mut extra = String::new();
            if found.more {
                extra.push(' ');
                extra.push_str(t!("settings-wallpaper-more-than-looked-at"));
            }
            if let Some(w) = found.left.words() {
                extra.push(' ');
                extra.push_str(&w);
            }
            if src == own_pictures() {
                t!("settings-wallpaper-changing-own", every = every, used = used, all = all, extra = extra)
            } else {
                let folder = src.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                t!("settings-wallpaper-changing-folder", every = every, used = used, all = all, folder = folder, extra = extra)
            }
        }
    } else if is_file {
        let name = shown_name(src);
        let mut words = if raw.is_empty() {
            t!("settings-wallpaper-file-own-said", name = name)
        } else {
            let name = match name.split_once(' ') {
                Some(("The", rest)) => format!("the {rest}"),
                _ => name.clone(),
            };
            t!("settings-wallpaper-file-one", name = name)
        };

        if let Some(why) = refused {
            words = t!("settings-wallpaper-file-refused", why = why);
        }
        words
    } else {
        t!("settings-wallpaper-file-gone").to_string()
    }
}

fn vet(path: PathBuf) -> (PathBuf, Over) {
    let over = if path.is_dir() { Over::default() } else { limits::over(&path) };
    (path, over)
}

fn rotating_source(raw: &str, on: bool) -> PathBuf {
    let here = if raw.is_empty() { default_picture() } else { expand_user(raw) };
    if on {
        if here.is_dir() {
            here
        } else {
            let beside = here.parent().map(PathBuf::from).unwrap_or_default();
            if looked_in(&beside).pictures.len() > 1 { beside } else { own_pictures() }
        }
    } else if here.is_dir() {

        showing()
            .filter(|p| p.parent() == Some(here.as_path()))
            .or_else(|| looked_in(&here).pictures.into_iter().next())
            .unwrap_or_else(default_picture)
    } else {
        here
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::wallpaper_limits as limits;
    use crate::toolkit::event::Loop;

    #[test]
    fn a_look_gives_the_pictures_and_the_words_about_them() {
        let dir = std::env::temp_dir().join(format!("trindle-wallpaper-look-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let folder = dir.join("Holiday");
        std::fs::create_dir_all(&folder).unwrap();
        for name in ["a.png", "b.jpg", "c.png"] {
            std::fs::write(folder.join(name), b"").unwrap();
        }
        let skip: HashSet<String> = ["b.jpg".to_string()].into();
        let raw = folder.to_string_lossy().to_string();
        let seen = look(&raw, 60, &skip, false);
        assert!(seen.is_dir && seen.rotating && seen.src == folder);
        assert_eq!(seen.choices, [folder.join("a.png"), folder.join("b.jpg"), folder.join("c.png")]);
        assert_eq!((seen.showing.clone(), seen.showing_there), (folder.join("a.png"), true));
        assert_eq!(seen.said, "A new picture every 1 minute, from 2 of the 3 pictures in Holiday.");
        assert_eq!(seen.showing_words, format!("2 of 3 pictures from {}", folder.display()));
        assert_eq!(look(&raw, 60, &skip, true).showing_words, "2 of 3 pictures");

        let one = folder.join("c.png");
        let seen = look(&one.to_string_lossy(), 300, &skip, false);
        assert!(!seen.is_dir && !seen.rotating && seen.showing == one && seen.showing_there);
        assert_eq!(seen.choices.len(), 3);
        assert_eq!(seen.said, "One picture, C, all the time.");

        let plain = look(NO_PICTURE, 300, &skip, false);
        assert!(plain.choices.is_empty() && !plain.showing_there && plain.said == "A plain colour, with no picture.");
        assert_eq!(plain.showing_words, "A plain colour, set below.");
        let gone = look(&dir.join("gone.png").to_string_lossy(), 300, &skip, false);
        assert_eq!(gone.said, "The picture is not there any more. Choose a picture or a folder.");
        assert_eq!(gone.showing_words, "Nothing there any more. Choose a picture or a folder.");

        let empty = dir.join("Empty");
        std::fs::create_dir_all(&empty).unwrap();
        let seen = look(&empty.to_string_lossy(), 300, &skip, false);
        assert_eq!(seen.said, format!("The folder {} has no pictures in it. Choose another.", empty.display()));

        assert_eq!(rotating_source(&one.to_string_lossy(), true), folder);
        assert_eq!(rotating_source(&raw, true), folder);
        assert_eq!(rotating_source(&one.to_string_lossy(), false), one);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_picture_too_large_is_left_out_and_the_page_says_how_many_and_why() {
        let dir = std::env::temp_dir().join(format!("trindle-wallpaper-limits-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        image::RgbImage::new(64, 48).save(dir.join("a.png")).unwrap();
        let big = dir.join("b.png");
        image::RgbImage::new(3841, 16).save(&big).unwrap();
        let skip = HashSet::new();

        let seen = look(&dir.to_string_lossy(), 300, &skip, false);
        assert_eq!(seen.choices, [dir.join("a.png")]);
        assert_eq!(seen.said, format!("A new picture every 5 minutes, from 1 of the 1 picture in {}. 1 left out: larger than 4K.",
                                      dir.file_name().unwrap().to_string_lossy()));

        let seen = look(&big.to_string_lossy(), 300, &skip, false);
        assert_eq!(seen.said, "The picture is not used. 1 left out: larger than 4K. Trindle's own is shown.");
        assert_eq!(seen.showing, default_picture());
        assert_eq!(seen.choices, [dir.join("a.png")], "the others beside it are only those that fit");

        assert_eq!(vet(dir.clone()).1, Over::default(), "a folder is taken; its pictures are sorted when it is looked at");
        assert_eq!(vet(dir.join("a.png")).1, Over::default());
        let over = vet(big.clone()).1;
        assert!(over.pixels);
        let events = Loop::new();
        let asks = events.post(|_: crate::settings::Do| {});
        let mut kit = Kit::new(events.clone(), asks, None, None);
        let mut page = Wallpaper::made_up();
        page.chosen(&mut kit, big, over);
        assert_eq!(kit.said("source"), Some(("1 left out: larger than 4K.", true)));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_moving_wallpaper_is_given_its_note_about_memory() {
        let seen = look(&default_picture().to_string_lossy(), 300, &HashSet::new(), false);
        if seen.showing_there {
            assert!(seen.memory.contains(&limits::moving_costs()), "the moving mosaic carries no note about memory");
        }
    }
}
