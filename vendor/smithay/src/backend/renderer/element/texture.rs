//! Element to render a texture.

use std::sync::{Arc, Mutex};

use tracing::{instrument, warn};

use crate::{
    backend::{
        allocator::Fourcc,
        renderer::{
            utils::{DamageBag, DamageSet, DamageSnapshot, OpaqueRegions},
            ContextId, Frame, ImportMem, Renderer, Texture,
        },
    },
    utils::{Buffer, Coordinate, Logical, Physical, Point, Rectangle, Scale, Size, Transform},
};

use super::{CommitCounter, Element, Id, Kind, RenderElement};

#[derive(Debug, Clone)]
pub struct TextureBuffer<T: Texture> {
    id: Id,
    context_id: ContextId<T>,
    texture: T,
    scale: i32,
    transform: Transform,
    opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
}

impl<T: Texture> TextureBuffer<T> {
    pub fn from_texture<R: Renderer<TextureId = T>>(
        renderer: &R,
        texture: T,
        scale: i32,
        transform: Transform,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    ) -> Self {
        TextureBuffer {
            id: Id::new(),
            context_id: renderer.context_id(),
            texture,
            scale,
            transform,
            opaque_regions,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_memory<R: Renderer<TextureId = T> + ImportMem>(
        renderer: &mut R,
        data: &[u8],
        format: Fourcc,
        size: impl Into<Size<i32, Buffer>>,
        flipped: bool,
        scale: i32,
        transform: Transform,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    ) -> Result<Self, R::Error> {
        let texture = renderer.import_memory(data, format, size.into(), flipped)?;
        Ok(TextureBuffer::from_texture(
            renderer,
            texture,
            scale,
            transform,
            opaque_regions,
        ))
    }

    pub fn format(&self) -> Option<Fourcc>
    where
        T: Texture,
    {
        self.texture.format()
    }
}

#[derive(Debug, Clone)]
pub struct TextureRenderBuffer<T: Texture> {
    id: Id,
    context_id: ContextId<T>,
    texture: T,
    scale: i32,
    transform: Transform,
    opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    damage_tracker: Arc<Mutex<DamageBag<i32, Buffer>>>,
}

impl<T: Texture> TextureRenderBuffer<T> {
    pub fn from_texture<R: Renderer<TextureId = T>>(
        renderer: &R,
        texture: T,
        scale: i32,
        transform: Transform,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    ) -> Self {
        TextureRenderBuffer {
            id: Id::new(),
            context_id: renderer.context_id(),
            texture,
            scale,
            transform,
            opaque_regions,
            damage_tracker: Arc::new(Mutex::new(DamageBag::default())),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_memory<R: Renderer<TextureId = T> + ImportMem>(
        renderer: &mut R,
        data: &[u8],
        format: Fourcc,
        size: impl Into<Size<i32, Buffer>>,
        flipped: bool,
        scale: i32,
        transform: Transform,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    ) -> Result<Self, R::Error> {
        let texture = renderer.import_memory(data, format, size.into(), flipped)?;
        Ok(TextureRenderBuffer::from_texture(
            renderer,
            texture,
            scale,
            transform,
            opaque_regions,
        ))
    }

    pub fn update_from_texture<R: Renderer<TextureId = T>>(
        &mut self,
        renderer: &R,
        texture: T,
        scale: i32,
        transform: Transform,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    ) {
        assert_eq!(self.context_id, renderer.context_id());
        self.texture = texture;
        self.scale = scale;
        self.transform = transform;
        self.opaque_regions = opaque_regions;
        self.damage_tracker.lock().unwrap().reset();
    }

    pub fn update_from_memory<R: Renderer<TextureId = T> + ImportMem>(
        &mut self,
        renderer: &mut R,
        data: &[u8],
        region: Rectangle<i32, Buffer>,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
    ) -> Result<(), R::Error> {
        assert_eq!(self.context_id, renderer.context_id());
        renderer.update_memory(&self.texture, data, region)?;
        self.damage_tracker.lock().unwrap().add([region]);
        self.opaque_regions = opaque_regions;
        Ok(())
    }

    pub fn render(&mut self) -> RenderContext<'_, T> {
        RenderContext {
            buffer: self,
            damage: Vec::new(),
            opaque_regions: None,
        }
    }

    pub fn format(&self) -> Option<Fourcc> {
        self.texture.format()
    }
}

#[derive(Debug)]
pub struct RenderContext<'a, T: Texture> {
    buffer: &'a mut TextureRenderBuffer<T>,
    damage: Vec<Rectangle<i32, Buffer>>,
    opaque_regions: Option<Option<Vec<Rectangle<i32, Buffer>>>>,
}

impl<T: Texture> RenderContext<'_, T> {
    pub fn draw<F, E>(&mut self, f: F) -> Result<(), E>
    where
        F: FnOnce(&mut T) -> Result<Vec<Rectangle<i32, Buffer>>, E>,
    {
        let draw_damage = f(&mut self.buffer.texture)?;
        self.damage.extend(draw_damage);
        Ok(())
    }

    pub fn update_opaque_regions(&mut self, opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>) {
        self.opaque_regions = Some(opaque_regions);
    }
}

impl<T: Texture> Drop for RenderContext<'_, T> {
    fn drop(&mut self) {
        self.buffer
            .damage_tracker
            .lock()
            .unwrap()
            .add(std::mem::take(&mut self.damage));
        if let Some(opaque_regions) = self.opaque_regions.take() {
            self.buffer.opaque_regions = opaque_regions;
        }
    }
}

