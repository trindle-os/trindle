//! Implementation of the backend types using X11.

mod buffer;
mod error;
#[macro_use]
mod extension;
mod input;
mod surface;
mod window_inner;

use crate::{
    backend::{
        allocator::{Allocator, Swapchain},
        drm::{CreateDrmNodeError, DrmNode, NodeType},
        egl::{native::X11DefaultDisplay, EGLDevice, EGLDisplay, Error as EGLError},
        input::{Axis, ButtonState, InputEvent, KeyState, Keycode},
    },
    utils::{x11rb::X11Source, Logical, Size},
};
use calloop::{EventSource, Poll, PostAction, Readiness, Token, TokenFactory};
use drm::node::path_to_type;
use drm_fourcc::{DrmFourcc, DrmModifier};
use rustix::fs::{Mode, OFlags};
use std::{
    collections::HashMap,
    io,
    os::unix::io::OwnedFd,
    sync::{
        atomic::{AtomicU32, Ordering},
        mpsc, Arc, Mutex, Weak,
    },
};
use tracing::{debug_span, error, info, instrument, warn};
use x11rb::{
    atom_manager,
    connection::Connection,
    protocol::{
        self as x11,
        dri3::ConnectionExt as _,
        xinput,
        xproto::{ColormapAlloc, ConnectionExt, CreateWindowAux, VisualClass, WindowClass, WindowWrapper},
        ErrorKind,
    },
    rust_connection::{ReplyError, RustConnection},
};

use self::{extension::Extensions, window_inner::WindowInner};

pub use self::error::*;
pub use self::input::*;
pub use self::surface::*;

use super::allocator::dmabuf::{AnyError, Dmabuf};

#[derive(Debug)]
pub enum X11Event {
    Refresh {
        window_id: u32,
    },

    Focus {
        focused: bool,
        window_id: u32,
    },

    Input {
        event: InputEvent<X11Input>,
        window_id: Option<u32>,
    },

    Resized {
        new_size: Size<u16, Logical>,
        window_id: u32,
    },

    PresentCompleted {
        window_id: u32,
    },

    CloseRequested {
        window_id: u32,
    },
}

#[derive(Debug)]
pub struct X11Backend {
    connection: Arc<RustConnection>,
    source: X11Source,
    inner: Arc<Mutex<X11Inner>>,
    span: tracing::Span,
}

impl X11Backend {
    pub fn new() -> Result<X11Backend, X11Error> {
        let span = debug_span!("backend_x11");
        let _guard = span.enter();

        info!("Connecting to the X server");

        let (connection, screen_number) = RustConnection::connect(None)?;
        let connection = Arc::new(connection);
        info!(screen = screen_number, "Connected");
        span.record("screen", screen_number);

        let extensions = Extensions::check_extensions(&*connection)?;

        let screen = &connection.setup().roots[screen_number];

        let depth = screen
            .allowed_depths
            .iter()
            .find(|depth| depth.depth == 32)
            .or_else(|| screen.allowed_depths.iter().find(|depth| depth.depth == 24))
            .cloned()
            .ok_or(CreateWindowError::NoDepth)?;

        let visual_id = depth
            .visuals
            .iter()
            .filter(|visual| visual.red_mask == 0xff0000)
            .find(|visual| visual.class == VisualClass::TRUE_COLOR)
            .ok_or(CreateWindowError::NoVisual)?
            .visual_id;

        let format = match depth.depth {
            24 => DrmFourcc::Xrgb8888,
            32 => DrmFourcc::Argb8888,
            _ => unreachable!(),
        };
        info!(?depth, visual_id, %format, "Window parameters selected");

        let colormap = connection.generate_id()?;
        connection.create_colormap(ColormapAlloc::NONE, colormap, screen.root, visual_id)?;

        let atoms = Atoms::new(&*connection)?.reply()?;

        let close_window = WindowWrapper::create_window(
            &*connection,
            x11rb::COPY_DEPTH_FROM_PARENT,
            screen.root,
            0,
            0,
            1,
            1,
            0,
            WindowClass::INPUT_OUTPUT,
            x11rb::COPY_FROM_PARENT,
            &CreateWindowAux::new(),
        )?
        .into_window();

        let source = X11Source::new(connection.clone(), close_window, atoms._SMITHAY_X11_BACKEND_CLOSE);

        let inner = X11Inner {
            connection: connection.clone(),
            screen_number,
            windows: HashMap::new(),
            key_counter: Arc::new(AtomicU32::new(0)),
            window_format: format,
            extensions,
            colormap,
            atoms,
            depth,
            visual_id,
            devices: false,
        };

        drop(_guard);
        Ok(X11Backend {
            connection,
            source,
            inner: Arc::new(Mutex::new(inner)),
            span,
        })
    }

