use std::{
    env,
    ffi::OsStr,
    os::fd::{BorrowedFd, OwnedFd},
    os::unix::{
        io::{AsRawFd, RawFd},
        net::UnixStream,
        process::CommandExt,
    },
    process::{Child, Command},
    sync::{Arc, Mutex},
    thread,
};

use tracing::{error, info, trace};
use wayland_server::backend::{ClientData, ClientId, DisconnectReason};
use wayland_server::{Client, DisplayHandle};

use crate::{utils::user_data::UserDataMap, wayland::compositor::CompositorClientState};

use super::x11_sockets::{prepare_x11_sockets, X11Lock};

#[derive(Debug)]
pub struct XWayland {
    inner: Arc<Mutex<Instance>>,
    source: calloop::generic::Generic<calloop::generic::FdWrapper<RawFd>>,

    dh: DisplayHandle,
    client: Client,
}

#[derive(Debug)]
pub enum XWaylandEvent {
    Ready {
        x11_socket: UnixStream,

        display_number: u32,
    },

    Error,
}

impl XWayland {
    pub fn spawn<K, V, I, F>(
        dh: &DisplayHandle,
        display: impl Into<Option<u32>>,
        envs: I,
        open_abstract_socket: bool,
        stdout: impl Into<std::process::Stdio>,
        stderr: impl Into<std::process::Stdio>,
        user_data: F,
    ) -> std::io::Result<(Self, Client)>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
        F: FnOnce(&UserDataMap),
    {
        let (x_wm_x11, x_wm_me) = UnixStream::pair()?;
        let (wl_x11, wl_me) = UnixStream::pair()?;

        let (lock, listen_sockets) = prepare_x11_sockets(display.into(), open_abstract_socket)?;
        let display_number = lock.display_number();

        let (displayfd_recv, displayfd_send) =
            rustix::pipe::pipe_with(rustix::pipe::PipeFlags::NONBLOCK | rustix::pipe::PipeFlags::CLOEXEC)?;

        let mut command = Command::new("Xwayland");

        command
            .stdout(stdout)
            .stderr(stderr)
            .arg(format!(":{}", display_number))
            .arg("-verbose")
            .arg("-rootless")
            .arg("-terminate")
            .arg("-wm")
            .arg(x_wm_x11.as_raw_fd().to_string())
            .arg("-displayfd")
            .arg(displayfd_send.as_raw_fd().to_string());

        for socket in &listen_sockets {
            command.arg("-listenfd").arg(socket.as_raw_fd().to_string());
        }

        command.env_clear();
        for (key, value) in env::vars_os() {
            if key.to_str() == Some("PATH") || key.to_str() == Some("XDG_RUNTIME_DIR") {
                command.env(key, value);
                continue;
            }
        }

        command.env("WAYLAND_SOCKET", format!("{}", wl_x11.as_raw_fd()));
        command.envs(envs);

        unsafe {
            let wayland_socket_fd = wl_x11.as_raw_fd();
            let wm_socket_fd = x_wm_x11.as_raw_fd();
            let pipe_fd = displayfd_send.as_raw_fd();
            let socket_fds: Vec<_> = listen_sockets.iter().map(|socket| socket.as_raw_fd()).collect();

            command.pre_exec(move || {
                unset_cloexec(wayland_socket_fd)?;
                unset_cloexec(wm_socket_fd)?;
                unset_cloexec(pipe_fd)?;
                for &socket in socket_fds.iter() {
                    unset_cloexec(socket)?;
                }

                Ok(())
            });
        }

        info!("spawning XWayland instance");

        let child = command.spawn()?;

        // SAFETY: RawFd's AsRawFd impl is infallible.
        let wrapper = unsafe { calloop::generic::FdWrapper::new(displayfd_recv.as_raw_fd()) };
        let source = calloop::generic::Generic::new(wrapper, calloop::Interest::READ, calloop::Mode::Level);
        let inner = Instance {
            display_lock: lock,
            display_fd: displayfd_recv,
            x11_socket: Some(x_wm_me),
        };

        let data_map = UserDataMap::new();
        user_data(&data_map);

        let inner = Arc::new(Mutex::new(inner));
        let mut dh = dh.clone();
        let client = dh.insert_client(
            wl_me,
            Arc::new(XWaylandClientData {
                #[cfg(feature = "wayland_frontend")]
                compositor_state: CompositorClientState::default(),
                data_map,
                child: Mutex::new(Some(child)),
            }),
        )?;

        Ok((
            Self {
                inner,
                source,
                dh,
                client: client.clone(),
            },
            client,
        ))
    }

