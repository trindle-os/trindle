//! How big things are drawn on a screen.

use std::fmt::Write as _;
use std::io::{Read, Write};
use std::os::unix::fs::{MetadataExt, OpenOptionsExt};
use std::path::{Path, PathBuf};

pub const FOLDER: &str = "/var/lib/trindle/login-scales";
const FOLDER_ENV: &str = "TRINDLE_LOGIN_SCALES";
const TARGET_DPI: f64 = 125.0;
const STEPS: f64 = 4.0;
pub const SMALLEST: f64 = 1.0;
pub const LARGEST: f64 = 3.0;
pub const LOGIN_ROOM: f64 = 600.0;
const MOST: usize = 16;
const MOST_BYTES: u64 = 4096;
const LONGEST: usize = 64;
const FIRST_PERSON: u32 = 1000;

#[derive(Clone, Debug, PartialEq)]
pub struct Screen {
    pub connector: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub pixels: (i32, i32),
    pub millimetres: (i32, i32),
}

impl Screen {
    pub fn built_in(&self) -> bool {
        ["eDP", "LVDS", "DSI"].iter().any(|p| self.connector.starts_with(p))
    }

    pub fn worked_out(&self) -> f64 {
        let (across, mm) = (self.pixels.0 as f64, self.millimetres.0 as f64);
        let scale = if mm >= 100.0 && across > 0.0 {
            (across / (mm / 25.4) / TARGET_DPI * STEPS).round() / STEPS
        } else if self.built_in() {
            1.25
        } else {
            1.0
        };
        scale.clamp(SMALLEST, LARGEST)
    }

    fn names(&self) -> [String; 3] {
        [clean(&self.make), clean(&self.model), clean(&self.serial)]
    }
}

fn clean(text: &str) -> String {
    text.chars().map(|c| if c.is_control() { ' ' } else { c }).take(LONGEST).collect::<String>().trim().to_string()
}

pub fn for_login(screen: &Screen) -> f64 {
    let wanted = remembered(&folder(), screen).unwrap_or_else(|| screen.worked_out());
    fit(wanted, screen.pixels.1)
}

pub fn fit(scale: f64, tall: i32) -> f64 {
    let room = if tall > 0 { tall as f64 / LOGIN_ROOM } else { LARGEST };
    scale.min(room).clamp(SMALLEST, LARGEST)
}

fn folder() -> PathBuf {
    std::env::var_os(FOLDER_ENV).filter(|f| !f.is_empty()).map(PathBuf::from).unwrap_or_else(|| PathBuf::from(FOLDER))
}

#[derive(Clone, Debug, PartialEq)]
struct Kept {
    names: [String; 3],
    across: i32,
    scale: f64,
}

fn parse(text: &str) -> Vec<Kept> {
    text.lines().filter_map(|line| {
        let parts: Vec<&str> = line.split('\t').collect();
        let [make, model, serial, across, scale] = parts.as_slice() else { return None };
        let across: i32 = across.parse().ok().filter(|a| *a > 0)?;
        let scale: f64 = scale.parse().ok().filter(|s: &f64| s.is_finite())?;
        Some(Kept { names: [make.to_string(), model.to_string(), serial.to_string()], across, scale: scale.clamp(SMALLEST, LARGEST) })
    }).collect()
}

fn write_out(kept: &[Kept]) -> String {
    kept.iter().fold(String::new(), |mut out, k| {
        let _ = writeln!(out, "{}\t{}\t{}\t{}\t{}", k.names[0], k.names[1], k.names[2], k.across, k.scale);
        out
    })
}

