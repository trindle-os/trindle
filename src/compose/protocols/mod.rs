//! Wayland protocols Trindle's own programs need that Smithay does not carry.

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
