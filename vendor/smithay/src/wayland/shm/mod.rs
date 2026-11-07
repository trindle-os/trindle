//! SHM handling helpers.

use std::{
    any::Any,
    collections::HashSet,
    sync::{Arc, Mutex},
};

use wayland_server::{
    backend::GlobalId,
    protocol::{
        wl_buffer,
        wl_shm::{self, WlShm},
        wl_shm_pool::WlShmPool,
    },
    Dispatch, DisplayHandle, GlobalDispatch, Resource, WEnum,
};

mod handlers;
mod pool;

use crate::{
    backend::allocator::format::get_bpp,
    utils::{hook::Hook, HookId, UnmanagedResource},
};

use self::pool::Pool;

use super::buffer::BufferHandler;

#[derive(Debug)]
pub struct ShmState {
    formats: HashSet<wl_shm::Format>,
    shm: GlobalId,
}

impl ShmState {
    pub fn new<D>(display: &DisplayHandle, formats: impl IntoIterator<Item = wl_shm::Format>) -> ShmState
    where
        D: GlobalDispatch<WlShm, ()>
            + Dispatch<WlShm, ()>
            + Dispatch<WlShmPool, ShmPoolUserData>
            + BufferHandler
            + ShmHandler
            + 'static,
    {
        let mut formats = formats.into_iter().collect::<HashSet<_>>();

        formats.insert(wl_shm::Format::Argb8888);
        formats.insert(wl_shm::Format::Xrgb8888);

        let shm = display.create_global::<D, WlShm, _>(2, ());

        ShmState { formats, shm }
    }

    pub fn global(&self) -> GlobalId {
        self.shm.clone()
    }

    pub fn update_formats(&mut self, formats: impl IntoIterator<Item = wl_shm::Format>) {
        self.formats = formats.into_iter().collect::<HashSet<_>>();
        self.formats.insert(wl_shm::Format::Argb8888);
        self.formats.insert(wl_shm::Format::Xrgb8888);
    }
}

pub trait ShmHandler {
    fn shm_state(&self) -> &ShmState;
}

#[derive(Debug, thiserror::Error)]
pub enum BufferAccessError {
    #[error("non-SHM buffer")]
    NotManaged,

    #[error("invalid client buffer")]
    BadMap,

    #[error("Client has not indicated read permission for the buffer")]
    NotReadable,

    #[error("Client has not indicated write permission for the buffer")]
    NotWritable,
}

impl From<UnmanagedResource> for BufferAccessError {
    #[inline]
    fn from(_: UnmanagedResource) -> Self {
        Self::NotManaged
    }
}

pub fn with_buffer_contents<F, T>(buffer: &wl_buffer::WlBuffer, f: F) -> Result<T, BufferAccessError>
where
    F: FnOnce(*const u8, usize, BufferData) -> T,
{
    let data = buffer
        .data::<ShmBufferUserData>()
        .ok_or(BufferAccessError::NotManaged)?;

    match data.pool.with_data(|ptr, len| f(ptr, len, data.data)) {
        Ok(t) => Ok(t),
        Err(()) => {
            buffer.post_error(wl_shm::Error::InvalidFd, "Bad pool size.");
            Err(BufferAccessError::BadMap)
        }
    }
}

pub fn with_buffer_contents_mut<F, T>(buffer: &wl_buffer::WlBuffer, f: F) -> Result<T, BufferAccessError>
where
    F: FnOnce(*mut u8, usize, BufferData) -> T,
{
    let data = buffer
        .data::<ShmBufferUserData>()
        .ok_or(BufferAccessError::NotManaged)?;

    match data.pool.with_data_mut(|ptr, len| f(ptr, len, data.data)) {
        Ok(t) => Ok(t),
        Err(()) => {
            buffer.post_error(wl_shm::Error::InvalidFd, "Bad pool size.");
            Err(BufferAccessError::BadMap)
        }
    }
}

