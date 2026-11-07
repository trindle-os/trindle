//! A controller read as it is used, for Settings' picture and list.

use std::collections::{HashMap, HashSet};
use std::io::Read;
use std::os::fd::AsRawFd;
use std::os::unix::fs::OpenOptionsExt;

use super::{hat_buttons, From, Range, AXES, BUTTONS, DIRECTIONS, EV_ABS, EV_KEY, TRIGGERS};

const ABS_BITS: libc::c_ulong = 0x8008_4523;
const ABS_INFO: libc::c_ulong = 0x8018_4540;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pairs: [(u16, u16); 6],
}

impl Layout {
    pub const STANDARD: Layout = Layout { pairs: [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)] };

    pub fn of(has: u64) -> Layout {
        let has = |a: u16| (has >> a) & 1 == 1;
        if !has(3) && has(2) && has(5) && has(9) && has(10) {
            return Layout { pairs: [(0, 0), (1, 1), (3, 2), (4, 5), (2, 10), (5, 9)] };
        }
        Layout::STANDARD
    }

    pub fn table(&self, real: u16) -> u16 {
        self.pairs.iter().find(|p| p.1 == real).map_or(real, |p| p.0)
    }

    pub fn real(&self, table: u16) -> u16 {
        self.pairs.iter().find(|p| p.0 == table).map_or(table, |p| p.1)
    }

    pub fn places(&self, real: u16) -> bool {
        self.pairs.iter().any(|p| p.1 == real)
    }
}

#[derive(Debug)]
pub struct Live {
    files: Vec<std::fs::File>,
    pub layout: Layout,
    down: HashSet<u16>,
    axes: HashMap<u16, (i32, Range)>,
}

impl Live {
    pub fn open(nodes: &[String]) -> Live {
        let files: Vec<std::fs::File> = nodes
            .iter()
            .filter_map(|n| std::fs::OpenOptions::new().read(true).custom_flags(libc::O_NONBLOCK).open(format!("/dev/input/{n}")).ok())
            .collect();
        let mut has = 0u64;
        for f in &files {
            let mut bits = [0u8; 8];
            // SAFETY: the number says 8 bytes of axis bits and bits is that.
            if unsafe { libc::ioctl(f.as_raw_fd(), ABS_BITS, bits.as_mut_ptr()) } >= 0 {
                has |= u64::from_le_bytes(bits);
            }
        }
        Live { files, layout: Layout::of(has), down: HashSet::new(), axes: HashMap::new() }
    }

    pub fn made_up(layout: Layout) -> Live {
        Live { files: Vec::new(), layout, down: HashSet::new(), axes: HashMap::new() }
    }

    pub fn is_open(&self) -> bool {
        !self.files.is_empty()
    }

    pub fn poll(&mut self) -> Vec<From> {
        let mut pressed = Vec::new();
        let mut events: Vec<(usize, u16, u16, i32)> = Vec::new();
        for (i, file) in self.files.iter_mut().enumerate() {
            let mut chunk = [0u8; 24 * 64];
            while let Ok(n) = file.read(&mut chunk) {
                if n == 0 {
                    break;
                }
                for e in chunk[..n].chunks_exact(24) {
                    events.push((i, u16::from_ne_bytes([e[16], e[17]]), u16::from_ne_bytes([e[18], e[19]]), i32::from_ne_bytes([e[20], e[21], e[22], e[23]])));
                }
            }
        }
        for (i, kind, code, value) in events {
            if kind == EV_ABS && !self.axes.contains_key(&code) {
                let mut info = [0i32; 6];
                // SAFETY: the number says 24 bytes, a struct input_absinfo, and info is that.
                let got = unsafe { libc::ioctl(self.files[i].as_raw_fd(), ABS_INFO + libc::c_ulong::from(code), info.as_mut_ptr()) };
                if got >= 0 && info[2] > info[1] {
                    self.axes.insert(code, (info[0], Range { min: info[1], max: info[2] }));
                }
            }
            pressed.extend(self.feed(kind, code, value));
        }
        pressed
    }

    pub fn feed(&mut self, kind: u16, code: u16, value: i32) -> Option<From> {
        if kind == EV_KEY {
            if !BUTTONS.iter().any(|b| b.0 == code) {
                return None;
            }
            return if value == 0 { self.down.remove(&code); None } else { self.down.insert(code).then_some(From::Button(code)) };
        }
        if kind != EV_ABS {
            return None;
        }
        if let Some((lo, hi)) = hat_buttons(code) {
            let (was_lo, was_hi) = (self.down.contains(&lo), self.down.contains(&hi));
            for (b, on) in [(lo, value < 0), (hi, value > 0)] {
                if on { self.down.insert(b); } else { self.down.remove(&b); }
            }
            return if value < 0 && !was_lo { Some(From::Button(lo)) } else if value > 0 && !was_hi { Some(From::Button(hi)) } else { None };
        }
        let table = self.layout.table(code);
        if !self.layout.places(code) || !AXES.iter().any(|a| a.0 == table) {
            return None;
        }
        let range = self.axes.get(&code).map_or(Range { min: -32768, max: 32767 }, |a| a.1);
        let before = self.axes.get(&code).map_or(range.middle(), |a| a.0);
        self.axes.insert(code, (value, range));
        if TRIGGERS.contains(&table) {
            return (range.pulled(value) && !range.pulled(before)).then_some(From::Trigger(table));
        }
        let half = (i64::from(range.max) - i64::from(range.min)) / 2;
        let off = |v: i32| i64::from(v) - i64::from(range.middle());
        let past = |v: i32| off(v).abs() * 2 > half;
        (past(value) && !past(before) && DIRECTIONS.iter().any(|d| d.0 == table)).then_some(From::Direction(table, off(value) > 0))
    }

