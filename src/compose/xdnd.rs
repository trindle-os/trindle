// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::os::fd::{AsFd, OwnedFd};
use std::time::{Duration, Instant};

use smithay::desktop::space::SpaceElement;
use smithay::input::pointer::CursorImageStatus;
use smithay::reexports::calloop::timer::{TimeoutAction, Timer};
use smithay::reexports::wayland_server::protocol::wl_data_device_manager::DndAction;
use smithay::reexports::wayland_server::protocol::wl_data_source::{self, WlDataSource};
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::reexports::wayland_server::Resource;
use smithay::utils::{IsAlive, Logical, Point, SERIAL_COUNTER};
use smithay::wayland::seat::WaylandFocus;
use smithay::wayland::selection::data_device::{start_dnd_following, with_source_metadata, SourceMetadata};
use smithay::xwayland::xwm::{X11Window, XdndEvent, XwmId};
use smithay::xwayland::X11Surface;
use tracing::{debug, info, warn};

use crate::focus::PointerFocusTarget;
use crate::state::{AnvilState, Backend};

pub const XDND_FINISH_SECS: u64 = 30;

const DROP_WAIT: Duration = Duration::from_millis(1500);

const TICK: Duration = Duration::from_millis(16);

const MOST_WAITING: usize = 8;

#[derive(Debug, Clone)]
pub enum Source {
    Wayland(WlDataSource),

    Old,
}

#[derive(Debug)]
struct Armed {
    xwm: XwmId,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Stage {

    Held,

    Released(Instant),

    Dropped(Instant),
}

#[derive(Debug)]
struct FromX {
    xwm: XwmId,
    kinds: Vec<String>,
    stage: Stage,

    in_proxy: bool,

    mime_ok: bool,
    action: DndAction,
    said_over: Option<smithay::reexports::wayland_server::backend::ObjectId>,

    foreign: Option<(bool, DndAction)>,

    waiting: Vec<(String, OwnedFd)>,
    reads_started: usize,
    reads_done: usize,

    cancelled: bool,

    told: Option<(bool, DndAction)>,
}

#[derive(Debug)]
struct OverX {
    xwm: XwmId,
    source: Source,
    window: X11Window,
    accepted: bool,
    action: DndAction,
    dropped: Option<Instant>,

    awaiting: Option<Instant>,

    latest: Option<Point<f64, Logical>>,
}

#[derive(Debug, Default)]
pub struct Xdnd {
    armed: Option<Armed>,
    from_x: Option<FromX>,
    over_x: Option<OverX>,

    wl_source: Option<WlDataSource>,
    ticking: bool,
}

impl Xdnd {
    fn busy(&self) -> bool {
        self.armed.is_some() || self.from_x.is_some() || self.over_x.is_some()
    }

    pub fn takes_over(&self) -> Option<DndAction> {
        self.over_x.as_ref().filter(|o| o.accepted).map(|o| o.action)
    }
}

fn only_copy(actions: DndAction) -> DndAction {
    actions & DndAction::Copy
}

fn offerable(kinds: &[String]) -> Vec<String> {
    kinds
        .iter()
        .filter(|k| !k.is_empty() && k.len() <= smithay::xwayland::xwm::MAX_KIND_NAME && !k.chars().any(|c| c.is_control()))
        .take(smithay::xwayland::xwm::MAX_KINDS)
        .cloned()
        .collect()
}

impl<BackendData: Backend + 'static> AnvilState<BackendData> {

    pub fn xdnd_element_under(&self, pos: Point<f64, Logical>) -> Option<(&crate::shell::WindowElement, Point<i32, Logical>)> {
        let Some(xwm) = self.xdnd.from_x.as_ref().map(|f| f.xwm) else { return self.space.element_under(pos) };
        self.space.elements().rev().find_map(|e| {
            let x = e.0.x11_surface();
            if x.is_some_and(|x| x.is_override_redirect() && x.xwm_id() == Some(xwm)) {
                return None;
            }
            if !self.space.element_bbox(e)?.to_f64().contains(pos) {
                return None;
            }
            let loc = self.space.element_location(e)? - e.geometry().loc;
            e.is_in_input_region(&(pos - loc.to_f64())).then_some((e, loc))
        })
    }

