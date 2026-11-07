//! A made-up mouse (wlr-virtual-pointer-unstable-v1), for steering this computer from somewhere else.

use std::sync::Mutex;

use smithay::backend::input::AxisSource;
use smithay::reexports::wayland_protocols_wlr::virtual_pointer::v1::server::{
    zwlr_virtual_pointer_manager_v1::{self, ZwlrVirtualPointerManagerV1},
    zwlr_virtual_pointer_v1::{self, ZwlrVirtualPointerV1},
};
use smithay::reexports::wayland_server::protocol::wl_pointer;
use smithay::reexports::wayland_server::{Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource, WEnum};

const VERSION: u32 = 2;

#[derive(Clone, Debug, PartialEq)]
pub enum PointerAction {
    Motion { time: u32, dx: f64, dy: f64 },
    MotionAbsolute { time: u32, x: f64, y: f64 },
    Button { time: u32, button: u32, pressed: bool },
    Axis { time: u32, source: AxisSource, amount: [f64; 2], v120: [Option<f64>; 2], stopped: [bool; 2] },
}

pub trait VirtualPointerHandler {
    fn virtual_pointer(&mut self, client: &Client, action: PointerAction);
}

pub struct VirtualPointerManagerState;

pub struct VirtualPointerGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

#[derive(Default)]
pub struct VirtualPointerData {
    pending: Mutex<Pending>,
}

#[derive(Default)]
struct Pending {
    time: u32,
    source: Option<AxisSource>,
    amount: [f64; 2],
    v120: [Option<f64>; 2],
    stopped: [bool; 2],
    any: bool,
}

impl VirtualPointerManagerState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> Self
    where
        D: GlobalDispatch<ZwlrVirtualPointerManagerV1, VirtualPointerGlobalData> + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        display.create_global::<D, ZwlrVirtualPointerManagerV1, _>(VERSION, VirtualPointerGlobalData { filter: Box::new(filter) });
        Self
    }
}