    pub fn handle(&self) -> X11Handle {
        X11Handle {
            connection: self.connection.clone(),
            inner: self.inner.clone(),
            span: self.span.clone(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum EGLInitError {
    #[error(transparent)]
    EGL(#[from] EGLError),
    #[error(transparent)]
    IO(#[from] io::Error),
}

#[derive(Debug)]
pub struct X11Handle {
    connection: Arc<RustConnection>,
    inner: Arc<Mutex<X11Inner>>,
    span: tracing::Span,
}

impl X11Handle {
    pub fn screen(&self) -> usize {
        self.inner.lock().unwrap().screen_number
    }

    pub fn connection(&self) -> Arc<RustConnection> {
        self.connection.clone()
    }

    pub fn format(&self) -> DrmFourcc {
        self.inner.lock().unwrap().window_format
    }

    #[instrument(parent = &self.span, skip(self), ret, err)]
    pub fn drm_node(&self) -> Result<(DrmNode, OwnedFd), X11Error> {

        let inner = self.inner.lock().unwrap();

        egl_init(&inner).or_else(|err| {
            warn!(
                "Failed to init X11 surface via egl, falling back to dri3: {}",
                err
            );
            dri3_init(&inner)
        })
    }

    #[instrument(parent = &self.span, skip(self, allocator, modifiers))]
    pub fn create_surface<A: Allocator<Buffer = Dmabuf, Error = AnyError> + 'static>(
        &self,
        window: &Window,
        allocator: A,
        modifiers: impl Iterator<Item = DrmModifier>,
    ) -> Result<X11Surface, X11Error> {
        let has_resize = { window.0.resize.lock().unwrap().is_some() };

        if has_resize {
            return Err(X11Error::SurfaceExists);
        }

        let inner = self.inner.clone();
        let inner_guard = inner.lock().unwrap();

        if !inner_guard.windows.contains_key(&window.id()) {
            return Err(X11Error::InvalidWindow);
        }

        let mut modifiers = modifiers.collect::<Vec<_>>();
        if window.0.extensions.dri3 < Some((1, 2)) {
            modifiers.retain(|modi| modi == &DrmModifier::Invalid || modi == &DrmModifier::Linear);
        }

        let format = window.0.format;
        let size = window.size();
        let swapchain = Swapchain::new(
            Box::new(allocator) as Box<dyn Allocator<Buffer = Dmabuf, Error = AnyError> + 'static>,
            size.w as u32,
            size.h as u32,
            format,
            modifiers,
        );

        let (sender, recv) = mpsc::channel();

        {
            let mut resize = window.0.resize.lock().unwrap();
            *resize = Some(sender);
        }

        Ok(X11Surface {
            connection: Arc::downgrade(&inner_guard.connection),
            window: Arc::downgrade(&window.0),
            swapchain,
            format,
            width: size.w,
            height: size.h,
            buffer: None,
            resize: recv,
            span: self.span.clone(),
        })
    }

    pub fn window_ref_from_id(&self, id: u32) -> Option<impl AsRef<Window> + '_> {
        X11Inner::window_ref_from_id(&self.inner, &id)
            .and_then(|w| w.upgrade())
            .map(Window)
            .map(WindowTemporary)
    }
}

#[derive(Debug)]
pub struct WindowBuilder<'a> {
    name: Option<&'a str>,
    size: Option<Size<u16, Logical>>,
}

impl<'a> WindowBuilder<'a> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> WindowBuilder<'a> {
        WindowBuilder {
            name: None,
            size: None,
        }
    }

    pub fn title(self, name: &'a str) -> Self {
        Self {
            name: Some(name),
            ..self
        }
    }

    pub fn size(self, size: Size<u16, Logical>) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    pub fn build(self, handle: &X11Handle) -> Result<Window, X11Error> {
        let _guard = handle.span.enter();
        let connection = handle.connection();

        let inner = &mut *handle.inner.lock().unwrap();

        let window = Arc::new(WindowInner::new(
            Arc::downgrade(&connection),
            &connection.setup().roots[inner.screen_number],
            self.size.unwrap_or_else(|| (1280, 800).into()),
            self.name.unwrap_or("Smithay"),
            inner.window_format,
            inner.atoms,
            inner.depth.clone(),
            inner.visual_id,
            inner.colormap,
            inner.extensions,
        )?);

        let downgrade = Arc::downgrade(&window);
        inner.windows.insert(window.id, downgrade);

        Ok(Window(window))
    }
}

#[derive(Debug)]
pub struct Window(Arc<WindowInner>);

impl Window {
    pub fn set_title(&self, title: &str) {
        self.0.set_title(title);
    }

