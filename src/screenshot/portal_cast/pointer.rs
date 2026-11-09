//! The pointer, followed apart from the picture and sent beside each frame.

use std::rc::Rc;
use std::time::Instant;

use super::{cast_capture, wait_for, Cast, Casting, Pointer};
use super::super::cast_capture::{Capture, Heard};

impl Casting {

    pub(super) fn pointer_heard(&self, cast: &Rc<Cast>, index: usize, heard: Heard) {
        let Some(capture) = self.capture() else { return };
        let (place, seen) = {
            let parts = cast.parts.borrow();
            parts.get(index).map(|p| (p.at, p.size)).unwrap_or(((0, 0, 0, 0), (0, 0)))
        };

        let turn = |x: i32, y: i32| -> (i32, i32) {
            if !cast.desktop || seen.0 == 0 || seen.1 == 0 {
                return (x, y);
            }
            (place.0 + x * place.2 as i32 / seen.0 as i32, place.1 + y * place.3 as i32 / seen.1 as i32)
        };
        let mut guard = cast.pointer.borrow_mut();
        let Some(p) = guard.as_mut() else { return };
        if !p.heard(index, heard, &capture, &turn) {
            return;
        }
        drop(guard);
        self.send_pointer(cast);
    }

    pub(super) fn send_pointer(&self, cast: &Rc<Cast>) {
        let pending = cast.pointer.borrow().as_ref().is_some_and(|p| p.pending);
        if !pending || !cast.streaming.get() || cast.retrying.get() || cast.held.borrow().is_some() {
            return;
        }
        let wait = wait_for(cast.sent.get(), cast.gap.get(), Instant::now());
        if !wait.is_zero() {
            self.pace(cast, wait);
            return;
        }
        let taken = cast.spare.borrow_mut().take();
        let buffer = match taken {
            Some(b) => Some(b),
            None => cast.stream.borrow().as_ref().and_then(|s| s.dequeue()),
        };
        match buffer {
            Some(buffer) => self.send(cast, buffer, &[], true),
            None => self.retry(cast),
        }
    }
}

impl Pointer {

    fn heard(&mut self, index: usize, heard: Heard, capture: &Capture, turn: &dyn Fn(i32, i32) -> (i32, i32)) -> bool {
        let Some(f) = self.follows.get_mut(index) else { return false };
        let mut again = false;
        match heard {
            Heard::Size(told) => {
                let len = (told.w * told.h * 4) as usize;
                f.memory = crate::common::shm::Memory::new(c"trindle-cast-pointer", len);
                f.buffer = f.memory.as_ref().and_then(|m| capture.see_through_buffer(m.fd(), len, told.w, told.h));
                f.size = (told.w, told.h);
                again = true;
            }
            Heard::Ready(_) => {

                if f.over || !self.over {
                    if let Some(m) = &f.memory {
                        self.picture = m.bytes().to_vec();
                        self.size = f.size;
                        self.pending = true;
                    }
                }
                again = true;
            }
            Heard::Failed => {}
            Heard::Stopped => f.id = None,
            Heard::Pointer(cast_capture::PointerHeard::Over) => f.over = true,
            Heard::Pointer(cast_capture::PointerHeard::Gone) => f.over = false,
            Heard::Pointer(cast_capture::PointerHeard::At(x, y)) => {
                self.at = turn(x, y);
                self.pending = true;
            }
            Heard::Pointer(cast_capture::PointerHeard::Hotspot(x, y)) => {
                let (hx, hy) = turn(x, y);
                let (ox, oy) = turn(0, 0);
                self.hotspot = (hx - ox, hy - oy);
                self.pending = true;
            }
        }

        if again {
            if let (Some(id), Some(buffer)) = (f.id, f.buffer.as_ref()) {
                capture.capture(id, buffer);
            }
        }
        let over = self.follows.iter().any(|f| f.over);
        if over != self.over {
            self.over = over;
            self.pending = true;
        }
        true
    }
}
