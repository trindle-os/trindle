//! Utilities and helpers around the `Element` trait.

use crate::{
    backend::renderer::{
        element::{AsRenderElements, Element, Id, Kind, RenderElement, UnderlyingStorage},
        utils::{DamageSet, OpaqueRegions},
        Renderer,
    },
    utils::{Buffer, Physical, Point, Rectangle, Scale},
};

#[derive(Debug)]
pub struct RescaleRenderElement<E> {
    element: E,
    origin: Point<i32, Physical>,
    scale: Scale<f64>,
}

impl<E: Element> RescaleRenderElement<E> {
    pub fn from_element(element: E, origin: Point<i32, Physical>, scale: impl Into<Scale<f64>>) -> Self {
        RescaleRenderElement {
            element,
            origin,
            scale: scale.into(),
        }
    }
}

impl<E: Element> Element for RescaleRenderElement<E> {
    fn id(&self) -> &Id {
        self.element.id()
    }

    fn current_commit(&self) -> crate::backend::renderer::utils::CommitCounter {
        self.element.current_commit()
    }

    fn src(&self) -> crate::utils::Rectangle<f64, crate::utils::Buffer> {
        self.element.src()
    }

    fn geometry(
        &self,
        scale: crate::utils::Scale<f64>,
    ) -> crate::utils::Rectangle<i32, crate::utils::Physical> {
        let mut element_geometry = self.element.geometry(scale);
        element_geometry.loc -= self.origin;
        element_geometry = element_geometry.to_f64().upscale(self.scale).to_i32_round();
        element_geometry.loc += self.origin;
        element_geometry
    }

    fn transform(&self) -> crate::utils::Transform {
        self.element.transform()
    }

    fn damage_since(
        &self,
        scale: crate::utils::Scale<f64>,
        commit: Option<crate::backend::renderer::utils::CommitCounter>,
    ) -> DamageSet<i32, Physical> {
        self.element
            .damage_since(scale, commit)
            .into_iter()
            .map(|rect| rect.to_f64().upscale(self.scale).to_i32_up())
            .collect::<DamageSet<_, _>>()
    }

    fn opaque_regions(&self, scale: crate::utils::Scale<f64>) -> OpaqueRegions<i32, Physical> {
        self.element
            .opaque_regions(scale)
            .into_iter()
            .map(|rect| rect.to_f64().upscale(self.scale).to_i32_round())
            .collect::<OpaqueRegions<_, _>>()
    }

    fn alpha(&self) -> f32 {
        self.element.alpha()
    }

    fn kind(&self) -> Kind {
        self.element.kind()
    }
}

impl<R: Renderer, E: RenderElement<R>> RenderElement<R> for RescaleRenderElement<E> {
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: crate::utils::Rectangle<f64, crate::utils::Buffer>,
        dst: crate::utils::Rectangle<i32, crate::utils::Physical>,
        damage: &[crate::utils::Rectangle<i32, crate::utils::Physical>],
        opaque_regions: &[crate::utils::Rectangle<i32, crate::utils::Physical>],
    ) -> Result<(), R::Error> {
        self.element.draw(frame, src, dst, damage, opaque_regions)
    }

    #[inline]
    fn underlying_storage(&self, renderer: &mut R) -> Option<UnderlyingStorage<'_>> {
        self.element.underlying_storage(renderer)
    }
}

#[derive(Debug)]
pub struct CropRenderElement<E> {
    element: E,
    src: Rectangle<f64, Buffer>,
    crop_rect: Rectangle<i32, Physical>,
}

impl<E: Element> CropRenderElement<E> {
    pub fn from_element(
        element: E,
        scale: impl Into<Scale<f64>>,
        crop_rect: Rectangle<i32, Physical>,
    ) -> Option<Self> {
        let scale = scale.into();

        let element_geometry = element.geometry(scale);

        if let Some(intersection) = element_geometry.intersection(crop_rect) {
            if intersection.is_empty() {
                return None;
            }

            let mut element_relative_intersection = intersection;
            element_relative_intersection.loc -= element_geometry.loc;

            let element_src = element.src();
            let transform = element.transform();
            let physical_to_buffer_scale =
                element_src.size / transform.invert().transform_size(element_geometry.size).to_f64();

            let mut src = element_relative_intersection.to_f64().to_logical(1.0).to_buffer(
                physical_to_buffer_scale,
                transform,
                &element_geometry.size.to_f64().to_logical(1.0),
            );

            src.loc += element_src.loc;

            Some(CropRenderElement {
                element,
                src,
                crop_rect,
            })
        } else {
            None
        }
    }

    fn element_crop_rect(&self, scale: Scale<f64>) -> Option<Rectangle<i32, Physical>> {
        let element_geometry = self.element.geometry(scale);

        if let Some(mut intersection) = element_geometry.intersection(self.crop_rect) {
            if intersection.is_empty() {
                return None;
            }

            intersection.loc -= element_geometry.loc;
            Some(intersection)
        } else {
            None
        }
    }
}

