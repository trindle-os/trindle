//! trindle-wallpaper: shows the wallpaper (see backdrop) and rotates pictures from a folder.
//! SIGHUP rereads settings, SIGUSR1 skips ahead; a second instance just signals the first.

use std::cell::RefCell;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::os::unix::net::UnixStream;
use std::time::Instant;

use crate::common::files::{gently, setting, START_WAIT};
use crate::common::renewed;
use crate::toolkit::event::{Flow, Loop};

const PICTURES: &[&str] = &["png", "jpg", "jpeg", "webp", "bmp", "tif", "tiff", "gif", "webm"];
const FITS: &[&str] = &["fill", "fit", "center", "stretch", "tile"];
const MIN_EVERY: u64 = 5;

const HANDS_OVER_AFTER: std::time::Duration = std::time::Duration::from_secs(1);
const HAND_OVER_TRIES: u32 = 3;

fn pidfile() -> PathBuf {
    crate::common::files::scratch().join("trindle-wallpaper.pid")
}

fn showing_file() -> PathBuf {
    crate::common::files::scratch().join("trindle-wallpaper-showing")
}

pub fn showing() -> Option<PathBuf> {
    let text = std::fs::read_to_string(showing_file()).ok()?;
    let path = PathBuf::from(text.trim());
    gently(&path, START_WAIT, |p| p.is_file()).unwrap_or(false).then_some(path)
}

pub const PICTURE_WAIT: std::time::Duration = std::time::Duration::from_secs(5);

const LOOK_AGAIN: std::time::Duration = std::time::Duration::from_secs(30);

#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {

    Colour,
    Picture,
    Folder,

    Unknown,
}

fn kind_of(path: &Path) -> Kind {
    if path.as_os_str().is_empty() {
        return Kind::Colour;
    }
    match gently(path, START_WAIT, |p| std::fs::metadata(p).map(|m| (m.is_file(), m.is_dir()))) {
        None => Kind::Unknown,
        Some(Ok((true, _))) => Kind::Picture,
        Some(Ok((_, true))) => Kind::Folder,
        Some(_) => Kind::Colour,
    }
}

pub fn skipped() -> std::collections::HashSet<String> {
    setting("wallpaper_skip").split('\t').filter(|s| !s.is_empty()).map(String::from).collect()
}

pub fn pictures_used(folder: &Path) -> Vec<PathBuf> {
    let skip = skipped();
    pictures_in(folder).into_iter()
        .filter(|p| !skip.contains(&p.file_name().unwrap_or_default().to_string_lossy().to_string()))
        .collect()
}

pub const MOST_LOOKED_AT: usize = 20_000;

pub struct Found {

    pub pictures: Vec<PathBuf>,

    pub more: bool,

    pub left: super::wallpaper_limits::Left,
}

pub fn pictures_in(folder: &Path) -> Vec<PathBuf> {
    looked_in(folder).pictures
}

pub fn looked_in(folder: &Path) -> Found {
    use std::os::unix::fs::MetadataExt;
    let mut found = Vec::new();
    let mut walked = std::collections::HashSet::new();
    let mut stack = vec![folder.to_path_buf()];
    let (mut looked, mut more) = (0usize, false);
    'walk: while let Some(dir) = stack.pop() {
        let Ok(meta) = std::fs::metadata(&dir) else { continue };
        if !walked.insert((meta.dev(), meta.ino())) {
            continue;
        }
        let Ok(entries) = std::fs::read_dir(&dir) else { continue };
        for e in entries.flatten() {
            if looked == MOST_LOOKED_AT {
                more = true;
                break 'walk;
            }
            looked += 1;
            if e.file_name().as_encoded_bytes().first() == Some(&b'.') {
                continue;
            }
            let p = e.path();

            let kind = match e.file_type() {
                Ok(kind) if !kind.is_symlink() => kind,
                _ => match std::fs::metadata(&p) {
                    Ok(meta) => meta.file_type(),
                    Err(_) => continue,
                },
            };
            if kind.is_dir() {
                stack.push(p);
            } else if kind.is_file() && p.extension().is_some_and(|x| PICTURES.contains(&x.to_string_lossy().to_lowercase().as_str())) {
                found.push(p);
            }
        }
    }
    found.sort_by_key(|p| p.to_string_lossy().to_lowercase());
    let mut left = super::wallpaper_limits::Left::default();
    found.retain(|p| {
        let over = super::wallpaper_limits::over(p);
        left.add(over);
        !over.any()
    });
    Found { pictures: found, more, left }
}

pub fn own_pictures() -> std::path::PathBuf {
    crate::common::files::share("wallpapers")
}

