//! How many threads a stranger on the network can start.

use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(super) static DESCRIBING: AtomicUsize = AtomicUsize::new(0);
pub(super) const MOST_DESCRIBING: usize = 16;
pub(super) static ANSWERING: AtomicUsize = AtomicUsize::new(0);
pub(super) const MOST_ANSWERING: usize = 16;
pub(super) static REPLYING: AtomicUsize = AtomicUsize::new(0);
pub(super) const MOST_REPLYING: usize = 32;
pub(super) static SHOUTING: AtomicUsize = AtomicUsize::new(0);
pub(super) const MOST_SHOUTING: usize = 32;

pub(super) fn started(job: impl FnOnce() + Send + 'static) -> bool {
    thread::Builder::new().spawn(job).map_err(|e| eprintln!("trindle-wsdd: no thread: {e}")).is_ok()
}

struct Place(&'static AtomicUsize);

impl Drop for Place {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::AcqRel);
    }
}

pub(super) fn spawn_within(count: &'static AtomicUsize, most: usize, job: impl FnOnce() + Send + 'static) -> bool {
    if count.fetch_add(1, Ordering::AcqRel) >= most {
        count.fetch_sub(1, Ordering::AcqRel);
        return false;
    }
    let place = Place(count);
    started(move || {
        let _place = place;
        job();
    })
}