    pub fn map(&self) {
        self.0.map();
    }

    pub fn unmap(&self) {
        self.0.unmap();
    }

    pub fn size(&self) -> Size<u16, Logical> {
        self.0.size()
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        self.0.set_cursor_visible(visible);
    }

    pub fn id(&self) -> u32 {
        self.0.id
    }

    pub fn depth(&self) -> u8 {
        self.0.depth.depth
    }

    pub fn format(&self) -> DrmFourcc {
        self.0.format
    }
}

impl PartialEq for Window {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

struct WindowTemporary(Window);

impl AsRef<Window> for WindowTemporary {
    #[inline]
    fn as_ref(&self) -> &Window {
        &self.0
    }
}

impl EventSource for X11Backend {
    type Event = X11Event;
    type Metadata = ();
    type Ret = ();
    type Error = X11Error;

    #[profiling::function]
    fn process_events<F>(
        &mut self,
        readiness: Readiness,
        token: Token,
        mut callback: F,
    ) -> Result<PostAction, X11Error>
    where
        F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        let connection = self.connection.clone();
        let inner = self.inner.clone();
        let _guard = self.span.enter();

        let post_action = self
            .source
            .process_events(readiness, token, |event, _| match event {
                calloop::channel::Event::Msg(event) => {
                    X11Inner::process_event(&inner, event, &mut callback);
                }
                calloop::channel::Event::Closed => {}
            })
            .map_err(|_| X11Error::ConnectionLost)?;

        let _ = connection.flush();

        Ok(post_action)
    }

    fn register(&mut self, poll: &mut Poll, token_factory: &mut TokenFactory) -> calloop::Result<()> {
        self.source.register(poll, token_factory)
    }

    fn reregister(&mut self, poll: &mut Poll, token_factory: &mut TokenFactory) -> calloop::Result<()> {
        self.source.reregister(poll, token_factory)
    }

    fn unregister(&mut self, poll: &mut Poll) -> calloop::Result<()> {
        self.source.unregister(poll)
    }
}

atom_manager! {
    pub(crate) Atoms: AtomCollectionCookie {
        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        _NET_WM_NAME,
        UTF8_STRING,
        _SMITHAY_X11_BACKEND_CLOSE,
    }
}

#[derive(Debug)]
pub(crate) struct X11Inner {
    connection: Arc<RustConnection>,
    screen_number: usize,
    windows: HashMap<u32, Weak<WindowInner>>,
    key_counter: Arc<AtomicU32>,
    window_format: DrmFourcc,
    extensions: Extensions,
    colormap: u32,
    atoms: Atoms,
    depth: x11::xproto::Depth,
    visual_id: u32,
    devices: bool,
}

impl X11Inner {
    fn window_ref_from_id(inner: &Arc<Mutex<X11Inner>>, id: &u32) -> Option<Weak<WindowInner>> {
        let mut inner = inner.lock().unwrap();
        inner.windows.retain(|_, weak| weak.strong_count() != 0);
        inner.windows.get(id).cloned()
    }

