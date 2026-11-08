//! Trindle: drag and drop between X11 programs and Wayland ones (XDND, version
//! 5), the X11 side. The compositor joins this to the Wayland data device
//! (`src/compose/xdnd.rs`); the way the two meet is written in the spec,
//! "Older programs", "Dragging between older programs and Trindle's".
//!
//! Two directions are spoken here:
//!
//! - An X11 program is the source of a drag. It takes the XDND selection
//!   and sends its messages to the window under the pointer. The compositor
//!   shows it one window of ours that is marked as taking drops ([`X11Wm::dnd_proxy`]),
//!   only while the pointer is over something that is not an X11 window,
//!   and hears of what the source says ([`XdndEvent`]).
//! - The compositor is the source of a drag, for a Wayland program, towards a
//!   window of an X11 program: it takes the XDND selection itself and sends
//!   the messages ([`X11Wm::dnd_offer`] and the functions after it).
//!
//! The data is read through the selection transfers the clipboard uses, with
//! the same limits, and given out only as the spec says: never before the
//! drop, and only to the program it was dropped on ([`DndState::may_ask`]).

#![allow(missing_docs)]

use std::os::unix::io::OwnedFd;

use calloop::LoopHandle;
use tracing::debug;
use wayland_server::protocol::wl_data_device_manager::DndAction;
use x11rb::{
    connection::Connection as _,
    errors::ReplyOrIdError,
    protocol::xproto::{
        Atom, AtomEnum, ClientMessageEvent, ConfigureWindowAux, ConnectionExt as _,
        CreateWindowAux, EventMask, PropMode, Screen, StackMode, WindowClass,
    },
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
};

use super::{Atoms, SelectionError, Slot, X11Window, X11Wm, XwmHandler};

/// The most kinds a drag lists, and the longest name of one.
pub const MAX_KINDS: usize = 64;
pub const MAX_KIND_NAME: usize = 255;

/// The XDND version spoken: 5, and 3 or 4 with a program that speaks them.
const VERSION: u32 = 5;
const OLDEST: u32 = 3;

/// What happened in a drag involving an X11 program.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XdndEvent {
    /// An X11 program took the drag selection: it may be about to drag.
    SourceTook(X11Window),
    /// The drag selection has no owner.
    SourceGone,
    /// The source says what it holds, to the window shown to it.
    Enter {
        /// The kinds, named as Wayland programs name them.
        kinds: Vec<String>,
        /// What it allows to be done; empty until it says (with a position).
        actions: DndAction,
    },
    /// The source says where it is, and the way it would like it taken.
    Position { action: DndAction },
    /// The source went away from the window shown to it.
    Leave,
    /// The source let go on the window shown to it.
    Drop,
    /// The program the compositor offered to has answered a position.
    TargetStatus { accepted: bool, action: DndAction },
    /// The program the compositor offered to says it has finished.
    TargetFinished { accepted: bool, action: DndAction },
    /// A read of the X11 source's data to a Wayland program ended: every
    /// byte was passed, or not.
    ReadDone { complete: bool },
}

/// What an X11 source has said.
#[derive(Debug)]
struct FromX {
    source: X11Window,
    /// Each kind with the atom it is asked for by.
    kinds: Vec<(String, Atom)>,
    /// When it let go (the time it said), once it has.
    dropped: Option<u32>,
}

/// What the compositor has offered to an X11 program's window.
#[derive(Debug)]
struct ToX {
    /// The window of the program the drag is over, and where messages go
    /// (the same, or the proxy it names).
    window: X11Window,
    send_to: X11Window,
    version: u32,
    dropped: bool,
}

#[derive(Debug)]
pub(crate) struct DndState {
    /// The window shown to an X11 source: marked as taking drops, across the
    /// screen, never mapped except while it is wanted.
    proxy: X11Window,
    proxy_mapped: bool,
    from_x: Option<FromX>,
    to_x: Option<ToX>,
}

