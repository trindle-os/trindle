//! Element to render a solid color.

use crate::{
    backend::renderer::{
        utils::{CommitCounter, OpaqueRegions},
        Color32F, Frame, Renderer,
    },
    utils::{Buffer, Logical, Physical, Point, Rectangle, Scale, Size, Transform},
};

use super::{AsRenderElements, Element, Id, Kind, RenderElement};

#[derive(Debug, Clone)]
pub struct SolidColorBuffer {
    id: Id,
    size: Size<i32, Logical>,
    commit: CommitCounter,
    color: Color32F,
}

impl Default for SolidColorBuffer {
    fn default() -> Self {
        Self {
            id: Id::new(),
            size: Default::default(),
            commit: Default::default(),
            color: Default::default(),
        }
    }
}

impl SolidColorBuffer {
    pub fn new(size: impl Into<Size<i32, Logical>>, color: impl Into<Color32F>) -> Self {
        SolidColorBuffer {
            id: Id::new(),
            color: color.into(),
            commit: CommitCounter::default(),
            size: size.into(),
        }
    }

    pub fn resize(&mut self, size: impl Into<Size<i32, Logical>>) {
        let size = size.into();
        if size != self.size {
            self.size = size;
            self.commit.increment();
        }
    }

    pub fn set_color(&mut self, color: impl Into<Color32F>) {
        let color = color.into();
        if color != self.color {
            self.color = color;
            self.commit.increment();
        }
    }

    pub fn update(&mut self, size: impl Into<Size<i32, Logical>>, color: impl Into<Color32F>) {
        let color = color.into();
        let size = size.into();
        if size != self.size || color != self.color {
            self.size = size;
            self.color = color;
            self.commit.increment();
        }
    }

    pub fn color(&self) -> Color32F {
        self.color
    }
}

#[derive(Debug, Clone)]
pub struct SolidColorRenderElement {
    id: Id,
    geometry: Rectangle<i32, Physical>,
    src: Rectangle<f64, Buffer>,
    opaque_regions: Vec<Rectangle<i32, Physical>>,
    commit: CommitCounter,
    color: Color32F,
    kind: Kind,
}

impl SolidColorRenderElement {
    pub fn from_buffer(
        buffer: &SolidColorBuffer,
        location: impl Into<Point<i32, Physical>>,
        scale: impl Into<Scale<f64>>,
        alpha: f32,
        kind: Kind,
    ) -> Self {
        let geo = Rectangle::new(location.into(), buffer.size.to_physical_precise_round(scale));
        let color = buffer.color * alpha;
        Self::new(buffer.id.clone(), geo, buffer.commit, color, kind)
    }

    pub fn new(
        id: impl Into<Id>,
        geometry: Rectangle<i32, Physical>,
        commit: impl Into<CommitCounter>,
        color: impl Into<Color32F>,
        kind: Kind,
    ) -> Self {
        let color = color.into();
        let src = Rectangle::from_size(geometry.size)
            .to_f64()
            .to_logical(1f64)
            .to_buffer(1f64, Transform::Normal, &geometry.size.to_f64().to_logical(1f64));
        let opaque_regions = if color.is_opaque() {
            vec![Rectangle::from_size(geometry.size)]
        } else {
            vec![]
        };
        SolidColorRenderElement {
            id: id.into(),
            geometry,
            src,
            opaque_regions,
            commit: commit.into(),
            color,
            kind,
        }
    }

    pub fn color(&self) -> Color32F {
        self.color
    }
}

impl Element for SolidColorRenderElement {
    fn id(&self) -> &Id {
        &self.id
    }

    fn current_commit(&self) -> CommitCounter {
        self.commit
    }

    fn src(&self) -> Rectangle<f64, Buffer> {
        self.src
    }

    fn geometry(&self, _scale: Scale<f64>) -> Rectangle<i32, Physical> {
        self.geometry
    }

    fn opaque_regions(&self, _scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        OpaqueRegions::from_slice(&self.opaque_regions)
    }

    fn alpha(&self) -> f32 {
        self.color.a()
    }

    fn kind(&self) -> Kind {
        self.kind
    }
}

impl<R: Renderer> RenderElement<R> for SolidColorRenderElement {
    #[profiling::function]
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        _src: Rectangle<f64, Buffer>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        _opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<(), R::Error> {
        frame.draw_solid(dst, damage, self.color)
    }

    #[inline]
    fn underlying_storage(&self, _renderer: &mut R) -> Option<super::UnderlyingStorage<'_>> {
        None
    }
}

impl<R> AsRenderElements<R> for SolidColorBuffer
where
    R: Renderer,
{
    type RenderElement = SolidColorRenderElement;

    fn render_elements<C: From<Self::RenderElement>>(
        &self,
        _renderer: &mut R,
        location: crate::utils::Point<i32, crate::utils::Physical>,
        scale: crate::utils::Scale<f64>,
        alpha: f32,
    ) -> Vec<C> {
        vec![SolidColorRenderElement::from_buffer(self, location, scale, alpha, Kind::Unspecified).into()]
    }
}
