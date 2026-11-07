//! This module contains the [`Space`] helper class as well has related rendering helpers to add custom elements or different clients to a space.

use crate::{
    backend::renderer::{
        damage::{Error as OutputDamageTrackerError, OutputDamageTracker, RenderOutputResult},
        element::{AsRenderElements, RenderElement, Wrap},
        Color32F, Renderer, Texture,
    },
    output::{Output, OutputModeSource, OutputNoMode},
    utils::{IsAlive, Logical, Point, Rectangle, Scale, Transform},
};
#[cfg(feature = "wayland_frontend")]
use crate::{
    backend::renderer::{element::surface::WaylandSurfaceRenderElement, ImportAll},
    desktop::{layer_map_for_output, LayerSurface, WindowSurfaceType},
    wayland::shell::wlr_layer::Layer,
};
use std::{collections::HashMap, fmt};
use tracing::{debug, debug_span, instrument};
#[cfg(feature = "wayland_frontend")]
use wayland_server::protocol::wl_surface::WlSurface;

mod element;
mod output;
mod utils;

#[cfg(feature = "wayland_frontend")]
pub(crate) mod wayland;

pub use self::element::*;
use self::output::*;
pub use self::utils::*;

crate::utils::ids::id_gen!(space_id);

#[derive(Debug)]
struct InnerElement<E> {
    element: E,
    location: Point<i32, Logical>,
    outputs: HashMap<Output, Rectangle<i32, Logical>>,
}

#[derive(Debug)]
pub struct Space<E: SpaceElement> {
    pub(super) id: usize,
    elements: Vec<InnerElement<E>>,
    outputs: Vec<Output>,
    span: tracing::Span,
}

impl<E: SpaceElement> PartialEq for Space<E> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<E: SpaceElement> Drop for Space<E> {
    #[inline]
    fn drop(&mut self) {
        space_id::remove(self.id);
    }
}

impl<E: SpaceElement> Default for Space<E> {
    #[inline]
    fn default() -> Self {
        let id = space_id::next();
        let span = debug_span!("desktop_space", id);

        Self {
            id,
            elements: Default::default(),
            outputs: Default::default(),
            span,
        }
    }
}

impl<E: SpaceElement + PartialEq> Space<E> {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn map_element<P>(&mut self, element: E, location: P, activate: bool)
    where
        P: Into<Point<i32, Logical>>,
    {
        #[allow(clippy::mutable_key_type)]
        let outputs = if let Some(pos) = self.elements.iter().position(|inner| inner.element == element) {
            self.elements.remove(pos).outputs
        } else {
            HashMap::new()
        };

        let inner = InnerElement {
            element,
            location: location.into(),
            outputs,
        };
        self.insert_elem(inner, activate);
    }

    pub fn raise_element(&mut self, element: &E, activate: bool) {
        if let Some(pos) = self.elements.iter().position(|inner| &inner.element == element) {
            let inner = self.elements.remove(pos);
            self.insert_elem(inner, activate);
        }
    }

    fn insert_elem(&mut self, elem: InnerElement<E>, activate: bool) {
        if activate {
            elem.element.set_activate(true);
            for e in self.elements.iter() {
                e.element.set_activate(false);
            }
        }

        self.elements.push(elem);
        self.elements
            .sort_by(|e1, e2| e1.element.z_index().cmp(&e2.element.z_index()));
    }

    pub fn unmap_elem(&mut self, element: &E) {
        if let Some(pos) = self.elements.iter().position(|inner| &inner.element == element) {
            let elem = self.elements.remove(pos);
            for output in elem.outputs.keys() {
                elem.element.output_leave(output);
            }
        }
    }

    pub fn elements(&self) -> impl DoubleEndedIterator<Item = &E> + ExactSizeIterator {
        self.elements.iter().map(|e| &e.element)
    }

