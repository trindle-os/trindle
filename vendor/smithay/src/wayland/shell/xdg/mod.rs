//! Utilities for handling shell surfaces with the `xdg_shell` protocol.

use crate::utils::alive_tracker::IsAlive;
use crate::utils::{user_data::UserDataMap, Logical, Point, Rectangle, Size};
use crate::utils::{Serial, SERIAL_COUNTER};
use crate::wayland::compositor;
use crate::wayland::compositor::Cacheable;
use std::cmp::min;
use std::{collections::HashSet, fmt::Debug, sync::Mutex};

use wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1;
use wayland_protocols::xdg::shell::server::xdg_positioner::{Anchor, ConstraintAdjustment, Gravity};
use wayland_protocols::xdg::shell::server::xdg_surface;
use wayland_protocols::xdg::shell::server::xdg_wm_base::XdgWmBase;
use wayland_protocols::xdg::shell::server::{xdg_popup, xdg_positioner, xdg_toplevel, xdg_wm_base};
use wayland_server::backend::GlobalId;
use wayland_server::{
    protocol::{wl_output, wl_seat, wl_surface},
    DisplayHandle, GlobalDispatch, Resource,
};

use super::PingError;

pub mod decoration;
pub mod dialog;

pub(super) mod handlers;
pub use handlers::{XdgPositionerUserData, XdgShellSurfaceUserData, XdgSurfaceUserData, XdgWmBaseUserData};

pub const XDG_TOPLEVEL_ROLE: &str = "xdg_toplevel";

pub const XDG_POPUP_ROLE: &str = "xdg_popup";

const XDG_TOPLEVEL_STATE_TILED_SINCE: u32 = 2;
const XDG_TOPLEVEL_STATE_SUSPENDED_SINCE: u32 = 6;

