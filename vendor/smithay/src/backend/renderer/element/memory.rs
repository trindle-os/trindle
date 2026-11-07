//! Element to render from RGBA memory.

use std::{
    any::Any,
    collections::{hash_map::Entry, HashMap},
    sync::{Arc, Mutex, MutexGuard},
};

use tracing::{instrument, trace, warn};

use crate::{
    backend::{
        allocator::{format::get_bpp, Fourcc},
        renderer::{
            utils::{CommitCounter, DamageBag, DamageSet, DamageSnapshot, OpaqueRegions},
            ErasedContextId, Frame, ImportMem, Renderer,
        },
    },
    utils::{Buffer, Logical, Physical, Point, Rectangle, Scale, Size, Transform},
};

use super::{Element, Id, Kind, RenderElement, UnderlyingStorage};

#[derive(Debug, Clone)]
pub struct MemoryBuffer {
    mem: Arc<Vec<u8>>,
    format: Fourcc,
    size: Size<i32, Buffer>,
    stride: i32,
}

impl Default for MemoryBuffer {
    fn default() -> Self {
        Self {
            mem: Default::default(),
            format: Fourcc::Abgr8888,
            size: Default::default(),
            stride: Default::default(),
        }
    }
}

impl MemoryBuffer {
    pub fn new(format: Fourcc, size: impl Into<Size<i32, Buffer>>) -> Self {
        let size = size.into();
        let stride = size.w * (get_bpp(format).expect("Format with unknown bits per pixel") / 8) as i32;
        let mem = vec![0; (stride * size.h) as usize];
        Self {
            mem: Arc::new(mem),
            format,
            size,
            stride,
        }
    }

    pub fn from_slice(mem: &[u8], format: Fourcc, size: impl Into<Size<i32, Buffer>>) -> Self {
        let size = size.into();
        let stride = size.w * (get_bpp(format).expect("Format with unknown bits per pixel") / 8) as i32;
        assert!(mem.len() >= (stride * size.h) as usize);
        Self {
            mem: Arc::new(mem.to_vec()),
            format,
            size,
            stride,
        }
    }

    pub fn size(&self) -> Size<i32, Buffer> {
        self.size
    }

    pub fn format(&self) -> Fourcc {
        self.format
    }

    pub fn stride(&self) -> i32 {
        self.stride
    }

    pub fn resize(&mut self, size: impl Into<Size<i32, Buffer>>) -> bool {
        self.size = size.into();
        self.stride =
            self.size.w * (get_bpp(self.format).expect("Format with unknown bits per pixel") / 8) as i32;
        let mem_size = (self.stride * self.size.h) as usize;
        if self.mem.len() != mem_size {
            let mem = Arc::make_mut(&mut self.mem);
            mem.resize(mem_size, 0);
            true
        } else {
            false
        }
    }
}

impl std::ops::Deref for MemoryBuffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.mem
    }
}

impl std::ops::DerefMut for MemoryBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *Arc::make_mut(&mut self.mem)
    }
}

#[derive(Debug)]
struct MemoryRenderBufferInner {
    mem: MemoryBuffer,
    scale: i32,
    transform: Transform,
    opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    damage_bag: DamageBag<i32, Buffer>,
    textures: HashMap<ErasedContextId, Box<dyn Any + Send>>,
    renderer_seen: HashMap<ErasedContextId, CommitCounter>,
}

impl Default for MemoryRenderBufferInner {
    fn default() -> Self {
        MemoryRenderBufferInner {
            mem: Default::default(),
            scale: 1,
            transform: Transform::Normal,
            opaque_regions: None,
            damage_bag: DamageBag::default(),
            textures: HashMap::default(),
            renderer_seen: HashMap::default(),
        }
    }
}

impl MemoryRenderBufferInner {
    fn new(
        format: Fourcc,
        size: impl Into<Size<i32, Buffer>>,
        scale: i32,
        transform: Transform,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    ) -> Self {
        MemoryRenderBufferInner {
            mem: MemoryBuffer::new(format, size),
            scale,
            transform,
            opaque_regions,
            damage_bag: DamageBag::default(),
            textures: HashMap::default(),
            renderer_seen: HashMap::default(),
        }
    }

    fn from_memory(
        mem: MemoryBuffer,
        scale: i32,
        transform: Transform,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    ) -> Self {
        MemoryRenderBufferInner {
            mem,
            scale,
            transform,
            opaque_regions,
            damage_bag: DamageBag::default(),
            textures: HashMap::default(),
            renderer_seen: HashMap::default(),
        }
    }

    fn from_slice(
        mem: &[u8],
        format: Fourcc,
        size: impl Into<Size<i32, Buffer>>,
        scale: i32,
        transform: Transform,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    ) -> Self {
        Self::from_memory(
            MemoryBuffer::from_slice(mem, format, size),
            scale,
            transform,
            opaque_regions,
        )
    }

    fn resize(&mut self, size: impl Into<Size<i32, Buffer>>) {
        if self.mem.resize(size) {
            self.renderer_seen.clear();
            self.textures.clear();
            self.damage_bag.reset();
            self.opaque_regions = None;
        }
    }

