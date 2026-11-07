//! What the portal's smaller dialogs share.

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use tiny_skia::{FilterQuality, Paint, Pattern, Pixmap, PixmapMut, SpreadMode, Transform};
use zbus::zvariant::{OwnedValue, Value};

use super::portal::{answer, Stay, CANCELLED, FAILED};
use crate::toolkit::bus::{Asked, Bus, Offer, Props, Served};
use crate::toolkit::event::Loop;
use crate::toolkit::t;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{button, one_of, column, drawing, entry, label, main_action, note, row, space, switch, title, Length, Node, Pen, Rect};
use crate::toolkit::window::{Display, Kind};

pub(super) const REQUEST: &str = "org.freedesktop.impl.portal.Request";
pub(super) const REQUEST_XML: &str = r#"<interface name="org.freedesktop.impl.portal.Request">
    <method name="Close"/>
  </interface>"#;
pub(super) const APP_ID: &str = "org.trindle.Portal";
pub(super) const CANCEL_ICON: &str = "window-close-symbolic";
pub(super) const ACCEPT_ICON: &str = crate::common::no_tick::GO_ICON;

pub(super) type Options = HashMap<String, OwnedValue>;
pub(super) type Answer = (u32, HashMap<&'static str, Value<'static>>);
pub(super) type Done = Rc<dyn Fn(Answer)>;

pub(super) fn get<T: TryFrom<OwnedValue>>(options: &Options, key: &str) -> Option<T> {
    T::try_from(options.get(key)?.try_clone().ok()?).ok()
}

pub(super) fn cancelled() -> Answer {
    answer(CANCELLED, HashMap::new())
}

pub(super) fn failed() -> Answer {
    answer(FAILED, HashMap::new())
}

pub(super) fn app_name(app_id: &str) -> String {
    if app_id.is_empty() || app_id.contains('/') {
        return app_id.to_string();
    }
    crate::common::apps::app_dirs().iter()
        .map(|d| d.join(format!("{app_id}.desktop")))
        .find(|p| p.is_file())
        .and_then(|p| crate::common::keyfile::KeyFile::load(&p).ok())
        .and_then(|k| k.string("Desktop Entry", "Name"))
        .unwrap_or_else(|| app_id.to_string())
}

pub(super) fn plain_label(words: &str) -> String {
    words.replace("__", "\0").replace('_', "").replace("\0", "_")
}

pub(super) type Choice = (String, String, Vec<(String, String)>, String);

pub(super) fn choices_of(options: &Options) -> Vec<Choice> {
    get::<Vec<Choice>>(options, "choices").unwrap_or_default().into_iter().map(|(id, label, options, value)| {
        let value = match (options.is_empty(), value.as_str()) {
            (true, "true") => value,
            (true, _) => "false".into(),
            (false, _) if options.iter().any(|(o, _)| *o == value) => value,
            (false, _) => options.first().map(|(o, _)| o.clone()).unwrap_or_default(),
        };
        (id, label, options, value)
    }).collect()
}

pub(super) fn chosen(parts: &[Part]) -> Vec<(String, String)> {
    parts.iter().filter_map(|p| match p {
        Part::Pick { id, value, .. } => Some((id.clone(), value.clone())),
        _ => None,
    }).collect()
}

type Screen = Rc<RefCell<Option<Rc<Display>>>>;

#[derive(Clone)]
pub(super) struct Kit {
    pub bus: Bus,
    pub events: Loop,
    pub stay: Rc<Stay>,
    held: Screen,
}

impl Kit {
    pub fn new(bus: &Bus, events: &Loop, stay: &Rc<Stay>) -> Kit {
        Kit { bus: bus.clone(), events: events.clone(), stay: stay.clone(), held: Rc::new(RefCell::new(None)) }
    }

    pub(super) fn screen(&self) -> Option<Rc<Display>> {
        if let Some(d) = self.held.borrow().as_ref() {
            return Some(d.clone());
        }
        let gone = self.held.clone();
        let display = Display::connect(&self.events, move || {
            gone.borrow_mut().take();
        })?;
        *self.held.borrow_mut() = Some(display.clone());
        Some(display)
    }

