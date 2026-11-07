//! The ScreenCast portal.

mod casting;
mod desktop;
mod methods;
mod pointer;
#[cfg(test)]
mod tests;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use zbus::zvariant::OwnedValue;

use crate::common::in_use;
use crate::toolkit::bus::{Asked, Offer, Props, Served};
use crate::toolkit::ui::Shown;

use methods::{create_session, select_sources, start};
use super::{cast_capture, cast_pick, gbm};
use super::cast_capture::Capture;
use super::cast_pick::{Choice, Picker};
use super::portal_ask::Kit;
use super::pw::connection::PipeWire;
use super::pw::stream::{self, Stream};
use super::pw::pod;

pub(super) const INTERFACE: &str = "org.freedesktop.impl.portal.ScreenCast";
const XML: &str = r#"<interface name="org.freedesktop.impl.portal.ScreenCast">
    <method name="CreateSession">
      <arg type="o" name="handle" direction="in"/>
      <arg type="o" name="session_handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <method name="SelectSources">
      <arg type="o" name="handle" direction="in"/>
      <arg type="o" name="session_handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <method name="Start">
      <arg type="o" name="handle" direction="in"/>
      <arg type="o" name="session_handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <property name="AvailableSourceTypes" type="u" access="read"/>
    <property name="AvailableCursorModes" type="u" access="read"/>
    <property name="version" type="u" access="read"/>
  </interface>"#;

const SESSION: &str = "org.freedesktop.impl.portal.Session";
const SESSION_XML: &str = r#"<interface name="org.freedesktop.impl.portal.Session">
    <method name="Close"/>
    <signal name="Closed"/>
    <property name="version" type="u" access="read"/>
  </interface>"#;

const MONITOR: u32 = 1;
const WINDOW: u32 = 2;
const HIDDEN: u32 = 1;
const EMBEDDED: u32 = 2;
const METADATA: u32 = 4;
const VENDOR: &str = "Trindle";
const RESTORE_VERSION: u32 = 1;
const START_WAIT: Duration = Duration::from_secs(15);
const RETRY: Duration = Duration::from_millis(8);
const FRAME_GAP: Duration = Duration::from_nanos(1_000_000_000 / 60);

type Kept = (String, String, String, String);

#[derive(Clone, Debug)]
struct Wants {
    types: u32,
    multiple: bool,
    cursor: u32,
    persist: u32,
    restore: Option<Vec<Kept>>,
}

impl Default for Wants {
    fn default() -> Wants {
        Wants { types: MONITOR, multiple: false, cursor: EMBEDDED, persist: 0, restore: None }
    }
}

struct Casting {
    kit: Kit,
    capture: RefCell<Option<Rc<Capture>>>,
    pw: RefCell<Option<Rc<PipeWire>>>,
    cards: RefCell<Vec<(u64, Rc<gbm::Device>)>>,
    sessions: RefCell<HashMap<String, Rc<Session>>>,
    me: Weak<Casting>,
}

struct Session {
    path: String,
    app_name: String,
    app_pid: Cell<Option<u32>>,
    wants: RefCell<Wants>,
    casts: RefCell<Vec<Rc<Cast>>>,
    served: RefCell<Option<Served>>,
    picker: RefCell<Option<Rc<Shown<Picker>>>>,
    request: RefCell<Option<Served>>,
    started: Cell<bool>,
    closed: Cell<bool>,
}

struct Part {
    capture: Option<u64>,
    size: (u32, u32),
    at: (i32, i32, u32, u32),
    memory: Option<crate::common::shm::Memory>,
    buffer: Option<cast_capture::Buffer>,
    busy: bool,
    fresh: Vec<(i32, i32, u32, u32)>,
}

struct Cast {
    choice: Choice,
    parts: RefCell<Vec<Part>>,
    desktop: bool,
    stream: RefCell<Option<Stream>>,
    size: Cell<(u32, u32)>,
    filling: RefCell<Option<stream::Buffer>>,
    spare: RefCell<Option<stream::Buffer>>,
    streaming: Cell<bool>,
    node: Cell<Option<u32>>,
    seq: Cell<u64>,
    retrying: Cell<bool>,
    failed: Cell<bool>,
    pointer: RefCell<Option<Pointer>>,
    sent: Cell<Option<Instant>>,
    gap: Cell<Duration>,
    held: RefCell<Option<(stream::Buffer, Vec<(i32, i32, u32, u32)>)>>,
    pacing: Cell<bool>,
}