pub fn wl_bytes_per_pixel(format: WEnum<wl_shm::Format>) -> i32 {
    match format {
        WEnum::Value(f) => {
            shm_format_to_fourcc(f).map_or(0, |fourcc| get_bpp(fourcc).map_or(0, |bpp| bpp / 8))
        }
        WEnum::Unknown(_) => 0,
    }
    .try_into()
    .unwrap()
}

macro_rules! shm_format_table {
    (
        $(
            $fourcc: ident => $shm: ident
        ),* $(,)?
    ) => {
        pub const fn fourcc_to_shm_format(value: $crate::backend::allocator::Fourcc) -> Option<$crate::reexports::wayland_server::protocol::wl_shm::Format> {
            match value {
                $(
                    $crate::backend::allocator::Fourcc::$fourcc => Some($crate::reexports::wayland_server::protocol::wl_shm::Format::$shm),
                )*
                _ => None,
            }
        }

        pub const fn shm_format_to_fourcc(value: $crate::reexports::wayland_server::protocol::wl_shm::Format) -> Option<$crate::backend::allocator::Fourcc> {
            match value {
                $(
                    $crate::reexports::wayland_server::protocol::wl_shm::Format::$shm => Some($crate::backend::allocator::Fourcc::$fourcc),
                )*
                _ => None,
            }
        }
    }
}

shm_format_table! {
    Argb8888 => Argb8888,
    Xrgb8888 => Xrgb8888,
    C8 => C8,
    Rgb332 => Rgb332,
    Bgr233 => Bgr233,
    Xrgb4444 => Xrgb4444,
    Xbgr4444 => Xbgr4444,
    Rgbx4444 => Rgbx4444,
    Bgrx4444 => Bgrx4444,
    Argb4444 => Argb4444,
    Abgr4444 => Abgr4444,
    Rgba4444 => Rgba4444,
    Bgra4444 => Bgra4444,
    Xrgb1555 => Xrgb1555,
    Xbgr1555 => Xbgr1555,
    Rgbx5551 => Rgbx5551,
    Bgrx5551 => Bgrx5551,
    Argb1555 => Argb1555,
    Abgr1555 => Abgr1555,
    Rgba5551 => Rgba5551,
    Bgra5551 => Bgra5551,
    Rgb565 => Rgb565,
    Bgr565 => Bgr565,
    Rgb888 => Rgb888,
    Bgr888 => Bgr888,
    Xbgr8888 => Xbgr8888,
    Rgbx8888 => Rgbx8888,
    Bgrx8888 => Bgrx8888,
    Abgr8888 => Abgr8888,
    Rgba8888 => Rgba8888,
    Bgra8888 => Bgra8888,
    Xrgb2101010 => Xrgb2101010,
    Xbgr2101010 => Xbgr2101010,
    Rgbx1010102 => Rgbx1010102,
    Bgrx1010102 => Bgrx1010102,
    Argb2101010 => Argb2101010,
    Abgr2101010 => Abgr2101010,
    Rgba1010102 => Rgba1010102,
    Bgra1010102 => Bgra1010102,
    Yuyv => Yuyv,
    Yvyu => Yvyu,
    Uyvy => Uyvy,
    Vyuy => Vyuy,
    Ayuv => Ayuv,
    Nv12 => Nv12,
    Nv21 => Nv21,
    Nv16 => Nv16,
    Nv61 => Nv61,
    Yuv410 => Yuv410,
    Yvu410 => Yvu410,
    Yuv411 => Yuv411,
    Yvu411 => Yvu411,
    Yuv420 => Yuv420,
    Yvu420 => Yvu420,
    Yuv422 => Yuv422,
    Yvu422 => Yvu422,
    Yuv444 => Yuv444,
    Yvu444 => Yvu444,
    R8 => R8,
    R16 => R16,
    Rg88 => Rg88,
    Gr88 => Gr88,
    Rg1616 => Rg1616,
    Gr1616 => Gr1616,
    Xrgb16161616f => Xrgb16161616f,
    Xbgr16161616f => Xbgr16161616f,
    Argb16161616f => Argb16161616f,
    Abgr16161616f => Abgr16161616f,
    Xyuv8888 => Xyuv8888,
    Vuy888 => Vuy888,
    Vuy101010 => Vuy101010,
    Y210 => Y210,
    Y212 => Y212,
    Y216 => Y216,
    Y410 => Y410,
    Y412 => Y412,
    Y416 => Y416,
    Xvyu2101010 => Xvyu2101010,
    Xvyu12_16161616 => Xvyu1216161616,
    Xvyu16161616 => Xvyu16161616,
    Y0l0 => Y0l0,
    X0l0 => X0l0,
    Y0l2 => Y0l2,
    X0l2 => X0l2,
    Yuv420_8bit => Yuv4208bit,
    Yuv420_10bit => Yuv42010bit,
    Xrgb8888_a8 => Xrgb8888A8,
    Xbgr8888_a8 => Xbgr8888A8,
    Rgbx8888_a8 => Rgbx8888A8,
    Bgrx8888_a8 => Bgrx8888A8,
    Rgb888_a8 => Rgb888A8,
    Bgr888_a8 => Bgr888A8,
    Rgb565_a8 => Rgb565A8,
    Bgr565_a8 => Bgr565A8,
    Nv24 => Nv24,
    Nv42 => Nv42,
    P210 => P210,
    P010 => P010,
    P012 => P012,
    P016 => P016,
    Axbxgxrx106106106106 => Axbxgxrx106106106106,
    Nv15 => Nv15,
    Q410 => Q410,
    Q401 => Q401,

}

