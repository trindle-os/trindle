//! trindle-screenshot: a picture of the screen, or of part of it, to draw on
//! before it is kept.

pub(crate) unsafe fn function<F: Copy>(p: *mut std::ffi::c_void) -> F {
    assert_eq!(std::mem::size_of::<F>(), std::mem::size_of::<*mut std::ffi::c_void>(), "a function pointer is as big as an address");
    // SAFETY: F is as big as the address, and the caller says the address is a function of that type.
    unsafe { std::mem::transmute_copy(&p) }
}

mod area;
mod frame;
pub(crate) mod cast_capture;
mod cast_pick;
mod marks;
mod eye_dropper;
mod gbm;
pub mod file_chooser;
pub mod colours;
mod pin;
pub mod portal;
mod portal_access;
mod portal_apps;
mod portal_cast;
mod portal_ask;
mod portal_email;
mod portal_files;
mod portal_inhibit;
mod portal_launcher;
mod portal_lockdown;
mod portal_notify;
mod portal_print;
mod portal_settings;
mod portal_wallpaper;
mod pw;
mod toolbar;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
use std::time::Duration;

use tiny_skia::{FilterQuality, Pixmap, PixmapMut, PixmapPaint, Transform};
use zbus::zvariant::Value;

use area::{Handle, Pt, Rect, View};
use marks::{History, Mark, Shape, Tool, COLOURS, THICKNESS};
use pin::Pin;
use toolbar::{Act, Now};
use crate::common::files::{home, private_temp, set_setting, setting};
use crate::common::clock::Local;
use crate::common::kit::Rgb;
use crate::common::capture;
use crate::toolkit::bus::{Asked, Bus, Listen, Offer, Props, Served};
use crate::toolkit::chooser::{self, Ask};
use crate::toolkit::event::{Loop, Post};
use crate::toolkit::input::{Cursor, Input, Keysym, Mods, Typing};
use crate::toolkit::t;
use crate::toolkit::text::Fonts;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{button, card, drawing, layers, note, placed, row, Length, Node, Pen, Pointer};
use crate::toolkit::window::{Display, Edges, Keys, Kind, Layer, Panel, Window};

const FOLDER: &str = "Pictures/Screenshots";

const ASKING_DIM: f64 = 0.2;
const CHOOSING_DIM: f64 = 0.45;

const SMALLEST: f64 = 4.0;

const REACH: f64 = 12.0;

const GAP: f64 = 12.0;

const APP: &str = "Screenshots";

const COLOUR_SETTING: &str = "screenshot_colour";
const THICKNESS_SETTING: &str = "screenshot_thickness";

const NAME: &str = "org.trindle.Screenshot";
const PATH: &str = "/org/trindle/Screenshot";
const XML: &str = r#"<interface name="org.trindle.Screenshot"><method name="Show"/></interface>"#;

const PUT_AWAY: Duration = Duration::from_millis(120);

pub fn main(args: Vec<String>) -> i32 {

    let _ = crate::common::renewed::not_traceable();
    if args.iter().skip(1).any(|a| a == "--eye-dropper") {
        return eye_dropper::main();
    }
    if args.iter().skip(1).any(|a| a == "--whole") {
        return keep_whole_screen();
    }

    crate::ring::kept::take();
    let events = Loop::new();
    let gone = events.clone();
    let Some(display) = Display::connect(&events, move || gone.quit()) else {
        eprintln!("trindle-screenshot: no Wayland display to open; run it inside a Trindle session");
        return 1;
    };

    crate::ring::kept::pulse_while_keys_held(&events, &display);
    let bus = Bus::session(&events).ok();
    let life = Life::new(&events, display, bus.clone());

    let served: Rc<RefCell<Option<Served>>> = Rc::new(RefCell::new(None));
    match &bus {
        Some(bus) => be_the_one(&events, bus, &life, &served),
        None => life.show(),
    }
    events.run();
    drop(served);
    drop(life);
    0
}

fn keep_whole_screen() -> i32 {
    let events = Loop::new();
    let say = Say { bus: Bus::session(&events).ok(), events: events.clone() };
    let Some((scratch, shot)) = photograph() else { return 1 };
    let code = if keep_whole(&shot, &say) { 0 } else { 1 };
    let _ = std::fs::remove_dir_all(scratch);
    let quit = events.clone();
    events.after(Duration::ZERO, move || quit.quit());
    events.run();
    code
}

