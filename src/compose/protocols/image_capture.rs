//! Sharing a screen or one window: ext-image-copy-capture, with the screens
//! and windows it can copy named through ext-image-capture-source. Trindle's
//! portal uses it to send the screen to a browser or a video call, and any
//! other recorder may use it too.

use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use smithay::backend::allocator::dmabuf::Dmabuf;
use smithay::backend::allocator::Fourcc;
use smithay::backend::renderer::damage::OutputDamageTracker;
use smithay::backend::renderer::element::RenderElement;
use smithay::backend::renderer::gles::GlesTexture;
use smithay::backend::renderer::sync::SyncPoint;
use smithay::backend::renderer::{Bind, Color32F, ExportMem, Offscreen, Renderer, Texture};
use smithay::output::{Output, OutputModeSource, WeakOutput};
use smithay::reexports::calloop::generic::Generic;
use smithay::reexports::calloop::{Interest, LoopHandle, Mode, PostAction};
use smithay::reexports::wayland_protocols::ext::image_capture_source::v1::server::{
    ext_foreign_toplevel_image_capture_source_manager_v1::{self, ExtForeignToplevelImageCaptureSourceManagerV1},
    ext_image_capture_source_v1::{self, ExtImageCaptureSourceV1},
    ext_output_image_capture_source_manager_v1::{self, ExtOutputImageCaptureSourceManagerV1},
};
use smithay::reexports::wayland_protocols::ext::image_copy_capture::v1::server::{
    ext_image_copy_capture_cursor_session_v1::{self, ExtImageCopyCaptureCursorSessionV1},
    ext_image_copy_capture_frame_v1::{self, ExtImageCopyCaptureFrameV1, FailureReason},
    ext_image_copy_capture_manager_v1::{self, ExtImageCopyCaptureManagerV1},
    ext_image_copy_capture_session_v1::{self, ExtImageCopyCaptureSessionV1},
};
use smithay::reexports::wayland_server::backend::ClientId;
use smithay::reexports::wayland_server::protocol::{wl_buffer::WlBuffer, wl_output, wl_shm};
use smithay::reexports::wayland_server::{
    Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource,
};
use smithay::utils::{Physical, Size, Transform};
use smithay::wayland::foreign_toplevel_list::{ForeignToplevelHandle, ForeignToplevelWeakHandle};
use tracing::warn;

use super::fill::{damage_in_buffer, Canvas};
use crate::in_use::Recorders;

const VERSION: u32 = 1;

const SHM_FORMATS: [wl_shm::Format; 2] = [wl_shm::Format::Xrgb8888, wl_shm::Format::Argb8888];

pub enum CaptureSource {
    Output(WeakOutput),
    Toplevel(Option<ForeignToplevelWeakHandle>),
}

#[derive(Clone, Debug)]
pub struct DmabufTarget {
    pub device: u64,
    pub formats: Vec<(Fourcc, Vec<u64>)>,
}

pub fn dmabuf_target<'a>(
    device: u64,
    formats: impl Iterator<Item = &'a smithay::backend::allocator::Format>,
) -> Option<DmabufTarget> {
    let mut out: Vec<(Fourcc, Vec<u64>)> = Vec::new();
    for format in formats {
        if !matches!(format.code, Fourcc::Xrgb8888 | Fourcc::Argb8888) {
            continue;
        }
        let modifier = u64::from(format.modifier);
        match out.iter_mut().find(|(code, _)| *code == format.code) {
            Some((_, modifiers)) => modifiers.push(modifier),
            None => out.push((format.code, vec![modifier])),
        }
    }
    (!out.is_empty()).then_some(DmabufTarget { device, formats: out })
}

#[derive(Clone)]
enum Target {
    Output(Output),
    Toplevel(ForeignToplevelHandle),
}

impl Target {
    fn view(&self) -> CaptureTarget<'_> {
        match self {
            Target::Output(o) => CaptureTarget::Output(o),
            Target::Toplevel(h) => CaptureTarget::Toplevel(h),
        }
    }
}