impl<D> GlobalDispatch<ZwlrVirtualPointerManagerV1, VirtualPointerGlobalData, D> for VirtualPointerManagerState
where
    D: GlobalDispatch<ZwlrVirtualPointerManagerV1, VirtualPointerGlobalData> + Dispatch<ZwlrVirtualPointerManagerV1, ()> + 'static,
{
    fn bind(
        _state: &mut D,
        _dh: &DisplayHandle,
        _client: &Client,
        resource: New<ZwlrVirtualPointerManagerV1>,
        _global_data: &VirtualPointerGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }

    fn can_view(client: Client, global_data: &VirtualPointerGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

impl<D> Dispatch<ZwlrVirtualPointerManagerV1, (), D> for VirtualPointerManagerState
where
    D: Dispatch<ZwlrVirtualPointerManagerV1, ()> + Dispatch<ZwlrVirtualPointerV1, VirtualPointerData> + 'static,
{
    fn request(
        _state: &mut D,
        _client: &Client,
        _manager: &ZwlrVirtualPointerManagerV1,
        request: zwlr_virtual_pointer_manager_v1::Request,
        _data: &(),
        _dh: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            zwlr_virtual_pointer_manager_v1::Request::CreateVirtualPointer { id, .. }
            | zwlr_virtual_pointer_manager_v1::Request::CreateVirtualPointerWithOutput { id, .. } => {
                data_init.init(id, VirtualPointerData::default());
            }
            _ => {}
        }
    }
}

fn axis_index(pointer: &ZwlrVirtualPointerV1, axis: WEnum<wl_pointer::Axis>) -> Option<usize> {
    match axis {
        WEnum::Value(wl_pointer::Axis::HorizontalScroll) => Some(0),
        WEnum::Value(wl_pointer::Axis::VerticalScroll) => Some(1),
        _ => {
            pointer.post_error(zwlr_virtual_pointer_v1::Error::InvalidAxis, "no such axis");
            None
        }
    }
}

fn source_of(source: wl_pointer::AxisSource) -> AxisSource {
    match source {
        wl_pointer::AxisSource::Finger => AxisSource::Finger,
        wl_pointer::AxisSource::Continuous => AxisSource::Continuous,
        wl_pointer::AxisSource::WheelTilt => AxisSource::WheelTilt,
        _ => AxisSource::Wheel,
    }
}

impl<D> Dispatch<ZwlrVirtualPointerV1, VirtualPointerData, D> for VirtualPointerManagerState
where
    D: Dispatch<ZwlrVirtualPointerV1, VirtualPointerData> + VirtualPointerHandler + 'static,
{
    fn request(
        state: &mut D,
        client: &Client,
        pointer: &ZwlrVirtualPointerV1,
        request: zwlr_virtual_pointer_v1::Request,
        data: &VirtualPointerData,
        _dh: &DisplayHandle,
        _data_init: &mut DataInit<'_, D>,
    ) {
        use zwlr_virtual_pointer_v1::Request;
        let mut pending = data.pending.lock().unwrap_or_else(|e| e.into_inner());
        match request {
            Request::Motion { time, dx, dy } => {
                drop(pending);
                state.virtual_pointer(client, PointerAction::Motion { time, dx, dy });
            }
            Request::MotionAbsolute { time, x, y, x_extent, y_extent } => {
                drop(pending);
                if x_extent > 0 && y_extent > 0 {
                    let x = (f64::from(x) / f64::from(x_extent)).clamp(0.0, 1.0);
                    let y = (f64::from(y) / f64::from(y_extent)).clamp(0.0, 1.0);
                    state.virtual_pointer(client, PointerAction::MotionAbsolute { time, x, y });
                }
            }
            Request::Button { time, button, state: pressed } => {
                drop(pending);
                let pressed = matches!(pressed, WEnum::Value(wl_pointer::ButtonState::Pressed));
                state.virtual_pointer(client, PointerAction::Button { time, button, pressed });
            }
            Request::Frame => {
                let p = std::mem::take(&mut *pending);
                drop(pending);
                if p.any {
                    let source = p.source.unwrap_or(AxisSource::Wheel);
                    state.virtual_pointer(client, PointerAction::Axis {
                        time: p.time, source, amount: p.amount, v120: p.v120, stopped: p.stopped,
                    });
                }
            }
            other => collect_scroll(pointer, &mut pending, other),
        }
    }
}

fn collect_scroll(pointer: &ZwlrVirtualPointerV1, pending: &mut Pending, request: zwlr_virtual_pointer_v1::Request) {
    use zwlr_virtual_pointer_v1::Request;
    match request {
        Request::Axis { time, axis, value } => {
            if let Some(i) = axis_index(pointer, axis) {
                pending.time = time;
                pending.amount[i] += value;
                pending.any = true;
            }
        }
        Request::AxisDiscrete { time, axis, value, discrete } => {
            if let Some(i) = axis_index(pointer, axis) {
                pending.time = time;
                pending.amount[i] += value;
                pending.v120[i] = Some(pending.v120[i].unwrap_or(0.0) + f64::from(discrete) * 120.0);
                pending.any = true;
            }
        }
        Request::AxisSource { axis_source } => match axis_source {
            WEnum::Value(s) => pending.source = Some(source_of(s)),
            _ => pointer.post_error(zwlr_virtual_pointer_v1::Error::InvalidAxisSource, "no such source"),
        },
        Request::AxisStop { time, axis } => {
            if let Some(i) = axis_index(pointer, axis) {
                pending.time = time;
                pending.stopped[i] = true;
                pending.any = true;
            }
        }
        _ => {}
    }
}

#[macro_export]
macro_rules! delegate_virtual_pointer {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols_wlr::virtual_pointer::v1::server::zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1: $crate::protocols::virtual_pointer::VirtualPointerGlobalData
        ] => $crate::protocols::virtual_pointer::VirtualPointerManagerState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols_wlr::virtual_pointer::v1::server::zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1: ()
        ] => $crate::protocols::virtual_pointer::VirtualPointerManagerState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols_wlr::virtual_pointer::v1::server::zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1: $crate::protocols::virtual_pointer::VirtualPointerData
        ] => $crate::protocols::virtual_pointer::VirtualPointerManagerState);
    };
}

#[cfg(test)]
mod tests {
    use std::os::unix::net::UnixStream;
    use std::sync::Arc;

