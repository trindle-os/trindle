//! Hearing a Wayland connection on the loop of a background part, until it goes.

use std::cell::RefCell;
use std::os::fd::AsRawFd;
use std::rc::Rc;

use wayland_client::{Connection, EventQueue};

use crate::toolkit::event::{Flow, Loop};

pub(super) fn dispatch_on<S: 'static>(events: &Loop, queue: &Rc<RefCell<EventQueue<S>>>, state: &Rc<RefCell<S>>, conn: &Connection) {
    let fd = conn.backend().poll_fd().as_raw_fd();
    let (ml, q, st, c) = (events.clone(), queue.clone(), state.clone(), conn.clone());
    events.watch(fd, move |hung_up| {
        if hung_up {
            ml.quit();
            return Flow::Stop;
        }
        if let Some(guard) = c.prepare_read() {
            if guard.read().is_err() {
                ml.quit();
                return Flow::Stop;
            }
        }
        if q.borrow_mut().dispatch_pending(&mut st.borrow_mut()).is_err() {
            ml.quit();
            return Flow::Stop;
        }
        let _ = c.flush();
        Flow::Continue
    });
}
