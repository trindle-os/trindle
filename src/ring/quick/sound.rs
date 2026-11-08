//! The sound panel: volume, mute, the output and input devices and the level meter.

use std::time::{Duration, Instant};

use crate::common::audio;
use crate::common::run::capture;
use crate::ring::meter;
use crate::ring::ctx::Msg;
use crate::toolkit::t;
use crate::toolkit::widget::{button, choice, column, drawing, label, Length, Node, note, row, scroll, slider};

use super::{Body, BODY_GAP, Level, m, Quick, ROW_GAP, Well};
use super::battery::TOUCH_HOLD;

const SOUND_EVERY: Duration = Duration::from_secs(2);

const METER_EVERY: Duration = Duration::from_millis(100);

const METER_HEIGHT: f32 = 8.0;

pub(super) const PCT_WIDTH: f32 = 40.0;

fn sound_levels() -> [(&'static str, &'static str, bool); 2] {
    [(t!("ring-quick-speakers"), "@DEFAULT_AUDIO_SINK@", true), (t!("ring-quick-microphone"), "@DEFAULT_AUDIO_SOURCE@", false)]
}

pub(super) fn short_names(names: &[String]) -> Vec<String> {
    let words: Vec<Vec<&str>> = names.iter().map(|n| n.split_whitespace().collect()).collect();
    let mut common = 0;
    if words.len() > 1 {
        while words.iter().all(|w| w.len() > common + 1 && w.get(common) == words[0].get(common)) {
            common += 1;
        }
    }
    words.iter().map(|w| {
        let s = w[common..].join(" ");
        if s.chars().count() > 30 { format!("{}\u{2026}", s.chars().take(29).collect::<String>()) } else { s }
    }).collect()
}

impl Quick {
    pub(super) fn sound(&mut self) {
        self.body = Body::Sound(sound_levels().iter().map(|&(name, target, out)| Level {
            name, target, out, value: 0.0, pct: String::new(), muted: false, touched: None, nodes: vec![], chosen: 0,
        }).collect());
        let b = self.back();
        audio::list(move |found| b.then(move |q| q.got_devices(found)));
        self.read_levels();
        self.repeat(SOUND_EVERY, |q| q.read_levels());
        self.heard = 0.0;
        self.meter = (!crate::common::in_use::microphone_off()).then(meter::Meter::start);
        self.repeat(METER_EVERY, |q| q.heard = q.meter.as_ref().map_or(0.0, |m| m.level()));
    }

    pub(super) fn sound_view(&self, levels: &[Level]) -> Vec<Node<Msg>> {
        let mut out = Vec::new();
        for (i, l) in levels.iter().enumerate() {
            out.push(note(l.name));
            if !l.out && crate::common::in_use::microphone_off() {
                out.push(note(crate::common::in_use::microphone_off_note()));
            }
            let (icon_name, tip) = match (l.out, l.muted) {
                (true, true) => ("audio-volume-muted-symbolic", t!("ring-quick-sound-on")),
                (true, false) => ("audio-volume-high-symbolic", t!("ring-quick-mute")),
                (false, true) => ("microphone-sensitivity-muted-symbolic", t!("ring-quick-sound-on")),
                (false, false) => ("audio-input-microphone-symbolic", t!("ring-quick-mute")),
            };
            out.push(row(vec![
                button(icon_name, tip).on(l.muted).on_press(m(move |q| q.toggle_mute(i))).well(),
                slider(l.value, 0.0, 100.0, 1.0, l.name, move |v| m(move |q| q.set_volume(i, v))),
                label(l.pct.clone()).width(Length::Fixed(PCT_WIDTH)),
            ]).gap(ROW_GAP));
            if l.out {
                out.push(row(vec![
                    button("media-playback-start-symbolic", t!("ring-quick-test-speakers")).on_press(m(|_| meter::beep())).well(),
                    label(t!("ring-quick-test-speakers")),
                ]).gap(ROW_GAP));
            } else if self.meter.is_some() {
                out.push(self.meter_bar());
            }
            if l.nodes.len() >= 2 {
                let names: Vec<String> = l.nodes.iter().map(|n| n.1.clone()).collect();
                out.push(choice(short_names(&names), l.chosen, l.name, move |k| m(move |q| q.choose_device(i, k))));
            }
        }

        vec![scroll("quick:sound", column(out).gap(BODY_GAP)).height(Length::Shrink)]
    }