    use smithay::reexports::wayland_server::backend::{ClientData, ClientId, DisconnectReason};
    use smithay::reexports::wayland_server::Display;
    use wayland_client::globals::{registry_queue_init, GlobalListContents};
    use wayland_client::protocol::{wl_pointer as cp, wl_registry};
    use wayland_client::{delegate_noop, Connection, EventQueue, QueueHandle};
    use wayland_protocols_wlr::virtual_pointer::v1::client::{
        zwlr_virtual_pointer_manager_v1::ZwlrVirtualPointerManagerV1 as Manager, zwlr_virtual_pointer_v1::ZwlrVirtualPointerV1 as Ptr,
    };

    use super::*;

    struct Server {
        heard: Vec<PointerAction>,
    }

    impl VirtualPointerHandler for Server {
        fn virtual_pointer(&mut self, _client: &Client, action: PointerAction) {
            self.heard.push(action);
        }
    }
    crate::delegate_virtual_pointer!(Server);

    struct Allowed(bool);
    impl ClientData for Allowed {
        fn initialized(&self, _: ClientId) {}
        fn disconnected(&self, _: ClientId, _: DisconnectReason) {}
    }

    struct Seen;
    impl wayland_client::Dispatch<wl_registry::WlRegistry, GlobalListContents> for Seen {
        fn event(_: &mut Self, _: &wl_registry::WlRegistry, _: wl_registry::Event, _: &GlobalListContents, _: &Connection, _: &QueueHandle<Self>) {}
    }
    delegate_noop!(Seen: ignore Manager);
    delegate_noop!(Seen: ignore Ptr);

    fn client(display: &mut Display<Server>, server: &mut Server, allowed: bool) -> (Connection, EventQueue<Seen>, Vec<String>, Option<Manager>) {
        let (server_end, client_end) = UnixStream::pair().unwrap();
        display.handle().insert_client(server_end, Arc::new(Allowed(allowed))).unwrap();
        let conn = Connection::from_socket(client_end).unwrap();
        let c = conn.clone();
        let asked = std::thread::spawn(move || registry_queue_init::<Seen>(&c));
        let mut done = None;
        for _ in 0..200 {
            display.dispatch_clients(server).unwrap();
            display.flush_clients().unwrap();
            if asked.is_finished() {
                done = Some(asked.join().unwrap().unwrap());
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        let (globals, queue) = done.expect("the registry answered");
        let names = globals.contents().clone_list().into_iter().map(|g| g.interface).collect();
        let manager = globals.bind::<Manager, _, _>(&queue.handle(), 1..=2, ()).ok();
        (conn, queue, names, manager)
    }

    #[test]
    fn moves_clicks_and_scrolls_are_handed_on_and_only_to_whom_the_filter_allows() {
        let mut display = Display::<Server>::new().unwrap();
        VirtualPointerManagerState::new::<Server, _>(&display.handle(), |c: &Client| {
            c.get_data::<Allowed>().is_some_and(|a| a.0)
        });
        let mut server = Server { heard: Vec::new() };

        let (_c, _q, names, manager) = client(&mut display, &mut server, false);
        assert!(!names.iter().any(|n| n == "zwlr_virtual_pointer_manager_v1") && manager.is_none());

        let (conn, queue, _, manager) = client(&mut display, &mut server, true);
        let ptr = manager.expect("offered").create_virtual_pointer(None, &queue.handle(), ());
        ptr.motion(10, 3.0, -2.0);
        ptr.button(11, 0x110, cp::ButtonState::Pressed);
        ptr.button(12, 0x110, cp::ButtonState::Released);
        ptr.axis_source(cp::AxisSource::Wheel);
        ptr.axis_discrete(13, cp::Axis::VerticalScroll, 15.0, 1);
        ptr.frame();
        ptr.motion_absolute(14, 960, 270, 1920, 1080);
        conn.flush().unwrap();
        for _ in 0..4 {
            display.dispatch_clients(&mut server).unwrap();
        }
        assert_eq!(server.heard, vec![
            PointerAction::Motion { time: 10, dx: 3.0, dy: -2.0 },
            PointerAction::Button { time: 11, button: 0x110, pressed: true },
            PointerAction::Button { time: 12, button: 0x110, pressed: false },
            PointerAction::Axis { time: 13, source: AxisSource::Wheel, amount: [0.0, 15.0], v120: [None, Some(120.0)], stopped: [false; 2] },
            PointerAction::MotionAbsolute { time: 14, x: 0.5, y: 0.25 },
        ]);
    }
}
