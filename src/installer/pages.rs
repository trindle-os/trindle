//! The installer's pages and the parts they are built from.

use super::{ABOUT_YOU, BESIDE_WIDTH, DISK_PASSWORD, DONE, Installer, MARK, MARKS, MARK_GAP, Msg, READY, RUNNING, SWAP_WIDTH, tour};
use super::plan::{self, Lock, Where};
use crate::common::terms;
use crate::toolkit::colour::Colour;
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::tiles::{standard_ring_with, tile_head};
use crate::toolkit::widget::{button, card, column, drawing, entry, form_label, heading, headline, icon_circle, item, label, lead,
                             list, main_action, note, page_title, row, scroll, slider, space, switch, Length, Node, Pen, Tone};
use tiny_skia::{Paint, PixmapMut, Transform};

pub(super) fn matching<'a>(items: &'a [(String, String)], find: &str) -> Vec<&'a (String, String)> {
    let find = find.trim().to_lowercase();
    items.iter().filter(|(_, n)| n.to_lowercase().contains(&find)).collect()
}

pub(super) fn half_way(part: &plan::Part) -> u64 {
    (((part.floor() + part.size) / 2).min(part.most_kept()) / plan::GIB) * plan::GIB
}

impl Installer {

    pub(super) fn heading(&self, words: String, help: &str) -> Node<Msg> {
        let mut items = vec![page_title(words)];
        if !help.is_empty() {
            items.push(headline(help));
        }
        column(items).gap(4.0)
    }

    pub(super) fn layout(&self, words: String, help: &str, items: Vec<Node<Msg>>) -> Node<Msg> {
        column(vec![self.heading(words, help), column(items).gap(theme::bento().gap).height(Length::Fill)]).gap(12.0)
    }

    fn radio(&self, words: &str, chosen: bool, pick: Option<Msg>) -> Node<Msg> {
        let icon = if chosen { "radio-checked-symbolic" } else { "radio-symbolic" };
        let mut b = button(icon, words).on(chosen);
        if let Some(m) = pick {
            b = b.on_press(m);
        }
        row(vec![b, label(words).width(Length::Fill)]).gap(12.0)
    }

    fn labelled(&self, name: &str, what: Node<Msg>) -> Node<Msg> {
        column(vec![form_label(name), what]).gap(4.0).width(Length::Fill)
    }

    pub(super) fn password(&self, text: &str, name: &str, key: &str, change: fn(String) -> Msg) -> Node<Msg> {
        entry(text, name, change).key(key).secret()
    }

    pub(super) fn field(&self, at: usize, icon: &str, name: &str, items: &[(String, String)], chosen: &str) -> Node<Msg> {
        let shown = matching(items, &self.find[at]);
        let place = shown.iter().position(|(k, _)| k == chosen);
        let picked = items.iter().find(|(k, _)| k == chosen).map_or(t!("installer-select"), |(_, n)| n.as_str());
        let rows = shown.iter().map(|(_, n)| item(n.as_str())).collect();
        let head = row(vec![
            icon_circle(icon),
            column(vec![heading(name), note(picked).cut()]).gap(2.0).width(Length::Fill),
        ]).gap(12.0);
        card(column(vec![
            head,
            entry(self.find[at].clone(), name, move |t| Msg::Find(at, t)).key(format!("find-{at}")),
            scroll(format!("scroll-{at}"), list(rows, place, move |i| Msg::Pick(at, i)).key(format!("list-{at}"))),
        ]).gap(8.0)).width(Length::Fill).height(Length::Fill)
    }

    fn start_page(&self) -> Node<Msg> {
        let m = &self.machine;
        let p = &self.plan;
        let keyboard = format!("{}\t{}", p.layout, p.variant);
        let keyboard = if p.layout.is_empty() { "" } else { keyboard.as_str() };
        column(vec![
            self.heading(t!("installer-welcome-title", name = &self.name), t!("installer-welcome-help")),
            row(vec![
                self.field(0, "preferences-desktop-locale-symbolic", t!("installer-language"), &m.locales, &p.locale),
                self.field(1, "input-keyboard-symbolic", t!("installer-keyboard"), &m.keyboards, keyboard),
                self.field(2, "preferences-system-time-symbolic", t!("installer-time"), &m.zones, &p.timezone),
            ]).gap(theme::bento().gap).height(Length::Fill),
        ]).gap(12.0)
    }