    #[instrument(level = "trace", skip(renderer))]
    #[profiling::function]
    fn import_texture<R>(&mut self, renderer: &mut R) -> Result<R::TextureId, R::Error>
    where
        R: Renderer + ImportMem,
        R::TextureId: Send + Clone + 'static,
    {
        let context_id = renderer.context_id().erased();
        let current_commit = self.damage_bag.current_commit();
        let last_commit = self.renderer_seen.get(&context_id).copied();
        let buffer_damage = self
            .damage_bag
            .damage_since(last_commit)
            .map(|d| d.into_iter().reduce(|a, b| a.merge(b)).unwrap_or_default())
            .unwrap_or_else(|| Rectangle::from_size(self.mem.size()));

        let tex = match self.textures.entry(context_id.clone()) {
            Entry::Occupied(entry) => {
                let tex = entry.get().downcast_ref().unwrap();
                if !buffer_damage.is_empty() {
                    trace!("updating memory with damage {:#?}", &buffer_damage);
                    renderer.update_memory(tex, &self.mem, buffer_damage)?
                }
                tex.clone()
            }
            Entry::Vacant(entry) => {
                trace!("importing memory");
                let tex = renderer.import_memory(&self.mem, self.mem.format(), self.mem.size(), false)?;
                entry.insert(Box::new(tex.clone()));
                tex
            }
        };

        self.renderer_seen.insert(context_id, current_commit);
        Ok(tex)
    }
}

#[derive(Debug, Clone)]
pub struct MemoryRenderBuffer {
    id: Id,
    inner: Arc<Mutex<MemoryRenderBufferInner>>,
}

impl Default for MemoryRenderBuffer {
    fn default() -> Self {
        Self {
            id: Id::new(),
            inner: Default::default(),
        }
    }
}

impl MemoryRenderBuffer {
    pub fn new(
        format: Fourcc,
        size: impl Into<Size<i32, Buffer>>,
        scale: i32,
        transform: Transform,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    ) -> Self {
        let inner = MemoryRenderBufferInner::new(format, size, scale, transform, opaque_regions);
        MemoryRenderBuffer {
            id: Id::new(),
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn from_memory(
        mem: MemoryBuffer,
        scale: i32,
        transform: Transform,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    ) -> Self {
        let inner = MemoryRenderBufferInner::from_memory(mem, scale, transform, opaque_regions);
        MemoryRenderBuffer {
            id: Id::new(),
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn from_slice(
        mem: &[u8],
        format: Fourcc,
        size: impl Into<Size<i32, Buffer>>,
        scale: i32,
        transform: Transform,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    ) -> Self {
        let inner = MemoryRenderBufferInner::from_slice(mem, format, size, scale, transform, opaque_regions);
        MemoryRenderBuffer {
            id: Id::new(),
            inner: Arc::new(Mutex::new(inner)),
        }
    }

    pub fn render(&mut self) -> RenderContext<'_> {
        let guard = self.inner.lock().unwrap();
        RenderContext {
            buffer: guard,
            damage: Vec::new(),
            opaque_regions: None,
        }
    }
}

#[derive(Debug)]
pub struct RenderContext<'a> {
    buffer: MutexGuard<'a, MemoryRenderBufferInner>,
    damage: Vec<Rectangle<i32, Buffer>>,
    opaque_regions: Option<Option<Vec<Rectangle<i32, Buffer>>>>,
}

impl RenderContext<'_> {
    pub fn resize(&mut self, size: impl Into<Size<i32, Buffer>>) {
        self.buffer.resize(size);
    }

    pub fn draw<F, E>(&mut self, f: F) -> Result<(), E>
    where
        F: FnOnce(&mut [u8]) -> Result<Vec<Rectangle<i32, Buffer>>, E>,
    {
        let draw_damage = f(&mut self.buffer.mem)?;
        self.damage.extend(draw_damage);
        Ok(())
    }

    pub fn update_opaque_regions(&mut self, opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>) {
        self.opaque_regions = Some(opaque_regions);
    }
}

impl Drop for RenderContext<'_> {
    fn drop(&mut self) {
        self.buffer.damage_bag.add(std::mem::take(&mut self.damage));
        if let Some(opaque_regions) = self.opaque_regions.take() {
            self.buffer.opaque_regions = opaque_regions;
        }
    }
}

#[derive(Debug)]
pub struct MemoryRenderBufferRenderElement<R: Renderer> {
    id: Id,
    location: Point<f64, Physical>,
    buffer: MemoryBuffer,
    alpha: f32,
    src: Rectangle<f64, Logical>,
    buffer_scale: i32,
    buffer_transform: Transform,
    size: Size<i32, Logical>,
    damage: DamageSnapshot<i32, Buffer>,
    opaque_regions: OpaqueRegions<i32, Buffer>,
    texture: R::TextureId,
    kind: Kind,
}

