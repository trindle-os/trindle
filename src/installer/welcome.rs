//! trindle-live-welcome.

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use crate::common::files::{set_setting, setting};
use crate::common::run::spawn;
use crate::common::style;
use crate::toolkit::colour::Colour;
use crate::toolkit::event::{Flow, Loop, Post};
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::tiles::tile_head;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{button, card, choice, column, drawing, form_label, headline, label, main_action, note, page_title, space,
                             row, Length, Node};
use crate::toolkit::window::{Display, Kind};

use super::SIZE;
const PICTURE_WIDTH: f32 = 360.0;
fn caption() -> &'static str {
    t!("installer-live-caption")
}
const FRAME: Duration = Duration::from_millis(50);
const STILL: Duration = Duration::from_secs(30);

#[derive(Clone)]
enum Msg {
    Look(bool),
    Later,
    Install,
}

struct Welcome {
    name: String,
    light: bool,
    voice: bool,
    vm: Option<(&'static str, String)>,
    accent: Colour,
    began: Instant,
    events: Loop,
    restyle: Post<()>,
}

impl App for Welcome {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Look(light) => {
                if light == self.light {
                    return;
                }
                self.light = light;
                let _ = set_setting("theme", if light { "light" } else { "dark" });
                self.restyle.send(());
                std::thread::spawn(|| {
                    style::apply_system_theme();
                    spawn(&["trindle-ring", "--restyle"]);
                });
            }
            Msg::Later => {
                spawn(&["trindle-ring"]);
                self.events.quit();
            }
            Msg::Install => {
                spawn(&["trindle-installer"]);
                self.events.quit();
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let (accent, began) = (self.accent, self.began);
        let poster = drawing((300.0, 300.0), caption(), move |canvas, pen| {
            super::poster::paint(canvas, pen, accent, began.elapsed().as_secs_f32() * super::poster::SPEED);
        }).width(Length::Fill).height(Length::Fill);
        let picture = panel("trindle-ring-symbolic", caption(), vec![poster]).width(Length::Fixed(PICTURE_WIDTH));

        let second = if self.voice {
            t!("installer-live-speak-line")
        } else {
            t!("installer-live-speak-add")
        };
        let steps = [
            ("input-mouse-symbolic", t!("installer-live-open"), t!("installer-live-open-line")),
            ("audio-input-microphone-symbolic", t!("installer-live-speak"), second),
            ("view-pin-symbolic", t!("installer-live-tab"), t!("installer-live-tab-line")),
        ];
        let rows: Vec<Node<Msg>> = steps.iter().enumerate().map(|(i, (icon, name, line))| {
            let r = row(vec![crate::settings::ui::icon(*icon, 24.0),
                             column(vec![form_label(*name), note(*line)]).gap(4.0).width(Length::Fill)])
                .gap(12.0).padding(8.0);
            (if i % 2 == 1 { r.banded() } else { card(r).snug().flat() }).height(Length::Fill)
        }).collect();
        let first = panel("view-app-grid-symbolic", t!("installer-live-begin"), vec![column(rows).gap(0.0).height(Length::Fill)]);

        let strip = choice(vec![t!("installer-live-dark").to_string(), t!("installer-live-light").to_string()], usize::from(self.light),
                           t!("installer-live-look"), |i| Msg::Look(i == 1));

        let mut right = vec![first.height(Length::Fill)];
        if let Some((heading, body)) = &self.vm {
            right.insert(0, panel("dialog-information-symbolic", heading, vec![label(body.clone()).width(Length::Fill)])
                .height(Length::Shrink));
        }
        let right = column(right).gap(theme::bento().gap).width(Length::Fill).height(Length::Fill);

        let head = column(vec![
            page_title(t!("installer-welcome-title", name = &self.name)),
            headline(t!("installer-live-help")).width(Length::Fill),
        ]).gap(4.0);
        let foot = row(vec![
            space().width(Length::Fill),
            strip,
            button("view-reveal-symbolic", t!("installer-live-around")).on_press(Msg::Later),
            main_action("system-software-install-symbolic", t!("installer-live-install", name = &self.name))
                .on_press(Msg::Install).first(),
        ]).gap(theme::sheet().gap);
        column(vec![
            head,
            row(vec![picture, right]).gap(theme::bento().gap).height(Length::Fill),
            foot,
        ]).gap(16.0).padding(theme::sheet().page_padding)
    }
}

fn panel(icon: &str, name: &str, body: Vec<Node<Msg>>) -> Node<Msg> {
    let mut all = vec![tile_head(icon, name, Vec::new())];
    all.extend(body);
    card(column(all).gap(theme::sheet().gap)).height(Length::Fill)
}

pub fn main(_args: Vec<String>) -> i32 {
    let brand = super::brand::brand();
    let events = Loop::new();
    let gone = events.clone();
    let Some(display) = Display::connect(&events, move || gone.quit()) else { return 1 };

    let slot: Rc<RefCell<Weak<Shown<Welcome>>>> = Rc::new(RefCell::new(Weak::new()));
    let s = slot.clone();
    let restyle = events.post(move |()| {
        if let Some(shown) = s.borrow().upgrade() {
            shown.restyle();
        }
    });
    let (r, g, b) = brand.accent;
    let vm = crate::common::graphics::virtual_machine().map(|m| crate::common::graphics::vm_note(&m));
    let app = Welcome {
        name: brand.name.clone(),
        light: setting("theme") == "light",
        voice: crate::voice::available(),
        vm: vm.clone(),
        accent: Colour::rgb(r as f32, g as f32, b as f32),
        began: Instant::now(),
        events: events.clone(),
        restyle,
    };
    let kind = Kind::Window { title: t!("installer-welcome-title", name = &brand.name), app_id: "org.trindle.Welcome".into() };
    let Some(shown) = Shown::open(&events, &display, kind, SIZE, app) else { return 1 };
    let shown = Rc::new(shown);
    *slot.borrow_mut() = Rc::downgrade(&shown);
    if vm.is_some() {
        shown.fit();
    }
    let ev = events.clone();
    shown.window.on_close(move || ev.quit());

    let w = Rc::downgrade(&shown.window);
    let every = if crate::common::graphics::moving() { FRAME } else { STILL };
    events.every(every, move || match w.upgrade() {
        Some(window) => {
            window.redraw();
            Flow::Continue
        }
        None => Flow::Stop,
    });
    events.run();
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;

    #[test]
    fn everything_fits_the_window() {
        let events = Loop::new();
        let mut app = Welcome {
            name: "Trindle".into(), light: false, voice: false, vm: None,
            accent: Theme::chosen().accent, began: Instant::now(),
            events: events.clone(), restyle: events.post(|()| {}),
        };
        let ui = Ui::new(app.view(), Theme::chosen());
        let h = ui.natural_height(SIZE.0 as f32);
        assert!(h <= SIZE.1 as f32, "{h}");
        app.vm = Some(crate::common::graphics::vm_note("QEMU"));
        let ui = Ui::new(app.view(), Theme::chosen());
        let h = ui.natural_height(SIZE.0 as f32);
        assert!(h <= 720.0, "{h}");
    }

    #[test]
    fn the_sticks_welcome_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::drawn;
        let events = Loop::new();
        drawn::none(drawn::in_every_look("the stick's welcome", 20, || {
            for (state, voice, vm) in [("no voice", false, None), ("voice, in a virtual machine", true, Some(crate::common::graphics::vm_note("QEMU")))] {
                let look = Theme::for_page("installer");
                let app = Welcome {
                    name: "Trindle".into(), light: look.light, voice, vm, accent: look.accent, began: Instant::now(),
                    events: events.clone(), restyle: events.post(|()| {}),
                };
                drawn::window(state, app.view(), look, (SIZE.0, 720), false);
            }
        }));
    }
}
