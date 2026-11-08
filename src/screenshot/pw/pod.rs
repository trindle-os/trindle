//! Writing the PODs PipeWire reads: formats, buffers and metadata, and the pointer sent beside a frame.

use super::format::VideoFormat;

pub(super) const POD_ID: u32 = 3;
pub(super) const POD_INT: u32 = 4;
pub(super) const POD_LONG: u32 = 5;
pub(super) const POD_RECTANGLE: u32 = 10;
pub(super) const POD_FRACTION: u32 = 11;
pub(super) const POD_OBJECT: u32 = 15;
pub(super) const POD_CHOICE: u32 = 19;

pub(super) const CHOICE_RANGE: u32 = 1;
pub(super) const CHOICE_ENUM: u32 = 3;
const CHOICE_FLAGS: u32 = 4;

pub(super) const OBJECT_FORMAT: u32 = 0x40003;
pub(super) const OBJECT_PARAM_BUFFERS: u32 = 0x40004;
const OBJECT_PARAM_META: u32 = 0x40005;

pub(super) const PARAM_ENUM_FORMAT: u32 = 3;
pub(super) const PARAM_FORMAT: u32 = 4;
const PARAM_BUFFERS: u32 = 5;
const PARAM_META: u32 = 6;

pub(super) const FORMAT_MEDIA_TYPE: u32 = 1;
pub(super) const FORMAT_MEDIA_SUBTYPE: u32 = 2;
pub(super) const FORMAT_VIDEO_FORMAT: u32 = 0x20001;
pub(super) const FORMAT_VIDEO_MODIFIER: u32 = 0x20002;
pub(super) const FORMAT_VIDEO_SIZE: u32 = 0x20003;
pub(super) const FORMAT_VIDEO_FRAMERATE: u32 = 0x20004;
pub(super) const FORMAT_VIDEO_MAX_FRAMERATE: u32 = 0x20005;
pub(super) const MEDIA_TYPE_VIDEO: u32 = 2;
pub(super) const MEDIA_SUBTYPE_RAW: u32 = 1;

const BUFFERS_BUFFERS: u32 = 1;
const BUFFERS_BLOCKS: u32 = 2;
pub(super) const BUFFERS_SIZE: u32 = 3;
const BUFFERS_STRIDE: u32 = 4;
const BUFFERS_DATA_TYPE: u32 = 6;
const META_TYPE: u32 = 1;
const META_SIZE: u32 = 2;

pub const VIDEO_BGRX: u32 = 8;

pub(super) const DATA_MEMFD: u32 = 2;

pub const MOD_LINEAR: u64 = 0;

pub(super) const RATE: (u32, u32) = (60, 1);
pub(super) const DATA_DMABUF: u32 = 3;

pub(super) const PROP_MANDATORY: u32 = 1 << 3;
pub(super) const PROP_DONT_FIXATE: u32 = 1 << 4;
pub(super) const DATA_FLAG_READWRITE: u32 = 0b11;
pub(super) const DATA_FLAG_MAPPABLE: u32 = 1 << 3;

pub(super) const META_HEADER: u32 = 1;
pub(super) const META_VIDEO_DAMAGE: u32 = 3;
pub(super) const META_CURSOR: u32 = 5;

pub(super) const CURSOR_SIZE: usize = 28;
pub(super) const BITMAP_SIZE: usize = 20;

pub const POINTER_PIXELS: u32 = 128;
pub(super) const VIDEO_BGRA: u32 = 12;

const DAMAGE_RECTS: usize = 16;

pub(super) const REGION_SIZE: usize = 16;

pub(super) const HEADER_SIZE: usize = 32;

#[derive(Default)]
pub struct Pod(Vec<u8>);

impl Pod {
    fn word(&mut self, v: u32) {
        self.0.extend_from_slice(&v.to_ne_bytes());
    }

    fn pad(&mut self) {
        while self.0.len() % 8 != 0 {
            self.0.push(0);
        }
    }

    fn wrap(&mut self, kind: u32, body: impl FnOnce(&mut Pod)) {
        let at = self.0.len();
        self.word(0);
        self.word(kind);
        let start = self.0.len();
        body(self);
        let size = (self.0.len() - start) as u32;
        self.0[at..at + 4].copy_from_slice(&size.to_ne_bytes());
        self.pad();
    }

    pub(super) fn id(&mut self, v: u32) {
        self.wrap(POD_ID, |p| p.word(v));
    }

