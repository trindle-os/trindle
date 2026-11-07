//! Windows that move rather than jump.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use smithay::backend::renderer::element::surface::WaylandSurfaceTexture;
use smithay::backend::renderer::element::utils::{Relocate, RelocateRenderElement};
use smithay::backend::renderer::element::{
    AsRenderElements, Element, Id, Kind, RenderElement, UnderlyingStorage,
};
use smithay::backend::renderer::utils::{CommitCounter, DamageSet, OpaqueRegions};
use smithay::backend::renderer::{Color32F, Frame, ImportAll, ImportMem, Renderer, RendererSuper, Texture};
use smithay::output::Output;
use smithay::reexports::wayland_server::backend::ObjectId;
use smithay::utils::{Buffer as BufferCoords, Logical, Physical, Point, Rectangle, Scale, Transform};

use crate::shell::{WindowElement, WindowRenderElement};

const OPEN: Duration = Duration::from_millis(200);
const AWAY: Duration = Duration::from_millis(240);
const CLOSE: Duration = Duration::from_millis(200);
const SMALL: f64 = 0.92;
const TINY: f64 = 0.08;

static ENABLED: AtomicBool = AtomicBool::new(true);

pub fn disable() {
    ENABLED.store(false, Ordering::Relaxed);
}

fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed) && std::env::var("TRINDLE_DRAWING").map_or(true, |d| d != "light")
}

fn span(d: Duration) -> Duration {
    let slow = std::env::var("TRINDLE_SLOW").ok().and_then(|v| v.parse::<u32>().ok()).unwrap_or(1).max(1);
    d * slow
}

fn ease_out(t: f64) -> f64 {
    1.0 - (1.0 - t.clamp(0.0, 1.0)).powi(3)
}

pub struct Faded<R: Renderer> {
    inner: Rc<WindowRenderElement<R>>,
    alpha: f32,
}

impl<R> Element for Faded<R>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: 'static,
{
    fn id(&self) -> &Id {
        self.inner.id()
    }
    fn current_commit(&self) -> CommitCounter {
        self.inner.current_commit()
    }
    fn src(&self) -> Rectangle<f64, BufferCoords> {
        self.inner.src()
    }
    fn transform(&self) -> Transform {
        self.inner.transform()
    }
    fn geometry(&self, scale: Scale<f64>) -> Rectangle<i32, Physical> {
        self.inner.geometry(scale)
    }
    fn damage_since(&self, scale: Scale<f64>, commit: Option<CommitCounter>) -> DamageSet<i32, Physical> {
        self.inner.damage_since(scale, commit)
    }
    fn opaque_regions(&self, scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        if self.alpha < 1.0 {
            OpaqueRegions::default()
        } else {
            self.inner.opaque_regions(scale)
        }
    }
    fn alpha(&self) -> f32 {
        self.inner.alpha() * self.alpha
    }
    fn kind(&self) -> Kind {
        self.inner.kind()
    }
}

impl<R> RenderElement<R> for Faded<R>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: 'static,
{
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: Rectangle<f64, BufferCoords>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<(), R::Error> {
        if self.alpha >= 1.0 {
            return self.inner.draw(frame, src, dst, damage, opaque_regions);
        }
        let alpha = self.alpha();
        match &*self.inner {
            WindowRenderElement::Window(surface) => match surface.texture() {
                WaylandSurfaceTexture::Texture(texture) => {
                    frame.render_texture_from_to(texture, src, dst, damage, &[], surface.transform(), alpha)
                }
                WaylandSurfaceTexture::SolidColor(colour) => frame.draw_solid(dst, damage, *colour * alpha),
            },
            WindowRenderElement::Decoration(bar) => frame.draw_solid(dst, damage, bar.color() * alpha),
            WindowRenderElement::Picture(picture) => {
                frame.render_texture_from_to(picture.texture(), src, dst, damage, &[], Transform::Normal, alpha)
            }
            other => other.draw(frame, src, dst, damage, opaque_regions),
        }
    }

    fn underlying_storage(&self, renderer: &mut R) -> Option<UnderlyingStorage<'_>> {
        if self.alpha >= 1.0 {
            self.inner.underlying_storage(renderer)
        } else {
            None
        }
    }
}

pub struct Scaled<E> {
    inner: E,
    origin: Point<i32, Physical>,
    by: f64,
}

impl<E> Scaled<E> {
    fn new(inner: E, origin: Point<i32, Physical>, by: f64) -> Self {
        Scaled { inner, origin, by }
    }

    fn place(&self, point: Point<i32, Physical>) -> Point<i32, Physical> {
        ((point - self.origin).to_f64().upscale(self.by) + self.origin.to_f64()).to_i32_round()
    }
}

