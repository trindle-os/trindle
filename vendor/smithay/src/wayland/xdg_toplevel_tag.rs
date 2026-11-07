//! XDG Toplevel Tag.

use std::sync::{Arc, Mutex};

use wayland_protocols::xdg::{
    shell::server::xdg_toplevel::XdgToplevel,
    toplevel_tag::v1::server::xdg_toplevel_tag_manager_v1::{self, XdgToplevelTagManagerV1},
};

use wayland_server::{
    backend::GlobalId, Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource,
};

use crate::wayland::{compositor, shell::xdg::XdgShellSurfaceUserData};

#[derive(Default, Debug)]
pub struct XdgToplevelTagSurfaceData {
    tag: Mutex<Option<Arc<str>>>,
    description: Mutex<Option<Arc<str>>>,
}

impl XdgToplevelTagSurfaceData {
    pub fn tag(&self) -> Option<Arc<str>> {
        self.tag.lock().unwrap().clone()
    }

    pub fn description(&self) -> Option<Arc<str>> {
        self.description.lock().unwrap().clone()
    }
}

pub trait XdgToplevelTagHandler:
    GlobalDispatch<XdgToplevelTagManagerV1, ()> + Dispatch<XdgToplevelTagManagerV1, ()> + 'static
{
    #[allow(unused)]
    fn set_tag(&mut self, toplevel: XdgToplevel, tag: String) {}

    #[allow(unused)]
    fn set_description(&mut self, toplevel: XdgToplevel, description: String) {}
}

#[derive(Debug)]
pub struct XdgToplevelTagManager {
    global: GlobalId,
}

impl XdgToplevelTagManager {
    pub fn new<D: XdgToplevelTagHandler>(display: &DisplayHandle) -> Self {
        let global = display.create_global::<D, XdgToplevelTagManagerV1, _>(1, ());
        XdgToplevelTagManager { global }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

impl<D: XdgToplevelTagHandler> GlobalDispatch<XdgToplevelTagManagerV1, (), D> for XdgToplevelTagManager {
    fn bind(
        _state: &mut D,
        _dh: &DisplayHandle,
        _client: &Client,
        resource: New<XdgToplevelTagManagerV1>,
        _data: &(),
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }
}

impl<D: XdgToplevelTagHandler> Dispatch<XdgToplevelTagManagerV1, (), D> for XdgToplevelTagManager {
    fn request(
        state: &mut D,
        _client: &Client,
        _resource: &XdgToplevelTagManagerV1,
        request: xdg_toplevel_tag_manager_v1::Request,
        _data: &(),
        _dh: &DisplayHandle,
        _data_init: &mut DataInit<'_, D>,
    ) {
        use xdg_toplevel_tag_manager_v1::Request;

        match request {
            Request::SetToplevelTag { toplevel, tag } => {
                let data = toplevel.data::<XdgShellSurfaceUserData>().unwrap();

                compositor::with_states(&data.wl_surface, |states| {
                    let data = states
                        .data_map
                        .get_or_insert_threadsafe(XdgToplevelTagSurfaceData::default);
                    *data.tag.lock().unwrap() = Some(tag.clone().into());
                });

                state.set_tag(toplevel, tag);
            }
            Request::SetToplevelDescription {
                toplevel,
                description,
            } => {
                let data = toplevel.data::<XdgShellSurfaceUserData>().unwrap();

                compositor::with_states(&data.wl_surface, |states| {
                    let data = states
                        .data_map
                        .get_or_insert_threadsafe(XdgToplevelTagSurfaceData::default);
                    *data.tag.lock().unwrap() = Some(description.clone().into());
                });

                state.set_description(toplevel, description);
            }
            Request::Destroy => {}
            _ => unreachable!(),
        }
    }
}

#[macro_export]
macro_rules! delegate_xdg_toplevel_tag {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::toplevel_tag::v1::server::xdg_toplevel_tag_manager_v1::XdgToplevelTagManagerV1: ()
        ] => $crate::wayland::xdg_toplevel_tag::XdgToplevelTagManager);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::xdg::toplevel_tag::v1::server::xdg_toplevel_tag_manager_v1::XdgToplevelTagManagerV1: ()
        ] => $crate::wayland::xdg_toplevel_tag::XdgToplevelTagManager);
    };
}
