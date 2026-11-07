//! The window that asks what a program may share of the screen.

use std::rc::Rc;

use tiny_skia::Pixmap;

use super::cast_capture::{Screen, Window};
use super::portal_ask::{paint_picture, ACCEPT_ICON, CANCEL_ICON};
use crate::toolkit::t;
use crate::toolkit::ui::App;
use crate::toolkit::widget::{button, card, column, drawing, even_grid, heading, headline, label, main_action, note, row, scroll, space, title, Length, Node};

pub const SIZE: (u32, u32) = (760, 600);
const CARD: f32 = 210.0;
const PICTURE: (f32, f32) = (184.0, 104.0);
const DESKTOP_ICON: &str = "video-display-symbolic";

#[derive(Clone, Debug, PartialEq)]
pub enum Choice {
    Desktop,
    Screen(Screen),
    Window(Window, Option<String>),
}

impl Choice {
    fn name(&self) -> String {
        match self {
            Choice::Desktop => t!("screenshot-pick-desktop").into(),
            Choice::Screen(s) if !s.description.is_empty() => s.description.clone(),
            Choice::Screen(s) => s.name.clone(),
            Choice::Window(w, _) if !w.title.is_empty() => w.title.clone(),
            Choice::Window(w, _) => w.app_id.clone(),
        }
    }

    fn detail(&self) -> String {
        match self {
            Choice::Desktop => t!("screenshot-pick-desktop-detail").into(),
            Choice::Screen(s) => t!("screenshot-pick-size", width = s.pixels.0, height = s.pixels.1),
            Choice::Window(w, _) => super::portal_ask::app_name(&w.app_id),
        }
    }

    fn icon(&self) -> String {
        match self {
            Choice::Desktop | Choice::Screen(_) => DESKTOP_ICON.into(),
            Choice::Window(w, icon) => icon.clone().unwrap_or_else(|| crate::common::apps::generic_icon_for(&w.app_id, &w.title)),
        }
    }
}

pub type Picked = Option<Vec<Choice>>;

#[derive(Clone, Debug)]
pub enum Msg {
    Pick(usize),
    Share,
    Refuse,
}

pub struct Picker {
    app: String,
    pub choices: Vec<Choice>,
    pub pictures: Vec<Option<Rc<Pixmap>>>,
    picked: Vec<usize>,
    multiple: bool,
    done: Option<Box<dyn FnOnce(Picked)>>,
}

impl Picker {
    pub fn new(app: String, choices: Vec<Choice>, multiple: bool, done: impl FnOnce(Picked) + 'static) -> Picker {
        let pictures = vec![None; choices.len()];
        Picker { app, choices, pictures, picked: Vec::new(), multiple, done: Some(Box::new(done)) }
    }

    fn finish(&mut self, picked: Picked) {
        if let Some(done) = self.done.take() {
            done(picked);
        }
    }

    fn tile(&self, i: usize) -> Node<Msg> {
        let choice = &self.choices[i];
        let picture = self.pictures[i].clone();
        let icon = choice.icon();
        let tip = choice.name();
        let face = drawing(PICTURE, tip.clone(), move |canvas, pen| match &picture {
            Some(px) => paint_picture(canvas, pen, px, false),
            None => {
                let size = (canvas.width().min(canvas.height()) / 2).max(1);
                let (x, y) = ((canvas.width() - size) / 2, (canvas.height() - size) / 2);
                let _ = pen.icons.draw(canvas, &icon, x as i32, y as i32, size, 1, pen.theme.fg);
            }
        });
        let picked = self.picked.contains(&i);
        let words = column(vec![label(choice.name()).cut(), note(choice.detail()).cut()]).gap(4.0);
        let mut c = card(column(vec![row(vec![space(), face, space()]), words]).gap(8.0))
            .on_press(Msg::Pick(i))
            .lit(if picked { 1.0 } else { 0.0 });
        c = c.spoken(if picked { t!("screenshot-pick-chosen", name = &tip) } else { tip });
        c
    }
}

