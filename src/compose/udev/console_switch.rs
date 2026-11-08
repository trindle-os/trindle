// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use smithay::{
    backend::drm::DrmNode,
    reexports::input::Libinput,
};
use tracing::{error, info, warn};

use super::{
    gamma::{apply_gamma, gamma_length, linear_ramp, read_gamma},
    AnvilState, BackendData, UdevData,
};
use crate::drm_extras::leftovers;

pub(super) fn pause(data: &mut AnvilState<UdevData>, libinput_context: &Libinput) {
    libinput_context.suspend();
    info!("pausing session");

    data.reader_keys.release();
    data.expander.clear();

    for backend in data.backend_data.backends.values_mut() {
        backend.drm_output_manager.pause();
        backend.active_leases.clear();
        if let Some(lease_global) = backend.leasing_global.as_mut() {
            lease_global.suspend();
        }
    }
}

pub(super) fn resume(data: &mut AnvilState<UdevData>, libinput_context: &mut Libinput) {
    info!("resuming session");
    let mut failed = Vec::new();

    if let Err(err) = libinput_context.resume() {
        error!("Failed to resume libinput context: {:?}", err);
    }
    for (node, backend) in data
        .backend_data
        .backends
        .iter_mut()
        .map(|(handle, backend)| (*handle, backend))
    {
        if !retake_screens(node, backend) {
            failed.push(node);
            continue;
        }

        if let Err(err) = leftovers::clear(backend.drm_output_manager.device()) {
            warn!(?err, "could not switch off what was left lit on {}", node);
        }
        restore_screens(backend);
        if let Some(lease_global) = backend.leasing_global.as_mut() {
            lease_global.resume::<AnvilState<UdevData>>();
        }
    }

    data.handle.insert_idle(move |data| data.recheck_devices(&failed));
}

fn retake_screens(node: DrmNode, backend: &mut BackendData) -> bool {
    let manager = &mut backend.drm_output_manager;
    match crate::modeset_recovery::retake(true, |full| manager.activate(full)) {
        Ok(crate::modeset_recovery::Retaken::AsWas) => {}
        Ok(crate::modeset_recovery::Retaken::StartedOver) => {
            warn!("could not take the screens back as they were, started them over");
        }
        Err(err) => {
            error!(?err, "could not take the screens back on {}, opening it again", node);
            return false;
        }
    }
    true
}

fn restore_screens(backend: &mut BackendData) {
    let drm = backend.drm_output_manager.device();
    for (crtc, surface) in backend.surfaces.iter_mut() {
        if !(surface.enabled && surface.powered) {
            if let Err(err) = surface.drm_output.with_compositor(|c| c.clear()) {
                warn!(?err, "could not keep a screen off");
            }
        }
        if surface.gamma.is_some() && surface.original_gamma.is_none() {

            surface.original_gamma = gamma_length(drm, *crtc)
                .map(|n| read_gamma(drm, *crtc, n).unwrap_or_else(|_| linear_ramp(n)));
        }
        if let Some(table) = surface.gamma.as_ref().or(surface.original_gamma.as_ref()) {
            if let Err(err) = apply_gamma(drm, *crtc, table) {
                warn!(?err, "could not set a colour ramp again");
            }
        }
    }
}
