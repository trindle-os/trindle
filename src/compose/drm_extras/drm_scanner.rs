// Origin: smithay-drm-extras (MIT, Copyright (c) Bartłomiej Maryńczak), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! # Drm Scanner
//!
//! - [`ConnectorScanner`] is responsible for tracking connected/disconnected events.
//! - [`CrtcMapper`] trait and [`SimpleCrtcMapper`] are meant for mapping crtc to connector.
//! - [`DrmScanner`] combines two above into single abstraction.
//!   If it does not fit your needs you can always drop down to using [`ConnectoScanner`] alone.
//!
//! ### Example
//! ```no_run
//! # mod helpers { include!("./docs/doctest_helpers.rs"); };
//! # let drm_device: helpers::FakeDevice = todo!();
//! use smithay_drm_extras::drm_scanner::{DrmScanner, DrmScanEvent};
//!
//! let mut scanner: DrmScanner = DrmScanner::new();
//!
//! for event in scanner.scan_connectors(&drm_device) {
//!     match event {
//!         DrmScanEvent::Connected { .. } => {},
//!         DrmScanEvent::Disconnected { .. } => {},
//!     }
//! }
//! ```

use std::{
    collections::HashMap,
    iter::{Chain, Map},
};

use drm::control::{connector, crtc, Device as ControlDevice};

mod connector_scanner;
pub use connector_scanner::ConnectorScanner;

mod crtc_mapper;
pub use crtc_mapper::{CrtcMapper, SimpleCrtcMapper};

#[derive(Debug, Default)]
pub struct DrmScanner<Mapper = SimpleCrtcMapper>
where
    Mapper: CrtcMapper,
{
    connectors: ConnectorScanner,
    crtc_mapper: Mapper,
}

impl<M> DrmScanner<M>
where
    M: CrtcMapper + Default,
{
    pub fn new() -> Self {
        Self::new_with_mapper(Default::default())
    }
}

impl<M> DrmScanner<M>
where
    M: CrtcMapper,
{
    pub fn new_with_mapper(mapper: M) -> Self {
        Self {
            connectors: Default::default(),
            crtc_mapper: mapper,
        }
    }

    pub fn scan_connectors(&mut self, drm: &impl ControlDevice) -> std::io::Result<DrmScanResult> {
        let scan = self.connectors.scan(drm)?;

        let removed = scan
            .disconnected
            .into_iter()
            .map(|info| {
                let crtc = self.crtc_mapper.crtc_for_connector(&info.handle());
                (info, crtc)
            })
            .collect();

        self.crtc_mapper
            .map(drm, self.connectors.connectors().values());

        let added = scan
            .connected
            .into_iter()
            .map(|info| {
                let crtc = self.crtc_mapper.crtc_for_connector(&info.handle());
                (info, crtc)
            })
            .collect();

        Ok(DrmScanResult {
            disconnected: removed,
            connected: added,
        })
    }

    pub fn connectors(&self) -> &HashMap<connector::Handle, connector::Info> {
        self.connectors.connectors()
    }

    pub fn crtc_for_connector(&self, connector: &connector::Handle) -> Option<crtc::Handle> {
        self.crtc_mapper.crtc_for_connector(connector)
    }

    pub fn crtcs(&self) -> impl Iterator<Item = (&connector::Info, crtc::Handle)> {
        self.connectors()
            .iter()
            .filter_map(|(handle, info)| Some((info, self.crtc_for_connector(handle)?)))
    }
}

type DrmScanItem = (connector::Info, Option<crtc::Handle>);

#[derive(Debug, Default, Clone)]
pub struct DrmScanResult {
    pub connected: Vec<DrmScanItem>,
    pub disconnected: Vec<DrmScanItem>,
}

#[derive(Debug, Clone)]
pub enum DrmScanEvent {
    Connected {
        connector: connector::Info,
        crtc: Option<crtc::Handle>,
    },
    Disconnected {
        connector: connector::Info,
        crtc: Option<crtc::Handle>,
    },
}

impl DrmScanEvent {
    fn connected((connector, crtc): (connector::Info, Option<crtc::Handle>)) -> Self {
        DrmScanEvent::Connected { connector, crtc }
    }

    fn disconnected((connector, crtc): (connector::Info, Option<crtc::Handle>)) -> Self {
        DrmScanEvent::Disconnected { connector, crtc }
    }
}

type DrmScanItemToEvent = fn(DrmScanItem) -> DrmScanEvent;

impl IntoIterator for DrmScanResult {
    type Item = DrmScanEvent;
    type IntoIter = Chain<
        Map<std::vec::IntoIter<DrmScanItem>, DrmScanItemToEvent>,
        Map<std::vec::IntoIter<DrmScanItem>, DrmScanItemToEvent>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        self.disconnected
            .into_iter()
            .map(DrmScanEvent::disconnected as DrmScanItemToEvent)
            .chain(
                self.connected
                    .into_iter()
                    .map(DrmScanEvent::connected as DrmScanItemToEvent),
            )
    }
}
