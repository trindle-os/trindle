//! Rendering functionality and abstractions.

use crate::utils::{ids::id_gen, Buffer as BufferCoord, Physical, Point, Rectangle, Scale, Size, Transform};
use cgmath::Matrix3;
use std::{
    any::TypeId,
    cmp::Ordering,
    error::Error,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    sync::Arc,
};

#[cfg(feature = "wayland_frontend")]
use crate::wayland::{compositor::SurfaceData, shm::fourcc_to_shm_format};
#[cfg(feature = "wayland_frontend")]
use wayland_server::protocol::{wl_buffer, wl_shm};

#[cfg(feature = "renderer_gl")]
pub mod gles;

#[cfg(feature = "renderer_glow")]
pub mod glow;

#[cfg(feature = "renderer_pixman")]
pub mod pixman;

mod color;
pub use color::Color32F;

use crate::backend::allocator::{dmabuf::Dmabuf, Format, Fourcc};
#[cfg(all(
    feature = "wayland_frontend",
    feature = "backend_egl",
    feature = "use_system_lib"
))]
use crate::backend::egl::{
    display::{EGLBufferReader, BUFFER_READER},
    Error as EglError,
};

use super::allocator::format::FormatSet;

#[cfg(feature = "renderer_multi")]
pub mod multigpu;

pub mod utils;

pub mod element;

pub mod damage;

pub mod sync;

#[cfg(any(feature = "renderer_test", test, doctest))]
pub mod test;

pub struct ContextId<T: Texture>(Arc<InnerContextId>, PhantomData<fn() -> T>);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ErasedContextId(Arc<InnerContextId>, TypeId);

impl<T: Texture> ContextId<T> {
    pub fn new() -> Self {
        ContextId(Arc::new(InnerContextId::new()), PhantomData)
    }

    pub fn map<Tex: Texture>(self) -> ContextId<Tex> {
        ContextId(self.0, PhantomData)
    }

    pub fn erased(self) -> ErasedContextId
    where
        T: 'static,
    {
        ErasedContextId(self.0, TypeId::of::<T>())
    }
}

impl<T: Texture> Default for ContextId<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Texture> fmt::Debug for ContextId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ContextId")
            .field(&self.0)
            .field(&format_args!("_"))
            .finish()
    }
}

impl<T: Texture> Clone for ContextId<T> {
    fn clone(&self) -> Self {
        ContextId(self.0.clone(), PhantomData)
    }
}

impl<T: Texture> PartialEq for ContextId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Texture> Eq for ContextId<T> {}

impl<T: Texture> PartialOrd for ContextId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Texture> Ord for ContextId<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T: Texture> Hash for ContextId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

id_gen!(context_id);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct InnerContextId(usize);

impl InnerContextId {
    fn new() -> Self {
        Self(context_id::next())
    }
}

