//! trindle-eye-dropper: the Eye dropper as a program of its own. A button
//! takes one colour off the screen, with the same held picture and
//! magnified circle the colour portal uses, and the colours picked are kept
//! in a list, newest first, each one copied as a hex code with one press.

use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::rc::{Rc, Weak};
use std::time::Duration;

use tiny_skia::{FillRule, Paint, PathBuilder, Transform};

use super::eye_dropper::{parse_colour, ICON_FALLBACK};
use crate::common::files::home;
use crate::common::no_tick::SHOWN_SECONDS;
use crate::toolkit::bus::{Asked, Bus, Offer, Props};
use crate::toolkit::colour::Colour;
use crate::toolkit::event::{Loop, Post};
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::popup;
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::tiles::{empty_state, picture};
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{bento, button, card, column, drawing, headline, heading, icon_circle, label, main_action, note,
                             page_title, remove, row, scroll, space, word_button, Length, Node, Rect};
use crate::toolkit::window::{Display, Kind};

fn help() -> &'static str {
    t!("screenshot-eye-help")
}

const FILE: &str = ".local/share/trindle/eye-dropper";

pub(crate) const TAKE: &str = "--take";

const KEEP: usize = 30;

const HIDE_MS: u64 = 300;
const SWATCH: f32 = 34.0;
const SIZE: (u32, u32) = (720, 640);

const HELP_WIDE: f32 = 360.0;
const CARD_LEAST: f32 = 300.0;

const NAME: &str = "org.trindle.EyeDropperList";
const PATH: &str = "/org/trindle/EyeDropperList";
const XML: &str = r#"<interface name="org.trindle.EyeDropperList"><method name="Show"/><method name="Take"/></interface>"#;

#[derive(Clone)]
enum Msg {

    Idle,
    Take,
    Copy(String),
    Remove(String),
    Clear,

    Help,
}

enum Do {
    Hide,

    Back(Option<String>),
    Copy(String),
    Uncopy,
    Show,

    Take,
}

struct Dropper {
    colours: Vec<String>,

    file: PathBuf,

    reset: String,
    taking: bool,

    copied: Option<String>,
    events: Loop,
    asks: Post<Do>,

    help: bool,
    help_at: Rect,
    pressed_at: Rect,
}

impl App for Dropper {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Idle => {}
            Msg::Take => {
                if self.taking {
                    return;
                }
                self.taking = true;
                self.asks.send(Do::Hide);
                let back = self.asks.clone();
                self.events.after(Duration::from_millis(HIDE_MS), move || {
                    let back = back.clone();
                    std::thread::spawn(move || {
                        back.send(Do::Back(pick()));
                    });
                });
            }
            Msg::Copy(hex) => {
                self.asks.send(Do::Copy(hex.clone()));
                self.mark_copied(hex);
            }
            Msg::Remove(hex) => {
                self.colours.retain(|c| *c != hex);
                save_in(&self.file, &self.colours);
            }

            Msg::Clear => {
                self.colours.clear();
                save_in(&self.file, &self.colours);
            }
            Msg::Help => {
                self.help = !self.help;
                self.help_at = self.pressed_at;
            }
        }
    }

    fn view(&self) -> Node<Msg> {
        let page = self.page();
        if self.help {
            popup::dismissing(page, self.help_panel(), Msg::Help, Msg::Idle)
        } else {
            page
        }
    }

    fn pressed(&mut self, place: Rect) {
        self.pressed_at = place;
    }

    fn intercept(&self, input: &Input, _keys: bool) -> Option<Msg> {
        match input {
            Input::Key { sym: Keysym::Escape, .. } if self.help => Some(Msg::Help),
            _ => None,
        }
    }
}

impl Dropper {

    fn page(&self) -> Node<Msg> {
        let info = button("dialog-information-symbolic", t!("screenshot-eye-about")).size(theme::sheet().info)
            .on(self.help).on_press(Msg::Help);
        let take = main_action(ICON_FALLBACK, t!("screenshot-eye-take")).off(self.taking);
        let take = if self.taking { take } else { take.on_press(Msg::Take) };
        let head = row(vec![page_title(t!("screenshot-eye-title")), info, space(), take.first()]).gap(12.0);
        let words = match self.colours.first() {
            None if !self.reset.is_empty() => self.reset.clone(),
            None => t!("screenshot-eye-none").to_string(),
            Some(c) => t!("screenshot-eye-taken", n = self.colours.len(), newest = c),
        };
        let line = column(vec![headline(words).width(Length::Fill), space().height(Length::Fixed(12.0))]).gap(0.0);
        let all = vec![head, line, self.latest(), self.list()];
        scroll("eye-dropper", bento(CARD_LEAST, theme::bento().page_gap, all).padding(theme::sheet().page_padding)).height(Length::Fill)
    }

