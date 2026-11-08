//! Game controllers, changed: takes a controller that has a profile and hands games a changed one.
// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::fs::File;
use std::io::Read;
use std::os::fd::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;
use std::sync::mpsc;
use std::sync::Arc;
use std::time::{Duration, Instant};

use smithay::reexports::calloop::channel;
use smithay::reexports::calloop::generic::Generic;
use smithay::reexports::calloop::timer::{TimeoutAction, Timer};
use smithay::reexports::calloop::{Interest, Mode, PostAction, RegistrationToken};
use tracing::{info, warn};

use crate::controller_wire::{self as wire, Axis, Client, Event, Refusal, EV_SYN};
use crate::pads::{self, Change, Changer, Layout, Out, Range, EV_ABS, EV_KEY};
use crate::state::{AnvilState, Backend};

const SCAN_EVERY: Duration = Duration::from_secs(2);

const QUIET_MOST: Duration = Duration::from_secs(60);

const EVENT_BYTES: usize = 24;

const _: () = assert!(std::mem::size_of::<libc::input_event>() == EVENT_BYTES);
const BUTTONS: usize = 96;
const AXES: usize = 8;

const GRAB: libc::c_ulong = 0x4004_4590;
const ID: libc::c_ulong = 0x8008_4502;
fn bits(kind: u16, bytes: usize) -> libc::c_ulong {
    0x8000_0000 | ((bytes as libc::c_ulong) << 16) | 0x4500 | (0x20 + libc::c_ulong::from(kind))
}
fn absolute(axis: u16) -> libc::c_ulong {
    0x8000_0000 | (24 << 16) | 0x4500 | (0x40 + libc::c_ulong::from(axis))
}

#[derive(Default)]
pub struct Pads {
    held: Vec<Held>,

    client: Option<Arc<Client>>,
    desk: Option<Desk>,

    asked: Vec<(String, u8)>,

    refused: Vec<(String, Vec<Change>)>,

    quiet_until: Option<Instant>,

    fails: u32,

    games: Arc<Games>,
}

#[derive(Default)]
struct Games {
    wanted: std::sync::atomic::AtomicBool,
    running: std::sync::Mutex<Vec<u32>>,
}

impl Games {
    fn now(&self) -> Vec<u32> {
        self.running.lock().map(|r| r.clone()).unwrap_or_default()
    }

    fn watch(me: Arc<Games>) {
        let _ = std::thread::Builder::new().name("pad-games".into()).spawn(move || loop {
            let seen = if me.wanted.load(std::sync::atomic::Ordering::Relaxed) { pads::steam_games_now(std::path::Path::new("/proc")) } else { Vec::new() };
            if let Ok(mut running) = me.running.lock() {
                *running = seen;
            }
            std::thread::sleep(SCAN_EVERY);
        });
    }
}

enum Job {
    Make { slot: u8, pad: wire::Pad, pending: Box<Pending> },
    Remove(u8),
    Drop,
}

struct Pending {
    node: String,
    file: File,
    slot: u8,
    changes: Vec<Change>,
    keys: Vec<u16>,
    axes: Vec<Axis>,
}

enum Reply {
    Made { pending: Box<Pending>, client: Option<Arc<Client>>, outcome: std::io::Result<Result<(), Refusal>> },

    Lost,
}

struct Desk {
    jobs: mpsc::Sender<Job>,
}

impl Desk {