    fn disks_page(&self) -> Node<Msg> {
        let head = self.heading(t!("installer-disks-title", name = &self.name), t!("installer-disks-help"));
        if self.machine.disks.is_empty() {
            return column(vec![head, note(t!("installer-disks-none"))]).gap(12.0);
        }
        let mut cards = Vec::new();
        for disk in &self.machine.disks {
            let erasing = matches!(&self.plan.target, Some(Where::Erase { disk: d }) if *d == disk.dev);
            let mut items = vec![
                tile_head("drive-harddisk-symbolic", disk.what(), Vec::new()),
                note(disk.holds()),
                self.radio(t!("installer-disks-whole"), erasing, Some(Msg::Erase(disk.dev.clone()))),
            ];
            for part in disk.parts.iter().filter(|p| p.shrinkable()) {
                let sharing = matches!(&self.plan.target, Some(Where::Beside { part: d, .. }) if *d == part.dev);
                let words = t!("installer-disks-share", what = part.what());
                let leave = self.leave.get(&part.dev).copied().unwrap_or_else(|| half_way(part));
                let dev = part.dev.clone();
                items.push(column(vec![
                    self.radio(&words, sharing, Some(Msg::Beside(part.dev.clone()))),
                    slider(leave as f32, part.floor() as f32, part.most_kept() as f32, (plan::GIB / 2) as f32, words.as_str(),
                           move |v| Msg::Leave(dev.clone(), v)).key(format!("leave-{}", part.dev)),
                    note(t!("installer-disks-split", kept = plan::size(leave), mine = plan::size(part.size.saturating_sub(leave)))),
                ]).gap(8.0));
            }
            cards.push(card(column(items).gap(12.0)));
        }
        let gap = theme::bento().gap;
        let swap = column(vec![self.swap_card(), space()]).width(Length::Fixed(SWAP_WIDTH)).height(Length::Fill);
        column(vec![head, row(vec![scroll("disks", column(cards).gap(gap)), swap]).gap(gap).height(Length::Fill)]).gap(12.0)
    }

    fn swap_card(&self) -> Node<Msg> {
        let words = if self.machine.secure_boot {
            t!("installer-swap-secure-boot")
        } else {
            t!("installer-swap-note")
        };
        card(column(vec![
            row(vec![tile_head("system-hibernate-symbolic", t!("installer-swap-title", gb = self.machine.memory), Vec::new()),
                     switch(self.plan.swap, t!("installer-swap-switch"), Msg::Swap).key("swap")]).gap(12.0),
            note(words),
        ]).gap(12.0))
    }

    pub(super) fn pair(&self, main: Node<Msg>, beside: Node<Msg>) -> Node<Msg> {
        row(vec![main.height(Length::Fill), beside.width(Length::Fixed(BESIDE_WIDTH)).height(Length::Fill)])
            .gap(theme::bento().gap).height(Length::Fill)
    }

    pub(super) fn privacy(&self) -> Node<Msg> {
        let chip = self.machine.chip;
        let lock = self.plan.lock;
        let words = if chip {
            t!("installer-privacy-chip")
        } else {
            t!("installer-privacy-no-chip")
        };
        let choices = card(column(vec![
            tile_head("system-lock-screen-symbolic", t!("installer-privacy-locked"), Vec::new()),
            self.radio(t!("installer-privacy-none"), lock == Lock::None, Some(Msg::Lock(Lock::None))),
            self.radio(t!("installer-privacy-password"), lock == Lock::Password,
                       Some(Msg::Lock(Lock::Password))),
            self.radio(t!("installer-privacy-chip-choice"), lock == Lock::Chip,
                       chip.then_some(Msg::Lock(Lock::Chip))),
        ]).gap(12.0));
        let about = card(column(vec![
            tile_head("dialog-information-symbolic", t!("installer-privacy-about"), Vec::new()),
            note(words),
        ]).gap(12.0));
        self.layout(t!("installer-privacy-title").into(),
                    t!("installer-privacy-help"),
                    vec![self.pair(choices, about)])
    }