pub enum CaptureTarget<'a> {
    Output(&'a Output),
    Toplevel(&'a ForeignToplevelHandle),
}

pub struct PointerView<E> {
    pub over: bool,
    pub position: (i32, i32),
    pub hotspot: (i32, i32),
    pub elements: Vec<E>,
    pub scale: f64,
}

pub const POINTER_SIZE: i32 = 128;

struct Pointer {
    target: Option<Target>,
    over: bool,
    position: Option<(i32, i32)>,
    hotspot: Option<(i32, i32)>,

    image: Option<ExtImageCopyCaptureSessionV1>,
}

struct Session {
    target: Target,

    pointer: bool,
    paint_cursors: bool,
    pid: Option<u32>,

    size: Size<i32, Physical>,

    tracker: OutputDamageTracker,

    frame: Option<ExtImageCopyCaptureFrameV1>,

    waiting: Option<Capture>,
    stopped: bool,
}

impl Session {
    fn stop(&mut self, session: &ExtImageCopyCaptureSessionV1) {
        if self.stopped {
            return;
        }
        self.stopped = true;
        if let Some(mut capture) = self.waiting.take() {
            capture.fail(FailureReason::Stopped);
        }
        if session.is_alive() {
            session.stopped();
        }
    }
}

#[derive(Default)]
pub struct ImageCaptureState {
    sessions: HashMap<ExtImageCopyCaptureSessionV1, Session>,
    pointers: HashMap<ExtImageCopyCaptureCursorSessionV1, Pointer>,

    recorders: Recorders,
    dmabuf: Option<DmabufTarget>,
}

impl fmt::Debug for ImageCaptureState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImageCaptureState").field("sessions", &self.sessions.len()).finish()
    }
}

pub struct ImageCaptureGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

pub trait ImageCaptureHandler {
    fn image_capture_state(&mut self) -> &mut ImageCaptureState;

    fn toplevel_capture_size(&self, handle: &ForeignToplevelHandle) -> Option<Size<i32, Physical>>;
}

fn output_size(output: &Output) -> Option<Size<i32, Physical>> {
    output.current_mode().map(|m| output.current_transform().transform_size(m.size))
}

impl ImageCaptureState {

    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> Self
    where
        D: GlobalDispatch<ExtImageCopyCaptureManagerV1, ImageCaptureGlobalData>
            + GlobalDispatch<ExtOutputImageCaptureSourceManagerV1, ImageCaptureGlobalData>
            + GlobalDispatch<ExtForeignToplevelImageCaptureSourceManagerV1, ImageCaptureGlobalData>
            + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + Clone + 'static,
    {
        let data = |f: &F| ImageCaptureGlobalData { filter: Box::new(f.clone()) };
        display.create_global::<D, ExtImageCopyCaptureManagerV1, _>(VERSION, data(&filter));
        display.create_global::<D, ExtOutputImageCaptureSourceManagerV1, _>(VERSION, data(&filter));
        display.create_global::<D, ExtForeignToplevelImageCaptureSourceManagerV1, _>(VERSION, data(&filter));
        Self::default()
    }

    pub fn set_dmabuf(&mut self, target: Option<DmabufTarget>) {
        self.dmabuf = target;
    }

    pub fn has_output_frames(&self, output: &Output) -> bool {
        self.sessions.values().any(|s| !s.pointer && s.waiting.is_some() && matches!(&s.target, Target::Output(o) if o == output))
    }

    pub fn has_toplevel_frames(&self) -> bool {
        self.sessions.values().any(|s| !s.pointer && s.waiting.is_some() && matches!(s.target, Target::Toplevel(_)))
    }

    pub fn has_pointers(&self) -> bool {
        !self.pointers.is_empty()
    }

