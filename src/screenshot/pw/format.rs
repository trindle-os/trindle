//! Reading the video format both sides settled on.

use std::time::Duration;

use super::pod::{CHOICE_ENUM, FORMAT_MEDIA_SUBTYPE, FORMAT_MEDIA_TYPE, FORMAT_VIDEO_FORMAT, FORMAT_VIDEO_FRAMERATE, FORMAT_VIDEO_MAX_FRAMERATE, FORMAT_VIDEO_MODIFIER, FORMAT_VIDEO_SIZE, MEDIA_SUBTYPE_RAW, MEDIA_TYPE_VIDEO, MOD_LINEAR, OBJECT_FORMAT, POD_CHOICE, POD_FRACTION, POD_ID, POD_LONG, POD_OBJECT, POD_RECTANGLE, PROP_DONT_FIXATE, RATE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VideoFormat {
    pub width: u32,
    pub height: u32,

    pub modifier: Option<u64>,

    pub rate: (u32, u32),
}

impl VideoFormat {

    pub fn frame_gap(&self) -> Duration {
        let (num, denom) = if self.rate.0 > 0 && self.rate.1 > 0 { self.rate } else { RATE };
        let gap = Duration::from_secs(u64::from(denom)) / num;
        gap.clamp(Duration::from_secs(1) / 360, Duration::from_secs(1))
    }
}

pub fn linear_first(modifiers: &[u64]) -> Vec<u64> {
    let linear = modifiers.iter().filter(|&&m| m == MOD_LINEAR);
    linear.chain(modifiers.iter().filter(|&&m| m != MOD_LINEAR)).copied().collect()
}

pub fn tries(open: &[u64]) -> Vec<Vec<u64>> {
    let rest: Vec<u64> = open.iter().copied().filter(|&m| m != MOD_LINEAR).collect();
    let mut out = Vec::new();
    if open.contains(&MOD_LINEAR) {
        out.push(vec![MOD_LINEAR]);
    }
    if !rest.is_empty() {
        out.push(rest);
    }
    out
}

pub(super) fn read_pod(bytes: &[u8]) -> Option<(u32, &[u8], usize)> {
    let size = u32::from_ne_bytes(bytes.get(0..4)?.try_into().ok()?) as usize;
    let kind = u32::from_ne_bytes(bytes.get(4..8)?.try_into().ok()?);
    let body = bytes.get(8..8 + size)?;
    let total = (8 + size).div_ceil(8) * 8;
    Some((kind, body, total))
}

pub(super) fn word_at(bytes: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_ne_bytes(bytes.get(at..at + 4)?.try_into().ok()?))
}

pub(super) fn chosen(kind: u32, body: &[u8]) -> Option<(u32, &[u8])> {
    if kind != POD_CHOICE {
        return Some((kind, body));
    }
    let child_size = word_at(body, 8)? as usize;
    let child_type = word_at(body, 12)?;
    Some((child_type, body.get(16..16 + child_size)?))
}

fn longs_of(kind: u32, body: &[u8]) -> Option<Vec<u64>> {
    let long = |b: &[u8]| b.try_into().ok().map(u64::from_ne_bytes);
    if kind == POD_LONG {
        return Some(vec![long(body.get(0..8)?)?]);
    }
    if kind != POD_CHOICE || word_at(body, 8)? != 8 || word_at(body, 12)? != POD_LONG {
        return None;
    }
    let all: Vec<u64> = body.get(16..)?.chunks_exact(8).filter_map(long).collect();

    Some(match word_at(body, 0)? {
        CHOICE_ENUM if all.len() > 1 => all[1..].to_vec(),
        _ => all,
    })
}

pub fn parse_format(pod: &[u8]) -> Option<(VideoFormat, Option<Vec<u64>>)> {
    let (kind, body, _) = read_pod(pod)?;
    if kind != POD_OBJECT || word_at(body, 0)? != OBJECT_FORMAT {
        return None;
    }
    let mut at = 8;
    let (mut format, mut size, mut media, mut modifier, mut open) = (None, None, (None, None), None, None);
    let (mut most, mut rate) = ((0, 0), (0, 0));
    while at + 8 < body.len() {
        let key = word_at(body, at)?;
        let flags = word_at(body, at + 4)?;
        let (kind, value, total) = read_pod(body.get(at + 8..)?)?;
        if key == FORMAT_VIDEO_MODIFIER {
            let longs = longs_of(kind, value)?;
            if flags & PROP_DONT_FIXATE != 0 && kind == POD_CHOICE {
                open = Some(longs.clone());
            }
            modifier = longs.first().copied();
        }
        let (kind, value) = chosen(kind, value)?;
        match (key, kind) {
            (FORMAT_MEDIA_TYPE, POD_ID) => media.0 = word_at(value, 0),
            (FORMAT_MEDIA_SUBTYPE, POD_ID) => media.1 = word_at(value, 0),
            (FORMAT_VIDEO_FORMAT, POD_ID) => format = word_at(value, 0),
            (FORMAT_VIDEO_SIZE, POD_RECTANGLE) => size = Some((word_at(value, 0)?, word_at(value, 4)?)),
            (FORMAT_VIDEO_MAX_FRAMERATE, POD_FRACTION) => most = (word_at(value, 0)?, word_at(value, 4)?),
            (FORMAT_VIDEO_FRAMERATE, POD_FRACTION) => rate = (word_at(value, 0)?, word_at(value, 4)?),
            _ => {}
        }
        at += 8 + total;
    }
    if media != (Some(MEDIA_TYPE_VIDEO), Some(MEDIA_SUBTYPE_RAW)) {
        return None;
    }
    let (width, height) = size?;

    format?;

    let rate = if most.0 > 0 && most.1 > 0 { most } else { rate };
    Some((VideoFormat { width, height, modifier, rate }, open))
}