    pub(super) fn disk_password(&self) -> Node<Msg> {
        let suggest = button("media-playlist-shuffle-symbolic", t!("installer-disk-suggest")).on_press(Msg::Suggest);
        let mut items = vec![
            tile_head("dialog-password-symbolic", t!("installer-disk-password"), vec![suggest]),
            self.labelled(t!("installer-disk-password"), self.password(&self.disk_one, t!("installer-disk-password"), "disk-one", Msg::DiskOne)
                .shown(self.suggested)),
            self.labelled(t!("installer-same-again"), self.password(&self.disk_two, t!("installer-same-again"), "disk-two", Msg::DiskTwo)
                .shown(self.suggested)),
        ];
        if plan::mistyped(&self.disk_one, &self.disk_two) {
            items.push(label(t!("installer-mismatch")).tone(Tone::Problem));
        }
        let keep = card(column(vec![
            tile_head("dialog-warning-symbolic", t!("installer-disk-write-down"), Vec::new()),
            label(t!("installer-disk-write-down-text")).width(Length::Fill),
        ]).gap(12.0));
        self.layout(t!("installer-disk-title").into(), t!("installer-disk-help"), vec![
            self.pair(card(column(items).gap(8.0)), keep),
        ])
    }

    fn about_you(&self) -> Node<Msg> {
        let (score, guess) = plan::password_strength(&self.one);
        let accent = self.accent;
        let meter = drawing((MARKS as f32 * MARK.0 + (MARKS - 1) as f32 * MARK_GAP, theme_entry()),
                            t!("installer-strength"), move |canvas, pen| marks(canvas, pen, score as usize, accent));
        let account_fault = Some(self.user.as_str()).filter(|n| !n.is_empty()).and_then(plan::username_fault)
            .or_else(|| Some(self.host.as_str()).filter(|n| !n.is_empty()).and_then(plan::hostname_fault));
        let mut account = vec![
            tile_head("avatar-default-symbolic", t!("installer-account"), Vec::new()),
            self.labelled(t!("installer-your-name"), entry(self.full.clone(), t!("installer-your-name"), Msg::Full).key("full")),
            self.labelled(t!("installer-user-name"), entry(self.user.clone(), t!("installer-user-name"), Msg::User).key("user")),
            self.labelled(t!("installer-host-name"), entry(self.host.clone(), t!("installer-host-name"), Msg::Host).key("host")),
        ];
        if let Some(f) = account_fault {
            account.push(label(f).tone(Tone::Problem));
        }
        let mut secret = vec![
            tile_head("dialog-password-symbolic", t!("installer-your-password"), Vec::new()),
            self.labelled(t!("installer-password"), self.password(&self.one, t!("installer-password"), "one", Msg::One)),
            self.labelled(t!("installer-same-again"), self.password(&self.two, t!("installer-same-again"), "two", Msg::Two)),
            row(vec![form_label(t!("installer-strength")), meter]).gap(12.0),
        ];
        if !guess.is_empty() {
            secret.push(note(guess));
        }
        if plan::mistyped(&self.one, &self.two) {
            secret.push(label(t!("installer-mismatch")).tone(Tone::Problem));
        }
        let gap = theme::bento().gap;
        self.layout(t!("installer-you-title").into(), t!("installer-you-help"), vec![
            row(vec![card(column(account).gap(8.0)).height(Length::Fill), card(column(secret).gap(8.0)).height(Length::Fill)])
                .gap(gap).height(Length::Fill),
        ])
    }