impl DndState {
    pub(crate) fn new(
        conn: &RustConnection,
        screen: &Screen,
        atoms: &Atoms,
    ) -> Result<Self, ReplyOrIdError> {
        let proxy = conn.generate_id()?;
        conn.create_window(
            screen.root_depth,
            proxy,
            screen.root,
            0,
            0,
            screen.width_in_pixels.max(1),
            screen.height_in_pixels.max(1),
            0,
            WindowClass::INPUT_OUTPUT,
            x11rb::COPY_FROM_PARENT,
            &CreateWindowAux::new().override_redirect(1),
        )?;
        conn.change_property32(PropMode::REPLACE, proxy, atoms.XdndAware, AtomEnum::ATOM, &[VERSION])?;
        // Toolkits that look for the window an older program's drag is over
        // pass over one that does not look like a program's.
        conn.change_property32(PropMode::REPLACE, proxy, atoms.WM_STATE, atoms.WM_STATE, &[1, 0])?;
        conn.flush()?;
        Ok(DndState { proxy, proxy_mapped: false, from_x: None, to_x: None })
    }

    pub(crate) fn is_ours(&self, window: X11Window) -> bool {
        window == self.proxy
    }

    pub(crate) fn is_dnd_message(&self, atoms: &Atoms, type_: Atom) -> bool {
        [
            atoms.XdndEnter,
            atoms.XdndPosition,
            atoms.XdndLeave,
            atoms.XdndDrop,
            atoms.XdndStatus,
            atoms.XdndFinished,
        ]
        .contains(&type_)
    }

    /// Whether the program that sent this request may be given what the drag
    /// holds as `target`: only a program of the same X client as the window
    /// the drag is over, and the data only once it is let go.
    pub(crate) fn may_ask(&self, conn: &RustConnection, requestor: X11Window, target: Atom, atoms: &Atoms) -> bool {
        let Some(to) = &self.to_x else { return false };
        let mask = conn.setup().resource_id_mask;
        if requestor & !mask != to.window & !mask && requestor & !mask != to.send_to & !mask {
            return false;
        }
        if target == atoms.TARGETS || target == atoms.TIMESTAMP {
            return true;
        }
        to.dropped
    }
}

/// A drag action as the compositor's data device names it.
fn action_of(atoms: &Atoms, atom: Atom) -> DndAction {
    if atom == atoms.XdndActionCopy {
        DndAction::Copy
    } else if atom == atoms.XdndActionMove {
        DndAction::Move
    } else if atom == atoms.XdndActionAsk {
        DndAction::Ask
    } else {
        DndAction::empty()
    }
}

fn atom_of(atoms: &Atoms, action: DndAction) -> Atom {
    if action.contains(DndAction::Move) {
        atoms.XdndActionMove
    } else if action.contains(DndAction::Ask) {
        atoms.XdndActionAsk
    } else if action.contains(DndAction::Copy) {
        atoms.XdndActionCopy
    } else {
        x11rb::NONE
    }
}

/// How an X11 name for a kind of data is said to Wayland programs, or None
/// for one that is no kind of data (or too long, or not text).
pub fn kind_named(atoms: &Atoms, atom: Atom, name: Option<&str>) -> Option<String> {
    if atom == atoms.UTF8_STRING {
        return Some("text/plain;charset=utf-8".into());
    }
    if atom == atoms.TEXT {
        return Some("text/plain".into());
    }
    let name = name?;
    // Names like STRING, TARGETS or COMPOUND_TEXT are not kinds a Wayland
    // program asks for.
    if !name.contains('/') || name.len() > MAX_KIND_NAME || name.chars().any(|c| c.is_control()) {
        return None;
    }
    Some(name.to_string())
}

impl X11Wm {
    fn dnd_atom_name(&self, atom: Atom) -> Option<String> {
        let reply = self.conn.get_atom_name(atom).ok()?.reply_unchecked().ok()??;
        String::from_utf8(reply.name).ok()
    }

    fn dnd_atom_list(&self, window: X11Window, property: Atom) -> Vec<Atom> {
        let prop = self
            .conn
            .get_property(false, window, property, AtomEnum::ATOM, 0, MAX_KINDS as u32)
            .ok()
            .and_then(|c| c.reply_unchecked().ok().flatten());
        prop.and_then(|p| p.value32().map(|v| v.take(MAX_KINDS).collect())).unwrap_or_default()
    }

    fn dnd_send(&self, to: X11Window, window: X11Window, type_: Atom, data: [u32; 5]) -> Result<(), ReplyOrIdError> {
        let event = ClientMessageEvent::new(32, window, type_, data);
        self.conn.send_event(false, to, EventMask::NO_EVENT, event)?;
        self.conn.flush()?;
        Ok(())
    }

