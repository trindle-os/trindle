//! Buffer management traits.

use wayland_server::protocol::wl_buffer;

pub trait BufferHandler {
    fn buffer_destroyed(&mut self, buffer: &wl_buffer::WlBuffer);
}
