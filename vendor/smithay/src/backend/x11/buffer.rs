//! Utilities for importing buffers into X11.

use std::sync::atomic::Ordering;

use super::{PresentError, Window, X11Error};
use drm_fourcc::DrmFourcc;
use x11rb::{
    connection::Connection,
    protocol::{
        dri3::ConnectionExt as _,
        present::{self, ConnectionExt},
        xproto::PixmapWrapper,
    },
};

use crate::backend::allocator::{dmabuf::Dmabuf, Buffer};

pub trait PixmapWrapperExt<C>
where
    C: Connection,
{
    fn with_dmabuf(connection: C, window: &Window, dmabuf: &Dmabuf) -> Result<PixmapWrapper<C>, X11Error>;

    fn present(self, connection: C, window: &Window) -> Result<u32, X11Error>;
}

impl<C> PixmapWrapperExt<C> for PixmapWrapper<C>
where
    C: Connection,
{
    #[profiling::function]
    fn with_dmabuf(connection: C, window: &Window, dmabuf: &Dmabuf) -> Result<PixmapWrapper<C>, X11Error> {
        if dmabuf.format().code != window.format() {
            return Err(PresentError::IncorrectFormat(window.format()).into());
        }

        let mut fds = Vec::new();

        for handle in dmabuf.handles() {
            let fd = rustix::io::fcntl_dupfd_cloexec(
                handle, 3,
            )
            .map_err(|e| PresentError::DupFailed(e.to_string()))?;

            fds.push(fd);
        }

        let xid = if window.0.extensions.dri3 >= Some((1, 2)) {
            if dmabuf.num_planes() > 4 {
                return Err(PresentError::TooManyPlanes.into());
            }

            let xid = connection.generate_id()?;
            let mut strides = dmabuf.strides();
            let mut offsets = dmabuf.offsets();

            connection.dri3_pixmap_from_buffers(
                xid,
                window.id(),
                dmabuf.width() as u16,
                dmabuf.height() as u16,
                strides.next().unwrap(),
                offsets.next().unwrap(),
                strides.next().unwrap_or(x11rb::NONE),
                offsets.next().unwrap_or(x11rb::NONE),
                strides.next().unwrap_or(x11rb::NONE),
                offsets.next().unwrap_or(x11rb::NONE),
                strides.next().unwrap_or(x11rb::NONE),
                offsets.next().unwrap_or(x11rb::NONE),
                window.depth(),
                match window.format() {
                    DrmFourcc::Argb8888 => 32,
                    DrmFourcc::Xrgb8888 => 24,
                    _ => unreachable!(),
                },
                dmabuf.format().modifier.into(),
                fds,
            )?;

            xid
        } else {
            if dmabuf.num_planes() != 1 {
                return Err(PresentError::TooManyPlanes.into());
            }

            let xid = connection.generate_id()?;
            let mut strides = dmabuf.strides();
            let stride = strides.next().unwrap();

            connection.dri3_pixmap_from_buffer(
                xid,
                window.id(),
                dmabuf.height() * stride,
                dmabuf.width() as u16,
                dmabuf.height() as u16,
                stride as u16,
                window.depth(),
                match window.format() {
                    DrmFourcc::Argb8888 => 32,
                    DrmFourcc::Xrgb8888 => 24,
                    _ => unreachable!(),
                },
                fds.remove(0),
            )?;

            xid
        };

        Ok(PixmapWrapper::for_pixmap(connection, xid))
    }

    #[profiling::function]
    fn present(self, connection: C, window: &Window) -> Result<u32, X11Error> {
        let next_serial = window.0.next_serial.fetch_add(1, Ordering::SeqCst);
        let msc = window.0.last_msc.load(Ordering::SeqCst) + 1;

        const OPTIONS: present::Option = present::Option::NONE;

        connection.present_pixmap(
            window.id(),
            self.pixmap(),
            next_serial,
            x11rb::NONE,
            x11rb::NONE,
            0,
            0,
            x11rb::NONE,
            x11rb::NONE,
            x11rb::NONE,
            OPTIONS.into(),
            msc,
            0,
            0,
            &[],
        )?;

        Ok(self.pixmap())
    }
}
