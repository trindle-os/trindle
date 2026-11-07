//! Photographs of the screen, asked of Trindle's compositor over ext-image-copy-capture.

use std::path::Path;

use tiny_skia::{FillRule, FilterQuality, IntSize, Mask, PathBuilder, Pixmap, PixmapPaint, Rect};
use wayland_client::globals::{registry_queue_init, GlobalListContents};
use wayland_client::protocol::wl_output::{self, Transform, WlOutput};
use wayland_client::protocol::{wl_buffer, wl_registry, wl_shm, wl_shm_pool};
use wayland_client::{Connection, Dispatch, EventQueue, Proxy, QueueHandle, WEnum};
use wayland_protocols::ext::image_capture_source::v1::client::{
    ext_image_capture_source_v1::ExtImageCaptureSourceV1,
    ext_output_image_capture_source_manager_v1::ExtOutputImageCaptureSourceManagerV1,
};
use wayland_protocols::ext::image_copy_capture::v1::client::{
    ext_image_copy_capture_frame_v1::{self, ExtImageCopyCaptureFrameV1},
    ext_image_copy_capture_manager_v1::{self, ExtImageCopyCaptureManagerV1},
    ext_image_copy_capture_session_v1::{self, ExtImageCopyCaptureSessionV1},
};
use wayland_protocols::xdg::xdg_output::zv1::client::{
    zxdg_output_manager_v1::ZxdgOutputManagerV1,
    zxdg_output_v1::{self, ZxdgOutputV1},
};

use crate::common::shm::Memory;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Area {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Area {
    fn meet(self, other: Area) -> Option<Area> {
        let (x0, y0) = (self.x.max(other.x), self.y.max(other.y));
        let x1 = (self.x + self.width).min(other.x + other.width);
        let y1 = (self.y + self.height).min(other.y + other.height);
        (x1 > x0 && y1 > y0).then_some(Area { x: x0, y: y0, width: x1 - x0, height: y1 - y0 })
    }

    fn join(self, other: Area) -> Area {
        let (x0, y0) = (self.x.min(other.x), self.y.min(other.y));
        let x1 = (self.x + self.width).max(other.x + other.width);
        let y1 = (self.y + self.height).max(other.y + other.height);
        Area { x: x0, y: y0, width: x1 - x0, height: y1 - y0 }
    }
}

pub struct Picture {
    pub width: i32,
    pub height: i32,
    pub stride: usize,
    pub data: Vec<u8>,
}

impl Picture {
    pub fn save_png(&self, path: &Path) -> Result<(), String> {
        let (w, h) = (self.width as u32, self.height as u32);
        let mut rgba = Vec::with_capacity(self.width as usize * self.height as usize * 4);
        for row in self.data.chunks_exact(self.stride).take(self.height as usize) {
            for px in row[..self.width as usize * 4].chunks_exact(4) {
                rgba.extend_from_slice(&unpremultiplied([px[0], px[1], px[2], px[3]]));
            }
        }
        let saved = if rgba.chunks_exact(4).all(|p| p[3] == 255) {
            let rgb: Vec<u8> = rgba.chunks_exact(4).flat_map(|p| [p[0], p[1], p[2]]).collect();
            image::save_buffer_with_format(path, &rgb, w, h, image::ExtendedColorType::Rgb8, image::ImageFormat::Png)
        } else {
            image::save_buffer_with_format(path, &rgba, w, h, image::ExtendedColorType::Rgba8, image::ImageFormat::Png)
        };
        saved.map_err(|e| e.to_string())
    }
}

impl Picture {
    pub fn rgba_at(&self, x: i32, y: i32) -> Option<[u8; 4]> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }
        let at = y as usize * self.stride + x as usize * 4;
        let px = self.data.get(at..at + 4)?;
        Some(unpremultiplied([px[0], px[1], px[2], px[3]]))
    }
}

fn unpremultiplied(px: [u8; 4]) -> [u8; 4] {
    let a = u32::from(px[3]);
    let c = |x: u8| -> u8 { (u32::from(x) * 255 + a / 2).checked_div(a).map_or(0, |v| v.min(255) as u8) };
    [c(px[0]), c(px[1]), c(px[2]), px[3]]
}

