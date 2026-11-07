//! The Updates page.

pub(super) mod cards;
pub(super) mod check;
pub(super) mod hold;
pub(super) mod install;
pub(super) mod read;
pub(super) mod rows;
pub(super) mod stopping;
pub(super) mod system_cards;
pub(super) mod view;
pub(super) mod words;
#[cfg(test)]
mod tests;

use crate::toolkit::t;
use std::collections::{BTreeSet, HashMap};
use crate::store::pages::job_rows::updating;
use crate::store::plugin_page;
use crate::store::Store;
use crate::common::held::Thing;
use crate::common::stopping::Stop;
#[cfg(test)]
use crate::common::held;
#[cfg(test)]
use crate::store::catalogue::REMOTE;

const APPS: &str = "apps";
const SYSTEM: &str = "system";
const FIRMWARE: &str = "firmware";

const APPS_ROW: &str = "up:apps";
const SYSTEM_ROW: &str = "up:system";

const TRINDLE_ROW: &str = "up:trindle";
const DEBIAN_ROW: &str = "up:debian";

const PART_ICON: &str = "application-x-addon-symbolic";

pub(super) const HOLD_ICON: &str = "view-pin-symbolic";

const STOP_ICON: &str = "media-playback-stop-symbolic";

fn debian_name() -> &'static str {
    t!("store-upd-debian-name")
}

const FIRST_ROWS: usize = 8;
const MORE_ROWS: usize = 20;

const NEEDS_RESTART: &[&str] = &["linux-image", "linux-firmware", "systemd", "libc6", "dbus", "firmware-"];

const REBOOT_REQUIRED: &str = "/run/reboot-required";

pub fn app_key(id: &str) -> String {
    format!("up:app:{id}")
}

#[derive(Clone, Debug, PartialEq)]
pub struct Flat {
    pub id: String,
    arch: String,
    branch: String,
    app: bool,
    origin: String,
    version: String,
    size: u64,
    name: String,
}

impl Flat {
    fn named(&self) -> String {
        format!("{}/{}/{}", self.id, self.arch, self.branch)
    }

    fn key(&self) -> String {
        if self.app { app_key(&self.id) } else { format!("up:part:{}:{}", self.id, self.branch) }
    }

    fn thing(&self) -> Option<Thing> {
        if self.app { Thing::app(&self.id) } else { Thing::part(&self.id, &self.branch) }
    }

    fn plain_name(&self) -> String {
        part_name(&self.name, &self.id, &self.branch)
    }
}

fn part_name(name: &str, id: &str, branch: &str) -> String {
    let name = if name.trim().is_empty() { tidy_id(id) } else { name.trim().to_string() };
    if name.split([' ', '(', ')']).any(|w| w == branch) { name } else { format!("{name} {branch}") }
}

fn tidy_id(id: &str) -> String {
    let words: Vec<&str> = id.split('.').filter(|w| !w.is_empty()).collect();
    let words = if words.len() >= 3 { &words[1..] } else { &words[..] };
    let mut name = words.join(" ");
    if let Some(first) = name.get(..1) {
        let upper = first.to_uppercase();
        name.replace_range(..1, &upper);
    }
    if name.is_empty() { id.to_string() } else { name }
}

fn flat_held(s: &Store, f: &Flat) -> bool {
    f.thing().is_some_and(|t| s.held.has(&t))
}

pub(super) fn counts(s: &Store) -> (usize, usize) {
    let held = s.updatable.iter().filter(|id| s.held.app(id)).count()
        + s.upd.flats.iter().filter(|f| !f.app && flat_held(s, f)).count()
        + s.firmware.iter().filter(|f| s.held.firmware(&f.id)).count()
        + plugin_page::waiting(s).iter().filter(|(mine, _)| s.held.plugin(&mine.id)).count();
    let all = s.updatable.len() + s.upd.parts() + s.debian.len() + s.firmware.len() + s.trindle_waiting.is_some() as usize
        + plugin_page::waiting(s).len();
    (all - held, held)
}

