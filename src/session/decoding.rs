//! The film decoder, which is this program started again as `trindle decode-film`.

use std::ffi::{c_char, c_int, c_long, c_uint, c_void, CStr};
use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::{FromRawFd, OwnedFd};
use std::os::unix::net::UnixStream;
use std::sync::OnceLock;
use std::time::Duration;

use super::moving::{Clip, Codec, Room, FILM_FD, FRAME_CPU, HELLO, NO_LIBRARY, ROOM_FD, TALK_FD};

#[repr(C)]
struct CodecCtx {
    name: *const c_char,
    iface: *const c_void,
    err: c_int,
    err_detail: *const c_char,
    init_flags: c_long,
    config: *const c_void,
    private: *mut c_void,
}

#[repr(C)]
struct DecCfg {
    threads: c_uint,
    w: c_uint,
    h: c_uint,
}

#[repr(C)]
struct Image {
    fmt: c_int,
    cs: c_int,
    range: c_int,
    w: c_uint,
    h: c_uint,
    bit_depth: c_uint,
    d_w: c_uint,
    d_h: c_uint,
    r_w: c_uint,
    r_h: c_uint,
    x_chroma_shift: c_uint,
    y_chroma_shift: c_uint,
    planes: [*mut u8; 4],
    stride: [c_int; 4],
    bps: c_int,
    user_priv: *mut c_void,
    img_data: *mut u8,
    img_data_owner: c_int,
    self_allocd: c_int,
    fb_priv: *mut c_void,
}

const DECODER_ABI: c_int = 12;

type InitFn = unsafe extern "C" fn(*mut CodecCtx, *const c_void, *const DecCfg, c_long, c_int) -> c_int;
type DecodeFn = unsafe extern "C" fn(*mut CodecCtx, *const u8, c_uint, *mut c_void, c_long) -> c_int;
type FrameFn = unsafe extern "C" fn(*mut CodecCtx, *mut *const c_void) -> *const Image;
type DestroyFn = unsafe extern "C" fn(*mut CodecCtx) -> c_int;

struct Vpx {
    vp8: unsafe extern "C" fn() -> *const c_void,
    vp9: unsafe extern "C" fn() -> *const c_void,
    init: InitFn,
    decode: DecodeFn,
    frame: FrameFn,
    destroy: DestroyFn,
}

fn vpx() -> Option<&'static Vpx> {
    static LIB: OnceLock<Option<Vpx>> = OnceLock::new();
    LIB.get_or_init(|| {
        // SAFETY: dlopen and dlsym with NUL-terminated names; each symbol is
        // cast to the signature vpx_decoder.h gives it, for the ABI checked
        // at init. The library is never closed, so the pointers stay good.
        unsafe {
            let lib = libc::dlopen(c"libvpx.so.9".as_ptr(), libc::RTLD_NOW | libc::RTLD_LOCAL);
            if lib.is_null() {
                return None;
            }
            let sym = |name: &CStr| {
                let p = libc::dlsym(lib, name.as_ptr());
                (!p.is_null()).then_some(p)
            };
            Some(Vpx {
                vp8: std::mem::transmute::<*mut c_void, unsafe extern "C" fn() -> *const c_void>(sym(c"vpx_codec_vp8_dx")?),
                vp9: std::mem::transmute::<*mut c_void, unsafe extern "C" fn() -> *const c_void>(sym(c"vpx_codec_vp9_dx")?),
                init: std::mem::transmute::<*mut c_void, InitFn>(sym(c"vpx_codec_dec_init_ver")?),
                decode: std::mem::transmute::<*mut c_void, DecodeFn>(sym(c"vpx_codec_decode")?),
                frame: std::mem::transmute::<*mut c_void, FrameFn>(sym(c"vpx_codec_get_frame")?),
                destroy: std::mem::transmute::<*mut c_void, DestroyFn>(sym(c"vpx_codec_destroy")?),
            })
        }
    }).as_ref()
}

struct Bits<'a> {
    data: &'a [u8],
    at: usize,
}

