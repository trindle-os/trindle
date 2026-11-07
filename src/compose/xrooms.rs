// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Older programs' Xwayland servers: the desktop's own, for programs started
//! outside any box, and one of its own for each box that asks, shared by
//! every launch of that box. Each waits as a stand-in until its first older
//! program connects (xwayland_later). The desktop's stops a while after its
//! last program closes and waits again on the same display; a box's lasts
//! while the box holds it, and after that while any program is still on it.
//! One that ends before it is ready is started again.

use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::net::{UnixListener, UnixStream};
use std::process::Stdio;
use std::time::{Duration, Instant};

use smithay::{
    input::pointer::CursorImageStatus,
    reexports::calloop::{
        generic::Generic,
        timer::{TimeoutAction, Timer},
        Interest, Mode, PostAction, RegistrationToken,
    },
    reexports::wayland_server::Client,
    utils::{Point, Size},
    wayland::{compositor::CompositorHandler, selection::SelectionTarget},
    xwayland::{xwm::XwmId, X11Surface, X11Wm, XWayland, XWaylandClientData, XWaylandEvent},
};
use tracing::{info, warn};

use crate::cursor::Cursor;
use crate::focus::KeyboardFocusTarget;
use crate::shell::WindowElement;
use crate::state::{selection_slot, AnvilState, Backend};
use crate::x11_rooms::{self, Answer, Told};

pub const STOP_AFTER_SECS: u32 = 10;
pub const FIRST_PROGRAM_SECS: u32 = 60;
const RETIRE_SECS: u64 = 60;
const WATCH_EVERY: Duration = Duration::from_secs(2);
const RESTART_AFTER: Duration = Duration::from_millis(500);
const FAILURES_MAX: usize = 3;
const RESTART_WINDOW: Duration = Duration::from_secs(60);