    pub fn service_pointers<R, E, F, T>(
        &mut self,
        renderer: &mut R,
        mut view: F,
        now: Duration,
        event_loop: &LoopHandle<'static, T>,
    ) where
        R: Renderer + Offscreen<GlesTexture> + Bind<Dmabuf> + ExportMem,
        R::TextureId: Texture + 'static,
        E: RenderElement<R>,
        F: FnMut(&mut R, CaptureTarget<'_>) -> Option<PointerView<E>>,
        T: 'static,
    {
        for (resource, pointer) in self.pointers.iter_mut() {
            let Some(target) = &pointer.target else { continue };
            let seen = view(renderer, target.view());
            let over = seen.as_ref().is_some_and(|v| v.over);
            if over != pointer.over {
                pointer.over = over;
                if over {
                    resource.enter();
                } else {
                    resource.leave();
                    pointer.position = None;
                    pointer.hotspot = None;
                }
            }
            let Some(seen) = seen else { continue };
            if over {
                if pointer.position != Some(seen.position) {
                    pointer.position = Some(seen.position);
                    resource.position(seen.position.0, seen.position.1);
                }
                if pointer.hotspot != Some(seen.hotspot) {
                    pointer.hotspot = Some(seen.hotspot);
                    resource.hotspot(seen.hotspot.0, seen.hotspot.1);
                }
            }
            let Some(image) = &pointer.image else { continue };
            let Some(session) = self.sessions.get_mut(image) else { continue };
            if session.waiting.is_some() {
                fill(renderer, session, &seen.elements, Color32F::TRANSPARENT, seen.scale, Transform::Normal, true, now, event_loop);
            }
        }
    }

    pub fn remove_output(&mut self, output: &Output) {
        for (resource, session) in self.sessions.iter_mut() {
            if matches!(&session.target, Target::Output(o) if o == output) {
                session.stop(resource);
            }
        }
        for (resource, pointer) in self.pointers.iter_mut() {
            if matches!(&pointer.target, Some(Target::Output(o)) if o == output) {
                if pointer.over {
                    resource.leave();
                }
                pointer.target = None;
            }
        }
    }

    pub fn refresh(&mut self) {
        for (resource, session) in self.sessions.iter_mut() {
            if let Target::Toplevel(handle) = &session.target {
                if handle.is_closed() {
                    session.stop(resource);
                }
            }
        }
        for (resource, pointer) in self.pointers.iter_mut() {
            if matches!(&pointer.target, Some(Target::Toplevel(h)) if h.is_closed()) {
                if pointer.over {
                    resource.leave();
                }
                pointer.target = None;
            }
        }
        self.sessions.retain(|resource, _| resource.is_alive());
        self.pointers.retain(|resource, _| resource.is_alive());
    }

    pub fn recording(&mut self) -> Vec<u32> {
        let now = Instant::now();
        for session in self.sessions.values() {
            if let (true, Some(pid)) = (session.waiting.is_some() && !session.stopped, session.pid) {
                self.recorders.note(pid, now);
            }
        }
        self.recorders.recording(now)
    }

    pub fn service_output<R, E, F, T>(
        &mut self,
        renderer: &mut R,
        output: &Output,
        mut elements_for: F,
        now: Duration,
        event_loop: &LoopHandle<'static, T>,
    ) where
        R: Renderer + Offscreen<GlesTexture> + Bind<Dmabuf> + ExportMem,
        R::TextureId: Texture + 'static,
        E: RenderElement<R>,
        F: FnMut(&mut R, bool) -> (Vec<E>, Color32F),
        T: 'static,
    {
        if !self.has_output_frames(output) {
            return;
        }
        let Some(size) = output_size(output) else { return };
        let scale = output.current_scale().fractional_scale();

        let transform = Transform::Normal;
        let mut scenes: [Option<(Vec<E>, Color32F)>; 2] = [None, None];
        let dmabuf = self.dmabuf.clone();
        for (resource, session) in self.sessions.iter_mut() {
            if session.pointer || session.waiting.is_none() || !matches!(&session.target, Target::Output(o) if o == output) {
                continue;
            }
            if resized(resource, session, size, &dmabuf) {
                continue;
            }
            let (scene, clear) = scenes[usize::from(session.paint_cursors)]
                .get_or_insert_with(|| elements_for(renderer, session.paint_cursors));
            let clear = *clear;
            fill(renderer, session, scene, clear, scale, transform, false, now, event_loop);
        }
    }

    pub fn service_toplevels<R, E, F, T>(
        &mut self,
        renderer: &mut R,
        mut elements_for: F,
        now: Duration,
        event_loop: &LoopHandle<'static, T>,
    ) where
        R: Renderer + Offscreen<GlesTexture> + Bind<Dmabuf> + ExportMem,
        R::TextureId: Texture + 'static,
        E: RenderElement<R>,
        F: FnMut(&mut R, &ForeignToplevelHandle, bool) -> Option<(Vec<E>, Size<i32, Physical>, f64)>,
        T: 'static,
    {
        if !self.has_toplevel_frames() {
            return;
        }
        let dmabuf = self.dmabuf.clone();
        for (resource, session) in self.sessions.iter_mut() {
            let Target::Toplevel(handle) = &session.target else { continue };
            if session.pointer || session.waiting.is_none() {
                continue;
            }
            let handle = handle.clone();
            let Some((elements, size, scale)) = elements_for(renderer, &handle, session.paint_cursors) else {
                continue;
            };
            if resized(resource, session, size, &dmabuf) {
                continue;
            }
            fill(renderer, session, &elements, Color32F::TRANSPARENT, scale, Transform::Normal, false, now, event_loop);
        }
    }
}

fn resized(resource: &ExtImageCopyCaptureSessionV1, session: &mut Session, size: Size<i32, Physical>,
           dmabuf: &Option<DmabufTarget>) -> bool {
    if size == session.size {
        return false;
    }
    session.size = size;
    session.tracker = OutputDamageTracker::new((0, 0), 1.0, Transform::Normal);
    if let Some(mut capture) = session.waiting.take() {
        capture.fail(FailureReason::BufferConstraints);
    }
    send_constraints(resource, size, dmabuf);
    true
}

#[allow(clippy::too_many_arguments)]
fn fill<R, E, T>(
    renderer: &mut R,
    session: &mut Session,
    elements: &[E],
    clear: Color32F,
    scale: f64,
    transform: Transform,
    see_through: bool,
    now: Duration,
    event_loop: &LoopHandle<'static, T>,
) where
    R: Renderer + Offscreen<GlesTexture> + Bind<Dmabuf> + ExportMem,
    R::TextureId: Texture + 'static,
    E: RenderElement<R>,
    T: 'static,
{
    let size = session.size;
    let same_mode = matches!(
        session.tracker.mode(),
        OutputModeSource::Static { size: s, scale: sc, transform: t }
            if *s == size && sc.x == scale && sc.y == scale && *t == transform
    );
    if !same_mode {
        session.tracker = OutputDamageTracker::new(size, scale, transform);
    }
    let damage = match session.tracker.damage_output(1, elements) {
        Ok((Some(damage), _)) if !damage.is_empty() => damage_in_buffer(damage, size, transform),

        _ => return,
    };
    let Some(mut capture) = session.waiting.take() else { return };
    let Some(buffer) = capture.buffer.take() else { return };
    match buffer.paint(renderer, size, elements, clear, scale, transform, see_through) {
        Ok(sync) => {
            capture.damage = damage;
            capture.transform = transform;
            capture.submit_after_sync(sync, now, event_loop);
        }
        Err(err) => {
            warn!("image capture: could not copy: {err}");
            session.tracker = OutputDamageTracker::new((0, 0), 1.0, Transform::Normal);
            capture.fail(FailureReason::Unknown);
        }
    }
}

fn send_constraints(session: &ExtImageCopyCaptureSessionV1, size: Size<i32, Physical>, dmabuf: &Option<DmabufTarget>) {
    session.buffer_size(size.w.max(1) as u32, size.h.max(1) as u32);
    for format in SHM_FORMATS {
        session.shm_format(format);
    }
    if let Some(target) = dmabuf {
        session.dmabuf_device(target.device.to_ne_bytes().to_vec());
        for (fourcc, modifiers) in &target.formats {
            let bytes: Vec<u8> = modifiers.iter().flat_map(|m| m.to_ne_bytes()).collect();
            session.dmabuf_format(*fourcc as u32, bytes);
        }
    }
    session.done();
}

struct Capture {
    frame: ExtImageCopyCaptureFrameV1,
    buffer: Option<Canvas>,
    damage: Vec<smithay::utils::Rectangle<i32, smithay::utils::Buffer>>,
    transform: Transform,
    done: bool,
}

impl Drop for Capture {
    fn drop(&mut self) {
        if !self.done {
            self.fail(FailureReason::Unknown);
        }
    }
}

impl Capture {
    fn fail(&mut self, reason: FailureReason) {
        self.done = true;
        if self.frame.is_alive() {
            self.frame.failed(reason);
        }
    }