#[derive(Debug)]
pub struct TextureRenderElement<T: Texture> {
    location: Point<f64, Physical>,
    id: Id,
    context_id: ContextId<T>,
    pub(crate) texture: T,
    scale: i32,
    transform: Transform,
    alpha: f32,
    src: Option<Rectangle<f64, Logical>>,
    size: Option<Size<i32, Logical>>,
    opaque_regions: Option<Vec<Rectangle<i32, Logical>>>,
    snapshot: DamageSnapshot<i32, Buffer>,
    kind: Kind,
}

impl<T: Texture> TextureRenderElement<T> {
    fn damage_since(&self, commit: Option<CommitCounter>) -> DamageSet<i32, Buffer> {
        self.snapshot
            .damage_since(commit)
            .unwrap_or_else(|| DamageSet::from_slice(&[Rectangle::from_size(self.texture.size())]))
    }
}

impl<T: Texture + Clone> TextureRenderElement<T> {
    pub fn from_texture_render_buffer(
        location: impl Into<Point<f64, Physical>>,
        buffer: &TextureRenderBuffer<T>,
        alpha: Option<f32>,
        src: Option<Rectangle<f64, Logical>>,
        size: Option<Size<i32, Logical>>,
        kind: Kind,
    ) -> Self {
        TextureRenderElement::from_texture_with_damage(
            buffer.id.clone(),
            buffer.context_id.clone(),
            location,
            buffer.texture.clone(),
            buffer.scale,
            buffer.transform,
            alpha,
            src,
            size,
            buffer.opaque_regions.clone(),
            buffer.damage_tracker.lock().unwrap().snapshot(),
            kind,
        )
    }

    pub fn from_texture_buffer(
        location: impl Into<Point<f64, Physical>>,
        buffer: &TextureBuffer<T>,
        alpha: Option<f32>,
        src: Option<Rectangle<f64, Logical>>,
        size: Option<Size<i32, Logical>>,
        kind: Kind,
    ) -> Self {
        TextureRenderElement::from_static_texture(
            buffer.id.clone(),
            buffer.context_id.clone(),
            location,
            buffer.texture.clone(),
            buffer.scale,
            buffer.transform,
            alpha,
            src,
            size,
            buffer.opaque_regions.clone(),
            kind,
        )
    }
}

