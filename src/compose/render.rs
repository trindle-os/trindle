//! Origin: began as a file of anvil, Smithay's example compositor (MIT, Victor Berger and
//! Victoria Brekenfeld). About 24% of this file's lines still read as
//! they did there; the rest is Trindle's own.

use std::{sync::Mutex, time::Duration};

use smithay::{
    backend::renderer::{
        damage::{Error as OutputDamageTrackerError, OutputDamageTracker, RenderOutputResult},
        element::{surface::WaylandSurfaceRenderElement, AsRenderElements, RenderElement, Wrap},
        Color32F, ImportAll, ImportMem, Renderer, Texture,
    },
    desktop::space::{Space, SpaceRenderElements, SurfaceTree},
    input::pointer::{CursorImageAttributes, CursorImageStatus},
    output::Output,
    utils::{IsAlive, Logical, Point, Rectangle, Scale},
    wayland::compositor,
};

use crate::{
    drawing::{PointerElement, PointerRenderElement, CLEAR_COLOR, CLEAR_COLOR_FULLSCREEN},
    lock::{SessionLock, LOCK_COLOR},
    shell::{FullscreenSurface, WindowElement, WindowRenderElement},
    state::DndIcon,
};

pub const ASSUMED_REFRESH: i32 = 60_000;

const FRAME_MILLIS_PER_MILLIHERTZ: u64 = 1_000_000;

pub fn frame_duration(refresh: i32) -> Duration {
    Duration::from_secs_f64(1_000f64 / refresh as f64)
}

pub fn frames_to_millis(frames: u64, refresh: u64) -> u64 {
    frames * FRAME_MILLIS_PER_MILLIHERTZ / refresh
}

pub fn frames_in(wait: Duration, refresh: u64) -> u64 {
    (wait.as_millis() as u64 * refresh).div_ceil(FRAME_MILLIS_PER_MILLIHERTZ)
}

smithay::backend::renderer::element::render_elements! {
    pub CustomRenderElements<R> where
        R: ImportAll + ImportMem;
    Pointer=PointerRenderElement<R>,
    Surface=WaylandSurfaceRenderElement<R>,
    Dots=smithay::backend::renderer::element::memory::MemoryRenderBufferRenderElement<R>,
}

impl<R: Renderer> std::fmt::Debug for CustomRenderElements<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pointer(arg0) => f.debug_tuple("Pointer").field(arg0).finish(),
            Self::Surface(arg0) => f.debug_tuple("Surface").field(arg0).finish(),
            Self::Dots(arg0) => f.debug_tuple("Dots").field(arg0).finish(),
            Self::_GenericCatcher(arg0) => f.debug_tuple("_GenericCatcher").field(arg0).finish(),
        }
    }
}

smithay::backend::renderer::element::render_elements! {
    pub OutputRenderElements<R, E> where R: ImportAll + ImportMem;
    Space=SpaceRenderElements<R, E>,
    Window=Wrap<E>,
    Custom=CustomRenderElements<R>,
    Moving=crate::animate::Moving<R>,
    Kept=crate::animate::KeptMoving<R>,
}

impl<R: Renderer + ImportAll + ImportMem, E: RenderElement<R> + std::fmt::Debug> std::fmt::Debug
    for OutputRenderElements<R, E>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Space(arg0) => f.debug_tuple("Space").field(arg0).finish(),
            Self::Window(arg0) => f.debug_tuple("Window").field(arg0).finish(),
            Self::Custom(arg0) => f.debug_tuple("Custom").field(arg0).finish(),
            Self::Moving(_) => f.debug_tuple("Moving").finish(),
            Self::Kept(_) => f.debug_tuple("Kept").finish(),
            Self::_GenericCatcher(arg0) => f.debug_tuple("_GenericCatcher").field(arg0).finish(),
        }
    }
}

pub fn pointer_elements<R>(
    renderer: &mut R,
    output_geometry: Rectangle<i32, Logical>,
    scale: Scale<f64>,
    pointer_location: Point<f64, Logical>,
    pointer_element: &PointerElement,
    cursor_status: &CursorImageStatus,
    dnd_icon: Option<&DndIcon>,
) -> Vec<CustomRenderElements<R>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Texture + Clone + Send + 'static,
{
    let mut elements = Vec::new();
    crate::indicators::pointer_at(pointer_location);
    if !output_geometry.to_f64().contains(pointer_location) {
        return elements;
    }

    let cursor_hotspot = if let CursorImageStatus::Surface(ref surface) = cursor_status {
        compositor::with_states(surface, |states| {
            states
                .data_map
                .get::<Mutex<CursorImageAttributes>>()
                .unwrap()
                .lock()
                .unwrap()
                .hotspot
        })
    } else {
        (0, 0).into()
    };
    let cursor_pos = pointer_location - output_geometry.loc.to_f64();

    elements.extend(pointer_element.render_elements(
        renderer,
        (cursor_pos - cursor_hotspot.to_f64())
            .to_physical(scale)
            .to_i32_round(),
        scale,
        1.0,
    ));

    if let Some(icon) = dnd_icon {
        if icon.surface.alive() {
            let dnd_icon_pos = (cursor_pos + icon.offset.to_f64())
                .to_physical(scale)
                .to_i32_round();
            elements.extend(AsRenderElements::<R>::render_elements(
                &SurfaceTree::from_surface(&icon.surface),
                renderer,
                dnd_icon_pos,
                scale,
                1.0,
            ));
        }
    }

    elements
}

