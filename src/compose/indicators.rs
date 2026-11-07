// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.
//! The dots in the middle of the top edge that say a program is using the
//! microphone (orange), the camera (green) or recording the screen (blue).
//! They are drawn by the compositor itself over everything, full-screen
//! windows and the lock screen too, take no clicks, and show in screenshots.
//!
//! Who is using what is found three ways: PipeWire's streams, through one
//! `pw-dump --monitor` that says when anything changes; the processes holding
//! a camera device open, looked for when the kernel says one was opened or
//! closed; and image capture, for the programs copying the screen over and over.
//! What was found is left in a small file for the ring's window list.
//! Resting the pointer on the dots shows, just under them, which program
//! is using each; the dots still take no clicks.
//!
//! The microphone switch in the ring is kept here too: while it is off,
//! every microphone PipeWire has is muted, and muted again the moment any
//! program unmutes it or a new one is plugged in.

use std::fmt::Write as _;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::CString;
use std::io::BufReader;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde_json::Value;
use smithay::{
    backend::{
        allocator::Fourcc,
        renderer::{
            element::{
                memory::{MemoryRenderBuffer, MemoryRenderBufferRenderElement},
                Kind as ElementKind,
            },
            ImportAll, ImportMem, Renderer,
        },
    },
    output::Output,
    reexports::calloop::timer::{TimeoutAction, Timer},
    utils::{Logical, Point, Rectangle, Size, Transform},
};
use tracing::warn;

use crate::in_use::{self, Kind, User};
use crate::render::CustomRenderElements;
use crate::shell::ssd::{self, Paint};
use crate::state::{AnvilState, Backend};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Metrics {
    pub dot: i32,
    pub gap: i32,
    pub margin: i32,
    pub edge: f32,
    pub edge_strength: f32,
    pub reach: i32,
    pub pad: i32,
}

impl Default for Metrics {
    fn default() -> Self {
        Metrics { dot: 14, gap: 8, margin: 10, edge: 2.0, edge_strength: 0.55, reach: 10, pad: 8 }
    }
}

impl Metrics {
    pub fn now() -> Metrics {
        Metrics::from_layers(&crate::common::sheet::layers(), crate::common::sheet::asked_light())
    }

    pub(crate) fn from_layers(layers: &crate::common::sheet::Layers, light: bool) -> Metrics {
        let d = Metrics::default();
        let n = |name: &str, was: f32, least: f32, most: f32| layers.number(name, light, least, most).unwrap_or(was);
        Metrics {
            dot: n("trindle-dots-size", d.dot as f32, 8.0, 24.0).round() as i32,
            gap: n("trindle-dots-gap", d.gap as f32, 2.0, 16.0).round() as i32,
            margin: n("trindle-dots-margin", d.margin as f32, 0.0, 24.0).round() as i32,
            edge: n("trindle-dots-edge", d.edge, 0.0, 3.0),
            edge_strength: n("trindle-dots-edge-strength", d.edge_strength, 0.0, 1.0),
            reach: n("trindle-dots-reach", d.reach as f32, 0.0, 20.0).round() as i32,
            pad: n("trindle-dots-label-padding", d.pad as f32, 4.0, 16.0).round() as i32,
        }
    }
}

const TICK: Duration = Duration::from_millis(500);

const LINE: f32 = ssd::FONT_PX * 1.6;

static MASK: AtomicU8 = AtomicU8::new(0);
static LINES: Mutex<Vec<String>> = Mutex::new(Vec::new());
static POINTER: Mutex<Option<Point<f64, Logical>>> = Mutex::new(None);
static MICROPHONE_OFF: AtomicBool = AtomicBool::new(false);

type LabelKey = (Vec<String>, u64, [[f32; 4]; 3], i32, Metrics);

static HOLDER: Mutex<Option<String>> = Mutex::new(None);

pub fn capture_changed(who: Option<String>) {
    let held = who.is_some();
    *HOLDER.lock().unwrap_or_else(|e| e.into_inner()) = who;
    match held {
        true => MASK.fetch_or(Kind::Input.bit(), Ordering::Relaxed),
        false => MASK.fetch_and(!Kind::Input.bit(), Ordering::Relaxed),
    };
}

fn holder_user() -> Option<User> {
    HOLDER.lock().unwrap_or_else(|e| e.into_inner()).clone().map(|who| User { kind: Kind::Input, pid: None, name: who, app_ids: Vec::new() })
}

pub(crate) fn shown_mask(users: &[User], held: bool) -> u8 {
    in_use::mask(users) | if held { Kind::Input.bit() } else { 0 }
}

thread_local! {
    static PAINTED: RefCell<HashMap<(u8, u64, i32, i32), Overlay>> = RefCell::new(HashMap::new());
    static PAINTED_WITH: RefCell<Option<Metrics>> = const { RefCell::new(None) };
    static LABEL: RefCell<Option<(LabelKey, Overlay)>> = const { RefCell::new(None) };
}

pub(crate) fn pixels(logical: i32, scale: f64) -> i32 {
    (logical as f64 * scale).round() as i32
}

pub(crate) fn pixel_size(logical: Size<i32, Logical>, scale: f64) -> (i32, i32) {
    (pixels(logical.w, scale), pixels(logical.h, scale))
}

#[derive(Clone)]
pub(crate) struct Overlay {
    buffer: MemoryRenderBuffer,
    px: Size<i32, Logical>,
    pub logical: Size<i32, Logical>,
}

impl Overlay {
    pub(crate) fn new(bytes: &[u8], logical: Size<i32, Logical>, scale: f64) -> Overlay {
        let px: Size<i32, Logical> = pixel_size(logical, scale).into();
        let want = (px.w.max(0) * px.h.max(0) * 4) as usize;
        debug_assert_eq!(bytes.len(), want);
        let fitted;
        let bytes = if bytes.len() == want {
            bytes
        } else {
            fitted = vec![0u8; want];
            &fitted[..]
        };
        let buffer = MemoryRenderBuffer::from_slice(bytes, Fourcc::Argb8888, (px.w, px.h), 1, Transform::Normal, None);
        Overlay { buffer, px, logical }
    }

