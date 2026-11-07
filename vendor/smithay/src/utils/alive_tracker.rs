//! Utilities to track object's life cycle All our AliveTracker usage is internal and only used, when the wayland_frontend feature is enabled.

#![allow(dead_code)]

use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug)]
pub struct AliveTracker {
    is_alive: AtomicBool,
}

impl Default for AliveTracker {
    fn default() -> Self {
        Self {
            is_alive: AtomicBool::new(true),
        }
    }
}

impl AliveTracker {
    pub fn destroy_notify(&self) {
        self.is_alive.store(false, Ordering::Release);
    }

    #[inline]
    pub fn alive(&self) -> bool {
        self.is_alive.load(Ordering::Acquire)
    }
}

pub trait IsAlive {
    fn alive(&self) -> bool;
}

impl<T: IsAlive> IsAlive for &T {
    #[inline]
    fn alive(&self) -> bool {
        IsAlive::alive(*self)
    }
}