fn programs_connected(n: u32) -> Option<usize> {
    use smithay::reexports::x11rb::{
        protocol::res::{ClientIdMask, ClientIdSpec, ConnectionExt},
        rust_connection::RustConnection,
    };
    let (conn, _) = RustConnection::connect(Some(&format!(":{n}"))).ok()?;
    let clients = conn.res_query_clients().ok()?.reply().ok()?.clients;
    let me = std::process::id();
    let mut others = 0;
    for c in clients {
        if c.resource_base == 0 {
            continue;
        }
        let spec = ClientIdSpec { client: c.resource_base, mask: ClientIdMask::LOCAL_CLIENT_PID };
        let ids = conn.res_query_client_ids(&[spec]).ok()?.reply().ok()?.ids;
        let pid = ids.first().and_then(|v| v.value.first()).copied();
        if pid != Some(me) {
            others += 1;
        }
    }
    Some(others)
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Seen {
    pub running: bool,
    pub starting: bool,
    pub programs: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Keep {
    pub owner: Option<String>,
    holders: Vec<u64>,
    idle_since: Option<Instant>,
    failures: Vec<Instant>,
}

impl Keep {
    pub fn new(owner: Option<String>) -> Keep {
        Keep { owner, ..Keep::default() }
    }

    pub fn held(&self) -> bool {
        !self.holders.is_empty()
    }

    pub fn join(&mut self, holder: u64) {
        self.holders.push(holder);
        self.idle_since = None;
    }

    pub fn leave(&mut self, holder: u64, now: Instant) -> bool {
        let before = self.holders.len();
        self.holders.retain(|h| *h != holder);
        let left = self.holders.len() != before;
        if left && self.holders.is_empty() {
            self.idle_since = Some(now);
        }
        left
    }

    pub fn stops(&mut self, seen: Seen, now: Instant) -> bool {
        if self.held() {
            self.idle_since = None;
            return false;
        }
        let wait = if seen.running {
            match seen.programs {
                Some(0) => STOP_AFTER_SECS,
                Some(_) => {
                    self.idle_since = None;
                    return false;
                }
                None => return false,
            }
        } else if self.owner.is_none() || seen.starting {
            return false;
        } else {
            FIRST_PROGRAM_SECS
        };
        let since = *self.idle_since.get_or_insert(now);
        now.saturating_duration_since(since) >= Duration::from_secs(wait as u64)
    }

    pub fn failed(&mut self, now: Instant) -> bool {
        self.failures.retain(|t| now.saturating_duration_since(*t) < RESTART_WINDOW);
        self.failures.push(now);
        self.failures.len() >= FAILURES_MAX
    }
}

pub fn room_of<'a>(keeps: impl IntoIterator<Item = &'a Keep>, name: &str) -> Option<usize> {
    keeps.into_iter().position(|k| k.owner.as_deref() == Some(name))
}

#[derive(Debug)]
pub struct Room {
    key: u64,
    pub display: u32,
    keep: Keep,
    source: Option<RegistrationToken>,
    pub wm: Option<X11Wm>,
    client: Client,
    holders: Vec<(u64, UnixStream)>,
}

impl Room {
    fn owner(&self) -> &str {
        self.keep.owner.as_deref().unwrap_or("the desktop")
    }
}

type Parked = (u32, Keep, Vec<(u64, UnixStream)>);

#[derive(Debug, Default)]
pub struct Rooms {
    pub list: Vec<Room>,
    next: u64,
    next_holder: u64,
    parked: usize,
    left: Vec<u64>,
    pub desktop_display: Option<u32>,
    retired: Vec<X11Wm>,
    pub selection_from: [Option<(XwmId, Vec<String>)>; 2],
}

impl Rooms {
    pub fn display_of(&self, id: XwmId) -> Option<u32> {
        self.list.iter().find(|r| r.wm.as_ref().is_some_and(|w| w.id() == id)).map(|r| r.display)
    }

    pub fn wm(&mut self, id: XwmId) -> Option<&mut X11Wm> {
        self.list.iter_mut().filter_map(|r| r.wm.as_mut()).chain(self.retired.iter_mut()).find(|w| w.id() == id)
    }

    pub fn running(&mut self) -> impl Iterator<Item = &mut X11Wm> {
        self.list.iter_mut().filter_map(|r| r.wm.as_mut())
    }

    pub fn boxes(&self) -> usize {
        self.list.iter().filter(|r| r.keep.owner.is_some()).count()
    }

    pub fn has_desktop(&self) -> bool {
        self.list.iter().any(|r| r.keep.owner.is_none())
    }
}

fn same_person(stream: &UnixStream) -> bool {
    let mut cred = libc::ucred { pid: 0, uid: u32::MAX, gid: u32::MAX };
    let mut len = std::mem::size_of::<libc::ucred>() as libc::socklen_t;
    // SAFETY: the fd is an open socket for the call, and cred and len are
    // valid, with len holding cred's size.
    let rc = unsafe {
        libc::getsockopt(stream.as_raw_fd(), libc::SOL_SOCKET, libc::SO_PEERCRED,
                         (&mut cred as *mut libc::ucred).cast(), &mut len)
    };
    // SAFETY: geteuid takes nothing and cannot fail.
    rc == 0 && len as usize == std::mem::size_of::<libc::ucred>() && cred.uid == unsafe { libc::geteuid() }
}

fn woken(client: &Client) -> bool {
    let Some(pid) = client.get_data::<XWaylandClientData>().and_then(|d| d.pid()) else { return false };
    let (Ok(now), Ok(me)) = (std::fs::read_link(format!("/proc/{pid}/exe")), std::env::current_exe()) else {
        return false;
    };
    now != me
}

fn tell_holders(holders: &[(u64, UnixStream)], told: Told) {
    for (_, s) in holders {
        let mut s: &UnixStream = s;
        let _ = s.write_all(told.line().as_bytes());
    }
}

fn tell_person() {
    std::thread::spawn(|| {
        let _ = std::process::Command::new("busctl")
            .args(["--user", "call", "org.freedesktop.Notifications", "/org/freedesktop/Notifications",
                   "org.freedesktop.Notifications", "Notify", "susssasa{sv}i", "Trindle", "0", "dialog-warning-symbolic",
                   x11_rooms::would_not_start(), x11_rooms::try_again(), "0", "0", "-1"])
            .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status();
    });
}

impl<BackendData: Backend + 'static> AnvilState<BackendData> {
    pub fn start_xwayland(&mut self) {
        self.listen_for_rooms();
        self.watch_rooms();
        self.watch_pads();
        if !self.config.older_programs {
            info!("older programs are off; no Xwayland waits");
            self.session_ready();
            return;
        }
        let later = crate::xwayland_later::path().is_some();
        match self.open_room(None, None) {
            Ok(n) if later => {
                self.xdisplay = Some(n);
                self.session_ready();
            }
            Ok(_) => {}
            Err(e) => {
                warn!("no Xwayland for older programs: {e}");
                self.session_ready();
            }
        }
    }

    fn watch_rooms(&mut self) {
        let _ = self.handle.insert_source(Timer::from_duration(WATCH_EVERY), |_, _, state| {
            state.stop_idle_rooms();
            TimeoutAction::ToDuration(WATCH_EVERY)
        });
    }

    fn stop_idle_rooms(&mut self) {
        let now = Instant::now();
        let mut stopping = Vec::new();
        for room in &mut self.rooms.list {
            let running = room.wm.is_some();
            let free = !room.keep.held();
            let seen = Seen {
                running,
                starting: !running && free && room.keep.owner.is_some() && woken(&room.client),
                programs: if running && free { programs_connected(room.display) } else { None },
            };
            if room.keep.stops(seen, now) {
                stopping.push((room.key, room.keep.owner.is_none(), room.wm.as_ref().map(|w| w.id())));
            }
        }
        for (key, desktop, wm) in stopping {
            match wm {
                Some(id) if desktop => self.room_lost(id),
                _ => self.close_room(key),
            }
        }
    }

    fn open_room(&mut self, owner: Option<String>, display: Option<u32>) -> Result<u32, String> {
        let later = crate::xwayland_later::path();
        if owner.is_some() && later.is_none() {
            return Err(format!("{} is not installed", crate::xwayland_later::WAITING_ROOM));
        }
        let mut envs: Vec<(String, String)> = later.into_iter().map(|p| ("PATH".to_string(), p)).collect();
        if owner.is_none() {
            envs.push((crate::xwayland_later::STOP_ENV.to_string(), STOP_AFTER_SECS.to_string()));
        }
        let (xwayland, client) =
            XWayland::spawn(&self.display_handle, display, envs, false, Stdio::null(), Stdio::null(), |_| ())
                .map_err(|e| e.to_string())?;
        let number = xwayland.display_number();
        let key = self.rooms.next;
        self.rooms.next += 1;
        let ready_client = client.clone();
        let token = self
            .handle
            .insert_source(xwayland, move |event, _, state| match event {
                XWaylandEvent::Ready { x11_socket, .. } => state.room_ready(key, x11_socket, ready_client.clone()),
                XWaylandEvent::Error => state.room_failed(key),
            })
            .map_err(|e| e.to_string())?;
        if owner.is_none() {
            self.rooms.desktop_display = Some(number);
        }
        info!(display = number, owner = owner.as_deref().unwrap_or("the desktop"), "Xwayland waiting");
        self.rooms.list.push(Room {
            key,
            display: number,
            keep: Keep::new(owner),
            source: Some(token),
            wm: None,
            client,
            holders: Vec::new(),
        });
        Ok(number)
    }

    fn room_ready(&mut self, key: u64, socket: UnixStream, client: Client) {
        let scale = std::env::var("ANVIL_XWAYLAND_SCALE").ok().and_then(|s| s.parse::<f64>().ok()).unwrap_or(1.);
        self.client_compositor_state(&client).set_client_scale(scale);
        let mut wm = match X11Wm::start_wm(self.handle.clone(), socket, client) {
            Ok(wm) => wm,
            Err(err) => {
                warn!(?err, "could not be Xwayland's window manager");
                self.room_failed(key);
                return;
            }
        };
        let mut cursor = Cursor::load();
        let image = cursor.get_image(1, Duration::ZERO);
        let size = Size::from((image.width as u16, image.height as u16));
        if let Err(err) = wm.set_cursor(&image.pixels_rgba, size, Point::from((image.xhot as u16, image.yhot as u16))) {
            warn!(?err, "Xwayland's pointer not set");
        }
        for ty in [SelectionTarget::Clipboard, SelectionTarget::Primary] {
            let slot = selection_slot(ty);
            let offer = self.wayland_selection[slot].clone()
                .or_else(|| self.rooms.selection_from[slot].as_ref().map(|(_, m)| m.clone()));
            if let Some(mimes) = offer {
                if let Err(err) = wm.new_selection(ty, Some(mimes)) {
                    warn!(?err, ?ty, "Failed to set Xwayland selection");
                }
            }
        }
        let Some(room) = self.rooms.list.iter_mut().find(|r| r.key == key) else {
            self.rooms.retired.push(wm);
            return;
        };
        info!(display = room.display, owner = room.owner(), "Xwayland running");
        let desktop = room.keep.owner.is_none().then_some(room.display);
        room.wm = Some(wm);
        if let Some(n) = desktop {
            if self.xdisplay.is_none() {
                self.xdisplay = Some(n);
                self.session_ready();
            }
        }
    }

    pub fn close_room(&mut self, key: u64) {
        let Some(at) = self.rooms.list.iter().position(|r| r.key == key) else { return };
        let room = self.rooms.list.remove(at);
        self.retire(room);
    }

    fn retire(&mut self, room: Room) -> (u32, Keep, Vec<(u64, UnixStream)>) {
        info!(display = room.display, owner = room.owner(), "Xwayland stopped");
        let Room { display, keep, source, wm, holders, .. } = room;
        if let Some(token) = source {
            self.handle.remove(token);
        }
        if let Some(wm) = wm {
            let id = wm.id();
            self.forget_x11_windows(id);
            for ty in [SelectionTarget::Clipboard, SelectionTarget::Primary] {
                if self.rooms.selection_from[selection_slot(ty)].as_ref().is_some_and(|(from, _)| *from == id) {
                    self.selection_cleared(id, ty);
                }
            }
            self.rooms.retired.push(wm);
            let _ = self.handle.insert_source(Timer::from_duration(Duration::from_secs(RETIRE_SECS)), move |_, _, state| {
                state.rooms.retired.retain(|w| w.id() != id);
                TimeoutAction::Drop
            });
        }
        (display, keep, holders)
    }

    fn room_failed(&mut self, key: u64) {
        let Some(at) = self.rooms.list.iter().position(|r| r.key == key) else { return };
        let room = self.rooms.list.remove(at);
        warn!(display = room.display, owner = room.owner(), "Xwayland ended before it was ready");
        if self.startup.is_some() {
            self.session_ready();
        }
        self.start_again(room, true);
    }

    pub fn room_lost(&mut self, id: XwmId) {
        let Some(at) = self.rooms.list.iter().position(|r| r.wm.as_ref().is_some_and(|w| w.id() == id)) else { return };
        let room = self.rooms.list.remove(at);
        if room.keep.owner.is_some() {
            self.start_again(room, false);
            return;
        }
        self.retire(room);
        if self.config.older_programs {
            self.open_desktop_room();
        }
    }

    fn start_again(&mut self, room: Room, failed: bool) {
        let (display, mut keep, holders) = self.retire(room);
        let desktop = keep.owner.is_none();
        if !desktop && holders.is_empty() {
            return;
        }
        if !self.config.older_programs {
            tell_holders(&holders, Told::Failed);
            return;
        }
        let given_up = failed && keep.failed(Instant::now());
        if given_up {
            let number = display;
            warn!("Xwayland on :{number} failed {FAILURES_MAX} times within {} s", RESTART_WINDOW.as_secs());
            if !desktop {
                tell_holders(&holders, Told::Failed);
                return;
            }
            tell_person();
        } else {
            tell_holders(&holders, Told::Lost);
        }
        let wait = if given_up { RESTART_WINDOW } else { RESTART_AFTER };
        self.rooms.parked += 1;
        let mut parked: Option<Parked> = Some((display, keep, holders));
        let _ = self.handle.insert_source(Timer::from_duration(wait), move |_, _, state| {
            if let Some(p) = parked.take() {
                state.rooms.parked -= 1;
                state.reopen(p);
                if state.rooms.parked == 0 {
                    state.rooms.left.clear();
                }
            }
            TimeoutAction::Drop
        });
    }

    fn reopen(&mut self, (display, mut keep, mut holders): Parked) {
        let now = Instant::now();
        let mut gone = Vec::new();
        self.rooms.left.retain(|id| {
            let was = keep.leave(*id, now);
            if was {
                gone.push(*id);
            }
            !was
        });
        holders.retain(|(id, _)| !gone.contains(id));
        if !self.config.older_programs {
            tell_holders(&holders, Told::Failed);
            return;
        }
        let Some(name) = keep.owner.clone() else {
            if !self.rooms.has_desktop() {
                self.open_desktop_room();
                if let Some(room) = self.rooms.list.iter_mut().find(|r| r.keep.owner.is_none()) {
                    room.keep.failures = keep.failures;
                }
            }
            return;
        };
        if !keep.held() {
            return;
        }
        if let Some(at) = room_of(self.rooms.list.iter().map(|r| &r.keep), &name) {
            let room = &mut self.rooms.list[at];
            for (id, s) in holders {
                room.keep.join(id);
                room.holders.push((id, s));
            }
            return;
        }
        let opened = self.open_room(Some(name.clone()), Some(display)).or_else(|_| self.open_room(Some(name.clone()), None));
        match opened {
            Ok(n) => {
                info!(display = n, owner = name, "Xwayland started again");
                let room = self.rooms.list.last_mut().expect("the room just opened");
                room.keep = keep;
                room.holders = holders;
            }
            Err(e) => {
                warn!(owner = name, "Xwayland could not start again: {e}");
                tell_holders(&holders, Told::Failed);
            }
        }
    }

    fn open_desktop_room(&mut self) {
        let before = self.rooms.desktop_display;
        let opened = match self.open_room(None, before) {
            Err(e) if before.is_some() => {
                warn!("display :{} is taken ({e}); another is used", before.unwrap_or_default());
                self.open_room(None, None)
            }
            other => other,
        };
        match opened {
            Ok(n) if crate::xwayland_later::path().is_some() => {
                if self.xdisplay != Some(n) {
                    self.xdisplay = Some(n);
                    std::env::set_var("DISPLAY", format!(":{n}"));
                }
            }
            Ok(_) => {}
            Err(e) => warn!("no Xwayland for older programs: {e}"),
        }
    }

    fn forget_x11_windows(&mut self, id: XwmId) {
        let of = |e: &WindowElement| e.0.x11_surface().is_some_and(|s| s.xwm_id() == Some(id));
        let gone: Vec<WindowElement> = self.space.elements().filter(|e| of(e)).cloned().collect();
        let had_keyboard = self.seat.get_keyboard().and_then(|k| k.current_focus())
            .is_some_and(|f| gone.iter().any(|e| f == KeyboardFocusTarget::Window(e.0.clone())));
        for e in &gone {
            self.space.unmap_elem(e);
        }
        self.minimized.retain(|(e, _)| !of(e));
        self.hidden.retain(|(e, _)| !of(e));
        if had_keyboard {
            self.focus_topmost();
        }
        if !gone.is_empty() {
            self.cursor_status = CursorImageStatus::default_named();
        }
    }

    pub fn raise_x11(&mut self, surface: &X11Surface) {
        let Some(id) = surface.xwm_id() else { return };
        if let Some(wm) = self.rooms.wm(id) {
            if let Err(err) = wm.raise_window(surface) {
                warn!(?err, "could not raise an X11 window");
            }
        }
    }

    pub fn older_programs_changed(&mut self, before: bool) {
        let now = self.config.older_programs;
        if before == now {
            return;
        }
        if now {
            info!("older programs are on");
            if !self.rooms.has_desktop() {
                self.open_desktop_room();
            }
        } else {
            info!("older programs are off");
            let waiting: Vec<u64> = self.rooms.list.iter().filter(|r| r.wm.is_none()).map(|r| r.key).collect();
            for key in waiting {
                self.close_room(key);
            }
        }
    }

    fn listen_for_rooms(&mut self) {
        let Some(path) = x11_rooms::socket() else { return };
        let _ = std::fs::remove_file(&path);
        let listener = match UnixListener::bind(&path) {
            Ok(l) => l,
            Err(e) => {
                warn!("{}: {e}; boxes get no Xwayland of their own", path.display());
                return;
            }
        };
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
        }
        if listener.set_nonblocking(true).is_err() {
            return;
        }
        let ret = self.handle.insert_source(Generic::new(listener, Interest::READ, Mode::Level), |_, listener, state| {
            loop {
                match listener.accept() {
                    Ok((stream, _)) => state.take_asker(stream),
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(e) => {
                        warn!("{}: {e}", x11_rooms::ROOMS_SOCKET);
                        break;
                    }
                }
            }
            Ok(PostAction::Continue)
        });
        if let Err(e) = ret {
            warn!("{}: {e}", x11_rooms::ROOMS_SOCKET);
        }
    }

    fn take_asker(&mut self, stream: UnixStream) {
        if !same_person(&stream) || stream.set_nonblocking(true).is_err() {
            return;
        }
        let holder = self.rooms.next_holder;
        self.rooms.next_holder += 1;
        let mut heard: Vec<u8> = Vec::new();
        let mut answered = false;
        let mut holding = false;
        let ret = self.handle.insert_source(Generic::new(stream, Interest::READ, Mode::Level), move |_, stream, state| {
            let mut chunk = [0u8; 256];
            loop {
                let mut s: &UnixStream = stream;
                match s.read(&mut chunk) {
                    Ok(0) => break,
                    Ok(n) if !answered => {
                        heard.extend_from_slice(&chunk[..n]);
                        let Some(end) = heard.iter().position(|&b| b == b'\n') else {
                            if heard.len() > x11_rooms::NAME_MAX + 8 {
                                let _ = s.write_all(Answer::Bad.line().as_bytes());
                                break;
                            }
                            continue;
                        };
                        answered = true;
                        let request = String::from_utf8_lossy(&heard[..end]).into_owned();
                        let answer = match (x11_rooms::name_in(&request), s.try_clone()) {
                            (Some(name), Ok(told)) => state.room_for_box(name, holder, told),
                            _ => Answer::Bad,
                        };
                        holding = matches!(answer, Answer::Display(_));
                        if s.write_all(answer.line().as_bytes()).is_err() {
                            break;
                        }
                    }
                    Ok(_) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(PostAction::Continue),
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(_) => break,
                }
            }
            if std::mem::take(&mut holding) {
                state.holder_left(holder);
            }
            Ok(PostAction::Remove)
        });
        if let Err(e) = ret {
            warn!("{}: {e}", x11_rooms::ROOMS_SOCKET);
        }
    }

    fn room_for_box(&mut self, name: &str, holder: u64, told: UnixStream) -> Answer {
        if !self.config.older_programs {
            return Answer::Off;
        }
        let at = match room_of(self.rooms.list.iter().map(|r| &r.keep), name) {
            Some(at) => at,
            None => {
                if self.rooms.boxes() >= x11_rooms::ROOMS_MAX {
                    warn!(name, "too many Xwayland servers for boxes");
                    return Answer::Full;
                }
                if let Err(e) = self.open_room(Some(name.to_string()), None) {
                    warn!(name, "no Xwayland for the box: {e}");
                    return Answer::Full;
                }
                self.rooms.list.len() - 1
            }
        };
        let room = &mut self.rooms.list[at];
        room.keep.join(holder);
        room.holders.push((holder, told));
        Answer::Display(room.display)
    }

    fn holder_left(&mut self, holder: u64) {
        let now = Instant::now();
        for room in &mut self.rooms.list {
            if room.keep.leave(holder, now) {
                room.holders.retain(|(h, _)| *h != holder);
                return;
            }
        }
        if self.rooms.parked > 0 {
            self.rooms.left.push(holder);
        }
    }
}

