//! A Debian package someone has downloaded, such as Chrome, VS Code, Zoom or a printer driver.

use crate::toolkit::t;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::rc::{Rc, Weak};
use std::time::Duration;

use super::boxed::install::Installed;
use super::origin::{self, Origin, Verdict};
use super::app_page::{fact_line, part};
use super::listing::BLANK;
use super::pages::pictures::icon;
use super::screen::CARD_GAP;
use crate::common::filling::Dial;
use crate::common::run::{on_thread, text};
use crate::settings::ui::{INSTALL_ICON, UPDATE_ICON};
use crate::toolkit::event::{Flow, Loop, Post};
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::theme;
use crate::toolkit::widget::{button, card, column, drawing, headline, label, layers, main_action, note, page_title, row, scroll, space, Length, Node, Tone};
use crate::toolkit::window::{Display, Kind};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Deb {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub about: String,
    pub installed_kib: Option<u64>,
    pub maker: String,
    pub arch: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum State {
    New,
    Elsewhere(String),
    Trindle,
    Service,
}

pub(super) fn fields(control: &str) -> HashMap<String, String> {
    let mut out: HashMap<String, String> = HashMap::new();
    let mut last: Option<String> = None;
    for line in control.lines() {
        if let Some(more) = line.strip_prefix(' ').or_else(|| line.strip_prefix('\t')) {
            if let Some(v) = last.as_ref().and_then(|k| out.get_mut(k)) {
                v.push('\n');
                v.push_str(more);
            }
            continue;
        }
        if let Some((k, v)) = line.split_once(':') {
            let k = k.trim().to_string();
            out.insert(k.clone(), v.trim().to_string());
            last = Some(k);
        }
    }
    out
}

fn paragraphs(lines: &[&str]) -> String {
    let mut out = String::new();
    for line in lines {
        if line.trim() == "." {
            out.push_str("\n\n");
        } else if line.starts_with(' ') || line.starts_with('\t') {
            if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(line.trim());
        } else {
            if !out.is_empty() && !out.ends_with('\n') {
                out.push(' ');
            }
            out.push_str(line.trim());
        }
    }
    out.trim().to_string()
}

pub fn parse(control: &str) -> Option<Deb> {
    let f = fields(control);
    let get = |k: &str| f.get(k).cloned().unwrap_or_default();
    let (name, version) = (get("Package"), get("Version"));
    if name.is_empty() || version.is_empty() {
        return None;
    }
    let description = get("Description");
    let mut lines = description.lines();
    let summary = lines.next().unwrap_or("").trim().to_string();
    let rest: Vec<&str> = lines.collect();
    Some(Deb {
        name,
        version,
        summary,
        about: paragraphs(&rest),
        installed_kib: get("Installed-Size").parse().ok(),
        maker: maker_name(&get("Maintainer")),
        arch: get("Architecture"),
    })
}

fn maker_name(maintainer: &str) -> String {
    let name = maintainer.split('<').next().unwrap_or("").trim().trim_matches('"');
    if name.is_empty() { maintainer.trim_matches(['<', '>', ' ']).to_string() } else { name.to_string() }
}

pub fn valid_name(name: &str) -> bool {
    name.len() >= 2
        && name.starts_with(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit())
        && name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || "+-.".contains(c))
}

fn look(path: &Path) -> Result<(Deb, State), String> {
    let out = Command::new("dpkg-deb").arg("-f").arg(path)
        .args(["Package", "Version", "Description", "Installed-Size", "Maintainer", "Architecture"])
        .stderr(Stdio::null()).output().map_err(|e| e.to_string())?;
    let deb = out.status.success().then(|| parse(&String::from_utf8_lossy(&out.stdout))).flatten()
        .filter(|d| valid_name(&d.name))
        .ok_or(t!("store-deb-not-finished"))?;
    let state = state_of(&deb, path);
    Ok((deb, state))
}

fn is_trindle(name: &str) -> bool {
    name == "trindle" || name.starts_with("trindle-")
}

fn listed(path: &Path) -> Vec<String> {
    Command::new("dpkg-deb").arg("-c").arg(path).stderr(Stdio::null()).output()
        .map(|o| String::from_utf8_lossy(&o.stdout).lines()
            .filter_map(|l| l.split_whitespace().nth(5).map(String::from)).collect())
        .unwrap_or_default()
}

