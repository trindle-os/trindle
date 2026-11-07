//! Device-wide synchronization helpers.

use std::{
    collections::HashMap,
    fmt,
    marker::PhantomData,
    os::fd::AsFd,
    sync::{Arc, Mutex, RwLock, TryLockError},
};

use drm::control::{self, connector, crtc, Mode};
use drm_fourcc::{DrmFormat, DrmFourcc, DrmModifier};

use crate::{
    backend::{
        allocator::{
            dmabuf::{AsDmabuf, Dmabuf},
            gbm::GbmDevice,
            Allocator,
        },
        renderer::{element::RenderElement, Bind, Color32F, DebugFlags, Renderer, RendererSuper, Texture},
    },
    output::OutputModeSource,
};

use super::{
    compositor::{
        DrmCompositor, FrameError, FrameFlags, FrameResult, PrimaryPlaneElement, RenderFrameError,
        RenderFrameErrorType, RenderFrameResult,
    },
    exporter::ExportFramebuffer,
    DrmDevice, DrmError, Planes,
};

type CompositorList<A, F, U, G> = Arc<RwLock<HashMap<crtc::Handle, Mutex<DrmCompositor<A, F, U, G>>>>>;

pub struct DrmOutputManager<A, F, U, G>
where
    A: Allocator,
    F: ExportFramebuffer<<A as Allocator>::Buffer>,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: fmt::Debug + 'static,
    G: AsFd + 'static,
{
    device: DrmDevice,
    allocator: A,
    exporter: F,
    gbm: Option<GbmDevice<G>>,
    compositor: CompositorList<A, F, U, G>,
    color_formats: Vec<DrmFourcc>,
    renderer_formats: Vec<DrmFormat>,
}

impl<A, F, U, G> fmt::Debug for DrmOutputManager<A, F, U, G>
where
    A: Allocator + fmt::Debug,
    <A as Allocator>::Buffer: fmt::Debug,
    F: ExportFramebuffer<<A as Allocator>::Buffer> + fmt::Debug,
    U: fmt::Debug,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: fmt::Debug + 'static,
    G: AsFd + fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DrmOutputManager")
            .field("device", &self.device)
            .field("allocator", &self.allocator)
            .field("exporter", &self.exporter)
            .field("gbm", &self.gbm)
            .field("compositor", &self.compositor)
            .field("color_formats", &self.color_formats)
            .field("renderer_formats", &self.renderer_formats)
            .finish()
    }
}

impl<A, F, U, G> DrmOutputManager<A, F, U, G>
where
    A: Allocator,
    F: ExportFramebuffer<<A as Allocator>::Buffer>,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: fmt::Debug + 'static,
    G: AsFd + 'static,
{
    pub fn device(&self) -> &DrmDevice {
        &self.device
    }

    pub fn device_mut(&mut self) -> &mut DrmDevice {
        &mut self.device
    }

    pub fn allocator(&self) -> &A {
        &self.allocator
    }
}

