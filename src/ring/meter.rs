//! The microphone's live level and the speaker test on the ring's Sound page.
//! The level is read only while the page is open and is kept nowhere.

use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

const RATE: u32 = 8000;

const CHUNK: usize = (RATE as usize / 10) * 2;

const LOUD: f32 = 0.35;

pub struct Meter {
    peak: Arc<AtomicU32>,
    child: Option<Child>,
}

impl Meter {
    pub fn start() -> Self {
        Self::start_with(&["pw-record", "--rate", &RATE.to_string(), "--channels", "1", "--format", "s16", "-"])
    }

    fn start_with(argv: &[&str]) -> Self {
        let peak = Arc::new(AtomicU32::new(0));
        let mut child = Command::new(argv[0]).args(&argv[1..]).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn().ok();
        if let Some(out) = child.as_mut().and_then(|c| c.stdout.take()) {
            let p = peak.clone();
            std::thread::spawn(move || listen(&p, out));
        }
        Meter { peak, child }
    }

    pub fn level(&self) -> f32 {
        f32::from_bits(self.peak.load(Ordering::Relaxed))
    }
}

impl Drop for Meter {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn listen(peak: &AtomicU32, mut out: impl Read) {
    let mut chunk = [0u8; CHUNK];
    while out.read_exact(&mut chunk).is_ok() {
        let top = chunk.chunks_exact(2).map(|b| i16::from_le_bytes([b[0], b[1]]).unsigned_abs()).max().unwrap_or(0);
        peak.store(shown(top).to_bits(), Ordering::Relaxed);
    }
}

fn shown(top: u16) -> f32 {
    ((top as f32 / i16::MAX as f32) / LOUD).sqrt().min(1.0)
}

pub fn beep() {
    std::thread::spawn(|| {
        let Ok(mut child) = Command::new("pw-play").args(["--media-role", crate::common::sounds::ROLE, "-"]).stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::null()).spawn() else { return };
        if let Some(mut input) = child.stdin.take() {
            let _ = input.write_all(&tones());
        }
        let _ = child.wait();
    });
}

fn tones() -> Vec<u8> {
    const PLAY: u32 = crate::common::sounds::RATE;
    let note = |hz: f32| (0..PLAY * 3 / 10).map(move |i| {
        let t = i as f32 / PLAY as f32;
        let edge = (i as f32 / 800.0).min(((PLAY * 3 / 10 - i) as f32) / 800.0).min(1.0);
        ((t * hz * std::f32::consts::TAU).sin() * edge * 9000.0) as i16
    });
    let samples: Vec<i16> = note(440.0).chain(note(660.0)).collect();
    let bytes = samples.len() as u32 * 2;
    let mut wav = Vec::with_capacity(44 + bytes as usize);
    wav.extend(b"RIFF");
    wav.extend((36 + bytes).to_le_bytes());
    wav.extend(b"WAVE");
    wav.extend(b"fmt ");
    wav.extend(16u32.to_le_bytes());
    wav.extend(1u16.to_le_bytes());
    wav.extend(1u16.to_le_bytes());
    wav.extend(PLAY.to_le_bytes());
    wav.extend((PLAY * 2).to_le_bytes());
    wav.extend(2u16.to_le_bytes());
    wav.extend(16u16.to_le_bytes());
    wav.extend(b"data");
    wav.extend(bytes.to_le_bytes());
    for s in samples {
        wav.extend(s.to_le_bytes());
    }
    wav
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_meter_that_hears_nothing_still_ends_its_program_when_dropped() {
        let meter = Meter::start_with(&["sleep", "30"]);
        let pid = meter.child.as_ref().expect("started").id() as libc::pid_t;
        assert_eq!(meter.level(), 0.0);
        drop(meter);
        // SAFETY: signal 0 only checks whether the number is still a process.
        let alive = unsafe { libc::kill(pid, 0) } == 0;
        assert!(!alive, "the program was left running");
    }

    #[test]
    fn silence_is_empty_and_a_loud_voice_fills_the_bar() {
        assert_eq!(shown(0), 0.0);
        assert_eq!(shown(i16::MAX as u16), 1.0);
        assert!(shown(500) > 0.1);
    }

    #[test]
    fn the_test_sound_is_a_whole_wav_file() {
        let wav = tones();
        assert_eq!(&wav[..4], b"RIFF");
        assert_eq!(u32::from_le_bytes(wav[4..8].try_into().unwrap()) as usize, wav.len() - 8);
    }
}