impl Bits<'_> {
    fn get(&mut self, count: usize) -> Option<u32> {
        let mut value = 0;
        for _ in 0..count {
            let byte = *self.data.get(self.at / 8)?;
            value = (value << 1) | u32::from((byte >> (7 - self.at % 8)) & 1);
            self.at += 1;
        }
        Some(value)
    }
}

fn said_size(codec: Codec, frame: &[u8]) -> Result<Option<(u32, u32)>, ()> {
    match codec {
        Codec::Vp8 => vp8_size(frame),
        Codec::Vp9 => vp9_sizes(frame),
    }
}

fn vp8_size(frame: &[u8]) -> Result<Option<(u32, u32)>, ()> {
    let tag = frame.get(..3).ok_or(())?;
    if tag[0] & 1 == 1 {
        return Ok(None);
    }
    let head = frame.get(3..10).ok_or(())?;
    if head[..3] != [0x9d, 0x01, 0x2a] {
        return Err(());
    }
    let side = |low: u8, high: u8| u32::from(u16::from_le_bytes([low, high]) & 0x3fff);
    Ok(Some((side(head[3], head[4]), side(head[5], head[6]))))
}

fn vp9_list(piece: &[u8]) -> Option<(usize, usize, usize)> {
    let last = *piece.last()?;
    if last & 0xe0 != 0xc0 {
        return None;
    }
    let (count, each) = (usize::from(last & 7) + 1, usize::from((last >> 3) & 3) + 1);
    let list = 2 + count * each;
    (piece.len() >= list && piece[piece.len() - list] == last).then_some((count, each, list))
}

fn vp9_one(length: usize) -> [u8; 6] {
    let mark = 0xc0 | (3 << 3);
    let l = (length as u32).to_le_bytes();
    [mark, l[0], l[1], l[2], l[3], mark]
}

fn vp9_sizes(piece: &[u8]) -> Result<Option<(u32, u32)>, ()> {
    let mut largest: Option<(u32, u32)> = None;
    let mut take = |frame: &[u8]| -> Result<(), ()> {
        if let Some((w, h)) = vp9_size(frame)? {
            let (mw, mh) = largest.unwrap_or((0, 0));
            largest = Some((mw.max(w), mh.max(h)));
        }
        Ok(())
    };
    let Some((_, each, list)) = vp9_list(piece) else {
        take(piece)?;
        return Ok(largest);
    };
    let lengths = &piece[piece.len() - list + 1..piece.len() - 1];
    let mut at = 0usize;
    for length in lengths.chunks_exact(each) {
        let length = length.iter().rev().fold(0usize, |v, b| (v << 8) | usize::from(*b));
        let end = at.checked_add(length).filter(|end| *end <= piece.len() - list).ok_or(())?;
        if length > 0 {
            take(&piece[at..end])?;
        }
        at = end;
    }
    Ok(largest)
}

fn vp9_size(frame: &[u8]) -> Result<Option<(u32, u32)>, ()> {
    read_vp9_size(&mut Bits { data: frame, at: 0 }).ok_or(())?
}

fn read_vp9_size(bits: &mut Bits) -> Option<Result<Option<(u32, u32)>, ()>> {
    if bits.get(2)? != 2 {
        return Some(Err(()));
    }
    let profile = bits.get(1)? | (bits.get(1)? << 1);
    if profile == 3 && bits.get(1)? != 0 {
        return Some(Err(()));
    }
    if bits.get(1)? == 1 {
        return Some(Ok(None));
    }
    let (key, shown, resilient) = (bits.get(1)? == 0, bits.get(1)? == 1, bits.get(1)? == 1);
    if key {
        if !vp9_sync(bits)? {
            return Some(Err(()));
        }
        vp9_colour(bits, profile)?;
        return Some(Ok(Some(vp9_picture(bits)?)));
    }
    let intra = !shown && bits.get(1)? == 1;
    if !resilient {
        bits.get(2)?;
    }
    if intra {
        if !vp9_sync(bits)? {
            return Some(Err(()));
        }
        if profile > 0 {
            vp9_colour(bits, profile)?;
        }
        bits.get(8)?;
        return Some(Ok(Some(vp9_picture(bits)?)));
    }
    bits.get(8 + 3 * 4)?;
    for _ in 0..3 {
        if bits.get(1)? == 1 {
            return Some(Ok(None));
        }
    }
    Some(Ok(Some(vp9_picture(bits)?)))
}