    pub fn is_down(&self, button: u16) -> bool {
        self.down.contains(&button)
    }

    pub fn moved(&self, table: u16) -> f32 {
        let Some((v, r)) = self.axes.get(&self.layout.real(table)) else { return 0.0 };
        let half = (i64::from(r.max) - i64::from(r.min)) as f32 / 2.0;
        ((i64::from(*v) - i64::from(r.middle())) as f32 / half).clamp(-1.0, 1.0)
    }

    pub fn pulled(&self, table: u16) -> f32 {
        let Some((v, r)) = self.axes.get(&self.layout.real(table)) else { return 0.0 };
        ((i64::from(*v) - i64::from(r.min)) as f32 / (i64::from(r.max) - i64::from(r.min)) as f32).clamp(0.0, 1.0)
    }

    pub fn held(&self, from: From) -> bool {
        match from {
            From::Button(b) => self.is_down(b),
            From::Trigger(a) => self.pulled(a) > 0.5,
            From::Direction(a, positive) => {
                let m = self.moved(a);
                if positive { m > 0.5 } else { m < -0.5 }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOUTH: u16 = 0x130;

    fn pad(layout: Layout) -> Live {
        Live::made_up(layout)
    }

    #[test]
    fn a_button_is_down_from_its_press_to_its_release_and_is_pressed_once() {
        let mut live = pad(Layout::STANDARD);
        assert_eq!(live.feed(EV_KEY, SOUTH, 1), Some(From::Button(SOUTH)));
        assert!(live.is_down(SOUTH));
        assert_eq!(live.feed(EV_KEY, SOUTH, 1), None);
        assert_eq!(live.feed(EV_KEY, SOUTH, 0), None);
        assert!(!live.is_down(SOUTH));
    }

    #[test]
    fn the_d_pad_hat_is_four_buttons() {
        let mut live = pad(Layout::STANDARD);
        assert_eq!(live.feed(EV_ABS, 17, -1), Some(From::Button(0x220)));
        assert!(live.is_down(0x220));
        assert_eq!(live.feed(EV_ABS, 17, 1), Some(From::Button(0x221)));
        assert!(!live.is_down(0x220) && live.is_down(0x221));
        assert_eq!(live.feed(EV_ABS, 17, 0), None);
        assert!(!live.is_down(0x221));
    }

    #[test]
    fn a_stick_is_pressed_once_past_half_way_and_moves_the_picture_all_along() {
        let mut live = pad(Layout::STANDARD);
        assert_eq!(live.feed(EV_ABS, 1, -8000), None);
        assert!(live.moved(1) < 0.0 && !live.held(From::Direction(1, false)));
        assert_eq!(live.feed(EV_ABS, 1, -20000), Some(From::Direction(1, false)));
        assert!(live.held(From::Direction(1, false)));
        assert_eq!(live.feed(EV_ABS, 1, -30000), None);
        assert_eq!(live.feed(EV_ABS, 1, 0), None);
        assert!(live.moved(1).abs() < 0.01);
    }

    #[test]
    fn a_trigger_is_pressed_once_half_pulled() {
        let mut live = pad(Layout::STANDARD);
        live.axes.insert(2, (0, Range { min: 0, max: 255 }));
        assert_eq!(live.feed(EV_ABS, 2, 60), None);
        assert_eq!(live.feed(EV_ABS, 2, 200), Some(From::Trigger(2)));
        assert!(live.pulled(2) > 0.7);
        assert_eq!(live.feed(EV_ABS, 2, 255), None);
    }

    #[test]
    fn a_controller_with_the_right_stick_on_the_odd_axes_is_read_by_its_table() {
        let has = [0u16, 1, 2, 5, 9, 10, 16, 17].iter().fold(0u64, |a, b| a | (1 << b));
        let layout = Layout::of(has);
        assert_eq!((layout.table(2), layout.table(5), layout.table(10), layout.table(9)), (3, 4, 2, 5));
        assert_eq!((layout.real(3), layout.real(4), layout.real(2), layout.real(5)), (2, 5, 10, 9));
        let mut live = pad(layout);
        assert_eq!(live.feed(EV_ABS, 5, 30000), Some(From::Direction(4, true)));
        assert!(live.moved(4) > 0.9);
        assert_eq!(live.feed(EV_ABS, 3, 30000), None);
        assert_eq!(Layout::of([0u16, 1, 2, 3, 4, 5].iter().fold(0u64, |a, b| a | (1 << b))), Layout::STANDARD);
    }

    #[test]
    fn what_is_not_a_button_or_stick_is_left_alone() {
        let mut live = pad(Layout::STANDARD);
        assert_eq!(live.feed(EV_KEY, 0x2ff, 1), None);
        assert_eq!(live.feed(4, 4, 589825), None);
        assert_eq!(live.feed(EV_ABS, 40, 1), None);
    }
}