impl<E: Element> Element for CropRenderElement<E> {
    fn id(&self) -> &Id {
        self.element.id()
    }

    fn current_commit(&self) -> crate::backend::renderer::utils::CommitCounter {
        self.element.current_commit()
    }

    fn src(&self) -> crate::utils::Rectangle<f64, crate::utils::Buffer> {
        self.src
    }

    fn geometry(&self, scale: Scale<f64>) -> crate::utils::Rectangle<i32, Physical> {
        let element_geometry = self.element.geometry(scale);
        if let Some(intersection) = element_geometry.intersection(self.crop_rect) {
            if intersection.is_empty() {
                return Default::default();
            }

            intersection
        } else {
            Default::default()
        }
    }

    fn transform(&self) -> crate::utils::Transform {
        self.element.transform()
    }

    fn damage_since(
        &self,
        scale: Scale<f64>,
        commit: Option<crate::backend::renderer::utils::CommitCounter>,
    ) -> DamageSet<i32, Physical> {
        if let Some(element_crop_rect) = self.element_crop_rect(scale) {
            self.element
                .damage_since(scale, commit)
                .into_iter()
                .flat_map(|rect| {
                    rect.intersection(element_crop_rect).map(|mut rect| {
                        rect.loc -= element_crop_rect.loc;
                        rect
                    })
                })
                .collect::<DamageSet<_, _>>()
        } else {
            Default::default()
        }
    }

    fn opaque_regions(&self, scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        if let Some(element_crop_rect) = self.element_crop_rect(scale) {
            self.element
                .opaque_regions(scale)
                .into_iter()
                .flat_map(|rect| {
                    rect.intersection(element_crop_rect).map(|mut rect| {
                        rect.loc -= element_crop_rect.loc;
                        rect
                    })
                })
                .collect::<OpaqueRegions<_, _>>()
        } else {
            Default::default()
        }
    }

    fn alpha(&self) -> f32 {
        self.element.alpha()
    }

    fn kind(&self) -> Kind {
        self.element.kind()
    }
}

impl<R: Renderer, E: RenderElement<R>> RenderElement<R> for CropRenderElement<E> {
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: crate::utils::Rectangle<f64, crate::utils::Buffer>,
        dst: crate::utils::Rectangle<i32, crate::utils::Physical>,
        damage: &[crate::utils::Rectangle<i32, crate::utils::Physical>],
        opaque_regions: &[crate::utils::Rectangle<i32, crate::utils::Physical>],
    ) -> Result<(), R::Error> {
        self.element.draw(frame, src, dst, damage, opaque_regions)
    }

    #[inline]
    fn underlying_storage(&self, renderer: &mut R) -> Option<UnderlyingStorage<'_>> {
        self.element.underlying_storage(renderer)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Relocate {
    Absolute,
    Relative,
}

#[derive(Debug)]
pub struct RelocateRenderElement<E> {
    element: E,
    relocate: Relocate,
    location: Point<i32, Physical>,
}

impl<E: Element> RelocateRenderElement<E> {
    pub fn from_element(element: E, location: impl Into<Point<i32, Physical>>, relocate: Relocate) -> Self {
        let location = location.into();

        RelocateRenderElement {
            element,
            location,
            relocate,
        }
    }
}

impl<E: Element> Element for RelocateRenderElement<E> {
    fn id(&self) -> &Id {
        self.element.id()
    }

    fn current_commit(&self) -> crate::backend::renderer::utils::CommitCounter {
        self.element.current_commit()
    }

    fn src(&self) -> Rectangle<f64, Buffer> {
        self.element.src()
    }

    fn geometry(&self, scale: Scale<f64>) -> Rectangle<i32, Physical> {
        let mut geo = self.element.geometry(scale);
        match self.relocate {
            Relocate::Absolute => geo.loc = self.location,
            Relocate::Relative => geo.loc += self.location,
        }
        geo
    }

    fn location(&self, scale: Scale<f64>) -> Point<i32, Physical> {
        match self.relocate {
            Relocate::Absolute => self.location,
            Relocate::Relative => self.element.location(scale) + self.location,
        }
    }

    fn transform(&self) -> crate::utils::Transform {
        self.element.transform()
    }

    fn damage_since(
        &self,
        scale: Scale<f64>,
        commit: Option<crate::backend::renderer::utils::CommitCounter>,
    ) -> DamageSet<i32, Physical> {
        self.element.damage_since(scale, commit)
    }

    fn opaque_regions(&self, scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        self.element.opaque_regions(scale)
    }

    fn alpha(&self) -> f32 {
        self.element.alpha()
    }

    fn kind(&self) -> Kind {
        self.element.kind()
    }
}

