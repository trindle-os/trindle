// The transaction model for handling surface states in Smithay.

use std::{
    collections::HashSet,
    fmt,
    sync::{atomic::AtomicBool, Arc, Mutex},
};

use wayland_server::{protocol::wl_surface::WlSurface, DisplayHandle, Resource, Weak};

use crate::utils::Serial;

use super::{tree::PrivateSurfaceData, CompositorHandler};

pub trait Blocker {
    fn state(&self) -> BlockerState;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockerState {
    Pending,
    Released,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct Barrier(Arc<AtomicBool>);

impl PartialEq for Barrier {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for Barrier {}

impl Barrier {
    pub fn new(signaled: bool) -> Self {
        Self(Arc::new(AtomicBool::new(signaled)))
    }

    #[inline]
    pub fn is_signaled(&self) -> bool {
        self.0.load(std::sync::atomic::Ordering::Acquire)
    }

    #[inline]
    pub fn signal(&self) {
        self.0.store(true, std::sync::atomic::Ordering::Release)
    }
}

impl Blocker for Barrier {
    fn state(&self) -> BlockerState {
        if self.is_signaled() {
            BlockerState::Released
        } else {
            BlockerState::Pending
        }
    }
}

#[derive(Default)]
struct TransactionState {
    surfaces: Vec<(Weak<WlSurface>, Serial)>,
    blockers: Vec<Box<dyn Blocker + Send>>,
}

impl TransactionState {
    fn insert(&mut self, surface: WlSurface, id: Serial) {
        if let Some(place) = self.surfaces.iter_mut().find(|place| place.0 == surface) {
            if place.1 < id {
                place.1 = id;
            }
        } else {
            self.surfaces.push((surface.downgrade(), id));
        }
    }
}

enum TransactionInner {
    Data(TransactionState),
    Fused(Arc<Mutex<TransactionInner>>),
}

pub(crate) struct PendingTransaction {
    inner: Arc<Mutex<TransactionInner>>,
}

impl Default for PendingTransaction {
    fn default() -> Self {
        PendingTransaction {
            inner: Arc::new(Mutex::new(TransactionInner::Data(Default::default()))),
        }
    }
}

impl PendingTransaction {
    fn with_inner_state<T, F: FnOnce(&mut TransactionState) -> T>(&self, f: F) -> T {
        let mut next = self.inner.clone();
        loop {
            let tmp = match *next.lock().unwrap() {
                TransactionInner::Data(ref mut state) => return f(state),
                TransactionInner::Fused(ref into) => into.clone(),
            };
            next = tmp;
        }
    }

    pub(crate) fn insert_state(&self, surface: WlSurface, id: Serial) {
        self.with_inner_state(|state| state.insert(surface, id))
    }

    pub(crate) fn add_blocker<B: Blocker + Send + 'static>(&self, blocker: B) {
        self.with_inner_state(|state| state.blockers.push(Box::new(blocker) as Box<_>))
    }

    pub(crate) fn is_same_as(&self, other: &PendingTransaction) -> bool {
        let ptr1 = self.with_inner_state(|state| state as *const _);
        let ptr2 = other.with_inner_state(|state| state as *const _);
        ptr1 == ptr2
    }

    pub(crate) fn merge_into(&self, into: &PendingTransaction) {
        if self.is_same_as(into) {
            return;
        }
        let mut next = self.inner.clone();
        let my_state;
        loop {
            let tmp = {
                let mut guard = next.lock().unwrap();
                match *guard {
                    TransactionInner::Data(ref mut state) => {
                        my_state = std::mem::take(state);
                        *guard = TransactionInner::Fused(into.inner.clone());
                        break;
                    }
                    TransactionInner::Fused(ref into) => into.clone(),
                }
            };
            next = tmp;
        }
        self.with_inner_state(|state| {
            for (surface, id) in my_state.surfaces {
                if let Ok(surface) = surface.upgrade() {
                    state.insert(surface, id);
                }
            }
            state.blockers.extend(my_state.blockers);
        });
    }

    pub(crate) fn finalize(mut self) -> Transaction {
        loop {
            let inner = match Arc::try_unwrap(self.inner) {
                Ok(mutex) => mutex.into_inner().unwrap(),
                Err(_) => panic!("Attempting to finalize a transaction but handle is not the last."),
            };
            match inner {
                TransactionInner::Data(TransactionState {
                    surfaces, blockers, ..
                }) => return Transaction { surfaces, blockers },
                TransactionInner::Fused(into) => self.inner = into,
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Transaction {
    surfaces: Vec<(Weak<WlSurface>, Serial)>,
    blockers: Vec<Box<dyn Blocker + Send>>,
}

impl fmt::Debug for Box<dyn Blocker + Send> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blocker").field("state", &self.state()).finish()
    }
}

impl Transaction {
    pub(crate) fn state(&self) -> BlockerState {
        if !self.surfaces.iter().any(|surface| surface.0.is_alive()) {
            return BlockerState::Cancelled;
        }

        use BlockerState::*;
        self.blockers
            .iter()
            .fold(Released, |acc, blocker| match (acc, blocker.state()) {
                (Cancelled, _) | (_, Cancelled) => Cancelled,
                (Pending, _) | (_, Pending) => Pending,
                (Released, Released) => Released,
            })
    }

    pub(crate) fn apply<C: CompositorHandler + 'static>(self, dh: &DisplayHandle, state: &mut C) {
        for (surface, id) in self.surfaces {
            let Ok(surface) = surface.upgrade() else {
                continue;
            };

            PrivateSurfaceData::with_states(&surface, |states| {
                states.cached_state.apply_state(id, dh);
            });

            PrivateSurfaceData::invoke_post_commit_hooks::<C>(state, dh, &surface);

            tracing::trace!("Calling user implementation for wl_surface.commit");

            state.commit(&surface);
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct TransactionQueue {
    transactions: Vec<Transaction>,
    seen_surfaces: HashSet<u32>,
}

impl TransactionQueue {
    pub(crate) fn append(&mut self, t: Transaction) {
        self.transactions.push(t);
    }

    pub(crate) fn take_ready(&mut self) -> Vec<Transaction> {
        let mut ready_transactions = Vec::new();
        self.seen_surfaces.clear();
        let mut i = 0;
        while i < self.transactions.len() {
            let mut skip = false;
            match self.transactions[i].state() {
                BlockerState::Cancelled => {
                    self.transactions.remove(i);
                    continue;
                }
                BlockerState::Pending => {
                    skip = true;
                }
                BlockerState::Released => {}
            }
            if !skip {
                for (s, _) in &self.transactions[i].surfaces {
                    if !s.is_alive() {
                        continue;
                    }
                    if self.seen_surfaces.contains(&s.id().protocol_id()) {
                        skip = true;
                        break;
                    }
                }
            }

            if skip {
                for (s, _) in &self.transactions[i].surfaces {
                    if !s.is_alive() {
                        continue;
                    }
                    self.seen_surfaces.insert(s.id().protocol_id());
                }
                i += 1;
            } else {
                ready_transactions.push(self.transactions.remove(i));
            }
        }

        ready_transactions
    }
}