fn state_of(deb: &Deb, path: &Path) -> State {
    if is_trindle(&deb.name) {
        return State::Trindle;
    }
    let here = text(&["dpkg", "--print-architecture"]);
    let foreign = text(&["dpkg", "--print-foreign-architectures"]);
    let fits = deb.arch.is_empty() || deb.arch == "all" || deb.arch == here || foreign.lines().any(|a| a == deb.arch);
    if !fits {
        return State::Elsewhere(deb.arch.clone());
    }
    if super::parts::cannot_box(&listed(path)) { State::Service } else { State::New }
}

fn sentence(deb: &Deb, state: &State) -> (String, Tone) {
    match state {
        State::New => (t!("store-deb-ready", name = &deb.name), Tone::Calm),
        State::Elsewhere(arch) => (t!("store-deb-elsewhere", arch = arch), Tone::Problem),
        State::Trindle => (t!("store-deb-trindle").into(), Tone::Problem),
        State::Service => (t!("store-deb-service", name = &deb.name), Tone::Problem),
    }
}

fn caution() -> &'static str {
    t!("store-deb-caution")
}

#[derive(Clone)]
pub struct Msg(Rc<dyn Fn(&mut Package)>);

fn go(f: impl Fn(&mut Package) + 'static) -> Msg {
    Msg(Rc::new(f))
}

fn says(words: &str, tone: Tone) -> Node<Msg> {
    if tone == Tone::Problem {
        let text = words.to_string();
        super::help::red(words, go(move |_| super::help::open_with(super::help::DEB, &text)))
    } else {
        note(words.to_string()).tone(tone)
    }
}

type Then = Box<dyn FnOnce(&mut Package)>;

pub struct Package {
    events: Loop,
    asks: Post<Then>,
    me: Rc<RefCell<Weak<Shown<Package>>>>,
    path: PathBuf,
    found: Option<Result<(Deb, State), String>>,
    origin: Option<(Origin, Verdict)>,
    boxed: Option<String>,
    headline: (String, Tone),
    said: Option<(String, Tone)>,
    dial: Option<Dial>,
}

impl Package {
    fn then<T: 'static>(&self, f: impl FnOnce(&mut Package, T) + 'static) -> impl FnOnce(T) + 'static {
        let asks = self.asks.clone();
        move |t| {
            asks.send(Box::new(move |p| f(p, t)));
        }
    }

    fn busy(&self) -> bool {
        self.dial.as_ref().is_some_and(Dial::running)
    }

    fn begin(&mut self, set: impl FnOnce(&mut Dial)) {
        let mut d = Dial::default();
        set(&mut d);
        self.dial = Some(d);
        let me = self.me.clone();
        self.events.every(super::TICK, move || {
            let Some(shown) = me.borrow().upgrade() else { return Flow::Stop };
            if !shown.with(Package::busy) {
                return Flow::Stop;
            }
            shown.change(|_| {});
            Flow::Continue
        });
    }

    fn settle(&mut self, words: &str, after: impl FnOnce(&mut Package) + 'static) {
        if let Some(d) = &mut self.dial {
            d.finish(true, words);
        }
        let back = self.then(move |p: &mut Package, ()| {
            p.dial = None;
            after(p);
        });
        self.events.after(Duration::from_secs(2), move || back(()));
    }

    fn failed(&mut self, e: String) {
        self.dial = None;
        self.said = Some((e.clone(), Tone::Problem));
        self.headline = (e, Tone::Problem);
    }

    fn stand(&mut self) {
        let Some(Ok((deb, state))) = &self.found else { return };
        self.headline = match self.boxed {
            Some(_) if *state == State::New => (t!("store-deb-installed", name = &deb.name), Tone::Plain),
            _ => sentence(deb, state),
        };
    }
}

impl App for Package {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        (msg.0)(self)
    }

    fn view(&self) -> Node<Msg> {
        let k = theme::sheet();
        let deb = match &self.found {
            Some(Ok((deb, state))) => Some((deb, state)),
            _ => None,
        };
        let file = self.path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        let name = deb.map_or(file, |(d, _)| d.name.clone());
        let line = deb.map(|(d, _)| d.summary.clone()).filter(|s| !s.trim().is_empty()).unwrap_or_else(|| BLANK.into());
        let names = column(vec![page_title(name).cut(), headline(line).cut(), says(&self.headline.0, self.headline.1)])
            .gap(4.0);
        let head = row(vec![icon("package-x-generic-symbolic".into(), theme::app_page().icon), names, self.actions(deb)]).gap(CARD_GAP);
        let failure = match &self.said {
            Some((w, t)) if *w != self.headline.0 => says(w, *t),
            _ => note(BLANK),
        };
        let mut items = vec![column(vec![head, failure]).gap(k.gap), self.origin_card(),
                             card(row(vec![icon("dialog-warning-symbolic".into(), 20.0), note(caution()).width(Length::Fill)]).gap(k.gap))];
        if let Some((deb, state)) = deb {
            items.extend(self.details(deb, state));
        }
        scroll("package", column(items).gap(theme::bento().page_gap).padding(k.page_padding))
    }
}