pub fn take(area: Option<Area>, cursor: bool) -> Result<Picture, String> {
    let conn = Connection::connect_to_env().map_err(|_| "There is no desktop to photograph.".to_string())?;
    let mut desk = Desk::open(&conn)?;
    let boxes: Vec<Option<Area>> = desk.state.displays.iter().map(Display::area).collect();
    let everything = boxes.iter().flatten().copied().reduce(Area::join).ok_or("There is no display to photograph.")?;
    let area = match area {
        Some(a) => a.meet(everything).ok_or("That part of the desktop has no display on it.")?,
        None => everything,
    };
    desk.ask_for_shots(&boxes, area, cursor);
    desk.wait_for_shots()?;
    let parts: Vec<(Area, Upright)> =
        desk.state.shots.iter().filter_map(|s| Some((boxes[s.display]?, upright(s)?))).collect();
    desk.let_go();
    let _ = conn.flush();
    lay_out(area, parts)
}

struct Desk {
    queue: EventQueue<State>,
    qh: QueueHandle<State>,
    copier: ExtImageCopyCaptureManagerV1,
    sources: ExtOutputImageCaptureSourceManagerV1,
    state: State,
}

impl Desk {
    fn open(conn: &Connection) -> Result<Desk, String> {
        let (globals, mut queue) = registry_queue_init::<State>(conn).map_err(|e| e.to_string())?;
        let qh = queue.handle();
        let copier: ExtImageCopyCaptureManagerV1 = globals
            .bind(&qh, 1..=1, ())
            .map_err(|_| "This desktop cannot be photographed.".to_string())?;
        let sources: ExtOutputImageCaptureSourceManagerV1 = globals
            .bind(&qh, 1..=1, ())
            .map_err(|_| "This desktop cannot be photographed.".to_string())?;
        let shm: wl_shm::WlShm = globals.bind(&qh, 1..=1, ()).map_err(|e| e.to_string())?;
        let places: Option<ZxdgOutputManagerV1> = globals.bind(&qh, 1..=3, ()).ok();
        let mut state = State { shm, displays: Vec::new(), shots: Vec::new() };
        for g in globals.contents().clone_list() {
            if g.interface == WlOutput::interface().name {
                let i = state.displays.len();
                let output: WlOutput = globals.registry().bind(g.name, g.version.min(4), &qh, i);
                if let Some(p) = &places {
                    p.get_xdg_output(&output, &qh, i);
                }
                state.displays.push(Display {
                    output,
                    position: (0, 0),
                    pixels: (0, 0),
                    scale: 1,
                    transform: Transform::Normal,
                    place: None,
                    size: None,
                });
            }
        }
        queue.roundtrip(&mut state).map_err(|e| e.to_string())?;
        Ok(Desk { queue, qh, copier, sources, state })
    }

    fn ask_for_shots(&mut self, boxes: &[Option<Area>], area: Area, cursor: bool) {
        let options = if cursor {
            ext_image_copy_capture_manager_v1::Options::PaintCursors
        } else {
            ext_image_copy_capture_manager_v1::Options::empty()
        };
        for (i, b) in boxes.iter().enumerate() {
            if b.is_some_and(|b| b.meet(area).is_some()) {
                let source = self.sources.create_source(&self.state.displays[i].output, &self.qh, ());
                let session = self.copier.create_session(&source, options, &self.qh, self.state.shots.len());
                source.destroy();
                self.state.shots.push(Shot { display: i, session: Some(session), ..Shot::default() });
            }
        }
    }