fn remembered(folder: &Path, screen: &Screen) -> Option<f64> {
    let names = screen.names();
    let mut files: Vec<(i64, Vec<Kept>)> = std::fs::read_dir(folder).ok()?
        .filter_map(|e| e.ok())
        .filter_map(|e| read_trusted(&e.path()))
        .collect();
    files.sort_by_key(|(when, _)| std::cmp::Reverse(*when));
    let kept = files.into_iter().find_map(|(_, all)| all.into_iter().find(|k| k.names == names))?;
    let scale = if screen.pixels.0 > 0 && screen.pixels.0 != kept.across {
        kept.scale * screen.pixels.0 as f64 / kept.across as f64
    } else {
        kept.scale
    };
    Some(scale.clamp(SMALLEST, LARGEST))
}

fn read_trusted(path: &Path) -> Option<(i64, Vec<Kept>)> {
    read_login_file(path, MOST_BYTES).map(|(when, text)| (when, parse(&text)))
}

pub fn read_login_file(path: &Path, most: u64) -> Option<(i64, String)> {
    let uid: u32 = path.file_name()?.to_str()?.parse().ok().filter(|u| *u >= FIRST_PERSON)?;
    let file = std::fs::OpenOptions::new().read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(path).ok()?;
    let meta = file.metadata().ok()?;
    if !meta.is_file() || meta.uid() != uid {
        return None;
    }
    let mut text = String::new();
    file.take(most).read_to_string(&mut text).ok()?;
    Some((meta.mtime(), text))
}

pub fn login_files(folder: &Path, most: u64) -> Vec<(i64, String)> {
    let Ok(dir) = std::fs::read_dir(folder) else { return Vec::new() };
    let mut files: Vec<(i64, String)> = dir.filter_map(|e| e.ok()).filter_map(|e| read_login_file(&e.path(), most)).collect();
    files.sort_by_key(|(when, _)| std::cmp::Reverse(*when));
    files
}

pub fn remember(screens: &[(Screen, f64)]) {
    if let Err(e) = remember_in(&folder(), screens) {
        eprintln!("trindle: could not keep the screens' scales for the login screen: {e}");
    }
}

fn remember_in(folder: &Path, screens: &[(Screen, f64)]) -> std::io::Result<()> {
    if !folder.is_dir() {
        return Ok(());
    }
    // SAFETY: asks the kernel for this process's own user id. It reads
    // nothing and cannot fail.
    let me = unsafe { libc::getuid() };
    let path = folder.join(me.to_string());
    let mut kept: Vec<Kept> = screens.iter()
        .filter(|(s, _)| s.pixels.0 > 0)
        .map(|(s, scale)| Kept { names: s.names(), across: s.pixels.0, scale: scale.clamp(SMALLEST, LARGEST) })
        .collect();
    if let Some((_, before)) = read_trusted(&path) {
        for k in before {
            if !kept.iter().any(|n| n.names == k.names) {
                kept.push(k);
            }
        }
    }
    kept.truncate(MOST);
    write_login_file(folder, &write_out(&kept), MOST_BYTES)
}

