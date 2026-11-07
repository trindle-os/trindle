use std::os::unix::io::{AsFd, OwnedFd};

use wayland_protocols::ext::data_control::v1::server::ext_data_control_source_v1::ExtDataControlSourceV1;
use wayland_protocols::wp::primary_selection::zv1::server::zwp_primary_selection_source_v1::ZwpPrimarySelectionSourceV1 as PrimarySource;
use wayland_protocols_wlr::data_control::v1::server::zwlr_data_control_source_v1::ZwlrDataControlSourceV1;
use wayland_server::{protocol::wl_data_source::WlDataSource, Resource};

use crate::utils::IsAlive;
use crate::wayland::selection::primary_selection::PrimarySourceUserData;

use super::data_device::DataSourceUserData;
use super::ext_data_control::ExtDataControlSourceUserData;
use super::private::selection_dispatch;
use super::wlr_data_control::DataControlSourceUserData;
use super::SelectionTarget;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionSource {
    pub(crate) provider: SelectionSourceProvider,
}

impl SelectionSource {
    pub fn mime_types(&self) -> Vec<String> {
        self.provider.mime_types()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionSourceProvider {
    DataDevice(WlDataSource),
    Primary(PrimarySource),
    WlrDataControl(ZwlrDataControlSourceV1),
    ExtDataControl(ExtDataControlSourceV1),
}

impl SelectionSourceProvider {
    pub fn cancel(&self) {
        selection_dispatch!(self; Self(source) => source.cancelled())
    }

    pub fn send(&self, mime_type: String, fd: OwnedFd) {
        selection_dispatch!(self; Self(source) => source.send(mime_type, fd.as_fd()))
    }

    pub fn contains_mime_type(&self, mime_type: &String) -> bool {
        match self {
            Self::DataDevice(source) => {
                let data: &DataSourceUserData = source.data().unwrap();
                data.inner.lock().unwrap().mime_types.contains(mime_type)
            }
            Self::Primary(source) => {
                let data: &PrimarySourceUserData = source.data().unwrap();
                data.inner.lock().unwrap().mime_types.contains(mime_type)
            }
            Self::WlrDataControl(source) => {
                let data: &DataControlSourceUserData = source.data().unwrap();
                data.inner.lock().unwrap().mime_types.contains(mime_type)
            }
            Self::ExtDataControl(source) => {
                let data: &ExtDataControlSourceUserData = source.data().unwrap();
                data.inner.lock().unwrap().mime_types.contains(mime_type)
            }
        }
    }

    pub fn mime_types(&self) -> Vec<String> {
        match self {
            Self::DataDevice(source) => {
                let data: &DataSourceUserData = source.data().unwrap();
                data.inner.lock().unwrap().mime_types.clone()
            }
            Self::Primary(source) => {
                let data: &PrimarySourceUserData = source.data().unwrap();
                data.inner.lock().unwrap().mime_types.clone()
            }
            Self::WlrDataControl(source) => {
                let data: &DataControlSourceUserData = source.data().unwrap();
                data.inner.lock().unwrap().mime_types.clone()
            }
            Self::ExtDataControl(source) => {
                let data: &ExtDataControlSourceUserData = source.data().unwrap();
                data.inner.lock().unwrap().mime_types.clone()
            }
        }
    }
}

impl IsAlive for SelectionSourceProvider {
    #[inline]
    fn alive(&self) -> bool {
        selection_dispatch!(self; Self(source) => source.alive())
    }
}

#[derive(Debug, Clone)]
pub struct CompositorSelectionProvider<U: Clone + Send + Sync + 'static> {
    pub ty: SelectionTarget,
    pub mime_types: Vec<String>,
    pub user_data: U,
}
