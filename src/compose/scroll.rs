//! How far a scroll goes in every program.

use smithay::backend::input::AxisSource;

use crate::config::Pointer;

const CLICK: f64 = 15.0;

pub fn factor(pointer: &Pointer, source: AxisSource) -> f64 {
    match source {
        AxisSource::Finger => pointer.touchpad_scroll,
        _ => pointer.scroll,
    }
}

#[derive(Debug, PartialEq)]
pub struct Scaled {
    pub value: f64,
    pub v120: Option<i32>,
}

#[derive(Debug, Default)]
pub struct Scaler {
    carry: [f64; 2],
}

impl Scaler {
    pub fn scale(&mut self, axis: usize, factor: f64, amount: Option<f64>, v120: Option<f64>) -> Scaled {
        let value = factor * amount.unwrap_or_else(|| v120.unwrap_or(0.0) * CLICK / 120.0);
        let v120 = v120.map(|v| {
            let carry = &mut self.carry[axis];
            if *carry * v < 0.0 {
                *carry = 0.0;
            }
            let total = v * factor + *carry;
            let whole = total.trunc();
            *carry = total - whole;
            whole as i32
        });
        Scaled { value, v120 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pointer(scroll: f64, touchpad_scroll: f64) -> Pointer {
        let mut p = Pointer::load();
        p.scroll = scroll;
        p.touchpad_scroll = touchpad_scroll;
        p
    }

    #[test]
    fn each_kind_takes_its_own_speed() {
        let p = pointer(2.0, 1.5);
        assert_eq!(factor(&p, AxisSource::Wheel), 2.0);
        assert_eq!(factor(&p, AxisSource::WheelTilt), 2.0);
        assert_eq!(factor(&p, AxisSource::Continuous), 2.0);
        assert_eq!(factor(&p, AxisSource::Finger), 1.5);
    }

    #[test]
    fn a_wheel_click_moves_both_figures_alike() {
        let mut s = Scaler::default();
        assert_eq!(s.scale(1, 2.0, Some(15.0), Some(120.0)), Scaled { value: 30.0, v120: Some(240) });
        assert_eq!(s.scale(1, 2.0, Some(-15.0), Some(-120.0)), Scaled { value: -30.0, v120: Some(-240) });
        assert_eq!(s.scale(0, 1.0, Some(15.0), Some(120.0)), Scaled { value: 15.0, v120: Some(120) });
    }

    #[test]
    fn a_device_that_gives_only_clicks_is_worth_fifteen_degrees_a_click() {
        let mut s = Scaler::default();
        assert_eq!(s.scale(1, 2.0, None, Some(120.0)), Scaled { value: 30.0, v120: Some(240) });
    }

    #[test]
    fn a_fine_wheel_loses_nothing_at_an_odd_speed() {
        for factor in [0.25, 1.5, 2.0, 2.75] {
            let mut s = Scaler::default();
            let sent: i32 = (0..8).map(|_| s.scale(1, factor, Some(15.0 / 8.0), Some(15.0)).v120.unwrap()).sum();
            assert_eq!(sent, (120.0 * factor) as i32, "at {factor}");
        }
    }

    #[test]
    fn whole_clicks_stay_whole_for_programs_that_count_steps() {
        let mut s = Scaler::default();
        let sent: Vec<i32> = (0..2).map(|_| s.scale(1, 1.5, Some(15.0), Some(120.0)).v120.unwrap()).collect();
        assert_eq!(sent, vec![180, 180]);
        assert_eq!(sent.iter().sum::<i32>() / 120, 3);
    }

    #[test]
    fn turning_back_drops_what_was_left_over() {
        let mut s = Scaler::default();
        assert_eq!(s.scale(1, 0.25, Some(1.0), Some(15.0)).v120, Some(3));
        assert_eq!(s.scale(1, 0.25, Some(-1.0), Some(-15.0)).v120, Some(-3));
        assert_eq!(s.scale(0, 0.25, Some(1.0), Some(15.0)).v120, Some(3));
    }

    #[test]
    fn a_touchpad_moves_smoothly_with_no_clicks() {
        let mut s = Scaler::default();
        assert_eq!(s.scale(1, 1.5, Some(4.0), None), Scaled { value: 6.0, v120: None });
        assert_eq!(s.scale(1, 1.5, Some(0.0), None), Scaled { value: 0.0, v120: None });
    }
}