pub fn origin_card<M: Clone + 'static>(found: Option<&(Origin, Verdict)>, link: M) -> Node<M> {
    let (words, warn) = match found {
        None => (t!("store-deb-checking-origin").to_string(), false),
        Some((o, v)) => origin::words(o, v),
    };
    let mut lines = vec![label(t!("store-deb-where")), note(words).tone(if warn { Tone::Warn } else { Tone::Plain })];
    if found.is_some_and(|(o, _)| !o.hosts().is_empty()) {
        lines.push(note(format!("{}.", origin::credit())));
    }
    card(row(vec![
        icon(if warn { "dialog-warning-symbolic" } else { "web-browser-symbolic" }.into(), 24.0),
        column(lines).gap(4.0).width(Length::Fill),
        crate::toolkit::widget::link(origin::HOME).on_press(link),
    ]).gap(12.0).centred())
}

impl Package {
    fn origin_card(&self) -> Node<Msg> {
        origin_card(self.origin.as_ref(), go(|_| {
            crate::settings::ui::open_link(origin::HOME);
        }))
    }

    fn actions(&self, deb: Option<(&Deb, &State)>) -> Node<Msg> {
        let wide = theme::app_page().action;
        let tall = theme::sheet().action_height;
        let can = deb.filter(|(_, state)| **state == State::New);
        let remove = match (can, &self.boxed) {
            (Some(_), Some(_)) if self.dial.is_some() => Some(button("user-trash-symbolic", t!("store-deb-remove"))),
            (Some(_), Some(_)) => Some(crate::toolkit::widget::remove("user-trash-symbolic", t!("store-deb-remove")).key("remove-boxed")
                                           .on_press(go(remove_boxed))),
            _ => None,
        };
        let round = Length::Fixed(theme::icon_button());
        let remove = match remove {
            Some(b) => row(vec![b]).width(round),
            None => space().width(round),
        };
        let main = match (can, &self.dial) {
            (None, _) => space(),
            (Some(_), Some(d)) => row(vec![d.node(), note(if d.words().is_empty() { t!("store-deb-installing") } else { d.words() }.to_string()).tone(d.tone()).cut()]).gap(8.0),
            (Some((deb, _)), None) => {
                let (icon, words) = match self.boxed.as_deref() {
                    None => (INSTALL_ICON, t!("store-deb-install")),
                    Some(have) if have == deb.version => (INSTALL_ICON, t!("store-deb-reinstall")),
                    Some(_) => (UPDATE_ICON, t!("store-deb-update")),
                };
                main_action(icon, words).width(Length::Fixed(wide)).on_press(go(pressed_install_boxed))
            }
        };
        let place = drawing((wide, tall), "", |_, _| {});
        let main = layers(vec![place, main]).width(Length::Fixed(wide)).height(Length::Fixed(tall));
        row(vec![remove, main]).gap(theme::sheet().gap).width(Length::Shrink)
    }

