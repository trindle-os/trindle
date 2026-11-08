//! Moving wallpapers: WebM files (VP8 or VP9) played by Trindle itself.

use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::fs::FileExt;
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::{mpsc, OnceLock};
use std::time::{Duration, Instant};

pub fn is_moving(path: &Path) -> bool {
    path.extension().is_some_and(|x| x.eq_ignore_ascii_case("webm"))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Info {
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub bytes: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Codec {
    Vp8,
    Vp9,
}

const MOST_FRAME: u64 = 64 << 20;

const MOST_FRAMES: usize = 1 << 20;

const MOST_PARTS: usize = 4 * MOST_FRAMES;

const LEAST_RATE: f64 = 1.0;
const MOST_RATE: f64 = 120.0;

const MOST_PASSED: usize = 4;

const PIECE: usize = 64 * 1024;

pub struct Clip {
    file: File,
    pub(super) codec: Codec,
    frames: Vec<(u64, u32)>,
    pub info: Info,

    #[cfg(test)]
    looks: usize,
}

struct Pieces<'a> {
    file: &'a File,
    from: u64,
    held: Vec<u8>,
    piece: usize,
    #[cfg(test)]
    looks: usize,
}

impl<'a> Pieces<'a> {
    fn new(file: &'a File, piece: usize) -> Pieces<'a> {
        Pieces {
            file,
            from: 0,
            held: Vec::new(),
            piece: piece.max(16),
            #[cfg(test)]
            looks: 0,
        }
    }

    fn bytes(&mut self, at: u64, len: usize) -> Option<&[u8]> {
        let end = at.checked_add(len as u64)?;
        if at < self.from || end > self.from + self.held.len() as u64 {
            self.held.resize(self.piece.max(len), 0);
            let mut got = 0;
            while got < self.held.len() {
                match self.file.read_at(&mut self.held[got..], at + got as u64) {
                    Ok(0) => break,
                    Ok(n) => got += n,
                    Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                    Err(_) => return None,
                }
            }
            #[cfg(test)]
            {
                self.looks += 1;
            }
            self.held.truncate(got);
            self.from = at;
            if got < len {
                return None;
            }
        }
        let start = (at - self.from) as usize;
        self.held.get(start..start + len)
    }
}

const SEGMENT: u64 = 0x1853_8067;
const INFO: u64 = 0x1549_A966;
const TIMECODE_SCALE: u64 = 0x2A_D7B1;
const TRACKS: u64 = 0x1654_AE6B;
const TRACK_ENTRY: u64 = 0xAE;
const TRACK_NUMBER: u64 = 0xD7;
const TRACK_TYPE: u64 = 0x83;
const CODEC_ID: u64 = 0x86;
const DEFAULT_DURATION: u64 = 0x23_E383;
const VIDEO: u64 = 0xE0;
const PIXEL_WIDTH: u64 = 0xB0;
const PIXEL_HEIGHT: u64 = 0xBA;
const CLUSTER: u64 = 0x1F43_B675;
const TIMECODE: u64 = 0xE7;
const SIMPLE_BLOCK: u64 = 0xA3;
const BLOCK_GROUP: u64 = 0xA0;
const BLOCK: u64 = 0xA1;

const CONTAINERS: &[u64] = &[SEGMENT, INFO, TRACKS, TRACK_ENTRY, VIDEO, CLUSTER, BLOCK_GROUP];

fn vint(file: &mut Pieces, at: u64, id: bool) -> Option<(Option<u64>, u64)> {
    let first = file.bytes(at, 1)?[0];
    let len = first.leading_zeros() as u64 + 1;
    if len > 8 {
        return None;
    }
    let rest = file.bytes(at.checked_add(1)?, len as usize - 1)?;
    let marker = if id { first as u64 } else { first as u64 & (0xFF >> len) };
    let mut value = marker;
    for b in rest {
        value = (value << 8) | *b as u64;
    }
    let all_ones = (1u64 << (7 * len)) - 1;
    Some((if !id && value == all_ones { None } else { Some(value) }, len))
}

fn uint(file: &mut Pieces, at: u64, len: u64) -> Option<u64> {
    if len > 8 {
        return None;
    }
    Some(file.bytes(at, len as usize)?.iter().fold(0, |v, x| (v << 8) | *x as u64))
}

#[derive(Default)]
struct Track {
    number: u64,
    kind: u64,
    codec: Option<Codec>,
    default_duration: u64,
    width: u64,
    height: u64,
}

struct Scan {
    scale: u64,
    tracks: Vec<Track>,

    video: Option<u64>,
    cluster_time: u64,
    frames: Vec<(u64, u32)>,
    times: Vec<u64>,
}

impl Scan {

    fn element(&mut self, file: &mut Pieces, (id, data, size): (u64, u64, u64), bytes: u64) -> Option<bool> {
        let track = self.tracks.last_mut();
        match id {
            TIMECODE_SCALE => self.scale = uint(file, data, size).unwrap_or(self.scale).max(1),
            TRACK_NUMBER => if let Some(t) = track { t.number = uint(file, data, size)? },
            TRACK_TYPE => if let Some(t) = track { t.kind = uint(file, data, size)? },
            DEFAULT_DURATION => if let Some(t) = track { t.default_duration = uint(file, data, size)? },
            PIXEL_WIDTH => if let Some(t) = track { t.width = uint(file, data, size)? },
            PIXEL_HEIGHT => if let Some(t) = track { t.height = uint(file, data, size)? },
            CODEC_ID => if let Some(t) = track {
                let name = file.bytes(data, size.min(32) as usize)?;
                t.codec = match name.split(|b| *b == 0).next() {
                    Some(b"V_VP8") => Some(Codec::Vp8),
                    Some(b"V_VP9") => Some(Codec::Vp9),
                    _ => None,
                };
            },
            TIMECODE => self.cluster_time = uint(file, data, size).unwrap_or(0),
            SIMPLE_BLOCK | BLOCK => return self.block(file, data, size, bytes),
            _ => {}
        }
        Some(true)
    }

    fn block(&mut self, file: &mut Pieces, data: u64, size: u64, bytes: u64) -> Option<bool> {
        if self.video.is_none() {
            self.video = self.tracks.iter().find(|t| t.kind == 1 && t.codec.is_some()).map(|t| t.number);
        }
        let (Some(want), Some((Some(number), n))) = (self.video, vint(file, data, false)) else { return Some(true) };
        let head: [u8; 3] = file.bytes(data + n, 3)?.try_into().ok()?;

        if number == want && head[2] & 0x06 == 0 && size > n + 3 {

            let len = size - n - 3;
            if len > MOST_FRAME || data.checked_add(size).is_none_or(|end| end > bytes) || self.frames.len() == MOST_FRAMES {
                return Some(false);
            }
            let rel = i16::from_be_bytes([head[0], head[1]]) as i64;
            self.frames.push((data + n + 3, len as u32));
            self.times.push((self.cluster_time as i64 + rel).max(0) as u64 * self.scale);
        }
        Some(true)
    }

    fn into_clip(self, opened: File, within: bool, bytes: u64, #[cfg(test)] looks: usize) -> Option<Clip> {
        let Scan { tracks, video, frames, times, .. } = self;
        let track = tracks.into_iter().find(|t| Some(t.number) == video)?;
        let (wide, high) = (u32::try_from(track.width).unwrap_or(u32::MAX), u32::try_from(track.height).unwrap_or(u32::MAX));
        if frames.is_empty() || track.width == 0 || track.height == 0 || (within && !super::wallpaper_limits::fits(wide, high)) {
            return None;
        }
        let fps = if track.default_duration > 0 {
            1e9 / track.default_duration as f64
        } else if times.len() > 1 {
            let span = times.iter().max()? - times.iter().min()?;
            if span == 0 { 25.0 } else { (times.len() - 1) as f64 * 1e9 / span as f64 }
        } else {
            25.0
        };
        Some(Clip {
            file: opened,
            codec: track.codec?,
            frames,
            info: Info { width: wide, height: high, fps: fps.clamp(LEAST_RATE, MOST_RATE), bytes },
            #[cfg(test)]
            looks,
        })
    }
}

impl Clip {

    pub fn open(path: &Path) -> Option<Clip> {
        Clip::open_by(path, PIECE)
    }

    fn open_by(path: &Path, piece: usize) -> Option<Clip> {
        Clip::of_by(crate::toolkit::picture::file(path).ok()?, piece, true)
    }

    pub(super) fn of(opened: File) -> Option<Clip> {
        Clip::of_by(opened, PIECE, true)
    }

    fn of_by(opened: File, piece: usize, within: bool) -> Option<Clip> {
        let about = opened.metadata().ok()?;
        if !about.is_file() {
            return None;
        }
        let bytes = about.len();
        let mut file = Pieces::new(&opened, piece);
        let mut scan = Scan { scale: 1_000_000, tracks: Vec::new(), video: None, cluster_time: 0, frames: Vec::new(), times: Vec::new() };
        let mut parts = 0usize;
        let mut at = 0u64;
        while at < bytes && parts < MOST_PARTS {
            parts += 1;
            let Some((Some(id), id_len)) = vint(&mut file, at, true) else { break };
            let Some((size, size_len)) = vint(&mut file, at + id_len, false) else { break };
            let data = at + id_len + size_len;
            if CONTAINERS.contains(&id) {
                if id == TRACK_ENTRY {
                    scan.tracks.push(Track::default());
                }
                at = data;
                continue;
            }

            let Some(size) = size else { break };
            if !scan.element(&mut file, (id, data, size), bytes)? {
                break;
            }
            at = data + size;
        }
        #[cfg(test)]
        let looks = file.looks;
        scan.into_clip(opened, within, bytes, #[cfg(test)] looks)
    }

    pub fn len(&self) -> usize {
        self.frames.len()
    }

    pub(super) fn read(&self, i: usize, buf: &mut Vec<u8>) -> bool {
        let Some(&(at, len)) = self.frames.get(i) else { return false };
        buf.resize(len as usize, 0);
        self.file.read_exact_at(buf, at).is_ok()
    }
}

pub fn measure(path: &Path) -> Option<Info> {
    Clip::of_by(crate::toolkit::picture::file(path).ok()?, PIECE, false).map(|c| c.info)
}

pub const DECODING: &str = "decode-film";

pub(super) const FILM_FD: i32 = 0;
pub(super) const TALK_FD: i32 = 3;
pub(super) const ROOM_FD: i32 = 4;

pub(super) const HELLO: [u8; 4] = *b"TFD1";

pub(super) const FRAME_CPU: Duration = Duration::from_secs(2);

const START_WAIT: Duration = Duration::from_secs(10);
const FRAME_WAIT: Duration = Duration::from_secs(5);

const MOST_ENDINGS: usize = 2;

const MOST_CAUGHT_UP: u64 = 1 << 28;

const ROOM_FOR_PROGRAM: u64 = 384 << 20;
const ROOM_A_PIXEL: u64 = 32;

fn goes_on_from(at: usize, info: &Info) -> usize {
    if at as u64 * u64::from(info.width) * u64::from(info.height) > MOST_CAUGHT_UP { 0 } else { at }
}

fn ceiling(info: &Info) -> u64 {
    ROOM_FOR_PROGRAM + ROOM_A_PIXEL * u64::from(info.width) * u64::from(info.height)
}

pub(super) struct Room {
    fd: OwnedFd,
    map: *mut u8,
    len: usize,
}

// SAFETY: the mapping belongs to this value alone in this program and is

unsafe impl Send for Room {}

impl Room {

    fn fixed(len: usize) -> Option<Room> {
        if len == 0 {
            return None;
        }
        // SAFETY: a plain call with a name that ends in a nought; it gives

        let fd = unsafe { libc::memfd_create(c"trindle-film".as_ptr(), libc::MFD_CLOEXEC | libc::MFD_ALLOW_SEALING) };
        if fd < 0 {
            return None;
        }
        // SAFETY: the descriptor just made, which nothing else owns.
        let fd = unsafe { OwnedFd::from_raw_fd(fd) };
        let seals = libc::F_SEAL_SHRINK | libc::F_SEAL_GROW | libc::F_SEAL_SEAL;
        // SAFETY: both act on that descriptor alone: its length is set,

        let set = unsafe { libc::ftruncate(fd.as_raw_fd(), len as libc::off_t) == 0 && libc::fcntl(fd.as_raw_fd(), libc::F_ADD_SEALS, seals) == 0 };
        if !set {
            return None;
        }
        Room::mapped(fd, len)
    }

    pub(super) fn of(fd: OwnedFd, len: usize) -> Option<Room> {
        let held = File::from(fd);
        if len == 0 || held.metadata().ok()?.len() < len as u64 {
            return None;
        }
        Room::mapped(OwnedFd::from(held), len)
    }

    fn mapped(fd: OwnedFd, len: usize) -> Option<Room> {
        // SAFETY: `len` bytes of a memory file at least that long, through

        let map = unsafe { libc::mmap(std::ptr::null_mut(), len, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd.as_raw_fd(), 0) };
        (map != libc::MAP_FAILED).then_some(Room { fd, map: map as *mut u8, len })
    }

    pub(super) fn bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: `len` bytes mapped to be read and written, alive until

        unsafe { std::slice::from_raw_parts_mut(self.map, self.len) }
    }

    fn rows(&self, across: usize, (w, h): (usize, usize), out: &mut [u8], stride: usize) -> bool {
        let wide = w * 4;
        let (Some(from), Some(to)) = (span(across, wide, h), span(stride, wide, h)) else { return false };
        if from > self.len || to > out.len() {
            return false;
        }
        for row in 0..h {
            let dst = &mut out[row * stride..row * stride + wide];
            // SAFETY: every row's `wide` bytes from `row * across` end at

            unsafe { std::ptr::copy_nonoverlapping(self.map.add(row * across), dst.as_mut_ptr(), wide) };
        }
        true
    }
}

pub(super) fn span(apart: usize, wide: usize, rows: usize) -> Option<usize> {
    if rows == 0 || wide == 0 || apart < wide {
        return None;
    }
    (rows - 1).checked_mul(apart)?.checked_add(wide)
}

impl Drop for Room {
    fn drop(&mut self) {
        // SAFETY: the address and length mapped in `mapped`, freed once.
        unsafe { libc::munmap(self.map as *mut libc::c_void, self.len) };
    }
}

struct Running {
    child: Child,
    talk: UnixStream,
}

impl Drop for Running {
    fn drop(&mut self) {
        // bubblewrap takes the boxed program with it.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

enum Heard {
    All,

    Ended,
    Late,
}

fn hear(talk: &UnixStream, buf: &mut [u8], within: Duration) -> Heard {
    let end = Instant::now() + within;
    let mut got = 0;
    let mut talk = talk;
    while got < buf.len() {
        let Some(left) = end.checked_duration_since(Instant::now()).filter(|left| !left.is_zero()) else { return Heard::Late };
        if talk.set_read_timeout(Some(left)).is_err() {
            return Heard::Ended;
        }
        match talk.read(&mut buf[got..]) {
            Ok(0) => return Heard::Ended,
            Ok(n) => got += n,
            Err(e) if matches!(e.kind(), std::io::ErrorKind::Interrupted) => {}
            Err(e) if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => return Heard::Late,
            Err(_) => return Heard::Ended,
        }
    }
    Heard::All
}

fn hand(cmd: &mut Command, talk: i32, room: i32) -> Option<[OwnedFd; 2]> {
    let high = |fd: i32| {
        // SAFETY: asks for a copy of a descriptor that is open, numbered

        let copy = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 10) };
        // SAFETY: the copy just made, which nothing else owns.
        (copy >= 0).then(|| unsafe { OwnedFd::from_raw_fd(copy) })
    };
    let held = [high(talk)?, high(room)?];
    let moves = [(held[0].as_raw_fd(), TALK_FD), (held[1].as_raw_fd(), ROOM_FD)];
    // SAFETY: between fork and exec the hook only calls dup2, which is

    unsafe {
        cmd.pre_exec(move || {
            for (from, to) in moves {
                if libc::dup2(from, to) < 0 {
                    return Err(std::io::Error::last_os_error());
                }
            }
            Ok(())
        });
    }
    Some(held)
}

fn started(cmd: Command, held: [OwnedFd; 2]) -> Option<Child> {
    type Asked = (Command, [OwnedFd; 2], mpsc::Sender<Option<Child>>);
    static KEEPER: OnceLock<Option<mpsc::Sender<Asked>>> = OnceLock::new();
    let keeper = KEEPER.get_or_init(|| {
        let (ask, asked) = mpsc::channel::<Asked>();
        let keep = move || {
            for (mut cmd, held, back) in asked {
                let child = cmd.spawn().ok();
                drop(held);

                // here, which leaves it to end with this program.
                let _ = back.send(child);
            }
        };
        std::thread::Builder::new().name("films".into()).stack_size(256 << 10).spawn(keep).ok().map(|_| ask)
    });
    let (back, child) = mpsc::channel();
    keeper.as_ref()?.send((cmd, held, back)).ok()?;
    child.recv().ok()?
}

const ENDING_WAIT: Duration = Duration::from_millis(500);

fn ended_as(child: &mut Child, within: Duration) -> Option<i32> {
    let end = Instant::now() + within;
    loop {
        match child.try_wait() {
            Ok(Some(ended)) => return ended.code(),
            Ok(None) if Instant::now() < end => std::thread::sleep(Duration::from_millis(5)),
            _ => return None,
        }
    }
}

pub(super) const NO_LIBRARY: i32 = 3;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Unstarted {

    NoLibrary,

    Other,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum How {

    NoBox,

    Ended(Option<i32>),

    Late,

    Another,
}

fn say_no_decoder(why: Unstarted, how: How) {
    static SAID: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if !SAID.swap(true, std::sync::atomic::Ordering::Relaxed) {
        eprintln!("trindle: {}", no_decoder_said(why, how));
    }
}

fn no_decoder_said(why: Unstarted, how: How) -> String {
    let words = no_decoder_words(why);
    if why == Unstarted::NoLibrary {
        return words.to_string();
    }
    let how = match how {
        How::NoBox => "bubblewrap is not installed or could not be started".to_string(),
        How::Ended(Some(code)) => format!("its box ended at once, with {code}"),
        How::Ended(None) => "its box ended at once".to_string(),
        How::Late => format!("it said nothing in {} seconds", START_WAIT.as_secs()),
        How::Another => "it answered for another film".to_string(),
    };
    format!("{words} ({how})")
}

fn no_decoder_words(why: Unstarted) -> &'static str {
    match why {
        Unstarted::NoLibrary => "the library that decodes films is not installed, so moving wallpapers stand still",
        Unstarted::Other => "the decoder for films would not start, so moving wallpapers stand still",
    }
}

pub struct Player {
    clip: Clip,

    at: usize,
    room: Room,
    running: Option<Running>,

    endings: usize,

    over: bool,

    wait: Duration,

    #[cfg(test)]
    part: String,
    #[cfg(test)]
    starts: usize,
}

impl Player {
    pub fn open(path: &Path) -> Option<Player> {
        Player::of(Clip::open(path)?)
    }

    pub fn of(clip: Clip) -> Option<Player> {
        let mut player = Player::unstarted(clip)?;
        player.start().then_some(player)
    }

    fn unstarted(clip: Clip) -> Option<Player> {
        let room = Room::fixed(clip.info.width as usize * clip.info.height as usize * 4)?;
        Some(Player {
            clip,
            at: 0,
            room,
            running: None,
            endings: 0,
            over: false,
            wait: FRAME_WAIT,
            #[cfg(test)]
            part: "decode".into(),
            #[cfg(test)]
            starts: 0,
        })
    }

    pub fn info(&self) -> Info {
        self.clip.info
    }

    pub fn frames(&self) -> usize {
        self.clip.len()
    }

    pub fn rest(&mut self) {
        self.running = None;
    }

    fn start(&mut self) -> bool {
        if self.over {
            return false;
        }
        #[cfg(test)]
        {
            self.starts += 1;
        }
        match self.begin_how() {
            Ok(running) => self.running = Some(running),
            Err((why, how)) => {
                self.running = None;
                self.over = true;
                say_no_decoder(why, how);
            }
        }
        self.running.is_some()
    }

    #[cfg(test)]
    fn begin(&self) -> Result<Running, Unstarted> {
        self.begin_how().map_err(|(why, _)| why)
    }

    fn begin_how(&self) -> Result<Running, (Unstarted, How)> {
        let mut running = self.boxed().ok_or((Unstarted::Other, How::NoBox))?;
        let mut hello = [0u8; 16];
        match hear(&running.talk, &mut hello, START_WAIT) {
            Heard::All => {}

            Heard::Ended => {
                let ended = ended_as(&mut running.child, ENDING_WAIT);
                return Err((if ended == Some(NO_LIBRARY) { Unstarted::NoLibrary } else { Unstarted::Other }, How::Ended(ended)));
            }
            Heard::Late => return Err((Unstarted::Other, How::Late)),
        }
        let word = |at: usize| u32::from_le_bytes([hello[at], hello[at + 1], hello[at + 2], hello[at + 3]]);
        let same = hello[..4] == HELLO && word(4) as usize == self.clip.len() && (word(8), word(12)) == (self.clip.info.width, self.clip.info.height);
        if same { Ok(running) } else { Err((Unstarted::Other, How::Another)) }
    }

    fn boxed(&self) -> Option<Running> {
        let (mine, theirs) = UnixStream::pair().ok()?;
        let film = self.clip.file.try_clone().ok()?;
        #[cfg(not(test))]
        let (more, run): ([OsString; 0], [OsString; 1]) = ([], [DECODING.into()]);

        #[cfg(test)]
        let (more, run) = (["--setenv", tests::CHILD, self.part.as_str()].map(OsString::from),
                           ["--exact", "session::moving::tests::the_boxed_decoder", "--nocapture", "--test-threads=1"].map(OsString::from));

        let mut cmd = crate::common::boxed_self::boxed_self(&more, &run, ceiling(&self.clip.info), libc::RLIM_INFINITY)?;
        cmd.stdin(Stdio::from(film)).stdout(Stdio::null()).stderr(Stdio::null());
        let held = hand(&mut cmd, theirs.as_raw_fd(), self.room.fd.as_raw_fd())?;
        let child = started(cmd, held)?;

        drop(theirs);
        Some(Running { child, talk: mine })
    }

    fn ended(&mut self) {
        self.running = None;
        self.endings += 1;
        self.over |= self.endings >= MOST_ENDINGS;
    }

    fn give_up(&mut self) {
        self.running = None;
        self.over = true;
    }

    fn frame(&mut self, step: usize) -> Result<Option<(u32, u32)>, ()> {
        let step = step.clamp(1, u32::MAX as usize);
        let (w, h) = (self.clip.info.width, self.clip.info.height);
        if self.running.is_none() {

            self.at = goes_on_from(self.at, &self.clip.info);
            if !self.start() {
                return Err(());
            }
        }
        let i = self.at;
        let mut ask = [0u8; 8];
        ask[..4].copy_from_slice(&(i as u32).to_le_bytes());
        ask[4..].copy_from_slice(&(step as u32).to_le_bytes());
        let running = self.running.as_mut().ok_or(())?;
        let mut answer = [0u8; 8];
        let heard = if running.talk.write_all(&ask).is_ok() { hear(&running.talk, &mut answer, self.wait) } else { Heard::Ended };
        match heard {
            Heard::All => {}
            Heard::Ended => {
                self.ended();
                return Err(());
            }
            Heard::Late => {
                self.give_up();
                return Err(());
            }
        }
        self.at = (i + 1) % self.clip.len();
        let said = (u32::from_le_bytes([answer[0], answer[1], answer[2], answer[3]]), u32::from_le_bytes([answer[4], answer[5], answer[6], answer[7]]));
        let most = (w / step as u32, h / step as u32);
        match said {
            (0, 0) => Ok(None),
            (fw, fh) if (1..=most.0).contains(&fw) && (1..=most.1).contains(&fh) => Ok(Some((fw, fh))),
            _ => {
                self.give_up();
                Err(())
            }
        }
    }

    fn across(&self, step: usize) -> usize {
        (self.clip.info.width as usize / step.max(1)) * 4
    }

    pub fn next(&mut self, out: &mut [u8], stride: usize) -> Option<(u32, u32)> {
        self.next_small(out, stride, 1)
    }

    pub fn next_small(&mut self, out: &mut [u8], stride: usize, step: usize) -> Option<(u32, u32)> {
        let step = step.max(1);
        for _ in 0..self.clip.len().min(MOST_PASSED) {
            match self.frame(step) {
                Err(()) => return None,
                Ok(None) => {}
                Ok(Some((w, h))) => return self.room.rows(self.across(step), (w as usize, h as usize), out, stride).then_some((w, h)),
            }
        }
        None
    }

    pub fn over(&self) -> bool {
        self.over
    }
}

pub fn first_frame(path: &Path, step: usize) -> Option<(u32, u32, Vec<u8>)> {
    let mut p = Player::open(path)?;
    let step = step.max(1);
    let (w, h) = p.frame(step).ok()??;

    let mut xrgb = vec![0u8; w as usize * h as usize * 4];
    if !p.room.rows(p.across(step), (w as usize, h as usize), &mut xrgb, w as usize * 4) {
        return None;
    }
    let rgb = xrgb.chunks_exact(4).flat_map(|px| [px[2], px[1], px[0]]).collect();
    Some((w, h, rgb))
}

#[cfg(test)]
pub fn info(path: &Path) -> Option<Info> {
    Clip::open(path).map(|c| c.info)
}

#[cfg(test)]
mod tests;