    #[profiling::function]
    fn process_event<F>(inner: &Arc<Mutex<X11Inner>>, event: x11::Event, callback: &mut F)
    where
        F: FnMut(X11Event, &mut ()),
    {
        {
            let mut inner = inner.lock().unwrap();
            if !inner.windows.is_empty() && !inner.devices {
                callback(
                    Input {
                        event: InputEvent::DeviceAdded {
                            device: X11VirtualDevice,
                        },
                        window_id: None,
                    },
                    &mut (),
                );
                inner.devices = true;
            } else if inner.windows.is_empty() && inner.devices {
                callback(
                    Input {
                        event: InputEvent::DeviceRemoved {
                            device: X11VirtualDevice,
                        },
                        window_id: None,
                    },
                    &mut (),
                );
                inner.devices = false;
            }
        }

        use self::X11Event::{Focus, Input};

        match event {
            x11::Event::XinputFocusIn(focus_in) => {
                callback(
                    Focus {
                        focused: true,
                        window_id: focus_in.event,
                    },
                    &mut (),
                );
            }

            x11::Event::XinputFocusOut(focus_out) => {
                callback(
                    Focus {
                        focused: false,
                        window_id: focus_out.event,
                    },
                    &mut (),
                );
            }

            x11::Event::XinputButtonPress(button_press) => {
                if let Some(window) = X11Inner::window_ref_from_id(inner, &button_press.event) {

                    if button_press.detail >= 4 && button_press.detail <= 7 {
                        callback(
                            Input {
                                event: InputEvent::PointerAxis {
                                    event: X11MouseWheelEvent {
                                        time: button_press.time,
                                        axis: match button_press.detail {
                                            4 | 5 => Axis::Vertical,

                                            6 | 7 => Axis::Horizontal,

                                            _ => unreachable!(),
                                        },
                                        amount: match button_press.detail {
                                            4 | 7 => 1.0,

                                            5 | 6 => -1.0,

                                            _ => unreachable!(),
                                        },
                                        window,
                                    },
                                },
                                window_id: Some(button_press.event),
                            },
                            &mut (),
                        )
                    } else {
                        callback(
                            Input {
                                event: InputEvent::PointerButton {
                                    event: X11MouseInputEvent {
                                        time: button_press.time,
                                        raw: button_press.detail,
                                        state: ButtonState::Pressed,
                                        window,
                                    },
                                },
                                window_id: Some(button_press.event),
                            },
                            &mut (),
                        )
                    }
                }
            }

            x11::Event::XinputButtonRelease(button_release) => {
                if button_release.detail >= 4 && button_release.detail <= 7 {
                    return;
                }

                if let Some(window) = X11Inner::window_ref_from_id(inner, &button_release.event) {
                    callback(
                        Input {
                            event: InputEvent::PointerButton {
                                event: X11MouseInputEvent {
                                    time: button_release.time,
                                    raw: button_release.detail,
                                    state: ButtonState::Released,
                                    window,
                                },
                            },
                            window_id: Some(button_release.event),
                        },
                        &mut (),
                    );
                }
            }

            x11::Event::XinputKeyPress(key_press) => {
                if key_press.flags.contains(xinput::KeyEventFlags::KEY_REPEAT) {
                    return;
                }

                if let Some(window) = X11Inner::window_ref_from_id(inner, &key_press.event) {
                    let count = { inner.lock().unwrap().key_counter.fetch_add(1, Ordering::SeqCst) + 1 };

                    callback(
                        Input {
                            event: InputEvent::Keyboard {
                                event: X11KeyboardInputEvent {
                                    time: key_press.time,
                                    key: Keycode::from(key_press.detail),
                                    count,
                                    state: KeyState::Pressed,
                                    window,
                                },
                            },
                            window_id: Some(key_press.event),
                        },
                        &mut (),
                    )
                }
            }

            x11::Event::XinputKeyRelease(key_release) => {
                if let Some(window) = X11Inner::window_ref_from_id(inner, &key_release.event) {
                    let count = {
                        let key_counter = inner.lock().unwrap().key_counter.clone();

                        let mut count = key_counter.load(Ordering::SeqCst);
                        count = count.saturating_sub(1);
                        key_counter.store(count, Ordering::SeqCst);

                        count
                    };

                    callback(
                        Input {
                            event: InputEvent::Keyboard {
                                event: X11KeyboardInputEvent {
                                    time: key_release.time,
                                    key: Keycode::from(key_release.detail),
                                    count,
                                    state: KeyState::Released,
                                    window,
                                },
                            },
                            window_id: Some(key_release.event),
                        },
                        &mut (),
                    );
                }
            }

            x11::Event::XinputMotion(motion_notify) => {
                if let Some(window) =
                    X11Inner::window_ref_from_id(inner, &motion_notify.event).and_then(|w| w.upgrade())
                {
                    let x = fixed_point_to_float(motion_notify.event_x);
                    let y = fixed_point_to_float(motion_notify.event_y);

                    let window_size = { *window.size.lock().unwrap() };

                    callback(
                        Input {
                            event: InputEvent::PointerMotionAbsolute {
                                event: X11MouseMovedEvent {
                                    time: motion_notify.time,
                                    x,
                                    y,
                                    size: window_size,
                                    window: Arc::downgrade(&window),
                                },
                            },
                            window_id: Some(motion_notify.event),
                        },
                        &mut (),
                    )
                }
            }

            x11::Event::ConfigureNotify(configure_notify) => {
                if let Some(window) =
                    X11Inner::window_ref_from_id(inner, &configure_notify.window).and_then(|w| w.upgrade())
                {
                    let previous_size = { *window.size.lock().unwrap() };

                    let configure_notify_size: Size<u16, Logical> =
                        (configure_notify.width, configure_notify.height).into();

                    if configure_notify_size != previous_size {
                        {
                            let mut resize_guard = window.size.lock().unwrap();
                            *resize_guard = configure_notify_size;
                        }

                        (callback)(
                            X11Event::Resized {
                                new_size: configure_notify_size,
                                window_id: configure_notify.window,
                            },
                            &mut (),
                        );

                        if let Some(resize_sender) = window.resize.lock().unwrap().as_ref() {
                            let _ = resize_sender.send(configure_notify_size);
                        }
                    }
                }
            }

            x11::Event::XinputEnter(enter_notify) => {
                if let Some(window) =
                    X11Inner::window_ref_from_id(inner, &enter_notify.event).and_then(|w| w.upgrade())
                {
                    window.cursor_enter();
                }
            }

            x11::Event::XinputLeave(leave_notify) => {
                if let Some(window) =
                    X11Inner::window_ref_from_id(inner, &leave_notify.event).and_then(|w| w.upgrade())
                {
                    window.cursor_leave();
                }
            }

            x11::Event::ClientMessage(client_message) => {
                if let Some(window) =
                    X11Inner::window_ref_from_id(inner, &client_message.window).and_then(|w| w.upgrade())
                {
                    if client_message.data.as_data32()[0] == window.atoms.WM_DELETE_WINDOW
                    {
                        (callback)(
                            X11Event::CloseRequested {
                                window_id: client_message.window,
                            },
                            &mut (),
                        );
                    }
                }
            }

            x11::Event::Expose(expose) => {
                if expose.count == 0 {
                    (callback)(
                        X11Event::Refresh {
                            window_id: expose.window,
                        },
                        &mut (),
                    );
                }
            }

            x11::Event::PresentCompleteNotify(complete_notify) => {
                if let Some(window) =
                    X11Inner::window_ref_from_id(inner, &complete_notify.window).and_then(|w| w.upgrade())
                {
                    window.last_msc.store(complete_notify.msc, Ordering::SeqCst);

                    (callback)(
                        X11Event::PresentCompleted {
                            window_id: complete_notify.window,
                        },
                        &mut (),
                    );
                }
            }

            x11::Event::PresentIdleNotify(_) => {
            }

            x11::Event::Error(e) => {
                error!("X11 protocol error: {:?}", e);
            }

            _ => (),
        }
    }
}

fn fixed_point_to_float(value: i32) -> f64 {
    let int = value >> 16;
    let frac = value & 0xffff;
    int as f64 + frac as f64 / u16::MAX as f64
}

fn egl_init(_: &X11Inner) -> Result<(DrmNode, OwnedFd), EGLInitError> {
    let display = unsafe { EGLDisplay::new(X11DefaultDisplay)? };
    let device = EGLDevice::device_for_display(&display)?;
    let path = path_to_type(device.drm_device_path()?, NodeType::Render)?;
    let node = DrmNode::from_path(&path)
        .map_err(|err| match err {
            CreateDrmNodeError::Io(err) => err,
            _ => unreachable!(),
        })
        .map_err(EGLInitError::IO)?;
    let fd = rustix::fs::open(&path, OFlags::RDWR | OFlags::CLOEXEC, Mode::empty())
        .map_err(Into::<io::Error>::into)
        .map_err(EGLInitError::IO)?;
    Ok((node, fd))
}

fn dri3_init(x11: &X11Inner) -> Result<(DrmNode, OwnedFd), X11Error> {
    let connection = &x11.connection;

    let screen = &connection.setup().roots[x11.screen_number];
    let dri3 = match connection.dri3_open(screen.root, x11rb::NONE)?.reply() {
        Ok(reply) => reply,
        Err(err) => {
            return Err(if let ReplyError::X11Error(ref protocol_error) = err {
                match protocol_error.error_kind {
                    ErrorKind::Implementation => X11Error::CannotDirectRender,
                    ErrorKind::Match => X11Error::CannotDirectRender,
                    _ => err.into(),
                }
            } else {
                err.into()
            });
        }
    };
    let device_fd = dri3.device_fd;

    let dri_node = DrmNode::from_file(&device_fd).map_err(Into::<AllocateBuffersError>::into)?;
    if dri_node.ty() != NodeType::Render {
        match dri_node.node_with_type(NodeType::Render) {
            Some(Ok(node)) => {
                match node
                    .dev_path()
                    .map(|path| rustix::fs::open(path, OFlags::RDWR | OFlags::CLOEXEC, Mode::empty()))
                {
                    Some(Ok(fd)) => return Ok((node, fd)),
                    Some(Err(err)) => {
                        warn!("Could not create render node from existing DRM node ({:?}): {}, falling back to primary node", dri_node.dev_path().as_ref().map(|x| x.display()), err);
                    }
                    None => {
                        warn!("Could not create render node from existing DRM node ({:?}), falling back to primary node", dri_node.dev_path().as_ref().map(|x| x.display()));
                    }
                }
            }
            Some(Err(err)) => {
                warn!("Could not create render node from existing DRM node ({:?}): {}, falling back to primary node", dri_node.dev_path().as_ref().map(|x| x.display()), err);
            }
            None => {
                warn!(
                    "No render node available for DRM node ({:?}), falling back to primary node",
                    dri_node.dev_path().as_ref().map(|x| x.display())
                );
            }
        };
    }

    let fd_flags = rustix::io::fcntl_getfd(&device_fd).map_err(AllocateBuffersError::from)?;

    rustix::io::fcntl_setfd(&device_fd, fd_flags | rustix::io::FdFlags::CLOEXEC)
        .map_err(AllocateBuffersError::from)?;

    Ok((dri_node, device_fd))
}
