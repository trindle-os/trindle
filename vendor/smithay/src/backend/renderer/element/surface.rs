//! Element to render a wayland surface.

use std::fmt;

use tracing::{instrument, warn};
use wayland_server::protocol::wl_surface;

use crate::{
    backend::renderer::{
        utils::{
            Buffer, DamageSet, DamageSnapshot, OpaqueRegions, RendererSurfaceState,
            RendererSurfaceStateUserData, SurfaceView,
        },
        Color32F, Frame, ImportAll, Renderer, Texture,
    },
    utils::{Buffer as BufferCoords, Logical, Physical, Point, Rectangle, Scale, Size, Transform},
    wayland::{
        alpha_modifier::AlphaModifierSurfaceCachedState,
        compositor::{self, SurfaceData, TraversalAction},
    },
};

use super::{CommitCounter, Element, Id, Kind, RenderElement, UnderlyingStorage};

#[instrument(level = "trace", skip(renderer, location, scale))]
#[profiling::function]
pub fn render_elements_from_surface_tree<R, E>(
    renderer: &mut R,
    surface: &wl_surface::WlSurface,
    location: impl Into<Point<i32, Physical>>,
    scale: impl Into<Scale<f64>>,
    alpha: f32,
    kind: Kind,
) -> Vec<E>
where
    R: Renderer + ImportAll,
    R::TextureId: Clone + 'static,
    E: From<WaylandSurfaceRenderElement<R>>,
{
    let location = location.into().to_f64();
    let scale = scale.into();
    let mut surfaces: Vec<E> = Vec::new();

    compositor::with_surface_tree_downward(
        surface,
        location,
        |_, states, location| {
            let mut location = *location;
            let data = states.data_map.get::<RendererSurfaceStateUserData>();

            if let Some(data) = data {
                if let Some(view) = data.lock().unwrap().view() {
                    location += view.offset.to_f64().to_physical(scale);
                    TraversalAction::DoChildren(location)
                } else {
                    TraversalAction::SkipChildren
                }
            } else {
                TraversalAction::SkipChildren
            }
        },
        |surface, states, location| {
            let mut location = *location;
            let data = states.data_map.get::<RendererSurfaceStateUserData>();

            if let Some(data) = data {
                let has_view = if let Some(view) = data.lock().unwrap().view() {
                    location += view.offset.to_f64().to_physical(scale);
                    true
                } else {
                    false
                };

                if has_view {
                    match WaylandSurfaceRenderElement::from_surface(
                        renderer, surface, states, location, alpha, kind,
                    ) {
                        Ok(Some(surface)) => surfaces.push(surface.into()),
                        Ok(None) => {}
                        Err(err) => {
                            warn!("Failed to import surface: {}", err);
                        }
                    };
                }
            }
        },
        |_, _, _| true,
    );

    surfaces
}

#[derive(Debug)]
pub enum WaylandSurfaceTexture<R: Renderer> {
    Texture(R::TextureId),
    SolidColor(Color32F),
}

pub struct WaylandSurfaceRenderElement<R: Renderer> {
    id: Id,
    location: Point<f64, Physical>,
    alpha: f32,
    kind: Kind,

    view: SurfaceView,
    buffer: Buffer,
    buffer_scale: i32,
    buffer_transform: Transform,
    buffer_dimensions: Size<i32, BufferCoords>,
    damage: DamageSnapshot<i32, BufferCoords>,
    opaque_regions: OpaqueRegions<i32, Logical>,
    texture: WaylandSurfaceTexture<R>,
}

impl<R: Renderer> fmt::Debug for WaylandSurfaceRenderElement<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WaylandSurfaceRenderElement")
            .field("id", &self.id)
            .field("location", &self.location)
            .finish()
    }
}

impl<R: Renderer + ImportAll> WaylandSurfaceRenderElement<R> {
    #[profiling::function]
    pub fn from_surface(
        renderer: &mut R,
        surface: &wl_surface::WlSurface,
        states: &SurfaceData,
        location: Point<f64, Physical>,
        alpha: f32,
        kind: Kind,
    ) -> Result<Option<Self>, R::Error>
    where
        R::TextureId: Clone + 'static,
    {
        let id = Id::from_wayland_resource(surface);
        crate::backend::renderer::utils::import_surface(renderer, states)?;

        let mut alpha_modifier_state = states.cached_state.get::<AlphaModifierSurfaceCachedState>();
        let alpha_multiplier = alpha_modifier_state.current().multiplier_f32().unwrap_or(1.0);

        let Some(data_ref) = states.data_map.get::<RendererSurfaceStateUserData>() else {
            return Ok(None);
        };
        Ok(Self::from_state(
            renderer,
            id,
            location,
            alpha * alpha_multiplier,
            kind,
            &data_ref.lock().unwrap(),
        ))
    }

