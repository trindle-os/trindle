//! The Print portal. A program that wants to print asks PreparePrint, which
//! shows Trindle's own print window above it and answers with what was
//! chosen, a page setup and a token; nothing is sent yet. Print then takes
//! the document and the token and sends the document to CUPS as one job on
//! the printer chosen.

use std::cell::RefCell;
use std::collections::HashMap;
use std::os::fd::OwnedFd;
use std::rc::Rc;
use std::time::{Duration, Instant};

use zbus::zvariant::{OwnedObjectPath, Value};

use super::portal::{answer, DONE, FAILED};
use super::portal_ask::{app_name, ask, failed, get, cancelled, Answer, Done, Kit, Options, ACCEPT_ICON, CANCEL_ICON};
use crate::common::cups::{self, Choices, Orientation, Paper, Printer, Sides};
use crate::toolkit::bus::{Asked, Offer, Props};
use crate::toolkit::t;
use crate::toolkit::ui::App;
use crate::toolkit::widget::{button, column, dropdown, entry, label, main_action, note, one_of, row, space, title, Length, Node, Tone};

pub(super) const INTERFACE: &str = "org.freedesktop.impl.portal.Print";
const XML: &str = r#"<interface name="org.freedesktop.impl.portal.Print">
    <method name="PreparePrint">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="s" name="title" direction="in"/>
      <arg type="a{sv}" name="settings" direction="in"/>
      <arg type="a{sv}" name="page_setup" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <method name="Print">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="s" name="title" direction="in"/>
      <arg type="h" name="fd" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <property name="version" type="u" access="read"/>
  </interface>"#;

const SIZE: (u32, u32) = (480, 600);

const TOKEN_LIFE: Duration = Duration::from_secs(10 * 60);
const MOST_TOKENS: usize = 16;

const MARGIN_MM: f64 = 6.35;

const HELD: &str = "\u{a0}";

fn no_printer() -> &'static str {
    t!("screenshot-print-no-printer")
}
fn not_answering() -> &'static str {
    t!("screenshot-print-not-answering")
}
fn not_installed() -> &'static str {
    t!("screenshot-print-not-installed")
}

struct Prepared {
    app: String,
    choices: Choices,
    made: Instant,
}

#[derive(Default)]
pub(super) struct Tokens {
    held: Vec<(u32, Prepared)>,
    last: u32,
}

impl Tokens {
    fn give(&mut self, app: &str, choices: Choices, now: Instant) -> u32 {
        self.held.retain(|(_, p)| now.duration_since(p.made) < TOKEN_LIFE);
        while self.held.len() >= MOST_TOKENS {
            self.held.remove(0);
        }

        let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.subsec_nanos()).unwrap_or(1);
        loop {
            self.last = self.last.wrapping_add(nanos | 1).max(1);
            if !self.held.iter().any(|(t, _)| *t == self.last) {
                break;
            }
        }
        self.held.push((self.last, Prepared { app: app.into(), choices, made: now }));
        self.last
    }

    fn take(&mut self, token: u32, app: &str, now: Instant) -> Option<Choices> {
        let at = self.held.iter().position(|(t, p)| *t == token && p.app == app && now.duration_since(p.made) < TOKEN_LIFE)?;
        Some(self.held.remove(at).1.choices)
    }
}

fn text_dict(pairs: Vec<(&'static str, String)>) -> Value<'static> {
    let mut d = zbus::zvariant::Dict::new(&zbus::zvariant::Signature::Str, &zbus::zvariant::Signature::Variant);
    for (k, v) in pairs {
        let _ = d.append(Value::from(k.to_string()), Value::new(Value::from(v)));
    }
    Value::from(d)
}

fn page_setup(c: &Choices) -> Value<'static> {
    let (mut w, mut h) = c.paper.mm();
    if c.orientation == Orientation::Landscape {
        std::mem::swap(&mut w, &mut h);
    }
    let mut d = zbus::zvariant::Dict::new(&zbus::zvariant::Signature::Str, &zbus::zvariant::Signature::Variant);
    let mut put = |k: &str, v: Value<'static>| {
        let _ = d.append(Value::from(k.to_string()), Value::new(v));
    };
    put("Name", Value::from(c.paper.gtk().to_string()));
    put("DisplayName", Value::from(c.paper.label().to_string()));
    put("Width", Value::from(w));
    put("Height", Value::from(h));
    for side in ["MarginTop", "MarginBottom", "MarginLeft", "MarginRight"] {
        put(side, Value::from(MARGIN_MM));
    }
    put("Orientation", Value::from(c.orientation.gtk().to_string()));
    Value::from(d)
}