impl Drop for InnerContextId {
    fn drop(&mut self) {
        context_id::remove(self.0);
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum TextureFilter {
    Linear,
    Nearest,
}

impl Transform {
    #[inline]
    pub fn matrix(&self) -> Matrix3<f32> {
        match self {
            Transform::Normal => Matrix3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
            Transform::_90 => Matrix3::new(0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            Transform::_180 => Matrix3::new(-1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0),
            Transform::_270 => Matrix3::new(0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            Transform::Flipped => Matrix3::new(-1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
            Transform::Flipped90 => Matrix3::new(0.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
            Transform::Flipped180 => Matrix3::new(1.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 1.0),
            Transform::Flipped270 => Matrix3::new(0.0, -1.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0),
        }
    }
}

#[cfg(feature = "wayland_frontend")]
impl From<wayland_server::protocol::wl_output::Transform> for Transform {
    #[inline]
    fn from(transform: wayland_server::protocol::wl_output::Transform) -> Transform {
        use wayland_server::protocol::wl_output::Transform as WlTransform;
        match transform {
            WlTransform::Normal => Transform::Normal,
            WlTransform::_90 => Transform::_90,
            WlTransform::_180 => Transform::_180,
            WlTransform::_270 => Transform::_270,
            WlTransform::Flipped => Transform::Flipped,
            WlTransform::Flipped90 => Transform::Flipped90,
            WlTransform::Flipped180 => Transform::Flipped180,
            WlTransform::Flipped270 => Transform::Flipped270,
            _ => Transform::Normal,
        }
    }
}

pub trait Bind<Target>: Renderer {
    fn bind<'a>(&mut self, target: &'a mut Target) -> Result<Self::Framebuffer<'a>, Self::Error>;

    fn supported_formats(&self) -> Option<FormatSet> {
        None
    }
}

pub trait Texture: fmt::Debug {
    fn size(&self) -> Size<i32, BufferCoord> {
        Size::from((self.width() as i32, self.height() as i32))
    }

    fn width(&self) -> u32;
    fn height(&self) -> u32;

    fn format(&self) -> Option<Fourcc>;
}

pub trait TextureMapping: Texture {
    fn flipped(&self) -> bool;

    fn format(&self) -> Fourcc {
        Texture::format(self).expect("Texture Mappings need to have a format")
    }
}

pub trait Frame {
    type Error: Error;
    type TextureId: Texture;

    fn context_id(&self) -> ContextId<Self::TextureId>;

    fn clear(&mut self, color: Color32F, at: &[Rectangle<i32, Physical>]) -> Result<(), Self::Error>;

    fn draw_solid(
        &mut self,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        color: Color32F,
    ) -> Result<(), Self::Error>;

    #[allow(clippy::too_many_arguments)]
    fn render_texture_at(
        &mut self,
        texture: &Self::TextureId,
        pos: Point<i32, Physical>,
        texture_scale: i32,
        output_scale: impl Into<Scale<f64>>,
        src_transform: Transform,
        damage: &[Rectangle<i32, Physical>],
        opaque_regions: &[Rectangle<i32, Physical>],
        alpha: f32,
    ) -> Result<(), Self::Error> {
        self.render_texture_from_to(
            texture,
            Rectangle::from_size(texture.size()).to_f64(),
            Rectangle::new(
                pos,
                texture
                    .size()
                    .to_logical(texture_scale, src_transform)
                    .to_physical_precise_round(output_scale),
            ),
            damage,
            opaque_regions,
            src_transform,
            alpha,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn render_texture_from_to(
        &mut self,
        texture: &Self::TextureId,
        src: Rectangle<f64, BufferCoord>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        opaque_regions: &[Rectangle<i32, Physical>],
        src_transform: Transform,
        alpha: f32,
    ) -> Result<(), Self::Error>;

    fn transformation(&self) -> Transform;

    fn wait(&mut self, sync: &sync::SyncPoint) -> Result<(), Self::Error>;

    fn finish(self) -> Result<sync::SyncPoint, Self::Error>;
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct DebugFlags: u32 {
        const TINT = 0b00000001;
    }
}

pub trait RendererSuper: fmt::Debug {
    type Error: Error;
    type TextureId: Texture;
    type Framebuffer<'buffer>: Texture;
    type Frame<'frame, 'buffer>: Frame<Error = Self::Error, TextureId = Self::TextureId>
    where
        'buffer: 'frame,
        Self: 'frame;
}

pub trait Renderer: RendererSuper {
    fn context_id(&self) -> ContextId<Self::TextureId>;

    fn downscale_filter(&mut self, filter: TextureFilter) -> Result<(), Self::Error>;
    fn upscale_filter(&mut self, filter: TextureFilter) -> Result<(), Self::Error>;

    fn set_debug_flags(&mut self, flags: DebugFlags);
    fn debug_flags(&self) -> DebugFlags;

    fn render<'frame, 'buffer>(
        &'frame mut self,
        framebuffer: &'frame mut Self::Framebuffer<'buffer>,
        output_size: Size<i32, Physical>,
        dst_transform: Transform,
    ) -> Result<Self::Frame<'frame, 'buffer>, Self::Error>
    where
        'buffer: 'frame;

    fn wait(&mut self, sync: &sync::SyncPoint) -> Result<(), Self::Error>;

    fn cleanup_texture_cache(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

pub trait Offscreen<Target>: Renderer + Bind<Target> {
    fn create_buffer(&mut self, format: Fourcc, size: Size<i32, BufferCoord>) -> Result<Target, Self::Error>;
}

#[cfg(feature = "wayland_frontend")]
pub trait ImportMemWl: ImportMem {
    fn import_shm_buffer(
        &mut self,
        buffer: &wl_buffer::WlBuffer,
        surface: Option<&crate::wayland::compositor::SurfaceData>,
        damage: &[Rectangle<i32, BufferCoord>],
    ) -> Result<Self::TextureId, Self::Error>;

    fn shm_formats(&self) -> Box<dyn Iterator<Item = wl_shm::Format>> {
        Box::new(self.mem_formats().flat_map(fourcc_to_shm_format))
    }
}

pub trait ImportMem: Renderer {
    fn import_memory(
        &mut self,
        data: &[u8],
        format: Fourcc,
        size: Size<i32, BufferCoord>,
        flipped: bool,
    ) -> Result<Self::TextureId, Self::Error>;

    fn update_memory(
        &mut self,
        texture: &Self::TextureId,
        data: &[u8],
        region: Rectangle<i32, BufferCoord>,
    ) -> Result<(), Self::Error>;

    fn mem_formats(&self) -> Box<dyn Iterator<Item = Fourcc>>;
}

#[cfg(all(
    feature = "wayland_frontend",
    feature = "backend_egl",
    feature = "use_system_lib"
))]
pub trait ImportEgl: Renderer {
    fn bind_wl_display(&mut self, display: &wayland_server::DisplayHandle) -> Result<(), EglError>;

    fn unbind_wl_display(&mut self);

    fn egl_reader(&self) -> Option<&EGLBufferReader>;

    fn import_egl_buffer(
        &mut self,
        buffer: &wl_buffer::WlBuffer,
        surface: Option<&crate::wayland::compositor::SurfaceData>,
        damage: &[Rectangle<i32, BufferCoord>],
    ) -> Result<Self::TextureId, Self::Error>;
}

#[cfg(feature = "wayland_frontend")]
pub trait ImportDmaWl: ImportDma {
    fn import_dma_buffer(
        &mut self,
        buffer: &wl_buffer::WlBuffer,
        _surface: Option<&crate::wayland::compositor::SurfaceData>,
        damage: &[Rectangle<i32, BufferCoord>],
    ) -> Result<Self::TextureId, Self::Error> {
        let dmabuf = crate::wayland::dmabuf::get_dmabuf(buffer)
            .expect("import_dma_buffer without checking buffer type?");
        self.import_dmabuf(dmabuf, Some(damage))
    }
}

pub trait ImportDma: Renderer {
    fn dmabuf_formats(&self) -> FormatSet {
        FormatSet::default()
    }

    fn has_dmabuf_format(&self, format: Format) -> bool {
        self.dmabuf_formats().contains(&format)
    }

    fn import_dmabuf(
        &mut self,
        dmabuf: &Dmabuf,
        damage: Option<&[Rectangle<i32, BufferCoord>]>,
    ) -> Result<Self::TextureId, Self::Error>;
}

#[cfg(feature = "wayland_frontend")]
pub trait ImportAll: Renderer {
    fn import_buffer(
        &mut self,
        buffer: &wl_buffer::WlBuffer,
        surface: Option<&crate::wayland::compositor::SurfaceData>,
        damage: &[Rectangle<i32, BufferCoord>],
    ) -> Option<Result<Self::TextureId, Self::Error>>;
}

#[cfg(all(
    feature = "wayland_frontend",
    feature = "backend_egl",
    feature = "use_system_lib"
))]
impl<R: Renderer + ImportMemWl + ImportEgl + ImportDmaWl> ImportAll for R {
    #[profiling::function]
    fn import_buffer(
        &mut self,
        buffer: &wl_buffer::WlBuffer,
        surface: Option<&SurfaceData>,
        damage: &[Rectangle<i32, BufferCoord>],
    ) -> Option<Result<Self::TextureId, Self::Error>> {
        match buffer_type(buffer) {
            Some(BufferType::Shm) => Some(self.import_shm_buffer(buffer, surface, damage)),
            Some(BufferType::Egl) => Some(self.import_egl_buffer(buffer, surface, damage)),
            Some(BufferType::Dma) => Some(self.import_dma_buffer(buffer, surface, damage)),
            _ => None,
        }
    }
}

#[cfg(all(
    feature = "wayland_frontend",
    not(all(feature = "backend_egl", feature = "use_system_lib"))
))]
impl<R: Renderer + ImportMemWl + ImportDmaWl> ImportAll for R {
    fn import_buffer(
        &mut self,
        buffer: &wl_buffer::WlBuffer,
        surface: Option<&SurfaceData>,
        damage: &[Rectangle<i32, BufferCoord>],
    ) -> Option<Result<Self::TextureId, Self::Error>> {
        match buffer_type(buffer) {
            Some(BufferType::Shm) => Some(self.import_shm_buffer(buffer, surface, damage)),
            Some(BufferType::Dma) => Some(self.import_dma_buffer(buffer, surface, damage)),
            _ => None,
        }
    }
}

