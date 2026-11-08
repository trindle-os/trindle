// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::{borrow::Cow, time::Duration};

use smithay::{
    backend::renderer::{
        element::{
            solid::{SolidColorRenderElement},
            surface::{render_elements_from_surface_tree, WaylandSurfaceRenderElement},
            AsRenderElements, Kind,
        },
        ImportAll, ImportMem, Renderer, Texture,
    },
    desktop::{
        space::SpaceElement, utils::OutputPresentationFeedback, PopupManager, Window, WindowSurface,
        WindowSurfaceType,
    },
    input::{
        pointer::{
            AxisFrame, ButtonEvent, GestureHoldBeginEvent, GestureHoldEndEvent, GesturePinchBeginEvent,
            GesturePinchEndEvent, GesturePinchUpdateEvent, GestureSwipeBeginEvent, GestureSwipeEndEvent,
            GestureSwipeUpdateEvent, MotionEvent, PointerTarget, RelativeMotionEvent,
        },
        touch::TouchTarget,
        Seat,
    },
    output::Output,
    reexports::{
        wayland_protocols::{
            wp::presentation_time::server::wp_presentation_feedback, xdg::shell::server::xdg_toplevel,
        },
        wayland_server::protocol::wl_surface::WlSurface,
    },
    render_elements,
    utils::{user_data::UserDataMap, IsAlive, Logical, Physical, Point, Rectangle, Scale, Serial, Size},
    wayland::{
        compositor::{with_states, SurfaceData as WlSurfaceData},
        dmabuf::DmabufFeedback,
        seat::WaylandFocus,
        shell::xdg::XdgToplevelSurfaceData,
    },
};

use super::picture::{whole_scale, Picture};
use super::ssd::{self, Action, Look, BORDER, HEADER_BAR_HEIGHT};
use crate::{focus::PointerFocusTarget, state::Backend, AnvilState};

const BTN_LEFT: u32 = 0x110;

#[derive(Debug, Clone, PartialEq)]
pub struct WindowElement(pub Window);

impl WindowElement {

    pub fn has_ssd(&self) -> bool {
        self.decoration_state().is_ssd && !self.is_fullscreen()
    }

    pub fn is_fullscreen(&self) -> bool {
        match self.0.underlying_surface() {
            WindowSurface::Wayland(t) => t.current_state().states.contains(xdg_toplevel::State::Fullscreen),
            WindowSurface::X11(x) => x.is_fullscreen(),
        }
    }

    pub fn is_maximized(&self) -> bool {
        match self.0.underlying_surface() {
            WindowSurface::Wayland(t) => t.with_pending_state(|s| s.states.contains(xdg_toplevel::State::Maximized)),
            WindowSurface::X11(x) => x.is_maximized(),
        }
    }

    pub fn is_active(&self) -> bool {
        match self.0.underlying_surface() {
            WindowSurface::Wayland(t) => t.with_pending_state(|s| s.states.contains(xdg_toplevel::State::Activated)),
            WindowSurface::X11(x) => x.is_activated(),
        }
    }

    pub fn title(&self) -> String {
        match self.0.underlying_surface() {
            WindowSurface::Wayland(t) => with_states(t.wl_surface(), |states| {
                states
                    .data_map
                    .get::<XdgToplevelSurfaceData>()
                    .and_then(|d| d.lock().ok()?.title.clone())
                    .unwrap_or_default()
            }),
            WindowSurface::X11(x) => x.title(),
        }
    }

    pub fn deco_offset(&self) -> Point<i32, Logical> {
        if self.has_ssd() {
            (BORDER, HEADER_BAR_HEIGHT).into()
        } else {
            (0, 0).into()
        }
    }

    pub fn deco_extra(&self) -> Size<i32, Logical> {
        frame_extra(self.has_ssd())
    }

    pub fn client_size(&self) -> Size<i32, Logical> {
        SpaceElement::geometry(&self.0).size
    }

