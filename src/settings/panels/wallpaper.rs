//! Wallpaper.

use looking::Film;
mod cards;
mod looking;

use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::Duration;

use tiny_skia::Pixmap;

use crate::common::files::{home, set_setting, setting};
use crate::common::run::spawn;
use crate::session::wallpaper_limits::{self as limits};
use crate::session::wallpaper::skipped;
use crate::settings::ui::{headline, page, Cause, Kit, Msg, Page, not_saved, no_chooser};
use crate::toolkit::bus::{Bus, Listen};
use crate::toolkit::chooser::{self, Ask};
use crate::toolkit::colour::Colour;
use crate::toolkit::event::Loop;
use crate::toolkit::widget::Node;
use crate::toolkit::t;

fn every_choices() -> [(u64, &'static str); 8] {
    [
        (10, t!("settings-wallpaper-every-10s")),
        (30, t!("settings-wallpaper-every-30s")),
        (60, t!("settings-wallpaper-every-1m")),
        (300, t!("settings-wallpaper-every-5m")),
        (900, t!("settings-wallpaper-every-15m")),
        (1800, t!("settings-wallpaper-every-30m")),
        (3600, t!("settings-wallpaper-every-1h")),
        (86400, t!("settings-wallpaper-every-1d")),
    ]
}
fn fits() -> [(&'static str, &'static str); 5] {
    [
        ("fill", t!("settings-wallpaper-fit-fill")),
        ("fit", t!("settings-wallpaper-fit-fit")),
        ("center", t!("settings-wallpaper-fit-center")),
        ("stretch", t!("settings-wallpaper-fit-stretch")),
        ("tile", t!("settings-wallpaper-fit-tile")),
    ]
}
fn orders() -> [(&'static str, &'static str); 2] {
    [("shuffle", t!("settings-wallpaper-order-shuffle")), ("name", t!("settings-wallpaper-order-name"))]
}

const PICTURES: &str = "wallpaper:pictures";
pub(crate) const NEXT: &str = "wallpaper:next";

fn no_changer() -> &'static str {
    t!("settings-wallpaper-no-changer")
}

fn tell_changer(next_picture: bool) -> bool {
    if next_picture {
        spawn(&["trindle-wallpaper", "--next"])
    } else {
        spawn(&["trindle-wallpaper"])
    }
}

fn expand_user(p: &str) -> PathBuf {
    match p.strip_prefix('~') {
        Some(rest) if rest.is_empty() || rest.starts_with('/') => home().join(rest.trim_start_matches('/')),
        _ => PathBuf::from(p),
    }
}

pub const NO_PICTURE: &str = "none";

const STILL_WIDTH: u32 = 1920;

const PREVIEW_STEP: usize = 2;
const TILE_STEP: usize = 4;

const PREVIEW_LOOPS: usize = 5;

fn screen_size() -> Option<(u32, u32)> {
    limits::screen_biggest()
}

fn shown_name(path: &Path) -> String {
    let stem = path.file_stem().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let dated = stem.len() > 8 && stem.as_bytes()[4] == b'-' && stem.as_bytes()[7] == b'-'
        && stem[..4].bytes().chain(stem[5..7].bytes()).all(|b| b.is_ascii_digit());
    let name = if dated { &stem[8..] } else { &stem[..] }.replace(['-', '_'], " ");
    let mut c = name.chars();
    c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
}

fn or_default(value: String, fallback: &str) -> String {
    if value.is_empty() { fallback.to_string() } else { value }
}

fn typed_colour(code: &str) -> Option<Colour> {
    if code.starts_with('#') && (code.len() == 7 || code.len() == 4) { Colour::parse(code) } else { None }
}

pub(crate) fn pick_folder(bus: &Bus, events: &Loop, title: &str, folder: &Path, done: impl FnOnce(Option<PathBuf>) + 'static) -> Option<Listen> {
    let ask = Ask { title: title.to_string(), folder: Some(folder.to_path_buf()), ..Ask::default() };
    chooser::open_folder(bus, events, ask, done)
}

