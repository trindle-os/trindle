//! The computer maker's logo, as the firmware showed it while switching on
//! (ACPI's BGRT). It stays on the screen above the turning mark, the way other
//! systems do it, so the start looks like one piece from the power button on
//! instead of the logo vanishing into black.

pub struct Logo {
    pub width: usize,
    pub height: usize,
    x: usize,
    y: usize,
    pixels: Vec<(u8, u8, u8)>,
}

impl Logo {

    pub fn firmware() -> Option<Logo> {
        let dir = "/sys/firmware/acpi/bgrt";
        let number = |f: &str| std::fs::read_to_string(format!("{dir}/{f}")).ok()?.trim().parse::<usize>().ok();
        let image = std::fs::read(format!("{dir}/image")).ok()?;
        from_bmp(&image, number("xoffset")?, number("yoffset")?)
    }

    pub fn pixel(&self, x: usize, y: usize) -> (u8, u8, u8) {
        self.pixels[y * self.width + x]
    }

    pub fn place(&self, width: usize, height: usize) -> Option<(usize, usize)> {
        if self.width > width || self.height > height {
            return None;
        }
        let same = (self.x * 2 + self.width).abs_diff(width) <= 2 && self.y + self.height <= height;
        if same {
            return Some((self.x, self.y));
        }
        let middle = (height as f32 * 0.382) as usize;
        Some(((width - self.width) / 2, middle.saturating_sub(self.height / 2)))
    }
}

fn from_bmp(b: &[u8], x: usize, y: usize) -> Option<Logo> {
    let u16_at = |i: usize| b.get(i..i + 2).map(|v| u16::from_le_bytes([v[0], v[1]]));
    let u32_at = |i: usize| b.get(i..i + 4).map(|v| u32::from_le_bytes([v[0], v[1], v[2], v[3]]));
    if b.get(..2)? != b"BM" {
        return None;
    }
    let start = u32_at(10)? as usize;
    let width = u32_at(18)? as i32;
    let rows = u32_at(22)? as i32;
    let bits = u16_at(28)? as usize;
    let packing = u32_at(30)?;
    if width <= 0 || rows == 0 || !(bits == 24 || bits == 32) || !(packing == 0 || packing == 3) {
        return None;
    }
    let (width, height) = (width as usize, rows.unsigned_abs() as usize);
    if width > 8192 || height > 8192 {
        return None;
    }
    let stride = (width * bits).div_ceil(32) * 4;
    let bytes = bits / 8;
    let mut pixels = Vec::with_capacity(width * height);
    for row in 0..height {

        let from = if rows > 0 { height - 1 - row } else { row };
        let line = b.get(start + from * stride..start + from * stride + width * bytes)?;
        pixels.extend(line.chunks_exact(bytes).map(|p| (p[2], p[1], p[0])));
    }
    Some(Logo { width, height, x, y, pixels })
}

#[cfg(test)]
mod tests {

    fn two_by_two() -> Vec<u8> {
        let mut b = vec![0u8; 54];
        b[..2].copy_from_slice(b"BM");
        b[10..14].copy_from_slice(&54u32.to_le_bytes());
        b[18..22].copy_from_slice(&2u32.to_le_bytes());
        b[22..26].copy_from_slice(&2u32.to_le_bytes());
        b[28..30].copy_from_slice(&24u16.to_le_bytes());

        b.extend([255, 0, 0, 255, 255, 255, 0, 0]);
        b.extend([0, 0, 255, 0, 255, 0, 0, 0]);
        b
    }

    #[test]
    fn a_bmp_reads_the_right_way_up() {
        let logo = super::from_bmp(&two_by_two(), 0, 0).unwrap();
        assert_eq!((logo.width, logo.height), (2, 2));
        assert_eq!(logo.pixel(0, 0), (255, 0, 0));
        assert_eq!(logo.pixel(1, 0), (0, 255, 0));
        assert_eq!(logo.pixel(0, 1), (0, 0, 255));
        assert_eq!(logo.pixel(1, 1), (255, 255, 255));
    }

    #[test]
    fn the_logo_goes_where_the_firmware_put_it() {
        let mut logo = super::from_bmp(&two_by_two(), 0, 0).unwrap();
        (logo.width, logo.height, logo.x, logo.y) = (768, 256, 576, 284);
        assert_eq!(logo.place(1920, 1080), Some((576, 284)));

        assert_eq!(logo.place(2560, 1600), Some((896, 483)));
        assert_eq!(logo.place(640, 480), None);
    }
}
