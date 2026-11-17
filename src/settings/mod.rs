//! trindle-settings: a sidebar of pages with search, drawn by the Trindle
//! Toolkit. `--panel <name>` opens at one page; a second call reuses the
//! open window.

mod command_line;
#[cfg(test)]
mod dropped;
pub mod help_topics;
pub mod pad_picture;
pub mod panels;
pub mod pictures;
pub mod session_env;
pub mod ui;
mod window;
pub mod wordsheet;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::toolkit::bus::Bus;
use crate::toolkit::event::{Loop, Post};
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::ui::App as Shows;
use crate::toolkit::widget::{card, column, side_heading, entry, item, list, note, row, scroll, space, Length, Node};
use ui::{Do, Kit, Msg, Page};

const NAME: &str = "org.trindle.Settings";
const PATH: &str = "/org/trindle/Settings";
const XML: &str = r#"<interface name="org.trindle.Settings"><method name="Show"/><method name="Open"><arg type="as" direction="in"/></method></interface>"#;
const SIZE: (u32, u32) = (1180, 800);

const LEAST: (u32, u32) = (560, 480);

const SIDE_ROW: f32 = 18.0;

const SIDE_ROW_MOST: f32 = 40.0;

const SIDE_SLACK: f32 = 4.0;

const SIDE_WIDTH: f32 = 270.0;

const SIDE_GAP: f32 = 12.0;

const SIDE_HALF_GAP: f32 = 6.0;

const SIDE_HEAD: f32 = 18.0;
const SIDE_HEAD_GAP: f32 = 4.0;
const SIDE_HEAD_INDENT: f32 = 12.0;

fn alias(id: &str) -> &str {
    match id {
        "time" | "date" => "datetime",
        "wifi" => "network",
        "sound" => "audio",
        "battery" => "power",
        "language" => "region",
        other => other,
    }
}

fn page_groups() -> [(&'static str, &'static [&'static str]); 5] {
    [
        (t!("settings-window-group-personal"), &["ring", "wallpaper", "appearance", "sounds"]),
        (t!("settings-window-group-devices"), &["display", "audio", "bluetooth", "keyboard", "printers", "power", "games"]),
        (t!("settings-window-group-network"), &["blocking", "network", "tailscale"]),
        (t!("settings-window-group-system"), &["users", "snapshots", "ai-programs", "region", "datetime"]),
        (t!("settings-window-group-about"), &["about", "thanks", "help"]),
    ]
}

pub const IN_APPS: &[&str] = &["default-apps", "startup", "permissions", "plugins"];

fn own_pages() -> Vec<Box<dyn Page>> {
    let mut pages = panels::all();
    pages.retain(|p| !IN_APPS.contains(&p.id()));
    pages
}

fn apps_pages() -> &'static [(&'static str, &'static str, &'static str, String)] {
    use crate::common::live_language::{per_language, Kept};
    static ALL: Kept<Vec<(&'static str, &'static str, &'static str, String)>> = std::sync::Mutex::new(Vec::new());
    per_language(&ALL, || panels::all().into_iter().filter(|p| IN_APPS.contains(&p.id()))
        .map(|p| (p.id(), p.title(), p.icon(), format!("{} {} {}", p.title(), p.keywords(), p.help()).to_lowercase())).collect()).as_slice()
}

pub(crate) fn apps_titles() -> Vec<&'static str> {
    panels::all().into_iter().filter(|p| IN_APPS.contains(&p.id())).map(|p| p.title()).collect()
}

fn in_folders() -> (&'static str, &'static str, &'static str) {
    (t!("settings-window-in-folders-name"), "system-file-manager-symbolic", t!("settings-window-in-folders-keywords"))
}

fn open_in_folders() {
    crate::common::run::spawn(&["trindle-folders", "--settings"]);
}

pub fn open_in_apps(id: &str) {

    let page = if id == "plugins" { "your-plugins" } else { id };
    crate::common::run::spawn(&["trindle-apps", "--page", page]);
}

