//! Wayland protocols Trindle's own programs need that Smithay does not carry.
//! Trindle's own, each with its XML beside it and named trindle_*_v1: the
//! window list (windows), the row of workspaces (workspaces), the screens
//! and how they are arranged (screens), night light (night), steering this
//! computer from another (input_capture), unlocking with the PIN (unlock),
//! and the word to a window of which presses the person made themselves
//! (own_press). Standard ones Smithay lacks: copies of one screen or window
//! for screenshots and screen sharing (image_capture, ext-image-copy-capture),
//! screens off when idle (output_power), and a made-up mouse (virtual_pointer).
//! `fill` draws into the buffers copies are made in.

pub mod fill;
pub mod night;
pub mod image_capture;
pub mod input_capture;
pub mod own_press;
pub mod screens;
pub mod unlock;
pub mod output_power;
pub mod virtual_pointer;
pub mod windows;
pub mod workspaces;

#[cfg(test)]
mod tests;