    pub(super) fn ready(&self) -> Node<Msg> {
        let p = &self.plan;
        let name = &self.name;
        let (words, tone) = ready_sentence(&p.target);
        let target = match &p.target {
            Some(Where::Erase { disk }) => (t!("installer-ready-whole", disk = disk), t!("installer-ready-whole-text", name = name)),
            Some(Where::Beside { part, leave }) => (t!("installer-ready-beside", part = part),
                t!("installer-ready-beside-text", part = part, size = plan::size(*leave), name = name)),
            None => (t!("installer-ready-no-disk").into(), t!("installer-ready-no-disk-text").into()),
        };
        let lock = match p.lock {
            Lock::None => (t!("installer-ready-plain").to_string(), t!("installer-ready-plain-text").to_string()),
            Lock::Password => (t!("installer-ready-locked").into(), t!("installer-ready-locked-password").into()),
            Lock::Chip => (t!("installer-ready-locked").into(), t!("installer-ready-locked-chip").into()),
        };
        let language = (plan::language_name(&p.locale), t!("installer-ready-language", keyboard = &p.layout, zone = &p.timezone));
        let security = t!("installer-ready-security-text", name = name);
        let facts = [
            ("drive-harddisk-symbolic", target),
            ("system-lock-screen-symbolic", lock),
            ("preferences-desktop-locale-symbolic", language),
            ("software-update-available-symbolic", (t!("installer-ready-security").to_string(), security)),
        ];
        let rows: Vec<Node<Msg>> = facts.into_iter().enumerate().map(|(i, (icon, (figure, words)))| {
            let r = row(vec![crate::settings::ui::icon(icon, 24.0),
                             column(vec![form_label(figure), if i < 3 { note(words).cut() } else { note(words) }]).gap(4.0).width(Length::Fill)])
                .gap(12.0).padding(1.0);
            (if i % 2 == 1 { r.banded() } else { card(r).snug().flat() }).height(Length::Fill)
        }).collect();
        let tone = if tone == Tone::Calm { Tone::Plain } else { tone };
        let head = column(vec![page_title(t!("installer-ready-title")), headline(words).tone(tone)]).gap(4.0);
        let facts = card(column(vec![
            tile_head("dialog-information-symbolic", t!("installer-ready-nothing-changed"), Vec::new()),
            column(rows).gap(0.0).height(Length::Fill),
        ]).gap(12.0));
        let main = if self.reading_terms {
            card(column(vec![
                button("go-previous-symbolic", t!("installer-back")).on_press(Msg::ReadTerms(false)),
                terms::reading(),
            ]).gap(12.0))
        } else {
            facts
        };
        column(vec![head, main.height(Length::Fill), terms::tick_row(self.terms, Msg::Terms, Msg::ReadTerms(true))]).gap(8.0)
    }

    pub(super) fn installing(&self) -> Node<Msg> {
        let (slide, cache) = (self.slide, self.cache.clone());
        let tour = drawing((400.0, 120.0), tour::title(slide), move |canvas, pen| tour::paint(canvas, pen, slide, &cache))
            .width(Length::Fill).height(Length::Fill);
        column(vec![
            self.heading(t!("installer-installing-title", name = &self.name), tour::title(slide)),
            card(column(vec![tile_head("help-about-symbolic", t!("installer-tour"), Vec::new()), tour]).gap(12.0))
                .height(Length::Fill),
        ]).gap(12.0)
    }

    pub(super) fn done(&self) -> Node<Msg> {
        let mut items = vec![];
        if let Some(words) = keep_words(self.plan.lock) {
            items.push(card(column(vec![
                tile_head("dialog-password-symbolic", t!("installer-done-keep"), Vec::new()),
                lead(words).width(Length::Fill),
            ]).gap(12.0)).height(Length::Fill));
        }
        if !self.problems.is_empty() {
            let mut said = vec![tile_head("dialog-warning-symbolic", t!("installer-done-problems"), Vec::new())];
            said.extend(self.problems.iter().map(|p| lead(p.as_str()).tone(Tone::Problem).width(Length::Fill)));
            items.push(card(column(said).gap(12.0)).height(Length::Fill));
        }
        self.layout(t!("installer-done-title", name = &self.name), t!("installer-done-help"), items)
    }

    pub(super) fn body(&self) -> Node<Msg> {
        match self.page {
            0 => self.start_page(),
            1 => self.disks_page(),
            2 => self.privacy(),
            DISK_PASSWORD => self.disk_password(),
            READY => self.ready(),
            ABOUT_YOU => self.about_you(),
            RUNNING => self.installing(),
            _ => self.done(),
        }
    }

