// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Lending screens that are not desktop screens (headsets) to programs that ask.

use std::collections::hash_map::HashMap;

use smithay::{
    backend::drm::DrmNode,
    delegate_drm_lease,
    wayland::drm_lease::{DrmLease, DrmLeaseBuilder, DrmLeaseHandler, DrmLeaseRequest, DrmLeaseState, LeaseRejected},
};
use tracing::warn;

use super::{AnvilState, BackendData, UdevData};

impl DrmLeaseHandler for AnvilState<UdevData> {
    fn drm_lease_state(&mut self, node: DrmNode) -> &mut DrmLeaseState {
        let data = &mut self.backend_data;
        lease_state_for(&mut data.backends, &mut data.retired_leases, node)
    }

    fn lease_request(
        &mut self,
        node: DrmNode,
        request: DrmLeaseRequest,
    ) -> Result<DrmLeaseBuilder, LeaseRejected> {
        let backend = self
            .backend_data
            .backends
            .get(&node)
            .ok_or(LeaseRejected::default())?;

        let drm_device = backend.drm_output_manager.device();
        let mut builder = DrmLeaseBuilder::new(drm_device);
        for conn in request.connectors {
            if let Some((_, crtc)) = backend
                .non_desktop_connectors
                .iter()
                .find(|(handle, _)| *handle == conn)
            {
                builder.add_connector(conn);
                builder.add_crtc(*crtc);
                let planes = drm_device.planes(crtc).map_err(LeaseRejected::with_cause)?;
                let (primary_plane, primary_plane_claim) = planes
                    .primary
                    .iter()
                    .find_map(|plane| {
                        drm_device
                            .claim_plane(plane.handle, *crtc)
                            .map(|claim| (plane, claim))
                    })
                    .ok_or_else(LeaseRejected::default)?;
                builder.add_plane(primary_plane.handle, primary_plane_claim);
                if let Some((cursor, claim)) = planes.cursor.iter().find_map(|plane| {
                    drm_device
                        .claim_plane(plane.handle, *crtc)
                        .map(|claim| (plane, claim))
                }) {
                    builder.add_plane(cursor.handle, claim);
                }
            } else {
                tracing::warn!(?conn, "Lease requested for desktop connector, denying request");
                return Err(LeaseRejected::default());
            }
        }

        Ok(builder)
    }

    fn new_active_lease(&mut self, node: DrmNode, lease: DrmLease) {
        let Some(backend) = self.backend_data.backends.get_mut(&node) else { return };
        backend.active_leases.push(lease);
    }

    fn lease_destroyed(&mut self, node: DrmNode, lease: u32) {
        let Some(backend) = self.backend_data.backends.get_mut(&node) else { return };
        backend.active_leases.retain(|l| l.id() != lease);
    }
}

delegate_drm_lease!(AnvilState<UdevData>);

fn lease_state_for<'a>(backends: &'a mut HashMap<DrmNode, BackendData>, retired: &'a mut HashMap<DrmNode, DrmLeaseState>, node: DrmNode) -> &'a mut DrmLeaseState {
    let state = if backends.get(&node).is_some_and(|b| b.leasing_global.is_some()) {
        backends.get_mut(&node).and_then(|b| b.leasing_global.as_mut())
    } else if retired.contains_key(&node) {
        retired.get_mut(&node)
    } else {
        warn!("a lease state was asked for by a card Trindle does not know: {node}");
        match retired.values_mut().next() {
            Some(other) => Some(other),
            None => backends.values_mut().find_map(|b| b.leasing_global.as_mut()),
        }
    };
    state.expect("a lease global exists only beside its state")
}