impl App for Picker {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Pick(i) if i < self.choices.len() => {
                match self.picked.iter().position(|&p| p == i) {
                    Some(at) => {
                        self.picked.remove(at);
                    }
                    None if self.multiple => self.picked.push(i),
                    None => self.picked = vec![i],
                }
            }
            Msg::Pick(_) => {}
            Msg::Share if !self.picked.is_empty() => {
                let picked = self.picked.iter().map(|&i| self.choices[i].clone()).collect();
                self.finish(Some(picked));
            }
            Msg::Share => {}
            Msg::Refuse => self.finish(None),
        }
    }

    fn view(&self) -> Node<Msg> {
        let screens: Vec<usize> = (0..self.choices.len()).filter(|&i| !matches!(self.choices[i], Choice::Window(..))).collect();
        let windows: Vec<usize> = (0..self.choices.len()).filter(|&i| matches!(self.choices[i], Choice::Window(..))).collect();
        let mut body: Vec<Node<Msg>> = Vec::new();
        if !screens.is_empty() {
            body.push(heading(t!("screenshot-pick-screens")));
            body.push(even_grid(CARD, screens.iter().map(|&i| self.tile(i)).collect()).gap(12.0));
        }
        if !windows.is_empty() {
            body.push(heading(t!("screenshot-pick-windows")));
            body.push(even_grid(CARD, windows.iter().map(|&i| self.tile(i)).collect()).gap(12.0));
        }
        if self.choices.is_empty() {
            body.push(label(t!("screenshot-pick-nothing")));
        }
        let ask = if self.multiple {
            t!("screenshot-pick-many")
        } else {
            t!("screenshot-pick-one")
        };
        column(vec![
            title(t!("screenshot-pick-title", app = &self.app)),
            headline(ask),
            scroll("share", column(body).gap(12.0)).height(Length::Fill),
            row(vec![
                space(),
                button(CANCEL_ICON, t!("screenshot-pick-refuse")).on_press(Msg::Refuse),
                main_action(ACCEPT_ICON, t!("screenshot-pick-share")).on_press(Msg::Share).off(self.picked.is_empty()),
            ]).gap(8.0),
        ])
        .gap(12.0)
        .padding(16.0)
        .on_escape(Msg::Refuse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    fn screen(name: &str) -> Choice {
        Choice::Screen(Screen { name: name.into(), description: String::new(), place: [0, 0, 10, 10], pixels: (10, 10) })
    }

    #[test]
    fn the_picker_of_what_to_share_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        let mut still = Pixmap::new(64, 36).unwrap();
        still.fill(tiny_skia::Color::from_rgba8(90, 120, 60, 255));
        let still = Rc::new(still);
        drawn::none(drawn::in_every_look("the picker of what to share", 8, || {
            let mut p = Picker::new("Brave".into(), vec![Choice::Desktop, screen("DP-1"), screen("HDMI-A-1")], true, |_| {});
            drawn::window("nothing chosen", p.view(), Theme::chosen(), SIZE, false);
            p.pictures[0] = Some(still.clone());
            p.update(Msg::Pick(1));
            drawn::window("one chosen", p.view(), Theme::chosen(), (1280, 800), false);
        }));
    }

    #[test]
    #[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
    fn shoot_the_picker() {
        use crate::toolkit::theme::Theme;
        use crate::toolkit::ui::Ui;
        let Some(dir) = std::env::var_os("TRINDLE_SHOTS") else { return };
        let mut p = Picker::new("Brave".into(), vec![Choice::Desktop, screen("DP-1"), screen("HDMI-A-1")], true, |_| {});
        p.update(Msg::Pick(1));
        for (w, h) in [(720u32, 520u32), (1280, 800), (1920, 1080)] {
            let mut ui = Ui::new(p.view(), Theme::new(false, crate::common::style::DEFAULT_ACCENT));
            let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            pix.save_png(std::path::Path::new(&dir).join(format!("picker-{w}.png"))).unwrap();
        }
    }

    #[test]
    fn one_pick_replaces_another_unless_several_are_allowed() {
        let got: Rc<RefCell<Option<Picked>>> = Rc::new(RefCell::new(None));
        let g = got.clone();
        let mut p = Picker::new("Brave".into(), vec![Choice::Desktop, screen("A"), screen("B")], false, move |x| *g.borrow_mut() = Some(x));
        p.update(Msg::Share);
        assert!(got.borrow().is_none(), "nothing picked, nothing shared");
        p.update(Msg::Pick(1));
        p.update(Msg::Pick(2));
        p.update(Msg::Share);
        assert_eq!(*got.borrow(), Some(Some(vec![screen("B")])));
        let _ = p.view();

        let g = got.clone();
        let mut p = Picker::new("Brave".into(), vec![screen("A"), screen("B")], true, move |x| *g.borrow_mut() = Some(x));
        p.update(Msg::Pick(0));
        p.update(Msg::Pick(1));
        p.update(Msg::Share);
        assert_eq!(*got.borrow(), Some(Some(vec![screen("A"), screen("B")])));

        let g = got.clone();
        let mut p = Picker::new("Brave".into(), vec![screen("A")], true, move |x| *g.borrow_mut() = Some(x));
        p.update(Msg::Refuse);
        assert_eq!(*got.borrow(), Some(None));
    }
}
