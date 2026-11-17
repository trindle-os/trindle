//! The whole desktop put together from each screen's picture, and a choice kept for next time.

use super::{cast_capture, Kept};
use super::super::cast_pick::Choice;

pub(super) struct Out {
    ptr: *mut u8,
    len: usize,
}

impl Out {

    #[cfg(test)]
    pub(super) fn of(slice: &mut [u8]) -> Out {
        Out { ptr: slice.as_mut_ptr(), len: slice.len() }
    }

    /// # Safety
    /// `ptr` must be valid for writes of `len` bytes for as long as this is used.
    pub(super) unsafe fn raw(ptr: *mut u8, len: usize) -> Out {
        Out { ptr, len }
    }

    pub(super) fn zero(&self) {
        // SAFETY: `ptr` is valid for `len` bytes, as `raw` requires.
        unsafe { std::ptr::write_bytes(self.ptr, 0, self.len) };
    }

    fn put(&self, at: usize, bytes: &[u8]) {
        if at.checked_add(bytes.len()).is_some_and(|end| end <= self.len) {
            // SAFETY: the range is inside the `len` bytes valid for writes, and `bytes` is

            unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), self.ptr.add(at), bytes.len()) };
        }
    }
}

pub(super) fn place(out: &Out, stride: u32, dims: (u32, u32), src: &[u8], size: (u32, u32), at: (i32, i32, u32, u32)) {
    let (sw, sh) = size;
    let (x0, y0, aw, ah) = at;
    if sw == 0 || sh == 0 || aw == 0 || ah == 0 || src.len() < (sw * sh * 4) as usize {
        return;
    }
    for ty in 0..ah {
        let y = y0 + ty as i32;
        if y < 0 || y as u32 >= dims.1 {
            continue;
        }
        let row = (y as u32 * stride) as usize;
        let sy = (ty * sh / ah).min(sh - 1);
        if aw == sw && x0 >= 0 && x0 as u32 + aw <= dims.0 {
            let from = (sy * sw * 4) as usize;
            let to = row + (x0 as usize) * 4;
            out.put(to, &src[from..from + (sw * 4) as usize]);
            continue;
        }
        for tx in 0..aw {
            let x = x0 + tx as i32;
            if x < 0 || x as u32 >= dims.0 {
                continue;
            }
            let sx = (tx * sw / aw).min(sw - 1);
            let from = ((sy * sw + sx) * 4) as usize;
            let to = row + (x as usize) * 4;
            out.put(to, &src[from..from + 4]);
        }
    }
}

pub(super) fn desktop_layout(screens: &[cast_capture::Screen]) -> ((u32, u32), Vec<(i32, i32, u32, u32)>) {
    let scale = screens.iter()
        .map(|s| if s.place[2] > 0 { s.pixels.0 as f64 / s.place[2] as f64 } else { 1.0 })
        .fold(1.0_f64, f64::max);
    let (bx, by) = (screens.iter().map(|s| s.place[0]).min().unwrap_or(0), screens.iter().map(|s| s.place[1]).min().unwrap_or(0));
    let ex = screens.iter().map(|s| s.place[0] + s.place[2]).max().unwrap_or(0);
    let ey = screens.iter().map(|s| s.place[1] + s.place[3]).max().unwrap_or(0);
    let px = |v: i32| (v as f64 * scale).round() as i32;
    let at = screens.iter()
        .map(|s| (px(s.place[0] - bx), px(s.place[1] - by), px(s.place[2]).max(1) as u32, px(s.place[3]).max(1) as u32))
        .collect();
    ((px(ex - bx).max(1) as u32, px(ey - by).max(1) as u32), at)
}

pub(super) fn kept(choice: &Choice) -> Kept {
    match choice {
        Choice::Desktop => ("desktop".into(), String::new(), String::new(), String::new()),
        Choice::Screen(s) => ("screen".into(), s.name.clone(), String::new(), String::new()),
        Choice::Window(w, _) => ("window".into(), String::new(), w.app_id.clone(), w.title.clone()),
    }
}

pub(super) fn recall(kept: &[Kept], screens: &[cast_capture::Screen], windows: &[cast_capture::Window]) -> Option<Vec<Choice>> {
    if kept.is_empty() {
        return None;
    }
    kept.iter().map(|(kind, name, app, title)| match kind.as_str() {
        "desktop" => (screens.len() > 1).then_some(Choice::Desktop),
        "screen" => screens.iter().find(|s| s.name == *name).cloned().map(Choice::Screen),
        "window" => {
            let same: Vec<&cast_capture::Window> = windows.iter().filter(|w| w.app_id == *app).collect();
            same.iter().find(|w| w.title == *title).or_else(|| (same.len() == 1).then(|| &same[0]))
                .map(|w| Choice::Window((*w).clone(), None))
        }
        _ => None,
    }).collect()
}