impl<A, F, U, G> DrmOutputManager<A, F, U, G>
where
    A: Allocator,
    F: ExportFramebuffer<<A as Allocator>::Buffer>,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: std::fmt::Debug + 'static,
    G: AsFd + 'static,
{
    pub fn pause(&mut self) {
        self.device.pause();
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DrmOutputManagerError<A, B, F, R>
where
    A: std::error::Error + Send + Sync + 'static,
    B: std::error::Error + Send + Sync + 'static,
    F: std::error::Error + Send + Sync + 'static,
    R: std::error::Error + Send + Sync + 'static,
{
    #[error("The specified CRTC {0:?} is already in use.")]
    DuplicateCrtc(crtc::Handle),
    #[error(transparent)]
    Drm(#[from] DrmError),
    #[error(transparent)]
    Frame(FrameError<A, B, F>),
    #[error(transparent)]
    RenderFrame(RenderFrameError<A, B, F, R>),
}

pub type DrmOutputManagerResult<U, A, F, R> = Result<
    U,
    DrmOutputManagerError<
        <A as Allocator>::Error,
        <<A as Allocator>::Buffer as AsDmabuf>::Error,
        <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Error,
        <R as RendererSuper>::Error,
    >,
>;

impl<A, F, U, G> DrmOutputManager<A, F, U, G>
where
    A: Allocator + std::clone::Clone + std::fmt::Debug,
    <A as Allocator>::Buffer: AsDmabuf,
    <A as Allocator>::Error: Send + Sync + 'static,
    <<A as crate::backend::allocator::Allocator>::Buffer as AsDmabuf>::Error:
        std::marker::Send + std::marker::Sync + 'static,
    F: ExportFramebuffer<<A as Allocator>::Buffer> + std::clone::Clone,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: std::fmt::Debug + 'static,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Error:
        std::marker::Send + std::marker::Sync + 'static,
    G: AsFd + std::clone::Clone + 'static,
    U: 'static,
{
    pub fn new(
        device: DrmDevice,
        allocator: A,
        exporter: F,
        gbm: Option<GbmDevice<G>>,
        color_formats: impl IntoIterator<Item = DrmFourcc>,
        renderer_formats: impl IntoIterator<Item = DrmFormat>,
    ) -> Self {
        Self {
            device,
            allocator,
            exporter,
            gbm,
            compositor: Default::default(),
            color_formats: color_formats.into_iter().collect(),
            renderer_formats: renderer_formats.into_iter().collect(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn initialize_output<R, E>(
        &mut self,
        crtc: crtc::Handle,
        mode: control::Mode,
        connectors: &[connector::Handle],
        output_mode_source: impl Into<OutputModeSource> + std::fmt::Debug,
        planes: Option<Planes>,
        renderer: &mut R,
        render_elements: &DrmOutputRenderElements<R, E>,
    ) -> DrmOutputManagerResult<DrmOutput<A, F, U, G>, A, F, R>
    where
        E: RenderElement<R>,
        R: Renderer + Bind<Dmabuf>,
        R::TextureId: Texture + 'static,
        R::Error: Send + Sync + 'static,
    {
        let output_mode_source = output_mode_source.into();

        let mut write_guard = self.compositor.write().unwrap();
        if write_guard.contains_key(&crtc) {
            return Err(DrmOutputManagerError::DuplicateCrtc(crtc));
        }

        let mut create_compositor =
            |implicit_modifiers: bool| -> FrameResult<DrmCompositor<A, F, U, G>, A, F> {
                let surface = self.device.create_surface(crtc, mode, connectors)?;

                if implicit_modifiers {
                    DrmCompositor::<A, F, U, G>::new(
                        output_mode_source.clone(),
                        surface,
                        planes.clone(),
                        self.allocator.clone(),
                        self.exporter.clone(),
                        self.color_formats.iter().copied(),
                        self.renderer_formats
                            .iter()
                            .filter(|f| f.modifier == DrmModifier::Invalid)
                            .copied(),
                        self.device.cursor_size(),
                        self.gbm.clone(),
                    )
                } else {
                    DrmCompositor::<A, F, U, G>::new(
                        output_mode_source.clone(),
                        surface,
                        planes.clone(),
                        self.allocator.clone(),
                        self.exporter.clone(),
                        self.color_formats.iter().copied(),
                        self.renderer_formats.iter().copied(),
                        self.device.cursor_size(),
                        self.gbm.clone(),
                    )
                }
            };

        let compositor = create_compositor(false);

        match compositor {
            Ok(compositor) => {
                write_guard.insert(crtc, Mutex::new(compositor));
            }
            Err(err) => {
                tracing::warn!(
                    ?crtc,
                    ?err,
                    "failed to initialize crtc, trying to lower bandwidth"
                );

                for compositor in write_guard.values_mut() {
                    let compositor = compositor.get_mut().unwrap();
                    if let Err(err) = render_elements.submit_composited_frame(&mut *compositor, renderer) {
                        if !matches!(err, DrmOutputManagerError::Frame(FrameError::EmptyFrame)) {
                            return Err(err);
                        }
                    }
                }

                let compositor = create_compositor(false);

                match compositor {
                    Ok(compositor) => {
                        write_guard.insert(crtc, Mutex::new(compositor));
                    }
                    Err(err) => {
                        tracing::warn!(
                            ?crtc,
                            ?err,
                            "failed to initialize crtc, trying implicit modifiers"
                        );

                        for compositor in write_guard.values_mut() {
                            let compositor = compositor.get_mut().unwrap();

                            let current_format = compositor.format();
                            if let Err(err) = compositor.set_format(
                                self.allocator.clone(),
                                current_format,
                                [DrmModifier::Invalid],
                            ) {
                                tracing::warn!(?err, "failed to set new format");
                                continue;
                            }

                            render_elements.submit_composited_frame(&mut *compositor, renderer)?;
                        }

                        let compositor = create_compositor(true);

                        match compositor {
                            Ok(compositor) => {
                                write_guard.insert(crtc, Mutex::new(compositor));
                            }
                            Err(err) => {
                                for compositor in write_guard.values_mut() {
                                    let compositor = compositor.get_mut().unwrap();

                                    let current_format = compositor.format();
                                    if let Err(err) = compositor.set_format(
                                        self.allocator.clone(),
                                        current_format,
                                        self.renderer_formats
                                            .iter()
                                            .filter(|f| f.code == current_format)
                                            .map(|f| f.modifier),
                                    ) {
                                        tracing::warn!(?err, "failed to reset format");
                                        continue;
                                    }

                                    render_elements.submit_composited_frame(&mut *compositor, renderer)?;
                                }

                                return Err(DrmOutputManagerError::Frame(err));
                            }
                        }
                    }
                }
            }
        };

        let compositor = write_guard.get_mut(&crtc).unwrap();
        let compositor = compositor.get_mut().unwrap();
        render_elements.submit_composited_frame(&mut *compositor, renderer)?;

        Ok(DrmOutput {
            compositor: self.compositor.clone(),
            crtc,
            allocator: self.allocator.clone(),
            renderer_formats: self.renderer_formats.clone(),
        })
    }

    pub fn with_compositors<R>(
        &mut self,
        f: impl FnOnce(&HashMap<crtc::Handle, Mutex<DrmCompositor<A, F, U, G>>>) -> R,
    ) -> R {
        let write_guard = self.compositor.write().unwrap();
        f(&*write_guard)
    }

    pub fn use_mode<R, E>(
        &mut self,
        crtc: &crtc::Handle,
        mode: Mode,
        renderer: &mut R,
        render_elements: &DrmOutputRenderElements<R, E>,
    ) -> DrmOutputManagerResult<(), A, F, R>
    where
        E: RenderElement<R>,
        R: Renderer + Bind<Dmabuf>,
        R::TextureId: Texture + 'static,
        R::Error: Send + Sync + 'static,
    {
        use_mode_internal(
            &self.compositor,
            crtc,
            mode,
            &self.allocator,
            &self.renderer_formats,
            renderer,
            render_elements,
        )
    }

    pub fn try_to_restore_modifiers<R, E>(
        &mut self,
        renderer: &mut R,
        render_elements: &DrmOutputRenderElements<R, E>,
    ) -> DrmOutputManagerResult<(), A, F, R>
    where
        E: RenderElement<R>,
        R: Renderer + Bind<Dmabuf>,
        R::TextureId: Texture + 'static,
        R::Error: Send + Sync + 'static,
    {
        let mut write_guard = self.compositor.write().unwrap();

        if write_guard
            .values_mut()
            .any(|c| c.get_mut().unwrap().modifiers() == [DrmModifier::Invalid])
        {
            for compositor in write_guard.values_mut() {
                let compositor = compositor.get_mut().unwrap();
                if let Err(err) = render_elements.submit_composited_frame(&mut *compositor, renderer) {
                    if !matches!(err, DrmOutputManagerError::Frame(FrameError::EmptyFrame)) {
                        return Err(err);
                    }
                }
            }

            for compositor in write_guard.values_mut() {
                let compositor = compositor.get_mut().unwrap();
                if compositor.modifiers() != [DrmModifier::Invalid] {
                    continue;
                }

                let current_format = compositor.format();
                if let Err(err) = compositor.set_format(
                    self.allocator.clone(),
                    current_format,
                    self.renderer_formats
                        .iter()
                        .filter(|f| f.code == current_format)
                        .map(|f| f.modifier),
                ) {
                    tracing::warn!(?err, "failed to reset format");
                    continue;
                }

                render_elements.submit_composited_frame(&mut *compositor, renderer)?;
            }
        }

        Ok(())
    }

    pub fn activate(&mut self, disable_connectors: bool) -> Result<(), DrmError> {
        self.device.activate(disable_connectors)?;

        let mut write_guard = self.compositor.write().unwrap();
        for compositor in write_guard.values_mut() {
            if let Err(err) = compositor.get_mut().unwrap().reset_state() {
                tracing::warn!("Failed to reset drm surface state: {}", err);
            }
        }

        Ok(())
    }
}

pub struct DrmOutput<A, F, U, G>
where
    A: Allocator,
    F: ExportFramebuffer<<A as Allocator>::Buffer>,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: std::fmt::Debug + 'static,
    G: AsFd + 'static,
{
    compositor: CompositorList<A, F, U, G>,
    crtc: crtc::Handle,
    allocator: A,
    renderer_formats: Vec<DrmFormat>,
}

impl<A, F, U, G> fmt::Debug for DrmOutput<A, F, U, G>
where
    A: Allocator + fmt::Debug,
    <A as Allocator>::Buffer: fmt::Debug,
    F: ExportFramebuffer<<A as Allocator>::Buffer> + fmt::Debug,
    U: fmt::Debug,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: fmt::Debug + 'static,
    G: AsFd + fmt::Debug + 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut d = f.debug_struct("DrmOutput");
        match self.compositor.try_read() {
            Ok(guard) => d.field("compositor", &guard.get(&self.crtc)),
            Err(TryLockError::Poisoned(err)) => d.field("compositor", &&**err.get_ref()),
            Err(TryLockError::WouldBlock) => d.field("compositor", &"<locked>"),
        };
        d.finish()
    }
}

impl<A, F, U, G> DrmOutput<A, F, U, G>
where
    A: Allocator + std::clone::Clone + fmt::Debug,
    <A as Allocator>::Buffer: AsDmabuf,
    <A as Allocator>::Error: Send + Sync + 'static,
    <<A as Allocator>::Buffer as AsDmabuf>::Error: std::marker::Send + std::marker::Sync + 'static,
    F: ExportFramebuffer<<A as Allocator>::Buffer> + std::clone::Clone,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: std::fmt::Debug + 'static,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Error:
        std::marker::Send + std::marker::Sync + 'static,
    G: AsFd + std::clone::Clone + 'static,
    U: 'static,
{
    pub fn set_debug_flags(&self, flags: DebugFlags) {
        self.with_compositor(|compositor| compositor.set_debug_flags(flags));
    }

    pub fn reset_buffers(&self) {
        self.with_compositor(|compositor| compositor.reset_buffers());
    }

    pub fn frame_submitted(&self) -> FrameResult<Option<U>, A, F> {
        self.with_compositor(|compositor| compositor.frame_submitted())
    }

    pub fn format(&self) -> DrmFourcc {
        self.with_compositor(|compositor| compositor.format())
    }

    pub fn render_frame<'a, R, E>(
        &mut self,
        renderer: &mut R,
        elements: &'a [E],
        clear_color: impl Into<Color32F>,
        frame_mode: FrameFlags,
    ) -> Result<RenderFrameResult<'a, A::Buffer, F::Framebuffer, E>, RenderFrameErrorType<A, F, R>>
    where
        E: RenderElement<R>,
        R: Renderer + Bind<Dmabuf>,
        R::TextureId: Texture + 'static,
        R::Error: Send + Sync + 'static,
    {
        self.with_compositor(|compositor| {
            compositor.render_frame(renderer, elements, clear_color, frame_mode)
        })
    }

    pub fn queue_frame(&mut self, user_data: U) -> FrameResult<(), A, F> {
        self.with_compositor(|compositor| compositor.queue_frame(user_data))
    }

    pub fn commit_frame(&mut self) -> FrameResult<(), A, F> {
        self.with_compositor(|compositor| compositor.commit_frame())
    }

    pub fn use_mode<R, E>(
        &mut self,
        mode: Mode,
        renderer: &mut R,
        render_elements: &DrmOutputRenderElements<R, E>,
    ) -> DrmOutputManagerResult<(), A, F, R>
    where
        E: RenderElement<R>,
        R: Renderer + Bind<Dmabuf>,
        R::TextureId: Texture + 'static,
        R::Error: Send + Sync + 'static,
    {
        use_mode_internal(
            &self.compositor,
            &self.crtc,
            mode,
            &self.allocator,
            &self.renderer_formats,
            renderer,
            render_elements,
        )
    }
}

impl<A, F, U, G> DrmOutput<A, F, U, G>
where
    A: Allocator,
    F: ExportFramebuffer<<A as Allocator>::Buffer>,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: std::fmt::Debug + 'static,
    G: AsFd + 'static,
{
    pub fn crtc(&self) -> crtc::Handle {
        self.crtc
    }

    pub fn with_compositor<T, R>(&self, f: T) -> R
    where
        T: FnOnce(&mut DrmCompositor<A, F, U, G>) -> R,
    {
        let read_guard = self.compositor.read().unwrap();
        let mut compositor_guard = read_guard.get(&self.crtc).unwrap().lock().unwrap();
        f(&mut compositor_guard)
    }
}

impl<A, F, U, G> Drop for DrmOutput<A, F, U, G>
where
    A: Allocator,
    F: ExportFramebuffer<<A as Allocator>::Buffer>,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: std::fmt::Debug + 'static,
    G: AsFd + 'static,
{
    fn drop(&mut self) {
        let mut write_guard = self.compositor.write().unwrap();
        write_guard.remove(&self.crtc);
    }
}

fn use_mode_internal<A, F, U, G, R, E>(
    compositor: &CompositorList<A, F, U, G>,
    crtc: &crtc::Handle,
    mode: Mode,
    allocator: &A,
    renderer_formats: &[DrmFormat],
    renderer: &mut R,
    render_elements: &DrmOutputRenderElements<R, E>,
) -> DrmOutputManagerResult<(), A, F, R>
where
    A: Allocator + std::clone::Clone + fmt::Debug,
    <A as Allocator>::Buffer: AsDmabuf,
    <A as Allocator>::Error: Send + Sync + 'static,
    <<A as crate::backend::allocator::Allocator>::Buffer as AsDmabuf>::Error:
        std::marker::Send + std::marker::Sync + 'static,
    F: ExportFramebuffer<<A as Allocator>::Buffer> + std::clone::Clone,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: std::fmt::Debug + 'static,
    <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Error:
        std::marker::Send + std::marker::Sync + 'static,
    G: AsFd + std::clone::Clone + 'static,
    U: 'static,
    E: RenderElement<R>,
    R: Renderer + Bind<Dmabuf>,
    R::TextureId: Texture + 'static,
    R::Error: Send + Sync + 'static,
{
    let mut write_guard = compositor.write().unwrap();

    let res = write_guard.get(crtc).unwrap().lock().unwrap().use_mode(mode);

    if let Err(err @ FrameError::DrmError(DrmError::TestFailed(_))) = res.as_ref() {
        tracing::warn!(?crtc, ?err, "failed to set mode, trying to lower bandwidth usage");

        for compositor in write_guard.values_mut() {
            let compositor = compositor.get_mut().unwrap();
            if let Err(err) = render_elements.submit_composited_frame(&mut *compositor, renderer) {
                if !matches!(err, DrmOutputManagerError::Frame(FrameError::EmptyFrame)) {
                    return Err(err);
                }
            }
        }

        let compositor = write_guard.get_mut(crtc).unwrap().get_mut().unwrap();
        match compositor.use_mode(mode) {
            Ok(_) => {
                if let Err(err) = render_elements.submit_composited_frame(&mut *compositor, renderer) {
                    if !matches!(err, DrmOutputManagerError::Frame(FrameError::EmptyFrame)) {
                        return Err(err);
                    }
                }
            }
            Err(err @ FrameError::DrmError(DrmError::TestFailed(_))) => {
                tracing::warn!(?crtc, ?err, "failed to set mode, trying implicit modifiers");

                for compositor in write_guard.values_mut() {
                    let compositor = compositor.get_mut().unwrap();

                    let current_format = compositor.format();
                    if let Err(err) =
                        compositor.set_format(allocator.clone(), current_format, [DrmModifier::Invalid])
                    {
                        tracing::warn!(?err, "failed to set new format");
                        continue;
                    }

                    render_elements.submit_composited_frame(&mut *compositor, renderer)?;
                }

                let compositor = write_guard.get_mut(crtc).unwrap().get_mut().unwrap();
                match compositor.use_mode(mode) {
                    Ok(_) => render_elements.submit_composited_frame(&mut *compositor, renderer)?,
                    Err(err) => {

                        for compositor in write_guard.values_mut() {
                            let compositor = compositor.get_mut().unwrap();

                            let current_format = compositor.format();
                            if let Err(err) = compositor.set_format(
                                allocator.clone(),
                                current_format,
                                renderer_formats
                                    .iter()
                                    .filter(|f| f.code == current_format)
                                    .map(|f| f.modifier),
                            ) {
                                tracing::warn!(?err, "failed to reset format");
                                continue;
                            }

                            render_elements.submit_composited_frame(&mut *compositor, renderer)?;
                        }

                        return Err(DrmOutputManagerError::Frame(err));
                    }
                }
            }
            Err(err) => return Err(DrmOutputManagerError::Frame(err)),
        };
    }

    Ok(())
}

#[derive(Debug)]
pub struct DrmOutputRenderElements<R, E>
where
    E: RenderElement<R>,
    R: Renderer + Bind<Dmabuf>,
    R::TextureId: Texture + 'static,
    R::Error: Send + Sync + 'static,
{
    render_elements: HashMap<crtc::Handle, (Vec<E>, Color32F)>,
    _renderer: PhantomData<R>,
}

impl<R, E> DrmOutputRenderElements<R, E>
where
    E: RenderElement<R>,
    R: Renderer + Bind<Dmabuf>,
    R::TextureId: Texture + 'static,
    R::Error: Send + Sync + 'static,
{
    pub fn new() -> Self {
        DrmOutputRenderElements {
            render_elements: HashMap::new(),
            _renderer: PhantomData,
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        DrmOutputRenderElements {
            render_elements: HashMap::with_capacity(cap),
            _renderer: PhantomData,
        }
    }
}

impl<R, E> Default for DrmOutputRenderElements<R, E>
where
    E: RenderElement<R>,
    R: Renderer + Bind<Dmabuf>,
    R::TextureId: Texture + 'static,
    R::Error: Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<R, E> DrmOutputRenderElements<R, E>
where
    E: RenderElement<R>,
    R: Renderer + Bind<Dmabuf>,
    R::TextureId: Texture + 'static,
    R::Error: Send + Sync + 'static,
{
    pub fn add_output(
        &mut self,
        crtc: &crtc::Handle,
        clear_color: Color32F,
        elements: impl IntoIterator<Item = E>,
    ) {
        self.render_elements
            .insert(*crtc, (elements.into_iter().collect(), clear_color));
    }

    fn submit_composited_frame<A, F, U, G>(
        &self,
        compositor: &mut DrmCompositor<A, F, U, G>,
        renderer: &mut R,
    ) -> DrmOutputManagerResult<(), A, F, R>
    where
        A: Allocator + std::clone::Clone + std::fmt::Debug,
        <A as Allocator>::Buffer: AsDmabuf,
        <A as Allocator>::Error: Send + Sync + 'static,
        <<A as crate::backend::allocator::Allocator>::Buffer as AsDmabuf>::Error:
            std::marker::Send + std::marker::Sync + 'static,
        F: ExportFramebuffer<<A as Allocator>::Buffer> + std::clone::Clone,
        <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Framebuffer: std::fmt::Debug + 'static,
        <F as ExportFramebuffer<<A as Allocator>::Buffer>>::Error:
            std::marker::Send + std::marker::Sync + 'static,
        G: AsFd + std::clone::Clone + 'static,
        U: 'static,
    {
        let (elements, clear_color) = self
            .render_elements
            .get(&compositor.crtc())
            .map(|(ref elements, ref color)| (&**elements, color))
            .unwrap_or((&[], &Color32F::BLACK));
        let frame_result = compositor
            .render_frame(renderer, elements, *clear_color, FrameFlags::empty())
            .map_err(DrmOutputManagerError::RenderFrame)?;
        if frame_result.needs_sync() {
            if let PrimaryPlaneElement::Swapchain(primary_swapchain_element) = frame_result.primary_element {
                let _ = primary_swapchain_element.sync.wait();
            }
        }
        compositor.commit_frame().map_err(DrmOutputManagerError::Frame)?;
        Ok(())
    }
}
