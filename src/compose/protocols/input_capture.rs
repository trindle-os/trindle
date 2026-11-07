//! trindle_input_capture_v1 (input_capture.xml).

use smithay::reexports::wayland_server::{Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource, WEnum};

#[allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables, non_upper_case_globals, non_snake_case,
        unused_imports, missing_docs, clippy::all)]
pub mod server {
    use smithay::reexports::wayland_server;
    use smithay::reexports::wayland_server::protocol::*;

    pub mod __interfaces {
        use smithay::reexports::wayland_server::backend as wayland_backend;
        use smithay::reexports::wayland_server::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("src/compose/protocols/input_capture.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_server_code!("src/compose/protocols/input_capture.xml");
}

pub use server::trindle_input_capture_manager_v1::{self as manager, TrindleInputCaptureManagerV1};
pub use server::trindle_input_capture_v1::{self as capture, TrindleInputCaptureV1};

const VERSION: u32 = 2;

pub use crate::screen_edge::Side;

pub trait InputCaptureHandler {
    fn capture_asked(&mut self, capture: TrindleInputCaptureV1, side: Side);
    fn capture_take(&mut self, capture: &TrindleInputCaptureV1, side: Side);
    fn capture_mode(&mut self, capture: &TrindleInputCaptureV1, manual: bool);
    fn capture_release(&mut self, capture: &TrindleInputCaptureV1, along: f64);
    fn capture_gone(&mut self, capture: &TrindleInputCaptureV1);
}

#[derive(Debug)]
pub struct InputCaptureState;

pub struct InputCaptureGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

impl InputCaptureState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> InputCaptureState
    where
        D: GlobalDispatch<TrindleInputCaptureManagerV1, InputCaptureGlobalData>
            + Dispatch<TrindleInputCaptureManagerV1, ()>
            + Dispatch<TrindleInputCaptureV1, ()>
            + InputCaptureHandler
            + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        display.create_global::<D, TrindleInputCaptureManagerV1, _>(
            VERSION,
            InputCaptureGlobalData { filter: Box::new(filter) },
        );
        InputCaptureState
    }
}

pub fn side_of(raw: u32) -> Option<Side> {
    match raw {
        1 => Some(Side::Left),
        2 => Some(Side::Right),
        3 => Some(Side::Top),
        4 => Some(Side::Bottom),
        _ => None,
    }
}

impl<D> GlobalDispatch<TrindleInputCaptureManagerV1, InputCaptureGlobalData, D> for InputCaptureState
where
    D: GlobalDispatch<TrindleInputCaptureManagerV1, InputCaptureGlobalData>
        + Dispatch<TrindleInputCaptureManagerV1, ()>
        + Dispatch<TrindleInputCaptureV1, ()>
        + InputCaptureHandler
        + 'static,
{
    fn bind(
        _: &mut D,
        _: &DisplayHandle,
        _: &Client,
        resource: New<TrindleInputCaptureManagerV1>,
        _: &InputCaptureGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }

    fn can_view(client: Client, global_data: &InputCaptureGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

impl<D> Dispatch<TrindleInputCaptureManagerV1, (), D> for InputCaptureState
where
    D: Dispatch<TrindleInputCaptureManagerV1, ()> + Dispatch<TrindleInputCaptureV1, ()> + InputCaptureHandler + 'static,
{
    fn request(
        state: &mut D,
        _: &Client,
        manager: &TrindleInputCaptureManagerV1,
        request: manager::Request,
        _: &(),
        _: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        match request {
            manager::Request::GetCapture { id, side } => {
                let raw = match side {
                    WEnum::Value(s) => s as u32,
                    WEnum::Unknown(n) => n,
                };
                let capture = data_init.init(id, ());
                match side_of(raw) {
                    Some(side) => state.capture_asked(capture, side),
                    None => manager.post_error(0u32, "no such side"),
                }
            }
            manager::Request::Destroy => {}
        }
    }
}

impl<D> Dispatch<TrindleInputCaptureV1, (), D> for InputCaptureState
where
    D: Dispatch<TrindleInputCaptureV1, ()> + InputCaptureHandler + 'static,
{
    fn request(
        state: &mut D,
        _: &Client,
        capture: &TrindleInputCaptureV1,
        request: capture::Request,
        _: &(),
        _: &DisplayHandle,
        _: &mut DataInit<'_, D>,
    ) {
        match request {
            capture::Request::Release { along } => state.capture_release(capture, along.clamp(0.0, 1.0)),
            capture::Request::Take { side } => match side_of(side) {
                Some(side) => state.capture_take(capture, side),
                None => capture.post_error(0u32, "no such side"),
            },
            capture::Request::SetMode { mode } => match mode {
                WEnum::Value(capture::Mode::Push) => state.capture_mode(capture, false),
                WEnum::Value(capture::Mode::Manual) => state.capture_mode(capture, true),
                _ => capture.post_error(0u32, "no such mode"),
            },
            capture::Request::Destroy => {}
        }
    }

    fn destroyed(state: &mut D, _: smithay::reexports::wayland_server::backend::ClientId, capture: &TrindleInputCaptureV1, _: &()) {
        state.capture_gone(capture);
    }
}

#[macro_export]
macro_rules! delegate_input_capture {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::input_capture::TrindleInputCaptureManagerV1: $crate::protocols::input_capture::InputCaptureGlobalData
        ] => $crate::protocols::input_capture::InputCaptureState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::input_capture::TrindleInputCaptureManagerV1: ()
        ] => $crate::protocols::input_capture::InputCaptureState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::input_capture::TrindleInputCaptureV1: ()
        ] => $crate::protocols::input_capture::InputCaptureState);
    };
}