    fn old_window_of(&self, surface: &WlSurface) -> Option<X11Surface> {
        self.space
            .elements()
            .find_map(|e| e.0.x11_surface().filter(|x| x.wl_surface().as_ref() == Some(surface)))
            .cloned()
    }

    fn xwm_scale(&mut self, xwm: XwmId) -> f64 {
        self.rooms.wm(xwm).map_or(1.0, |w| w.client_scale())
    }

    fn xdnd_arm(&mut self, xwm: XwmId) {
        if self.xdnd.busy() {
            return;
        }
        let Some(start) = self.pointer.grab_start_data() else {
            debug!("an older program took the drag selection with no button held: not a drag");
            return;
        };
        let held_on_it = start
            .focus
            .as_ref()
            .is_some_and(|(f, _)| matches!(f, PointerFocusTarget::X11Surface(s) if s.xwm_id() == Some(xwm)));
        if !held_on_it {
            debug!("an older program took the drag selection while the button was held elsewhere: not a drag");
            return;
        }
        debug!("an older program may be dragging");
        self.xdnd.armed = Some(Armed { xwm });
        self.xdnd_start_ticking();
        self.xdnd_look();
    }

    fn pointer_over_window_of(&self, xwm: XwmId) -> bool {
        let at = self.pointer.current_location();
        let Some((focus, _)) = self.surface_under(at) else { return false };
        match focus {
            PointerFocusTarget::X11Surface(s) if s.is_override_redirect() => self
                .space
                .elements()
                .rev()
                .filter(|e| e.0.x11_surface().is_none_or(|x| !x.is_override_redirect()))
                .find(|e| self.space.element_location(e).is_some_and(|loc| e.is_in_input_region(&(at - loc.to_f64()))))
                .and_then(|e| e.0.x11_surface())
                .is_some_and(|x| x.xwm_id() == Some(xwm)),
            PointerFocusTarget::X11Surface(s) => s.xwm_id() == Some(xwm),
            _ => false,
        }
    }

    fn xdnd_look(&mut self) {
        let xwm = match (&self.xdnd.armed, &self.xdnd.from_x) {
            (_, Some(f)) if f.stage == Stage::Held => f.xwm,
            (Some(a), None) => a.xwm,
            _ => return,
        };
        let show = !self.pointer_over_window_of(xwm);
        if let Some(wm) = self.rooms.wm(xwm) {
            if let Err(err) = wm.dnd_proxy(show) {
                warn!(?err, "could not show or hide the window that takes an older program's drag");
            }
        }
    }

    fn xdnd_start_ticking(&mut self) {
        if self.xdnd.ticking {
            return;
        }
        self.xdnd.ticking = true;
        let timer = Timer::from_duration(TICK);
        let _ = self.handle.insert_source(timer, |_, _, state| {
            if state.xdnd_tick() {
                TimeoutAction::ToDuration(TICK)
            } else {
                state.xdnd.ticking = false;
                TimeoutAction::Drop
            }
        });
    }

    fn xdnd_tick(&mut self) -> bool {
        let held = self.pointer.is_grabbed();
        if self.xdnd.armed.is_some() && self.xdnd.from_x.is_none() && !held {

            self.xdnd_stop_armed();
        }
        if self.xdnd.armed.is_some() && self.xdnd.from_x.is_none() {
            self.xdnd_look();
        }
        let now = Instant::now();

        if self.xdnd.from_x.as_ref().is_some_and(|f| f.cancelled && !matches!(f.stage, Stage::Dropped(_))) {
            self.xdnd_clear_from_x(false);
        }

        if let Some(f) = &self.xdnd.from_x {
            match f.stage {
                Stage::Held => self.xdnd_look(),
                Stage::Released(by) if now > by => {
                    info!("an older program did not say it let go: the drag came to nothing");
                    self.xdnd_end_grab(false);
                    return self.xdnd.busy();
                }
                Stage::Dropped(by) if now > by => {
                    info!("the program let go on did not finish in time: the drag came to nothing");
                    self.xdnd_finish_from_x(false);
                    return self.xdnd.busy();
                }
                _ => {}
            }
            self.xdnd_answers_still_hold();
        }

        if let Some(o) = &self.xdnd.over_x {
            if o.dropped.is_some_and(|by| now > by) {
                info!("an older program did not finish a drop in time: the drag came to nothing");
                self.xdnd_over_x_done(false);
            } else if o.awaiting.is_some_and(|at| now > at + Duration::from_millis(400)) {

                self.xdnd_send_position(None);
            }
        }
        self.xdnd.busy()
    }

