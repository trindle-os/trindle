//! Text: a plain text editor drawn by the Trindle Toolkit. Several tabs,
//! each file read and written in the encoding it came in, work kept in
//! Trindle's state folder as it goes, and a question before anything not
//! saved to a file is closed.

use crate::toolkit::t;
use crate::toolkit::editing::doc;
use crate::toolkit::editing::editor;
mod app;
mod disk;
mod enc;
mod find;
mod keys;
mod markdown;
mod pdf;
mod proofing;
mod spell;
pub(crate) mod store;
mod tabs;
mod view;

use std::cell::{Cell, RefCell};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
use std::time::Duration;

use crate::common::files;
use crate::toolkit::bus::{Asked, Bus, Listen, Offer, Props, Served};
use crate::toolkit::chooser;
use crate::toolkit::dnd::{Given, Way};
use crate::toolkit::event::{Flow, Loop, Post};
use crate::toolkit::input::Input;
use crate::toolkit::ui::Shown;
use crate::toolkit::watch::Watch;
use crate::toolkit::widget::{Held, Part, Pointer};
use crate::toolkit::window::{Display, Kind, Window};
use doc::{Doc, Find};
use editor::{Look, Pane};
use disk::is_markdown;
use enc::Form;

const NAME: &str = "org.trindle.Text";
const PATH: &str = "/org/trindle/Text";
const XML: &str = r#"<interface name="org.trindle.Text"><method name="Show"/><method name="Open"><arg type="as" direction="in"/></method></interface>"#;
const SIZE: (u32, u32) = (960, 640);

const SLIDER_REST: Duration = Duration::from_millis(500);

const KEEP_EVERY: Duration = Duration::from_secs(2);

const NOTE_FOR: Duration = Duration::from_secs(2);

const TAB_WIDTH: f32 = 180.0;

const TAB_NAME: usize = 24;

struct Tab {
    id: u64,
    path: Option<PathBuf>,
    form: Form,
    pane: Rc<RefCell<Pane>>,
    markdown: bool,

    shown: bool,
    finished: Rc<RefCell<markdown::finished::Finished>>,

    kept: u64,

    stamp: Option<store::Stamp>,

    changed: bool,
}

