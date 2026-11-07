//! The outer edges of the screens, where the ring is reached.

use smithay::utils::{Logical, Point, Rectangle};

pub const CROSS_PUSH: f64 = 48.0;
pub const CORNER_GUARD: f64 = 140.0;
const PUSH_PAUSE_MS: u32 = 300;
const ALONG: f64 = 2.0;

pub fn at_outer_edge(screens: &[Rectangle<i32, Logical>], pos: Point<f64, Logical>, band: i32) -> bool {
    !outer_sides(screens, pos, band).is_empty()
}

pub fn bounding_box(screens: &[Rectangle<i32, Logical>]) -> Option<(i32, i32, i32, i32)> {
    screens.iter().fold(None, |b, s| {
        let (x0, y0, x1, y1) = (s.loc.x, s.loc.y, s.loc.x + s.size.w, s.loc.y + s.size.h);
        Some(b.map_or((x0, y0, x1, y1), |(a, b2, c, d)| (a.min(x0), b2.min(y0), c.max(x1), d.max(y1))))
    })
}

pub fn absolute_to_screens(screens: &[Rectangle<i32, Logical>], fx: f64, fy: f64) -> Option<Point<f64, Logical>> {
    let (x0, y0, x1, y1) = bounding_box(screens)?;
    let at = (f64::from(x0) + fx * f64::from(x1 - x0), f64::from(y0) + fy * f64::from(y1 - y0)).into();
    clamp_to_screens(screens, at)
}

const INSIDE: f64 = 0.6;

pub fn clamp_to_screens(screens: &[Rectangle<i32, Logical>], pos: Point<f64, Logical>) -> Option<Point<f64, Logical>> {
    let on: Point<i32, Logical> = (pos.x.round() as i32, pos.y.round() as i32).into();
    if screens.iter().any(|s| s.contains(on)) {
        return Some(pos);
    }
    screens
        .iter()
        .map(|s| {
            let x = pos.x.clamp(f64::from(s.loc.x), f64::from(s.loc.x + s.size.w) - INSIDE);
            let y = pos.y.clamp(f64::from(s.loc.y), f64::from(s.loc.y + s.size.h) - INSIDE);
            (((x - pos.x).powi(2) + (y - pos.y).powi(2)), Point::<f64, Logical>::from((x, y)))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, p)| p)
}

pub fn near_corner(screens: &[Rectangle<i32, Logical>], pos: Point<f64, Logical>, guard: f64) -> bool {
    let p: Point<i32, Logical> = (pos.x.floor() as i32, pos.y.floor() as i32).into();
    let Some(s) = screens.iter().find(|s| s.contains(p)) else { return false };
    let (x0, y0) = (f64::from(s.loc.x), f64::from(s.loc.y));
    let (x1, y1) = (x0 + f64::from(s.size.w) - 1.0, y0 + f64::from(s.size.h) - 1.0);
    [(x0, y0), (x1, y0), (x0, y1), (x1, y1)]
        .into_iter()
        .any(|(cx, cy)| (pos.x - cx).hypot(pos.y - cy) < guard)
}

