//! trindle-unlock: the box that asks for the disk password while the computer
//! starts, drawn straight to the screen before any desktop exists.

#[macro_use]
#[path = "../said.rs"]
#[allow(dead_code)]
mod said;
mod fb;
mod fifo;
mod keys;
mod logo;
mod paint;
mod pointer;
mod spin;

use trindle_shared::boot;
use std::io::Write;

use trindle_toolkit::common::eye;
use trindle_toolkit::t;

struct Words;

impl trindle_toolkit::host::Host for Words {
    fn words(&self) -> &'static [(&'static str, &'static str)] {
        &[("unlock.ftl", include_str!("../../share/lang/en/unlock.ftl"))]
    }
}

extern "C" fn give_words() {
    trindle_toolkit::host::set(&Words);
}

#[used]
#[link_section = ".init_array"]
static TOOLKIT_HOST: extern "C" fn() = give_words;

fn main() -> std::process::ExitCode {
    let first = std::env::args().nth(1).unwrap_or_default();
    match first.as_str() {
        "--spin" => return spin::run(spin::Kind::Starting),
        "--spin-closing" => return spin::run(spin::Kind::Closing),
        "--spin-between" => return spin::run(spin::Kind::Between),
        "--spin-stop" => return spin::stop(),
        _ => {}
    }
    let prompt = if first.is_empty() { t!("unlock-title-default").to_string() } else { first };
    let prompt = tidy(&prompt);

    let mut pipe = fifo::PassFifo::open();

    if let Some(key) = pipe.as_mut().and_then(fifo::PassFifo::wait_for_chip) {
        return give(&key);
    }

    let _paused = spin::pause();
    let asked = std::time::Instant::now();
    let answer = ask_on_screen(&prompt, &mut pipe)
        .or_else(|| keys::ask_plainly(&prompt, &mut pipe));

    let waited = boot::seconds().map(|_| {
        let before: f64 = std::fs::read_to_string(boot::WAITED).ok()
            .and_then(|s| s.trim().parse().ok()).unwrap_or(0.0);
        before + asked.elapsed().as_secs_f64()
    });
    if let Some(w) = waited {
        let _ = std::fs::write(boot::WAITED, format!("{w:.2}\n"));
    }
    let Some(password) = answer else {
        return std::process::ExitCode::FAILURE;
    };
    give(&password)
}

fn give(key: &[u8]) -> std::process::ExitCode {
    let mut out = std::io::stdout();
    match out.write_all(key).and_then(|()| out.flush()) {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(_) => std::process::ExitCode::FAILURE,
    }
}

fn tidy(prompt: &str) -> String {
    let p = prompt.trim().trim_end_matches(':').trim();
    let l = p.to_lowercase();
    if p.is_empty() || l.contains("passphrase for") || l.contains("unlock disk") || l == "enter passphrase" {
        t!("unlock-title-encryption").to_string()
    } else {
        p.to_string()
    }
}

fn leave(off: bool) -> ! {
    // SAFETY: neither call touches memory of ours. sync flushes what is

    unsafe {
        libc::sync();
        libc::reboot(if off { libc::RB_POWER_OFF } else { libc::RB_AUTOBOOT });
    }

    std::process::exit(1);
}

fn left_showing(show: bool, used: std::time::Instant, now: std::time::Instant) -> bool {
    show && eye::rested(used, now)
}

fn ask_on_screen(prompt: &str, pipe: &mut Option<fifo::PassFifo>) -> Option<Vec<u8>> {
    let mut asking = Asking::open(prompt, pipe)?;
    loop {
        match asking.round()? {
            Next::Ask => {}
            Next::Answer(key) => return Some(key),
            Next::Leave(off) => asking.leave_from(off),
        }
    }
}

enum Next {

    Ask,

    Answer(Vec<u8>),

    Leave(bool),
}

struct Asking<'a> {
    screen: fb::Screen,
    tty: keys::Tty,
    mouse: pointer::Pointer,
    pipe: &'a mut Option<fifo::PassFifo>,
    prompt: &'a str,
    typed: String,

    show: bool,
    used: std::time::Instant,
    wrong: bool,
    eye: paint::Spot,

    drawn: Option<(f32, f32)>,
    looked: std::time::Instant,
}

