//! Sharing what was chosen.

use std::rc::Rc;
use std::time::{Duration, Instant};

use crate::toolkit::t;
use crate::toolkit::event::Flow;

use super::{cast_capture, gbm, monotonic_ns, tell_in_use, wait_for, Cast, Casting, RETRY, SESSION, Session};
use super::desktop::place;
use super::super::cast_capture::{Capture, Heard};
use super::super::pw::stream;
use super::super::pw::connection::PipeWire;
use super::super::pw::stream::Stream;

impl Casting {
    pub(super) fn capture(&self) -> Option<Rc<Capture>> {
        if let Some(c) = self.capture.borrow().as_ref() {
            return Some(c.clone());
        }
        let c = Capture::connect(&self.kit.events)?;
        *self.capture.borrow_mut() = Some(c.clone());
        Some(c)
    }

    fn pipewire(&self) -> Result<Rc<PipeWire>, String> {
        if let Some(pw) = self.pw.borrow().as_ref() {
            return Ok(pw.clone());
        }
        let pw = PipeWire::connect(t!("screenshot-cast-name"))?;
        let me = self.me.clone();
        let fd = std::os::fd::AsRawFd::as_raw_fd(&pw.fd());
        self.kit.events.watch(fd, move |hung_up| {
            let Some(me) = me.upgrade() else { return Flow::Stop };
            let pw = me.pw.borrow().clone();
            let Some(pw) = pw else { return Flow::Stop };
            pw.dispatch();
            me.pipewire_events();
            if hung_up {
                me.pw_lost();
                return Flow::Stop;
            }
            Flow::Continue
        });
        *self.pw.borrow_mut() = Some(pw.clone());
        Ok(pw)
    }

    fn pw_lost(&self) {
        let sessions: Vec<Rc<Session>> = self.sessions.borrow().values().cloned().collect();
        for s in sessions {
            self.end(&s, true);
        }
        self.pw.borrow_mut().take();
    }

