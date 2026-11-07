//! What the chooser does with each message, and the keys it listens for.

use crate::common::kinds::parts;
use crate::toolkit::{popup, t, theme};
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::popup::{Done, PopupMsg};
use crate::toolkit::ui::{App, Keyboard};
use crate::toolkit::widget::Node;

use super::{Act, Answer, Chooser, Clash, Mode, Msg, On, Outcome, SIDEBAR, Where};

impl App for Chooser {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        self.act_on(msg);
        self.look_taken();
    }

    fn intercept_with(&self, input: &Input, at: Keyboard) -> Option<Msg> {
        match input {
            Input::PointerMove { x, y } | Input::Press { x, y, .. } => {
                self.pointer.set((*x, *y));
                None
            }
            Input::Key { sym, .. } if self.menu.is_some() => Some(Msg::Pop(popup::key(*sym))),
            Input::Key { sym: Keysym::F2, .. } if !at.text && self.renaming.is_none() => Some(Msg::Rename),
            Input::Key { sym, mods, .. } if !at.text && self.renaming.is_none() && self.asking.is_none() && self.deleting.is_none() && act_key(*sym, *mods).is_some() => act_key(*sym, *mods).map(Msg::Key),
            Input::Key { sym, mods, .. } if !at.text && self.chosen.is_none() && self.renaming.is_none()
                && (*sym == Keysym::Menu || (*sym == Keysym::F10 && mods.shift && !mods.ctrl)) => Some(Msg::RoomMenu(true)),
            _ => None,
        }
    }

    fn view(&self) -> Node<Msg> {
        self.drawn()
    }
}

fn act_key(sym: Keysym, mods: crate::toolkit::input::Mods) -> Option<Act> {
    if mods.alt {
        return None;
    }
    match sym {
        Keysym::c | Keysym::C if mods.ctrl && !mods.shift => Some(Act::Copy),
        Keysym::x | Keysym::X if mods.ctrl && !mods.shift => Some(Act::Cut),
        Keysym::v | Keysym::V if mods.ctrl && !mods.shift => Some(Act::Paste),
        Keysym::Delete | Keysym::KP_Delete if !mods.ctrl => Some(if mods.shift { Act::Delete } else { Act::Trash }),
        _ => None,
    }
}

impl Chooser {
    fn act_on(&mut self, msg: Msg) {
        match msg {
            Msg::Place(i) => self.go_to_place(i),
            Msg::Part(i) => self.go_to_part(i),
            Msg::Choose(i) => self.choose(i),
            Msg::Pick(p) => self.picked = p,
            Msg::Open(i) => self.open(i),
            Msg::Name(n) => self.name_typed(n),
            Msg::Filter(i) => self.filter_chosen(i),
            Msg::Choice(i, j) => self.choice_made(i, j),
            Msg::Toggle(i, on) => self.toggled(i, on),
            Msg::Accept => self.accept_if_ready(true),
            Msg::Submit => self.accept_if_ready(false),
            Msg::Cancel => self.finish(Outcome::Cancelled),
            Msg::Changed => self.list(),
            Msg::Hidden => self.toggle_hidden(),
            Msg::NewFolder => self.make_folder(),
            Msg::Menu(i, x, y) => self.open_menu(Some(i), (x, y), false),
            Msg::RoomMenu(keys) => self.open_room_menu(keys),
            Msg::Pop(m) => self.menu_answered(m),
            Msg::Rename => self.act(Act::Rename, On::Keys),
            Msg::RenameText(text) => self.rename_text(text),
            Msg::RenameDone => self.rename_end(true),
            Msg::RenameLeft => self.rename_end(false),
            Msg::RenameStop => self.renaming = None,
            Msg::Pasted(got, to) => self.pasted(got, to),
            Msg::Worked(failed, verb) => self.worked(failed, verb),
            Msg::Began(start) => self.has_begun(start),
            Msg::Candidates(named) => self.candidates(named),
            Msg::Checked(rank, place) => self.checked(rank, place),
            Msg::Late(waited) => self.late(waited),
            Msg::Listed(entries, mark, at, hidden) => self.listed(entries, mark, at, hidden),
            Msg::Seen(taken, target) => self.seen_taken(taken, target),
            Msg::Target(found, target, pressed) => self.save_to(found, target, pressed),
            Msg::Named(paths, into) => self.named(paths, into),
            Msg::Made(made) => self.folder_was_made(made),
            Msg::Renamed(done, change, asked) => self.renamed(done, change, asked),
            Msg::Asked(clash) => self.asked(clash),
            Msg::Answer(answer) => self.answer(answer),
            Msg::SameForRest(on) => self.same = on,
            Msg::AskStop => self.stop_asking(),
            Msg::Key(act) => self.keyed(act),
            Msg::DeleteSure => self.delete_sure(),
            Msg::DeleteNo => self.deleting = None,
            Msg::Still => {}
        }
    }