fn prepared(token: u32, c: &Choices) -> Answer {
    answer(DONE, HashMap::from([
        ("settings", text_dict(c.for_program())),
        ("page-setup", page_setup(c)),
        ("token", Value::from(token)),
    ]))
}

#[derive(Clone, Debug)]
pub(super) enum Msg {
    Cancel,
    Print,
    Printer(usize),
    AllPages(usize),
    Range(String),
    Copies(String),
    Sides(usize),
    Colour(usize),
    Paper(usize),
    Orientation(usize),
}

#[derive(Default, Debug, PartialEq)]
pub(super) struct Why {
    printer: Option<String>,
    pages: Option<&'static str>,
    copies: Option<&'static str>,
}

impl Why {
    fn none(&self) -> bool {
        self.printer.is_none() && self.pages.is_none() && self.copies.is_none()
    }
}

pub(super) struct PrintWindow {
    heading: String,
    printers: Vec<Printer>,

    unread: Option<&'static str>,
    chosen: Option<usize>,
    copies: String,
    all_pages: bool,
    range: String,
    sides: Sides,
    colour: bool,
    paper: Paper,
    orientation: Orientation,
    done: Done,
    give: Box<dyn Fn(Choices) -> Answer>,
}

impl PrintWindow {

    pub(super) fn new(heading: String, listing: Result<(Vec<Printer>, Option<String>), &'static str>, before: &Choices, done: Done,
                      give: impl Fn(Choices) -> Answer + 'static) -> PrintWindow {
        let (printers, default, unread) = match listing {
            Ok((p, d)) => (p, d, None),
            Err(why) => (Vec::new(), None, Some(why)),
        };
        let at = |name: &str| printers.iter().position(|p| p.name == name);
        let chosen = at(&before.printer).or_else(|| default.as_deref().and_then(at)).or(if printers.is_empty() { None } else { Some(0) });
        PrintWindow {
            heading, printers, unread, chosen,
            copies: before.copies.to_string(),
            all_pages: before.pages.is_none(),
            range: before.pages.as_ref().map(|r| r.iter().map(|&(a, b)| if a == b { a.to_string() } else { format!("{a}-{b}") }).collect::<Vec<_>>().join(", ")).unwrap_or_default(),
            sides: before.sides, colour: before.colour, paper: before.paper, orientation: before.orientation,
            done, give: Box::new(give),
        }
    }

    pub(super) fn why(&self) -> Why {
        let printer = match (self.unread, self.chosen.and_then(|i| self.printers.get(i))) {
            (Some(w), _) => Some(w.to_string()),
            (None, None) => Some(no_printer().to_string()),
            (None, Some(p)) if p.disabled => Some(t!("screenshot-print-paused", name = &p.name)),
            (None, Some(_)) => None,
        };
        Why {
            printer,
            pages: if self.all_pages { None } else { cups::parse_ranges(&self.range).err() },
            copies: cups::parse_copies(&self.copies).err(),
        }
    }

    pub(super) fn choices(&self) -> Option<Choices> {
        if !self.why().none() {
            return None;
        }
        Some(Choices {
            printer: self.printers.get(self.chosen?)?.name.clone(),
            copies: cups::parse_copies(&self.copies).ok()?,
            pages: if self.all_pages { None } else { Some(cups::parse_ranges(&self.range).ok()?) },
            sides: self.sides, colour: self.colour, paper: self.paper, orientation: self.orientation,
        })
    }
}