pub fn default_picture() -> std::path::PathBuf {
    own_pictures().join("2026-10-the-moving-mosaic.webm")
}

struct Conf {
    source: PathBuf,
    kind: Kind,
    looked_at: Instant,
    every: u64,
    shuffle: bool,
    fit: String,
    color: String,
}

fn conf() -> Conf {
    let source = setting("wallpaper");
    let source = match source.as_str() {
        "" => default_picture().to_string_lossy().to_string(),

        "none" => String::new(),
        _ => source,
    };
    let source = match source.strip_prefix("~/") {
        Some(rest) => crate::common::files::home().join(rest),
        None => PathBuf::from(source),
    };

    let (source, kind) = match kind_of(&source) {
        Kind::Colour if !source.as_os_str().is_empty() => (default_picture(), kind_of(&default_picture())),
        kind => (source, kind),
    };

    let (source, kind) = if kind == Kind::Picture && too_large(&source) {
        (default_picture(), kind_of(&default_picture()))
    } else {
        (source, kind)
    };
    let fit = setting("wallpaper_fit");
    Conf {
        source,
        kind,
        looked_at: Instant::now(),
        every: setting("wallpaper_every").parse().unwrap_or(300).max(MIN_EVERY),
        shuffle: setting("wallpaper_order") != "name",
        fit: if FITS.contains(&fit.as_str()) { fit } else { "fill".into() },
        color: { let c = setting("wallpaper_color"); if c.is_empty() { "#09090b".into() } else { c } },
    }
}

fn too_large(picture: &Path) -> bool {
    gently(picture, START_WAIT, |p| super::wallpaper_limits::over(p).any()).unwrap_or(false)
}

struct Changer {
    screen: Option<Rc<super::backdrop::Backdrop>>,
    queue: Vec<PathBuf>,
    shown_at: Option<Instant>,
}

impl Changer {
    fn next_picture(&mut self, c: &Conf) -> Option<PathBuf> {
        match c.kind {
            Kind::Picture => return Some(c.source.clone()),
            Kind::Folder => {}
            Kind::Colour | Kind::Unknown => return None,
        }
        if self.queue.is_empty() {
            self.queue = gently(&c.source, PICTURE_WAIT, pictures_used).unwrap_or_default();
            if c.shuffle {
                shuffle(&mut self.queue);
            }
        }
        if self.queue.is_empty() { None } else { Some(self.queue.remove(0)) }
    }

    fn next_look(&mut self, c: &Conf) -> super::backdrop::Look {
        let picture = self.next_picture(c);
        if let Some(p) = &picture {
            let _ = std::fs::write(showing_file(), p.to_string_lossy().as_bytes());
        }
        self.shown_at = Some(Instant::now());
        super::backdrop::Look { picture, fit: c.fit.clone(), color: c.color.clone() }
    }

    fn show(&mut self, c: &Conf) {
        let look = self.next_look(c);
        if let Some(screen) = &self.screen {
            screen.show(look);
        }
    }
}

fn shuffle(v: &mut [PathBuf]) {
    let mut seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(1);
    for i in (1..v.len()).rev() {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        v.swap(i, (seed % (i as u64 + 1)) as usize);
    }
}

fn named_picture(named: &Path) -> Option<PathBuf> {
    let path = std::fs::canonicalize(named).unwrap_or_else(|_| named.to_path_buf());
    path.is_file().then_some(path)
}

fn running() -> Option<i32> {
    let pid: i32 = std::fs::read_to_string(pidfile()).ok()?.trim().parse().ok()?;
    let cmdline = std::fs::read(format!("/proc/{pid}/cmdline")).ok()?;
    let text = String::from_utf8_lossy(&cmdline);
    (text.contains("trindle-wallpaper") && pid != std::process::id() as i32).then_some(pid)
}

pub fn main(args: Vec<String>) -> i32 {

    if let Some(picture) = args.windows(2).find(|w| w[0] == "--set").map(|w| w[1].clone()) {
        if !set_wallpaper(&picture) {
            return 1;
        }
    }

    let old = if args.iter().any(|a| a == renewed::RENEWED) {
        let Some(old) = renewed::from_old() else { return 1 };
        Some(old)
    } else {
        None
    };
    if let Some(pid) = running().filter(|_| old.is_none()) {

        let sig = if args.iter().any(|a| a == "--next") { libc::SIGUSR1 } else { libc::SIGHUP };
        // SAFETY: a signal to the pid in the pidfile, which `running` has just

        unsafe { libc::kill(pid, sig) };
        return 0;
    }
    let _ = std::fs::write(pidfile(), std::process::id().to_string());

    let events = Loop::new();
    let settings = Rc::new(RefCell::new(conf()));
    let changer = Rc::new(RefCell::new(Changer { screen: None, queue: Vec::new(), shown_at: None }));
    let first = first_look(&changer, &settings, old.is_some());

    let ml = events.clone();
    let Some(screen) = super::backdrop::Backdrop::start(first, &events, move || ml.quit()) else {
        let _ = std::fs::remove_file(pidfile());
        return 1;
    };
    if let Some(old) = old {
        tell_old_when_drawn(&events, &screen, old);
    }
    changer.borrow_mut().screen = Some(screen);
    hand_over_after_update(&events);
    on_signals(&events, &changer, &settings);
    change_with_time(&events, &changer, &settings);
    events.run();
    0
}

