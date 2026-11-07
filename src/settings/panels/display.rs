//! Display settings, asked of the desktop directly (common::outputs).

use std::time::Duration;

use crate::common::files::{set_setting, setting};
use crate::common::outputs::{self, Head, Resolution, Setting, Turn};
use crate::common::run::on_thread;
use crate::session::{nightlight, scale};
use crate::settings::ui::{dropdown, group, headline, help_link, line, on, page, FootLook, Kit, Msg, Page, Trial, not_saved};
use crate::toolkit::theme::{alpha, mix, Palette};
use crate::toolkit::tiles::{picture, sketch, Sketch};
use crate::toolkit::widget::{column, note, row, slider, switch, Length, Node};
use crate::toolkit::t;

const SIZES: &[f64] = &[0.8, 0.9, 1.0, 1.1, 1.25, 1.5, 1.75, 2.0];
fn turns() -> [(Turn, &'static str); 4] {
    [
        (Turn::Normal, t!("settings-display-turn-normal")),
        (Turn::R90, t!("settings-display-turn-right")),
        (Turn::R270, t!("settings-display-turn-left")),
        (Turn::R180, t!("settings-display-turn-upside")),
    ]
}
fn places() -> [(&'static str, &'static str); 4] {
    [
        ("right", t!("settings-display-place-right")),
        ("left", t!("settings-display-place-left")),
        ("above", t!("settings-display-place-above")),
        ("below", t!("settings-display-place-below")),
    ]
}
const REVERT_AFTER: i32 = 15;
const WARMTH_SETTLE_MS: u64 = 400;
const REFUSED: &str = "display:refused";

fn base_scale(out: &Head) -> f64 {
    scale::choose(out).unwrap_or(1.0)
}

fn mode_label(m: &Resolution) -> String {
    if m.refresh > 0 {
        t!("settings-display-mode-hz", width = m.width, height = m.height, hz = format!("{:.0}", m.refresh as f64 / 1000.0))
    } else {
        t!("settings-display-mode", width = m.width, height = m.height)
    }
}

fn out_scale(out: &Head) -> f64 {
    if out.scale > 0.0 { out.scale } else { 1.0 }
}

fn settings_for(outputs: &[Head]) -> Vec<Setting> {
    outputs.iter().map(Head::setting).collect()
}

fn guess_place(out: &Head, anchor: &Head) -> &'static str {
    let ((ox, oy), (ax, ay)) = (out.position, anchor.position);
    let (dx, dy) = (ox - ax, oy - ay);
    if dx.abs() >= dy.abs() {
        if dx >= 0 { "right" } else { "left" }
    } else if dy >= 0 {
        "below"
    } else {
        "above"
    }
}

enum Change {
    Enabled(bool),
    Mode(Resolution),
    Scale(f64),
    Turn(Turn),
}

fn facts_of(out: &Head) -> Option<(String, Option<String>, String)> {
    let m = out.current_mode()?;
    let hz = m.hz().map(|h| t!("settings-display-hz", hz = format!("{h:.0}")));
    let size = format!("{:.0}%", out_scale(out) / base_scale(out) * 100.0);
    Some((format!("{} \u{00d7} {}", m.width, m.height), hz, size))
}

fn sum_up(outputs: &[Head], night: &Night) -> String {
    let on = outputs.iter().filter(|o| o.enabled).count();
    let used = match on {
        0 => t!("settings-display-used-none").to_string(),
        1 => t!("settings-display-used-one").to_string(),
        n => t!("settings-display-used-many", n = n),
    };
    let warm = if !night.on {
        t!("settings-display-warm-off").to_string()
    } else if night.when == nightlight::BY_TIMES || night.place.is_none() {
        t!("settings-display-warm-times", from = &night.from, to = &night.to)
    } else {
        t!("settings-display-warm-sun").to_string()
    };
    t!("settings-display-sum-up", used = used, warm = warm)
}

struct Night {
    on: bool,
    when: String,
    from: String,
    to: String,
    warmth: u32,
    place: Option<(f64, f64)>,
    zone: String,
}

impl Night {
    fn read() -> Night {
        let when = if setting(nightlight::WHEN) == nightlight::BY_TIMES { nightlight::BY_TIMES } else { nightlight::BY_SUN };
        Night {
            on: setting(nightlight::ON) == "on",
            when: when.to_string(),
            from: nightlight::from_time(&setting(nightlight::FROM)),
            to: nightlight::to_time(&setting(nightlight::TO)),
            warmth: nightlight::warmth(&setting(nightlight::WARMTH)),
            place: nightlight::place(),
            zone: nightlight::zone(),
        }
    }
}

