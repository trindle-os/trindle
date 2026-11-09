//! Tests of the PipeWire source.

use std::ffi::{c_int, c_void, CString};
use std::rc::Rc;
use std::sync::OnceLock;
use std::time::Duration;

use super::connection::PipeWire;
use super::format::{chosen, linear_first, parse_format, read_pod, tries, word_at, VideoFormat};
use super::native::{properties, Data, Lib, MetaHeader, MetaRegion, PwBuffer, SpaBuffer, SpaChunk, SpaData, SpaHook, SpaMeta, StreamEvents};
use super::pod::{buffers, enum_format, enum_format_card, metas, write_pointer, BITMAP_SIZE, BUFFERS_SIZE, CHOICE_ENUM, CHOICE_RANGE, CURSOR_SIZE, DATA_DMABUF, FORMAT_MEDIA_SUBTYPE, FORMAT_MEDIA_TYPE, FORMAT_VIDEO_FORMAT, FORMAT_VIDEO_FRAMERATE, FORMAT_VIDEO_MODIFIER, FORMAT_VIDEO_SIZE, HEADER_SIZE, MEDIA_SUBTYPE_RAW, MEDIA_TYPE_VIDEO, META_CURSOR, MOD_LINEAR, MetaBitmap, MetaCursor, OBJECT_FORMAT, OBJECT_PARAM_BUFFERS, PARAM_ENUM_FORMAT, PARAM_FORMAT, POD_FRACTION, POD_ID, POD_INT, POD_OBJECT, POD_RECTANGLE, PROP_DONT_FIXATE, PROP_MANDATORY, Pod, PointerMeta, REGION_SIZE, VIDEO_BGRA, VIDEO_BGRX};
use super::stream::{fill_damage, offered, Card, Stream};

#[test]
fn a_format_written_reads_back() {
    let pod = enum_format(1920, 1080);
    assert_eq!(pod.bytes().len() % 8, 0);
    let got = parse_format(pod.bytes()).expect("parsed");
    assert_eq!(got, (VideoFormat { width: 1920, height: 1080, modifier: None, rate: (60, 1) }, None));

    let pod = enum_format_card(64, 32, &[7, 0], false);
    let got = parse_format(pod.bytes()).expect("parsed");
    assert_eq!(got, (VideoFormat { width: 64, height: 32, modifier: Some(7), rate: (60, 1) }, Some(vec![7, 0])));
    let pod = enum_format_card(64, 32, &[0], true);
    assert_eq!(parse_format(pod.bytes()).expect("parsed").1, None);
    assert_eq!(parse_format(pod.bytes()).expect("parsed").0.modifier, Some(0));
}

const AMD_DCC: u64 = 0x0200_0000_1046_bb04;

#[test]
fn the_plain_layout_comes_first_and_is_tried_first() {
    assert_eq!(linear_first(&[AMD_DCC, 5, MOD_LINEAR]), vec![MOD_LINEAR, AMD_DCC, 5]);
    assert_eq!(linear_first(&[AMD_DCC, 5]), vec![AMD_DCC, 5]);
    assert_eq!(tries(&[AMD_DCC, MOD_LINEAR, 5]), vec![vec![MOD_LINEAR], vec![AMD_DCC, 5]]);
    assert_eq!(tries(&[MOD_LINEAR]), vec![vec![MOD_LINEAR]]);
    assert_eq!(tries(&[AMD_DCC]), vec![vec![AMD_DCC]]);
    assert!(tries(&[]).is_empty());
}

