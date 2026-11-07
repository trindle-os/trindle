//! .

use crate::{
    output::Output,
    utils::{x11rb::X11Source, Client, Logical, Point, Rectangle, Size},
    wayland::{
        selection::SelectionTarget,
        xwayland_shell::{self, XWaylandShellHandler},
    },
};
use atomic_float::AtomicF64;
use calloop::{generic::Generic, Interest, LoopHandle, Mode, PostAction, RegistrationToken};
use rustix::fs::OFlags;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt,
    os::unix::{
        io::{AsFd, BorrowedFd, OwnedFd},
        net::UnixStream,
    },
    sync::{atomic::Ordering, Arc},
};
use tracing::{debug, debug_span, info, trace, warn};
use wayland_server::Resource;

pub use x11rb::protocol::xproto::Window as X11Window;
use x11rb::{
    connection::Connection as _,
    errors::{ReplyError, ReplyOrIdError},
    protocol::{
        composite::{ConnectionExt as _, Redirect},
        randr::{ConnectionExt, Notify, NotifyMask},
        render::{ConnectionExt as _, CreatePictureAux, PictureWrapper},
        xfixes::{ConnectionExt as _, SelectionEventMask},
        xproto::{
            Atom, AtomEnum, ChangeWindowAttributesAux, ConfigWindow, ConfigureNotifyEvent,
            ConfigureWindowAux, ConnectionExt as _, CreateGCAux, CreateWindowAux, CursorWrapper, EventMask,
            FontWrapper, GcontextWrapper, GetPropertyReply, ImageFormat, PixmapWrapper, PropMode, Property,
            QueryExtensionReply, Screen, SelectionNotifyEvent, SelectionRequestEvent, StackMode, WindowClass,
            CONFIGURE_NOTIFY_EVENT, SELECTION_NOTIFY_EVENT,
        },
        Event,
    },
    rust_connection::{ConnectionError, DefaultStream, RustConnection},
    wrapper::ConnectionExt as _,
    COPY_DEPTH_FROM_PARENT,
};

pub mod settings;
use settings::{NameError, Value, XSettings};
mod dnd;
pub use self::dnd::{XdndEvent, MAX_KINDS, MAX_KIND_NAME};
mod surface;
pub use self::surface::*;

const INCR_CHUNK_SIZE: usize = 64 * 1024;

pub const MAX_TRANSFER_BYTES: usize = 64 << 20;

#[allow(missing_docs)]
mod atoms {
    x11rb::atom_manager! {
        pub Atoms:
        AtomsCookie {
            WL_SURFACE_ID,
            WL_SURFACE_SERIAL,

            _SMITHAY_CLOSE_CONNECTION,

            UTF8_STRING,
            TEXT,

            WM_HINTS,
            WM_PROTOCOLS,
            WM_TAKE_FOCUS,
            WM_DELETE_WINDOW,
            WM_CHANGE_STATE,
            _NET_WM_NAME,
            _NET_WM_MOVERESIZE,
            _NET_FRAME_EXTENTS,
            _NET_REQUEST_FRAME_EXTENTS,
            _NET_WM_PID,
            _NET_WM_WINDOW_TYPE,
            _NET_WM_WINDOW_TYPE_DROPDOWN_MENU,
            _NET_WM_WINDOW_TYPE_DIALOG,
            _NET_WM_WINDOW_TYPE_MENU,
            _NET_WM_WINDOW_TYPE_NOTIFICATION,
            _NET_WM_WINDOW_TYPE_NORMAL,
            _NET_WM_WINDOW_TYPE_POPUP_MENU,
            _NET_WM_WINDOW_TYPE_SPLASH,
            _NET_WM_WINDOW_TYPE_TOOLBAR,
            _NET_WM_WINDOW_TYPE_TOOLTIP,
            _NET_WM_WINDOW_TYPE_UTILITY,
            _NET_WM_STATE_MODAL,
            _MOTIF_WM_HINTS,
            _NET_STARTUP_ID,
            _XWAYLAND_RANDR_EMU_MONITOR_RECTS,

            WM_S0,
            WM_STATE,
            _NET_WM_CM_S0,
            _NET_SUPPORTED,
            _NET_ACTIVE_WINDOW,
            _NET_CLIENT_LIST,
            _NET_CLIENT_LIST_STACKING,
            _NET_WM_PING,
            _NET_WM_STATE,
            _NET_WM_STATE_MAXIMIZED_VERT,
            _NET_WM_STATE_MAXIMIZED_HORZ,
            _NET_WM_STATE_HIDDEN,
            _NET_WM_STATE_FULLSCREEN,
            _NET_WM_STATE_FOCUSED,
            _NET_SUPPORTING_WM_CHECK,
            _XSETTINGS_SETTINGS,

            _WL_SELECTION,
            CLIPBOARD_MANAGER,
            CLIPBOARD,
            PRIMARY,
            TARGETS,
            TIMESTAMP,
            INCR,
            DELETE,
            _XSETTINGS_S0,

            XdndAware,
            XdndEnter,
            XdndPosition,
            XdndStatus,
            XdndLeave,
            XdndDrop,
            XdndFinished,
            XdndSelection,
            XdndTypeList,
            XdndActionList,
            XdndActionCopy,
            XdndActionMove,
            XdndActionAsk,
            XdndActionPrivate,
            XdndProxy,
        }
    }
}
pub use self::atoms::Atoms;

use super::XWaylandClientData;

crate::utils::ids::id_gen!(xwm_id);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct XwmId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Reorder {
    Top,
    Above(X11Window),
    Below(X11Window),
    Bottom,
}

enum StackingDirection {
    Downwards,
    Upwards,
}

impl StackingDirection {
    fn pos_comparator(&self, pos: usize, last_pos: usize) -> bool {
        match self {
            Self::Downwards => last_pos < pos,
            Self::Upwards => last_pos > pos,
        }
    }

    fn stack_mode(&self) -> StackMode {
        match self {
            Self::Downwards => StackMode::BELOW,
            Self::Upwards => StackMode::ABOVE,
        }
    }
}

pub trait XwmHandler {
    fn xwm_state(&mut self, xwm: XwmId) -> &mut X11Wm;

    fn new_window(&mut self, xwm: XwmId, window: X11Surface);
    fn new_override_redirect_window(&mut self, xwm: XwmId, window: X11Surface);
    fn map_window_request(&mut self, xwm: XwmId, window: X11Surface);
    fn map_window_notify(&mut self, xwm: XwmId, window: X11Surface) {
        let _ = (xwm, window);
    }
    fn mapped_override_redirect_window(&mut self, xwm: XwmId, window: X11Surface);
    fn unmapped_window(&mut self, xwm: XwmId, window: X11Surface);
    fn destroyed_window(&mut self, xwm: XwmId, window: X11Surface);

    #[allow(clippy::too_many_arguments)]
    fn configure_request(
        &mut self,
        xwm: XwmId,
        window: X11Surface,
        x: Option<i32>,
        y: Option<i32>,
        w: Option<u32>,
        h: Option<u32>,
        reorder: Option<Reorder>,
    );
    fn configure_notify(
        &mut self,
        xwm: XwmId,
        window: X11Surface,
        geometry: Rectangle<i32, Logical>,
        above: Option<X11Window>,
    );
    fn property_notify(&mut self, xwm: XwmId, window: X11Surface, property: WmWindowProperty) {
        let _ = (xwm, window, property);
    }
    fn maximize_request(&mut self, xwm: XwmId, window: X11Surface) {
        let _ = (xwm, window);
    }
    fn unmaximize_request(&mut self, xwm: XwmId, window: X11Surface) {
        let _ = (xwm, window);
    }
    fn fullscreen_request(&mut self, xwm: XwmId, window: X11Surface) {
        let _ = (xwm, window);
    }
    fn unfullscreen_request(&mut self, xwm: XwmId, window: X11Surface) {
        let _ = (xwm, window);
    }
    fn minimize_request(&mut self, xwm: XwmId, window: X11Surface) {
        let _ = (xwm, window);
    }
    fn unminimize_request(&mut self, xwm: XwmId, window: X11Surface) {
        let _ = (xwm, window);
    }

    fn resize_request(&mut self, xwm: XwmId, window: X11Surface, button: u32, resize_edge: ResizeEdge);
    fn move_request(&mut self, xwm: XwmId, window: X11Surface, button: u32);

    fn xdnd_event(&mut self, xwm: XwmId, event: XdndEvent) {
        let _ = (xwm, event);
    }

    fn send_xdnd(&mut self, xwm: XwmId, mime_type: String, fd: OwnedFd) {
        let _ = (xwm, mime_type, fd);
    }

    fn allow_selection_access(&mut self, xwm: XwmId, selection: SelectionTarget) -> bool {
        let _ = (xwm, selection);
        false
    }

    fn send_selection(&mut self, xwm: XwmId, selection: SelectionTarget, mime_type: String, fd: OwnedFd) {
        let _ = (xwm, selection, mime_type, fd);
        panic!("`allow_selection_access` returned true without `send_selection` implementation to handle transfers.");
    }

    fn new_selection(&mut self, xwm: XwmId, selection: SelectionTarget, mime_types: Vec<String>) {
        let _ = (xwm, selection, mime_types);
    }

    fn cleared_selection(&mut self, xwm: XwmId, selection: SelectionTarget) {
        let _ = (xwm, selection);
    }

    fn randr_primary_output_change(&mut self, xwm: XwmId, output_name: Option<String>) {
        let _ = (xwm, output_name);
    }

    fn disconnected(&mut self, _xwm: XwmId) {}
}

#[derive(Debug)]
pub struct X11Wm {
    id: XwmId,
    conn: Arc<RustConnection>,
    client_scale: Arc<AtomicF64>,
    screen: Screen,
    wm_window: X11Window,
    atoms: Atoms,
    xsettings: XSettings,
    randr_primary: u32,

    pub(crate) unpaired_surfaces: HashMap<u64, X11Window>,
    sequences_to_ignore: BinaryHeap<Reverse<u16>>,

    _xfixes_data: QueryExtensionReply,
    clipboard: XWmSelection,
    primary: XWmSelection,
    xdnd: XWmSelection,
    dnd: dnd::DndState,