    fn start(connect: Box<dyn Fn() -> std::io::Result<Client> + Send>, tell: Box<dyn Fn(Reply) + Send>) -> Desk {
        let (jobs, queue) = mpsc::channel::<Job>();
        let _ = std::thread::Builder::new().name("pad-desk".into()).spawn(move || {
            let mut client: Option<Arc<Client>> = None;
            for job in queue {
                match job {
                    Job::Make { slot, pad, pending } => {
                        let have = match client.clone() {
                            Some(c) => Ok(c),
                            None => connect().map(Arc::new),
                        };
                        let (now, outcome) = match have {
                            Ok(c) => match c.make(slot, &pad) {
                                Ok(answer) => (Some(c), Ok(answer)),

                                Err(e) => (None, Err(e)),
                            },
                            Err(e) => (None, Err(e)),
                        };
                        client = now.clone();
                        tell(Reply::Made { pending, client: now, outcome });
                    }
                    Job::Remove(slot) => {
                        if client.as_ref().is_some_and(|c| c.remove(slot).is_err()) {
                            client = None;
                            tell(Reply::Lost);
                        }
                    }
                    Job::Drop => client = None,
                }
            }
        });
        Desk { jobs }
    }

    fn ask(&self, job: Job) {
        let _ = self.jobs.send(job);
    }
}

impl Pads {

    fn quiet(fails: u32) -> Duration {
        SCAN_EVERY.saturating_mul(1 << fails.min(6)).min(QUIET_MOST)
    }

    fn unanswered(&mut self, now: Instant) {
        self.client = None;
        self.fails = self.fails.saturating_add(1);
        self.quiet_until = Some(now + Pads::quiet(self.fails));
    }
}

struct Held {
    node: String,
    file: File,
    slot: u8,
    changes: Vec<Change>,
    changer: Changer,
    keys: Vec<u16>,
    axes: Vec<u16>,

    layout: Layout,
    batch: Vec<Event>,

    source: Option<RegistrationToken>,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Passed {
    to_pad: Vec<Event>,
    to_door: Vec<(u32, bool)>,
}

impl Held {

    fn pass(&mut self, events: &[(u16, u16, i32)]) -> Passed {
        let mut passed = Passed::default();
        for &(kind, code, value) in events {
            if kind == EV_SYN {
                if code == 0 && !self.batch.is_empty() {
                    passed.to_pad.append(&mut self.batch);
                    passed.to_pad.push(Event { kind: EV_SYN, code: 0, value: 0 });
                }
                continue;
            }
            let known = match kind {
                EV_KEY => self.keys.contains(&code) && matches!(value, 0 | 1),
                EV_ABS => self.axes.contains(&code),
                _ => false,
            };
            if !known {
                continue;
            }
            let table = if kind == EV_ABS { self.layout.table(code) } else { code };
            for out in self.changer.event(kind, table, value) {
                match out {
                    Out::Pad(kind, code, value) => {
                        let code = if kind == EV_ABS { self.layout.real(code) } else { code };
                        self.batch.push(Event { kind, code, value });
                    }
                    Out::Key(code, down) => passed.to_door.push((code, down)),
                }
            }
        }
        passed
    }
}

impl std::fmt::Debug for Pads {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} game controllers taken", self.held.len())
    }
}