#[test]
fn a_reader_that_turns_the_layout_down_has_others_then_shared_memory() {

    let pods = offered(2235, 1482, Some(&[AMD_DCC, MOD_LINEAR]), None);
    let read: Vec<_> = pods.iter().map(|p| parse_format(p.bytes()).expect("parsed")).collect();
    assert_eq!(read.len(), 2);
    assert_eq!(read[0].0.modifier, Some(MOD_LINEAR));
    assert_eq!(read[0].1, Some(vec![MOD_LINEAR, AMD_DCC]));
    assert_eq!((read[1].0.modifier, read[1].0.width), (None, 2235));

    let pods = offered(2235, 1482, Some(&[AMD_DCC, MOD_LINEAR]), Some(AMD_DCC));
    let read: Vec<_> = pods.iter().map(|p| parse_format(p.bytes()).expect("parsed")).collect();
    assert_eq!(read.len(), 3);
    assert_eq!((read[0].0.modifier, read[0].1.clone()), (Some(AMD_DCC), None));
    assert_eq!(read[1].1, Some(vec![MOD_LINEAR, AMD_DCC]));
    assert_eq!(read[2].0.modifier, None);

    let pods = offered(2235, 1482, None, None);
    assert_eq!(pods.len(), 1);
    assert_eq!(parse_format(pods[0].bytes()).expect("parsed").0.modifier, None);
}

#[test]
fn pictures_go_no_faster_than_the_rate_agreed() {
    let at = |rate| VideoFormat { width: 1, height: 1, modifier: None, rate }.frame_gap();
    assert_eq!(at((60, 1)), Duration::from_secs(1) / 60);
    assert_eq!(at((30, 1)), Duration::from_secs(1) / 30);
    assert_eq!(at((0, 1)), Duration::from_secs(1) / 60, "none named");
    assert_eq!(at((1000, 1)), Duration::from_secs(1) / 360, "never faster than 360");
    assert_eq!(at((1, 5)), Duration::from_secs(1), "never slower than one a second");

    let pod = Pod::object(OBJECT_FORMAT, PARAM_FORMAT, |p| {
        p.key(FORMAT_MEDIA_TYPE).id(MEDIA_TYPE_VIDEO);
        p.key(FORMAT_MEDIA_SUBTYPE).id(MEDIA_SUBTYPE_RAW);
        p.key(FORMAT_VIDEO_FORMAT).id(VIDEO_BGRX);
        p.key(FORMAT_VIDEO_SIZE).rectangle(2235, 1482);
        p.key(FORMAT_VIDEO_FRAMERATE).fraction(30, 1);
    });
    assert_eq!(parse_format(pod.bytes()).expect("parsed").0.rate, (30, 1));
}

#[test]
fn buffers_ask_for_whole_rows_in_shared_memory() {
    let pod = buffers(&VideoFormat { width: 100, height: 50, modifier: None, rate: (60, 1) });
    let (kind, body, total) = read_pod(pod.bytes()).expect("pod");
    assert_eq!((kind, total), (POD_OBJECT, pod.bytes().len()));
    assert_eq!(word_at(body, 0), Some(OBJECT_PARAM_BUFFERS));

    let words: Vec<u32> = body.chunks(4).map(|c| u32::from_ne_bytes(c.try_into().unwrap())).collect();
    let at = words.windows(2).position(|w| w == [BUFFERS_SIZE, 0]).expect("size");
    assert_eq!(&words[at + 2..at + 5], &[4, POD_INT, 20000]);
}

#[test]
fn a_choice_reads_as_its_first_value() {
    let mut pod = Pod::default();
    pod.choice(CHOICE_ENUM, POD_ID, 4, &[&[VIDEO_BGRA], &[VIDEO_BGRX], &[VIDEO_BGRA]]);
    let (kind, body, _) = read_pod(pod.bytes()).unwrap();
    assert_eq!(chosen(kind, body), Some((POD_ID, &VIDEO_BGRA.to_ne_bytes()[..])));
}

#[test]
fn damage_fits_the_room_it_has() {
    let mut room: Vec<MetaRegion> = (0..3).map(|_| MetaRegion { x: 9, y: 9, w: 9, h: 9 }).collect();
    fill_damage(&mut room, &[(1, 2, 3, 4)], 100, 50);
    assert_eq!((room[0].x, room[0].y, room[0].w, room[0].h), (1, 2, 3, 4));
    assert_eq!(room[1].w, 0, "the list ends with an empty one");
    fill_damage(&mut room, &[(0, 0, 1, 1), (10, 10, 1, 1), (20, 5, 5, 5)], 100, 50);
    assert_eq!((room[0].x, room[0].y, room[0].w, room[0].h), (0, 0, 25, 11), "too many are merged");
    fill_damage(&mut room, &[], 100, 50);
    assert_eq!((room[0].w, room[0].h), (100, 50), "nothing named means all of it");
}