fn pictures_folder() -> PathBuf {
    home().join("Pictures")
}

#[derive(Default)]
struct Now {
    src: PathBuf,
    plain: bool,
    is_dir: bool,
    rotating: bool,
    showing: PathBuf,
    showing_there: bool,
    showing_words: String,
    choices: Vec<PathBuf>,
    skip: HashSet<String>,
    said: String,
    memory: Vec<&'static str>,
    every: u64,
    order: String,
    fit: String,
}

enum Thumb {
    Waiting,
    Ready(Rc<Pixmap>),
    Failed,
}

#[derive(Clone, Copy, PartialEq)]
enum Slot {
    Preview,
    Tile,
}

struct Play {
    gen: u64,
    path: PathBuf,
    step: usize,
    film: Option<Film>,
    left: usize,
    gap: Duration,
    frame: Option<Rc<Pixmap>>,
}

#[derive(Default)]
pub struct Wallpaper {
    now: Now,
    colour: String,
    thumbs: HashMap<(PathBuf, usize), Thumb>,
    queue: VecDeque<(PathBuf, usize)>,
    decoding: bool,
    away: bool,
    preview: Option<Play>,
    tile: Option<Play>,
    gen: u64,
    looks: u64,
    asking: Option<Listen>,
}

impl Wallpaper {

    fn set(&mut self, kit: &mut Kit, key: &str, value: &str, line: Option<&str>) {
        let result = set_setting(key, value).map_err(|e| e.words(not_saved())).and_then(|()| match tell_changer(false) {
            true => Ok(()),
            false => Err(no_changer().to_string()),
        });
        match (line, &result) {
            (Some(k), Ok(())) => kit.say(k, t!("settings-wallpaper-saved"), true, false),
            (Some(k), Err(e)) => kit.say(k, e, false, false),
            (None, Ok(())) => {}
            (None, Err(e)) => kit.say("source", e, false, false),
        }
        self.refresh(kit);
    }

    fn set_wallpaper(&mut self, kit: &mut Kit, on: bool) {
        let raw = setting("wallpaper");
        if on {
            let before = setting("wallpaper_before");
            let back = if before == NO_PICTURE { String::new() } else { before };
            self.set(kit, "wallpaper", &back, Some("switch"));
        } else if raw != NO_PICTURE {
            if kit.settle_quietly("switch", set_setting("wallpaper_before", &raw), not_saved()).is_none() {
                self.refresh(kit);
                return;
            }
            self.set(kit, "wallpaper", NO_PICTURE, Some("switch"));
        }
    }

    fn use_picture(&mut self, kit: &mut Kit, name: &str, used: bool) {
        let mut skip = skipped();
        if used {
            skip.remove(name);
        } else {
            skip.insert(name.to_string());
        }
        let mut list: Vec<String> = skip.into_iter().collect();
        list.sort();
        if kit.settle_quietly(PICTURES, set_setting("wallpaper_skip", &list.join("\t")), not_saved()).is_some() {
            let told = tell_changer(false);
            kit.started(PICTURES, told, no_changer());
        }
        self.look_again(kit, true);
    }

    fn press_tile(&mut self, kit: &mut Kit, path: &Path) {
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        if self.now.rotating {
            let used = self.now.skip.contains(&name);
            self.use_picture(kit, &name, used);
        } else if path != self.now.src {
            self.choose(kit, path.to_path_buf());
        }
    }

    fn pick_file(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.session_for("source") else { return };
        let globs = ["*.jpg", "*.jpeg", "*.png", "*.webp", "*.gif", "*.bmp", "*.tif", "*.tiff", "*.avif", "*.heic",
                     "*.webm", "*.mp4", "*.mkv"];
        let ask = Ask {
            title: t!("settings-wallpaper-chooser-title").into(),
            folder: Some(pictures_folder()),
            name: None,
            filters: vec![(t!("settings-wallpaper-chooser-filter").into(), globs.iter().map(|g| g.to_string()).collect())],
        };
        let back = kit.then::<Wallpaper, Option<PathBuf>>(|me, k, path| me.picked(k, path));
        self.asking = chooser::open(&bus, &kit.events, ask, back);
        kit.started("source", self.asking.is_some(), no_chooser());
    }

