//! The microphone switch in the middle of the ring: off, every microphone
//! is muted for every program, boxed, Flatpak, plugin or not, until it is
//! pressed again. The compositor keeps them muted; this only sets the mark.

use crate::common::in_use;
use crate::toolkit::widget::{button, Node};

use super::ctx::{msg, Msg};
use crate::toolkit::t;

pub fn off_tip() -> &'static str {
    t!("ring-mic-off-tip")
}
pub fn on_tip() -> &'static str {
    t!("ring-mic-on-tip")
}

pub fn not_own() -> &'static str {
    t!("ring-mic-own-press")
}

#[derive(Debug, PartialEq, Eq)]
pub enum Step {
    Set(bool),
    Refuse,
}

pub fn step(off: bool, own: bool) -> Step {
    match (off, own) {
        (false, _) => Step::Set(true),
        (true, true) => Step::Set(false),
        (true, false) => Step::Refuse,
    }
}

pub fn switch(size: f32) -> Node<Msg> {
    let off = in_use::microphone_off();
    let (icon, tip) = if off { ("microphone-sensitivity-muted-symbolic", on_tip()) } else { ("audio-input-microphone-symbolic", off_tip()) };
    button(icon, tip).size(size).on(off).on_press(msg(|f| {
        match step(in_use::microphone_off(), super::kept::own_press()) {
            Step::Refuse => f.say(not_own(), 5),
            Step::Set(off) => {
                if let Err(e) = in_use::set_microphone_off(off) {
                    f.say(&t!("ring-mic-not-changed", why = e.to_string()), 5);
                }
            }
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_persons_own_press_turns_it_back_on() {
        assert_eq!(step(false, false), Step::Set(true));
        assert_eq!(step(false, true), Step::Set(true));
        assert_eq!(step(true, true), Step::Set(false));
        assert_eq!(step(true, false), Step::Refuse);
    }
}