    fn xdnd_answers_still_hold(&mut self) {
        let under = self.surface_under(self.pointer.current_location()).and_then(|(f, _)| f.wl_surface().map(|s| s.id()));
        let Some(f) = &mut self.xdnd.from_x else { return };
        if f.stage != Stage::Held {
            return;
        }
        if f.said_over.is_some() && f.said_over != under {
            f.said_over = None;
            f.mime_ok = false;
            f.action = DndAction::empty();
        }
        self.xdnd_answer_source();
    }

    fn xdnd_answer_source(&mut self) {
        let Some(f) = &mut self.xdnd.from_x else { return };
        if !f.in_proxy {
            return;
        }
        let answer = match f.foreign {
            Some(a) => a,
            None => (f.mime_ok && !f.action.is_empty(), f.action),
        };
        if f.told == Some(answer) {
            return;
        }
        debug!(?answer, "telling an older program what its drag would do");
        f.told = Some(answer);
        let xwm = f.xwm;
        if let Some(wm) = self.rooms.wm(xwm) {
            if let Err(err) = wm.dnd_answer_source(answer.0, answer.1) {
                warn!(?err, "could not answer an older program's drag");
            }
        }
    }

    fn xdnd_stop_armed(&mut self) {
        let Some(a) = self.xdnd.armed.take() else { return };
        debug!("an older program is no longer armed");
        if let Some(wm) = self.rooms.wm(a.xwm) {
            let _ = wm.dnd_proxy(false);
        }
    }

    pub fn xdnd_event(&mut self, xwm: XwmId, event: XdndEvent) {
        debug!(?event, "an older program in a drag");
        match event {
            XdndEvent::SourceTook(_) => self.xdnd_arm(xwm),
            XdndEvent::SourceGone => {
                if self.xdnd.armed.as_ref().is_some_and(|a| a.xwm == xwm) && self.xdnd.from_x.is_none() {
                    self.xdnd_stop_armed();
                }

                let gave_up = self.xdnd.from_x.as_ref().is_some_and(|f| f.xwm == xwm && !matches!(f.stage, Stage::Dropped(_)));
                if gave_up {
                    self.xdnd_end_grab(false);
                }
            }
            XdndEvent::Enter { kinds, actions } => self.xdnd_entered(xwm, kinds, actions),
            XdndEvent::Position { action } => {
                let _ = action;
                if let Some(f) = &mut self.xdnd.from_x {
                    if f.xwm == xwm {

                        f.told = None;
                    }
                }
                self.xdnd_answer_source();
            }
            XdndEvent::Leave => {
                let released = {
                    let Some(f) = &mut self.xdnd.from_x else { return };
                    if f.xwm != xwm {
                        return;
                    }
                    f.in_proxy = false;
                    f.told = None;
                    matches!(f.stage, Stage::Released(_))
                };

                if released {
                    self.xdnd_end_grab(false);
                }
            }
            XdndEvent::Drop => self.xdnd_dropped(xwm),
            XdndEvent::TargetStatus { accepted, action } => self.xdnd_target_answered(xwm, accepted, action),
            XdndEvent::TargetFinished { accepted, .. } => {
                if self.xdnd.over_x.as_ref().is_some_and(|o| o.xwm == xwm) {
                    self.xdnd_over_x_done(accepted);
                }
            }
            XdndEvent::ReadDone { complete } => {
                if let Some(f) = self.xdnd.from_x.as_mut().filter(|f| f.xwm == xwm) {
                    if complete {
                        f.reads_done += 1;
                    }
                }
            }
        }
    }