fn set_wallpaper(picture: &str) -> bool {

    let named = gently(Path::new(picture), PICTURE_WAIT, named_picture).flatten();
    let Some(path) = named else { return false };
    !too_large(&path) && crate::common::files::set_setting("wallpaper", &path.to_string_lossy()).is_ok()
}

fn first_look(changer: &Rc<RefCell<Changer>>, settings: &Rc<RefCell<Conf>>, renewed: bool) -> super::backdrop::Look {
    let (mut ch, c) = (changer.borrow_mut(), settings.borrow());
    match showing().filter(|_| renewed && c.kind == Kind::Folder) {
        Some(picture) => {
            ch.shown_at = Some(Instant::now());
            super::backdrop::Look { picture: Some(picture), fit: c.fit.clone(), color: c.color.clone() }
        }
        None => ch.next_look(&c),
    }
}

fn tell_old_when_drawn(events: &Loop, screen: &Rc<super::backdrop::Backdrop>, mut old: UnixStream) {
    let (screen, mut drawn, mut looks) = (screen.clone(), None::<Instant>, 0);
    events.every(std::time::Duration::from_millis(100), move || {
        looks += 1;
        if drawn.is_none() && (screen.drawn() || looks > 100) {
            drawn = Some(Instant::now());
        }
        if drawn.is_some_and(|at| at.elapsed() >= HANDS_OVER_AFTER) {
            let _ = std::io::Write::write_all(&mut old, b"1");
            return Flow::Stop;
        }
        Flow::Continue
    });
}

fn hand_over_after_update(events: &Loop) {
    let (ev, trying, mut tries) = (events.clone(), Rc::new(std::cell::Cell::new(false)), 0);
    renewed::watch_with(events, move |program| {
        if trying.get() {
            return Flow::Continue;
        }
        tries += 1;
        if tries > HAND_OVER_TRIES {
            return Flow::Stop;
        }

        let Some((mut child, mut new)) = renewed::beside(program, "trindle-wallpaper") else { return Flow::Stop };
        std::thread::spawn(move || child.wait());
        let _ = new.set_nonblocking(true);
        trying.set(true);
        let (quit, trying) = (ev.clone(), trying.clone());
        ev.watch(std::os::fd::AsRawFd::as_raw_fd(&new), move |hung_up| {
            let mut word = [0u8; 1];
            match std::io::Read::read(&mut new, &mut word) {

                Ok(1) => quit.quit(),
                Err(e) if !hung_up && matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::Interrupted) => return Flow::Continue,

                _ => trying.set(false),
            }
            Flow::Stop
        });
        Flow::Continue
    });
}

fn on_signals(events: &Loop, changer: &Rc<RefCell<Changer>>, settings: &Rc<RefCell<Conf>>) {
    let (ch, st) = (changer.clone(), settings.clone());
    events.on_signal(libc::SIGHUP, move || {
        *st.borrow_mut() = conf();
        ch.borrow_mut().queue.clear();
        ch.borrow_mut().show(&st.borrow());
        Flow::Continue
    });
    let (ch, st) = (changer.clone(), settings.clone());
    events.on_signal(libc::SIGUSR1, move || {
        ch.borrow_mut().show(&st.borrow());
        Flow::Continue
    });
    let ml = events.clone();
    events.on_signal(libc::SIGTERM, move || {
        ml.quit();
        Flow::Stop
    });
}

