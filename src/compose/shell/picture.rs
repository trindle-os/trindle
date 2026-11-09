//! A piece of a window drawn from a picture the compositor made itself:
//! the title bar it draws for a program, and the corners of the line round
//! a window that draws its own. Each keeps the texture it is drawn from in
//! plain sight, so a window can be faded as it opens and still be drawn
//! from what it last showed after it has closed.

use smithay::backend::renderer::element::{Element, Id, RenderElement};
use smithay::backend::renderer::utils::CommitCounter;
use smithay::backend::renderer::{Frame, Renderer};
use smithay::utils::{Buffer, Logical, Physical, Point, Rectangle, Scale, Transform};

pub(super) fn whole_scale(scale: Scale<f64>) -> u32 {
    scale.x.max(scale.y).ceil().max(1.0) as u32
}

pub(super) struct Placer {
    pub(super) location: Point<i32, Physical>,
    pub(super) scale: Scale<f64>,
}

impl Placer {
    pub(super) fn at(&self, x: i32, y: i32) -> Point<i32, Physical> {
        self.location + Point::<i32, Logical>::from((x, y)).to_physical_precise_round(self.scale)
    }

    pub(super) fn span(&self, x: i32, y: i32, pw: i32, ph: i32) -> Rectangle<i32, Physical> {
        let a = self.at(x, y);
        Rectangle::new(a, (self.at(x + pw, y + ph) - a).to_size())
    }
}

pub struct Picture<R: Renderer> {
    id: Id,
    commit: CommitCounter,
    texture: R::TextureId,
    src: Rectangle<f64, Buffer>,
    geo: Rectangle<i32, Physical>,
    alpha: f32,
}

impl<R: Renderer> Picture<R> {

    pub fn new(
        id: Id,
        commit: CommitCounter,
        texture: R::TextureId,
        src: Rectangle<f64, Buffer>,
        geo: Rectangle<i32, Physical>,
        alpha: f32,
    ) -> Self {
        Picture { id, commit, texture, src, geo, alpha }
    }

    pub fn texture(&self) -> &R::TextureId {
        &self.texture
    }
}

impl<R: Renderer> std::fmt::Debug for Picture<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Picture").field("geo", &self.geo).finish()
    }
}

impl<R: Renderer> Element for Picture<R> {
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
        self.geo
    }
    fn transform(&self) -> Transform {
        Transform::Normal
    }
    fn alpha(&self) -> f32 {
        self.alpha
    }
}

impl<R> RenderElement<R> for Picture<R>
where
    R: Renderer,
    R::TextureId: 'static,
{
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: Rectangle<f64, Buffer>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        _opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<(), R::Error> {
        frame.render_texture_from_to(&self.texture, src, dst, damage, &[], Transform::Normal, self.alpha)
    }
}
