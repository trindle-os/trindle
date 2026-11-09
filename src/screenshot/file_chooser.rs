//! The file chooser's window, opened by the portal for any program.

mod accept;
mod changes;
mod draw;
mod entries;
pub(crate) mod kept;
mod lent;
mod looks;
mod menu;
mod places;
mod update;
#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::common::files::home;
use crate::toolkit::event::Post;
use crate::toolkit::popup::{Popup, PopupMsg};
use crate::toolkit::t;
use crate::toolkit::ui::App;

pub use crate::common::kinds::{file_uri, Mime};
use entries::passes;
use kept::folders_file;
use lent::{Answer, Clash};
use places::first_places;

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub enum Mode {
    Open { many: bool, folder: bool },
    Save,

    SaveMany(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Filter {
    pub name: String,
    pub rules: Vec<(u32, String)>,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Pick {
    pub id: String,
    pub label: String,
    pub options: Vec<(String, String)>,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Ask {
    pub title: String,
    pub accept: Option<String>,
    pub mode: Mode,
    pub filters: Vec<Filter>,
    pub filter: Option<usize>,
    pub choices: Vec<Pick>,
    pub folder: Option<PathBuf>,
    pub name: String,
    pub app_id: String,
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub enum Outcome {
    Chosen { uris: Vec<String>, choices: Vec<(String, String)>, filter: Option<Filter> },
    Cancelled,
}

#[derive(Clone, Debug)]
pub enum Msg {
    Place(usize),
    Part(usize),
    Choose(usize),
    Pick(Vec<usize>),
    Open(usize),
    Name(String),
    Filter(usize),
    Choice(usize, usize),
    Toggle(usize, bool),

    Accept,

    Submit,
    Cancel,
    Changed,
    Hidden,

    NewFolder,

    Menu(usize, f32, f32),

    RoomMenu(bool),

    Pop(PopupMsg),

    Rename,
    RenameText(String),

    RenameDone,

    RenameStop,

    RenameLeft,

    Pasted(Option<(Vec<PathBuf>, bool)>, PathBuf),

    Worked(Vec<String>, Verb),

    Began(PathBuf),

    Candidates(Vec<Place>),

    Checked(usize, Place),

    Late(Waited),

    Listed(Vec<Entry>, Option<i32>, Where, bool),

    Seen(bool, PathBuf),

    Target(Found, PathBuf, bool),

    Named(Vec<PathBuf>, PathBuf),

    Made(Result<PathBuf, String>),

    Renamed(Result<PathBuf, String>, Renaming, bool),

    Asked(Clash),

    Answer(Answer),

    SameForRest(bool),

    AskStop,

    Key(Act),

    DeleteSure,

    DeleteNo,

    Still,
}

enum On {

    Menu(Option<PathBuf>),

    Keys,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verb {
    Moved,
    Copied,
    Trashed,
    Deleted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Act {
    Rename,
    Copy,
    Cut,
    Paste,
    NewFolder,
    Trash,
    Delete,
    CopyPath,
    Pin,
}

#[derive(Clone, Debug)]
pub struct Renaming {
    path: PathBuf,
    text: String,
    folder: bool,

    fresh: bool,

    why: Option<String>,

    out: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Where {
    Recent,
    Folder(PathBuf),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Waited {

    Place(Where, bool),

    Start,

    Save(PathBuf),
}

const LATE: std::time::Duration = std::time::Duration::from_secs(3);

pub fn not_answering() -> &'static str {
    t!("screenshot-chooser-not-answering")
}

#[derive(Clone, Debug)]
pub struct Place {
    label: String,
    icon: &'static str,

    folder: bool,
    at: Where,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Entry {
    name: String,
    path: PathBuf,
    folder: bool,
    size: u64,
    when: i64,
}

#[derive(Clone, Copy, Debug)]
pub struct Found {

    folder: bool,

    parent: bool,

    there: bool,
}

pub struct Chooser {
    ask: Ask,
    mime: Mime,
    home: PathBuf,
    places: Vec<Place>,
    at: Where,
    entries: Vec<Entry>,
    chosen: Option<usize>,
    picked: Vec<usize>,
    name: String,

    armed: Option<PathBuf>,

    offered: std::cell::RefCell<Option<PathBuf>>,

    kept: PathBuf,
    said: String,
    done: Option<Box<dyn FnOnce(Outcome)>>,

    watching: Option<crate::toolkit::watch::Watch>,
    watched: HashSet<PathBuf>,

    hidden: bool,

    menu: Option<(Popup, Vec<Act>)>,

    menu_on: Option<PathBuf>,

    renaming: Option<Renaming>,

    asked_rename: u32,

    clip: Option<(Vec<PathBuf>, bool)>,

    failed: String,

    post: Option<Post<Msg>>,

    pointer: std::cell::Cell<(f32, f32)>,

    bin: Option<lent::Bin>,

    pins: PathBuf,

    deleting: Option<(Vec<PathBuf>, Vec<lent::Asked>)>,

    began: bool,

    start_out: bool,

    found: Vec<(usize, Place)>,

    moved: bool,

    listed: bool,

    looks: Vec<(Where, bool)>,

    again: bool,

    late_place: bool,

    late_save: bool,

    pick_next: Option<PathBuf>,

    seen: Option<(PathBuf, bool)>,

    seen_old: bool,

    seen_out: Vec<PathBuf>,

    accepting: Vec<PathBuf>,

    asking: Option<Clash>,
    reply: Option<std::sync::mpsc::Sender<(Answer, bool)>>,

    same: bool,

    questions: u32,
}

impl Chooser {

    pub fn new(mut ask: Ask, done: impl FnOnce(Outcome) + 'static) -> Chooser {
        let home = home();
        if ask.filter.is_none() && !ask.filters.is_empty() {
            ask.filter = Some(0);
        }
        let start = ask.folder.clone().filter(|f| f.is_absolute()).unwrap_or_else(|| home.clone());
        let name = ask.name.clone();
        Chooser {
            ask, mime: Mime::load(), places: first_places(&home), home, at: Where::Folder(start), entries: Vec::new(), chosen: None, picked: Vec::new(),
            name, armed: None, offered: Default::default(), kept: folders_file(), said: crate::common::damaged::take_notes().join(" "), done: Some(Box::new(done)), watching: crate::toolkit::watch::Watch::new().ok(),
            watched: HashSet::new(), hidden: false,
            menu: None, menu_on: None, renaming: None, asked_rename: 0, clip: None, failed: String::new(), post: None, pointer: Default::default(), bin: None, pins: lent::pins_file(), deleting: None,
            began: false, start_out: false, found: Vec::new(), moved: false, listed: false, looks: Vec::new(), again: false, late_place: false, late_save: false, pick_next: None, seen: None, seen_old: false, seen_out: Vec::new(), accepting: Vec::new(),
            asking: None, reply: None, same: false, questions: 0,
        }
    }

    pub fn watch_fd(&self) -> Option<std::os::fd::RawFd> {
        self.watching.as_ref().map(|w| w.fd())
    }

    pub fn saw(&self) -> bool {
        let Some(w) = &self.watching else { return false };
        let changed = w.changed();

        let early = w.unplaced();
        match &self.at {
            Where::Folder(f) => early || changed.iter().any(|p| p.parent() == Some(f.as_path()) || p == f),
            Where::Recent => false,
        }
    }

    fn saving(&self) -> bool {
        matches!(self.ask.mode, Mode::Save | Mode::SaveMany(_))
    }

    fn folders_only(&self) -> bool {
        matches!(self.ask.mode, Mode::Open { folder: true, .. } | Mode::SaveMany(_))
    }

    fn shows(&self, e: &Entry) -> bool {
        if e.folder {
            return true;
        }
        if self.folders_only() {
            return false;
        }
        match self.ask.filter.and_then(|i| self.ask.filters.get(i)) {
            Some(f) => passes(f, &self.mime, &e.name, self.mime.of(&e.name).as_deref()),
            None => true,
        }
    }

    fn go(&mut self, at: Where) {
        self.moved = true;
        if at != self.at {
            self.at = at;
            self.chosen = None;
            self.picked.clear();

            self.entries.clear();
            self.listed = false;
            self.again = false;
            self.late_place = false;
        }
        self.armed = None;
        self.said.clear();
        self.failed.clear();
        self.renaming = None;
        self.list();
    }

    fn folder(&self) -> Option<&Path> {
        match &self.at {
            Where::Folder(f) => Some(f),
            Where::Recent => None,
        }
    }

    pub fn set_post(&mut self, post: Post<Msg>) {
        self.post = Some(post);
        self.begin();
    }
}

pub fn wire(events: &crate::toolkit::event::Loop, shown: &std::rc::Rc<crate::toolkit::ui::Shown<Chooser>>) {
    use crate::toolkit::event::Flow;
    let weak = std::rc::Rc::downgrade(shown);
    let post = events.post(move |m: Msg| {
        if let Some(shown) = weak.upgrade() {
            shown.change(|c| c.update(m));
        }
    });
    shown.change(|c| c.set_post(post));
    if let Some(fd) = shown.with(|c| c.watch_fd()) {
        let weak = std::rc::Rc::downgrade(shown);
        events.watch(fd, move |_| {
            let Some(shown) = weak.upgrade() else { return Flow::Stop };
            if shown.with(|c| c.saw()) {
                shown.change(|c| c.update(Msg::Changed));
            }
            Flow::Continue
        });
    }
}

pub const SIZE: (u32, u32) = (900, 640);

const SIDEBAR: f32 = 230.0;

const NAME_BOX: f32 = 340.0;
