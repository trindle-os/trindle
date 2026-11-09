//! The quick panels opened from the tray row: sound, Bluetooth, Wi-Fi,
//! battery, brightness, the clipboard and notifications. Each fills the
//! ring's middle; a question (a PIN, a password) takes its place until it
//! is answered.

mod questions;
mod sound;
mod bluetooth;
mod wifi;
mod battery;
mod clip_panel;
mod notifications;
mod shortcuts;
#[cfg(test)]
pub mod tests;

use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::process::Child;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use battery::battery_view;
use shortcuts::{settings_page, shortcut_rows};
use sound::PCT_WIDTH;
use zeroize::Zeroizing;

use crate::common::clipboard;
use crate::common::kept_order::KeptOrder;
use crate::ring::{Job, menu_button, meter, sni, view};
use crate::ring::ctx::{Ctx, Msg, msg};
use crate::ring::notify::Note;
use crate::settings::ui::icon;
use crate::toolkit::{t, theme};
use crate::toolkit::bus::Served;
use crate::toolkit::event::Flow;
use crate::toolkit::text::{Align, Style};
use crate::toolkit::widget::{button, column, drawing, label, Length, Node, row, scroll, slider, space, title};

const BODY_GAP: f32 = 8.0;

const FRAME_GAP: f32 = 12.0;

const HEAD_GAP: f32 = 8.0;

const ROW_GAP: f32 = 8.0;

const LIST_GAP: f32 = 0.0;

const PAGE_GAP: f32 = 8.0;

const HEAD_ICON: f32 = 18.0;

const ROW_ICON: f32 = 16.0;

const LINE_HEIGHT: f32 = 22.0;

const BIG_TEXT: f32 = 44.0;

const BIG_WEIGHT: u16 = 200;

const BIG_HEIGHT: f32 = 58.0;

const BOLD: u16 = 600;

const LIST_HEIGHT: f32 = 170.0;

const TRAY_PER_ROW: usize = 3;

const COLUMN_SHARE: f32 = 0.78;

const LIST_SHARE: f32 = 0.48;

const POP_SHARE: f32 = 0.68;

const POP_LEAST: f32 = 300.0;

const POP_BODY_SHARE: f32 = 0.64;

const NO_ICON: &str = "window-close-symbolic";

const BLANK: &str = "\u{a0}";

fn frame(head: Node<Msg>, body: Vec<Node<Msg>>, said: &str, width: Option<f32>, disc: Option<f32>) -> Node<Msg> {
    let said_row = label(if said.is_empty() { BLANK.to_string() } else { said.to_string() }).cut_after(2);
    let (frame_gap, body_gap) = (FRAME_GAP, BODY_GAP);
    let between = |tall: f32| space().height(Length::Fixed(tall));
    let mut rows = vec![head, between(frame_gap)];
    for (i, row) in body.into_iter().enumerate() {
        if i > 0 {
            rows.push(between(body_gap));
        }
        rows.push(row);
    }
    rows.extend([between(frame_gap), said_row]);
    rows.extend([between(frame_gap), button(NO_ICON, t!("ring-close")).on_press(msg(|f| f.quick_done())).well()]);
    on_disc(column(rows).gap(0.0).centred(), disc).width(width.map_or(Length::Fill, Length::Fixed)).on_escape(msg(|f| f.quick_done()))
}

pub fn on_disc(column: Node<Msg>, disc: Option<f32>) -> Node<Msg> {
    match disc {
        Some(across) => column.in_circle(across, crate::common::style::ring_page_margin()),
        None => column,
    }
}

trait Well {
    fn well(self) -> Node<Msg>;
}

impl Well for Node<Msg> {
    fn well(self) -> Node<Msg> {
        view::middle::band(vec![self.size(menu_button()).tinted(Job::Tools.share())])
    }
}

fn m(f: impl Fn(&mut Quick) + 'static) -> Msg {
    msg(move |face| f(&mut face.quick))
}