    fn submit(mut self, now: Duration) {
        self.done = true;
        if !self.frame.is_alive() {
            return;
        }
        self.frame.transform(wl_output::Transform::from(self.transform));
        for rect in &self.damage {
            self.frame.damage(rect.loc.x, rect.loc.y, rect.size.w, rect.size.h);
        }
        let secs = now.as_secs();
        self.frame.presentation_time((secs >> 32) as u32, (secs & 0xFFFF_FFFF) as u32, now.subsec_nanos());
        self.frame.ready();
    }

    fn submit_after_sync<T: 'static>(self, sync: Option<SyncPoint>, now: Duration, event_loop: &LoopHandle<'static, T>) {
        match sync.and_then(|sync| sync.export()) {
            None => self.submit(now),
            Some(fence) => {
                let mut capture = Some(self);
                let source = Generic::new(fence, Interest::READ, Mode::OneShot);
                let inserted = event_loop.insert_source(source, move |_, _, _| {
                    if let Some(capture) = capture.take() {
                        capture.submit(now);
                    }
                    Ok(PostAction::Remove)
                });
                if let Err(err) = inserted {
                    warn!("image capture: could not wait for the drawing to finish: {}", err.error);
                }
            }
        }
    }
}

pub struct FrameData {
    session: ExtImageCopyCaptureSessionV1,
    buffer: Mutex<Option<WlBuffer>>,
    captured: AtomicBool,
}

