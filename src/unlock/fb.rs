//! The screen, as the kernel offers it before anything graphical is running.

use std::os::unix::io::{AsRawFd, RawFd};

const FBIOGET_VSCREENINFO: libc::c_ulong = 0x4600;
const FBIOGET_FSCREENINFO: libc::c_ulong = 0x4602;

#[repr(C)]
#[derive(Default)]
struct Bitfield {
    offset: u32,
    length: u32,
    msb_right: u32,
}

#[repr(C)]
#[derive(Default)]
struct VarInfo {
    xres: u32,
    yres: u32,
    xres_virtual: u32,
    yres_virtual: u32,
    xoffset: u32,
    yoffset: u32,
    bits_per_pixel: u32,
    grayscale: u32,
    red: Bitfield,
    green: Bitfield,
    blue: Bitfield,
    transp: Bitfield,
    nonstd: u32,
    activate: u32,
    height: u32,
    width: u32,
    accel_flags: u32,
    pixclock: u32,
    left_margin: u32,
    right_margin: u32,
    upper_margin: u32,
    lower_margin: u32,
    hsync_len: u32,
    vsync_len: u32,
    sync: u32,
    vmode: u32,
    rotate: u32,
    colorspace: u32,
    reserved: [u32; 4],
}

#[repr(C)]
#[derive(Default)]
struct FixInfo {
    id: [u8; 16],
    smem_start: libc::c_ulong,
    smem_len: u32,
    kind: u32,
    type_aux: u32,
    visual: u32,
    xpanstep: u16,
    ypanstep: u16,
    ywrapstep: u16,
    line_length: u32,
    mmio_start: libc::c_ulong,
    mmio_len: u32,
    accel: u32,
    capabilities: u16,
    reserved: [u16; 2],
}

pub struct Screen {
    file: std::fs::File,
    pixels: *mut u8,
    size: usize,
    pub width: usize,
    pub height: usize,
    stride: usize,
    bytes: usize,

    places: (u32, u32, u32),

    back: Vec<u8>,

    scene: Vec<u8>,
}

impl Screen {
    pub fn open() -> Option<Screen> {
        let file = std::fs::OpenOptions::new().read(true).write(true).open("/dev/fb0").ok()?;
        let fd: RawFd = file.as_raw_fd();
        let mut var = VarInfo::default();
        let mut fix = FixInfo::default();
        // SAFETY: each asks the framebuffer driver to fill in a local struct

        if unsafe { libc::ioctl(fd, FBIOGET_VSCREENINFO, &mut var) } != 0
            // SAFETY: as above, for the fixed screen information.
            || unsafe { libc::ioctl(fd, FBIOGET_FSCREENINFO, &mut fix) } != 0
        {
            return None;
        }
        let bytes = (var.bits_per_pixel / 8) as usize;
        if !(2..=4).contains(&bytes) {
            return None;
        }
        let stride = fix.line_length as usize;
        let size = stride * var.yres as usize;
        // SAFETY: `size` is the screen's own stride times its height, as the

        let pixels = unsafe {
            libc::mmap(std::ptr::null_mut(), size, libc::PROT_READ | libc::PROT_WRITE,
                       libc::MAP_SHARED, fd, 0)
        };
        if pixels == libc::MAP_FAILED {
            return None;
        }
        Some(Screen {
            file,
            pixels: pixels as *mut u8,
            size,
            width: var.xres as usize,
            height: var.yres as usize,
            stride,
            bytes,
            places: (var.red.offset, var.green.offset, var.blue.offset),
            back: vec![0; size],
            scene: Vec::new(),
        })
    }

    pub fn fill(&mut self, colour: (u8, u8, u8)) {
        let word = self.word(colour);
        for y in 0..self.height {
            for x in 0..self.width {
                self.put(x, y, word);
            }
        }
    }

    pub fn blend(&mut self, x: usize, y: usize, colour: (u8, u8, u8), alpha: f32) {
        if x >= self.width || y >= self.height || alpha <= 0.0 {
            return;
        }
        let at = y * self.stride + x * self.bytes;
        let have = self.read(at);
        let mix = |a: u8, b: u8| (a as f32 * (1.0 - alpha) + b as f32 * alpha) as u8;
        let out = (mix(have.0, colour.0), mix(have.1, colour.1), mix(have.2, colour.2));
        let word = self.word(out);
        self.put(x, y, word);
    }

    fn read(&self, at: usize) -> (u8, u8, u8) {
        let mut word = 0u32;
        for i in 0..self.bytes {
            word |= (self.back[at + i] as u32) << (8 * i);
        }
        let (r, g, b) = self.places;
        let take = |shift: u32| ((word >> shift) & 0xff) as u8;
        if self.bytes == 2 {

            (((word >> 11) as u8 & 0x1f) << 3, ((word >> 5) as u8 & 0x3f) << 2, (word as u8 & 0x1f) << 3)
        } else {
            (take(r), take(g), take(b))
        }
    }

    fn word(&self, c: (u8, u8, u8)) -> u32 {
        if self.bytes == 2 {
            return ((c.0 as u32 >> 3) << 11) | ((c.1 as u32 >> 2) << 5) | (c.2 as u32 >> 3);
        }
        let (r, g, b) = self.places;
        ((c.0 as u32) << r) | ((c.1 as u32) << g) | ((c.2 as u32) << b)
    }

    fn put(&mut self, x: usize, y: usize, word: u32) {
        let at = y * self.stride + x * self.bytes;
        if at + self.bytes > self.size {
            return;
        }
        for i in 0..self.bytes {
            self.back[at + i] = (word >> (8 * i)) as u8;
        }
    }

    #[cfg(test)]
    pub fn in_memory(width: usize, height: usize) -> Screen {
        Screen {
            file: std::fs::File::open("/dev/null").expect("/dev/null"),
            pixels: std::ptr::null_mut(),
            size: width * height * 4,
            width,
            height,
            stride: width * 4,
            bytes: 4,
            places: (16, 8, 0),
            back: vec![0; width * height * 4],
            scene: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn rgb(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.width * self.height * 3);
        for y in 0..self.height {
            for x in 0..self.width {
                let (r, g, b) = self.read(y * self.stride + x * self.bytes);
                out.extend_from_slice(&[r, g, b]);
            }
        }
        out
    }

    pub fn show(&mut self) {
        self.show_rows(0, self.height);
    }

    pub fn show_rows(&mut self, top: usize, bottom: usize) {
        if self.pixels.is_null() {
            return;
        }
        let start = (top * self.stride).min(self.size);
        let end = (bottom * self.stride).min(self.size);
        if start >= end {
            return;
        }
        // SAFETY: `back` was made `size` bytes long and never resized, and

        unsafe { std::ptr::copy_nonoverlapping(self.back.as_ptr().add(start), self.pixels.add(start), end - start) };
    }

    pub fn keep(&mut self) {
        self.scene.clone_from(&self.back);
    }

    pub fn restore_rows(&mut self, top: usize, bottom: usize) {
        let start = (top * self.stride).min(self.scene.len());
        let end = (bottom * self.stride).min(self.scene.len());
        if start < end {
            self.back[start..end].copy_from_slice(&self.scene[start..end]);
        }
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        if self.pixels.is_null() {
            return;
        }
        // SAFETY: the same address and length mmap returned in `open`, released

        unsafe { libc::munmap(self.pixels as *mut libc::c_void, self.size) };
        let _ = &self.file;
    }
}
