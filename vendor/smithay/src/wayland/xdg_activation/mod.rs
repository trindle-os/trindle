//! Utilities for handling activation requests with the `xdg_activation` protocol.

use std::{
    collections::HashMap,
    ops,
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Instant,
};

use wayland_protocols::xdg::activation::v1::server::xdg_activation_v1;
use wayland_server::{
    backend::{ClientId, GlobalId},
    protocol::{wl_seat::WlSeat, wl_surface::WlSurface},
    Dispatch, DisplayHandle, GlobalDispatch,
};

use rand::distr::{Alphanumeric, SampleString};

use crate::utils::{user_data::UserDataMap, Serial};

mod dispatch;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct XdgActivationToken(String);

impl XdgActivationToken {
    fn new() -> Self {
        Self(Alphanumeric.sample_string(&mut rand::rng(), 32))
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ops::Deref for XdgActivationToken {
    type Target = str;
    #[inline]
    fn deref(&self) -> &str {
        &self.0
    }
}

impl From<String> for XdgActivationToken {
    #[inline]
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<XdgActivationToken> for String {
    #[inline]
    fn from(s: XdgActivationToken) -> Self {
        s.0
    }
}

#[derive(Debug, Clone)]
pub struct XdgActivationTokenData {
    pub client_id: Option<ClientId>,
    pub serial: Option<(Serial, WlSeat)>,
    pub app_id: Option<String>,
    pub surface: Option<WlSurface>,
    pub timestamp: Instant,
    pub user_data: Arc<UserDataMap>,
}

impl XdgActivationTokenData {
    fn new(
        client_id: Option<ClientId>,
        serial: Option<(Serial, WlSeat)>,
        app_id: Option<String>,
        surface: Option<WlSurface>,
    ) -> (XdgActivationToken, XdgActivationTokenData) {
        (
            XdgActivationToken::new(),
            XdgActivationTokenData {
                client_id,
                serial,
                app_id,
                surface,
                timestamp: Instant::now(),
                user_data: Arc::new(UserDataMap::new()),
            },
        )
    }
}

impl Default for XdgActivationTokenData {
    fn default() -> Self {
        Self {
            client_id: None,
            serial: None,
            app_id: None,
            surface: None,
            timestamp: Instant::now(),
            user_data: Arc::new(UserDataMap::new()),
        }
    }
}

#[derive(Debug)]
pub struct XdgActivationState {
    global: GlobalId,
    known_tokens: HashMap<XdgActivationToken, XdgActivationTokenData>,
}

impl XdgActivationState {
    pub fn new<D>(display: &DisplayHandle) -> XdgActivationState
    where
        D: GlobalDispatch<xdg_activation_v1::XdgActivationV1, ()>
            + Dispatch<xdg_activation_v1::XdgActivationV1, ()>
            + XdgActivationHandler
            + 'static,
    {
        let global = display.create_global::<D, xdg_activation_v1::XdgActivationV1, _>(1, ());

        XdgActivationState {
            global,
            known_tokens: HashMap::new(),
        }
    }

    pub fn create_external_token(
        &mut self,
        data: impl Into<Option<XdgActivationTokenData>>,
    ) -> (&XdgActivationToken, &XdgActivationTokenData) {
        let token = XdgActivationToken::new();
        let data = data.into().unwrap_or_default();
        self.known_tokens.insert(token.clone(), data);
        self.known_tokens.get_key_value(&token).unwrap()
    }

    pub fn tokens(&self) -> impl Iterator<Item = (&XdgActivationToken, &XdgActivationTokenData)> {
        self.known_tokens.iter()
    }

    pub fn data_for_token(&self, token: &XdgActivationToken) -> Option<&XdgActivationTokenData> {
        self.known_tokens.get(token)
    }

    pub fn retain_tokens<F>(&mut self, mut f: F)
    where
        F: FnMut(&XdgActivationToken, &XdgActivationTokenData) -> bool,
    {
        self.known_tokens.retain(|k, v| f(k, v))
    }

    pub fn remove_token(&mut self, token: &XdgActivationToken) -> bool {
        self.known_tokens.remove(token).is_some()
    }

    pub fn global(&self) -> GlobalId {
        self.global.clone()
    }
}

pub trait XdgActivationHandler {
    fn activation_state(&mut self) -> &mut XdgActivationState;

    fn token_created(&mut self, token: XdgActivationToken, data: XdgActivationTokenData) -> bool {
        let _ = (token, data);
        true
    }

    fn request_activation(
        &mut self,
        token: XdgActivationToken,
        token_data: XdgActivationTokenData,
        surface: WlSurface,
    );
}

#[derive(Debug)]
pub struct ActivationTokenData {
    constructed: AtomicBool,
    build: Mutex<TokenBuilder>,
    token: Mutex<Option<XdgActivationToken>>,
}

#[macro_export]
macro_rules! delegate_xdg_activation {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        type __XdgActivationV1 =
            $crate::reexports::wayland_protocols::xdg::activation::v1::server::xdg_activation_v1::XdgActivationV1;
        type __XdgActivationTokenV1 =
            $crate::reexports::wayland_protocols::xdg::activation::v1::server::xdg_activation_token_v1::XdgActivationTokenV1;

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            __XdgActivationV1: ()
        ] => $crate::wayland::xdg_activation::XdgActivationState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            __XdgActivationTokenV1: $crate::wayland::xdg_activation::ActivationTokenData
        ] => $crate::wayland::xdg_activation::XdgActivationState);

        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty:
            [
                __XdgActivationV1: ()
            ] => $crate::wayland::xdg_activation::XdgActivationState
        );
    };
}

#[derive(Debug)]
struct TokenBuilder {
    serial: Option<(Serial, WlSeat)>,
    app_id: Option<String>,
    surface: Option<WlSurface>,
}