    fn pipewire_events(&self) {
        let sessions: Vec<Rc<Session>> = self.sessions.borrow().values().cloned().collect();
        for session in sessions {
            let casts: Vec<Rc<Cast>> = session.casts.borrow().clone();
            for cast in casts {
                let events = match cast.stream.borrow().as_ref() {
                    Some(s) => s.take_events(),
                    None => continue,
                };
                for event in events {
                    match event {
                        stream::Event::State(stream::State::Paused, _) => {
                            if cast.node.get().is_none() {
                                let node = cast.stream.borrow().as_ref().map(|s| s.node_id());
                                cast.node.set(node.filter(|&n| n != u32::MAX));
                            }
                            cast.streaming.set(false);
                        }
                        stream::Event::State(stream::State::Streaming, _) => {
                            cast.streaming.set(true);
                            self.kick(&cast);
                        }
                        stream::Event::State(stream::State::Error, why) => {
                            eprintln!("trindle-portal: screen sharing stopped: {}", why.unwrap_or_default());
                            cast.failed.set(true);
                            self.end(&session, true);
                        }
                        stream::Event::State(stream::State::Unconnected, _) if cast.node.get().is_some() => {
                            cast.failed.set(true);
                            self.end(&session, true);
                        }
                        stream::Event::Format(Some(f)) => {
                            cast.gap.set(f.frame_gap());
                            self.kick(&cast);
                            self.send_pointer(&cast);
                        }
                        stream::Event::Process => {
                            self.kick(&cast);
                            self.send_pointer(&cast);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn kick(&self, cast: &Rc<Cast>) {
        if !cast.streaming.get() || cast.failed.get() {
            return;
        }
        if cast.desktop {
            self.kick_parts(cast);
            return;
        }
        if cast.filling.borrow().is_some() || cast.held.borrow().is_some() {
            return;
        }
        let Some(capture) = self.capture() else { return };
        let (id, size) = {
            let parts = cast.parts.borrow();
            match parts.first() {
                Some(p) => (p.capture, p.size),
                None => return,
            }
        };
        let Some(id) = id else { return };
        let taken = cast.spare.borrow_mut().take();
        let buffer = match taken {
            Some(b) => Some(b),
            None => cast.stream.borrow().as_ref().and_then(|s| s.dequeue()),
        };
        let Some(buffer) = buffer else {
            self.retry(cast);
            return;
        };
        let ready = {
            let stream = cast.stream.borrow();
            let Some(stream) = stream.as_ref() else { return };
            let Some(slot) = stream.slot(&buffer) else { return };
            if (slot.format.width, slot.format.height) != size {
                None
            } else {
                if slot.attached.borrow().is_none() {
                    let made = match &slot.store {
                        stream::Store::Shared(m) => capture.buffer(m.fd(), m.len(), size.0, size.1, slot.stride),
                        stream::Store::Card(bo) => capture.dmabuf_buffer(bo, size.0, size.1),
                    };
                    *slot.attached.borrow_mut() = made.map(|b| Box::new(b) as Box<dyn std::any::Any>);
                }
                let attached = slot.attached.borrow();
                attached.as_ref().and_then(|a| a.downcast_ref::<cast_capture::Buffer>()).map(|b| capture.capture(id, b))
            }
        };
        match ready {
            Some(true) => *cast.filling.borrow_mut() = Some(buffer),
            _ => *cast.spare.borrow_mut() = Some(buffer),
        }
    }

    pub(super) fn retry(&self, cast: &Rc<Cast>) {
        if cast.retrying.replace(true) {
            return;
        }
        let (me, c) = (self.me.clone(), Rc::downgrade(cast));
        self.kit.events.after(RETRY, move || {
            let (Some(me), Some(cast)) = (me.upgrade(), c.upgrade()) else { return };
            cast.retrying.set(false);
            me.kick(&cast);
            me.send_pointer(&cast);
        });
    }

    fn kick_parts(&self, cast: &Rc<Cast>) {
        let Some(capture) = self.capture() else { return };
        let mut parts = cast.parts.borrow_mut();
        for part in parts.iter_mut() {
            if part.busy {
                continue;
            }
            if let (Some(id), Some(buffer)) = (part.capture, part.buffer.as_ref()) {
                part.busy = capture.capture(id, buffer);
            }
        }
    }

    pub(super) fn heard(&self, session: &Rc<Session>, cast: &Rc<Cast>, index: usize, heard: Heard) {
        match heard {
            Heard::Size(told) => self.sized(session, cast, index, told),
            Heard::Ready(damage) if cast.desktop => {
                if let Some(part) = cast.parts.borrow_mut().get_mut(index) {
                    part.busy = false;
                    let (x, y) = (part.at.0, part.at.1);
                    let damage = if damage.is_empty() { vec![(0, 0, part.size.0, part.size.1)] } else { damage };
                    part.fresh.extend(damage.into_iter().map(|(dx, dy, w, h)| (x + dx, y + dy, w, h)));
                }
                self.put_together(cast);
                self.kick(cast);
            }
            Heard::Ready(damage) => {
                let filled = cast.filling.borrow_mut().take();
                if let Some(buffer) = filled {
                    let wait = wait_for(cast.sent.get(), cast.gap.get(), Instant::now());
                    if wait.is_zero() {
                        self.send(cast, buffer, &damage, false);
                    } else {
                        *cast.held.borrow_mut() = Some((buffer, damage));
                        self.pace(cast, wait);
                        return;
                    }
                }
                self.kick(cast);
            }
            Heard::Failed => {
                if cast.desktop {
                    if let Some(part) = cast.parts.borrow_mut().get_mut(index) {
                        part.busy = false;
                    }
                } else {
                    let buffer = cast.filling.borrow_mut().take();
                    if buffer.is_some() {
                        *cast.spare.borrow_mut() = buffer;
                    }
                }
                self.retry(cast);
            }
            Heard::Stopped => self.end(session, true),
            Heard::Pointer(_) => {}
        }
    }

    fn sized(&self, session: &Rc<Session>, cast: &Rc<Cast>, index: usize, told: cast_capture::Constraints) {
        let size = (told.w, told.h);
        if cast.desktop {
            let Some(capture) = self.capture() else { return };
            let mut parts = cast.parts.borrow_mut();
            let Some(part) = parts.get_mut(index) else { return };
            part.size = size;
            part.busy = false;
            let len = (size.0 * size.1 * 4) as usize;
            part.memory = crate::common::shm::Memory::new(c"trindle-cast-screen", len);
            part.buffer = part.memory.as_ref().and_then(|m| capture.buffer(m.fd(), len, size.0, size.1, size.0 * 4));
            let all_known = parts.iter().all(|p| p.size.0 > 0);
            drop(parts);
            if all_known && cast.stream.borrow().is_none() {
                self.make_stream(session, cast, cast.size.get(), None);
            }
            self.kick(cast);
            return;
        }
        if let Some(part) = cast.parts.borrow_mut().get_mut(index) {
            part.size = size;
        }
        let had = cast.stream.borrow().is_some();
        if !had {
            cast.size.set(size);
            let card = told.device.filter(|_| !told.modifiers.is_empty())
                .and_then(|d| self.card(d))
                .map(|device| stream::Card { device, modifiers: told.modifiers.clone() });
            self.make_stream(session, cast, size, card);
        } else if cast.size.get() != size {
            cast.size.set(size);
            cast.spare.borrow_mut().take();
            cast.filling.borrow_mut().take();
            cast.held.borrow_mut().take();
            if let Some(stream) = cast.stream.borrow().as_ref() {
                stream.resize(size.0, size.1);
            }
        }
        self.kick(cast);
    }

    fn card(&self, device: u64) -> Option<Rc<gbm::Device>> {
        let mut cards = self.cards.borrow_mut();
        if let Some((_, d)) = cards.iter().find(|(n, _)| *n == device) {
            return Some(d.clone());
        }
        let d = Rc::new(gbm::Device::open(device)?);
        cards.push((device, d.clone()));
        Some(d)
    }

    fn make_stream(&self, session: &Rc<Session>, cast: &Rc<Cast>, size: (u32, u32), card: Option<stream::Card>) {
        let made = self.pipewire().and_then(|pw| {
            Stream::new(&pw, "trindle-screencast", &[
                ("media.class", "Video/Source"),
                ("media.role", "Screen"),
                ("media.name", t!("screenshot-cast-name")),
                ("node.name", "trindle-screencast"),
                ("node.description", t!("screenshot-cast-node")),
            ], size.0, size.1, card, cast.pointer.borrow().is_some())
        });
        match made {
            Ok(stream) => *cast.stream.borrow_mut() = Some(stream),
            Err(e) => {
                eprintln!("trindle-portal: {e}");
                cast.failed.set(true);
                self.end(session, true);
            }
        }
    }

    fn put_together(&self, cast: &Rc<Cast>) {
        let fresh: Vec<(i32, i32, u32, u32)> = cast.parts.borrow_mut().iter_mut().flat_map(|p| std::mem::take(&mut p.fresh)).collect();
        if fresh.is_empty() || !cast.streaming.get() {
            return;
        }
        let wait = wait_for(cast.sent.get(), cast.gap.get(), Instant::now());
        if !wait.is_zero() {
            if let Some(p) = cast.parts.borrow_mut().first_mut() {
                p.fresh.extend(fresh);
            }
            self.pace(cast, wait);
            return;
        }
        let taken = cast.spare.borrow_mut().take();
        let buffer = match taken {
            Some(b) => Some(b),
            None => cast.stream.borrow().as_ref().and_then(|s| s.dequeue()),
        };
        let Some(buffer) = buffer else {
            if let Some(p) = cast.parts.borrow_mut().first_mut() {
                p.fresh.extend(fresh);
            }
            self.retry_together(cast);
            return;
        };
        let held = cast.stream.borrow();
        let Some(stream) = held.as_ref() else { return };
        let Some(slot) = stream.slot(&buffer) else {
            if let Some(p) = cast.parts.borrow_mut().first_mut() {
                p.fresh.extend(fresh);
            }
            drop(held);
            self.retry_together(cast);
            return;
        };
        let (w, h, stride) = (slot.format.width, slot.format.height, slot.stride);
        let stream::Store::Shared(memory) = &slot.store else { return };
        // SAFETY: the slot's memory is PipeWire's buffer, ours to write
        // while it is taken (dequeued) and not yet sent; nothing else in this
        // program holds a view of it now, and the slice is made from the
        // mapping's own address, not from a shared borrow.
        let out = unsafe { std::slice::from_raw_parts_mut(memory.as_mut_ptr(), memory.len()) };
        out.fill(0);
        for part in cast.parts.borrow().iter() {
            if let Some(m) = &part.memory {
                place(out, stride, (w, h), m.bytes(), part.size, part.at);
            }
        }
        drop(held);
        self.send(cast, buffer, &fresh, false);
    }

    pub(super) fn send(&self, cast: &Rc<Cast>, buffer: stream::Buffer, damage: &[(i32, i32, u32, u32)], only_pointer: bool) {
        let sent = {
            let stream = cast.stream.borrow();
            let Some(stream) = stream.as_ref() else { return };
            let seq = cast.seq.get();
            let pointer = cast.pointer.borrow();
            let meta = pointer.as_ref().map(|p| p.meta());
            let sent = stream.queue(buffer, damage, monotonic_ns(), seq, meta.as_ref(), only_pointer);
            if sent {
                cast.seq.set(seq + 1);
            }
            sent
        };
        if sent {
            cast.sent.set(Some(Instant::now()));
            if let Some(p) = cast.pointer.borrow_mut().as_mut() {
                p.pending = false;
            }
        }
    }

    pub(super) fn pace(&self, cast: &Rc<Cast>, wait: Duration) {
        if cast.pacing.replace(true) {
            return;
        }
        let (me, c) = (self.me.clone(), Rc::downgrade(cast));
        self.kit.events.after(wait, move || {
            let (Some(me), Some(cast)) = (me.upgrade(), c.upgrade()) else { return };
            cast.pacing.set(false);
            if cast.failed.get() {
                return;
            }
            if cast.desktop {
                me.put_together(&cast);
            } else {
                let held = cast.held.borrow_mut().take();
                if let Some((buffer, damage)) = held {
                    me.send(&cast, buffer, &damage, false);
                }
                me.kick(&cast);
            }
            me.send_pointer(&cast);
        });
    }

    fn retry_together(&self, cast: &Rc<Cast>) {
        if cast.retrying.replace(true) {
            return;
        }
        let (me, c) = (self.me.clone(), Rc::downgrade(cast));
        self.kit.events.after(RETRY, move || {
            let (Some(me), Some(cast)) = (me.upgrade(), c.upgrade()) else { return };
            cast.retrying.set(false);
            me.put_together(&cast);
        });
    }

    pub(super) fn end(&self, session: &Rc<Session>, tell: bool) {
        if session.closed.replace(true) {
            return;
        }
        if let Some(capture) = self.capture.borrow().as_ref() {
            for cast in session.casts.borrow().iter() {
                for part in cast.parts.borrow().iter() {
                    if let Some(id) = part.capture {
                        capture.close(id);
                    }
                }
                if let Some(p) = cast.pointer.borrow().as_ref() {
                    for id in p.follows.iter().filter_map(|f| f.id) {
                        capture.close(id);
                    }
                }
            }
        }
        let casts = std::mem::take(&mut *session.casts.borrow_mut());
        let picker = session.picker.borrow_mut().take();
        let request = session.request.borrow_mut().take();
        let served = session.served.borrow_mut().take();
        if tell {
            let _ = self.kit.bus.emit(&session.path, SESSION, "Closed", &());
        }
        self.sessions.borrow_mut().remove(&session.path);
        tell_in_use(self);
        self.kit.release();
        let me = self.me.clone();
        self.kit.events.after(Duration::ZERO, move || {
            for cast in &casts {
                cast.filling.borrow_mut().take();
                cast.spare.borrow_mut().take();
                cast.held.borrow_mut().take();
                for part in cast.parts.borrow_mut().iter_mut() {
                    part.buffer.take();
                }
                cast.pointer.borrow_mut().take();
                cast.stream.borrow_mut().take();
            }
            drop((casts, picker, request, served));
            if let Some(me) = me.upgrade() {
                if me.sessions.borrow().values().all(|s| s.casts.borrow().is_empty()) {
                    me.pw.borrow_mut().take();
                    me.cards.borrow_mut().clear();
                }
            }
        });
    }
}
