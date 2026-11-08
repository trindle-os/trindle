//! The middle of the ring: the tool rows, the time and date, the All apps and battery bands and the cover for the tray.

use crate::ring::{Job, kept, menu_button, quick, sni};
use crate::ring::ctx::{msg, Msg};
use crate::ring::layout::CENTRE;
use crate::toolkit::t;
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::text::Style;
use crate::toolkit::widget::{button, column, drawing, layers, Length, Node, placed, Pointer, row};

use super::{ALL, Face, FAVOURITES};
use super::paint::{fill_round, paint_ring, Scene};

pub(super) const SEGMENT: f64 = 28.0;

pub(super) const ALL_ICON: f32 = 16.0;

pub(super) const BATTERY_TEXT: f32 = 14.0;

const CLOCK_SIZE: f32 = 26.0;

pub(super) const FINDING: (f32, f32) = (196.0, 34.0);

const DATE_SIZE: f32 = 17.0;

pub(super) fn core() -> f32 {
    menu_button()
}

pub(super) fn tool_rows() -> [Vec<(&'static str, &'static str, &'static str)>; 2] {
    [
        vec![("settings", "preferences-system-symbolic", t!("ring-settings")),
             ("store", "system-software-install-symbolic", t!("ring-tool-apps")),
             ("activity", "power-profile-performance-symbolic", t!("ring-tool-activity")),
             ("drives", "drive-harddisk-symbolic", t!("ring-tool-drives"))],
        vec![("clipboard", "edit-paste-symbolic", t!("ring-tool-clipboard")),
             ("screenshot", "camera-photo-symbolic", t!("ring-tool-screenshot")),
             ("notes", "preferences-system-notifications-symbolic", t!("ring-tool-notifications")),
             ("magnifier", "zoom-in-symbolic", t!("ring-tool-magnifier")),
             ("eye-dropper", "color-select-symbolic", t!("ring-tool-eye-dropper")),
             ("report", "dialog-warning-symbolic", t!("ring-tool-report"))],
    ]
}

pub(super) fn lower_tools() -> Vec<(&'static str, &'static str, &'static str)> {
    tool_rows()[1].to_vec()
}

pub(super) fn centre_of(upper: Vec<Node<Msg>>, status: Vec<Node<Msg>>, clock: Node<Msg>, lower: Vec<Node<Msg>>, foot: Option<Node<Msg>>) -> Node<Msg> {
    let mut tools = vec![band(lower)];

    tools.extend(foot);
    let gap = crate::toolkit::theme::sheet().ring_band_gap;
    column(vec![
        column(vec![band(upper), band(status)]).gap(gap).centred(),
        clock,
        column(tools).gap(gap).centred(),
    ])
    .gap(gap)
    .centred()
}

pub(super) fn time_and_date_drawn(time: String, date: String) -> Node<Msg> {
    drawing(FINDING, t!("ring-date-time"), move |pm, pen| {
        let s = pen.scale;
        let mut big = Style::new(CLOCK_SIZE * s);
        big.weight = 600;
        let a = pen.fonts.layout(&time, &big, None);
        let b = pen.fonts.layout(&date, &Style::new(DATE_SIZE * s), None);
        let x = (pm.width() as f32 - a.width - b.width - 8.0 * s) / 2.0;
        pen.fonts.draw(pm, &a, x, (pm.height() as f32 - a.height) / 2.0, pen.theme.fg);
        pen.fonts.draw(pm, &b, x + a.width + 8.0 * s, (pm.height() as f32 - b.height) / 2.0, pen.theme.muted);
    })
}

pub(in crate::ring) fn band(items: Vec<Node<Msg>>) -> Node<Msg> {
    let n = items.len() as f32;
    let gap = crate::toolkit::theme::sheet().ring_button_gap;
    strip_of(row(items).gap(gap), n * core() + (n - 1.0).max(0.0) * gap)
}

pub(super) fn strip_of(content: Node<Msg>, across: f32) -> Node<Msg> {
    let room = crate::toolkit::theme::sheet().ring_band_room;
    let size = (across + 2.0 * room, core() + 2.0 * room);
    let pill = drawing(size, "", move |pm, pen| {
        let r = crate::toolkit::widget::Rect { x: 0.0, y: 0.0, w: size.0, h: size.1 };
        fill_round(pm, r, pen.theme.well, crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Box, size.0.min(size.1) * pen.scale, pen.scale) / pen.scale, pen.scale);
    });
    layers(vec![pill, row(vec![content]).padding(room)]).width(Length::Fixed(size.0)).height(Length::Fixed(size.1))
}