    fn xdnd_entered(&mut self, xwm: XwmId, kinds: Vec<String>, actions: DndAction) {
        if let Some(f) = &mut self.xdnd.from_x {

            if f.xwm == xwm && matches!(f.stage, Stage::Held | Stage::Released(_)) {
                f.in_proxy = true;
                f.told = None;
            }
            return;
        }
        if !self.xdnd.armed.as_ref().is_some_and(|a| a.xwm == xwm) || kinds.is_empty() {
            return;
        }
        let Some(start) = self.pointer.grab_start_data() else { return };
        let allowed = if actions.is_empty() { DndAction::Copy } else { actions };
        let metadata = SourceMetadata { mime_types: kinds.clone(), dnd_action: allowed };
        self.xdnd.from_x = Some(FromX {
            xwm,
            kinds,
            stage: Stage::Held,
            in_proxy: true,
            mime_ok: false,
            action: DndAction::empty(),
            said_over: None,
            foreign: None,
            waiting: Vec::new(),
            reads_started: 0,
            reads_done: 0,
            cancelled: false,
            told: None,
        });
        let (dh, seat) = (self.display_handle.clone(), self.seat.clone());
        start_dnd_following(&dh, &seat, self, SERIAL_COUNTER.next_serial(), start, metadata);
        debug!(grabbed = self.pointer.is_grabbed(), "an older program's drag was started for the Wayland programs");
        self.xdnd_start_ticking();
    }

    pub fn xdnd_released(&mut self) {
        debug!(drag = self.xdnd.from_x.is_some(), "the last button came up in a drag");
        let Some(f) = &mut self.xdnd.from_x else { return };
        if f.stage != Stage::Held {
            return;
        }
        if f.in_proxy {
            f.stage = Stage::Released(Instant::now() + DROP_WAIT);
        } else {
            self.xdnd_end_grab(false);
        }
    }

    fn xdnd_dropped(&mut self, xwm: XwmId) {
        let Some(f) = &mut self.xdnd.from_x else { return };
        if f.xwm != xwm || !matches!(f.stage, Stage::Released(_)) {

            if f.xwm == xwm && f.stage == Stage::Held {
                debug!("an older program dropped while the button was still down: left alone");
            }
            return;
        }
        f.stage = Stage::Dropped(Instant::now() + Duration::from_secs(XDND_FINISH_SECS));
        f.cancelled = false;
        let pointer = self.pointer.clone();
        pointer.unset_grab(self, SERIAL_COUNTER.next_serial(), self.clock.now().as_millis());
        let Some(f) = &mut self.xdnd.from_x else { return };
        if f.cancelled {

            self.xdnd_clear_from_x(true);
            return;
        }

        let waiting = std::mem::take(&mut f.waiting);
        for (kind, fd) in waiting {
            self.xdnd_read(kind, fd);
        }
    }

    fn xdnd_end_grab(&mut self, tell: bool) {
        if let Some(f) = &mut self.xdnd.from_x {
            f.cancelled = true;
            f.waiting.clear();
        }
        if self.pointer.is_grabbed() {
            let pointer = self.pointer.clone();
            pointer.unset_grab(self, SERIAL_COUNTER.next_serial(), self.clock.now().as_millis());
        }
        self.xdnd_clear_from_x(tell);
    }

    fn xdnd_clear_from_x(&mut self, tell: bool) {
        let Some(f) = self.xdnd.from_x.take() else { return };
        self.xdnd.armed = None;
        if let Some(wm) = self.rooms.wm(f.xwm) {
            if tell {
                let _ = wm.dnd_finish_source(false, DndAction::empty());
            } else {
                wm.dnd_forget_source();
            }
            let _ = wm.dnd_proxy(false);
        }
        if let Some(o) = self.xdnd.over_x.take() {
            if let Some(wm) = self.rooms.wm(o.xwm) {
                let _ = wm.dnd_offer_end();
            }
        }
    }

