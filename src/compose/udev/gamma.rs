// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::io;

use smithay::{
    backend::drm::DrmDevice,
    reexports::drm::control::{crtc, Device},
};

pub(super) fn gamma_length(drm: &DrmDevice, crtc: crtc::Handle) -> Option<usize> {
    drm.get_crtc(crtc)
        .ok()
        .map(|info| info.gamma_length() as usize)
        .filter(|&n| n > 0)
}

pub(super) fn apply_gamma(drm: &DrmDevice, crtc: crtc::Handle, table: &[u16]) -> io::Result<()> {
    let n = table.len() / 3;
    let (red, rest) = table.split_at(n);
    let (green, blue) = rest.split_at(n);
    drm.set_gamma(crtc, red, green, blue)
}

pub(super) fn read_gamma(drm: &DrmDevice, crtc: crtc::Handle, n: usize) -> io::Result<Vec<u16>> {
    let mut table = vec![0u16; n * 3];
    let (red, rest) = table.split_at_mut(n);
    let (green, blue) = rest.split_at_mut(n);
    drm.get_gamma(crtc, red, green, blue)?;
    Ok(table)
}

pub(super) fn linear_ramp(n: usize) -> Vec<u16> {
    let step = |i: usize| (0xFFFF * i / (n - 1).max(1)) as u16;
    (0..3).flat_map(|_| (0..n).map(step)).collect()
}
