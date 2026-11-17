//! The one optional picture a report may carry: checked on a thread, stripped of everything but the image, and shown before Send.

use std::io::{Cursor, Read};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageFormat};

use crate::toolkit::t;

pub const MOST_BYTES: usize = 2 * 1024 * 1024;

pub const MOST_SIDE: u32 = 8192;
pub const MOST_PIXELS: u64 = 16_000_000;

const PREVIEW: (u32, u32) = (320, 180);

const MOST_MEMORY: u64 = MOST_PIXELS * 8;
pub(super) const SIGNATURE: [u8; 8] = *b"\x89PNG\r\n\x1a\n";

const KEPT: [&[u8; 4]; 5] = [b"IHDR", b"PLTE", b"tRNS", b"IDAT", b"IEND"];

pub struct Loaded {
    pub png: Vec<u8>,
    pub size: (u32, u32),
    pub preview: image::RgbaImage,
}

const OVERLAY: &str = crate::common::combos::SCREENSHOT_COMMAND;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Fault {
    NotPng,
    TooBig,
    TooLarge,
    Damaged,
    Unreadable,

    NoShot,

    Cancelled,
}

impl Fault {
    pub fn words(self) -> String {
        match self {
            Fault::NotPng => t!("report-picture-not-png").to_string(),
            Fault::TooBig => t!("report-picture-too-big", megabytes = MOST_BYTES / (1024 * 1024)),
            Fault::TooLarge => t!("report-picture-too-large", side = MOST_SIDE),
            Fault::Damaged => t!("report-picture-damaged").to_string(),
            Fault::Unreadable => t!("report-picture-unreadable").to_string(),
            Fault::NoShot => t!("report-picture-no-shot").to_string(),
            Fault::Cancelled => String::new(),
        }
    }
}

pub fn read(path: &Path) -> Result<Loaded, Fault> {

    let file = std::fs::OpenOptions::new().read(true).custom_flags(libc::O_NONBLOCK).open(path).map_err(|_| Fault::Unreadable)?;
    let meta = file.metadata().map_err(|_| Fault::Unreadable)?;
    if !meta.is_file() {
        return Err(Fault::Unreadable);
    }
    if meta.len() > MOST_BYTES as u64 {
        return Err(Fault::TooBig);
    }
    let mut bytes = Vec::new();
    file.take(MOST_BYTES as u64 + 1).read_to_end(&mut bytes).map_err(|_| Fault::Unreadable)?;
    load(&bytes)
}

pub fn load(bytes: &[u8]) -> Result<Loaded, Fault> {
    if bytes.len() > MOST_BYTES {
        return Err(Fault::TooBig);
    }
    let (png, size) = strip(bytes)?;
    let img = decode(&png)?;
    Ok(Loaded { png, size, preview: img.thumbnail(PREVIEW.0, PREVIEW.1).to_rgba8() })
}

fn decode(png: &[u8]) -> Result<DynamicImage, Fault> {
    let mut reader = image::ImageReader::with_format(Cursor::new(png), ImageFormat::Png);
    let mut limits = image::Limits::default();
    limits.max_image_width = Some(MOST_SIDE);
    limits.max_image_height = Some(MOST_SIDE);
    limits.max_alloc = Some(MOST_MEMORY);
    reader.limits(limits);
    reader.decode().map_err(|_| Fault::Damaged)
}

fn strip(bytes: &[u8]) -> Result<(Vec<u8>, (u32, u32)), Fault> {
    if bytes.len() < SIGNATURE.len() || bytes[..SIGNATURE.len()] != SIGNATURE {
        return Err(Fault::NotPng);
    }
    let mut out = SIGNATURE.to_vec();
    let mut at = SIGNATURE.len();
    let mut size = None;
    while at + 12 <= bytes.len() {
        let length = u32::from_be_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]) as usize;
        let kind: [u8; 4] = [bytes[at + 4], bytes[at + 5], bytes[at + 6], bytes[at + 7]];
        let end = at.checked_add(12).and_then(|e| e.checked_add(length)).filter(|e| *e <= bytes.len()).ok_or(Fault::Damaged)?;
        if size.is_none() {
            if &kind != b"IHDR" || length != 13 {
                return Err(Fault::Damaged);
            }
            let side = |o: usize| u32::from_be_bytes([bytes[at + 8 + o], bytes[at + 9 + o], bytes[at + 10 + o], bytes[at + 11 + o]]);
            let (w, h) = (side(0), side(4));
            if w == 0 || h == 0 {
                return Err(Fault::Damaged);
            }
            if w > MOST_SIDE || h > MOST_SIDE || u64::from(w) * u64::from(h) > MOST_PIXELS {
                return Err(Fault::TooLarge);
            }
            size = Some((w, h));
        }
        if KEPT.iter().any(|k| **k == kind) {
            out.extend_from_slice(&bytes[at..end]);
        }
        at = end;
        if &kind == b"IEND" {
            return size.map(|s| (out, s)).ok_or(Fault::Damaged);
        }
    }
    Err(Fault::Damaged)
}

pub fn fit_down(img: &DynamicImage) -> Result<Loaded, Fault> {
    let (w, h) = img.dimensions();
    let mut scale = (MOST_PIXELS as f64 / (f64::from(w) * f64::from(h)).max(1.0)).sqrt().min(f64::from(MOST_SIDE) / f64::from(w.max(h).max(1))).min(1.0);
    for _ in 0..12 {
        let (nw, nh) = (((f64::from(w) * scale) as u32).max(1), ((f64::from(h) * scale) as u32).max(1));
        let small = if (nw, nh) == (w, h) { img.clone() } else { img.resize_exact(nw, nh, image::imageops::FilterType::Triangle) };
        let mut encoded = Vec::new();
        small.write_to(&mut Cursor::new(&mut encoded), ImageFormat::Png).map_err(|_| Fault::Unreadable)?;
        match load(&encoded) {
            Err(Fault::TooBig) => scale *= 0.7,
            done => return done,
        }
    }
    Err(Fault::TooBig)
}

pub fn screen() -> Result<Loaded, Fault> {
    let scratch = crate::common::files::private_temp("trindle-report-shot").map_err(|_| Fault::NoShot)?;
    let file = scratch.join("area.png");
    let made = area_into(&file);
    let _ = std::fs::remove_dir_all(&scratch);
    fit_down(&made?)
}

fn area_into(file: &Path) -> Result<DynamicImage, Fault> {
    let status = std::process::Command::new(OVERLAY).arg("--for-report").arg(file).status().map_err(|_| Fault::NoShot)?;
    after_overlay(status.success(), file)
}

pub(super) fn after_overlay(chosen: bool, file: &Path) -> Result<DynamicImage, Fault> {
    if !chosen {
        return Err(Fault::Cancelled);
    }
    image::open(file).map_err(|_| Fault::NoShot)
}

pub fn base64(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = (u32::from(chunk[0]) << 16) | (u32::from(*chunk.get(1).unwrap_or(&0)) << 8) | u32::from(*chunk.get(2).unwrap_or(&0));
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(DIGITS[((n >> (18 - 6 * i)) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}