    fn wait_for_shots(&mut self) -> Result<(), String> {
        while self.state.shots.iter().any(|s| s.outcome.is_none()) {
            self.queue.blocking_dispatch(&mut self.state).map_err(|e| e.to_string())?;
        }
        match self.state.shots.iter().find_map(|s| s.outcome.clone().and_then(Result::err)) {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    fn let_go(&self) {
        for s in &self.state.shots {
            if let Some(f) = &s.frame {
                f.destroy();
            }
            if let Some(b) = &s.buffer {
                b.destroy();
            }
            if let Some(session) = &s.session {
                session.destroy();
            }
        }
    }
}

struct Upright {
    width: i32,
    height: i32,
    data: Vec<u8>,
}

fn upright(shot: &Shot) -> Option<Upright> {
    let (offer, memory) = (shot.offer?, shot.memory.as_ref()?);
    let (w, h, stride) = (offer.width as usize, offer.height as usize, offer.stride as usize);
    let bytes = memory.bytes();
    if stride < w * 4 || bytes.len() < stride * h {
        return None;
    }
    let mut data = Vec::with_capacity(w * h * 4);
    for row in bytes.chunks_exact(stride).take(h) {
        for px in row[..w * 4].chunks_exact(4) {
            data.extend_from_slice(&[px[2], px[1], px[0], 255]);
        }
    }
    Some(Upright { width: w as i32, height: h as i32, data })
}

fn sideways(t: Transform) -> bool {
    matches!(t, Transform::_90 | Transform::_270 | Transform::Flipped90 | Transform::Flipped270)
}

fn lay_out(area: Area, parts: Vec<(Area, Upright)>) -> Result<Picture, String> {
    let scale = parts.iter().map(|(b, u)| u.width as f64 / b.width.max(1) as f64).fold(1.0_f64, f64::max);
    let (width, height) = ((area.width as f64 * scale) as i32, (area.height as f64 * scale) as i32);
    let mut picture = Pixmap::new(width as u32, height as u32).ok_or("The picture is too big.")?;
    for (b, u) in parts {
        let source = IntSize::from_wh(u.width as u32, u.height as u32)
            .and_then(|size| Pixmap::from_vec(u.data, size))
            .ok_or("A display gave a picture of no size.")?;
        let (x, y) = (((b.x - area.x) as f64 * scale).trunc(), ((b.y - area.y) as f64 * scale).trunc());
        let (w, h) = (b.width as f64 * scale, b.height as f64 * scale);
        let mut clip = Mask::new(width as u32, height as u32).ok_or("The picture is too big.")?;
        if let Some(rect) = Rect::from_xywh(x as f32, y as f32, w as f32, h as f32) {
            clip.fill_path(&PathBuilder::from_rect(rect), FillRule::Winding, false, tiny_skia::Transform::identity());
        }
        let k = (w / u.width as f64) as f32;
        let place = tiny_skia::Transform::from_scale(k, k).post_translate(x as f32, y as f32);
        let paint = PixmapPaint { blend_mode: tiny_skia::BlendMode::Source, quality: FilterQuality::Bilinear, ..PixmapPaint::default() };
        picture.draw_pixmap(0, 0, source.as_ref(), &paint, place, Some(&clip));
    }
    let stride = width as usize * 4;
    let data = picture.take();
    Ok(Picture { width, height, stride, data })
}

struct Display {
    output: WlOutput,
    position: (i32, i32),
    pixels: (i32, i32),
    scale: i32,
    transform: Transform,
    place: Option<(i32, i32)>,
    size: Option<(i32, i32)>,
}

impl Display {
    fn area(&self) -> Option<Area> {
        area_of(self.place.zip(self.size), self.position, self.pixels, self.scale, self.transform)
    }
}

fn area_of(says: Option<((i32, i32), (i32, i32))>, position: (i32, i32), pixels: (i32, i32), scale: i32,
           transform: Transform) -> Option<Area> {
    if let Some(((x, y), (w, h))) = says {
        return (w > 0 && h > 0).then_some(Area { x, y, width: w, height: h });
    }
    let (mut w, mut h) = pixels;
    if sideways(transform) {
        std::mem::swap(&mut w, &mut h);
    }
    let s = scale.max(1);
    (w > 0 && h > 0).then_some(Area { x: position.0, y: position.1, width: w / s, height: h / s })
}

#[derive(Clone, Copy)]
struct Offer {
    width: u32,
    height: u32,
    stride: u32,
}

#[derive(Default)]
struct Shot {
    display: usize,
    session: Option<ExtImageCopyCaptureSessionV1>,
    size: Option<(u32, u32)>,
    readable: bool,
    offer: Option<Offer>,
    frame: Option<ExtImageCopyCaptureFrameV1>,
    memory: Option<Memory>,
    buffer: Option<wl_buffer::WlBuffer>,
    outcome: Option<Result<(), String>>,
}

struct State {
    shm: wl_shm::WlShm,
    displays: Vec<Display>,
    shots: Vec<Shot>,
}

impl State {
    fn ask(&mut self, index: usize, qh: &QueueHandle<Self>) {
        let Some(shot) = self.shots.get_mut(index) else { return };
        if shot.frame.is_some() || shot.outcome.is_some() {
            return;
        }
        let (Some((width, height)), true) = (shot.size, shot.readable) else {
            shot.outcome = Some(Err("The desktop offered no picture this can read.".into()));
            return;
        };
        let stride = width * 4;
        let len = stride as usize * height as usize;
        let Some(memory) = Memory::new(c"trindle-capture", len) else {
            shot.outcome = Some(Err("There was no memory for the picture.".into()));
            return;
        };
        let pool = self.shm.create_pool(memory.fd(), len as i32, qh, ());
        let buffer = pool.create_buffer(0, width as i32, height as i32, stride as i32, wl_shm::Format::Xrgb8888, qh, ());
        pool.destroy();
        let Some(session) = &shot.session else { return };
        let frame = session.create_frame(qh, index);
        frame.attach_buffer(&buffer);
        frame.damage_buffer(0, 0, width as i32, height as i32);
        frame.capture();
        shot.offer = Some(Offer { width, height, stride });
        shot.frame = Some(frame);
        shot.memory = Some(memory);
        shot.buffer = Some(buffer);
    }
}

impl Dispatch<ExtImageCopyCaptureSessionV1, usize> for State {
    fn event(state: &mut Self, _: &ExtImageCopyCaptureSessionV1, event: ext_image_copy_capture_session_v1::Event,
             index: &usize, _: &Connection, qh: &QueueHandle<Self>) {
        use ext_image_copy_capture_session_v1::Event;
        let Some(shot) = state.shots.get_mut(*index) else { return };
        match event {
            Event::BufferSize { width, height } => shot.size = Some((width, height)),
            Event::ShmFormat { format: WEnum::Value(wl_shm::Format::Xrgb8888) } => shot.readable = true,
            Event::Done => state.ask(*index, qh),
            Event::Stopped => {
                shot.outcome.get_or_insert(Err("The desktop stopped photographing the screen.".into()));
            }
            _ => {}
        }
    }
}

impl Dispatch<ExtImageCopyCaptureFrameV1, usize> for State {
    fn event(state: &mut Self, _: &ExtImageCopyCaptureFrameV1, event: ext_image_copy_capture_frame_v1::Event,
             index: &usize, _: &Connection, _: &QueueHandle<Self>) {
        use ext_image_copy_capture_frame_v1::Event;
        let Some(shot) = state.shots.get_mut(*index) else { return };
        match event {
            Event::Ready => shot.outcome = Some(Ok(())),
            Event::Failed { .. } => shot.outcome = Some(Err("The desktop could not photograph the screen.".into())),
            _ => {}
        }
    }
}

impl Dispatch<WlOutput, usize> for State {
    fn event(state: &mut Self, _: &WlOutput, event: wl_output::Event, index: &usize,
             _: &Connection, _: &QueueHandle<Self>) {
        let Some(d) = state.displays.get_mut(*index) else { return };
        match event {
            wl_output::Event::Geometry { x, y, transform: WEnum::Value(t), .. } => {
                d.position = (x, y);
                d.transform = t;
            }
            wl_output::Event::Mode { flags: WEnum::Value(f), width, height, .. } if f.contains(wl_output::Mode::Current) => {
                d.pixels = (width, height);
            }
            wl_output::Event::Scale { factor } => d.scale = factor,
            _ => {}
        }
    }
}

impl Dispatch<ZxdgOutputV1, usize> for State {
    fn event(state: &mut Self, _: &ZxdgOutputV1, event: zxdg_output_v1::Event, index: &usize,
             _: &Connection, _: &QueueHandle<Self>) {
        let Some(d) = state.displays.get_mut(*index) else { return };
        match event {
            zxdg_output_v1::Event::LogicalPosition { x, y } => d.place = Some((x, y)),
            zxdg_output_v1::Event::LogicalSize { width, height } => d.size = Some((width, height)),
            _ => {}
        }
    }
}

macro_rules! ignore {
    ($($t:ty),*) => {$(
        impl Dispatch<$t, ()> for State {
            fn event(_: &mut Self, _: &$t, _: <$t as Proxy>::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
        }
    )*};
}
ignore!(
    ExtImageCopyCaptureManagerV1,
    ExtOutputImageCaptureSourceManagerV1,
    ExtImageCaptureSourceV1,
    ZxdgOutputManagerV1,
    wl_shm::WlShm,
    wl_shm_pool::WlShmPool,
    wl_buffer::WlBuffer
);

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(_: &mut Self, _: &wl_registry::WlRegistry, _: wl_registry::Event, _: &GlobalListContents,
             _: &Connection, _: &QueueHandle<Self>) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn areas_meet_and_join() {
        let a = Area { x: 0, y: 0, width: 1920, height: 1080 };
        let b = Area { x: 1920, y: 0, width: 1280, height: 1024 };
        assert_eq!(a.join(b), Area { x: 0, y: 0, width: 3200, height: 1080 });
        assert_eq!(a.meet(b), None);
        let part = Area { x: 1900, y: 100, width: 100, height: 50 };
        assert_eq!(part.meet(b), Some(Area { x: 1920, y: 100, width: 80, height: 50 }));
    }

    #[test]
    fn a_display_on_its_side_is_measured_the_right_way_round() {
        let got = area_of(None, (0, 0), (1920, 1080), 1, Transform::_90);
        assert_eq!(got, Some(Area { x: 0, y: 0, width: 1080, height: 1920 }));
        let said = area_of(Some(((10, 20), (540, 960))), (0, 0), (1920, 1080), 2, Transform::_90);
        assert_eq!(said, Some(Area { x: 10, y: 20, width: 540, height: 960 }));
        assert_eq!(area_of(None, (0, 0), (0, 0), 1, Transform::Normal), None);
    }

    #[test]
    fn two_displays_meet_at_the_sharpest_scale() {
        let solid = |w: i32, h: i32, rgb: [u8; 3]| Upright { width: w, height: h, data: (0..w * h).flat_map(|_| [rgb[0], rgb[1], rgb[2], 255]).collect() };
        let parts = vec![
            (Area { x: 0, y: 0, width: 4, height: 2 }, solid(4, 2, [255, 0, 0])),
            (Area { x: 4, y: 0, width: 4, height: 2 }, solid(8, 4, [0, 0, 255])),
        ];
        let got = lay_out(Area { x: 0, y: 0, width: 8, height: 2 }, parts).unwrap();
        assert_eq!((got.width, got.height), (16, 4));
        let px = |x: usize, y: usize| &got.data[y * got.stride + x * 4..][..4];
        assert_eq!(px(1, 1), [255, 0, 0, 255]);
        assert_eq!(px(7, 3), [255, 0, 0, 255]);
        assert_eq!(px(8, 0), [0, 0, 255, 255]);
        assert_eq!(px(14, 3), [0, 0, 255, 255]);
    }

    #[test]
    fn a_filled_buffer_is_read_blue_first_and_made_solid() {
        let mut memory = Memory::new(c"trindle-capture-test", 12).unwrap();
        memory.bytes_mut().copy_from_slice(&[10, 20, 30, 7, 40, 50, 60, 0, 0, 0, 0, 0]);
        let shot = Shot { offer: Some(Offer { width: 2, height: 1, stride: 12 }), memory: Some(memory), ..Shot::default() };
        let got = upright(&shot).unwrap();
        assert_eq!((got.width, got.height), (2, 1));
        assert_eq!(got.data, vec![30, 20, 10, 255, 60, 50, 40, 255]);
    }
}