    fn go_to_place(&mut self, i: usize) {
        if let Some(at) = self.places.get(i).map(|p| p.at.clone()) {
            self.go(at);
        }
    }

    fn go_to_part(&mut self, i: usize) {
        if let Some((_, path)) = self.folder().and_then(|f| parts(f, &self.home).into_iter().nth(i)) {
            self.go(Where::Folder(path));
        }
    }

    fn choose(&mut self, i: usize) {
        self.chosen = Some(i);
        if !matches!(self.ask.mode, Mode::Open { many: true, .. }) {
            self.picked = vec![i];
        }
        if let (Mode::Save, Some(e)) = (&self.ask.mode, self.entries.get(i)) {
            if !e.folder {
                self.name = e.name.clone();
                self.armed = None;
            }
        }
    }

    fn name_typed(&mut self, n: String) {
        self.name = n;
        self.armed = None;
        self.said.clear();
    }

    fn choice_made(&mut self, i: usize, j: usize) {
        if let Some(p) = self.ask.choices.get_mut(i) {
            if let Some((id, _)) = p.options.get(j) {
                p.value = id.clone();
            }
        }
    }

    fn filter_chosen(&mut self, i: usize) {
        self.ask.filter = Some(i);
        self.list();
    }

    fn toggled(&mut self, i: usize, on: bool) {
        if let Some(p) = self.ask.choices.get_mut(i) {
            p.value = on.to_string();
        }
    }

    fn toggle_hidden(&mut self) {
        self.hidden = !self.hidden;
        self.list();
    }

    fn rename_text(&mut self, text: String) {
        if let Some(r) = &mut self.renaming {
            r.text = text;
            r.why = None;
        }
    }

    fn stop_asking(&mut self) {
        self.same = true;
        self.answer(Answer::Skip);
    }

    fn accept_if_ready(&mut self, pressed: bool) {
        if self.ready() {
            self.accept(pressed);
        }
    }

    fn open_room_menu(&mut self, keys: bool) {
        let at = if keys { (SIDEBAR + 4.0 * theme::sheet().panel_gap, 3.0 * theme::sheet().row) } else { self.pointer.get() };
        self.open_menu(None, at, keys);
    }

    fn menu_answered(&mut self, m: PopupMsg) {
        let Some((menu, acts)) = &mut self.menu else { return };
        match menu.update(m) {
            Done::Open => {}
            Done::Closed | Done::PickedIn(..) => {
                self.menu = None;
                self.menu_on = None;
            }
            Done::Picked(i) => {
                let act = acts.get(i).copied();
                self.menu = None;
                let on = self.menu_on.take();
                if let Some(act) = act {
                    self.act(act, On::Menu(on));
                }
            }
        }
    }

    fn asked(&mut self, clash: Clash) {
        self.asking = Some(clash);
        self.same = false;
        self.questions += 1;
    }

    fn answer(&mut self, answer: Answer) {
        if self.asking.take().is_some() {
            if let Some(reply) = &self.reply {
                let _ = reply.send((answer, self.same));
            }
        }
        self.same = false;
    }

    fn keyed(&mut self, act: Act) {
        let on = self.chosen.or_else(|| (!self.picked.is_empty()).then_some(0));
        if !self.acts(on).contains(&act) {
            self.said = if self.folder().is_none() { t!("screenshot-chooser-recent-not-folder").into() } else { t!("screenshot-chooser-nothing-picked").into() };
            return;
        }
        self.act(act, On::Keys);
    }
}