pub trait ExportMem: Renderer {
    type TextureMapping: TextureMapping;

    fn copy_framebuffer(
        &mut self,
        target: &Self::Framebuffer<'_>,
        region: Rectangle<i32, BufferCoord>,
        format: Fourcc,
    ) -> Result<Self::TextureMapping, Self::Error>;

    fn copy_texture(
        &mut self,
        texture: &Self::TextureId,
        region: Rectangle<i32, BufferCoord>,
        format: Fourcc,
    ) -> Result<Self::TextureMapping, Self::Error>;

    fn can_read_texture(&mut self, texture: &Self::TextureId) -> Result<bool, Self::Error>;

    fn map_texture<'a>(&mut self, texture_mapping: &'a Self::TextureMapping)
        -> Result<&'a [u8], Self::Error>;
}

pub trait Blit
where
    Self: Renderer,
{
    fn blit(
        &mut self,
        from: &Self::Framebuffer<'_>,
        to: &mut Self::Framebuffer<'_>,
        src: Rectangle<i32, Physical>,
        dst: Rectangle<i32, Physical>,
        filter: TextureFilter,
    ) -> Result<(), Self::Error>;
}

pub trait BlitFrame<Framebuffer>
where
    Self: Frame,
{
    fn blit_to(
        &mut self,
        to: &mut Framebuffer,
        src: Rectangle<i32, Physical>,
        dst: Rectangle<i32, Physical>,
        filter: TextureFilter,
    ) -> Result<(), Self::Error>;

    fn blit_from(
        &mut self,
        from: &Framebuffer,
        src: Rectangle<i32, Physical>,
        dst: Rectangle<i32, Physical>,
        filter: TextureFilter,
    ) -> Result<(), Self::Error>;
}