    fn meter_bar(&self) -> Node<Msg> {
        let share = self.heard;
        drawing((PCT_WIDTH * 4.0, METER_HEIGHT), t!("ring-quick-mic-level"), move |canvas, pen| {
            let (w, h) = (canvas.width() as f32, canvas.height() as f32);
            let fill = |canvas: &mut tiny_skia::PixmapMut, wide: f32, c| {
                if let Some(r) = tiny_skia::Rect::from_xywh(0.0, 0.0, wide, h) {
                    canvas.fill_rect(r, &crate::toolkit::colour::paint(c), tiny_skia::Transform::identity(), None);
                }
            };
            fill(canvas, w, pen.theme.well);
            fill(canvas, (w * share).round(), pen.theme.accent);
        }).width(Length::Fill).height(Length::Fixed(METER_HEIGHT))
    }

    fn got_devices(&mut self, found: Result<audio::Devices, String>) {
        let Body::Sound(levels) = &mut self.body else { return };
        let found = found.unwrap_or_default();
        for l in levels.iter_mut() {
            let (nodes, current) = if l.out { (&found.outputs, &found.output) } else { (&found.inputs, &found.input) };
            l.nodes = nodes.iter().map(|n| (n.id, n.label.clone())).collect();
            l.chosen = nodes.iter().position(|n| Some(&n.name) == current.as_ref()).unwrap_or(0);
        }
    }

    fn read_levels(&mut self) {
        for i in 0..sound_levels().len() {
            self.read_level(i);
        }
    }

    fn read_level(&mut self, i: usize) {
        let b = self.back();
        let target = sound_levels()[i].1;
        capture(&["wpctl", "get-volume", target], None, move |ok, out, _| b.then(move |q| q.got_level(i, ok, &out)));
    }

    fn got_level(&mut self, i: usize, ok: bool, out: &str) {
        let Body::Sound(levels) = &mut self.body else { return };
        let Some(l) = levels.get_mut(i) else { return };
        let level = out.split_whitespace().nth(1).and_then(|v| v.parse::<f64>().ok());
        let (true, Some(level)) = (ok, level) else { return };
        l.muted = out.contains("MUTED");
        if l.touched.is_none_or(|t| t.elapsed() >= TOUCH_HOLD) {
            l.value = (level * 100.0).round() as f32;
            l.pct = format!("{}%", l.value as i32);
        }
    }

    fn set_volume(&mut self, i: usize, v: f32) {
        let Body::Sound(levels) = &mut self.body else { return };
        let Some(l) = levels.get_mut(i) else { return };
        let v = v.round();
        l.touched = Some(Instant::now());
        l.value = v;
        l.pct = format!("{}%", v as i32);
        capture(&["wpctl", "set-volume", "-l", "1.0", l.target, &format!("{:.2}", v / 100.0)], None, |_, _, _| {});
    }

    fn toggle_mute(&mut self, i: usize) {
        let b = self.back();
        capture(&["wpctl", "set-mute", sound_levels()[i].1, "toggle"], None, move |_, _, _| b.then(move |q| q.read_level(i)));
    }

    fn choose_device(&mut self, i: usize, k: usize) {
        let Body::Sound(levels) = &mut self.body else { return };
        let Some(l) = levels.get_mut(i) else { return };
        let Some(&(id, _)) = l.nodes.get(k) else { return };
        l.chosen = k;
        let b = self.back();
        audio::set_default(id, move |_| b.then(move |q| q.read_level(i)));
    }
}
