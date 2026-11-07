//! Audio devices, volume and defaults, via pw-dump and wpctl.

use crate::toolkit::colour::paint as brush;
use std::io::Read;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

use tiny_skia::{LineCap, PathBuilder, PixmapMut, Stroke, Transform};

use crate::common::audio::{self as devices, Devices, Kind};
use crate::common::run::{capture, have};
use crate::settings::ui::{dropdown, group, headline, line, missing, no_more, on, page, ran, Kit, Msg, Page, Raw};
use crate::toolkit::colour::Colour;
use crate::toolkit::theme;
use crate::toolkit::widget::{button, drawing, slider, switch, Length, Node, Tone};
use crate::toolkit::t;

const PICK: &str = "audio-device";
const TEST: &str = "audio-test";
const ASKED: &str = "audio-pipewire";

fn test_sound() -> PathBuf {
    let dir = crate::common::files::scratch();
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("trindle-test-sound.wav");
    if path.exists() {
        return path;
    }
    let rate: u32 = crate::common::sounds::RATE;
    let total = (rate as f64 * 0.9) as u32;
    let mut frames: Vec<u8> = Vec::with_capacity(total as usize * 4);
    for i in 0..total {
        let t = i as f64 / rate as f64;
        let mut v = 0.0;
        for (freq, start, length) in [(660.0, 0.0, 0.4), (880.0, 0.3, 0.6)] {
            if start <= t && t < start + length {
                let env = (-5.0 * (t - start)).exp() * ((t - start) * 200.0).min(1.0);
                v += 0.35 * env * (2.0 * std::f64::consts::PI * freq * (t - start)).sin();
            }
        }
        let s = (v.clamp(-1.0, 1.0) * 32000.0) as i16;
        frames.extend_from_slice(&s.to_le_bytes());
        frames.extend_from_slice(&s.to_le_bytes());
    }
    let mut wav = Vec::with_capacity(44 + frames.len());
    wav.extend_from_slice(b"RIFF");
    wav.extend_from_slice(&(36 + frames.len() as u32).to_le_bytes());
    wav.extend_from_slice(b"WAVE");
    wav.extend_from_slice(b"fmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&rate.to_le_bytes());
    wav.extend_from_slice(&(rate * 4).to_le_bytes());
    wav.extend_from_slice(&4u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&(frames.len() as u32).to_le_bytes());
    wav.extend_from_slice(&frames);
    if let Err(e) = std::fs::write(&path, wav) {
        eprintln!("trindle-settings: could not write the test sound: {e}");
    }
    path
}

struct Meter {
    child: Child,
    level: Arc<AtomicU32>,
}

impl Meter {
    const CHUNK: usize = 1600;

    fn start(target: i64) -> Option<Meter> {
        let mut child = Command::new("pw-record")
            .args(["--target", &target.to_string(), "--rate", "16000", "--channels", "1", "--format", "s16", "-"])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;
        let mut out = child.stdout.take()?;
        let level = Arc::new(AtomicU32::new(0));
        let shared = level.clone();
        std::thread::spawn(move || {
            let mut chunk = [0u8; Meter::CHUNK];
            let mut now = 0.0f64;
            while out.read_exact(&mut chunk).is_ok() {
                let peak = chunk.chunks_exact(2).map(|b| i16::from_le_bytes([b[0], b[1]]).unsigned_abs()).max().unwrap_or(0) as f64 / 32768.0;
                let target = peak.sqrt();
                now = if target > now { target } else { now * 0.8 + target * 0.2 };
                shared.store((now as f32).to_bits(), Ordering::Relaxed);
            }
        });
        Some(Meter { child, level })
    }

    fn level(&self) -> f32 {
        f32::from_bits(self.level.load(Ordering::Relaxed))
    }

    fn stop(mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn mute_note(output: bool) -> &'static str {
    if output {
        t!("settings-audio-mute-out-about")
    } else if crate::common::in_use::microphone_off() {
        crate::common::in_use::microphone_off_note()
    } else {
        t!("settings-audio-mute-in-about")
    }
}

fn with_kind(label: &str, kind: Kind) -> String {
    let word = match kind {
        Kind::Bluetooth => t!("settings-audio-kind-bluetooth"),
        Kind::Hdmi => t!("settings-audio-kind-hdmi"),
        Kind::Usb => t!("settings-audio-kind-usb"),
        Kind::Headphones => t!("settings-audio-kind-headphones"),
        Kind::Speakers => t!("settings-audio-kind-speakers"),
        Kind::Other => return label.to_string(),
    };
    if label.to_lowercase().contains(&word.to_lowercase()) {
        return label.to_string();
    }
    t!("settings-audio-with-kind", name = label, kind = word).to_string()
}

#[derive(Default)]
struct Section {
    title: &'static str,
    target: &'static str,
    nodes: Vec<(i64, String)>,
    current: Option<i64>,
    device: String,
    pct: f32,
    muted: bool,
    ready: bool,
    told: bool,
    round: u32,
    real: (f32, bool),
    kind: Kind,
    speakers: Option<(i64, String)>,
    silent: String,
}

impl Section {
    fn put_back(&mut self) {
        (self.pct, self.muted) = self.real;
    }
}

fn volume_key(at: usize) -> String {
    format!("audio-volume-{at}")
}

fn mute_key(at: usize) -> String {
    format!("audio-mute-{at}")
}

fn no_volume() -> &'static str {
    t!("settings-audio-no-volume")
}
fn no_mute() -> &'static str {
    t!("settings-audio-no-mute")
}
fn no_unmute() -> &'static str {
    t!("settings-audio-no-unmute")
}