fn under(words: Option<&str>) -> Node<Msg> {
    note(words.unwrap_or(HELD).to_string()).tone(Tone::Problem)
}

fn asked_of(words: &str, control: Node<Msg>) -> Node<Msg> {
    row(vec![label(words.to_string()), space(), control]).gap(12.0)
}

impl App for PrintWindow {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Cancel => (self.done)(cancelled()),
            Msg::Print => {
                if let Some(c) = self.choices() {
                    (self.done)((self.give)(c));
                }
            }
            Msg::Printer(i) => self.chosen = Some(i).filter(|&i| i < self.printers.len()),
            Msg::AllPages(i) => self.all_pages = i == 0,
            Msg::Range(t) => self.range = t,
            Msg::Copies(t) => self.copies = t,
            Msg::Sides(i) => self.sides = Sides::ALL.get(i).copied().unwrap_or(self.sides),
            Msg::Colour(i) => self.colour = i == 0,
            Msg::Paper(i) => self.paper = Paper::ALL.get(i).copied().unwrap_or(self.paper),
            Msg::Orientation(i) => self.orientation = Orientation::ALL.get(i).copied().unwrap_or(self.orientation),
        }
    }

    fn view(&self) -> Node<Msg> {
        let why = self.why();
        let names: Vec<String> = self.printers.iter().map(|p| p.name.clone()).collect();
        let mut all: Vec<Node<Msg>> = vec![title(self.heading.clone())];
        all.push(asked_of(t!("screenshot-print-printer"), dropdown(names, self.chosen, t!("screenshot-print-printer"), Msg::Printer)));
        all.push(under(why.printer.as_deref()));
        all.push(asked_of(t!("screenshot-print-pages"), one_of(vec![t!("screenshot-print-all").into(), t!("screenshot-print-range").into()],
                                                                Some(if self.all_pages { 0 } else { 1 }), t!("screenshot-print-pages"), Msg::AllPages)));
        let range = entry(self.range.clone(), "1-3, 5", Msg::Range).width(Length::Fill).on_submit(Msg::Print).off(self.all_pages);
        all.push(range);
        all.push(under(why.pages));
        all.push(asked_of(t!("screenshot-print-copies"), entry(self.copies.clone(), t!("screenshot-print-copies"), Msg::Copies).width(Length::Fixed(80.0)).on_submit(Msg::Print)));
        all.push(under(why.copies));
        all.push(asked_of(t!("screenshot-print-sides"), dropdown(Sides::ALL.iter().map(|s| s.label().to_string()).collect(),
            Sides::ALL.iter().position(|s| *s == self.sides), t!("screenshot-print-sides"), Msg::Sides)));
        all.push(asked_of(t!("screenshot-print-colour"), one_of(vec![t!("screenshot-print-colour").into(), t!("screenshot-print-mono").into()],
                                                                Some(if self.colour { 0 } else { 1 }), t!("screenshot-print-colour"), Msg::Colour)));
        all.push(asked_of(t!("screenshot-print-paper"), dropdown(Paper::ALL.iter().map(|p| p.label().to_string()).collect(),
            Paper::ALL.iter().position(|p| *p == self.paper), t!("screenshot-print-paper"), Msg::Paper)));
        all.push(asked_of(t!("screenshot-print-page"), one_of(Orientation::ALL.iter().map(|o| o.label().to_string()).collect(),
            Orientation::ALL.iter().position(|o| *o == self.orientation), t!("screenshot-print-page"), Msg::Orientation)));
        all.push(row(vec![
            space(),
            button(CANCEL_ICON, t!("screenshot-portal-cancel")).on_press(Msg::Cancel),
            main_action(ACCEPT_ICON, t!("screenshot-print-go")).on_press(Msg::Print).off(!why.none()),
        ]).gap(8.0));
        column(all).gap(8.0).padding(16.0).on_escape(Msg::Cancel)
    }
}

