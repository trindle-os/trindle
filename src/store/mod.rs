//! The Store: Flathub's apps, updates of every kind, Trindle's plugins,
//! apps from GitHub, makers' own Debian sources and moving to another
//! computer, drawn by the Trindle Toolkit.

pub mod backup;
pub mod catalogue;
pub mod deb;
pub mod boxed;
pub mod loose;
pub mod downloaded;
pub mod fence;
pub mod fence_broker;
pub mod fence_net;
pub mod home_net;
pub mod usb_devices;
pub mod accessibility;
mod fence_window;
pub mod firmware;
pub mod help;
pub mod flat;
pub mod link;
pub mod github;
pub mod hub;
mod age;
mod came_from;
mod browse;
mod formats;
pub mod key;
pub mod progress;
mod put_back;
mod refusal;
pub mod reviews;
mod steam;
pub mod updates;
pub mod parts;
pub mod secctx;
pub mod pulse_guard;
pub mod copied_guard;
pub mod patch;
pub mod asking;
pub mod origin;
mod look;
pub(crate) mod listing;
mod app_page;
mod heard;
mod pages;
mod dropped;
mod flatpak_peek;
mod screen;
mod update_page;
mod plugin_page;
#[cfg(test)]
#[path = "shots_test.rs"]
mod shots;
#[cfg(test)]
mod shared_clipboard_test;
#[cfg(test)]
mod lang_test;
mod answers;
mod cli;
mod draw;
mod launch;
mod nav;
mod opts;
mod reading;
mod reason;
mod row_words;
mod side;
mod spot;

pub use launch::main;
pub(crate) use side::with_rules;

use std::cell::Cell;
use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::common::filling::Dial;
use crate::common::run;
use crate::toolkit::bus::{Bus, Listen};
use crate::toolkit::event::{Loop, Post};
use crate::toolkit::motion::Tween;
use crate::toolkit::ui::Shown;
use crate::toolkit::widget::Tone;
use crate::toolkit::window::Display;
use catalogue::{App, Catalogue};

const NAME: &str = "org.trindle.Store";
const PATH: &str = "/org/trindle/Store";
const XML: &str = r#"<interface name="org.trindle.Store"><method name="Show"/><method name="Open"><arg type="as" direction="in"/></method></interface>"#;
const SIZE: (u32, u32) = (1100, 760);

const TICK: Duration = crate::common::filling::TURN;

const PAGE: usize = 120;

const GITHUB_CHECKS: usize = 5;

const FADE_MS: u64 = 180;

#[derive(Clone, PartialEq)]
enum View {
    Home,
    Installed,
    Updates,
    Plugins,
    Browse,
    GitHub,
    Backup,
    Sources,
    Category(String),
    Search(String),

    Hosted(String),
}

impl View {
    fn nav_key(&self) -> String {
        match self {
            View::Home => "home".into(),
            View::Installed => "installed".into(),
            View::Updates => "updates".into(),
            View::Plugins => "plugins".into(),
            View::Browse => "browse".into(),
            View::GitHub => "github".into(),
            View::Backup => "backup".into(),
            View::Sources => "sources".into(),
            View::Category(c) => format!("cat:{c}"),
            View::Search(_) => String::new(),
            View::Hosted(id) => format!("set:{id}"),
        }
    }
}

#[derive(Clone)]
pub struct Msg(Rc<dyn Fn(&mut Store)>);

fn go(f: impl Fn(&mut Store) + 'static) -> Msg {
    Msg(Rc::new(f))
}

enum Do {

    Then(Box<dyn FnOnce(&mut Store)>),

    ThenIf(Box<dyn FnOnce(&mut Store) -> bool>),
    Show,

    Open(Vec<String>),

    Tick,

    Pulse,

    Fade(f32),

    Lit(f32),

    Settings(crate::settings::ui::Do),
}

#[derive(Default)]
struct ReadAhead {
    done: bool,
    stale: bool,

    #[cfg(test)]
    count: u32,
}

#[derive(Default)]
struct Opening {
    app: Option<String>,
    github: Option<String>,
}

struct Spot {

    asked: Option<String>,

    kept: Option<Instant>,

    last: &'static str,
}

struct FadeIn {

    shown: f32,
    tween: Option<Tween>,

    lit: f32,
}

#[derive(Default)]
struct RowWords {

    said: HashMap<String, (String, Tone)>,

    on: HashMap<String, &'static str>,

    behind: HashMap<String, Vec<String>>,
    open: BTreeSet<String>,
}

#[derive(Default)]
struct Hub {

    list: Option<Rc<Vec<github::Found>>>,

    asked: bool,
}

struct Prefs {
    show: formats::Show,
    warning_off: bool,
    conf: std::path::PathBuf,
}

#[derive(Default)]
struct Answers {

    facts: heard::Heard<pages::install::AppFacts>,
    ratings: heard::Heard<reviews::Ratings>,

    reviews: HashMap<String, Vec<reviews::Review>>,

