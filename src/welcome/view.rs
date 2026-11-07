//! What each page of the tour looks like.

use crate::toolkit::{t, theme};
use crate::toolkit::tiles::{empty_state, standard_ring_with, tile_head};
use crate::toolkit::widget::{button, card, choice, column, drawing, form_label, label, lead, Length, main_action, Node, note, row, space, Tone};

use super::{Install, MainAct, Msg, pictures, Welcome};
use super::pages::{admin_cards, files_cards, in_the_ring, Kinds, know_cards, Level, name_of, named, Page, reports_question, snapshot_cards, software_cards, words_cards};

pub(super) const PART_GAP: f32 = 16.0;

const FOOT: f32 = 34.0;

const ROW_GAP: f32 = 8.0;

const DOT: f32 = 24.0;

const BUTTON: f32 = 34.0;

const NAME_LEAST: f32 = 118.0;

const PLUGIN_ICON: f32 = 32.0;

pub(super) fn no_plugins_yet() -> &'static str {
    t!("welcome-plugins-none")
}

pub(super) fn panel(icon: &str, name: &str, body: Vec<Node<Msg>>) -> Node<Msg> {
    let mut all = vec![tile_head(icon, name, Vec::new())];
    all.extend(body);
    card(column(all).gap(theme::sheet().gap)).height(Length::Fill)
}

fn picture(words: String, paint: impl Fn(&mut tiny_skia::PixmapMut, &crate::toolkit::widget::Pen) + 'static) -> Node<Msg> {
    drawing((240.0, 120.0), words, paint).width(Length::Fill).height(Length::Fill)
}

fn banded(rows: Vec<Node<Msg>>, height: Length) -> Node<Msg> {
    let rows = rows.into_iter().enumerate().map(|(i, r)| {
        let r = if i % 2 == 1 { r.banded() } else { card(r).snug().flat() };
        r.height(height)
    }).collect();
    column(rows).gap(0.0).height(height)
}

fn dots_page() -> Node<Msg> {
    use crate::common::in_use::Kind;
    let one = |kind: Kind| {
        row(vec![crate::settings::ui::in_use_dot_sized(kind, DOT), label(kind.spoken(&[])).width(Length::Fill)]).gap(12.0)
    };
    let (left, right) = Kind::ALL.split_at(Kind::ALL.len().div_ceil(2));
    let side = |kinds: &[Kind]| {
        let mut rows: Vec<Node<Msg>> = kinds.iter().map(|k| one(*k)).collect();
        rows.extend((kinds.len()..left.len()).map(|_| row(Vec::new()).height(Length::Fixed(DOT))));
        column(rows).gap(12.0).width(Length::Fill)
    };
    let columns = row(vec![side(left), side(right)]).gap(PART_GAP * 2.0).width(Length::Fill);
    panel("dialog-information-symbolic", t!("common-in-use-key"), vec![
        columns,
        picture(pictures::dots_words().to_string(), pictures::dots_where),
    ])
}