    pub fn surface_under(
        &self,
        location: Point<f64, Logical>,
        window_type: WindowSurfaceType,
    ) -> Option<(PointerFocusTarget, Point<i32, Logical>)> {
        let offset = self.deco_offset();
        if self.has_ssd() {

            if window_type.contains(WindowSurfaceType::POPUP) {
                if let Some((surface, loc)) = self.0.surface_under(location - offset.to_f64(), WindowSurfaceType::POPUP) {
                    return Some((PointerFocusTarget::WlSurface(surface), loc + offset));
                }
            }
            let width = self.client_size().w + 2 * BORDER;
            if ssd::part_at(width, location).is_some() {
                return Some((PointerFocusTarget::Ssd(Ssd(self.clone())), Point::default()));
            }
        }

        let surface_under = self.0.surface_under(location - offset.to_f64(), window_type);
        let (under, loc) = match self.0.underlying_surface() {
            WindowSurface::Wayland(_) => {
                surface_under.map(|(surface, loc)| (PointerFocusTarget::WlSurface(surface), loc))
            }
            WindowSurface::X11(s) => {
                surface_under.map(|(_, loc)| (PointerFocusTarget::X11Surface(s.clone()), loc))
            }
        }?;
        Some((under, loc + offset))
    }

    pub fn with_surfaces<F>(&self, processor: F)
    where
        F: FnMut(&WlSurface, &WlSurfaceData),
    {
        self.0.with_surfaces(processor);
    }

    pub fn send_frame<T, F>(
        &self,
        output: &Output,
        time: T,
        throttle: Option<Duration>,
        primary_scan_out_output: F,
    ) where
        T: Into<Duration>,
        F: FnMut(&WlSurface, &WlSurfaceData) -> Option<Output> + Copy,
    {
        self.0.send_frame(output, time, throttle, primary_scan_out_output)
    }

    pub fn send_dmabuf_feedback<'a, P, F>(
        &self,
        output: &Output,
        primary_scan_out_output: P,
        select_dmabuf_feedback: F,
    ) where
        P: FnMut(&WlSurface, &WlSurfaceData) -> Option<Output> + Copy,
        F: Fn(&WlSurface, &WlSurfaceData) -> &'a DmabufFeedback + Copy,
    {
        self.0
            .send_dmabuf_feedback(output, primary_scan_out_output, select_dmabuf_feedback)
    }

    pub fn take_presentation_feedback<F1, F2>(
        &self,
        output_feedback: &mut OutputPresentationFeedback,
        primary_scan_out_output: F1,
        presentation_feedback_flags: F2,
    ) where
        F1: FnMut(&WlSurface, &WlSurfaceData) -> Option<Output> + Copy,
        F2: FnMut(&WlSurface, &WlSurfaceData) -> wp_presentation_feedback::Kind + Copy,
    {
        self.0.take_presentation_feedback(
            output_feedback,
            primary_scan_out_output,
            presentation_feedback_flags,
        )
    }

    #[inline]
    pub fn is_x11(&self) -> bool {
        self.0.is_x11()
    }

    #[inline]
    pub fn is_wayland(&self) -> bool {
        self.0.is_wayland()
    }

    #[inline]
    pub fn wl_surface(&self) -> Option<Cow<'_, WlSurface>> {
        self.0.wl_surface()
    }

    #[inline]
    pub fn user_data(&self) -> &UserDataMap {
        self.0.user_data()
    }
}

impl IsAlive for WindowElement {
    #[inline]
    fn alive(&self) -> bool {
        self.0.alive()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ssd(WindowElement);

impl IsAlive for Ssd {
    #[inline]
    fn alive(&self) -> bool {
        self.0.alive()
    }
}

impl WaylandFocus for Ssd {
    #[inline]
    fn wl_surface(&self) -> Option<Cow<'_, WlSurface>> {
        self.0.wl_surface()
    }
}

impl Ssd {

    fn width(&self) -> i32 {
        self.0.client_size().w + 2 * BORDER
    }

