//! The mouse, the touchpad and the touchscreen, read straight from the kernel before anything else has started, so the eye in the password box can be clicked.

use std::io::Read;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::os::unix::io::{AsRawFd, RawFd};

const EV_KEY: u16 = 1;
const EV_REL: u16 = 2;
const EV_ABS: u16 = 3;
const REL_X: u16 = 0;
const REL_Y: u16 = 1;
const ABS_X: u16 = 0;
const ABS_Y: u16 = 1;
const BTN_LEFT: u16 = 0x110;
const BTN_TOOL_FINGER: u16 = 0x145;
const BTN_TOUCH: u16 = 0x14a;
const INPUT_PROP_DIRECT: u16 = 0x01;

const TAP_MS: i64 = 250;
const TAP_TRAVEL: f32 = 12.0;

pub enum Happened {
    Moved,
    Clicked(f32, f32),
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Mouse,
    Touchpad,
    Absolute,
}

#[derive(Clone, Copy, Default)]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn span(&self) -> f32 {
        (self.max - self.min).max(1) as f32
    }
}

struct Device {
    file: std::fs::File,
    number: u64,
    kind: Kind,
    x: Range,
    y: Range,
    last: (Option<i32>, Option<i32>),
    touching: bool,
    touch_began: i64,
    travelled: f32,
}

pub struct Pointer {
    devices: Vec<Device>,
    passed: Vec<u64>,
    width: f32,
    height: f32,
    pub at: (f32, f32),
    pub seen: bool,
}

fn read_ioc(nr: u64, size: u64) -> u64 {
    (2 << 30) | (size << 16) | ((b'E' as u64) << 8) | nr
}

fn bits(fd: RawFd, request: u64, into: &mut [u8]) -> bool {
    // SAFETY: the kernel writes at most `into.len()` bytes, the size encoded
    // in the request, into a buffer of ours that is that long; the
    // descriptor is open for the whole call.
    unsafe { libc::ioctl(fd, request as _, into.as_mut_ptr()) >= 0 }
}

fn has(set: &[u8], bit: u16) -> bool {
    set.get(bit as usize / 8).is_some_and(|b| b & (1 << (bit % 8)) != 0)
}

fn range(fd: RawFd, axis: u16) -> Option<Range> {
    let mut info = [0i32; 6];
    // SAFETY: EVIOCGABS fills in a struct input_absinfo, six 32-bit numbers,
    // which is exactly this array; the descriptor is open for the call.
    let ok = unsafe { libc::ioctl(fd, read_ioc(0x40 + axis as u64, 24) as _, info.as_mut_ptr()) } >= 0;
    ok.then_some(Range { min: info[1], max: info[2] })
}

fn now_ms() -> i64 {
    let mut t = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: fills in a local timespec.
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut t) };
    t.tv_sec * 1000 + t.tv_nsec / 1_000_000
}

impl Pointer {
    pub fn open(width: usize, height: usize) -> Pointer {
        let mut p = Pointer {
            devices: Vec::new(),
            passed: Vec::new(),
            width: width as f32,
            height: height as f32,
            at: (width as f32 / 2.0, height as f32 * 0.7),
            seen: false,
        };
        p.rescan();
        p
    }

    pub fn rescan(&mut self) {
        let Ok(dir) = std::fs::read_dir("/dev/input") else { return };
        let mut names: Vec<_> = dir.flatten().map(|e| e.path())
            .filter(|p| p.file_name().is_some_and(|n| n.to_string_lossy().starts_with("event")))
            .collect();
        names.sort();
        for path in names {
            let Ok(file) = std::fs::OpenOptions::new().read(true)
                .custom_flags(libc::O_NONBLOCK | libc::O_CLOEXEC).open(&path) else { continue };
            let Ok(number) = file.metadata().map(|m| m.rdev()) else { continue };
            if self.devices.iter().any(|d| d.number == number) || self.passed.contains(&number) {
                continue;
            }
            match classify(file, number) {
                Some(device) => self.devices.push(device),
                None => self.passed.push(number),
            }
        }
    }

    pub fn fds(&self) -> Vec<RawFd> {
        self.devices.iter().map(|d| d.file.as_raw_fd()).collect()
    }

    pub fn read(&mut self, fd: RawFd) -> Vec<Happened> {
        let mut out = Vec::new();
        let Some(i) = self.devices.iter().position(|d| d.file.as_raw_fd() == fd) else { return out };
        let mut buf = [0u8; 24 * 64];
        loop {
            let n = match self.devices[i].file.read(&mut buf) {
                Ok(n) if n > 0 => n,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                _ => {
                    self.devices.remove(i);
                    return out;
                }
            };
            for event in buf[..n].chunks_exact(24) {
                let kind = u16::from_ne_bytes([event[16], event[17]]);
                let code = u16::from_ne_bytes([event[18], event[19]]);
                let value = i32::from_ne_bytes([event[20], event[21], event[22], event[23]]);
                if let Some(h) = self.event(i, kind, code, value) {
                    out.push(h);
                }
            }
        }
        out
    }