fn vp9_sync(bits: &mut Bits) -> Option<bool> {
    Some(bits.get(24)? == 0x49_83_42)
}

fn vp9_colour(bits: &mut Bits, profile: u32) -> Option<()> {
    if profile >= 2 {
        bits.get(1)?;
    }
    let odd = profile == 1 || profile == 3;
    if bits.get(3)? != 7 {
        bits.get(1)?;
        if odd {
            bits.get(3)?;
        }
    } else if odd {
        bits.get(1)?;
    }
    Some(())
}

fn vp9_picture(bits: &mut Bits) -> Option<(u32, u32)> {
    Some((bits.get(16)? + 1, bits.get(16)? + 1))
}

fn fits(codec: Codec, frame: &[u8], most: (u32, u32)) -> bool {
    matches!(said_size(codec, frame), Ok(said) if said.is_none_or(|(w, h)| w <= most.0 && h <= most.1))
}

struct Decoder {
    lib: &'static Vpx,
    ctx: Box<CodecCtx>,
    codec: Codec,
    one: Vec<u8>,
}

impl Decoder {
    fn new(codec: Codec) -> Option<Decoder> {
        let lib = vpx()?;
        let mut ctx = Box::new(CodecCtx {
            name: std::ptr::null(),
            iface: std::ptr::null(),
            err: 0,
            err_detail: std::ptr::null(),
            init_flags: 0,
            config: std::ptr::null(),
            private: std::ptr::null_mut(),
        });
        let cfg = DecCfg { threads: 1, w: 0, h: 0 };
        // SAFETY: the iface functions take nothing and return a static table;
        // init fills `ctx`, which lives in a Box so it never moves after.
        let ok = unsafe {
            let iface = match codec { Codec::Vp8 => (lib.vp8)(), Codec::Vp9 => (lib.vp9)() };
            (lib.init)(&mut *ctx, iface, &cfg, 0, DECODER_ABI) == 0
        };
        ok.then_some(Decoder { lib, ctx, codec, one: Vec::new() })
    }

    fn decode<T>(&mut self, data: &[u8], most: (u32, u32), out: impl FnOnce(&Image) -> T) -> Option<T> {
        if !fits(self.codec, data, most) {
            return None;
        }
        let data = if self.codec == Codec::Vp9 && vp9_list(data).is_none() {
            self.one.clear();
            self.one.extend_from_slice(data);
            self.one.extend(vp9_one(data.len()));
            &self.one[..]
        } else {
            data
        };
        // SAFETY: `data` is valid for its length through the call; the image
        // returned stays valid until the next decode, and is only read
        // within `out`, before this borrow of self ends.
        unsafe {
            if (self.lib.decode)(&mut *self.ctx, data.as_ptr(), data.len() as c_uint, std::ptr::null_mut(), 0) != 0 {
                return None;
            }
            let mut iter: *const c_void = std::ptr::null();
            let mut last = None;
            loop {
                let img = (self.lib.frame)(&mut *self.ctx, &mut iter);
                if img.is_null() {
                    break;
                }
                last = Some(img);
            }
            let img = &*last?;
            usable(img).then(|| out(img))
        }
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        // SAFETY: the context set up in `new`, freed once.
        unsafe { (self.lib.destroy)(&mut *self.ctx) };
    }
}

fn usable(img: &Image) -> bool {
    matches!(img.fmt, 0x102 | 0x105 | 0x106 | 0x107) && img.d_w > 0 && img.d_h > 0
        && img.x_chroma_shift <= 1 && img.y_chroma_shift <= 1
}

struct Matrix {
    y: i32,
    black: i32,
    rv: i32,
    gu: i32,
    gv: i32,
    bu: i32,
}