#[derive(Clone)]
struct Back {
    ctx: Weak<Ctx>,
    gen: Rc<Cell<u32>>,
    at: u32,
}

impl Back {
    fn live(&self) -> bool {
        self.gen.get() == self.at && self.ctx.strong_count() > 0
    }

    fn then(&self, f: impl FnOnce(&mut Quick) + 'static) {
        let Some(ctx) = self.ctx.upgrade() else { return };
        let at = self.at;
        ctx.face(move |face| {
            if face.quick.gen.get() == at {
                f(&mut face.quick);
            }
        });
    }
}

struct Level {
    name: &'static str,
    target: &'static str,
    out: bool,
    value: f32,
    pct: String,
    muted: bool,
    touched: Option<Instant>,
    nodes: Vec<(i64, String)>,
    chosen: usize,
}

#[derive(Clone, Debug, PartialEq)]
struct Dev {
    mac: String,
    name: String,
    paired: bool,
    connected: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct Net {
    ssid: String,
    signal: u32,
    inuse: bool,
    secured: bool,
}

enum Body {
    Empty,
    Sound(Vec<Level>),

    Bt(Option<bool>, Vec<Dev>),
    Wifi(Option<bool>, Vec<Net>, HashSet<String>),
    Battery { big: String, state: String, health: String, mode: Option<String> },
    Bright(f32, String),
    Clip(Vec<clipboard::Item>, String),
    Notes(Vec<(Note, String)>),
    Whole(Note, String),

    Tray,

    Shortcuts(Vec<crate::common::key_list::Row>),
}

type Done = Box<dyn FnOnce(&mut Quick, Option<Zeroizing<String>>)>;

struct Ask {
    icon: &'static str,
    title: String,
    words: &'static str,
    code: Option<String>,

    entry: Option<(&'static str, bool)>,
    yes: &'static str,
    no: &'static str,
    text: Zeroizing<String>,
    done: Option<Done>,
}

pub struct Quick {
    ctx: Option<Rc<Ctx>>,

    gen: Rc<Cell<u32>>,
    panel: Option<&'static str>,
    head: (&'static str, &'static str),
    said: String,
    body: Body,
    ask: Option<Ask>,

    turn: u32,
    ticked: bool,
    tick_gen: u32,
    bright_gen: u32,
    width: Option<f32>,

    across: Option<f32>,
    pop_width: Option<f32>,
    list_height: f32,
    wifi_asking: bool,
    agent: Option<Served>,
    bt_names: HashMap<String, String>,
    scan: Option<Child>,

    meter: Option<meter::Meter>,
    heard: f32,
    busy: HashSet<String>,

    order: KeptOrder,
}

pub fn tray_rows() -> Node<Msg> {
    let mut all: Vec<Option<Node<Msg>>> = sni::tray().map(|t| t.icons(true)).unwrap_or_default().into_iter().map(|(_, n)| Some(n)).collect();
    if all.is_empty() {
        return line_of(t!("ring-tray-none").into(), true, 400);
    }
    let mut rows = Vec::new();
    while !all.is_empty() {
        let take = all.len().min(TRAY_PER_ROW);
        let icons: Vec<Node<Msg>> = all.drain(..take).flatten().collect();
        rows.push(view::middle::band(icons));
    }
    column(rows).gap(crate::toolkit::theme::sheet().ring_band_gap).centred()
}

fn banded(rows: Vec<Node<Msg>>) -> Vec<Node<Msg>> {
    crate::toolkit::widget::banded_list(rows)
}

fn line_of(text: String, dim: bool, weight: u16) -> Node<Msg> {
    let (spoken, copied) = (text.clone(), text.clone());
    drawing((1.0, LINE_HEIGHT), spoken, move |canvas, pen| {
        let mut style = Style::new(theme::sheet().text.0 * pen.scale);
        style.weight = weight;
        let dots = pen.fonts.layout("\u{2026}", &style, None);
        let layout = pen.fonts.layout(&text, &style, None).cut(canvas.width() as f32, &dots);
        let y = ((canvas.height() as f32 - layout.height) / 2.0).round();
        pen.fonts.draw(canvas, &layout, 0.0, y, if dim { pen.theme.muted } else { pen.theme.fg });
    }).copies(copied).width(Length::Fill)
}

fn big_line(text: String) -> Node<Msg> {
    let (spoken, copied) = (text.clone(), text.clone());
    drawing((1.0, BIG_HEIGHT), spoken, move |canvas, pen| {
        let mut style = Style::new(BIG_TEXT * pen.scale);
        style.weight = BIG_WEIGHT;
        style.align = Align::Start;
        let layout = pen.fonts.layout(&text, &style, None);
        let x = ((canvas.width() as f32 - layout.width) / 2.0).max(0.0).round();
        let y = ((canvas.height() as f32 - layout.height) / 2.0).round();
        pen.fonts.draw(canvas, &layout, x, y, pen.theme.fg);
    }).copies(copied).width(Length::Fill)
}

impl Drop for Quick {
    fn drop(&mut self) {
        self.stop_scan();
    }
}

impl Quick {
    pub fn new(ctx: &Rc<Ctx>) -> Quick {
        Quick::with(Some(ctx.clone()))
    }