#[cfg(test)]
mod tests {

    use std::path::Path;
    use std::time::{Duration, Instant};

    use smithay::reexports::x11rb::{
        connection::Connection,
        protocol::xproto::{AtomEnum, ConnectionExt, CreateWindowAux, PropMode, WindowClass},
        rust_connection::RustConnection,
        wrapper::ConnectionExt as _,
    };

    use crate::x11_rooms::{self, Answer};

    fn server(n: u32) -> Option<(&'static str, u64)> {
        server_pid(n).map(|(kind, pss, _)| (kind, pss))
    }

    fn server_pid(n: u32) -> Option<(&'static str, u64, i32)> {
        let want = format!(":{n}");
        for e in std::fs::read_dir("/proc").ok()?.flatten() {
            let p = e.path();
            let Ok(cmd) = std::fs::read(p.join("cmdline")) else { continue };
            let args: Vec<&[u8]> = cmd.split(|b| *b == 0).collect();
            if args.first().is_none_or(|a| !a.ends_with(b"Xwayland")) || !args.contains(&want.as_bytes()) {
                continue;
            }
            let exe = std::fs::read_link(p.join("exe")).unwrap_or_default();
            let pss = std::fs::read_to_string(p.join("smaps_rollup")).unwrap_or_default().lines()
                .find_map(|l| l.strip_prefix("Pss:").and_then(|v| v.trim().trim_end_matches("kB").trim().parse().ok()))
                .unwrap_or(0);
            let kind = if exe.to_string_lossy().contains("trindle-compose") { "waiting" } else { "running" };
            let pid = p.file_name()?.to_str()?.parse().ok()?;
            return Some((kind, pss, pid));
        }
        None
    }