pub struct Settings {
    pub(crate) kit: Kit,
    pages: Vec<Box<dyn Page>>,
    current: usize,
    search: String,

    tall: f32,
}

impl Settings {

    pub(crate) fn with<P: Page + 'static>(&mut self, f: impl FnOnce(&mut P, &mut Kit)) {
        for page in &mut self.pages {
            if let Some(p) = page.as_any_mut().downcast_mut::<P>() {
                f(p, &mut self.kit);
                return;
            }
        }
    }

    fn find(&self, id: &str) -> Option<usize> {
        let id = alias(id);
        self.pages.iter().position(|p| p.id() == id && p.listed())
    }

    fn show(&mut self, id: &str) {
        let (id, topic) = match id.split_once(':') {
            Some((page, topic)) => (page, Some(topic)),
            None => (id, None),
        };
        if self.find(id).is_none() && IN_APPS.contains(&alias(id)) && !self.kit.hosted {
            open_in_apps(alias(id));
            return;
        }
        if self.find(id).is_none() && self.kit.hosted {
            let page = match topic {
                Some(t) => format!("{id}:{t}"),
                None => id.to_string(),
            };
            crate::common::run::spawn(&["trindle-settings", "--panel", &page]);
            return;
        }
        let Some(at) = self.find(id) else { return };
        if at != self.current {
            self.pages[self.current].hidden(&mut self.kit);
            self.current = at;
            self.kit.current = self.pages[at].id().to_string();
            self.kit.icon = self.pages[at].icon();
            self.pages[at].shown(&mut self.kit);
        }
        if let Some(t) = topic {
            self.pages[at].open_topic(&mut self.kit, t);
        }
    }

    fn second(&mut self) -> bool {
        let at = self.current;
        let found = self.pages[at].second(&mut self.kit);
        let moves = self.kit.want.is_some() || self.kit.back;
        self.follow();
        found || moves
    }

    fn follow(&mut self) {
        if let Some(want) = self.kit.want.take() {
            self.show(&want);
        }
        if std::mem::take(&mut self.kit.back) {
            match self.kit.from.take() {
                Some(from) => self.show(&from),
                None => self.show("help"),
            }
        }
    }

    fn handle(&mut self, ask: Do) {
        match ask {
            Do::Then(f) => f(self),
            Do::Look(f) => {
                f(self);
            }
            Do::Open(args) => {
                self.relist();
                if let Some(p) = panel_of(&args) {
                    self.show(&p);
                }
            }
            Do::Conf => {
                for p in &mut self.pages {
                    p.conf_changed(&mut self.kit);
                }
            }
            Do::Tick | Do::Show | Do::Say(_) => {}
        }
        self.follow();
    }

    pub(crate) fn relanguage(&mut self) {
        let mut fresh = panels::all();
        for (at, page) in self.pages.iter_mut().enumerate() {
            if at == self.current || self.kit.awaited((*page.as_any_mut()).type_id()) {
                continue;
            }
            if let Some(i) = fresh.iter().position(|p| p.id() == page.id()) {
                *page = fresh.swap_remove(i);
            }
        }
        self.relist();
        self.kit.retitle.set(true);
    }

    fn relist(&mut self) {
        for p in &mut self.pages {
            p.relist();
        }
        if !self.pages[self.current].listed() {
            self.pages[self.current].hidden(&mut self.kit);
            self.current = 0;
            self.kit.current = self.pages[0].id().to_string();
            self.kit.icon = self.pages[0].icon();
            self.pages[0].shown(&mut self.kit);
        }
    }

    fn moving(&self) -> bool {
        self.kit.moving() || self.pages[self.current].moving()
    }

    fn turn(&self) -> bool {
        self.kit.turn() | self.pages[self.current].moving()
    }

    fn matches(&self, page: &dyn Page) -> bool {
        let search = self.search.to_lowercase();
        let mut words = search.split_whitespace().peekable();
        if words.peek().is_none() {
            return true;
        }
        haystack(page, |text| words.all(|w| text.contains(w)))
    }

    fn first_found(&self) -> Option<&'static str> {
        page_groups().iter().flat_map(|(_, ids)| ids.iter())
            .find(|id| self.find(id).is_some_and(|i| self.matches(&*self.pages[i])))
            .copied()
    }

    fn found_in_apps(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        if self.search.trim().is_empty() {
            return Vec::new();
        }
        apps_pages().iter().filter(|(_, _, _, text)| self.search.to_lowercase().split_whitespace().all(|w| text.contains(w)))
            .map(|&(id, title, icon, _)| (id, title, icon)).collect()
    }

    fn found_in_folders(&self) -> bool {
        let words = self.search.to_lowercase();
        let known = format!("{} {}", in_folders().0, in_folders().2).to_lowercase();
        !words.trim().is_empty() && words.split_whitespace().all(|w| known.contains(w))
    }

    fn typed(&mut self, text: String) {
        self.search = text;
        if !self.search.trim().is_empty() {
            if let Some(id) = self.first_found() {
                self.show(id);
            }
        }
    }

    fn side(&self) -> Node<Msg> {
        card(scroll("side", self.side_list())).key("side-bar").snug().side().width(Length::Fixed(SIDE_WIDTH)).height(Length::Fill)
    }

    fn side_list(&self) -> Node<Msg> {
        let search = entry(self.search.clone(), t!("settings-window-search"), |t| go(move |s| s.typed(t.clone())))
            .on_submit(go(|s| match s.first_found() {
                Some(id) => s.show(id),
                None => {
                    if let Some(id) = s.found_in_apps().first() {
                        open_in_apps(id.0);
                    } else if s.found_in_folders() {
                        open_in_folders();
                    }
                }
            }));

        let mut groups = Vec::new();
        let row = self.side_row();
        for (name, ids) in page_groups() {
            let found: Vec<usize> = ids.iter().filter_map(|id| self.find(id)).filter(|&i| self.matches(&*self.pages[i])).collect();
            if found.is_empty() {
                continue;
            }
            let rows = found.iter().map(|&i| item(self.pages[i].title()).icon(self.pages[i].icon())).collect();
            let chosen = found.iter().position(|&i| i == self.current);
            let ids: Rc<Vec<&'static str>> = Rc::new(found.iter().map(|&i| self.pages[i].id()).collect());
            let pick = list(rows, chosen, move |n| {
                let id = ids[n];
                go(move |s| s.show(id))
            }).called(name).here().row_height(row);
            groups.push(column(vec![crate::toolkit::widget::row(vec![space().width(Length::Fixed(SIDE_HEAD_INDENT)), side_heading(name)]), pick]).gap(SIDE_HEAD_GAP));
        }

        let elsewhere = self.found_in_apps();
        if !elsewhere.is_empty() {
            let rows = elsewhere.iter().map(|(_, title, icon)| item(*title).icon(*icon)).collect();
            let ids: Rc<Vec<&'static str>> = Rc::new(elsewhere.iter().map(|(id, _, _)| *id).collect());
            let pick = list(rows, None, move |n| {
                let id = ids[n];
                go(move |_| open_in_apps(id))
            }).called(t!("settings-window-in-apps")).here().row_height(row);
            groups.push(column(vec![crate::toolkit::widget::row(vec![space().width(Length::Fixed(SIDE_HEAD_INDENT)), side_heading(t!("settings-window-in-apps"))]), pick]).gap(SIDE_HEAD_GAP));
        }

        if self.found_in_folders() {
            let pick = list(vec![item(in_folders().0).icon(in_folders().1)], None, |_| go(|_| open_in_folders())).called(t!("settings-window-in-folders")).here().row_height(row);
            groups.push(column(vec![crate::toolkit::widget::row(vec![space().width(Length::Fixed(SIDE_HEAD_INDENT)), side_heading(t!("settings-window-in-folders"))]), pick]).gap(SIDE_HEAD_GAP));
        }
        let pages = if groups.is_empty() {
            note(t!("settings-window-no-page"))
        } else {
            column(crate::store::with_rules(groups)).gap(SIDE_HALF_GAP)
        };
        column(vec![search, pages]).gap(SIDE_GAP).padding(theme::sheet().sidebar_room)
    }

    fn side_row(&self) -> f32 {
        let groups: Vec<usize> = page_groups().iter()
            .map(|(_, ids)| ids.iter().filter_map(|id| self.find(id)).filter(|&i| self.matches(&*self.pages[i])).count())
            .filter(|&n| n > 0)
            .collect();
        let rows: usize = groups.iter().sum();
        if rows == 0 || self.tall <= 0.0 {
            return SIDE_ROW;
        }
        let s = theme::sheet();
        let heads = groups.len() as f32;

        let fixed = 2.0 * s.snug_padding + 2.0 * s.sidebar_room + s.entry_height + SIDE_GAP + (heads - 1.0).max(0.0) * (2.0 * SIDE_HALF_GAP + s.section_line_width) + heads * (SIDE_HEAD + SIDE_HEAD_GAP) + SIDE_SLACK;
        ((self.tall - fixed) / rows as f32).floor().clamp(SIDE_ROW, SIDE_ROW_MOST)
    }
}