impl<E: Element> Element for Scaled<E> {
    fn id(&self) -> &Id {
        self.inner.id()
    }
    fn current_commit(&self) -> CommitCounter {
        self.inner.current_commit()
    }
    fn src(&self) -> Rectangle<f64, BufferCoords> {
        self.inner.src()
    }
    fn transform(&self) -> Transform {
        self.inner.transform()
    }
    fn geometry(&self, scale: Scale<f64>) -> Rectangle<i32, Physical> {
        let geo = self.inner.geometry(scale);
        if self.by == 1.0 {
            return geo;
        }
        let mut out = Rectangle::from_extremities(self.place(geo.loc), self.place(geo.loc + geo.size.to_point()));
        out.size.w = out.size.w.max(geo.size.w.min(1));
        out.size.h = out.size.h.max(geo.size.h.min(1));
        out
    }
    fn damage_since(&self, scale: Scale<f64>, commit: Option<CommitCounter>) -> DamageSet<i32, Physical> {
        let damage = self.inner.damage_since(scale, commit);
        if self.by == 1.0 {
            return damage;
        }
        damage.into_iter().map(|r| r.to_f64().upscale(self.by).to_i32_up()).collect()
    }
    fn opaque_regions(&self, scale: Scale<f64>) -> OpaqueRegions<i32, Physical> {
        let opaque = self.inner.opaque_regions(scale);
        if self.by == 1.0 {
            return opaque;
        }
        opaque.into_iter().map(|r| r.to_f64().upscale(self.by).to_i32_down()).collect()
    }
    fn alpha(&self) -> f32 {
        self.inner.alpha()
    }
    fn kind(&self) -> Kind {
        self.inner.kind()
    }
}

impl<R: Renderer, E: RenderElement<R>> RenderElement<R> for Scaled<E> {
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: Rectangle<f64, BufferCoords>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<(), R::Error> {
        self.inner.draw(frame, src, dst, damage, opaque_regions)
    }

    fn underlying_storage(&self, renderer: &mut R) -> Option<UnderlyingStorage<'_>> {
        self.inner.underlying_storage(renderer)
    }
}

pub type Moving<R> = RelocateRenderElement<Scaled<Faded<R>>>;

#[derive(Clone)]
pub struct Kept<T> {
    id: Id,
    commit: CommitCounter,
    texture: Option<T>,
    colour: Color32F,
    src: Rectangle<f64, BufferCoords>,
    geo: Rectangle<i32, Physical>,
    transform: Transform,
    alpha: f32,
}

impl<T: Texture + 'static> Element for Kept<T> {
    fn id(&self) -> &Id {
        &self.id
    }
    fn current_commit(&self) -> CommitCounter {
        self.commit
    }
    fn src(&self) -> Rectangle<f64, BufferCoords> {
        self.src
    }
    fn transform(&self) -> Transform {
        self.transform
    }
    fn geometry(&self, _scale: Scale<f64>) -> Rectangle<i32, Physical> {
        self.geo
    }
    fn alpha(&self) -> f32 {
        self.alpha
    }
}

impl<R, T> RenderElement<R> for Kept<T>
where
    R: Renderer<TextureId = T>,
    T: Texture + 'static,
{
    fn draw(
        &self,
        frame: &mut R::Frame<'_, '_>,
        src: Rectangle<f64, BufferCoords>,
        dst: Rectangle<i32, Physical>,
        damage: &[Rectangle<i32, Physical>],
        _opaque_regions: &[Rectangle<i32, Physical>],
    ) -> Result<(), R::Error> {
        match &self.texture {
            Some(t) => frame.render_texture_from_to(t, src, dst, damage, &[], self.transform, self.alpha),
            None => frame.draw_solid(dst, damage, self.colour * self.alpha),
        }
    }
}

pub type KeptMoving<R> = RelocateRenderElement<Scaled<Kept<<R as RendererSuper>::TextureId>>>;

fn keep<R>(piece: &WindowRenderElement<R>, scale: Scale<f64>) -> Kept<R::TextureId>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + 'static,
{
    let (texture, colour) = match piece {
        WindowRenderElement::Window(surface) => match surface.texture() {
            WaylandSurfaceTexture::Texture(t) => (Some(t.clone()), Color32F::TRANSPARENT),
            WaylandSurfaceTexture::SolidColor(c) => (None, *c),
        },
        WindowRenderElement::Decoration(bar) => (None, bar.color()),
        WindowRenderElement::Picture(picture) => (Some(picture.texture().clone()), Color32F::TRANSPARENT),
        _ => (None, Color32F::TRANSPARENT),
    };
    Kept {
        id: Id::new(),
        commit: CommitCounter::default(),
        texture,
        colour,
        src: piece.src(),
        geo: piece.geometry(scale),
        transform: piece.transform(),
        alpha: piece.alpha(),
    }
}