    fn act<BackendData: Backend>(
        &self,
        seat: &Seat<AnvilState<BackendData>>,
        data: &mut AnvilState<BackendData>,
        action: Action,
        serial: Serial,
    ) {
        let window = self.0.clone();
        let seat = seat.clone();
        data.handle.insert_idle(move |data| match action {
            Action::Move => match window.0.underlying_surface() {
                WindowSurface::Wayland(w) => data.move_request_xdg(&w.clone(), &seat, serial),
                WindowSurface::X11(w) => data.move_request_x11(&w.clone()),
            },
            Action::Minimize => {
                if let Some(surface) = window.wl_surface() {
                    data.set_window_minimized(&surface, true);
                }
            }
            Action::ToggleMaximize => data.toggle_maximize(&window),
            Action::Close => data.close_window(&window),
        });
    }
}

impl<BackendData: Backend> PointerTarget<AnvilState<BackendData>> for Ssd {
    fn enter(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        data: &mut AnvilState<BackendData>,
        event: &MotionEvent,
    ) {

        data.cursor_status = smithay::input::pointer::CursorImageStatus::default_named();
        let mut state = self.0.decoration_state();
        if state.is_ssd {
            state.header_bar.pointer_enter(event.location);
        }
    }
    fn motion(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        event: &MotionEvent,
    ) {
        let mut state = self.0.decoration_state();
        if state.is_ssd {
            state.header_bar.pointer_enter(event.location);
        }
    }
    fn relative_motion(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _event: &RelativeMotionEvent,
    ) {
    }
    fn button(
        &self,
        seat: &Seat<AnvilState<BackendData>>,
        data: &mut AnvilState<BackendData>,
        event: &ButtonEvent,
    ) {
        if event.button != BTN_LEFT {
            return;
        }
        let width = self.width();
        let action = {
            let mut state = self.0.decoration_state();
            if !state.is_ssd {
                return;
            }
            match event.state {
                smithay::backend::input::ButtonState::Pressed => state.header_bar.press(width, event.time),
                smithay::backend::input::ButtonState::Released => state.header_bar.release(width),
            }
        };
        if let Some(action) = action {
            self.act(seat, data, action, event.serial);
        }
    }
    fn axis(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _frame: AxisFrame,
    ) {
    }
    fn frame(&self, _seat: &Seat<AnvilState<BackendData>>, _data: &mut AnvilState<BackendData>) {}
    fn leave(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _serial: Serial,
        _time: u32,
    ) {
        let mut state = self.0.decoration_state();
        if state.is_ssd {
            state.header_bar.pointer_leave();
        }
    }
    fn gesture_swipe_begin(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _event: &GestureSwipeBeginEvent,
    ) {
    }
    fn gesture_swipe_update(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _event: &GestureSwipeUpdateEvent,
    ) {
    }
    fn gesture_swipe_end(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _event: &GestureSwipeEndEvent,
    ) {
    }
    fn gesture_pinch_begin(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _event: &GesturePinchBeginEvent,
    ) {
    }
    fn gesture_pinch_update(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _event: &GesturePinchUpdateEvent,
    ) {
    }
    fn gesture_pinch_end(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _event: &GesturePinchEndEvent,
    ) {
    }
    fn gesture_hold_begin(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _event: &GestureHoldBeginEvent,
    ) {
    }
    fn gesture_hold_end(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _event: &GestureHoldEndEvent,
    ) {
    }
}

impl<BackendData: Backend> TouchTarget<AnvilState<BackendData>> for Ssd {
    fn down(
        &self,
        seat: &Seat<AnvilState<BackendData>>,
        data: &mut AnvilState<BackendData>,
        event: &smithay::input::touch::DownEvent,
        _seq: Serial,
    ) {
        let width = self.width();
        let action = {
            let mut state = self.0.decoration_state();
            if !state.is_ssd {
                return;
            }
            state.header_bar.pointer_enter(event.location);
            state.header_bar.press(width, event.time)
        };
        if let Some(action) = action {
            self.act(seat, data, action, event.serial);
        }
    }

