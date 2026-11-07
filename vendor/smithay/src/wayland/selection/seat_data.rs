use wayland_protocols_wlr::data_control::v1::server::zwlr_data_control_device_v1::EVT_PRIMARY_SELECTION_SINCE;
use wayland_server::protocol::wl_data_device::WlDataDevice;
use wayland_server::{Client, DisplayHandle, Resource};

use crate::utils::IsAlive;

use super::device::SelectionDevice;
use super::offer::{OfferReplySource, SelectionOffer};
use super::{SelectionHandler, SelectionTarget};

pub(crate) struct SeatData<U: Clone + Sync + Send + 'static> {
    known_devices: Vec<SelectionDevice>,
    clipboard_selection: Option<OfferReplySource<U>>,
    clipboard_selection_focus: Option<Client>,
    primary_selection: Option<OfferReplySource<U>>,
    primary_selection_focus: Option<Client>,
}

impl<U: Clone + Send + Sync + 'static> SeatData<U> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn retain_devices<F: FnMut(&SelectionDevice) -> bool>(&mut self, retain: F) {
        self.known_devices.retain(retain);
    }

    pub fn add_device(&mut self, device: SelectionDevice) {
        self.known_devices.push(device);
    }

    pub fn known_data_devices(&self) -> impl Iterator<Item = &WlDataDevice> {
        self.known_devices.iter().filter_map(|device| match device {
            SelectionDevice::DataDevice(device) => Some(device),
            _ => None,
        })
    }

    pub fn set_clipboard_focus<D>(&mut self, dh: &DisplayHandle, new_focus: Option<Client>)
    where
        D: SelectionHandler<SelectionUserData = U> + 'static,
    {
        if self.clipboard_selection_focus != new_focus {
            self.clipboard_selection_focus = new_focus;
            self.send_selection::<D>(dh, SelectionTarget::Clipboard, None, false)
        }
    }

    pub fn set_clipboard_selection<D>(
        &mut self,
        dh: &DisplayHandle,
        new_selection: Option<OfferReplySource<U>>,
    ) where
        D: SelectionHandler<SelectionUserData = U> + 'static,
    {
        if let Some(OfferReplySource::Client(source)) = self.clipboard_selection.as_ref() {
            match new_selection.as_ref() {
                Some(OfferReplySource::Client(new_source)) if new_source == source => (),
                _ => source.cancel(),
            }
        }
        self.clipboard_selection = new_selection;
        self.send_selection::<D>(dh, SelectionTarget::Clipboard, None, true)
    }

    pub fn set_primary_focus<D>(&mut self, dh: &DisplayHandle, new_focus: Option<Client>)
    where
        D: SelectionHandler<SelectionUserData = U> + 'static,
    {
        if self.primary_selection_focus != new_focus {
            self.primary_selection_focus = new_focus;
            self.send_selection::<D>(dh, SelectionTarget::Primary, None, false)
        }
    }

    pub fn set_primary_selection<D>(&mut self, dh: &DisplayHandle, new_selection: Option<OfferReplySource<U>>)
    where
        D: SelectionHandler<SelectionUserData = U> + 'static,
    {
        if let Some(OfferReplySource::Client(source)) = self.primary_selection.as_ref() {
            match new_selection.as_ref() {
                Some(OfferReplySource::Client(new_source)) if new_source == source => (),
                _ => source.cancel(),
            }
        }
        self.primary_selection = new_selection;
        self.send_selection::<D>(dh, SelectionTarget::Primary, None, true)
    }

    pub fn get_clipboard_selection(&self) -> Option<&OfferReplySource<U>> {
        self.clipboard_selection.as_ref()
    }

    pub fn get_primary_selection(&self) -> Option<&OfferReplySource<U>> {
        self.primary_selection.as_ref()
    }

    pub fn send_selection<D>(
        &mut self,
        dh: &DisplayHandle,
        ty: SelectionTarget,
        mut restrict_to: Option<&SelectionDevice>,
        mut update_data_control: bool,
    ) where
        D: SelectionHandler<SelectionUserData = U> + 'static,
    {
        let (client, selection) = match ty {
            SelectionTarget::Primary => (self.primary_selection_focus.as_ref(), &mut self.primary_selection),
            SelectionTarget::Clipboard => (
                self.clipboard_selection_focus.as_ref(),
                &mut self.clipboard_selection,
            ),
        };

        if selection.as_ref().is_some_and(|selection| {
            if let OfferReplySource::Client(source) = selection {
                !source.alive()
            } else {
                false
            }
        }) {
            update_data_control |= true;
            *selection = None;

            restrict_to = None;
        }

        for device in self
            .known_devices
            .iter()
            .filter(|&device| restrict_to.is_none() || restrict_to == Some(device))
            .filter(|&device| match device {
                SelectionDevice::DataDevice(_) => ty == SelectionTarget::Clipboard,
                SelectionDevice::Primary(_) => ty == SelectionTarget::Primary,
                SelectionDevice::WlrDataControl(data_control) => {
                    update_data_control
                        && (data_control.version() >= EVT_PRIMARY_SELECTION_SINCE
                            || ty != SelectionTarget::Primary)
                }
                SelectionDevice::ExtDataControl(_) => update_data_control,
            })
        {
            if !matches!(
                device,
                SelectionDevice::WlrDataControl(_) | SelectionDevice::ExtDataControl(_)
            ) && dh
                .get_client(device.id())
                .map(|c| Some(&c) != client)
                .unwrap_or(true)
            {
                continue;
            }

            match (&*selection, ty) {
                (None, SelectionTarget::Primary) => {
                    device.unset_primary_selection();
                    continue;
                }
                (None, SelectionTarget::Clipboard) => {
                    device.unset_selection();
                    continue;
                }
                (Some(ref selection), _) => {
                    let client_id = match device {
                        SelectionDevice::WlrDataControl(device) => {
                            dh.get_client(device.id()).ok().map(|c| c.id())
                        }
                        SelectionDevice::ExtDataControl(device) => {
                            dh.get_client(device.id()).ok().map(|c| c.id())
                        }
                        _ => client.map(|c| c.id()),
                    };

                    let client_id = match client_id {
                        Some(client_id) => client_id,
                        None => continue,
                    };

                    let offer = SelectionOffer::new::<D>(dh, device, client_id, selection.clone());

                    device.offer(&offer);

                    for mime_type in selection.mime_types() {
                        offer.offer(mime_type);
                    }

                    match ty {
                        SelectionTarget::Primary => device.primary_selection(&offer),
                        SelectionTarget::Clipboard => device.selection(&offer),
                    }
                }
            };
        }
    }
}

impl<U: Clone + Send + Sync + 'static> Default for SeatData<U> {
    fn default() -> Self {
        Self {
            known_devices: Vec::new(),
            clipboard_selection: None,
            clipboard_selection_focus: None,
            primary_selection: None,
            primary_selection_focus: None,
        }
    }
}