    pub fn hold(&self) {
        self.stay.busy.set(self.stay.busy.get() + 1);
    }

    pub fn release(&self) {
        self.stay.busy.set(self.stay.busy.get().saturating_sub(1));
        self.stay.last.set(Instant::now());
    }
}

struct Request {
    asked: Option<Asked>,
    code_only: bool,
    shown: Option<Rc<dyn Any>>,
    close: Option<Served>,
}

fn finish(request: &Rc<RefCell<Request>>, kit: &Kit, outcome: Answer) {
    let (asked, code_only) = {
        let mut r = request.borrow_mut();
        (r.asked.take(), r.code_only)
    };
    let Some(asked) = asked else { return };
    match code_only {
        true => asked.reply(&(outcome.0,)),
        false => asked.reply(&outcome),
    }
    kit.release();
    let request = request.clone();
    kit.events.after(Duration::ZERO, move || {
        let mut r = request.borrow_mut();
        r.shown = None;
        r.close = None;
    });
}

#[allow(clippy::too_many_arguments)]
pub(super) fn ask<A: App + 'static>(kit: &Kit, asked: Asked, handle: &str, parent: &str, words: &str, size: (u32, u32),
                                    make: impl FnOnce(Done) -> A) -> Option<Rc<Shown<A>>> where A::Msg: Clone + 'static {
    show(kit, asked, false, handle, parent, words, size, make)
}

#[allow(clippy::too_many_arguments)]
pub(super) fn ask_code<A: App + 'static>(kit: &Kit, asked: Asked, handle: &str, parent: &str, words: &str, size: (u32, u32),
                                         make: impl FnOnce(Done) -> A) -> Option<Rc<Shown<A>>> where A::Msg: Clone + 'static {
    show(kit, asked, true, handle, parent, words, size, make)
}

#[allow(clippy::too_many_arguments)]
fn show<A: App + 'static>(kit: &Kit, asked: Asked, code_only: bool, handle: &str, parent: &str, words: &str, size: (u32, u32),
                          make: impl FnOnce(Done) -> A) -> Option<Rc<Shown<A>>> where A::Msg: Clone + 'static {
    let Some(display) = kit.screen() else {
        match code_only {
            true => asked.reply(&(FAILED,)),
            false => asked.reply(&failed()),
        }
        return None;
    };
    kit.hold();
    let request = Rc::new(RefCell::new(Request { asked: Some(asked), code_only, shown: None, close: None }));
    let done: Done = {
        let (request, kit) = (request.clone(), kit.clone());
        Rc::new(move |outcome: Answer| finish(&request, &kit, outcome))
    };
    let kind = Kind::Window { title: words.into(), app_id: APP_ID.into() };
    let Some(shown) = Shown::open(&kit.events, &display, kind, size, make(done.clone())) else {
        done(cancelled());
        return None;
    };
    let shown = Rc::new(shown);
    shown.fit();
    if let Some(handle) = parent.strip_prefix("wayland:") {
        shown.window.set_parent(handle);
    }
    {
        let done = done.clone();
        shown.window.on_close(move || done(cancelled()));
    }
    let close = {
        let done = done.clone();
        kit.bus.serve(handle, Offer {
            name: REQUEST.into(),
            xml: REQUEST_XML.into(),
            call: Box::new(move |asked: Asked| {
                asked.reply(&());
                done(cancelled());
            }),
            props: Box::new(Props::new),
            set: None,
        }).ok()
    };
    let mut r = request.borrow_mut();
    r.asked.as_ref()?;
    r.shown = Some(shown.clone() as Rc<dyn Any>);
    r.close = close;
    Some(shown)
}