    /// Reads what an X11 program in a drag says (messages to the window shown
    /// to it, or answers to the compositor's offer).
    pub(crate) fn handle_dnd_message(&mut self, msg: &ClientMessageEvent) -> Vec<XdndEvent> {
        let a = self.atoms;
        let d = msg.data.as_data32();
        let mut out = Vec::new();
        let proxy = self.dnd.proxy;
        if msg.type_ == a.XdndEnter && msg.window == proxy {
            let (source, flags) = (d[0], d[1]);
            let version = flags >> 24;
            // Only the program that took the drag selection can be dragging.
            if source != self.xdnd.owner || source == self.xdnd.window || !(OLDEST..=VERSION).contains(&version) {
                debug!(source, version, "ignored a drag that is not the selection's owner's");
                return out;
            }
            let atoms: Vec<Atom> = if flags & 1 != 0 {
                self.dnd_atom_list(source, a.XdndTypeList)
            } else {
                d[2..5].iter().copied().filter(|a| *a != 0).collect()
            };
            let mut kinds: Vec<(String, Atom)> = Vec::new();
            for atom in atoms.into_iter().take(MAX_KINDS) {
                let name = if atom == a.UTF8_STRING || atom == a.TEXT { None } else { self.dnd_atom_name(atom) };
                if let Some(kind) = kind_named(&a, atom, name.as_deref()) {
                    if !kinds.iter().any(|(k, _)| *k == kind) {
                        kinds.push((kind, atom));
                    }
                }
            }
            let actions = self
                .dnd_atom_list(source, a.XdndActionList)
                .into_iter()
                .fold(DndAction::empty(), |acc, at| acc | action_of(&a, at));
            out.push(XdndEvent::Enter { kinds: kinds.iter().map(|(k, _)| k.clone()).collect(), actions });
            self.dnd.from_x = Some(FromX { source, kinds, dropped: None });
        } else if msg.type_ == a.XdndPosition && msg.window == proxy {
            if self.dnd.from_x.as_ref().is_some_and(|f| f.source == d[0]) {
                out.push(XdndEvent::Position { action: action_of(&a, d[4]) });
            }
        } else if msg.type_ == a.XdndLeave && msg.window == proxy {
            if self.dnd.from_x.as_ref().is_some_and(|f| f.source == d[0] && f.dropped.is_none()) {
                self.dnd.from_x = None;
                out.push(XdndEvent::Leave);
            }
        } else if msg.type_ == a.XdndDrop && msg.window == proxy {
            if let Some(from) = self.dnd.from_x.as_mut().filter(|f| f.source == d[0] && f.dropped.is_none()) {
                from.dropped = Some(d[2]);
                out.push(XdndEvent::Drop);
            }
        } else if msg.type_ == a.XdndStatus && msg.window == self.xdnd.window {
            if self.dnd.to_x.as_ref().is_some_and(|t| t.window == d[0]) {
                out.push(XdndEvent::TargetStatus { accepted: d[1] & 1 != 0, action: action_of(&a, d[4]) });
            }
        } else if msg.type_ == a.XdndFinished && msg.window == self.xdnd.window {
            if self.dnd.to_x.as_ref().is_some_and(|t| t.window == d[0] && t.dropped) {
                out.push(XdndEvent::TargetFinished {
                    accepted: d[1] & 1 != 0,
                    action: if d[1] & 1 != 0 { action_of(&a, d[2]) } else { DndAction::empty() },
                });
            }
        }
        out
    }

    // ---- An X11 program drags to the compositor's side.

    /// Shows the window that takes an X11 source's drag (above every other
    /// window, across the screen), or takes it away.
    pub fn dnd_proxy(&mut self, show: bool) -> Result<(), ReplyOrIdError> {
        if show == self.dnd.proxy_mapped {
            return Ok(());
        }
        let proxy = self.dnd.proxy;
        if show {
            let root = self.conn.get_geometry(self.screen.root)?.reply()?;
            self.conn.configure_window(
                proxy,
                &ConfigureWindowAux::new()
                    .x(0)
                    .y(0)
                    .width(u32::from(root.width.max(1)))
                    .height(u32::from(root.height.max(1)))
                    .stack_mode(StackMode::ABOVE),
            )?;
            self.conn.map_window(proxy)?;
        } else {
            self.conn.unmap_window(proxy)?;
        }
        self.dnd.proxy_mapped = show;
        self.conn.flush()?;
        Ok(())
    }

    /// How many pixels an X11 coordinate is for one of the compositor's.
    pub fn client_scale(&self) -> f64 {
        self.client_scale.load(std::sync::atomic::Ordering::Acquire)
    }