pub(super) fn listing(ok: bool, out: &str, err: &str) -> Result<(Vec<Printer>, Option<String>), &'static str> {
    let quiet = err.contains("No destinations") || err.to_lowercase().contains("no system default");
    if !ok && !quiet {
        return Err(not_answering());
    }
    Ok((cups::parse_printers(out), cups::parse_default(out)))
}

fn before(settings: &Options, setup: &Options) -> Choices {
    let usual = Choices {
        printer: String::new(), copies: 1, pages: None, sides: Sides::One, colour: true,
        paper: Paper::usual(&std::env::var("LANG").unwrap_or_default()), orientation: Orientation::Portrait,
    };
    let mut c = Choices::from_program(usual, |k| get::<String>(settings, k));
    let size = get::<f64>(setup, "Width").zip(get::<f64>(setup, "Height"));
    if let Some(p) = Paper::from_program(get::<String>(setup, "Name").as_deref(), size) {
        c.paper = p;
    }
    if let Some(o) = get::<String>(setup, "Orientation").and_then(|w| Orientation::ALL.into_iter().find(|o| o.gtk() == w || format!("reverse_{}", o.gtk()) == w)) {
        c.orientation = o;
    }
    c
}

fn prepare(kit: &Kit, tokens: &Rc<RefCell<Tokens>>, asked: Asked) {
    let Ok((handle, app_id, parent, doc, settings, setup, _options)) = asked.message().body()
        .deserialize::<(OwnedObjectPath, String, String, String, Options, Options, Options)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "PreparePrint takes a handle, an app, a window, a title and three sets of options");
    };
    if kit.screen().is_none() {
        return asked.reply(&failed());
    }
    let start = before(&settings, &setup);
    let who = app_name(&app_id);
    let heading = match (who.is_empty(), doc.trim().is_empty()) {
        (_, false) => t!("screenshot-print-doc", doc = doc.chars().filter(|c| !c.is_control()).take(60).collect::<String>()),
        (false, true) => t!("screenshot-print-from", who = &who),
        (true, true) => t!("screenshot-print-go").to_string(),
    };
    let (kit, tokens) = (kit.clone(), tokens.clone());
    if !crate::common::run::have("lpstat") {
        return open(&kit, &tokens, asked, (handle, app_id, parent), heading, Err(not_installed()), start);
    }
    kit.hold();
    let held = kit.clone();
    crate::common::run::capture(&["lpstat", "-p", "-d"], None, move |ok, out, err| {
        held.release();
        open(&held, &tokens, asked, (handle, app_id, parent), heading, listing(ok, &out, &err), start);
    });
}

type Listing = Result<(Vec<Printer>, Option<String>), &'static str>;

fn open(kit: &Kit, tokens: &Rc<RefCell<Tokens>>, asked: Asked, who: (OwnedObjectPath, String, String), heading: String, got: Listing, start: Choices) {
    let (handle, app_id, parent) = who;
    let tokens = tokens.clone();
    let words = heading.clone();
    ask(kit, asked, handle.as_str(), &parent, &words, SIZE, move |done| {
        PrintWindow::new(heading, got, &start, done, move |c| {
            let token = tokens.borrow_mut().give(&app_id, c.clone(), Instant::now());
            prepared(token, &c)
        })
    });
}

fn tell(kit: &Kit, why: &str) {
    let hold = kit.events.hold();
    kit.bus.call("org.freedesktop.Notifications", "/org/freedesktop/Notifications", "org.freedesktop.Notifications")
        .method("Notify", &("Trindle", 0u32, "printer-error-symbolic", t!("screenshot-print-failed"), why, Vec::<String>::new(),
                            HashMap::<&str, Value>::new(), -1i32),
                false, Some(Duration::from_secs(2)), &kit.events, move |_| drop(hold));
}

