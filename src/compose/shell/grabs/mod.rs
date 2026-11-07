// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! The pointer and touch grabs that move and resize windows and hold presses on panels.
//!
//! Origin: began as a file of anvil, Smithay's example compositor (MIT, Victor Berger and
//! Victoria Brekenfeld). About 74% of this file's lines still read as
//! they did there; the rest is Trindle's own.

mod layer;
mod move_grab;
mod resize;
mod resize_touch;

pub use layer::LayerClickGrab;
pub use move_grab::{PointerMoveSurfaceGrab, TouchMoveSurfaceGrab};
pub use resize::{ResizeData, ResizeEdge, ResizeState};
pub use resize_touch::TouchResizeSurfaceGrab;
