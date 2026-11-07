use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use drm::control::{connector, crtc, Device as ControlDevice};

use super::DrmDeviceFd;
use crate::backend::drm::error::{AccessError, Error};
use crate::utils::DevPath;

use tracing::{debug, error, info_span, trace};

#[derive(Debug)]
pub struct LegacyDrmDevice {
    pub(crate) fd: DrmDeviceFd,
    pub(crate) active: Arc<AtomicBool>,
    old_state: HashMap<crtc::Handle, (crtc::Info, Vec<connector::Handle>)>,
    pub(super) span: tracing::Span,
}

impl LegacyDrmDevice {
    pub fn new(fd: DrmDeviceFd, active: Arc<AtomicBool>, disable_connectors: bool) -> Result<Self, Error> {
        let span = info_span!("drm_legacy");
        let mut dev = LegacyDrmDevice {
            fd,
            active,
            old_state: HashMap::new(),
            span,
        };
        let _guard = dev.span.enter();

        let res_handles = dev.fd.resource_handles().map_err(|source| {
            Error::Access(AccessError {
                errmsg: "Error loading drm resources",
                dev: dev.fd.dev_path(),
                source,
            })
        })?;
        for &con in res_handles.connectors() {
            let con_info = dev.fd.get_connector(con, false).map_err(|source| {
                Error::Access(AccessError {
                    errmsg: "Error loading connector info",
                    dev: dev.fd.dev_path(),
                    source,
                })
            })?;
            if let Some(enc) = con_info.current_encoder() {
                let enc_info = dev.fd.get_encoder(enc).map_err(|source| {
                    Error::Access(AccessError {
                        errmsg: "Error loading encoder info",
                        dev: dev.fd.dev_path(),
                        source,
                    })
                })?;
                if let Some(crtc) = enc_info.crtc() {
                    let info = dev.fd.get_crtc(crtc).map_err(|source| {
                        Error::Access(AccessError {
                            errmsg: "Error loading crtc info",
                            dev: dev.fd.dev_path(),
                            source,
                        })
                    })?;
                    dev.old_state
                        .entry(crtc)
                        .or_insert((info, Vec::new()))
                        .1
                        .push(con);
                }
            }
        }

        if disable_connectors {
            debug!("Resetting drm device to known state");
            dev.reset_state()?;
        }

        drop(_guard);
        Ok(dev)
    }

    pub(super) fn reset_state(&self) -> Result<(), Error> {
        let res_handles = self.fd.resource_handles().map_err(|source| {
            Error::Access(AccessError {
                errmsg: "Failed to query resource handles",
                dev: self.fd.dev_path(),
                source,
            })
        })?;
        set_connector_state(&self.fd, res_handles.connectors().iter().copied(), false)?;

        for crtc in res_handles.crtcs() {
            #[allow(deprecated)]
            let _ = self
                .fd
                .set_cursor(*crtc, Option::<&drm::control::dumbbuffer::DumbBuffer>::None);
            self.fd
                .set_crtc(*crtc, None, (0, 0), &[], None)
                .map_err(|source| {
                    Error::Access(AccessError {
                        errmsg: "Error setting crtc",
                        dev: self.fd.dev_path(),
                        source,
                    })
                })?;
        }

        Ok(())
    }
}

impl Drop for LegacyDrmDevice {
    fn drop(&mut self) {
        if self.active.load(Ordering::SeqCst) {
            let _guard = self.span.enter();

            debug!("Device still active, trying to restore previous state");
            for (handle, (info, connectors)) in self.old_state.drain() {
                trace!(
                    framebuffer = ?info.framebuffer(),
                    offset = ?info.position(),
                    ?connectors,
                    mode = ?info.mode(),
                    "Resetting crtc {:?}",
                    handle,
                );
                if let Err(err) = self.fd.set_crtc(
                    handle,
                    info.framebuffer(),
                    info.position(),
                    &connectors,
                    info.mode(),
                ) {
                    error!("Failed to reset crtc ({:?}). Error: {}", handle, err);
                }
            }
        }
    }
}

pub fn set_connector_state<D>(
    dev: &D,
    connectors: impl Iterator<Item = connector::Handle>,
    enabled: bool,
) -> Result<(), Error>
where
    D: DevPath + ControlDevice,
{
    for conn in connectors {
        let info = dev.get_connector(conn, false).map_err(|source| {
            Error::Access(AccessError {
                errmsg: "Failed to get connector infos",
                dev: dev.dev_path(),
                source,
            })
        })?;
        if info.state() == connector::State::Connected {
            let props = dev.get_properties(conn).map_err(|source| {
                Error::Access(AccessError {
                    errmsg: "Failed to get properties for connector",
                    dev: dev.dev_path(),
                    source,
                })
            })?;
            let (handles, _) = props.as_props_and_values();
            for handle in handles {
                let info = dev.get_property(*handle).map_err(|source| {
                    Error::Access(AccessError {
                        errmsg: "Failed to get property of connector",
                        dev: dev.dev_path(),
                        source,
                    })
                })?;
                if info.name().to_str().map(|x| x == "DPMS").unwrap_or(false) {
                    trace!(connector = ?conn, "Setting DPMS {}", enabled);
                    dev.set_property(
                        conn,
                        *handle,
                        if enabled {
                            drm_ffi::DRM_MODE_DPMS_ON.into()
                        } else {
                            drm_ffi::DRM_MODE_DPMS_OFF.into()
                        },
                    )
                    .map_err(|source| {
                        Error::Access(AccessError {
                            errmsg: "Failed to set property of connector",
                            dev: dev.dev_path(),
                            source,
                        })
                    })?;
                }
            }
        }
    }
    Ok(())
}
