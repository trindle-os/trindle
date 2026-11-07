//! What whoever held the screens before (the login screen, the turning mark while the computer starts, a text console) left lit.

use smithay::{
    backend::drm::DrmDevice,
    reexports::drm::control::{
        atomic::AtomicModeReq, connector, crtc, property, AtomicCommitFlags, Device, PlaneType,
        ResourceHandle, ResourceHandles,
    },
};
use std::{collections::HashSet, io};

pub fn clear(drm: &DrmDevice) -> io::Result<()> {
    let res = drm.resource_handles()?;
    let (stale, stale_connectors) = stale_pipes(drm, &res);

    if !drm.is_atomic() {
        clear_without_atomic(drm, &res, &stale);
        return Ok(());
    }

    let mut req = AtomicModeReq::new();
    let mut changes = false;
    for plane in drm.plane_handles()? {
        let Some(crtc) = drm.get_plane(plane)?.crtc() else { continue };
        if is_primary(drm, plane) && !stale.contains(&crtc) {
            continue;
        }
        req.add_property(plane, prop(drm, plane, "CRTC_ID")?, property::Value::CRTC(None));
        req.add_property(plane, prop(drm, plane, "FB_ID")?, property::Value::Framebuffer(None));
        changes = true;
    }
    for conn in stale_connectors {
        req.add_property(conn, prop(drm, conn, "CRTC_ID")?, property::Value::CRTC(None));
    }
    for crtc in &stale {
        req.add_property(*crtc, prop(drm, *crtc, "ACTIVE")?, property::Value::Boolean(false));
        req.add_property(*crtc, prop(drm, *crtc, "MODE_ID")?, property::Value::Unknown(0));
        changes = true;
    }
    if changes {
        drm.atomic_commit(AtomicCommitFlags::ALLOW_MODESET, req)?;
    }
    Ok(())
}

fn stale_pipes(drm: &DrmDevice, res: &ResourceHandles) -> (HashSet<crtc::Handle>, Vec<connector::Handle>) {
    let mut stale: HashSet<crtc::Handle> = HashSet::new();
    let mut lit: HashSet<crtc::Handle> = HashSet::new();
    let mut stale_connectors = Vec::new();
    for conn in res.connectors().iter().filter_map(|c| drm.get_connector(*c, false).ok()) {
        let Some(crtc) = conn.current_encoder().and_then(|e| drm.get_encoder(e).ok()).and_then(|e| e.crtc())
        else {
            continue;
        };
        if conn.state() == connector::State::Connected {
            lit.insert(crtc);
        } else {
            stale.insert(crtc);
            stale_connectors.push(conn.handle());
        }
    }
    stale.retain(|crtc| !lit.contains(crtc));
    for crtc in res.crtcs() {
        if !lit.contains(crtc) && drm.get_crtc(*crtc).is_ok_and(|info| info.mode().is_some()) {
            stale.insert(*crtc);
        }
    }
    (stale, stale_connectors)
}

fn clear_without_atomic(drm: &DrmDevice, res: &ResourceHandles, stale: &HashSet<crtc::Handle>) {
    for crtc in res.crtcs() {
        #[allow(deprecated)]
        let _ = drm.set_cursor(*crtc, Option::<&smithay::reexports::drm::control::dumbbuffer::DumbBuffer>::None);
    }
    for crtc in stale {
        let _ = drm.set_crtc(*crtc, None, (0, 0), &[], None);
    }
}

fn prop(drm: &DrmDevice, handle: impl ResourceHandle, name: &str) -> io::Result<property::Handle> {
    let props = drm.get_properties(handle)?;
    for p in props.as_props_and_values().0 {
        if drm.get_property(*p)?.name().to_bytes() == name.as_bytes() {
            return Ok(*p);
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, format!("no {name} property")))
}

fn is_primary(drm: &DrmDevice, plane: impl ResourceHandle) -> bool {
    let Ok(props) = drm.get_properties(plane) else { return false };
    let (handles, values) = props.as_props_and_values();
    handles.iter().zip(values).any(|(p, v)| {
        drm.get_property(*p).is_ok_and(|info| {
            info.name().to_bytes() == b"type"
                && matches!(info.value_type().convert_value(*v),
                    property::Value::Enum(Some(e)) if e.value() == PlaneType::Primary as u64)
        })
    })
}

pub fn cap_depth(drm: &DrmDevice, conn: connector::Handle) -> io::Result<()> {
    let Ok(handle) = prop(drm, conn, "max bpc") else { return Ok(()) };
    let info = drm.get_property(handle)?;
    let property::ValueType::UnsignedRange(min, max) = info.value_type() else { return Ok(()) };
    let want = 8u64.clamp(min, max);
    let props = drm.get_properties(conn)?;
    let (handles, values) = props.as_props_and_values();
    let now = handles.iter().zip(values).find(|(p, _)| **p == handle).map(|(_, v)| *v);
    if now != Some(want) {
        drm.set_property(conn, handle, want)?;
    }
    Ok(())
}
