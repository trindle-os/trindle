//! Utilities for handling the security context protocol.

use std::{
    os::unix::{io::OwnedFd, net::UnixListener},
    sync::Mutex,
};
use tracing::error;
use wayland_protocols::wp::security_context::v1::server::{
    wp_security_context_manager_v1::{self, WpSecurityContextManagerV1},
    wp_security_context_v1::{self, WpSecurityContextV1},
};
use wayland_server::{
    backend::{ClientId, GlobalId},
    Client, DataInit, Dispatch, DisplayHandle, GlobalDispatch, New, Resource,
};

mod listener_source;
pub use listener_source::SecurityContextListenerSource;

const MANAGER_VERSION: u32 = 1;

pub trait SecurityContextHandler {
    fn context_created(&mut self, source: SecurityContextListenerSource, context: SecurityContext);
}

#[derive(Debug)]
pub struct SecurityContextState {
    global: GlobalId,
}

#[derive(Debug)]
pub struct SecurityContextUserData(Mutex<Option<SecurityContextBuilder>>);

#[derive(Debug)]
struct SecurityContextBuilder {
    listen_fd: UnixListener,
    close_fd: OwnedFd,
    sandbox_engine: Option<String>,
    app_id: Option<String>,
    instance_id: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SecurityContext {
    pub sandbox_engine: Option<String>,
    pub app_id: Option<String>,
    pub instance_id: Option<String>,
    pub creator_client_id: ClientId,
}

impl SecurityContextState {
    pub fn new<D, F>(display: &DisplayHandle, filter: F) -> Self
    where
        D: GlobalDispatch<WpSecurityContextManagerV1, SecurityContextGlobalData>,
        D: Dispatch<WpSecurityContextManagerV1, ()>,
        D: Dispatch<WpSecurityContextV1, SecurityContextUserData>,
        D: 'static,
        F: for<'c> Fn(&'c Client) -> bool + Send + Sync + 'static,
    {
        let global_data = SecurityContextGlobalData {
            filter: Box::new(filter),
        };
        let global = display.create_global::<D, WpSecurityContextManagerV1, _>(MANAGER_VERSION, global_data);

        Self { global }
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

#[allow(missing_debug_implementations)]
pub struct SecurityContextGlobalData {
    filter: Box<dyn for<'c> Fn(&'c Client) -> bool + Send + Sync>,
}

impl<D> GlobalDispatch<WpSecurityContextManagerV1, SecurityContextGlobalData, D> for SecurityContextState
where
    D: GlobalDispatch<WpSecurityContextManagerV1, SecurityContextGlobalData>,
    D: Dispatch<WpSecurityContextManagerV1, ()>,
    D: 'static,
{
    fn bind(
        _state: &mut D,
        _dh: &DisplayHandle,
        _client: &Client,
        resource: New<WpSecurityContextManagerV1>,
        _global_data: &SecurityContextGlobalData,
        data_init: &mut DataInit<'_, D>,
    ) {
        data_init.init(resource, ());
    }

    fn can_view(client: Client, global_data: &SecurityContextGlobalData) -> bool {
        (global_data.filter)(&client)
    }
}

impl<D> Dispatch<WpSecurityContextManagerV1, (), D> for SecurityContextState
where
    D: Dispatch<WpSecurityContextManagerV1, ()>,
    D: Dispatch<WpSecurityContextV1, SecurityContextUserData>,
    D: 'static,
{
    fn request(
        _state: &mut D,
        _client: &wayland_server::Client,
        _manager: &WpSecurityContextManagerV1,
        request: wp_security_context_manager_v1::Request,
        _data: &(),
        _dh: &DisplayHandle,
        data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        match request {
            wp_security_context_manager_v1::Request::CreateListener {
                id,
                listen_fd,
                close_fd,
            } => {
                let data = SecurityContextUserData(Mutex::new(Some(SecurityContextBuilder {
                    listen_fd: UnixListener::from(listen_fd),
                    close_fd,
                    sandbox_engine: None,
                    app_id: None,
                    instance_id: None,
                })));
                data_init.init(id, data);
            }
            wp_security_context_manager_v1::Request::Destroy => {}
            _ => unreachable!(),
        }
    }
}

impl<D> Dispatch<WpSecurityContextV1, SecurityContextUserData, D> for SecurityContextState
where
    D: Dispatch<WpSecurityContextV1, SecurityContextUserData> + 'static,
    D: SecurityContextHandler,
{
    fn request(
        state: &mut D,
        client: &wayland_server::Client,
        context: &WpSecurityContextV1,
        request: wp_security_context_v1::Request,
        data: &SecurityContextUserData,
        _dh: &DisplayHandle,
        _data_init: &mut wayland_server::DataInit<'_, D>,
    ) {
        let mut data = data.0.lock().unwrap();

        if matches!(request, wp_security_context_v1::Request::Destroy) {
            return;
        }

        let Some(builder) = &mut *data else {
            context.post_error(
                wp_security_context_v1::Error::AlreadyUsed,
                "Security context already used",
            );
            return;
        };

        match request {
            wp_security_context_v1::Request::SetSandboxEngine { name } => {
                if builder.sandbox_engine.is_some() {
                    context.post_error(
                        wp_security_context_v1::Error::AlreadySet,
                        "Security context already has a sandbox engine",
                    );
                }
                builder.sandbox_engine = Some(name);
            }
            wp_security_context_v1::Request::SetAppId { app_id } => {
                if builder.app_id.is_some() {
                    context.post_error(
                        wp_security_context_v1::Error::AlreadySet,
                        "Security context already has an app id",
                    );
                }
                builder.app_id = Some(app_id);
            }
            wp_security_context_v1::Request::SetInstanceId { instance_id } => {
                if builder.instance_id.is_some() {
                    context.post_error(
                        wp_security_context_v1::Error::AlreadySet,
                        "Security context already has an instance id",
                    );
                }
                builder.instance_id = Some(instance_id);
            }
            wp_security_context_v1::Request::Commit => {
                let builder = data.take().unwrap();
                let listener_source = SecurityContextListenerSource::new(builder.listen_fd, builder.close_fd);
                let security_context = SecurityContext {
                    sandbox_engine: builder.sandbox_engine,
                    app_id: builder.app_id,
                    instance_id: builder.instance_id,
                    creator_client_id: client.id(),
                };
                match listener_source {
                    Ok(listener_source) => state.context_created(listener_source, security_context),
                    Err(e) => {
                        error!(error = ?e, "Failed to create security context listener source");
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

#[macro_export]
macro_rules! delegate_security_context {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::security_context::v1::server::wp_security_context_manager_v1::WpSecurityContextManagerV1: $crate::wayland::security_context::SecurityContextGlobalData
        ] => $crate::wayland::security_context::SecurityContextState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::security_context::v1::server::wp_security_context_manager_v1::WpSecurityContextManagerV1: ()
        ] => $crate::wayland::security_context::SecurityContextState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_protocols::wp::security_context::v1::server::wp_security_context_v1::WpSecurityContextV1: $crate::wayland::security_context::SecurityContextUserData
        ] => $crate::wayland::security_context::SecurityContextState);
    };
}