    pub(crate) windows: Vec<X11Surface>,
    client_list: Vec<X11Window>,
    client_list_stacking: Vec<X11Window>,

    span: tracing::Span,
}

impl Drop for X11Wm {
    fn drop(&mut self) {
        let _ = self.conn.destroy_window(self.wm_window);
        xwm_id::remove(self.id.0);
    }
}

#[derive(Debug)]
struct XWmSelection {
    atom: Atom,
    slot: Slot,

    conn: Arc<RustConnection>,
    window: X11Window,
    owner: X11Window,
    mime_types: Vec<String>,
    timestamp: u32,

    incoming: Vec<IncomingTransfer>,
    outgoing: Vec<OutgoingTransfer>,
}

macro_rules! sel {
    ($xwm:expr, $slot:expr) => {
        match $slot {
            Slot::Clipboard => &mut $xwm.clipboard,
            Slot::Primary => &mut $xwm.primary,
            Slot::Xdnd => &mut $xwm.xdnd,
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Slot {
    Clipboard,
    Primary,
    Xdnd,
}

impl Slot {
    fn of(target: SelectionTarget) -> Slot {
        match target {
            SelectionTarget::Clipboard => Slot::Clipboard,
            SelectionTarget::Primary => Slot::Primary,
        }
    }
}

struct IncomingTransfer {
    conn: Arc<RustConnection>,
    token: Option<RegistrationToken>,
    window: X11Window,

    incr: bool,
    source_data: Vec<u8>,
    incr_done: bool,
    too_big: bool,
}

impl fmt::Debug for IncomingTransfer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IncomingTransfer")
            .field("conn", &"...")
            .field("token", &self.token)
            .field("window", &self.window)
            .field("incr", &self.incr)
            .field("source_data", &self.source_data)
            .field("incr_done", &self.incr_done)
            .finish()
    }
}

impl IncomingTransfer {
    fn read_selection_prop(&mut self, reply: GetPropertyReply) {
        if self.too_big {
            return;
        }
        if self.source_data.len() + reply.value.len() > MAX_TRANSFER_BYTES {
            self.too_big = true;
            self.source_data.clear();
            return;
        }
        self.source_data.extend(&reply.value)
    }

    fn write_selection(&mut self, fd: BorrowedFd<'_>) -> std::io::Result<bool> {
        if self.source_data.is_empty() {
            return Ok(true);
        }

        let len = rustix::io::write(fd, &self.source_data)?;
        self.source_data = self.source_data.split_off(len);

        Ok(self.source_data.is_empty())
    }

    fn destroy<D>(mut self, handle: &LoopHandle<'_, D>) {
        if let Some(token) = self.token.take() {
            handle.remove(token);
        }
    }
}

impl Drop for IncomingTransfer {
    fn drop(&mut self) {
        let _ = self.conn.destroy_window(self.window);
        if self.token.is_some() {
            tracing::warn!(
                ?self,
                "IncomingTransfer freed before being removed from EventLoop"
            );
        }
    }
}

struct OutgoingTransfer {
    conn: Arc<RustConnection>,
    token: Option<RegistrationToken>,

    incr: bool,
    source_data: Vec<u8>,
    request: SelectionRequestEvent,

    property_set: bool,
    flush_property_on_delete: bool,
    answered: bool,
    sent_finished: bool,
}

impl fmt::Debug for OutgoingTransfer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OutgoingTransfer")
            .field("conn", &"...")
            .field("token", &self.token)
            .field("incr", &self.incr)
            .field("source_data", &self.source_data)
            .field("request", &self.request)
            .field("property_set", &self.property_set)
            .field("flush_property_on_delete", &self.flush_property_on_delete)
            .field("answered", &self.answered)
            .finish()
    }
}

impl OutgoingTransfer {
    fn flush_data(&mut self) -> Result<usize, ReplyOrIdError> {
        let len = std::cmp::min(self.source_data.len(), INCR_CHUNK_SIZE);

        if len == 0 {
            self.sent_finished = true;
        }

        let mut data = self.source_data.split_off(len);
        std::mem::swap(&mut data, &mut self.source_data);

        self.conn.change_property8(
            PropMode::REPLACE,
            self.request.requestor,
            self.request.property,
            self.request.target,
            &data,
        )?;
        self.conn.flush()?;

        let remaining = self.source_data.len();
        self.property_set = true;
        Ok(remaining)
    }

    fn destroy<D>(mut self, handle: &LoopHandle<'_, D>) {
        if let Some(token) = self.token.take() {
            handle.remove(token);
        }
    }
}

impl Drop for OutgoingTransfer {
    fn drop(&mut self) {
        if self.token.is_some() {
            tracing::warn!(
                ?self,
                "OutgoingTransfer freed before being removed from EventLoop"
            );
        }
    }
}

impl XWmSelection {
    fn new(
        conn: &Arc<RustConnection>,
        screen: &Screen,
        atoms: &Atoms,
        atom: Atom,
    ) -> Result<Self, ReplyOrIdError> {
        let window = conn.generate_id()?;
        conn.create_window(
            screen.root_depth,
            window,
            screen.root,
            0,
            0,
            10,
            10,
            0,
            WindowClass::INPUT_OUTPUT,
            screen.root_visual,
            &CreateWindowAux::new().event_mask(EventMask::PROPERTY_CHANGE),
        )?;

        if atom == atoms.CLIPBOARD {
            conn.set_selection_owner(window, atoms.CLIPBOARD_MANAGER, x11rb::CURRENT_TIME)?;
        }
        conn.xfixes_select_selection_input(
            window,
            atom,
            SelectionEventMask::SET_SELECTION_OWNER
                | SelectionEventMask::SELECTION_WINDOW_DESTROY
                | SelectionEventMask::SELECTION_CLIENT_CLOSE,
        )?;
        conn.flush()?;

        let selection = match atom {
            x if x == atoms.CLIPBOARD => Slot::Clipboard,
            x if x == atoms.PRIMARY => Slot::Primary,
            x if x == atoms.XdndSelection => Slot::Xdnd,
            _ => unreachable!(),
        };

        debug!(
            selection_window = ?window,
            ?selection,
            ?atom,
            "Selection init",
        );

        Ok(XWmSelection {
            atom,
            slot: selection,
            conn: conn.clone(),
            window,
            owner: window,
            mime_types: Vec::new(),
            timestamp: x11rb::CURRENT_TIME,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        })
    }
}