    fn until(what: &str, secs: u64, mut ok: impl FnMut() -> bool) {
        let end = Instant::now() + Duration::from_secs(secs);
        while !ok() {
            assert!(Instant::now() < end, "{what}: not within {secs} s");
            std::thread::sleep(Duration::from_millis(200));
        }
    }

    fn window(n: u32, title: &str) -> RustConnection {
        let (conn, screen) = RustConnection::connect(Some(&format!(":{n}"))).expect("X11 connects");
        let root = conn.setup().roots[screen].root;
        let win = conn.generate_id().unwrap();
        conn.create_window(0, win, root, 0, 0, 200, 100, 0, WindowClass::INPUT_OUTPUT, 0, &CreateWindowAux::new()).unwrap();
        conn.change_property8(PropMode::REPLACE, win, AtomEnum::WM_NAME, AtomEnum::STRING, title.as_bytes()).unwrap();
        conn.map_window(win).unwrap();
        conn.get_input_focus().unwrap().reply().unwrap();
        conn
    }

    fn titles(n: u32) -> Vec<String> {
        let (conn, screen) = RustConnection::connect(Some(&format!(":{n}"))).expect("X11 connects");
        let mut out = Vec::new();
        let mut todo = vec![conn.setup().roots[screen].root];
        while let Some(w) = todo.pop() {
            let Ok(tree) = conn.query_tree(w).unwrap().reply() else { continue };
            for c in tree.children {
                if let Ok(p) = conn.get_property(false, c, AtomEnum::WM_NAME, AtomEnum::STRING, 0, 64).unwrap().reply() {
                    out.push(String::from_utf8_lossy(&p.value).into_owned());
                }
                todo.push(c);
            }
        }
        out
    }