    fn details(&self, deb: &Deb, state: &State) -> Vec<Node<Msg>> {
        let v = &deb.version;
        let mut out = Vec::new();
        if *state == State::New {
            let (head, sub) = match self.boxed.as_deref() {
                None => (t!("store-deb-goes-in").to_string(), t!("store-deb-goes-in-note").to_string()),
                Some(have) if have == v => (t!("store-deb-same-version", version = v), t!("store-deb-same-version-note").into()),
                Some(have) => (t!("store-deb-other-version", version = have), t!("store-deb-other-version-note", version = v)),
            };
            out.push(card(column(vec![label(head), note(sub)]).gap(4.0)));
        }
        let or_dash = |s: String| if s.trim().is_empty() { "\u{2013}".to_string() } else { s };
        out.push(part("dialog-information-symbolic", t!("store-app-facts"), vec![
            fact_line(t!("store-app-fact-version"), or_dash(deb.version.clone())),
            fact_line(t!("store-deb-fact-size"), deb.installed_kib.map_or_else(|| "\u{2013}".to_string(), |kib| super::github::size_text(kib * 1024))),
            fact_line(t!("store-app-fact-made-by"), or_dash(deb.maker.clone())),
            fact_line(t!("store-deb-fact-arch"), or_dash(deb.arch.clone())),
        ]));
        let about = if deb.about.trim().is_empty() { note(t!("store-deb-no-description")) } else { label(deb.about.clone()) };
        out.push(part("text-x-generic-symbolic", t!("store-app-about"), vec![about]));
        out
    }
}
#[cfg(test)]
pub(super) fn made_up(events: &Loop, found: Option<Result<(Deb, State), String>>, origin: Option<(Origin, Verdict)>, boxed: Option<String>,
                      going: bool) -> Package {
    let mut p = Package {
        events: events.clone(), asks: events.post(|_: Then| {}), me: Rc::default(), path: PathBuf::from("/tmp/example_1.2_amd64.deb"),
        found, origin, boxed, headline: (t!("store-deb-reading").into(), Tone::Plain), said: None,
        dial: going.then(Dial::default),
    };
    p.stand();
    if let Some(Err(e)) = &p.found {
        p.headline = (e.clone(), Tone::Problem);
    }
    p
}

fn deb_of(p: &Package) -> Option<Deb> {
    match &p.found {
        Some(Ok((deb, _))) => Some(deb.clone()),
        _ => None,
    }
}

fn pressed_install_boxed(p: &mut Package) {
    if !crate::toolkit::window::persons_own(crate::common::person_only::APPS_INSTALL) {
        return;
    }
    install_boxed(p);
}

fn install_boxed(p: &mut Package) {
    if deb_of(p).is_none() {
        return;
    }
    p.begin(|d| d.expect(20.0));
    p.said = Some((t!("store-deb-installing-note").into(), Tone::Plain));
    let file = p.path.clone();
    on_thread(move || super::boxed::install::install(&file, |_| {}), p.then(move |p: &mut Package, res: Result<Installed, String>| match res {
        Err(e) => p.failed(e),
        Ok(done) => p.settle(t!("store-done-installed"), move |p| {
            p.said = None;
            p.boxed = super::boxed::version(&done.name);
            p.stand();
            p.headline = if done.missing.is_empty() {
                (t!("store-deb-done").into(), Tone::Plain)
            } else {
                (t!("store-deb-missing", parts = done.missing.join(", ")), Tone::Problem)
            };
            if done.ask {
                crate::common::run::spawn(&["trindle-boxed", "--ask", &done.name]);
            }
        }),
    }));
}

fn remove_boxed(p: &mut Package) {
    let Some(deb) = deb_of(p) else { return };
    match super::boxed::install::remove(&deb.name) {
        Ok(()) => {
            p.said = None;
            p.boxed = super::boxed::version(&deb.name);
            p.stand();
            p.headline = (t!("store-deb-removed").into(), Tone::Plain);
        }
        Err(e) => p.headline = (t!("store-plugins-not-removed", why = e.to_string()), Tone::Problem),
    }
}

pub fn open(events: &Loop, display: &Rc<Display>, path: PathBuf, closed: impl Fn() + 'static) -> Option<Rc<Shown<Package>>> {
    let me: Rc<RefCell<Weak<Shown<Package>>>> = Rc::new(RefCell::new(Weak::new()));
    let m = me.clone();
    let asks = events.post(move |f: Then| {
        if let Some(shown) = m.borrow().upgrade() {
            shown.change(f);
        }
    });
    let package = Package {
        events: events.clone(),
        asks,
        me: me.clone(),
        path: path.clone(),
        found: None,
        origin: None,
        boxed: None,
        headline: (t!("store-deb-reading").into(), Tone::Plain),
        said: None,
        dial: None,
    };
    let kind = Kind::Window { title: t!("store-deb-title").into(), app_id: "org.trindle.Store".into() };
    let shown = Rc::new(Shown::open(events, display, kind, SIZE, package)?);
    *me.borrow_mut() = Rc::downgrade(&shown);
    shown.window.on_close(closed);
    let back = shown.with(|p| p.then(|p: &mut Package, found: Result<(Deb, State), String>| {
        if let Ok((deb, _)) = &found {
            p.boxed = super::boxed::version(&deb.name);
        }
        if let Err(e) = &found {
            p.headline = (e.clone(), Tone::Problem);
        }
        p.found = Some(found);
        p.stand();
    }));
    let checked = shown.with(|p| p.then(|p: &mut Package, found: (Origin, Verdict)| p.origin = Some(found)));
    let file = path.clone();
    on_thread(move || look(&path), back);
    on_thread(move || origin::check(&file), checked);
    Some(shown)
}

