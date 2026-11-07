//! trindle_workspaces_v1 (workspaces.xml).

use smithay::reexports::wayland_server::backend::ClientId;
use smithay::reexports::wayland_server::{
    Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource,
};

#[allow(dead_code, non_camel_case_types, unused_unsafe, unused_variables, non_upper_case_globals, non_snake_case,
        unused_imports, missing_docs, clippy::all)]
pub mod server {
    use smithay::reexports::wayland_server;
    use smithay::reexports::wayland_server::protocol::*;

    pub mod __interfaces {
        use smithay::reexports::wayland_server::backend as wayland_backend;
        use smithay::reexports::wayland_server::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("src/compose/protocols/workspaces.xml");
    }
    use self::__interfaces::*;

    wayland_scanner::generate_server_code!("src/compose/protocols/workspaces.xml");
}

pub use server::trindle_workspaces_manager_v1::{self as manager, TrindleWorkspacesManagerV1};

const VERSION: u32 = 1;

pub trait WorkspacesHandler {
    fn workspaces_state(&mut self) -> &mut WorkspacesState;
    fn workspace_asked(&mut self, index: usize);
}

pub struct WorkspacesGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

#[derive(Debug)]
pub struct WorkspacesState {
    listeners: Vec<TrindleWorkspacesManagerV1>,
    row: Option<(u32, u32)>,
}

impl WorkspacesState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> WorkspacesState
    where
        D: GlobalDispatch<TrindleWorkspacesManagerV1, WorkspacesGlobalData>
            + Dispatch<TrindleWorkspacesManagerV1, ()>
            + WorkspacesHandler
            + 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        display.create_global::<D, TrindleWorkspacesManagerV1, _>(
            VERSION,
            WorkspacesGlobalData { filter: Box::new(filter) },
        );
        WorkspacesState { listeners: Vec::new(), row: None }
    }
}

pub fn refresh<D: WorkspacesHandler>(state: &mut D, count: usize, showing: usize) {
    let now = (count as u32, showing as u32);
    let st = state.workspaces_state();
    st.listeners.retain(|l| l.is_alive());
    if st.row == Some(now) {
        return;
    }
    st.row = Some(now);
    for listener in &st.listeners {
        listener.row(now.0, now.1);
    }
}

impl<D> GlobalDispatch<TrindleWorkspacesManagerV1, WorkspacesGlobalData, D> for WorkspacesState
where
    D: GlobalDispatch<TrindleWorkspacesManagerV1, WorkspacesGlobalData>
        + Dispatch<TrindleWorkspacesManagerV1, ()>
        + WorkspacesHandler
        + 'static,
{
    fn bind(
        state: &mut D,
        _: &DisplayHandle,
        _: &Client,
        resource: New<TrindleWorkspacesManagerV1>,
        _: &WorkspacesGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        let listener = data_init.init(resource, ());
        let st = state.workspaces_state();
        if let Some((count, showing)) = st.row {
            listener.row(count, showing);
        }
        st.listeners.push(listener);
    }

    fn can_view(client: Client, data: &WorkspacesGlobalData) -> bool {
        (data.filter)(&client)
    }
}

impl<D> Dispatch<TrindleWorkspacesManagerV1, (), D> for WorkspacesState
where
    D: Dispatch<TrindleWorkspacesManagerV1, ()> + WorkspacesHandler + 'static,
{
    fn request(
        state: &mut D,
        _: &Client,
        _: &TrindleWorkspacesManagerV1,
        request: manager::Request,
        _: &(),
        _: &DisplayHandle,
        _: &mut DataInit<'_, D>,
    ) {
        match request {
            manager::Request::Destroy => {}
            manager::Request::Go { index } => state.workspace_asked(index as usize),
        }
    }

    fn destroyed(state: &mut D, _: ClientId, listener: &TrindleWorkspacesManagerV1, _: &()) {
        state.workspaces_state().listeners.retain(|l| l != listener);
    }
}

#[macro_export]
macro_rules! delegate_workspaces {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        smithay::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::workspaces::TrindleWorkspacesManagerV1: $crate::protocols::workspaces::WorkspacesGlobalData
        ] => $crate::protocols::workspaces::WorkspacesState);
        smithay::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::protocols::workspaces::TrindleWorkspacesManagerV1: ()
        ] => $crate::protocols::workspaces::WorkspacesState);
    };
}