    fn abstract_reachable(n: u32) -> bool {
        use std::os::linux::net::SocketAddrExt;
        let name = format!("/tmp/.X11-unix/X{n}");
        std::os::unix::net::SocketAddr::from_abstract_name(name.as_bytes())
            .and_then(|a| std::os::unix::net::UnixStream::connect_addr(&a))
            .is_ok()
    }

    fn set_older(conf: &Path, on: &str) {
        let text = std::fs::read_to_string(conf).unwrap_or_default();
        let mut kept: Vec<String> = text.lines().filter(|l| !l.starts_with("older_programs=")).map(String::from).collect();
        kept.push(format!("older_programs={on}"));
        std::fs::write(conf, kept.join("\n") + "\n").unwrap();
    }

    fn nothing_runs_until_an_older_program_opens(desktop: u32) -> (u64, RustConnection, u64) {
        assert_eq!(server(desktop).map(|s| s.0), Some("waiting"), "the desktop's Xwayland waits");
        let waiting = server(desktop).unwrap().1;
        assert!(!abstract_reachable(desktop), "the abstract socket is open");

        let desk = window(desktop, "on-the-desktop");
        assert_eq!(server(desktop).map(|s| s.0), Some("running"));
        std::thread::sleep(Duration::from_secs(1));
        let running = server(desktop).unwrap().1;
        (waiting, desk, running)
    }