fn be_the_one(events: &Loop, bus: &Bus, life: &Rc<Life>, served: &Rc<RefCell<Option<Served>>>) {
    let (ev, b, l, s) = (events.clone(), bus.clone(), life.clone(), served.clone());
    bus.own(NAME, false, events, move |ok| {
        if !ok {
            let quit = ev.clone();
            b.call(NAME, PATH, NAME).method("Show", &(), false, Some(Duration::from_secs(2)), &ev, move |_| quit.quit());
            return;
        }
        let shows = l.shows.clone();
        *s.borrow_mut() = b.serve(PATH, Offer {
            name: NAME.into(),
            xml: XML.into(),
            call: Box::new(move |asked: Asked| {
                shows.send(());
                asked.reply(&());
            }),
            props: Box::new(Props::new),
            set: None,
        }).ok();
        l.show();
    });
}

fn photograph() -> Option<(PathBuf, PathBuf)> {
    let scratch = private_temp("trindle-screenshot").ok()?;
    let shot = scratch.join("screen.png");
    if let Err(e) = capture::take(None, false).and_then(|p| p.save_png(&shot)) {
        eprintln!("trindle-screenshot: could not photograph the screen: {e}");
        let _ = std::fs::remove_dir_all(&scratch);
        return None;
    }
    Some((scratch, shot))
}

fn file_name(when: &Local, n: u32) -> String {
    let stamp = when.format("%Y-%m-%d %H.%M.%S");
    if n > 1 { format!("Screenshot {stamp} ({n}).png") } else { format!("Screenshot {stamp}.png") }
}

fn now() -> Local {
    Local::now()
}

fn next_file() -> Result<PathBuf, String> {
    let folder = home().join(FOLDER);
    std::fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
    let when = now();

    (1..1000).map(|n| folder.join(file_name(&when, n)))
        .find(|p| std::fs::OpenOptions::new().write(true).create_new(true).open(p).is_ok())
        .ok_or_else(|| t!("screenshot-no-free-name").to_string())
}

fn to_clipboard(png: &[u8]) -> bool {
    use std::io::{BufRead, Write};
    use std::process::{Command, Stdio};
    let Ok(mut child) = Command::new("trindle-clipboard").args(["copy", "--type", "image/png"])
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn() else { return false };
    let written = child.stdin.take().is_some_and(|mut i| i.write_all(png).is_ok());
    let mut line = String::new();
    let said = child.stdout.take()
        .is_some_and(|o| std::io::BufReader::new(o).read_line(&mut line).is_ok());

    std::thread::spawn(move || child.wait());
    written && said && line.trim() == "copied"
}

fn keep_whole(shot: &Path, say: &Say) -> bool {
    match next_file().and_then(|p| std::fs::copy(shot, &p).map(|_| p).map_err(|e| e.to_string())) {
        Ok(_) => {

            let _ = std::fs::read(shot).is_ok_and(|b| to_clipboard(&b));
            say.done();
            true
        }
        Err(e) => {
            say.not_saved(&e);
            false
        }
    }
}

fn write_png(path: &Path, png: &[u8]) -> Result<(), String> {
    crate::common::files::write_whole(path, png).map_err(|e| t!("screenshot-write-failed", path = path.display().to_string(), why = e.to_string()))
}

struct Say {
    bus: Option<Bus>,
    events: Loop,
}

impl Say {
    fn not_saved(&self, why: &str) {
        eprintln!("trindle-screenshot: {why}");
        self.tell(t!("screenshot-not-saved"), why, "dialog-warning-symbolic");
    }

    fn done(&self) {
        if let Some(bus) = &self.bus {
            crate::ring::tick_at_edge(bus, &self.events);
        }
    }

    fn tell(&self, summary: &str, body: &str, icon: &str) {
        let Some(bus) = &self.bus else { return };
        let hold = self.events.hold();
        bus.call("org.freedesktop.Notifications", "/org/freedesktop/Notifications", "org.freedesktop.Notifications")
            .method("Notify", &(APP, 0u32, icon, summary, body, Vec::<String>::new(), HashMap::<&str, Value>::new(), -1i32),
                    false, Some(Duration::from_secs(2)), &self.events, move |_| drop(hold));
    }
}

