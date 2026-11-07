//! The calls into libpipewire, found with dlopen, and the C structs they pass.

use std::ffi::{c_char, c_int, c_void, CString};
use std::sync::OnceLock;

use super::callbacks::{on_add_buffer, on_param, on_process, on_remove_buffer, on_state};

#[repr(C)]
pub(super) struct SpaList {
    next: *mut SpaList,
    prev: *mut SpaList,
}

#[repr(C)]
pub(super) struct SpaCallbacks {
    pub(super) funcs: *const c_void,
    pub(super) data: *mut c_void,
}

#[repr(C)]
pub(super) struct SpaHook {
    link: SpaList,
    cb: SpaCallbacks,
    removed: *const c_void,
    private: *mut c_void,
}

#[repr(C)]
pub(super) struct SpaInterface {
    kind: *const c_char,
    version: u32,
    pub(super) cb: SpaCallbacks,
}

#[repr(C)]
pub(super) struct LoopControlMethods {
    version: u32,
    pub(super) get_fd: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    add_hook: *const c_void,
    pub(super) enter: Option<unsafe extern "C" fn(*mut c_void)>,
    pub(super) leave: Option<unsafe extern "C" fn(*mut c_void)>,
    pub(super) iterate: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    check: *const c_void,
}

#[repr(C)]
pub(super) struct PwLoop {
    system: *mut c_void,
    spa_loop: *mut c_void,
    pub(super) control: *mut SpaInterface,
    utils: *mut c_void,
    name: *const c_char,
}

#[repr(C)]
pub(super) struct SpaDictItem {
    key: *const c_char,
    value: *const c_char,
}

#[repr(C)]
pub(super) struct SpaDict {
    flags: u32,
    n_items: u32,
    items: *const SpaDictItem,
}

#[repr(C)]
pub(super) struct PwBuffer {
    pub(super) buffer: *mut SpaBuffer,
    pub(super) user_data: *mut c_void,
    size: u64,
    requested: u64,
    time: u64,
}

#[repr(C)]
pub(super) struct SpaBuffer {
    pub(super) n_metas: u32,
    pub(super) n_datas: u32,
    pub(super) metas: *mut SpaMeta,
    pub(super) datas: *mut SpaData,
}

#[repr(C)]
pub(super) struct SpaMeta {
    pub(super) kind: u32,
    pub(super) size: u32,
    pub(super) data: *mut c_void,
}

#[repr(C)]
pub(super) struct SpaData {
    pub(super) kind: u32,
    pub(super) flags: u32,
    pub(super) fd: i64,
    pub(super) mapoffset: u32,
    pub(super) maxsize: u32,
    pub(super) data: *mut c_void,
    pub(super) chunk: *mut SpaChunk,
}

#[repr(C)]
pub(super) struct SpaChunk {
    pub(super) offset: u32,
    pub(super) size: u32,
    pub(super) stride: i32,
    pub(super) flags: i32,
}

#[repr(C)]
pub(super) struct MetaHeader {
    pub(super) flags: u32,
    pub(super) offset: u32,
    pub(super) pts: i64,
    pub(super) dts_offset: i64,
    pub(super) seq: u64,
}

#[repr(C)]
pub(super) struct MetaRegion {
    pub(super) x: i32,
    pub(super) y: i32,
    pub(super) w: u32,
    pub(super) h: u32,
}

pub(super) type Data = *mut c_void;

#[repr(C)]
pub(super) struct StreamEvents {
    pub(super) version: u32,
    pub(super) destroy: Option<unsafe extern "C" fn(Data)>,
    pub(super) state_changed: Option<unsafe extern "C" fn(Data, c_int, c_int, *const c_char)>,
    pub(super) control_info: *const c_void,
    pub(super) io_changed: *const c_void,
    pub(super) param_changed: Option<unsafe extern "C" fn(Data, u32, *const u8)>,
    pub(super) add_buffer: Option<unsafe extern "C" fn(Data, *mut PwBuffer)>,
    pub(super) remove_buffer: Option<unsafe extern "C" fn(Data, *mut PwBuffer)>,
    pub(super) process: Option<unsafe extern "C" fn(Data)>,
    pub(super) drained: *const c_void,
    pub(super) command: *const c_void,
    pub(super) trigger_done: *const c_void,
}

// SAFETY: the table holds only function addresses and nulls, never changed,
// so sharing it between threads is harmless; PipeWire only reads it.
unsafe impl Sync for StreamEvents {}

pub(super) static EVENTS: StreamEvents = StreamEvents {
    version: 2,
    destroy: None,
    state_changed: Some(on_state),
    control_info: std::ptr::null(),
    io_changed: std::ptr::null(),
    param_changed: Some(on_param),
    add_buffer: Some(on_add_buffer),
    remove_buffer: Some(on_remove_buffer),
    process: Some(on_process),
    drained: std::ptr::null(),
    command: std::ptr::null(),
    trigger_done: std::ptr::null(),
};