fn matrix(img: &Image) -> Matrix {
    let hd = img.cs == 2 || (img.cs == 0 && img.d_h >= 720);
    match (hd, img.range == 1) {
        (true, false) => Matrix { y: 298, black: 16, rv: 459, gu: -55, gv: -136, bu: 541 },
        (true, true) => Matrix { y: 256, black: 0, rv: 403, gu: -48, gv: -120, bu: 475 },
        (false, false) => Matrix { y: 298, black: 16, rv: 409, gu: -100, gv: -208, bu: 516 },
        (false, true) => Matrix { y: 256, black: 0, rv: 359, gu: -88, gv: -183, bu: 454 },
    }
}

fn to_xrgb(img: &Image, out: &mut [u8], stride: usize, step: usize) -> bool {
    let m = matrix(img);
    let step = step.max(1);
    let (w, h) = (img.d_w as usize / step, img.d_h as usize / step);
    if super::moving::span(stride, w * 4, h).is_none_or(|end| end > out.len()) {
        return false;
    }
    let (xs, ys) = (img.x_chroma_shift, img.y_chroma_shift);
    let (mut rs, mut gs, mut bs) = (vec![0i32; w], vec![0i32; w], vec![0i32; w]);
    let mut ys_row = vec![0i32; w];
    for row in 0..h {
        let sy = row * step;
        // SAFETY: rows below d_h and columns below d_w lie inside each plane,
        // whose strides libvpx gives; chroma rows and columns are shifted
        // down to match the plane's smaller size.
        let (yr, ur, vr) = unsafe {
            (
                std::slice::from_raw_parts(img.planes[0].offset(sy as isize * img.stride[0] as isize), img.d_w as usize),
                std::slice::from_raw_parts(img.planes[1].offset((sy >> ys) as isize * img.stride[1] as isize),
                                           (img.d_w as usize + xs as usize) >> xs),
                std::slice::from_raw_parts(img.planes[2].offset((sy >> ys) as isize * img.stride[2] as isize),
                                           (img.d_w as usize + xs as usize) >> xs),
            )
        };
        let dst = &mut out[row * stride..row * stride + w * 4];
        if sy == 0 || (sy >> ys) != ((sy - step) >> ys) || step > 1 {
            for x in 0..w {
                let c = (x * step) >> xs;
                let (u, v) = (ur[c] as i32 - 128, vr[c] as i32 - 128);
                rs[x] = m.rv * v + 128;
                gs[x] = m.gu * u + m.gv * v + 128;
                bs[x] = m.bu * u + 128;
            }
        }
        for (y, src) in ys_row.iter_mut().zip(yr.iter().step_by(step)) {
            *y = *src as i32 - m.black;
        }
        // SAFETY: any bytes can be read as whole u32s; align_to_mut leaves
        // out whatever does not line up, which is nothing for the buffers
        // used here (page-aligned memory, four bytes a pixel).
        let (_, dst, _) = unsafe { dst.align_to_mut::<u32>() };
        #[cfg(target_arch = "x86_64")]
        if std::arch::is_x86_feature_detected!("avx2") {
            // SAFETY: only reached on a processor that has AVX2.
            unsafe { pixels_avx2(dst, &ys_row, m.y, &rs, &gs, &bs) };
            continue;
        }
        pixels(dst, &ys_row, m.y, &rs, &gs, &bs);
    }
    true
}

#[inline(always)]
fn pixels(dst: &mut [u32], ys: &[i32], k: i32, rs: &[i32], gs: &[i32], bs: &[i32]) {
    for ((((px, y), r), g), b) in dst.iter_mut().zip(ys).zip(rs).zip(gs).zip(bs) {
        let y = y * k;
        *px = 0xFF00_0000 | (clamp(y + r) << 16) | (clamp(y + g) << 8) | clamp(y + b);
    }
}

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
unsafe fn pixels_avx2(dst: &mut [u32], ys: &[i32], k: i32, rs: &[i32], gs: &[i32], bs: &[i32]) {
    pixels(dst, ys, k, rs, gs, bs)
}

fn clamp(v: i32) -> u32 {
    (v >> 8).clamp(0, 255) as u32
}

const NO_START: i32 = 4;