impl Face {

    pub(super) fn backdrop(&self, bare: bool) -> Node<Msg> {
        let lay = self.lay.clone();
        let (seg, listening, pulse, all, dim) = (self.segment_lit(), self.listening, self.pulse, self.level == ALL, self.dim);

        let top = self.tray.battery().map(|_| self.top_hover);
        let empty = lay.placed.is_empty() && self.level == FAVOURITES;
        let scene = Scene { seg, top, bare, listening, pulse, empty, all, dim };
        drawing((self.size.0 as f32, self.size.1 as f32), "", move |pm, pen| {
            paint_ring(pm, pen, &lay, &scene);
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .on_pointer(|p| match p {
            Pointer::Up(x, y) => msg(move |f| f.background_click(x, y)),
            _ => Msg::idle(),
        })
    }

    pub(super) fn background_click(&mut self, x: f32, y: f32) {
        if self.listening {
            return self.stop_listening();
        }
        let (cx, cy) = (self.size.0 / 2.0, self.size.1 / 2.0);
        let d = ((x as f64 - cx).powi(2) + (y as f64 - cy).powi(2)).sqrt();
        if !self.quick.is_open() && !self.power && crate::voice::available() && d <= CENTRE / 2.0 {

            if kept::own_press() {
                self.listen(crate::voice::MAX_SECONDS as u32);
            }
            return;
        }
        if self.win_menu.is_some() {
            self.win_menu = None;
        } else if self.menu.is_some() || self.beside.is_some() {
            self.close_menu();
            self.close_beside();
        } else if self.quick.is_open() {
            self.quick.hide();
        } else if d > self.lay.outer_radius {
            self.close();
        }
    }

    fn segment_lit(&self) -> bool {
        self.level == ALL || self.segment_hover
    }

    pub(super) fn top_segment(&self, cx: f32, cy: f32) -> Option<Node<Msg>> {
        let (icon, tip, text) = self.tray.battery()?;
        let m = CENTRE / 2.0;
        let half = (m * m - (m - SEGMENT).powi(2)).sqrt() as f32;
        let top = cy - m as f32;
        let lit = self.top_hover;
        let node = drawing((2.0 * half, SEGMENT as f32), tip.clone(), move |canvas, pen| {
            let ink = if lit { pen.theme.on_accent } else { pen.theme.fg };
            let s = pen.scale;
            let size = (ALL_ICON * s).round();
            let words = text.as_deref().map(|t| pen.fonts.layout(t, &Style::new(BATTERY_TEXT * s), None));
            let gap = (4.0 * s).round();
            let w = size + words.as_ref().map_or(0.0, |l| gap + l.width);
            let x = ((canvas.width() as f32 - w) / 2.0).round();
            let y = ((canvas.height() as f32 - size) / 2.0).round();
            pen.icons.draw(canvas, &icon, x as i32, y as i32, size as u32, 1, ink);
            if let Some(l) = words {
                pen.fonts.draw(canvas, &l, x + size + gap, (canvas.height() as f32 - l.height) / 2.0, ink);
            }
        })
        .spoken(tip)
        .pressable(msg(|f| f.open_quick("power")))
        .on_hover()
        .on_pointer(|p| match p {
            Pointer::Up(_, _) => msg(|f| f.open_quick("power")),
            Pointer::Hover(_, _) => Msg::unless(|f| f.top_hover = true, |f| f.top_hover),
            Pointer::Leave => Msg::unless(|f| f.top_hover = false, |f| !f.top_hover),
            _ => Msg::idle(),
        });
        Some(placed(node, move |_, _| (cx - half, top)))
    }

    pub(super) fn segment(&self, cx: f32, cy: f32) -> Node<Msg> {
        let m = CENTRE / 2.0;
        let half = (m * m - (m - SEGMENT).powi(2)).sqrt() as f32;
        let top = cy + m as f32 - SEGMENT as f32;

        let lit = self.segment_lit();
        let node = drawing((2.0 * half, SEGMENT as f32), t!("ring-all-apps"), move |canvas, pen| {
            let size = (ALL_ICON * pen.scale).round();
            let (x, y) = ((canvas.width() as f32 - size) / 2.0, (canvas.height() as f32 - size) / 2.0);
            pen.icons.draw(canvas, "view-app-grid-symbolic", x.round() as i32, y.round() as i32, size as u32, 1,
                           if lit { pen.theme.on_accent } else { pen.theme.fg });
        })
            .spoken(t!("ring-all-apps"))
            .pressable(msg(Face::toggle_level))
            .on_hover()
            .on_keys(|i| match i {
                Input::Key { sym: Keysym::Return | Keysym::space, .. } => Some(msg(Face::toggle_level)),
                _ => None,
            })
            .on_pointer(|p| match p {
                Pointer::Up(_, _) => msg(Face::toggle_level),

                Pointer::Hover(_, _) => Msg::unless(|f| f.segment_hover = true, |f| f.segment_hover),
                Pointer::Leave => Msg::unless(|f| f.segment_hover = false, |f| !f.segment_hover),
                _ => Msg::idle(),
            });
        placed(node, move |_, _| (cx - half, top))
    }

    fn tool(&self, key: &'static str, icon: &str, tip: &str, job: Job) -> Node<Msg> {
        button(icon, tip).size(core()).tinted(job.share()).on_press(msg(move |f| f.use_tool(key)))
    }

    pub(super) fn centre(&self) -> Node<Msg> {
        let mut upper: Vec<Node<Msg>> = tool_rows()[0].iter().map(|(k, i, t)| self.tool(k, i, t, Job::Programs)).collect();

        upper.extend(self.updates.button(core()).map(|u| u.tinted(Job::Programs.share())));
        let power = button("system-shutdown-symbolic", t!("ring-power")).size(core()).tinted(Job::Computer.share()).spoken(t!("ring-power-spoken")).on_press(msg(|f| f.show_power(true)));
        let status: Vec<Node<Msg>> = std::iter::once(power).chain(self.tray.buttons(core())).collect();
        let mut lower: Vec<Node<Msg>> = lower_tools().into_iter().map(|(k, i, t)| self.tool(k, i, t, Job::Tools)).collect();
        lower.extend(self.tray.keyboard_button(core()));

        let foot = sni::tray().map(|t| t.strip()).filter(|(_, w)| *w > 0.0).map(|(strip, width)| strip_of(strip, width));
        let clock = if self.level == ALL && !self.typed.is_empty() {
            self.finding()
        } else {
            self.clock()
        };
        centre_of(upper, status, clock, lower, foot)
    }

    fn finding(&self) -> Node<Msg> {
        let typed = self.typed.clone();
        drawing(FINDING, t!("ring-finding", typed = &typed), move |pm, pen| {
            let s = pen.scale;
            let mut big = Style::new(CLOCK_SIZE * s);
            big.weight = 600;
            let a = pen.fonts.layout(&typed, &big, None);
            let caret = (2.0 * s).round().max(1.0);
            let gap = 2.0 * s;
            let room = pm.width() as f32 - caret - gap;
            let x = if a.width <= room { (room - a.width) / 2.0 } else { room - a.width };
            let y = (pm.height() as f32 - a.height) / 2.0;
            pen.fonts.draw(pm, &a, x, y, pen.theme.fg);
            let bar = crate::toolkit::widget::Rect { x: (x + a.width + gap).round() / s, y: y / s, w: caret / s, h: a.height / s };
            fill_round(pm, bar, pen.theme.accent, crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Box, bar.w.min(bar.h) * s, s) / s, s);
        })
    }

    fn clock(&self) -> Node<Msg> {
        let (time, date) = self.clock.clone();
        time_and_date_drawn(time, date)
        .spoken(t!("ring-date-time"))
        .pressable(msg(|f| f.settings(Some("datetime"))))
        .on_keys(|i| match i {
            Input::Key { sym: Keysym::Return | Keysym::space, .. } => Some(msg(|f| f.settings(Some("datetime")))),
            _ => None,
        })
        .on_hover()
        .on_pointer(|p| match p {
            Pointer::Up(_, _) => msg(|f| f.settings(Some("datetime"))),
            _ => Msg::idle(),
        })
    }
}

pub(super) fn tray_cover() -> Node<Msg> {
    let close = button("window-close-symbolic", t!("ring-close")).size(core()).tinted(Job::Tray.share()).on_press(msg(|f| f.quick.hide()));
    let gap = crate::toolkit::theme::sheet().ring_band_gap;
    quick::on_disc(column(vec![quick::tray_rows(), band(vec![close])]).gap(gap).centred(), Some(CENTRE as f32))
}