    pub(super) fn foot(&self) -> Node<Msg> {
        let n = self.page;
        let missing = self.missing();
        let mut items = Vec::new();
        if (n > 0 && n <= READY) || (self.failed && !self.stuck) {
            items.push(button("go-previous-symbolic", t!("installer-back")).on_press(Msg::Back));
        }
        if n == 0 {
            items.push(label(t!("installer-try-keyboard")));
            items.push(entry(self.tried.clone(), t!("installer-try-keyboard-box"), Msg::Try).key("try")
                .width(Length::Fixed(280.0)));
        }
        if n == RUNNING || n == ABOUT_YOU && (self.helper.is_some() || self.failed) {
            if self.failed {
                items.push(label(self.doing.clone()).tone(Tone::Problem).cut().width(Length::Fill));
            } else {
                let figure = Some(format!("{}%", (self.share * 100.0).round()));
                items.push(standard_ring_with(Some(self.share), figure, 34.0).width(Length::Shrink));
                items.push(column(vec![note(t!("installer-started")), label(self.doing.clone()).cut()]).gap(2.0).width(Length::Fill));
            }
        }
        items.push(space());
        if self.armed {
            items.push(note(self.arm_words()).tone(if self.erases() { Tone::Problem } else { Tone::Warn }));
        } else if let Some(m) = missing {
            items.push(note(m));
        }
        if self.helper.is_some() || self.stopping {
            let stop = button("process-stop-symbolic", t!("installer-halt"));
            let stop = stop.key("installer:stop");
            items.push(if self.stopping { stop.off(true) } else { stop.on_press(Msg::Stop) });
        }
        if n == DONE || self.stuck {
            items.push(main_action("system-reboot-symbolic", t!("installer-restart")).on_press(Msg::Restart).key("installer:restart"));
        } else if n != RUNNING && n != READY {
            items.push(main_action("go-next-symbolic", t!("installer-next")).on_press(Msg::Next).key("installer:next").off(missing.is_some() || self.failed));
        }
        if n == READY {
            let word = if self.armed { t!("installer-sure") } else { t!("installer-install") };
            let install = main_action("system-software-install-symbolic", word).on_press(Msg::Install).key("installer:install").off(!self.terms);
            items.push(if self.erases() { install.cannot_be_undone() } else { install });
        }
        row(vec![space().width(Length::Fixed(8.0)), row(items).gap(theme::sheet().gap).width(Length::Fill),
                 space().width(Length::Fixed(8.0))]).gap(0.0).padding(16.0)
    }

}

pub(super) fn marks(canvas: &mut PixmapMut, pen: &Pen, score: usize, accent: Colour) {
    let s = pen.scale;
    let y = (canvas.height() as f32 - MARK.1 * s) / 2.0;
    for i in 0..MARKS {
        let x = i as f32 * (MARK.0 + MARK_GAP) * s;
        let r = crate::toolkit::widget::Rect { x, y, w: MARK.0 * s, h: MARK.1 * s };
        let Some(path) = crate::toolkit::ui::rounded(r, crate::toolkit::theme::corner_px(crate::toolkit::theme::Part::Badge, r.w.min(r.h), 1.0)) else { continue };
        let mut paint = Paint { anti_alias: true, ..Paint::default() };
        paint.set_color((if i < score { accent } else { pen.theme.bg }).to_skia());
        canvas.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);
    }
}

fn theme_entry() -> f32 {
    crate::toolkit::theme::sheet().entry_height
}

pub(super) fn ready_sentence(target: &Option<Where>) -> (String, Tone) {
    match target {
        Some(Where::Erase { disk }) => (t!("installer-ready-replaced", disk = disk), Tone::Warn),
        Some(Where::Beside { part, .. }) => (t!("installer-ready-kept", part = part), Tone::Warn),
        None => (t!("installer-ready-none").into(), Tone::Problem),
    }
}

pub(super) fn keep_words(lock: Lock) -> Option<&'static str> {
    match lock {
        Lock::None => None,
        Lock::Chip => Some(t!("installer-keep-chip")),
        Lock::Password => Some(t!("installer-keep-password")),
    }
}