    fn int(&mut self, v: i32) {
        self.wrap(POD_INT, |p| p.word(v as u32));
    }

    fn long(&mut self, v: u64) {
        self.wrap(POD_LONG, |p| p.0.extend_from_slice(&v.to_ne_bytes()));
    }

    pub(super) fn choice_long(&mut self, values: &[u64]) {
        self.wrap(POD_CHOICE, |p| {
            p.word(CHOICE_ENUM);
            p.word(0);
            p.word(8);
            p.word(POD_LONG);
            for v in values.iter().take(1).chain(values) {
                p.0.extend_from_slice(&v.to_ne_bytes());
            }
        });
    }

    pub(super) fn rectangle(&mut self, w: u32, h: u32) {
        self.wrap(POD_RECTANGLE, |p| {
            p.word(w);
            p.word(h);
        });
    }

    pub(super) fn fraction(&mut self, num: u32, denom: u32) {
        self.wrap(POD_FRACTION, |p| {
            p.word(num);
            p.word(denom);
        });
    }

    pub(super) fn choice(&mut self, kind: u32, child_type: u32, child_size: u32, values: &[&[u32]]) {
        self.wrap(POD_CHOICE, |p| {
            p.word(kind);
            p.word(0);
            p.word(child_size);
            p.word(child_type);
            for v in values {
                for w in *v {
                    p.word(*w);
                }
            }
        });
    }

    pub(super) fn object(kind: u32, id: u32, props: impl FnOnce(&mut Props)) -> Pod {
        let mut pod = Pod::default();
        pod.wrap(POD_OBJECT, |p| {
            p.word(kind);
            p.word(id);
            props(&mut Props(p));
        });
        pod
    }

    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}

pub(super) struct Props<'a>(&'a mut Pod);

impl Props<'_> {
    pub(super) fn key(&mut self, key: u32) -> &mut Pod {
        self.key_with(key, 0)
    }

    pub(super) fn key_with(&mut self, key: u32, flags: u32) -> &mut Pod {
        self.0.word(key);
        self.0.word(flags);
        self.0
    }
}

pub fn enum_format(w: u32, h: u32) -> Pod {
    Pod::object(OBJECT_FORMAT, PARAM_ENUM_FORMAT, |p| {
        p.key(FORMAT_MEDIA_TYPE).id(MEDIA_TYPE_VIDEO);
        p.key(FORMAT_MEDIA_SUBTYPE).id(MEDIA_SUBTYPE_RAW);
        p.key(FORMAT_VIDEO_FORMAT).id(VIDEO_BGRX);
        p.key(FORMAT_VIDEO_SIZE).rectangle(w, h);
        p.key(FORMAT_VIDEO_FRAMERATE).fraction(0, 1);
        p.key(FORMAT_VIDEO_MAX_FRAMERATE).choice(CHOICE_RANGE, POD_FRACTION, 8, &[&[RATE.0, RATE.1], &[1, 1], &[360, 1]]);
    })
}

pub fn enum_format_card(w: u32, h: u32, modifiers: &[u64], fixed: bool) -> Pod {
    Pod::object(OBJECT_FORMAT, PARAM_ENUM_FORMAT, |p| {
        p.key(FORMAT_MEDIA_TYPE).id(MEDIA_TYPE_VIDEO);
        p.key(FORMAT_MEDIA_SUBTYPE).id(MEDIA_SUBTYPE_RAW);
        p.key(FORMAT_VIDEO_FORMAT).id(VIDEO_BGRX);
        match (fixed, modifiers) {
            (true, [m, ..]) => p.key_with(FORMAT_VIDEO_MODIFIER, PROP_MANDATORY).long(*m),
            _ => p.key_with(FORMAT_VIDEO_MODIFIER, PROP_MANDATORY | PROP_DONT_FIXATE).choice_long(modifiers),
        }
        p.key(FORMAT_VIDEO_SIZE).rectangle(w, h);
        p.key(FORMAT_VIDEO_FRAMERATE).fraction(0, 1);
        p.key(FORMAT_VIDEO_MAX_FRAMERATE).choice(CHOICE_RANGE, POD_FRACTION, 8, &[&[RATE.0, RATE.1], &[1, 1], &[360, 1]]);
    })
}

