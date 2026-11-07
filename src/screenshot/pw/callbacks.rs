//! What PipeWire calls back with.

use std::cell::RefCell;
use std::ffi::{c_char, c_int, c_void, CStr};
use std::os::fd::AsRawFd;

use crate::common::shm::Memory;

use super::super::gbm::Bo;
use super::format::{parse_format, tries, VideoFormat};
use super::pod::Pod;
use super::native::{Data, PwBuffer};
use super::pod::{buffers, buffers_card, metas, DATA_DMABUF, DATA_FLAG_MAPPABLE, DATA_FLAG_READWRITE, DATA_MEMFD, PARAM_FORMAT};
use super::stream::{state_of, Event, Inner, Slot, Store};

unsafe fn inner<'a>(data: Data) -> &'a Inner {
    // SAFETY: as the caller promises.
    unsafe { &*(data as *const Inner) }
}

pub(super) unsafe extern "C" fn on_state(data: Data, _old: c_int, state: c_int, error: *const c_char) {
    // SAFETY: PipeWire passes back the data given with the listener, and an
    // error that is null or a NUL-terminated string.
    let (inner, error) = unsafe {
        (inner(data), (!error.is_null()).then(|| CStr::from_ptr(error).to_string_lossy().into_owned()))
    };
    if let Ok(mut events) = inner.events.try_borrow_mut() {
        events.push(Event::State(state_of(state), error));
    }
}

pub(super) unsafe extern "C" fn on_param(data: Data, id: u32, param: *const u8) {
    // SAFETY: as in on_state.
    let inner = unsafe { inner(data) };
    if id != PARAM_FORMAT {
        return;
    }
    let parsed = if param.is_null() {
        None
    } else {
        // SAFETY: a spa_pod: its first word is the body's size, and the
        // whole POD is that size plus the eight-byte header.
        let bytes = unsafe {
            let size = std::ptr::read_unaligned(param as *const u32) as usize;
            std::slice::from_raw_parts(param, 8 + size)
        };
        parse_format(bytes)
    };
    let (format, buffers_pod) = settle(inner, &parsed);
    inner.format.set(format);
    if let Some(buffers) = buffers_pod {
        let mut pods = vec![buffers];
        pods.extend(metas(inner.pointer));
        let params: Vec<*const u8> = pods.iter().map(|p| p.bytes().as_ptr()).collect();
        // SAFETY: the live stream, with well-formed PODs alive for the call.
        unsafe { (inner.lib.stream_update_params)(inner.raw.get(), params.as_ptr(), params.len() as u32) };
    }
    if let Ok(mut events) = inner.events.try_borrow_mut() {
        events.push(Event::Format(format));
    }
}

fn settle(inner: &Inner, parsed: &Option<(VideoFormat, Option<Vec<u64>>)>) -> (Option<VideoFormat>, Option<Pod>) {
    let mut format = parsed.as_ref().map(|(f, _)| *f);
    let mut buffers_pod = None;
    match (parsed, format) {
        (Some((f, Some(open))), _) if open.len() == 1 && inner.fixed.get().map(|x| x.0) == open.first().copied() => {
            buffers_pod = Some(buffers_card(inner.fixed.get().map_or(1, |x| x.1)));
            format = Some(VideoFormat { modifier: open.first().copied(), ..*f });
        }
        (Some((f, Some(open))), _) => {
            let made = inner.card.borrow().as_ref().and_then(|c| {
                tries(open).iter().find_map(|t| c.device.alloc(f.width, f.height, super::super::gbm::XRGB8888, t))
            });
            match made {
                Some(bo) => inner.fixed.set(Some((bo.modifier, bo.planes.len() as u32))),
                None => {
                    inner.card.borrow_mut().take();
                }
            }
            inner.offer();
            format = None;
        }
        (_, Some(f)) if f.modifier.is_some() => {
            let planes = match inner.fixed.get() {
                Some((m, planes)) if Some(m) == f.modifier => Some(planes),
                _ => inner.card.borrow().as_ref()
                    .and_then(|c| c.device.alloc(f.width, f.height, super::super::gbm::XRGB8888, &[f.modifier.unwrap_or(0)]))
                    .map(|bo| bo.planes.len() as u32),
            };
            match planes {
                Some(planes) => buffers_pod = Some(buffers_card(planes)),
                None => {
                    inner.card.borrow_mut().take();
                    inner.offer();
                    format = None;
                }
            }
        }
        (_, Some(f)) => buffers_pod = Some(buffers(&f)),
        _ => {}
    }
    (format, buffers_pod)
}