    pub(crate) fn src(&self) -> Rectangle<f64, Logical> {
        Rectangle::from_size(self.px.to_f64())
    }

    pub(crate) fn element<R>(&self, renderer: &mut R, at: (f64, f64)) -> Option<CustomRenderElements<R>>
    where
        R: Renderer + ImportAll + ImportMem,
        R::TextureId: Clone + Send + 'static,
    {
        MemoryRenderBufferRenderElement::from_buffer(renderer, at, &self.buffer, None, Some(self.src()), Some(self.logical), ElementKind::Unspecified)
            .ok()
            .map(CustomRenderElements::Dots)
    }
}

pub(crate) fn kept<K: PartialEq>(slot: &RefCell<Option<(K, Overlay)>>, key: K, paint: impl FnOnce(&K) -> Overlay) -> Overlay {
    let mut slot = slot.borrow_mut();
    match slot.as_ref() {
        Some((k, overlay)) if *k == key => overlay.clone(),
        _ => {
            let overlay = paint(&key);
            *slot = Some((key, overlay.clone()));
            overlay
        }
    }
}

pub fn pointer_at(at: Point<f64, Logical>) {
    if let Ok(mut p) = POINTER.lock() {
        *p = Some(at);
    }
}

pub(crate) fn pointer() -> Option<Point<f64, Logical>> {
    POINTER.lock().ok().and_then(|p| *p)
}

pub(crate) fn outside_round_box(x: f32, y: f32, w: f32, h: f32, r: f32) -> f32 {
    let qx = (x - w / 2.0).abs() - (w / 2.0 - r);
    let qy = (y - h / 2.0).abs() - (h / 2.0 - r);
    (qx.max(0.0).powi(2) + qy.max(0.0).powi(2)).sqrt() + qx.max(qy).min(0.0) - r
}

fn paint_label(lines: &[String], w: i32, h: i32, s: f32, m: &Metrics) -> Vec<u8> {
    let (theme, radius) = ssd::theme();
    let c = theme.active;
    let (pw, ph) = (w as usize, h as usize);
    let mut px = vec![Paint::CLEAR; pw * ph];
    let r = (radius as f32 * s).min(h as f32 / 2.0);
    let edge = s.max(1.0);
    for y in 0..ph {
        for x in 0..pw {
            let (fx, fy) = (x as f32 + 0.5, y as f32 + 0.5);
            let outer = 0.5 - outside_round_box(fx, fy, w as f32, h as f32, r);
            let inner = 0.5 - outside_round_box(fx - edge, fy - edge, w as f32 - 2.0 * edge, h as f32 - 2.0 * edge, (r - edge).max(0.0));
            px[y * pw + x] = Paint::of(c.background, inner).over(Paint::of(c.border, outer));
        }
    }
    if let Some(font) = ssd::face(400.0) {
        let size = ab_glyph::PxScale::from(ssd::FONT_PX * s);
        for (i, line) in lines.iter().enumerate() {
            let text: Vec<char> = line.chars().collect();
            let x = w as f32 - m.pad as f32 * s - ssd::chars_width(&font, size, &text);
            let middle = (m.pad as f32 + LINE * (i as f32 + 0.5)) * s;
            let baseline = (middle + 0.36 * size.y).round();
            ssd::draw_chars(&mut px, pw, ph, &font, size, &text, c.text, x, baseline);
        }
    }
    ssd::to_bytes(&px)
}

pub(crate) fn label(lines: Vec<String>, scale: f64) -> Overlay {
    let (theme, radius) = ssd::theme();
    let c = theme.active;
    let m = Metrics::now();
    let key: LabelKey = (lines, scale.to_bits(), [c.background, c.border, c.text], radius, m);
    LABEL.with(|l| {
        kept(l, key, |key| {
            let widest = ssd::face(400.0).map_or(0.0, |font| {
                let size = ab_glyph::PxScale::from(ssd::FONT_PX);
                key.0.iter().map(|t| ssd::chars_width(&font, size, &t.chars().collect::<Vec<_>>())).fold(0.0, f32::max)
            });
            let logical: Size<i32, Logical> = ((widest.ceil() as i32) + 2 * m.pad, (LINE * key.0.len() as f32).ceil() as i32 + 2 * m.pad).into();
            let (w, h) = pixel_size(logical, scale);
            let bytes = paint_label(&key.0, w, h, scale as f32, &m);
            Overlay::new(&bytes, logical, scale)
        })
    })
}

pub(crate) fn label_at(screen_width: i32, logical: Size<i32, Logical>, scale: f64, m: &Metrics) -> (f64, f64) {
    let w = pixels(logical.w, scale) as f64;
    (((screen_width as f64 - w) / 2.0).round().max(0.0), ((m.margin + m.dot + m.gap) as f64 * scale).round())
}

fn kinds(mask: u8) -> Vec<Kind> {
    Kind::ALL.into_iter().filter(|k| mask & k.bit() != 0).collect()
}

