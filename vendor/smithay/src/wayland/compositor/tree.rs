use crate::{
    utils::{
        hook::{Hook, HookId},
        Serial,
    },
    wayland::compositor::SUBSURFACE_ROLE,
};

use super::{
    cache::MultiCache,
    handlers::{is_effectively_sync, SurfaceUserData},
    transaction::{Blocker, PendingTransaction, TransactionQueue},
    BufferAssignment, CompositorHandler, SurfaceAttributes, SurfaceData,
};
use std::{
    any::Any,
    fmt,
    sync::{atomic::Ordering, Arc, Mutex, MutexGuard},
};
use wayland_server::{
    protocol::{wl_output::Transform, wl_surface::WlSurface},
    DisplayHandle, Resource,
};

type CommitHook = dyn Fn(&mut dyn Any, &DisplayHandle, &WlSurface) + Send + Sync;
type DestructionHook = dyn Fn(&mut dyn Any, &WlSurface) + Send + Sync;

pub struct PrivateSurfaceData {
    parent: Option<WlSurface>,
    children: Vec<WlSurface>,
    public_data: SurfaceData,
    pending_transaction: PendingTransaction,
    current_txid: Serial,
    pre_commit_hooks: Vec<Hook<CommitHook>>,
    post_commit_hooks: Vec<Hook<CommitHook>>,
    destruction_hooks: Vec<Hook<DestructionHook>>,
}

impl fmt::Debug for PrivateSurfaceData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrivateSurfaceData")
            .field("parent", &self.parent)
            .field("children", &self.children)
            .field("public_data", &self.public_data)
            .field("pending_transaction", &"...")
            .field("current_txid", &self.current_txid)
            .field("commit_hooks", &"...")
            .field("pre_commit_hooks.len", &self.pre_commit_hooks.len())
            .field("post_commit_hooks.len", &self.post_commit_hooks.len())
            .field("destruction_hooks.len", &self.destruction_hooks.len())
            .finish()
    }
}

#[derive(Debug)]
pub struct AlreadyHasRole;

impl std::fmt::Display for AlreadyHasRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Surface already has a role.")
    }
}

impl std::error::Error for AlreadyHasRole {}

pub enum Location {
    Before,
    After,
}

#[derive(Debug)]
pub enum TraversalAction<T> {
    DoChildren(T),
    SkipChildren,
    Break,
}

impl PrivateSurfaceData {
    pub fn new() -> Mutex<PrivateSurfaceData> {
        Mutex::new(PrivateSurfaceData {
            parent: None,
            children: vec![],
            public_data: SurfaceData {
                role: Default::default(),
                data_map: Default::default(),
                cached_state: MultiCache::new(),
            },
            pending_transaction: Default::default(),
            current_txid: Serial(0),
            pre_commit_hooks: Vec::new(),
            post_commit_hooks: Vec::new(),
            destruction_hooks: Vec::new(),
        })
    }

    pub fn init(surface: &WlSurface) {
        let mut my_data = Self::lock_user_data(surface);
        debug_assert!(my_data.children.is_empty());
        my_data.children.push(surface.clone());
    }

    pub fn cleanup<D: 'static>(state: &mut D, surface_data: &SurfaceUserData, surface: &WlSurface) {
        let old_parent = surface_data.inner.lock().unwrap().parent.take();
        if let Some(old_parent) = old_parent {
            let old_parent_mutex = &old_parent.data::<SurfaceUserData>().unwrap().inner;
            let mut old_parent_guard = old_parent_mutex.lock().unwrap();
            old_parent_guard.children.retain(|c| c.id() != surface.id());
        }

        let my_data_mutex = &surface_data.inner;
        let mut my_data = my_data_mutex.lock().unwrap();
        for child in my_data.children.drain(..) {
            let child_mutex = &child.data::<SurfaceUserData>().unwrap().inner;
            if std::ptr::eq(child_mutex, my_data_mutex) {
                continue;
            }

            let mut child_guard = child_mutex.lock().unwrap();
            child_guard.parent = None;
        }
        let mut guard = my_data.public_data.cached_state.get::<SurfaceAttributes>();
        if let Some(BufferAssignment::NewBuffer(buffer)) = guard.current().buffer.take() {
            buffer.release();
        };
        if let Some(BufferAssignment::NewBuffer(buffer)) = guard.pending().buffer.take() {
            buffer.release();
        };