fn moving<R>(pieces: &[Rc<WindowRenderElement<R>>], alpha: f32, centre: Point<i32, Physical>, by: f64,
             shift: Point<i32, Physical>) -> Vec<Moving<R>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: 'static,
{
    pieces.iter()
        .map(|p| Faded { inner: p.clone(), alpha })
        .map(|e| Scaled::new(e, centre, by))
        .map(|e| RelocateRenderElement::from_element(e, shift, Relocate::Relative))
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind2 {
    Opening,
    Restoring(Point<f64, Logical>),
}

#[derive(Default)]
struct Motion(RefCell<Option<(Instant, Kind2)>>);

thread_local! {
    static LAST: RefCell<HashMap<(String, ObjectId), Box<dyn std::any::Any>>> = RefCell::new(HashMap::new());
    static GOING: RefCell<Vec<(ObjectId, Instant, Option<Point<f64, Logical>>)>> = const { RefCell::new(Vec::new()) };
    static MOVED: std::cell::Cell<Option<Instant>> = const { std::cell::Cell::new(None) };
}

fn moved() {
    MOVED.with(|m| m.set(Some(Instant::now())));
}

pub fn moving_now() -> bool {
    MOVED.with(|m| m.get()).is_some_and(|at| at.elapsed() < Duration::from_millis(50))
}

fn key(window: &WindowElement) -> Option<ObjectId> {
    use smithay::reexports::wayland_server::Resource;
    window.wl_surface().map(|s| s.id())
}

fn set(window: &WindowElement, kind: Option<Kind2>) {
    let motion = window.user_data().get_or_insert(Motion::default);
    *motion.0.borrow_mut() = kind.map(|k| (Instant::now(), k));
}

fn motion(window: &WindowElement) -> Option<(Instant, Kind2)> {
    window.user_data().get::<Motion>().and_then(|m| *m.0.borrow())
}

pub fn tab_point(output_geo: Rectangle<i32, Logical>) -> Point<f64, Logical> {
    (output_geo.loc.x as f64 + output_geo.size.w as f64 / 2.0, (output_geo.loc.y + output_geo.size.h) as f64).into()
}

pub fn restoring(window: &WindowElement, tab: Point<f64, Logical>) {
    if let Some(id) = key(window) {
        GOING.with(|g| g.borrow_mut().retain(|(w, _, _)| *w != id));
    }
    if enabled() {
        set(window, Some(Kind2::Restoring(tab)));
    }
}

pub fn minimizing(window: &WindowElement, tab: Point<f64, Logical>) {
    go(window, Some(tab));
}

pub fn closing(window: &WindowElement) {
    go(window, None);
}

fn go(window: &WindowElement, tab: Option<Point<f64, Logical>>) {
    let Some(id) = key(window) else { return };
    if !enabled() {
        if tab.is_none() {
            forget(&id);
        }
        return;
    }
    set(window, None);
    GOING.with(|g| {
        let mut g = g.borrow_mut();
        g.retain(|(w, _, _)| *w != id);
        g.push((id, Instant::now(), tab));
    });
}

fn forget(id: &ObjectId) {
    LAST.with(|l| l.borrow_mut().retain(|(_, w), _| w != id));
}

fn middle<R>(pieces: &[Rc<WindowRenderElement<R>>], scale: Scale<f64>) -> Point<i32, Physical>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: 'static,
{
    let area = pieces.iter().map(|p| p.geometry(scale)).reduce(|a, b| a.merge(b)).unwrap_or_default();
    area.loc + Point::from((area.size.w / 2, area.size.h / 2))
}

fn to_local(point: Point<f64, Logical>, output_geo: Rectangle<i32, Logical>, scale: Scale<f64>) -> Point<i32, Physical> {
    (point - output_geo.loc.to_f64()).to_physical(scale).to_i32_round()
}

pub fn window<R>(
    output: &Output,
    output_geo: Rectangle<i32, Logical>,
    window: &WindowElement,
    renderer: &mut R,
    location: Point<i32, Physical>,
    scale: Scale<f64>,
) -> Vec<Moving<R>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Send + 'static,
{
    if window.user_data().get::<Motion>().is_none() {
        set(window, enabled().then_some(Kind2::Opening));
    }
    let pieces: Vec<Rc<WindowRenderElement<R>>> =
        AsRenderElements::<R>::render_elements::<WindowRenderElement<R>>(window, renderer, location, scale, 1.0)
            .into_iter().map(Rc::new).collect();
    if let Some(id) = key(window) {
        let kept: Vec<Kept<R::TextureId>> = pieces.iter().map(|p| keep(p, scale)).collect();
        LAST.with(|l| l.borrow_mut().insert((output.name(), id), Box::new(kept)));
    }
    let centre = middle(&pieces, scale);
    let (alpha, by, shift) = match motion(window) {
        Some((start, kind)) => {
            let length = span(match kind {
                Kind2::Opening => OPEN,
                _ => AWAY,
            });
            let t = start.elapsed().as_secs_f64() / length.as_secs_f64();
            if t >= 1.0 {
                set(window, None);
                (1.0, 1.0, Point::default())
            } else {
                moved();
                let p = ease_out(t);
                match kind {
                    Kind2::Opening => (p as f32, SMALL + (1.0 - SMALL) * p, Point::default()),
                    Kind2::Restoring(tab) => {
                        let tab = to_local(tab, output_geo, scale);
                        let away = tab - centre;
                        let q = 1.0 - p;
                        let shift = Point::from(((away.x as f64 * q) as i32, (away.y as f64 * q) as i32));
                        (p as f32, TINY + (1.0 - TINY) * p, shift)
                    }
                }
            }
        }
        None => (1.0, 1.0, Point::default()),
    };
    moving(&pieces, alpha, centre, by, shift)
}

pub fn going<R>(output: &Output, output_geo: Rectangle<i32, Logical>, scale: Scale<f64>) -> Vec<KeptMoving<R>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + 'static,
{
    let mut out = Vec::new();
    let going: Vec<(ObjectId, Instant, Option<Point<f64, Logical>>)> = GOING.with(|g| g.borrow().clone());
    let mut done = Vec::new();
    for (id, start, tab) in going {
        let length = span(if tab.is_some() { AWAY } else { CLOSE });
        let t = start.elapsed().as_secs_f64() / length.as_secs_f64();
        if t >= 1.0 {
            done.push((id, tab.is_none()));
            continue;
        }
        let kept: Option<Vec<Kept<R::TextureId>>> = LAST.with(|l| {
            l.borrow().get(&(output.name(), id.clone()))
                .and_then(|b| b.downcast_ref::<Vec<Kept<R::TextureId>>>()).cloned()
        });
        let Some(kept) = kept else { continue };
        moved();
        let p = ease_out(t);
        let area = kept.iter().map(|k| k.geo).reduce(|a, b| a.merge(b)).unwrap_or_default();
        let centre = area.loc + Point::from((area.size.w / 2, area.size.h / 2));
        let (by, shift) = match tab {
            None => (1.0 - (1.0 - SMALL) * p, Point::default()),
            Some(tab) => {
                let to = to_local(tab, output_geo, scale) - centre;
                (1.0 - (1.0 - TINY) * p, Point::from(((to.x as f64 * p) as i32, (to.y as f64 * p) as i32)))
            }
        };
        let fade = (1.0 - p) as f32;
        out.extend(kept.into_iter().map(|mut k| {
            k.alpha *= fade;
            let e = Scaled::new(k, centre, by);
            RelocateRenderElement::from_element(e, shift, Relocate::Relative)
        }));
    }
    if !done.is_empty() {
        GOING.with(|g| g.borrow_mut().retain(|(w, _, _)| !done.iter().any(|(d, _)| d == w)));
        for (id, closed) in &done {
            if *closed {
                forget(id);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use smithay::backend::renderer::element::solid::{SolidColorBuffer, SolidColorRenderElement};

    fn piece(buffer: &SolidColorBuffer, x: i32, y: i32) -> SolidColorRenderElement {
        SolidColorRenderElement::from_buffer(buffer, (x, y), 1.0, 1.0, Kind::Unspecified)
    }

    #[test]
    fn pieces_that_meet_still_meet_when_scaled() {
        let corner = SolidColorBuffer::new((11, 11), [1.0; 4]);
        let side = SolidColorBuffer::new((300, 1), [1.0; 4]);
        let origin = Point::from((400, 300));
        for step in 0..=100 {
            let by = SMALL + (1.0 - SMALL) * step as f64 / 100.0;
            for x in [103, 217, 350] {
                let a = Scaled::new(piece(&corner, x, 120), origin, by).geometry(Scale::from(1.0));
                let b = Scaled::new(piece(&side, x + 11, 120), origin, by).geometry(Scale::from(1.0));
                assert_eq!(a.loc.x + a.size.w, b.loc.x, "by {by} at {x}");
                assert_eq!(a.loc.y, b.loc.y);
                assert_eq!(b.size.h, 1, "by {by} at {x}");
            }
        }
    }
}
