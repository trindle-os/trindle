//! KDE Window decoration manager.

use wayland_protocols_misc::server_decoration::server::org_kde_kwin_server_decoration::{
    Mode, OrgKdeKwinServerDecoration,
};
use wayland_protocols_misc::server_decoration::server::org_kde_kwin_server_decoration_manager::{
    Mode as DefaultMode, OrgKdeKwinServerDecorationManager,
};
use wayland_server::backend::GlobalId;
use wayland_server::protocol::wl_surface::WlSurface;
use wayland_server::{Client, Dispatch, DisplayHandle, GlobalDispatch, WEnum};

pub trait KdeDecorationHandler {
    fn kde_decoration_state(&self) -> &KdeDecorationState;

    fn new_decoration(&mut self, _surface: &WlSurface, _decoration: &OrgKdeKwinServerDecoration) {}

    fn request_mode(
        &mut self,
        _surface: &WlSurface,
        decoration: &OrgKdeKwinServerDecoration,
        mode: WEnum<Mode>,
    ) {
        if let WEnum::Value(mode) = mode {
            decoration.mode(mode);
        }
    }

    fn release(&mut self, _decoration: &OrgKdeKwinServerDecoration, _surface: &WlSurface) {}
}

#[derive(Debug)]
pub struct KdeDecorationState {
    pub(crate) default_mode: DefaultMode,

    kde_decoration_manager: GlobalId,
}

#[allow(missing_debug_implementations)]
pub struct KdeDecorationManagerGlobalData {
    pub(crate) filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

impl KdeDecorationState {
    pub fn new<D>(display: &DisplayHandle, default_mode: DefaultMode) -> Self
    where
        D: GlobalDispatch<OrgKdeKwinServerDecorationManager, KdeDecorationManagerGlobalData>
            + Dispatch<OrgKdeKwinServerDecorationManager, ()>
            + Dispatch<OrgKdeKwinServerDecoration, WlSurface>
            + KdeDecorationHandler
            + 'static,
    {
        Self::new_with_filter::<D, _>(display, default_mode, |_| true)
    }

    pub fn new_with_filter<D, F>(display: &DisplayHandle, default_mode: DefaultMode, filter: F) -> Self
    where
        D: GlobalDispatch<OrgKdeKwinServerDecorationManager, KdeDecorationManagerGlobalData>
            + Dispatch<OrgKdeKwinServerDecorationManager, ()>
            + Dispatch<OrgKdeKwinServerDecoration, WlSurface>
            + KdeDecorationHandler
            + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let data = KdeDecorationManagerGlobalData {
            filter: Box::new(filter),
        };
        let kde_decoration_manager =
            display.create_global::<D, OrgKdeKwinServerDecorationManager, _>(1, data);

        Self {
            kde_decoration_manager,
            default_mode,
        }
    }

    pub fn global(&self) -> GlobalId {
        self.kde_decoration_manager.clone()
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_kde_decoration {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_misc::server_decoration::server::org_kde_kwin_server_decoration_manager::OrgKdeKwinServerDecorationManager: $crate::wayland::shell::kde::decoration::KdeDecorationManagerGlobalData
        ] => $crate::wayland::shell::kde::decoration::KdeDecorationState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_misc::server_decoration::server::org_kde_kwin_server_decoration_manager::OrgKdeKwinServerDecorationManager: ()
        ] => $crate::wayland::shell::kde::decoration::KdeDecorationState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols_misc::server_decoration::server::org_kde_kwin_server_decoration::OrgKdeKwinServerDecoration: $crate::reexports::wayland_server::protocol::wl_surface::WlSurface
        ] => $crate::wayland::shell::kde::decoration::KdeDecorationState);
    };
}
