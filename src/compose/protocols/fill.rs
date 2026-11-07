//! Putting a picture of the desktop into a buffer a program brought.

use std::ptr;

use smithay::backend::allocator::dmabuf::Dmabuf;
use smithay::backend::allocator::{Buffer as _, Fourcc};
use smithay::backend::renderer::damage::OutputDamageTracker;
use smithay::backend::renderer::element::RenderElement;
use smithay::backend::renderer::gles::GlesTexture;
use smithay::backend::renderer::sync::SyncPoint;
use smithay::backend::renderer::{Bind, Color32F, ExportMem, Offscreen, Renderer, Texture};
use smithay::reexports::wayland_server::protocol::{wl_buffer::WlBuffer, wl_shm};
use smithay::utils::{Buffer as BufferCoords, Physical, Rectangle, Size, Transform};
use smithay::wayland::{dmabuf, shm};

const PIXEL: usize = 4;

pub(super) enum Canvas {
    Card(Dmabuf),
    Memory(WlBuffer),
}

impl Canvas {
    pub(super) fn accept(buffer: WlBuffer, size: Size<i32, Physical>) -> Option<Canvas> {
        if let Ok(card) = dmabuf::get_dmabuf(&buffer) {
            let right_kind = matches!(card.format().code, Fourcc::Xrgb8888 | Fourcc::Argb8888);
            let right_size = card.width() == size.w as u32 && card.height() == size.h as u32;
            return (right_kind && right_size).then(|| Canvas::Card(card.clone()));
        }
        let fits = shm::with_buffer_contents(&buffer, |_, _, d| {
            fourcc_of(d.format).is_some() && (d.width, d.height) == (size.w, size.h) && d.stride >= size.w.saturating_mul(4)
        })
        .unwrap_or(false);
        fits.then_some(Canvas::Memory(buffer))
    }

    pub(super) fn paint<R, E>(
        &self,
        renderer: &mut R,
        size: Size<i32, Physical>,
        elements: &[E],
        backdrop: Color32F,
        scale: f64,
        transform: Transform,
        keep_alpha: bool,
    ) -> Result<Option<SyncPoint>, String>
    where
        R: Renderer + Offscreen<GlesTexture> + Bind<Dmabuf> + ExportMem,
        R::TextureId: Texture + 'static,
        E: RenderElement<R>,
    {
        let backdrop = if backdrop.a() < 1.0 && !keep_alpha { Color32F::BLACK } else { backdrop };
        let mut tracker = OutputDamageTracker::new(size, scale, transform);
        match self {
            Canvas::Card(card) => {
                let mut card = card.clone();
                let mut target = Bind::<Dmabuf>::bind(renderer, &mut card).map_err(|e| format!("binding the dmabuf: {e}"))?;
                let done = tracker
                    .render_output(renderer, &mut target, 0, elements, backdrop)
                    .map_err(|e| format!("drawing into the dmabuf: {e:?}"))?;
                Ok(Some(done.sync))
            }
            Canvas::Memory(buffer) => paint_memory(buffer, renderer, &mut tracker, size, elements, backdrop),
        }
    }
}

fn paint_memory<R, E>(
    buffer: &WlBuffer,
    renderer: &mut R,
    tracker: &mut OutputDamageTracker,
    size: Size<i32, Physical>,
    elements: &[E],
    backdrop: Color32F,
) -> Result<Option<SyncPoint>, String>
where
    R: Renderer + Offscreen<GlesTexture> + Bind<Dmabuf> + ExportMem,
    R::TextureId: Texture + 'static,
    E: RenderElement<R>,
{
    shm::with_buffer_contents_mut(buffer, |pool, pool_len, d| {
        let fourcc = fourcc_of(d.format).ok_or("unsupported shm format")?;
        let rows = Rows::within(d.offset, d.stride, size.w, size.h, pool_len)
            .ok_or("buffer does not fit in its shm pool")?;
        let area = size.to_logical(1).to_buffer(1, Transform::Normal);
        let mut texture: GlesTexture =
            renderer.create_buffer(fourcc, area).map_err(|e| format!("making a texture: {e}"))?;
        let mut target = Bind::<GlesTexture>::bind(renderer, &mut texture)
            .map_err(|e| format!("binding the texture: {e}"))?;
        tracker
            .render_output(renderer, &mut target, 0, elements, backdrop)
            .map_err(|e| format!("drawing into the texture: {e:?}"))?;
        let copy = renderer
            .copy_framebuffer(&target, Rectangle::from_size(area), fourcc)
            .map_err(|e| format!("reading the texture back: {e}"))?;
        let pixels = renderer.map_texture(&copy).map_err(|e| format!("mapping the texture: {e}"))?;
        if pixels.len() < rows.length * rows.count {
            return Err("texture came back short".to_string());
        }
        // SAFETY: smithay maps the client's whole pool, `pool_len`
        // bytes, for as long as this closure runs. `Rows::within`
        // proved every row written here, at offset + n * stride and
        // `length` bytes long, ends inside it, and `pixels` was just
        // checked to hold `count` rows of `length` bytes. The pool
        // belongs to the client, so it is written through a raw
        // pointer and never held as a Rust reference; a client
        // changing it meanwhile can only spoil its own picture.
        unsafe { rows.write(pool, pixels.as_ptr()) };
        Ok(None)
    })
    .map_err(|e| format!("not a shm buffer: {e}"))?
}