#[cfg(feature = "wayland_frontend")]
#[non_exhaustive]
#[derive(Debug)]
pub enum BufferType {
    Shm,
    #[cfg(all(feature = "backend_egl", feature = "use_system_lib"))]
    Egl,
    Dma,
    SinglePixel,
}

#[cfg(feature = "wayland_frontend")]
pub fn buffer_type(buffer: &wl_buffer::WlBuffer) -> Option<BufferType> {
    use crate::wayland::shm::BufferAccessError;

    if crate::wayland::dmabuf::get_dmabuf(buffer).is_ok() {
        return Some(BufferType::Dma);
    }

    if !matches!(
        crate::wayland::shm::with_buffer_contents(buffer, |_, _, _| ()),
        Err(BufferAccessError::NotManaged)
    ) {
        return Some(BufferType::Shm);
    }

    if crate::wayland::single_pixel_buffer::get_single_pixel_buffer(buffer).is_ok() {
        return Some(BufferType::SinglePixel);
    }

    #[cfg(all(feature = "backend_egl", feature = "use_system_lib"))]
    if BUFFER_READER
        .lock()
        .unwrap()
        .as_ref()
        .and_then(|x| x.upgrade())
        .and_then(|x| x.egl_buffer_dimensions(buffer))
        .is_some()
    {
        return Some(BufferType::Egl);
    }

    None
}