pub fn pointer_hotspot(cursor_status: &CursorImageStatus, pointer_element: &PointerElement) -> Point<i32, Logical> {
    match cursor_status {
        CursorImageStatus::Surface(surface) => compositor::with_states(surface, |states| {
            states
                .data_map
                .get::<Mutex<CursorImageAttributes>>()
                .and_then(|a| a.lock().ok().map(|a| a.hotspot))
                .unwrap_or_default()
        }),
        CursorImageStatus::Named(_) => pointer_element.hotspot(),
        CursorImageStatus::Hidden => Point::default(),
    }
}

pub fn pointer_view<R>(
    renderer: &mut R,
    area: Rectangle<i32, Logical>,
    scale: f64,
    pointer_location: Point<f64, Logical>,
    pointer_element: &PointerElement,
    cursor_status: &CursorImageStatus,
) -> crate::protocols::image_capture::PointerView<CustomRenderElements<R>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Texture + Clone + Send + 'static,
{
    let hot = pointer_hotspot(cursor_status, pointer_element);
    let side = (crate::protocols::image_capture::POINTER_SIZE as f64 / scale).ceil() as i32;
    let corner = (pointer_location - hot.to_f64()).to_i32_round();
    let picture = Rectangle::new(corner, (side, side).into());
    let elements = pointer_elements(renderer, picture, Scale::from(scale), pointer_location, pointer_element, cursor_status, None);
    let at = pointer_location - area.loc.to_f64();
    crate::protocols::image_capture::PointerView {
        over: area.overlaps(picture),
        position: ((at.x * scale).round() as i32, (at.y * scale).round() as i32),
        hotspot: ((hot.x as f64 * scale).round() as i32, (hot.y as f64 * scale).round() as i32),
        elements,
        scale,
    }
}

pub fn window_elements<R>(
    renderer: &mut R,
    window: &WindowElement,
    scale: f64,
    pointer: Vec<CustomRenderElements<R>>,
) -> Vec<OutputRenderElements<R, WindowRenderElement<R>>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Texture + Send + 'static,
{
    use smithay::desktop::space::SpaceElement;
    let geometry = SpaceElement::geometry(window);
    let location = (Point::<i32, Logical>::default() - geometry.loc).to_physical_precise_round(scale);
    let mut elements: Vec<OutputRenderElements<R, WindowRenderElement<R>>> =
        pointer.into_iter().map(OutputRenderElements::from).collect();
    let window_elements: Vec<WindowRenderElement<R>> =
        AsRenderElements::<R>::render_elements(window, renderer, location, Scale::from(scale), 1.0);
    elements.extend(window_elements.into_iter().map(|e| OutputRenderElements::Window(Wrap::from(e))));
    elements
}

pub fn output_elements<R>(
    output: &Output,
    space: &Space<WindowElement>,
    custom_elements: impl IntoIterator<Item = CustomRenderElements<R>>,
    renderer: &mut R,
    session_lock: &SessionLock,
) -> (Vec<OutputRenderElements<R, WindowRenderElement<R>>>, Color32F)
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Send + 'static,
{
    let custom_elements = with_overlays(custom_elements, renderer, output, space, session_lock);
    if session_lock.is_active() {
        return locked_elements(custom_elements, renderer, output, session_lock);
    }
    if let Some(window) = output
        .user_data()
        .get::<FullscreenSurface>()
        .and_then(|f| f.get())
    {
        fullscreen_elements(custom_elements, renderer, output, &window)
    } else {
        let mut output_render_elements = custom_elements
            .into_iter()
            .map(OutputRenderElements::from)
            .collect::<Vec<_>>();

        output_render_elements.extend(desktop_elements(output, space, renderer));

        (output_render_elements, CLEAR_COLOR)
    }
}

fn with_overlays<R>(
    custom_elements: impl IntoIterator<Item = CustomRenderElements<R>>,
    renderer: &mut R,
    output: &Output,
    space: &Space<WindowElement>,
    session_lock: &SessionLock,
) -> Vec<CustomRenderElements<R>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Send + 'static,
{
    let dots = crate::indicators::elements(renderer, output, space.output_geometry(output));
    let stopped = if session_lock.is_active() {
        Vec::new()
    } else {
        crate::stopped::elements(renderer, output, space.output_geometry(output))
    };
    let level = if session_lock.is_active() { Vec::new() } else { crate::level::elements(renderer, output) };
    custom_elements.into_iter().chain(dots).chain(stopped).chain(level).collect()
}