fn fourcc_of(format: wl_shm::Format) -> Option<Fourcc> {
    match format {
        wl_shm::Format::Xrgb8888 => Some(Fourcc::Xrgb8888),
        wl_shm::Format::Argb8888 => Some(Fourcc::Argb8888),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
struct Rows {
    start: usize,
    stride: usize,
    length: usize,
    count: usize,
}

impl Rows {
    fn within(offset: i32, stride: i32, width: i32, height: i32, pool_len: usize) -> Option<Rows> {
        let start = usize::try_from(offset).ok()?;
        let stride = usize::try_from(stride).ok()?;
        let length = usize::try_from(width).ok()?.checked_mul(PIXEL)?;
        let count = usize::try_from(height).ok()?;
        if length == 0 || count == 0 || stride < length {
            return None;
        }
        let end = stride.checked_mul(count - 1)?.checked_add(length)?.checked_add(start)?;
        (end <= pool_len).then_some(Rows { start, stride, length, count })
    }

    unsafe fn write(&self, pool: *mut u8, from: *const u8) {
        for n in 0..self.count {
            // SAFETY: the caller's promise covers every row.
            unsafe { ptr::copy_nonoverlapping(from.add(n * self.length), pool.add(self.start + n * self.stride), self.length) };
        }
    }
}

pub(super) fn damage_in_buffer(
    drawn: &[Rectangle<i32, Physical>],
    buffer_size: Size<i32, Physical>,
    transform: Transform,
) -> Vec<Rectangle<i32, BufferCoords>> {
    let whole = transform.transform_size(buffer_size).to_logical(1);
    drawn.iter().map(|r| r.to_logical(1).to_buffer(1, transform.invert(), &whole)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at<K>(x: i32, y: i32, w: i32, h: i32) -> Rectangle<i32, K> {
        Rectangle::new((x, y).into(), (w, h).into())
    }

    #[test]
    fn whole_picture_damage_fills_the_buffer_however_it_is_turned() {
        for turn in [Transform::Normal, Transform::_90, Transform::_180, Transform::_270, Transform::Flipped180] {
            let buffer = (200, 100).into();
            let drawn = turn.transform_size(buffer);
            let got = damage_in_buffer(&[Rectangle::from_size(drawn)], buffer, turn);
            assert_eq!(got, vec![at(0, 0, 200, 100)], "{turn:?}");
        }
    }

    #[test]
    fn untouched_damage_stays_where_it_was() {
        assert_eq!(damage_in_buffer(&[at(5, 6, 7, 8)], (100, 100).into(), Transform::Normal), vec![at(5, 6, 7, 8)]);
    }

    #[test]
    fn a_flipped_picture_mirrors_its_damage() {
        assert_eq!(damage_in_buffer(&[at(0, 0, 100, 10)], (100, 50).into(), Transform::Flipped180), vec![at(0, 40, 100, 10)]);
    }

    #[test]
    fn only_the_two_four_byte_formats_are_drawn() {
        assert_eq!(fourcc_of(wl_shm::Format::Xrgb8888), Some(Fourcc::Xrgb8888));
        assert_eq!(fourcc_of(wl_shm::Format::Argb8888), Some(Fourcc::Argb8888));
        assert_eq!(fourcc_of(wl_shm::Format::Rgb565), None);
        assert_eq!(fourcc_of(wl_shm::Format::Xbgr8888), None);
    }

    #[test]
    fn rows_inside_the_pool_are_accepted() {
        assert_eq!(Rows::within(0, 400, 100, 50, 400 * 50), Some(Rows { start: 0, stride: 400, length: 400, count: 50 }));
        assert!(Rows::within(64, 512, 100, 50, 64 + 512 * 49 + 400).is_some());
    }

    #[test]
    fn rows_outside_the_pool_are_refused() {
        assert_eq!(Rows::within(0, 400, 100, 50, 400 * 50 - 1), None, "one byte short");
        assert_eq!(Rows::within(0, 396, 100, 50, 1 << 20), None, "stride smaller than a row");
        assert_eq!(Rows::within(-4, 400, 100, 50, 1 << 20), None, "starts before the pool");
        assert_eq!(Rows::within(0, 400, 0, 50, 1 << 20), None, "no width");
        assert_eq!(Rows::within(0, 400, 100, 0, 1 << 20), None, "no height");
        assert_eq!(Rows::within(i32::MAX, i32::MAX, 100, i32::MAX, 1 << 20), None, "sizes that overflow");
    }

    #[test]
    fn writing_rows_skips_the_padding() {
        let rows = Rows::within(2, 10, 2, 3, 2 + 10 * 2 + 8).unwrap();
        let from: Vec<u8> = (1..=24).collect();
        let mut pool = vec![0u8; 2 + 10 * 2 + 8];
        // SAFETY: the pool is exactly as long as `within` was told, and `from`
        // holds 3 rows of 8 bytes.
        unsafe { rows.write(pool.as_mut_ptr(), from.as_ptr()) };
        assert_eq!(&pool[0..2], &[0, 0]);
        assert_eq!(&pool[2..10], &from[0..8]);
        assert_eq!(&pool[10..12], &[0, 0], "the padding after a row is not touched");
        assert_eq!(&pool[12..20], &from[8..16]);
        assert_eq!(&pool[22..30], &from[16..24]);
    }
}