pub struct Hosted {
    s: Settings,
}

impl Hosted {

    pub(crate) fn new(events: &Loop, asks: Post<Do>, session: Option<Bus>, system: Option<Bus>) -> Hosted {
        let mut pages = panels::all();
        pages.retain(|p| IN_APPS.contains(&p.id()));
        pages.sort_by_key(|p| IN_APPS.iter().position(|i| *i == p.id()));
        let mut kit = Kit::new(events.clone(), asks, session, system);
        kit.hosted = true;
        if let Some(p) = pages.first() {
            kit.current = p.id().to_string();
            kit.icon = p.icon();
        }
        Hosted { s: Settings { kit, pages, current: 0, search: String::new(), tall: 0.0 } }
    }

    pub fn pages(&self) -> Vec<(&'static str, &'static str, &'static str)> {
        self.s.pages.iter().filter(|p| p.listed()).map(|p| (p.id(), p.title(), p.icon())).collect()
    }

    pub fn has(&self, id: &str) -> bool {
        self.s.find(id).is_some()
    }

    pub fn show(&mut self, id: &str) {
        if let Some(at) = self.s.find(id) {
            if at == self.s.current {
                self.s.pages[at].shown(&mut self.s.kit);
            } else {
                self.s.show(id);
            }
        }
    }