pub(super) fn paint_picture(canvas: &mut PixmapMut, pen: &Pen, px: &Pixmap, cover: bool) {
    let (cw, ch) = (canvas.width() as f32, canvas.height() as f32);
    let (iw, ih) = (px.width().max(1) as f32, px.height().max(1) as f32);
    let k = if cover { (cw / iw).max(ch / ih) } else { (cw / iw).min(ch / ih) };
    let (dx, dy) = ((cw - iw * k) / 2.0, (ch - ih * k) / 2.0);
    let area = if cover { Rect { x: 0.0, y: 0.0, w: cw, h: ch } } else { Rect { x: dx, y: dy, w: iw * k, h: ih * k } };
    let Some(shape) = crate::toolkit::ui::rounded(area, crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Picture, area.w.min(area.h), pen.scale)) else { return };
    let mut paint = Paint { anti_alias: true, ..Paint::default() };
    paint.shader = Pattern::new(px.as_ref(), SpreadMode::Pad, FilterQuality::Bicubic, 1.0, Transform::from_row(k, 0.0, 0.0, k, dx, dy));
    canvas.fill_path(&shape, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
}

pub(super) fn picture_from(bytes: &[u8], px: u32) -> Option<Pixmap> {
    if let Ok(img) = image::load_from_memory(bytes) {
        return crate::toolkit::picture::to_pixmap(&img);
    }
    let tree = crate::toolkit::picture::svg_tree(bytes)?;
    let s = tree.size();
    let k = px as f32 / s.width().max(s.height());
    let mut out = Pixmap::new(((s.width() * k).ceil() as u32).max(1), ((s.height() * k).ceil() as u32).max(1))?;
    resvg::render(&tree, Transform::from_row(k, 0.0, 0.0, k, 0.0, 0.0), &mut out.as_mut());
    Some(out)
}

pub(super) enum Part {
    Heading(String),
    Words(String),
    Quiet(String),
    Icon(String),
    Picture { px: Rc<Pixmap>, size: (f32, f32), cover: bool, tip: String },
    Pick { id: String, label: String, options: Vec<(String, String)>, value: String },
    Name { text: String, editable: bool },
}

#[derive(Clone, Debug)]
pub(super) enum Msg {
    Cancel,
    Accept,
    Choose(usize, usize),
    Turn(usize, bool),
    Name(String),
}

pub(super) struct Confirm {
    pub parts: Vec<Part>,
    pub deny: String,
    pub grant: String,
    done: Done,
    answer: Box<dyn Fn(&[Part]) -> Answer>,
}

impl Confirm {
    pub fn new(parts: Vec<Part>, deny: &str, grant: &str, done: Done, answer: impl Fn(&[Part]) -> Answer + 'static) -> Confirm {
        Confirm { parts, deny: deny.into(), grant: grant.into(), done, answer: Box::new(answer) }
    }

    fn ready(&self) -> bool {
        self.parts.iter().all(|p| !matches!(p, Part::Name { text, .. } if text.trim().is_empty()))
    }
}