    /// Whether the X11 source's drag is being shown to us now.
    pub fn dnd_from_x(&self) -> bool {
        self.dnd.from_x.is_some()
    }

    /// Tells the X11 source whether the drag would be taken where it is and
    /// the way it would be taken.
    pub fn dnd_answer_source(&mut self, accepted: bool, action: DndAction) -> Result<(), ReplyOrIdError> {
        let Some(from) = &self.dnd.from_x else { return Ok(()) };
        let a = self.atoms;
        let action = if accepted { atom_of(&a, action) } else { x11rb::NONE };
        let accepted = accepted && action != x11rb::NONE;
        self.dnd_send(from.source, from.source, a.XdndStatus, [self.dnd.proxy, u32::from(accepted), 0, 0, action])
    }

    /// Tells the X11 source its drop is finished with, and ends the drag.
    pub fn dnd_finish_source(&mut self, accepted: bool, action: DndAction) -> Result<(), ReplyOrIdError> {
        let Some(from) = self.dnd.from_x.take() else { return Ok(()) };
        let a = self.atoms;
        let action = if accepted { atom_of(&a, action) } else { x11rb::NONE };
        self.dnd_send(
            from.source,
            from.source,
            a.XdndFinished,
            [self.dnd.proxy, u32::from(accepted && action != x11rb::NONE), action, 0, 0],
        )
    }

    /// The X11 source's drag is over (given up, or ended some other way)
    /// without a word to it.
    pub fn dnd_forget_source(&mut self) {
        self.dnd.from_x = None;
        let slot = self.slot_mut(Slot::Xdnd);
        slot.incoming.clear();
    }

    /// Asks the X11 source for what it holds as `kind` and writes it to
    /// `fd`. Only after it let go; false if that has not happened.
    pub fn dnd_read_source<D>(
        &mut self,
        kind: &str,
        fd: OwnedFd,
        loop_handle: LoopHandle<'_, D>,
    ) -> Result<bool, SelectionError>
    where
        D: XwmHandler + 'static,
    {
        let Some(from) = &self.dnd.from_x else { return Ok(false) };
        let Some(time) = from.dropped else { return Ok(false) };
        let Some(atom) = from.kinds.iter().find(|(k, _)| k == kind).map(|(_, a)| *a) else { return Ok(false) };
        self.convert_into(Slot::Xdnd, atom, fd, loop_handle, time)?;
        Ok(true)
    }

    // ---- The compositor drags to an X11 program's window.

    /// Whether this window can take a drop, and from which window to ask it
    /// (itself, or the proxy it names), and in which version.
    fn dnd_target_of(&self, window: X11Window) -> Option<(X11Window, u32)> {
        let a = self.atoms;
        let read = |w: X11Window, property: Atom, type_: AtomEnum| -> Option<Vec<u32>> {
            let reply = self
                .conn
                .get_property(false, w, property, type_, 0, 1)
                .ok()?
                .reply_unchecked()
                .ok()??;
            reply.value32().map(|v| v.collect())
        };
        let mut send_to = window;
        if let Some(p) = read(window, a.XdndProxy, AtomEnum::WINDOW).and_then(|v| v.first().copied()) {
            // A proxy counts only if it names itself as its own.
            if read(p, a.XdndProxy, AtomEnum::WINDOW).and_then(|v| v.first().copied()) == Some(p) {
                send_to = p;
            }
        }
        let aware = read(send_to, a.XdndAware, AtomEnum::ATOM)?.first().copied()?;
        if aware < OLDEST {
            return None;
        }
        Some((send_to, aware.min(VERSION)))
    }