pub(super) unsafe extern "C" fn on_add_buffer(data: Data, buffer: *mut PwBuffer) {
    // SAFETY: as in on_state.
    let inner = unsafe { inner(data) };
    let Some(format) = inner.format.get() else { return };
    // SAFETY: PipeWire hands over a buffer it made from our Buffers param,
    // its first block's type a mask of the kinds of memory allowed.
    let (allowed, blocks) = unsafe {
        let spa = &*(*buffer).buffer;
        if spa.n_datas < 1 {
            return;
        }
        ((*spa.datas).kind, spa.n_datas as usize)
    };
    let slot = match backing(inner, &format, allowed, blocks) {
        Backing::Card(bo) => on_card(buffer, bo, format),
        Backing::Shared => {
            let Some(slot) = in_memory(buffer, format) else { return };
            slot
        }
        Backing::Refused => return,
    };
    // SAFETY: the buffer is PipeWire's until on_remove_buffer, which frees
    // this box again.
    unsafe { (*buffer).user_data = Box::into_raw(slot) as *mut c_void };
    if let Ok(mut live) = inner.live.try_borrow_mut() {
        live.insert(buffer as usize);
    }
}

enum Backing {
    Card(Bo),
    Shared,
    Refused,
}

fn backing(inner: &Inner, format: &VideoFormat, allowed: u32, blocks: usize) -> Backing {
    match format.modifier {
        Some(m) if allowed & (1 << DATA_DMABUF) != 0 => {
            let made = inner.card.borrow().as_ref()
                .and_then(|c| c.device.alloc(format.width, format.height, super::super::gbm::XRGB8888, &[m]));
            match made {
                Some(bo) if bo.planes.len() == blocks => Backing::Card(bo),
                _ => {
                    inner.card.borrow_mut().take();
                    inner.offer();
                    Backing::Refused
                }
            }
        }
        _ => Backing::Shared,
    }
}

fn on_card(buffer: *mut PwBuffer, bo: Bo, format: VideoFormat) -> Box<Slot> {
    // SAFETY: PipeWire's buffer, with as many blocks as the card's planes.
    unsafe {
        let spa = &mut *(*buffer).buffer;
        for (i, plane) in bo.planes.iter().enumerate() {
            let d = &mut *spa.datas.add(i);
            d.kind = DATA_DMABUF;
            d.flags = DATA_FLAG_READWRITE;
            d.fd = plane.fd.as_raw_fd() as i64;
            d.mapoffset = 0;
            d.maxsize = plane.stride * format.height;
            d.data = std::ptr::null_mut();
            let chunk = &mut *d.chunk;
            chunk.offset = plane.offset;
            chunk.stride = plane.stride as i32;
            chunk.size = plane.stride * format.height;
        }
    }
    let stride = bo.planes[0].stride;
    Box::new(Slot { store: Store::Card(bo), stride, format, attached: RefCell::new(None) })
}

fn in_memory(buffer: *mut PwBuffer, format: VideoFormat) -> Option<Box<Slot>> {
    let stride = format.width * 4;
    let memory = Memory::new(c"trindle-screencast", (stride * format.height) as usize)?;
    // SAFETY: PipeWire's buffer, with at least one block.
    unsafe {
        let d = &mut *(*(*buffer).buffer).datas;
        d.kind = DATA_MEMFD;
        d.flags = DATA_FLAG_READWRITE | DATA_FLAG_MAPPABLE;
        d.fd = memory.fd().as_raw_fd() as i64;
        d.mapoffset = 0;
        d.maxsize = memory.len() as u32;
        d.data = memory.bytes().as_ptr() as *mut c_void;
    }
    Some(Box::new(Slot { store: Store::Shared(memory), stride, format, attached: RefCell::new(None) }))
}

pub(super) unsafe extern "C" fn on_remove_buffer(data: Data, buffer: *mut PwBuffer) {
    // SAFETY: as in on_state.
    let inner = unsafe { inner(data) };
    inner.round.set(inner.round.get() + 1);
    if let Ok(mut live) = inner.live.try_borrow_mut() {
        live.remove(&(buffer as usize));
    }
    // SAFETY: user_data is null or the box made in on_add_buffer, freed once
    // here; the blocks' pointers into it are cleared first.
    unsafe {
        let slot = (*buffer).user_data as *mut Slot;
        (*buffer).user_data = std::ptr::null_mut();
        let spa = &mut *(*buffer).buffer;
        for i in 0..spa.n_datas as usize {
            let d = &mut *spa.datas.add(i);
            d.data = std::ptr::null_mut();
            d.fd = -1;
        }
        if !slot.is_null() {
            drop(Box::from_raw(slot));
        }
    }
}

pub(super) unsafe extern "C" fn on_process(data: Data) {
    // SAFETY: as in on_state.
    let inner = unsafe { inner(data) };
    if let Ok(mut events) = inner.events.try_borrow_mut() {
        if !events.contains(&Event::Process) {
            events.push(Event::Process);
        }
    }
}