impl Drop for XWmSelection {
    fn drop(&mut self) {
        let _ = self.conn.destroy_window(self.window);
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum ResizeEdge {
    Top,
    Bottom,
    Left,
    TopLeft,
    BottomLeft,
    Right,
    TopRight,
    BottomRight,
}

#[derive(thiserror::Error, Debug)]
pub enum SelectionError {
    #[error(transparent)]
    X11Error(#[from] ReplyOrIdError),
    #[error(transparent)]
    Calloop(#[from] calloop::Error),
    #[error("Unable to determine ATOM matching mime-type")]
    UnableToDetermineAtom,
}

impl From<ConnectionError> for SelectionError {
    #[inline]
    fn from(err: ConnectionError) -> Self {
        SelectionError::X11Error(err.into())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error(transparent)]
    X11Error(#[from] ConnectionError),
    #[error("Name '{name}' isn't valid: {error}")]
    NameError {
        name: String,
        #[source]
        error: NameError,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum PrimaryOutputError {
    #[error(transparent)]
    X11Error(#[from] ReplyError),
    #[error("The provided output wasn't known to Xwayland")]
    OutputUnknown,
}

impl From<ConnectionError> for PrimaryOutputError {
    fn from(value: ConnectionError) -> Self {
        PrimaryOutputError::X11Error(value.into())
    }
}

impl X11Wm {
    pub fn start_wm<D>(
        handle: LoopHandle<'static, D>,
        connection: UnixStream,
        client: wayland_server::Client,
    ) -> Result<Self, Box<dyn std::error::Error>>
    where
        D: XwmHandler,
        D: xwayland_shell::XWaylandShellHandler,
        D: 'static,
    {
        let id = XwmId(xwm_id::next());
        let span = debug_span!("xwayland_wm", id = id.0);
        let _guard = span.enter();

        let screen = 0;
        let stream = DefaultStream::from_unix_stream(connection)?.0;
        let conn = RustConnection::connect_to_stream(stream, screen)?;
        let atoms = Atoms::new(&conn)?.reply()?;
        let screen = conn.setup().roots[0].clone();
        let randr_primary = conn.randr_get_output_primary(screen.root)?.reply()?.output;

        {
            let font = FontWrapper::open_font(&conn, "cursor".as_bytes())?;
            let cursor = CursorWrapper::create_glyph_cursor(
                &conn,
                font.font(),
                font.font(),
                68,
                69,
                0,
                0,
                0,
                u16::MAX,
                u16::MAX,
                u16::MAX,
            )?;

            conn.change_window_attributes(
                screen.root,
                &ChangeWindowAttributesAux::default()
                    .event_mask(
                        EventMask::SUBSTRUCTURE_REDIRECT
                            | EventMask::SUBSTRUCTURE_NOTIFY
                            | EventMask::PROPERTY_CHANGE
                            | EventMask::FOCUS_CHANGE,
                    )
                    .cursor(cursor.cursor()),
            )?;
            conn.randr_select_input(screen.root, NotifyMask::OUTPUT_CHANGE)?;
        }

        let win = conn.generate_id()?;
        conn.create_window(
            screen.root_depth,
            win,
            screen.root,
            0,
            0,
            1,
            1,
            0,
            WindowClass::INPUT_OUTPUT,
            x11rb::COPY_FROM_PARENT,
            &Default::default(),
        )?;
        conn.set_selection_owner(win, atoms.WM_S0, x11rb::CURRENT_TIME)?;
        conn.set_selection_owner(win, atoms._NET_WM_CM_S0, x11rb::CURRENT_TIME)?;
        conn.composite_redirect_subwindows(screen.root, Redirect::MANUAL)?;

        conn.change_property32(
            PropMode::REPLACE,
            screen.root,
            atoms._NET_SUPPORTED,
            AtomEnum::ATOM,
            &[
                atoms._NET_WM_STATE,
                atoms._NET_WM_STATE_MAXIMIZED_HORZ,
                atoms._NET_WM_STATE_MAXIMIZED_VERT,
                atoms._NET_WM_STATE_HIDDEN,
                atoms._NET_WM_STATE_FULLSCREEN,
                atoms._NET_WM_STATE_MODAL,
                atoms._NET_WM_STATE_FOCUSED,
                atoms._NET_ACTIVE_WINDOW,
                atoms._NET_WM_MOVERESIZE,
                atoms._NET_FRAME_EXTENTS,
                atoms._NET_REQUEST_FRAME_EXTENTS,
                atoms._NET_CLIENT_LIST,
                atoms._NET_CLIENT_LIST_STACKING,
            ],
        )?;
        conn.change_property32(
            PropMode::REPLACE,
            screen.root,
            atoms._NET_CLIENT_LIST,
            AtomEnum::WINDOW,
            &[],
        )?;
        conn.change_property32(
            PropMode::REPLACE,
            screen.root,
            atoms._NET_CLIENT_LIST_STACKING,
            AtomEnum::WINDOW,
            &[],
        )?;
        conn.change_property32(
            PropMode::REPLACE,
            screen.root,
            atoms._NET_ACTIVE_WINDOW,
            AtomEnum::WINDOW,
            &[0],
        )?;
        conn.change_property32(
            PropMode::REPLACE,
            screen.root,
            atoms._NET_SUPPORTING_WM_CHECK,
            AtomEnum::WINDOW,
            &[win],
        )?;
        conn.change_property32(
            PropMode::REPLACE,
            win,
            atoms._NET_SUPPORTING_WM_CHECK,
            AtomEnum::WINDOW,
            &[win],
        )?;
        conn.change_property8(
            PropMode::REPLACE,
            win,
            atoms._NET_WM_NAME,
            atoms.UTF8_STRING,
            "Smithay X WM".as_bytes(),
        )?;
        debug!(window = win, "Created WM Window");
        let xsettings = XSettings::new(&conn, screen.root_depth, screen.root, &atoms)?;
        conn.flush()?;

        let conn = Arc::new(conn);
        let source = X11Source::new(Arc::clone(&conn), win, atoms._SMITHAY_CLOSE_CONNECTION);

        let client_data = client.get_data::<XWaylandClientData>().unwrap();
        let client_scale = client_data.compositor_state.clone_client_scale();

        client_data.user_data().insert_if_missing(|| id);

        let _xfixes_data = conn
            .query_extension(x11rb::protocol::xfixes::X11_EXTENSION_NAME.as_bytes())?
            .reply_unchecked()?
            .ok_or(ConnectionError::UnsupportedExtension)?;
        if !_xfixes_data.present {
            return Err(ConnectionError::UnsupportedExtension.into());
        }
        conn.xfixes_query_version(1, 0)?.reply_unchecked()?;

        let clipboard = XWmSelection::new(&conn, &screen, &atoms, atoms.CLIPBOARD)?;
        let primary = XWmSelection::new(&conn, &screen, &atoms, atoms.PRIMARY)?;
        let xdnd = XWmSelection::new(&conn, &screen, &atoms, atoms.XdndSelection)?;
        let dnd = dnd::DndState::new(&conn, &screen, &atoms)?;

        drop(_guard);
        let wm = Self {
            id,
            conn,
            client_scale,
            screen,
            atoms,
            xsettings,
            randr_primary,
            wm_window: win,
            _xfixes_data,
            clipboard,
            primary,
            xdnd,
            dnd,
            unpaired_surfaces: Default::default(),
            sequences_to_ignore: Default::default(),
            windows: Vec::new(),
            client_list: Vec::new(),
            client_list_stacking: Vec::new(),
            span,
        };

        let event_handle = handle.clone();
        handle.insert_source(source, move |event, _, data| match event {
            calloop::channel::Event::Msg(event) => {
                if let Err(err) = handle_event(&event_handle, data, id, event) {
                    warn!(id = id.0, err = ?err, "Failed to handle X11 event");
                }
            }
            calloop::channel::Event::Closed => {
                data.disconnected(id);
            }
        })?;
        Ok(wm)
    }

    fn slot_mut(&mut self, slot: Slot) -> &mut XWmSelection {
        match slot {
            Slot::Clipboard => &mut self.clipboard,
            Slot::Primary => &mut self.primary,
            Slot::Xdnd => &mut self.xdnd,
        }
    }

    fn slot_of_atom(&self, atom: Atom) -> Option<Slot> {
        match atom {
            x if x == self.atoms.CLIPBOARD => Some(Slot::Clipboard),
            x if x == self.atoms.PRIMARY => Some(Slot::Primary),
            x if x == self.atoms.XdndSelection => Some(Slot::Xdnd),
            _ => None,
        }
    }

    fn slot_incoming_on(&self, window: X11Window) -> Option<Slot> {
        [Slot::Clipboard, Slot::Primary, Slot::Xdnd].into_iter().find(|s| {
            let sel = match s {
                Slot::Clipboard => &self.clipboard,
                Slot::Primary => &self.primary,
                Slot::Xdnd => &self.xdnd,
            };
            sel.incoming.iter().any(|t| t.window == window)
        })
    }

    pub fn id(&self) -> XwmId {
        self.id
    }

    pub fn raise_window<'a, W: X11Relatable + 'a>(&mut self, window: &'a W) -> Result<(), ConnectionError> {
        if let Some(elem) = self.windows.iter().find(|s| window.is_window(s)) {
            if self.client_list_stacking.last() == Some(&elem.window_id()) {
                return Ok(());
            }

            let _guard = scopeguard::guard((), |_| {
                let _ = self.conn.ungrab_server();
                let _ = self.conn.flush();
            });
            self.conn.grab_server()?;
            self.conn.configure_window(
                elem.mapped_window_id().unwrap_or_else(|| elem.window_id()),
                &ConfigureWindowAux::new().stack_mode(StackMode::ABOVE),
            )?;
            self.client_list_stacking.retain(|e| *e != elem.window_id());
            self.client_list_stacking.push(elem.window_id());
            self.conn.change_property32(
                PropMode::REPLACE,
                self.screen.root,
                self.atoms._NET_CLIENT_LIST_STACKING,
                AtomEnum::WINDOW,
                &self.client_list_stacking,
            )?;
        }
        Ok(())
    }

    fn update_stacking_order_impl<'a, W: X11Relatable + 'a>(
        &mut self,
        order: impl Iterator<Item = &'a W>,
        direction: StackingDirection,
    ) -> Result<(), ConnectionError> {
        let _guard = scopeguard::guard((), |_| {
            let _ = self.conn.ungrab_server();
            let _ = self.conn.flush();
        });
        self.conn.grab_server()?;

        let mut last_pos = None;
        let mut changed = false;

        for relatable in order {
            let stacking_ordered_elems: Vec<&X11Surface> = self
                .client_list_stacking
                .iter()
                .filter_map(|w| self.windows.iter().find(|s| s.window_id() == *w))
                .collect();
            let pos = stacking_ordered_elems.iter().position(|w| relatable.is_window(w));
            if let (Some(pos), Some(last_pos)) = (pos, last_pos) {
                if direction.pos_comparator(pos, last_pos) {
                    let sibling = stacking_ordered_elems[last_pos];
                    let sibling_id = self.client_list_stacking[last_pos];
                    let elem = stacking_ordered_elems[pos];
                    let elem_id = self.client_list_stacking.remove(pos);
                    self.conn.configure_window(
                        elem.mapped_window_id().unwrap_or(elem_id),
                        &ConfigureWindowAux::new()
                            .sibling(sibling.mapped_window_id().unwrap_or(sibling_id))
                            .stack_mode(direction.stack_mode()),
                    )?;
                    self.client_list_stacking.insert(last_pos, elem_id);
                    changed = true;
                    continue;
                }
            }

            if pos.is_some() {
                last_pos = pos;
            }
        }

        if changed {
            self.conn.change_property32(
                PropMode::REPLACE,
                self.screen.root,
                self.atoms._NET_CLIENT_LIST_STACKING,
                AtomEnum::WINDOW,
                &self.client_list_stacking,
            )?;
        }
        Ok(())
    }

    pub fn update_stacking_order_downwards<'a, W: X11Relatable + 'a>(
        &mut self,
        order: impl Iterator<Item = &'a W>,
    ) -> Result<(), ConnectionError> {
        self.update_stacking_order_impl(order, StackingDirection::Downwards)
    }

    pub fn update_stacking_order_upwards<'a, W: X11Relatable + 'a>(
        &mut self,
        order: impl Iterator<Item = &'a W>,
    ) -> Result<(), ConnectionError> {
        self.update_stacking_order_impl(order, StackingDirection::Upwards)
    }

    pub fn set_cursor(
        &mut self,
        pixels: &[u8],
        size: Size<u16, Logical>,
        hotspot: Point<u16, Logical>,
    ) -> Result<(), ReplyOrIdError> {
        assert!(pixels.len() >= size.w as usize * size.h as usize * 4usize);
        let pixmap = PixmapWrapper::create_pixmap(&*self.conn, 32, self.screen.root, size.w, size.h)?;
        let Some(render_format) = self
            .conn
            .render_query_pict_formats()?
            .reply_unchecked()?
            .unwrap_or_default()
            .formats
            .into_iter()
            .filter(|f| f.depth == 32)
            .map(|f| f.id)
            .next()
        else {
            return Err(ReplyOrIdError::ConnectionError(ConnectionError::UnknownError));
        };
        let picture = PictureWrapper::create_picture(
            &*self.conn,
            pixmap.pixmap(),
            render_format,
            &CreatePictureAux::new(),
        )?;
        {
            let gc = GcontextWrapper::create_gc(&*self.conn, pixmap.pixmap(), &CreateGCAux::new())?;
            self.conn.put_image(
                ImageFormat::Z_PIXMAP,
                pixmap.pixmap(),
                gc.gcontext(),
                size.w,
                size.h,
                0,
                0,
                0,
                32,
                pixels,
            )?;
        }
        let cursor = self.conn.generate_id()?;
        self.conn
            .render_create_cursor(cursor, picture.picture(), hotspot.x, hotspot.y)?;
        self.conn
            .change_window_attributes(self.screen.root, &ChangeWindowAttributesAux::new().cursor(cursor))?;
        let _ = self.conn.free_cursor(cursor);
        Ok(())
    }

    pub fn new_selection(
        &mut self,
        selection: SelectionTarget,
        mime_types: Option<Vec<String>>,
    ) -> Result<(), ReplyOrIdError> {
        let selection = match selection {
            SelectionTarget::Clipboard => &mut self.clipboard,
            SelectionTarget::Primary => &mut self.primary,
        };

        if let Some(mime_types) = mime_types {
            selection.mime_types = mime_types;
            self.conn
                .set_selection_owner(selection.window, selection.atom, x11rb::CURRENT_TIME)?;
        } else if selection.owner == selection.window {
            selection.mime_types = Vec::new();
            self.conn
                .set_selection_owner(x11rb::NONE, selection.atom, selection.timestamp)?;
        }
        self.conn.flush()?;

        Ok(())
    }

    pub fn send_selection<D>(
        &mut self,
        selection: SelectionTarget,
        mime_type: String,
        fd: OwnedFd,
        loop_handle: LoopHandle<'_, D>,
    ) -> Result<(), SelectionError>
    where
        D: XwmHandler + 'static,
    {
        let slot = Slot::of(selection);
        let selection = match selection {
            SelectionTarget::Clipboard => &mut self.clipboard,
            SelectionTarget::Primary => &mut self.primary,
        };

        info!(
            selection = ?selection.slot,
            ?mime_type,
            "Send request from XWayland",
        );

        let atom = match &*mime_type {
            "text/plain;charset=utf-8" => self.atoms.UTF8_STRING,
            "text/plain" => self.atoms.TEXT,
            x => {
                let prop = self
                    .conn
                    .get_property(true, selection.window, self.atoms.TARGETS, AtomEnum::ANY, 0, 4096)?
                    .reply_unchecked()?
                    .ok_or(SelectionError::UnableToDetermineAtom)?;
                if prop.type_ != AtomEnum::ATOM.into() {
                    return Err(SelectionError::UnableToDetermineAtom);
                }
                let values = prop.value32().ok_or(SelectionError::UnableToDetermineAtom)?;

                let Some(atom) = values
                    .filter_map(|atom| {
                        let cookie = self.conn.get_atom_name(atom).ok()?;
                        let reply = cookie.reply_unchecked().ok()?;
                        std::str::from_utf8(&reply?.name)
                            .ok()
                            .map(|name| (atom, name.to_string()))
                    })
                    .find_map(|(atom, name)| if name == x { Some(atom) } else { None })
                else {
                    return Err(SelectionError::UnableToDetermineAtom);
                };

                atom
            }
        };

        debug!("Mime-type {:?} / Atom {:?}", mime_type, atom);

        self.convert_into(slot, atom, fd, loop_handle, x11rb::CURRENT_TIME)
    }

    fn convert_into<D>(
        &mut self,
        slot: Slot,
        atom: Atom,
        fd: OwnedFd,
        loop_handle: LoopHandle<'_, D>,
        time: u32,
    ) -> Result<(), SelectionError>
    where
        D: XwmHandler + 'static,
    {
        let xwm_id = self.id();
        let incoming_window = self.conn.generate_id()?;
        self.conn.create_window(
            self.screen.root_depth,
            incoming_window,
            self.screen.root,
            0,
            0,
            1,
            1,
            0,
            WindowClass::INPUT_OUTPUT,
            self.screen.root_visual,
            &CreateWindowAux::new().event_mask(EventMask::PROPERTY_CHANGE),
        )?;
        let selection = self.slot_mut(slot);
        let selection_atom = selection.atom;
        self.conn.convert_selection(
            incoming_window,
            selection_atom,
            atom,
            self.atoms._WL_SELECTION,
            time,
        )?;

        if let Err(err) = rustix::fs::fcntl_setfl(&fd, OFlags::WRONLY | OFlags::NONBLOCK) {
            warn!(?err, "Failed to restrict wl file descriptor");
        }

        let loop_handle_clone = loop_handle.clone();
        let token = loop_handle
            .insert_source(
                Generic::new(fd, Interest::WRITE, Mode::Level),
                move |_, fd, data| {
                    let xwm = data.xwm_state(xwm_id);
                    let conn = xwm.conn.clone();
                    let atoms = xwm.atoms;
                    let selection = sel!(xwm, slot);
                    let mut finished = None;
                    if let Some(transfer) = selection
                        .incoming
                        .iter_mut()
                        .find(|t| t.window == incoming_window)
                    {
                        let whole = !transfer.too_big;
                        match write_selection_callback(fd.as_fd(), &conn, &atoms, transfer) {
                            Ok(IncomingAction::WaitForWritable) => return Ok(PostAction::Continue),
                            Ok(IncomingAction::WaitForProperty) if !transfer.incr_done => {
                                return Ok(PostAction::Disable)
                            }
                            result => {
                                finished = Some(result.is_ok() && whole);
                                if let Some(pos) = selection
                                    .incoming
                                    .iter()
                                    .position(|t| t.window == incoming_window)
                                {
                                    selection.incoming.remove(pos).destroy(&loop_handle_clone);
                                }
                            }
                        };
                    }
                    if let (Slot::Xdnd, Some(complete)) = (slot, finished) {
                        data.xdnd_event(xwm_id, XdndEvent::ReadDone { complete });
                    }
                    Ok(PostAction::Remove)
                },
            )
            .map_err(|err| err.error)?;
        loop_handle.disable(&token)?;

        let transfer = IncomingTransfer {
            conn: self.conn.clone(),
            token: Some(token),
            window: incoming_window,
            incr: false,
            source_data: Vec::new(),
            incr_done: false,
            too_big: false,
        };
        let selection = self.slot_mut(slot);
        selection.incoming.push(transfer);

        self.conn.flush()?;
        Ok(())
    }

    pub fn set_xsettings(
        &mut self,
        settings: impl Iterator<Item = (String, Value)>,
    ) -> Result<(), SettingsError> {
        for (name, value) in settings {
            self.xsettings
                .set(name.clone(), value)
                .map_err(move |error| SettingsError::NameError { name, error })?;
        }

        self.xsettings.update(&self.conn)?;

        Ok(())
    }

    pub fn get_randr_primary_output(&self) -> Result<Option<String>, ReplyError> {
        let current_primary = self
            .conn
            .randr_get_output_primary(self.screen.root)?
            .reply()?
            .output;
        if current_primary == x11rb::NONE {
            return Ok(None);
        }

        let res = self
            .conn
            .randr_get_screen_resources_current(self.screen.root)?
            .reply()?;
        let info = self
            .conn
            .randr_get_output_info(current_primary, res.config_timestamp)?
            .reply()?;
        Ok(Some(
            std::str::from_utf8(&info.name)
                .expect("X11 protocol violation, output name not utf-8")
                .to_string(),
        ))
    }

    pub fn set_randr_primary_output(&mut self, output: Option<&Output>) -> Result<(), PrimaryOutputError> {
        let current_primary = self
            .conn
            .randr_get_output_primary(self.screen.root)?
            .reply()?
            .output;
        let Some(output) = output else {
            if current_primary != x11rb::NONE {
                let cookie = self
                    .conn
                    .randr_set_output_primary(self.screen.root, x11rb::NONE)?;
                self.sequences_to_ignore
                    .push(Reverse(cookie.sequence_number() as u16));
            }
            return Ok(());
        };

        let res = self
            .conn
            .randr_get_screen_resources_current(self.screen.root)?
            .reply()?;
        for output_xid in res.outputs {
            let info = self
                .conn
                .randr_get_output_info(output_xid, res.config_timestamp)?
                .reply()?;
            let name =
                std::str::from_utf8(&info.name).expect("X11 protocol violation, output name not utf-8");
            if name == output.name() {
                if output_xid != current_primary {
                    let cookie = self.conn.randr_set_output_primary(self.screen.root, output_xid)?;
                    self.sequences_to_ignore
                        .push(Reverse(cookie.sequence_number() as u16));
                    return Ok(());
                }
            }
        }

        Err(PrimaryOutputError::OutputUnknown)
    }
}

fn handle_event<D>(
    loop_handle: &LoopHandle<'_, D>,
    state: &mut D,
    xwm_id: XwmId,
    event: Event,
) -> Result<(), ReplyOrIdError>
where
    D: XwmHandler,
    D: xwayland_shell::XWaylandShellHandler,
    D: 'static,
{
    let xwm = state.xwm_state(xwm_id);
    let _guard = xwm.span.enter();
    let conn = xwm.conn.clone();

    let mut should_ignore = false;
    if let Some(seqno) = event.wire_sequence_number() {
        while let Some(&Reverse(to_ignore)) = xwm.sequences_to_ignore.peek() {
            if to_ignore.wrapping_sub(seqno) <= u16::MAX / 2 {
                should_ignore = to_ignore == seqno;
                break;
            }
            xwm.sequences_to_ignore.pop();
        }
    }

    debug!(
        event = ?event,
        should_ignore = should_ignore,
        "Got X11 event",
    );
    if should_ignore {
        return Ok(());
    }

    match event {
        Event::CreateNotify(n) => {
            if n.window == xwm.wm_window
                || n.window == xwm.xsettings.window
                || n.window == xwm.clipboard.window
                || xwm.clipboard.incoming.iter().any(|i| n.window == i.window)
                || n.window == xwm.primary.window
                || xwm.primary.incoming.iter().any(|i| n.window == i.window)
                || n.window == xwm.xdnd.window
                || xwm.xdnd.incoming.iter().any(|i| n.window == i.window)
                || xwm.dnd.is_ours(n.window)
            {
                return Ok(());
            }

            xwm.conn.change_window_attributes(
                n.window,
                &ChangeWindowAttributesAux::new().event_mask(EventMask::PROPERTY_CHANGE),
            )?;
            xwm.conn.flush()?;

            if xwm.windows.iter().any(|s| s.mapped_window_id() == Some(n.window)) {
                return Ok(());
            }

            let geo = conn.get_geometry(n.window)?.reply()?;
            let geometry = Rectangle::<i32, Client>::new(
                (geo.x as i32, geo.y as i32).into(),
                (geo.width as i32, geo.height as i32).into(),
            )
            .to_logical(xwm.client_scale.load(Ordering::Acquire) as i32);

            let surface = X11Surface::new(
                Some(xwm),
                n.window,
                n.override_redirect,
                Arc::downgrade(&conn),
                xwm.atoms,
                geometry,
            );
            surface.update_properties()?;
            xwm.windows.push(surface.clone());

            drop(_guard);
            if n.override_redirect {
                state.new_override_redirect_window(xwm_id, surface);
            } else {
                state.new_window(xwm_id, surface);
            }
        }
        Event::MapRequest(r) => {
            if let Some(surface) = xwm.windows.iter().find(|x| x.window_id() == r.window).cloned() {
                if surface.state.lock().unwrap().mapped_onto.is_none() {
                    let geo = conn.get_geometry(r.window)?.reply()?;
                    let win = r.window;
                    let frame_win = conn.generate_id()?;
                    let win_aux = CreateWindowAux::new().event_mask(
                        EventMask::SUBSTRUCTURE_NOTIFY
                            | EventMask::SUBSTRUCTURE_REDIRECT
                            | EventMask::PROPERTY_CHANGE,
                    );

                    {
                        let _guard = scopeguard::guard((), |_| {
                            let _ = conn.ungrab_server();
                        });

                        conn.grab_server()?;
                        set_no_frame_extents(&conn, &xwm.atoms, win)?;
                        let cookie1 = conn.create_window(
                            COPY_DEPTH_FROM_PARENT,
                            frame_win,
                            xwm.screen.root,
                            geo.x,
                            geo.y,
                            geo.width,
                            geo.height,
                            0,
                            WindowClass::INPUT_OUTPUT,
                            x11rb::COPY_FROM_PARENT,
                            &win_aux,
                        )?;
                        let cookie2 = conn.reparent_window(win, frame_win, 0, 0)?;
                        conn.map_window(win)?;

                        xwm.sequences_to_ignore
                            .push(Reverse(cookie1.sequence_number() as u16));
                        xwm.sequences_to_ignore
                            .push(Reverse(cookie2.sequence_number() as u16));
                    }

                    surface.state.lock().unwrap().mapped_onto = Some(frame_win);

                    surface.state.lock().unwrap().override_redirect = false;

                    let asked: Vec<u32> = conn
                        .get_property(false, win, xwm.atoms._NET_WM_STATE, AtomEnum::ATOM, 0, 32)?
                        .reply_unchecked()
                        .ok()
                        .flatten()
                        .and_then(|r| r.value32().map(|v| v.collect()))
                        .unwrap_or_default();
                    let wants = |a: Atom| asked.contains(&a);
                    let fullscreen = wants(xwm.atoms._NET_WM_STATE_FULLSCREEN);
                    let maximized = wants(xwm.atoms._NET_WM_STATE_MAXIMIZED_VERT)
                        && wants(xwm.atoms._NET_WM_STATE_MAXIMIZED_HORZ);

                    drop(_guard);
                    state.map_window_request(xwm_id, surface.clone());
                    if fullscreen {
                        state.fullscreen_request(xwm_id, surface);
                    } else if maximized {
                        state.maximize_request(xwm_id, surface);
                    }
                }
            }
        }
        Event::MapNotify(n) => {
            trace!(window = ?n.window, "mapped X11 Window");
            if let Some(surface) = xwm
                .windows
                .iter()
                .find(|x|
                      (x.window_id() == n.window && n.override_redirect) ||
                      x.mapped_window_id() == Some(n.window))
                .cloned()
            {
                surface.state.lock().unwrap().override_redirect = n.override_redirect;

                if n.override_redirect {
                    drop(_guard);
                    state.mapped_override_redirect_window(xwm_id, surface);
                } else {
                    xwm.client_list.push(surface.window_id());
                    xwm.client_list_stacking.push(surface.window_id());
                    conn.change_property32(
                        PropMode::APPEND,
                        xwm.screen.root,
                        xwm.atoms._NET_CLIENT_LIST,
                        AtomEnum::WINDOW,
                        &[surface.window_id()],
                    )?;
                    conn.change_property32(
                        PropMode::APPEND,
                        xwm.screen.root,
                        xwm.atoms._NET_CLIENT_LIST_STACKING,
                        AtomEnum::WINDOW,
                        &[surface.window_id()],
                    )?;
                    drop(_guard);
                    state.map_window_notify(xwm_id, surface);
                }
            }
        }
        Event::ConfigureRequest(r) => {
            if let Some(surface) = xwm.windows.iter().find(|x| x.window_id() == r.window).cloned() {
                drop(_guard);

                let client_scale = xwm.client_scale.load(Ordering::Acquire);

                state.configure_request(
                    xwm_id,
                    surface.clone(),
                    if u16::from(r.value_mask) & u16::from(ConfigWindow::X) != 0 {
                        Some(((r.x as f64) / client_scale).round() as i32)
                    } else {
                        None
                    },
                    if u16::from(r.value_mask) & u16::from(ConfigWindow::Y) != 0 {
                        Some(((r.y as f64) / client_scale).round() as i32)
                    } else {
                        None
                    },
                    if u16::from(r.value_mask) & u16::from(ConfigWindow::WIDTH) != 0 {
                        Some(((r.width as f64) / client_scale).round() as u32)
                    } else {
                        None
                    },
                    if u16::from(r.value_mask) & u16::from(ConfigWindow::HEIGHT) != 0 {
                        Some(((r.height as f64) / client_scale).round() as u32)
                    } else {
                        None
                    },
                    if u16::from(r.value_mask) & u16::from(ConfigWindow::STACK_MODE) != 0 {
                        match r.stack_mode {
                            StackMode::ABOVE => {
                                if u16::from(r.value_mask) & u16::from(ConfigWindow::SIBLING) != 0 {
                                    Some(Reorder::Above(r.sibling))
                                } else {
                                    Some(Reorder::Top)
                                }
                            }
                            StackMode::BELOW => {
                                if u16::from(r.value_mask) & u16::from(ConfigWindow::SIBLING) != 0 {
                                    Some(Reorder::Below(r.sibling))
                                } else {
                                    Some(Reorder::Bottom)
                                }
                            }
                            _ => None,
                        }
                    } else {
                        None
                    },
                );
                surface.configure(None).map_err(|err| match err {
                    X11SurfaceError::Connection(err) => err,
                    X11SurfaceError::UnsupportedForOverrideRedirect => unreachable!(),
                })?;
            }
        }
        Event::ConfigureNotify(n) => {
            trace!(window = ?n, "configured X11 Window");

            let client_scale = xwm.client_scale.load(Ordering::Acquire);
            let geometry = Rectangle::<i32, Client>::new(
                (n.x as i32, n.y as i32).into(),
                (n.width as i32, n.height as i32).into(),
            )
            .to_f64()
            .to_logical(client_scale)
            .to_i32_round();

            if let Some(surface) = xwm
                .windows
                .iter()
                .find(|x| x.mapped_window_id() == Some(n.window))
                .cloned()
            {
                drop(_guard);
                state.configure_notify(
                    xwm_id,
                    surface,
                    geometry,
                    if n.above_sibling == x11rb::NONE {
                        None
                    } else {
                        Some(n.above_sibling)
                    },
                );
            } else if let Some(surface) = xwm.windows.iter().find(|x| x.window_id() == n.window).cloned() {
                if surface.is_override_redirect() {
                    surface.state.lock().unwrap().geometry = geometry;
                    drop(_guard);
                    state.configure_notify(
                        xwm_id,
                        surface,
                        geometry,
                        if n.above_sibling == x11rb::NONE {
                            None
                        } else {
                            Some(n.above_sibling)
                        },
                    );
                }
            }
        }
        Event::UnmapNotify(n) => {
            if let Some(surface) = xwm.windows.iter().find(|x| x.window_id() == n.window).cloned() {
                xwm.client_list.retain(|w| *w != surface.window_id());
                xwm.client_list_stacking.retain(|w| *w != surface.window_id());
                {
                    let _guard = scopeguard::guard((), |_| {
                        let _ = conn.ungrab_server();
                    });
                    conn.grab_server()?;
                    conn.change_property32(
                        PropMode::REPLACE,
                        xwm.screen.root,
                        xwm.atoms._NET_CLIENT_LIST,
                        AtomEnum::WINDOW,
                        &xwm.client_list,
                    )?;
                    conn.change_property32(
                        PropMode::REPLACE,
                        xwm.screen.root,
                        xwm.atoms._NET_CLIENT_LIST_STACKING,
                        AtomEnum::WINDOW,
                        &xwm.client_list_stacking,
                    )?;
                    {
                        let mut state = surface.state.lock().unwrap();
                        conn.reparent_window(
                            n.window,
                            xwm.screen.root,
                            state.geometry.loc.x as i16,
                            state.geometry.loc.y as i16,
                        )?;
                        if let Some(frame) = state.mapped_onto.take() {
                            conn.destroy_window(frame)?;
                        }
                    }
                }
                drop(_guard);
                state.unmapped_window(xwm_id, surface.clone());
                {
                    let mut state = surface.state.lock().unwrap();
                    state.wl_surface = None;
                }
            }
        }
        Event::DestroyNotify(n) => {
            if let Some(selection) = if xwm.clipboard.incoming.iter().any(|t| t.window == n.window)
                || xwm
                    .clipboard
                    .outgoing
                    .iter()
                    .any(|t| t.request.requestor == n.window)
            {
                Some(&mut xwm.clipboard)
            } else if xwm.primary.incoming.iter().any(|t| t.window == n.window)
                || xwm
                    .primary
                    .outgoing
                    .iter()
                    .any(|t| t.request.requestor == n.window)
            {
                Some(&mut xwm.primary)
            } else {
                None
            } {
                let mut i = 0;
                while i < selection.incoming.len() {
                    if selection.incoming[i].window == n.window {
                        selection.incoming.remove(i).destroy(loop_handle);
                    } else {
                        i += 1;
                    }
                }
                let mut i = 0;
                while i < selection.outgoing.len() {
                    if selection.outgoing[i].request.requestor == n.window {
                        selection.outgoing.remove(i).destroy(loop_handle);
                    } else {
                        i += 1;
                    }
                }
            }

            if let Some(pos) = xwm.windows.iter().position(|x| x.window_id() == n.window) {
                let surface = xwm.windows.remove(pos);
                surface.state.lock().unwrap().alive = false;
                drop(_guard);
                state.destroyed_window(xwm_id, surface);
            }
        }
        Event::XfixesSelectionNotify(n) => {
            if n.selection == xwm.atoms.XdndSelection {
                xwm.xdnd.owner = n.owner;
                if n.owner == xwm.xdnd.window {
                    xwm.xdnd.timestamp = n.timestamp;
                    return Ok(());
                }
                drop(_guard);
                let event = if n.owner == x11rb::NONE {
                    XdndEvent::SourceGone
                } else {
                    XdndEvent::SourceTook(n.owner)
                };
                state.xdnd_event(xwm_id, event);
                return Ok(());
            }
            let selection = match n.selection {
                x if x == xwm.atoms.CLIPBOARD => &mut xwm.clipboard,
                x if x == xwm.atoms.PRIMARY => &mut xwm.primary,
                _ => return Ok(()),
            };

            selection.owner = n.owner;
            if selection.owner == selection.window {
                selection.timestamp = n.timestamp;
                return Ok(());
            }

            if n.owner == x11rb::NONE && selection.owner != selection.window {
                let selection = match selection.slot {
                    Slot::Primary => SelectionTarget::Primary,
                    _ => SelectionTarget::Clipboard,
                };
                drop(_guard);
                state.cleared_selection(xwm_id, selection);
                return Ok(());
            }

            conn.convert_selection(
                selection.window,
                selection.atom,
                xwm.atoms.TARGETS,
                xwm.atoms._WL_SELECTION,
                n.timestamp,
            )?;
        }
        Event::SelectionNotify(n) => {
            let selection = match n.selection {
                x if x == xwm.atoms.CLIPBOARD => &mut xwm.clipboard,
                x if x == xwm.atoms.PRIMARY => &mut xwm.primary,
                x if x == xwm.atoms.XdndSelection => &mut xwm.xdnd,
                _ => return Ok(()),
            };

            match n.target {
                x if x == xwm.atoms.TARGETS && selection.slot == Slot::Xdnd => {}
                x if x == xwm.atoms.TARGETS => {
                    if let Some(prop) = conn
                        .get_property(
                            true,
                            selection.window,
                            xwm.atoms._WL_SELECTION,
                            AtomEnum::ANY,
                            0,
                            4096,
                        )?
                        .reply_unchecked()?
                    {
                        if prop.type_ == AtomEnum::ATOM.into() {
                            if let Some(values) = prop.value32() {
                                let mime_types = values
                                    .filter_map(|val| {
                                        match val {
                                            val if val == xwm.atoms.UTF8_STRING => {
                                                Some(Ok(String::from("text/plain;charset=utf-8")))
                                            }
                                            val if val == xwm.atoms.TEXT => {
                                                Some(Ok(String::from("text/plain")))
                                            }
                                            val if val == xwm.atoms.TARGETS || val == xwm.atoms.TIMESTAMP => {
                                                None
                                            }
                                            val => {
                                                let cookie = match conn.get_atom_name(val) {
                                                    Ok(cookie) => cookie,
                                                    Err(err) => return Some(Err(err)),
                                                };
                                                let reply = match cookie.reply_unchecked() {
                                                    Ok(reply) => reply,
                                                    Err(err) => return Some(Err(err)),
                                                };
                                                if let Some(reply) = reply {
                                                    if let Ok(name) = std::str::from_utf8(&reply.name) {
                                                        if name.contains('/') {
                                                            Some(Ok(String::from(name)))
                                                        } else {
                                                            None
                                                        }
                                                    } else {
                                                        None
                                                    }
                                                } else {
                                                    None
                                                }
                                            }
                                        }
                                    })
                                    .collect::<Result<Vec<String>, _>>()?;

                                let selection = match selection.slot {
                                    Slot::Primary => SelectionTarget::Primary,
                                    _ => SelectionTarget::Clipboard,
                                };
                                drop(_guard);
                                state.new_selection(xwm_id, selection, mime_types);
                            }
                        }
                    }
                }
                _ if n.property == AtomEnum::NONE.into() => {
                    if let Some(pos) = selection.incoming.iter().position(|t| t.window == n.requestor) {
                        selection.incoming.remove(pos).destroy(loop_handle);
                    }
                }
                x if x == AtomEnum::NONE.into() => {
                    if let Some(pos) = selection.incoming.iter().position(|t| t.window == n.requestor) {
                        selection.incoming.remove(pos).destroy(loop_handle);
                    }
                }
                _ => {
                    let Some(transfer) = selection.incoming.iter_mut().find(|t| t.window == n.requestor)
                    else {
                        return Ok(());
                    };

                    if let Some(prop) = conn
                        .get_property(
                            true,
                            transfer.window,
                            xwm.atoms._WL_SELECTION,
                            AtomEnum::ANY,
                            0,
                            0x1fffffff,
                        )?
                        .reply_unchecked()?
                    {
                        let type_ = prop.type_;
                        transfer.read_selection_prop(prop);
                        if type_ == xwm.atoms.INCR {
                            transfer.incr = true;
                            return Ok(());
                        } else if let Some(token) = transfer.token.as_ref() {
                            let _ = loop_handle.enable(token);
                        } else if let Some(pos) =
                            selection.incoming.iter().position(|t| t.window == n.requestor)
                        {
                            selection.incoming.remove(pos);
                        }
                    }
                }
            }
        }
        Event::SelectionRequest(n) => {
            let slot = match xwm.slot_of_atom(n.selection) {
                Some(slot) => slot,
                None => {
                    warn!(
                        target = ?n.selection,
                        "Got SelectionRequest for unknown Target",
                    );
                    send_selection_notify_resp(&conn, &n, false)?;
                    return Ok(());
                }
            };

            drop(_guard);
            let allow_access = match slot {
                Slot::Clipboard => state.allow_selection_access(xwm_id, SelectionTarget::Clipboard),
                Slot::Primary => state.allow_selection_access(xwm_id, SelectionTarget::Primary),
                Slot::Xdnd => {
                    let xwm = state.xwm_state(xwm_id);
                    let atoms = xwm.atoms;
                    xwm.dnd.may_ask(&conn, n.requestor, n.target, &atoms)
                }
            };
            let xwm = state.xwm_state(xwm_id);
            let selection = sel!(xwm, slot);

            let _guard = xwm.span.enter();
            if n.requestor == selection.window {
                warn!("Got SelectionRequest from our own selection window.");
                send_selection_notify_resp(&conn, &n, false)?;
                return Ok(());
            }

            if selection.window != n.owner {
                if n.time != x11rb::CURRENT_TIME && n.time < selection.timestamp {
                    warn!(
                        got = n.time,
                        expected = selection.timestamp,
                        "Ignoring request with too old timestamp",
                    );
                    send_selection_notify_resp(&conn, &n, false)?;
                }

                return Ok(());
            }

            if allow_access {
                match n.target {
                    x if x == xwm.atoms.TARGETS => {
                        if selection.mime_types.is_empty() {
                            send_selection_notify_resp(&conn, &n, false)?;
                            return Ok(());
                        }

                        let targets = [xwm.atoms.TARGETS, xwm.atoms.TIMESTAMP]
                            .iter()
                            .copied()
                            .chain(selection.mime_types.iter().filter_map(|mime| {
                                Some(match &**mime {
                                    "text/plain" => xwm.atoms.TEXT,
                                    "text/plain;charset=utf-8" => xwm.atoms.UTF8_STRING,
                                    mime => {
                                        conn.intern_atom(false, mime.as_bytes())
                                            .ok()?
                                            .reply_unchecked()
                                            .ok()??
                                            .atom
                                    }
                                })
                            }))
                            .collect::<Vec<u32>>();
                        let mut targets = targets;
                        if offered_mime("STRING", &selection.mime_types).is_some() {
                            targets.extend([xwm.atoms.UTF8_STRING, AtomEnum::STRING.into()]);
                        }
                        let mut seen = std::collections::HashSet::new();
                        targets.retain(|t| seen.insert(*t));
                        trace!(requstor = n.requestor, ?targets, "Sending TARGETS");
                        conn.change_property32(
                            PropMode::REPLACE,
                            n.requestor,
                            n.property,
                            AtomEnum::ATOM,
                            &targets,
                        )?;
                        send_selection_notify_resp(&conn, &n, true)?;
                    }
                    x if x == xwm.atoms.TIMESTAMP => {
                        trace!(
                            requestor = n.requestor,
                            timestamp = selection.timestamp,
                            "Sending TIMESTAMP",
                        );
                        conn.change_property32(
                            PropMode::REPLACE,
                            n.requestor,
                            n.property,
                            AtomEnum::INTEGER,
                            &[selection.timestamp],
                        )?;
                        send_selection_notify_resp(&conn, &n, true)?;
                    }
                    x if x == xwm.atoms.DELETE => {
                        send_selection_notify_resp(&conn, &n, true)?;
                    }
                    target => {
                        let name = match target {
                            x if x == xwm.atoms.TEXT => Some("TEXT".to_string()),
                            x if x == xwm.atoms.UTF8_STRING => Some("UTF8_STRING".to_string()),
                            x => conn
                                .get_atom_name(x)?
                                .reply_unchecked()?
                                .and_then(|reply| String::from_utf8(reply.name).ok()),
                        };
                        let Some(name) = name else {
                            debug!("Unable to determine mime type from atom: {}", target);
                            send_selection_notify_resp(&conn, &n, false)?;
                            return Ok(());
                        };
                        let Some(mime_type) = offered_mime(&name, &selection.mime_types) else {
                            warn!(mime = name, "Mime type requested by X client not offered",);
                            send_selection_notify_resp(&conn, &n, false)?;
                            return Ok(());
                        };

                        let (recv_fd, send_fd) = rustix::pipe::pipe_with(
                            rustix::pipe::PipeFlags::CLOEXEC | rustix::pipe::PipeFlags::NONBLOCK,
                        )
                        .map_err(|err| ConnectionError::IoError(std::io::Error::from(err)))?;
                        let _ = rustix::fs::fcntl_setfl(&send_fd, OFlags::empty());

                        let mut i = 0;
                        while i < selection.outgoing.len() {
                            let transfer = &mut selection.outgoing[i];
                            if transfer.request.requestor == n.requestor {
                                debug!(
                                    requestor = transfer.request.requestor,
                                    "Destroying stale transfer",
                                );
                                if !transfer.answered {
                                    send_selection_notify_resp(&transfer.conn, &transfer.request, false)?;
                                }
                                selection.outgoing.remove(i).destroy(loop_handle);
                            } else {
                                i += 1;
                            }
                        }

                        let requestor = n.requestor;

                        let token = loop_handle.insert_source(
                            Generic::new(recv_fd, Interest::READ, Mode::Level),
                            move |_, fd, data| {
                                let xwm = data.xwm_state(xwm_id);
                                let selection = sel!(xwm, slot);

                                if let Some(transfer) = selection
                                    .outgoing
                                    .iter_mut()
                                    .find(|t| t.request.requestor == requestor)
                                {
                                    let (conn, atoms) = (xwm.conn.clone(), xwm.atoms);
                                    match read_selection_callback(&conn, &atoms, fd.as_fd(), transfer)
                                    {
                                        Ok(OutgoingAction::WaitForReadable) => {
                                            return Ok(PostAction::Continue);
                                        }
                                        Ok(_) => {}
                                        Err(err) => {
                                            warn!(?err, "Transfer aborted");
                                        }
                                    };
                                    let _ = transfer.token.take();
                                }

                                Ok(PostAction::Remove)
                            },
                        );

                        let token = match token {
                            Ok(token) => token,
                            Err(err) => {
                                warn!(
                                    err = ?err.error,
                                    "Failed to initialize event loop source for clipboard transfer",

                                );
                                send_selection_notify_resp(&conn, &n, false)?;
                                return Ok(());
                            }
                        };

                        debug!(
                            selection = ?selection.slot,
                            requestor = n.requestor,
                            ?mime_type,
                            "Created outgoing transfer",
                        );
                        let transfer = OutgoingTransfer {
                            conn: conn.clone(),
                            incr: false,
                            token: Some(token),
                            source_data: Vec::new(),
                            request: n,
                            property_set: false,
                            flush_property_on_delete: false,
                            sent_finished: false,
                            answered: false,
                        };
                        selection.outgoing.push(transfer);

                        drop(_guard);
                        match slot {
                            Slot::Clipboard => state.send_selection(xwm_id, SelectionTarget::Clipboard, mime_type, send_fd),
                            Slot::Primary => state.send_selection(xwm_id, SelectionTarget::Primary, mime_type, send_fd),
                            Slot::Xdnd => state.send_xdnd(xwm_id, mime_type, send_fd),
                        }
                    }
                }
            } else {
                send_selection_notify_resp(&conn, &n, false)?;
            }
        }
        Event::PropertyNotify(n) => {
            if n.state == Property::NEW_VALUE && n.atom == xwm.atoms._WL_SELECTION {
                let found = xwm.slot_incoming_on(n.window);
                if let Some(selection) = found.map(|slot| sel!(xwm, slot)) {
                    let transfer = selection
                        .incoming
                        .iter_mut()
                        .find(|t| t.window == n.window)
                        .unwrap();
                    if transfer.incr {
                        if let Some(prop) = conn
                            .get_property(
                                true,
                                transfer.window,
                                xwm.atoms._WL_SELECTION,
                                AtomEnum::ANY,
                                0,
                                0x1fffffff,
                            )?
                            .reply_unchecked()?
                        {
                            if prop.value_len == 0 {
                                debug!(?transfer, "Incr Transfer complete!");
                                if transfer.source_data.is_empty() {
                                    if let Some(pos) =
                                        selection.incoming.iter().position(|t| t.window == n.window)
                                    {
                                        selection.incoming.remove(pos).destroy(loop_handle);
                                    }
                                } else {
                                    transfer.incr_done = true;
                                }
                            } else {
                                transfer.read_selection_prop(prop);
                                if let Some(token) = transfer.token.as_ref() {
                                    let _ = loop_handle.enable(token);
                                } else if let Some(pos) =
                                    selection.incoming.iter().position(|t| t.window == n.window)
                                {
                                    selection.incoming.remove(pos);
                                }
                            }
                        }
                    }
                }
            }

            if n.state == Property::DELETE {
                let found = [Slot::Clipboard, Slot::Primary, Slot::Xdnd].into_iter().find(|slot| {
                    sel!(xwm, *slot)
                        .outgoing
                        .iter()
                        .any(|t| t.incr && t.request.requestor == n.window && t.request.property == n.atom)
                });
                if let Some(selection) = found.map(|slot| sel!(xwm, slot)) {
                    let transfer = selection
                        .outgoing
                        .iter_mut()
                        .find(|t| t.incr && t.request.requestor == n.window && t.request.property == n.atom)
                        .unwrap();

                    transfer.property_set = false;
                    if transfer.flush_property_on_delete {
                        transfer.flush_property_on_delete = false;
                        let len = transfer.flush_data()?;
                        let requestor = transfer.request.requestor;
                        trace!(requestor, len, "Send data chunk");

                        if transfer.token.is_none() {
                            if len > 0 || !transfer.sent_finished {
                                transfer.flush_property_on_delete = true;
                            } else if let Some(pos) = selection
                                .outgoing
                                .iter()
                                .position(|t| t.request.requestor == requestor)
                            {
                                selection.outgoing.remove(pos);
                            }
                        }
                    }
                }
            }

            if let Some(surface) = xwm.windows.iter().find(|x| x.window_id() == n.window).cloned() {
                if let Some(property) = surface.update_property(n.atom)? {
                    drop(_guard);
                    state.property_notify(xwm_id, surface, property);
                }
            }
        }
        Event::FocusIn(n) => {
            conn.change_property32(
                PropMode::REPLACE,
                xwm.screen.root,
                xwm.atoms._NET_ACTIVE_WINDOW,
                AtomEnum::WINDOW,
                &[n.event],
            )?;
        }
        Event::FocusOut(n) => {
            conn.change_property32(
                PropMode::REPLACE,
                xwm.screen.root,
                xwm.atoms._NET_ACTIVE_WINDOW,
                AtomEnum::WINDOW,
                &[n.event],
            )?;
        }
        Event::ClientMessage(msg) => {
            if let Some(reply) = conn.get_atom_name(msg.type_)?.reply_unchecked()? {
                debug!(
                    event = std::str::from_utf8(&reply.name).unwrap(),
                    message = ?msg,
                    "got X11 client event message",
                );
            }
            match msg.type_ {
                x if x == xwm.atoms._NET_REQUEST_FRAME_EXTENTS => {
                    set_no_frame_extents(&conn, &xwm.atoms, msg.window)?;
                    conn.flush()?;
                }
                x if x == xwm.atoms.WL_SURFACE_ID => {
                    let wid = msg.data.as_data32()[0];
                    info!(
                        window = ?msg.window,
                        surface = ?wid,
                        "mapped X11 window to surface",
                    );
                    if let Some(surface) = xwm
                        .windows
                        .iter_mut()
                        .find(|x| x.window_id() == msg.window || x.mapped_window_id() == Some(msg.window))
                    {
                        surface.state.lock().unwrap().wl_surface_id = Some(wid);
                    }
                }
                x if x == xwm.atoms.WL_SURFACE_SERIAL => {
                    if let Some(xsurface) = xwm
                        .windows
                        .iter_mut()
                        .find(|x| x.window_id() == msg.window || x.mapped_window_id() == Some(msg.window))
                        .cloned()
                    {
                        drop(_guard);

                        let serial_lo = msg.data.as_data32()[0];
                        let serial_hi = msg.data.as_data32()[1];
                        let serial = u64::from(serial_lo) | (u64::from(serial_hi) << 32);

                        let surface_state = xsurface.state.clone();
                        let mut guard = surface_state.lock().unwrap();
                        guard.wl_surface_serial = Some(serial);

                        if let Some(wl_surface) =
                            xwayland_shell::XWaylandShellHandler::xwayland_shell_state(state)
                                .surface_for_serial(serial)
                                .clone()
                        {
                            debug!(
                                window = ?xsurface.window_id(),
                                wl_surface = ?wl_surface.id().protocol_id(),
                                "associated X11 window to wl_surface",
                            );

                            guard.wl_surface = Some(wl_surface.clone());
                            std::mem::drop(guard);
                            XWaylandShellHandler::surface_associated(state, xwm_id, wl_surface, xsurface);
                        } else {
                            debug!(
                                window = ?msg.window,
                                serial = serial,
                                "no matching wl_surface for X11 window",
                            );
                            let xwm = state.xwm_state(xwm_id);
                            xwm.unpaired_surfaces.insert(serial, xsurface.window_id());
                            guard.wl_surface = None;
                        }
                    }
                }
                x if x == xwm.atoms.WM_CHANGE_STATE => {
                    let data = msg.data.as_data32();
                    if let Some(surface) = xwm.windows.iter().find(|x| x.window_id() == msg.window).cloned() {
                        drop(_guard);
                        match data[0] {
                            1 => state.unminimize_request(xwm_id, surface),
                            3 => state.minimize_request(xwm_id, surface),
                            _ => {}
                        }
                    }
                }
                x if x == xwm.atoms._NET_WM_STATE => {
                    let data = msg.data.as_data32();
                    debug!(
                        "X11: Got _NET_WM_STATE change request to ({:?}): {:?} / {:?}",
                        match &data[0] {
                            0 => "REMOVE",
                            1 => "SET",
                            2 => "TOGGLE",
                            _ => "Unknown",
                        },
                        conn.get_atom_name(data[1])?
                            .reply_unchecked()?
                            .map(|reply| String::from_utf8(reply.name)),
                        conn.get_atom_name(data[2])?
                            .reply_unchecked()?
                            .map(|reply| String::from_utf8(reply.name)),
                    );
                    if let Some(surface) = xwm.windows.iter().find(|x| x.window_id() == msg.window).cloned() {
                        drop(_guard);
                        match &data[1..=2] {
                            &[x, y]
                                if (x == xwm.atoms._NET_WM_STATE_MAXIMIZED_HORZ
                                    && y == xwm.atoms._NET_WM_STATE_MAXIMIZED_VERT)
                                    || (x == xwm.atoms._NET_WM_STATE_MAXIMIZED_VERT
                                        && y == xwm.atoms._NET_WM_STATE_MAXIMIZED_HORZ) =>
                            {
                                match data[0] {
                                    0 => {
                                        if surface.is_maximized() {
                                            state.unmaximize_request(xwm_id, surface)
                                        }
                                    }
                                    1 => {
                                        if !surface.is_maximized() {
                                            state.maximize_request(xwm_id, surface)
                                        }
                                    }
                                    2 => {
                                        if surface.is_maximized() {
                                            state.unmaximize_request(xwm_id, surface)
                                        } else {
                                            state.maximize_request(xwm_id, surface)
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            actions if actions.contains(&xwm.atoms._NET_WM_STATE_FULLSCREEN) => {
                                match data[0] {
                                    0 => state.unfullscreen_request(xwm_id, surface),
                                    1 => state.fullscreen_request(xwm_id, surface),
                                    2 => {
                                        if surface.is_fullscreen() {
                                            state.unfullscreen_request(xwm_id, surface)
                                        } else {
                                            state.fullscreen_request(xwm_id, surface)
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
                x if xwm.dnd.is_dnd_message(&xwm.atoms, x) => {
                    drop(_guard);
                    let events = state.xwm_state(xwm_id).handle_dnd_message(&msg);
                    for event in events {
                        state.xdnd_event(xwm_id, event);
                    }
                }
                x if x == xwm.atoms._NET_WM_MOVERESIZE => {
                    if let Some(surface) = xwm.windows.iter().find(|x| x.window_id() == msg.window).cloned() {
                        drop(_guard);
                        let data = msg.data.as_data32();
                        match data[2] {
                            x @ 0..=7 => {
                                let resize_edge = match x {
                                    0 => ResizeEdge::TopLeft,
                                    1 => ResizeEdge::Top,
                                    2 => ResizeEdge::TopRight,
                                    3 => ResizeEdge::Right,
                                    4 => ResizeEdge::BottomRight,
                                    5 => ResizeEdge::Bottom,
                                    6 => ResizeEdge::BottomLeft,
                                    7 => ResizeEdge::Left,
                                    _ => unreachable!(),
                                };
                                state.resize_request(xwm_id, surface, data[3], resize_edge);
                            }
                            8 => state.move_request(xwm_id, surface, data[3]),
                            _ => {}
                        }
                    }
                }
                x => {
                    debug!(
                        "Unhandled client msg of type {:?}",
                        String::from_utf8(conn.get_atom_name(x)?.reply_unchecked()?.unwrap().name).ok()
                    )
                }
            }
        }
        Event::RandrNotify(n) => {
            if n.sub_code == Notify::OUTPUT_CHANGE {
                let current_primary = xwm
                    .conn
                    .randr_get_output_primary(xwm.screen.root)?
                    .reply()?
                    .output;
                if current_primary != xwm.randr_primary {
                    xwm.randr_primary = current_primary;
                    let primary_name = xwm.get_randr_primary_output().unwrap();
                    drop(_guard);

                    state.randr_primary_output_change(xwm_id, primary_name);
                }
            }
        }
        Event::Error(err) => {
            info!(?err, "Got X11 Error");
        }
        _ => {}
    }
    conn.flush()?;
    Ok(())
}

enum OutgoingAction {
    Done,
    DoneReading,
    WaitForReadable,
}

fn read_selection_callback(
    conn: &RustConnection,
    atoms: &Atoms,
    fd: BorrowedFd<'_>,
    transfer: &mut OutgoingTransfer,
) -> Result<OutgoingAction, ReplyOrIdError> {
    let mut buf = [0; INCR_CHUNK_SIZE];
    let Ok(len) = rustix::io::read(fd, &mut buf) else {
        debug!(
            requestor = transfer.request.requestor,
            "File descriptor closed, aborting transfer."
        );
        transfer.answered = true;
        send_selection_notify_resp(conn, &transfer.request, false)?;
        return Ok(OutgoingAction::Done);
    };
    trace!(
        requestor = transfer.request.requestor,
        "Transfer became readable, read {} bytes",
        len
    );

    if transfer.source_data.len() + len > MAX_TRANSFER_BYTES {
        transfer.answered = true;
        send_selection_notify_resp(conn, &transfer.request, false)?;
        return Ok(OutgoingAction::Done);
    }
    transfer.source_data.extend_from_slice(&buf[..len]);
    if transfer.source_data.len() >= INCR_CHUNK_SIZE {
        if !transfer.incr {
            trace!(
                requestor = transfer.request.requestor,
                "Transfer became incremental",
            );
            conn.change_property32(
                PropMode::REPLACE,
                transfer.request.requestor,
                transfer.request.property,
                atoms.INCR,
                &[INCR_CHUNK_SIZE as u32],
            )?;
            conn.flush()?;
            transfer.incr = true;
            transfer.property_set = true;
            transfer.flush_property_on_delete = true;
            transfer.answered = true;
            send_selection_notify_resp(conn, &transfer.request, true)?;
        } else if transfer.property_set {
            transfer.flush_property_on_delete = true;
        } else {
            let len = transfer.flush_data()?;
            trace!(
                requestor = transfer.request.requestor,
                "Send data chunk: {} bytes",
                len
            );
        }
    }

    if len == 0 {
        if transfer.incr {
            debug!("Incr transfer completed");
            if !transfer.property_set {
                let len = transfer.flush_data()?;
                trace!(
                    requestor = transfer.request.requestor,
                    "Send data chunk: {} bytes",
                    len
                );
            }
            transfer.flush_property_on_delete = true;
            Ok(OutgoingAction::DoneReading)
        } else {
            let len = transfer.flush_data()?;
            debug!("Non-Incr transfer completed with {} bytes", len);
            transfer.answered = true;
            send_selection_notify_resp(conn, &transfer.request, true)?;
            Ok(OutgoingAction::Done)
        }
    } else {
        Ok(OutgoingAction::WaitForReadable)
    }
}

enum IncomingAction {
    Done,
    WaitForProperty,
    WaitForWritable,
}

fn write_selection_callback(
    fd: BorrowedFd<'_>,
    conn: &RustConnection,
    atoms: &Atoms,
    transfer: &mut IncomingTransfer,
) -> Result<IncomingAction, ReplyOrIdError> {
    match transfer.write_selection(fd) {
        Ok(true) => {
            if transfer.incr {
                conn.delete_property(transfer.window, atoms._WL_SELECTION)?;
                Ok(IncomingAction::WaitForProperty)
            } else {
                debug!(?transfer, "Non-Incr Transfer complete!");
                Ok(IncomingAction::Done)
            }
        }
        Ok(false) => Ok(IncomingAction::WaitForWritable),
        Err(err) => {
            warn!(?err, "Transfer errored");
            if transfer.incr {
                conn.delete_property(transfer.window, atoms._WL_SELECTION)?;
            }
            Ok(IncomingAction::Done)
        }
    }
}

fn send_selection_notify_resp(
    conn: &RustConnection,
    req: &SelectionRequestEvent,
    success: bool,
) -> Result<(), ReplyOrIdError> {
    conn.send_event(
        false,
        req.requestor,
        EventMask::NO_EVENT,
        SelectionNotifyEvent {
            response_type: SELECTION_NOTIFY_EVENT,
            sequence: 0,
            time: req.time,
            requestor: req.requestor,
            selection: req.selection,
            target: req.target,
            property: if success {
                req.property
            } else {
                AtomEnum::NONE.into()
            },
        },
    )?;
    conn.flush()?;
    Ok(())
}

fn send_configure_notify(
    conn: &RustConnection,
    win: &X11Window,
    geometry: Rectangle<i32, Client>,
    override_redirect: bool,
) -> Result<(), ConnectionError> {
    conn.send_event(
        false,
        *win,
        EventMask::STRUCTURE_NOTIFY,
        ConfigureNotifyEvent {
            response_type: CONFIGURE_NOTIFY_EVENT,
            sequence: 0,
            event: *win,
            window: *win,
            above_sibling: x11rb::NONE,
            x: geometry.loc.x as i16,
            y: geometry.loc.y as i16,
            width: geometry.size.w as u16,
            height: geometry.size.h as u16,
            border_width: 0,
            override_redirect,
        },
    )?;
    conn.flush()?;
    Ok(())
}

fn offered_mime(name: &str, offered: &[String]) -> Option<String> {
    let has = |m: &str| offered.iter().any(|o| o == m);
    let usual = match name {
        "TEXT" => "text/plain",
        "UTF8_STRING" => "text/plain;charset=utf-8",
        other => other,
    };
    if has(usual) {
        return Some(usual.to_string());
    }
    if has(name) {
        return Some(name.to_string());
    }
    if !matches!(name, "TEXT" | "UTF8_STRING" | "STRING" | "COMPOUND_TEXT") {
        return None;
    }
    [
        "text/plain;charset=utf-8",
        "UTF8_STRING",
        "text/plain;charset=UTF-8",
        "text/plain",
        "STRING",
        "TEXT",
    ]
    .into_iter()
    .find(|m| has(m))
    .map(str::to_string)
}

fn set_no_frame_extents(conn: &RustConnection, atoms: &Atoms, win: X11Window) -> Result<(), ConnectionError> {
    conn.change_property32(
        PropMode::REPLACE,
        win,
        atoms._NET_FRAME_EXTENTS,
        AtomEnum::CARDINAL,
        &[0, 0, 0, 0],
    )?;
    Ok(())
}
