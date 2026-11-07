// The caching logic is used to process surface synchronization.

use std::{
    collections::VecDeque,
    sync::{Mutex, MutexGuard},
};

use downcast_rs::{impl_downcast, Downcast};
use wayland_server::DisplayHandle;

use crate::utils::Serial;

pub trait Cacheable: Default {
    fn commit(&mut self, dh: &DisplayHandle) -> Self;
    fn merge_into(self, into: &mut Self, dh: &DisplayHandle);
}

#[derive(Debug)]
pub struct CachedState<T> {
    pending: T,
    cache: VecDeque<(Serial, T)>,
    current: T,
}

impl<T: Default> Default for CachedState<T> {
    fn default() -> Self {
        CachedState {
            pending: T::default(),
            cache: VecDeque::new(),
            current: T::default(),
        }
    }
}

impl<T> CachedState<T> {
    pub fn current(&mut self) -> &mut T {
        &mut self.current
    }

    pub fn pending(&mut self) -> &mut T {
        &mut self.pending
    }
}

trait Cache: Downcast {
    fn commit(&self, commit_id: Option<Serial>, dh: &DisplayHandle);
    fn apply_state(&self, commit_id: Serial, dh: &DisplayHandle);
}

impl_downcast!(Cache);

impl<T: Cacheable + 'static> Cache for Mutex<CachedState<T>> {
    fn commit(&self, commit_id: Option<Serial>, dh: &DisplayHandle) {
        let mut guard = self.lock().unwrap();
        let me = &mut *guard;
        let new_state = me.pending.commit(dh);
        if let Some(id) = commit_id {
            match me.cache.back_mut() {
                Some(&mut (cid, ref mut state)) if cid == id => new_state.merge_into(state, dh),
                _ => me.cache.push_back((id, new_state)),
            }
        } else {
            for (_, state) in me.cache.drain(..) {
                state.merge_into(&mut me.current, dh);
            }
            new_state.merge_into(&mut me.current, dh);
        }
    }

    fn apply_state(&self, commit_id: Serial, dh: &DisplayHandle) {
        let mut me = self.lock().unwrap();
        loop {
            if me.cache.front().map(|&(s, _)| s > commit_id).unwrap_or(true) {
                break;
            }
            me.cache.pop_front().unwrap().1.merge_into(&mut me.current, dh);
        }
    }
}

pub struct MultiCache {
    caches: appendlist::AppendList<Box<dyn Cache + Send>>,
}

impl std::fmt::Debug for MultiCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MultiCache").finish_non_exhaustive()
    }
}

impl MultiCache {
    pub(crate) fn new() -> Self {
        Self {
            caches: appendlist::AppendList::new(),
        }
    }

    fn find_or_insert<T: Cacheable + Send + 'static>(&self) -> &Mutex<CachedState<T>> {
        for cache in &self.caches {
            if let Some(v) = (**cache).as_any().downcast_ref() {
                return v;
            }
        }
        self.caches
            .push(Box::new(Mutex::new(CachedState::<T>::default())) as Box<_>);
        (*self.caches[self.caches.len() - 1])
            .as_any()
            .downcast_ref()
            .unwrap()
    }

    pub fn get<T: Cacheable + Send + 'static>(&self) -> MutexGuard<'_, CachedState<T>> {
        self.find_or_insert::<T>().lock().unwrap()
    }

    pub fn has<T: Cacheable + Send + 'static>(&self) -> bool {
        self.caches
            .iter()
            .any(|c| (**c).as_any().is::<Mutex<CachedState<T>>>())
    }

    pub(crate) fn commit(&mut self, commit_id: Option<Serial>, dh: &DisplayHandle) {
        for cache in &self.caches {
            cache.commit(commit_id, dh);
        }
    }

    pub(crate) fn apply_state(&self, commit_id: Serial, dh: &DisplayHandle) {
        for cache in &self.caches {
            cache.apply_state(commit_id, dh);
        }
    }
}