impl<T: Texture> TextureRenderElement<T> {
    #[allow(clippy::too_many_arguments)]
    pub fn from_texture_with_damage(
        id: Id,
        context_id: ContextId<T>,
        location: impl Into<Point<f64, Physical>>,
        texture: T,
        scale: i32,
        transform: Transform,
        alpha: Option<f32>,
        src: Option<Rectangle<f64, Logical>>,
        size: Option<Size<i32, Logical>>,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
        snapshot: DamageSnapshot<i32, Buffer>,
        kind: Kind,
    ) -> Self {
        let opaque_regions = opaque_regions.map(|regions| {
            regions
                .into_iter()
                .map(|region| region.to_logical(scale, transform, &texture.size()))
                .collect::<Vec<_>>()
        });
        TextureRenderElement {
            location: location.into(),
            id,
            context_id,
            texture,
            scale,
            transform,
            alpha: alpha.unwrap_or(1.0),
            src,
            size,
            opaque_regions,
            snapshot,
            kind,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn from_static_texture(
        id: Id,
        context_id: ContextId<T>,
        location: impl Into<Point<f64, Physical>>,
        texture: T,
        scale: i32,
        transform: Transform,
        alpha: Option<f32>,
        src: Option<Rectangle<f64, Logical>>,
        size: Option<Size<i32, Logical>>,
        opaque_regions: Option<Vec<Rectangle<i32, Buffer>>>,
        kind: Kind,
    ) -> Self {
        TextureRenderElement::from_texture_with_damage(
            id,
            context_id,
            location,
            texture,
            scale,
            transform,
            alpha,
            src,
            size,
            opaque_regions,
            DamageSnapshot::empty(),
            kind,
        )
    }

    fn logical_size(&self) -> Size<i32, Logical> {
        self.size
            .or_else(|| {
                self.src
                    .map(|src| Size::from((src.size.w as i32, src.size.h as i32)))
            })
            .unwrap_or_else(|| self.texture.size().to_logical(self.scale, self.transform))
    }

    fn physical_size(&self, scale: Scale<f64>) -> Size<i32, Physical> {
        let logical_size = self.logical_size();
        ((logical_size.to_f64().to_physical(scale).to_point() + self.location).to_i32_round()
            - self.location.to_i32_round())
        .to_size()
    }

    fn src(&self) -> Rectangle<f64, Logical> {
        self.src
            .unwrap_or_else(|| Rectangle::from_size(self.logical_size().to_f64()))
    }

    fn scale(&self) -> Scale<f64> {
        let size = self.logical_size();
        let src = self.src();
        Scale::from((size.w as f64 / src.size.w, size.h as f64 / src.size.h))
    }

    fn rect_to_global<N>(&self, rect: Rectangle<N, Logical>) -> Rectangle<f64, Logical>
    where
        N: Coordinate,
    {
        let scale = self.scale();
        let mut rect = rect.to_f64();
        rect.loc -= self.src.map(|rect| rect.loc).unwrap_or_default();
        rect.upscale(scale)
    }
}

impl<T> Element for TextureRenderElement<T>
where
    T: Texture,
{
    fn id(&self) -> &Id {
        &self.id
    }

    fn current_commit(&self) -> CommitCounter {
        self.snapshot.current_commit()
    }

    fn geometry(&self, scale: Scale<f64>) -> Rectangle<i32, Physical> {
        Rectangle::new(self.location.to_i32_round(), self.physical_size(scale))
    }

    fn transform(&self) -> Transform {
        self.transform
    }

    fn src(&self) -> Rectangle<f64, Buffer> {
        let size = self.logical_size();
        self.src()
            .to_buffer(self.scale as f64, self.transform, &size.to_f64())
    }

    fn damage_since(&self, scale: Scale<f64>, commit: Option<CommitCounter>) -> DamageSet<i32, Physical> {
        let src = self.src();
        let texture_size = self.texture.size();
        let physical_size = self.physical_size(scale);
        let logical_size = self.logical_size();
        self.damage_since(commit)
            .into_iter()
            .filter_map(|rect| {
                rect.to_f64()
                    .to_logical(self.scale as f64, self.transform, &texture_size.to_f64())
                    .intersection(src)
                    .map(|rect| self.rect_to_global(rect).to_i32_up::<i32>())
                    .map(|rect| {
                        let surface_scale = physical_size.to_f64() / logical_size.to_f64().to_physical(scale);
                        rect.to_physical_precise_up(surface_scale * scale)
                    })
            })
            .collect::<DamageSet<_, _>>()
    }

    fn opaque_regions(&self, scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        if self.alpha < 1.0 {
            return OpaqueRegions::default();
        }

        let src = self.src();
        let physical_size = self.physical_size(scale);
        let logical_size = self.logical_size();
        self.opaque_regions
            .as_ref()
            .map(|r| {
                r.iter()
                    .filter_map(|rect| {
                        rect.to_f64()
                            .intersection(src)
                            .map(|rect| self.rect_to_global(rect).to_i32_up::<i32>())
                            .map(|rect| {
                                let surface_scale =
                                    physical_size.to_f64() / logical_size.to_f64().to_physical(scale);
                                rect.to_physical_precise_up(surface_scale * scale)
                            })
                    })
                    .collect::<OpaqueRegions<_, _>>()
            })
            .unwrap_or_default()
    }

    fn alpha(&self) -> f32 {
        self.alpha
    }

    fn kind(&self) -> Kind {
        self.kind
    }
}

impl<R, T> RenderElement<R> for TextureRenderElement<T>
where
    R: Renderer<TextureId = T>,
    T: Texture,
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
        if frame.context_id() != self.context_id {
            warn!("trying to render texture from different renderer context");
            return Ok(());
        }

        frame.render_texture_from_to(
            &self.texture,
            src,
            dst,
            damage,
            opaque_regions,
            self.transform,
            self.alpha,
        )
    }
}
