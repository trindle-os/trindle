//! The question that takes the middle's place until it is answered.

use zeroize::Zeroizing;

use crate::ring::ctx::Msg;
use crate::settings::ui::icon;
use crate::toolkit::widget::{button, column, entry, label, Length, Node, row, title};

use super::{Ask, big_line, FRAME_GAP, m, NO_ICON, on_disc, Quick, Well};

const ASK_GAP: f32 = 12.0;

const ASK_ICON: f32 = 20.0;

const ASK_ENTRY_WIDTH: f32 = 150.0;

impl Quick {
    pub(super) fn ask_view(&self, a: &Ask) -> Node<Msg> {
        let mut items = vec![icon(a.icon, ASK_ICON), title(a.title.clone())];
        if let Some(code) = &a.code {
            items.push(big_line(code.clone()));
        }
        if let Some((hint, secret)) = a.entry {
            let mut e = entry(a.text.as_str(), hint, |t| m(move |q| q.ask_typed(&t))).key("ring:answer")
                .on_submit(m(|q| q.answer(true))).grab(self.turn).width(Length::Fixed(ASK_ENTRY_WIDTH));
            if secret {
                e = e.secret();
            }
            items.push(e);
        }
        items.push(label(a.words));
        let mut answers = vec![button(NO_ICON, a.no).on_press(m(|q| q.answer(false))).well()];
        if !a.yes.is_empty() {
            answers.push(crate::toolkit::widget::word_button(a.yes).on_press(m(|q| q.answer(true))));
        }
        items.push(row(answers).gap(ASK_GAP).width(Length::Shrink));
        on_disc(column(items).gap(FRAME_GAP).centred(), self.across.map(|a| self.pop_size(a as i32) as f32))
            .width(self.pop_width.map_or(Length::Fill, Length::Fixed)).on_escape(m(|q| q.answer(false)))
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn ask(&mut self, icon: &'static str, title: String, words: &'static str, code: Option<String>,
           entry: Option<(&'static str, bool)>, yes: &'static str, no: &'static str,
           done: impl FnOnce(&mut Quick, Option<Zeroizing<String>>) + 'static) {
        self.cancel_ask();
        self.turn += 1;
        self.ask = Some(Ask { icon, title, words, code, entry, yes, no, text: Zeroizing::new(String::new()),
                              done: Some(Box::new(done)) });
    }

    fn ask_typed(&mut self, text: &str) {
        if let Some(a) = &mut self.ask {
            a.text = Zeroizing::new(text.to_string());
        }
    }

    fn answer(&mut self, yes: bool) {
        let Some(mut a) = self.ask.take() else { return };
        let text = std::mem::replace(&mut a.text, Zeroizing::new(String::new()));
        if let Some(done) = a.done.take() {
            done(self, if yes { Some(text) } else { None });
        }
    }

    pub(super) fn cancel_ask(&mut self) {
        self.answer(false);
    }
}