#[derive(Default)]
pub struct Display {
    loaded: bool,
    outputs: Vec<Head>,
    err: String,
    before: Option<Vec<Setting>>,
    trial: Trial,
    trials: u32,
    refused: Option<String>,
    night: Option<Night>,
    nvidia: bool,
    warming: u32,
}

impl Display {
    fn load(&mut self, kit: &mut Kit) {
        on_thread(outputs::heads, kit.then::<Display, _>(|me, _, got| me.got(got)));
    }

    fn got(&mut self, got: Result<Vec<Head>, String>) {
        self.loaded = true;
        match got {
            Ok(o) => {
                self.outputs = o;
                self.err.clear();
            }
            Err(e) => {
                self.outputs.clear();
                self.err = e;
            }
        }
    }

    fn output_card(&self, kit: &Kit, out: &Head) -> Node<Msg> {
        let oname = out.name.clone();
        let title = if out.description.is_empty() { oname.clone() } else { out.description.clone() };
        let heading = if title != oname { format!("{oname}  {title}") } else { title };
        let in_use = self.outputs.iter().filter(|o| o.enabled).count();
        let only = out.enabled && in_use == 1;
        let card = group(heading).icon("video-display-symbolic")
            .about(t!("settings-display-card-about"));
        let off = !out.enabled;
        card.rows(vec![
            use_line(kit, out, only),
            resolution_line(kit, out, off),
            size_line(kit, out, off),
            turn_line(kit, out, off),
        ]).node(kit)
    }

    fn place_card(&self, kit: &Kit) -> Option<Node<Msg>> {
        let in_use = self.outputs.iter().filter(|o| o.enabled).count();
        let mut rows = Vec::new();
        for (index, out) in self.outputs.iter().enumerate() {
            if index == 0 {
                continue;
            }
            let others = || self.outputs.iter().enumerate().filter(move |(j, _)| *j != index).map(|(_, o)| o);
            let Some(anchor) = others().find(|o| o.enabled).or_else(|| others().next()) else {
                continue;
            };
            let usable = out.enabled && anchor.enabled;
            let place_words: Vec<String> = places().iter().map(|(_, l)| l.to_string()).collect();
            let guess = guess_place(out, anchor);
            let chosen = places().iter().position(|(v, _)| *v == guess);
            let (o, a) = (out.clone(), anchor.clone());
            let pick = dropdown(kit, &format!("display:{}:place", out.name), place_words, chosen, &t!("settings-display-place-name", display = &out.name), move |i| {
                let (o, a, v) = (o.clone(), a.clone(), places()[i].0);
                on::<Display>(move |me, k| me.place(k, &o, &a, v))
            });
            let heading = if self.outputs.len() == 2 { t!("settings-display-place-it").to_string() } else { t!("settings-display-place-name", display = &out.name) };
            rows.push(line(heading).sub(t!("settings-display-relative", display = &out.name, other = &anchor.name)).control(pick).node(kit).off(!usable));
        }
        if rows.is_empty() {
            return None;
        }
        Some(group(t!("settings-display-arrangement-title")).icon("view-dual-symbolic")
            .about(t!("settings-display-arrangement-about"))
            .waits_for((in_use < 2).then_some(t!("settings-display-arrangement-waits")))
            .rows(rows)
            .node(kit))
    }

    fn change(&mut self, kit: &mut Kit, name: &str, what: Change) {
        let s = match what {
            Change::Enabled(true) => Setting::on(name),
            Change::Enabled(false) => Setting::off(name),
            Change::Mode(m) => Setting { mode: Some(m), ..Setting::on(name) },
            Change::Scale(s) => Setting { scale: Some(s), ..Setting::on(name) },
            Change::Turn(t) => Setting { turn: Some(t), ..Setting::on(name) },
        };
        self.apply(kit, vec![s]);
    }

    fn place(&mut self, kit: &mut Kit, out: &Head, anchor: &Head, where_: &str) {
        let (aw, ah) = anchor.logical_size();
        let (ow, oh) = out.logical_size();
        let (ax, ay) = anchor.position;
        let (x, y) = match where_ {
            "right" => (ax + aw, ay),
            "left" => (ax - ow, ay),
            "above" => (ax, ay - oh),
            _ => (ax, ay + ah),
        };
        self.apply(kit, vec![Setting { position: Some((x, y)), ..Setting::on(&out.name) }]);
    }