macro_rules! xdg_role {
    ($state:ty,
     $(#[$configure_meta:meta])* $configure_name:ident $({$($(#[$configure_field_meta:meta])* $configure_field_vis:vis$configure_field_name:ident:$configure_field_type:ty),*}),*,
     $(#[$attributes_meta:meta])* $attributes_name:ident {$($(#[$attributes_field_meta:meta])* $attributes_field_vis:vis$attributes_field_name:ident:$attributes_field_type:ty),*}) => {

        $(#[$configure_meta])*
        pub struct $configure_name {
                pub state: $state,
                pub serial: Serial,

                $($(
                    $(#[$configure_field_meta])*
                    $configure_field_vis $configure_field_name: $configure_field_type,
                )*)*
        }

        $(#[$attributes_meta])*
        pub struct $attributes_name {
                pub configured: bool,
                pub configure_serial: Option<Serial>,
                pub initial_configure_sent: bool,
                pending_configures: Vec<$configure_name>,
                pub server_pending: Option<$state>,
                pub last_acked: Option<$state>,
                pub current: $state,
                pub current_serial: Option<Serial>,
                has_buffer: bool,

                $(
                    $(#[$attributes_field_meta])*
                    $attributes_field_vis $attributes_field_name: $attributes_field_type,
                )*
        }

        impl $attributes_name {
            fn ack_configure(&mut self, serial: Serial) -> Option<Configure> {
                let configure = match self
                    .pending_configures
                    .iter()
                    .find(|configure| configure.serial == serial)
                {
                    Some(configure) => (*configure).clone(),
                    None => {
                        return None;
                    }
                };

                self.last_acked = Some(configure.state.clone());

                self.configured = true;

                self.configure_serial = Some(Serial::from(serial));

                self.pending_configures.retain(|c| c.serial > serial);

                Some(configure.into())
            }

            pub fn current_server_state(&self) -> &$state {
                self.pending_configures
                    .last()
                    .map(|c| &c.state)
                    .or_else(|| self.last_acked.as_ref())
                    .unwrap_or(&self.current)
            }

            pub fn has_pending_changes(&self) -> bool {
                self.server_pending.as_ref().map(|s| s != self.current_server_state()).unwrap_or(false)
            }

            pub fn pending_configures(&self) -> &[$configure_name] {
                &self.pending_configures
            }
        }

        impl Default for $attributes_name {
            fn default() -> Self {
                Self {
                    configured: false,
                    configure_serial: None,
                    pending_configures: Vec::new(),
                    initial_configure_sent: false,
                    server_pending: None,
                    last_acked: None,
                    current: Default::default(),
                    current_serial: None,
                    has_buffer: false,

                    $(
                        $attributes_field_name: Default::default(),
                    )*
                }
            }
        }
    };
}

xdg_role!(
    ToplevelState,
    #[derive(Debug, Clone)]
    ToplevelConfigure,
    #[derive(Debug)]
    XdgToplevelSurfaceRoleAttributes {
        pub parent: Option<wl_surface::WlSurface>,
        pub modal: bool,
        pub title: Option<String>,
        pub app_id: Option<String>,
        pub initial_decoration_configure_sent: bool
    }
);

pub type XdgToplevelSurfaceData = Mutex<XdgToplevelSurfaceRoleAttributes>;

xdg_role!(
    PopupState,
    #[derive(Debug, Clone, Copy)]
    PopupConfigure {
        pub reposition_token: Option<u32>
    },
    #[derive(Debug)]
    XdgPopupSurfaceRoleAttributes {
        pub parent: Option<wl_surface::WlSurface>,

        pub committed: bool,

        popup_handle: Option<xdg_popup::XdgPopup>
    }
);

pub type XdgPopupSurfaceData = Mutex<XdgPopupSurfaceRoleAttributes>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PopupState {
    pub positioner: PositionerState,
    pub geometry: Rectangle<i32, Logical>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PositionerState {
    pub rect_size: Size<i32, Logical>,
    pub anchor_rect: Rectangle<i32, Logical>,
    pub anchor_edges: xdg_positioner::Anchor,
    pub gravity: xdg_positioner::Gravity,
    pub constraint_adjustment: xdg_positioner::ConstraintAdjustment,
    pub offset: Point<i32, Logical>,
    pub reactive: bool,
    pub parent_size: Option<Size<i32, Logical>>,
    pub parent_configure: Option<Serial>,
}

impl Default for PositionerState {
    fn default() -> Self {
        PositionerState {
            anchor_edges: xdg_positioner::Anchor::None,
            anchor_rect: Default::default(),
            constraint_adjustment: xdg_positioner::ConstraintAdjustment::empty(),
            gravity: xdg_positioner::Gravity::None,
            offset: Default::default(),
            rect_size: Default::default(),
            reactive: false,
            parent_size: None,
            parent_configure: None,
        }
    }
}

impl PositionerState {
    pub(crate) fn anchor_has_edge(&self, edge: xdg_positioner::Anchor) -> bool {
        match edge {
            xdg_positioner::Anchor::Top => {
                self.anchor_edges == xdg_positioner::Anchor::Top
                    || self.anchor_edges == xdg_positioner::Anchor::TopLeft
                    || self.anchor_edges == xdg_positioner::Anchor::TopRight
            }
            xdg_positioner::Anchor::Bottom => {
                self.anchor_edges == xdg_positioner::Anchor::Bottom
                    || self.anchor_edges == xdg_positioner::Anchor::BottomLeft
                    || self.anchor_edges == xdg_positioner::Anchor::BottomRight
            }
            xdg_positioner::Anchor::Left => {
                self.anchor_edges == xdg_positioner::Anchor::Left
                    || self.anchor_edges == xdg_positioner::Anchor::TopLeft
                    || self.anchor_edges == xdg_positioner::Anchor::BottomLeft
            }
            xdg_positioner::Anchor::Right => {
                self.anchor_edges == xdg_positioner::Anchor::Right
                    || self.anchor_edges == xdg_positioner::Anchor::TopRight
                    || self.anchor_edges == xdg_positioner::Anchor::BottomRight
            }
            _ => unreachable!(),
        }
    }

    pub fn get_anchor_point(&self) -> Point<i32, Logical> {
        let mut point = self.anchor_rect.loc;

        point.y += if self.anchor_has_edge(xdg_positioner::Anchor::Top) {
            0
        } else if self.anchor_has_edge(xdg_positioner::Anchor::Bottom) {
            self.anchor_rect.size.h
        } else {
            self.anchor_rect.size.h / 2
        };

        point.x += if self.anchor_has_edge(xdg_positioner::Anchor::Left) {
            0
        } else if self.anchor_has_edge(xdg_positioner::Anchor::Right) {
            self.anchor_rect.size.w
        } else {
            self.anchor_rect.size.w / 2
        };

        point
    }

    pub(crate) fn gravity_has_edge(&self, edge: xdg_positioner::Gravity) -> bool {
        match edge {
            xdg_positioner::Gravity::Top => {
                self.gravity == xdg_positioner::Gravity::Top
                    || self.gravity == xdg_positioner::Gravity::TopLeft
                    || self.gravity == xdg_positioner::Gravity::TopRight
            }
            xdg_positioner::Gravity::Bottom => {
                self.gravity == xdg_positioner::Gravity::Bottom
                    || self.gravity == xdg_positioner::Gravity::BottomLeft
                    || self.gravity == xdg_positioner::Gravity::BottomRight
            }
            xdg_positioner::Gravity::Left => {
                self.gravity == xdg_positioner::Gravity::Left
                    || self.gravity == xdg_positioner::Gravity::TopLeft
                    || self.gravity == xdg_positioner::Gravity::BottomLeft
            }
            xdg_positioner::Gravity::Right => {
                self.gravity == xdg_positioner::Gravity::Right
                    || self.gravity == xdg_positioner::Gravity::TopRight
                    || self.gravity == xdg_positioner::Gravity::BottomRight
            }
            _ => unreachable!(),
        }
    }

    pub fn get_geometry(&self) -> Rectangle<i32, Logical> {
        let mut geometry = Rectangle::new(self.offset, self.rect_size);

        geometry.loc += self.get_anchor_point();

        if self.gravity_has_edge(xdg_positioner::Gravity::Top) {
            geometry.loc.y -= geometry.size.h;
        } else if !self.gravity_has_edge(xdg_positioner::Gravity::Bottom) {
            geometry.loc.y -= geometry.size.h / 2;
        }

        if self.gravity_has_edge(xdg_positioner::Gravity::Left) {
            geometry.loc.x -= geometry.size.w;
        } else if !self.gravity_has_edge(xdg_positioner::Gravity::Right) {
            geometry.loc.x -= geometry.size.w / 2;
        }

        geometry
    }

    pub fn get_unconstrained_geometry(mut self, target: Rectangle<i32, Logical>) -> Rectangle<i32, Logical> {
        let mut geo = self.get_geometry();
        let (mut off_left, mut off_right, mut off_top, mut off_bottom) = compute_offsets(target, geo);

        if (off_left > 0 || off_right > 0) && self.constraint_adjustment.contains(ConstraintAdjustment::FlipX)
        {
            let mut new = self;
            new.anchor_edges = invert_anchor_x(new.anchor_edges);
            new.gravity = invert_gravity_x(new.gravity);
            let new_geo = new.get_geometry();
            let (new_off_left, new_off_right, _, _) = compute_offsets(target, new_geo);

            if new_off_left <= 0 && new_off_right <= 0 {
                self = new;
                geo = new_geo;
                off_left = 0;
                off_right = 0;
            }
        }

        if (off_top > 0 || off_bottom > 0) && self.constraint_adjustment.contains(ConstraintAdjustment::FlipY)
        {
            let mut new = self;
            new.anchor_edges = invert_anchor_y(new.anchor_edges);
            new.gravity = invert_gravity_y(new.gravity);
            let new_geo = new.get_geometry();
            let (_, _, new_off_top, new_off_bottom) = compute_offsets(target, new_geo);

            if new_off_top <= 0 && new_off_bottom <= 0 {
                self = new;
                geo = new_geo;
                off_top = 0;
                off_bottom = 0;
            }
        }

        if (off_left > 0 || off_right > 0)
            && self.constraint_adjustment.contains(ConstraintAdjustment::SlideX)
        {
            if off_left > 0 {
                geo.loc.x += off_left;
            } else if off_right > 0 {
                geo.loc.x -= min(off_right, -off_left);
            }

            (off_left, off_right, _, _) = compute_offsets(target, geo);
        }

        if (off_top > 0 || off_bottom > 0)
            && self.constraint_adjustment.contains(ConstraintAdjustment::SlideY)
        {
            if off_top > 0 {
                geo.loc.y += off_top;
            } else if off_bottom > 0 {
                geo.loc.y -= min(off_bottom, -off_top);
            }

            (_, _, off_top, off_bottom) = compute_offsets(target, geo);
        }

        if self.constraint_adjustment.contains(ConstraintAdjustment::ResizeX) {

            if off_left > 0 && off_left < geo.size.w {
                geo.loc.x += off_left;
                geo.size.w -= off_left;
            }
            if off_right > 0 && off_right < geo.size.w {
                geo.size.w -= off_right;
            }
        }

        if self.constraint_adjustment.contains(ConstraintAdjustment::ResizeY) {

            if off_top > 0 && off_top < geo.size.h {
                geo.loc.y += off_top;
                geo.size.h -= off_top;
            }
            if off_bottom > 0 && off_bottom < geo.size.h {
                geo.size.h -= off_bottom;
            }
        }

        geo
    }
}

fn compute_offsets(target: Rectangle<i32, Logical>, popup: Rectangle<i32, Logical>) -> (i32, i32, i32, i32) {
    let off_left = target.loc.x - popup.loc.x;
    let off_right = (popup.loc.x + popup.size.w) - (target.loc.x + target.size.w);
    let off_top = target.loc.y - popup.loc.y;
    let off_bottom = (popup.loc.y + popup.size.h) - (target.loc.y + target.size.h);
    (off_left, off_right, off_top, off_bottom)
}

fn invert_anchor_x(anchor: Anchor) -> Anchor {
    match anchor {
        Anchor::Left => Anchor::Right,
        Anchor::Right => Anchor::Left,
        Anchor::TopLeft => Anchor::TopRight,
        Anchor::TopRight => Anchor::TopLeft,
        Anchor::BottomLeft => Anchor::BottomRight,
        Anchor::BottomRight => Anchor::BottomLeft,
        x => x,
    }
}

fn invert_anchor_y(anchor: Anchor) -> Anchor {
    match anchor {
        Anchor::Top => Anchor::Bottom,
        Anchor::Bottom => Anchor::Top,
        Anchor::TopLeft => Anchor::BottomLeft,
        Anchor::TopRight => Anchor::BottomRight,
        Anchor::BottomLeft => Anchor::TopLeft,
        Anchor::BottomRight => Anchor::TopRight,
        x => x,
    }
}

fn invert_gravity_x(gravity: Gravity) -> Gravity {
    match gravity {
        Gravity::Left => Gravity::Right,
        Gravity::Right => Gravity::Left,
        Gravity::TopLeft => Gravity::TopRight,
        Gravity::TopRight => Gravity::TopLeft,
        Gravity::BottomLeft => Gravity::BottomRight,
        Gravity::BottomRight => Gravity::BottomLeft,
        x => x,
    }
}

fn invert_gravity_y(gravity: Gravity) -> Gravity {
    match gravity {
        Gravity::Top => Gravity::Bottom,
        Gravity::Bottom => Gravity::Top,
        Gravity::TopLeft => Gravity::BottomLeft,
        Gravity::TopRight => Gravity::BottomRight,
        Gravity::BottomLeft => Gravity::TopLeft,
        Gravity::BottomRight => Gravity::TopRight,
        x => x,
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ToplevelState {
    pub size: Option<Size<i32, Logical>>,

    pub bounds: Option<Size<i32, Logical>>,

    pub states: ToplevelStateSet,

    pub fullscreen_output: Option<wl_output::WlOutput>,

    pub decoration_mode: Option<zxdg_toplevel_decoration_v1::Mode>,

    pub capabilities: WmCapabilitySet,
}

impl Clone for ToplevelState {
    fn clone(&self) -> ToplevelState {
        ToplevelState {
            fullscreen_output: self.fullscreen_output.clone(),
            states: self.states.clone(),
            size: self.size,
            bounds: self.bounds,
            decoration_mode: self.decoration_mode,
            capabilities: self.capabilities.clone(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ToplevelStateSet {
    states: Vec<xdg_toplevel::State>,
}

impl ToplevelStateSet {
    pub fn contains(&self, state: xdg_toplevel::State) -> bool {
        self.states.contains(&state)
    }

    pub fn set(&mut self, state: xdg_toplevel::State) -> bool {
        if self.contains(state) {
            false
        } else {
            self.states.push(state);
            true
        }
    }

    pub fn unset(&mut self, state: xdg_toplevel::State) -> bool {
        if !self.contains(state) {
            false
        } else {
            self.states.retain(|s| *s != state);
            true
        }
    }

    pub(crate) fn into_filtered_states(self, version: u32) -> Vec<xdg_toplevel::State> {
        let tiled_supported = version >= XDG_TOPLEVEL_STATE_TILED_SINCE;
        let suspended_supported = version >= XDG_TOPLEVEL_STATE_SUSPENDED_SINCE;

        if suspended_supported {
            return self.states;
        }

        let is_suspended = |state: &xdg_toplevel::State| *state == xdg_toplevel::State::Suspended;
        let contains_suspended = self.states.contains(&xdg_toplevel::State::Suspended);

        if tiled_supported && !contains_suspended {
            return self.states;
        }

        let is_tiled = |state: &xdg_toplevel::State| {
            matches!(
                state,
                xdg_toplevel::State::TiledTop
                    | xdg_toplevel::State::TiledBottom
                    | xdg_toplevel::State::TiledLeft
                    | xdg_toplevel::State::TiledRight
            )
        };
        let contains_tiled = self.states.iter().any(is_tiled);

        if !contains_suspended && !contains_tiled {
            return self.states;
        }

        self.states
            .into_iter()
            .filter(|state| {
                if tiled_supported {
                    !is_suspended(state)
                } else {
                    !is_suspended(state) && !is_tiled(state)
                }
            })
            .collect()
    }
}

impl IntoIterator for ToplevelStateSet {
    type Item = xdg_toplevel::State;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.states.into_iter()
    }
}

impl From<ToplevelStateSet> for Vec<xdg_toplevel::State> {
    #[inline]
    fn from(states: ToplevelStateSet) -> Self {
        states.states
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct WmCapabilitySet {
    capabilities: HashSet<xdg_toplevel::WmCapabilities>,
}

impl WmCapabilitySet {
    pub fn contains(&self, capability: xdg_toplevel::WmCapabilities) -> bool {
        self.capabilities.contains(&capability)
    }

    pub fn set(&mut self, capability: xdg_toplevel::WmCapabilities) -> bool {
        self.capabilities.insert(capability)
    }

    pub fn unset(&mut self, capability: xdg_toplevel::WmCapabilities) -> bool {
        self.capabilities.remove(&capability)
    }

    pub fn replace(&mut self, capabilities: impl IntoIterator<Item = xdg_toplevel::WmCapabilities>) {
        self.capabilities.clear();
        self.capabilities.extend(capabilities);
    }

    pub fn capabilities(&self) -> impl Iterator<Item = &xdg_toplevel::WmCapabilities> {
        self.capabilities.iter()
    }
}

impl<T> From<T> for WmCapabilitySet
where
    T: IntoIterator<Item = xdg_toplevel::WmCapabilities>,
{
    #[inline]
    fn from(capabilities: T) -> Self {
        let capabilities = capabilities.into_iter().collect();
        Self { capabilities }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct SurfaceCachedState {
    pub geometry: Option<Rectangle<i32, Logical>>,
    pub min_size: Size<i32, Logical>,
    pub max_size: Size<i32, Logical>,
}

impl Cacheable for SurfaceCachedState {
    fn commit(&mut self, _dh: &DisplayHandle) -> Self {
        *self
    }
    fn merge_into(self, into: &mut Self, _dh: &DisplayHandle) {
        *into = self;
    }
}

#[allow(unused_variables)]
pub trait XdgShellHandler {
    fn xdg_shell_state(&mut self) -> &mut XdgShellState;

    fn new_client(&mut self, client: ShellClient) {}

    fn client_pong(&mut self, client: ShellClient) {}

    fn new_toplevel(&mut self, surface: ToplevelSurface);

    fn new_popup(&mut self, surface: PopupSurface, positioner: PositionerState);

    fn move_request(&mut self, surface: ToplevelSurface, seat: wl_seat::WlSeat, serial: Serial) {}

    fn resize_request(
        &mut self,
        surface: ToplevelSurface,
        seat: wl_seat::WlSeat,
        serial: Serial,
        edges: xdg_toplevel::ResizeEdge,
    ) {
    }

    fn grab(&mut self, surface: PopupSurface, seat: wl_seat::WlSeat, serial: Serial);

    fn maximize_request(&mut self, surface: ToplevelSurface) {
        surface.send_configure();
    }

    fn unmaximize_request(&mut self, surface: ToplevelSurface) {}

    fn fullscreen_request(&mut self, surface: ToplevelSurface, output: Option<wl_output::WlOutput>) {
        surface.send_configure();
    }

    fn unfullscreen_request(&mut self, surface: ToplevelSurface) {}

    fn minimize_request(&mut self, surface: ToplevelSurface) {}

    fn show_window_menu(
        &mut self,
        surface: ToplevelSurface,
        seat: wl_seat::WlSeat,
        serial: Serial,
        location: Point<i32, Logical>,
    ) {
    }

    fn ack_configure(&mut self, surface: wl_surface::WlSurface, configure: Configure) {}

    fn reposition_request(&mut self, surface: PopupSurface, positioner: PositionerState, token: u32);

    fn client_destroyed(&mut self, client: ShellClient) {}

    fn toplevel_destroyed(&mut self, surface: ToplevelSurface) {}

    fn popup_destroyed(&mut self, surface: PopupSurface) {}

    fn app_id_changed(&mut self, surface: ToplevelSurface) {}

    fn title_changed(&mut self, surface: ToplevelSurface) {}

    fn parent_changed(&mut self, surface: ToplevelSurface) {}
}

#[derive(Debug)]
pub struct XdgShellState {
    known_toplevels: Vec<ToplevelSurface>,
    known_popups: Vec<PopupSurface>,
    default_capabilities: WmCapabilitySet,
    global: GlobalId,
}

impl XdgShellState {
    pub fn new<D>(display: &DisplayHandle) -> XdgShellState
    where
        D: GlobalDispatch<XdgWmBase, ()> + 'static,
    {
        Self::new_with_capabilities::<D>(
            display,
            [
                xdg_toplevel::WmCapabilities::Fullscreen,
                xdg_toplevel::WmCapabilities::Maximize,
                xdg_toplevel::WmCapabilities::Minimize,
                xdg_toplevel::WmCapabilities::WindowMenu,
            ],
        )
    }

    pub fn new_with_capabilities<D>(
        display: &DisplayHandle,
        capabilities: impl Into<WmCapabilitySet>,
    ) -> XdgShellState
    where
        D: GlobalDispatch<XdgWmBase, ()> + 'static,
    {
        let global = display.create_global::<D, XdgWmBase, _>(6, ());

        XdgShellState {
            known_toplevels: Vec::new(),
            known_popups: Vec::new(),
            default_capabilities: capabilities.into(),
            global,
        }
    }

    pub fn replace_capabilities(&mut self, capabilities: impl Into<WmCapabilitySet>) {
        self.default_capabilities = capabilities.into();
    }

    pub fn toplevel_surfaces(&self) -> &[ToplevelSurface] {
        &self.known_toplevels
    }

    pub fn get_toplevel(&self, toplevel: &xdg_toplevel::XdgToplevel) -> Option<ToplevelSurface> {
        self.known_toplevels
            .iter()
            .find(|surface| surface.xdg_toplevel() == toplevel)
            .cloned()
    }

    pub fn popup_surfaces(&self) -> &[PopupSurface] {
        &self.known_popups
    }

    pub fn get_popup(&self, popup: &xdg_popup::XdgPopup) -> Option<PopupSurface> {
        self.known_popups
            .iter()
            .find(|surface| surface.xdg_popup() == popup)
            .cloned()
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

#[derive(Default, Debug)]
pub(crate) struct ShellClientData {
    pending_ping: Option<Serial>,
    data: UserDataMap,
}

#[derive(Debug)]
pub struct ShellClient {
    kind: xdg_wm_base::XdgWmBase,
}

impl std::cmp::PartialEq for ShellClient {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl ShellClient {
    fn new(resource: &xdg_wm_base::XdgWmBase) -> Self {
        Self {
            kind: resource.clone(),
        }
    }

    #[inline]
    pub fn alive(&self) -> bool {
        self.kind.alive()
    }

    pub fn send_ping(&self, serial: Serial) -> Result<(), PingError> {
        if !self.alive() {
            return Err(PingError::DeadSurface);
        }
        let user_data = self.kind.data::<self::handlers::XdgWmBaseUserData>().unwrap();
        let mut guard = user_data.client_data.lock().unwrap();
        if let Some(pending_ping) = guard.pending_ping {
            return Err(PingError::PingAlreadyPending(pending_ping));
        }
        guard.pending_ping = Some(serial);
        self.kind.ping(serial.into());

        Ok(())
    }

    pub fn unresponsive(&self) -> Result<(), crate::utils::DeadResource> {
        if !self.alive() {
            return Err(crate::utils::DeadResource);
        }
        self.kind.post_error(
            xdg_wm_base::Error::Unresponsive,
            "client did not respond to ping on time",
        );

        Ok(())
    }

    pub fn with_data<F, T>(&self, f: F) -> Result<T, crate::utils::DeadResource>
    where
        F: FnOnce(&mut UserDataMap) -> T,
    {
        if !self.alive() {
            return Err(crate::utils::DeadResource);
        }
        let data = self.kind.data::<self::handlers::XdgWmBaseUserData>().unwrap();
        let mut guard = data.client_data.lock().unwrap();
        Ok(f(&mut guard.data))
    }
}

#[derive(Debug, Clone)]
pub struct ToplevelSurface {
    wl_surface: wl_surface::WlSurface,
    shell_surface: xdg_toplevel::XdgToplevel,
}

impl std::cmp::PartialEq for ToplevelSurface {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.wl_surface == other.wl_surface
    }
}

impl ToplevelSurface {
    #[inline]
    pub fn alive(&self) -> bool {
        self.wl_surface.alive() && self.shell_surface.alive()
    }

    pub fn version(&self) -> u32 {
        self.shell_surface.version()
    }

    pub fn client(&self) -> ShellClient {
        let shell = {
            let data = self
                .shell_surface
                .data::<self::handlers::XdgShellSurfaceUserData>()
                .unwrap();
            data.wm_base.clone()
        };

        ShellClient { kind: shell }
    }

    fn get_pending_state(&self, attributes: &mut XdgToplevelSurfaceRoleAttributes) -> Option<ToplevelState> {
        if !attributes.initial_configure_sent {
            return Some(
                attributes
                    .server_pending
                    .take()
                    .unwrap_or_else(|| attributes.current_server_state().clone()),
            );
        }

        if !attributes.has_pending_changes() {
            return None;
        }

        attributes.server_pending.take()
    }

    pub fn send_pending_configure(&self) -> Option<Serial> {
        if self.has_pending_changes() {
            Some(self.send_configure())
        } else {
            None
        }
    }

    pub fn send_configure(&self) -> Serial {
        let shell_surface_data = self.shell_surface.data::<XdgShellSurfaceUserData>();
        let decoration =
            shell_surface_data.and_then(|data| data.decoration.lock().unwrap().as_ref().cloned());
        let (configure, decoration_mode_changed, bounds_changed, capabilities_changed) =
            compositor::with_states(&self.wl_surface, |states| {
                let mut attributes = states
                    .data_map
                    .get::<XdgToplevelSurfaceData>()
                    .unwrap()
                    .lock()
                    .unwrap();

                let pending = self
                    .get_pending_state(&mut attributes)
                    .unwrap_or_else(|| attributes.current_server_state().clone());
                let current = attributes.current_server_state();

                let decoration_mode_changed = !attributes.initial_decoration_configure_sent
                    || (pending.decoration_mode != current.decoration_mode);

                let bounds_changed = !attributes.initial_configure_sent || (pending.bounds != current.bounds);

                let capabilities_changed =
                    !attributes.initial_configure_sent || (pending.capabilities != current.capabilities);

                let configure = ToplevelConfigure {
                    serial: SERIAL_COUNTER.next_serial(),
                    state: pending,
                };

                attributes.pending_configures.push(configure.clone());
                attributes.initial_configure_sent = true;
                if decoration.is_some() {
                    attributes.initial_decoration_configure_sent = true;
                }

                (
                    configure,
                    decoration_mode_changed,
                    bounds_changed,
                    capabilities_changed,
                )
            });

        if decoration_mode_changed {
            if let Some(decoration) = &decoration {
                self::decoration::send_decoration_configure(
                    decoration,
                    configure
                        .state
                        .decoration_mode
                        .unwrap_or(zxdg_toplevel_decoration_v1::Mode::ClientSide),
                );
            }
        }

        let serial = configure.serial;
        self::handlers::send_toplevel_configure(
            &self.shell_surface,
            configure,
            bounds_changed,
            capabilities_changed,
        );

        serial
    }

    pub fn is_initial_configure_sent(&self) -> bool {
        compositor::with_states(&self.wl_surface, |states| {
            states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .unwrap()
                .lock()
                .unwrap()
                .initial_configure_sent
        })
    }

    pub fn reset_initial_configure_sent(&self) {
        compositor::with_states(&self.wl_surface, |states| {
            let mut data = states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .unwrap()
                .lock()
                .unwrap();
            data.initial_configure_sent = false;
            data.initial_decoration_configure_sent = false;
        });
    }

    pub(crate) fn commit_hook<D: 'static>(
        _state: &mut D,
        _dh: &DisplayHandle,
        surface: &wl_surface::WlSurface,
    ) {
        let has_buffer = crate::backend::renderer::utils::with_renderer_surface_state(surface, |state| {
            state.buffer().is_some()
        });

        compositor::with_states(surface, |states| {
            let mut guard = states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .unwrap()
                .lock()
                .unwrap();

            if let Some(has_buffer) = has_buffer {
                if guard.has_buffer && !has_buffer {
                    guard.initial_configure_sent = false;
                    guard.initial_decoration_configure_sent = false;
                }

                guard.has_buffer = has_buffer;
            }

            if let Some(state) = guard.last_acked.clone() {
                guard.current = state;
                guard.current_serial = guard.configure_serial;
            }
        });
    }

    pub fn ensure_configured(&self) -> bool {
        let configured = compositor::with_states(&self.wl_surface, |states| {
            states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .unwrap()
                .lock()
                .unwrap()
                .configured
        });
        if !configured {
            let data = self
                .shell_surface
                .data::<self::handlers::XdgShellSurfaceUserData>()
                .unwrap();
            data.xdg_surface.post_error(
                xdg_surface::Error::NotConstructed,
                "Surface has not been configured yet.",
            );
        }
        configured
    }

    pub fn send_close(&self) {
        self.shell_surface.close()
    }

    #[inline]
    pub fn wl_surface(&self) -> &wl_surface::WlSurface {
        &self.wl_surface
    }

    pub fn xdg_toplevel(&self) -> &xdg_toplevel::XdgToplevel {
        &self.shell_surface
    }

    pub fn with_pending_state<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut ToplevelState) -> T,
    {
        compositor::with_states(&self.wl_surface, |states| {
            let mut attributes = states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .unwrap()
                .lock()
                .unwrap();
            if attributes.server_pending.is_none() {
                attributes.server_pending = Some(attributes.current_server_state().clone());
            }

            let server_pending = attributes.server_pending.as_mut().unwrap();
            f(server_pending)
        })
    }

    pub fn has_pending_changes(&self) -> bool {
        compositor::with_states(&self.wl_surface, |states| {
            let attributes = states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .unwrap()
                .lock()
                .unwrap();

            !attributes.initial_configure_sent || attributes.has_pending_changes()
        })
    }

    pub fn current_state(&self) -> ToplevelState {
        compositor::with_states(&self.wl_surface, |states| {
            let attributes = states
                .data_map
                .get::<XdgToplevelSurfaceData>()
                .unwrap()
                .lock()
                .unwrap();

            attributes.current.clone()
        })
    }

    pub fn parent(&self) -> Option<wl_surface::WlSurface> {
        handlers::get_parent(&self.shell_surface)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PopupConfigureError {
    #[error("The popup has already been configured")]
    AlreadyConfigured,
    #[error("The popup positioner is not reactive")]
    NotReactive,
}

#[derive(Debug, Clone)]
pub struct PopupSurface {
    wl_surface: wl_surface::WlSurface,
    shell_surface: xdg_popup::XdgPopup,
}

impl std::cmp::PartialEq for PopupSurface {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.wl_surface == other.wl_surface
    }
}

impl PopupSurface {
    #[inline]
    pub fn alive(&self) -> bool {
        self.wl_surface.alive() && self.shell_surface.alive()
    }

    pub fn get_parent_surface(&self) -> Option<wl_surface::WlSurface> {
        compositor::with_states(&self.wl_surface, |states| {
            states
                .data_map
                .get::<XdgPopupSurfaceData>()
                .unwrap()
                .lock()
                .unwrap()
                .parent
                .clone()
        })
    }

    pub fn client(&self) -> ShellClient {
        let shell = {
            let data = self
                .shell_surface
                .data::<self::handlers::XdgShellSurfaceUserData>()
                .unwrap();
            data.wm_base.clone()
        };

        ShellClient { kind: shell }
    }

    fn version(&self) -> u32 {
        self.shell_surface.version()
    }

    fn send_configure_internal(&self, reposition_token: Option<u32>) -> Serial {
        let configure = compositor::with_states(&self.wl_surface, |states| {
            let mut attributes = states
                .data_map
                .get::<XdgPopupSurfaceData>()
                .unwrap()
                .lock()
                .unwrap();

            let pending = attributes
                .server_pending
                .take()
                .unwrap_or_else(|| *attributes.current_server_state());

            let configure = PopupConfigure {
                state: pending,
                serial: SERIAL_COUNTER.next_serial(),
                reposition_token,
            };

            attributes.pending_configures.push(configure);
            attributes.initial_configure_sent = true;

            configure
        });
        let serial = configure.serial;
        self::handlers::send_popup_configure(&self.shell_surface, configure);
        serial
    }

    pub fn send_pending_configure(&self) -> Result<Option<Serial>, PopupConfigureError> {
        if self.has_pending_changes() {
            self.send_configure().map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn send_configure(&self) -> Result<Serial, PopupConfigureError> {
        compositor::with_states(&self.wl_surface, |states| {
            let attributes = states
                .data_map
                .get::<XdgPopupSurfaceData>()
                .unwrap()
                .lock()
                .unwrap();

            if attributes.initial_configure_sent && self.version() < xdg_popup::EVT_REPOSITIONED_SINCE {
                return Err(PopupConfigureError::AlreadyConfigured);
            }

            let is_reactive = attributes.current.positioner.reactive;

            if attributes.initial_configure_sent && !is_reactive {
                return Err(PopupConfigureError::NotReactive);
            }

            Ok(())
        })?;

        let serial = self.send_configure_internal(None);

        Ok(serial)
    }

    pub fn is_initial_configure_sent(&self) -> bool {
        compositor::with_states(&self.wl_surface, |states| {
            states
                .data_map
                .get::<XdgPopupSurfaceData>()
                .unwrap()
                .lock()
                .unwrap()
                .initial_configure_sent
        })
    }

    pub fn reset_initial_configure_sent(&self) {
        compositor::with_states(&self.wl_surface, |states| {
            let mut data = states
                .data_map
                .get::<XdgPopupSurfaceData>()
                .unwrap()
                .lock()
                .unwrap();
            data.initial_configure_sent = false;
        });
    }

    pub fn send_repositioned(&self, token: u32) -> Serial {
        self.send_configure_internal(Some(token))
    }

    pub(crate) fn pre_commit_hook<D: 'static>(
        _state: &mut D,
        _dh: &DisplayHandle,
        surface: &wl_surface::WlSurface,
    ) {
        let send_error_to = compositor::with_states(surface, |states| {
            let attributes = states
                .data_map
                .get::<XdgPopupSurfaceData>()
                .unwrap()
                .lock()
                .unwrap();
            if attributes.parent.is_none() {
                attributes.popup_handle.clone()
            } else {
                None
            }
        });
        if let Some(handle) = send_error_to {
            let data = handle.data::<self::handlers::XdgShellSurfaceUserData>().unwrap();
            data.xdg_surface.post_error(
                xdg_surface::Error::NotConstructed,
                "Surface has not been configured yet.",
            );
        }
    }

    pub(crate) fn post_commit_hook<D: 'static>(
        _state: &mut D,
        _dh: &DisplayHandle,
        surface: &wl_surface::WlSurface,
    ) {
        let has_buffer = crate::backend::renderer::utils::with_renderer_surface_state(surface, |state| {
            state.buffer().is_some()
        });

        compositor::with_states(surface, |states| {
            let mut attributes = states
                .data_map
                .get::<XdgPopupSurfaceData>()
                .unwrap()
                .lock()
                .unwrap();
            attributes.committed = true;

            if let Some(has_buffer) = has_buffer {
                if attributes.has_buffer && !has_buffer {
                    attributes.initial_configure_sent = false;
                }

                attributes.has_buffer = has_buffer;
            }

            if attributes.initial_configure_sent {
                if let Some(state) = attributes.last_acked {
                    attributes.current = state;
                    attributes.current_serial = attributes.configure_serial;
                }
            }
        });
    }

    pub fn ensure_configured(&self) -> bool {
        let configured = compositor::with_states(&self.wl_surface, |states| {
            states
                .data_map
                .get::<XdgPopupSurfaceData>()
                .unwrap()
                .lock()
                .unwrap()
                .configured
        });
        if !configured {
            let data = self
                .shell_surface
                .data::<self::handlers::XdgShellSurfaceUserData>()
                .unwrap();
            data.xdg_surface.post_error(
                xdg_surface::Error::NotConstructed,
                "Surface has not been configured yet.",
            );
        }
        configured
    }

    pub fn send_popup_done(&self) {
        self.shell_surface.popup_done();
    }

    #[inline]
    pub fn wl_surface(&self) -> &wl_surface::WlSurface {
        &self.wl_surface
    }

    pub fn xdg_popup(&self) -> &xdg_popup::XdgPopup {
        &self.shell_surface
    }

    pub fn with_pending_state<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut PopupState) -> T,
    {
        compositor::with_states(&self.wl_surface, |states| {
            let mut attributes = states
                .data_map
                .get::<XdgPopupSurfaceData>()
                .unwrap()
                .lock()
                .unwrap();
            if attributes.server_pending.is_none() {
                attributes.server_pending = Some(*attributes.current_server_state());
            }

            let server_pending = attributes.server_pending.as_mut().unwrap();
            f(server_pending)
        })
    }

    pub fn has_pending_changes(&self) -> bool {
        compositor::with_states(&self.wl_surface, |states| {
            let attributes = states
                .data_map
                .get::<XdgPopupSurfaceData>()
                .unwrap()
                .lock()
                .unwrap();

            !attributes.initial_configure_sent || attributes.has_pending_changes()
        })
    }
}

#[derive(Debug)]
pub enum Configure {
    Toplevel(ToplevelConfigure),
    Popup(PopupConfigure),
}

impl From<ToplevelConfigure> for Configure {
    #[inline]
    fn from(configure: ToplevelConfigure) -> Self {
        Configure::Toplevel(configure)
    }
}

impl From<PopupConfigure> for Configure {
    #[inline]
    fn from(configure: PopupConfigure) -> Self {
        Configure::Popup(configure)
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_xdg_shell {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::shell::server::xdg_wm_base::XdgWmBase: ()
        ] => $crate::wayland::shell::xdg::XdgShellState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::shell::server::xdg_wm_base::XdgWmBase: $crate::wayland::shell::xdg::XdgWmBaseUserData
        ] => $crate::wayland::shell::xdg::XdgShellState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::shell::server::xdg_positioner::XdgPositioner: $crate::wayland::shell::xdg::XdgPositionerUserData
        ] => $crate::wayland::shell::xdg::XdgShellState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::shell::server::xdg_popup::XdgPopup: $crate::wayland::shell::xdg::XdgShellSurfaceUserData
        ] => $crate::wayland::shell::xdg::XdgShellState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::shell::server::xdg_surface::XdgSurface: $crate::wayland::shell::xdg::XdgSurfaceUserData
        ] => $crate::wayland::shell::xdg::XdgShellState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::shell::server::xdg_toplevel::XdgToplevel: $crate::wayland::shell::xdg::XdgShellSurfaceUserData
        ] => $crate::wayland::shell::xdg::XdgShellState);
    };
}