    fn latest(&self) -> Node<Msg> {
        let art = match self.colours.first() {
            None => empty_state(t!("screenshot-eye-latest-empty"), None),
            Some(hex) => {
                let fill = colour_of(hex);
                let blob = drawing((SWATCH, SWATCH), hex.clone(), move |canvas, pen| {

                    let (w, h) = (canvas.width() as f32, canvas.height() as f32);
                    let inset = 8.0 * pen.scale;
                    let radius = theme::sheet().corner_of(crate::toolkit::theme::Part::Button) * pen.scale;
                    for (rect, colour) in [(Rect { x: 0.0, y: 0.0, w, h }, pen.theme.bg),
                                           (Rect { x: inset, y: inset, w: (w - 2.0 * inset).max(1.0), h: (h - 2.0 * inset).max(1.0) }, fill)] {
                        if let Some(path) = crate::toolkit::ui::rounded(rect, radius) {
                            let mut paint = Paint { anti_alias: true, ..Paint::default() };
                            paint.set_color(colour.to_skia());
                            canvas.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
                        }
                    }
                }).width(Length::Fill).height(Length::Fill);
                column(vec![blob, self.row(hex, false)]).gap(12.0)
            }
        };
        picture(ICON_FALLBACK, t!("screenshot-eye-latest"), art)
    }

    fn list(&self) -> Node<Msg> {
        let b = theme::bento();
        let clear = remove("user-trash-symbolic", t!("screenshot-eye-clear")).key("clear").on_press(Msg::Clear);
        let head = row(vec![icon_circle("view-list-symbolic"), heading(t!("screenshot-eye-list")), space(), clear]).gap(12.0);
        let body = if self.colours.is_empty() {
            empty_state(t!("screenshot-eye-list-empty"), None)
        } else {
            column(self.colours.iter().enumerate().map(|(i, hex)| {
                let line = self.row(hex, true);
                if i % 2 == 1 { line.banded() } else { card(line).snug().flat() }
            }).collect()).gap(0.0)
        };
        card(column(vec![head, body]).gap(16.0)).height(Length::Fill).beside().span(b.card.0, b.card.1, b.words)
    }

    fn help_panel(&self) -> Node<Msg> {
        let at = self.help_at;
        let panel = card(note(help()).width(Length::Fill)).floats().width(Length::Fixed(HELP_WIDE));
        popup::under_the_i(panel, at)
    }
}

impl Dropper {

    fn row(&self, hex: &str, listed: bool) -> Node<Msg> {
        let fill = colour_of(hex);
        let swatch = listed.then(|| drawing((SWATCH, SWATCH), hex, move |canvas, pen| {

            let (w, h) = (canvas.width() as f32, canvas.height() as f32);
            let mut paint = Paint { anti_alias: true, ..Paint::default() };
            for (r, colour) in [(w.min(h) / 2.0, pen.theme.bg), (w.min(h) / 2.0 - 8.0 * pen.scale, fill)] {
                if let Some(path) = PathBuilder::from_circle(w / 2.0, h / 2.0, r.max(1.0)) {
                    paint.set_color(colour.to_skia());
                    canvas.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);
                }
            }
        }));
        let copied = self.copied.as_deref() == Some(hex);
        let copy = if copied { word_button(t!("screenshot-copied")).on(true) } else { button("edit-copy-symbolic", t!("screenshot-copy")) };
        let mut parts: Vec<Node<Msg>> = swatch.into_iter().collect();
        parts.push(label(hex).width(Length::Fill));
        if listed {
            parts.push(remove("list-remove-symbolic", t!("screenshot-eye-remove")).key(format!("remove-{hex}")).on_press(Msg::Remove(hex.to_string())));
        }
        parts.push(copy.on_press(Msg::Copy(hex.to_string())));
        let line = row(parts).gap(12.0);
        if listed { line.key(hex) } else { line }
    }

    fn mark_copied(&mut self, hex: String) {
        self.copied = Some(hex);
        let back = self.asks.clone();
        self.events.after(Duration::from_secs(SHOWN_SECONDS.into()), move || {
            back.send(Do::Uncopy);
        });
    }

    fn taken(&mut self, hex: Option<String>) {
        self.taking = false;
        let Some(hex) = hex else { return };
        self.colours.retain(|c| *c != hex);
        self.colours.insert(0, hex.clone());
        self.colours.truncate(KEEP);
        save_in(&self.file, &self.colours);
        self.mark_copied(hex);
    }
}