impl<'a> Asking<'a> {
    fn open(prompt: &'a str, pipe: &'a mut Option<fifo::PassFifo>) -> Option<Asking<'a>> {
        let mut screen = fb::Screen::open()?;
        let tty = keys::Tty::raw()?;
        let mouse = pointer::Pointer::open(screen.width, screen.height);
        let typed = String::new();
        let used = std::time::Instant::now();
        let eye = paint::draw(&mut screen, prompt, &typed, false, false, None);
        Some(Asking { screen, tty, mouse, pipe, prompt, typed, show: false, used, wrong: false, eye, drawn: None, looked: std::time::Instant::now() })
    }

    fn wait(&mut self) -> (Vec<libc::pollfd>, Option<usize>, usize) {
        let mut waiting = vec![libc::pollfd { fd: self.tty.fd(), events: libc::POLLIN, revents: 0 }];
        let piped = self.pipe.as_ref().map(|p| {
            waiting.push(libc::pollfd { fd: p.fd(), events: libc::POLLIN, revents: 0 });
            waiting.len() - 1
        });
        let mice = waiting.len();
        waiting.extend(self.mouse.fds().into_iter().map(|fd| libc::pollfd { fd, events: libc::POLLIN, revents: 0 }));
        // SAFETY: `waiting` is a live array of exactly that many pollfd, and

        let ready = unsafe { libc::poll(waiting.as_mut_ptr(), waiting.len() as libc::nfds_t, 1000) };
        if ready < 0 && std::io::Error::last_os_error().kind() != std::io::ErrorKind::Interrupted {

            waiting.truncate(1);
            waiting[0].revents = libc::POLLIN;
        }

        if self.looked.elapsed() >= std::time::Duration::from_secs(1) {
            self.mouse.rescan();
            self.looked = std::time::Instant::now();
        }
        (waiting, piped, mice)
    }

    fn chip_key(&mut self, waiting: &[libc::pollfd], piped: Option<usize>) -> Option<Vec<u8>> {
        if piped.and_then(|i| waiting.get(i)).is_some_and(|w| w.revents != 0) {
            if let Some(key) = self.pipe.as_mut().and_then(fifo::PassFifo::take) {
                paint::goodbye(&mut self.screen);
                return Some(key);
            }
        }
        None
    }

    fn mouse_news(&mut self, waiting: &[libc::pollfd], mice: usize) -> (bool, bool) {
        let (mut changed, mut moved) = (false, false);
        for w in waiting.iter().skip(mice).filter(|w| w.revents != 0) {
            for happened in self.mouse.read(w.fd) {
                match happened {
                    pointer::Happened::Moved => moved = true,
                    pointer::Happened::Clicked(x, y) => {
                        if self.eye.holds(x, y) {
                            self.show = !self.show;
                            self.used = std::time::Instant::now();
                            changed = true;
                        }
                    }
                }
            }
        }
        (changed, moved)
    }

    fn press(&mut self) -> Option<Next> {
        match self.tty.next()? {
            keys::Key::Char(c) => self.typed.push(c),
            keys::Key::Backspace => {
                self.typed.pop();
            }
            keys::Key::Clear => self.typed.clear(),
            keys::Key::Show => self.show = !self.show,
            key @ (keys::Key::Restart | keys::Key::Off) => {
                paint::goodbye(&mut self.screen);
                return Some(Next::Leave(matches!(key, keys::Key::Off)));
            }
            keys::Key::Enter => {
                if self.typed.is_empty() {
                    self.wrong = true;
                } else {
                    paint::goodbye(&mut self.screen);
                    return Some(Next::Answer(std::mem::take(&mut self.typed).into_bytes()));
                }
            }
        }
        if !self.typed.is_empty() {
            self.wrong = false;
        }
        Some(Next::Ask)
    }

    fn round(&mut self) -> Option<Next> {
        let (waiting, piped, mice) = self.wait();
        if let Some(key) = self.chip_key(&waiting, piped) {
            return Some(Next::Answer(key));
        }
        let (mut changed, moved) = self.mouse_news(&waiting, mice);
        if waiting[0].revents != 0 {
            changed = true;
            self.used = std::time::Instant::now();
            match self.press()? {
                Next::Ask => {}
                done => return Some(done),
            }
        }
        if left_showing(self.show, self.used, std::time::Instant::now()) {
            self.show = false;
            changed = true;
        }
        let at = self.mouse.seen.then_some(self.mouse.at);
        if changed {
            self.eye = paint::draw(&mut self.screen, self.prompt, &self.typed, self.show, self.wrong, at);
            self.drawn = at;
        } else if let (true, Some(to)) = (moved, at) {
            paint::move_pointer(&mut self.screen, self.drawn, to);
            self.drawn = at;
        }
        Some(Next::Ask)
    }

    fn leave_from(self, off: bool) -> ! {
        let Asking { screen, tty, .. } = self;
        drop(tty);
        drop(screen);
        leave(off)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    #[test]
    fn a_disk_password_left_showing_hides_after_half_a_minute_without_a_key() {
        let t = Instant::now();
        assert!(!left_showing(true, t, t + Duration::from_secs(29)));
        assert!(left_showing(true, t, t + eye::REST));

        assert!(!left_showing(true, t + Duration::from_secs(20), t + eye::REST));

        assert!(!left_showing(false, t, t + Duration::from_secs(600)));
    }

    #[test]
    fn the_prompt_is_kept_short_and_plain() {
        assert_eq!(tidy("Please unlock disk sda3_crypt:"), "Enter encryption password");
        assert_eq!(tidy("  Enter passphrase: "), "Enter encryption password");
        assert_eq!(tidy("Disk password"), "Disk password");
    }

    #[test]
    fn the_box_follows_the_language_and_falls_back_word_by_word() {
        use trindle_toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let french = Bundle::from_text("fr", "unlock-title-encryption = Saisir le mot de passe de chiffrement\nunlock-hide = Cacher\n");
        with_catalogue(Catalogue::new(french, english()), || {
            assert_eq!(tidy("Please unlock disk sda3_crypt:"), "Saisir le mot de passe de chiffrement");
            assert_eq!((t!("unlock-hide"), t!("unlock-show")), ("Cacher", "Show"), "a word French lacks is English");
        });
    }
}