#[cfg(feature = "wayland_frontend")]
pub fn buffer_has_alpha(buffer: &wl_buffer::WlBuffer) -> Option<bool> {
    use super::allocator::format::has_alpha;
    use crate::wayland::shm::shm_format_to_fourcc;

    if let Ok(dmabuf) = crate::wayland::dmabuf::get_dmabuf(buffer) {
        return Some(crate::backend::allocator::format::has_alpha(dmabuf.0.format));
    }

    if let Ok(has_alpha) = crate::wayland::shm::with_buffer_contents(buffer, |_, _, data| {
        shm_format_to_fourcc(data.format).is_some_and(has_alpha)
    }) {
        return Some(has_alpha);
    }

    if let Ok(spb) = crate::wayland::single_pixel_buffer::get_single_pixel_buffer(buffer) {
        return Some(spb.has_alpha());
    }

    #[cfg(all(feature = "backend_egl", feature = "use_system_lib"))]
    if let Some(format) = BUFFER_READER
        .lock()
        .unwrap()
        .as_ref()
        .and_then(|x| x.upgrade())
        .and_then(|x| x.egl_buffer_contents(buffer).ok())
        .map(|b| b.format)
    {
        return Some(crate::backend::egl::display::EGLBufferReader::egl_buffer_has_alpha(format));
    }

    None
}

#[cfg(feature = "wayland_frontend")]
pub fn buffer_dimensions(buffer: &wl_buffer::WlBuffer) -> Option<Size<i32, BufferCoord>> {
    use crate::{
        backend::allocator::Buffer,
        wayland::shm::{self, BufferAccessError},
    };

    if let Ok(buf) = crate::wayland::dmabuf::get_dmabuf(buffer) {
        return Some((buf.width() as i32, buf.height() as i32).into());
    }

    if crate::wayland::single_pixel_buffer::get_single_pixel_buffer(buffer).is_ok() {
        return Some(Size::from((1, 1)));
    }

    match shm::with_buffer_contents(buffer, |_, _, data| (data.width, data.height).into()) {
        Ok(data) => Some(data),

        Err(BufferAccessError::NotManaged) => {
            #[cfg(all(feature = "backend_egl", feature = "use_system_lib"))]
            if let Some(dim) = BUFFER_READER
                .lock()
                .unwrap()
                .as_ref()
                .and_then(|x| x.upgrade())
                .and_then(|x| x.egl_buffer_dimensions(buffer))
            {
                return Some(dim);
            }

            None
        }

        Err(_) => None,
    }
}

#[cfg(feature = "wayland_frontend")]
#[profiling::function]
pub fn buffer_y_inverted(buffer: &wl_buffer::WlBuffer) -> Option<bool> {
    if let Ok(dmabuf) = crate::wayland::dmabuf::get_dmabuf(buffer) {
        return Some(dmabuf.y_inverted());
    }

    #[cfg(all(feature = "backend_egl", feature = "use_system_lib"))]
    if let Some(Ok(egl_buffer)) = BUFFER_READER
        .lock()
        .unwrap()
        .as_ref()
        .and_then(|x| x.upgrade())
        .map(|x| x.egl_buffer_contents(buffer))
    {
        return Some(egl_buffer.y_inverted);
    }

    None
}