    pub fn elements_for_output<'output>(
        &'output self,
        output: &'output Output,
    ) -> impl DoubleEndedIterator<Item = &'output E> {
        self.elements
            .iter()
            .filter(|e| e.outputs.contains_key(output))
            .map(|e| &e.element)
    }

    pub fn element_under<P: Into<Point<f64, Logical>>>(&self, point: P) -> Option<(&E, Point<i32, Logical>)> {
        let point = point.into();
        self.elements
            .iter()
            .rev()
            .filter(|e| e.bbox().to_f64().contains(point))
            .find_map(|e| {
                let render_location = e.render_location();
                if e.element.is_in_input_region(&(point - render_location.to_f64())) {
                    Some((&e.element, render_location))
                } else {
                    None
                }
            })
    }

    pub fn output_under<P: Into<Point<f64, Logical>>>(&self, point: P) -> impl Iterator<Item = &Output> {
        let point = point.into();
        self.outputs.iter().rev().filter(move |o| {
            let bbox = self.output_geometry(o);
            bbox.map(|bbox| bbox.to_f64().contains(point)).unwrap_or(false)
        })
    }

    #[cfg(feature = "wayland_frontend")]
    pub fn layer_for_surface(
        &self,
        surface: &WlSurface,
        surface_type: WindowSurfaceType,
    ) -> Option<LayerSurface> {
        self.outputs.iter().find_map(|o| {
            let map = layer_map_for_output(o);
            map.layer_for_surface(surface, surface_type).cloned()
        })
    }

    pub fn element_location(&self, elem: &E) -> Option<Point<i32, Logical>> {
        self.elements
            .iter()
            .find(|e| &e.element == elem)
            .map(|e| e.location)
    }

    pub fn element_bbox(&self, elem: &E) -> Option<Rectangle<i32, Logical>> {
        self.elements
            .iter()
            .find(|e| &e.element == elem)
            .map(|e| e.bbox())
    }

    pub fn element_geometry(&self, elem: &E) -> Option<Rectangle<i32, Logical>> {
        self.elements
            .iter()
            .find(|e| &e.element == elem)
            .map(|e| e.geometry())
    }

    pub fn map_output<P: Into<Point<i32, Logical>>>(&mut self, output: &Output, location: P) {
        let location = location.into();
        set_output_location(self.id, output, location);
        if !self.outputs.contains(output) {
            debug!(parent: &self.span, output = output.name(), "Mapping output at {:?}", location);
            self.outputs.push(output.clone());
        }
    }

    pub fn outputs(&self) -> impl Iterator<Item = &Output> {
        self.outputs.iter()
    }

    pub fn unmap_output(&mut self, output: &Output) {
        if !self.outputs.contains(output) {
            return;
        }
        debug!(parent: &self.span, output = output.name(), "Unmapping output");
        set_output_location(self.id, output, None);
        self.outputs.retain(|o| o != output);
    }

    pub fn output_geometry(&self, o: &Output) -> Option<Rectangle<i32, Logical>> {
        if !self.outputs.contains(o) {
            return None;
        }

        let transform: Transform = o.current_transform();
        let location = output_location(self.id, o);
        o.current_mode().map(|mode| {
            Rectangle::new(
                location,
                transform
                    .transform_size(mode.size)
                    .to_f64()
                    .to_logical(o.current_scale().fractional_scale())
                    .to_i32_ceil(),
            )
        })
    }

    pub fn outputs_for_element(&self, elem: &E) -> Vec<Output> {
        if !self.elements.iter().any(|e| &e.element == elem) {
            return Vec::new();
        }

        self.elements
            .iter()
            .find(|e| &e.element == elem)
            .into_iter()
            .flat_map(|e| &e.outputs)
            .map(|(o, _)| o)
            .cloned()
            .collect()
    }

    #[profiling::function]
    pub fn refresh(&mut self) {
        self.elements.retain(|e| e.alive());

        let outputs = self
            .outputs
            .iter()
            .cloned()
            .map(|o| {
                let geo = self.output_geometry(&o).unwrap_or_else(Rectangle::zero);
                (o, geo)
            })
            .collect::<Vec<_>>();
        for e in &mut self.elements {
            let bbox = e.bbox();

            for (output, output_geometry) in &outputs {
                if let Some(mut overlap) = output_geometry.intersection(bbox) {
                    overlap.loc -= bbox.loc;
                    let old = e.outputs.insert(output.clone(), overlap);
                    if old.is_none() || matches!(old, Some(old_overlap) if old_overlap != overlap) {
                        e.element.output_enter(output, overlap);
                    }
                } else if e.outputs.remove(output).is_some() {
                    e.element.output_leave(output);
                }
            }
            e.outputs.retain(|output, _| {
                if !outputs.iter().any(|(o, _)| o == output) {
                    e.element.output_leave(output);
                    false
                } else {
                    true
                }
            });
        }

        self.elements.iter().for_each(|e| e.element.refresh());
        for (output, _) in outputs {
            output.cleanup();
        }
    }

    #[instrument(level = "trace", skip(self, renderer, scale), parent = &self.span)]
    #[profiling::function]
    pub fn render_elements_for_region<'a, R: Renderer, S: Into<Scale<f64>>>(
        &'a self,
        renderer: &mut R,
        region: &Rectangle<i32, Logical>,
        scale: S,
        alpha: f32,
    ) -> Vec<<E as AsRenderElements<R>>::RenderElement>
    where
        R::TextureId: Texture + 'static,
        E: AsRenderElements<R>,
        <E as AsRenderElements<R>>::RenderElement: 'a,
    {
        let scale = scale.into();

        self.elements
            .iter()
            .rev()
            .filter(|e| {
                let geometry = e.bbox();
                region.overlaps(geometry)
            })
            .flat_map(|e| {
                let location = e.render_location() - region.loc;
                e.element
                    .render_elements::<<E as AsRenderElements<R>>::RenderElement>(
                        renderer,
                        location.to_physical_precise_round(scale),
                        scale,
                        alpha,
                    )
            })
            .collect::<Vec<_>>()
    }

    #[instrument(level = "trace", skip(self, renderer), parent = &self.span)]
    #[profiling::function]
    pub fn render_elements_for_output<
        'a,
        #[cfg(feature = "wayland_frontend")] R: Renderer + ImportAll,
        #[cfg(not(feature = "wayland_frontend"))] R: Renderer,
    >(
        &'a self,
        renderer: &mut R,
        output: &Output,
        alpha: f32,
    ) -> Result<Vec<SpaceRenderElements<R, <E as AsRenderElements<R>>::RenderElement>>, OutputError>
    where
        R::TextureId: Clone + Texture + 'static,
        E: AsRenderElements<R>,
        <E as AsRenderElements<R>>::RenderElement: 'a,
        SpaceRenderElements<R, <E as AsRenderElements<R>>::RenderElement>:
            From<Wrap<<E as AsRenderElements<R>>::RenderElement>>,
    {
        if !self.outputs.contains(output) {
            return Err(OutputError::Unmapped);
        }

        let output_scale = output.current_scale().fractional_scale();
        let output_geo = self.output_geometry(output).unwrap();

        let mut space_elements: Vec<SpaceElements<'a, E>> =
            self.elements.iter().rev().map(SpaceElements::Element).collect();

        #[cfg(feature = "wayland_frontend")]
        {
            let layer_map = layer_map_for_output(output);
            space_elements.extend(layer_map.layers().rev().cloned().map(|l| SpaceElements::Layer {
                surface: l,
                output_location: output_geo.loc,
            }));
        }

        space_elements.sort_by_key(|e| std::cmp::Reverse(e.z_index()));

        Ok(space_elements
            .into_iter()
            .filter(|e| {
                let geometry = e.bbox();
                output_geo.overlaps(geometry)
            })
            .flat_map(|e| {
                let location = e.render_location() - output_geo.loc;
                e.render_elements::<SpaceRenderElements<R, <E as AsRenderElements<R>>::RenderElement>>(
                    renderer,
                    location.to_physical_precise_round(output_scale),
                    Scale::from(output_scale),
                    alpha,
                )
            })
            .collect::<Vec<_>>())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum OutputError {
    #[error(transparent)]
    NoMode(#[from] OutputNoMode),
    #[error("Output was not mapped to this space")]
    Unmapped,
}

impl<E: IsAlive> IsAlive for InnerElement<E> {
    #[inline]
    fn alive(&self) -> bool {
        self.element.alive()
    }
}

impl<E: SpaceElement> InnerElement<E> {
    fn geometry(&self) -> Rectangle<i32, Logical> {
        let mut geo = self.element.geometry();
        geo.loc = self.location;
        geo
    }

    fn bbox(&self) -> Rectangle<i32, Logical> {
        let mut bbox = self.element.bbox();
        bbox.loc += self.location - self.element.geometry().loc;
        bbox
    }

    fn render_location(&self) -> Point<i32, Logical> {
        self.location - self.element.geometry().loc
    }
}

#[cfg(feature = "wayland_frontend")]
crate::backend::renderer::element::render_elements! {
    pub SpaceRenderElements<R, E> where
        R: ImportAll;
    Surface=WaylandSurfaceRenderElement<R>,
    Element=Wrap<E>,
}
#[cfg(not(feature = "wayland_frontend"))]
crate::backend::renderer::element::render_elements! {
    pub SpaceRenderElements<R, E>;
    Element=Wrap<E>,
}

impl<
        #[cfg(feature = "wayland_frontend")] R: Renderer + ImportAll,
        #[cfg(not(feature = "wayland_frontend"))] R: Renderer,
        E: RenderElement<R> + std::fmt::Debug,
    > std::fmt::Debug for SpaceRenderElements<R, E>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "wayland_frontend")]
            Self::Surface(arg0) => f.debug_tuple("Surface").field(arg0).finish(),
            Self::Element(arg0) => f.debug_tuple("Element").field(arg0).finish(),
            Self::_GenericCatcher(_) => unreachable!(),
        }
    }
}