impl<R: Renderer> MemoryRenderBufferRenderElement<R> {
    pub fn from_buffer(
        renderer: &mut R,
        location: impl Into<Point<f64, Physical>>,
        buffer: &MemoryRenderBuffer,
        alpha: Option<f32>,
        src: Option<Rectangle<f64, Logical>>,
        size: Option<Size<i32, Logical>>,
        kind: Kind,
    ) -> Result<Self, R::Error>
    where
        R: ImportMem,
        R::TextureId: Send + Clone + 'static,
    {
        let mut inner = buffer.inner.lock().unwrap();
        let texture = inner.import_texture(renderer)?;

        let size = size
            .or_else(|| src.map(|src| Size::from((src.size.w as i32, src.size.h as i32))))
            .unwrap_or_else(|| inner.mem.size().to_logical(inner.scale, inner.transform));

        let src = src.unwrap_or_else(|| Rectangle::from_size(size.to_f64()));

        Ok(MemoryRenderBufferRenderElement {
            id: buffer.id.clone(),
            buffer: inner.mem.clone(),
            location: location.into(),
            alpha: alpha.unwrap_or(1.0),
            src,
            buffer_scale: inner.scale,
            buffer_transform: inner.transform,
            size,
            opaque_regions: inner
                .opaque_regions
                .as_deref()
                .map(OpaqueRegions::from_slice)
                .unwrap_or_default(),
            damage: inner.damage_bag.snapshot(),
            texture,
            kind,
        })
    }

    fn physical_size(&self, scale: Scale<f64>) -> Size<i32, Physical> {
        ((self.size.to_f64().to_physical(scale).to_point() + self.location).to_i32_round()
            - self.location.to_i32_round())
        .to_size()
    }

    fn scale(&self) -> Scale<f64> {
        let src = self.src();
        Scale::from((self.size.w as f64 / src.size.w, self.size.h as f64 / src.size.h))
    }
}

impl<R: Renderer> Element for MemoryRenderBufferRenderElement<R> {
    fn id(&self) -> &Id {
        &self.id
    }

    fn current_commit(&self) -> CommitCounter {
        self.damage.current_commit()
    }

    fn transform(&self) -> Transform {
        self.buffer_transform
    }

    fn src(&self) -> Rectangle<f64, Buffer> {
        self.src.to_buffer(
            self.buffer_scale as f64,
            self.buffer_transform,
            &self.size.to_f64(),
        )
    }

    fn geometry(&self, scale: Scale<f64>) -> Rectangle<i32, Physical> {
        Rectangle::new(self.location.to_i32_round(), self.physical_size(scale))
    }

    fn damage_since(&self, scale: Scale<f64>, commit: Option<CommitCounter>) -> DamageSet<i32, Physical> {
        let physical_size = self.physical_size(scale);
        let logical_scale = self.scale();

        self.damage
            .damage_since(commit)
            .map(|damage| {
                damage
                    .into_iter()
                    .filter_map(|rect| {
                        rect.to_f64()
                            .to_logical(
                                self.buffer_scale as f64,
                                self.buffer_transform,
                                &self.buffer.size.to_f64(),
                            )
                            .intersection(self.src)
                            .map(|mut rect| {
                                rect.loc -= self.src.loc;
                                rect.upscale(logical_scale)
                            })
                            .map(|rect| {
                                let surface_scale =
                                    physical_size.to_f64() / self.size.to_f64().to_physical(scale);
                                rect.to_physical_precise_up(surface_scale * scale)
                            })
                    })
                    .collect::<DamageSet<_, _>>()
            })
            .unwrap_or_else(|| DamageSet::from_slice(&[Rectangle::from_size(physical_size)]))
    }

    fn opaque_regions(&self, scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        if self.alpha < 1.0 {
            return OpaqueRegions::default();
        }

        let physical_size = self.physical_size(scale);
        let logical_scale = self.scale();

        self.opaque_regions
            .iter()
            .filter_map(|rect| {
                rect.to_f64()
                    .to_logical(
                        self.buffer_scale as f64,
                        self.buffer_transform,
                        &self.buffer.size.to_f64(),
                    )
                    .intersection(self.src)
                    .map(|mut rect| {
                        rect.loc -= self.src.loc;
                        rect.upscale(logical_scale)
                    })
                    .map(|rect| {
                        let surface_scale = physical_size.to_f64() / self.size.to_f64().to_physical(scale);
                        rect.to_physical_precise_up(surface_scale * scale)
                    })
            })
            .collect::<OpaqueRegions<_, _>>()
    }

    fn alpha(&self) -> f32 {
        self.alpha
    }

    fn kind(&self) -> Kind {
        self.kind
    }
}

impl<R> RenderElement<R> for MemoryRenderBufferRenderElement<R>
where
    R: Renderer + ImportMem,
    R::TextureId: 'static,
{
    #[instrument(level = "trace", skip(self, frame))]
    #[profiling::function]
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: Rectangle<f64, Buffer>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<(), R::Error> {
        frame.render_texture_from_to(
            &self.texture,
            src,
            dst,
            damage,
            opaque_regions,
            self.buffer_transform,
            self.alpha,
        )
    }

    #[inline]
    fn underlying_storage(&self, _renderer: &mut R) -> Option<UnderlyingStorage<'_>> {
        Some(UnderlyingStorage::Memory(&self.buffer))
    }
}