    fn xdnd_finish_from_x(&mut self, accepted: bool) {
        let Some(f) = self.xdnd.from_x.as_ref() else { return };

        let mut action = f.action;
        let whole = f.reads_started > 0 && f.reads_done == f.reads_started;
        if let Some((_, a)) = f.foreign {
            action = a;
        }
        if action.contains(DndAction::Move) && !whole {
            action = DndAction::Copy;
        }
        let xwm = f.xwm;
        if let Some(wm) = self.rooms.wm(xwm) {
            let _ = wm.dnd_finish_source(accepted, action);
            let _ = wm.dnd_proxy(false);
        }
        self.xdnd.from_x = None;
        self.xdnd.armed = None;
    }

    fn xdnd_read(&mut self, kind: String, fd: OwnedFd) {
        let Some(f) = &mut self.xdnd.from_x else { return };
        f.reads_started += 1;
        let xwm = f.xwm;
        let handle = self.handle.clone();
        let started = match self.rooms.wm(xwm) {
            Some(wm) => wm.dnd_read_source(&kind, fd, handle).unwrap_or_else(|err| {
                warn!(?err, "could not read an older program's drag");
                false
            }),
            None => false,
        };
        if !started {
            if let Some(f) = &mut self.xdnd.from_x {
                f.reads_started -= 1;
            }
        }
    }

    pub fn xdnd_server_accept(&mut self, mime: Option<String>) {
        debug!(?mime, "a Wayland program answered a drag from an older program");
        let over = self.surface_under(self.pointer.current_location()).and_then(|(t, _)| t.wl_surface().map(|s| s.id()));
        if let Some(f) = &mut self.xdnd.from_x {
            f.mime_ok = mime.is_some();
            f.said_over = over;
        }
        self.xdnd_answer_source();
    }

    pub fn xdnd_server_action(&mut self, action: DndAction) {
        debug!(?action, "a Wayland program chose a way to take a drag from an older program");
        let over = self.surface_under(self.pointer.current_location()).and_then(|(t, _)| t.wl_surface().map(|s| s.id()));
        if let Some(f) = &mut self.xdnd.from_x {
            f.action = action;
            f.said_over = over;
        }
        self.xdnd_answer_source();
    }

    pub fn xdnd_server_send(&mut self, mime: String, fd: OwnedFd) {
        debug!(mime, stage = ?self.xdnd.from_x.as_ref().map(|f| f.stage), "a Wayland program asks for an older program's drag");
        let Some(f) = &mut self.xdnd.from_x else { return };
        if !matches!(f.stage, Stage::Dropped(_)) {

            if f.waiting.len() < MOST_WAITING {
                f.waiting.push((mime, fd));
            }
            return;
        }
        self.xdnd_read(mime, fd);
    }

    pub fn xdnd_server_finished(&mut self) {
        if self.xdnd.from_x.as_ref().is_some_and(|f| matches!(f.stage, Stage::Dropped(_))) {
            self.xdnd_finish_from_x(true);
        }
    }

    pub fn xdnd_server_cancelled(&mut self) {
        if let Some(f) = &mut self.xdnd.from_x {
            f.cancelled = true;
            f.waiting.clear();
        }
    }

    pub fn xdnd_drop_allowed(&self) -> bool {
        self.xdnd.from_x.as_ref().is_some_and(|f| matches!(f.stage, Stage::Dropped(_)))
    }

    fn xdnd_dragged(&self) -> Option<(Source, Vec<String>)> {
        if let Some(f) = &self.xdnd.from_x {
            return Some((Source::Old, f.kinds.clone()));
        }
        let source = self.xdnd.wl_source.clone()?;
        if !source.alive() {
            return None;
        }
        let kinds = with_source_metadata(&source, |m| m.mime_types.clone()).ok()?;
        Some((Source::Wayland(source), kinds))
    }

    pub fn xdnd_foreign_target(&mut self, surface: &WlSurface) -> bool {
        let Some(window) = self.old_window_of(surface) else { return false };
        let from = self.xdnd.from_x.as_ref().map(|f| f.xwm);

        window.xwm_id().is_some() && window.xwm_id() != from
    }

