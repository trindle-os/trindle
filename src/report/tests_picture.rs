//! Tests for the one optional picture a report may carry. The server here is a fake on this computer's own address only.

use std::io::Cursor;

use super::picture::{self, Fault, Loaded};
use super::send::{post, Answer};
use super::tests_send::Fake;
use super::*;

fn png(w: u32, h: u32, noise: bool) -> Vec<u8> {
    let mut seed = 12345u32;
    let img = image::RgbaImage::from_fn(w, h, |x, y| {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        if noise { image::Rgba([(seed >> 24) as u8, (seed >> 16) as u8, (seed >> 8) as u8, 255]) } else { image::Rgba([x as u8, y as u8, 7, 255]) }
    });
    let mut out = Vec::new();
    img.write_to(&mut Cursor::new(&mut out), image::ImageFormat::Png).unwrap();
    out
}

fn chunk(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut c = (data.len() as u32).to_be_bytes().to_vec();
    c.extend_from_slice(kind);
    c.extend_from_slice(data);
    c.extend_from_slice(&[0; 4]);
    c
}

fn with_parts(bytes: &[u8], parts: &[Vec<u8>]) -> Vec<u8> {
    let mut out = bytes[..33].to_vec();
    for p in parts {
        out.extend_from_slice(p);
    }
    out.extend_from_slice(&bytes[33..]);
    out
}

fn header(w: u32, h: u32) -> Vec<u8> {
    let mut data = w.to_be_bytes().to_vec();
    data.extend_from_slice(&h.to_be_bytes());
    data.extend_from_slice(&[8, 6, 0, 0, 0]);
    let mut out = picture::SIGNATURE.to_vec();
    out.extend_from_slice(&chunk(b"IHDR", &data));
    out
}

fn fault(bytes: &[u8]) -> Fault {
    picture::load(bytes).err().expect("refused")
}

fn who() -> Who {
    Who { home: "/home/jsmith".into(), names: vec!["jsmith".into()], ssids: Vec::new() }
}

fn with_picture(mode: Mode) -> Reporter {
    let mut r = Reporter::new(who(), "Example Computer".into(), PathBuf::from("/nonexistent"));
    r.mode = mode;
    r.switch = true;
    r.gate = Gate::Open;
    r.log = Some(vec!["line one".into(); 3]);
    r.typed = "It stopped working.".into();
    r.picture_is(picture::load(&png(40, 30, false)));
    r
}

#[test]
fn only_the_image_parts_of_a_png_are_kept() {
    let plain = png(8, 6, false);
    let dirty = with_parts(&plain, &[chunk(b"tEXt", b"Author\0jsmith"), chunk(b"eXIf", b"GPS 52.3 0.5"), chunk(b"iTXt", b"jsmith@example.com"), chunk(b"tIME", &[0; 7]), chunk(b"iCCP", b"name\0\0data"), chunk(b"zTXt", b"x")]);
    assert!(dirty.windows(4).any(|w| w == b"tEXt"));
    let loaded = picture::load(&dirty).unwrap();
    for gone in [&b"tEXt"[..], b"eXIf", b"iTXt", b"tIME", b"iCCP", b"zTXt", b"jsmith", b"GPS"] {
        assert!(!loaded.png.windows(gone.len()).any(|w| w == gone), "{} left in", String::from_utf8_lossy(gone));
    }
    assert!(loaded.png.windows(4).any(|w| w == b"IDAT") && loaded.png.windows(4).any(|w| w == b"IEND"));
    assert_eq!(loaded.size, (8, 6));
    assert_eq!(image::load_from_memory(&loaded.png).unwrap().to_rgba8(), image::load_from_memory(&plain).unwrap().to_rgba8(), "the picture is unchanged");
    let trailing = [plain.clone(), b"secret after the end".to_vec()].concat();
    assert!(!picture::load(&trailing).unwrap().png.windows(6).any(|w| w == b"secret"), "anything after the end is dropped");
}

#[test]
fn what_is_not_a_real_png_is_refused_by_its_bytes() {
    assert_eq!(fault(b""), Fault::NotPng);
    assert_eq!(fault(b"GIF89a not a png"), Fault::NotPng);
    assert_eq!(fault(&[0xff, 0xd8, 0xff, 0xe0, 0, 0x10, b'J', b'F', b'I', b'F', 0, 1, 1, 0, 0, 1, 0, 1, 0, 0]), Fault::NotPng);
    let mut named_png = b"<svg>".to_vec();
    named_png.resize(100, b' ');
    assert_eq!(fault(&named_png), Fault::NotPng, "the name decides nothing");
}