    fn ctx(&self) -> &Rc<Ctx> {
        self.ctx.as_ref().expect("a panel that is only drawn asks the ring for nothing")
    }

    fn with(ctx: Option<Rc<Ctx>>) -> Quick {
        Quick {
            ctx, gen: Rc::new(Cell::new(0)), panel: None, head: ("", ""), said: String::new(),
            body: Body::Empty, ask: None, turn: 0, ticked: false, tick_gen: 0,
            bright_gen: 0, width: None, across: None, pop_width: None, list_height: LIST_HEIGHT, wifi_asking: false,
            agent: None, bt_names: HashMap::new(), scan: None, meter: None, heard: 0.0, busy: HashSet::new(), order: KeptOrder::default(),
        }
    }

    pub fn set_size(&mut self, across: i32) {
        let across = across as f32;
        self.across = Some(across);
        self.width = Some(across * COLUMN_SHARE);
        self.list_height = across * LIST_SHARE;
        self.pop_width = Some(self.pop_size(across as i32) as f32 * POP_BODY_SHARE);
    }

    pub fn pop_size(&self, across: i32) -> i32 {
        (across as f32 * POP_SHARE).max(POP_LEAST).min(across as f32) as i32
    }

    pub fn asking(&self) -> bool {
        self.ask.is_some()
    }

    pub fn is_open(&self) -> bool {
        self.panel.is_some()
    }

    pub fn is_tray(&self) -> bool {
        self.panel == Some("tray")
    }

    pub fn shows(&self, panel: &str) -> bool {
        self.panel == Some(panel)
    }

    pub fn show(&mut self, panel: &str) {
        let was = self.panel;
        self.hide();
        self.said.clear();
        let (key, icon, words) = match panel {
            "audio" => ("audio", "audio-volume-high-symbolic", t!("ring-quick-title-sound")),
            "bluetooth" => ("bluetooth", "bluetooth-active-symbolic", t!("ring-quick-title-bluetooth")),
            "network" => ("network", "network-wireless-symbolic", t!("ring-quick-title-wifi")),
            "brightness" => ("brightness", "display-brightness-symbolic", t!("ring-quick-title-brightness")),
            "clipboard" => ("clipboard", "edit-paste-symbolic", t!("ring-quick-title-clipboard")),
            "notes" => ("notes", "preferences-system-notifications-symbolic", t!("ring-quick-title-notifications")),
            "tray" => ("tray", "view-more-symbolic", t!("ring-quick-title-tray")),
            "shortcuts" => ("shortcuts", "input-keyboard-symbolic", t!("ring-quick-title-shortcuts")),
            _ => ("power", "battery-good-symbolic", t!("ring-quick-title-battery")),
        };

        if was != Some(key) {
            self.order.forget();
        }
        if key != "audio" {
            self.meter = None;
        }
        self.panel = Some(key);
        self.head = (icon, words);
        self.turn += 1;
        match key {
            "audio" => self.sound(),
            "bluetooth" => self.bluetooth(),
            "network" => self.wifi(),
            "brightness" => self.brightness(),
            "clipboard" => self.clipboard(),
            "notes" => self.notes(),
            "tray" => self.body = Body::Tray,
            "shortcuts" => self.body = Body::Shortcuts(crate::common::key_list::rows()),
            _ => self.battery(),
        }
    }