fn pick() -> Option<String> {
    let mut command = Command::new(sibling("trindle-screenshot"));
    command.arg("--eye-dropper");
    match run_limited(command, PICK_LIMIT) {
        Ok(out) if out.status.success() => parse_colour(String::from_utf8_lossy(&out.stdout).trim()).map(to_hex),
        Ok(out) => {
            let why = String::from_utf8_lossy(&out.stderr);
            if !why.trim().is_empty() {
                eprintln!("trindle-eye-dropper: {}", why.trim());
            }
            None
        }
        Err(e) => {
            eprintln!("trindle-eye-dropper: {e}");
            None
        }
    }
}

const PICK_LIMIT: Duration = Duration::from_secs(300);

fn sibling(name: &str) -> PathBuf {
    std::env::current_exe().ok().and_then(|p| p.parent().map(|d| d.join(name))).filter(|p| p.exists()).unwrap_or_else(|| PathBuf::from(name))
}

fn run_limited(mut command: Command, limit: Duration) -> std::io::Result<std::process::Output> {
    use std::io::Read;
    let mut child = command.stdout(Stdio::piped()).stderr(Stdio::piped()).stdin(Stdio::null()).spawn()?;
    let read = |pipe: Option<Box<dyn Read + Send>>| std::thread::spawn(move || {
        let mut bytes = Vec::new();
        if let Some(mut pipe) = pipe {
            let _ = pipe.read_to_end(&mut bytes);
        }
        bytes
    });
    let out = read(child.stdout.take().map(|p| Box::new(p) as Box<dyn Read + Send>));
    let err = read(child.stderr.take().map(|p| Box::new(p) as Box<dyn Read + Send>));
    let started = std::time::Instant::now();
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break status;
        }
        if started.elapsed() >= limit {
            let _ = child.kill();
            break child.wait()?;
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    Ok(std::process::Output { status, stdout: out.join().unwrap_or_default(), stderr: err.join().unwrap_or_default() })
}

pub fn main(args: Vec<String>) -> i32 {
    let take = args.iter().any(|a| a == TAKE);
    let events = Loop::new();
    let Some(bus) = Bus::session(&events).ok() else {
        return open(&events, take);
    };

    let code = Rc::new(Cell::new(0));
    let (ev, c) = (events.clone(), code.clone());
    let kept = Rc::new(RefCell::new(None));
    let (k, b) = (kept.clone(), bus.clone());
    let (ev2, b2, ev3) = (ev.clone(), b.clone(), ev.clone());
    crate::toolkit::once::claim(&bus, &events, NAME, PATH, move || {
        match start(&ev, &b, take) {
            Some(running) => *k.borrow_mut() = Some(running),
            None => {
                c.set(1);
                ev.quit();
            }
        }
    }, move || {
        b2.call(NAME, PATH, NAME).method(if take { "Take" } else { "Show" }, &(), false, Some(Duration::from_secs(2)), &ev2, move |_| ev3.quit());
    });
    events.run();
    drop(kept);
    code.get()
}

fn start(events: &Loop, bus: &Bus, take: bool) -> Option<(Rc<Shown<Dropper>>, Option<crate::toolkit::bus::Served>, Option<crate::toolkit::bus::Served>)> {
    let (shown, asks) = window(events)?;
    if take {
        asks.send(Do::Take);
    }
    let served = bus.serve(PATH, Offer {
        name: NAME.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| {
            asks.send(if asked.member() == "Take" { Do::Take } else { Do::Show });
            asked.reply(&());
        }),
        props: Box::new(Props::new),
        set: None,
    }).map_err(|e| eprintln!("{NAME}: cannot be asked to show: {e:?}")).ok();
    let raise = crate::toolkit::once::serve_window(bus, PATH, &shown.window).map_err(|e| eprintln!("{NAME}: cannot be raised: {e:?}")).ok();
    Some((shown, served, raise))
}