fn remembered() -> (Rgb, f64) {
    let colour = setting(COLOUR_SETTING);
    let thick = setting(THICKNESS_SETTING);
    (COLOURS.iter().find(|(_, n)| *n == colour).map(|(c, _)| *c).unwrap_or(COLOURS[0].0),
     THICKNESS.iter().find(|(_, n)| *n == thick).map(|(t, _)| *t).unwrap_or(THICKNESS[1].0))
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Doing {
    Idle,

    Choosing(Pt, Option<Rect>),

    Resizing(Handle, Rect),

    Moving(Rect),
    Drawing,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Finish {

    Keep,
    Copy,
    Save,
    SaveAs,
    Pin,
}

struct Shot {

    part: Option<Rect>,
    doing: Doing,
    tool: Option<Tool>,
    colour: Rgb,
    thick: f64,
    marks: History<Mark>,

    drawing: Option<Mark>,

    typing: Option<Mark>,
}

struct Core {
    shot: RefCell<Shot>,
    still: Pixmap,

    size: Pt,

    shown: Cell<Rect>,
    win: RefCell<Weak<Window>>,

    display: Option<Rc<Display>>,
    fonts: Fonts,
}

impl Core {

    fn refresh(&self) {
        let Some(w) = self.win.borrow().upgrade() else { return };
        let Some(display) = &self.display else { return };
        let places = display.places();
        if let Some(this) = w.place().or_else(|| (places.len() == 1).then(|| places[0])) {
            let rect = |p: [i32; 4]| Rect { x: p[0] as f64, y: p[1] as f64, w: p[2] as f64, h: p[3] as f64 };
            let all: Vec<Rect> = places.iter().map(|p| rect(*p)).collect();
            self.shown.set(area::display_in_picture(self.size, &all, rect(this)));
        }
    }

    fn view(&self) -> View {
        let size = self.win.borrow().upgrade().map_or((1, 1), |w| w.size());
        eye_dropper::view(&size, self.shown.get())
    }

    fn result(&self) -> Option<(Pixmap, Vec<u8>, Pt)> {
        let view = self.view();
        let st = self.shot.borrow();
        let part = st.part.unwrap_or(Rect { x: 0.0, y: 0.0, w: view.size.0, h: view.size.1 });
        let picture = marks::render(&self.still, st.marks.done().iter(), area::to_pixels(part, view.size, view.shown), &self.fonts)?;
        let png = marks::png(&picture)?;
        Some((picture, png, (part.w, part.h)))
    }

    fn paint(&self, canvas: &mut PixmapMut, pen: &Pen) {
        let scale = pen.scale;
        let size = (canvas.width() as f64 / scale as f64, canvas.height() as f64 / scale as f64);
        let view = View { shown: self.shown.get(), size };
        let shown = view.shown;
        let k = scale / view.scale() as f32;
        let photo = Transform::from_scale(k, k).pre_translate(-shown.x as f32, -shown.y as f32);
        let good = PixmapPaint { quality: FilterQuality::Bicubic, ..PixmapPaint::default() };
        canvas.draw_pixmap(0, 0, self.still.as_ref(), &good, photo, None);

        let st = self.shot.borrow();
        frame::dim_outside(canvas, pen, size, st.part);
        let Some(part) = st.part else { return };
        let Some(box_) = frame::in_pixels(part, scale) else { return };
        frame::draw_marks(canvas, pen, &st, photo, box_);
        frame::draw_frame(canvas, pen, (part, box_), &st.doing, &view);
    }
}

#[derive(Clone)]
enum Msg {
    Pointer(Pointer),
    Act(Act),
    Whole,
    Close,
    Finish(Finish),
    Undo,
    Redo,
    Typed(String),
    Back,
    StopTyping(bool),

    Then(Box<Msg>),
    Nothing,
}

enum Out {
    Close,

    Then(Box<dyn FnOnce(&Rc<Life>)>),
    SaveAs(Vec<u8>),

    Saved(Option<PathBuf>, Option<Vec<u8>>),
}

struct Screen {
    core: Rc<Core>,
    out: Post<Out>,

    start: Pt,
}

impl Screen {
    fn begin(&mut self, p: Pt) {
        self.stop_typing(true);
        let view = self.core.view();
        let mut st = self.core.shot.borrow_mut();
        if let Some((part, h)) = st.part.and_then(|part| area::handle_at(part, p, REACH).map(|h| (part, h))) {
            st.doing = Doing::Resizing(h, part);
        } else if let (Some(tool), Some(_)) = (st.tool, st.part) {
            let n = marks::next_number(st.marks.done());
            st.drawing = Some(Mark::begin(tool, view.in_picture(p), st.colour, st.thick, view.scale(), n));
            st.doing = Doing::Drawing;
        } else if let Some(part) = st.part.filter(|part| part.contains(p)) {
            st.doing = Doing::Moving(part);
        } else {
            st.doing = Doing::Choosing(p, st.part);
            st.part = Some(area::between(p, p));
        }
    }

    fn drag(&mut self, p: Pt) {
        let view = self.core.view();
        let d = (p.0 - self.start.0, p.1 - self.start.1);
        let mut st = self.core.shot.borrow_mut();
        match st.doing {
            Doing::Idle => {}
            Doing::Choosing(from, _) => st.part = Some(area::inside(area::between(from, p), view.size)),
            Doing::Resizing(h, was) => st.part = Some(area::inside(area::resized(was, h, d.0, d.1), view.size)),
            Doing::Moving(was) => st.part = Some(area::moved(was, d.0, d.1, view.size)),
            Doing::Drawing => {
                if let Some(m) = st.drawing.as_mut() {
                    m.drag(view.in_picture(p));
                    m.make_blur(&self.core.still);
                }
            }
        }
    }

    fn end(&mut self) {
        let mut st = self.core.shot.borrow_mut();
        let tiny = |r: Option<Rect>| r.is_none_or(|r| r.w < SMALLEST || r.h < SMALLEST);
        let mut typing = None;

        match st.doing {
            Doing::Choosing(_, was) if tiny(st.part) => st.part = was,
            Doing::Resizing(_, was) if tiny(st.part) => st.part = Some(was),
            Doing::Drawing => {
                if let Some(mut m) = st.drawing.take() {
                    if st.tool == Some(Tool::Text) {
                        typing = Some(m);
                    } else if m.worth_keeping() {
                        m.make_blur(&self.core.still);
                        st.marks.push(m);
                    }
                }
            }
            _ => {}
        }
        st.doing = Doing::Idle;
        if typing.is_some() {
            st.typing = typing;
        }
    }

    fn pointer_at(core: &Core, p: Pt) -> Option<Cursor> {
        let st = core.shot.borrow();
        if st.doing != Doing::Idle {
            return None;
        }
        Some(match (st.part, st.tool) {
            (Some(part), _) if area::handle_at(part, p, REACH).is_some() => {
                area::handle_at(part, p, REACH).map_or(Cursor::Crosshair, |h| h.cursor())
            }
            (Some(_), Some(Tool::Text)) => Cursor::Text,
            (Some(_), Some(_)) => Cursor::Crosshair,
            (Some(part), None) if part.contains(p) => Cursor::Move,
            _ => Cursor::Crosshair,
        })
    }

    fn pick(&mut self, tool: Tool) {
        let same = self.core.shot.borrow().tool == Some(tool);
        if same {
            self.core.shot.borrow_mut().tool = None;
        } else {
            self.stop_typing(true);
            self.core.shot.borrow_mut().tool = Some(tool);
        }
    }

    fn stop_typing(&mut self, keep: bool) {
        let mut st = self.core.shot.borrow_mut();
        if let Some(m) = st.typing.take() {
            if keep && m.worth_keeping() {
                st.marks.push(m);
            }
        }
    }

    fn typed(&mut self, words: &str) {
        let mut st = self.core.shot.borrow_mut();
        if let Some(Mark { shape: Shape::Text(_, w), .. }) = st.typing.as_mut() {
            w.push_str(words);
        }
    }

    fn back(&mut self) {
        let mut st = self.core.shot.borrow_mut();
        if let Some(Mark { shape: Shape::Text(_, w), .. }) = st.typing.as_mut() {
            w.pop();
        }
    }

    fn finish(&mut self, how: Finish) {
        self.stop_typing(true);
        let Some((picture, png, size)) = self.core.result() else {
            self.out.send(Out::Then(Box::new(|l: &Rc<Life>| l.say.not_saved(t!("screenshot-no-picture")))));
            return;
        };
        let view = self.core.view().size;
        let then: Box<dyn FnOnce(&Rc<Life>)> = match how {
            Finish::Keep => Box::new(move |l| match next_file().and_then(|p| write_png(&p, &png)) {
                Ok(()) => {
                    let _ = to_clipboard(&png);
                    l.say.done();
                }
                Err(e) => l.say.not_saved(&e),
            }),
            Finish::Save => Box::new(move |l| match next_file().and_then(|p| write_png(&p, &png)) {
                Ok(()) => l.say.done(),
                Err(e) => l.say.not_saved(&e),
            }),
            Finish::Copy => Box::new(move |l| {
                if to_clipboard(&png) {
                    l.say.done();
                } else {
                    l.say.tell(t!("screenshot-not-copied"), t!("screenshot-clipboard-refused"), "dialog-warning-symbolic");
                }
            }),
            Finish::Pin => Box::new(move |l| l.pin(picture, size, view)),
            Finish::SaveAs => {
                self.out.send(Out::SaveAs(png));
                return;
            }
        };
        self.out.send(Out::Then(then));
    }

    fn colour(&mut self, i: usize) {
        let Some(&(c, name)) = COLOURS.get(i) else { return };
        let mut st = self.core.shot.borrow_mut();
        st.colour = c;
        if let Some(m) = st.typing.as_mut() {
            m.colour = c;
        }
        let _ = set_setting(COLOUR_SETTING, name);
    }

    fn thick(&mut self, i: usize) {
        let Some(&(t, name)) = THICKNESS.get(i) else { return };
        let mut st = self.core.shot.borrow_mut();
        st.thick = t;
        if let Some(m) = st.typing.as_mut() {
            m.thick = t;
        }
        let _ = set_setting(THICKNESS_SETTING, name);
    }

    fn undo(&mut self) {
        if self.core.shot.borrow().typing.is_some() {
            self.stop_typing(false);
        } else {
            self.core.shot.borrow_mut().marks.undo();
        }
    }

    fn redo(&mut self) {
        self.stop_typing(true);
        self.core.shot.borrow_mut().marks.redo();
    }

    fn keys(sym: Keysym, mods: &Mods) -> Option<Msg> {
        let ctrl = mods.ctrl;
        Some(match sym {
            Keysym::Escape => Msg::Close,

            Keysym::Return | Keysym::KP_Enter => Msg::Finish(Finish::Keep),
            Keysym::c | Keysym::C if ctrl => Msg::Finish(Finish::Copy),
            Keysym::s | Keysym::S if ctrl && mods.shift => Msg::Finish(Finish::SaveAs),
            Keysym::s | Keysym::S if ctrl => Msg::Finish(Finish::Save),
            Keysym::z | Keysym::Z if ctrl && mods.shift => Msg::Redo,
            Keysym::y | Keysym::Y if ctrl => Msg::Redo,
            Keysym::z | Keysym::Z if ctrl => Msg::Undo,
            _ => return None,
        })
    }
}

impl App for Screen {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        self.core.refresh();
        match msg {
            Msg::Pointer(Pointer::Down(x, y)) => {
                self.start = (x as f64, y as f64);
                self.begin(self.start);
            }
            Msg::Pointer(Pointer::Move(x, y)) => self.drag((x as f64, y as f64)),
            Msg::Pointer(Pointer::Up(..)) => self.end(),
            Msg::Pointer(_) => {}
            Msg::Act(Act::Tool(t)) => self.pick(t),
            Msg::Act(Act::Colour(i)) => self.colour(i),
            Msg::Act(Act::Thick(i)) => self.thick(i),
            Msg::Act(Act::Undo) | Msg::Undo => self.undo(),
            Msg::Act(Act::Redo) | Msg::Redo => self.redo(),
            Msg::Act(Act::Copy) => self.finish(Finish::Copy),
            Msg::Act(Act::Save) => self.finish(Finish::Save),
            Msg::Act(Act::SaveAs) => self.finish(Finish::SaveAs),
            Msg::Act(Act::Pin) => self.finish(Finish::Pin),
            Msg::Finish(how) => self.finish(how),
            Msg::Act(Act::Cancel) | Msg::Close => {
                self.out.send(Out::Close);
            }
            Msg::Whole => {
                let size = self.core.view().size;
                self.core.shot.borrow_mut().part = Some(Rect { x: 0.0, y: 0.0, w: size.0, h: size.1 });
            }
            Msg::Typed(t) => self.typed(&t),
            Msg::Back => self.back(),
            Msg::StopTyping(keep) => self.stop_typing(keep),
            Msg::Then(m) => {
                self.stop_typing(true);
                self.update(*m);
            }
            Msg::Nothing => {}
        }
    }

    fn view(&self) -> Node<Msg> {
        let core = self.core.clone();
        let at = self.core.clone();
        let mut all = vec![
            drawing((0.0, 0.0), t!("screenshot-name"), move |canvas, pen| core.paint(canvas, pen))
                .width(Length::Fill).height(Length::Fill)
                .on_pointer(Msg::Pointer)
                .cursor_with(move |x, y| Screen::pointer_at(&at, (x as f64, y as f64))
                    .unwrap_or_else(|| match at.shot.borrow().doing {
                        Doing::Resizing(h, _) => h.cursor(),
                        Doing::Moving(_) => Cursor::Move,
                        _ => Cursor::Crosshair,
                    })),
        ];
        let st = self.core.shot.borrow();
        match st.part {
            None => {
                all.push(placed(card(note(t!("screenshot-drag"))).width(Length::Shrink).floats(), |(w, _), (aw, _)| ((aw - w) / 2.0, 40.0)));

                all.push(placed(card(row(vec![
                    button("video-display-symbolic", t!("screenshot-whole")).on_press(Msg::Whole),
                    button("window-close-symbolic", t!("screenshot-close")).on_press(Msg::Close),
                ]).gap(12.0).width(Length::Shrink)).width(Length::Shrink).floats(), |(w, h), (aw, ah)| ((aw - w) / 2.0, ah - h - 48.0)));
            }
            Some(part) if !matches!(st.doing, Doing::Choosing(..)) => {
                let now = Now { tool: st.tool, colour: st.colour, thick: st.thick,
                                undo: st.marks.can_undo() || st.typing.is_some(), redo: st.marks.can_redo() };
                all.push(placed(toolbar::card_of(&now, Msg::Act).floats(), move |(w, h), (aw, ah)| {
                    let (x, y) = area::tools_at(part, (w as f64, h as f64), (aw as f64, ah as f64), GAP);
                    (x as f32, y as f32)
                }));
            }
            Some(_) => {}
        }
        layers(all)
    }

    fn intercept(&self, input: &Input, _keys: bool) -> Option<Msg> {
        self.core.refresh();
        let typing = self.core.shot.borrow().typing.is_some();
        match input {
            Input::Composed { commit, .. } if typing => {
                Some(if commit.is_empty() { Msg::Nothing } else { Msg::Typed(commit.clone()) })
            }
            Input::Key { sym, text, mods, .. } if typing => Some(match *sym {
                Keysym::Escape => Msg::StopTyping(false),
                Keysym::Return | Keysym::KP_Enter => Msg::StopTyping(true),
                Keysym::BackSpace => Msg::Back,
                _ if mods.ctrl => Screen::keys(*sym, mods).map_or(Msg::Nothing, |m| Msg::Then(Box::new(m))),
                _ => match text {
                    Some(t) if !t.is_empty() && !t.chars().any(char::is_control) => Msg::Typed(t.clone()),
                    _ => Msg::Nothing,
                },
            }),
            Input::Key { sym, mods, .. } => Screen::keys(*sym, mods),
            _ => None,
        }
    }

    fn typing(&self) -> Option<Typing> {
        let st = self.core.shot.borrow();
        let mark = st.typing.as_ref()?;
        let Shape::Text(_, words) = &mark.shape else { return None };
        let ((x, y), h) = mark.caret(&self.core.fonts)?;
        let view = self.core.view();
        let s = view.scale().max(f64::EPSILON);
        Some(Typing {
            id: "shot-words".into(), text: words.clone(), caret: words.len(), anchor: words.len(),
            area: (((x - view.shown.x) / s) as i32, ((y - view.shown.y) / s) as i32, 2, (h / s).ceil() as i32),
            secret: false, purpose: Default::default(),
        })
    }
}

struct Life {
    events: Loop,
    display: Rc<Display>,
    say: Say,
    overlay: RefCell<Option<Rc<Shown<Screen>>>>,
    pins: RefCell<Vec<(u64, Shown<Pin>)>>,
    next: Cell<u64>,

    asking: RefCell<Option<Listen>>,
    out: Post<Out>,
    unpin: Post<u64>,
    shows: Post<()>,
}

impl Life {
    fn new(events: &Loop, display: Rc<Display>, bus: Option<Bus>) -> Rc<Life> {
        Rc::new_cyclic(|me: &Weak<Life>| {
            let m = me.clone();
            let out = events.post(move |o: Out| {
                if let Some(l) = m.upgrade() {
                    l.answer(o);
                }
            });
            let m = me.clone();
            let unpin = events.post(move |id: u64| {
                let Some(l) = m.upgrade() else { return };
                let gone: Vec<_> = {
                    let mut pins = l.pins.borrow_mut();
                    let (gone, kept) = pins.drain(..).partition(|(i, _)| *i == id);
                    *pins = kept;
                    gone
                };
                drop(gone);
                l.maybe_quit();
            });
            let m = me.clone();
            let shows = events.post(move |()| {
                if let Some(l) = m.upgrade() {
                    l.show();
                }
            });
            Life {
                events: events.clone(), display, say: Say { bus, events: events.clone() },
                overlay: RefCell::new(None), pins: RefCell::new(Vec::new()), next: Cell::new(0),
                asking: RefCell::new(None), out, unpin, shows,
            }
        })
    }

    fn show(self: &Rc<Self>) {
        if self.overlay.borrow().is_some() {
            return;
        }

        let picture = match capture::take(None, false) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("trindle-screenshot: could not photograph the screen: {e}");
                self.maybe_quit();
                return;
            }
        };
        let Some(still) = eye_dropper::still(&picture) else {
            self.maybe_quit();
            return;
        };
        let size = (picture.width as f64, picture.height as f64);
        drop(picture);
        let (colour, thick) = remembered();
        let core = Rc::new(Core {
            shot: RefCell::new(Shot { part: None, doing: Doing::Idle, tool: None, colour, thick,
                                      marks: History::default(), drawing: None, typing: None }),
            still, size, shown: Cell::new(Rect { x: 0.0, y: 0.0, w: size.0, h: size.1 }),
            win: RefCell::new(Weak::new()), display: Some(self.display.clone()), fonts: Fonts::new(),
        });
        let kind = Kind::Panel(Panel {
            layer: Layer::Overlay, edges: Edges::ALL, keep: -1, margins: [0; 4], keys: Keys::Always,
            name: "trindle-screenshot".into(), screen: None,
        });
        let app = Screen { core: core.clone(), out: self.out.clone(), start: (0.0, 0.0) };
        let Some(shown) = Shown::open(&self.events, &self.display, kind, (0, 0), app) else {
            eprintln!("trindle-screenshot: this compositor does not offer the layer-shell protocol; run it inside a Trindle session");
            self.maybe_quit();
            return;
        };
        *core.win.borrow_mut() = Rc::downgrade(&shown.window);
        let out = self.out.clone();
        shown.window.on_close(move || {
            out.send(Out::Close);
        });
        *self.overlay.borrow_mut() = Some(Rc::new(shown));
    }

    fn answer(self: &Rc<Self>, o: Out) {
        match o {
            Out::Close => {
                self.overlay.borrow_mut().take();
                self.maybe_quit();
            }
            Out::Then(then) => self.close_then(then),
            Out::SaveAs(png) => self.save_as(png),
            Out::Saved(path, png) => {
                self.asking.borrow_mut().take();
                match (path, png) {
                    (Some(path), Some(png)) => self.close_then(Box::new(move |l| match write_png(&path, &png) {
                        Ok(()) => l.say.done(),
                        Err(e) => l.say.not_saved(&e),
                    })),
                    _ => match self.overlay.borrow().as_ref() {
                        Some(o) => o.window.set_visible(true),
                        None => self.maybe_quit(),
                    },
                }
            }
        }
    }

    fn close_then(self: &Rc<Self>, then: Box<dyn FnOnce(&Rc<Life>)>) {
        self.overlay.borrow_mut().take();
        let hold = self.events.hold();
        let me = Rc::downgrade(self);
        self.events.after(PUT_AWAY, move || {
            if let Some(l) = me.upgrade() {
                then(&l);
                l.maybe_quit();
            }
            drop(hold);
        });
    }

    fn pin(&self, picture: Pixmap, size: Pt, view: Pt) {
        let id = self.next.get();
        self.next.set(id + 1);
        let unpin = self.unpin.clone();
        if let Some(shown) = pin::show(&self.events, &self.display, picture, size, view, move || {
            unpin.send(id);
        }) {
            self.pins.borrow_mut().push((id, shown));
        }
    }

    fn save_as(self: &Rc<Self>, png: Vec<u8>) {
        let Some(bus) = self.say.bus.clone() else {
            self.say.not_saved(t!("screenshot-no-bus"));
            return;
        };
        let folder = home().join(FOLDER);
        let _ = std::fs::create_dir_all(&folder);
        let out = self.out.clone();
        let ask = Ask { title: t!("screenshot-save-title").into(), folder: Some(folder), name: Some(file_name(&now(), 1)), filters: Vec::new() };
        let Some(listen) = chooser::save(&bus, &self.events, ask, move |chosen| { out.send(Out::Saved(chosen, Some(png))); }) else {
            self.say.not_saved(t!("screenshot-no-chooser"));
            return;
        };
        *self.asking.borrow_mut() = Some(listen);
        if let Some(o) = self.overlay.borrow().as_ref() {
            o.window.set_visible(false);
        }
    }

    fn maybe_quit(&self) {
        if self.overlay.borrow().is_none() && self.pins.borrow().is_empty() && self.asking.borrow().is_none() {
            self.events.quit();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_screenshot_overlay_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        let (w, h) = (1280u32, 800u32);
        let mut still = Pixmap::new(w, h).unwrap();
        for (i, px) in still.pixels_mut().iter_mut().enumerate() {
            let (x, y) = ((i % w as usize) as u32, (i / w as usize) as u32);
            *px = tiny_skia::PremultipliedColorU8::from_rgba((x / 6) as u8, (y / 4) as u8, 120, 255).unwrap();
        }
        let size = (f64::from(w), f64::from(h));
        let events = Loop::new();
        let core = Rc::new(Core {
            shot: RefCell::new(Shot { part: None, doing: Doing::Idle, tool: None, colour: COLOURS[0].0, thick: THICKNESS[1].0,
                                      marks: History::default(), drawing: None, typing: None }),
            still, size, shown: Cell::new(Rect { x: 0.0, y: 0.0, w: size.0, h: size.1 }),
            win: RefCell::new(Weak::new()), display: None, fonts: Fonts::new(),
        });
        let screen = Screen { core: core.clone(), out: events.post(|_: Out| {}), start: (0.0, 0.0) };
        drawn::none(drawn::in_every_look("the screenshot overlay", 3, || {
            let part = Rect { x: 200.0, y: 150.0, w: 700.0, h: 420.0 };
            *core.shot.borrow_mut() = Shot { part: None, doing: Doing::Idle, tool: None, colour: COLOURS[0].0, thick: THICKNESS[1].0,
                                             marks: History::default(), drawing: None, typing: None };
            drawn::window("asking for a part", screen.view(), Theme::chosen(), (w, h), true);
            {
                let mut st = core.shot.borrow_mut();
                st.part = Some(part);
                st.doing = Doing::Choosing((200.0, 150.0), None);
            }
            drawn::window("dragging a part out", screen.view(), Theme::chosen(), (w, h), true);
            {
                let mut st = core.shot.borrow_mut();
                st.doing = Doing::Idle;
                st.tool = Some(Tool::Number);
                for (i, (colour, _)) in COLOURS.iter().enumerate() {
                    st.marks.push(Mark::begin(Tool::Number, (260.0 + 80.0 * i as f64, 300.0), *colour, THICKNESS[i % 3].0, 1.0, i as u32 + 1));
                }
            }
            drawn::window("a part chosen, with numbered spots", screen.view(), Theme::chosen(), (w, h), true);
        }));
    }

    #[test]
    fn named_by_the_time_and_never_over_another() {
        let when = Local::utc(1_788_505_389);
        assert_eq!(file_name(&when, 1), "Screenshot 2026-09-04 07.03.09.png");
        assert_eq!(file_name(&when, 2), "Screenshot 2026-09-04 07.03.09 (2).png");
    }

    #[test]
    fn a_picture_replaced_is_whole_as_it_was_when_the_new_one_cannot_be_written() {
        let dir = std::env::temp_dir().join(format!("trindle-screenshot-whole-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("holiday.png");
        std::fs::write(&file, b"the old picture").unwrap();
        crate::common::files::FULL.set(true);
        let failed = write_png(&file, b"a new picture, longer than the old one");
        crate::common::files::FULL.set(false);
        assert!(failed.is_err());
        assert_eq!(std::fs::read(&file).unwrap(), b"the old picture");
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);
        write_png(&file, b"a new picture").unwrap();
        assert_eq!(std::fs::read(&file).unwrap(), b"a new picture");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
