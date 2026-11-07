//! The Store.

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
mod pages;
mod dropped;
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
const TICK: Duration = Duration::from_millis(250);

const PAGE: usize = 120;
const GITHUB_CHECKS: usize = 12;
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
    Show,
    Open(Vec<String>),
    Tick,
    Pulse,
    Fade(f32),
    Lit(f32),
    Settings(crate::settings::ui::Do),
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
    pending_github: Option<String>,
    preloaded: bool,
    stale: bool,
    #[cfg(test)]
    read_aheads: u32,
    pressed_at: Option<crate::toolkit::widget::Rect>,
    pending_app: Option<String>,
    pending_spot: Option<String>,
    spot_kept: Option<Instant>,
    spot_last: &'static str,
    fade: f32,
    fading: Option<Tween>,
    lit: f32,
    dials: HashMap<String, Dial>,
    said: HashMap<String, (String, Tone)>,
    said_on: HashMap<String, &'static str>,
    behind: HashMap<String, Vec<String>>,
    behind_open: BTreeSet<String>,
    top: Option<(String, Tone)>,
    headline: Option<(String, Tone)>,
    shown: usize,
    hub: Option<Rc<Vec<github::Found>>>,
    hub_asked: bool,
    show: formats::Show,
    warning_off: bool,
    conf: std::path::PathBuf,
    warn: Option<pages::appimage_warning::Warn>,
    browse: browse::Page,
    github: pages::GitHubPage,
    backup: pages::BackupPage,
    sources: pages::SourcesPage,
    plugins: plugin_page::PluginPage,
    reviews: HashMap<String, Option<reviews::Ratings>>,
    trouble: HashMap<String, String>,
    owned: pages::InstalledPage,
    reviews_heard: std::collections::HashSet<String>,
    said_about: HashMap<String, Vec<reviews::Review>>,
    shot: usize,
    across: f32,
    tall: f32,
    shots: HashMap<String, Option<Rc<image::DynamicImage>>>,
    shots_missed: std::collections::HashSet<String>,
    upd: update_page::UpdatesPage,
    facts: HashMap<String, Option<pages::install::AppFacts>>,
    facts_heard: std::collections::HashSet<String>,
    pulsing: Rc<Cell<bool>>,
    pulse_timer: Rc<Cell<bool>>,
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
            view: View::Home, detail: None, image: None, search: String::new(), pending_github: None,
            preloaded: false, stale: false,
            #[cfg(test)]
            read_aheads: 0,
            pressed_at: None, pending_app: None, pending_spot: None, spot_kept: None, spot_last: spot::SPOT,
            fade: 1.0, fading: None, lit: 0.0,
            dials: HashMap::new(), said: HashMap::new(), said_on: HashMap::new(), behind: HashMap::new(), behind_open: BTreeSet::new(),
            top: None, headline: None, shown: PAGE,
            hub: None, hub_asked: false,
            show: formats::show_in(&conf), warning_off: formats::warning_off_in(&conf), conf, warn: None,
            browse: Default::default(), github: Default::default(), backup: Default::default(), sources: Default::default(), plugins: Default::default(),
            reviews: HashMap::new(), trouble: HashMap::new(), owned: Default::default(), reviews_heard: Default::default(),
            said_about: HashMap::new(), shots: HashMap::new(), shots_missed: Default::default(), shot: 0,
            across: SIZE.0 as f32, tall: SIZE.1 as f32,
            upd: update_page::UpdatesPage::default(), facts: HashMap::new(), facts_heard: Default::default(),
            pulsing: Rc::default(), pulse_timer: Rc::default(),
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