impl App for Confirm {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Cancel => (self.done)(cancelled()),
            Msg::Accept if self.ready() => (self.done)((self.answer)(&self.parts)),
            Msg::Accept => {}
            Msg::Choose(i, k) => {
                if let Some(Part::Pick { options, value, .. }) = self.parts.get_mut(i) {
                    if let Some((id, _)) = options.get(k) {
                        *value = id.clone();
                    }
                }
            }
            Msg::Turn(i, on) => {
                if let Some(Part::Pick { value, .. }) = self.parts.get_mut(i) {
                    *value = on.to_string();
                }
            }
            Msg::Name(words) => {
                for part in &mut self.parts {
                    if let Part::Name { text, .. } = part {
                        *text = words.clone();
                    }
                }
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let mut all: Vec<Node<Msg>> = Vec::new();
        for (i, part) in self.parts.iter().enumerate() {
            all.push(match part {
                Part::Heading(t) => title(t.clone()),
                Part::Words(t) => label(t.clone()),
                Part::Quiet(t) => note(t.clone()),
                Part::Icon(name) => {
                    let name = name.clone();
                    row(vec![space(), drawing((48.0, 48.0), String::new(), move |canvas, pen| {
                        let px = canvas.width().min(canvas.height());
                        let _ = pen.icons.draw(canvas, &name, 0, 0, px, 1, pen.theme.fg);
                    }), space()])
                }
                Part::Picture { px, size, cover, tip } => {
                    let (px, cover) = (px.clone(), *cover);
                    row(vec![space(), drawing(*size, tip.clone(), move |canvas, pen| paint_picture(canvas, pen, &px, cover)), space()])
                }
                Part::Pick { label: words, options, value, .. } if options.is_empty() => row(vec![
                    label(words.clone()), space(),
                    switch(value == "true", words.clone(), move |on| Msg::Turn(i, on)),
                ]).gap(12.0),
                Part::Pick { label: words, options, value, .. } => {
                    let at = options.iter().position(|(id, _)| id == value).unwrap_or(0);
                    let names = options.iter().map(|(_, n)| n.clone()).collect();
                    row(vec![label(words.clone()), space(), one_of(names, Some(at), words.clone(), move |k| Msg::Choose(i, k))]).gap(12.0)
                }
                Part::Name { text, editable: true } => entry(text.clone(), t!("screenshot-portal-name"), Msg::Name).width(Length::Fill).on_submit(Msg::Accept).first(),
                Part::Name { text, editable: false } => title(text.clone()),
            });
        }
        all.push(row(vec![
            space(),
            button(CANCEL_ICON, self.deny.clone()).on_press(Msg::Cancel),
            main_action(ACCEPT_ICON, self.grant.clone()).on_press(Msg::Accept).off(!self.ready()),
        ]).gap(8.0));
        column(all).gap(12.0).padding(16.0).on_escape(Msg::Cancel)
    }
}

pub(super) const SIZE: (u32, u32) = (440, 240);

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::portal::DONE;
    use std::cell::Cell;

    #[test]
    fn a_portals_question_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        let mut px = Pixmap::new(64, 36).unwrap();
        px.fill(tiny_skia::Color::from_rgba8(90, 120, 60, 255));
        let px = Rc::new(px);
        drawn::none(drawn::in_every_look("a portal's question", 8, || {
            for (state, name, editable) in [("ready", "Notes", true), ("no name yet", "", true), ("a name that is set", "Notes", false)] {
                let parts = vec![
                    Part::Heading("Paint wants to keep running when its window is shut".into()),
                    Part::Icon("dialog-question-symbolic".into()),
                    Part::Words("It will go on using the network and the battery.".into()),
                    Part::Quiet("You can change this later in Settings.".into()),
                    Part::Picture { px: px.clone(), size: (160.0, 90.0), cover: false, tip: "The picture it made".into() },
                    Part::Name { text: name.into(), editable },
                    Part::Pick { id: "ro".into(), label: "Read only".into(), options: vec![], value: "true".into() },
                    Part::Pick { id: "c".into(), label: "Colour".into(), options: vec![("r".into(), "Red".into()), ("b".into(), "Blue".into())], value: "b".into() },
                ];
                let c = Confirm::new(parts, "Deny", "Allow", Rc::new(|_: Answer| {}), |_| cancelled());
                drawn::window(state, c.view(), Theme::chosen(), (SIZE.0, 560), false);
            }
        }));
    }

    #[test]
    fn a_dialog_answers_once_with_what_it_shows() {
        let said: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let s = said.clone();
        let done: Done = Rc::new(move |a: Answer| s.borrow_mut().push(a.0));
        let parts = vec![
            Part::Name { text: "Notes".into(), editable: true },
            Part::Pick { id: "ro".into(), label: "Read only".into(), options: vec![], value: "false".into() },
            Part::Pick { id: "c".into(), label: "Colour".into(), options: vec![("r".into(), "Red".into()), ("b".into(), "Blue".into())], value: "r".into() },
        ];
        let got: Rc<Cell<bool>> = Rc::new(Cell::new(false));
        let g = got.clone();
        let mut c = Confirm::new(parts, "Deny", "Allow", done, move |parts| {
            let ok = matches!(&parts[0], Part::Name { text, .. } if text == "Jot")
                && matches!(&parts[1], Part::Pick { value, .. } if value == "true")
                && matches!(&parts[2], Part::Pick { value, .. } if value == "b");
            g.set(ok);
            answer(DONE, HashMap::new())
        });
        c.update(Msg::Name(" ".into()));
        c.update(Msg::Accept);
        assert!(said.borrow().is_empty(), "an empty name cannot be accepted");
        c.update(Msg::Name("Jot".into()));
        c.update(Msg::Turn(1, true));
        c.update(Msg::Choose(2, 1));
        c.update(Msg::Accept);
        assert!(got.get());
        assert_eq!(*said.borrow(), vec![DONE]);
        let _ = c.view();
    }

    #[test]
    fn pictures_are_read_from_bytes_whatever_they_are() {
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="8"><rect width="16" height="8" fill="#fff"/></svg>"##;
        let px = picture_from(svg, 64).expect("svg");
        assert_eq!((px.width(), px.height()), (64, 32));
        let mut png = Vec::new();
        image::DynamicImage::new_rgba8(3, 2).write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png).expect("png");
        let px = picture_from(&png, 64).expect("png read");
        assert_eq!((px.width(), px.height()), (3, 2));
        assert!(picture_from(b"nothing", 64).is_none());
    }
}