pub fn buffers_card(planes: u32) -> Pod {
    Pod::object(OBJECT_PARAM_BUFFERS, PARAM_BUFFERS, |p| {
        p.key(BUFFERS_BUFFERS).choice(CHOICE_RANGE, POD_INT, 4, &[&[3], &[2], &[8]]);
        p.key(BUFFERS_BLOCKS).int(planes as i32);
        p.key(BUFFERS_DATA_TYPE).choice(CHOICE_FLAGS, POD_INT, 4, &[&[1 << DATA_DMABUF]]);
    })
}

pub fn buffers(format: &VideoFormat) -> Pod {
    let stride = format.width * 4;
    Pod::object(OBJECT_PARAM_BUFFERS, PARAM_BUFFERS, |p| {
        p.key(BUFFERS_BUFFERS).choice(CHOICE_RANGE, POD_INT, 4, &[&[3], &[2], &[8]]);
        p.key(BUFFERS_BLOCKS).int(1);
        p.key(BUFFERS_SIZE).int((stride * format.height) as i32);
        p.key(BUFFERS_STRIDE).int(stride as i32);
        p.key(BUFFERS_DATA_TYPE).choice(CHOICE_FLAGS, POD_INT, 4, &[&[1 << DATA_MEMFD]]);
    })
}

pub fn metas(pointer: bool) -> Vec<Pod> {
    let mut all = vec![
        Pod::object(OBJECT_PARAM_META, PARAM_META, |p| {
            p.key(META_TYPE).id(META_HEADER);
            p.key(META_SIZE).int(HEADER_SIZE as i32);
        }),
        Pod::object(OBJECT_PARAM_META, PARAM_META, |p| {
            p.key(META_TYPE).id(META_VIDEO_DAMAGE);
            let (most, least) = ((REGION_SIZE * DAMAGE_RECTS) as u32, REGION_SIZE as u32);
            p.key(META_SIZE).choice(CHOICE_RANGE, POD_INT, 4, &[&[most], &[least], &[most]]);
        }),
    ];
    if pointer {
        let size = CURSOR_SIZE + BITMAP_SIZE + (POINTER_PIXELS * POINTER_PIXELS * 4) as usize;
        all.push(Pod::object(OBJECT_PARAM_META, PARAM_META, |p| {
            p.key(META_TYPE).id(META_CURSOR);
            p.key(META_SIZE).int(size as i32);
        }));
    }
    all
}

pub struct PointerMeta<'a> {
    pub over: bool,
    pub at: (i32, i32),
    pub hotspot: (i32, i32),
    pub picture: &'a [u8],
    pub size: (u32, u32),
}

#[repr(C)]
pub(super) struct MetaCursor {
    pub(super) id: u32,
    flags: u32,
    pub(super) x: i32,
    pub(super) y: i32,
    pub(super) hot_x: i32,
    pub(super) hot_y: i32,
    pub(super) bitmap_offset: u32,
}

#[repr(C)]
pub(super) struct MetaBitmap {
    format: u32,
    pub(super) width: u32,
    pub(super) height: u32,
    stride: i32,
    offset: u32,
}

pub(super) unsafe fn write_pointer(data: *mut u8, room: usize, pointer: &PointerMeta) {
    let (w, h) = pointer.size;
    let fits = pointer.over && w > 0 && h > 0 && pointer.picture.len() >= (w * h * 4) as usize
        && CURSOR_SIZE + BITMAP_SIZE + (w * h * 4) as usize <= room;
    let (w, h) = if fits { (w, h) } else { (1, 1) };
    if CURSOR_SIZE + BITMAP_SIZE + 4 > room {
        return;
    }
    // SAFETY: as the caller promises; each write lies inside `room`, which

    unsafe {
        std::ptr::write_unaligned(data as *mut MetaCursor, MetaCursor {
            id: 1,
            flags: 0,
            x: pointer.at.0,
            y: pointer.at.1,
            hot_x: if fits { pointer.hotspot.0 } else { 0 },
            hot_y: if fits { pointer.hotspot.1 } else { 0 },
            bitmap_offset: CURSOR_SIZE as u32,
        });
        std::ptr::write_unaligned(data.add(CURSOR_SIZE) as *mut MetaBitmap, MetaBitmap {
            format: VIDEO_BGRA,
            width: w,
            height: h,
            stride: (w * 4) as i32,
            offset: BITMAP_SIZE as u32,
        });
        let pixels = data.add(CURSOR_SIZE + BITMAP_SIZE);
        if fits {
            std::ptr::copy_nonoverlapping(pointer.picture.as_ptr(), pixels, (w * h * 4) as usize);
        } else {
            std::ptr::write_bytes(pixels, 0, 4);
        }
    }
}