    fn up(
        &self,
        seat: &Seat<AnvilState<BackendData>>,
        data: &mut AnvilState<BackendData>,
        event: &smithay::input::touch::UpEvent,
        _seq: Serial,
    ) {
        let width = self.width();
        let action = {
            let mut state = self.0.decoration_state();
            if !state.is_ssd {
                return;
            }
            state.header_bar.release(width)
        };
        if let Some(action) = action {
            self.act(seat, data, action, event.serial);
        }
    }

    fn motion(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        event: &smithay::input::touch::MotionEvent,
        _seq: Serial,
    ) {
        let mut state = self.0.decoration_state();
        if state.is_ssd {
            state.header_bar.pointer_enter(event.location);
        }
    }

    fn frame(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _seq: Serial,
    ) {
    }

    fn cancel(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _seq: Serial,
    ) {
    }

    fn shape(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _event: &smithay::input::touch::ShapeEvent,
        _seq: Serial,
    ) {
    }

    fn orientation(
        &self,
        _seat: &Seat<AnvilState<BackendData>>,
        _data: &mut AnvilState<BackendData>,
        _event: &smithay::input::touch::OrientationEvent,
        _seq: Serial,
    ) {
    }
}

impl SpaceElement for WindowElement {
    fn geometry(&self) -> Rectangle<i32, Logical> {
        let mut geo = SpaceElement::geometry(&self.0);
        geo.size += self.deco_extra();
        geo
    }
    fn bbox(&self) -> Rectangle<i32, Logical> {
        let bbox = SpaceElement::bbox(&self.0);
        if !self.has_ssd() {
            return bbox;
        }

        let geo = SpaceElement::geometry(&self.0);
        let frame = Rectangle::new(geo.loc, geo.size + self.deco_extra());
        frame.merge(Rectangle::new(bbox.loc + self.deco_offset(), bbox.size))
    }
    fn is_in_input_region(&self, point: &Point<f64, Logical>) -> bool {
        if self.has_ssd() {

            ssd::part_at(self.client_size().w + 2 * BORDER, *point).is_some()
                || SpaceElement::is_in_input_region(&self.0, &(*point - self.deco_offset().to_f64()))
        } else {
            SpaceElement::is_in_input_region(&self.0, point)
        }
    }
    fn z_index(&self) -> u8 {
        SpaceElement::z_index(&self.0)
    }

    fn set_activate(&self, activated: bool) {
        SpaceElement::set_activate(&self.0, activated);
    }
    fn output_enter(&self, output: &Output, overlap: Rectangle<i32, Logical>) {
        SpaceElement::output_enter(&self.0, output, overlap);
    }
    fn output_leave(&self, output: &Output) {
        SpaceElement::output_leave(&self.0, output);
    }
    fn refresh(&self) {
        SpaceElement::refresh(&self.0);
    }
}

render_elements!(
    pub WindowRenderElement<R> where R: ImportAll + ImportMem;
    Window=WaylandSurfaceRenderElement<R>,
    Decoration=SolidColorRenderElement,
    Picture=Picture<R>,
);

impl<R: Renderer> std::fmt::Debug for WindowRenderElement<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Window(arg0) => f.debug_tuple("Window").field(arg0).finish(),
            Self::Decoration(arg0) => f.debug_tuple("Decoration").field(arg0).finish(),
            Self::Picture(arg0) => f.debug_tuple("Picture").field(arg0).finish(),
            Self::_GenericCatcher(arg0) => f.debug_tuple("_GenericCatcher").field(arg0).finish(),
        }
    }
}

impl WindowElement {

