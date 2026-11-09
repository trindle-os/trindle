//! The parts every Settings page is built from: the page and its "i", cards, the line with its words and the glow that says a change was kept, the round button that becomes a ring while its job runs, and the button pressed twice.

mod clashes;
mod controls;
mod failures;
mod group;
mod icons;
mod kit;
mod line;
mod page;
mod trial;

pub use clashes::*;
pub use controls::*;
pub use failures::*;
pub use group::*;
pub use icons::*;
pub use kit::*;
pub use line::*;
pub use page::*;
pub use trial::*;

use std::any::Any;
use std::rc::Rc;

use super::Settings;
use crate::toolkit::widget::Node;

pub use crate::common::ui::SAVE_ICON;

pub const INSTALL_ICON: &str = "folder-download-symbolic";

pub const UPDATE_ICON: &str = "software-update-available-symbolic";

pub const UP_TO_DATE_ICON: &str = "view-refresh-symbolic";

#[derive(Clone)]
pub struct Msg(pub(crate) Rc<dyn Fn(&mut Settings)>);

pub(crate) enum Do {
    Then(Box<dyn FnOnce(&mut Settings)>),
    Show,
    Open(Vec<String>),
    Tick,
    Conf,

    Say(String),
}

pub trait AsAny {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> AsAny for T {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Page: AsAny {
    fn id(&self) -> &'static str;
    fn title(&self) -> &'static str;
    fn icon(&self) -> &'static str;

    fn keywords(&self) -> &'static str {
        ""
    }

    fn help(&self) -> &'static str {
        ""
    }

    fn listed(&self) -> bool {
        true
    }

    fn relist(&mut self) {}
    fn view(&self, kit: &Kit) -> Node<Msg>;

    fn shown(&mut self, _kit: &mut Kit) {}

    fn hidden(&mut self, _kit: &mut Kit) {}

    fn preload(&mut self, _kit: &mut Kit) {}

    fn open_topic(&mut self, _kit: &mut Kit, _id: &str) {}

    fn conf_changed(&mut self, _kit: &mut Kit) {}

    fn second(&mut self, _kit: &mut Kit) -> bool {
        false
    }

    fn moving(&self) -> bool {
        false
    }

    fn foot(&self, _kit: &Kit) -> Option<Node<Msg>> {
        None
    }

    fn intercept(&self, _input: &crate::toolkit::input::Input) -> Option<Msg> {
        None
    }
}

pub fn on<P: Page + 'static>(f: impl Fn(&mut P, &mut Kit) + 'static) -> Msg {
    Msg(Rc::new(move |s: &mut Settings| s.with::<P>(|p, k| f(p, k))))
}

fn on_kit(f: impl Fn(&mut Kit) + 'static) -> Msg {
    Msg(Rc::new(move |s: &mut Settings| f(&mut s.kit)))
}

#[cfg(test)]
pub(crate) mod stand;

#[cfg(test)]
mod tests;