    fn a_box_asks_for_its_own_display(desktop: u32) -> (std::os::unix::net::UnixStream, u32, RustConnection) {
        let (held, answer) = x11_rooms::ask("xrooms-test").expect("the compositor answers");
        let Answer::Display(own) = answer else { panic!("{answer:?}") };
        assert_ne!(own, desktop);
        assert_eq!(server(own).map(|s| s.0), Some("waiting"));
        assert!(!abstract_reachable(own));
        let in_box = window(own, "in-the-box");
        assert_eq!(server(own).map(|s| s.0), Some("running"));
        let from_box = titles(own);
        let from_desktop = titles(desktop);
        assert!(from_box.iter().any(|t| t == "in-the-box"), "{from_box:?}");
        assert!(!from_box.iter().any(|t| t == "on-the-desktop"), "the box sees the desktop's window");
        assert!(!from_desktop.iter().any(|t| t == "in-the-box"), "the desktop sees the box's window");
        (held, own, in_box)
    }

    fn another_launch_and_a_second_box(desktop: u32, own: u32) {
        let again: Vec<_> = (0..4).map(|_| std::thread::spawn(|| x11_rooms::ask("xrooms-test").unwrap())).collect();
        let again: Vec<_> = again.into_iter().map(|t| t.join().unwrap()).collect();
        assert!(again.iter().all(|(_, a)| *a == Answer::Display(own)), "{:?}", again.iter().map(|a| &a.1).collect::<Vec<_>>());
        drop(again);

        let (held2, answer2) = x11_rooms::ask("xrooms-test-two").unwrap();
        let Answer::Display(other) = answer2 else { panic!("{answer2:?}") };
        assert!(other != own && other != desktop);
        assert!(!titles(other).iter().any(|t| t == "in-the-box"));
        drop(held2);
        until("the second box's Xwayland goes", super::FIRST_PROGRAM_SECS as u64 + 10, || server(other).is_none());
    }

    fn a_launcher_that_ends_at_once() {
        let (held3, answer3) = x11_rooms::ask("xrooms-test-three").unwrap();
        let Answer::Display(third) = answer3 else { panic!("{answer3:?}") };
        drop(held3);
        std::thread::sleep(Duration::from_secs(3));
        let late = window(third, "late-program");
        std::thread::sleep(Duration::from_secs(super::STOP_AFTER_SECS as u64 + 3));
        assert_eq!(server(third).map(|s| s.0), Some("running"), "the display went while its program was on it");
        drop(late);
        until("the third box's Xwayland goes after its program", super::STOP_AFTER_SECS as u64 + 10, || server(third).is_none());
    }

    fn the_desktops_stops_after_its_last_program(desktop: u32, desk: RustConnection) {
        drop(desk);
        let closed = Instant::now();
        until("the desktop's Xwayland stops and waits again", 40, || server(desktop).is_some_and(|s| s.0 == "waiting"));
        let took = closed.elapsed().as_secs();
        assert!(took + 1 >= super::STOP_AFTER_SECS as u64, "stopped after {took} s");
        assert!(Path::new(&format!("/tmp/.X11-unix/X{desktop}")).exists());
        let again = window(desktop, "back-again");
        assert_eq!(server(desktop).map(|s| s.0), Some("running"));
        drop(again);
    }

