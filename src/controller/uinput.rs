//! The kernel's side.

use super::Kernel;
use crate::common::controller_wire::{Event, Pad, EV_ABS, EV_KEY, NAME};
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

const UI_DEV_CREATE: u64 = 0x5501;
const UI_DEV_SETUP: u64 = 0x405c_5503;
const UI_ABS_SETUP: u64 = 0x401c_5504;
const UI_SET_EVBIT: u64 = 0x4004_5564;
const UI_SET_KEYBIT: u64 = 0x4004_5565;
const UI_SET_ABSBIT: u64 = 0x4004_5567;

#[repr(C)]
struct Setup {
    bus: u16,
    vendor: u16,
    product: u16,
    version: u16,
    name: [u8; 80],
    ff_effects_max: u32,
}

#[repr(C)]
struct AbsSetup {
    code: u16,
    _pad: u16,
    value: i32,
    min: i32,
    max: i32,
    fuzz: i32,
    flat: i32,
    resolution: i32,
}

#[repr(C)]
struct InputEvent {
    time: libc::timeval,
    kind: u16,
    code: u16,
    value: i32,
}

const _: () = assert!(std::mem::size_of::<Setup>() == 92 && std::mem::size_of::<AbsSetup>() == 28);

pub struct Uinput;

fn ioctl(fd: &OwnedFd, request: u64, arg: libc::c_ulong) -> io::Result<()> {
    // SAFETY: each request is paired with the argument the kernel's uinput.h gives it: an int, or the address of a whole struct of its size that outlives the call.
    let r = unsafe { libc::ioctl(fd.as_raw_fd(), request as _, arg) };
    if r < 0 { Err(io::Error::last_os_error()) } else { Ok(()) }
}

impl Kernel for Uinput {
    type Made = OwnedFd;

    fn make(&mut self, pad: &Pad) -> io::Result<OwnedFd> {
        // SAFETY: the path is a C string literal; the result is checked before it is owned.
        let fd = unsafe { libc::open(c"/dev/uinput".as_ptr(), libc::O_RDWR | libc::O_CLOEXEC | libc::O_NONBLOCK) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: fd is fresh and nothing else owns it.
        let fd = unsafe { OwnedFd::from_raw_fd(fd) };
        ioctl(&fd, UI_SET_EVBIT, EV_KEY as libc::c_ulong)?;
        for k in &pad.keys {
            ioctl(&fd, UI_SET_KEYBIT, *k as libc::c_ulong)?;
        }
        if !pad.axes.is_empty() {
            ioctl(&fd, UI_SET_EVBIT, EV_ABS as libc::c_ulong)?;
        }
        for a in &pad.axes {
            ioctl(&fd, UI_SET_ABSBIT, a.code as libc::c_ulong)?;
            let abs = AbsSetup { code: a.code, _pad: 0, value: a.min, min: a.min, max: a.max, fuzz: a.fuzz, flat: a.flat, resolution: 0 };
            ioctl(&fd, UI_ABS_SETUP, &abs as *const AbsSetup as libc::c_ulong)?;
        }
        let mut setup = Setup { bus: pad.bus, vendor: pad.vendor, product: pad.product, version: pad.version, name: [0; 80], ff_effects_max: 0 };
        setup.name[..NAME.len()].copy_from_slice(NAME.as_bytes());
        ioctl(&fd, UI_DEV_SETUP, &setup as *const Setup as libc::c_ulong)?;
        ioctl(&fd, UI_DEV_CREATE, 0)?;
        Ok(fd)
    }

    fn emit(&mut self, made: &mut OwnedFd, events: &[Event]) -> io::Result<()> {
        let out: Vec<InputEvent> = events
            .iter()
            .map(|e| InputEvent { time: libc::timeval { tv_sec: 0, tv_usec: 0 }, kind: e.kind, code: e.code, value: e.value })
            .collect();
        let bytes = std::mem::size_of_val(out.as_slice());
        loop {
            // SAFETY: out is a whole array of input_event, valid for `bytes`.
            let n = unsafe { libc::write(made.as_raw_fd(), out.as_ptr().cast(), bytes) };
            if n < 0 {
                let e = io::Error::last_os_error();
                if e.kind() == io::ErrorKind::Interrupted {
                    continue;
                }
                return Err(e);
            }
            return if n as usize == bytes { Ok(()) } else { Err(io::Error::new(io::ErrorKind::WriteZero, "the kernel took only part of the events")) };
        }
    }
}