fn tiles(cards: &[(&'static str, &'static str, &'static str)]) -> Node<Msg> {
    let tile = |&(icon, head, line): &(&'static str, &'static str, &'static str)| {
        panel(icon, head, vec![lead(line).width(Length::Fill)])
    };
    let gap = theme::bento().gap;
    let per = if cards.len() == 4 { 2 } else { cards.len() };
    let rows = cards.chunks(per).map(|c| row(c.iter().map(tile).collect()).gap(gap).height(Length::Fill)).collect();
    column(rows).gap(gap).height(Length::Fill)
}

fn words_page() -> Node<Msg> {
    let open = row(vec![note(t!("welcome-words-where")).width(Length::Fill),
                        button("preferences-desktop-locale-symbolic", t!("welcome-words-open")).on_press(Msg::Words)]).gap(12.0);
    column(vec![tiles(&words_cards()), open]).gap(theme::bento().gap).height(Length::Fill)
}

impl Welcome {
    fn clock(&self) -> impl Fn() -> Option<f32> + 'static {
        let (began, moving, held) = (self.began, self.moving, self.held);
        move || moving.then(|| held.unwrap_or_else(|| began.elapsed().as_secs_f32() % theme::welcome().run))
    }

    pub(super) fn tile(&self) -> f32 {
        let look = theme::welcome();
        let room = self.body_height() - 2.0 * theme::sheet().card_padding - theme::sheet().badge - theme::sheet().gap;
        (room / 5.0).floor().clamp(look.tile_least, look.tile)
    }

    fn body_height(&self) -> f32 {
        self.size.1 - 2.0 * theme::sheet().page_padding - theme::welcome().header - 2.0 * PART_GAP - FOOT
    }

    fn beside(&self, share: f32) -> f32 {
        let room = self.size.0 - 2.0 * theme::sheet().page_padding - theme::bento().gap;
        (room * share / 4.0).round() * 4.0
    }

    fn row_width(&self) -> f32 {
        let across = self.size.0 - 2.0 * theme::sheet().page_padding - 3.0 * theme::bento().gap;
        across / 4.0 - 2.0 * theme::sheet().card_padding - 2.0 * theme::sheet().snug_padding
    }

    pub(super) fn animating(&self) -> bool {
        (self.moving && self.here().moves()) || matches!(self.install, Install::Running(_, share, _) if share <= 0.0)
    }

    pub(super) fn view_page(&self) -> Node<Msg> {
        let page = self.here();
        match page {
            Page::Level => self.level_page(),
            Page::Files => tiles(&files_cards()),
            Page::Software => tiles(&software_cards()),
            Page::Admin => tiles(&admin_cards()),
            Page::Snapshots => tiles(&snapshot_cards()),
            Page::Words => words_page(),
            Page::Hello => self.hello(),
            Page::Ring => self.ring_page(),
            Page::Apps | Page::MoreApps => self.picker(page.kinds().unwrap_or_default()),
            Page::Plugins => self.plugins(),
            Page::Reports => self.reports_page(),
            Page::Dots => dots_page(),
            Page::Know => tiles(&know_cards()),
        }
    }

    fn level_page(&self) -> Node<Msg> {
        let cards = Level::ALL.into_iter().map(|l| {
            let (icon, name, line) = l.card();
            let pick = button("go-next-symbolic", name).on(self.level == l).on_press(Msg::Level(l));
            panel(icon, name, vec![lead(line).width(Length::Fill), row(vec![space().width(Length::Fill), pick])]).on_press(Msg::Level(l))
        }).collect();
        row(cards).gap(theme::bento().gap).height(Length::Fill)
    }

    fn pair(&self, shown: Node<Msg>, beside: Node<Msg>, share: f32) -> Node<Msg> {
        row(vec![shown.width(Length::Fill), beside.width(Length::Fixed(self.beside(share)))])
            .gap(theme::bento().gap).height(Length::Fill)
    }

    fn hello(&self) -> Node<Msg> {
        let shown = panel("trindle-ring-symbolic", t!("welcome-hello-ring"), vec![
            picture(pictures::hello_words().to_string(), pictures::hello)]);
        let rows = in_the_ring().into_iter().map(|(icon, name, line)| {
            row(vec![crate::settings::ui::icon(icon, 24.0),
                     column(vec![form_label(name), note(line)]).gap(4.0).width(Length::Fill)]).gap(12.0).padding(8.0)
        }).collect();
        let held = panel("view-app-grid-symbolic", t!("welcome-hello-held"), vec![banded(rows, Length::Fill)]);
        self.pair(shown, held, 0.38)
    }

    fn ring_page(&self) -> Node<Msg> {
        let (t, k) = (self.clock(), self.clock());
        let shown = panel("input-mouse-symbolic", t!("welcome-ring-pointer"), vec![
            picture(pictures::ring_words().to_string(), move |c, p| pictures::open_ring(c, p, t()))]);
        let keys = panel("input-keyboard-symbolic", t!("welcome-ring-keyboard"), vec![
            picture(pictures::super_words().to_string(), move |c, p| pictures::super_key(c, p, k())),
            note(t!("welcome-ring-super-note")),
        ]);
        self.pair(shown, keys, 0.34)
    }

    fn picker(&self, kinds: Kinds) -> Node<Msg> {
        let cards = kinds.iter().map(|(kind, icon, apps)| {
            let offered = self.offered(apps);
            let body = if offered.is_empty() {
                empty_state(t!("welcome-kind-empty"), None)
            } else {
                let rows = offered.into_iter().enumerate().map(|(i, (id, name))| self.app_row(id, name, i % 2 == 1)).collect();
                column(rows).gap(0.0)
            };
            panel(icon, kind, vec![body])
        }).collect();
        row(cards).gap(theme::bento().gap).height(Length::Fill)
    }

    fn app_row(&self, id: &str, name: &str, band: bool) -> Node<Msg> {
        let tile = self.tile();
        let installed = self.installed.contains(id);
        let going = matches!(&self.install, Install::Running(ids, ..) if ids.iter().any(|i| i == id));
        let picked = self.picked.contains(id);
        let icon_size = (tile - 12.0).min(32.0);
        let with_icon = self.row_width() - icon_size - BUTTON - 2.0 * ROW_GAP;
        let icon = (with_icon >= NAME_LEAST).then(|| self.app_icon(id, name, icon_size));
        let end = if installed {
            note(t!("welcome-app-installed"))
        } else if going {
            note(t!("welcome-app-installing"))
        } else {
            let b = button("list-add-symbolic", if picked { t!("welcome-app-picked", name = name) } else { t!("welcome-app-pick", name = name) })
                .on(picked);
            if self.installing() { b.off(true) } else { b.on_press(Msg::Pick(id.to_string())) }
        };
        let mut parts: Vec<Node<Msg>> = icon.into_iter().collect();
        parts.push(label(name).cut().width(Length::Fill));
        parts.push(end);
        let inner = row(parts).gap(ROW_GAP);
        let glows = matches!(&self.install, Install::Done(ids) if ids.iter().any(|i| i == id));
        let c = if glows {
            card(inner).snug().glow(self.glow)
        } else if band {
            inner.banded()
        } else {
            card(inner).snug().flat()
        };
        let c = c.height(Length::Fixed(tile));
        if installed || going || self.installing() {
            return c;
        }
        c.on_press(Msg::Pick(id.to_string()))
    }

    fn app_icon(&self, id: &str, name: &str, size: f32) -> Node<Msg> {
        let icon = self.icons.get(id).map(|p| p.to_string_lossy().into_owned());
        let name = name.to_string();
        drawing((size, size), "", move |canvas, pen| {
            let px = (size * pen.scale).round() as u32;
            let drawn = icon.as_ref().is_some_and(|i| pen.icons.draw(canvas, i, 0, 0, px, 1, pen.theme.fg).is_some());
            if !drawn {
                pictures::letter(canvas, pen, &name);
            }
        })
    }

    fn plugins(&self) -> Node<Msg> {
        let icons: Vec<String> = self.plugins.iter().filter_map(|o| o.icon.as_ref())
            .map(|i| i.to_string_lossy().into_owned()).collect();
        let shown = panel("trindle-ring-symbolic", t!("welcome-plugins-ring"), vec![
            picture(pictures::plugins_words().to_string(), move |c, p| pictures::plugins(c, p, &icons))]);
        let rows: Vec<Node<Msg>> = self.plugins.iter().map(|o| {
            let (file, name) = (o.icon.as_ref().map(|i| i.to_string_lossy().into_owned()), o.name.clone());
            let icon = drawing((PLUGIN_ICON, PLUGIN_ICON), "", move |canvas, pen| {
                let px = (PLUGIN_ICON * pen.scale).round() as u32;
                let drawn = file.as_ref().is_some_and(|f| pen.icons.draw(canvas, f, 0, 0, px, 1, pen.theme.fg).is_some());
                if !drawn {
                    pictures::letter(canvas, pen, &name);
                }
            });
            row(vec![icon, column(vec![form_label(o.name.clone()), note(o.line.clone())]).gap(4.0).width(Length::Fill)])
                .gap(12.0).padding(8.0)
        }).collect();
        let open = row(vec![note(t!("welcome-plugins-where")).width(Length::Fill),
                            button("view-app-grid-symbolic", t!("welcome-plugins-open")).on_press(Msg::Plugins)]).gap(12.0);
        let list = if rows.is_empty() {
            note(no_plugins_yet()).width(Length::Fill).height(Length::Fill)
        } else {
            banded(rows, Length::Fill)
        };
        let held = panel("applications-other-symbolic", t!("welcome-plugins-card"), vec![list, open]);
        self.pair(shown, held, 0.44)
    }

    fn reports_page(&self) -> Node<Msg> {
        let answer = choice(vec![t!("welcome-reports-no").to_string(), t!("welcome-reports-yes").to_string()], usize::from(self.reports),
                            t!("welcome-reports-choice"), |i| Msg::Reports(i == 1));
        let body = vec![
            lead(reports_question()).width(Length::Fill),
            note(t!("welcome-reports-note")),
            note(t!("welcome-reports-settings")),
            answer,
        ];
        panel("dialog-warning-symbolic", t!("welcome-reports-card"), body)
    }

    pub(super) fn foot(&self) -> Node<Msg> {
        let mut items = vec![button("window-close-symbolic", t!("welcome-close")).on_press(Msg::Close)];
        if self.page > 0 {
            items.push(button("go-previous-symbolic", t!("welcome-back")).on_press(Msg::Back));
        }
        items.push(self.status().width(Length::Fill));
        let words = self.main_words();
        let installing = self.main_act() == MainAct::Install;
        let icon = if installing { crate::settings::ui::INSTALL_ICON } else { "go-next-symbolic" };
        let blocked = installing && !self.online;
        items.push(main_action(icon, words).on_press(Msg::Main).off(blocked).key("main").first());
        row(items).gap(theme::sheet().gap).height(Length::Fixed(FOOT))
    }

    pub(super) fn status(&self) -> Node<Msg> {
        let names = |ids: &[String]| named(&ids.iter().map(|i| name_of(i)).collect::<Vec<_>>());
        match &self.install {
            Install::Running(ids, share, left) => {
                let words = match left {
                    Some(l) => t!("welcome-installing-left", names = names(ids), left = l),
                    None => t!("welcome-installing", names = names(ids)),
                };
                let known = (*share > 0.0).then_some(*share);
                let figure = known.map(|v| format!("{}%", (v * 100.0).round()));
                row(vec![standard_ring_with(known, figure, 34.0).width(Length::Shrink),
                         label(words).cut().width(Length::Fill)])
                    .gap(theme::sheet().gap)
            }
            Install::Done(ids) => label(t!("welcome-installed", n = ids.len(), names = names(ids))).cut(),
            Install::Failed(_, why) => row(vec![
                label(t!("welcome-failed", why = why)).tone(Tone::Problem).cut().width(Length::Fill),
                button("view-refresh-symbolic", t!("welcome-retry")).on_press(Msg::Retry).off(!self.online),
            ]).gap(theme::sheet().gap),
            Install::Idle if !self.online && self.here().kinds().is_some() => label(t!("welcome-offline")),
            Install::Idle if !self.picked.is_empty() && self.here().kinds().is_some() => {
                let ids: Vec<String> = self.picked.iter().cloned().collect();
                label(t!("welcome-chosen", names = names(&ids))).cut()
            }
            Install::Idle => space(),
        }
    }
}