fn print(kit: &Kit, tokens: &Rc<RefCell<Tokens>>, asked: Asked) {
    let Ok((_handle, app_id, _parent, doc, fd, options)) = asked.message().body()
        .deserialize::<(OwnedObjectPath, String, String, String, zbus::zvariant::OwnedFd, Options)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "Print takes a handle, an app, a window, a title, a file and options");
    };
    let chosen = get::<u32>(&options, "token").and_then(|t| tokens.borrow_mut().take(t, &app_id, Instant::now()));
    let Some(chosen) = chosen else {
        tell(kit, t!("screenshot-print-lost"));
        return asked.reply(&failed());
    };
    let document: OwnedFd = fd.into();
    kit.hold();
    let kit = kit.clone();
    crate::common::run::on_thread(move || cups::send(&chosen, &doc, document), move |sent| {
        kit.release();
        match sent {
            Ok(_) => asked.reply(&answer(DONE, HashMap::new())),
            Err(why) => {
                tell(&kit, &why);
                asked.reply(&answer(FAILED, HashMap::new()))
            }
        }
    });
}

pub fn offer(kit: &Kit) -> Offer {
    let kit = kit.clone();
    let tokens = Rc::new(RefCell::new(Tokens::default()));
    Offer {
        name: INTERFACE.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| match asked.member().as_str() {
            "PreparePrint" => prepare(&kit, &tokens, asked),
            "Print" => print(&kit, &tokens, asked),
            _ => asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "Not part of Print"),
        }),
        props: Box::new(|| Props::from([("version".to_string(), zbus::zvariant::OwnedValue::from(1u32))])),
        set: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::portal::FAILED;
    use super::super::portal_ask::testkit::{call, props, Reply, Rig};
    use crate::toolkit::theme::{drawn, Theme};
    use zbus::zvariant::{ObjectPath, OwnedValue};

    fn chosen() -> Choices {
        Choices { printer: "Office".into(), copies: 2, pages: Some(vec![(1, 3), (5, 5)]), sides: Sides::LongEdge, colour: false,
                  paper: Paper::A4, orientation: Orientation::Landscape }
    }

    fn printer(name: &str, disabled: bool) -> Printer {
        Printer { name: name.into(), state: "idle".into(), disabled }
    }

    fn window(listing: Listing) -> (PrintWindow, Rc<RefCell<Vec<u32>>>) {
        let said: Rc<RefCell<Vec<u32>>> = Rc::new(RefCell::new(Vec::new()));
        let s = said.clone();
        let done: Done = Rc::new(move |a: Answer| s.borrow_mut().push(a.0));
        let start = Choices { printer: String::new(), copies: 1, pages: None, sides: Sides::One, colour: true, paper: Paper::A4, orientation: Orientation::Portrait };
        (PrintWindow::new("Print report.pdf".into(), listing, &start, done, |c| prepared(7, &c)), said)
    }

    fn two() -> Listing {
        Ok((vec![printer("Office", false), printer("Old", true)], Some("Old".into())))
    }

    fn over_the_wire(a: &Answer) -> (u32, HashMap<String, OwnedValue>) {
        let ctx = zbus::zvariant::serialized::Context::new_dbus(zbus::zvariant::LE, 0);
        let bytes = zbus::zvariant::to_bytes(ctx, a).expect("serialises");
        bytes.deserialize::<Reply>().expect("reads back").0
    }

    #[test]
    fn a_chosen_print_answers_with_settings_page_setup_and_a_token() {
        let (code, results) = over_the_wire(&prepared(42, &chosen()));
        assert_eq!(code, DONE);
        assert_eq!(u32::try_from(results["token"].try_clone().expect("token")).expect("a number"), 42);
        let settings = HashMap::<String, OwnedValue>::try_from(results["settings"].try_clone().expect("settings")).expect("a dictionary");
        let text = |k: &str| String::try_from(settings[k].try_clone().expect(k)).expect("text");
        assert_eq!(text("printer"), "Office");
        assert_eq!(text("n-copies"), "2");
        assert_eq!(text("print-pages"), "ranges");
        assert_eq!(text("page-ranges"), "0-2,4");
        assert_eq!(text("duplex"), "horizontal");
        assert_eq!(text("use-color"), "false");
        assert_eq!(text("paper-format"), "iso_a4");
        let setup = HashMap::<String, OwnedValue>::try_from(results["page-setup"].try_clone().expect("page")).expect("a dictionary");
        assert_eq!(String::try_from(setup["Name"].try_clone().expect("name")).expect("text"), "iso_a4");
        assert_eq!(f64::try_from(setup["Width"].try_clone().expect("w")).expect("a size"), 297.0, "sideways swaps the sides");
        assert_eq!(f64::try_from(setup["Height"].try_clone().expect("h")).expect("a size"), 210.0);
        assert_eq!(String::try_from(setup["Orientation"].try_clone().expect("o")).expect("text"), "landscape");

        assert_eq!(over_the_wire(&cancelled()).1.len(), 0);
        assert_eq!(over_the_wire(&failed()).0, FAILED);
    }

    #[test]
    fn what_a_program_sent_last_time_is_filled_in() {
        let text = |s: &str| OwnedValue::try_from(Value::from(s.to_string())).expect("value");
        let settings: Options = HashMap::from([
            ("printer".into(), text("Office")), ("n-copies".into(), text("3")), ("duplex".into(), text("vertical")),
            ("paper-format".into(), text("na_letter")),
        ]);
        let setup: Options = HashMap::from([("Orientation".into(), text("reverse_landscape"))]);
        let c = before(&settings, &setup);
        assert_eq!((c.printer.as_str(), c.copies, c.sides, c.paper, c.orientation), ("Office", 3, Sides::ShortEdge, Paper::Letter, Orientation::Landscape));
    }

    #[test]
    fn a_token_works_once_for_the_program_it_was_given_to() {
        let mut t = Tokens::default();
        let now = Instant::now();
        let a = t.give("org.example.A", chosen(), now);
        assert_ne!(a, 0);
        assert!(t.take(a, "org.example.B", now).is_none(), "another program cannot use it");
        assert!(t.take(a, "org.example.A", now + TOKEN_LIFE).is_none(), "it lapses");
        let b = t.give("org.example.A", chosen(), now);
        assert_ne!(a, b);
        assert_eq!(t.take(b, "org.example.A", now), Some(chosen()));
        assert!(t.take(b, "org.example.A", now).is_none(), "it works once");
        assert!(t.take(0, "org.example.A", now).is_none());
        for _ in 0..(MOST_TOKENS * 2) {
            t.give("org.example.A", chosen(), now);
        }
        assert_eq!(t.held.len(), MOST_TOKENS);
    }

    #[test]
    fn a_print_that_cannot_be_made_says_why_and_stays_shut() {
        let (mut w, said) = window(two());

        assert_eq!(w.why().printer.as_deref(), Some("Old is paused."));
        w.update(Msg::Print);
        assert!(said.borrow().is_empty());
        w.update(Msg::Printer(0));
        assert!(w.why().none());
        w.update(Msg::AllPages(1));
        assert_eq!(w.why().pages, Some(cups::bad_range()), "a range with nothing in it");
        w.update(Msg::Range("3-1".into()));
        w.update(Msg::Print);
        w.update(Msg::Copies("0".into()));
        assert_eq!(w.why().copies, Some(cups::bad_copies()));
        w.update(Msg::Print);
        assert!(said.borrow().is_empty(), "nothing is answered while a box is wrong");
        w.update(Msg::Range("1-3, 5".into()));
        w.update(Msg::Copies("2".into()));
        w.update(Msg::Sides(1));
        w.update(Msg::Colour(1));
        w.update(Msg::Paper(3));
        w.update(Msg::Orientation(1));
        w.update(Msg::Print);
        assert_eq!(*said.borrow(), vec![DONE]);
        let c = w.choices().expect("choices");
        assert_eq!((c.printer.as_str(), c.copies, c.pages.clone(), c.sides, c.colour, c.paper, c.orientation),
                   ("Office", 2, Some(vec![(1, 3), (5, 5)]), Sides::LongEdge, false, Paper::Letter, Orientation::Landscape));
        let _ = w.view();
        let (none, _) = window(Ok((Vec::new(), None)));
        assert_eq!(none.why().printer.as_deref(), Some(no_printer()));
        let (down, _) = window(Err(not_answering()));
        assert_eq!(down.why().printer.as_deref(), Some(not_answering()));
        assert!(none.choices().is_none() && down.choices().is_none());
    }

    #[test]
    fn cups_words_are_read_into_a_listing() {
        let out = "printer Office is idle.  enabled since x\nsystem default destination: Office\n";
        assert_eq!(listing(true, out, "").map(|(p, d)| (p.len(), d)), Ok((1, Some("Office".into()))));
        assert_eq!(listing(false, "", "lpstat: No destinations added.").map(|(p, _)| p.len()), Ok(0));
        assert_eq!(listing(false, "", "lpstat: Connection refused").map(|(p, _)| p.len()), Err(not_answering()));
    }

    #[test]
    fn with_no_screen_a_print_window_fails_and_a_print_without_a_token_fails() {
        let rig = Rig::new();
        let _p = rig.a.serve(super::super::portal::PATH, offer(&rig.kit())).expect("serve");
        let handle = ObjectPath::try_from("/org/freedesktop/portal/desktop/request/1_1/p1").expect("path");
        let none: HashMap<&str, Value> = HashMap::new();
        let r: Reply = call(&rig, INTERFACE, "PreparePrint", &(&handle, "org.example.App", "", "Report", &none, &none, &none)).expect("reply");
        assert_eq!(r.0, FAILED);
        assert!(r.1.is_empty());
        let null = std::fs::File::open("/dev/null").expect("a file");
        let fd = zbus::zvariant::Fd::from(&null);
        let r: Reply = call(&rig, INTERFACE, "Print", &(&handle, "org.example.App", "", "Report", fd, &none)).expect("reply");
        assert_eq!(r.0, FAILED, "a print with no token goes nowhere");
        assert_eq!(rig.stay.busy.get(), 0);
        let p = props(&rig, INTERFACE).expect("properties");
        assert!(p.contains_key("version"));
    }

    #[test]
    fn the_print_window_draws_only_pairs_the_table_holds() {
        drawn::none(drawn::in_every_look("the print window", 8, || {
            for (state, listing, range, copies) in [("ready", two(), "", "1"), ("wrong", two(), "3-1", "0"), ("none", Ok((Vec::new(), None)), "", "1")] {
                let (mut w, _) = window(listing);
                if state == "ready" {
                    w.update(Msg::Printer(0));
                }
                if !range.is_empty() {
                    w.update(Msg::AllPages(1));
                    w.update(Msg::Range(range.into()));
                }
                w.update(Msg::Copies(copies.into()));
                drawn::window(state, w.view(), Theme::chosen(), (SIZE.0, 640), false);
            }
        }));
    }

    #[test]
    #[ignore = "writes pictures when TRINDLE_SHOTS names a folder"]
    fn shoot_the_print_window() {
        let Some(dir) = std::env::var_os("TRINDLE_SHOTS").map(std::path::PathBuf::from) else { return };
        std::fs::create_dir_all(&dir).expect("folder");
        for light in [false, true] {
            for (name, range, copies) in [("ready", "", "1"), ("wrong", "3-1", "0")] {
                let (mut w, _) = window(two());
                w.update(Msg::Printer(0));
                w.update(Msg::Sides(1));
                if !range.is_empty() {
                    w.update(Msg::AllPages(1));
                    w.update(Msg::Range(range.into()));
                }
                w.update(Msg::Copies(copies.into()));
                for _ in 0..2 {
                    let mut ui = crate::toolkit::ui::Ui::new(w.view(), Theme::new(light, &crate::common::style::accent()));
                    let mut pix = tiny_skia::Pixmap::new(SIZE.0, SIZE.1).expect("picture");
                    ui.paint(&mut pix.as_mut(), 1.0);
                    pix.save_png(dir.join(format!("print-{name}-{}.png", if light { "light" } else { "dark" }))).expect("saved");
                }
            }
        }
    }
}