#[cfg(test)]
pub(super) mod testkit {
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::rc::Rc;
    use std::time::Instant;

    use serde::Serialize;
    use zbus::zvariant::{DynamicType, OwnedValue};

    use super::super::portal::{Stay, PATH};
    use crate::toolkit::bus::{testing, Bus, Props};
    use crate::toolkit::event::Loop;

    pub struct Rig {
        pub _daemon: testing::Daemon,
        pub lp: Loop,
        pub a: Bus,
        pub b: Bus,
        pub stay: Rc<Stay>,
    }

    impl Rig {
        pub fn new() -> Rig {
            let daemon = testing::own_daemon();
            std::env::remove_var("WAYLAND_DISPLAY");
            let lp = Loop::new();
            let a = Bus::at(&lp, &daemon.1).expect("the portal's connection to the test's own bus");
            let b = Bus::at(&lp, &daemon.1).expect("the asking program's connection to the test's own bus");
            let stay = Rc::new(Stay { last: Cell::new(Instant::now()), busy: Cell::new(0), settled: Cell::new(false) });
            Rig { _daemon: daemon, lp, a, b, stay }
        }

        pub fn kit(&self) -> super::Kit {
            super::Kit::new(&self.a, &self.lp, &self.stay)
        }
    }

    pub fn props(rig: &Rig, iface: &str) -> Option<Props> {
        let got: Rc<RefCell<Option<Option<Props>>>> = Rc::new(RefCell::new(None));
        let g = got.clone();
        rig.b.call(&rig.a.unique_name(), PATH, iface).get_all(&rig.lp, move |r| *g.borrow_mut() = Some(r.ok()));
        let g = got.clone();
        assert!(testing::until(&rig.lp, move || g.borrow().is_some()));
        let out = got.borrow_mut().take().flatten();
        out
    }

    pub fn call<B: Serialize + DynamicType, T: for<'d> serde::Deserialize<'d> + zbus::zvariant::Type + 'static>(
        rig: &Rig, iface: &str, member: &str, body: &B) -> Option<T> {
        let got: Rc<RefCell<Option<Option<T>>>> = Rc::new(RefCell::new(None));
        let g = got.clone();
        rig.b.call(&rig.a.unique_name(), PATH, iface).method(member, body, false, None, &rig.lp, move |r| {
            *g.borrow_mut() = Some(r.ok().and_then(|m| m.body().deserialize::<T>().ok()))
        });
        let g = got.clone();
        assert!(testing::until(&rig.lp, move || g.borrow().is_some()));
        let out = got.borrow_mut().take().flatten();
        out
    }

    pub type Reply = (u32, HashMap<String, OwnedValue>);
}
