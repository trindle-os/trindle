//! A box's own way in to the desktop, through a security-context socket.

use std::os::fd::{AsFd, OwnedFd};
use std::os::unix::net::UnixListener;
use std::path::{Path, PathBuf};

use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_client::protocol::wl_registry;
use wayland_client::{delegate_noop, Connection, Dispatch, QueueHandle};
use wayland_protocols::wp::security_context::v1::client::{
    wp_security_context_manager_v1::WpSecurityContextManagerV1, wp_security_context_v1::WpSecurityContextV1,
};

#[cfg(test)]
use crate::common::sandbox::{BOXED, CAPTURE, CONTROL};

struct State;

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(_: &mut Self, _: &wl_registry::WlRegistry, _: wl_registry::Event, _: &GlobalListContents,
             _: &Connection, _: &QueueHandle<Self>) {}
}
delegate_noop!(State: ignore WpSecurityContextManagerV1);
delegate_noop!(State: ignore WpSecurityContextV1);

pub fn listen(socket: &Path, app_id: &str, engine: &str, close: impl AsFd) -> Result<(), String> {
    let conn = Connection::connect_to_env().map_err(|e| format!("no desktop to ask: {e}"))?;
    let (globals, mut queue) = registry_queue_init::<State>(&conn).map_err(|e| e.to_string())?;
    let qh = queue.handle();
    let manager: WpSecurityContextManagerV1 = globals.bind(&qh, 1..=1, ())
        .map_err(|_| "the desktop has no security context".to_string())?;
    let _ = std::fs::remove_file(socket);
    let listener = UnixListener::bind(socket).map_err(|e| format!("{}: {e}", socket.display()))?;
    let context = manager.create_listener(listener.as_fd(), close.as_fd(), &qh, ());
    context.set_sandbox_engine(engine.into());
    context.set_app_id(app_id.into());
    context.set_instance_id(std::process::id().to_string());
    context.commit();
    queue.roundtrip(&mut State).map_err(|e| e.to_string())?;
    context.destroy();
    manager.destroy();
    queue.roundtrip(&mut State).map_err(|e| e.to_string())?;

    drop(listener);
    Ok(())
}

pub fn close_pipe() -> Option<(OwnedFd, OwnedFd)> {
    use std::os::fd::FromRawFd;
    let mut fds = [0i32; 2];
    // SAFETY: pipe2 writes two descriptors into the array it is given.
    if unsafe { libc::pipe2(fds.as_mut_ptr(), libc::O_CLOEXEC) } != 0 {
        return None;
    }
    // SAFETY: both descriptors are new and owned by nothing else.
    Some(unsafe { (OwnedFd::from_raw_fd(fds[0]), OwnedFd::from_raw_fd(fds[1])) })
}

pub fn keep_across_exec(fd: &impl AsFd) {
    use std::os::fd::AsRawFd;
    let raw = fd.as_fd().as_raw_fd();
    // SAFETY: F_GETFD/F_SETFD only change the descriptor's close-on-exec flag.
    unsafe {
        let flags = libc::fcntl(raw, libc::F_GETFD);
        if flags >= 0 {
            libc::fcntl(raw, libc::F_SETFD, flags & !libc::FD_CLOEXEC);
        }
    }
}

#[cfg(test)]
pub fn globals_at(socket: &Path) -> Result<Vec<String>, String> {
    let stream = std::os::unix::net::UnixStream::connect(socket).map_err(|e| e.to_string())?;
    let conn = Connection::from_socket(stream).map_err(|e| e.to_string())?;
    let (globals, _queue) = registry_queue_init::<State>(&conn).map_err(|e| e.to_string())?;
    Ok(globals.contents().clone_list().into_iter().map(|g| g.interface).collect())
}

