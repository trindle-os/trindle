//! Implementation `xdg_foreign` protocol.

use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use rand::distr::{Alphanumeric, SampleString};
use wayland_protocols::xdg::foreign::zv2::server::{
    zxdg_exporter_v2::ZxdgExporterV2, zxdg_imported_v2::ZxdgImportedV2, zxdg_importer_v2::ZxdgImporterV2,
};
use wayland_server::{backend::GlobalId, protocol::wl_surface::WlSurface, DisplayHandle, GlobalDispatch};

mod handlers;

pub trait XdgForeignHandler: 'static {
    fn xdg_foreign_state(&mut self) -> &mut XdgForeignState;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct XdgForeignHandle(String);

impl XdgForeignHandle {
    fn new() -> Self {
        Self(Alphanumeric.sample_string(&mut rand::rng(), 32))
    }

    fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for XdgForeignHandle {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct XdgExportedUserData {
    handle: XdgForeignHandle,
}

#[derive(Debug)]
pub struct XdgImportedUserData {
    handle: XdgForeignHandle,
}

#[derive(Debug)]
struct ExportedState {
    exported_surface: WlSurface,
    requested_child: Option<(WlSurface, ZxdgImportedV2)>,
    imported_by: HashSet<ZxdgImportedV2>,
}

#[derive(Debug)]
pub struct XdgForeignState {
    exported: HashMap<XdgForeignHandle, ExportedState>,
    exporter: GlobalId,
    importer: GlobalId,
}

impl XdgForeignState {
    pub fn new<D>(display: &DisplayHandle) -> Self
    where
        D: XdgForeignHandler,
        D: GlobalDispatch<ZxdgExporterV2, ()>,
        D: GlobalDispatch<ZxdgImporterV2, ()>,
    {
        let exporter = display.create_global::<D, ZxdgExporterV2, _>(1, ());
        let importer = display.create_global::<D, ZxdgImporterV2, _>(1, ());

        Self {
            exported: HashMap::new(),
            exporter,
            importer,
        }
    }

    pub fn exporter_global(&self) -> GlobalId {
        self.exporter.clone()
    }

    pub fn importer_global(&self) -> GlobalId {
        self.importer.clone()
    }
}

#[macro_export]
macro_rules! delegate_xdg_foreign {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        type __ZxdgExporterV2 =
            $crate::reexports::wayland_protocols::xdg::foreign::zv2::server::zxdg_exporter_v2::ZxdgExporterV2;
        type __ZxdgImporterV2 =
            $crate::reexports::wayland_protocols::xdg::foreign::zv2::server::zxdg_importer_v2::ZxdgImporterV2;

        type __ZxdgExportedV2 =
            $crate::reexports::wayland_protocols::xdg::foreign::zv2::server::zxdg_exported_v2::ZxdgExportedV2;
        type __ZxdgImportedV2 =
            $crate::reexports::wayland_protocols::xdg::foreign::zv2::server::zxdg_imported_v2::ZxdgImportedV2;

        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __ZxdgExporterV2: ()
            ] => $crate::wayland::xdg_foreign::XdgForeignState
        );
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __ZxdgImporterV2: ()
            ] => $crate::wayland::xdg_foreign::XdgForeignState
        );

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __ZxdgExporterV2: ()
            ] => $crate::wayland::xdg_foreign::XdgForeignState
        );
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __ZxdgImporterV2: ()
            ] => $crate::wayland::xdg_foreign::XdgForeignState
        );

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __ZxdgExportedV2: $crate::wayland::xdg_foreign::XdgExportedUserData
            ] => $crate::wayland::xdg_foreign::XdgForeignState
        );
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __ZxdgImportedV2: $crate::wayland::xdg_foreign::XdgImportedUserData
            ] => $crate::wayland::xdg_foreign::XdgForeignState
        );
    };
}