    fn apply(&mut self, kit: &mut Kit, settings: Vec<Setting>) {
        if self.before.is_none() {
            self.before = Some(settings_for(&self.outputs));
        }
        self.refused = None;
        kit.hush(REFUSED);
        on_thread(move || outputs::apply(&settings), kit.then::<Display, _>(|me, k, done| me.applied(k, done)));
    }

    fn applied(&mut self, kit: &mut Kit, done: Result<(), String>) {
        if let Err(err) = done {
            self.load(kit);
            let e = err.trim();
            let words = if e.is_empty() { t!("settings-display-refused") } else { e };
            self.refused = Some(words.to_string());
            kit.say(REFUSED, words, false, false);
            return;
        }
        on_thread(outputs::heads, kit.then::<Display, _>(|me, k, got| {
            me.got(got);
            me.offer_keep(k);
        }));
    }

    fn offer_keep(&mut self, kit: &mut Kit) {
        self.trials += 1;
        Trial::start(kit, self, REVERT_AFTER, |me: &mut Display| &mut me.trial, Display::revert);
        kit.aloud(&t!("settings-display-aloud-changed", seconds = REVERT_AFTER));
    }

    fn stop_countdown(&mut self) {
        self.trial.stop();
    }

    fn revert(&mut self, kit: &mut Kit) {
        self.stop_countdown();
        let Some(before) = self.before.take() else {
            self.load(kit);
            return;
        };
        on_thread(move || outputs::apply(&before), kit.then::<Display, _>(|me, k, undone: Result<(), String>| {
            if let Err(e) = undone {
                k.say(REFUSED, &t!("settings-display-not-undone", why = e), false, false);
            }
            me.load(k);
        }));
    }

    fn keep(&mut self, kit: &mut Kit) {
        self.stop_countdown();
        self.before = None;
        let saved = outputs::save(&settings_for(&self.outputs));
        if let Err(e) = &saved {
            eprintln!("trindle-settings: could not keep the display arrangement: {e}");
            crate::common::crash::error("E_SAVE_FAILED");
        }
        kit.settle_quietly(REFUSED, saved, t!("settings-display-not-kept"));
        outputs::remember_for_login(&self.outputs);
        self.load(kit);
    }

    fn keep_line(&self, kit: &Kit) -> Node<Msg> {
        let (heading, sub) = match &self.refused {
            Some(_) if self.trial.left().is_none() => (t!("settings-display-nothing-changed"), t!("settings-display-refused-change").to_string()),
            _ => (t!("settings-display-apply-at-once"), t!("settings-display-keep-asked", seconds = REVERT_AFTER)),
        };
        line(heading).sub(sub).key(REFUSED).node(kit)
    }

    fn save_night(&mut self, kit: &mut Kit, key: &str, value: &str) {
        if kit.settle(key, set_setting(key, value), not_saved()).is_some() {
            nightlight::restart();
        }
        self.night = Some(Night::read());
    }

    fn night_card(&self, kit: &Kit, n: &Night) -> Node<Msg> {
        let card = group(t!("settings-display-night-title")).icon("night-light-symbolic")
            .about(t!("settings-display-night-about"));
        let mut rows = Vec::new();
        let on_switch = switch(n.on, t!("settings-display-night"), |lit| {
            on::<Display>(move |me, k| me.save_night(k, nightlight::ON, if lit { "on" } else { "off" }))
        });
        rows.push(line(t!("settings-display-night"))
            .sub(t!("settings-display-night-about-line"))
            .control(on_switch)
            .key(nightlight::ON)
            .node(kit));

        let when_sub = match n.place {
            Some(_) => t!("settings-display-when-zone", zone = &n.zone),
            None => t!("settings-display-when-no-place").to_string(),
        };
        let whens = [nightlight::BY_SUN, nightlight::BY_TIMES];
        let chosen = whens.iter().position(|w| *w == n.when);
        let pick = dropdown(kit, "night:when", vec![t!("settings-display-when-sun").to_string(), t!("settings-display-when-times").to_string()], chosen,
                            t!("settings-display-when"), move |i| {
            let v = whens[i];
            on::<Display>(move |me, k| me.save_night(k, nightlight::WHEN, v))
        });
        rows.push(line(t!("settings-display-when")).sub(when_sub).control(pick).key(nightlight::WHEN).node(kit));

        if n.when == nightlight::BY_TIMES || n.place.is_none() {
            rows.push(time_line(kit, t!("settings-display-from"), nightlight::FROM, nightlight::evening_times(), &n.from));
            rows.push(time_line(kit, t!("settings-display-to"), nightlight::TO, nightlight::morning_times(), &n.to));
        }
        rows.extend(self.warmth_rows(kit, n));
        card.rows(rows).node(kit)
    }

