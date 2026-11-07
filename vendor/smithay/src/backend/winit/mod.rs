//! Implementation of backend traits for types provided by `winit`.

use std::io::Error as IoError;
use std::sync::Arc;
use std::time::Duration;

use calloop::generic::Generic;
use calloop::{EventSource, Interest, PostAction, Readiness, Token};
use tracing::{debug, info, info_span, instrument, trace, warn};
use wayland_egl as wegl;
use winit::platform::pump_events::PumpStatus;
use winit::platform::scancode::PhysicalKeyExtScancode;
use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::{ElementState, Touch, TouchPhase, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    platform::pump_events::EventLoopExtPumpEvents,
    window::{Window as WinitWindow, WindowAttributes, WindowId},
};

use crate::{
    backend::{
        egl::{
            context::{GlAttributes, PixelFormatRequirements},
            display::EGLDisplay,
            native, EGLContext, EGLSurface, Error as EGLError,
        },
        input::InputEvent,
        renderer::{
            gles::{GlesError, GlesRenderer},
            Bind,
        },
    },
    utils::{Clock, Monotonic, Physical, Rectangle, Size},
};

mod input;

pub use self::input::*;

pub fn init<R>() -> Result<(WinitGraphicsBackend<R>, WinitEventLoop), Error>
where
    R: From<GlesRenderer> + Bind<EGLSurface>,
    crate::backend::SwapBuffersError: From<R::Error>,
{
    init_from_attributes(
        WinitWindow::default_attributes()
            .with_inner_size(LogicalSize::new(1280.0, 800.0))
            .with_title("Smithay")
            .with_visible(true),
    )
}

pub fn init_from_attributes<R>(
    attributes: WindowAttributes,
) -> Result<(WinitGraphicsBackend<R>, WinitEventLoop), Error>
where
    R: From<GlesRenderer> + Bind<EGLSurface>,
    crate::backend::SwapBuffersError: From<R::Error>,
{
    init_from_attributes_with_gl_attr(
        attributes,
        GlAttributes {
            version: (3, 0),
            profile: None,
            debug: cfg!(debug_assertions),
            vsync: false,
        },
    )
}

