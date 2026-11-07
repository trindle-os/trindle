use crate::utils::{Logical, Size};

use super::{extension::Extensions, Atoms, Window, X11Error};
use drm_fourcc::DrmFourcc;
use std::sync::{
    atomic::{AtomicU32, AtomicU64},
    mpsc::Sender,
    Arc, Mutex, Weak,
};
use x11rb::{
    connection::Connection,
    protocol::{
        present::{self, ConnectionExt as _},
        xfixes::ConnectionExt as _,
        xinput::{self, ConnectionExt as _},
        xproto::{
            self as x11, AtomEnum, ConnectionExt, CreateWindowAux, Depth, EventMask, PropMode, Screen,
            UnmapNotifyEvent, WindowClass,
        },
    },
    rust_connection::RustConnection,
    wrapper::ConnectionExt as _,
};

impl From<Arc<WindowInner>> for Window {
    #[inline]
    fn from(inner: Arc<WindowInner>) -> Self {
        Window(inner)
    }
}

#[derive(Debug)]
pub struct CursorState {
    pub inside_window: bool,
    pub visible: bool,
}

impl Default for CursorState {
    #[inline]
    fn default() -> Self {
        CursorState {
            inside_window: false,
            visible: true,
        }
    }
}

#[derive(Debug)]
pub(crate) struct WindowInner {
    pub connection: Weak<RustConnection>,
    pub id: x11::Window,
    root: x11::Window,
    pub atoms: Atoms,
    pub cursor_state: Arc<Mutex<CursorState>>,
    pub size: Mutex<Size<u16, Logical>>,
    pub resize: Mutex<Option<Sender<Size<u16, Logical>>>>,
    pub next_serial: AtomicU32,
    pub last_msc: Arc<AtomicU64>,
    pub format: DrmFourcc,
    pub depth: Depth,
    pub extensions: Extensions,
}

impl WindowInner {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        connection: Weak<RustConnection>,
        screen: &Screen,
        size: Size<u16, Logical>,
        title: &str,
        format: DrmFourcc,
        atoms: Atoms,
        depth: Depth,
        visual_id: u32,
        colormap: u32,
        extensions: Extensions,
    ) -> Result<WindowInner, X11Error> {
        let weak = connection;
        let connection = weak.upgrade().unwrap();

        let window = connection.generate_id()?;

        let window_aux = CreateWindowAux::new()
            .event_mask(
                EventMask::EXPOSURE
            | EventMask::STRUCTURE_NOTIFY
            | EventMask::EXPOSURE
            | EventMask::NO_EVENT,
            )
            .border_pixel(screen.black_pixel)
            .colormap(colormap);

        let _ = connection.create_window(
            depth.depth,
            window,
            screen.root,
            0,
            0,
            size.w,
            size.h,
            0,
            WindowClass::INPUT_OUTPUT,
            visual_id,
            &window_aux,
        )?;

        connection.xinput_xi_select_events(
            window,
            &[xinput::EventMask {
                deviceid: 1,
                mask: vec![
                    xinput::XIEventMask::KEY_PRESS
                        | xinput::XIEventMask::KEY_RELEASE
                        | xinput::XIEventMask::BUTTON_PRESS
                        | xinput::XIEventMask::BUTTON_RELEASE
                        | xinput::XIEventMask::MOTION
                        | xinput::XIEventMask::ENTER
                        | xinput::XIEventMask::LEAVE
                        | xinput::XIEventMask::FOCUS_IN
                        | xinput::XIEventMask::FOCUS_OUT,
                ],
            }],
        )?;

        let present_event_id = connection.generate_id()?;
        connection.present_select_input(
            present_event_id,
            window,
            present::EventMask::COMPLETE_NOTIFY | present::EventMask::IDLE_NOTIFY,
        )?;

        let window = WindowInner {
            connection: weak,
            id: window,
            root: screen.root,
            atoms,
            cursor_state: Arc::new(Mutex::new(CursorState::default())),
            size: Mutex::new(size),
            next_serial: AtomicU32::new(0),
            last_msc: Arc::new(AtomicU64::new(0)),
            format,
            depth,
            extensions,
            resize: Mutex::new(None),
        };

        connection.change_property32(
            PropMode::REPLACE,
            window.id,
            atoms.WM_PROTOCOLS,
            AtomEnum::ATOM,
            &[atoms.WM_DELETE_WINDOW],
        )?;

        let _ = connection.change_property8(
            PropMode::REPLACE,
            window.id,
            AtomEnum::WM_CLASS,
            AtomEnum::STRING,
            b"Smithay\0Wayland_Compositor\0",
        )?;

        window.set_title(title);
        window.map();

        connection.flush()?;

        Ok(window)
    }

    pub fn map(&self) {
        if let Some(connection) = self.connection.upgrade() {
            let _ = connection.map_window(self.id);
        }
    }

    pub fn unmap(&self) {
        if let Some(connection) = self.connection.upgrade() {
            let _ = connection.unmap_window(self.id);

            let _ = connection.send_event(
                false,
                self.id,
                EventMask::STRUCTURE_NOTIFY | EventMask::SUBSTRUCTURE_NOTIFY,
                UnmapNotifyEvent {
                    response_type: x11rb::protocol::xproto::UNMAP_NOTIFY_EVENT,
                    sequence: 0,
                    event: self.root,
                    window: self.id,
                    from_configure: false,
                },
            );
        }
    }

    pub fn size(&self) -> Size<u16, Logical> {
        *self.size.lock().unwrap()
    }

    pub fn set_title(&self, title: &str) {
        if let Some(connection) = self.connection.upgrade() {
            let _ = connection.change_property8(
                PropMode::REPLACE,
                self.id,
                AtomEnum::WM_NAME,
                AtomEnum::STRING,
                title.as_bytes(),
            );

            let _ = connection.change_property8(
                PropMode::REPLACE,
                self.id,
                self.atoms._NET_WM_NAME,
                self.atoms.UTF8_STRING,
                title.as_bytes(),
            );
        }
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        if let Some(connection) = self.connection.upgrade() {
            let mut state = self.cursor_state.lock().unwrap();
            let changed = state.visible != visible;

            if changed && state.inside_window {
                state.visible = visible;
                self.update_cursor(&*connection, state.visible);
            }
        }
    }

    pub fn cursor_enter(&self) {
        if let Some(connection) = self.connection.upgrade() {
            let mut state = self.cursor_state.lock().unwrap();
            state.inside_window = true;
            self.update_cursor(&*connection, state.visible);
        }
    }

    pub fn cursor_leave(&self) {
        if let Some(connection) = self.connection.upgrade() {
            let mut state = self.cursor_state.lock().unwrap();
            state.inside_window = false;
            self.update_cursor(&*connection, true);
        }
    }

    fn update_cursor<C: ConnectionExt>(&self, connection: &C, visible: bool) {
        let _ = match visible {
            true => connection
                .xfixes_show_cursor(self.id)
                .map(|cookie| cookie.ignore_error()),
            false => connection
                .xfixes_hide_cursor(self.id)
                .map(|cookie| cookie.ignore_error()),
        };
    }
}

impl PartialEq for WindowInner {
    fn eq(&self, other: &Self) -> bool {
        match (self.connection.upgrade(), other.connection.upgrade()) {
            (Some(conn1), Some(conn2)) => Arc::ptr_eq(&conn1, &conn2) && self.id == other.id,

            _ => false,
        }
    }
}

impl Drop for WindowInner {
    fn drop(&mut self) {
        if let Some(connection) = self.connection.upgrade() {
            let _ = connection.destroy_window(self.id);
        }
    }
}