    pub fn xdnd_foreign_enter(&mut self, surface: &WlSurface, location: Point<f64, Logical>) {
        if !self.config.older_programs {
            return;
        }
        let Some(window) = self.old_window_of(surface) else { return };
        let Some(xwm) = window.xwm_id() else { return };
        let Some((source, kinds)) = self.xdnd_dragged() else { return };
        let allowed = match &source {
            Source::Wayland(s) => only_copy(with_source_metadata(s, |m| m.dnd_action).unwrap_or(DndAction::empty())),
            Source::Old => DndAction::Copy,
        };
        if allowed.is_empty() {
            return;
        }
        let kinds = offerable(&kinds);
        if kinds.is_empty() {
            return;
        }
        let id = window.window_id();
        let Some(wm) = self.rooms.wm(xwm) else { return };
        match wm.dnd_offer(id, &kinds, allowed) {
            Ok(true) => {
                self.xdnd.over_x = Some(OverX {
                    xwm,
                    source,
                    window: id,
                    accepted: false,
                    action: DndAction::empty(),
                    dropped: None,
                    awaiting: None,
                    latest: None,
                });
                self.xdnd_send_position(Some(location));
                self.xdnd_start_ticking();
            }
            Ok(false) => debug!("an older program's window does not take drops"),
            Err(err) => warn!(?err, "could not offer a drag to an older program"),
        }
    }

    pub fn xdnd_foreign_motion(&mut self, location: Point<f64, Logical>) {
        let Some(o) = &mut self.xdnd.over_x else { return };
        if o.dropped.is_some() {
            return;
        }
        if o.awaiting.is_some() {

            o.latest = Some(location);
            return;
        }
        self.xdnd_send_position(Some(location));
    }

    fn xdnd_send_position(&mut self, at: Option<Point<f64, Logical>>) {
        let Some(o) = &mut self.xdnd.over_x else { return };
        let Some(at) = at.or_else(|| o.latest.take()) else {
            o.awaiting = None;
            return;
        };
        o.latest = None;
        o.awaiting = Some(Instant::now());
        let (xwm, action) = (o.xwm, only_copy(DndAction::Copy));
        let scale = self.xwm_scale(xwm);
        if let Some(wm) = self.rooms.wm(xwm) {
            let (x, y) = ((at.x * scale).round() as i32, (at.y * scale).round() as i32);
            if let Err(err) = wm.dnd_offer_position(x, y, action) {
                warn!(?err, "could not tell an older program where a drag is");
            }
        }
    }

    pub fn xdnd_foreign_leave(&mut self) {
        let Some(o) = &self.xdnd.over_x else { return };
        if o.dropped.is_some() {
            return;
        }
        let xwm = o.xwm;
        self.xdnd.over_x = None;
        if let Some(wm) = self.rooms.wm(xwm) {
            let _ = wm.dnd_offer_leave();
        }

        if let Some(f) = &mut self.xdnd.from_x {
            f.foreign = None;
        }
        self.xdnd_answer_source();
    }

    pub fn xdnd_foreign_drop(&mut self) -> bool {
        let Some(o) = &mut self.xdnd.over_x else { return false };
        let xwm = o.xwm;
        if !o.accepted || o.action.is_empty() {

            self.xdnd.over_x = None;
            if let Some(wm) = self.rooms.wm(xwm) {
                let _ = wm.dnd_offer_leave();
            }
            return false;
        }
        o.dropped = Some(Instant::now() + Duration::from_secs(XDND_FINISH_SECS));
        if let Some(wm) = self.rooms.wm(xwm) {
            if let Err(err) = wm.dnd_offer_drop() {
                warn!(?err, "could not let go on an older program");
                self.xdnd.over_x = None;
                return false;
            }
        }
        true
    }