pub fn write_login_file(folder: &Path, text: &str, most: u64) -> std::io::Result<()> {
    if !folder.is_dir() {
        return Ok(());
    }
    // SAFETY: asks the kernel for this process's own user id. It reads
    // nothing and cannot fail.
    let me = unsafe { libc::getuid() };
    let path = folder.join(me.to_string());
    if read_login_file(&path, most).is_some_and(|(_, before)| before == text) {
        let now = std::fs::FileTimes::new().set_modified(std::time::SystemTime::now());
        return std::fs::File::options().write(true).custom_flags(libc::O_NOFOLLOW).open(&path)?.set_times(now);
    }
    let mut file = std::fs::OpenOptions::new().write(true).create(true).truncate(false).mode(0o644)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(&path)?;
    let meta = file.metadata()?;
    if !meta.is_file() || meta.uid() != me {
        return Err(std::io::Error::other("the file there is not this person's own"));
    }
    file.set_len(0)?;
    file.write_all(text.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn screen(connector: &str, pixels: (i32, i32), millimetres: (i32, i32)) -> Screen {
        Screen { connector: connector.into(), make: "BOE".into(), model: "NE135A1M-NY1".into(), serial: "".into(), pixels, millimetres }
    }

    #[test]
    fn worked_out_from_the_screens_size() {
        assert_eq!(screen("eDP-1", (2256, 1504), (285, 190)).worked_out(), 1.5);
        assert_eq!(screen("HDMI-A-1", (1920, 1080), (698, 393)).worked_out(), 1.0);
        assert_eq!(screen("eDP-1", (1920, 1080), (310, 174)).worked_out(), 1.25);
        assert_eq!(screen("DP-1", (3840, 2160), (597, 336)).worked_out(), 1.25);
        assert_eq!(screen("eDP-1", (1920, 1080), (0, 0)).worked_out(), 1.25);
        assert_eq!(screen("HDMI-A-1", (1920, 1080), (0, 0)).worked_out(), 1.0);
        assert_eq!(screen("DSI-1", (3840, 2400), (120, 75)).worked_out(), 3.0);
    }

    #[test]
    fn the_login_screen_always_fits() {
        assert_eq!(fit(1.75, 1504), 1.75);
        assert_eq!(fit(2.0, 900), 1.5);
        assert_eq!(fit(1.5, 500), 1.0);
        assert_eq!(fit(0.5, 1080), 1.0);
        assert_eq!(fit(9.0, 0), 3.0);
    }

    fn scratch(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("trindle-scales-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn a_kept_scale_comes_back_for_the_same_screen_only() {
        let d = scratch("same");
        let laptop = screen("eDP-1", (2256, 1504), (285, 190));
        let other = Screen { model: "Other".into(), ..laptop.clone() };
        remember_in(&d, &[(laptop.clone(), 1.75)]).unwrap();
        assert_eq!(remembered(&d, &laptop), Some(1.75));
        assert_eq!(remembered(&d, &other), None);
        let smaller = Screen { pixels: (1128, 752), ..laptop.clone() };
        assert_eq!(remembered(&d, &smaller), Some(1.0));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn other_screens_are_kept_when_one_changes() {
        let d = scratch("keep");
        let laptop = screen("eDP-1", (2256, 1504), (285, 190));
        let desk = Screen { connector: "HDMI-A-1".into(), make: "Dell".into(), model: "U3223".into(), serial: "X1".into(),
                            pixels: (1920, 1080), millimetres: (698, 393) };
        remember_in(&d, &[(laptop.clone(), 1.75)]).unwrap();
        remember_in(&d, &[(desk.clone(), 1.0)]).unwrap();
        assert_eq!(remembered(&d, &laptop), Some(1.75));
        assert_eq!(remembered(&d, &desk), Some(1.0));
        remember_in(&d, &[(laptop.clone(), 2.0)]).unwrap();
        assert_eq!(remembered(&d, &laptop), Some(2.0));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn untrusted_files_are_passed_over() {
        let d = scratch("trust");
        let laptop = screen("eDP-1", (2256, 1504), (285, 190));
        let line = "BOE\tNE135A1M-NY1\t\t2256\t2.5\n";
        std::fs::write(d.join("4242"), line).unwrap();
        std::fs::write(d.join("notes"), line).unwrap();
        std::os::unix::fs::symlink(d.join("4242"), d.join("999999")).unwrap();
        assert_eq!(remembered(&d, &laptop), None);
        assert_eq!(parse("a\tb\n\tx\ty\t0\t2\nm\tn\ts\t100\tNaN\nm\tn\ts\t100\t7\n"),
                   vec![Kept { names: ["m".into(), "n".into(), "s".into()], across: 100, scale: 3.0 }]);
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn names_cannot_break_a_line() {
        let s = Screen { make: "Bad\tMaker\nX".into(), ..screen("DP-1", (100, 100), (0, 0)) };
        assert_eq!(s.names()[0], "Bad Maker X");
        let k = Kept { names: s.names(), across: 100, scale: 1.5 };
        assert_eq!(parse(&write_out(std::slice::from_ref(&k))), vec![k]);
    }
}