    fn a_boxs_lasts_while_the_box_does(own: u32, in_box: RustConnection, held: std::os::unix::net::UnixStream) {
        drop(in_box);
        std::thread::sleep(Duration::from_secs(super::STOP_AFTER_SECS as u64 + 3));
        assert_eq!(server(own).map(|s| s.0), Some("running"), "a box's Xwayland stopped while the box ran");
        drop(held);
        until("the box's Xwayland goes after the box", super::STOP_AFTER_SECS as u64 + 10, || server(own).is_none());
        assert!(!Path::new(&format!("/tmp/.X11-unix/X{own}")).exists());
    }

    fn a_dead_stand_in_is_started_again() {
        let (held4, answer4) = x11_rooms::ask("xrooms-test-four").unwrap();
        let Answer::Display(fourth) = answer4 else { panic!("{answer4:?}") };
        let (_, _, pid) = server_pid(fourth).expect("the fourth box's stand-in");
        // SAFETY: a plain signal to the harness's own stand-in process.
        unsafe { libc::kill(pid, libc::SIGKILL) };
        held4.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        let mut line = String::new();
        std::io::BufRead::read_line(&mut std::io::BufReader::new(&held4), &mut line).unwrap();
        assert_eq!(x11_rooms::Told::parse(&line), Some(x11_rooms::Told::Lost), "{line:?}");
        until("the fourth box's Xwayland waits again", 5,
              || server_pid(fourth).is_some_and(|(kind, _, now)| kind == "waiting" && now != pid));
        let back = window(fourth, "after-a-restart");
        assert_eq!(server(fourth).map(|s| s.0), Some("running"));
        drop(back);
        drop(held4);
    }

    fn off_and_on(desktop: u32, conf: &Path) {
        until("the desktop's waits", 40, || server(desktop).is_some_and(|s| s.0 == "waiting"));
        set_older(conf, "no");
        until("nothing waits", 5, || server(desktop).is_none());
        assert_eq!(x11_rooms::ask("xrooms-test").unwrap().1, Answer::Off);
        set_older(conf, "yes");
        until("the desktop's waits again", 5, || server(desktop).is_some_and(|s| s.0 == "waiting"));
    }

    #[test]
    #[ignore]
    fn each_box_has_its_own_xwayland_started_and_stopped_as_needed() {
        let Ok(run) = std::env::var("TRINDLE_XROOMS_TEST") else { return };
        let desktop: u32 = std::env::var("TRINDLE_XROOMS_DESKTOP").unwrap().parse().unwrap();
        let conf = std::path::PathBuf::from(std::env::var("TRINDLE_XROOMS_CONF").unwrap());
        std::env::set_var("XDG_RUNTIME_DIR", &run);

        let (waiting, desk, running) = nothing_runs_until_an_older_program_opens(desktop);
        let (held, own, in_box) = a_box_asks_for_its_own_display(desktop);
        another_launch_and_a_second_box(desktop, own);
        a_launcher_that_ends_at_once();
        the_desktops_stops_after_its_last_program(desktop, desk);
        a_boxs_lasts_while_the_box_does(own, in_box, held);
        a_dead_stand_in_is_started_again();
        off_and_on(desktop, &conf);
        println!("memory (PSS): stand-in waiting {waiting} kB, Xwayland with one window {running} kB");
    }
}

#[cfg(test)]
mod keeping {

    use std::time::{Duration, Instant};

    use super::{room_of, Keep, Seen, FIRST_PROGRAM_SECS, STOP_AFTER_SECS};

    fn secs(n: u32) -> Duration {
        Duration::from_secs(n as u64)
    }

    fn running(programs: usize) -> Seen {
        Seen { running: true, starting: false, programs: Some(programs) }
    }

    const WAITING: Seen = Seen { running: false, starting: false, programs: None };

    #[test]
    fn a_launcher_that_ends_at_once_leaves_the_display_to_its_program() {
        let t = Instant::now();
        let mut keep = Keep::new(Some("com.example.Drive".into()));
        keep.join(1);
        assert!(keep.leave(1, t));
        assert!(!keep.stops(WAITING, t + secs(FIRST_PROGRAM_SECS) - Duration::from_millis(1)));
        let starting = Seen { starting: true, ..WAITING };
        assert!(!keep.stops(starting, t + secs(FIRST_PROGRAM_SECS * 5)));
        assert!(!keep.stops(running(1), t + secs(FIRST_PROGRAM_SECS * 10)));
        let closed = t + secs(FIRST_PROGRAM_SECS * 11);
        assert!(!keep.stops(running(0), closed));
        assert!(!keep.stops(running(0), closed + secs(STOP_AFTER_SECS) - Duration::from_millis(1)));
        assert!(keep.stops(running(0), closed + secs(STOP_AFTER_SECS)));
    }

    #[test]
    fn a_display_nobody_came_to_goes_only_after_the_first_program_wait() {
        let t = Instant::now();
        let mut keep = Keep::new(Some("com.example.App".into()));
        keep.join(7);
        assert!(!keep.stops(WAITING, t + secs(1000)), "it went while the box held it");
        keep.leave(7, t);
        assert!(!keep.stops(WAITING, t + secs(FIRST_PROGRAM_SECS - 1)));
        assert!(keep.stops(WAITING, t + secs(FIRST_PROGRAM_SECS)));
        let mut other = Keep::new(Some("x".into()));
        assert!(!other.leave(3, t));
    }