    /// Offers what a Wayland program drags to `window`: takes the drag
    /// selection and tells the window. False if it is not a window that takes
    /// drops.
    pub fn dnd_offer(
        &mut self,
        window: X11Window,
        kinds: &[String],
        actions: DndAction,
    ) -> Result<bool, ReplyOrIdError> {
        let a = self.atoms;
        let Some((send_to, version)) = self.dnd_target_of(window) else { return Ok(false) };
        if self.dnd.to_x.is_some() {
            self.dnd_offer_end()?;
        }
        let mut atoms: Vec<Atom> = Vec::new();
        for kind in kinds.iter().take(MAX_KINDS) {
            let literal = self.conn.intern_atom(false, kind.as_bytes())?.reply()?.atom;
            atoms.push(literal);
            if kind == "text/plain;charset=utf-8" {
                atoms.push(a.UTF8_STRING);
            } else if kind == "text/plain" {
                atoms.push(a.TEXT);
                atoms.push(AtomEnum::STRING.into());
            }
        }
        atoms.dedup();
        atoms.truncate(MAX_KINDS);
        let source = self.xdnd.window;
        let all_actions: Vec<Atom> = [DndAction::Copy, DndAction::Move, DndAction::Ask]
            .into_iter()
            .filter(|x| actions.contains(*x))
            .map(|x| atom_of(&a, x))
            .collect();
        self.conn.change_property32(PropMode::REPLACE, source, a.XdndActionList, AtomEnum::ATOM, &all_actions)?;
        let more = atoms.len() > 3;
        if more {
            self.conn.change_property32(PropMode::REPLACE, source, a.XdndTypeList, AtomEnum::ATOM, &atoms)?;
        }
        let xdnd = self.slot_mut(Slot::Xdnd);
        xdnd.mime_types = kinds.to_vec();
        self.conn.set_selection_owner(source, a.XdndSelection, x11rb::CURRENT_TIME)?;
        let t = |i: usize| if more { 0 } else { atoms.get(i).copied().unwrap_or(0) };
        self.dnd_send(
            send_to,
            window,
            a.XdndEnter,
            [source, (version << 24) | u32::from(more), t(0), t(1), t(2)],
        )?;
        self.dnd.to_x = Some(ToX { window, send_to, version, dropped: false });
        Ok(true)
    }

    /// The drag is at `x`, `y` on the screen, over the offered window, and
    /// would be taken as `action`.
    pub fn dnd_offer_position(&mut self, x: i32, y: i32, action: DndAction) -> Result<(), ReplyOrIdError> {
        let Some(to) = &self.dnd.to_x else { return Ok(()) };
        let a = self.atoms;
        let packed = (((x.clamp(0, 0xffff)) as u32) << 16) | (y.clamp(0, 0xffff) as u32);
        self.dnd_send(to.send_to, to.window, a.XdndPosition, [self.xdnd.window, 0, packed, 0, atom_of(&a, action)])
    }

    /// The drag left the offered window.
    pub fn dnd_offer_leave(&mut self) -> Result<(), ReplyOrIdError> {
        let Some(to) = &self.dnd.to_x else { return Ok(()) };
        if !to.dropped {
            let a = self.atoms;
            self.dnd_send(to.send_to, to.window, a.XdndLeave, [self.xdnd.window, 0, 0, 0, 0])?;
        }
        self.dnd_offer_end()
    }

    /// The drag was let go over the offered window.
    pub fn dnd_offer_drop(&mut self) -> Result<(), ReplyOrIdError> {
        let a = self.atoms;
        let source = self.xdnd.window;
        let Some(to) = &mut self.dnd.to_x else { return Ok(()) };
        if to.dropped {
            return Ok(());
        }
        to.dropped = true;
        let (send_to, window) = (to.send_to, to.window);
        self.dnd_send(send_to, window, a.XdndDrop, [source, 0, 0, 0, 0])
    }

    /// Nothing is offered any more: the selection is let go of, and what was
    /// still being passed to the window is dropped.
    pub fn dnd_offer_end(&mut self) -> Result<(), ReplyOrIdError> {
        if self.dnd.to_x.take().is_none() {
            return Ok(());
        }
        let a = self.atoms;
        let (window, timestamp) = (self.xdnd.window, self.xdnd.timestamp);
        let xdnd = self.slot_mut(Slot::Xdnd);
        xdnd.mime_types = Vec::new();
        xdnd.outgoing.clear();
        if xdnd.owner == window {
            self.conn.set_selection_owner(x11rb::NONE, a.XdndSelection, timestamp)?;
        }
        self.conn.flush()?;
        Ok(())
    }

    /// Whether a drag is offered to an X11 program's window now.
    pub fn dnd_offered(&self) -> bool {
        self.dnd.to_x.is_some()
    }

    /// The version of the program's XDND the offer is made in.
    pub fn dnd_offer_version(&self) -> Option<u32> {
        self.dnd.to_x.as_ref().map(|t| t.version)
    }

    /// The window the drag is offered to, if it is.
    pub fn dnd_offer_window(&self) -> Option<X11Window> {
        self.dnd.to_x.as_ref().map(|t| t.window)
    }

    /// Whether the window the drag is offered to has been let go on.
    pub fn dnd_offer_dropped(&self) -> bool {
        self.dnd.to_x.as_ref().is_some_and(|t| t.dropped)
    }
}