    fn warmth_rows(&self, kit: &Kit, n: &Night) -> Vec<Node<Msg>> {
        let shown = (nightlight::WARMEST + nightlight::LEAST_WARM - n.warmth.clamp(nightlight::WARMEST, nightlight::LEAST_WARM)) as f32;
        let warm = slider(shown, nightlight::WARMEST as f32, nightlight::LEAST_WARM as f32, 100.0, t!("settings-display-warmth"), |v| {
            let kelvin = nightlight::WARMEST + nightlight::LEAST_WARM - ((v / 100.0).round() * 100.0) as u32;
            on::<Display>(move |me, k| me.warm(k, kelvin))
        });
        let ends = row(vec![note(t!("settings-display-little")).width(Length::Fill), note(t!("settings-display-very"))]).width(Length::Fill);
        let scale = column(vec![warm, ends]).gap(4.0).width(Length::Fixed(220.0));
        let span = (nightlight::LEAST_WARM - nightlight::WARMEST).max(1) as f32;
        let share = (shown - nightlight::WARMEST as f32) / span;
        vec![line(t!("settings-display-warmth")).sub(t!("settings-display-warmth-about")).control(scale)
                 .key(nightlight::WARMTH).below(warmth_strip(share)).node(kit)]
    }

    fn warm(&mut self, kit: &mut Kit, kelvin: u32) {
        if let Some(n) = self.night.as_mut() {
            n.warmth = kelvin;
        }
        self.warming += 1;
        let round = self.warming;
        kit.after::<Display>(Duration::from_millis(WARMTH_SETTLE_MS), move |me, k| {
            if me.warming == round {
                me.save_night(k, nightlight::WARMTH, &kelvin.to_string());
            }
        });
    }
}

fn use_line(kit: &Kit, out: &Head, only: bool) -> Node<Msg> {
    let name = out.name.clone();
    let sw = switch(out.enabled, t!("settings-display-use"), move |v| {
        let name = name.clone();
        on::<Display>(move |me, k| {
            if !only || v {
                me.change(k, &name, Change::Enabled(v));
            }
        })
    });
    let sub = if only { t!("settings-display-use-only") } else { t!("settings-display-use-off") };
    line(t!("settings-display-use")).sub(sub).control(sw.off(only)).node(kit)
}

fn resolution_line(kit: &Kit, out: &Head, off: bool) -> Node<Msg> {
    let mut modes: Vec<Resolution> = out.modes.iter().map(|m| m.resolution()).collect();
    modes.sort_by(|a, b| (b.width * b.height).cmp(&(a.width * a.height)).then(b.refresh.cmp(&a.refresh)));
    let mut seen = Vec::new();
    modes.retain(|m| !seen.contains(m) && {
        seen.push(*m);
        true
    });
    let labels: Vec<String> = modes
        .iter()
        .map(|m| {
            if out.modes.iter().any(|x| x.preferred && x.resolution() == *m) {
                t!("settings-display-recommended", mode = mode_label(m))
            } else {
                mode_label(m)
            }
        })
        .collect();
    let cur = out.current_mode().map(|m| m.resolution());
    let chosen = cur.and_then(|c| modes.iter().position(|m| *m == c));
    let name = out.name.clone();
    let pick = dropdown(kit, &format!("display:{}:mode", out.name), labels, chosen, &t!("settings-display-resolution-of", display = &out.name), move |i| {
        let (name, m) = (name.clone(), modes[i]);
        on::<Display>(move |me, k| me.change(k, &name, Change::Mode(m)))
    });
    line(t!("settings-display-resolution")).sub(t!("settings-display-resolution-about")).control(pick).node(kit).off(off)
}

