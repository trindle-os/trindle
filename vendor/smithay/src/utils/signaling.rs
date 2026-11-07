//! A general purpose signaling mechanism.

use std::{
    any::Any,
    cell::RefCell,
    collections::VecDeque,
    fmt,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Signaler<S> {
    inner: Rc<SignalInner<S>>,
}

impl<S> Clone for Signaler<S> {
    fn clone(&self) -> Signaler<S> {
        Signaler {
            inner: self.inner.clone(),
        }
    }
}

impl<S> Signaler<S> {
    pub fn new() -> Signaler<S> {
        Signaler {
            inner: Rc::new(SignalInner::new()),
        }
    }

    #[must_use]
    pub fn register<F: FnMut(&S) + 'static>(&self, f: F) -> SignalToken {
        let rc = Rc::new(RefCell::new(f));
        let weak = Rc::downgrade(&rc) as Weak<RefCell<dyn FnMut(&S)>>;
        self.inner.insert(weak);
        SignalToken { signal: rc }
    }

    pub fn signal(&self, signal: S) {
        self.inner.send(signal);
    }
}

impl<S> Default for Signaler<S> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct SignalToken {
    signal: Rc<dyn Any>,
}

impl SignalToken {
    pub fn leak(self) {
        let _ = Rc::into_raw(self.signal);
    }
}

type WeakCallback<S> = Weak<RefCell<dyn FnMut(&S)>>;

struct SignalInner<S> {
    callbacks: RefCell<Vec<WeakCallback<S>>>,
    pending_callbacks: RefCell<Vec<WeakCallback<S>>>,
    pending_events: RefCell<VecDeque<S>>,
}

impl<S: fmt::Debug> fmt::Debug for SignalInner<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SignalInner")
            .field("callbacks::len()", &self.callbacks.borrow().len())
            .field("pending_callbacks::len()", &self.pending_callbacks.borrow().len())
            .field("pending_events", &self.pending_events)
            .finish()
    }
}

impl<S> SignalInner<S> {
    fn new() -> SignalInner<S> {
        SignalInner {
            callbacks: RefCell::new(Vec::new()),
            pending_callbacks: RefCell::new(Vec::new()),
            pending_events: RefCell::new(VecDeque::new()),
        }
    }

    fn insert(&self, weak: WeakCallback<S>) {
        if let Ok(mut guard) = self.callbacks.try_borrow_mut() {
            guard.push(weak);
        } else {
            self.pending_callbacks.borrow_mut().push(weak);
        }
    }

    fn send(&self, event: S) {
        self.pending_events.borrow_mut().push_back(event);
        if let Ok(mut guard) = self.callbacks.try_borrow_mut() {
            loop {
                let next_event = self.pending_events.borrow_mut().pop_front();
                if let Some(event) = next_event {
                    guard.retain(|weak| {
                        if let Some(cb) = Weak::upgrade(weak) {
                            (*cb.borrow_mut())(&event);
                            true
                        } else {
                            false
                        }
                    });
                    guard.extend(self.pending_callbacks.borrow_mut().drain(..));
                } else {
                    break;
                }
            }
        }
    }
}

pub trait Linkable<S> {
    fn link(&mut self, signaler: Signaler<S>);
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{cell::Cell, rc::Rc};

    #[test]
    fn basic_signal() {
        let signaler = Signaler::<u32>::new();

        let signaled = Rc::new(Cell::new(false));
        let signaled2 = signaled.clone();

        let _token = signaler.register(move |_| signaled2.set(true));

        signaler.signal(0);

        assert!(signaled.get());
    }

    #[test]
    fn remove_callback() {
        let signaler = Signaler::<u32>::new();

        let token = signaler.register(|&i| assert_eq!(i, 42));

        signaler.signal(42);

        ::std::mem::drop(token);

        signaler.signal(41);

        let _token = signaler.register(|&i| assert_eq!(i, 39));

        signaler.signal(39);
    }

    #[test]
    fn delayed_signal() {
        let signaler = Signaler::<u32>::new();

        let mut signaled = false;
        let sign2 = signaler.clone();
        let _token = signaler.register(move |&i| {
            if !signaled {
                sign2.signal(42);
                signaled = true;
            } else {
                assert_eq!(i, 42);
            }
        });

        signaler.signal(0);
    }

    #[test]
    fn delayed_register() {
        let signaler = Signaler::<bool>::new();

        let signaled = Rc::new(Cell::new(0u32));
        let signaled2 = signaled.clone();
        let sign2 = signaler.clone();

        let _token1 = signaler.register(move |&original| {
            signaled2.set(signaled2.get() + 1);
            if original {
                let signaled3 = signaled2.clone();
                sign2.register(move |_| signaled3.set(signaled3.get() + 1)).leak();
                sign2.signal(false);
            }
        });

        signaler.signal(true);

        assert_eq!(signaled.get(), 3);
    }
}
