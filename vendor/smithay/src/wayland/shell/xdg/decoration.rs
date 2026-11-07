//! XDG Window decoration manager.

use wayland_protocols::xdg::decoration::zv1::server::{
    zxdg_decoration_manager_v1,
    zxdg_toplevel_decoration_v1::{self, Mode},
};
use wayland_server::{
    backend::GlobalId, Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource, WEnum,
};

use super::{ToplevelSurface, XdgShellHandler};
use crate::wayland::shell::xdg::XdgShellSurfaceUserData;

#[derive(Debug)]
pub struct XdgDecorationState {
    global: GlobalId,
}

#[allow(missing_debug_implementations)]
pub struct XdgDecorationManagerGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

impl XdgDecorationState {
    pub fn new<D>(display: &DisplayHandle) -> XdgDecorationState
    where
        D: GlobalDispatch<
                zxdg_decoration_manager_v1::ZxdgDecorationManagerV1,
                XdgDecorationManagerGlobalData,
            > + Dispatch<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1, ()>
            + 'static,
    {
        Self::new_with_filter::<D, _>(display, |_| true)
    }

    pub fn new_with_filter<D, F>(display: &DisplayHandle, filter: F) -> XdgDecorationState
    where
        D: GlobalDispatch<
                zxdg_decoration_manager_v1::ZxdgDecorationManagerV1,
                XdgDecorationManagerGlobalData,
            > + Dispatch<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1, ()>
            + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let data = XdgDecorationManagerGlobalData {
            filter: Box::new(filter),
        };
        let global =
            display.create_global::<D, zxdg_decoration_manager_v1::ZxdgDecorationManagerV1, _>(1, data);

        XdgDecorationState { global }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

pub trait XdgDecorationHandler {
    fn new_decoration(&mut self, toplevel: ToplevelSurface);

    fn request_mode(&mut self, toplevel: ToplevelSurface, mode: Mode);

    fn unset_mode(&mut self, toplevel: ToplevelSurface);
}

#[macro_export]
macro_rules! delegate_xdg_decoration {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::decoration::zv1::server::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1: $crate::wayland::shell::xdg::decoration::XdgDecorationManagerGlobalData
        ] => $crate::wayland::shell::xdg::decoration::XdgDecorationState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::decoration::zv1::server::zxdg_decoration_manager_v1::ZxdgDecorationManagerV1: ()
        ] => $crate::wayland::shell::xdg::decoration::XdgDecorationState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::decoration::zv1::server::zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1: $crate::wayland::shell::xdg::ToplevelSurface
        ] => $crate::wayland::shell::xdg::decoration::XdgDecorationState);
    };
}

pub(super) fn send_decoration_configure(
    id: &zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1,
    mode: Mode,
) {
    id.configure(mode)
}

impl<D> GlobalDispatch<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1, XdgDecorationManagerGlobalData, D>
    for XdgDecorationState
where
    D: GlobalDispatch<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1, XdgDecorationManagerGlobalData>
        + Dispatch<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1, ()>
        + Dispatch<zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1, ToplevelSurface>
        + XdgShellHandler
        + XdgDecorationHandler
        + 'static,
{
    fn bind(
        _: &mut D,
        _: &DisplayHandle,
        _: &Client,
        resource: New<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1>,
        _: &XdgDecorationManagerGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }

    fn can_view(client: Client, global_data: &XdgDecorationManagerGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

impl<D> Dispatch<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1, (), D> for XdgDecorationState
where
    D: Dispatch<zxdg_decoration_manager_v1::ZxdgDecorationManagerV1, ()>
        + Dispatch<zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1, ToplevelSurface>
        + XdgShellHandler
        + XdgDecorationHandler
        + 'static,
{
    fn request(
        state: &mut D,
        _: &Client,
        resource: &zxdg_decoration_manager_v1::ZxdgDecorationManagerV1,
        request: zxdg_decoration_manager_v1::Request,
        _: &(),
        _dh: &DisplayHandle,
        data_init: &mut DataInit<'_, D>,
    ) {
        use self::zxdg_decoration_manager_v1::Request;

        match request {
            Request::GetToplevelDecoration { id, toplevel } => {
                let data = toplevel.data::<XdgShellSurfaceUserData>().unwrap();

                let mut decoration_guard = data.decoration.lock().unwrap();

                if decoration_guard.is_some() {
                    resource.post_error(
                        zxdg_toplevel_decoration_v1::Error::AlreadyConstructed,
                        "toplevel decoration is already constructed",
                    );
                    return;
                }

                let toplevel = state.xdg_shell_state().get_toplevel(&toplevel).unwrap();
                let toplevel_decoration = data_init.init(id, toplevel.clone());

                *decoration_guard = Some(toplevel_decoration);
                drop(decoration_guard);

                state.new_decoration(toplevel);
            }

            Request::Destroy => {}

            _ => unreachable!(),
        }
    }
}

impl<D> Dispatch<zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1, ToplevelSurface, D>
    for XdgDecorationState
where
    D: Dispatch<zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1, ToplevelSurface>
        + XdgDecorationHandler,
{
    fn request(
        state: &mut D,
        _: &Client,
        _: &zxdg_toplevel_decoration_v1::ZxdgToplevelDecorationV1,
        request: zxdg_toplevel_decoration_v1::Request,
        data: &ToplevelSurface,
        _dh: &DisplayHandle,
        _: &mut DataInit<'_, D>,
    ) {
        use self::zxdg_toplevel_decoration_v1::Request;

        match request {
            Request::SetMode { mode } => {
                if let WEnum::Value(mode) = mode {
                    state.request_mode(data.clone(), mode);
                }
            }

            Request::UnsetMode => {
                state.unset_mode(data.clone());
            }

            Request::Destroy => {
                if let Some(data) = data.xdg_toplevel().data::<XdgShellSurfaceUserData>() {
                    data.decoration.lock().unwrap().take();
                }
            }

            _ => unreachable!(),
        }
    }
}
