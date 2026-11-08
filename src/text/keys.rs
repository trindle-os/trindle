//! Edits, shortcuts and the messages the window sends back.

use crate::toolkit::editing::editor::Did;
use crate::toolkit::input::{Keysym, Mods};
use super::{Bar, Do, Msg, Text, View};

impl Text {
    pub(super) fn edited(&mut self, did: Did) {
        match did {
            Did::Copy(s) => {
                if let Some(w) = self.window.upgrade() {
                    w.copy(&s);
                }
            }
            Did::Paste => {
                if let Some(w) = self.window.upgrade() {
                    let asks = self.asks.clone();
                    w.paste(move |t| {
                        asks.send(Do::Paste(t));
                    });
                }
            }
            Did::Edited => {
                self.said = None;
                if self.bar == Bar::Spell {
                    self.bar = Bar::None;
                }
            }
            Did::Moved | Did::Nothing => {}
        }
        if matches!(self.bar, Bar::Find | Bar::Replace) {
            self.mark_found();
        }
    }

    pub(super) fn shortcut(&self, sym: Keysym, mods: Mods) -> Option<Msg> {
        if sym == Keysym::Escape {
            return (self.bar != Bar::None || self.view != View::Edit).then_some(Msg::Back);
        }
        if sym == Keysym::F3 {
            return (self.view == View::Edit).then_some(Msg::Next(!mods.shift));
        }
        let menu = sym == Keysym::Menu || (sym == Keysym::F10 && mods.shift && !mods.ctrl && !mods.alt);
        if menu && self.view == View::Edit && !self.shown() && self.pane().borrow().editing {
            return Some(Msg::SpellHere);
        }
        if !mods.ctrl || mods.alt || mods.logo {
            return None;
        }
        let editing = self.view == View::Edit;
        Some(match sym {
            Keysym::n | Keysym::N => Msg::New,
            Keysym::o | Keysym::O => Msg::Open,
            Keysym::s if !mods.shift => Msg::Save,
            Keysym::s | Keysym::S => Msg::SaveAs,
            Keysym::w | Keysym::W => Msg::CloseTab(self.active),
            Keysym::e | Keysym::E if self.tabs[self.active].markdown => Msg::Finished,
            Keysym::q | Keysym::Q => Msg::Quit,
            Keysym::f | Keysym::F if editing => Msg::Bar(Bar::Find),
            Keysym::h | Keysym::H if editing => Msg::Bar(Bar::Replace),
            Keysym::g | Keysym::G | Keysym::l | Keysym::L if editing => Msg::Bar(Bar::Goto),
            Keysym::Page_Up => Msg::NextTab(false),
            Keysym::Page_Down => Msg::NextTab(true),
            Keysym::plus | Keysym::equal | Keysym::KP_Add => Msg::Zoom(1),
            Keysym::minus | Keysym::KP_Subtract => Msg::Zoom(-1),
            Keysym::_0 | Keysym::KP_0 => Msg::Zoom(0),
            _ => return None,
        })
    }

    pub(super) fn handle(&mut self, ask: Do) {
        match ask {
            Do::Open(args) => self.open_args(&args),
            Do::Chosen(purpose, paths) => self.chosen(purpose, paths),
            Do::Exported(done) => self.exported(done),
            Do::Speller(lang, read) => self.speller_read(lang, read),
            Do::Paste(t) => {
                let mut pane = self.pane().borrow_mut();
                if pane.editing {
                    pane.paste(&t);
                }
                drop(pane);
                self.edited(Did::Edited);
            }
            Do::SaveSettings(turn) => {
                if turn == self.settings_turn {
                    self.save_settings();
                }
            }
            Do::NoteGone(turn) => {
                if self.noted.is_some_and(|t| t.1 == turn) {
                    self.noted = None;
                }
            }
            Do::CloseWindow => {
                if self.view == View::Closing {
                    return;
                }
                self.quit();
            }
            Do::Stop => {
                self.keep();
                self.events.quit();
            }
            Do::Show | Do::Restyle => unreachable!("handled in window(), before this"),
        }
    }
}