fn open(events: &Loop, take: bool) -> i32 {
    let Some((_shown, asks)) = window(events) else { return 1 };
    if take {
        asks.send(Do::Take);
    }
    events.run();
    0
}

fn window(events: &Loop) -> Option<(Rc<Shown<Dropper>>, Post<Do>)> {
    let gone = events.clone();
    let display = Display::connect(events, move || gone.quit())?;
    let slot: Rc<RefCell<Weak<Shown<Dropper>>>> = Rc::new(RefCell::new(Weak::new()));
    let s = slot.clone();
    let asks = events.post(move |ask: Do| {
        let Some(shown) = s.borrow().upgrade() else { return };
        match ask {
            Do::Hide => shown.window.set_visible(false),
            Do::Back(hex) => {
                shown.window.set_visible(true);
                if let Some(hex) = &hex {
                    shown.window.copy(hex);
                }
                shown.change(|d| d.taken(hex));
            }
            Do::Copy(hex) => shown.window.copy(&hex),
            Do::Uncopy => shown.change(|d| d.copied = None),
            Do::Take => shown.change(|d| d.update(Msg::Take)),
            Do::Show => {
                shown.window.raise();
            }
        }
    });
    let app = Dropper {
        colours: load(), file: path(), reset: crate::common::damaged::take_notes().join(" "), taking: false, copied: None,
        events: events.clone(), asks: asks.clone(),
        help: false, help_at: Rect::default(), pressed_at: Rect::default(),
    };
    let kind = Kind::Window { title: t!("screenshot-eye-title").into(), app_id: NAME.into() };
    let shown = Rc::new(Shown::open(events, &display, kind, SIZE, app)?);
    *slot.borrow_mut() = Rc::downgrade(&shown);
    let ev = events.clone();
    shown.window.on_close(move || ev.quit());
    Some((shown, asks))
}

fn path() -> PathBuf {
    match std::env::var_os("XDG_DATA_HOME").map(PathBuf::from).filter(|p| p.is_absolute()) {
        Some(data) => data.join("trindle/eye-dropper"),
        None => home().join(FILE),
    }
}

fn damaged() -> &'static str {
    t!("screenshot-eye-damaged")
}

fn load() -> Vec<String> {
    load_from(&path())
}

pub(crate) fn load_from(file: &std::path::Path) -> Vec<String> {
    let text = crate::common::damaged::read(file, damaged(), |l| from_hex(l.trim()).is_some());
    let mut out: Vec<String> = Vec::new();
    for line in text.lines().map(str::trim) {
        if from_hex(line).is_some() && !out.iter().any(|c| c == line) {
            out.push(line.to_string());
        }
        if out.len() == KEEP {
            break;
        }
    }
    out
}

pub(crate) fn save_in(p: &std::path::Path, colours: &[String]) {

    if crate::common::damaged::may_write(p, damaged()).is_err() {
        return;
    }
    if let Some(dir) = p.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let mut text = colours.join("\n");
    text.push('\n');
    if let Err(e) = crate::common::files::write_text(p, &text) {
        eprintln!("trindle-eye-dropper: {}: {e}", p.display());
    }
}

fn to_hex((r, g, b): (f64, f64, f64)) -> String {
    let c = |v: f64| (v * 255.0).round().clamp(0.0, 255.0) as u8;
    format!("#{:02X}{:02X}{:02X}", c(r), c(g), c(b))
}

fn colour_of(hex: &str) -> Colour {
    let (r, g, b) = from_hex(hex).unwrap_or((0.0, 0.0, 0.0));
    Colour::rgb(r as f32, g as f32, b as f32)
}

fn from_hex(text: &str) -> Option<(f64, f64, f64)> {
    let h = text.strip_prefix('#')?;
    if h.len() != 6 || !h.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }
    let part = |i: usize| u8::from_str_radix(&h[i..i + 2], 16).ok().map(|v| f64::from(v) / 255.0);
    Some((part(0)?, part(2)?, part(4)?))
}