    fn framed_elements<R>(&self, renderer: &mut R, location: Point<i32, Physical>, scale: Scale<f64>, alpha: f32) -> Vec<WindowRenderElement<R>>
    where
        R: Renderer + ImportAll + ImportMem,
        R::TextureId: Clone + Texture + Send + 'static,
    {
        let client = self.client_size();
        let width = client.w + 2 * BORDER;
        let look = Look {
            width,
            scale: whole_scale(scale) as i32,
            title: self.title(),
            active: self.is_active(),
            maximized: self.is_maximized(),
            hover: None,
            pressed: None,
        };
        let offset = self.deco_offset();

        let mut state = self.decoration_state();
        let look = Look {
            hover: state.header_bar.hover(width),
            pressed: state.header_bar.pressed(),
            ..look
        };
        let (picture, colours) = state.header_bar.picture(renderer, look);
        state.header_bar.update_sides(client, colours.border);
        let at = |x: i32, y: i32| location + Point::<i32, Logical>::from((x, y)).to_physical_precise_round(scale);

        let mut vec: Vec<WindowRenderElement<R>> = AsRenderElements::render_elements(
            &self.0,
            renderer,
            location + offset.to_physical_precise_round(scale),
            scale,
            alpha,
        );
        if let Some((texture, size)) = picture {
            let bar = &state.header_bar;
            let geo = Rectangle::new(location, (at(width, HEADER_BAR_HEIGHT) - location).to_size());
            let src = Rectangle::from_size(size.to_f64());
            vec.push(Picture::new(bar.id.clone(), bar.commit, texture, src, geo, alpha).into());
        }
        let places = [
            at(0, HEADER_BAR_HEIGHT),
            at(width - BORDER, HEADER_BAR_HEIGHT),
            at(0, HEADER_BAR_HEIGHT + client.h),
        ];
        for (side, place) in state.header_bar.sides.iter().zip(places) {
            vec.push(SolidColorRenderElement::from_buffer(side, place, scale, alpha, Kind::Unspecified).into());
        }
        drop(state);

        vec.extend(super::shadow::elements(self, renderer, location, scale, alpha).into_iter().map(WindowRenderElement::Picture));
        vec
    }

    fn plain_elements<R>(&self, renderer: &mut R, location: Point<i32, Physical>, scale: Scale<f64>, alpha: f32) -> Vec<WindowRenderElement<R>>
    where
        R: Renderer + ImportAll + ImportMem,
        R::TextureId: Clone + Texture + Send + 'static,
    {

        let (corners, sides) = super::outline::elements(self, renderer, location, scale, alpha);
        let line = corners
            .into_iter()
            .map(WindowRenderElement::Picture)
            .chain(sides.into_iter().map(WindowRenderElement::Decoration));
        let mut vec: Vec<WindowRenderElement<R>> = Vec::new();
        match self.0.underlying_surface() {
            WindowSurface::Wayland(toplevel) => {
                let surface = toplevel.wl_surface();
                let geo = SpaceElement::geometry(&self.0).loc;
                for (popup, offset) in PopupManager::popups_for_surface(surface) {
                    let at = location + (geo + offset - popup.geometry().loc).to_physical_precise_round(scale);
                    vec.extend(render_elements_from_surface_tree(
                        renderer, popup.wl_surface(), at, scale, alpha, Kind::Unspecified,
                    ));
                }
                vec.extend(line);
                vec.extend(render_elements_from_surface_tree(
                    renderer, surface, location, scale, alpha, Kind::Unspecified,
                ));
            }
            WindowSurface::X11(_) => {
                vec.extend(line);
                vec.extend(AsRenderElements::render_elements(&self.0, renderer, location, scale, alpha));
            }
        }
        vec.extend(super::shadow::elements(self, renderer, location, scale, alpha).into_iter().map(WindowRenderElement::Picture));
        vec
    }
}

impl<R> AsRenderElements<R> for WindowElement
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Texture + Send + 'static,
{
    type RenderElement = WindowRenderElement<R>;

    fn render_elements<C: From<Self::RenderElement>>(
        &self,
        renderer: &mut R,
        location: Point<i32, Physical>,
        scale: Scale<f64>,
        alpha: f32,
    ) -> Vec<C> {
        let window_bbox = SpaceElement::bbox(&self.0);

        let vec = if self.has_ssd() && !window_bbox.is_empty() {
            self.framed_elements(renderer, location, scale, alpha)
        } else {
            self.plain_elements(renderer, location, scale, alpha)
        };
        vec.into_iter().map(C::from).collect()
    }
}

pub fn frame_extra(ssd: bool) -> Size<i32, Logical> {
    if ssd {
        (2 * BORDER, HEADER_BAR_HEIGHT + BORDER).into()
    } else {
        (0, 0).into()
    }
}