impl Cast {
    fn new(choice: Choice, parts: Vec<Part>, desktop: bool, size: (u32, u32), pointer: Option<Pointer>) -> Cast {
        Cast {
            choice,
            parts: RefCell::new(parts),
            desktop,
            stream: RefCell::new(None),
            size: Cell::new(size),
            filling: RefCell::new(None),
            spare: RefCell::new(None),
            streaming: Cell::new(false),
            node: Cell::new(None),
            seq: Cell::new(0),
            retrying: Cell::new(false),
            failed: Cell::new(false),
            pointer: RefCell::new(pointer),
            sent: Cell::new(None),
            gap: Cell::new(FRAME_GAP),
            held: RefCell::new(None),
            pacing: Cell::new(false),
        }
    }
}

fn wait_for(sent: Option<Instant>, gap: Duration, now: Instant) -> Duration {
    sent.map_or(Duration::ZERO, |at| (at + gap).saturating_duration_since(now))
}

#[derive(Default)]
struct Pointer {
    follows: Vec<Follow>,
    over: bool,
    at: (i32, i32),
    hotspot: (i32, i32),
    picture: Vec<u8>,
    size: (u32, u32),
    pending: bool,
}

#[derive(Default)]
struct Follow {
    id: Option<u64>,
    over: bool,
    memory: Option<crate::common::shm::Memory>,
    buffer: Option<cast_capture::Buffer>,
    size: (u32, u32),
}

impl Pointer {
    fn meta(&self) -> pod::PointerMeta<'_> {
        pod::PointerMeta { over: self.over, at: self.at, hotspot: self.hotspot, picture: &self.picture, size: self.size }
    }
}

fn monotonic_ns() -> i64 {
    let mut ts = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: fills the timespec this owns; CLOCK_MONOTONIC always exists.
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut ts) };
    ts.tv_sec * 1_000_000_000 + ts.tv_nsec
}

fn sender_of(path: &str) -> Option<String> {
    let mut parts = path.rsplit('/');
    parts.next()?;
    let who = parts.next()?;
    (!who.is_empty()).then(|| format!(":{}", who.replace('_', ".")))
}

fn tell_in_use(casting: &Casting) {
    let casts: Vec<in_use::Cast> = casting.sessions.borrow().values()
        .filter(|s| s.started.get() && !s.closed.get())
        .flat_map(|s| s.casts.borrow().iter().map(|_| in_use::Cast { pid: s.app_pid.get(), name: s.app_name.clone() }).collect::<Vec<_>>())
        .collect();
    let path = in_use::casts_path();
    if casts.is_empty() {
        let _ = std::fs::remove_file(&path);
        return;
    }
    let tmp = path.with_extension("new");
    if std::fs::write(&tmp, in_use::encode_casts(std::process::id(), &casts)).is_ok() {
        let _ = std::fs::rename(&tmp, &path);
    }
}

pub(super) fn offer(kit: &Kit) -> Offer {
    let casting = Rc::new_cyclic(|me| Casting {
        kit: kit.clone(),
        capture: RefCell::new(None),
        pw: RefCell::new(None),
        cards: RefCell::new(Vec::new()),
        sessions: RefCell::new(HashMap::new()),
        me: me.clone(),
    });
    let c = casting.clone();
    Offer {
        name: INTERFACE.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| {
            c.kit.stay.last.set(std::time::Instant::now());
            match asked.member().as_str() {
                "CreateSession" => create_session(&c, asked),
                "SelectSources" => select_sources(&c, asked),
                "Start" => start(&c, asked),
                _ => asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "No such method"),
            }
        }),
        props: Box::new(move || {
            Props::from([
                ("AvailableSourceTypes".to_string(), OwnedValue::from(MONITOR | WINDOW)),
                ("AvailableCursorModes".to_string(), OwnedValue::from(HIDDEN | EMBEDDED | METADATA)),
                ("version".to_string(), OwnedValue::from(5u32)),
            ])
        }),
        set: None,
    }
}