#[cfg(test)]
mod tests {
    use crate::common::info_popup_tests::{shuts_like_every_other, THE_I, UNDER_THE_I};

    #[test]
    fn the_i_shuts_on_a_press_anywhere_outside_its_words() {
        let events = Loop::new();
        let mut d = Dropper {
            reset: String::new(), file: PathBuf::from("/nonexistent/not-written"), colours: vec!["#FBC711".into()], taking: false, copied: None, events: events.clone(),
            asks: events.post(|_: Do| {}), help: false, help_at: Rect::default(), pressed_at: Rect::default(),
        };
        shuts_like_every_other("Eye dropper", UNDER_THE_I, &mut d,
            |d| { d.help = true; d.help_at = THE_I; },
            |d| d.help, |d| d.view(), |d, m| d.update(m), |d, i| d.intercept(i, false));
    }

    use super::*;
    use super::super::eye_dropper::format_colour;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;

    fn code(r: u8, g: u8, b: u8) -> String {
        format!("#{r:02X}{g:02X}{b:02X}")
    }

    #[test]
    fn the_rings_button_starts_it_taking_and_a_second_start_asks_the_first_to() {
        assert_eq!(TAKE, "--take");
        assert!(XML.contains(r#"<method name="Take"/>"#));
    }

    #[test]
    fn a_program_that_does_not_end_is_ended_and_one_that_does_is_read() {
        let mut slow = Command::new("sleep");
        slow.arg("30");
        let started = std::time::Instant::now();
        let out = run_limited(slow, Duration::from_millis(300)).unwrap();
        assert!(!out.status.success() && started.elapsed() < Duration::from_secs(10));
        let mut quick = Command::new("echo");
        quick.arg("#FBC711");
        assert_eq!(String::from_utf8_lossy(&run_limited(quick, Duration::from_secs(10)).unwrap().stdout).trim(), "#FBC711");
    }

    #[test]
    fn a_repeated_colour_in_the_file_is_kept_once() {
        let dir = std::env::temp_dir().join(format!("trindle-eye-{}-{:?}", std::process::id(), std::thread::current().id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("list");
        std::fs::write(&file, "#FBC711\n#2295F1\n#FBC711\n").unwrap();
        assert_eq!(load_from(&file), vec!["#FBC711".to_string(), "#2295F1".to_string()]);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_copy_button_says_copied_in_a_word_and_not_with_a_tick() {
        let d = Dropper { reset: String::new(), file: PathBuf::from("/nonexistent/not-written"), colours: vec!["#FBC711".into()], taking: false, copied: Some("#FBC711".into()),
            events: Loop::new(), asks: Loop::new().post(|_: Do| {}), help: false, help_at: Rect::default(), pressed_at: Rect::default() };
        let buttons = d.view().buttons();
        assert!(buttons.iter().any(|(tip, _)| tip == t!("screenshot-copied")), "{buttons:?}");
    }

    #[test]
    fn a_picked_colour_becomes_hex_and_back() {
        let picked = parse_colour(&format_colour([0xFB, 0xC7, 0x11])).unwrap();
        let yellow = code(0xFB, 0xC7, 0x11);
        assert_eq!(to_hex(picked), yellow);
        assert_eq!(to_hex(from_hex(&yellow).unwrap()), yellow);
        assert_eq!(from_hex(&yellow[1..]), None);
        assert_eq!(from_hex(&yellow[..6]), None);
        assert_eq!(from_hex(&format!("#{}", "G".repeat(6))), None);
    }

    #[test]
    fn the_eye_droppers_list_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::drawn;
        let events = Loop::new();
        let (yellow, blue, black, white, purple, grey) =
            (code(0xFB, 0xC7, 0x11), code(0x22, 0x95, 0xF1), code(0, 0, 0), code(255, 255, 255), code(0x9B, 0x26, 0xAE), code(0x77, 0x77, 0x77));
        drawn::none(drawn::in_every_look("the eye dropper's list", 4, || {
            for (state, colours, help, taking, ticked) in [
                ("empty", vec![], false, false, None),
                ("colours", vec![yellow.clone(), blue.clone(), black.clone(), white.clone(), purple.clone(), grey.clone()], false, false, Some(blue.clone())),
                ("taking", vec![yellow.clone()], false, true, None),
                ("the i's words", vec![yellow.clone(), blue.clone()], true, false, None),
            ] {
                let d = Dropper {
                    reset: String::new(), file: PathBuf::from("/nonexistent/not-written"), colours, taking, copied: ticked, events: events.clone(), asks: events.post(|_: Do| {}),
                    help, help_at: Rect { x: 220.0, y: 24.0, w: 24.0, h: 24.0 }, pressed_at: Rect::default(),
                };
                drawn::window(state, d.view(), Theme::chosen(), (720, 640), false);
            }
        }));
    }

    #[test]
    #[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
    fn shoot_the_colours_list() {
        let Some(dir) = std::env::var_os("TRINDLE_SHOTS") else { return };
        let events = Loop::new();
        let (yellow, blue, black, white, purple) =
            (code(0xFB, 0xC7, 0x11), code(0x22, 0x95, 0xF1), code(0, 0, 0), code(255, 255, 255), code(0x9B, 0x26, 0xAE));
        for (name, colours, help) in [("eyedropper-empty", vec![], false),
                                      ("eyedropper", vec![yellow.clone(), blue.clone(), black, white, purple], false),
                                      ("eyedropper-help", vec![yellow, blue], true)] {
            let d = Dropper {
                reset: String::new(), file: PathBuf::from("/nonexistent/not-written"), colours, taking: false,
                copied: None, events: events.clone(), asks: events.post(|_: Do| {}),
                help, help_at: Rect { x: 220.0, y: 24.0, w: 24.0, h: 24.0 }, pressed_at: Rect::default(),
            };
            for (w, h) in [(720u32, 640u32), (1280, 800), (1920, 1080)] {
                let mut ui = Ui::new(d.view(), Theme::new(false, crate::common::style::DEFAULT_ACCENT));
                let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
                ui.paint(&mut pix.as_mut(), 1.0);
                pix.save_png(std::path::Path::new(&dir).join(format!("{name}-{w}.png"))).unwrap();
            }
        }
    }

    #[test]
    fn the_i_leaves_the_page_where_it_was() {
        let events = Loop::new();
        let mut d = Dropper {
            reset: String::new(), file: PathBuf::from("/nonexistent/not-written"), colours: (0..40u8).map(|i| code(i * 6, 255 - i * 6, i)).collect(), taking: false,
            copied: None, events: events.clone(), asks: events.post(|_: Do| {}),
            help: false, help_at: Rect { x: 220.0, y: 24.0, w: 24.0, h: 24.0 }, pressed_at: Rect::default(),
        };
        let mut ui = Ui::new(d.view(), Theme::new(true, "#fbc711"));
        ui.lay_at(720, 400);
        ui.to_the_ends();
        assert!(ui.places().iter().any(|p| p.0 == "eye-dropper" && p.1 > 0.0), "{:?}", ui.places());
        for on in [true, false, true, false] {
            d.help = on;
            assert_eq!(ui.moved_by(d.view()), Vec::<String>::new(), "the i {on}");
        }
    }

    #[test]
    fn clear_empties_the_saved_list_and_a_taken_colour_goes_first_once() {
        let dir = std::env::temp_dir().join(format!("trindle-eye-clear-{}-{:?}", std::process::id(), std::thread::current().id()));
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("list");
        let events = Loop::new();
        let mut d = Dropper {
            reset: String::new(), file: file.clone(), colours: vec![code(0, 0, 0), code(255, 255, 255)], taking: true,
            copied: None, events: events.clone(), asks: events.post(|_: Do| {}),
            help: false, help_at: Rect::default(), pressed_at: Rect::default(),
        };
        let ui = Ui::new(d.view(), Theme::chosen());
        assert!(ui.natural_height(SIZE.0 as f32) <= SIZE.1 as f32);
        d.taken(Some(code(255, 255, 255)));
        assert_eq!(d.colours, [code(255, 255, 255), code(0, 0, 0)]);
        assert!(!d.taking && d.copied == Some(code(255, 255, 255)));
        assert_eq!(load_from(&file), [code(255, 255, 255), code(0, 0, 0)], "the taken colour was not saved");
        d.update(Msg::Clear);
        assert!(d.colours.is_empty() && load_from(&file).is_empty(), "Clear left colours in the saved list");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