#[test]
fn a_damaged_png_is_refused() {
    let good = png(16, 16, true);
    assert_eq!(fault(&good[..good.len() / 2]), Fault::Damaged, "cut short");
    assert_eq!(fault(&good[..20]), Fault::Damaged, "cut in the header");
    let mut flipped = good.clone();
    let at = flipped.windows(4).position(|w| w == b"IDAT").unwrap() + 10;
    flipped[at] ^= 0xff;
    assert_eq!(fault(&flipped), Fault::Damaged, "a changed byte");
    let mut long = good.clone();
    let at = flipped.windows(4).position(|w| w == b"IDAT").unwrap() - 4;
    long[at..at + 4].copy_from_slice(&u32::MAX.to_be_bytes());
    assert_eq!(fault(&long), Fault::Damaged, "a part longer than the file");
    assert_eq!(fault(&header(0, 5)), Fault::Damaged, "no width");
    assert_eq!(fault(&[picture::SIGNATURE.to_vec(), chunk(b"IDAT", b"x")].concat()), Fault::Damaged, "no header first");
}

#[test]
fn the_size_caps_are_read_from_the_header_before_anything_is_decoded() {
    assert_eq!(fault(&header(8193, 10)), Fault::TooLarge);
    assert_eq!(fault(&header(10, 8193)), Fault::TooLarge);
    assert_eq!(fault(&header(5000, 5000)), Fault::TooLarge, "25 million pixels");
    assert_eq!(fault(&header(u32::MAX, u32::MAX)), Fault::TooLarge);
    assert_ne!(fault(&header(4000, 4000)), Fault::TooLarge, "16 million is allowed, and this one has no image data");
    assert_ne!(fault(&header(8192, 1)), Fault::TooLarge);
}