fn change_with_time(events: &Loop, changer: &Rc<RefCell<Changer>>, settings: &Rc<RefCell<Conf>>) {
    let (ch, st) = (changer.clone(), settings.clone());
    events.every(std::time::Duration::from_secs(1), move || {

        let again = { let c = st.borrow(); c.kind == Kind::Unknown && c.looked_at.elapsed() >= LOOK_AGAIN };
        if again {
            *st.borrow_mut() = conf();
            if st.borrow().kind != Kind::Unknown {
                ch.borrow_mut().queue.clear();
                ch.borrow_mut().show(&st.borrow());
            }
        }
        let c = st.borrow();
        let due = c.kind == Kind::Folder && ch.borrow().shown_at.is_some_and(|t| t.elapsed().as_secs() >= c.every);
        if due {
            ch.borrow_mut().show(&c);
        }
        Flow::Continue
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-wallpaper-test-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn within_five_seconds(folder: &Path) -> Option<Vec<PathBuf>> {
        let (tell, told) = std::sync::mpsc::channel();
        let folder = folder.to_path_buf();
        std::thread::spawn(move || tell.send(pictures_in(&folder)));
        told.recv_timeout(std::time::Duration::from_secs(5)).ok()
    }

    #[test]
    fn a_folder_of_pictures_is_walked_once_and_only_files_are_pictures() {
        use std::os::unix::fs::symlink;
        let root = scratch("walk");
        let (folder, other) = (root.join("in/Wallpapers"), root.join("elsewhere/holiday"));
        std::fs::create_dir_all(folder.join("sub")).unwrap();
        std::fs::create_dir_all(&other).unwrap();
        for file in [folder.join("a.png"), folder.join("sub/b.JPG"), other.join("c.webp"), folder.join("notes.txt"), folder.join(".hidden.png")] {
            std::fs::write(file, b"x").unwrap();
        }
        symlink(".", folder.join("round")).unwrap();
        symlink("..", folder.join("up")).unwrap();
        symlink(&other, folder.join("far")).unwrap();
        symlink(folder.join("a.png"), folder.join("l.png")).unwrap();
        symlink(folder.join("gone.png"), folder.join("d.png")).unwrap();
        std::fs::create_dir_all(folder.join("folder.png")).unwrap();
        let pipe = folder.join("x.jpg");
        let c = std::ffi::CString::new(pipe.to_str().unwrap()).unwrap();
        // SAFETY: a path that ends in a nought, for a pipe made in this test's own folder.
        assert_eq!(unsafe { libc::mkfifo(c.as_ptr(), 0o600) }, 0);
        symlink(&pipe, folder.join("y.jpg")).unwrap();
        let found = within_five_seconds(&folder).expect("the walk went round and round");
        let names: Vec<String> = found.iter().map(|p| p.strip_prefix(&folder).unwrap().to_string_lossy().into_owned()).collect();
        assert_eq!(names, ["a.png", "far/c.webp", "l.png", "sub/b.JPG"]);
        assert!(!looked_in(&folder).more);
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn a_folder_with_more_than_is_looked_through_says_so() {
        let folder = scratch("many");
        for n in 0..MOST_LOOKED_AT + 5 {
            std::fs::write(folder.join(format!("{n}.png")), b"").unwrap();
        }
        let found = looked_in(&folder);
        assert!(found.more && found.pictures.len() == MOST_LOOKED_AT, "{} {}", found.more, found.pictures.len());
        for n in 0..5 {
            std::fs::remove_file(folder.join(format!("{n}.png"))).unwrap();
        }
        assert!(!looked_in(&folder).more, "a folder of just as many as are looked at is not said to hold more");
        std::fs::remove_dir_all(&folder).unwrap();
    }

    fn big_png(path: &Path) {
        image::RgbImage::new(3841, 16).save(path).unwrap();
    }

    #[test]
    fn a_walk_leaves_out_what_is_too_large_and_counts_it() {
        let folder = scratch("limits");
        image::RgbImage::new(3840, 2160).save(folder.join("a-fits.png")).unwrap();
        big_png(&folder.join("b-wide.png"));
        std::fs::File::create(folder.join("c-heavy.jpg")).unwrap().set_len(super::super::wallpaper_limits::MOST_STILL_BYTES + 1).unwrap();
        std::fs::File::create(folder.join("d-heavy.webm")).unwrap().set_len(super::super::wallpaper_limits::MOST_MOVING_BYTES + 1).unwrap();
        let found = looked_in(&folder);
        assert_eq!(found.pictures, [folder.join("a-fits.png")]);
        assert_eq!(found.left.words().as_deref(), Some("3 left out: larger than 4K or over 20 MB (200 MB if moving)."));
        assert_eq!(pictures_used(&folder), [folder.join("a-fits.png")]);
        std::fs::remove_dir_all(&folder).unwrap();
    }

    #[test]
    fn the_program_will_not_show_or_set_what_is_too_large() {
        let folder = scratch("program");
        let (big, fine) = (folder.join("big.png"), folder.join("fine.png"));
        big_png(&big);
        image::RgbImage::new(64, 48).save(&fine).unwrap();
        assert!(too_large(&big) && !too_large(&fine));
        assert_eq!(main(vec!["--set".into(), big.to_string_lossy().into_owned()]), 1);
        std::fs::remove_dir_all(&folder).unwrap();
    }
}
