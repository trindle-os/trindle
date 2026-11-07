//! A video source.

use std::any::Any;
use std::cell::{Cell, RefCell};
use std::collections::HashSet;
use std::ffi::{c_int, c_void, CString};
use std::rc::Rc;

use crate::common::shm::Memory;

use super::connection::PipeWire;
use super::format::{linear_first, VideoFormat};
use super::native::{properties, Data, EVENTS, Lib, MetaHeader, MetaRegion, PwBuffer, SpaHook};
use super::pod::{enum_format, enum_format_card, write_pointer, CURSOR_SIZE, HEADER_SIZE, META_CURSOR, META_HEADER, META_VIDEO_DAMAGE, MetaCursor, Pod, PointerMeta, REGION_SIZE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Error,
    Unconnected,
    Connecting,
    Paused,
    Streaming,
}

pub(super) fn state_of(n: c_int) -> State {
    match n {
        0 => State::Unconnected,
        1 => State::Connecting,
        2 => State::Paused,
        3 => State::Streaming,
        _ => State::Error,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    State(State, Option<String>),
    Format(Option<VideoFormat>),
    Process,
}

pub enum Store {
    Shared(Memory),
    Card(super::super::gbm::Bo),
}

pub struct Slot {
    pub store: Store,
    pub stride: u32,
    pub format: VideoFormat,
    pub attached: RefCell<Option<Box<dyn Any>>>,
}

pub struct Card {
    pub device: Rc<super::super::gbm::Device>,
    pub modifiers: Vec<u64>,
}

pub(super) fn offered(w: u32, h: u32, card: Option<&[u64]>, fixed: Option<u64>) -> Vec<Pod> {
    let mut pods = Vec::new();
    if let Some(modifiers) = card {
        if let Some(m) = fixed {
            pods.push(enum_format_card(w, h, &[m], true));
        }
        if !modifiers.is_empty() {
            pods.push(enum_format_card(w, h, &linear_first(modifiers), false));
        }
    }
    pods.push(enum_format(w, h));
    pods
}

pub(super) struct Inner {
    pub(super) lib: &'static Lib,
    pub(super) raw: Cell<*mut c_void>,
    hook: RefCell<SpaHook>,
    pub(super) events: RefCell<Vec<Event>>,
    pub(super) format: Cell<Option<VideoFormat>>,
    pub(super) live: RefCell<HashSet<usize>>,
    pub(super) round: Cell<u64>,
    size: Cell<(u32, u32)>,
    pub(super) card: RefCell<Option<Card>>,
    pub(super) pointer: bool,
    pub(super) fixed: Cell<Option<(u64, u32)>>,
}

impl Inner {
    fn formats(&self) -> Vec<Pod> {
        let (w, h) = self.size.get();
        let card = self.card.borrow();
        offered(w, h, card.as_ref().map(|c| &c.modifiers[..]), self.fixed.get().map(|f| f.0))
    }

    pub(super) fn offer(&self) {
        let pods = self.formats();
        let params: Vec<*const u8> = pods.iter().map(|p| p.bytes().as_ptr()).collect();
        // SAFETY: the live stream, with well-formed PODs alive for the call,
        // which PipeWire copies.
        unsafe { (self.lib.stream_update_params)(self.raw.get(), params.as_ptr(), params.len() as u32) };
    }
}

pub struct Stream {
    _pw: Rc<PipeWire>,
    inner: Box<Inner>,
}

pub struct Buffer {
    raw: *mut PwBuffer,
    round: u64,
}

impl Stream {
    pub fn new(pw: &Rc<PipeWire>, name: &str, props: &[(&str, &str)], w: u32, h: u32, card: Option<Card>,
               pointer: bool) -> Result<Stream, String> {
        let lib = pw.lib;
        let props = properties(lib, props);
        let name = CString::new(name).map_err(|e| e.to_string())?;
        // SAFETY: a stream on the live core; the properties are handed over.
        let raw = unsafe { (lib.stream_new)(pw.core, name.as_ptr(), props) };
        if raw.is_null() {
            return Err("the screen could not be offered to PipeWire".into());
        }
        let inner = Box::new(Inner {
            lib,
            raw: Cell::new(raw),
            // SAFETY: an all-zero spa_hook is the unused hook PipeWire expects.
            hook: RefCell::new(unsafe { std::mem::zeroed() }),
            events: RefCell::new(Vec::new()),
            format: Cell::new(None),
            live: RefCell::new(HashSet::new()),
            round: Cell::new(0),
            size: Cell::new((w, h)),
            card: RefCell::new(card),
            fixed: Cell::new(None),
            pointer,
        });
        let data = &*inner as *const Inner as Data;
        // SAFETY: the hook and the data live in the box, which outlives the
        // stream (Drop destroys the stream first); EVENTS is static.
        unsafe { (lib.stream_add_listener)(raw, inner.hook.as_ptr(), &EVENTS, data) };
        let pods = inner.formats();
        let params: Vec<*const u8> = pods.iter().map(|p| p.bytes().as_ptr()).collect();
        const DRIVER: c_int = 1 << 3;
        const ALLOC_BUFFERS: c_int = 1 << 8;
        // SAFETY: well-formed PODs alive for the call, which PipeWire copies.
        let res = unsafe {
            (lib.stream_connect)(raw, 1, u32::MAX, DRIVER | ALLOC_BUFFERS, params.as_ptr(), params.len() as u32)
        };
        let stream = Stream { _pw: pw.clone(), inner };
        if res < 0 {
            return Err(format!("PipeWire refused the stream ({res})"));
        }
        Ok(stream)
    }

    pub fn node_id(&self) -> u32 {
        // SAFETY: the live stream.
        unsafe { (self.inner.lib.stream_get_node_id)(self.inner.raw.get()) }
    }

    pub fn take_events(&self) -> Vec<Event> {
        let events = std::mem::take(&mut *self.inner.events.borrow_mut());
        if events.iter().any(|e| matches!(e, Event::State(State::Streaming, _))) {
            let raw = self.inner.raw.get();
            // SAFETY: the live stream, outside its callbacks.
            unsafe {
                if (self.inner.lib.stream_is_driving)(raw) {
                    (self.inner.lib.stream_trigger_process)(raw);
                }
            }
        }
        events
    }

    pub fn resize(&self, w: u32, h: u32) {
        self.inner.size.set((w, h));
        self.inner.fixed.set(None);
        self.inner.offer();
    }

    pub fn dequeue(&self) -> Option<Buffer> {
        // SAFETY: the live stream; null when none is free.
        let raw = unsafe { (self.inner.lib.stream_dequeue_buffer)(self.inner.raw.get()) };
        if raw.is_null() {
            return None;
        }
        let buffer = Buffer { raw, round: self.inner.round.get() };
        if self.slot(&buffer).is_none() {
            // SAFETY: the buffer was dequeued from this live stream just now.
            unsafe { (self.inner.lib.stream_return_buffer)(self.inner.raw.get(), raw) };
            return None;
        }
        Some(buffer)
    }

    pub fn slot(&self, buffer: &Buffer) -> Option<&Slot> {
        if buffer.round != self.inner.round.get() || !self.inner.live.borrow().contains(&(buffer.raw as usize)) {
            return None;
        }
        // SAFETY: a live pw_buffer (checked above) whose user_data was set
        // to a boxed Slot in on_add_buffer and is freed only in
        // on_remove_buffer, which would have taken it out of `live`.
        unsafe { ((*buffer.raw).user_data as *const Slot).as_ref() }
    }

    pub fn queue(&self, buffer: Buffer, damage: &[(i32, i32, u32, u32)], pts_ns: i64, seq: u64,
                 pointer: Option<&PointerMeta>, only_pointer: bool) -> bool {
        let Some(slot) = self.slot(&buffer) else { return false };
        let height = slot.format.height;
        // SAFETY: the buffer is live (checked by `slot`); its spa_buffer has
        // n_datas datas and n_metas metas, each meta `size` bytes at `data`,
        // as PipeWire made them for the Meta params given.
        unsafe {
            let spa = &mut *(*buffer.raw).buffer;
            for i in 0..spa.n_datas as usize {
                let chunk = &mut *(*spa.datas.add(i)).chunk;
                let (offset, stride) = match &slot.store {
                    Store::Shared(_) => (0, slot.stride),
                    Store::Card(bo) => bo.planes.get(i).map(|p| (p.offset, p.stride)).unwrap_or((0, slot.stride)),
                };
                chunk.offset = offset;
                chunk.size = if only_pointer { 0 } else { stride * height };
                chunk.stride = stride as i32;
                chunk.flags = 0;
            }
            for i in 0..spa.n_metas as usize {
                let meta = &*spa.metas.add(i);
                match meta.kind {
                    META_HEADER if meta.size as usize >= HEADER_SIZE => {
                        let header = &mut *(meta.data as *mut MetaHeader);
                        *header = MetaHeader { flags: 0, offset: 0, pts: pts_ns, dts_offset: 0, seq };
                    }
                    META_VIDEO_DAMAGE => {
                        let room = meta.size as usize / REGION_SIZE;
                        let regions = std::slice::from_raw_parts_mut(meta.data as *mut MetaRegion, room);
                        fill_damage(regions, if only_pointer { &[(0, 0, 0, 0)] } else { damage }, slot.format.width, height);
                    }
                    META_CURSOR => match pointer {
                        Some(p) => write_pointer(meta.data as *mut u8, meta.size as usize, p),
                        None if meta.size as usize >= CURSOR_SIZE => (*(meta.data as *mut MetaCursor)).id = 0,
                        None => {}
                    },
                    _ => {}
                }
            }
            (self.inner.lib.stream_queue_buffer)(self.inner.raw.get(), buffer.raw);
            if (self.inner.lib.stream_is_driving)(self.inner.raw.get()) {
                (self.inner.lib.stream_trigger_process)(self.inner.raw.get());
            }
        }
        true
    }
}

pub(super) fn fill_damage(regions: &mut [MetaRegion], damage: &[(i32, i32, u32, u32)], w: u32, h: u32) {
    if regions.is_empty() {
        return;
    }
    let whole = [(0, 0, w, h)];
    let damage = if damage.is_empty() { &whole[..] } else { damage };
    let bounds;
    let list = if damage.len() > regions.len() - usize::from(regions.len() > 1) {
        let x0 = damage.iter().map(|d| d.0).min().unwrap_or(0);
        let y0 = damage.iter().map(|d| d.1).min().unwrap_or(0);
        let x1 = damage.iter().map(|d| d.0 + d.2 as i32).max().unwrap_or(0);
        let y1 = damage.iter().map(|d| d.1 + d.3 as i32).max().unwrap_or(0);
        bounds = [(x0, y0, (x1 - x0).max(0) as u32, (y1 - y0).max(0) as u32)];
        &bounds[..]
    } else {
        damage
    };
    for (i, region) in regions.iter_mut().enumerate() {
        *region = match list.get(i) {
            Some(&(x, y, w, h)) => MetaRegion { x, y, w, h },
            None => MetaRegion { x: 0, y: 0, w: 0, h: 0 },
        };
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        // SAFETY: destroys the live stream once; its buffers are let go
        // through on_remove_buffer before this returns, while the box the
        // callbacks reach is still here.
        unsafe { (self.inner.lib.stream_destroy)(self.inner.raw.get()) };
        self.inner.raw.set(std::ptr::null_mut());
    }
}