#[test]
fn the_pointer_goes_beside_the_picture() {
    let mut room = vec![0u32; (CURSOR_SIZE + BITMAP_SIZE + 2 * 2 * 4) / 4];
    let picture = [7u8; 16];
    let p = PointerMeta { over: true, at: (100, 50), hotspot: (1, 1), picture: &picture, size: (2, 2) };
    // SAFETY: room is a Vec of four-byte words, its whole length given.
    unsafe { write_pointer(room.as_mut_ptr() as *mut u8, room.len() * 4, &p) };

    assert_eq!(&room[..12], &[1, 0, 100, 50, 1, 1, 28, VIDEO_BGRA, 2, 2, 8, 20]);
    assert_eq!(room[12], u32::from_ne_bytes([7; 4]));

    let p = PointerMeta { over: false, ..p };
    // SAFETY: as above.
    unsafe { write_pointer(room.as_mut_ptr() as *mut u8, room.len() * 4, &p) };
    assert_eq!(&room[..13], &[1, 0, 100, 50, 0, 0, 28, VIDEO_BGRA, 1, 1, 4, 20, 0]);
    assert_eq!(metas(true).len(), 3);
    assert_eq!(metas(false).len(), 2);
}

#[test]
fn struct_layouts_match_pipewire() {
    assert_eq!(std::mem::size_of::<MetaCursor>(), CURSOR_SIZE);
    assert_eq!(std::mem::size_of::<MetaBitmap>(), BITMAP_SIZE);
    assert_eq!(std::mem::size_of::<SpaHook>(), 48);
    assert_eq!(std::mem::size_of::<SpaData>(), 40);
    assert_eq!(std::mem::size_of::<SpaChunk>(), 16);
    assert_eq!(std::mem::size_of::<SpaBuffer>(), 24);
    assert_eq!(std::mem::size_of::<SpaMeta>(), 16);
    assert_eq!(std::mem::size_of::<MetaHeader>(), HEADER_SIZE);
    assert_eq!(std::mem::size_of::<MetaRegion>(), REGION_SIZE);
    assert_eq!(std::mem::size_of::<PwBuffer>(), 40);
    assert_eq!(std::mem::size_of::<StreamEvents>(), 8 + 11 * 8);
}

#[derive(Default)]
struct Seen {
    frames: u32,
    bytes: u64,
    lit: u64,
    format: Option<VideoFormat>,
    last: Vec<u8>,
    stride: u32,

    card: bool,

    pointer_only: u32,
    pointer: Option<(i32, i32, i32, i32, u32, u32)>,

    metas: Vec<u32>,
}

unsafe extern "C" fn reader_param(data: Data, id: u32, param: *const u8) {
    if id != PARAM_FORMAT || param.is_null() {
        return;
    }
    // SAFETY: data is the Seen given with the listener; param a spa_pod.
    unsafe {
        let size = std::ptr::read_unaligned(param as *const u32) as usize;
        (*(data as *mut Seen)).format = parse_format(std::slice::from_raw_parts(param, 8 + size)).map(|f| f.0);
    }

    let Some((lib, raw)) = READER.get() else { return };
    let pods = metas(true);
    let params: Vec<*const u8> = pods.iter().map(|p| p.bytes().as_ptr()).collect();
    // SAFETY: the live stream, with PODs alive for the call.
    unsafe { (lib.stream_update_params)(*raw as *mut c_void, params.as_ptr(), params.len() as u32) };
}

