//! Tests for moving.rs.

#[cfg(test)]
use super::*;

fn mosaic() -> std::path::PathBuf {
    let p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("share/wallpapers/2026-10-the-moving-mosaic.webm");
    crate::common::needs::file_with(&p, "the moving mosaic film", "it ships in share/wallpapers: run the tests from a whole checkout of Trindle")
}

#[test]
fn the_mosaic_is_read_right() {
    let p = mosaic();
    let clip = Clip::open(&p).expect("a webm");
    assert_eq!((clip.info.width, clip.info.height), (1280, 720));
    assert!((clip.info.fps - 10.0).abs() < 0.01, "{}", clip.info.fps);
    assert_eq!(clip.len(), 16 * 10);
}

#[test]
fn the_mosaic_plays_round_and_round() {
    let p = mosaic();
    crate::common::needs::program("bwrap");
    let mut player = Player::open(&p).expect("plays");
    let mut out = vec![0u8; 1280 * 720 * 4];
    let n = player.clip.len();
    for _ in 0..n + 2 {
        assert_eq!(player.next(&mut out, 1280 * 4), Some((1280, 720)));
    }
    assert!(out.chunks_exact(4).all(|px| px[3] == 0xFF));
    assert!(out.chunks_exact(4).any(|px| px[0] != px[1] || px[1] != px[2]), "has colour");
}

#[test]
fn a_small_frame_is_small() {
    let p = mosaic();
    crate::common::needs::program("bwrap");
    let mut player = Player::open(&p).expect("a player");
    assert_eq!(player.frames(), 16 * 10);
    let mut out = vec![0u8; 320 * 180 * 4];
    assert_eq!(player.next_small(&mut out, 320 * 4, 4), Some((320, 180)));
    assert!(out.chunks_exact(4).all(|px| px[3] == 0xFF), "every pixel is filled in");
}

#[test]
fn the_first_frame_comes_small() {
    let p = mosaic();
    crate::common::needs::program("bwrap");
    let (w, h, rgb) = first_frame(&p, 8).expect("a frame");
    assert_eq!((w, h, rgb.len()), (160, 90, 160 * 90 * 3));
}

fn el(id: &[u8], claimed: u64, data: &[u8]) -> Vec<u8> {
    let mut out = id.to_vec();
    out.push(0x01);
    out.extend_from_slice(&claimed.to_be_bytes()[1..]);
    out.extend_from_slice(data);
    out
}

fn whole(id: &[u8], data: &[u8]) -> Vec<u8> {
    el(id, data.len() as u64, data)
}

fn film(width: u32, height: u32, claimed: u64, frame: &[u8]) -> Vec<u8> {
    let video = [whole(&[0xB0], &width.to_be_bytes()), whole(&[0xBA], &height.to_be_bytes())].concat();
    let entry = [whole(&[0xD7], &[1]), whole(&[0x83], &[1]), whole(&[0x86], b"V_VP8"), whole(&[0xE0], &video)].concat();
    let tracks = whole(&[0x16, 0x54, 0xAE, 0x6B], &whole(&[0xAE], &entry));
    let block = [&[0x81, 0, 0, 0x80][..], frame].concat();
    let cluster = [whole(&[0xE7], &[0]), el(&[0xA3], claimed, &block)].concat();
    let inside = [tracks, el(&[0x1F, 0x43, 0xB6, 0x75], cluster.len() as u64, &cluster)].concat();
    el(&[0x18, 0x53, 0x80, 0x67], inside.len() as u64, &inside)
}

fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-moving-test-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn a_film_is_never_believed_about_how_big_its_frames_are() {
    let dir = scratch("claims");
    let file = dir.join("film.webm");
    std::fs::write(&file, film(64, 48, 4 + 10, &[7; 10])).unwrap();
    let clip = Clip::open(&file).expect("a small film");
    assert_eq!((clip.info.width, clip.info.height, clip.frames.len(), clip.frames[0].1), (64, 48, 1, 10));
    let mut buf = Vec::new();
    assert!(clip.read(0, &mut buf) && buf == [7; 10]);
    std::fs::write(&file, film(64, 48, 3 << 30, &[7; 10])).unwrap();
    assert!(Clip::open(&file).is_none(), "a frame that ends past its file was believed");
    std::fs::write(&file, film(64, 48, MOST_FRAME + 4 + 1, &[7; 10])).unwrap();
    File::options().write(true).open(&file).unwrap().set_len(2 * MOST_FRAME).unwrap();
    assert!(Clip::open(&file).is_none(), "a frame of {} bytes was believed", MOST_FRAME + 1);
    for (w, h) in [(60_000, 60_000), (3841, 48), (64, 3841), (2161, 2161), (7680, 4320), (u32::MAX, u32::MAX)] {
        std::fs::write(&file, film(w, h, 4 + 10, &[7; 10])).unwrap();
        assert!(Clip::open(&file).is_none() && info(&file).is_none() && first_frame(&file, 1).is_none(), "a picture {w} by {h} was believed");
        assert_eq!(measure(&file).map(|i| (i.width, i.height)), Some((w, h)), "a picture {w} by {h} was not measured");
    }
    for (w, h) in [(3840, 2160), (2160, 3840)] {
        std::fs::write(&file, film(w, h, 4 + 10, &[7; 10])).unwrap();
        assert!(Clip::open(&file).is_some(), "4K, {w} by {h}, was refused");
    }
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_film_larger_than_the_screen_is_left_out() {
    use super::super::wallpaper_limits::{over_on, Over};
    let dir = scratch("capped");
    let file = dir.join("film.webm");
    std::fs::write(&file, film(1920, 1080, 4 + 10, &[7; 10])).unwrap();
    assert_eq!(over_on(&file, Some((1920, 1080))), Over::default());
    assert_eq!(over_on(&file, Some((1080, 1920))), Over::default(), "a screen turned on its side");
    assert_eq!(over_on(&file, Some((1366, 768))), Over { film_bigger: true, ..Over::default() });
    assert_eq!(over_on(&file, None), Over::default());
    let file = dir.join("huge.webm");
    std::fs::write(&file, film(7680, 4320, 4 + 10, &[7; 10])).unwrap();
    let o = over_on(&file, Some((3840, 2160)));
    assert!(o.pixels && crate::session::wallpaper_limits::refusal(o).as_deref() == Some("1 left out: larger than 4K."), "{o:?}");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_file_of_tiny_parts_is_read_a_piece_at_a_time() {
    let dir = scratch("parts");
    let file = dir.join("film.webm");
    let video = [whole(&[0xB0], &64u32.to_be_bytes()), whole(&[0xBA], &48u32.to_be_bytes())].concat();
    let entry = [whole(&[0xD7], &[1]), whole(&[0x83], &[1]), whole(&[0x86], b"V_VP8"), whole(&[0xE0], &video)].concat();
    let tracks = whole(&[0x16, 0x54, 0xAE, 0x6B], &whole(&[0xAE], &entry));
    let voids = [0xEC, 0x80].repeat(1 << 20);
    let block = [&[0x81, 0, 0, 0x80][..], &[7; 10]].concat();
    let cluster = [whole(&[0xE7], &[0]), whole(&[0xA3], &block)].concat();
    let inside = [tracks, voids, whole(&[0x1F, 0x43, 0xB6, 0x75], &cluster)].concat();
    std::fs::write(&file, whole(&[0x18, 0x53, 0x80, 0x67], &inside)).unwrap();
    let bytes = std::fs::metadata(&file).unwrap().len() as usize;
    let clip = Clip::open(&file).expect("a film");
    assert_eq!(clip.frames.len(), 1);
    assert!(clip.looks <= 3 * (bytes / PIECE) + 8, "a file of {bytes} bytes was read {} times", clip.looks);
    let slow = Clip::open_by(&file, 16).expect("the same film");
    assert_eq!((slow.frames.clone(), slow.info), (clip.frames.clone(), clip.info));
    assert!(slow.looks > 1000 * clip.looks, "{} against {}", slow.looks, clip.looks);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn frames_that_will_not_decode_are_passed_over_a_few_at_a_time() {
    crate::common::needs::program("bwrap");
    let dir = scratch("passed");
    let file = dir.join("film.webm");
    let video = [whole(&[0xB0], &64u32.to_be_bytes()), whole(&[0xBA], &48u32.to_be_bytes())].concat();
    let entry = [whole(&[0xD7], &[1]), whole(&[0x83], &[1]), whole(&[0x86], b"V_VP8"), whole(&[0xE0], &video)].concat();
    let tracks = whole(&[0x16, 0x54, 0xAE, 0x6B], &whole(&[0xAE], &entry));
    let block = [&[0x81, 0, 0, 0x80][..], &[7; 10]].concat();
    let blocks = whole(&[0xA3], &block).repeat(100);
    let cluster = [whole(&[0xE7], &[0]), blocks].concat();
    let inside = [tracks, whole(&[0x1F, 0x43, 0xB6, 0x75], &cluster)].concat();
    std::fs::write(&file, whole(&[0x18, 0x53, 0x80, 0x67], &inside)).unwrap();
    let mut player = Player::open(&file).expect("a player");
    assert_eq!(player.frames(), 100);
    let mut out = vec![0u8; 64 * 48 * 4];
    assert_eq!(player.next(&mut out, 64 * 4), None);
    assert_eq!(player.at, MOST_PASSED, "one asking went through {} frames that would not decode", player.at);
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_film_whose_first_frame_says_it_is_huge_gives_no_picture() {
    let dir = scratch("huge");
    let file = dir.join("film.webm");
    crate::common::needs::program("bwrap");
    let frame = super::super::decoding::tests::vp8_key(16383, 16383);
    std::fs::write(&file, film(64, 48, 4 + frame.len() as u64, &frame)).unwrap();
    let clip = Clip::open(&file).expect("a film by its own words");
    let mut buf = Vec::new();
    assert!(clip.read(0, &mut buf) && buf == frame);
    let mut player = Player::of(clip).expect("a decoder");
    assert_eq!(player.frame(1), Ok(None));
    assert!(first_frame(&file, 1).is_none());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_pipe_named_as_a_film_keeps_nothing_waiting() {
    use std::os::unix::fs::OpenOptionsExt;
    let dir = scratch("pipe");
    let pipe = dir.join("x.webm");
    let c = std::ffi::CString::new(pipe.to_str().unwrap()).unwrap();
    // SAFETY: a path that ends in a nought, for a pipe made in this test's own folder.
    assert_eq!(unsafe { libc::mkfifo(c.as_ptr(), 0o600) }, 0);
    let (tell, told) = std::sync::mpsc::channel();
    let asked = pipe.clone();
    std::thread::spawn(move || tell.send(Clip::open(&asked).is_none() && info(&asked).is_none()));
    let got = told.recv_timeout(std::time::Duration::from_secs(5));
    let _ = std::fs::OpenOptions::new().write(true).custom_flags(libc::O_NONBLOCK).open(&pipe);
    assert_eq!(got, Ok(true), "a pipe was waited on, or read as a film");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_film_is_known_by_its_name() {
    assert!(is_moving(Path::new("a.WebM")) && !is_moving(Path::new("a.png")));
}

pub const CHILD: &str = "TRINDLE_FILM_DECODER";

#[test]
fn the_boxed_decoder() {
    if let Ok(word) = std::env::var(CHILD) {
        std::process::exit(super::super::decoding::tests::stand_in(&word));
    }
}

fn with(name: &str, part: &str) -> (std::path::PathBuf, Option<Player>) {
    crate::common::needs::program("bwrap");
    let dir = scratch(name);
    let file = dir.join("film.webm");
    std::fs::write(&file, film(64, 48, 4 + 10, &[7; 10])).unwrap();
    let mut player = Player::unstarted(Clip::open(&file).expect("a film")).expect("room for it");
    player.part = part.into();
    let started = player.start();
    (dir, started.then_some(player))
}

fn gone(ear: UnixStream) -> bool {
    ear.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
    matches!((&ear).read(&mut [0u8; 1]), Ok(0))
}

fn ear(player: &Player) -> UnixStream {
    player.running.as_ref().expect("a decoder").talk.try_clone().unwrap()
}

#[test]
fn a_decoder_that_crashes_ends_alone_and_is_started_once_more_and_no_further() {
    let (dir, player) = with("crash", "abort");
    let mut player = player.expect("it starts, and says the film is this one");
    let mut out = vec![9u8; 64 * 48 * 4];
    for _ in 0..5 {
        assert_eq!(player.next(&mut out, 64 * 4), None);
    }
    assert_eq!((player.starts, player.endings, player.over), (2, 2, true), "starts, endings, over");
    assert!(player.running.is_none() && out.iter().all(|b| *b == 9), "what was showing was written over");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_decoder_ended_by_a_signal_is_started_again_where_the_film_had_reached() {
    let p = mosaic();
    crate::common::needs::program("bwrap");
    let (mut straight, mut broken) = (Player::open(&p).expect("plays"), Player::open(&p).expect("plays"));
    let (mut a, mut b) = (vec![0u8; 1280 * 720 * 4], vec![0u8; 1280 * 720 * 4]);
    for _ in 0..4 {
        assert_eq!(straight.next(&mut a, 1280 * 4), Some((1280, 720)));
        assert_eq!(broken.next(&mut b, 1280 * 4), Some((1280, 720)));
    }
    assert!(a == b);
    let was = b.clone();
    let (listening, box_) = (ear(&broken), broken.running.as_ref().unwrap().child.id() as i32);
    // SAFETY: a signal to the box this test started and still holds.
    assert_eq!(unsafe { libc::kill(box_, libc::SIGKILL) }, 0);
    assert!(gone(listening), "the decoder outlived its box");
    assert_eq!(broken.next(&mut b, 1280 * 4), None);
    assert!(b == was, "what was showing was written over");
    assert_eq!(straight.next(&mut a, 1280 * 4), Some((1280, 720)));
    assert_eq!(broken.next(&mut b, 1280 * 4), Some((1280, 720)));
    assert!(a == b && a != was, "the film went on from somewhere else");
    assert_eq!((broken.starts, broken.endings, broken.over, broken.at), (2, 1, false, 5));
}

#[test]
fn a_decoder_that_hangs_is_ended_and_not_started_again() {
    let (dir, player) = with("sleep", "sleep");
    let mut player = player.expect("it starts");
    player.wait = Duration::from_millis(500);
    let listening = ear(&player);
    let mut out = vec![9u8; 64 * 48 * 4];
    let began = Instant::now();
    assert_eq!(player.next(&mut out, 64 * 4), None);
    assert!(began.elapsed() < Duration::from_secs(4), "it was waited on for {:?}", began.elapsed());
    assert!(player.running.is_none() && player.over);
    assert!(gone(listening), "the decoder that hung was left running");
    assert_eq!(player.next(&mut out, 64 * 4), None);
    assert_eq!(player.starts, 1);
    assert!(out.iter().all(|b| *b == 9));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_decoder_that_spins_on_one_frame_is_ended_by_the_kernel() {
    let (dir, player) = with("spin", "spin");
    let mut player = player.expect("it starts");
    player.wait = Duration::from_secs(40);
    let mut out = vec![0u8; 64 * 48 * 4];
    let began = Instant::now();
    assert_eq!(player.next(&mut out, 64 * 4), None);
    assert!(began.elapsed() < Duration::from_secs(20), "it had the processor for {:?}", began.elapsed());
    assert_eq!((player.endings, player.over), (1, false), "it was not the kernel that ended it");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_decoder_that_asks_for_more_than_its_ceiling_is_ended() {
    let (dir, player) = with("greedy", "greedy");
    let mut player = player.expect("it starts");
    let mut out = vec![0u8; 64 * 48 * 4];
    assert_eq!(player.next(&mut out, 64 * 4), None, "it was given 4 GB");
    assert_eq!(player.endings, 1);
    let of = |width, height| ceiling(&Info { width, height, fps: 10.0, bytes: 0 });
    assert_eq!((of(1280, 720) >> 20, of(3840, 2160) >> 20, of(2160, 3840) >> 20), (412, 637, 637));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_decoder_that_says_a_frame_is_larger_than_the_film_is_refused() {
    for (said, step) in [((60_000, 60_000), 1), ((u32::MAX, u32::MAX), 1), ((65, 48), 1), ((64, 49), 1), ((0, 48), 1), ((64, 0), 1), ((33, 24), 2), ((32, 25), 2), ((64, 48), 2)] {
        let (dir, player) = with("says", &format!("says {} {}", said.0, said.1));
        let mut player = player.expect("it starts");
        let mut out = vec![9u8; 64 * 48 * 4];
        assert_eq!(player.next_small(&mut out, 64 * 4, step), None, "{said:?} at 1/{step} was believed");
        assert!(player.over && player.running.is_none() && out.iter().all(|b| *b == 9), "{said:?} at 1/{step}");
        assert_eq!(player.next_small(&mut out, 64 * 4, step), None);
        assert_eq!(player.starts, 1);
        std::fs::remove_dir_all(&dir).unwrap();
    }
    for (said, step) in [((64, 48), 1), ((1, 1), 1), ((32, 24), 2)] {
        let (dir, player) = with("says", &format!("says {} {}", said.0, said.1));
        let mut out = vec![9u8; 64 * 48 * 4];
        assert_eq!(player.expect("it starts").next_small(&mut out, 64 * 4, step), Some(said));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}

#[test]
fn a_decoder_that_read_another_film_is_not_used() {
    let (dir, player) = with("other", "other-film");
    assert!(player.is_none());
    let (_, honest) = with("other", "decode");
    assert!(honest.is_some());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_frame_that_does_not_fit_where_it_is_to_go_is_no_frame() {
    let (dir, player) = with("short", "says 64 48");
    let mut player = player.expect("it starts");
    let mut short = vec![9u8; 64 * 47 * 4];
    assert_eq!(player.next(&mut short, 64 * 4), None, "a frame was said to be there that did not fit");
    assert!(short.iter().all(|b| *b == 9), "part of a frame was written");
    let mut whole = vec![9u8; 64 * 48 * 4];
    assert_eq!(player.next(&mut whole, 64 * 4 - 1), None);
    assert!(whole.iter().all(|b| *b == 9));
    assert_eq!(player.next(&mut whole, 64 * 4), Some((64, 48)));
    assert!(!player.over());
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!((span(256, 256, 48), span(300, 256, 2), span(256, 256, 1)), (Some(256 * 48), Some(556), Some(256)));
    assert_eq!((span(255, 256, 48), span(256, 256, 0), span(256, 0, 4), span(usize::MAX, 4, 3)), (None, None, None, None));
}

#[test]
fn a_decoder_that_ends_for_want_of_its_library_is_told_from_any_other() {
    let begun = |part: &str| {
        crate::common::needs::program("bwrap");
        let dir = scratch("unstarted");
        let file = dir.join("film.webm");
        std::fs::write(&file, film(64, 48, 4 + 10, &[7; 10])).unwrap();
        let mut player = Player::unstarted(Clip::open(&file).expect("a film")).expect("room for it");
        player.part = part.into();
        let why = player.begin().err();
        std::fs::remove_dir_all(&dir).unwrap();
        why
    };
    assert_eq!(begun("no-library"), Some(Unstarted::NoLibrary));
    assert_eq!(begun("other-film"), Some(Unstarted::Other));
    assert_eq!(begun("not a part at all"), Some(Unstarted::Other));
    assert_eq!(begun("decode"), None);
    assert!(no_decoder_words(Unstarted::NoLibrary).contains("library that decodes films is not installed"));
    assert_ne!(no_decoder_words(Unstarted::NoLibrary), no_decoder_words(Unstarted::Other));
    let how = |part: &str| {
        let dir = scratch("unstarted-how");
        let file = dir.join("film.webm");
        std::fs::write(&file, film(64, 48, 4 + 10, &[7; 10])).unwrap();
        let mut player = Player::unstarted(Clip::open(&file).expect("a film")).expect("room for it");
        player.part = part.into();
        let how = player.begin_how().err();
        std::fs::remove_dir_all(&dir).unwrap();
        how
    };
    crate::common::needs::program("bwrap");
    assert_eq!(how("no-library"), Some((Unstarted::NoLibrary, How::Ended(Some(NO_LIBRARY)))));
    assert_eq!(how("other-film"), Some((Unstarted::Other, How::Another)));
    assert!(matches!(how("not a part at all"), Some((Unstarted::Other, How::Ended(Some(2) | None)))));
    assert_eq!(no_decoder_said(Unstarted::NoLibrary, How::Ended(Some(NO_LIBRARY))), no_decoder_words(Unstarted::NoLibrary));
    let said = no_decoder_said(Unstarted::Other, How::Ended(Some(1)));
    assert!(said.starts_with(no_decoder_words(Unstarted::Other)) && said.ends_with("(its box ended at once, with 1)"), "{said}");
    for h in [How::NoBox, How::Ended(None), How::Late, How::Another] {
        let said = no_decoder_said(Unstarted::Other, h);
        assert!(said.len() > no_decoder_words(Unstarted::Other).len() + 5 && !said.contains('\u{2014}'), "{said}");
    }
    let (dir, player) = with("unstarted-player", "no-library");
    assert!(player.is_none());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn the_decoder_can_open_nothing_write_nothing_and_reach_no_network() {
    let (dir, player) = with("prisoner", "prisoner");
    let mut out = vec![0u8; 64 * 48 * 4];
    assert_eq!(player.expect("it starts").next(&mut out, 64 * 4), Some((1, 1)), "one more for a file opened by name, two for a file made, four for a network");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn a_film_at_rest_or_no_longer_played_has_no_decoder() {
    let p = mosaic();
    crate::common::needs::program("bwrap");
    let (mut straight, mut rested) = (Player::open(&p).expect("plays"), Player::open(&p).expect("plays"));
    let (mut a, mut b) = (vec![0u8; 1280 * 720 * 4], vec![0u8; 1280 * 720 * 4]);
    for _ in 0..7 {
        assert_eq!(straight.next(&mut a, 1280 * 4), Some((1280, 720)));
        assert_eq!(rested.next(&mut b, 1280 * 4), Some((1280, 720)));
    }
    let listening = ear(&rested);
    rested.rest();
    assert!(rested.running.is_none() && gone(listening), "a film at rest kept its decoder");
    assert_eq!(straight.next(&mut a, 1280 * 4), Some((1280, 720)));
    assert_eq!(rested.next(&mut b, 1280 * 4), Some((1280, 720)));
    assert!(a == b, "after a rest the film went on from somewhere else");
    assert_eq!((rested.starts, rested.endings, rested.at), (2, 0, 8));
    let listening = ear(&rested);
    drop(rested);
    assert!(gone(listening), "a film no longer played kept its decoder");
    let hd = Info { width: 1280, height: 720, fps: 10.0, bytes: 0 };
    let large = Info { width: 3840, height: 2160, ..hd };
    assert_eq!((goes_on_from(159, &hd), goes_on_from(32, &large), goes_on_from(33, &large), goes_on_from(0, &large)), (159, 32, 0, 0));
}

#[test]
fn a_film_opened_on_a_thread_that_ends_keeps_its_decoder() {
    let p = mosaic();
    crate::common::needs::program("bwrap");
    let mut player = std::thread::spawn(move || Player::open(&p)).join().unwrap().expect("plays");
    std::thread::sleep(Duration::from_millis(500));
    let mut out = vec![0u8; 320 * 180 * 4];
    assert_eq!(player.next_small(&mut out, 320 * 4, 4), Some((320, 180)));
    assert_eq!(player.starts, 1);
}

#[test]
fn the_room_for_a_films_frames_keeps_its_length() {
    let room = Room::fixed(8192).expect("room");
    // SAFETY: both act on the room's own descriptor and no memory.
    let (shorter, longer) = unsafe { (libc::ftruncate(room.fd.as_raw_fd(), 0), libc::ftruncate(room.fd.as_raw_fd(), 1 << 20)) };
    assert_eq!((shorter, longer), (-1, -1), "the room's length was changed");
    assert_eq!(File::from(room.fd.try_clone().unwrap()).metadata().unwrap().len(), 8192);
    assert!(Room::of(room.fd.try_clone().unwrap(), 8192).is_some() && Room::of(room.fd.try_clone().unwrap(), 8193).is_none());
    let mut out = vec![9u8; 64];
    assert!(!room.rows(8192, (4, 3), &mut out, 16) && out.iter().all(|b| *b == 9), "a row was read from past the room's end");
    assert!(room.rows(4096, (4, 2), &mut out, 16));
    assert!(out[..32].iter().all(|b| *b == 0) && out[32..].iter().all(|b| *b == 9));
}

fn held_by(pid: u32, out: &mut Vec<(String, u64, u64, u64)>) {
    let read = |name: &str| std::fs::read_to_string(format!("/proc/{pid}/{name}")).unwrap_or_default();
    let figure = |text: &str, key: &str| text.lines().find(|l| l.starts_with(key)).and_then(|l| l.split_whitespace().nth(1)).and_then(|n| n.parse().ok()).unwrap_or(0);
    let status = read("status");
    out.push((read("cmdline").replace('\0', " ").chars().take(40).collect(), figure(&status, "VmRSS:"), figure(&read("smaps_rollup"), "Pss:"), figure(&status, "VmPeak:")));
    for child in read(&format!("task/{pid}/children")).split_whitespace().filter_map(|c| c.parse().ok()) {
        held_by(child, out);
    }
}

#[test]
#[ignore]
fn how_long_a_frame_takes_in_this_process_and_through_the_decoders() {
    let folder = std::env::var_os("TRINDLE_FILMS").map_or_else(|| std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("share/wallpapers"), Into::into);
    let mut films: Vec<_> = std::fs::read_dir(&folder).unwrap().flatten().map(|e| e.path()).filter(|p| is_moving(p)).collect();
    films.sort();
    let said = |times: &mut Vec<Duration>| {
        times.sort();
        let ms = |d: Duration| d.as_secs_f64() * 1000.0;
        format!("mean {:.2} ms, 99 in 100 under {:.2} ms, longest {:.2} ms", ms(times.iter().sum::<Duration>()) / times.len() as f64, ms(times[times.len() * 99 / 100]), ms(times[times.len() - 1]))
    };
    for path in films {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        for step in [1usize, 4] {
            let clip = Clip::open(&path).expect("a film");
            let (w, h, n) = (clip.info.width as usize / step, clip.info.height as usize / step, clip.len());
            let (mut out, mut buf) = (vec![0u8; w * h * 4], Vec::new());
            let mut here = super::super::decoding::tests::Here::new(&clip);
            let mut in_this = Vec::new();
            for i in 0..3 * n {
                let began = Instant::now();
                assert!(here.frame(&clip, i % n, &mut buf, &mut out, step));
                in_this.push(began.elapsed());
            }
            drop(here);
            let began = Instant::now();
            let mut player = Player::of(clip).expect("plays");
            let starting = began.elapsed();
            let mut through = Vec::new();
            for _ in 0..3 * n {
                let began = Instant::now();
                assert!(player.next_small(&mut out, w * 4, step).is_some(), "{name}: the decoder gave no frame");
                through.push(began.elapsed());
            }
            let mut held = Vec::new();
            held_by(player.running.as_ref().unwrap().child.id(), &mut held);
            println!("{name} at 1/{step}, {n} frames three times round");
            println!("  in this process:      {}", said(&mut in_this));
            println!("  through the decoder:  {}; starting it took {:.1} ms", said(&mut through), starting.as_secs_f64() * 1000.0);
            for (what, resident, share, most) in held {
                println!("  {what:<40} resident {resident} kB, its share {share} kB, address space at most {most} kB");
            }
        }
    }
}