#[cfg(feature = "wayland_frontend")]
crate::backend::renderer::element::render_elements! {
    OutputRenderElements<'a, R, E, C> where
        R: ImportAll;
    Space=SpaceRenderElements<R, E>,
    Custom=&'a C,
}
#[cfg(not(feature = "wayland_frontend"))]
crate::backend::renderer::element::render_elements! {
    OutputRenderElements<'a, R, E, C>;
    Space=SpaceRenderElements<R, E>,
    Custom=&'a C,
}

#[instrument(level = "trace", skip(spaces, renderer))]
#[profiling::function]
pub fn space_render_elements<
    'a,
    #[cfg(feature = "wayland_frontend")] R: Renderer + ImportAll,
    #[cfg(not(feature = "wayland_frontend"))] R: Renderer,
    E: SpaceElement + PartialEq + AsRenderElements<R> + 'a,
    S: IntoIterator<Item = &'a Space<E>>,
>(
    renderer: &mut R,
    spaces: S,
    output: &Output,
    alpha: f32,
) -> Result<Vec<SpaceRenderElements<R, <E as AsRenderElements<R>>::RenderElement>>, OutputNoMode>
where
    R::TextureId: Clone + Texture + 'static,
    <E as AsRenderElements<R>>::RenderElement: 'a,
    SpaceRenderElements<R, <E as AsRenderElements<R>>::RenderElement>:
        From<Wrap<<E as AsRenderElements<R>>::RenderElement>>,
{
    let mut render_elements = Vec::new();
    let output_scale = output.current_scale().fractional_scale();

    #[cfg(feature = "wayland_frontend")]
    let layer_map = layer_map_for_output(output);
    #[cfg(feature = "wayland_frontend")]
    let lower = {
        let (lower, upper): (Vec<&LayerSurface>, Vec<&LayerSurface>) = layer_map
            .layers()
            .rev()
            .partition(|s| matches!(s.layer(), Layer::Background | Layer::Bottom));

        render_elements.extend(
            upper
                .into_iter()
                .filter_map(|surface| layer_map.layer_geometry(surface).map(|geo| (geo.loc, surface)))
                .flat_map(|(loc, surface)| {
                    AsRenderElements::<R>::render_elements::<WaylandSurfaceRenderElement<R>>(
                        surface,
                        renderer,
                        loc.to_physical_precise_round(output_scale),
                        Scale::from(output_scale),
                        alpha,
                    )
                    .into_iter()
                    .map(SpaceRenderElements::Surface)
                }),
        );

        lower
    };

    for space in spaces {
        let _guard = space.span.enter();
        if let Some(output_geo) = space.output_geometry(output) {
            render_elements.extend(
                space
                    .render_elements_for_region(renderer, &output_geo, output_scale, alpha)
                    .into_iter()
                    .map(|e| SpaceRenderElements::Element(Wrap::from(e))),
            );
        }
    }

    #[cfg(feature = "wayland_frontend")]
    render_elements.extend(
        lower
            .into_iter()
            .filter_map(|surface| layer_map.layer_geometry(surface).map(|geo| (geo.loc, surface)))
            .flat_map(|(loc, surface)| {
                AsRenderElements::<R>::render_elements::<WaylandSurfaceRenderElement<R>>(
                    surface,
                    renderer,
                    loc.to_physical_precise_round(output_scale),
                    Scale::from(output_scale),
                    alpha,
                )
                .into_iter()
                .map(SpaceRenderElements::Surface)
            }),
    );

    Ok(render_elements)
}