    fn event(&mut self, i: usize, kind: u16, code: u16, value: i32) -> Option<Happened> {
        let (w, h) = (self.width, self.height);
        let d = &mut self.devices[i];
        match (d.kind, kind, code) {
            (Kind::Mouse, EV_REL, REL_X) => self.nudge(value as f32, 0.0),
            (Kind::Mouse, EV_REL, REL_Y) => self.nudge(0.0, value as f32),
            (Kind::Absolute, EV_ABS, ABS_X) => {
                self.at.0 = (value - d.x.min) as f32 / d.x.span() * w;
                self.seen = true;
                Some(Happened::Moved)
            }
            (Kind::Absolute, EV_ABS, ABS_Y) => {
                self.at.1 = (value - d.y.min) as f32 / d.y.span() * h;
                self.seen = true;
                Some(Happened::Moved)
            }
            (Kind::Touchpad, EV_ABS, ABS_X | ABS_Y) if d.touching => {
                let (last, span, screen) = if code == ABS_X { (&mut d.last.0, d.x.span(), w) } else { (&mut d.last.1, d.y.span(), h) };
                let moved = last.map(|l| (value - l) as f32 / span * screen * 0.9).unwrap_or(0.0);
                *last = Some(value);
                d.travelled += moved.abs();
                if code == ABS_X { self.nudge(moved, 0.0) } else { self.nudge(0.0, moved) }
            }
            (Kind::Touchpad, EV_KEY, BTN_TOUCH) => {
                if value == 1 {
                    d.touching = true;
                    d.touch_began = now_ms();
                    d.travelled = 0.0;
                    d.last = (None, None);
                    None
                } else {
                    d.touching = false;
                    let tap = now_ms() - d.touch_began < TAP_MS && d.travelled < TAP_TRAVEL;
                    tap.then_some(Happened::Clicked(self.at.0, self.at.1))
                }
            }
            (Kind::Absolute, EV_KEY, BTN_TOUCH) if value == 0 => Some(Happened::Clicked(self.at.0, self.at.1)),
            (_, EV_KEY, BTN_LEFT) if value == 1 => {
                self.seen = true;
                Some(Happened::Clicked(self.at.0, self.at.1))
            }
            _ => None,
        }
    }

    fn nudge(&mut self, dx: f32, dy: f32) -> Option<Happened> {
        self.at.0 = (self.at.0 + dx).clamp(0.0, self.width - 1.0);
        self.at.1 = (self.at.1 + dy).clamp(0.0, self.height - 1.0);
        self.seen = true;
        Some(Happened::Moved)
    }
}

fn classify(file: std::fs::File, number: u64) -> Option<Device> {
    let fd = file.as_raw_fd();
    let mut types = [0u8; 4];
    let mut keys = [0u8; 96];
    let mut rel = [0u8; 2];
    let mut abs = [0u8; 8];
    let mut props = [0u8; 4];
    if !bits(fd, read_ioc(0x20, 4), &mut types) {
        return None;
    }
    let _ = bits(fd, read_ioc(0x20 + EV_KEY as u64, 96), &mut keys);
    let _ = bits(fd, read_ioc(0x20 + EV_REL as u64, 2), &mut rel);
    let _ = bits(fd, read_ioc(0x20 + EV_ABS as u64, 8), &mut abs);
    let _ = bits(fd, read_ioc(0x09, 4), &mut props);
    let kind = kind_of(
        has(&types, EV_REL) && has(&rel, REL_X),
        has(&types, EV_ABS) && has(&abs, ABS_X) && has(&abs, ABS_Y),
        has(&keys, BTN_TOOL_FINGER),
        has(&props, INPUT_PROP_DIRECT),
        has(&keys, BTN_LEFT) || has(&keys, BTN_TOUCH),
    )?;
    let (x, y) = match kind {
        Kind::Mouse => (Range::default(), Range::default()),
        _ => (range(fd, ABS_X)?, range(fd, ABS_Y)?),
    };
    Some(Device { file, number, kind, x, y, last: (None, None), touching: false, touch_began: 0, travelled: 0.0 })
}

fn kind_of(relative: bool, absolute: bool, finger: bool, direct: bool, button: bool) -> Option<Kind> {
    if relative && button {
        Some(Kind::Mouse)
    } else if absolute && finger && !direct {
        Some(Kind::Touchpad)
    } else if absolute && button {
        Some(Kind::Absolute)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{kind_of, Kind};

    #[test]
    fn each_kind_of_pointer_is_told_apart_and_keyboards_are_left_alone() {
        assert_eq!(kind_of(true, false, false, false, true), Some(Kind::Mouse));
        assert_eq!(kind_of(false, true, true, false, true), Some(Kind::Touchpad));
        assert_eq!(kind_of(false, true, true, true, true), Some(Kind::Absolute));
        assert_eq!(kind_of(false, true, false, false, true), Some(Kind::Absolute));
        assert_eq!(kind_of(false, false, false, false, false), None);
        assert_eq!(kind_of(true, false, false, false, false), None);
    }

    #[test]
    fn the_ioctl_numbers_are_the_kernels() {
        assert_eq!(super::read_ioc(0x20, 4), 0x8004_4520);
        assert_eq!(super::read_ioc(0x40, 24), 0x8018_4540);
    }
}