impl<B: Backend + 'static> AnvilState<B> {

    pub fn watch_pads(&mut self) {
        let (tell, inbox) = channel::channel::<Reply>();
        let _ = self.handle.insert_source(inbox, |event, _, state| {
            if let channel::Event::Msg(reply) = event {
                state.pad_reply(reply);
            }
        });
        self.pads.desk = Some(Desk::start(Box::new(Client::connect), Box::new(move |reply| drop(tell.send(reply)))));
        Games::watch(self.pads.games.clone());
        let _ = self.handle.insert_source(Timer::from_duration(SCAN_EVERY), |_, _, state| {
            state.scan_pads();
            TimeoutAction::ToDuration(SCAN_EVERY)
        });
    }

    fn scan_pads(&mut self) {
        let profiles = pads::load().unwrap_or_default();
        let running = self.pads.games.now();
        self.pads.games.wanted.store(!profiles.off && profiles.pads.iter().any(|p| !p.changes.is_empty()), std::sync::atomic::Ordering::Relaxed);
        let mut wanted: Vec<(String, Vec<Change>)> = Vec::new();
        for (pad, node) in pads::listed() {
            let Some(node) = node else { continue };
            if pad.name == wire::NAME {
                continue;
            }

            if let Some(changes) = profiles.in_force(pad.vendor, pad.product, &pad.name, &running).filter(|c| !c.is_empty()) {
                wanted.push((node, changes.to_vec()));
            }
        }
        let gone: Vec<usize> = (0..self.pads.held.len())
            .filter(|i| !wanted.iter().any(|w| w.0 == self.pads.held[*i].node && w.1 == self.pads.held[*i].changes))
            .collect();
        for i in gone.into_iter().rev() {
            self.let_go_pad(i);
        }
        self.pads.refused.retain(|r| wanted.contains(r));
        let now = Instant::now();
        for (node, changes) in wanted {
            if self.pads.held.iter().any(|h| h.node == node) || self.pads.asked.iter().any(|a| a.0 == node) || self.pads.refused.iter().any(|r| r.0 == node && r.1 == changes) {
                continue;
            }
            if self.pads.quiet_until.is_some_and(|t| now < t) || self.pads.held.len() + self.pads.asked.len() >= usize::from(wire::SLOTS) {
                break;
            }
            if let Err(why) = self.ask_for_pad(&node, changes) {
                warn!("a controller was not taken ({node}): {why}");
            }
        }
        if self.pads.held.is_empty() && self.pads.asked.is_empty() {
            self.pads.client = None;
            if let Some(desk) = &self.pads.desk {
                desk.ask(Job::Drop);
            }
        }
    }

    fn let_go_pad(&mut self, i: usize) {
        let held = self.pads.held.remove(i);
        if let Some(source) = held.source {
            self.handle.remove(source);
        }
        self.forget_slot(held.slot);
        info!("a controller was let go: {}", held.node);
    }

    fn forget_slot(&mut self, slot: u8) {
        if let Some(desk) = &self.pads.desk {
            desk.ask(Job::Remove(slot));
        }
    }

    fn ask_for_pad(&mut self, node: &str, changes: Vec<Change>) -> Result<(), std::io::Error> {
        let file = std::fs::OpenOptions::new().read(true).custom_flags(libc::O_NONBLOCK | libc::O_CLOEXEC).open(format!("/dev/input/{node}"))?;
        let fd = file.as_raw_fd();
        let mut id = [0u16; 4];
        // SAFETY: the number says an 8 byte read, and id is 8 bytes.
        check(unsafe { libc::ioctl(fd, ID, id.as_mut_ptr()) })?;
        let mut button_bits = [0u8; BUTTONS];
        // SAFETY: the number says BUTTONS bytes and the array has them.
        check(unsafe { libc::ioctl(fd, bits(EV_KEY, BUTTONS), button_bits.as_mut_ptr()) })?;
        let keys: Vec<u16> = (0..(BUTTONS * 8) as u16).filter(|c| (button_bits[usize::from(*c) / 8] >> (c % 8)) & 1 == 1 && wire::is_pad_button(*c)).take(wire::MOST_KEYS).collect();
        let mut axis_bits = [0u8; AXES];
        // SAFETY: as above, AXES bytes.
        check(unsafe { libc::ioctl(fd, bits(EV_ABS, AXES), axis_bits.as_mut_ptr()) })?;
        let mut axes = Vec::new();
        for code in (0..(AXES * 8) as u16).filter(|c| (axis_bits[usize::from(*c) / 8] >> (c % 8)) & 1 == 1 && wire::is_pad_axis(*c)).take(wire::MOST_AXES) {
            let mut info = [0i32; 6];
            // SAFETY: the number says 24 bytes, a struct input_absinfo, and info is that.
            check(unsafe { libc::ioctl(fd, absolute(code), info.as_mut_ptr()) })?;
            axes.push(Axis { code, min: info[1], max: info[2], fuzz: info[3], flat: info[4] });
        }
        let made = wire::Pad { bus: id[0], vendor: id[1], product: id[2], version: id[3], keys: keys.clone(), axes: axes.clone() };
        let slot = (0..wire::SLOTS)
            .find(|s| !self.pads.held.iter().any(|h| h.slot == *s) && !self.pads.asked.iter().any(|a| a.1 == *s))
            .ok_or(std::io::ErrorKind::OutOfMemory)?;
        let Some(desk) = &self.pads.desk else { return Err(std::io::ErrorKind::NotConnected.into()) };
        self.pads.asked.push((node.to_string(), slot));
        desk.ask(Job::Make { slot, pad: made, pending: Box::new(Pending { node: node.to_string(), file, slot, changes, keys, axes }) });
        Ok(())
    }

    fn pad_reply(&mut self, reply: Reply) {
        let now = Instant::now();
        match reply {
            Reply::Lost => {
                warn!("the controller service did not answer a removal; asked again in {:?}", Pads::quiet(self.pads.fails + 1));
                self.pads.unanswered(now);
            }
            Reply::Made { pending, client, outcome } => {
                self.pads.asked.retain(|a| a.1 != pending.slot);
                let node = pending.node.clone();
                match outcome {
                    Err(why) => {
                        warn!("the controller service did not answer ({node}): {why}; asked again in {:?}", Pads::quiet(self.pads.fails + 1));
                        self.pads.unanswered(now);
                    }
                    Ok(Err(why)) => {
                        self.pads.client = client;
                        warn!("a controller was refused ({node}): {why:?}; not asked again for this profile");
                        self.pads.refused.push((node, pending.changes));
                    }
                    Ok(Ok(())) => {
                        self.pads.client = client;

                        let slot = pending.slot;
                        let result = if self.pads.held.iter().any(|h| h.node == node) { Err((slot, std::io::ErrorKind::AlreadyExists.into())) } else { self.take_pad(*pending) };
                        match result {
                            Ok(()) => self.pads.fails = 0,
                            Err((slot, e)) => {
                                self.forget_slot(slot);
                                warn!("a controller was not taken ({node}): {e}");
                            }
                        }
                    }
                }
            }
        }
    }

    fn take_pad(&mut self, p: Pending) -> Result<(), (u8, std::io::Error)> {
        let Pending { node, file, slot, changes, keys, axes } = p;
        let fd = file.as_raw_fd();
        let watching = (|| -> std::io::Result<RegistrationToken> {
            // SAFETY: GRAB takes a number, 1 to take the controller for this program alone.
            check(unsafe { libc::ioctl(fd, GRAB, 1 as libc::c_int) })?;
            let watched = Generic::new(file.try_clone()?, Interest::READ, Mode::Level);
            let name = node.clone();
            self.handle
                .insert_source(watched, move |_, _, state| Ok(if state.read_pad(&name) { PostAction::Continue } else { PostAction::Remove }))
                .map_err(|e| std::io::Error::other(e.to_string()))
        })();

        let source = watching.map_err(|e| (slot, e))?;
        let layout = Layout::of(axes.iter().filter(|a| a.code < 64).fold(0u64, |has, a| has | (1 << a.code)));
        let ranges: Vec<(u16, Range)> = axes.iter().map(|a| (layout.table(a.code), Range { min: a.min, max: a.max })).collect();
        let changer = Changer::new(&changes, &ranges);
        self.pads.held.push(Held { node: node.clone(), file, slot, changes, changer, keys, axes: axes.iter().map(|a| a.code).collect(), layout, batch: Vec::new(), source: Some(source) });
        info!("a controller was taken: {node}");
        Ok(())
    }

    fn read_pad(&mut self, node: &str) -> bool {
        let Some(i) = self.pads.held.iter().position(|h| h.node == node) else { return false };
        let mut events = Vec::new();
        let mut chunk = [0u8; EVENT_BYTES * 64];
        let alive = loop {
            match self.pads.held[i].file.read(&mut chunk) {
                Ok(0) => break false,
                Ok(n) => {
                    for e in chunk[..n].chunks_exact(EVENT_BYTES) {
                        events.push((u16::from_ne_bytes([e[16], e[17]]), u16::from_ne_bytes([e[18], e[19]]), i32::from_ne_bytes([e[20], e[21], e[22], e[23]])));
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break true,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => break false,
            }
        };
        let passed = self.pads.held[i].pass(&events);
        let slot = self.pads.held[i].slot;
        if !passed.to_pad.is_empty() {
            if let Some(client) = &self.pads.client {
                let _ = client.send(slot, &passed.to_pad);
            }
        }
        for (code, down) in passed.to_door {
            self.pad_press(code, down);
        }
        if !alive {

            let held = self.pads.held.remove(i);
            self.forget_slot(held.slot);
            info!("a controller went: {node}");
        }
        alive
    }
}

fn check(r: libc::c_int) -> std::io::Result<()> {
    if r < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOUTH: u16 = 0x130;
    const EAST: u16 = 0x131;
    const SELECT: u16 = 0x13a;

    fn held(text: &str) -> Held {
        let profiles = pads::Profiles::read(&format!("[054c:09cc Pad]\n{text}"));
        let changes = profiles.in_force(0x054c, 0x09cc, "Pad", &[]).unwrap_or_default().to_vec();
        let ranges = [(0, Range { min: -32768, max: 32767 })];
        let file = File::open("/dev/null").unwrap();
        Held {
            node: "event0".into(),
            file,
            slot: 0,
            changer: Changer::new(&changes, &ranges),
            changes,
            keys: vec![SOUTH, EAST, SELECT],
            axes: vec![0],
            layout: Layout::STANDARD,
            batch: Vec::new(),
            source: None,
        }
    }

    #[test]
    fn a_button_into_a_letter_goes_to_the_door_and_not_to_the_made_controller() {
        let mut h = held("Select = Key A");
        let a = h.pass(&[(EV_KEY, SELECT, 1), (EV_SYN, 0, 0), (EV_KEY, SELECT, 0), (EV_SYN, 0, 0)]);
        assert_eq!(a.to_door, [(30, true), (30, false)], "the letter A is key 30");
        assert!(a.to_pad.is_empty(), "the button also went to the made controller: {:?}", a.to_pad);
    }

    #[test]
    fn standard_takes_nothing_and_passes_every_button_as_it_came() {
        let profiles = pads::Profiles::read("[054c:09cc Pad]\nProfile = Sands\nSelect = Key A\n[Games]\n13600 = Sands\n");
        assert!(profiles.in_force(0x054c, 0x09cc, "Pad", &[1593500]).is_none(), "God of War got the other game's keys");
        assert!(profiles.in_force(0x054c, 0x09cc, "Pad", &[]).is_none());
        assert!(profiles.in_force(0x054c, 0x09cc, "Pad", &[13600]).is_some());
        let mut h = held("");
        let presses: Vec<(u16, u16, i32)> = [SOUTH, EAST, SELECT].iter().flat_map(|b| [(EV_KEY, *b, 1), (EV_KEY, *b, 0)]).chain([(EV_ABS, 0, -200), (EV_SYN, 0, 0)]).collect();
        let a = h.pass(&presses);
        let mut sent: Vec<(u16, u16, i32)> = a.to_pad.iter().map(|e| (e.kind, e.code, e.value)).collect();
        assert_eq!(sent.pop(), Some((EV_SYN, 0, 0)));
        assert_eq!(sent, presses[..presses.len() - 1], "Standard changed what the controller said");
        assert!(a.to_door.is_empty());
    }

    #[test]
    fn what_is_not_changed_passes_on_with_a_sync_at_the_end() {
        let mut h = held("South = East");
        let a = h.pass(&[(EV_KEY, SOUTH, 1), (EV_SYN, 0, 0)]);
        assert_eq!(a.to_pad, [Event { kind: EV_KEY, code: EAST, value: 1 }, Event { kind: EV_SYN, code: 0, value: 0 }]);
        assert!(a.to_door.is_empty());
    }

    #[test]
    fn a_controller_with_its_right_stick_on_other_axes_is_changed_by_the_tables_names() {
        let mut h = held("Right Stick Y = inverted");
        h.layout = Layout::of([0u16, 1, 2, 5, 9, 10].iter().fold(0u64, |has, a| has | (1 << a)));
        h.axes = vec![0, 1, 2, 5, 9, 10];
        h.changer = Changer::new(&h.changes, &[(4, Range { min: 0, max: 255 })]);
        let a = h.pass(&[(EV_ABS, 5, 10), (EV_SYN, 0, 0)]);
        assert_eq!(a.to_pad, [Event { kind: EV_ABS, code: 5, value: 245 }, Event { kind: EV_SYN, code: 0, value: 0 }]);
    }

    #[test]
    fn nothing_the_made_controller_was_not_made_with_is_passed_on() {
        let mut h = held("South = East");

        let a = h.pass(&[(EV_KEY, 0x13f, 1), (EV_KEY, SOUTH, 2), (EV_ABS, 9, 5), (4, 4, 7), (EV_SYN, 0, 0)]);
        assert_eq!(a, Passed::default());
    }

    #[test]
    fn a_silent_service_is_asked_less_and_less_often() {
        let quiets: Vec<Duration> = (1..12).map(Pads::quiet).collect();
        assert_eq!(quiets[0], SCAN_EVERY * 2);
        assert!(quiets.windows(2).all(|w| w[0] <= w[1]));
        assert_eq!(*quiets.last().unwrap(), QUIET_MOST);
        let mut pads = Pads::default();
        let now = Instant::now();
        pads.unanswered(now);
        pads.unanswered(now);
        assert_eq!(pads.quiet_until, Some(now + SCAN_EVERY * 4));
        assert!(pads.client.is_none(), "a connection that gave no answer is never asked again");
    }

    #[test]
    fn the_loop_does_not_wait_for_the_service() {
        let (client, seen) = wire::double().unwrap();
        let client = std::sync::Mutex::new(Some(client));
        let (tell, heard) = mpsc::channel::<Reply>();
        let tell = std::sync::Mutex::new(tell);
        let desk = Desk::start(
            Box::new(move || {
                std::thread::sleep(Duration::from_millis(300));
                client.lock().unwrap().take().ok_or_else(|| std::io::ErrorKind::NotConnected.into())
            }),
            Box::new(move |reply| drop(tell.lock().unwrap().send(reply))),
        );
        let pad = wire::Pad { bus: 3, vendor: 1, product: 2, version: 1, keys: vec![SOUTH], axes: Vec::new() };
        let pending = Box::new(Pending { node: "event0".into(), file: File::open("/dev/null").unwrap(), slot: 0, changes: Vec::new(), keys: vec![SOUTH], axes: Vec::new() });
        let began = Instant::now();
        desk.ask(Job::Make { slot: 0, pad, pending });
        desk.ask(Job::Remove(0));
        assert!(began.elapsed() < Duration::from_millis(100), "the loop waited {:?}", began.elapsed());
        let Reply::Made { client, outcome, .. } = heard.recv_timeout(Duration::from_secs(5)).unwrap() else { panic!("a make was answered with something else") };
        assert!(matches!(outcome, Ok(Ok(()))) && client.is_some());
        drop(client);
        drop(desk);
        let asked = seen.join().unwrap();
        assert!(matches!(asked.as_slice(), [wire::Request::Make { slot: 0, .. }, wire::Request::Remove { slot: 0 }]), "{asked:?}");
    }
}