pub fn outer_sides(screens: &[Rectangle<i32, Logical>], pos: Point<f64, Logical>, band: i32) -> Vec<Side> {
    let band = band.max(1);
    let p: Point<i32, Logical> = (pos.x.floor() as i32, pos.y.floor() as i32).into();
    let Some(s) = screens.iter().find(|s| s.contains(p)) else { return Vec::new() };
    let open = |x: i32, y: i32| !screens.iter().any(|o| o.contains(Point::from((x, y))));
    let (x, y) = (p.x - s.loc.x, p.y - s.loc.y);
    [
        (Side::Left, x < band && open(s.loc.x - 1, p.y)),
        (Side::Right, x >= s.size.w - band && open(s.loc.x + s.size.w, p.y)),
        (Side::Top, y < band && open(p.x, s.loc.y - 1)),
        (Side::Bottom, y >= s.size.h - band && open(p.x, s.loc.y + s.size.h)),
    ]
    .into_iter()
    .filter_map(|(side, at)| at.then_some(side))
    .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

impl Side {
    pub fn beyond(self, tried: Point<f64, Logical>, now: Point<f64, Logical>) -> (f64, f64) {
        let (dx, dy) = (tried.x - now.x, tried.y - now.y);
        match self {
            Side::Left => (-dx, dy.abs()),
            Side::Right => (dx, dy.abs()),
            Side::Top => (-dy, dx.abs()),
            Side::Bottom => (dy, dx.abs()),
        }
    }
}

#[derive(Debug)]
pub struct EdgePush {
    side: Option<Side>,
    pushed: f64,
    last_ms: u32,
    told: bool,
    need: f64,
    resting: bool,
}

impl EdgePush {
    pub fn needing(need: f64) -> EdgePush {
        EdgePush { side: None, pushed: 0.0, last_ms: 0, told: false, need, resting: false }
    }

    pub fn feed(&mut self, beyond: Option<(Side, f64)>, along: f64, at_edge: bool, time_ms: u32) -> Option<Side> {
        let was = std::mem::replace(&mut self.resting, at_edge);
        let beyond = beyond.filter(|_| was);
        let Some((side, out)) = beyond.filter(|(_, out)| *out > 0.0) else {
            if !at_edge {
                *self = EdgePush::needing(self.need);
                self.resting = false;
            } else if along > ALONG {
                self.pushed = 0.0;
            }
            return None;
        };
        let pause = time_ms.wrapping_sub(self.last_ms) > PUSH_PAUSE_MS;
        if self.side != Some(side) || pause {
            self.pushed = 0.0;
        }
        self.side = Some(side);
        self.last_ms = time_ms;
        if out < along {
            self.pushed = 0.0;
            return None;
        }
        self.pushed += out;
        if self.pushed >= self.need && !self.told {
            self.told = true;
            return Some(side);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(x: i32, y: i32, w: i32, h: i32) -> Rectangle<i32, Logical> {
        Rectangle::new((x, y).into(), (w, h).into())
    }
    fn abs(screens: &[Rectangle<i32, Logical>], fx: f64, fy: f64) -> (f64, f64) {
        let p = absolute_to_screens(screens, fx, fy).unwrap();
        let on: Point<i32, Logical> = (p.x.round() as i32, p.y.round() as i32).into();
        assert!(screens.iter().any(|s| s.contains(on)), "{p:?} is off every screen");
        (p.x, p.y)
    }

    #[test]
    fn made_up_mouse_lands_on_a_screen_for_every_layout() {
        let one = [r(0, 0, 1920, 1080)];
        assert_eq!(abs(&one, 0.5, 0.5), (960.0, 540.0));
        assert!(abs(&one, 1.0, 1.0).0 < 1920.0);
        let side = [r(0, 0, 1920, 1080), r(1920, 0, 1920, 1080)];
        assert_eq!(abs(&side, 0.75, 0.0), (2880.0, 0.0));
        let gap = [r(100, 50, 1000, 800), r(1500, 50, 1000, 800)];
        assert_eq!(bounding_box(&gap), Some((100, 50, 2500, 850)));
        assert_eq!(abs(&gap, 0.0, 0.0), (100.0, 50.0));
        let p = abs(&gap, 0.5, 0.5);
        assert!(p.0 <= 1100.0 || p.0 >= 1500.0, "{p:?}");
        let stacked = [r(0, 0, 1920, 1080), r(0, 1080, 1920, 1080)];
        assert_eq!(abs(&stacked, 0.5, 0.75), (960.0, 1620.0));
        let mixed = [r(0, 0, 2560, 1440), r(2560, 0, 1280, 720)];
        let p = abs(&mixed, 1.0, 1.0);
        assert!(p.0 > 2559.0 && p.1 < 720.0, "{p:?}");
        let neg = [r(-1920, 0, 1920, 1080), r(0, 0, 1920, 1080)];
        assert_eq!(abs(&neg, 0.0, 0.0), (-1920.0, 0.0));
        assert_eq!(abs(&neg, 0.25, 0.5), (-960.0, 540.0));
        let above = [r(0, -1080, 1920, 1080), r(0, 0, 1920, 1080)];
        assert_eq!(abs(&above, 0.5, 0.25), (960.0, -540.0));
    }

    #[test]
    fn a_seam_between_screens_is_not_clamped() {
        let side = [r(0, 0, 1920, 1080), r(1920, 0, 1920, 1080)];
        let p: Point<f64, Logical> = (1919.9, 10.0).into();
        assert_eq!(clamp_to_screens(&side, p), Some(p));
        let lone = [r(0, 0, 1920, 1080)];
        assert!(clamp_to_screens(&lone, p).unwrap().x < 1919.5);
    }

    fn at(x: f64, y: f64) -> Point<f64, Logical> {
        (x, y).into()
    }

    #[test]
    fn a_click_inside_the_band_is_at_the_edge() {
        let one = [r(0, 0, 1280, 720)];
        for (x, y) in [(0.0, 300.0), (5.9, 300.0), (1279.4, 10.0), (1274.0, 600.0), (640.0, 0.0), (640.0, 719.4), (0.0, 0.0)] {
            assert!(at_outer_edge(&one, at(x, y), 6), "{x},{y}");
        }
    }

    #[test]
    fn a_click_outside_the_band_is_not() {
        let one = [r(0, 0, 1280, 720)];
        for (x, y) in [(6.0, 300.0), (640.0, 360.0), (1273.9, 300.0), (640.0, 6.0), (640.0, 713.9)] {
            assert!(!at_outer_edge(&one, at(x, y), 6), "{x},{y}");
        }
        assert!(!at_outer_edge(&one, at(-3.0, 300.0), 6));
    }

    #[test]
    fn where_two_screens_meet_is_not_an_edge() {
        let two = [r(0, 0, 1280, 720), r(1280, 0, 1920, 1080)];
        assert!(!at_outer_edge(&two, at(1279.4, 300.0), 6));
        assert!(!at_outer_edge(&two, at(1280.0, 300.0), 6));
        assert!(!at_outer_edge(&two, at(1283.0, 300.0), 6));
        assert!(at_outer_edge(&two, at(0.0, 300.0), 6));
        assert!(at_outer_edge(&two, at(3199.4, 300.0), 6));
        assert!(at_outer_edge(&two, at(600.0, 719.4), 6));
        assert!(at_outer_edge(&two, at(1280.0, 900.0), 6));
        let stacked = [r(0, 0, 1280, 720), r(0, 720, 1280, 720)];
        assert!(!at_outer_edge(&stacked, at(640.0, 719.4), 6));
        assert!(!at_outer_edge(&stacked, at(640.0, 720.0), 6));
        assert!(at_outer_edge(&stacked, at(640.0, 0.0), 6));
        assert!(at_outer_edge(&stacked, at(640.0, 1439.4), 6));
    }

    #[test]
    fn near_a_corner_is_within_the_guard_of_any_of_the_four() {
        let one = [r(0, 0, 1920, 1080)];
        for (x, y) in [(0.0, 0.0), (139.0, 0.0), (0.0, 139.0), (1919.0, 0.0), (1919.0, 1079.0), (1800.0, 1079.0), (0.0, 1000.0)] {
            assert!(near_corner(&one, at(x, y), CORNER_GUARD), "{x},{y}");
        }
        for (x, y) in [(0.0, 140.0), (140.0, 0.0), (0.0, 540.0), (1919.0, 540.0), (960.0, 0.0), (960.0, 1079.0), (100.0, 100.0 + 50.0)] {
            assert!(!near_corner(&one, at(x, y), CORNER_GUARD), "{x},{y}");
        }
        assert!(!near_corner(&one, at(-3.0, 0.0), CORNER_GUARD));
        let two = [r(0, 0, 1280, 720), r(1280, 0, 1920, 1080)];
        assert!(near_corner(&two, at(1280.0, 1079.0), CORNER_GUARD));
        assert!(!near_corner(&two, at(1280.0, 540.0), CORNER_GUARD));
    }

    #[test]
    fn a_move_that_only_arrives_at_the_edge_crosses_nowhere() {
        for far in [24.0, 60.0, 400.0] {
            let mut p = EdgePush::needing(CROSS_PUSH);
            assert_eq!(p.feed(None, 0.0, false, 990), None);
            assert_eq!(p.feed(Some((Side::Left, far)), 2.0, true, 1000), None, "{far} past the edge on arriving there");
            assert_eq!(p.feed(None, 3.0, false, 1010), None);
            assert_eq!(p.feed(Some((Side::Left, far)), 2.0, true, 1020), None, "{far} past the edge on arriving there again");
            assert_eq!(p.feed(Some((Side::Left, 50.0)), 2.0, true, 1030), Some(Side::Left));
        }
    }

    #[test]
    fn moving_down_along_the_edge_never_crosses() {
        let mut p = EdgePush::needing(CROSS_PUSH);
        for i in 0..200u32 {
            let drift = if i % 3 == 0 { 2.0 } else { 0.0 };
            let beyond = (drift > 0.0).then_some((Side::Left, drift));
            assert_eq!(p.feed(beyond, 8.0, true, 1000 + i * 8), None, "move {i}");
        }
    }

    #[test]
    fn being_near_the_edge_is_not_hitting_it() {
        let mut p = EdgePush::needing(CROSS_PUSH);
        for i in 0..100u32 {
            assert_eq!(p.feed(None, 1.0, false, 1000 + i * 8), None);
        }
    }

    #[test]
    fn the_sides_a_point_is_on() {
        let one = [r(0, 0, 1280, 720)];
        assert_eq!(outer_sides(&one, at(0.0, 0.0), 1), [Side::Left, Side::Top]);
        assert_eq!(outer_sides(&one, at(1279.4, 719.4), 1), [Side::Right, Side::Bottom]);
        assert_eq!(outer_sides(&one, at(640.0, 360.0), 1), []);
        let two = [r(0, 0, 1280, 720), r(1280, 0, 1920, 1080)];
        assert_eq!(outer_sides(&two, at(1279.4, 300.0), 1), []);
        assert_eq!(outer_sides(&two, at(3199.4, 300.0), 1), [Side::Right]);
        assert_eq!(outer_sides(&two, at(1280.0, 900.0), 1), [Side::Left]);
    }

    #[test]
    fn how_far_a_move_went_past_each_side() {
        let now = at(0.0, 300.0);
        assert_eq!(Side::Left.beyond(at(-30.0, 296.0), now), (30.0, 4.0));
        assert_eq!(Side::Right.beyond(at(-30.0, 296.0), now), (-30.0, 4.0));
        assert_eq!(Side::Top.beyond(at(5.0, 280.0), now), (20.0, 5.0));
        assert_eq!(Side::Bottom.beyond(at(5.0, 310.0), now), (10.0, 5.0));
    }

    #[test]
    fn crossing_needs_a_hard_push_and_any_side_will_do() {
        let mut p = EdgePush::needing(CROSS_PUSH);
        assert_eq!(p.feed(Some((Side::Top, 15.0)), 3.0, true, 990), None);
        assert_eq!(p.feed(Some((Side::Top, 15.0)), 3.0, true, 1000), None);
        assert_eq!(p.feed(Some((Side::Top, 15.0)), 3.0, true, 1010), None);
        assert_eq!(p.feed(Some((Side::Top, 20.0)), 3.0, true, 1020), Some(Side::Top));
        assert_eq!(p.feed(Some((Side::Top, 40.0)), 0.0, true, 1030), None);
        assert_eq!(p.feed(None, 0.0, false, 1100), None);
        assert_eq!(p.feed(Some((Side::Bottom, 30.0)), 0.0, true, 1190), None);
        assert_eq!(p.feed(Some((Side::Bottom, 30.0)), 0.0, true, 1200), None);
        assert_eq!(p.feed(Some((Side::Bottom, 30.0)), 0.0, true, 1210), Some(Side::Bottom));
        let mut q = EdgePush::needing(CROSS_PUSH);
        for i in 0..200u32 {
            let drift = if i % 3 == 0 { 2.0 } else { 0.0 };
            assert_eq!(q.feed((drift > 0.0).then_some((Side::Bottom, drift)), 8.0, true, 1000 + i * 8), None);
        }
    }

    #[test]
    fn little_pushes_far_apart_do_not_add_up() {
        let mut p = EdgePush::needing(CROSS_PUSH);
        for i in 0..20u32 {
            assert_eq!(p.feed(Some((Side::Right, 5.0)), 0.0, true, 1000 + i * 400), None);
        }
        let told: Vec<_> = (0..10u32).filter_map(|i| p.feed(Some((Side::Right, 5.0)), 0.0, true, 20000 + i * 10)).collect();
        assert_eq!(told, vec![Side::Right]);
    }
}