    #[cfg(not(test))]
    pub fn preload(&mut self) {
        for page in &mut self.s.pages {
            page.preload(&mut self.s.kit);
        }
    }

    pub fn hide(&mut self) {
        let at = self.s.current;
        self.s.pages[at].hidden(&mut self.s.kit);
    }

    pub fn view(&self) -> Node<Msg> {
        self.s.kit.pulse.set(false);
        ui::new_page(self.s.pages[self.s.current].icon());
        let page = self.s.pages[self.s.current].view(&self.s.kit).width(Length::Fill).height(Length::Fill);
        ui::with_info(&self.s.kit, page)
    }

    pub fn update(&mut self, msg: Msg) {
        (msg.0)(&mut self.s);
        self.s.follow();
    }

    pub(crate) fn handle(&mut self, d: Do) {
        self.s.handle(d);
    }

    pub fn second(&mut self) -> bool {
        self.s.second()
    }

    pub fn moving(&self) -> bool {
        self.s.moving()
    }

    pub fn turn(&self) -> bool {
        self.s.turn()
    }

    pub fn pressed(&mut self, place: crate::toolkit::widget::Rect) {
        self.s.kit.pressed_at = Some(place);
    }

    pub fn escape(&mut self) -> bool {
        self.s.kit.info.take().is_some()
    }
}