pub(super) struct Lib {
    pub(super) init: unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char),
    pub(super) loop_new: unsafe extern "C" fn(*const SpaDict) -> *mut PwLoop,
    pub(super) loop_destroy: unsafe extern "C" fn(*mut PwLoop),
    pub(super) context_new: unsafe extern "C" fn(*mut PwLoop, *mut c_void, usize) -> *mut c_void,
    pub(super) context_destroy: unsafe extern "C" fn(*mut c_void),
    pub(super) context_connect: unsafe extern "C" fn(*mut c_void, *mut c_void, usize) -> *mut c_void,
    pub(super) core_disconnect: unsafe extern "C" fn(*mut c_void) -> c_int,
    properties_new_dict: unsafe extern "C" fn(*const SpaDict) -> *mut c_void,
    pub(super) stream_new: unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void) -> *mut c_void,
    pub(super) stream_destroy: unsafe extern "C" fn(*mut c_void),
    pub(super) stream_add_listener: unsafe extern "C" fn(*mut c_void, *mut SpaHook, *const StreamEvents, Data),
    pub(super) stream_connect: unsafe extern "C" fn(*mut c_void, c_int, u32, c_int, *const *const u8, u32) -> c_int,
    pub(super) stream_get_node_id: unsafe extern "C" fn(*mut c_void) -> u32,
    pub(super) stream_update_params: unsafe extern "C" fn(*mut c_void, *const *const u8, u32) -> c_int,
    pub(super) stream_dequeue_buffer: unsafe extern "C" fn(*mut c_void) -> *mut PwBuffer,
    pub(super) stream_queue_buffer: unsafe extern "C" fn(*mut c_void, *mut PwBuffer) -> c_int,
    pub(super) stream_trigger_process: unsafe extern "C" fn(*mut c_void) -> c_int,
    pub(super) stream_return_buffer: unsafe extern "C" fn(*mut c_void, *mut PwBuffer) -> c_int,
    pub(super) stream_is_driving: unsafe extern "C" fn(*mut c_void) -> bool,
}

// SAFETY: plain function addresses into a library that is never closed.
unsafe impl Send for Lib {}
// SAFETY: as above; reading an address from several threads is harmless.
unsafe impl Sync for Lib {}

pub(super) fn lib() -> Option<&'static Lib> {
    static LIB: OnceLock<Option<Lib>> = OnceLock::new();
    LIB.get_or_init(|| {
        // SAFETY: dlopen with a constant, NUL-terminated name; the handle is
        // kept for the life of the program and never closed, so every
        // address taken from it stays good.
        let handle = unsafe { libc::dlopen(c"libpipewire-0.3.so.0".as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL) };
        if handle.is_null() {
            return None;
        }
        macro_rules! sym {
            ($name:literal) => {{
                // SAFETY: looks a NUL-terminated name up in the open handle.
                let p = unsafe { libc::dlsym(handle, concat!($name, "\0").as_ptr() as *const c_char) };
                if p.is_null() {
                    return None;
                }
                // SAFETY: the symbol is the PipeWire function of that name,
                // whose C signature the field it goes into copies.
                unsafe { super::super::function(p) }
            }};
        }
        Some(Lib {
            init: sym!("pw_init"),
            loop_new: sym!("pw_loop_new"),
            loop_destroy: sym!("pw_loop_destroy"),
            context_new: sym!("pw_context_new"),
            context_destroy: sym!("pw_context_destroy"),
            context_connect: sym!("pw_context_connect"),
            core_disconnect: sym!("pw_core_disconnect"),
            properties_new_dict: sym!("pw_properties_new_dict"),
            stream_new: sym!("pw_stream_new"),
            stream_destroy: sym!("pw_stream_destroy"),
            stream_add_listener: sym!("pw_stream_add_listener"),
            stream_connect: sym!("pw_stream_connect"),
            stream_get_node_id: sym!("pw_stream_get_node_id"),
            stream_update_params: sym!("pw_stream_update_params"),
            stream_dequeue_buffer: sym!("pw_stream_dequeue_buffer"),
            stream_queue_buffer: sym!("pw_stream_queue_buffer"),
            stream_trigger_process: sym!("pw_stream_trigger_process"),
            stream_return_buffer: sym!("pw_stream_return_buffer"),
            stream_is_driving: sym!("pw_stream_is_driving"),
        })
    })
    .as_ref()
}

pub(super) fn properties(lib: &Lib, pairs: &[(&str, &str)]) -> *mut c_void {
    let owned: Vec<(CString, CString)> = pairs
        .iter()
        .filter_map(|(k, v)| Some((CString::new(*k).ok()?, CString::new(*v).ok()?)))
        .collect();
    let items: Vec<SpaDictItem> = owned.iter().map(|(k, v)| SpaDictItem { key: k.as_ptr(), value: v.as_ptr() }).collect();
    let dict = SpaDict { flags: 0, n_items: items.len() as u32, items: items.as_ptr() };
    // SAFETY: the dictionary and the strings it points at live until this
    // returns, and PipeWire copies them into the new property list.
    unsafe { (lib.properties_new_dict)(&dict) }
}