#[derive(Clone, Copy, PartialEq)]
enum Part {
    Decode,
    #[cfg(test)]
    Abort,
    #[cfg(test)]
    Sleep,
    #[cfg(test)]
    Spin,
    #[cfg(test)]
    Greedy,
    #[cfg(test)]
    Prisoner,
    #[cfg(test)]
    OtherFilm,
    #[cfg(test)]
    NoLibrary,
    #[cfg(test)]
    Says(u32, u32),
}

pub fn main(_args: Vec<String>) -> i32 {
    child(Part::Decode)
}

fn shut_in() {
    let none = libc::rlimit { rlim_cur: 0, rlim_max: 0 };
    // SAFETY: two plain calls that read the limit handed to them and
    // touch no memory of ours.
    unsafe {
        libc::setrlimit(libc::RLIMIT_NOFILE, &none);
        libc::setrlimit(libc::RLIMIT_FSIZE, &none);
    }
}

fn limit(time: Duration) {
    let none = libc::timeval { tv_sec: 0, tv_usec: 0 };
    let value = libc::timeval { tv_sec: time.as_secs() as libc::time_t, tv_usec: time.subsec_micros() as libc::suseconds_t };
    let timer = libc::itimerval { it_interval: none, it_value: value };
    // SAFETY: a plain call that reads the timer handed to it; nothing is
    // asked back.
    unsafe { libc::setitimer(libc::ITIMER_PROF, &timer, std::ptr::null_mut()) };
}

#[cfg(not(test))]
fn instead(_part: Part) -> Option<(u32, u32)> {
    None
}

#[cfg(test)]
fn instead(part: Part) -> Option<(u32, u32)> {
    tests::instead(part)
}

fn begun_again(fresh: &dyn Fn() -> Option<Decoder>, clip: &Clip, index: usize, buf: &mut Vec<u8>) -> Option<Decoder> {
    let mut decoder = fresh()?;
    for before in 0..index {
        limit(FRAME_CPU);
        if clip.read(before, buf) {
            decoder.decode(&buf[..], (clip.info.width, clip.info.height), |_| ());
        }
    }
    Some(decoder)
}

fn one_frame(decoder: &mut Decoder, clip: &Clip, index: usize, step: usize, buf: &mut Vec<u8>, room: &mut Room) -> Option<(u32, u32)> {
    let (w, h) = (clip.info.width, clip.info.height);
    limit(FRAME_CPU);
    let made = clip.read(index, buf).then(|| decoder.decode(&buf[..], (w, h), |img| {
        let (fw, fh) = (img.d_w / step as u32, img.d_h / step as u32);
        if img.d_w > w || img.d_h > h || fw == 0 || fh == 0 {
            return None;
        }
        to_xrgb(img, room.bytes_mut(), (w as usize / step) * 4, step).then_some((fw, fh))
    })).flatten().flatten();
    limit(Duration::ZERO);
    made
}

fn child(part: Part) -> i32 {
    // SAFETY: the three descriptors whoever started this process handed
    // it (`moving::Player::begin`), each taken over here once and owned
    // by nothing else. If one is not there, the first use of it fails
    // and the process ends.
    let (film, mut talk, room) = unsafe { (File::from_raw_fd(FILM_FD), UnixStream::from_raw_fd(TALK_FD), OwnedFd::from_raw_fd(ROOM_FD)) };
    #[cfg(test)]
    if part == Part::NoLibrary {
        return NO_LIBRARY;
    }
    let Some(clip) = Clip::of(film) else { return NO_START };
    let (w, h) = (clip.info.width, clip.info.height);
    if vpx().is_none() {
        return NO_LIBRARY;
    }
    let codec = clip.codec;
    let fresh = move || Decoder::new(codec);
    let Some(decoder) = fresh() else { return NO_START };
    let Some(room) = Room::of(room, w as usize * h as usize * 4) else { return NO_START };
    shut_in();
    if talk.write_all(&hello(&clip, said_width(part, w))).is_err() {
        return NO_START;
    }
    serve_frames(part, &mut talk, (&clip, &fresh), decoder, room)
}

fn hello(clip: &Clip, width: u32) -> [u8; 16] {
    let mut hello = [0u8; 16];
    hello[..4].copy_from_slice(&HELLO);
    hello[4..8].copy_from_slice(&(clip.len() as u32).to_le_bytes());
    hello[8..12].copy_from_slice(&width.to_le_bytes());
    hello[12..].copy_from_slice(&clip.info.height.to_le_bytes());
    hello
}