pub fn init_from_attributes_with_gl_attr<R>(
    attributes: WindowAttributes,
    gl_attributes: GlAttributes,
) -> Result<(WinitGraphicsBackend<R>, WinitEventLoop), Error>
where
    R: From<GlesRenderer> + Bind<EGLSurface>,
    crate::backend::SwapBuffersError: From<R::Error>,
{
    let span = info_span!("backend_winit", window = tracing::field::Empty);
    let _guard = span.enter();
    info!("Initializing a winit backend");

    let event_loop = EventLoop::builder().build().map_err(Error::EventLoopCreation)?;

    #[allow(deprecated)]
    let window = Arc::new(
        event_loop
            .create_window(attributes)
            .map_err(Error::WindowCreation)?,
    );

    span.record("window", Into::<u64>::into(window.id()));
    debug!("Window created");

    let (display, context, surface, is_x11) = {
        let display = unsafe { EGLDisplay::new(window.clone())? };

        let context =
            EGLContext::new_with_config(&display, gl_attributes, PixelFormatRequirements::_10_bit())
                .or_else(|_| {
                    EGLContext::new_with_config(&display, gl_attributes, PixelFormatRequirements::_8_bit())
                })?;

        let (surface, is_x11) = match window.window_handle().map(|handle| handle.as_raw()) {
            Ok(RawWindowHandle::Wayland(handle)) => {
                debug!("Winit backend: Wayland");
                let size = window.inner_size();
                let surface = unsafe {
                    wegl::WlEglSurface::new_from_raw(
                        handle.surface.as_ptr() as *mut _,
                        size.width as i32,
                        size.height as i32,
                    )
                }
                .map_err(|err| Error::Surface(err.into()))?;
                unsafe {
                    (
                        EGLSurface::new(
                            &display,
                            context.pixel_format().unwrap(),
                            context.config_id(),
                            surface,
                        )
                        .map_err(EGLError::CreationFailed)?,
                        false,
                    )
                }
            }
            Ok(RawWindowHandle::Xlib(handle)) => {
                debug!("Winit backend: X11");
                unsafe {
                    (
                        EGLSurface::new(
                            &display,
                            context.pixel_format().unwrap(),
                            context.config_id(),
                            native::XlibWindow(handle.window),
                        )
                        .map_err(EGLError::CreationFailed)?,
                        true,
                    )
                }
            }
            _ => panic!("only running on Wayland or with Xlib is supported"),
        };

        let _ = context.unbind();
        (display, context, surface, is_x11)
    };

    let renderer = unsafe { GlesRenderer::new(context)?.into() };
    let damage_tracking = display.supports_damage();

    drop(_guard);

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let event_loop = Generic::new(event_loop, Interest::READ, calloop::Mode::Level);

    Ok((
        WinitGraphicsBackend {
            window: window.clone(),
            span: span.clone(),
            _display: display,
            egl_surface: surface,
            damage_tracking,
            bind_size: None,
            renderer,
        },
        WinitEventLoop {
            inner: WinitEventLoopInner {
                scale_factor: window.scale_factor(),
                clock: Clock::<Monotonic>::new(),
                key_counter: 0,
                window,
                is_x11,
            },
            fake_token: None,
            event_loop,
            pending_events: Vec::new(),
            span,
        },
    ))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to initialize an event loop")]
    EventLoopCreation(#[from] winit::error::EventLoopError),
    #[error("Failed to initialize a window")]
    WindowCreation(#[from] winit::error::OsError),
    #[error("Failed to create a surface for the window")]
    Surface(Box<dyn std::error::Error>),
    #[error("Context creation is not supported on the current window system")]
    NotSupported,
    #[error("EGL error: {0}")]
    Egl(#[from] EGLError),
    #[error("Renderer creation failed: {0}")]
    RendererCreationError(#[from] GlesError),
}

#[derive(Debug)]
pub struct WinitGraphicsBackend<R> {
    renderer: R,
    _display: EGLDisplay,
    egl_surface: EGLSurface,
    window: Arc<WinitWindow>,
    damage_tracking: bool,
    bind_size: Option<Size<i32, Physical>>,
    span: tracing::Span,
}

impl<R> WinitGraphicsBackend<R>
where
    R: Bind<EGLSurface>,
    crate::backend::SwapBuffersError: From<R::Error>,
{
    pub fn window_size(&self) -> Size<i32, Physical> {
        let (w, h): (i32, i32) = self.window.inner_size().into();
        (w, h).into()
    }

    pub fn scale_factor(&self) -> f64 {
        self.window.scale_factor()
    }

    pub fn window(&self) -> &WinitWindow {
        &self.window
    }

    pub fn renderer(&mut self) -> &mut R {
        &mut self.renderer
    }

    #[instrument(level = "trace", parent = &self.span, skip(self))]
    #[profiling::function]
    pub fn bind(&mut self) -> Result<(&mut R, R::Framebuffer<'_>), crate::backend::SwapBuffersError> {
        let window_size = self.window_size();
        if Some(window_size) != self.bind_size {
            self.egl_surface.resize(window_size.w, window_size.h, 0, 0);
        }
        self.bind_size = Some(window_size);

        let fb = self.renderer.bind(&mut self.egl_surface)?;

        Ok((&mut self.renderer, fb))
    }

    pub fn egl_surface(&self) -> &EGLSurface {
        &self.egl_surface
    }

    #[instrument(level = "trace", parent = &self.span, skip(self))]
    pub fn buffer_age(&self) -> Option<usize> {
        if self.damage_tracking {
            self.egl_surface.buffer_age().map(|x| x as usize)
        } else {
            Some(0)
        }
    }

    #[instrument(level = "trace", parent = &self.span, skip(self))]
    #[profiling::function]
    pub fn submit(
        &mut self,
        damage: Option<&[Rectangle<i32, Physical>]>,
    ) -> Result<(), crate::backend::SwapBuffersError> {
        let mut damage = match damage {
            Some(damage) if self.damage_tracking && !damage.is_empty() => {
                let bind_size = self
                    .bind_size
                    .expect("submitting without ever binding the renderer.");
                let damage = damage
                    .iter()
                    .map(|rect| {
                        Rectangle::new(
                            (rect.loc.x, bind_size.h - rect.loc.y - rect.size.h).into(),
                            rect.size,
                        )
                    })
                    .collect::<Vec<_>>();
                Some(damage)
            }
            _ => None,
        };

        self.window.pre_present_notify();
        self.egl_surface.swap_buffers(damage.as_deref_mut())?;
        Ok(())
    }
}

#[derive(Debug)]
struct WinitEventLoopInner {
    window: Arc<WinitWindow>,
    clock: Clock<Monotonic>,
    key_counter: u32,
    is_x11: bool,
    scale_factor: f64,
}

#[derive(Debug)]
pub struct WinitEventLoop {
    inner: WinitEventLoopInner,
    fake_token: Option<Token>,
    pending_events: Vec<WinitEvent>,
    event_loop: Generic<EventLoop<()>>,
    span: tracing::Span,
}

impl WinitEventLoop {
    #[instrument(level = "trace", parent = &self.span, skip_all)]
    #[profiling::function]
    pub fn dispatch_new_events<F>(&mut self, callback: F) -> PumpStatus
    where
        F: FnMut(WinitEvent),
    {
        // SAFETY: we don't drop event loop ourselves.
        let event_loop = unsafe { self.event_loop.get_mut() };

        event_loop.pump_app_events(
            Some(Duration::ZERO),
            &mut WinitEventLoopApp {
                inner: &mut self.inner,
                callback,
            },
        )
    }
}

struct WinitEventLoopApp<'a, F: FnMut(WinitEvent)> {
    inner: &'a mut WinitEventLoopInner,
    callback: F,
}

impl<F: FnMut(WinitEvent)> WinitEventLoopApp<'_, F> {
    fn timestamp(&self) -> u64 {
        self.inner.clock.now().as_micros()
    }
}

impl<F: FnMut(WinitEvent)> ApplicationHandler for WinitEventLoopApp<'_, F> {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        (self.callback)(WinitEvent::Input(InputEvent::DeviceAdded {
            device: WinitVirtualDevice,
        }));
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(size) => {
                trace!("Resizing window to {size:?}");
                let (w, h): (i32, i32) = size.into();

                (self.callback)(WinitEvent::Resized {
                    size: (w, h).into(),
                    scale_factor: self.inner.scale_factor,
                });
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor: new_scale_factor,
                ..
            } => {
                trace!("Scale factor changed to {new_scale_factor}");
                self.inner.scale_factor = new_scale_factor;
                let (w, h): (i32, i32) = self.inner.window.inner_size().into();
                (self.callback)(WinitEvent::Resized {
                    size: (w, h).into(),
                    scale_factor: self.inner.scale_factor,
                });
            }
            WindowEvent::RedrawRequested => {
                (self.callback)(WinitEvent::Redraw);
            }
            WindowEvent::CloseRequested => {
                (self.callback)(WinitEvent::CloseRequested);
            }
            WindowEvent::Focused(focused) => {
                (self.callback)(WinitEvent::Focus(focused));
            }
            WindowEvent::KeyboardInput {
                event, is_synthetic, ..
            } if !is_synthetic && !event.repeat => {
                match event.state {
                    ElementState::Pressed => self.inner.key_counter += 1,
                    ElementState::Released => {
                        self.inner.key_counter = self.inner.key_counter.saturating_sub(1);
                    }
                };

                let scancode = event.physical_key.to_scancode().unwrap_or(0);
                let event = InputEvent::Keyboard {
                    event: WinitKeyboardInputEvent {
                        time: self.timestamp(),
                        key: scancode,
                        count: self.inner.key_counter,
                        state: event.state,
                    },
                };
                (self.callback)(WinitEvent::Input(event));
            }
            WindowEvent::CursorMoved { position, .. } => {
                let size = self.inner.window.inner_size();
                let x = position.x / size.width as f64;
                let y = position.y / size.height as f64;
                let event = InputEvent::PointerMotionAbsolute {
                    event: WinitMouseMovedEvent {
                        time: self.timestamp(),
                        position: RelativePosition::new(x, y),
                        global_position: position,
                    },
                };
                (self.callback)(WinitEvent::Input(event));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let event = InputEvent::PointerAxis {
                    event: WinitMouseWheelEvent {
                        time: self.timestamp(),
                        delta,
                    },
                };
                (self.callback)(WinitEvent::Input(event));
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let event = InputEvent::PointerButton {
                    event: WinitMouseInputEvent {
                        time: self.timestamp(),
                        button,
                        state,
                        is_x11: self.inner.is_x11,
                    },
                };
                (self.callback)(WinitEvent::Input(event));
            }
            WindowEvent::Touch(Touch {
                phase: TouchPhase::Started,
                location,
                id,
                ..
            }) => {
                let size = self.inner.window.inner_size();
                let x = location.x / size.width as f64;
                let y = location.y / size.width as f64;
                let event = InputEvent::TouchDown {
                    event: WinitTouchStartedEvent {
                        time: self.timestamp(),
                        global_position: location,
                        position: RelativePosition::new(x, y),
                        id,
                    },
                };

                (self.callback)(WinitEvent::Input(event));
            }
            WindowEvent::Touch(Touch {
                phase: TouchPhase::Moved,
                location,
                id,
                ..
            }) => {
                let size = self.inner.window.inner_size();
                let x = location.x / size.width as f64;
                let y = location.y / size.width as f64;
                let event = InputEvent::TouchMotion {
                    event: WinitTouchMovedEvent {
                        time: self.timestamp(),
                        position: RelativePosition::new(x, y),
                        global_position: location,
                        id,
                    },
                };

                (self.callback)(WinitEvent::Input(event));
            }

            WindowEvent::Touch(Touch {
                phase: TouchPhase::Ended,
                location,
                id,
                ..
            }) => {
                let size = self.inner.window.inner_size();
                let x = location.x / size.width as f64;
                let y = location.y / size.width as f64;
                let event = InputEvent::TouchMotion {
                    event: WinitTouchMovedEvent {
                        time: self.timestamp(),
                        position: RelativePosition::new(x, y),
                        global_position: location,
                        id,
                    },
                };
                (self.callback)(WinitEvent::Input(event));

                let event = InputEvent::TouchUp {
                    event: WinitTouchEndedEvent {
                        time: self.timestamp(),
                        id,
                    },
                };

                (self.callback)(WinitEvent::Input(event));
            }

            WindowEvent::Touch(Touch {
                phase: TouchPhase::Cancelled,
                id,
                ..
            }) => {
                let event = InputEvent::TouchCancel {
                    event: WinitTouchCancelledEvent {
                        time: self.timestamp(),
                        id,
                    },
                };
                (self.callback)(WinitEvent::Input(event));
            }
            WindowEvent::DroppedFile(_)
            | WindowEvent::Destroyed
            | WindowEvent::CursorEntered { .. }
            | WindowEvent::AxisMotion { .. }
            | WindowEvent::CursorLeft { .. }
            | WindowEvent::ModifiersChanged(_)
            | WindowEvent::KeyboardInput { .. }
            | WindowEvent::HoveredFile(_)
            | WindowEvent::HoveredFileCancelled
            | WindowEvent::Ime(_)
            | WindowEvent::Moved(_)
            | WindowEvent::Occluded(_)
            | WindowEvent::DoubleTapGesture { .. }
            | WindowEvent::ThemeChanged(_)
            | WindowEvent::PinchGesture { .. }
            | WindowEvent::TouchpadPressure { .. }
            | WindowEvent::RotationGesture { .. }
            | WindowEvent::PanGesture { .. }
            | WindowEvent::ActivationTokenDone { .. } => (),
        }
    }
}