    pub fn display_number(&self) -> u32 {
        self.inner.lock().unwrap().display_lock.display_number()
    }

    pub fn poll_fd(&self) -> BorrowedFd<'_> {
        let guard = self.inner.lock().unwrap();

        // SAFETY: we never mutate display_fd, and it lives as long as the instance.
        unsafe { BorrowedFd::borrow_raw(guard.display_fd.as_raw_fd()) }
    }

    pub fn take_socket(&mut self) -> std::io::Result<Option<UnixStream>> {
        self.inner.lock().unwrap().take_socket()
    }
}

#[derive(Debug)]
struct Instance {
    display_lock: X11Lock,
    x11_socket: Option<UnixStream>,
    display_fd: OwnedFd,
}

impl calloop::EventSource for XWayland {
    type Event = XWaylandEvent;
    type Metadata = ();
    type Ret = ();
    type Error = std::io::Error;

    #[profiling::function]
    fn process_events<F>(
        &mut self,
        readiness: calloop::Readiness,
        token: calloop::Token,
        mut callback: F,
    ) -> std::io::Result<calloop::PostAction>
    where
        F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        let mut guard = self.inner.lock().unwrap();

        self.source.process_events(readiness, token, |_, _| {
            let x11_socket = match guard.take_socket() {
                Ok(Some(sockets)) => sockets,
                Ok(None) => return Ok(calloop::PostAction::Continue),
                Err(_) => {
                    callback(XWaylandEvent::Error, &mut ());
                    return Ok(calloop::PostAction::Disable);
                }
            };

            callback(
                XWaylandEvent::Ready {
                    x11_socket,
                    display_number: guard.display_lock.display_number(),
                },
                &mut (),
            );

            Ok(calloop::PostAction::Disable)
        })
    }

    fn register(
        &mut self,
        poll: &mut calloop::Poll,
        factory: &mut calloop::TokenFactory,
    ) -> calloop::Result<()> {
        self.source.register(poll, factory)
    }

    fn reregister(
        &mut self,
        poll: &mut calloop::Poll,
        factory: &mut calloop::TokenFactory,
    ) -> calloop::Result<()> {
        self.source.reregister(poll, factory)
    }

    fn unregister(&mut self, poll: &mut calloop::Poll) -> calloop::Result<()> {
        self.source.unregister(poll)
    }
}

impl Drop for XWayland {
    fn drop(&mut self) {
        self.dh
            .backend_handle()
            .kill_client(self.client.id(), DisconnectReason::ConnectionClosed);
    }
}

impl Instance {
    fn take_socket(&mut self) -> std::io::Result<Option<UnixStream>> {
        trace!("checking for XWayland readiness");

        if self.x11_socket.is_none() {
            return Ok(None);
        }

        let mut buf = [0; 64];
        loop {
            let res = rustix::io::read(&self.display_fd, &mut buf);
            trace!(?res, "read from XWayland displayfd");

            match res {
                Ok(0) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::UnexpectedEof,
                        "Xwayland ended before it was ready",
                    ))
                }
                Ok(len) if (buf[..len]).contains(&b'\n') => return Ok(self.x11_socket.take()),
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => return Ok(None),
                Err(err) => return Err(err.into()),
                _ => continue,
            }
        }
    }
}

#[derive(Debug)]
pub struct XWaylandClientData {
    #[cfg(feature = "wayland_frontend")]
    pub compositor_state: CompositorClientState,
    data_map: UserDataMap,
    child: Mutex<Option<Child>>,
}

impl ClientData for XWaylandClientData {
    fn disconnected(&self, _client_id: ClientId, reason: DisconnectReason) {
        if let DisconnectReason::ProtocolError(err) = reason {
            error!("Xwayland disconnected: {}", err);
        }

        let mut child = self.child.lock().unwrap().take().unwrap();
        thread::spawn(move || {
            if let Ok(status) = child.wait() {
                if !status.success() {
                    error!("Xwayland terminated: {}", status);
                }
            }
        });
    }
}

impl XWaylandClientData {
    pub fn user_data(&self) -> &UserDataMap {
        &self.data_map
    }

    pub fn pid(&self) -> Option<u32> {
        self.child.lock().unwrap().as_ref().map(|c| c.id())
    }
}

unsafe fn unset_cloexec(fd: RawFd) -> std::io::Result<()> {
    let fd = BorrowedFd::borrow_raw(fd);
    rustix::io::fcntl_setfd(fd, rustix::io::FdFlags::empty())?;
    Ok(())
}