fn go(f: impl Fn(&mut Settings) + 'static) -> Msg {
    Msg(Rc::new(f))
}

impl Shows for Settings {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        (msg.0)(self);
        self.follow();
    }

    fn resized(&mut self, _w: f32, h: f32) -> bool {
        let changed = (h - self.tall).abs() >= 1.0;
        self.tall = h;
        changed
    }

    fn view(&self) -> Node<Msg> {
        self.kit.pulse.set(false);
        ui::new_page(self.pages[self.current].icon());
        let page = self.pages[self.current].view(&self.kit).width(Length::Fill).height(Length::Fill);

        let page = match self.pages[self.current].foot(&self.kit) {
            Some(foot) => column(vec![page, foot]).gap(0.0).width(Length::Fill).height(Length::Fill),
            None => page,
        };
        let window = row(vec![self.side(), page]).gap(0.0);

        ui::with_info(&self.kit, window)
    }

    fn pressed(&mut self, place: crate::toolkit::widget::Rect) {
        self.kit.pressed_at = Some(place);
    }

    fn window_title(&self) -> Option<String> {
        self.kit.retitle.take().then(|| t!("settings-window-title").to_string())
    }

    fn intercept(&self, input: &crate::toolkit::input::Input, _keys: bool) -> Option<Msg> {
        if let Some(taken) = self.pages[self.current].intercept(input) {
            return Some(taken);
        }
        match input {
            crate::toolkit::input::Input::Key { sym: crate::toolkit::input::Keysym::Escape, .. } if self.kit.info.is_some() =>
                Some(go(|s| s.kit.info = None)),
            _ => None,
        }
    }
}

#[cfg(test)]
thread_local! {

    static LOWERED: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
}

fn haystack<T>(page: &dyn Page, f: impl FnOnce(&str) -> T) -> T {
    thread_local! {
        static KEPT: RefCell<HashMap<(&'static str, usize), String>> = RefCell::new(HashMap::new());
    }
    let key = (page.id(), page.help().as_ptr() as usize);
    KEPT.with(|kept| {
        let mut kept = kept.borrow_mut();
        let text = kept.entry(key).or_insert_with(|| {
            #[cfg(test)]
            LOWERED.with(|n| n.set(n.get() + 1));
            format!("{} {} {}", page.title(), page.keywords(), page.help()).to_lowercase()
        });
        f(text)
    })
}

fn panel_of(args: &[String]) -> Option<String> {
    args.windows(2).find(|w| w[0] == "--panel").map(|w| w[1].clone())
        .or_else(|| args.iter().find_map(|a| a.strip_prefix("--panel=")).map(String::from))
}

fn own_key_taken() -> bool {
    use crate::common::own_helpers::OWN_KEY;
    let said = std::env::var(OWN_KEY).is_ok_and(|v| v == "1");
    std::env::remove_var(OWN_KEY);
    said
}

pub fn main(mut args: Vec<String>) -> i32 {
    let own_key = own_key_taken();
    crate::common::no_driver::start();
    if args.get(1).map(String::as_str) == Some("--screen-reader") {
        if let Some(code) = command_line::screen_reader(&mut args, own_key) {
            return code;
        }
    }
    if let Some(code) = command_line::way_back(&args) {
        return code;
    }
    if let Some(code) = handed_on(&args) {
        return code;
    }
    let events = Loop::new();
    let Some(bus) = Bus::session(&events).ok() else { return window::open(&events, args) };
    window::share(&events, &bus, args)
}

fn handed_on(args: &[String]) -> Option<i32> {

    if panel_of(args).as_deref() == Some("disks") {
        crate::common::run::spawn(&["trindle-drives"]);
        return Some(0);
    }

    let page = panel_of(args).filter(|p| IN_APPS.contains(&alias(p)))?;
    open_in_apps(alias(&page));
    Some(0)
}

#[cfg(test)]
mod tests;