fn paint(kinds: &[Kind], w: i32, h: i32, s: f32, m: &Metrics) -> Vec<u8> {
    let mut bytes = vec![0u8; (w * h * 4) as usize];
    let r = m.dot as f32 / 2.0 * s;
    let edge_ink = [0.0, 0.0, 0.0, m.edge_strength];
    for (i, kind) in kinds.iter().enumerate() {
        let (cr, cg, cb) = kind.colour();
        let fill = [cr as f32, cg as f32, cb as f32, 1.0];
        let cx = w as f32 - (i as f32 * (m.dot + m.gap) as f32 * s + r);
        let cy = h as f32 / 2.0;
        for y in 0..h {
            for x in 0..w {
                let d = ((x as f32 + 0.5 - cx).powi(2) + (y as f32 + 0.5 - cy).powi(2)).sqrt();
                let within = |radius: f32| (0.5 - (d - radius)).clamp(0.0, 1.0);
                let outer = within(r);
                if outer <= 0.0 {
                    continue;
                }
                let inner = within(r - m.edge * s);
                let rim = outer - inner;
                let a = fill[3] * inner + edge_ink[3] * rim;
                let mix = |c: usize| fill[c] * fill[3] * inner + edge_ink[c] * edge_ink[3] * rim;
                let byte = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
                let at = ((y * w + x) * 4) as usize;
                bytes[at..at + 4].copy_from_slice(&[byte(mix(2)), byte(mix(1)), byte(mix(0)), byte(a)]);
            }
        }
    }
    bytes
}

const PAINTED_MOST: usize = 64;

#[derive(Debug, PartialEq)]
pub(crate) struct Placed {
    pub kinds: Vec<Kind>,
    pub logical: Size<i32, Logical>,
    pub w: i32,
    pub h: i32,
    pub at: (f64, f64),
}

pub(crate) fn placed(mask: u8, screen_width: i32, scale: f64, m: &Metrics) -> Option<Placed> {
    let kinds = kinds(mask);
    if kinds.is_empty() {
        return None;
    }
    let n = kinds.len() as i32;
    let logical: Size<i32, Logical> = (n * m.dot + (n - 1) * m.gap, m.dot).into();
    let (w, h) = pixel_size(logical, scale);
    let margin = (m.margin as f64 * scale).round();
    let x = ((screen_width as f64 - w as f64) / 2.0).round().max(0.0);
    Some(Placed { kinds, logical, w, h, at: (x, margin) })
}

pub(crate) fn screen_width(mode_width: Option<i32>, place_width: Option<i32>, scale: f64) -> Option<i32> {
    mode_width.or_else(|| place_width.map(|w| (w as f64 * scale).round() as i32)).filter(|w| *w > 0)
}

pub(crate) fn points_at_dots(px: f64, py: f64, screen_w: i32, dots_w: i32, m: &Metrics) -> bool {
    let left = (screen_w - dots_w) as f64 / 2.0;
    px >= left - m.reach as f64 && px <= left + (dots_w + m.reach) as f64 && py <= (m.margin + m.dot + m.reach) as f64
}

pub fn elements<R>(renderer: &mut R, output: &Output, place: Option<Rectangle<i32, Logical>>) -> Vec<CustomRenderElements<R>>
where
    R: Renderer + ImportAll + ImportMem,
    R::TextureId: Clone + Send + 'static,
{
    let mask = MASK.load(Ordering::Relaxed);
    let scale = output.current_scale().fractional_scale();
    let m = Metrics::now();
    let mode_width = output.current_mode().map(|mode| output.current_transform().transform_size(mode.size).w);
    let Some(screen_width) = screen_width(mode_width, place.map(|p| p.size.w), scale) else { return Vec::new() };
    let Some(spot) = placed(mask, screen_width, scale, &m) else { return Vec::new() };
    let Placed { kinds, logical, w, h, at } = spot;
    let key = (mask, scale.to_bits(), w, h);
    let dots = PAINTED.with(|p| {
        let mut p = p.borrow_mut();
        if PAINTED_WITH.with(|w| w.replace(Some(m))) != Some(m) || p.len() > PAINTED_MOST {
            p.clear();
        }
        p.entry(key).or_insert_with(|| Overlay::new(&paint(&kinds, w, h, scale as f32, &m), logical, scale)).clone()
    });
    let mut out: Vec<_> = dots.element(renderer, at).into_iter().collect();
    let pointer = POINTER.lock().ok().and_then(|p| *p);
    let Some((place, pointer)) = place.zip(pointer) else { return out };
    if !place.to_f64().contains(pointer) {
        return out;
    }
    let (px, py) = (pointer.x - place.loc.x as f64, pointer.y - place.loc.y as f64);
    let pointing = points_at_dots(px, py, place.size.w, logical.w, &m);
    let lines = LINES.lock().map(|l| l.clone()).unwrap_or_default();
    if !pointing || lines.is_empty() {
        return out;
    }
    let label = label(lines, scale);
    out.extend(label.element(renderer, label_at(screen_width, label.logical, scale, &m)));
    out
}

#[derive(Default)]
struct Found {
    pipewire: Vec<User>,
    cameras: Vec<(u32, String)>,
    microphones: Vec<in_use::Source>,
}