#[derive(Default)]
pub struct UpdatesPage {
    checking: BTreeSet<&'static str>,
    failed: HashMap<&'static str, String>,
    checked_at: Option<String>,
    pub(super) off: bool,
    sizes: HashMap<String, u64>,
    jobs: u32,
    app_rows: HashMap<String, f32>,
    app_order: Vec<String>,
    wants_restart: bool,
    waits: Option<String>,
    firmware_done: bool,
    steam_waits: bool,
    restart: bool,
    shown: usize,
    flats: Vec<Flat>,
    one: Option<String>,
    round: u32,
    held_card: bool,
    held_rows: Vec<Thing>,
    start_asked: bool,
    reads_left: u8,
    pub(super) stops: HashMap<String, Stop>,
    pub(super) stopped: BTreeSet<String>,
}

impl UpdatesPage {
    pub(super) fn parts(&self) -> usize {
        self.flats.iter().filter(|f| !f.app).count()
    }

    #[cfg(test)]
    pub(super) fn with_flats(mut self, flats: &[(&str, bool, &str, u64)]) -> UpdatesPage {
        self.flats = flats.iter().map(|(id, app, version, size)| Flat {
            id: id.to_string(), arch: "x86_64".into(), branch: if *app { "stable".into() } else { "25.08".into() }, app: *app,
            origin: REMOTE.into(), version: version.to_string(), size: *size, name: String::new(),
        }).collect();
        self
    }

    #[cfg(test)]
    pub(super) fn with_part_name(mut self, id: &str, name: &str) -> UpdatesPage {
        self.flats.iter_mut().filter(|f| !f.app && f.id == id).for_each(|f| f.name = name.to_string());
        self
    }

    #[cfg(test)]
    pub(super) fn opened_with(mut self, held: &held::Held) -> UpdatesPage {
        self.held_card = !held.is_empty();
        self.held_rows = held.things().cloned().collect();
        self
    }

    #[cfg(test)]
    pub(super) fn with_one(mut self, name: &str) -> UpdatesPage {
        self.one = Some(name.to_string());
        self.jobs = 1;
        self
    }
}

pub fn newer_version<'a>(s: &'a Store, id: &str) -> Option<&'a str> {
    s.upd.flats.iter().find(|f| f.app && f.id == id).map(|f| f.version.as_str()).filter(|v| !v.is_empty())
}

#[cfg(test)]
pub(super) fn made_up(checking: &[&'static str], jobs: u32, restart: bool, sizes: &[(&str, u64)], failed: &[(&'static str, &str)],
                      rows: &[(&str, f32)]) -> UpdatesPage {
    UpdatesPage {
        checking: checking.iter().copied().collect(),
        failed: failed.iter().map(|(k, w)| (*k, w.to_string())).collect(),
        checked_at: Some("14:32".into()),
        sizes: sizes.iter().map(|(k, n)| (k.to_string(), *n)).collect(),
        jobs, restart,
        app_rows: rows.iter().map(|(k, n)| (k.to_string(), *n)).collect(),
        ..UpdatesPage::default()
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Phase {
    Checking,
    Installing,
    Restart,
    Found,
    Done,
}

pub(super) fn refresh_off(s: &mut Store) {
    #[cfg(not(test))]
    {
        s.upd.off = !crate::common::update_checks::on();
    }
    #[cfg(test)]
    let _ = s;
}

pub(super) fn unchecked(s: &Store) -> bool {
    s.upd.off && s.upd.checked_at.is_none()
}

fn phase(s: &Store) -> Phase {
    if s.upd.jobs > 0 {
        Phase::Installing
    } else if !s.upd.checking.is_empty() {
        Phase::Checking
    } else if s.waiting_updates() > 0 {
        Phase::Found
    } else if s.upd.restart {
        Phase::Restart
    } else {
        Phase::Done
    }
}

pub fn app_share(s: &Store, id: &str) -> Option<f32> {
    if let Some(d) = s.dials.get(&app_key(id)).filter(|d| d.running()) {
        return Some(d.share());
    }
    s.dials.get(APPS_ROW).filter(|d| d.running())?;
    s.upd.app_rows.get(id).copied()
}

fn going_under(s: &Store, starts: &[&str]) -> Option<f32> {
    s.dials.iter().find(|(k, d)| d.running() && starts.iter().any(|w| k.starts_with(w))).map(|(_, d)| d.share())
}

pub(super) fn installing(s: &Store) -> bool {
    s.upd.jobs > 0 || updating(s) || going_under(s, &["pl:"]).is_some()
}

pub(super) fn own_busy(s: &Store) -> bool {
    installing(s) || !s.upd.checking.is_empty()
}

pub(super) fn busy(s: &Store) -> bool {
    own_busy(s) || s.other.is_some()
}