#[allow(clippy::too_many_arguments)]
#[profiling::function]
pub fn render_output<
    'a,
    'd,
    #[cfg(feature = "wayland_frontend")] R: Renderer + ImportAll,
    #[cfg(not(feature = "wayland_frontend"))] R: Renderer,
    C: RenderElement<R>,
    E: SpaceElement + PartialEq + AsRenderElements<R> + 'a,
    S: IntoIterator<Item = &'a Space<E>>,
>(
    output: &Output,
    renderer: &mut R,
    framebuffer: &mut R::Framebuffer<'_>,
    alpha: f32,
    age: usize,
    spaces: S,
    custom_elements: &'a [C],
    damage_tracker: &'d mut OutputDamageTracker,
    clear_color: impl Into<Color32F>,
) -> Result<RenderOutputResult<'d>, OutputDamageTrackerError<R::Error>>
where
    R::TextureId: Clone + Texture + 'static,
    <E as AsRenderElements<R>>::RenderElement: 'a,
    SpaceRenderElements<R, <E as AsRenderElements<R>>::RenderElement>:
        From<Wrap<<E as AsRenderElements<R>>::RenderElement>>,
{
    if let OutputModeSource::Auto(renderer_output) = damage_tracker.mode() {
        assert!(renderer_output == output);
    }

    let space_render_elements = space_render_elements(renderer, spaces, output, alpha)?;

    let mut render_elements: Vec<OutputRenderElements<'a, R, <E as AsRenderElements<R>>::RenderElement, C>> =
        Vec::with_capacity(custom_elements.len() + space_render_elements.len());

    render_elements.extend(custom_elements.iter().map(OutputRenderElements::Custom));
    render_elements.extend(space_render_elements.into_iter().map(OutputRenderElements::Space));

    damage_tracker.render_output(renderer, framebuffer, age, &render_elements, clear_color)
}
