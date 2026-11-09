//! Optional local voice control via whisper.cpp. Audio is recorded to
//! XDG_RUNTIME_DIR only while listening and deleted after transcription.

pub mod commands;
pub mod setup;

use std::cell::{Cell, RefCell};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::common::files::setting;
use crate::common::run::{self, capture, Proc};
use crate::toolkit::t;
use crate::toolkit::event::{Flow, Loop};

pub const WHISPER: &str = "/usr/local/lib/trindle-whisper/whisper-cli";
pub const MODEL: &str = "/usr/local/share/trindle/whisper/ggml-base.en.bin";

pub const MAX_SECONDS: u64 = 15;

const LOOK_MS: u64 = 150;

const QUIET_AFTER_MS: u64 = 1200;

const NOTHING_MS: u64 = 6000;

const WINDOW: usize = 4800;

pub fn installed() -> bool {
    let w = std::path::Path::new(WHISPER);
    w.is_file() && std::path::Path::new(MODEL).is_file()
}

pub fn enabled() -> bool {
    setting("voice") != "off"
}

pub fn available() -> bool {
    installed() && enabled()
}

struct Recording {
    proc: Proc,
    ended: Rc<Cell<bool>>,
    then: Rc<RefCell<Option<Box<dyn FnOnce()>>>>,
}

struct Inner {
    lp: Loop,
    recording: Option<Recording>,

    turn: u64,
}

fn level_now(path: &std::path::Path) -> Option<f64> {
    use std::io::{Read, Seek, SeekFrom};
    let mut f = std::fs::File::open(path).ok()?;
    let len = f.metadata().ok()?.len();
    let want = (WINDOW * 2) as u64;

    if len < 44 + want {
        return None;
    }
    f.seek(SeekFrom::Start(len - want)).ok()?;
    let mut bytes = vec![0u8; want as usize];
    f.read_exact(&mut bytes).ok()?;
    let sum: f64 = bytes.chunks_exact(2).map(|b| {
        let v = i16::from_le_bytes([b[0], b[1]]) as f64;
        v * v
    }).sum();
    Some((sum / WINDOW as f64).sqrt())
}

#[derive(Clone)]
pub struct Listener(Rc<RefCell<Inner>>);

fn wav_path() -> PathBuf {
    let dir = std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    dir.join("trindle-voice.wav")
}

impl Listener {

    pub fn new(lp: &Loop) -> Listener {
        Listener(Rc::new(RefCell::new(Inner { lp: lp.clone(), recording: None, turn: 0 })))
    }

    pub fn start(&self) -> Result<(), String> {
        if !installed() {
            return Err(t!("voice-optional").into());
        }
        if !enabled() {
            return Err(t!("voice-switched-off").into());
        }

        if std::env::var_os("XDG_RUNTIME_DIR").is_none() {
            return Err(t!("voice-mic-failed", why = "no private folder"));
        }
        self.stop_now();
        let path = wav_path();
        if run::find("pw-record").is_none() {
            let e = std::io::Error::from(std::io::ErrorKind::NotFound);
            return Err(t!("voice-mic-failed", why = e.to_string()));
        }
        let mut command = Command::new("pw-record");
        command.args(["--rate", "16000", "--channels", "1", "--format", "s16"]).arg(&path)
            .stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
        let ended = Rc::new(Cell::new(false));
        let then: Rc<RefCell<Option<Box<dyn FnOnce()>>>> = Rc::new(RefCell::new(None));
        let (e, t) = (ended.clone(), then.clone());
        let proc = run::child(command, move |_| {
            e.set(true);
            let next = t.borrow_mut().take();
            if let Some(next) = next {
                next();
            }
        }).ok_or_else(|| t!("voice-mic-not-started").to_string())?;
        let (lp, turn) = {
            let inner = self.0.borrow();
            (inner.lp.clone(), inner.turn)
        };

        let me = self.clone();
        lp.after(Duration::from_secs(MAX_SECONDS), move || {
            let inner = me.0.borrow();
            if inner.turn == turn {
                if let Some(r) = &inner.recording {
                    r.proc.signal(libc::SIGINT);
                }
            }
        });
        self.0.borrow_mut().recording = Some(Recording { proc, ended, then });
        Ok(())
    }

    pub fn when_finished(&self, finished: impl Fn() + 'static) {
        let path = wav_path();
        let started = Instant::now();

        let floor = Cell::new(f64::MAX);
        let spoke = Cell::new(false);
        let quiet_since = Cell::new(None::<Instant>);
        let me = self.clone();
        let (lp, turn) = {
            let inner = self.0.borrow();
            (inner.lp.clone(), inner.turn)
        };
        lp.every(Duration::from_millis(LOOK_MS), move || {
            if me.0.borrow().turn != turn {
                return Flow::Stop;
            }
            let Some(level) = level_now(&path) else {
                return Flow::Continue;
            };
            floor.set(floor.get().min(level.max(30.0)));
            let speaking = level > (floor.get() * 3.0).max(400.0);
            if speaking {
                spoke.set(true);
                quiet_since.set(None);
            } else if quiet_since.get().is_none() {
                quiet_since.set(Some(Instant::now()));
            }
            let quiet_long = quiet_since.get().is_some_and(|t| t.elapsed().as_millis() as u64 >= QUIET_AFTER_MS);
            let nothing = !spoke.get() && started.elapsed().as_millis() as u64 >= NOTHING_MS;
            if (spoke.get() && quiet_long) || nothing {
                finished();
                return Flow::Stop;
            }
            Flow::Continue
        });
    }

    fn take(&self) -> Option<Recording> {
        let mut inner = self.0.borrow_mut();
        inner.turn += 1;
        inner.recording.take()
    }

    pub fn stop_now(&self) {
        if let Some(r) = self.take() {
            r.proc.signal(libc::SIGINT);
        }
    }

    pub fn finish<F>(&self, hints: Vec<String>, done: F)
    where
        F: FnOnce(Result<String, String>) + 'static,
    {
        let Some(r) = self.take() else {
            done(Err(t!("voice-nothing-recorded").into()));
            return;
        };
        let transcribe = move || {
            let path = wav_path();
            let big_enough = std::fs::metadata(&path).map(|m| m.len() >= 16000).unwrap_or(false);
            if !big_enough {
                let _ = std::fs::remove_file(&path);
                done(Err(t!("voice-nothing-heard-after").into()));
                return;
            }
            let threads = std::thread::available_parallelism().map(|n| n.get().min(4)).unwrap_or(2).to_string();
            let prompt: String = hints.join(", ").chars().take(600).collect();
            let p = path.to_string_lossy().to_string();
            let mut args = vec![WHISPER, "-m", MODEL, "-f", p.as_str(), "-nt", "-np", "-t", threads.as_str()];
            if !prompt.is_empty() {

                args.push("--prompt");
                args.push(prompt.as_str());
            }
            capture(&args, None, move |ok, out, err| {
                let _ = std::fs::remove_file(wav_path());
                if ok {
                    done(Ok(out.split_whitespace().collect::<Vec<_>>().join(" ")));
                } else {
                    done(Err(err.lines().last().unwrap_or(t!("voice-whisper-failed")).to_string()));
                }
            });
        };

        r.proc.signal(libc::SIGINT);
        if r.ended.get() {
            transcribe();
        } else {
            *r.then.borrow_mut() = Some(Box::new(transcribe));
        }
    }
}
