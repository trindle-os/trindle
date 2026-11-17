//! Making the report window fit a short screen: fewer lines in the box for what happened, then tighter gaps, then a shorter details pane, never a scroll.

use super::{Msg, Reporter, DETAILS_HIGH, DETAILS_LEAST, SIZE, WRITING_LINES_DETAILS_LEAST, WRITING_LINES, WRITING_LINES_LEAST};
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::{App, Ui};
use crate::toolkit::widget::Node;

fn known(room: Option<f32>) -> Option<f32> {
    room.filter(|r| r.is_finite() && *r > 0.0)
}

pub(super) fn natural(node: Node<Msg>) -> f32 {
    Ui::new(node, Theme::chosen()).natural_height(SIZE.0 as f32)
}

pub(super) fn pick(room: Option<f32>, least: usize, mut height: impl FnMut(usize, bool) -> f32) -> (usize, bool) {
    let Some(room) = known(room) else { return (WRITING_LINES, false) };
    for tight in [false, true] {
        if let Some(n) = (least..=WRITING_LINES).rev().find(|n| height(*n, tight) <= room) {
            return (n, tight);
        }
    }
    (least, true)
}

pub(super) fn details_for(over: f32) -> f32 {
    if !over.is_finite() || over <= 0.0 {
        return DETAILS_HIGH;
    }
    (DETAILS_HIGH - over).clamp(DETAILS_LEAST, DETAILS_HIGH)
}

impl Reporter {

    pub(super) fn fit_to(&mut self, room: Option<f32>) {
        self.details_high = DETAILS_HIGH;
        let (lines, tight) = self.roomiest(room, WRITING_LINES_LEAST);
        self.set_box(lines, tight);
        if let (true, Some(room)) = (self.details || self.picture.is_some(), known(room)) {
            if self.details {
                self.details_high = details_for(natural(self.view()) - room);
            }

            let (lines, tight) = self.roomiest(Some(room), WRITING_LINES_DETAILS_LEAST);
            self.set_box(lines, tight);
        }
    }

    fn roomiest(&mut self, room: Option<f32>, least: usize) -> (usize, bool) {
        pick(room, least, |n, tight| {
            self.set_box(n, tight);
            natural(self.view())
        })
    }

    pub(super) fn set_box(&mut self, lines: usize, tight: bool) {
        self.lines = lines;
        self.tight = tight;
        self.box_high = Some(natural(self.writing_alone(lines)));
    }

    fn writing_alone(&self, lines: usize) -> crate::toolkit::widget::Node<Msg> {
        crate::toolkit::widget::entry(String::new(), "", Msg::Typed).lines(lines)
    }
}