    pub fn hide(&mut self) {
        self.gen.set(self.gen.get().wrapping_add(1));
        self.stop_scan();
        self.stop_agent();
        self.meter = None;
        self.panel = None;
        self.cancel_ask();
        self.body = Body::Empty;
        self.wifi_asking = false;
        self.ticked = false;
        self.busy.clear();
    }

    pub fn show_note(&mut self, id: u32) {
        self.show("notes");
        self.whole(id);
    }

    fn refresh_now(&mut self) {
        if let Some(p) = self.panel {
            self.show(p);
        }
    }

    fn say(&mut self, words: &str) {
        self.said = words.to_string();
    }

    fn back(&self) -> Back {
        Back { ctx: Rc::downgrade(self.ctx()), gen: self.gen.clone(), at: self.gen.get() }
    }

    fn repeat(&self, every: Duration, f: impl Fn(&mut Quick) + 'static) {
        let (b, f) = (self.back(), Rc::new(f));
        self.ctx().lp.every(every, move || {
            if !b.live() {
                return Flow::Stop;
            }
            let f = f.clone();
            b.then(move |q| f(q));
            Flow::Continue
        });
    }

    fn disc(&self) -> Option<f32> {
        self.across
    }

    pub fn view(&self) -> Node<Msg> {
        let Some(panel) = self.panel else { return space() };
        if let Some(a) = &self.ask {
            return self.ask_view(a);
        }
        let page = settings_page(panel);
        let head = row(vec![
            icon(self.head.0, HEAD_ICON),
            title(self.head.1),
            button("preferences-system-symbolic", t!("ring-quick-more-settings")).on_press(msg(move |face| face.settings(Some(page)))).well(),
        ]).gap(HEAD_GAP).width(Length::Shrink);
        frame(head, self.body_view(), &self.said, self.width, self.disc())
    }

    fn body_view(&self) -> Vec<Node<Msg>> {
        match &self.body {
            Body::Empty => vec![],
            Body::Sound(levels) => self.sound_view(levels),
            Body::Bt(on, devices) => self.bt_view(*on, devices),
            Body::Wifi(on, nets, saved) => self.wifi_view(*on, nets, saved),
            Body::Battery { big, state, health, mode } => battery_view(big, state, health, mode.as_deref()),
            Body::Bright(value, pct) => vec![row(vec![
                icon("display-brightness-symbolic", ROW_ICON),
                slider(*value, 1.0, 100.0, 1.0, t!("ring-quick-brightness-slider"), |v| m(move |q| q.set_brightness(v))),
                label(pct.clone()).width(Length::Fixed(PCT_WIDTH)),
            ]).gap(ROW_GAP)],
            Body::Clip(items, filter) => self.clip_view(items, filter),
            Body::Notes(notes) => self.notes_view(notes),
            Body::Whole(n, icon_name) => self.whole_view(n, icon_name),
            Body::Tray => vec![scroll("quick:tray", tray_rows()).height(Length::Fixed(self.list_height))],
            Body::Shortcuts(rows) => vec![self.scrolling(banded(shortcut_rows(rows)))],
        }
    }

    fn scrolling(&self, rows: Vec<Node<Msg>>) -> Node<Msg> {
        scroll(format!("quick:{}", self.panel.unwrap_or("")), column(rows).gap(LIST_GAP)).height(Length::Fixed(self.list_height))
    }
}