enum State {
    Waiting,
    Missing,
    Failed,
    Found { no_output: bool },
}

pub struct Audio {
    state: State,
    sections: Vec<Section>,
    meter: Option<Meter>,
    loads: u32,
}

impl Default for Audio {
    fn default() -> Self {
        Audio { state: State::Waiting, sections: Vec::new(), meter: None, loads: 0 }
    }
}

impl Audio {
    fn stop_meter(&mut self) {
        if let Some(m) = self.meter.take() {
            m.stop();
        }
    }

    fn load(&mut self, kit: &mut Kit) {
        self.loads += 1;
        if !have("pw-dump") || !have("wpctl") {
            self.stop_meter();
            self.sections.clear();
            self.state = State::Missing;
            return;
        }
        let at = self.loads;
        devices::list(kit.then::<Audio, Result<Devices, String>>(move |me, k, found| {
            if me.loads == at {
                me.got(k, found);
            }
        }));
    }

    fn got(&mut self, kit: &mut Kit, found: Result<Devices, String>) {
        self.stop_meter();
        self.sections.clear();
        let found = match found {
            Ok(found) => found,
            Err(err) => {
                kit.say(ASKED, &err, false, false);
                self.state = State::Failed;
                return;
            }
        };
        self.state = State::Found { no_output: found.current_output().is_none() };
        self.section(kit, "Output", &found.outputs, found.current_output(), "@DEFAULT_AUDIO_SINK@");
        if let Some(out) = self.sections.last_mut() {
            out.speakers = found.speakers().filter(|_| out.kind == Kind::Bluetooth).map(|n| (n.id, with_kind(&n.label, n.kind)));
            out.silent = found.bluetooth_silent.join(", ");
        }
        self.section(kit, "Input", &found.inputs, found.current_input(), "@DEFAULT_AUDIO_SOURCE@");
    }

    fn section(&mut self, kit: &mut Kit, title: &'static str, nodes: &[devices::Node], current: Option<&devices::Node>,
               target: &'static str) {
        let at = self.sections.len();
        let output = title == "Output";
        let tagged = |n: &devices::Node| if output { with_kind(&n.label, n.kind) } else { n.label.clone() };
        self.sections.push(Section {
            title,
            target,
            nodes: nodes.iter().map(|n| (n.id, tagged(n))).collect(),
            current: current.map(|c| c.id),
            device: current.map(tagged).unwrap_or_default(),
            kind: current.filter(|_| output).map_or(Kind::Other, |c| c.kind),
            ..Section::default()
        });
        let Some(current) = current else { return };
        if title == "Input" && kit.current == "audio" {
            self.meter = Meter::start(current.id);
        }
        self.read_level(kit, at, target);
    }

