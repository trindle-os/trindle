// Origin: smithay-drm-extras (MIT, Copyright (c) Bartłomiej Maryńczak), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! Help with screens plugged in and out: which connectors have a screen,
//! which display pipe (CRTC) drives each, and what the screen says it is.
//! From smithay-drm-extras (MIT, Bartłomiej Maryńczak), kept here because the
//! published crate is older than the Smithay Trindle builds on.

pub mod display_info;
pub mod drm_scanner;
pub mod leftovers;