fn locked_elements<R>(
    custom_elements: Vec<CustomRenderElements<R>>,
    renderer: &mut R,
    output: &Output,
    session_lock: &SessionLock,
) -> (Vec<OutputRenderElements<R, WindowRenderElement<R>>>, Color32F)
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Send + 'static,
{
    let lock = session_lock
        .render_elements(renderer, output)
        .into_iter()
        .map(CustomRenderElements::Surface);
    let elements = custom_elements
        .into_iter()
        .chain(lock)
        .map(OutputRenderElements::from)
        .collect();
    (elements, LOCK_COLOR)
}

fn fullscreen_elements<R>(
    custom_elements: Vec<CustomRenderElements<R>>,
    renderer: &mut R,
    output: &Output,
    window: &WindowElement,
) -> (Vec<OutputRenderElements<R, WindowRenderElement<R>>>, Color32F)
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Send + 'static,
{
    let scale = output.current_scale().fractional_scale().into();
    let window_render_elements: Vec<WindowRenderElement<R>> =
        AsRenderElements::<R>::render_elements(window, renderer, (0, 0).into(), scale, 1.0);

    let trusted = trusted_over_full_screen(renderer, output, scale);

    let elements = custom_elements
        .into_iter()
        .map(OutputRenderElements::from)
        .chain(trusted)
        .chain(
            window_render_elements
                .into_iter()
                .map(|e| OutputRenderElements::Window(Wrap::from(e))),
        )
        .collect::<Vec<_>>();
    (elements, CLEAR_COLOR_FULLSCREEN)
}

fn trusted_over_full_screen<R>(
    renderer: &mut R,
    output: &Output,
    scale: Scale<f64>,
) -> Vec<OutputRenderElements<R, WindowRenderElement<R>>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Send + 'static,
{
    let mut trusted = Vec::new();
    let map = smithay::desktop::layer_map_for_output(output);
    for surface in crate::trusted::over_full_screen(&map).rev() {
        let Some(geo) = map.layer_geometry(surface) else { continue };
        let loc = geo.loc.to_physical_precise_round(scale);
        trusted.extend(
            AsRenderElements::<R>::render_elements::<WaylandSurfaceRenderElement<R>>(surface, renderer, loc, scale, 1.0)
                .into_iter()
                .map(|e| OutputRenderElements::Custom(CustomRenderElements::Surface(e))),
        );
    }
    trusted
}

fn desktop_elements<R>(
    output: &Output,
    space: &Space<WindowElement>,
    renderer: &mut R,
) -> Vec<OutputRenderElements<R, WindowRenderElement<R>>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Send + 'static,
{
    use smithay::wayland::shell::wlr_layer::Layer;
    let Some(output_geo) = space.output_geometry(output) else { return Vec::new() };
    let scale = Scale::from(output.current_scale().fractional_scale());
    let layers = |renderer: &mut R, which: Layer| -> Vec<OutputRenderElements<R, WindowRenderElement<R>>> {
        let map = smithay::desktop::layer_map_for_output(output);
        let mut out = Vec::new();
        for surface in map.layers_on(which).rev() {
            let Some(geo) = map.layer_geometry(surface) else { continue };
            let loc = geo.loc.to_physical_precise_round(scale);
            out.extend(
                AsRenderElements::<R>::render_elements::<WaylandSurfaceRenderElement<R>>(surface, renderer, loc, scale, 1.0)
                    .into_iter()
                    .map(|e| OutputRenderElements::Custom(CustomRenderElements::Surface(e))),
            );
        }
        out
    };
    let mut out = layers(renderer, Layer::Overlay);
    out.extend(layers(renderer, Layer::Top));
    out.extend(crate::animate::going::<R>(output, output_geo, scale).into_iter().map(OutputRenderElements::Kept));
    let windows: Vec<WindowElement> = space.elements_for_output(output).cloned().collect();
    for window in windows.iter().rev() {
        let Some(at) = space.element_location(window) else { continue };
        let loc = (at - smithay::desktop::space::SpaceElement::geometry(window).loc - output_geo.loc).to_physical_precise_round(scale);
        out.extend(
            crate::animate::window(output, output_geo, window, renderer, loc, scale)
                .into_iter()
                .map(OutputRenderElements::Moving),
        );
    }
    out.extend(layers(renderer, Layer::Bottom));
    out.extend(layers(renderer, Layer::Background));
    out
}

pub fn render_output<'a, 'd, R>(
    output: &'a Output,
    space: &'a Space<WindowElement>,
    custom_elements: impl IntoIterator<Item = CustomRenderElements<R>>,
    renderer: &'a mut R,
    framebuffer: &'a mut R::Framebuffer<'_>,
    damage_tracker: &'d mut OutputDamageTracker,
    age: usize,
    session_lock: &SessionLock,
) -> Result<RenderOutputResult<'d>, OutputDamageTrackerError<R::Error>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Send + 'static,
{
    let (elements, clear_color) =
        output_elements(output, space, custom_elements, renderer, session_lock);
    damage_tracker.render_output(renderer, framebuffer, age, &elements, clear_color)
}