    #[test]
    fn the_desktops_waits_for_good_and_stops_after_its_last_program() {
        let t = Instant::now();
        let mut desktop = Keep::new(None);
        assert!(!desktop.stops(WAITING, t + secs(100_000)));
        assert!(!desktop.stops(running(2), t));
        assert!(!desktop.stops(running(0), t + secs(1)));
        assert!(desktop.stops(running(0), t + secs(1 + STOP_AFTER_SECS)));
        let mut unsure = Keep::new(None);
        assert!(!unsure.stops(Seen { programs: None, ..running(0) }, t + secs(100_000)));
    }

    #[test]
    fn launches_of_one_box_at_once_share_one_display() {
        let t = Instant::now();
        let mut keeps: Vec<Keep> = Vec::new();
        for (holder, name) in [(1, "a"), (2, "a"), (3, "b"), (4, "a"), (5, "b")] {
            let at = room_of(&keeps, name).unwrap_or_else(|| {
                keeps.push(Keep::new(Some(name.into())));
                keeps.len() - 1
            });
            keeps[at].join(holder);
        }
        assert_eq!(keeps.len(), 2, "one display for each box, not each launch");
        assert!(keeps[0].leave(1, t));
        assert!(!keeps[0].stops(running(0), t + secs(100_000)));
        assert!(keeps[0].leave(2, t) && keeps[0].leave(4, t));
        assert!(keeps[0].stops(running(0), t + secs(STOP_AFTER_SECS)));
        assert_eq!(room_of(&keeps, "c"), None);
    }

    #[test]
    fn failing_again_and_again_is_given_up_but_now_and_then_is_not() {
        let t = Instant::now();
        let mut keep = Keep::new(None);
        assert!(!keep.failed(t));
        assert!(!keep.failed(t + secs(1)));
        assert!(keep.failed(t + secs(2)));
        let mut seldom = Keep::new(None);
        for n in 0..10 {
            assert!(!seldom.failed(t + secs(n * 61)), "failure {n}");
        }
    }

    fn fake_xwayland(wait: &str, die: bool) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-379-fake-{}-{wait}-{die}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let script = format!(
            "#!/bin/bash\nfor a; do [ \"$prev\" = -displayfd ] && fd=$a; prev=$a; done\nsleep {wait}\n{}\necho \"${{1#:}}\" >&$fd\nexec sleep 3\n",
            if die { "exit 1" } else { "" }
        );
        let at = dir.join("Xwayland");
        std::fs::write(&at, script).unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&at, std::fs::Permissions::from_mode(0o755)).unwrap();
        dir
    }

    fn start(wait: &str, die: bool, n: u32, meanwhile: impl FnOnce(u32)) -> Option<(bool, Duration)> {
        use smithay::reexports::{calloop::EventLoop, wayland_server::Display};
        use smithay::xwayland::{XWayland, XWaylandEvent};
        let dir = fake_xwayland(wait, die);
        let display: Display<()> = Display::new().unwrap();
        let path = format!("{}:/usr/bin:/bin", dir.display());
        let started = Instant::now();
        let spawned = XWayland::spawn(&display.handle(), Some(n), [("PATH", path)], false,
                                      std::process::Stdio::null(), std::process::Stdio::null(), |_| ());
        let (xwayland, _client) = match spawned {
            Ok(s) => s,
            Err(e) => {
                let _ = std::fs::remove_dir_all(&dir);
                panic!("display :{n} could not be made here: {e}");
            }
        };
        meanwhile(xwayland.display_number());
        let mut events: EventLoop<Option<bool>> = EventLoop::try_new().unwrap();
        let token = events.handle().insert_source(xwayland, |event, _, out: &mut Option<bool>| {
            *out = Some(matches!(event, XWaylandEvent::Ready { .. }));
        }).unwrap();
        let mut out = None;
        let mut ended = None;
        let mut wakes = 0u32;
        while started.elapsed() < Duration::from_secs(10) {
            events.dispatch(Some(Duration::from_millis(100)), &mut out).unwrap();
            wakes += 1;
            if out.is_some() {
                ended = Some(started.elapsed());
                break;
            }
        }
        events.handle().remove(token);
        let _ = std::fs::remove_dir_all(&dir);
        assert!(wakes < 400, "the event loop woke {wakes} times: a closed pipe keeps waking it");
        out.zip(ended)
    }

    fn spare(offset: u32) -> u32 {
        700 + (std::process::id() % 100) * 3 + offset
    }

    #[test]
    fn a_display_handed_out_takes_programs_before_its_slow_xwayland_is_ready() {
        let (ready, took) = start("1.5", false, spare(0), |n| {
            let (tx, rx) = std::sync::mpsc::channel();
            for _ in 0..8 {
                let tx = tx.clone();
                std::thread::spawn(move || {
                    let _ = tx.send(std::os::unix::net::UnixStream::connect(format!("/tmp/.X11-unix/X{n}")).map(|_| ()));
                });
            }
            for _ in 0..8 {
                let got = rx.recv_timeout(Duration::from_secs(2)).expect("a program was left hanging at the socket");
                assert!(got.is_ok(), "a program was refused: {got:?}");
            }
        }).expect("no word from the slow Xwayland");
        assert!(ready, "the slow Xwayland was reported as failed");
        assert!(took >= Duration::from_millis(1400), "ready after only {took:?}");
    }

    #[test]
    fn an_xwayland_that_ends_before_it_is_ready_is_reported_at_once() {
        let (ready, took) = start("0.2", true, spare(1), |_| ()).expect("an Xwayland that ended was never reported");
        assert!(!ready, "an Xwayland that ended was reported ready");
        assert!(took < Duration::from_secs(5), "reported only after {took:?}");
    }
}