impl<BackendData: Backend> AnvilState<BackendData> {
    pub fn watch_in_use(&mut self) {
        if crate::session::launch().greeter {
            return;
        }
        let file = in_use::path();
        if let Err(e) = in_use::write(&file, &[]) {
            warn!("in use: cannot write {}: {e}", file.display());
        }
        let fake = std::env::var("TRINDLE_FAKE_IN_USE").ok().filter(|s| !s.is_empty());
        let found = Arc::new(Mutex::new(Found::default()));
        MICROPHONE_OFF.store(in_use::microphone_off(), Ordering::Relaxed);
        let (wake, woken) = channel::<()>();
        if fake.is_none() {
            let (to, tell) = (found.clone(), wake.clone());
            let _ = std::thread::Builder::new().name("in-use-pipewire".into()).spawn(move || pipewire(&to, &tell));
            let to = found.clone();
            let _ = std::thread::Builder::new().name("in-use-camera".into()).spawn(move || cameras(&to));
            let to = found.clone();
            let _ = std::thread::Builder::new().name("microphone-switch".into()).spawn(move || microphone_switch(&to, woken));
        }
        let mut last: Vec<User> = Vec::new();
        let mut last_off = false;
        let _ = self.handle.insert_source(Timer::from_duration(TICK), move |_, _, state| {
            let off = in_use::microphone_off();
            if MICROPHONE_OFF.swap(off, Ordering::Relaxed) != off || off {
                let _ = wake.send(());
            }
            let mut users = match &fake {
                Some(spec) => in_use::fake(spec),
                None => state.gather(&found),
            };
            if !users.is_empty() {
                let owners = state.window_owners();
                for u in users.iter_mut().filter(|u| u.app_ids.is_empty()) {
                    u.app_ids = in_use::match_windows(u.pid, &u.name, &owners, in_use::parent_of);
                }
            }
            let held = HOLDER.lock().unwrap_or_else(|e| e.into_inner()).is_some();
            MASK.store(shown_mask(&users, held), Ordering::Relaxed);
            if users != last || off != last_off {
                if let Ok(mut l) = LINES.lock() {
                    *l = in_use::lines(&users, off);
                }
                last_off = off;
                if let Err(e) = in_use::write(&file, &users) {
                    warn!("in use: cannot write {}: {e}", file.display());
                }
                last = std::mem::take(&mut users);
            }
            TimeoutAction::ToDuration(TICK)
        });
    }

    fn gather(&mut self, found: &Mutex<Found>) -> Vec<User> {
        let mut users = Vec::new();
        {
            let found = found.lock().unwrap_or_else(|e| e.into_inner());
            users.extend(found.pipewire.iter().cloned());
            for (pid, name) in &found.cameras {
                if !users.iter().any(|u| u.kind == Kind::Camera && u.pid == Some(*pid)) {
                    users.push(User { kind: Kind::Camera, pid: Some(*pid), name: name.clone(), app_ids: Vec::new() });
                }
            }
        }
        users.extend(holder_user());
        let mut recording = self.image_capture_state.recording();
        recording.sort_unstable();
        recording.dedup();
        let casts = std::fs::read_to_string(in_use::casts_path()).ok().and_then(|t| in_use::decode_casts(&t));
        let comm = |pid: u32| std::fs::read_to_string(format!("/proc/{pid}/comm")).ok().map(|c| c.trim().to_string());
        users.extend(in_use::screen_users(&recording, casts, comm));
        users
    }
}

pub(crate) fn forget_after(produced: bool) -> bool {
    !produced
}

fn pipewire(found: &Mutex<Found>, wake: &Sender<()>) {
    const FIRST: Duration = Duration::from_secs(2);
    const MOST: Duration = Duration::from_secs(60);
    let mut wait = FIRST;
    loop {
        let started = Instant::now();
        let mut cmd = Command::new("pw-dump");
        cmd.args(["--monitor", "--no-colors"]).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null());
        ends_with_us(&mut cmd);
        let mut produced = false;
        if let Ok(mut child) = cmd.spawn() {
            if let Some(out) = child.stdout.take() {
                let mut pw = in_use::Pw::default();
                for v in serde_json::Deserializer::from_reader(BufReader::new(out)).into_iter::<Value>() {
                    let Ok(v) = v else { break };
                    pw.feed(&v);
                    produced = true;
                    let users = pw.users(|pid| in_use::is_middleman(Path::new("/proc"), pid));
                    {
                        let mut f = found.lock().unwrap_or_else(|e| e.into_inner());
                        f.pipewire = users;
                        f.microphones = pw.sources();
                    }
                    let _ = wake.send(());
                }
            }
            let _ = child.kill();
            let _ = child.wait();
        }
        if forget_after(produced) {
            let mut f = found.lock().unwrap_or_else(|e| e.into_inner());
            f.pipewire.clear();
            f.microphones.clear();
        }
        if started.elapsed() > MOST {
            wait = FIRST;
        }
        std::thread::sleep(wait);
        wait = (wait * 2).min(MOST);
    }
}

fn microphone_switch(found: &Mutex<Found>, woken: Receiver<()>) {
    let list = in_use::microphone_muted_list();
    let mut switch = in_use::MicSwitch::default();
    switch.ours = std::fs::read_to_string(&list).unwrap_or_default().lines().filter(|l| !l.is_empty()).map(String::from).collect();
    let mut saved = switch.ours.clone();
    while woken.recv().is_ok() {
        while woken.try_recv().is_ok() {}
        let microphones = found.lock().unwrap_or_else(|e| e.into_inner()).microphones.clone();
        let off = MICROPHONE_OFF.load(Ordering::Relaxed);
        for (id, mute) in switch.plan(off, &microphones, Instant::now()) {
            let mut cmd = Command::new("wpctl");
            cmd.args(["set-mute", &id.to_string(), if mute { "1" } else { "0" }])
                .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
            ends_with_us(&mut cmd);
            match cmd.status() {
                Ok(s) if s.success() => {}
                Ok(s) => warn!("microphone switch: wpctl set-mute {id} ended with {s}"),
                Err(e) => warn!("microphone switch: cannot run wpctl: {e}"),
            }
        }
        if switch.ours != saved {
            let mut names: Vec<&String> = switch.ours.iter().collect();
            names.sort();
            let text: String = names.iter().fold(String::new(), |mut s, n| { let _ = writeln!(s, "{n}"); s });
            let wrote = list.parent().map_or(Ok(()), std::fs::create_dir_all).and_then(|_| crate::common::files::write_whole(&list, text.as_bytes()));
            if let Err(e) = wrote {
                warn!("microphone switch: cannot write {}: {e}", list.display());
            }
            saved = switch.ours.clone();
        }
    }
}

fn ends_with_us(cmd: &mut Command) {
    use std::os::unix::process::CommandExt;
    // SAFETY: between fork and exec the hook only calls prctl, which is safe
    // to call there and touches no memory of ours.
    unsafe {
        cmd.pre_exec(|| {
            libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGTERM);
            Ok(())
        });
    }
}

