//! trindle_windows_v1 (windows.xml): the list of open windows, kept for the
//! ring and the panel, and the few things they may do to a window.

use smithay::output::Output;
use smithay::reexports::wayland_server::backend::ClientId;
use smithay::reexports::wayland_server::protocol::wl_surface::WlSurface;
use smithay::reexports::wayland_server::{
    Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource,
};

#[allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables, non_upper_case_globals, non_snake_case,
        unused_imports, missing_docs, clippy::all)]
pub mod server {
    use smithay::reexports::wayland_server;
    use smithay::reexports::wayland_server::protocol::*;

    pub mod __interfaces {
        use smithay::reexports::wayland_server::backend as wayland_backend;
        use smithay::reexports::wayland_server::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("src/compose/protocols/windows.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_server_code!("src/compose/protocols/windows.xml");
}

pub use server::trindle_window_v1::{self as window, TrindleWindowV1};
pub use server::trindle_windows_manager_v1::{self as manager, TrindleWindowsManagerV1};

const VERSION: u32 = 1;

const ACTIVE: u32 = 1;
const MINIMIZED: u32 = 2;
const MAXIMIZED: u32 = 4;
const FULLSCREEN: u32 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Change {
    Minimized,
    Maximized,
    Fullscreen,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WindowInfo {
    pub surface: WlSurface,
    pub title: String,
    pub app_id: String,
    pub active: bool,
    pub minimized: bool,
    pub maximized: bool,
    pub fullscreen: bool,
    pub outputs: Vec<Output>,
    pub parent: Option<WlSurface>,
}

impl WindowInfo {
    fn flags(&self) -> u32 {
        [(self.active, ACTIVE), (self.minimized, MINIMIZED), (self.maximized, MAXIMIZED), (self.fullscreen, FULLSCREEN)]
            .iter()
            .filter(|(on, _)| *on)
            .fold(0, |bits, (_, bit)| bits | bit)
    }
}

pub trait WindowsHandler {
    fn windows_state(&mut self) -> &mut WindowsState;

    fn raise_window(&mut self, surface: WlSurface);

    fn close_window(&mut self, surface: WlSurface);

    fn change_window(&mut self, surface: WlSurface, change: Change, on: bool);
}

#[derive(Debug)]
struct Told {
    title: String,
    app_id: String,
    flags: u32,
    parent: Option<u64>,
    outputs: Vec<Output>,
}

#[derive(Debug)]
struct Seen {
    manager: TrindleWindowsManagerV1,
    window: TrindleWindowV1,
    told: Option<Told>,
}

#[derive(Debug)]
struct Known {
    id: u64,
    info: WindowInfo,
    copies: Vec<Seen>,
}

pub struct WindowKey(u64);

pub struct WindowsGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

#[derive(Debug)]
pub struct WindowsState {
    display: DisplayHandle,
    next_id: u64,
    managers: Vec<TrindleWindowsManagerV1>,
    known: Vec<Known>,
}

impl WindowsState {

    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> WindowsState
    where
        D: GlobalDispatch<TrindleWindowsManagerV1, WindowsGlobalData>
            + Dispatch<TrindleWindowsManagerV1, ()>
            + Dispatch<TrindleWindowV1, WindowKey>
            + WindowsHandler
            + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        display.create_global::<D, TrindleWindowsManagerV1, _>(VERSION, WindowsGlobalData { filter: Box::new(filter) });
        WindowsState { display: display.clone(), next_id: 1, managers: Vec::new(), known: Vec::new() }
    }

    fn surface_of(&self, key: &WindowKey) -> Option<WlSurface> {
        self.known.iter().find(|k| k.id == key.0).map(|k| k.info.surface.clone())
    }

    fn learn(&mut self, now: Vec<WindowInfo>) {
        let (kept, closed): (Vec<Known>, Vec<Known>) =
            std::mem::take(&mut self.known).into_iter().partition(|k| now.iter().any(|i| i.surface == k.info.surface));
        for gone in closed {
            for copy in gone.copies.iter().filter(|c| c.window.is_alive()) {
                copy.window.gone();
            }
        }
        self.known = kept;
        for info in now {
            match self.known.iter_mut().find(|k| k.info.surface == info.surface) {
                Some(known) => known.info = info,
                None => {
                    self.known.push(Known { id: self.next_id, info, copies: Vec::new() });
                    self.next_id += 1;
                }
            }
        }
    }

    fn tell<D>(&mut self, manager: &TrindleWindowsManagerV1)
    where
        D: Dispatch<TrindleWindowV1, WindowKey> + 'static,
    {
        let Some(client) = manager.client() else { return };
        self.give_objects::<D>(manager, &client);

        let numbered: Vec<(u64, WlSurface, Option<TrindleWindowV1>)> = self
            .known
            .iter()
            .map(|k| (k.id, k.info.surface.clone(), k.copies.iter().find(|c| &c.manager == manager).map(|c| c.window.clone())))
            .collect();
        for known in &mut self.known {
            let parent = known
                .info
                .parent
                .as_ref()
                .and_then(|p| numbered.iter().find(|(_, s, _)| s == p))
                .and_then(|(id, _, w)| w.clone().map(|w| (*id, w)));
            let Some(copy) = known.copies.iter_mut().find(|c| &c.manager == manager) else { continue };
            if !copy.window.is_alive() {
                continue;
            }
            send_changes(copy, &known.info, parent, &client);
        }
    }

    fn give_objects<D>(&mut self, manager: &TrindleWindowsManagerV1, client: &Client)
    where
        D: Dispatch<TrindleWindowV1, WindowKey> + 'static,
    {
        for known in &mut self.known {
            if known.copies.iter().any(|c| &c.manager == manager) {
                continue;
            }
            let Ok(window) =
                client.create_resource::<TrindleWindowV1, WindowKey, D>(&self.display, manager.version(), WindowKey(known.id))
            else {
                continue;
            };
            manager.window(&window);
            known.copies.push(Seen { manager: manager.clone(), window, told: None });
        }
    }
}

fn send_changes(copy: &mut Seen, info: &WindowInfo, parent: Option<(u64, TrindleWindowV1)>, client: &Client) {
    let (flags, parent_id) = (info.flags(), parent.as_ref().map(|(id, _)| *id));
    let first = copy.told.is_none();
    let before = copy.told.take().unwrap_or(Told {
        title: String::new(),
        app_id: String::new(),
        flags: 0,
        parent: None,
        outputs: Vec::new(),
    });
    let mut changed = first;
    if first || before.title != info.title {
        copy.window.title(info.title.clone());
        changed = true;
    }
    if first || before.app_id != info.app_id {
        copy.window.app_id(info.app_id.clone());
        changed = true;
    }
    if first || before.flags != flags {
        copy.window.flags(flags);
        changed = true;
    }
    if before.parent != parent_id {
        copy.window.parent(parent.as_ref().map(|(_, w)| w));
        changed = true;
    }
    for left in before.outputs.iter().filter(|o| !info.outputs.contains(o)) {
        for screen in left.client_outputs(client) {
            copy.window.leave(&screen);
        }
        changed = true;
    }
    for entered in info.outputs.iter().filter(|o| !before.outputs.contains(o)) {
        for screen in entered.client_outputs(client) {
            copy.window.enter(&screen);
        }
        changed = true;
    }
    if changed {
        copy.window.done();
    }
    copy.told = Some(Told {
        title: info.title.clone(),
        app_id: info.app_id.clone(),
        flags,
        parent: parent_id,
        outputs: info.outputs.clone(),
    });
}

pub fn refresh<D>(state: &mut D, windows: Vec<WindowInfo>)
where
    D: WindowsHandler + Dispatch<TrindleWindowV1, WindowKey> + 'static,
{
    let st = state.windows_state();
    st.learn(windows);
    st.managers.retain(|m| m.is_alive());
    for manager in st.managers.clone() {
        st.tell::<D>(&manager);
    }
}

impl<D> GlobalDispatch<TrindleWindowsManagerV1, WindowsGlobalData, D> for WindowsState
where
    D: GlobalDispatch<TrindleWindowsManagerV1, WindowsGlobalData>
        + Dispatch<TrindleWindowsManagerV1, ()>
        + Dispatch<TrindleWindowV1, WindowKey>
        + WindowsHandler
        + 'static,
{
    fn bind(
        state: &mut D,
        _: &DisplayHandle,
        _: &Client,
        resource: New<TrindleWindowsManagerV1>,
        _: &WindowsGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        let manager = data_init.init(resource, ());
        let st = state.windows_state();
        st.managers.push(manager.clone());
        st.tell::<D>(&manager);
    }

    fn can_view(client: Client, data: &WindowsGlobalData) -> bool {
        (data.filter)(&client)
    }
}

impl<D> Dispatch<TrindleWindowsManagerV1, (), D> for WindowsState
where
    D: Dispatch<TrindleWindowsManagerV1, ()> + Dispatch<TrindleWindowV1, WindowKey> + WindowsHandler + 'static,
{
    fn request(
        _: &mut D,
        _: &Client,
        _: &TrindleWindowsManagerV1,
        request: manager::Request,
        _: &(),
        _: &DisplayHandle,
        _: &mut DataInit<'_, D>,
    ) {
        match request {
            manager::Request::Destroy => {}
        }
    }

    fn destroyed(state: &mut D, _: ClientId, manager: &TrindleWindowsManagerV1, _: &()) {
        let st = state.windows_state();
        st.managers.retain(|m| m != manager);
        for known in &mut st.known {
            known.copies.retain(|c| &c.manager != manager);
        }
    }
}

impl<D> Dispatch<TrindleWindowV1, WindowKey, D> for WindowsState
where
    D: Dispatch<TrindleWindowV1, WindowKey> + WindowsHandler + 'static,
{
    fn request(
        state: &mut D,
        _: &Client,
        _: &TrindleWindowV1,
        request: window::Request,
        key: &WindowKey,
        _: &DisplayHandle,
        _: &mut DataInit<'_, D>,
    ) {

        let Some(surface) = state.windows_state().surface_of(key) else { return };
        match request {
            window::Request::Destroy => {}
            window::Request::Raise => state.raise_window(surface),
            window::Request::Close => state.close_window(surface),
            window::Request::Change { turn_on, turn_off } => {
                for (bit, change) in [(MINIMIZED, Change::Minimized), (MAXIMIZED, Change::Maximized), (FULLSCREEN, Change::Fullscreen)] {
                    let (on, off) = (turn_on & bit != 0, turn_off & bit != 0);
                    if on != off {
                        state.change_window(surface.clone(), change, on);
                    }
                }
            }
        }
    }

    fn destroyed(state: &mut D, _: ClientId, window: &TrindleWindowV1, _: &WindowKey) {

        // again.
        let _ = (state, window);
    }
}

#[macro_export]
macro_rules! delegate_windows {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::windows::TrindleWindowsManagerV1: $crate::protocols::windows::WindowsGlobalData
        ] => $crate::protocols::windows::WindowsState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::windows::TrindleWindowsManagerV1: ()
        ] => $crate::protocols::windows::WindowsState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::windows::TrindleWindowV1: $crate::protocols::windows::WindowKey
        ] => $crate::protocols::windows::WindowsState);
    };
}