pub(super) const SIZE: (u32, u32) = (640 + 48, 620);

#[cfg(test)]
mod tests {
    use super::{is_trindle, maker_name, origin_card, parse, sentence, valid_name, Deb, State};
    use crate::store::origin::{Origin, Verdict};
    use crate::toolkit::widget::Tone;

    #[test]
    fn says_how_the_package_stands() {
        let d = parse("Package: code\nVersion: 1.95\nDescription: An editor\n").unwrap();
        assert_eq!(sentence(&d, &State::New), ("code is ready to install in its own folder.".to_string(), Tone::Calm));
        assert_eq!(sentence(&d, &State::Elsewhere("arm64".into())).1, Tone::Problem);
        assert_eq!(sentence(&d, &State::Trindle).1, Tone::Problem);
        assert_eq!(sentence(&d, &State::Service).1, Tone::Problem);
        assert!(!super::caution().contains("whole computer"));
    }

    #[test]
    fn trindles_own_packages_are_known() {
        for name in ["trindle", "trindle-icons", "trindle-greeter"] {
            assert!(is_trindle(name), "{name}");
        }
        for name in ["trindles", "code", "xtrindle"] {
            assert!(!is_trindle(name), "{name}");
        }
    }

    #[test]
    fn the_origin_card_builds_for_every_answer() {
        let o = Origin { site: Some("dl.example.com".into()), referrer: None };
        for found in [None, Some((Origin::default(), Verdict::Unknown)), Some((o.clone(), Verdict::Clear)),
                      Some((o.clone(), Verdict::Listed("example.com".into()))), Some((o, Verdict::NotChecked))] {
            let _ = origin_card(found.as_ref(), ());
        }
    }

    #[test]
    fn reads_what_dpkg_deb_says() {
        let said = "Package: google-chrome-stable\n\
                    Version: 129.0.6668.58-1\n\
                    Architecture: amd64\n\
                    Maintainer: Chrome Linux Team <chromium-dev@chromium.org>\n\
                    Installed-Size: 355207\n\
                    Description: The web browser from Google\n \
                    Google Chrome is a browser that combines a minimal design with\n \
                    sophisticated technology.\n \
                    .\n \
                    It is fast.\n";
        let d = parse(said).unwrap();
        assert_eq!(d, Deb {
            name: "google-chrome-stable".into(),
            version: "129.0.6668.58-1".into(),
            summary: "The web browser from Google".into(),
            about: "Google Chrome is a browser that combines a minimal design with sophisticated technology.\n\nIt is fast.".into(),
            installed_kib: Some(355207),
            maker: "Chrome Linux Team".into(),
            arch: "amd64".into(),
        });
    }

    #[test]
    fn keeps_a_list_on_its_own_lines() {
        let d = parse("Package: thing\nVersion: 1:2.0~rc1\nDescription: A thing\n Does:\n  - one\n  - two\n").unwrap();
        assert_eq!(d.version, "1:2.0~rc1");
        assert_eq!(d.about, "Does:\n- one\n- two");
        assert_eq!(d.installed_kib, None);
        assert_eq!(d.maker, "");
    }

    #[test]
    fn nothing_without_a_name_and_version() {
        assert_eq!(parse(""), None);
        assert_eq!(parse("Package: thing\n"), None);
        assert_eq!(parse("dpkg-deb: error: 'x.deb' is not a Debian format archive\n"), None);
    }

    #[test]
    fn makers_without_their_address() {
        assert_eq!(maker_name("Microsoft Corporation <vscode-linux@microsoft.com>"), "Microsoft Corporation");
        assert_eq!(maker_name("\"Zoom, Inc.\" <support@zoom.us>"), "Zoom, Inc.");
        assert_eq!(maker_name("<someone@example.org>"), "someone@example.org");
        assert_eq!(maker_name("Brother Industries"), "Brother Industries");
    }

    #[test]
    fn package_names_dpkg_would_accept() {
        assert!(valid_name("code"));
        assert!(valid_name("libstdc++6"));
        assert!(valid_name("zoom"));
        assert!(!valid_name("-W"));
        assert!(!valid_name("Code"));
        assert!(!valid_name("a"));
        assert!(!valid_name("has space"));
    }
}