    fn from_state(
        renderer: &mut R,
        id: Id,
        location: Point<f64, Physical>,
        alpha: f32,
        kind: Kind,
        data: &RendererSurfaceState,
    ) -> Option<Self>
    where
        R::TextureId: Clone + 'static,
    {
        let buffer = data.buffer()?.clone();

        let texture = if let Ok(spb) = crate::wayland::single_pixel_buffer::get_single_pixel_buffer(&buffer) {
            WaylandSurfaceTexture::SolidColor(Color32F::from(spb.rgba32f()))
        } else {
            WaylandSurfaceTexture::Texture(data.texture(renderer.context_id())?.clone())
        };

        Some(Self {
            id,
            location,
            alpha,
            kind,
            view: data.view()?,
            buffer,
            buffer_scale: data.buffer_scale(),
            buffer_transform: data.buffer_transform(),
            buffer_dimensions: data.buffer_dimensions?,
            damage: data.damage.snapshot(),
            opaque_regions: data
                .opaque_regions()
                .map(OpaqueRegions::from_slice)
                .unwrap_or_default(),
            texture,
        })
    }

    fn size(&self, scale: impl Into<Scale<f64>>) -> Size<i32, Physical> {
        ((self.view.dst.to_f64().to_physical(scale).to_point() + self.location).to_i32_round()
            - self.location.to_i32_round())
        .to_size()
    }

    pub fn buffer_size(&self) -> Size<i32, Logical> {
        self.buffer_dimensions
            .to_logical(self.buffer_scale, self.buffer_transform)
    }

    pub fn view(&self) -> SurfaceView {
        self.view
    }

    pub fn texture(&self) -> &WaylandSurfaceTexture<R> {
        &self.texture
    }
}

impl<R: Renderer + ImportAll> Element for WaylandSurfaceRenderElement<R> {
    fn id(&self) -> &Id {
        &self.id
    }

    fn current_commit(&self) -> CommitCounter {
        self.damage.current_commit()
    }

    fn geometry(&self, scale: Scale<f64>) -> Rectangle<i32, Physical> {
        Rectangle::new(self.location.to_i32_round(), self.size(scale))
    }

    fn src(&self) -> Rectangle<f64, BufferCoords> {
        self.view.src.to_buffer(
            self.buffer_scale as f64,
            self.buffer_transform,
            &self.buffer_size().to_f64(),
        )
    }

    fn transform(&self) -> Transform {
        self.buffer_transform
    }

    fn damage_since(&self, scale: Scale<f64>, commit: Option<CommitCounter>) -> DamageSet<i32, Physical> {
        let dst_size = self.size(scale);
        self.damage
            .damage_since(commit)
            .unwrap_or_else(|| DamageSet::from_slice(&[Rectangle::from_size(self.buffer_dimensions)]))
            .iter()
            .filter_map(|rect| {
                rect.to_f64()
                    .to_logical(
                        self.buffer_scale as f64,
                        self.buffer_transform,
                        &self.buffer_dimensions.to_f64(),
                    )
                    .intersection(self.view.src)
                    .map(|rect| self.view.rect_to_global(rect).to_i32_up::<i32>())
                    .map(|rect| {
                        let surface_scale = dst_size.to_f64() / self.view.dst.to_f64().to_physical(scale);
                        rect.to_physical_precise_up(surface_scale * scale)
                    })
            })
            .collect::<DamageSet<_, _>>()
    }

    fn opaque_regions(&self, scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        if self.alpha < 1.0 {
            return OpaqueRegions::default();
        }

        self.opaque_regions
            .iter()
            .map(|r| {
                let loc = r.loc.to_physical_precise_round(scale);
                let size = ((r.size.to_f64().to_physical(scale).to_point() + self.location).to_i32_round()
                    - self.location.to_i32_round())
                .to_size();
                Rectangle::new(loc, size)
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

impl<R> RenderElement<R> for WaylandSurfaceRenderElement<R>
where
    R: Renderer + ImportAll,
    R::TextureId: Texture + 'static,
{
    #[inline]
    fn underlying_storage(&self, _renderer: &mut R) -> Option<UnderlyingStorage<'_>> {
        Some(UnderlyingStorage::Wayland(&self.buffer))
    }

    #[instrument(level = "trace", skip(frame))]
    #[profiling::function]
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: Rectangle<f64, BufferCoords>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<(), R::Error> {
        match self.texture {
            WaylandSurfaceTexture::Texture(ref texture) => frame.render_texture_from_to(
                texture,
                src,
                dst,
                damage,
                opaque_regions,
                self.buffer_transform,
                self.alpha,
            ),
            WaylandSurfaceTexture::SolidColor(color) => frame.draw_solid(dst, damage, color * self.alpha),
        }
    }
}