fn size_line(kit: &Kit, out: &Head, off: bool) -> Node<Msg> {
    let base = base_scale(out);
    let now = out_scale(out) / base;
    let size = SIZES.iter().copied().min_by(|a, b| (a - now).abs().total_cmp(&(b - now).abs())).unwrap_or(1.0);
    let sizes: Vec<String> = SIZES
        .iter()
        .map(|s| if *s == 1.0 {
            t!("settings-display-size-standard", percent = (s * 100.0).round())
        } else {
            format!("{}%", (s * 100.0).round())
        })
        .collect();
    let chosen = SIZES.iter().position(|s| *s == size);
    let name = out.name.clone();
    let pick = dropdown(kit, &format!("display:{}:size", out.name), sizes, chosen, &t!("settings-display-size-on", display = &out.name), move |i| {
        let (name, v) = (name.clone(), SIZES[i]);
        on::<Display>(move |me, k| me.change(k, &name, Change::Scale((v * base * 10000.0).round() / 10000.0)))
    });
    line(t!("settings-display-size")).sub(t!("settings-display-size-about")).control(pick).node(kit).off(off)
}

fn turn_line(kit: &Kit, out: &Head, off: bool) -> Node<Msg> {
    let turns_words: Vec<String> = turns().iter().map(|(_, l)| l.to_string()).collect();
    let chosen = turns().iter().position(|(t, _)| *t == out.turn);
    let name = out.name.clone();
    let pick = dropdown(kit, &format!("display:{}:turn", out.name), turns_words, chosen, &t!("settings-display-rotation-of", display = &out.name), move |i| {
        let (name, t) = (name.clone(), turns()[i].0);
        on::<Display>(move |me, k| me.change(k, &name, Change::Turn(t)))
    });
    line(t!("settings-display-rotation")).sub(t!("settings-display-rotation-about")).control(pick).node(kit).off(off)
}

fn time_line(kit: &Kit, title: &str, key: &'static str, times: Vec<String>, now: &str) -> Node<Msg> {
    let chosen = times.iter().position(|t| t == now);
    let options = times.clone();
    let pick = dropdown(kit, &format!("night:{key}"), options, chosen, title, move |i| {
        let v = times[i].clone();
        on::<Display>(move |me, k| me.save_night(k, key, &v))
    });
    line(title).control(pick).key(key).node(kit)
}

struct Screen {
    at: (f32, f32),
    size: (f32, f32),
    name: String,
    res: String,
}

fn screens_art(screens: Vec<Screen>, empty: &'static str, tip: String) -> Node<Msg> {
    sketch(tip, (520.0, 200.0), 160.0, move |s: &mut Sketch| {
        let (boxed, quiet) = (s.boxed(), s.quiet());
        if screens.is_empty() {
            s.rect((160.0, 20.0, 200.0, 130.0), 10.0, boxed);
            let glass = s.page();
            s.rect((167.0, 27.0, 186.0, 116.0), 6.0, glass);
            s.rect((250.0, 150.0, 20.0, 18.0), 2.0, boxed);
            s.rect((220.0, 166.0, 80.0, 8.0), 4.0, boxed);
            s.text(empty, (260.0, 85.0), 14.0, quiet);
            return;
        }
        let left = screens.iter().map(|d| d.at.0).fold(f32::INFINITY, f32::min);
        let top = screens.iter().map(|d| d.at.1).fold(f32::INFINITY, f32::min);
        let right = screens.iter().map(|d| d.at.0 + d.size.0).fold(f32::NEG_INFINITY, f32::max);
        let bottom = screens.iter().map(|d| d.at.1 + d.size.1).fold(f32::NEG_INFINITY, f32::max);
        let (bw, bh) = ((right - left).max(1.0), (bottom - top).max(1.0));
        let k = (500.0 / bw).min(166.0 / bh);
        let (ox, oy) = ((520.0 - bw * k) / 2.0, (176.0 - bh * k) / 2.0);
        for d in screens.iter() {
            let (x, y) = (ox + (d.at.0 - left) * k + 3.0, oy + (d.at.1 - top) * k + 3.0);
            let (w, h) = (d.size.0 * k - 6.0, d.size.1 * k - 6.0);
            if (d.at.1 + d.size.1 - bottom).abs() < 1.0 {
                s.rect((x + w / 2.0 - 8.0, y + h, 16.0, 14.0), 2.0, boxed);
                s.rect((x + w / 2.0 - w * 0.18, y + h + 12.0, w * 0.36, 6.0), 3.0, boxed);
            }
            s.rect((x, y, w, h), 10.0, boxed);
            let glass = s.wash(Palette::Blue, 0.55);
            s.rect((x + 6.0, y + 6.0, w - 12.0, h - 12.0), 6.0, glass);
            let big = (h * 0.2).clamp(10.0, 18.0);
            let on_glass = s.words_on(glass);
            s.text(&d.name, (x + w / 2.0, y + h / 2.0 - big * 0.7), big, on_glass);
            s.text(&d.res, (x + w / 2.0, y + h / 2.0 + big * 0.7), big * 0.8, on_glass);
        }
    })
}