pub fn socket_in(run: &Path) -> PathBuf {
    run.join("wayland-0")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "needs a real compositor, named by TRINDLE_SECCTX_TEST: run by the script that sets it up"]
    fn a_box_socket_hides_the_screen_and_the_clipboard() {
        let Ok(run) = std::env::var("TRINDLE_SECCTX_TEST") else { return };
        let run = PathBuf::from(run);
        let display = std::env::var("WAYLAND_DISPLAY").unwrap();
        let plain = globals_at(&run.join(&display)).unwrap();
        let (read, write) = close_pipe().unwrap();
        let socket = run.join("secctx-test");
        listen(&socket, "secctx-test", BOXED, &read).unwrap();
        let boxed = globals_at(&socket).unwrap();
        let hidden = ["ext_image_copy_capture_manager_v1", "ext_data_control_manager_v1", "zwlr_data_control_manager_v1",
                      "zwlr_layer_shell_v1", "zwp_virtual_keyboard_manager_v1", "zwlr_virtual_pointer_manager_v1",
                      "trindle_windows_manager_v1", "trindle_workspaces_manager_v1", "trindle_screens_manager_v1",
                      "trindle_night_manager_v1"];
        println!("plain offers {}, the box {}", plain.len(), boxed.len());
        for h in hidden {
            assert!(!boxed.iter().any(|g| g == h), "{h} is offered to a box");
        }
        for offered in ["ext_image_copy_capture_manager_v1", "trindle_windows_manager_v1", "trindle_screens_manager_v1"] {
            assert!(plain.iter().any(|g| g == offered), "{offered} is not offered to a plain program: {plain:?}");
        }
        assert!(boxed.iter().any(|g| g == "xdg_wm_base") && boxed.iter().any(|g| g == "wl_compositor"));
        drop(write);
        let _ = std::fs::remove_file(&socket);
    }

    #[test]
    #[ignore = "needs a real compositor, named by TRINDLE_SECCTX_TEST: run by the script that sets it up"]
    fn a_control_socket_adds_only_the_mouse_keyboard_and_clipboard() {
        let Ok(run) = std::env::var("TRINDLE_SECCTX_TEST") else { return };
        let run = PathBuf::from(run);
        let names = |engine: &str, name: &str| {
            let (read, write) = close_pipe().unwrap();
            let socket = run.join(name);
            listen(&socket, name, engine, &read).unwrap();
            let mut g = globals_at(&socket).unwrap();
            drop(write);
            let _ = std::fs::remove_file(&socket);
            g.sort();
            g.dedup();
            g
        };
        let boxed = names(BOXED, "secctx-boxed");
        let control = names(CONTROL, "secctx-control");
        let extra: Vec<&String> = control.iter().filter(|g| !boxed.contains(g)).collect();
        println!("a box is offered {} kinds, a box allowed control {}: {extra:?}", boxed.len(), control.len());
        assert_eq!(extra, ["zwlr_data_control_manager_v1", "zwlr_virtual_pointer_manager_v1", "zwp_virtual_keyboard_manager_v1"]);
        assert!(boxed.iter().all(|g| control.contains(g)));

        let capture = names(CAPTURE, "secctx-capture");
        let extra: Vec<&String> = capture.iter().filter(|g| !boxed.contains(g)).collect();
        assert_eq!(extra, ["trindle_input_capture_manager_v1"]);
        let both = names(crate::common::sandbox::CONTROL_CAPTURE, "secctx-both");
        let extra: Vec<&String> = both.iter().filter(|g| !boxed.contains(g)).collect();
        assert_eq!(extra, ["trindle_input_capture_manager_v1", "zwlr_data_control_manager_v1",
                           "zwlr_virtual_pointer_manager_v1", "zwp_virtual_keyboard_manager_v1"]);
        assert!(!control.iter().any(|g| g == "trindle_input_capture_manager_v1"));
        for h in ["ext_image_copy_capture_manager_v1", "zwlr_layer_shell_v1", "trindle_windows_manager_v1",
                  "trindle_workspaces_manager_v1", "trindle_screens_manager_v1", "trindle_night_manager_v1",
                  "zwp_input_method_manager_v2", "wp_security_context_manager_v1", "ext_session_lock_manager_v1"] {
            assert!(!control.iter().any(|g| g == h), "{h} is offered to a box allowed control");
        }
    }
}
