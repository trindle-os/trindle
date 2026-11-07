//! greetd IPC.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::rc::Rc;
use std::sync::mpsc;
use std::time::Duration;

use serde_json::Value;

use crate::toolkit::t;
use crate::toolkit::event::Loop;

const TIMEOUT: Duration = Duration::from_secs(20);

pub type Done = Box<dyn FnOnce(Option<Value>, Option<String>)>;

pub trait Backend {
    fn send(&self, request: Value, done: Done);
    fn reset(&self);
}

enum Job {
    Send(u64, Value),
    Reset,
}

pub struct Greetd {
    jobs: mpsc::Sender<Job>,
    waiting: Rc<RefCell<HashMap<u64, Done>>>,
    next: Cell<u64>,
}

impl Greetd {
    pub fn new(events: &Loop) -> Greetd {
        let waiting: Rc<RefCell<HashMap<u64, Done>>> = Rc::default();
        let w = waiting.clone();
        let back = events.post(move |(id, result): (u64, Result<Value, String>)| {
            let Some(done) = w.borrow_mut().remove(&id) else { return };
            match result {
                Ok(reply) => done(Some(reply), None),
                Err(e) => done(None, Some(if e.is_empty() { t!("greeter-service-no-talk").into() } else { e })),
            }
        });
        let (jobs, inbox) = mpsc::channel::<Job>();
        let path = std::env::var("GREETD_SOCK").ok();
        std::thread::spawn(move || {
            let mut sock = None;
            for job in inbox {
                match job {
                    Job::Send(id, request) => {
                        let r = exchange(&path, &mut sock, &request);
                        if r.is_err() {
                            sock = None;
                        }
                        if !back.send((id, r)) {
                            break;
                        }
                    }
                    Job::Reset => sock = None,
                }
            }
        });
        Greetd { jobs, waiting, next: Cell::new(0) }
    }
}

fn exchange(path: &Option<String>, sock: &mut Option<UnixStream>, request: &Value) -> Result<Value, String> {
    let path = path.as_deref().ok_or(t!("greeter-service-not-started"))?;
    if sock.is_none() {
        let s = UnixStream::connect(path).map_err(|e| e.to_string())?;
        s.set_read_timeout(Some(TIMEOUT)).map_err(|e| e.to_string())?;
        s.set_write_timeout(Some(TIMEOUT)).map_err(|e| e.to_string())?;
        *sock = Some(s);
    }
    let s = sock.as_mut().ok_or(t!("greeter-service-no-connection"))?;
    let data = request.to_string().into_bytes();
    let mut frame = (data.len() as u32).to_ne_bytes().to_vec();
    frame.extend(&data);
    let timed_out = |e: std::io::Error| {
        if matches!(e.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) {
            t!("greeter-service-timeout").to_string()
        } else if e.kind() == std::io::ErrorKind::UnexpectedEof {
            t!("greeter-service-closed").to_string()
        } else {
            e.to_string()
        }
    };
    s.write_all(&frame).map_err(timed_out)?;
    let mut len = [0u8; 4];
    s.read_exact(&mut len).map_err(timed_out)?;
    let size = u32::from_ne_bytes(len) as usize;
    if size > 1 << 20 {
        return Err(t!("greeter-service-too-big").into());
    }
    let mut body = vec![0u8; size];
    s.read_exact(&mut body).map_err(timed_out)?;
    serde_json::from_slice(&body).map_err(|e| e.to_string())
}

impl Backend for Greetd {
    fn send(&self, request: Value, done: Done) {
        let id = self.next.get() + 1;
        self.next.set(id);
        self.waiting.borrow_mut().insert(id, done);
        if self.jobs.send(Job::Send(id, request)).is_err() {
            let done = self.waiting.borrow_mut().remove(&id);
            if let Some(done) = done {
                done(None, Some(t!("greeter-service-no-talk").into()));
            }
        }
    }

    fn reset(&self) {
        let _ = self.jobs.send(Job::Reset);
    }
}
