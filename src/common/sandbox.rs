//! The names a box's own Wayland socket carries (wp_security_context_v1's
//! sandbox engine), shared by trindle-plugin, which makes the socket, and
//! the compositor, which reads them. Only a program outside every box can
//! make such a socket, so the name can be trusted.

pub const BOXED: &str = "org.trindle.boxed";

pub const CONTROL: &str = "org.trindle.boxed.control";

pub const CAPTURE: &str = "org.trindle.boxed.capture";

pub const CONTROL_CAPTURE: &str = "org.trindle.boxed.control.capture";

pub fn engine(control: bool, capture: bool) -> &'static str {
    match (control, capture) {
        (false, false) => BOXED,
        (true, false) => CONTROL,
        (false, true) => CAPTURE,
        (true, true) => CONTROL_CAPTURE,
    }
}

pub fn abilities(engine: &str) -> (bool, bool) {
    match engine {
        CONTROL => (true, false),
        CAPTURE => (false, true),
        CONTROL_CAPTURE => (true, true),
        _ => (false, false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_name_says_exactly_what_it_was_allowed() {
        for control in [false, true] {
            for capture in [false, true] {
                assert_eq!(abilities(engine(control, capture)), (control, capture));
            }
        }

        for odd in ["org.trindle.boxed.capture.control", "org.trindle.boxed.controlx", "org.trindle", "", "capture"] {
            assert_eq!(abilities(odd), (false, false), "{odd}");
        }
    }
}