pub struct SessionData;

impl<D> GlobalDispatch<ExtImageCopyCaptureManagerV1, ImageCaptureGlobalData, D> for ImageCaptureState
where
    D: Dispatch<ExtImageCopyCaptureManagerV1, ()> + 'static,
{
    fn bind(_: &mut D, _: &DisplayHandle, _: &Client, manager: New<ExtImageCopyCaptureManagerV1>,
            _: &ImageCaptureGlobalData, data_init: &mut DataInit<'_, D>) {
        data_init.init(manager, ());
    }

    fn can_view(client: Client, data: &ImageCaptureGlobalData) -> bool {
        (data.filter)(&client)
    }
}

impl<D> GlobalDispatch<ExtOutputImageCaptureSourceManagerV1, ImageCaptureGlobalData, D> for ImageCaptureState
where
    D: Dispatch<ExtOutputImageCaptureSourceManagerV1, ()> + 'static,
{
    fn bind(_: &mut D, _: &DisplayHandle, _: &Client, manager: New<ExtOutputImageCaptureSourceManagerV1>,
            _: &ImageCaptureGlobalData, data_init: &mut DataInit<'_, D>) {
        data_init.init(manager, ());
    }

    fn can_view(client: Client, data: &ImageCaptureGlobalData) -> bool {
        (data.filter)(&client)
    }
}

impl<D> GlobalDispatch<ExtForeignToplevelImageCaptureSourceManagerV1, ImageCaptureGlobalData, D> for ImageCaptureState
where
    D: Dispatch<ExtForeignToplevelImageCaptureSourceManagerV1, ()> + 'static,
{
    fn bind(_: &mut D, _: &DisplayHandle, _: &Client, manager: New<ExtForeignToplevelImageCaptureSourceManagerV1>,
            _: &ImageCaptureGlobalData, data_init: &mut DataInit<'_, D>) {
        data_init.init(manager, ());
    }

    fn can_view(client: Client, data: &ImageCaptureGlobalData) -> bool {
        (data.filter)(&client)
    }
}

impl<D> Dispatch<ExtOutputImageCaptureSourceManagerV1, (), D> for ImageCaptureState
where
    D: Dispatch<ExtImageCaptureSourceV1, CaptureSource> + 'static,
{
    fn request(_: &mut D, _: &Client, _: &ExtOutputImageCaptureSourceManagerV1,
               request: ext_output_image_capture_source_manager_v1::Request, _: &(), _: &DisplayHandle,
               data_init: &mut DataInit<'_, D>) {
        if let ext_output_image_capture_source_manager_v1::Request::CreateSource { source, output } = request {
            let weak = Output::from_resource(&output).map(|o| o.downgrade()).unwrap_or_default();
            data_init.init(source, CaptureSource::Output(weak));
        }
    }
}