        let hooks = my_data.destruction_hooks.clone();
        drop(guard);
        drop(my_data);
        for hook in hooks {
            (hook.cb)(state, surface)
        }
    }

    pub fn lock_user_data(surface: &WlSurface) -> MutexGuard<'_, PrivateSurfaceData> {
        surface.data::<SurfaceUserData>().unwrap().inner.lock().unwrap()
    }

    pub fn set_role(surface: &WlSurface, role: &'static str) -> Result<(), AlreadyHasRole> {
        let mut my_data = Self::lock_user_data(surface);
        if my_data.public_data.role.is_some() && my_data.public_data.role != Some(role) {
            return Err(AlreadyHasRole);
        }
        my_data.public_data.role = Some(role);
        Ok(())
    }

    pub fn get_role(surface: &WlSurface) -> Option<&'static str> {
        Self::lock_user_data(surface).public_data.role
    }

    pub fn with_states<T, F: FnOnce(&SurfaceData) -> T>(surface: &WlSurface, f: F) -> T {
        let guard = Self::lock_user_data(surface);
        f(&guard.public_data)
    }

    pub fn add_blocker(surface: &WlSurface, blocker: impl Blocker + Send + 'static) {
        Self::lock_user_data(surface)
            .pending_transaction
            .add_blocker(blocker)
    }

    pub fn remove_pre_commit_hook(surface: &WlSurface, hook_id: HookId) {
        Self::lock_user_data(surface)
            .pre_commit_hooks
            .retain(|hook| hook.id != hook_id);
    }

    pub fn remove_post_commit_hook(surface: &WlSurface, hook_id: HookId) {
        Self::lock_user_data(surface)
            .post_commit_hooks
            .retain(|hook| hook.id != hook_id);
    }

    pub fn remove_destruction_hook(surface: &WlSurface, hook_id: HookId) {
        Self::lock_user_data(surface)
            .destruction_hooks
            .retain(|hook| hook.id != hook_id);
    }

    pub fn add_pre_commit_hook(
        surface: &WlSurface,
        hook: impl Fn(&mut dyn Any, &DisplayHandle, &WlSurface) + Send + Sync + 'static,
    ) -> HookId {
        let hook: Hook<CommitHook> = Hook::new(Arc::new(hook));
        let id = hook.id.clone();
        Self::lock_user_data(surface).pre_commit_hooks.push(hook);
        id
    }

    pub fn add_post_commit_hook(
        surface: &WlSurface,
        hook: impl Fn(&mut dyn Any, &DisplayHandle, &WlSurface) + Send + Sync + 'static,
    ) -> HookId {
        let hook: Hook<CommitHook> = Hook::new(Arc::new(hook));
        let id = hook.id.clone();
        Self::lock_user_data(surface).post_commit_hooks.push(hook);
        id
    }

    pub fn add_destruction_hook(
        surface: &WlSurface,
        hook: impl Fn(&mut dyn Any, &WlSurface) + Send + Sync + 'static,
    ) -> HookId {
        let hook: Hook<DestructionHook> = Hook::new(Arc::new(hook));
        let id = hook.id.clone();
        Self::lock_user_data(surface).destruction_hooks.push(hook);
        id
    }

    pub fn invoke_pre_commit_hooks<D: 'static>(state: &mut D, dh: &DisplayHandle, surface: &WlSurface) {
        let hooks = Self::lock_user_data(surface).pre_commit_hooks.clone();
        for hook in hooks {
            (hook.cb)(state, dh, surface);
        }
    }

    pub fn invoke_post_commit_hooks<D: 'static>(state: &mut D, dh: &DisplayHandle, surface: &WlSurface) {
        let hooks = Self::lock_user_data(surface).post_commit_hooks.clone();
        for hook in hooks {
            (hook.cb)(state, dh, surface);
        }
    }

    fn commit_sync_surface_tree(
        surface: &WlSurface,
        parent_transaction: &PendingTransaction,
        dh: &DisplayHandle,
    ) {
        let children = PrivateSurfaceData::get_children(surface);
        let mut my_data = Self::lock_user_data(surface);

        for child in children {
            Self::commit_sync_surface_tree(&child, &my_data.pending_transaction, dh);
        }

        let current_txid = my_data.current_txid;
        my_data.public_data.cached_state.commit(Some(current_txid), dh);
        my_data
            .pending_transaction
            .insert_state(surface.clone(), current_txid);

        let child_tx = std::mem::take(&mut my_data.pending_transaction);
        child_tx.merge_into(parent_transaction);
        my_data.current_txid.0 = my_data.current_txid.0.wrapping_add(1);
    }

    pub fn commit<C: CompositorHandler + 'static>(surface: &WlSurface, dh: &DisplayHandle, state: &mut C) {
        let is_sync = is_effectively_sync(surface);
        let children = PrivateSurfaceData::get_children(surface);
        let mut my_data = Self::lock_user_data(surface);
        let current_txid = my_data.current_txid;
        my_data.public_data.cached_state.commit(Some(current_txid), dh);
        for child in children {
            let mut child_data = Self::lock_user_data(&child);
            let is_child_sync = child_data
                .public_data
                .data_map
                .get::<super::handlers::SubsurfaceState>()
                .map(|s| s.sync.load(Ordering::Acquire))
                .unwrap_or(false);

            if !is_sync && is_child_sync {
                std::mem::drop(child_data);
                Self::commit_sync_surface_tree(&child, &my_data.pending_transaction, dh);
            } else if is_sync || is_child_sync {
                let child_tx = std::mem::take(&mut child_data.pending_transaction);
                child_tx.merge_into(&my_data.pending_transaction);
                child_data.current_txid.0 = child_data.current_txid.0.wrapping_add(1);
            }
        }
        my_data
            .pending_transaction
            .insert_state(surface.clone(), current_txid);
        my_data.current_txid.0 = my_data.current_txid.0.wrapping_add(1);
        if !is_sync {
            let client = match surface.client() {
                Some(client) => client,
                None => return,
            };
            let tx = std::mem::take(&mut my_data.pending_transaction);
            let mut queue_guard = state.client_compositor_state(&client).queue.lock().unwrap();
            let queue = queue_guard.get_or_insert_with(TransactionQueue::default);
            queue.append(tx.finalize());
            std::mem::drop(my_data);
            let transactions = queue.take_ready();
            std::mem::drop(queue_guard);
            for transaction in transactions {
                transaction.apply(dh, state)
            }
        }
    }

    pub fn is_ancestor(a: &WlSurface, b: &WlSurface) -> bool {
        let b_guard = Self::lock_user_data(b);
        if let Some(ref parent) = b_guard.parent {
            if parent.id() == a.id() {
                true
            } else {
                let parent = parent.clone();
                std::mem::drop(b_guard);
                Self::is_ancestor(a, &parent)
            }
        } else {
            false
        }
    }

    pub fn set_parent(child: &WlSurface, parent: &WlSurface) -> Result<(), AlreadyHasRole> {

        if child == parent || Self::is_ancestor(child, parent) {
            return Err(AlreadyHasRole);
        }

        {
            let mut child_guard = Self::lock_user_data(child);
            if child_guard.public_data.role.is_some() && child_guard.public_data.role != Some(SUBSURFACE_ROLE)
            {
                return Err(AlreadyHasRole);
            }
            if child_guard.parent.is_some() {
                return Err(AlreadyHasRole);
            }
            child_guard.public_data.role = Some(SUBSURFACE_ROLE);
            child_guard.parent = Some(parent.clone());
        }
        Self::lock_user_data(parent).children.push(child.clone());

        Ok(())
    }

    pub fn unset_parent(child: &WlSurface) {
        let old_parent = Self::lock_user_data(child).parent.take();
        if let Some(old_parent) = old_parent {
            Self::lock_user_data(&old_parent)
                .children
                .retain(|c| c.id() != child.id());
        }
    }

    pub fn get_parent(child: &WlSurface) -> Option<WlSurface> {
        Self::lock_user_data(child).parent.clone()
    }

    pub fn get_children(parent: &WlSurface) -> Vec<WlSurface> {
        Self::lock_user_data(parent)
            .children
            .iter()
            .filter(|s| s.id() != parent.id())
            .cloned()
            .collect()
    }

    pub fn reorder(surface: &WlSurface, to: Location, relative_to: &WlSurface) -> Result<(), ()> {
        let parent = Self::get_parent(surface).ok_or(())?;

        fn index_of(surface: &WlSurface, slice: &[WlSurface]) -> Option<usize> {
            for (i, s) in slice.iter().enumerate() {
                if s.id() == surface.id() {
                    return Some(i);
                }
            }
            None
        }

        let mut parent_guard = Self::lock_user_data(&parent);
        let my_index = index_of(surface, &parent_guard.children).unwrap();
        let mut other_index = match index_of(relative_to, &parent_guard.children) {
            Some(idx) => idx,
            None => return Err(()),
        };
        let me = parent_guard.children.remove(my_index);
        if my_index < other_index {
            other_index -= 1;
        }
        let new_index = match to {
            Location::Before => other_index,
            Location::After => other_index + 1,
        };
        parent_guard.children.insert(new_index, me);

        Ok(())
    }
}