#[cfg(test)]
mod tests {
    use std::os::unix::net::UnixStream;
    use std::sync::Arc;

    use smithay::reexports::wayland_server::backend::{ClientData, ClientId, DisconnectReason};
    use smithay::reexports::wayland_server::Display;
    use wayland_client::globals::{registry_queue_init, GlobalListContents};
    use wayland_client::protocol::wl_registry;
    use wayland_client::{Connection, EventQueue, QueueHandle};

    use super::*;

    #[allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables, non_upper_case_globals,
            non_snake_case, unused_imports, missing_docs, clippy::all)]
    mod client {
        use wayland_client;
        use wayland_client::protocol::*;

        pub mod __interfaces {
            use wayland_client::backend as wayland_backend;
            use wayland_client::protocol::__interfaces::*;
            wayland_scanner::generate_interfaces!("src/compose/protocols/input_capture.xml");
        }
        use self::__interfaces::*;

        wayland_scanner::generate_client_code!("src/compose/protocols/input_capture.xml");
    }
    use client::trindle_input_capture_manager_v1::{Side as CSide, TrindleInputCaptureManagerV1 as Manager};
    use client::trindle_input_capture_v1::{self as cc, TrindleInputCaptureV1 as Cap};

    struct Server {
        heard: Vec<String>,
        captures: Vec<TrindleInputCaptureV1>,
    }

    impl InputCaptureHandler for Server {
        fn capture_asked(&mut self, capture: TrindleInputCaptureV1, side: Side) {
            self.heard.push(format!("asked {side:?}"));
            self.captures.push(capture);
        }
        fn capture_take(&mut self, _: &TrindleInputCaptureV1, side: Side) {
            self.heard.push(format!("take {side:?}"));
        }
        fn capture_mode(&mut self, _: &TrindleInputCaptureV1, manual: bool) {
            self.heard.push(format!("mode {}", if manual { "manual" } else { "push" }));
        }
        fn capture_release(&mut self, _: &TrindleInputCaptureV1, along: f64) {
            self.heard.push(format!("release {along}"));
        }
        fn capture_gone(&mut self, _: &TrindleInputCaptureV1) {
            self.heard.push("gone".into());
        }
    }
    crate::delegate_input_capture!(Server);

    struct Allowed(bool);
    impl ClientData for Allowed {
        fn initialized(&self, _: ClientId) {}
        fn disconnected(&self, _: ClientId, _: DisconnectReason) {}
    }

    #[derive(Default)]
    struct Seen(Vec<String>);
    impl wayland_client::Dispatch<wl_registry::WlRegistry, GlobalListContents> for Seen {
        fn event(_: &mut Self, _: &wl_registry::WlRegistry, _: wl_registry::Event, _: &GlobalListContents, _: &Connection, _: &QueueHandle<Self>) {}
    }
    wayland_client::delegate_noop!(Seen: ignore Manager);
    impl wayland_client::Dispatch<Cap, ()> for Seen {
        fn event(state: &mut Self, _: &Cap, event: cc::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
            state.0.push(match event {
                cc::Event::Ready => "ready".into(),
                cc::Event::Unavailable { reason } => format!("unavailable {reason:?}"),
                cc::Event::Activated { along } => format!("activated {along}"),
                cc::Event::Motion { time, dx, dy } => format!("motion {time} {dx} {dy}"),
                cc::Event::Button { time, button, state } => format!("button {time} {button} {state}"),
                cc::Event::Axis { time, axis, value, v120 } => format!("axis {time} {axis} {value} {v120}"),
                cc::Event::Key { time, key, state } => format!("key {time} {key} {state}"),
                cc::Event::Deactivated { reason } => format!("deactivated {reason:?}"),
            });
        }
    }

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

    fn turn(display: &mut Display<Server>, server: &mut Server, conn: &Connection, queue: &mut EventQueue<Seen>, seen: &mut Seen) {
        for _ in 0..4 {
            conn.flush().unwrap();
            display.dispatch_clients(server).unwrap();
            display.flush_clients().unwrap();
            if let Some(guard) = conn.prepare_read() {
                let _ = guard.read();
            }
            queue.dispatch_pending(seen).unwrap();
        }
    }

    #[test]
    fn a_side_is_asked_for_and_what_the_mouse_and_keyboard_do_goes_to_it() {
        let mut display = Display::<Server>::new().unwrap();
        InputCaptureState::new::<Server, _>(&display.handle(), |c: &Client| c.get_data::<Allowed>().is_some_and(|a| a.0));
        let mut server = Server { heard: Vec::new(), captures: Vec::new() };

        let (_c, _q, names, manager) = client(&mut display, &mut server, false);
        assert!(!names.iter().any(|n| n == "trindle_input_capture_manager_v1") && manager.is_none());

        let (conn, mut queue, _, manager) = client(&mut display, &mut server, true);
        let mut seen = Seen::default();
        let cap = manager.expect("offered").get_capture(CSide::Right, &queue.handle(), ());
        turn(&mut display, &mut server, &conn, &mut queue, &mut seen);
        assert_eq!(server.heard, ["asked Right"]);

        let on = server.captures[0].clone();
        on.ready();
        on.activated(0.25);
        on.motion(5, 3.5, -2.0);
        on.button(6, 0x110, 1);
        on.axis(7, 0, 15.0, 120);
        on.key(8, 30, 1);
        on.deactivated(capture::DeactivatedReason::SafetyKeys);
        turn(&mut display, &mut server, &conn, &mut queue, &mut seen);
        assert_eq!(seen.0, [
            "ready", "activated 0.25", "motion 5 3.5 -2", "button 6 272 1", "axis 7 0 15 120", "key 8 30 1",
            "deactivated Value(SafetyKeys)",
        ]);

        cap.release(0.75);
        cap.destroy();
        turn(&mut display, &mut server, &conn, &mut queue, &mut seen);
        assert_eq!(server.heard, ["asked Right", "release 0.75", "gone"]);
    }

    #[test]
    fn take_and_the_mode_reach_the_compositor_and_not_nonsense() {
        let mut display = Display::<Server>::new().unwrap();
        InputCaptureState::new::<Server, _>(&display.handle(), |_: &Client| true);
        let mut server = Server { heard: Vec::new(), captures: Vec::new() };
        let (conn, mut queue, _, manager) = client(&mut display, &mut server, true);
        let mut seen = Seen::default();
        let cap = manager.expect("offered").get_capture(CSide::Left, &queue.handle(), ());
        cap.set_mode(cc::Mode::Manual);
        cap.take(CSide::Bottom as u32);
        cap.set_mode(cc::Mode::Push);
        cap.take(CSide::Right as u32);
        turn(&mut display, &mut server, &conn, &mut queue, &mut seen);
        assert_eq!(server.heard, ["asked Left", "mode manual", "take Bottom", "mode push", "take Right"]);

        let on = server.captures[0].clone();
        on.unavailable(capture::UnavailableReason::NotNow);
        on.unavailable(capture::UnavailableReason::Taken);
        turn(&mut display, &mut server, &conn, &mut queue, &mut seen);
        assert_eq!(seen.0, ["unavailable Value(NotNow)", "unavailable Value(Taken)"]);

        cap.take(9);
        for _ in 0..4 {
            let _ = conn.flush();
            let _ = display.dispatch_clients(&mut server);
            let _ = display.flush_clients();
            if let Some(guard) = conn.prepare_read() {
                let _ = guard.read();
            }
            let _ = queue.dispatch_pending(&mut seen);
        }
        assert!(server.heard[5..].iter().all(|h| h == "gone"), "no take was heard for it: {:?}", server.heard);
        assert!(conn.protocol_error().is_some(), "no such side is a protocol error");
    }
}
