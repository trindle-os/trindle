//! Where things are.

use crate::toolkit::input::Cursor;

pub type Pt = (f64, f64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rect {
    pub fn contains(&self, p: Pt) -> bool {
        p.0 >= self.x && p.0 <= self.x + self.w && p.1 >= self.y && p.1 <= self.y + self.h
    }
}

pub fn between(a: Pt, b: Pt) -> Rect {
    Rect { x: a.0.min(b.0), y: a.1.min(b.1), w: (a.0 - b.0).abs(), h: (a.1 - b.1).abs() }
}

pub fn display_in_picture(picture: Pt, displays: &[Rect], this: Rect) -> Rect {
    let left = displays.iter().map(|d| d.x).fold(f64::INFINITY, f64::min);
    let top = displays.iter().map(|d| d.y).fold(f64::INFINITY, f64::min);
    let right = displays.iter().map(|d| d.x + d.w).fold(f64::NEG_INFINITY, f64::max);
    let bottom = displays.iter().map(|d| d.y + d.h).fold(f64::NEG_INFINITY, f64::max);
    if displays.is_empty() || right <= left || bottom <= top {
        return Rect { x: 0.0, y: 0.0, w: picture.0, h: picture.1 };
    }
    let (sx, sy) = (picture.0 / (right - left), picture.1 / (bottom - top));
    Rect { x: (this.x - left) * sx, y: (this.y - top) * sy, w: this.w * sx, h: this.h * sy }
}

pub fn to_pixels(chosen: Rect, view: Pt, display: Rect) -> (i32, i32, i32, i32) {
    let (sx, sy) = (display.w / view.0.max(1.0), display.h / view.1.max(1.0));
    let x0 = (display.x + chosen.x.max(0.0) * sx).round();
    let y0 = (display.y + chosen.y.max(0.0) * sy).round();
    let x1 = (display.x + (chosen.x + chosen.w).min(view.0) * sx).round();
    let y1 = (display.y + (chosen.y + chosen.h).min(view.1) * sy).round();
    (x0 as i32, y0 as i32, (x1 - x0).max(1.0) as i32, (y1 - y0).max(1.0) as i32)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub shown: Rect,
    pub size: Pt,
}

impl View {
    pub fn scale(&self) -> f64 {
        self.shown.w / self.size.0.max(1.0)
    }

    pub fn in_picture(&self, p: Pt) -> Pt {
        let s = self.scale();
        (self.shown.x + p.0 * s, self.shown.y + p.1 * s)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Handle {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
}

impl Handle {
    pub const ALL: [Handle; 8] = [Handle::TopLeft, Handle::Top, Handle::TopRight, Handle::Right,
                                  Handle::BottomRight, Handle::Bottom, Handle::BottomLeft, Handle::Left];

    pub fn at(self, r: Rect) -> Pt {
        let (cx, cy) = (r.x + r.w / 2.0, r.y + r.h / 2.0);
        let (right, bottom) = (r.x + r.w, r.y + r.h);
        match self {
            Handle::TopLeft => (r.x, r.y),
            Handle::Top => (cx, r.y),
            Handle::TopRight => (right, r.y),
            Handle::Right => (right, cy),
            Handle::BottomRight => (right, bottom),
            Handle::Bottom => (cx, bottom),
            Handle::BottomLeft => (r.x, bottom),
            Handle::Left => (r.x, cy),
        }
    }

    pub fn cursor(self) -> Cursor {
        match self {
            Handle::TopLeft => Cursor::NwResize,
            Handle::Top => Cursor::NResize,
            Handle::TopRight => Cursor::NeResize,
            Handle::Right => Cursor::EResize,
            Handle::BottomRight => Cursor::SeResize,
            Handle::Bottom => Cursor::SResize,
            Handle::BottomLeft => Cursor::SwResize,
            Handle::Left => Cursor::WResize,
        }
    }

    fn sides(self) -> (bool, bool, bool, bool) {
        match self {
            Handle::TopLeft => (true, true, false, false),
            Handle::Top => (false, true, false, false),
            Handle::TopRight => (false, true, true, false),
            Handle::Right => (false, false, true, false),
            Handle::BottomRight => (false, false, true, true),
            Handle::Bottom => (false, false, false, true),
            Handle::BottomLeft => (true, false, false, true),
            Handle::Left => (true, false, false, false),
        }
    }
}

pub fn handle_at(r: Rect, p: Pt, reach: f64) -> Option<Handle> {
    let far = |h: &Handle| {
        let (x, y) = h.at(r);
        (x - p.0).hypot(y - p.1)
    };
    let nearest = |list: &[Handle]| {
        list.iter().copied().filter(|h| far(h) <= reach).min_by(|a, b| far(a).total_cmp(&far(b)))
    };
    let corners = [Handle::TopLeft, Handle::TopRight, Handle::BottomRight, Handle::BottomLeft];
    nearest(&corners).or_else(|| nearest(&Handle::ALL))
}

pub fn resized(r: Rect, h: Handle, dx: f64, dy: f64) -> Rect {
    let (l, t, rt, b) = h.sides();
    let mut x0 = r.x;
    let mut y0 = r.y;
    let mut x1 = r.x + r.w;
    let mut y1 = r.y + r.h;
    if l {
        x0 += dx;
    }
    if rt {
        x1 += dx;
    }
    if t {
        y0 += dy;
    }
    if b {
        y1 += dy;
    }
    between((x0, y0), (x1, y1))
}

pub fn moved(r: Rect, dx: f64, dy: f64, view: Pt) -> Rect {
    let x = (r.x + dx).clamp(0.0, (view.0 - r.w).max(0.0));
    let y = (r.y + dy).clamp(0.0, (view.1 - r.h).max(0.0));
    Rect { x, y, ..r }
}

pub fn inside(r: Rect, view: Pt) -> Rect {
    let x0 = r.x.clamp(0.0, view.0);
    let y0 = r.y.clamp(0.0, view.1);
    let x1 = (r.x + r.w).clamp(0.0, view.0);
    let y1 = (r.y + r.h).clamp(0.0, view.1);
    between((x0, y0), (x1, y1))
}

pub fn tools_at(r: Rect, tools: Pt, view: Pt, gap: f64) -> Pt {
    let x = (r.x + r.w / 2.0 - tools.0 / 2.0).clamp(gap, (view.0 - tools.0 - gap).max(gap));
    let below = r.y + r.h + gap;
    let above = r.y - gap - tools.1;
    let y = if below + tools.1 + gap <= view.1 {
        below
    } else if above >= gap {
        above
    } else {
        (r.y + r.h - gap - tools.1).clamp(gap, (view.1 - tools.1 - gap).max(gap))
    };
    (x, y)
}

pub fn size_label_at(r: Rect, label: Pt, gap: f64) -> Pt {
    let y = if r.y - gap - label.1 >= 0.0 { r.y - gap - label.1 } else { r.y + gap };
    (r.x.max(0.0), y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_drag_is_the_same_part_whichever_way_it_goes() {
        let r = Rect { x: 10.0, y: 20.0, w: 30.0, h: 40.0 };
        assert_eq!(between((10.0, 20.0), (40.0, 60.0)), r);
        assert_eq!(between((40.0, 60.0), (10.0, 20.0)), r);
        assert_eq!(between((40.0, 20.0), (10.0, 60.0)), r);
    }

    #[test]
    fn one_display_is_the_whole_picture() {
        let screen = Rect { x: 0.0, y: 0.0, w: 1536.0, h: 864.0 };
        let got = display_in_picture((1920.0, 1080.0), &[screen], screen);
        assert_eq!(got, Rect { x: 0.0, y: 0.0, w: 1920.0, h: 1080.0 });
        assert_eq!(display_in_picture((800.0, 600.0), &[], screen), Rect { x: 0.0, y: 0.0, w: 800.0, h: 600.0 });
    }

    #[test]
    fn the_second_display_is_found_in_its_place() {
        let left = Rect { x: 0.0, y: 0.0, w: 1920.0, h: 1080.0 };
        let right = Rect { x: 1920.0, y: 0.0, w: 1280.0, h: 1024.0 };
        let got = display_in_picture((3200.0, 1080.0), &[left, right], right);
        assert_eq!(got, Rect { x: 1920.0, y: 0.0, w: 1280.0, h: 1024.0 });
    }

    #[test]
    fn a_part_becomes_pixels_of_the_picture() {
        let display = Rect { x: 0.0, y: 0.0, w: 1920.0, h: 1080.0 };
        let part = Rect { x: 100.0, y: 50.0, w: 200.0, h: 100.0 };
        assert_eq!(to_pixels(part, (1536.0, 864.0), display), (125, 63, 250, 125));
        let second = Rect { x: 1920.0, y: 0.0, w: 1280.0, h: 1024.0 };
        assert_eq!(to_pixels(part, (1280.0, 1024.0), second), (2020, 50, 200, 100));
    }

    #[test]
    fn a_part_dragged_off_the_edge_stops_at_it() {
        let display = Rect { x: 0.0, y: 0.0, w: 1000.0, h: 500.0 };
        let part = Rect { x: -20.0, y: 400.0, w: 120.0, h: 300.0 };
        assert_eq!(to_pixels(part, (1000.0, 500.0), display), (0, 400, 100, 100));
    }

    #[test]
    fn the_pointer_lands_on_the_right_pixel_of_the_picture() {
        let v = View { shown: Rect { x: 1920.0, y: 0.0, w: 1920.0, h: 1080.0 }, size: (1536.0, 864.0) };
        assert_eq!(v.scale(), 1.25);
        assert_eq!(v.in_picture((100.0, 40.0)), (2045.0, 50.0));
    }

    #[test]
    fn a_grip_is_found_near_the_pointer_and_corners_come_first() {
        let r = Rect { x: 100.0, y: 100.0, w: 200.0, h: 100.0 };
        assert_eq!(handle_at(r, (103.0, 98.0), 10.0), Some(Handle::TopLeft));
        assert_eq!(handle_at(r, (200.0, 205.0), 10.0), Some(Handle::Bottom));
        assert_eq!(handle_at(r, (300.0, 150.0), 10.0), Some(Handle::Right));
        assert_eq!(handle_at(r, (200.0, 150.0), 10.0), None);
        let tiny = Rect { x: 0.0, y: 0.0, w: 8.0, h: 8.0 };
        assert_eq!(handle_at(tiny, (7.0, 7.0), 10.0), Some(Handle::BottomRight));
    }

    #[test]
    fn a_grip_moves_only_its_own_sides() {
        let r = Rect { x: 100.0, y: 100.0, w: 200.0, h: 100.0 };
        assert_eq!(resized(r, Handle::Right, 50.0, 30.0), Rect { x: 100.0, y: 100.0, w: 250.0, h: 100.0 });
        assert_eq!(resized(r, Handle::Top, 50.0, -30.0), Rect { x: 100.0, y: 70.0, w: 200.0, h: 130.0 });
        assert_eq!(resized(r, Handle::TopLeft, 10.0, 10.0), Rect { x: 110.0, y: 110.0, w: 190.0, h: 90.0 });
    }

    #[test]
    fn a_grip_pulled_past_the_far_side_turns_the_part_over() {
        let r = Rect { x: 100.0, y: 100.0, w: 200.0, h: 100.0 };
        assert_eq!(resized(r, Handle::Left, 250.0, 0.0), Rect { x: 300.0, y: 100.0, w: 50.0, h: 100.0 });
    }

    #[test]
    fn a_moved_part_stops_at_the_screen_edge() {
        let r = Rect { x: 100.0, y: 100.0, w: 200.0, h: 100.0 };
        assert_eq!(moved(r, -500.0, 20.0, (1000.0, 500.0)), Rect { x: 0.0, y: 120.0, w: 200.0, h: 100.0 });
        assert_eq!(moved(r, 900.0, 900.0, (1000.0, 500.0)), Rect { x: 800.0, y: 400.0, w: 200.0, h: 100.0 });
        assert_eq!(inside(Rect { x: -10.0, y: 450.0, w: 50.0, h: 100.0 }, (1000.0, 500.0)),
                   Rect { x: 0.0, y: 450.0, w: 40.0, h: 50.0 });
    }

    #[test]
    fn the_tools_sit_below_then_above_then_inside() {
        let view = (1000.0, 800.0);
        let tools = (400.0, 100.0);
        let low = Rect { x: 300.0, y: 100.0, w: 400.0, h: 200.0 };
        assert_eq!(tools_at(low, tools, view, 10.0), (300.0, 310.0));
        let near_foot = Rect { x: 300.0, y: 500.0, w: 400.0, h: 250.0 };
        assert_eq!(tools_at(near_foot, tools, view, 10.0), (300.0, 390.0));
        let whole = Rect { x: 0.0, y: 0.0, w: 1000.0, h: 800.0 };
        assert_eq!(tools_at(whole, tools, view, 10.0), (300.0, 690.0));
        let side = Rect { x: 900.0, y: 100.0, w: 100.0, h: 100.0 };
        assert_eq!(tools_at(side, tools, view, 10.0).0, 590.0);
    }

    #[test]
    fn the_size_sits_above_unless_there_is_no_room() {
        let r = Rect { x: 50.0, y: 100.0, w: 10.0, h: 10.0 };
        assert_eq!(size_label_at(r, (80.0, 24.0), 6.0), (50.0, 70.0));
        let top = Rect { x: 50.0, y: 5.0, w: 10.0, h: 10.0 };
        assert_eq!(size_label_at(top, (80.0, 24.0), 6.0), (50.0, 11.0));
    }
}