#[cfg(not(test))]
fn said_width(_part: Part, width: u32) -> u32 {
    width
}

#[cfg(test)]
fn said_width(part: Part, width: u32) -> u32 {
    if part == Part::OtherFilm { width + 1 } else { width }
}

fn serve_frames(part: Part, talk: &mut UnixStream, (clip, fresh): (&Clip, &dyn Fn() -> Option<Decoder>), mut decoder: Decoder, mut room: Room) -> i32 {
    let mut at = 0usize;
    let mut buf = Vec::new();
    loop {
        let mut ask = [0u8; 8];
        if talk.read_exact(&mut ask).is_err() {
            return 0;
        }
        let index = u32::from_le_bytes([ask[0], ask[1], ask[2], ask[3]]) as usize;
        let step = u32::from_le_bytes([ask[4], ask[5], ask[6], ask[7]]) as usize;
        let made = if let Some(said) = instead(part) {
            Some(said)
        } else if index >= clip.len() || step == 0 {
            None
        } else {
            if index != at {
                let Some(again) = begun_again(fresh, clip, index, &mut buf) else { return NO_START };
                decoder = again;
            }
            at = (index + 1) % clip.len();
            one_frame(&mut decoder, clip, index, step, &mut buf, &mut room)
        };
        let (fw, fh) = made.unwrap_or((0, 0));
        let mut answer = [0u8; 8];
        answer[..4].copy_from_slice(&fw.to_le_bytes());
        answer[4..].copy_from_slice(&fh.to_le_bytes());
        if talk.write_all(&answer).is_err() {
            return 0;
        }
    }
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    #[test]
    fn the_decoders_endings_are_its_own() {
        const _: () = assert!(NO_START != 1 && NO_LIBRARY != 1 && NO_START != NO_LIBRARY && NO_START != 0, "the decoder's endings clash");
    }
    use super::super::moving::is_moving;

    pub fn stand_in(word: &str) -> i32 {
        let words: Vec<&str> = word.split(' ').collect();
        let part = match words[..] {
            ["decode"] => Part::Decode,
            ["abort"] => Part::Abort,
            ["sleep"] => Part::Sleep,
            ["spin"] => Part::Spin,
            ["greedy"] => Part::Greedy,
            ["prisoner"] => Part::Prisoner,
            ["other-film"] => Part::OtherFilm,
            ["no-library"] => Part::NoLibrary,
            ["says", w, h] => Part::Says(w.parse().unwrap(), h.parse().unwrap()),
            _ => return 2,
        };
        child(part)
    }

    pub struct Here(Decoder);

    impl Here {
        pub fn new(clip: &Clip) -> Here {
            Here(Decoder::new(clip.codec).expect("the decoder"))
        }

        pub fn frame(&mut self, clip: &Clip, i: usize, buf: &mut Vec<u8>, out: &mut [u8], step: usize) -> bool {
            let (w, h) = (clip.info.width, clip.info.height);
            clip.read(i, buf) && self.0.decode(buf, (w, h), |img| to_xrgb(img, out, (w as usize / step) * 4, step)) == Some(true)
        }
    }

    pub(super) fn instead(part: Part) -> Option<(u32, u32)> {
        match part {
            Part::Decode | Part::OtherFilm | Part::NoLibrary => None,
            Part::Abort => std::process::abort(),
            Part::Sleep => loop {
                std::thread::sleep(Duration::from_secs(3600));
            },
            Part::Spin => {
                limit(FRAME_CPU);
                let mut round = 0u64;
                loop {
                    round = std::hint::black_box(round.wrapping_add(1));
                }
            }
            Part::Greedy => {
                let mut kept: Vec<Vec<u8>> = Vec::new();
                for _ in 0..64 {
                    let mut more: Vec<u8> = Vec::new();
                    if more.try_reserve_exact(64 << 20).is_err() {
                        std::process::abort();
                    }
                    kept.push(more);
                }
                Some((1, 1))
            }
            Part::Prisoner => {
                let named = crate::common::files::open_file("/usr/bin/bwrap").is_ok() || std::fs::read_dir("/usr").is_ok();
                let made = std::fs::File::create("/tmp/made-by-the-decoder").is_ok();
                let out = std::net::UdpSocket::bind("127.0.0.1:0").is_ok() || std::net::TcpStream::connect("127.0.0.1:9").is_ok();
                Some((1 + u32::from(named) + 2 * u32::from(made) + 4 * u32::from(out), 1))
            }
            Part::Says(w, h) => Some((w, h)),
        }
    }

    #[test]
    fn a_picture_that_does_not_fit_is_not_written_at_all() {
        let (w, h) = (8u32, 4u32);
        let (mut y, mut u, mut v) = (vec![128u8; 8 * 4], vec![128u8; 4 * 2], vec![128u8; 4 * 2]);
        let none = std::ptr::null_mut();
        let img = Image {
            fmt: 0, cs: 0, range: 0, w, h, bit_depth: 8, d_w: w, d_h: h, r_w: w, r_h: h, x_chroma_shift: 1, y_chroma_shift: 1,
            planes: [y.as_mut_ptr(), u.as_mut_ptr(), v.as_mut_ptr(), none], stride: [8, 4, 4, 0], bps: 12,
            user_priv: std::ptr::null_mut(), img_data: none, img_data_owner: 0, self_allocd: 0, fb_priv: std::ptr::null_mut(),
        };
        let mut whole = vec![9u8; 8 * 4 * 4];
        assert!(to_xrgb(&img, &mut whole, 8 * 4, 1));
        assert!(whole.chunks_exact(4).all(|px| px[3] == 0xFF), "every pixel is filled in");
        let mut short = vec![9u8; 8 * 4 * 4 - 1];
        assert!(!to_xrgb(&img, &mut short, 8 * 4, 1));
        assert!(!to_xrgb(&img, &mut short, 8 * 4 - 1, 1));
        assert!(!to_xrgb(&img, &mut [], 8 * 4, 1));
        assert!(short.iter().all(|b| *b == 9), "part of a picture was written");
        let mut small = vec![9u8; 4 * 2 * 4];
        assert!(to_xrgb(&img, &mut small, 4 * 4, 2) && small.chunks_exact(4).all(|px| px[3] == 0xFF));
    }

    pub fn vp8_key(w: u16, h: u16) -> Vec<u8> {
        let mut frame = vec![0x10, 0x02, 0x00, 0x9d, 0x01, 0x2a];
        frame.extend(w.to_le_bytes());
        frame.extend(h.to_le_bytes());
        frame.extend([0u8; 32]);
        frame
    }

    fn bits(parts: &[(u32, usize)]) -> Vec<u8> {
        let mut out = vec![0u8; 64];
        let mut at = 0;
        for &(value, count) in parts {
            for i in (0..count).rev() {
                out[at / 8] |= (((value >> i) & 1) as u8) << (7 - at % 8);
                at += 1;
            }
        }
        out
    }

    fn vp9_key(w: u32, h: u32) -> Vec<u8> {
        bits(&[(2, 2), (0, 2), (0, 1), (0, 1), (1, 1), (0, 1), (0x49_83_42, 24), (2, 3), (0, 1), (w - 1, 16), (h - 1, 16)])
    }

    #[test]
    fn a_frame_larger_than_its_file_said_is_never_handed_to_the_decoder() {
        assert_eq!(said_size(Codec::Vp8, &vp8_key(64, 48)), Ok(Some((64, 48))));
        assert_eq!(said_size(Codec::Vp8, &vp8_key(16383, 16383)), Ok(Some((16383, 16383))));
        assert_eq!(said_size(Codec::Vp8, &vp8_key(0xC000 | 64, 48)), Ok(Some((64, 48))));
        assert_eq!(said_size(Codec::Vp8, &[0x11, 0, 0, 1, 2, 3]), Ok(None));
        assert!(said_size(Codec::Vp8, &[0x10, 0]).is_err() && said_size(Codec::Vp8, &[0x10, 2, 0, 1, 2, 3, 0, 0, 0, 0]).is_err());
        assert!(fits(Codec::Vp8, &vp8_key(64, 48), (64, 48)));
        for (w, h) in [(16383, 16383), (65, 48), (64, 49)] {
            assert!(!fits(Codec::Vp8, &vp8_key(w, h), (64, 48)), "a frame {w} by {h} would be decoded for a film 64 by 48");
        }
        assert!(!fits(Codec::Vp8, &[], (64, 48)) && !fits(Codec::Vp9, &[], (64, 48)) && !fits(Codec::Vp9, &[0xFF; 20], (64, 48)));
        assert_eq!(said_size(Codec::Vp9, &vp9_key(64, 48)), Ok(Some((64, 48))));
        assert!(fits(Codec::Vp9, &vp9_key(64, 48), (64, 48)) && !fits(Codec::Vp9, &vp9_key(65536, 65536), (8192, 8192)));
        assert_eq!(said_size(Codec::Vp9, &bits(&[(2, 2), (0, 2), (1, 1), (0, 3)])), Ok(None));
        let between = |found: u32, w: u32, h: u32| bits(&[(2, 2), (0, 2), (0, 1), (1, 1), (1, 1), (0, 1), (0, 2), (0, 8), (0, 12), (found, 3), (w - 1, 16), (h - 1, 16)]);
        assert_eq!(said_size(Codec::Vp9, &between(0b001, 9000, 9000)), Ok(None));
        assert_eq!(said_size(Codec::Vp9, &between(0, 9000, 9000)), Ok(Some((9000, 9000))));
        let alone = bits(&[(2, 2), (0, 2), (0, 1), (1, 1), (0, 1), (0, 1), (1, 1), (0, 2), (0x49_83_42, 24), (0, 8), (9000 - 1, 16), (7000 - 1, 16)]);
        assert_eq!(said_size(Codec::Vp9, &alone), Ok(Some((9000, 7000))));
        let (small, large) = (vp9_key(64, 48), vp9_key(60000, 60000));
        let mut piece = [small.clone(), large.clone()].concat();
        let mark = 0xc0 | 1;
        piece.extend([mark, small.len() as u8, large.len() as u8, mark]);
        assert_eq!(said_size(Codec::Vp9, &piece), Ok(Some((60000, 60000))));
        assert!(!fits(Codec::Vp9, &piece, (64, 48)));
        let hidden = [small.clone(), large.clone()].concat();
        assert_eq!(vp9_list(&hidden), None);
        assert_eq!(said_size(Codec::Vp9, &hidden), Ok(Some((64, 48))));
        let listed = [hidden.clone(), vp9_one(hidden.len()).to_vec()].concat();
        assert_eq!(vp9_list(&listed), Some((1, 4, 6)));
        assert_eq!(said_size(Codec::Vp9, &listed), Ok(Some((64, 48))));
        let mut short = small.clone();
        short.extend([mark, 200, 200, mark]);
        assert!(said_size(Codec::Vp9, &short).is_err());
    }

    #[test]
    fn every_frame_of_the_films_that_come_with_trindle_fits() {
        let folder = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("share/wallpapers");
        let mut films = 0;
        for entry in std::fs::read_dir(&folder).into_iter().flatten().flatten() {
            let path = entry.path();
            if !is_moving(&path) {
                continue;
            }
            films += 1;
            let clip = Clip::open(&path).expect("a film");
            let most = (clip.info.width, clip.info.height);
            let mut buf = Vec::new();
            let mut gave = 0;
            for i in 0..clip.len() {
                assert!(clip.read(i, &mut buf));
                let said = said_size(clip.codec, &buf);
                assert!(said.is_ok(), "{}: frame {i} could not be read", path.display());
                gave += usize::from(matches!(said, Ok(Some(_))));
                assert!(fits(clip.codec, &buf, most), "{}: frame {i} says {said:?}, the file {most:?}", path.display());
            }
            assert_eq!(said_size(clip.codec, &{ clip.read(0, &mut buf); buf.clone() }), Ok(Some(most)), "{}", path.display());
            assert!(gave >= 1);
        }
        assert!(films >= 4, "the films were not found in {}", folder.display());
    }
}
