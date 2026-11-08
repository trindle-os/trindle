//! Which kind of controller this is, for the names printed on its buttons.

use super::{From, Pad};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Family {
    Xbox,
    PlayStation,
    Nintendo,
}

impl Family {

    pub fn of(pad: &Pad) -> Family {
        match pad.vendor {
            0x054c => Family::PlayStation,
            0x057e => Family::Nintendo,
            _ => Family::Xbox,
        }
    }

    pub fn label(self, from: From) -> Option<&'static str> {
        let pick = |xbox, ps, nintendo| match self {
            Family::Xbox => xbox,
            Family::PlayStation => ps,
            Family::Nintendo => nintendo,
        };
        Some(match from {
            From::Button(0x130) => pick("A", "Cross", "B"),
            From::Button(0x131) => pick("B", "Circle", "A"),
            From::Button(0x133) => pick("X", "Triangle", "X"),
            From::Button(0x134) => pick("Y", "Square", "Y"),
            From::Button(0x136) => pick("LB", "L1", "L"),
            From::Button(0x137) => pick("RB", "R1", "R"),
            From::Button(0x138) => pick("LT", "L2", "ZL"),
            From::Button(0x139) => pick("RT", "R2", "ZR"),
            From::Button(0x13a) => pick("View", "Share", "Minus"),
            From::Button(0x13b) => pick("Menu", "Options", "Plus"),
            From::Button(0x13c) => pick("Guide", "PS", "Home"),
            From::Button(0x13d) => pick("LS", "L3", "LS"),
            From::Button(0x13e) => pick("RS", "R3", "RS"),
            From::Trigger(2) => pick("LT", "L2", "ZL"),
            From::Trigger(5) => pick("RT", "R2", "ZR"),
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_names_follow_the_makers_numbers() {
        let pad = |vendor| Pad { vendor, product: 1, name: String::new() };
        assert_eq!(Family::of(&pad(0x054c)), Family::PlayStation);
        assert_eq!(Family::of(&pad(0x057e)), Family::Nintendo);
        assert_eq!(Family::of(&pad(0x11c0)), Family::Xbox);
        assert_eq!(Family::Xbox.label(From::Button(0x130)), Some("A"));
        assert_eq!(Family::PlayStation.label(From::Button(0x130)), Some("Cross"));
        assert_eq!(Family::Nintendo.label(From::Button(0x130)), Some("B"));
        assert_eq!(Family::Xbox.label(From::Direction(0, true)), None);
    }
}
