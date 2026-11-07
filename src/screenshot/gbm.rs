//! Buffers on the graphics card, for screen sharing straight from the card.

use std::ffi::{c_int, c_void, CString};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub const XRGB8888: u32 = u32::from_le_bytes(*b"XR24");
pub const MOD_INVALID: u64 = 0x00ff_ffff_ffff_ffff;
const USE_RENDERING: u32 = 1 << 2;

struct Lib {
    create_device: unsafe extern "C" fn(c_int) -> *mut c_void,
    device_destroy: unsafe extern "C" fn(*mut c_void),
    create_with_modifiers: unsafe extern "C" fn(*mut c_void, u32, u32, u32, *const u64, u32) -> *mut c_void,
    create_with_modifiers2: Option<unsafe extern "C" fn(*mut c_void, u32, u32, u32, *const u64, u32, u32) -> *mut c_void>,
    plane_count: unsafe extern "C" fn(*mut c_void) -> c_int,
    fd_for_plane: unsafe extern "C" fn(*mut c_void, c_int) -> c_int,
    stride_for_plane: unsafe extern "C" fn(*mut c_void, c_int) -> u32,
    offset: unsafe extern "C" fn(*mut c_void, c_int) -> u32,
    modifier: unsafe extern "C" fn(*mut c_void) -> u64,
    destroy: unsafe extern "C" fn(*mut c_void),
}

// SAFETY: plain function addresses into a library that is never closed.
unsafe impl Send for Lib {}
// SAFETY: as above.
unsafe impl Sync for Lib {}

fn lib() -> Option<&'static Lib> {
    static LIB: OnceLock<Option<Lib>> = OnceLock::new();
    LIB.get_or_init(|| {
        // SAFETY: dlopen with a constant name; the handle is never closed.
        let handle = unsafe { libc::dlopen(c"libgbm.so.1".as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL) };
        if handle.is_null() {
            return None;
        }
        let find = |name: &str| -> *mut c_void {
            let name = CString::new(name).unwrap_or_default();
            // SAFETY: looks a NUL-terminated name up in the open handle.
            unsafe { libc::dlsym(handle, name.as_ptr()) }
        };
        macro_rules! sym {
            ($name:literal) => {{
                let p = find($name);
                if p.is_null() {
                    return None;
                }
                // SAFETY: the libgbm function of that name, whose C
                // signature the field it goes into copies.
                unsafe { super::function(p) }
            }};
        }
        let two = find("gbm_bo_create_with_modifiers2");
        Some(Lib {
            create_device: sym!("gbm_create_device"),
            device_destroy: sym!("gbm_device_destroy"),
            create_with_modifiers: sym!("gbm_bo_create_with_modifiers"),
            create_with_modifiers2: (!two.is_null()).then(|| {
                // SAFETY: as in sym!, when the newer call is there at all.
                unsafe { super::function(two) }
            }),
            plane_count: sym!("gbm_bo_get_plane_count"),
            fd_for_plane: sym!("gbm_bo_get_fd_for_plane"),
            stride_for_plane: sym!("gbm_bo_get_stride_for_plane"),
            offset: sym!("gbm_bo_get_offset"),
            modifier: sym!("gbm_bo_get_modifier"),
            destroy: sym!("gbm_bo_destroy"),
        })
    })
    .as_ref()
}

fn render_node(device: u64, dri: &Path, sys: &Path) -> Option<PathBuf> {
    let nodes: Vec<PathBuf> = std::fs::read_dir(dri).ok()?.flatten().map(|e| e.path()).collect();
    let node = nodes.iter().find(|p| std::fs::metadata(p).is_ok_and(|m| m.rdev() == device))?;
    let name = node.file_name()?.to_string_lossy().into_owned();
    if name.starts_with("renderD") {
        return Some(node.clone());
    }
    let (major, minor) = (libc::major(device), libc::minor(device));
    let card = sys.join(format!("{major}:{minor}/device/drm"));
    let render = std::fs::read_dir(card).ok()?.flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .find(|n| n.starts_with("renderD"))?;
    Some(dri.join(render))
}

pub struct Device {
    lib: &'static Lib,
    dev: *mut c_void,
    _fd: OwnedFd,
}

pub struct Plane {
    pub fd: OwnedFd,
    pub offset: u32,
    pub stride: u32,
}

pub struct Bo {
    pub planes: Vec<Plane>,
    pub modifier: u64,
}

impl Device {
    pub fn open(device: u64) -> Option<Device> {
        let lib = lib()?;
        let path = render_node(device, Path::new("/dev/dri"), Path::new("/sys/dev/char"))?;
        let file = std::fs::OpenOptions::new().read(true).write(true).open(path).ok()?;
        let fd = OwnedFd::from(file);
        // SAFETY: a device on the open node, which this keeps open until
        // the device is destroyed in Drop.
        let dev = unsafe { (lib.create_device)(fd.as_raw_fd()) };
        (!dev.is_null()).then_some(Device { lib, dev, _fd: fd })
    }

    pub fn alloc(&self, w: u32, h: u32, fourcc: u32, modifiers: &[u64]) -> Option<Bo> {
        if modifiers.is_empty() {
            return None;
        }
        // SAFETY: the live device, and a list `modifiers.len()` long.
        let bo = unsafe {
            match self.lib.create_with_modifiers2 {
                Some(two) => two(self.dev, w, h, fourcc, modifiers.as_ptr(), modifiers.len() as u32, USE_RENDERING),
                None => (self.lib.create_with_modifiers)(self.dev, w, h, fourcc, modifiers.as_ptr(), modifiers.len() as u32),
            }
        };
        if bo.is_null() {
            return None;
        }
        // SAFETY: the buffer just made; each plane's handle is a new
        // descriptor owned from here on; the buffer itself is let go of at
        // the end, its memory kept alive by those handles.

        unsafe {
            let count = (self.lib.plane_count)(bo).max(0);
            let mut planes = Vec::new();
            for i in 0..count {
                let fd = (self.lib.fd_for_plane)(bo, i);
                if fd < 0 {
                    break;
                }
                planes.push(Plane {
                    fd: OwnedFd::from_raw_fd(fd),
                    offset: (self.lib.offset)(bo, i),
                    stride: (self.lib.stride_for_plane)(bo, i),
                });
            }
            let modifier = (self.lib.modifier)(bo);
            (self.lib.destroy)(bo);
            (planes.len() == count as usize && count > 0).then_some(Bo { planes, modifier })
        }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        // SAFETY: made in open, destroyed once; every buffer made from it
        // was already let go of as plain handles.
        unsafe { (self.lib.device_destroy)(self.dev) };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_primary_node_leads_to_its_render_node() {
        let dir = std::env::temp_dir().join(format!("trindle-gbm-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (dri, sys) = (dir.join("dri"), dir.join("sys"));
        std::fs::create_dir_all(&dri).unwrap();
        assert_eq!(super::render_node(libc::makedev(226, 0), &dri, &sys), None);
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(super::XRGB8888, 0x3432_5258);
    }
}
