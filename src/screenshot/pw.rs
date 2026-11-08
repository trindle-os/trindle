//! PipeWire, for screen sharing: a video source the asking program reads
//! the screen from.

pub(super) mod callbacks;
pub(super) mod connection;
pub(super) mod format;
mod native;
pub(super) mod pod;
pub(super) mod stream;
#[cfg(test)]
mod tests;
