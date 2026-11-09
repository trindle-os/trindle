
use std::f64::consts::PI;

const RATE: u32 = 44100;

fn note(out: &mut Vec<f64>, start: f64, len: f64, hz: f64, gain: f64) {
    let first = (start * RATE as f64) as usize;
    let n = (len * RATE as f64) as usize;
    if out.len() < first + n {
        out.resize(first + n, 0.0);
    }
    for i in 0..n {
        let t = i as f64 / RATE as f64;
        let attack = (t / 0.012).min(1.0);
        let release = ((len - t) / len).powf(2.0);
        let env = attack * release;
        let tone = (2.0 * PI * hz * t).sin() + 0.25 * (2.0 * PI * hz * 2.0 * t).sin() * (-t * 9.0).exp();
        out[first + i] += tone * env * gain;
    }
}

fn chime(a: f64, b: f64, gap: f64, len: f64) -> Vec<i16> {
    let mut out = Vec::new();
    note(&mut out, 0.0, len * 0.7, a, 0.22);
    note(&mut out, gap, len - gap, b, 0.22);
    let total = out.len();
    let fade = (0.03 * RATE as f64) as usize;
    for i in 0..fade.min(total) {
        out[total - 1 - i] *= i as f64 / fade as f64;
    }
    out.iter().map(|v| (v * 32767.0) as i16).collect()
}

fn crc8(d: &[u8]) -> u8 {
    let mut c = 0u8;
    for &b in d {
        c ^= b;
        for _ in 0..8 {
            c = if c & 0x80 != 0 { (c << 1) ^ 0x07 } else { c << 1 };
        }
    }
    c
}

fn crc16(d: &[u8]) -> u16 {
    let mut c = 0u16;
    for &b in d {
        c ^= (b as u16) << 8;
        for _ in 0..8 {
            c = if c & 0x8000 != 0 { (c << 1) ^ 0x8005 } else { c << 1 };
        }
    }
    c
}

struct Bits(Vec<u8>, u32);
impl Bits {
    fn put(&mut self, v: u64, n: u32) {
        for i in (0..n).rev() {
            if self.1 == 0 {
                self.0.push(0);
            }
            if (v >> i) & 1 == 1 {
                *self.0.last_mut().unwrap() |= 0x80 >> self.1;
            }
            self.1 = (self.1 + 1) % 8;
        }
    }
}

fn flac(samples: &[i16]) -> Vec<u8> {
    let mut f = b"fLaC".to_vec();
    f.extend([0x80, 0, 0, 34, 0x10, 0, 0x10, 0, 0, 0, 0, 0, 0, 0]);
    let mut b = Bits(Vec::new(), 0);
    b.put(RATE as u64, 20);
    b.put(0, 3);
    b.put(15, 5);
    b.put(samples.len() as u64, 36);
    f.extend(b.0);
    f.extend([0u8; 16]);
    for (n, chunk) in samples.chunks(4096).enumerate() {
        let mut h = vec![0xFF, 0xF8];
        h.push(if chunk.len() == 4096 { 0xC0 } else { 0x70 });
        h.push(0x08);
        h.push(n as u8);
        if chunk.len() != 4096 {
            h.extend(((chunk.len() - 1) as u16).to_be_bytes());
        }
        h.push(crc8(&h));
        h.push(0x02);
        h.push(0x00);
        for s in chunk {
            h.extend(s.to_be_bytes());
        }
        let c = crc16(&h);
        h.extend(c.to_be_bytes());
        f.extend(h);
    }
    f
}

fn main() {
    let dir = std::env::args().nth(1).unwrap_or_else(|| "target/new-sounds".into());
    std::fs::create_dir_all(&dir).unwrap();
    let open = chime(659.25, 880.0, 0.11, 0.30);
    let close = chime(783.99, 587.33, 0.09, 0.25);
    std::fs::write(format!("{dir}/open.flac"), flac(&open)).unwrap();
    std::fs::write(format!("{dir}/close.flac"), flac(&close)).unwrap();
}