#[test]
fn a_file_over_two_megabytes_is_refused_without_reading_it_all() {
    let dir = std::env::temp_dir().join(format!("trindle-report-picture-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let big = dir.join("big.png");
    let mut bytes = picture::SIGNATURE.to_vec();
    bytes.resize(picture::MOST_BYTES + 1, 0);
    std::fs::write(&big, &bytes).unwrap();
    assert_eq!(picture::read(&big).err(), Some(Fault::TooBig));
    assert_eq!(picture::load(&bytes).err(), Some(Fault::TooBig));
    let good = dir.join("good.png");
    std::fs::write(&good, png(20, 10, false)).unwrap();
    assert_eq!(picture::read(&good).unwrap().size, (20, 10));
    assert_eq!(picture::read(&dir.join("missing.png")).err(), Some(Fault::Unreadable));
    assert_eq!(picture::read(&dir).err(), Some(Fault::Unreadable), "a folder");
    let pipe = dir.join("pipe.png");
    assert!(std::process::Command::new("mkfifo").arg(&pipe).status().unwrap().success());
    assert_eq!(picture::read(&pipe).err(), Some(Fault::Unreadable), "a pipe does not hold the reader");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_picture_the_computer_took_is_made_smaller_until_it_is_within_the_caps() {
    let big = image::DynamicImage::ImageRgba8(image::RgbaImage::from_fn(1500, 1500, |x, y| {
        let n = x.wrapping_mul(2654435761) ^ y.wrapping_mul(40503);
        image::Rgba([(n >> 3) as u8, (n >> 11) as u8, (n >> 19) as u8, 255])
    }));
    let Loaded { png, size, .. } = picture::fit_down(&big).unwrap();
    assert!(png.len() <= picture::MOST_BYTES && size.0 < 1500, "{} bytes, {size:?}", png.len());
    let wide = image::DynamicImage::ImageRgb8(image::RgbImage::new(12000, 20));
    let Loaded { size, .. } = picture::fit_down(&wide).unwrap();
    assert!(size.0 <= picture::MOST_SIDE, "{size:?}");
    let small = picture::fit_down(&image::DynamicImage::ImageRgb8(image::RgbImage::new(30, 20))).unwrap();
    assert_eq!(small.size, (30, 20), "a small one is not touched");
}

#[test]
fn base64_is_the_standard_text() {
    for (plain, text) in [("", ""), ("f", "Zg=="), ("fo", "Zm8="), ("foo", "Zm9v"), ("foob", "Zm9vYg=="), ("fooba", "Zm9vYmE="), ("foobar", "Zm9vYmFy")] {
        assert_eq!(picture::base64(plain.as_bytes()), text);
    }
    assert_eq!(picture::base64(&[0xfb, 0xff, 0xfe]), "+//+");
}

#[test]
fn the_picture_goes_out_in_the_picture_field_for_both_kinds_and_is_in_nothing_else() {
    for mode in [Mode::Note, Mode::Crash] {
        let r = with_picture(mode);
        let posted: serde_json::Value = serde_json::from_str(&r.body()).unwrap();
        let shown: serde_json::Value = serde_json::from_str(&r.text()).unwrap();
        assert!(shown.get("picture").is_none(), "not in the details, the saved file or the copy");
        assert_eq!(posted["kind"], mode.word());
        let mut without = posted.clone();
        let text = without.as_object_mut().unwrap().remove("picture").unwrap();
        assert_eq!(without, shown, "every other field is the same");
        assert_eq!(text.as_str().unwrap(), picture::base64(&r.picture.as_ref().unwrap().png));
        assert!(text.as_str().unwrap().starts_with("iVBOR"), "a PNG");
    }
    let none = made_without();
    assert_eq!(none.body(), none.text(), "no picture, no change");
}

fn made_without() -> Reporter {
    let mut r = with_picture(Mode::Note);
    r.picture = None;
    r
}

#[test]
fn send_posts_the_picture_to_the_server_and_a_server_refusal_is_red_and_loses_nothing() {
    let fake = Fake::start(400, r#"{"ok":false,"error":"\"picture\" must be a PNG"}"#, 200, r#"{"accepting":true}"#);
    let mut r = with_picture(Mode::Note);
    r.username = "john-smith".into();
    r.send();
    let body = r.outbox.pop().expect("posted");
    assert!(body.contains("\"picture\""));
    let answer = post(&fake.server, &body);
    let seen = fake.seen.lock().unwrap().last().cloned().unwrap();
    assert!(seen.line.starts_with("POST /") && seen.body.contains("\"picture\": \"iVBOR"));
    assert!(seen.body.len() == body.len() && seen.body.contains("It stopped working."));
    assert_eq!(answer, Answer::Refused("\"picture\" must be a PNG".into()));
    r.answered(answer);
    assert_eq!(r.refusal, "\"picture\" must be a PNG", "the server's words, shown red under the box");
    assert_eq!(r.typed, "It stopped working.");
    assert_eq!(r.username, "john-smith");
    assert!(!r.rejected && !r.sent && r.picture.is_some(), "the picture can be taken off and Send pressed again");
    r.update(Msg::PictureRemove);
    assert!(r.picture.is_none() && r.refusal.is_empty() && r.typed == "It stopped working.");
    r.send();
    assert!(!r.outbox.last().unwrap().contains("picture"));
    let mut plain = made_without();
    plain.send();
    plain.answered(Answer::Refused("No.".into()));
    assert!(plain.rejected, "a refusal with no picture is still for good");
}

#[test]
fn a_report_that_waits_keeps_its_picture_and_posts_it_whole_later() {
    let dir = std::env::temp_dir().join(format!("trindle-report-picture-wait-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let mut r = with_picture(Mode::Note);
    r.waiting_dir = dir.clone();
    r.send();
    r.answered(Answer::Later("No connection.".into()));
    assert!(r.waiting.is_some());
    let seen = std::cell::RefCell::new(Vec::new());
    let mut backoff = Backoff::new();
    let sent = flush_in(&dir, true, &mut backoff, 1000, &|_| true, &|body| {
        seen.borrow_mut().push(body.to_string());
        Answer::Taken(None)
    });
    let seen = seen.into_inner();
    assert_eq!((sent, seen.len()), (1, 1));
    assert_eq!(seen[0], r.posted);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_refused_picture_is_red_words_and_keeps_an_earlier_one() {
    let mut r = with_picture(Mode::Note);
    let had = r.picture.as_ref().unwrap().png.clone();
    r.picture_busy = true;
    r.picture_is(Err(Fault::NotPng));
    assert_eq!(r.picture_fault.as_deref(), Some(Fault::NotPng.words().as_str()));
    assert!(!r.picture_busy && r.picture.as_ref().unwrap().png == had);
    r.picture_is(picture::load(&png(9, 9, false)));
    assert!(r.picture_fault.is_none());
    assert_eq!(r.picture.as_ref().unwrap().size, (9, 9));
    for f in [Fault::NotPng, Fault::TooBig, Fault::TooLarge, Fault::Damaged, Fault::Unreadable, Fault::NoShot] {
        assert!(!f.words().is_empty() && !f.words().contains('{'), "{f:?}");
    }
}

#[test]
fn send_waits_while_a_picture_is_being_read_and_nothing_changes_once_sent() {
    let mut r = with_picture(Mode::Note);
    r.picture_busy = true;
    assert!(r.send_off().is_some());
    r.send();
    assert!(r.outbox.is_empty(), "not posted without the picture it was about to get");
    r.picture_busy = false;
    r.send();
    assert_eq!(r.outbox.len(), 1);
    r.update(Msg::PictureRemove);
    assert!(r.picture.is_some(), "after Send the picture stays as posted");
    r.update(Msg::PicturePick);
    r.update(Msg::PictureShot);
    assert!(!r.picture_busy, "nothing starts once sent, and nothing without a loop");
}

#[test]
fn the_window_with_a_picture_fits_a_short_screen_and_has_named_buttons() {
    for details in [false, true] {
        for room in [768.0 / 1.5, 768.0 / 1.5 - 48.0, 1000.0] {
            let mut r = with_picture(Mode::Note);
            r.details = details;
            r.picture_fault = Some(Fault::TooBig.words());
            r.fit_to(Some(room));
            let need = super::room::natural(r.view());
            assert!(need <= room + 0.5, "needs {need} of {room}, details {details}");
        }
    }
    let source = include_str!("mod.rs");
    for tip in ["report-picture-choose", "report-picture-shot", "report-picture-remove", "report-picture-name"] {
        assert!(source.contains(&format!("t!(\"{tip}\"")), "{tip} names its control");
    }
}

#[test]
fn the_screenshot_steps_the_window_aside_and_brings_it_back() {
    let window = include_str!("window.rs");
    let shoot = window.split("Do::Shoot =>").nth(1).and_then(|s| s.split("Do::Picture(result)").next()).unwrap_or_default();
    assert!(shoot.contains("window.minimise()") && shoot.contains("SHOT_WAIT") && shoot.contains("picture::screen()") && shoot.contains("raise_key(\"report\")"));
    assert!(shoot.contains("thread::spawn"), "photographed off the window's thread");
    assert!(window.contains("picture::read(&path)") && window.contains("thread::spawn(move || asks.send(Do::Picture"), "a chosen file is read on a thread");
}

#[test]
fn the_screenshot_is_the_overlay_and_escape_adds_nothing() {
    let source = include_str!("picture.rs");
    assert!(source.contains("\"--for-report\""), "the overlay is asked for the part, not the whole screen");
    assert!(!source.contains("capture::take"), "the whole screen is not photographed here");
    let nowhere = std::path::Path::new("/nonexistent/area.png");
    assert_eq!(picture::after_overlay(false, nowhere).err(), Some(Fault::Cancelled), "Escape in the overlay");
    assert_eq!(picture::after_overlay(true, nowhere).err(), Some(Fault::NoShot), "a part that was not left");
    assert_eq!(Fault::Cancelled.words(), "");
    let mut window = made_without();
    window.picture_busy = true;
    window.picture_is(Err(Fault::Cancelled));
    assert!(window.picture.is_none() && window.picture_fault.is_none() && !window.picture_busy, "nothing added, nothing said, Send open again");
}

#[test]
fn the_overlay_hands_the_part_over_only_when_asked_and_cancel_exits_with_one() {
    let source = include_str!("../screenshot/mod.rs");
    assert!(source.contains("\"--for-report\"") && source.contains("handing: self.hand_to.borrow().is_some()"));
    assert!(source.contains("life.code.set(1)") && source.contains("Ok(()) => self.code.set(0)"), "1 until a part is handed over");
    assert!(source.contains("Some(bus) if !handing"), "a report's overlay is its own copy, not the one Print shows");
}