#[derive(Copy, Clone, Debug)]
pub struct BufferData {
    pub offset: i32,
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub format: wl_shm::Format,
}

#[derive(Debug)]
pub struct ShmPoolUserData {
    inner: Arc<Pool>,
}

type DestructionHook = dyn Fn(&mut dyn Any, &wl_buffer::WlBuffer) + Send + Sync;

#[derive(Debug)]
pub struct ShmBufferUserData {
    pub(crate) pool: Arc<Pool>,
    pub(crate) data: BufferData,
    destruction_hooks: Mutex<Vec<Hook<DestructionHook>>>,
}

impl ShmBufferUserData {
    pub(crate) fn add_destruction_hook(
        &self,
        hook: impl Fn(&mut dyn Any, &wl_buffer::WlBuffer) + Send + Sync + 'static,
    ) -> HookId {
        let hook: Hook<DestructionHook> = Hook::new(Arc::new(hook));
        let id = hook.id.clone();
        self.destruction_hooks.lock().unwrap().push(hook);
        id
    }

    pub(crate) fn remove_destruction_hook(&self, hook_id: HookId) {
        let mut guard = self.destruction_hooks.lock().unwrap();
        if let Some(id) = guard.iter().position(|hook| hook.id != hook_id) {
            guard.remove(id);
        }
    }
}

#[allow(missing_docs)]
#[macro_export]
macro_rules! delegate_shm {
    ($(@<$( $lt:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+>)? $ty: ty) => {
        $crate::reexports::wayland_server::delegate_global_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_shm::WlShm: ()
        ] => $crate::wayland::shm::ShmState);

        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_shm::WlShm: ()
        ] => $crate::wayland::shm::ShmState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_shm_pool::WlShmPool: $crate::wayland::shm::ShmPoolUserData
        ] => $crate::wayland::shm::ShmState);
        $crate::reexports::wayland_server::delegate_dispatch!($(@< $( $lt $( : $clt $(+ $dlt )* )? ),+ >)? $ty: [
            $crate::reexports::wayland_server::protocol::wl_buffer::WlBuffer: $crate::wayland::shm::ShmBufferUserData
        ] => $crate::wayland::shm::ShmState);
    };
}