fn camera_devices() -> Vec<std::path::PathBuf> {
    let Ok(dir) = std::fs::read_dir("/dev") else { return Vec::new() };
    dir.flatten().map(|e| e.path()).filter(|p| in_use::is_video_link(p)).collect()
}

fn cameras(found: &Mutex<Found>) {
    const CAMERA: u32 = libc::IN_OPEN | libc::IN_CLOSE;
    const DEVICES: u32 = libc::IN_CREATE | libc::IN_DELETE;
    const SLOW: Duration = Duration::from_secs(3);
    // SAFETY: no pointers; the result is checked before it is owned.
    let fd = unsafe { libc::inotify_init1(libc::IN_CLOEXEC) };
    // SAFETY: fd is a fresh descriptor nothing else owns.
    let notify = (fd >= 0).then(|| unsafe { OwnedFd::from_raw_fd(fd) });
    let watch = |path: &Path, mask: u32| {
        let Some(notify) = &notify else { return };
        let Ok(name) = CString::new(path.as_os_str().as_bytes()) else { return };
        // SAFETY: name is a NUL-terminated path that lives through the call.
        unsafe { libc::inotify_add_watch(notify.as_raw_fd(), name.as_ptr(), mask) };
    };
    watch(Path::new("/dev"), DEVICES);
    let mut buf = [0u8; 4096];
    loop {
        for dev in camera_devices() {
            watch(&dev, CAMERA);
        }
        let holders = in_use::video_holders(Path::new("/proc"));
        found.lock().unwrap_or_else(|e| e.into_inner()).cameras = holders;
        let Some(notify) = &notify else {
            std::thread::sleep(SLOW);
            continue;
        };
        // SAFETY: buf is writable for its whole length; the read waits for
        // the kernel's next news.
        let n = unsafe { libc::read(notify.as_raw_fd(), buf.as_mut_ptr().cast(), buf.len()) };
        if n <= 0 {
            std::thread::sleep(SLOW);
            continue;
        }
        std::thread::sleep(Duration::from_millis(200));
        let mut ready = libc::pollfd { fd: notify.as_raw_fd(), events: libc::POLLIN, revents: 0 };
        // SAFETY: ready is one pollfd that lives through each call, and the
        // read only happens when poll says it will not wait.
        while unsafe { libc::poll(&mut ready, 1, 0) } > 0 {
            // SAFETY: as above, buf is writable for its whole length.
            if unsafe { libc::read(notify.as_raw_fd(), buf.as_mut_ptr().cast(), buf.len()) } <= 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
pub(crate) fn every_scale() -> Vec<f64> {
    let quarters = (4..=12).map(|q| q as f64 / 4.0);
    (20..=160).map(|i| i as f64 / 40.0).chain(quarters.flat_map(|b| [b * 0.9, b * 1.1])).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alpha_at(px: &[u8], w: i32, x: i32, y: i32) -> u8 {
        px[((y * w + x) * 4 + 3) as usize]
    }

    #[test]
    fn the_label_is_a_filled_box_with_words_in_it() {
        let lines = vec!["Microphone in use by Firefox".to_string()];
        let m = Metrics::default();
        let (w, h) = (220, (LINE.ceil() as i32) + 2 * m.pad);
        let px = paint_label(&lines, w, h, 1.0, &m);
        assert_eq!(px.len(), (w * h * 4) as usize);
        assert_eq!(alpha_at(&px, w, 0, 0), 0);
        assert_eq!(alpha_at(&px, w, w / 2, 3), 255);
        let plain = &px[((3 * w + w / 2) * 4) as usize..((3 * w + w / 2) * 4 + 3) as usize];
        let row = h / 2;
        let inked = (0..w).filter(|&x| &px[((row * w + x) * 4) as usize..((row * w + x) * 4 + 3) as usize] != plain).count();
        assert!(inked > 20, "{inked}");
        assert!(outside_round_box(5.0, 5.0, 10.0, 10.0, 2.0) < 0.0 && outside_round_box(0.1, 0.1, 10.0, 10.0, 4.0) > 0.0);
    }

    #[test]
    fn dots_from_the_corner_inwards() {
        assert_eq!(kinds(0), Vec::<Kind>::new());
        assert_eq!(kinds(1 | 4), vec![Kind::Microphone, Kind::Screen]);
        assert_eq!(kinds(8 | 1), vec![Kind::Microphone, Kind::Input]);
        let Metrics { dot, gap, .. } = Metrics::default();
        let w = 2 * dot + gap;
        let px = paint(&[Kind::Microphone, Kind::Screen], w, dot, 1.0, &Metrics::default());
        let right = ((dot / 2 * w + w - dot / 2) * 4) as usize;
        assert_eq!(px[right + 3], 255);
        assert!(px[right + 2] > 200 && px[right] < 40, "orange is mostly red and green, no blue");
        let left = ((dot / 2 * w + dot / 2) * 4) as usize;
        assert!(px[left] > 200, "the screen dot is blue");
        assert_eq!(alpha_at(&px, w, dot + gap / 2, dot / 2), 0);
        assert_eq!(alpha_at(&px, w, 0, 0), 0);
    }

    #[test]
    fn the_style_sheet_gives_the_dots_their_sizes() {
        use crate::common::sheet::{Layers, CSS};
        let d = Metrics::default();
        let l = Layers::new(CSS, "");
        let n = |name: &str| l.number(name, false, -1000.0, 1000.0).unwrap();
        assert_eq!(n("trindle-dots-size"), d.dot as f32);
        assert_eq!(n("trindle-dots-gap"), d.gap as f32);
        assert_eq!(n("trindle-dots-margin"), d.margin as f32);
        assert_eq!(n("trindle-dots-edge"), d.edge);
        assert_eq!(n("trindle-dots-edge-strength"), d.edge_strength);
        assert_eq!(n("trindle-dots-reach"), d.reach as f32);
        assert_eq!(n("trindle-dots-label-padding"), d.pad as f32);
        assert_eq!(Metrics::now(), d);
    }

    const SCALES: [f64; 6] = [1.0, 1.25, 1.5, 2.0, 2.5, 3.0];
    const WIDTHS: [i32; 5] = [800, 1366, 1920, 2560, 3840];

    #[test]
    fn the_dots_are_bigger_by_default_and_held_to_their_limits() {
        use crate::common::sheet::{Layers, CSS};
        let d = Metrics::default();
        assert_eq!((d.dot, d.gap, d.margin, d.edge), (14, 8, 10, 2.0));
        let with = |css: &str| Metrics::from_layers(&Layers::new(CSS, css), false);
        assert_eq!(with(":root { --trindle-dots-size: 2; }").dot, 8);
        assert_eq!(with(":root { --trindle-dots-size: 99; }").dot, 24);
        assert_eq!(with(":root { --trindle-dots-size: 20; }").dot, 20);
    }

    #[test]
    fn dots_are_whole_on_every_screen_at_every_scale_and_mix() {
        let m = Metrics::default();
        for mask in 1u8..=255 {
            let want = kinds(mask).len();
            for scale in SCALES {
                for width in WIDTHS {
                    let Some(p) = placed(mask, width, scale, &m) else {
                        assert_eq!(want, 0, "mask {mask}");
                        continue;
                    };
                    assert_eq!(p.kinds.len(), want);
                    assert_eq!(p.h, pixels(m.dot, scale));
                    assert!(p.at.0 >= 0.0 && p.at.0 + p.w as f64 <= width as f64, "mask {mask} scale {scale} width {width}");
                    let margin = (m.margin as f64 * scale).round();
                    assert_eq!(p.at.1, margin);
                    assert!((p.at.0 + p.w as f64 / 2.0 - width as f64 / 2.0).abs() <= 1.0, "centred");
                }
            }
        }
        assert!(placed(0, 1920, 1.0, &m).is_none());
    }

    #[test]
    fn every_screen_of_several_gets_its_dots() {
        let m = Metrics::default();
        let screens = [(1920, 1.0), (3840, 2.0), (1366, 1.25)];
        let mask = Kind::Microphone.bit() | Kind::Camera.bit();
        for (width, scale) in screens {
            let p = placed(mask, width, scale, &m).unwrap();
            assert!(p.at.0 > 0.0 && p.w > 0);
        }
    }

    #[test]
    fn a_screen_without_a_mode_yet_still_has_a_width() {
        assert_eq!(screen_width(Some(1920), Some(960), 2.0), Some(1920));
        assert_eq!(screen_width(None, Some(960), 2.0), Some(1920));
        assert_eq!(screen_width(None, Some(1280), 1.25), Some(1600));
        assert_eq!(screen_width(None, None, 1.0), None);
        assert_eq!(screen_width(Some(0), None, 1.0), None);
    }

    #[test]
    fn the_shown_mask_joins_every_source_whatever_came_last() {
        let user = |kind| User { kind, pid: Some(1), name: "Firefox".into(), app_ids: Vec::new() };
        let mic = vec![user(Kind::Microphone)];
        let all = vec![user(Kind::Microphone), user(Kind::Camera), user(Kind::Screen)];
        assert_eq!(shown_mask(&[], false), 0);
        assert_eq!(shown_mask(&mic, false), Kind::Microphone.bit());
        assert_eq!(shown_mask(&mic, true), Kind::Microphone.bit() | Kind::Input.bit());
        assert_eq!(shown_mask(&[], true), Kind::Input.bit());
        assert_eq!(shown_mask(&[user(Kind::Input)], false), Kind::Input.bit());
        for k in [Kind::Microphone, Kind::Camera, Kind::Screen, Kind::Input] {
            assert_ne!(shown_mask(&all, true) & k.bit(), 0, "{k:?}");
        }
    }

    #[test]
    fn the_mark_names_the_plugin_that_holds_the_keyboard_for_as_long_as_it_does() {
        capture_changed(Some("org.example.lantern".into()));
        let users: Vec<User> = holder_user().into_iter().collect();
        assert_eq!(in_use::lines(&users, false), ["org.example.lantern is using your keyboard and mouse"]);
        assert_ne!(shown_mask(&users, true) & Kind::Input.bit(), 0);
        assert_ne!(MASK.load(Ordering::Relaxed) & Kind::Input.bit(), 0, "the dot shows at once");
        capture_changed(None);
        assert!(holder_user().is_none() && in_use::lines(&holder_user().into_iter().collect::<Vec<_>>(), false).is_empty());
        assert_eq!(MASK.load(Ordering::Relaxed) & Kind::Input.bit(), 0, "the dot goes out at once");
    }

    #[test]
    fn a_hiccup_in_the_pipewire_monitor_does_not_hide_a_dot() {
        assert!(!forget_after(true));
        assert!(forget_after(false));
    }

    #[test]
    fn pointing_at_the_dots_reaches_at_least_24_points() {
        for dot in [8, 14, 24] {
            let m = Metrics { dot, ..Metrics::default() };
            let screen = 1920;
            let one = placed(Kind::Microphone.bit(), screen, 1.0, &m).unwrap();
            let (left, right) = ((screen - one.logical.w) as f64 / 2.0, (screen + one.logical.w) as f64 / 2.0);
            assert!(points_at_dots(screen as f64 / 2.0, (m.margin + dot / 2) as f64, screen, one.logical.w, &m));
            let across = dot + 2 * m.reach;
            assert!(across >= 24 || dot < 4, "{across}");
            assert!(!points_at_dots(left - m.reach as f64 - 1.0, 5.0, screen, one.logical.w, &m));
            assert!(!points_at_dots(right + m.reach as f64 + 1.0, 5.0, screen, one.logical.w, &m));
            assert!(!points_at_dots(screen as f64 / 2.0, (m.margin + dot + m.reach) as f64 + 1.0, screen, one.logical.w, &m));
        }
        let m = Metrics::default();
        let three = placed(7, 1920, 1.0, &m).unwrap();
        let far = (1920 - three.logical.w) as f64 / 2.0 + m.dot as f64 / 2.0;
        assert!(points_at_dots(far, 12.0, 1920, three.logical.w, &m));
    }

    #[test]
    fn a_bigger_dot_is_solid_in_the_middle_with_a_dark_edge_at_every_scale() {
        let m = Metrics::default();
        for scale in SCALES {
            let p = placed(Kind::Microphone.bit() | Kind::Camera.bit(), 1920, scale, &m).unwrap();
            let px = paint(&p.kinds, p.w, p.h, scale as f32, &m);
            for i in 0..2 {
                let cx = p.w - ((i as f64 * (m.dot + m.gap) as f64 + m.dot as f64 / 2.0) * scale).round() as i32;
                assert_eq!(alpha_at(&px, p.w, cx, p.h / 2), 255, "scale {scale} dot {i}");
            }
            let gap_x = p.w - ((m.dot as f64 + m.gap as f64 / 2.0) * scale).round() as i32;
            assert_eq!(alpha_at(&px, p.w, gap_x, p.h / 2), 0, "scale {scale}");
        }
    }

    #[test]
    fn the_dots_are_in_front_of_the_lock_screen_a_full_screen_window_and_the_ring() {
        let src = crate::source::code(include_str!("render.rs"));
        let body = &src[src.find("pub fn output_elements").unwrap()..];
        let overlays = body.find("with_overlays(custom_elements").unwrap();
        let lock = body.find("locked_elements(custom_elements").unwrap();
        let fullscreen = body.find("fullscreen_elements(custom_elements").unwrap();
        let desktop = body.find("desktop_elements(output").unwrap();
        assert!(overlays < lock && lock < fullscreen && fullscreen < desktop);
        let made = &body[body.find("fn with_overlays").unwrap()..];
        let dots = made.find("let dots = crate::indicators::elements").unwrap();
        let chain = made.find(".chain(dots)").unwrap();
        assert!(dots < chain);
        assert!(body.matches("custom_elements").count() >= 4);
        assert!(!body[..overlays].contains("return"));
        assert!(!made[..dots].contains("return"));
    }

    #[test]
    #[ignore]
    fn pictures_of_the_dots() {
        let Ok(dir) = std::env::var("TRINDLE_DOTS_PICTURES") else { return };
        std::fs::create_dir_all(&dir).unwrap();
        let old = Metrics { dot: 8, gap: 6, margin: 6, edge: 1.0, ..Metrics::default() };
        let now = Metrics::default();
        let grounds: [(&str, fn(i32, i32) -> [u8; 3]); 3] = [
            ("dark", |_, _| [0x16, 0x18, 0x1c]),
            ("light", |_, _| [0xf2, 0xf2, 0xf2]),
            ("busy", |x, y| if (x / 9 + y / 9) % 2 == 0 { [0x6a, 0x8f, 0x4e] } else { [0xd8, 0xc0, 0x8a] }),
        ];
        for (ground, colour) in grounds {
            for (which, m) in [("new", &now), ("old", &old)] {
                let p = placed(7, 200, 1.0, m).unwrap();
                let px = paint(&p.kinds, p.w, p.h, 1.0, m);
                let (cw, ch) = (200, 2 * m.margin + p.h);
                let mut img = image::RgbImage::new(cw as u32, ch as u32);
                for y in 0..ch {
                    for x in 0..cw {
                        let mut c = colour(x, y);
                        let (dx, dy) = (x - p.at.0 as i32, y - p.at.1 as i32);
                        if dx >= 0 && dy >= 0 && dx < p.w && dy < p.h {
                            let at = ((dy * p.w + dx) * 4) as usize;
                            let a = px[at + 3] as f32 / 255.0;
                            for (i, ch) in [2usize, 1, 0].into_iter().enumerate() {
                                c[i] = (px[at + ch] as f32 + c[i] as f32 * (1.0 - a)).round().min(255.0) as u8;
                            }
                        }
                        img.put_pixel(x as u32, y as u32, image::Rgb(c));
                    }
                }
                let big = image::imageops::resize(&img, img.width() * 6, img.height() * 6, image::imageops::FilterType::Nearest);
                big.save(format!("{dir}/dots-{which}-{ground}.png")).unwrap();
            }
        }
    }

    use smithay::backend::renderer::{element::Element, pixman::PixmanRenderer, utils::draw_render_elements, Bind, Color32F, ExportMem, Frame, Offscreen};
    use smithay::reexports::pixman::Image;
    use smithay::utils::{Buffer, Physical};

    const LAPTOP_SCALES: [f64; 5] = [1.0, 1.25, 1.5, 1.75, 2.0];

    fn on_screen(screen: (i32, i32), scale: f64, make: impl FnOnce(&mut PixmanRenderer) -> Vec<CustomRenderElements<PixmanRenderer>>) -> Vec<u8> {
        let mut r = PixmanRenderer::new().unwrap();
        let mut img: Image<'static, 'static> = r.create_buffer(Fourcc::Argb8888, Size::<i32, Buffer>::from(screen)).unwrap();
        let elements = make(&mut r);
        let whole = Rectangle::<i32, Physical>::from_size(screen.into());
        let mut fb = r.bind(&mut img).unwrap();
        {
            let mut frame = r.render(&mut fb, screen.into(), Transform::Normal).unwrap();
            frame.clear(Color32F::TRANSPARENT, &[whole]).unwrap();
            draw_render_elements::<PixmanRenderer, _, _>(&mut frame, scale, &elements, &[whole]).unwrap();
            frame.finish().unwrap().wait().unwrap();
        }
        let mapping = r.copy_framebuffer(&fb, Rectangle::from_size(screen.into()), Fourcc::Argb8888).unwrap();
        r.map_texture(&mapping).unwrap().to_vec()
    }

    const TWO_LINES: [&str; 2] = ["Microphone in use by lantern", "Camera in use by Firefox"];

    #[test]
    fn the_label_is_whole_and_on_the_screen_at_every_scale_and_at_the_right_edge() {
        let m = Metrics::now();
        let lines: Vec<String> = TWO_LINES.iter().map(|l| l.to_string()).collect();
        for scale in LAPTOP_SCALES {
            let label = label(lines.clone(), scale);
            let (w, h) = (pixels(label.logical.w, scale), pixels(label.logical.h, scale));
            let painted = paint_label(&lines, w, h, scale as f32, &m);
            let inked = painted.chunks(4).filter(|p| p[3] != 0).count();
            for sw in [w + 1, w + 7, 2256] {
                let at = label_at(sw, label.logical, scale, &m);
                assert!(at.0 >= 0.0 && at.0 as i32 + w <= sw, "scale {scale} screen {sw}");
                let sh = at.1 as i32 + h + 3;
                let shot = on_screen((sw, sh), scale, |r| label.element(r, at).into_iter().collect());
                for y in 0..h {
                    for x in 0..w {
                        let s = (((at.1 as i32 + y) * sw + at.0 as i32 + x) * 4) as usize;
                        let p = ((y * w + x) * 4) as usize;
                        for c in 0..4 {
                            assert!((shot[s + c] as i32 - painted[p + c] as i32).abs() <= 2, "scale {scale} screen {sw} at {x},{y}");
                        }
                    }
                }
                assert_eq!(shot.chunks(4).filter(|p| p[3] != 0).count(), inked, "scale {scale} screen {sw}");
            }
        }
    }

    #[test]
    fn every_overlay_shows_its_whole_buffer_at_its_exact_size() {
        let mut r = PixmanRenderer::new().unwrap();
        for scale in LAPTOP_SCALES.into_iter().chain([2.5, 3.0]) {
            for (lw, lh) in [(1, 1), (37, 11), (64, 24), (241, 61)] {
                let (w, h) = (pixels(lw, scale), pixels(lh, scale));
                let o = Overlay::new(&vec![0; (w * h * 4) as usize], (lw, lh).into(), scale);
                let Some(CustomRenderElements::Dots(e)) = o.element(&mut r, (10.0, 7.0)) else { panic!("no element") };
                assert_eq!(e.src(), Rectangle::<f64, Buffer>::from_size((w as f64, h as f64).into()), "scale {scale}");
                assert_eq!(e.geometry(scale.into()).size, (w, h).into(), "scale {scale}");
                let old = MemoryRenderBufferRenderElement::from_buffer(&mut r, (10.0, 7.0), &o.buffer, None, None, Some(o.logical), ElementKind::Unspecified).unwrap();
                assert_eq!(old.src() == e.src(), w == lw && h == lh, "scale {scale}");
            }
        }
    }

    #[test]
    #[ignore]
    fn pictures_of_the_label() {
        let Ok(dir) = std::env::var("TRINDLE_LABEL_PICTURES") else { return };
        std::fs::create_dir_all(&dir).unwrap();
        let m = Metrics::now();
        let lines: Vec<String> = TWO_LINES.iter().map(|l| l.to_string()).collect();
        for scale in LAPTOP_SCALES {
            let label = label(lines.clone(), scale);
            let sw = pixels(label.logical.w, scale) + 40;
            let at = label_at(sw, label.logical, scale, &m);
            let sh = at.1 as i32 + pixels(label.logical.h, scale) + 20;
            for which in ["old", "new"] {
                let shot = on_screen((sw, sh), scale, |r| match which {
                    "old" => MemoryRenderBufferRenderElement::from_buffer(r, at, &label.buffer, None, None, Some(label.logical), ElementKind::Unspecified).ok().map(CustomRenderElements::Dots).into_iter().collect(),
                    _ => label.element(r, at).into_iter().collect(),
                });
                let mut img = image::RgbImage::new(sw as u32, sh as u32);
                for (i, p) in shot.chunks(4).enumerate() {
                    let a = 255 - p[3] as u32;
                    let ground = |c: u8| (c as u32 + 0x30 * a / 255).min(255) as u8;
                    img.put_pixel(i as u32 % sw as u32, i as u32 / sw as u32, image::Rgb([ground(p[2]), ground(p[1]), ground(p[0])]));
                }
                img.save(format!("{dir}/label-{which}-{}.png", (scale * 100.0) as i32)).unwrap();
            }
        }
    }

    #[test]
    fn overlays_are_painted_exactly_their_declared_size_at_every_scale() {
        let m = Metrics::default();
        for scale in every_scale() {
            for width in [1, 5, 10, 15, 20, 25, 60, 99, 100, 141] {
                let logical: Size<i32, Logical> = (width, m.dot).into();
                let (w, h) = pixel_size(logical, scale);
                let bytes = paint(&[Kind::Microphone], w, h, scale as f32, &m);
                assert_eq!(bytes.len(), (w * h * 4) as usize, "dots x{scale} w{width}");
                let o = Overlay::new(&bytes, logical, scale);
                assert_eq!((o.px.w, o.px.h), (w, h));
            }
        }
    }

    #[test]
    #[cfg(not(debug_assertions))]
    fn a_short_buffer_is_drawn_clear_not_aborted() {
        let o = Overlay::new(&[0u8; 4], (15, 15).into(), 0.9);
        assert_eq!((o.px.w, o.px.h), pixel_size((15, 15).into(), 0.9));
    }
}