static READER: OnceLock<(&'static Lib, usize)> = OnceLock::new();

unsafe extern "C" fn reader_process(data: Data) {
    let Some((lib, raw)) = READER.get() else { return };
    // SAFETY: a capture stream's process: dequeued buffers are mapped

    unsafe {
        let seen = &mut *(data as *mut Seen);
        let stream = *raw as *mut c_void;
        loop {
            let b = (lib.stream_dequeue_buffer)(stream);
            if b.is_null() {
                break;
            }
            let spa = &*(*b).buffer;
            for i in 0..spa.n_metas as usize {
                let meta = &*spa.metas.add(i);
                if !seen.metas.contains(&meta.kind) {
                    seen.metas.push(meta.kind);
                }
                if meta.kind == META_CURSOR && meta.size as usize >= CURSOR_SIZE + BITMAP_SIZE {
                    let c = std::ptr::read_unaligned(meta.data as *const MetaCursor);
                    if c.id != 0 {
                        let bm = std::ptr::read_unaligned((meta.data as *const u8).add(c.bitmap_offset as usize) as *const MetaBitmap);
                        seen.pointer = Some((c.x, c.y, c.hot_x, c.hot_y, bm.width, bm.height));
                        if spa.n_datas >= 1 && (*(*spa.datas).chunk).size == 0 {
                            seen.pointer_only += 1;
                        }
                    }
                }
            }
            if spa.n_datas >= 1 {
                let d = &*spa.datas;
                let chunk = &*d.chunk;

                let len = chunk.offset as usize + chunk.size as usize;
                let mapped = if d.kind == DATA_DMABUF && d.fd >= 0 && chunk.size > 0 {
                    seen.card = true;
                    let m = libc::mmap(std::ptr::null_mut(), len, libc::PROT_READ, libc::MAP_SHARED, d.fd as c_int, 0);
                    (m != libc::MAP_FAILED).then_some(m)
                } else {
                    None
                };
                let base = mapped.unwrap_or(d.data) as *const u8;
                if !base.is_null() && chunk.size > 0 {
                    let bytes = std::slice::from_raw_parts(base.add(chunk.offset as usize), chunk.size as usize);
                    seen.frames += 1;
                    seen.bytes += bytes.len() as u64;
                    seen.lit += bytes.iter().step_by(4).filter(|&&b| b != 0).count() as u64;
                    seen.last = bytes.to_vec();
                    seen.stride = chunk.stride.max(0) as u32;
                }
                if let Some(m) = mapped {
                    libc::munmap(m, len);
                }
            }
            (lib.stream_queue_buffer)(stream, b);
        }
    }
}

static READER_EVENTS: StreamEvents = StreamEvents {
    version: 2,
    destroy: None,
    state_changed: None,
    control_info: std::ptr::null(),
    io_changed: std::ptr::null(),
    param_changed: Some(reader_param),
    add_buffer: None,
    remove_buffer: None,
    process: Some(reader_process),
    drained: std::ptr::null(),
    command: std::ptr::null(),
    trigger_done: std::ptr::null(),
};

fn reader_format() -> Pod {
    Pod::object(OBJECT_FORMAT, PARAM_ENUM_FORMAT, |p| {
        p.key(FORMAT_MEDIA_TYPE).id(MEDIA_TYPE_VIDEO);
        p.key(FORMAT_MEDIA_SUBTYPE).id(MEDIA_SUBTYPE_RAW);
        p.key(FORMAT_VIDEO_FORMAT).choice(CHOICE_ENUM, POD_ID, 4, &[&[VIDEO_BGRX], &[VIDEO_BGRX], &[VIDEO_BGRA]]);

        if std::env::var_os("TRINDLE_PW_READ_CARD").is_some() {
            p.key_with(FORMAT_VIDEO_MODIFIER, PROP_MANDATORY | PROP_DONT_FIXATE).choice_long(&[0]);
        }
        p.key(FORMAT_VIDEO_SIZE).choice(CHOICE_RANGE, POD_RECTANGLE, 8, &[&[640, 480], &[1, 1], &[16384, 16384]]);
        p.key(FORMAT_VIDEO_FRAMERATE).choice(CHOICE_RANGE, POD_FRACTION, 8, &[&[30, 1], &[0, 1], &[360, 1]]);
    })
}

fn save_the_last_frame(seen: &Seen) {
    if let (Ok(path), Some(f)) = (std::env::var("TRINDLE_PW_READ_PNG"), seen.format) {
        let mut rgba = Vec::with_capacity((f.width * f.height * 4) as usize);
        for y in 0..f.height as usize {
            for x in 0..f.width as usize {
                let at = y * seen.stride as usize + x * 4;
                if let Some(px) = seen.last.get(at..at + 4) {
                    rgba.extend_from_slice(&[px[2], px[1], px[0], 255]);
                }
            }
        }
        if let Some(img) = image::RgbaImage::from_raw(f.width, f.height, rgba) {
            img.save(&path).expect("saved");
            println!("saved {path}");
        }
    }
}

#[test]
#[ignore = "needs a real screen and PipeWire running: run by hand"]
fn a_reader_gets_frames() {
    let secs: u64 = std::env::var("TRINDLE_PW_READ_SECS").ok().and_then(|s| s.parse().ok()).unwrap_or(10);
    let pw = PipeWire::connect("Trindle test reader").expect("PipeWire");
    let lib = pw.lib;
    let props = properties(lib, &[("media.class", "Stream/Input/Video"), ("node.name", "trindle-test-reader")]);
    let name = CString::new("trindle-test-reader").unwrap();
    // SAFETY: as in Stream::new: a stream on the live core, a static

    let seen = Box::into_raw(Box::new(Seen::default()));
    // SAFETY: an all-zero spa_hook is the unused hook PipeWire expects.
    let mut hook: Box<SpaHook> = Box::new(unsafe { std::mem::zeroed() });
    // SAFETY: a stream on the live core; the properties are handed over.
    let raw = unsafe { (lib.stream_new)(pw.core, name.as_ptr(), props) };
    assert!(!raw.is_null());
    let _ = READER.set((lib, raw as usize));
    // SAFETY: the events table is static and `seen` outlives the stream.
    unsafe { (lib.stream_add_listener)(raw, &mut *hook, &READER_EVENTS, seen as Data) };
    let format = reader_format();
    let params = [format.bytes().as_ptr()];
    const MAP_BUFFERS: c_int = 1 << 2;
    // SAFETY: input (0), to no particular node, the POD alive for the call.
    let res = unsafe { (lib.stream_connect)(raw, 0, u32::MAX, MAP_BUFFERS, params.as_ptr(), 1) };
    assert!(res >= 0);
    let start = std::time::Instant::now();
    let mut told = false;
    while start.elapsed() < std::time::Duration::from_secs(secs) {
        let mut fds = libc::pollfd { fd: pw.fd, events: libc::POLLIN, revents: 0 };
        // SAFETY: one pollfd this owns.
        unsafe { libc::poll(&mut fds, 1, 50) };
        pw.dispatch();
        if !told {
            // SAFETY: the live stream.
            let node = unsafe { (lib.stream_get_node_id)(raw) };
            if node != u32::MAX {
                println!("reader node {node}");
                told = true;
            }
        }
    }
    // SAFETY: the stream goes first, then the box its callbacks reached.
    let seen = unsafe {
        (lib.stream_destroy)(raw);
        Box::from_raw(seen)
    };
    println!("format {:?}", seen.format);
    println!("frames {} bytes {} lit {} card {}", seen.frames, seen.bytes, seen.lit, seen.card);
    println!("pointer-only frames {} last pointer {:?} metas {:?}", seen.pointer_only, seen.pointer, seen.metas);
    save_the_last_frame(&seen);
    assert!(seen.frames > 0, "no frame came");
}

#[derive(Default)]
struct Turned {
    frames: Vec<(Option<u64>, u32, u32)>,
}

static CHOOSY: OnceLock<(&'static Lib, usize)> = OnceLock::new();

unsafe extern "C" fn choosy_param(data: Data, id: u32, param: *const u8) {
    if id != PARAM_FORMAT || param.is_null() {
        return;
    }
    // SAFETY: data is the Turned given with the listener; param a spa_pod.
    unsafe {
        let size = std::ptr::read_unaligned(param as *const u32) as usize;
        if let Some((f, _)) = parse_format(std::slice::from_raw_parts(param, 8 + size)) {
            (*(data as *mut Turned)).frames.push((f.modifier, 0, 0));
        }
    }
}

unsafe extern "C" fn choosy_process(data: Data) {
    let Some((lib, raw)) = CHOOSY.get() else { return };
    // SAFETY: a capture stream's process: buffers dequeued are its own

    unsafe {
        let seen = &mut *(data as *mut Turned);
        loop {
            let b = (lib.stream_dequeue_buffer)(*raw as *mut c_void);
            if b.is_null() {
                break;
            }
            let spa = &*(*b).buffer;
            if let Some(last) = seen.frames.last_mut() {
                if spa.n_datas >= 1 && (*(*spa.datas).chunk).size > 0 {
                    match (*spa.datas).kind {
                        DATA_DMABUF => last.1 += 1,
                        _ => last.2 += 1,
                    }
                }
            }
            (lib.stream_queue_buffer)(*raw as *mut c_void, b);
        }
    }
}

static CHOOSY_EVENTS: StreamEvents = StreamEvents {
    version: 2,
    destroy: None,
    state_changed: None,
    control_info: std::ptr::null(),
    io_changed: std::ptr::null(),
    param_changed: Some(choosy_param),
    add_buffer: None,
    remove_buffer: None,
    process: Some(choosy_process),
    drained: std::ptr::null(),
    command: std::ptr::null(),
    trigger_done: std::ptr::null(),
};

fn browser_formats(modifiers: &[u64]) -> Vec<Pod> {
    let one = |card: Option<&[u64]>| {
        Pod::object(OBJECT_FORMAT, PARAM_ENUM_FORMAT, |p| {
            p.key(FORMAT_MEDIA_TYPE).id(MEDIA_TYPE_VIDEO);
            p.key(FORMAT_MEDIA_SUBTYPE).id(MEDIA_SUBTYPE_RAW);
            p.key(FORMAT_VIDEO_FORMAT).id(VIDEO_BGRX);
            if let Some(m) = card {
                p.key_with(FORMAT_VIDEO_MODIFIER, PROP_MANDATORY | PROP_DONT_FIXATE).choice_long(m);
            }
            p.key(FORMAT_VIDEO_SIZE).choice(CHOICE_RANGE, POD_RECTANGLE, 8, &[&[640, 480], &[1, 1], &[16384, 16384]]);
            p.key(FORMAT_VIDEO_FRAMERATE).choice(CHOICE_RANGE, POD_FRACTION, 8, &[&[30, 1], &[0, 1], &[360, 1]]);
        })
    };
    let mut out = Vec::new();
    if !modifiers.is_empty() {
        out.push(one(Some(modifiers)));
    }
    out.push(one(None));
    out
}

fn choosy_stream(pw: &PipeWire, lib: &'static Lib, seen: *mut Turned) -> (*mut c_void, Box<SpaHook>) {
    let props = properties(lib, &[("media.class", "Stream/Input/Video"), ("node.name", "trindle-test-choosy")]);
    let name = CString::new("trindle-test-choosy").unwrap();
    // SAFETY: an all-zero spa_hook is the unused hook PipeWire expects.
    let mut hook: Box<SpaHook> = Box::new(unsafe { std::mem::zeroed() });
    // SAFETY: a stream on the live core, a static events table, and a

    let raw = unsafe { (lib.stream_new)(pw.core, name.as_ptr(), props) };
    assert!(!raw.is_null());
    let _ = CHOOSY.set((lib, raw as usize));
    // SAFETY: the events table is static and `seen` outlives the stream.
    unsafe { (lib.stream_add_listener)(raw, &mut *hook, &CHOOSY_EVENTS, seen as Data) };
    (raw, hook)
}

fn offer_only(lib: &Lib, raw: *mut c_void, left: &[u64]) {
    let pods = browser_formats(left);
    let params: Vec<*const u8> = pods.iter().map(|p| p.bytes().as_ptr()).collect();
    // SAFETY: the live stream, the PODs alive for the call.
    unsafe { (lib.stream_update_params)(raw, params.as_ptr(), params.len() as u32) };
}

fn the_formats_walked_down(seen: &Turned, stale: i32) {
    println!("buffers of an earlier round kept back: {stale}");
    println!("formats settled and pictures read (layout, on the card, in shared memory): {:x?}", seen.frames);
    let with_pictures: Vec<Option<u64>> = seen.frames.iter().filter(|f| f.1 + f.2 > 0).map(|f| f.0).collect();
    assert_eq!(with_pictures, vec![Some(MOD_LINEAR), Some(AMD_DCC), None], "plain, then compressed, then shared memory");
    let last = seen.frames.iter().rev().find(|f| f.1 + f.2 > 0).copied().unwrap_or_default();
    assert!(last.2 > 0 && last.1 == 0, "shared memory at the end");
}

#[test]
#[ignore = "needs a real graphics card and PipeWire running: run by hand"]
fn a_layout_turned_down_gives_way_to_the_next_then_shared_memory() {
    use std::os::unix::fs::MetadataExt;
    use std::time::Instant;
    let card = std::env::var("TRINDLE_PW_CARD").unwrap_or_else(|_| "/dev/dri/renderD128".into());
    let rdev = std::fs::metadata(&card).expect("the card").rdev();
    let device = Rc::new(super::super::gbm::Device::open(rdev).expect("gbm"));
    let (w, h) = (2235, 1482);
    assert!(device.alloc(w, h, super::super::gbm::XRGB8888, &[AMD_DCC]).is_some(), "the card makes the compressed layout");
    let pw = PipeWire::connect("Trindle test").expect("PipeWire");
    let lib = pw.lib;
    let sender = Stream::new(&pw, "trindle-screencast", &[("media.class", "Video/Source"), ("node.name", "trindle-test-sender")],
                             w, h, Some(Card { device, modifiers: vec![AMD_DCC, MOD_LINEAR] }), false).expect("sender");
    let seen = Box::into_raw(Box::new(Turned::default()));
    let (raw, _hook) = choosy_stream(&pw, lib, seen);
    let pods = browser_formats(&[AMD_DCC, MOD_LINEAR]);
    let params: Vec<*const u8> = pods.iter().map(|p| p.bytes().as_ptr()).collect();
    // SAFETY: input (0), to no particular node, the PODs alive for the call.
    assert!(unsafe { (lib.stream_connect)(raw, 0, u32::MAX, 0, params.as_ptr(), params.len() as u32) } >= 0);
    let start = Instant::now();
    let (mut linked, mut phase, mut phase_at, mut seq, mut stale) = (false, 0, Instant::now(), 0, 0);
    let mut link: Option<std::process::Child> = None;
    while start.elapsed() < Duration::from_secs(10) {
        let mut fds = libc::pollfd { fd: pw.fd, events: libc::POLLIN, revents: 0 };
        // SAFETY: one pollfd this owns.
        unsafe { libc::poll(&mut fds, 1, 10) };
        pw.dispatch();
        let _ = sender.take_events();
        if !linked && start.elapsed() > Duration::from_millis(300) {

            link = std::process::Command::new("pw-link").args(["trindle-test-sender", "trindle-test-choosy"]).spawn().ok();
            linked = true;
            phase_at = Instant::now();
        }

        if let Some(b) = sender.dequeue() {
            seq += 1;
            if !sender.queue(b, &[], 0, seq, None, false) {
                stale += 1;
            }
        }
        std::thread::sleep(Duration::from_millis(20));

        // SAFETY: the box lives until the end; callbacks run only in dispatch.
        let settled = unsafe { (*seen).frames.last().and_then(|f| f.0) };
        if linked && phase < 2 && phase_at.elapsed() > Duration::from_secs(3) {
            let left: Vec<u64> = if phase == 0 { vec![AMD_DCC] } else { Vec::new() };
            println!("turning down {settled:x?}, leaving {left:x?}");
            offer_only(lib, raw, &left);
            phase += 1;
            phase_at = Instant::now();
        }
    }
    if let Some(mut link) = link {
        let _ = link.kill();
        let _ = link.wait();
    }
    // SAFETY: the reader goes first, then the box its callbacks reached.
    let seen = unsafe {
        (lib.stream_destroy)(raw);
        Box::from_raw(seen)
    };
    drop(sender);
    the_formats_walked_down(&seen, stale);
}