impl<D> Dispatch<ExtForeignToplevelImageCaptureSourceManagerV1, (), D> for ImageCaptureState
where
    D: Dispatch<ExtImageCaptureSourceV1, CaptureSource> + 'static,
{
    fn request(_: &mut D, _: &Client, _: &ExtForeignToplevelImageCaptureSourceManagerV1,
               request: ext_foreign_toplevel_image_capture_source_manager_v1::Request, _: &(), _: &DisplayHandle,
               data_init: &mut DataInit<'_, D>) {
        if let ext_foreign_toplevel_image_capture_source_manager_v1::Request::CreateSource { source, toplevel_handle } = request {
            let weak = ForeignToplevelHandle::from_resource(&toplevel_handle).map(|h| h.downgrade());
            data_init.init(source, CaptureSource::Toplevel(weak));
        }
    }
}

impl<D> Dispatch<ExtImageCaptureSourceV1, CaptureSource, D> for ImageCaptureState
where
    D: 'static,
{
    fn request(_: &mut D, _: &Client, _: &ExtImageCaptureSourceV1, _: ext_image_capture_source_v1::Request,
               _: &CaptureSource, _: &DisplayHandle, _: &mut DataInit<'_, D>) {}
}

impl<D> Dispatch<ExtImageCopyCaptureManagerV1, (), D> for ImageCaptureState
where
    D: Dispatch<ExtImageCopyCaptureSessionV1, SessionData>
        + Dispatch<ExtImageCopyCaptureCursorSessionV1, ()>
        + ImageCaptureHandler
        + 'static,
{
    fn request(state: &mut D, client: &Client, manager: &ExtImageCopyCaptureManagerV1,
               request: ext_image_copy_capture_manager_v1::Request, _: &(), dh: &DisplayHandle,
               data_init: &mut DataInit<'_, D>) {
        match request {
            ext_image_copy_capture_manager_v1::Request::CreateSession { session, source, options } => {
                let paint_cursors = match options {
                    smithay::reexports::wayland_server::WEnum::Value(o) => {
                        o.contains(ext_image_copy_capture_manager_v1::Options::PaintCursors)
                    }
                    smithay::reexports::wayland_server::WEnum::Unknown(_) => {
                        manager.post_error(ext_image_copy_capture_manager_v1::Error::InvalidOption, "unknown option");
                        return;
                    }
                };
                let session = data_init.init(session, SessionData);
                let target = match source.data::<CaptureSource>() {
                    Some(CaptureSource::Output(weak)) => weak.upgrade().map(Target::Output),
                    Some(CaptureSource::Toplevel(weak)) => weak.as_ref().and_then(|w| w.upgrade()).filter(|h| !h.is_closed()).map(Target::Toplevel),
                    None => None,
                };
                let size = match &target {
                    Some(Target::Output(o)) => output_size(o),
                    Some(Target::Toplevel(h)) => state.toplevel_capture_size(h),
                    None => None,
                };
                let (Some(target), Some(size)) = (target, size) else {
                    session.stopped();
                    return;
                };
                let pid = client.get_credentials(dh).ok().and_then(|c| u32::try_from(c.pid).ok());
                let captures = state.image_capture_state();
                send_constraints(&session, size, &captures.dmabuf);
                captures.sessions.insert(session, Session {
                    target,
                    pointer: false,
                    paint_cursors,
                    pid,
                    size,
                    tracker: OutputDamageTracker::new((0, 0), 1.0, Transform::Normal),
                    frame: None,
                    waiting: None,
                    stopped: false,
                });
            }
            ext_image_copy_capture_manager_v1::Request::CreatePointerCursorSession { session, source, .. } => {

                let session = data_init.init(session, ());
                let target = match source.data::<CaptureSource>() {
                    Some(CaptureSource::Output(weak)) => weak.upgrade().map(Target::Output),
                    Some(CaptureSource::Toplevel(weak)) => weak.as_ref().and_then(|w| w.upgrade()).filter(|h| !h.is_closed()).map(Target::Toplevel),
                    None => None,
                };
                state.image_capture_state().pointers.insert(session, Pointer {
                    target, over: false, position: None, hotspot: None, image: None,
                });
            }
            _ => {}
        }
    }
}

impl<D> Dispatch<ExtImageCopyCaptureCursorSessionV1, (), D> for ImageCaptureState
where
    D: Dispatch<ExtImageCopyCaptureSessionV1, SessionData> + ImageCaptureHandler + 'static,
{
    fn request(state: &mut D, client: &Client, resource: &ExtImageCopyCaptureCursorSessionV1,
               request: ext_image_copy_capture_cursor_session_v1::Request, _: &(), dh: &DisplayHandle,
               data_init: &mut DataInit<'_, D>) {
        if let ext_image_copy_capture_cursor_session_v1::Request::GetCaptureSession { session } = request {
            let captures = state.image_capture_state();
            let Some(pointer) = captures.pointers.get_mut(resource) else { return };
            if pointer.image.is_some() {
                resource.post_error(ext_image_copy_capture_cursor_session_v1::Error::DuplicateSession, "already asked");
                return;
            }
            let session = data_init.init(session, SessionData);
            let Some(target) = pointer.target.clone() else {
                session.stopped();
                return;
            };
            pointer.image = Some(session.clone());

            let size = Size::from((POINTER_SIZE, POINTER_SIZE));
            send_constraints(&session, size, &None);
            let pid = client.get_credentials(dh).ok().and_then(|c| u32::try_from(c.pid).ok());
            captures.sessions.insert(session, Session {
                target,
                pointer: true,
                paint_cursors: false,
                pid,
                size,
                tracker: OutputDamageTracker::new((0, 0), 1.0, Transform::Normal),
                frame: None,
                waiting: None,
                stopped: false,
            });
        }
    }

    fn destroyed(state: &mut D, _: ClientId, resource: &ExtImageCopyCaptureCursorSessionV1, _: &()) {
        let captures = state.image_capture_state();

        if let Some(image) = captures.pointers.remove(resource).and_then(|p| p.image) {
            if let Some(session) = captures.sessions.get_mut(&image) {
                session.stop(&image);
            }
        }
    }
}

impl<D> Dispatch<ExtImageCopyCaptureSessionV1, SessionData, D> for ImageCaptureState
where
    D: Dispatch<ExtImageCopyCaptureFrameV1, FrameData> + ImageCaptureHandler + 'static,
{
    fn request(state: &mut D, _: &Client, resource: &ExtImageCopyCaptureSessionV1,
               request: ext_image_copy_capture_session_v1::Request, _: &SessionData, _: &DisplayHandle,
               data_init: &mut DataInit<'_, D>) {
        if let ext_image_copy_capture_session_v1::Request::CreateFrame { frame } = request {
            #[allow(clippy::mutable_key_type)]
            let sessions = &mut state.image_capture_state().sessions;
            let open = sessions.get(resource).and_then(|s| s.frame.as_ref()).is_some_and(|f| f.is_alive());
            if open {
                resource.post_error(ext_image_copy_capture_session_v1::Error::DuplicateFrame, "a frame is already open");
                return;
            }
            let frame = data_init.init(frame, FrameData {
                session: resource.clone(),
                buffer: Mutex::new(None),
                captured: AtomicBool::new(false),
            });
            match sessions.get_mut(resource) {
                Some(session) => session.frame = Some(frame),

                None => frame.failed(FailureReason::Stopped),
            }
        }
    }

    fn destroyed(state: &mut D, _: ClientId, resource: &ExtImageCopyCaptureSessionV1, _: &SessionData) {
        if let Some(mut session) = state.image_capture_state().sessions.remove(resource) {
            if let Some(mut capture) = session.waiting.take() {
                capture.fail(FailureReason::Stopped);
            }
        }
    }
}

impl<D> Dispatch<ExtImageCopyCaptureFrameV1, FrameData, D> for ImageCaptureState
where
    D: ImageCaptureHandler + 'static,
{
    fn request(state: &mut D, _: &Client, frame: &ExtImageCopyCaptureFrameV1,
               request: ext_image_copy_capture_frame_v1::Request, data: &FrameData, _: &DisplayHandle,
               _: &mut DataInit<'_, D>) {
        match request {
            ext_image_copy_capture_frame_v1::Request::AttachBuffer { buffer } => {
                if let Ok(mut held) = data.buffer.lock() {
                    *held = Some(buffer);
                }
            }
            ext_image_copy_capture_frame_v1::Request::DamageBuffer { x, y, width, height } => {

                if x < 0 || y < 0 || width <= 0 || height <= 0 {
                    frame.post_error(ext_image_copy_capture_frame_v1::Error::InvalidBufferDamage, "invalid damage");
                }
            }
            ext_image_copy_capture_frame_v1::Request::Capture => {
                if data.captured.swap(true, Ordering::SeqCst) {
                    frame.post_error(ext_image_copy_capture_frame_v1::Error::AlreadyCaptured, "already captured");
                    return;
                }
                let Some(buffer) = data.buffer.lock().ok().and_then(|mut b| b.take()) else {
                    frame.post_error(ext_image_copy_capture_frame_v1::Error::NoBuffer, "no buffer attached");
                    return;
                };
                let captures = state.image_capture_state();
                let Some(session) = captures.sessions.get_mut(&data.session) else {
                    frame.failed(FailureReason::Stopped);
                    return;
                };
                if session.stopped {
                    frame.failed(FailureReason::Stopped);
                    return;
                }
                let Some(buffer) = Canvas::accept(buffer, session.size) else {
                    frame.failed(FailureReason::BufferConstraints);
                    return;
                };
                if let Some(pid) = session.pid {
                    captures.recorders.note(pid, Instant::now());
                }
                session.waiting = Some(Capture {
                    frame: frame.clone(),
                    buffer: Some(buffer),
                    damage: Vec::new(),
                    transform: Transform::Normal,
                    done: false,
                });
            }
            _ => {}
        }
    }

    fn destroyed(state: &mut D, _: ClientId, frame: &ExtImageCopyCaptureFrameV1, data: &FrameData) {
        if let Some(session) = state.image_capture_state().sessions.get_mut(&data.session) {
            if session.waiting.as_ref().is_some_and(|c| c.frame == *frame) {
                if let Some(mut capture) = session.waiting.take() {
                    capture.done = true;
                }
            }
            if session.frame.as_ref() == Some(frame) {
                session.frame = None;
            }
        }
    }
}

macro_rules! delegate_image_capture {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols::ext::image_copy_capture::v1::server::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1:
                $crate::protocols::image_capture::ImageCaptureGlobalData
        ] => $crate::protocols::image_capture::ImageCaptureState);
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols::ext::image_capture_source::v1::server::ext_output_image_capture_source_manager_v1::ExtOutputImageCaptureSourceManagerV1:
                $crate::protocols::image_capture::ImageCaptureGlobalData
        ] => $crate::protocols::image_capture::ImageCaptureState);
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols::ext::image_capture_source::v1::server::ext_foreign_toplevel_image_capture_source_manager_v1::ExtForeignToplevelImageCaptureSourceManagerV1:
                $crate::protocols::image_capture::ImageCaptureGlobalData
        ] => $crate::protocols::image_capture::ImageCaptureState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols::ext::image_copy_capture::v1::server::ext_image_copy_capture_manager_v1::ExtImageCopyCaptureManagerV1: ()
        ] => $crate::protocols::image_capture::ImageCaptureState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols::ext::image_capture_source::v1::server::ext_output_image_capture_source_manager_v1::ExtOutputImageCaptureSourceManagerV1: ()
        ] => $crate::protocols::image_capture::ImageCaptureState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols::ext::image_capture_source::v1::server::ext_foreign_toplevel_image_capture_source_manager_v1::ExtForeignToplevelImageCaptureSourceManagerV1: ()
        ] => $crate::protocols::image_capture::ImageCaptureState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols::ext::image_capture_source::v1::server::ext_image_capture_source_v1::ExtImageCaptureSourceV1:
                $crate::protocols::image_capture::CaptureSource
        ] => $crate::protocols::image_capture::ImageCaptureState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols::ext::image_copy_capture::v1::server::ext_image_copy_capture_session_v1::ExtImageCopyCaptureSessionV1:
                $crate::protocols::image_capture::SessionData
        ] => $crate::protocols::image_capture::ImageCaptureState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols::ext::image_copy_capture::v1::server::ext_image_copy_capture_cursor_session_v1::ExtImageCopyCaptureCursorSessionV1: ()
        ] => $crate::protocols::image_capture::ImageCaptureState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            smithay::reexports::wayland_protocols::ext::image_copy_capture::v1::server::ext_image_copy_capture_frame_v1::ExtImageCopyCaptureFrameV1:
                $crate::protocols::image_capture::FrameData
        ] => $crate::protocols::image_capture::ImageCaptureState);
    };
}
pub(crate) use delegate_image_capture;