    fn read_level(&mut self, kit: &mut Kit, at: usize, target: &'static str) {
        let loads = self.loads;
        let done = kit.then::<Audio, (bool, String)>(move |me, _, (ok, out)| {
            if me.loads != loads {
                return;
            }
            let Some(s) = me.sections.get_mut(at) else { return };
            let level = out
                .split_once("Volume:")
                .and_then(|(_, rest)| rest.split_whitespace().next())
                .and_then(|v| v.parse::<f64>().ok());
            if let (true, Some(level)) = (ok, level) {
                s.pct = (level * 100.0).round() as f32;
                s.muted = out.contains("MUTED");
                s.real = (s.pct, s.muted);
            }
            s.told = true;
            s.ready = true;
        });
        capture(&["wpctl", "get-volume", target], None, move |ok, out, _| done((ok, out)));
    }

    fn pick(&mut self, kit: &mut Kit, at: usize, choice: usize) {
        let Some(id) = self.sections.get(at).and_then(|s| s.nodes.get(choice)).map(|n| n.0) else { return };
        let key = format!("{PICK}-{at}");
        devices::set_default(id, kit.then::<Audio, Option<String>>(move |_, k, failed| {
            k.settle(&key, failed.map_or(Ok(()), |e| Err(Raw(e))), t!("settings-audio-not-chosen"));
            k.after::<Audio>(Duration::from_millis(250), |me, k| me.load(k));
        }));
    }

    fn volume(&mut self, kit: &mut Kit, at: usize, value: f32) {
        let Some(s) = self.sections.get_mut(at) else { return };
        s.pct = value.round();
        if !s.ready {
            return;
        }
        s.told = true;
        s.round += 1;
        let round = s.round;
        kit.after::<Audio>(Duration::from_millis(60), move |me, k| {
            let Some(s) = me.sections.get(at) else { return };
            if s.round == round {
                let level = format!("{:.2}", s.pct / 100.0);
                let pct = s.pct;
                let back = me.changed(k, at, &volume_key(at), no_volume(), move |s| s.real.0 = pct);
                capture(&["wpctl", "set-volume", s.target, &level], None, move |ok, _, err| back(ran(ok, &err)));
            }
        });
    }

    fn mute(&mut self, kit: &mut Kit, at: usize, on: bool) {
        let Some(s) = self.sections.get_mut(at) else { return };
        s.muted = on;
        s.told = true;
        if s.ready {
            let target = s.target;
            let back = self.changed(kit, at, &mute_key(at), if on { no_mute() } else { no_unmute() }, move |s| s.real.1 = on);
            capture(&["wpctl", "set-mute", target, if on { "1" } else { "0" }], None, move |ok, _, err| back(ran(ok, &err)));
        }
    }