    fn xdnd_target_answered(&mut self, xwm: XwmId, accepted: bool, action: DndAction) {
        let Some(o) = self.xdnd.over_x.as_mut().filter(|o| o.xwm == xwm) else { return };

        o.accepted = accepted && action == DndAction::Copy;
        o.action = if o.accepted { DndAction::Copy } else { DndAction::empty() };
        o.awaiting = None;
        let answer = (o.accepted, o.action);

        if let Source::Wayland(src) = &o.source {
            if src.alive() {
                if src.version() >= wl_data_source::EVT_ACTION_SINCE {
                    src.action(if o.accepted { DndAction::Copy } else { DndAction::empty() });
                }
                let kind = with_source_metadata(src, |m| m.mime_types.first().cloned()).ok().flatten();
                src.target(if o.accepted { kind } else { None });
            }
        }

        if let Some(f) = &mut self.xdnd.from_x {
            f.foreign = Some(answer);
        }
        self.xdnd_answer_source();
        self.xdnd_send_position(None);
    }

    fn xdnd_over_x_done(&mut self, accepted: bool) {
        let Some(o) = self.xdnd.over_x.take() else { return };
        if let Some(wm) = self.rooms.wm(o.xwm) {
            let _ = wm.dnd_offer_end();
        }
        match o.source {
            Source::Wayland(src) => {
                if !src.alive() {
                    return;
                }
                if src.version() >= wl_data_source::EVT_DND_FINISHED_SINCE {
                    if accepted {
                        src.dnd_finished();
                    } else {
                        src.cancelled();
                    }
                }
            }
            Source::Old => self.xdnd_finish_from_x(accepted),
        }
    }

    pub fn xdnd_send_to_old(&mut self, xwm: XwmId, mime: String, fd: OwnedFd) {
        let Some(o) = self.xdnd.over_x.as_ref().filter(|o| o.xwm == xwm && o.dropped.is_some()) else { return };
        match o.source.clone() {
            Source::Wayland(src) => {
                let listed = with_source_metadata(&src, |m| m.mime_types.contains(&mime)).unwrap_or(false);
                if src.alive() && listed {
                    src.send(mime, fd.as_fd());
                }
            }
            Source::Old => self.xdnd_read(mime, fd),
        }
    }

    pub fn xdnd_wayland_drag(&mut self, source: Option<WlDataSource>) {
        self.xdnd.wl_source = source;
    }

    pub fn xdnd_lost(&mut self, xwm: XwmId) {
        if self.xdnd.armed.as_ref().is_some_and(|a| a.xwm == xwm) {
            self.xdnd.armed = None;
        }
        if self.xdnd.from_x.as_ref().is_some_and(|f| f.xwm == xwm) {
            self.xdnd_end_grab(false);
        }
        if let Some(o) = self.xdnd.over_x.as_ref().filter(|o| o.xwm == xwm) {
            let src = o.source.clone();
            self.xdnd.over_x = None;
            if let Source::Wayland(s) = src {
                if s.alive() && s.version() >= wl_data_source::EVT_DND_FINISHED_SINCE {
                    s.cancelled();
                }
            }
        }
    }

    pub fn xdnd_shape_over(&self, surface: &WlSurface) -> Option<DndAction> {
        let o = self.xdnd.over_x.as_ref()?;
        let window = self.old_window_of(surface)?;
        (window.window_id() == o.window).then_some(o.action)
    }

    pub fn xdnd_cursor_reset(&mut self) {
        if matches!(self.cursor_status, CursorImageStatus::Named(_)) {
            self.cursor_status = CursorImageStatus::default_named();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_older_program_gets_only_a_copy() {
        assert_eq!(only_copy(DndAction::Copy | DndAction::Move), DndAction::Copy);
        assert_eq!(only_copy(DndAction::Move), DndAction::empty());
        assert_eq!(only_copy(DndAction::Ask), DndAction::empty());
    }

    #[test]
    fn kinds_offered_are_few_short_and_plain() {
        let mut kinds: Vec<String> = (0..100).map(|i| format!("text/x-{i}")).collect();
        kinds.push("a".repeat(300));
        kinds.push("bad\u{7}/kind".into());
        kinds.push(String::new());
        let out = offerable(&kinds);
        assert_eq!(out.len(), smithay::xwayland::xwm::MAX_KINDS);
        assert!(out.iter().all(|k| k.len() <= smithay::xwayland::xwm::MAX_KIND_NAME && !k.is_empty()));
        assert!(offerable(&["a".repeat(300), "x\u{0}/y".into(), String::new()]).is_empty());
    }
}