    fn pick_folder(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.session_for("source") else { return };
        let back = kit.then::<Wallpaper, Option<PathBuf>>(|me, k, path| me.picked(k, path));
        self.asking = pick_folder(&bus, &kit.events, t!("settings-wallpaper-chooser-folder"), &pictures_folder(), back);
        kit.started("source", self.asking.is_some(), no_chooser());
    }

    fn picked(&mut self, kit: &mut Kit, path: Option<PathBuf>) {
        self.asking = None;
        match path {
            Some(path) => self.choose(kit, path),
            None => kit.unanswered("source"),
        }
    }

    fn typed(&mut self, kit: &mut Kit, code: String) {
        let save = typed_colour(&code).is_some() && code != setting("wallpaper_color");
        self.colour = code.clone();
        if save {
            self.set(kit, "wallpaper_color", &code, Some("colour"));
        }
    }

}

impl Page for Wallpaper {
    fn id(&self) -> &'static str {
        "wallpaper"
    }
    fn title(&self) -> &'static str {
        t!("settings-wallpaper-title")
    }
    fn icon(&self) -> &'static str {
        "preferences-desktop-wallpaper-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-wallpaper-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-wallpaper-help")
    }

    fn shown(&mut self, kit: &mut Kit) {
        self.away = false;
        self.now.showing = PathBuf::new();
        self.refresh(kit);
    }

    fn hidden(&mut self, _kit: &mut Kit) {
        self.away = true;
        self.gen += 1;
        self.looks += 1;
        self.preview = None;
        self.tile = None;
        self.queue.clear();
        self.thumbs.clear();
        self.asking = None;
    }

    fn conf_changed(&mut self, kit: &mut Kit) {
        if !self.away {
            self.refresh(kit);
        }
    }

    fn view(&self, kit: &Kit) -> Node<Msg> {
        let shade = typed_colour(&self.colour);
        let items = vec![
            headline(kit, self.now.said.clone()),
            self.picture_card(shade),
            self.wallpaper_card(kit, shade),
            self.changing_card(kit),
            self.pictures_card(kit),
        ];
        page(kit, self.title(), self.help(), items)
    }
}

#[cfg(test)]
impl Wallpaper {
    pub(crate) fn flipped() -> Vec<Wallpaper> {
        let mut plain = Wallpaper::made_up();
        plain.now.plain = true;
        let mut one = Wallpaper::made_up();
        one.now.rotating = false;
        one.now.choices.clear();
        let mut fitted = Wallpaper::made_up();
        fitted.now.fit = "fit".into();
        vec![Wallpaper::made_up(), plain, one, fitted]
    }

    pub(crate) fn with_memory() -> Wallpaper {
        let mut page = Wallpaper::made_up();
        page.now.memory = vec![limits::bigger_than_screen(), limits::moving_costs()];
        page
    }

    pub(crate) fn made_up() -> Wallpaper {
        let folder = PathBuf::from("/made-up/Pictures");
        let choices: Vec<PathBuf> = (1..=8).map(|n| folder.join(format!("holiday-{n}.jpg"))).collect();
        let now = Now { src: folder, plain: false, is_dir: true, rotating: true, showing: choices[0].clone(), showing_there: false,
                        showing_words: "8 of 8 pictures from /made-up/Pictures".into(), choices, skip: HashSet::new(),
                        said: "The picture changes every 5 minutes, from your own folder.".into(), every: 300,
                        order: "shuffle".into(), fit: "fill".into(), memory: Vec::new() };
        Wallpaper { now, colour: "#09090b".into(), ..Wallpaper::default() }
    }
}

#[cfg(test)]
mod tests;