    fn changed(&self, kit: &Kit, at: usize, key: &str, could_not: &'static str,
               took: impl FnOnce(&mut Section) + 'static) -> impl FnOnce(Result<(), Raw>) + 'static {
        let loads = self.loads;
        kit.settles_quietly::<Audio, (), Raw>(key, could_not, move |me, _, done| {
            if me.loads != loads {
                return;
            }
            let Some(s) = me.sections.get_mut(at) else { return };
            match done {
                Some(()) => took(s),
                None => s.put_back(),
            }
        })
    }

    fn play(&mut self, kit: &mut Kit, device: i64) {
        let sound = test_sound();
        let done = kit.settles::<Audio, (), Raw>(TEST, t!("settings-audio-test-failed"), no_more);
        capture(&["pw-play", "--media-role", crate::common::sounds::ROLE, "--target", &device.to_string(), &sound.to_string_lossy()], None, move |ok, _, e| done(ran(ok, &e)));
    }

    fn said(&self) -> (String, Tone) {
        match self.state {
            State::Missing => return (t!("settings-audio-top-missing").into(), Tone::Problem),
            State::Failed => return (t!("settings-audio-top-failed").into(), Tone::Problem),
            State::Found { no_output: true } => return (t!("settings-audio-top-no-output").into(), Tone::Problem),
            _ => {}
        }
        let output = self.sections.iter().find(|s| s.title == "Output" && s.current.is_some() && s.told);
        let Some(s) = output else { return (t!("settings-audio-top-listening").into(), Tone::Plain) };
        let pct = s.pct.round();
        if s.muted {
            (t!("settings-audio-top-muted", device = &s.device), Tone::Warn)
        } else if pct == 0.0 {
            (t!("settings-audio-top-down", device = &s.device), Tone::Warn)
        } else {
            (t!("settings-audio-top-playing", device = &s.device, percent = pct), Tone::Plain)
        }
    }

    fn section_card(&self, kit: &Kit, at: usize, s: &Section) -> Node<Msg> {
        let output = s.title == "Output";
        let (name, icon) = if output {
            (t!("settings-audio-out-title"), "audio-speakers-symbolic")
        } else {
            (t!("settings-audio-in-title"), "audio-input-microphone-symbolic")
        };
        let card = group(name).icon(if output && s.kind != Kind::Other { s.kind.icon() } else { icon });
        let Some(current) = s.current else {
            return card.rows(vec![line(if output { t!("settings-audio-out-none") } else { t!("settings-audio-in-none") })
                .sub(if output { t!("settings-audio-out-plug") } else { t!("settings-audio-in-plug") })
                .node(kit)]).node(kit);
        };
        let labels: Vec<String> = s.nodes.iter().map(|n| n.1.clone()).collect();
        let chosen = s.nodes.iter().position(|n| n.0 == current);
        let key = format!("{PICK}-{at}");
        let mut rows = vec![
            line(t!("settings-audio-device")).sub(if output { t!("settings-audio-out-device-about") } else { t!("settings-audio-in-device-about") }).key(key.clone())
                .control(dropdown(kit, &key, labels, chosen, if output { t!("settings-audio-out-device-name") } else { t!("settings-audio-in-device-name") }, move |i| {
                    on::<Audio>(move |me, k| me.pick(k, at, i))
                })).node(kit),
            line(t!("settings-audio-volume")).sub(format!("{}%", s.pct.round())).key(volume_key(at)).control(slider(s.pct, 0.0, 100.0, 1.0, if output { t!("settings-audio-out-volume-name") } else { t!("settings-audio-in-volume-name") }, move |v| {
                on::<Audio>(move |me, k| me.volume(k, at, v))
            }).width(Length::Fixed(220.0))).node(kit),
            line(t!("settings-audio-mute")).sub(mute_note(output)).key(mute_key(at))
                .control(switch(s.muted, if output { t!("settings-audio-out-mute-name") } else { t!("settings-audio-in-mute-name") }, move |b| on::<Audio>(move |me, k| me.mute(k, at, b))))
                .node(kit),
        ];
        if output {
            if let Some((id, label)) = s.speakers.clone() {
                rows.insert(0, line(t!("settings-audio-bt-now")).sub(t!("settings-audio-bt-now-about")).icon(Kind::Bluetooth.icon())
                    .control(button(Kind::Speakers.icon(), t!("settings-audio-use-speakers", device = &label))
                        .on_press(on::<Audio>(move |me, k| {
                            if let Some(i) = me.sections.get(at).and_then(|s| s.nodes.iter().position(|n| n.0 == id)) {
                                me.pick(k, at, i)
                            }
                        }))).node(kit));
            }
            if !s.silent.is_empty() {
                rows.push(line(t!("settings-audio-bt-silent", names = &s.silent)).sub(t!("settings-audio-bt-silent-about"))
                    .icon(Kind::Bluetooth.icon()).node(kit));
            }
            rows.push(line(t!("settings-audio-test")).sub(t!("settings-audio-test-about")).key(TEST)
                .control(button("media-playback-start-symbolic", t!("settings-audio-test-button"))
                    .on_press(on::<Audio>(move |me, k| me.play(k, current))))
                .node(kit));
            card.about(t!("settings-audio-out-about"))
                .rows(rows).node(kit)
        } else {
            let level = self.meter.as_ref().map(Meter::level).unwrap_or(0.0);
            rows.push(line(t!("settings-audio-level")).sub(t!("settings-audio-level-about"))
                .control(level_bar(level)).node(kit));
            card.about(t!("settings-audio-in-about"))
                .rows(rows).node(kit)
        }
    }
}

fn level_bar<M>(level: f32) -> Node<M> {
    drawing((260.0, 10.0), "", move |canvas, pen| {
        let (w, h) = (canvas.width() as f32, canvas.height() as f32);
        let end = h / 2.0;
        let stroke = |canvas: &mut PixmapMut, to: f32, colour: Colour| {
            let mut pb = PathBuilder::new();
            pb.move_to(end, h / 2.0);
            pb.line_to(to.max(end + 0.01), h / 2.0);
            if let Some(path) = pb.finish() {
                canvas.stroke_path(&path, &brush(colour), &Stroke { width: h, line_cap: LineCap::Round, ..Stroke::default() },
                                   Transform::identity(), None);
            }
        };
        stroke(canvas, w - end, theme::alpha(pen.theme.fg, 0.10));
        if level > 0.005 {
            stroke(canvas, end + (w - 2.0 * end) * level.clamp(0.0, 1.0), theme::calm());
        }
    })
}

impl Page for Audio {
    fn id(&self) -> &'static str {
        "audio"
    }
    fn title(&self) -> &'static str {
        t!("settings-audio-title")
    }
    fn icon(&self) -> &'static str {
        "audio-volume-high-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-audio-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-audio-help")
    }

    fn shown(&mut self, kit: &mut Kit) {
        self.load(kit);
    }

    fn hidden(&mut self, _kit: &mut Kit) {
        self.loads += 1;
        self.stop_meter();
    }

    fn moving(&self) -> bool {
        self.meter.is_some()
    }

    fn view(&self, kit: &Kit) -> Node<Msg> {
        let (words, _) = self.said();
        let mut items = vec![headline(kit, words)];
        match self.state {
            State::Missing => items.push(missing(kit, "PipeWire", "pipewire-audio")),
            State::Failed => items.push(group(t!("settings-audio-failed-title")).icon("audio-speakers-symbolic")
                .rows(vec![line(t!("settings-audio-failed-heading"))
                    .sub(t!("settings-audio-failed-about")).key(ASKED).node(kit)])
                .node(kit)),
            State::Waiting => {}
            _ => items.extend(self.sections.iter().enumerate().map(|(at, s)| self.section_card(kit, at, s))),
        }
        crate::settings::ui::add_no_driver(kit, &mut items, crate::common::no_driver::Class::Sound);
        page(kit, self.title(), self.help(), items)
    }
}