fn warmth_strip(share: f32) -> Node<Msg> {
    sketch(t!("settings-display-strip"), (240.0, 40.0), 40.0, move |s: &mut Sketch| {
        const N: usize = 12;
        let (cool, warm) = (s.words(), Palette::Orange.colour());
        let (left, wide) = ((240.0 - s.wide()) / 2.0, s.wide());
        let w = (wide - 4.0 * (N - 1) as f32) / N as f32;
        for i in 0..N {
            let t = i as f32 / (N - 1) as f32;
            let mut c = mix(cool, warm, 0.25 + 0.75 * t);
            if t > share + 0.01 {
                c = alpha(c, 0.3);
            }
            s.rect((left + i as f32 * (w + 4.0), 0.0, w, 40.0), 8.0, c);
        }
    }).height(Length::Fixed(40.0))
}

impl Page for Display {
    fn id(&self) -> &'static str {
        "display"
    }
    fn foot(&self, _kit: &Kit) -> Option<Node<Msg>> {
        self.trial.foot(FootLook { ask: t!("settings-display-foot-keep"), more: "", safe: false, escape: false }, self.trials, Display::revert, Display::keep)
    }
    fn title(&self) -> &'static str {
        t!("settings-display-title")
    }
    fn icon(&self) -> &'static str {
        "video-display-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-display-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-display-help")
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let fresh;
        let night = match &self.night {
            Some(n) => n,
            None => {
                fresh = Night::read();
                &fresh
            }
        };
        let mut items = Vec::new();
        if !self.loaded {
            items.push(headline(kit, t!("settings-display-looking")));
        } else if self.outputs.is_empty() {
            items.push(headline(kit, t!("settings-display-none-found")));
        } else {
            items.push(headline(kit, sum_up(&self.outputs, night)));
        }

        let screens: Vec<Screen> = self.outputs.iter().filter(|o| o.enabled).filter_map(|o| {
            let (w, h) = o.logical_size();
            let (res, _, _) = facts_of(o)?;
            (w > 0 && h > 0).then(|| Screen { at: (o.position.0 as f32, o.position.1 as f32), size: (w as f32, h as f32),
                                             name: o.name.clone(), res })
        }).collect();
        let tip = match screens.len() {
            0 => t!("settings-display-no-display-tip").to_string(),
            1 => t!("settings-display-one-display-tip", name = &screens[0].name, res = &screens[0].res),
            n => t!("settings-display-many-displays-tip", n = n),
        };
        let art = screens_art(screens, if self.loaded { t!("settings-display-no-display") } else { "" }, tip).height(Length::Fill);
        items.push(picture("video-display-symbolic", t!("settings-display-your-displays"), column(vec![art, self.keep_line(kit)]).gap(16.0)));
        if self.loaded && self.outputs.is_empty() {
            let why = if self.err.is_empty() { t!("settings-display-listed-none") } else { self.err.as_str() };
            items.push(group(t!("settings-display-none-title")).icon("video-display-symbolic")
                .rows(vec![line(t!("settings-display-none-heading")).sub(why).node(kit)]).node(kit));
        }
        for out in &self.outputs {
            items.push(self.output_card(kit, out));
        }
        items.extend(self.place_card(kit));
        if self.nvidia {
            items.push(group(t!("settings-display-graphics-title")).icon("video-display-symbolic")
                .about(t!("settings-display-graphics-about"))
                .rows(vec![line(t!("settings-display-nvidia"))
                    .sub(t!("settings-display-nvidia-about"))
                    .control(help_link(t!("settings-display-nvidia-help"), "nvidia"))
                    .node(kit)])
                .node(kit));
        }
        items.push(self.night_card(kit, night));
        crate::settings::ui::add_no_driver(kit, &mut items, crate::common::no_driver::Class::Graphics);
        page(kit, self.title(), self.help(), items)
    }
    fn shown(&mut self, kit: &mut Kit) {
        self.night = Some(Night::read());
        self.nvidia = crate::common::graphics::nvidia_on_open_driver();
        if self.trial.left().is_none() {
            self.load(kit);
        }
    }
}