impl PrivateSurfaceData {
    pub fn map_tree<F1, F2, F3, T>(
        surface: &WlSurface,
        initial: &T,
        mut filter: F1,
        mut processor: F2,
        mut post_filter: F3,
        reverse: bool,
    ) where
        F1: FnMut(&WlSurface, &SurfaceData, &T) -> TraversalAction<T>,
        F2: FnMut(&WlSurface, &SurfaceData, &T),
        F3: FnMut(&WlSurface, &SurfaceData, &T) -> bool,
    {
        Self::map(
            surface,
            initial,
            &mut filter,
            &mut processor,
            &mut post_filter,
            reverse,
        );
    }

    fn map<F1, F2, F3, T>(
        surface: &WlSurface,
        initial: &T,
        filter: &mut F1,
        processor: &mut F2,
        post_filter: &mut F3,
        reverse: bool,
    ) -> bool
    where
        F1: FnMut(&WlSurface, &SurfaceData, &T) -> TraversalAction<T>,
        F2: FnMut(&WlSurface, &SurfaceData, &T),
        F3: FnMut(&WlSurface, &SurfaceData, &T) -> bool,
    {
        let data_guard = &mut *Self::lock_user_data(surface);
        match filter(surface, &data_guard.public_data, initial) {
            TraversalAction::DoChildren(t) => {
                if reverse {
                    for c in data_guard.children.iter().rev() {
                        if c.id() == surface.id() {
                            processor(surface, &data_guard.public_data, initial);
                        } else if !Self::map(c, &t, filter, processor, post_filter, true) {
                            return false;
                        }
                    }
                } else {
                    for c in &data_guard.children {
                        if c.id() == surface.id() {
                            processor(surface, &data_guard.public_data, initial);
                        } else if !Self::map(c, &t, filter, processor, post_filter, false) {
                            return false;
                        }
                    }
                }
                post_filter(surface, &data_guard.public_data, initial)
            }
            TraversalAction::SkipChildren => {
                processor(surface, &data_guard.public_data, initial);
                true
            }
            TraversalAction::Break => false,
        }
    }
}

#[derive(Debug)]
pub struct SuggestedSurfaceState {
    pub scale: i32,
    pub transform: Transform,
}

impl Default for SuggestedSurfaceState {
    fn default() -> Self {
        Self {
            scale: 1,
            transform: Transform::Normal,
        }
    }
}