#[cfg(test)]
impl Audio {
    pub(crate) fn made_up() -> Audio {
        let section = |title: &'static str, device: &str, pct: f32| Section {
            title, target: "", nodes: vec![(1, device.to_string()), (2, "HDMI".to_string())], current: Some(1),
            device: device.to_string(), pct, ready: true, told: true, real: (pct, false), ..Section::default()
        };
        Audio { state: State::Found { no_output: false },
                sections: vec![section("Output", "Speakers", 62.0), section("Input", "Microphone", 80.0)],
                meter: None, loads: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::ui::Do;
    use crate::settings::Settings;
    use crate::toolkit::event::Loop;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App as _, Ui};

    fn section(title: &'static str, device: &str, pct: f32) -> Section {
        Section { title, target: "", nodes: vec![(1, device.to_string()), (2, "HDMI".to_string())], current: Some(1),
                  device: device.to_string(), pct, ready: true, told: true, real: (pct, false), ..Section::default() }
    }

    #[test]
    #[ignore]
    fn shoot_with_made_up_devices() {
        let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));
        let size = std::env::var("TRINDLE_SHOT_SIZE").unwrap_or_else(|_| "1280x800".into());
        let (w, h) = size.split_once('x').and_then(|(a, b)| Some((a.parse::<f32>().ok()?, b.parse::<f32>().ok()?))).unwrap_or((1280.0, 800.0));
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let a = Audio { state: State::Found { no_output: false },
                        sections: vec![section("Output", "Speakers", 62.0), section("Input", "Microphone", 80.0)],
                        meter: None, loads: 0 };
        let mut kit = Kit::new(events.clone(), asks, None, None);
        kit.current = "audio".into();
        let s = Settings { kit, pages: vec![Box::new(a)], current: 0, search: String::new(), tall: h };
        let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
        let mut pix = tiny_skia::Pixmap::new(w as u32, h as u32).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        std::fs::create_dir_all(&out).unwrap();
        pix.save_png(out.join(format!("audio-{size}.png"))).unwrap();
    }

    #[test]
    fn the_sentence_names_bluetooth_and_a_plain_output_says_its_kind() {
        let dump = crate::common::audio::parse_dump(r#"[
          { "id": 30, "type": "PipeWire:Interface:Metadata", "props": { "metadata.name": "default" },
            "metadata": [ { "key": "default.audio.sink", "value": { "name": "bluez_output.AA.1" } } ] },
          { "id": 42, "type": "PipeWire:Interface:Node", "info": { "props": { "media.class": "Audio/Sink",
            "node.name": "alsa_output.pci.analog-stereo", "node.description": "Built-in", "device.bus": "pci" } } },
          { "id": 43, "type": "PipeWire:Interface:Node", "info": { "props": { "media.class": "Audio/Sink",
            "node.name": "bluez_output.AA.1", "device.api": "bluez5", "node.description": "Sony headphones" } } }
        ]"#).unwrap();
        let found = crate::common::audio::parse_nodes(&dump);
        let current = found.current_output().unwrap();
        assert_eq!(with_kind(&current.label, current.kind), "Sony headphones (Bluetooth)");
        assert_eq!(found.speakers().map(|n| with_kind(&n.label, n.kind)).as_deref(), Some("Built-in (speakers)"));
        assert_eq!(with_kind("HDMI", Kind::Hdmi), "HDMI", "a name that holds the kind is not doubled");
        assert_eq!(with_kind("Odd box", Kind::Other), "Odd box");
        let mut a = Audio::made_up();
        a.sections[0].device = with_kind(&current.label, current.kind);
        assert_eq!(a.said().0, "Sound plays through Sony headphones (Bluetooth) at 62%.");
    }

    #[test]
    fn a_refused_volume_or_mute_goes_back_and_says_why() {
        use crate::settings::ui::stand::Stand;
        let mut s = Stand::new(Box::new(Audio::made_up()));
        let now = |s: &mut Stand| s.page::<Audio, _>(|a, _| (a.sections[0].pct, a.sections[0].muted));
        let answer = s.page::<Audio, _>(|a, k| {
            a.sections[0].muted = true;
            a.changed(k, 0, &mute_key(0), no_mute(), |s| s.real.1 = true)
        });
        assert_eq!(now(&mut s), (62.0, true), "it looks muted while the answer is awaited");
        answer(ran(false, "Could not connect to PipeWire"));
        s.turn();
        assert_eq!(now(&mut s), (62.0, false), "a switch that failed does not stay looking switched");
        assert_eq!(s.red(&mute_key(0)).as_deref(), Some(no_mute()));
        let answer = s.page::<Audio, _>(|a, k| {
            a.sections[0].pct = 10.0;
            a.changed(k, 0, &volume_key(0), no_volume(), |s| s.real.0 = 10.0)
        });
        answer(ran(false, ""));
        s.turn();
        assert_eq!(now(&mut s), (62.0, false));
        assert_eq!(s.red(&volume_key(0)).as_deref(), Some(no_volume()));
        let answer = s.page::<Audio, _>(|a, k| {
            a.sections[0].pct = 30.0;
            a.changed(k, 0, &volume_key(0), no_volume(), |s| s.real.0 = 30.0)
        });
        answer(ran(true, ""));
        s.turn();
        assert_eq!(now(&mut s), (30.0, false));
        assert_eq!(s.red(&volume_key(0)), None);
        assert_eq!(s.page::<Audio, _>(|a, _| a.sections[0].real), (30.0, false));
        for words in [no_volume(), no_mute(), no_unmute()] {
            assert!(!words.to_lowercase().contains("wpctl") && !words.contains("exit"), "{words}");
        }
    }
}