#[cfg(test)]
impl Display {
    pub(crate) fn on_trial() -> Display {
        Display { trial: Trial::at(9), trials: 1, ..Display::made_up() }
    }

    pub(crate) fn flipped() -> Vec<Display> {
        let mut one = Display::made_up();
        one.outputs[1].enabled = false;
        let keeping = Display { trial: Trial::at(9), ..Display::made_up() };
        let refused = Display { refused: Some("The desktop refused it.".into()), ..Display::made_up() };
        vec![Display::made_up(), one, keeping, refused]
    }

    pub(crate) fn made_up() -> Display {
        let head = |name: &str, size: (i32, i32), at: (i32, i32)| {
            let mode = |width: i32, height: i32, preferred: bool| crate::common::outputs::Mode { width, height, refresh: 60000, preferred };
            let modes = vec![mode(size.0, size.1, true), mode(1920, 1080, false), mode(1600, 900, false), mode(1280, 720, false)];
            Head { name: name.into(), description: String::new(), make: String::new(), model: String::new(),
                   serial: String::new(), physical_size: (0, 0), modes, current: Some(0), enabled: true,
                   position: at, turn: Turn::Normal, scale: 1.0, varies_rate: false }
        };
        let night = Night { on: true, when: nightlight::BY_SUN.to_string(), from: "20:00".into(), to: "07:00".into(),
                            warmth: 4000, place: Some((52.0, 0.0)), zone: "Europe/London".into() };
        Display { loaded: true, outputs: vec![head("eDP-1", (2256, 1504), (0, 0)), head("HDMI-A-1", (2560, 1440), (2256, 0))],
                  night: Some(night), ..Display::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::outputs::Mode;
    use crate::settings::ui::Do;
    use crate::settings::Settings;
    use crate::toolkit::event::Loop;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App as _, Ui};

    fn head(name: &str, size: (i32, i32), at: (i32, i32)) -> Head {
        let mode = Mode { width: size.0, height: size.1, refresh: 60000, preferred: true };
        Head { name: name.into(), description: String::new(), make: String::new(), model: String::new(),
               serial: String::new(), physical_size: (0, 0), modes: vec![mode], current: Some(0), enabled: true,
               position: at, turn: Turn::Normal, scale: 1.0, varies_rate: false }
    }

    #[test]
    fn a_change_to_the_displays_is_said_aloud_and_asked_in_the_foot() {
        let events = Loop::new();
        let said = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let heard = said.clone();
        let asks = events.post(move |d: Do| {
            if let Do::Say(words) = d {
                heard.borrow_mut().push(words);
            }
        });
        let mut kit = Kit::new(events.clone(), asks, None, None);
        let mut d = Display::made_up();
        assert!(d.foot(&kit).is_none() && d.trial.left().is_none());
        d.offer_keep(&mut kit);
        assert_eq!((d.trial.left(), d.trials), (Some(REVERT_AFTER), 1));
        assert!(d.foot(&kit).is_some());
        let done = events.clone();
        events.after(std::time::Duration::from_millis(50), move || done.quit());
        events.run();
        assert_eq!(*said.borrow(), vec![format!("The displays have changed. They go back in {REVERT_AFTER} seconds unless you press Keep.")]);
        d.stop_countdown();
        assert!(d.foot(&kit).is_none());
    }

    #[test]
    #[ignore]
    fn shoot_with_made_up_displays() {
        let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));
        let size = std::env::var("TRINDLE_SHOT_SIZE").unwrap_or_else(|_| "1280x800".into());
        let (w, h) = size.split_once('x').and_then(|(a, b)| Some((a.parse::<f32>().ok()?, b.parse::<f32>().ok()?))).unwrap_or((1280.0, 800.0));
        let two = std::env::var("TRINDLE_SHOT_HEADS").is_ok_and(|v| v == "2");
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut d = Display { loaded: true, ..Display::default() };
        d.outputs = vec![head("eDP-1", (2256, 1504), (0, 0))];
        if two {
            d.outputs.push(head("HDMI-A-1", (2560, 1440), (2256, 0)));
        }
        d.night = Some(Night::read());
        let mut kit = Kit::new(events.clone(), asks, None, None);
        kit.current = "display".into();
        let s = Settings { kit, pages: vec![Box::new(d)], current: 0, search: String::new(), tall: h };
        let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
        let mut pix = tiny_skia::Pixmap::new(w as u32, h as u32).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        std::fs::create_dir_all(&out).unwrap();
        pix.save_png(out.join(format!("display-{}-{size}.png", if two { 2 } else { 1 }))).unwrap();
    }

    fn made_up(by_times: bool) -> Settings {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut d = Display { loaded: true, ..Display::default() };
        d.outputs = vec![head("eDP-1", (2256, 1504), (0, 0)), head("HDMI-A-1", (2560, 1440), (2256, 0))];
        d.night = Some(Night {
            on: true, when: if by_times { nightlight::BY_TIMES } else { nightlight::BY_SUN }.to_string(),
            from: "20:00".into(), to: "07:00".into(), warmth: 4000, place: Some((52.2, 0.1)), zone: "Europe/London".into(),
        });
        let mut kit = Kit::new(events.clone(), asks, None, None);
        kit.current = "display".into();
        Settings { kit, pages: vec![Box::new(d)], current: 0, search: String::new(), tall: 692.0 }
    }

    #[test]
    fn showing_the_night_light_times_moves_only_what_is_under_them() {
        let (w, h) = (1278u32, 692u32);
        let mut ui = Ui::new(made_up(false).view(), Theme::new(true, "#fbc711"));
        ui.lay_at(w, h);
        let most = ui.places().iter().find(|p| p.0 == "page:Display").map(|p| p.2).expect("the page scrolls");
        assert!(most > 0.0);
        ui.scroll_area("page:Display", most);
        let (was, place) = (ui.puzzle_cards(), ui.places());
        ui.set_view(made_up(true).view());
        ui.lay_at(w, h);
        let now = ui.puzzle_cards();
        assert_eq!(ui.places().iter().map(|p| (&p.0, p.1)).collect::<Vec<_>>(), place.iter().map(|p| (&p.0, p.1)).collect::<Vec<_>>(), "the page stays where it was");
        assert_eq!(was.len(), now.len());
        let grew: Vec<usize> = (0..was.len()).filter(|&i| now[i].h > was[i].h + 20.0 && now[i].y == was[i].y).collect();
        assert!(!grew.is_empty(), "the night light card grew: {was:?} {now:?}");
        let beside = |a: &crate::toolkit::widget::Rect, b: &crate::toolkit::widget::Rect| a.x < b.x + b.w - 0.5 && b.x < a.x + a.w - 0.5;
        for i in 0..was.len() {
            assert_eq!((was[i].x, was[i].w), (now[i].x, now[i].w), "card {i} keeps its column and width");
            if was[i].y != now[i].y {
                assert!(now[i].y > was[i].y, "card {i} only went down");
                assert!((0..was.len()).any(|k| was[k].y < was[i].y && beside(&was[k], &was[i]) && (grew.contains(&k) || was[k].y != now[k].y)),
                        "card {i} moved though nothing over it did");
            }
        }
        ui.set_view(made_up(false).view());
        ui.lay_at(w, h);
        assert_eq!(ui.puzzle_cards(), was);
    }

    #[test]
    #[ignore]
    fn shoot_before_and_after_the_times_show() {
        let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));
        std::fs::create_dir_all(&out).unwrap();
        let (w, h) = (1278u32, 692u32);
        let mut ui = Ui::new(made_up(false).view(), Theme::new(true, "#fbc711"));
        ui.lay_at(w, h);
        let most = ui.places().iter().find(|p| p.0 == "page:Display").map_or(0.0, |p| p.2);
        ui.scroll_area("page:Display", most);
        for (by_times, name) in [(false, "still-display-1-before.png"), (true, "still-display-2-after.png")] {
            ui.set_view(made_up(by_times).view());
            let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            pix.save_png(out.join(name)).unwrap();
        }
        let mut anew = Ui::new(made_up(true).view(), Theme::new(true, "#fbc711"));
        anew.lay_at(w, h);
        anew.scroll_area("page:Display", 1e9);
        let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
        anew.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(out.join("still-display-3-opened-anew.png")).unwrap();
    }
}
