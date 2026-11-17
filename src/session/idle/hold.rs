//! The last way to lock before sleep, when the lock screen itself would not come up: the compositor's own lock, dark and taking no keys.

use std::time::{Duration, Instant};

use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_client::protocol::wl_registry;
use wayland_client::{Connection, Dispatch, QueueHandle};
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_manager_v1::ExtSessionLockManagerV1;
use wayland_protocols::ext::session_lock::v1::client::ext_session_lock_v1::{self, ExtSessionLockV1};

const WAIT: Duration = Duration::from_secs(3);

#[derive(Default)]
struct Held {
    locked: bool,
    refused: bool,
}

pub(super) fn compositor_lock() -> bool {
    let Ok(conn) = Connection::connect_to_env() else { return false };
    let Ok((globals, mut queue)) = registry_queue_init::<Held>(&conn) else { return false };
    let qh = queue.handle();
    let Ok(manager) = globals.bind::<ExtSessionLockManagerV1, _, _>(&qh, 1..=1, ()) else { return false };
    let _lock = manager.lock(&qh, ());
    let mut held = Held::default();
    let since = Instant::now();
    while !held.locked && !held.refused && since.elapsed() < WAIT {
        if queue.roundtrip(&mut held).is_err() {
            return false;
        }
        if !held.locked && !held.refused {
            std::thread::sleep(Duration::from_millis(20));
        }
    }
    if held.locked {

        use crate::session::lock::tries::{boot_now, file, Tries};
        let path = file();
        if let Err(e) = Tries::load(&path, boot_now().unwrap_or(0)).save(&path) {
            eprintln!("trindle-idle: the lock's count was not begun: {e}");
        }
    }
    held.locked
}

impl Dispatch<ExtSessionLockV1, ()> for Held {
    fn event(held: &mut Self, _: &ExtSessionLockV1, event: ext_session_lock_v1::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {
        match event {
            ext_session_lock_v1::Event::Locked => held.locked = true,
            ext_session_lock_v1::Event::Finished => held.refused = true,
            _ => {}
        }
    }
}

impl Dispatch<ExtSessionLockManagerV1, ()> for Held {
    fn event(_: &mut Self, _: &ExtSessionLockManagerV1, _: <ExtSessionLockManagerV1 as wayland_client::Proxy>::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for Held {
    fn event(_: &mut Self, _: &wl_registry::WlRegistry, _: wl_registry::Event, _: &GlobalListContents, _: &Connection, _: &QueueHandle<Self>) {}
}
