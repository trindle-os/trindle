//! How a box asks the compositor for an Xwayland of its own: one line with
//! the box's name, one line back with the display's number, and the
//! connection kept open for as long as the box runs. Every box of the same
//! name is given the same display. Later the compositor may send one more
//! line: `lost` when that Xwayland ended and is being started again, or
//! `failed` when it would not start. Shared by trindle-compose, which
//! answers, and trindle-boxed, which asks.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

pub const ROOMS_SOCKET: &str = "trindle-x11";

pub const NAME_MAX: usize = 128;

pub const ROOMS_MAX: usize = 24;

pub const SETTING: &str = "older_programs";

pub const LOST: &str = "lost";

pub const FAILED: &str = "failed";

pub fn would_not_start() -> &'static str {
    trindle_toolkit::t!("common-x11-would-not-start")
}

pub fn try_again() -> &'static str {
    trindle_toolkit::t!("common-x11-try-again")
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Told {
    Lost,
    Failed,
}

impl Told {
    pub fn line(self) -> String {
        format!("{}\n", match self { Told::Lost => LOST, Told::Failed => FAILED })
    }

    pub fn parse(line: &str) -> Option<Told> {
        match line.trim() {
            LOST => Some(Told::Lost),
            FAILED => Some(Told::Failed),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Answer {

    Display(u32),

    Off,

    Full,

    Bad,
}

impl Answer {
    pub fn line(&self) -> String {
        match self {
            Answer::Display(n) => format!("{n}\n"),
            Answer::Off => "off\n".into(),
            Answer::Full => "full\n".into(),
            Answer::Bad => "bad\n".into(),
        }
    }

    pub fn parse(line: &str) -> Answer {
        match line.trim() {
            "off" => Answer::Off,
            "full" => Answer::Full,
            n => n.parse().map(Answer::Display).unwrap_or(Answer::Bad),
        }
    }
}

pub fn allowed(value: &str) -> bool {
    value.trim() != "no"
}

pub fn name_in(request: &str) -> Option<&str> {
    let name = request.strip_prefix("room ")?.trim_end_matches('\n');
    let fine = !name.is_empty() && name.len() <= NAME_MAX
        && name.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'+'));
    fine.then_some(name)
}

pub fn socket() -> Option<PathBuf> {
    std::env::var_os("XDG_RUNTIME_DIR").filter(|d| !d.is_empty()).map(|d| PathBuf::from(d).join(ROOMS_SOCKET))
}

pub fn ask(name: &str) -> Result<(UnixStream, Answer), String> {
    let at = socket().ok_or("no runtime folder")?;
    let mut stream = UnixStream::connect(&at).map_err(|e| format!("{}: {e}", at.display()))?;
    stream.set_read_timeout(Some(std::time::Duration::from_secs(10))).map_err(|e| e.to_string())?;
    stream.write_all(format!("room {name}\n").as_bytes()).map_err(|e| e.to_string())?;
    let mut line = String::new();
    BufReader::new(&stream).read_line(&mut line).map_err(|e| e.to_string())?;
    let _ = stream.set_read_timeout(None);
    Ok((stream, Answer::parse(&line)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_and_answers_read_back() {
        assert_eq!(name_in("room org.example.App\n"), Some("org.example.App"));
        assert_eq!(name_in("room gimp_2.10+1-x"), Some("gimp_2.10+1-x"));
        for bad in ["room ", "room a b", "room ../x", "room a/b", "rooms a", "a", "room a\nroom b",
                    &format!("room {}", "a".repeat(NAME_MAX + 1))] {
            assert_eq!(name_in(bad), None, "{bad:?}");
        }
        for a in [Answer::Display(0), Answer::Display(31), Answer::Off, Answer::Full] {
            assert_eq!(Answer::parse(&a.line()), a);
        }
        assert_eq!(Answer::parse("x"), Answer::Bad);
        assert_eq!(Answer::parse(""), Answer::Bad);
        assert!(allowed("yes") && allowed("") && !allowed("no"));
        for t in [Told::Lost, Told::Failed] {
            assert_eq!(Told::parse(&t.line()), Some(t));
        }
        assert_eq!(Told::parse("3"), None);
    }
}