impl EventSource for WinitEventLoop {
    type Event = WinitEvent;
    type Metadata = ();
    type Ret = ();
    type Error = IoError;

    const NEEDS_EXTRA_LIFECYCLE_EVENTS: bool = true;

    fn before_sleep(&mut self) -> calloop::Result<Option<(calloop::Readiness, calloop::Token)>> {
        let mut pending_events = std::mem::take(&mut self.pending_events);
        let callback = |event| {
            pending_events.push(event);
        };
        self.dispatch_new_events(callback);
        self.pending_events = pending_events;
        if self.pending_events.is_empty() {
            Ok(None)
        } else {
            Ok(Some((Readiness::EMPTY, self.fake_token.unwrap())))
        }
    }

    fn process_events<F>(
        &mut self,
        _readiness: calloop::Readiness,
        _token: calloop::Token,
        mut callback: F,
    ) -> Result<PostAction, Self::Error>
    where
        F: FnMut(Self::Event, &mut Self::Metadata) -> Self::Ret,
    {
        let mut callback = |event| callback(event, &mut ());
        for event in self.pending_events.drain(..) {
            callback(event);
        }
        Ok(match self.dispatch_new_events(callback) {
            PumpStatus::Continue => PostAction::Continue,
            PumpStatus::Exit(_) => PostAction::Remove,
        })
    }

    fn register(
        &mut self,
        poll: &mut calloop::Poll,
        token_factory: &mut calloop::TokenFactory,
    ) -> calloop::Result<()> {
        self.fake_token = Some(token_factory.token());
        self.event_loop.register(poll, token_factory)
    }

    fn reregister(
        &mut self,
        poll: &mut calloop::Poll,
        token_factory: &mut calloop::TokenFactory,
    ) -> calloop::Result<()> {
        self.event_loop.register(poll, token_factory)
    }

    fn unregister(&mut self, poll: &mut calloop::Poll) -> calloop::Result<()> {
        self.event_loop.unregister(poll)
    }
}

#[derive(Debug)]
pub enum WinitEvent {
    Resized {
        size: Size<i32, Physical>,
        scale_factor: f64,
    },

    Focus(bool),

    Input(InputEvent<WinitInput>),

    CloseRequested,

    Redraw,
}