impl<R: Renderer, E: RenderElement<R>> RenderElement<R> for RelocateRenderElement<E> {
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: crate::utils::Rectangle<f64, crate::utils::Buffer>,
        dst: crate::utils::Rectangle<i32, crate::utils::Physical>,
        damage: &[crate::utils::Rectangle<i32, crate::utils::Physical>],
        opaque_regions: &[crate::utils::Rectangle<i32, crate::utils::Physical>],
    ) -> Result<(), R::Error> {
        self.element.draw(frame, src, dst, damage, opaque_regions)
    }

    #[inline]
    fn underlying_storage(&self, renderer: &mut R) -> Option<UnderlyingStorage<'_>> {
        self.element.underlying_storage(renderer)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ConstrainScaleBehavior {
    Fit,
    Zoom,
    Stretch,
    CutOff,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ConstrainAlign: u8 {
        const TOP = 0b000001;
        const LEFT = 0b000010;
        const RIGHT = 0b000100;
        const BOTTOM = 0b001000;
        const TOP_LEFT = Self::TOP.bits() | Self::LEFT.bits();
        const TOP_RIGHT = Self::TOP.bits() | Self::RIGHT.bits();
        const BOTTOM_LEFT = Self::BOTTOM.bits() | Self::LEFT.bits();
        const BOTTOM_RIGHT = Self::BOTTOM.bits() | Self::RIGHT.bits();
        const CENTER = Self::TOP.bits() | Self::LEFT.bits() | Self::BOTTOM.bits() | Self::RIGHT.bits();
    }
}

#[profiling::function]
#[allow(clippy::too_many_arguments)]
pub fn constrain_as_render_elements<R, E, C>(
    element: &E,
    renderer: &mut R,
    location: impl Into<Point<i32, Physical>>,
    alpha: f32,
    constrain: Rectangle<i32, Physical>,
    reference: Rectangle<i32, Physical>,
    behavior: ConstrainScaleBehavior,
    align: ConstrainAlign,
    output_scale: impl Into<Scale<f64>>,
) -> impl Iterator<Item = C>
where
    R: Renderer,
    E: AsRenderElements<R>,
    C: From<
        CropRenderElement<
            RelocateRenderElement<RescaleRenderElement<<E as AsRenderElements<R>>::RenderElement>>,
        >,
    >,
{
    let location = location.into();
    let output_scale = output_scale.into();
    let elements: Vec<<E as AsRenderElements<R>>::RenderElement> =
        AsRenderElements::<R>::render_elements(element, renderer, location, output_scale, alpha);
    constrain_render_elements(
        elements,
        location,
        constrain,
        reference,
        behavior,
        align,
        output_scale,
    )
    .map(C::from)
}

#[profiling::function]
pub fn constrain_render_elements<E>(
    elements: impl IntoIterator<Item = E>,
    origin: impl Into<Point<i32, Physical>>,
    constrain: Rectangle<i32, Physical>,
    reference: Rectangle<i32, Physical>,
    behavior: ConstrainScaleBehavior,
    align: ConstrainAlign,
    scale: impl Into<Scale<f64>>,
) -> impl Iterator<Item = CropRenderElement<RelocateRenderElement<RescaleRenderElement<E>>>>
where
    E: Element,
{
    let location = origin.into();
    let scale = scale.into();

    let element_scale = match behavior {
        ConstrainScaleBehavior::Fit => {
            let reference = reference.to_f64();
            let size = constrain.size.to_f64();
            let element_scale: Scale<f64> = size / reference.size;
            Scale::from(f64::min(element_scale.x, element_scale.y))
        }
        ConstrainScaleBehavior::Zoom => {
            let reference = reference.to_f64();
            let size = constrain.size.to_f64();
            let element_scale: Scale<f64> = size / reference.size;
            Scale::from(f64::max(element_scale.x, element_scale.y))
        }
        ConstrainScaleBehavior::Stretch => {
            let reference = reference.to_f64();
            let size = constrain.size.to_f64();
            size / reference.size
        }
        ConstrainScaleBehavior::CutOff => Scale::from(1.0),
    };

    let scaled_reference = reference.to_f64().upscale(element_scale);

    let top_offset: f64 = if align.contains(ConstrainAlign::TOP | ConstrainAlign::BOTTOM) {
        (constrain.size.h as f64 - scaled_reference.size.h) / 2f64
    } else if align.contains(ConstrainAlign::BOTTOM) {
        constrain.size.h as f64 - scaled_reference.size.h
    } else {
        0f64
    };

    let left_offset: f64 = if align.contains(ConstrainAlign::LEFT | ConstrainAlign::RIGHT) {
        (constrain.size.w as f64 - scaled_reference.size.w) / 2f64
    } else if align.contains(ConstrainAlign::RIGHT) {
        constrain.size.w as f64 - scaled_reference.size.w
    } else {
        0f64
    };

    let align_offset: Point<f64, Physical> = Point::from((left_offset, top_offset));

    let reference_offset =
        reference.loc.to_f64() - Point::from((scaled_reference.loc.x, scaled_reference.loc.y));

    let offset = (reference_offset + align_offset).to_i32_round();

    elements
        .into_iter()
        .map(move |e| RescaleRenderElement::from_element(e, location, element_scale))
        .map(move |e| RelocateRenderElement::from_element(e, offset, Relocate::Relative))
        .filter_map(move |e| CropRenderElement::from_element(e, scale, constrain))
}