impl Tab {
    fn name(&self) -> String {
        self.path
            .as_ref()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| t!("text-untitled").into())
    }

    fn unsaved(&self) -> bool {
        let pane = self.pane.borrow();
        pane.doc.modified() || (self.path.is_none() && !pane.doc.is_empty())
    }

    fn blank(&self) -> bool {
        let pane = self.pane.borrow();
        self.path.is_none() && pane.doc.is_empty() && !pane.doc.can_undo()
    }

    fn entry(&self) -> store::Entry {
        let pane = self.pane.borrow();
        store::Entry {
            id: self.id,
            path: self.path.clone(),
            enc: self.form.enc.name().to_string(),
            bom: self.form.bom,
            crlf: self.form.crlf,
            unsaved: self.unsaved(),
            caret: pane.doc.caret,
            markdown: self.markdown,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum View {
    Edit,
    Recent,
    Settings,
    Encodings,
    Closing,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Bar {
    None,
    Find,
    Replace,
    Goto,

    Spell,
}

struct Spelling {
    tab: u64,
    range: Range<usize>,
    word: String,
    offers: Vec<String>,
    pick: usize,

    version: u64,
    added: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Purpose {
    Open,

    SaveAs { id: u64, closing: bool },

    Export { id: u64, pdf: bool },
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Answer {
    Save,
    Discard,
    Keep,
}

#[derive(Clone)]
enum Msg {
    Key(Input),
    Pointer(Pointer),

    Dropped(f32, f32, Given),
    SaveAnyway,
    Reload,
    Scroll(f32, f32, bool),
    New,
    Open,
    Save,
    SaveAs,
    Tab(usize),
    CloseTab(usize),
    NextTab(bool),
    Quit,
    Show(View),
    Back,
    Bar(Bar),
    FindText(String),
    WithText(String),
    GotoText(String),
    Case(bool),
    Whole(bool),
    Next(bool),
    ReplaceOne,
    ReplaceAll,
    Goto,
    Undo,
    Redo,
    Recent(usize),
    ForgetRecent,
    ReadAs(usize),
    SaveIn(usize),
    Answer(Answer),
    Reopen(usize),
    Wrap(bool),
    Numbers(bool),
    Font(usize),
    Size(f32),
    Zoom(i32),
    Spell(bool),
    Lang(usize),
    Finished,
    Export(bool),

    SpellHere,

    CopyAll,
    Pick(usize),
    UseWord,
    AddWord,
}

enum Do {
    Show,
    Open(Vec<String>),
    Chosen(Purpose, Vec<PathBuf>),
    Exported(Result<(), String>),

    Speller(String, Result<Box<spell::Speller>, String>),
    Paste(String),
    NoteGone(u32),

    SaveSettings(u32),
    Restyle,
    CloseWindow,
    Stop,
}

struct Text {
    events: Loop,
    asks: Post<Do>,
    bus: Option<Bus>,
    window: Weak<Window>,
    settings: store::Settings,
    dir: PathBuf,
    recent: Vec<PathBuf>,
    tabs: Vec<Tab>,
    active: usize,
    next_id: u64,
    view: View,
    bar: Bar,
    find: String,
    with: String,
    how: Find,
    goto: String,

    closing: Vec<u64>,

    choosing: bool,

    settings_turn: u32,
    settings_unsaved: bool,
    quitting: bool,

    said: Option<String>,

    found: Option<String>,

    noted: Option<(&'static str, u32)>,
    turn: u32,

    grab: u32,
    grab_bar: bool,
    listen: Option<Listen>,

    at: Cell<(f32, f32)>,

    counted: RefCell<Option<(u64, u64, Range<usize>, doc::Counts)>>,

    written: Vec<store::Entry>,
    written_active: usize,
    speller: Option<Rc<RefCell<spell::Speller>>>,

    loading: Option<String>,
    spelling: Option<Spelling>,
}

fn look(s: &store::Settings) -> Look {
    Look { font: (!s.font.is_empty()).then(|| s.font.clone()), size: s.size, wrap: s.wrap, numbers: s.numbers }
}

fn dictionaries() -> Vec<String> {
    spell::installed(Path::new(spell::FOLDER))
}

fn absolute(arg: &str) -> Option<PathBuf> {
    if arg.starts_with('-') {
        return None;
    }
    let path = if arg.starts_with("file://") { chooser::from_uri(arg)? } else { PathBuf::from(arg) };
    if path.is_absolute() {
        Some(path)
    } else {
        std::env::current_dir().ok().map(|d| d.join(path))
    }
}

fn cut(name: &str, most: usize) -> String {
    if name.chars().count() <= most {
        return name.to_string();
    }
    let mut out: String = name.chars().take(most.saturating_sub(1)).collect();
    out.push('…');
    out
}

impl Text {
    fn new(events: &Loop, asks: Post<Do>, bus: Option<Bus>) -> Text {
        Text::in_folder(events, asks, bus, store::Settings::load(&store::settings_file()), store::state_dir())
    }

    fn in_folder(events: &Loop, asks: Post<Do>, bus: Option<Bus>, settings: store::Settings, dir: PathBuf) -> Text {

        let mended = store::mend(&dir);
        let recent = store::recent(&dir);
        let reset = crate::common::damaged::take_notes().join(" ");
        let mut text = Text {
            events: events.clone(),
            asks,
            bus,
            window: Weak::new(),
            settings,
            recent,
            dir,
            tabs: Vec::new(),
            active: 0,
            next_id: 1,
            view: View::Edit,
            bar: Bar::None,
            find: String::new(),
            with: String::new(),
            how: Find::default(),
            goto: String::new(),
            closing: Vec::new(),
            choosing: false,
            settings_turn: 0,
            settings_unsaved: false,
            quitting: false,
            said: (!reset.is_empty()).then_some(reset),
            found: None,
            noted: None,
            turn: 0,
            grab: 1,
            grab_bar: false,
            listen: None,
            at: Cell::new((0.0, 0.0)),
            counted: RefCell::new(None),
            written: Vec::new(),
            written_active: 0,
            speller: None,
            loading: None,
            spelling: None,
        };
        text.restore();
        for save in mended {
            text.take_back(save);
        }
        text.spell_setup();
        if text.tabs.is_empty() {
            text.add_tab(None, Form::default(), Doc::new(String::new()));
        }
        text.focus_editor();
        text
    }

    fn pane(&self) -> &Rc<RefCell<Pane>> {
        &self.tabs[self.active].pane
    }

    fn add_tab(&mut self, path: Option<PathBuf>, form: Form, doc: Doc) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.push_tab(id, path, form, doc, false)
    }

    fn push_tab(&mut self, id: u64, path: Option<PathBuf>, form: Form, doc: Doc, markdown: bool) -> usize {
        let mut pane = Pane::new(doc, look(&self.settings));
        pane.checker = self.checker();
        let kept = pane.doc.version;
        self.tabs.push(Tab { id, path, form, pane: Rc::new(RefCell::new(pane)), markdown, shown: false, finished: Rc::default(), kept, stamp: None, changed: false });
        self.tabs.len() - 1
    }

    fn restore(&mut self) {
        let (entries, active) = store::restore(&self.dir, self.settings.reopen);
        self.next_id = entries.iter().map(|e| e.id).max().unwrap_or(0) + 1;
        for e in entries {
            let named = enc::by_name(&e.enc).unwrap_or(encoding_rs::UTF_8);
            let (text, form, unsaved) = if e.unsaved {
                let Some(text) = store::kept_text(&self.dir, e.id) else { continue };
                (text, Form { enc: named, bom: e.bom, crlf: e.crlf }, true)
            } else {
                let Some(path) = e.path.as_ref() else { continue };
                let Ok(bytes) = store::read_document(path) else { continue };
                let (text, form) = enc::read_as(&bytes, named);
                (text, form, false)
            };
            let stamp = e.path.as_deref().and_then(store::stamp);
            let mut doc = Doc::new(text);
            if unsaved {
                doc.mark_unsaved();
            }
            doc.go(e.caret, false);
            let at = self.push_tab(e.id, e.path, form, doc, e.markdown);
            self.tabs[at].stamp = stamp;
            self.tabs[at].pane.borrow_mut().show_caret();
        }
        self.active = active.min(self.tabs.len().saturating_sub(1));
        self.written = self.tabs.iter().map(Tab::entry).collect();
        self.written_active = self.active;
    }

    fn take_back(&mut self, save: store::Mended) {
        if let Some(words) = save.words {
            let same = |p: &PathBuf| p == &save.document || std::fs::canonicalize(p).is_ok_and(|real| real == save.document);
            let open = self.tabs.iter().position(|t| t.path.as_ref().is_some_and(same));
            let (text, form) = match open {
                Some(at) => enc::read_as(&words, self.tabs[at].form.enc),
                None => enc::read(&words),
            };
            let mut doc = Doc::new(text);
            doc.mark_unsaved();
            let at = match open {
                Some(at) => {
                    let mut pane = Pane::new(doc, look(&self.settings));
                    pane.checker = self.checker();
                    self.tabs[at].form = form;
                    self.tabs[at].pane = Rc::new(RefCell::new(pane));
                    at
                }
                None => {
                    let at = self.add_tab(Some(save.document.clone()), form, doc);
                    self.tabs[at].markdown = is_markdown(&save.document);
                    at
                }
            };

            let version = self.tabs[at].pane.borrow().doc.version;
            self.tabs[at].kept = version.wrapping_sub(1);
            self.active = at;
            self.tabs[at].pane.borrow_mut().show_caret();
            *self.counted.borrow_mut() = None;
        }
        if let Some(problem) = save.problem {
            self.said = Some(match self.said.take() {
                Some(said) => t!("text-two-problems", said = said, problem = problem),
                None => problem,
            });
        }
    }

    fn set(&mut self, change: impl FnOnce(&mut store::Settings)) {
        self.apply(change);
        self.save_settings();
    }

    fn set_resting(&mut self, change: impl FnOnce(&mut store::Settings)) {
        self.apply(change);
        self.settings_unsaved = true;
        self.settings_turn = self.settings_turn.wrapping_add(1);
        let (asks, turn) = (self.asks.clone(), self.settings_turn);
        self.events.after(SLIDER_REST, move || {
            asks.send(Do::SaveSettings(turn));
        });
    }

    fn apply(&mut self, change: impl FnOnce(&mut store::Settings)) {
        change(&mut self.settings);
        let look = look(&self.settings);
        for t in &self.tabs {
            t.pane.borrow_mut().look = look.clone();
        }
    }

    fn save_settings(&mut self) {
        self.settings_unsaved = false;
        match self.settings.save(&store::settings_file()) {
            Ok(()) => self.noted_to("settings"),
            Err(e) => self.fail(t!("text-settings-not-saved", reason = e.to_string())),
        }
    }
}

struct Running {
    _shown: Rc<Shown<Text>>,
    _served: Option<Served>,
    _raise: Option<Served>,
}

pub fn main(args: Vec<String>) -> i32 {

    let args: Vec<String> = args.iter().skip(1).filter_map(|a| absolute(a)).map(|p| p.display().to_string()).collect();
    let events = Loop::new();
    let Some(bus) = Bus::session(&events).ok() else { return open(&events, args) };
    let code = Rc::new(Cell::new(0));
    let (ev, c) = (events.clone(), code.clone());
    let kept = Rc::new(RefCell::new(None));
    let (k, b) = (kept.clone(), bus.clone());
    let first = RefCell::new(Some(args.clone()));
    let (ev2, b2, ev3) = (ev.clone(), b.clone(), ev.clone());
    crate::toolkit::once::claim(&bus, &events, NAME, PATH, move || {
        match start(&ev, &b, first.borrow_mut().take().unwrap_or_default()) {
            Some(running) => *k.borrow_mut() = Some(running),
            None => {
                c.set(1);
                ev.quit();
            }
        }
    }, move || {
        b2.call(NAME, PATH, NAME).method("Open", &(args.clone(),), false, Some(Duration::from_secs(2)), &ev2, move |_| ev3.quit());
    });
    events.run();
    drop(kept);
    code.get()
}

fn start(events: &Loop, bus: &Bus, args: Vec<String>) -> Option<Running> {
    let (shown, asks) = window(events, args, Some(bus.clone()))?;
    let served = bus
        .serve(PATH, Offer {
            name: NAME.into(),
            xml: XML.into(),
            call: Box::new(move |asked: Asked| {
                if asked.member() == "Open" {
                    let args = asked.message().body().deserialize::<(Vec<String>,)>().map(|a| a.0).unwrap_or_default();
                    asks.send(Do::Open(args));
                }
                asks.send(Do::Show);
                asked.reply(&());
            }),
            props: Box::new(Props::new),
            set: None,
        })
        .map_err(|e| eprintln!("{NAME}: cannot be asked to show: {e:?}"))
        .ok();
    let raise = crate::toolkit::once::serve_window(bus, PATH, &shown.window).map_err(|e| eprintln!("{NAME}: cannot be raised: {e:?}")).ok();
    Some(Running { _shown: shown, _served: served, _raise: raise })
}

fn open(events: &Loop, args: Vec<String>) -> i32 {
    let Some(_running) = window(events, args, None) else { return 1 };
    events.run();
    0
}

fn window(events: &Loop, args: Vec<String>, bus: Option<Bus>) -> Option<(Rc<Shown<Text>>, Post<Do>)> {
    let gone = events.clone();
    let display = Display::connect(events, move || gone.quit())?;
    let slot: Rc<RefCell<Weak<Shown<Text>>>> = Rc::new(RefCell::new(Weak::new()));
    let s = slot.clone();
    let asks = events.post(move |ask: Do| {
        let Some(shown) = s.borrow().upgrade() else { return };
        match ask {
            Do::Show => {
                shown.window.raise();
            }
            Do::Restyle => shown.restyle(),
            ask => shown.change(|t| t.handle(ask)),
        }
    });
    let app = Text::new(events, asks.clone(), bus);
    let kind = Kind::Window { title: t!("text-title").into(), app_id: NAME.into() };
    let shown = Rc::new(Shown::open(events, &display, kind, SIZE, app)?);
    *slot.borrow_mut() = Rc::downgrade(&shown);
    let w = Rc::downgrade(&shown.window);
    shown.change(|t| {
        t.window = w;
        t.open_args(&args);
    });
    let c = asks.clone();
    shown.window.on_close(move || {
        c.send(Do::CloseWindow);
    });

    let kept = Rc::downgrade(&shown);
    events.every(KEEP_EVERY, move || match kept.upgrade() {
        Some(shown) => {
            keep_quietly(&shown);
            Flow::Continue
        }
        None => Flow::Stop,
    });
    for signal in [libc::SIGTERM, libc::SIGINT, libc::SIGHUP] {
        let stop = asks.clone();
        events.on_signal(signal, move || if stop.send(Do::Stop) { Flow::Continue } else { Flow::Stop });
    }
    watch_conf(events, asks.clone());
    Some((shown, asks))
}

fn keep_quietly(shown: &Shown<Text>) {
    let busy = shown.with(|t| t.tabs.iter().any(|t| t.kept != t.pane.borrow().doc.version) || t.to_keep() != (t.written.clone(), t.written_active));
    if busy {
        shown.change(Text::keep);
    }
}

fn watch_conf(events: &Loop, asks: Post<Do>) {
    let Ok(mut watch) = Watch::new() else { return };
    let folder = files::trindle_config();
    let _ = std::fs::create_dir_all(&folder);
    if watch.folder(&folder).is_err() {
        return;
    }
    let conf = files::trindle_conf();
    events.watch(watch.fd(), move |hung_up| {
        if hung_up {
            return Flow::Stop;
        }
        if watch.changed().contains(&conf) && !asks.send(Do::Restyle) {
            return Flow::Stop;
        }
        Flow::Continue
    });
}

fn takes(held: &Held, text: bool) -> Option<Way> {
    let ways = held.offered.ways;
    if held.offered.files {
        return ways.has(Way::Copy).then_some(Way::Copy);
    }
    if held.offered.text && text {
        return [Way::Copy, Way::Move].into_iter().find(|w| ways.has(*w));
    }
    None
}

fn dropped(held: &Held, given: Given, _: Way) -> Msg {
    match held.part {
        Part::Point(x, y) => Msg::Dropped(x, y, given),
        _ => Msg::Dropped(0.0, 0.0, given),
    }
}

fn same_file(a: &Path, b: &Path) -> bool {
    a == b || matches!((std::fs::canonicalize(a), std::fs::canonicalize(b)), (Ok(x), Ok(y)) if x == y)
}

#[cfg(test)]
pub(crate) mod kept_files {
    pub(crate) use super::spell::add_word;

    pub(crate) fn write_no_tabs(dir: &std::path::Path) -> std::io::Result<()> {
        super::store::write_list(dir, &[], 0)
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod language_tests {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
    use crate::toolkit::t;

    #[test]
    fn the_text_words_follow_the_language_and_fall_back_word_by_word() {
        let french = "text-open = Ouvrir\ntext-none-found = Rien trouve\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            assert_eq!(t!("text-open"), "Ouvrir");
            assert_eq!(t!("text-none-found"), "Rien trouve");
            assert_eq!(t!("text-title"), "Text");
        });
    }
}