    shots: HashMap<String, Option<Rc<image::DynamicImage>>>,

    missed: std::collections::HashSet<String>,
}

struct Size {

    across: f32,

    tall: f32,
}

#[derive(Default)]
struct Pulse {
    on: Rc<Cell<bool>>,
    timer: Rc<Cell<bool>>,
}

pub struct Store {
    events: Loop,
    asks: Post<Do>,

    display: Option<Rc<Display>>,

    other: Option<crate::common::updating::Who>,

    debs: Vec<Rc<Shown<deb::Package>>>,
    apps: Rc<Catalogue>,

    loading: bool,

    failed: Option<String>,
    online: bool,
    installed: HashMap<String, (String, String)>,
    updatable: Vec<String>,
    trindle_waiting: Option<String>,
    debian: Vec<String>,
    firmware: Vec<firmware::Firmware>,

    held: crate::common::held::Held,

    firmware_fetched: bool,

    lists_fetched: bool,

    busy: HashMap<String, &'static str>,

    jobs: HashMap<String, run::Proc>,

    working: u32,
    hidden: bool,
    view: View,

    detail: Option<App>,

    image: Option<github::Found>,
    search: String,

    ahead: ReadAhead,

    pressed_at: Option<crate::toolkit::widget::Rect>,

    opening: Opening,

    spot: Spot,

    fade_in: FadeIn,

    dials: HashMap<String, Dial>,

    turning: crate::common::filling::Turning,

    words: RowWords,

    top: Option<(String, Tone)>,

    headline: Option<(String, Tone)>,

    shown: usize,

    appimagehub: Hub,

    prefs: Prefs,

    warn: Option<pages::appimage_warning::Warn>,

    flatpaks: Vec<pages::flatpak_page::Page>,

    browse: browse::Page,

    github: pages::GitHubPage,

    backup: pages::BackupPage,

    sources: pages::SourcesPage,

    plugins: plugin_page::PluginPage,

    trouble: HashMap<String, String>,
    owned: pages::InstalledPage,

    heard: Answers,

    shot: usize,

    size: Size,

    upd: update_page::UpdatesPage,

    pulse: Pulse,

    bus: Option<Bus>,
    chooser: Option<Listen>,

    hosted: Option<crate::settings::Hosted>,
}

impl Store {
    fn new(events: &Loop, asks: Post<Do>, display: Option<Rc<Display>>) -> Store {
        let conf = crate::common::files::trindle_conf();
        Store {
            events: events.clone(), asks, display, other: None, debs: Vec::new(),
            apps: Rc::new(Catalogue::new()), loading: false, failed: None, online: crate::common::online(),
            installed: HashMap::new(), updatable: Vec::new(), trindle_waiting: None, debian: Vec::new(), firmware: Vec::new(),
            held: crate::common::held::Held::read(), firmware_fetched: false, lists_fetched: false,
            busy: HashMap::new(), jobs: HashMap::new(), working: 0, hidden: false,
            view: View::Home, detail: None, image: None, search: String::new(), ahead: ReadAhead::default(),
            pressed_at: None, opening: Opening::default(), spot: Spot { asked: None, kept: None, last: spot::SPOT },
            fade_in: FadeIn { shown: 1.0, tween: None, lit: 0.0 },
            dials: HashMap::new(), turning: Default::default(), words: RowWords::default(),
            top: None, headline: None, shown: PAGE, appimagehub: Hub::default(),
            prefs: Prefs { show: formats::show_in(&conf), warning_off: formats::warning_off_in(&conf), conf }, warn: None, flatpaks: Vec::new(),
            browse: Default::default(), github: Default::default(), backup: Default::default(), sources: Default::default(), plugins: Default::default(),
            trouble: HashMap::new(), owned: Default::default(), heard: Answers::default(), shot: 0,
            size: Size { across: SIZE.0 as f32, tall: SIZE.1 as f32 },
            upd: update_page::UpdatesPage::default(), pulse: Pulse::default(),
            bus: None, chooser: None, hosted: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::thousands;

    #[test]
    fn nothing_in_the_apps_window_touches_its_neighbour() {
        use crate::toolkit::{theme::Theme, ui::Ui};
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: super::Do| {});
        let s = super::Store::new(&events, asks, None);
        let mut wrong = Vec::new();
        for w in [760, 1278, 1920] {
            let mut ui = Ui::new(crate::toolkit::ui::App::view(&s), Theme::new(true, "#fbc711"));
            ui.lay_at(w, 692);
            wrong.extend(ui.squeezed_or_touching().into_iter().map(|found| format!("at {w}: {found}")));
        }
        assert!(wrong.is_empty(), "{} things touch or are squeezed:\n{}", wrong.len(), wrong.join("\n"));
    }

    #[test]
    fn figures_in_thousands() {
        assert_eq!(thousands(7), "7");
        assert_eq!(thousands(999), "999");
        assert_eq!(thousands(3245), "3,245");
        assert_eq!(thousands(1234567), "1,234,567");
    }
}
