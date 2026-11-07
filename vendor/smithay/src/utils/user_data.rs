//! Various utilities used for user data implementations.

use std::any::Any;
use std::mem::ManuallyDrop;
use std::sync::OnceLock;
use std::thread::{self, ThreadId};

use self::list::AppendList;

#[inline]
fn current_thread_id() -> ThreadId {
    thread_local! {
        static ID: ThreadId = thread::current().id();
    }
    ID.with(|id| *id)
}

#[derive(Debug)]
pub struct UserData {
    inner: OnceLock<UserDataInner>,
}

#[derive(Debug)]
enum UserDataInner {
    ThreadSafe(Box<dyn Any + Send + Sync + 'static>),
    NonThreadSafe(Box<ManuallyDrop<dyn Any + 'static>>, ThreadId),
}

unsafe impl Send for UserData {}
unsafe impl Sync for UserData {}

impl Default for UserData {
    fn default() -> Self {
        Self::new()
    }
}

impl UserData {
    pub const fn new() -> UserData {
        UserData {
            inner: OnceLock::new(),
        }
    }

    pub fn set<T: Any + 'static, F: FnOnce() -> T>(&self, f: F) {
        self.inner.get_or_init(|| {
            UserDataInner::NonThreadSafe(Box::new(ManuallyDrop::new(f())), current_thread_id())
        });
    }

    pub fn set_threadsafe<T: Any + Send + Sync + 'static, F: FnOnce() -> T>(&self, f: F) {
        self.inner
            .get_or_init(|| UserDataInner::ThreadSafe(Box::new(f())));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        match self.inner.get() {
            Some(UserDataInner::ThreadSafe(val)) => <dyn Any>::downcast_ref::<T>(&**val),
            Some(&UserDataInner::NonThreadSafe(ref val, threadid)) => {
                if threadid == current_thread_id() {
                    <dyn Any>::downcast_ref::<T>(&***val)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

impl Drop for UserData {
    fn drop(&mut self) {
        if let Some(&mut UserDataInner::NonThreadSafe(ref mut val, threadid)) = self.inner.get_mut() {
            if threadid == current_thread_id() {
                unsafe {
                    ManuallyDrop::drop(&mut **val);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct UserDataMap {
    list: AppendList<UserData>,
}

impl UserDataMap {
    pub fn new() -> UserDataMap {
        UserDataMap {
            list: AppendList::new(),
        }
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        for user_data in &self.list {
            if let Some(val) = user_data.get::<T>() {
                return Some(val);
            }
        }
        None
    }

    pub fn get_or_insert<T: 'static, F: FnOnce() -> T>(&self, init: F) -> &T {
        match self.get() {
            Some(data) => data,
            None => {
                self.insert(init);

                self.list.iter().last().and_then(|data| data.get::<T>()).unwrap()
            }
        }
    }

    pub fn get_or_insert_threadsafe<T: Send + Sync + 'static, F: FnOnce() -> T>(&self, init: F) -> &T {
        match self.get() {
            Some(data) => data,
            None => {
                self.insert_threadsafe(init);

                self.list.iter().last().and_then(|data| data.get::<T>()).unwrap()
            }
        }
    }

    pub fn insert_if_missing<T: 'static, F: FnOnce() -> T>(&self, init: F) -> bool {
        if self.get::<T>().is_some() {
            return false;
        }

        self.insert(init);

        true
    }

    pub fn insert_if_missing_threadsafe<T: Send + Sync + 'static, F: FnOnce() -> T>(&self, init: F) -> bool {
        if self.get::<T>().is_some() {
            return false;
        }

        self.insert_threadsafe(init);

        true
    }

    fn insert<T: 'static, F: FnOnce() -> T>(&self, init: F) {
        let data = UserData::new();
        data.set(init);
        self.list.append(data);
    }

    fn insert_threadsafe<T: Send + Sync + 'static, F: FnOnce() -> T>(&self, init: F) {
        let data = UserData::new();
        data.set_threadsafe(init);
        self.list.append(data);
    }
}

impl Default for UserDataMap {
    fn default() -> UserDataMap {
        UserDataMap::new()
    }
}

mod list {
    /*
     * This is a lock-free append-only list, it is used as an implementation
     * detail of the UserDataMap.
     *
     * It was extracted from https://github.com/Diggsey/lockless under MIT license
     * Copyright © Diggory Blake <diggsey@googlemail.com>
     */

    use std::sync::atomic::{AtomicPtr, Ordering};
    use std::{mem, ptr};

    type NodePtr<T> = Option<Box<Node<T>>>;

    #[derive(Debug)]
    struct Node<T> {
        value: T,
        next: AppendList<T>,
    }

    #[derive(Debug)]
    pub struct AppendList<T>(AtomicPtr<Node<T>>);

    impl<T> AppendList<T> {
        fn node_into_raw(ptr: NodePtr<T>) -> *mut Node<T> {
            match ptr {
                Some(b) => Box::into_raw(b),
                None => ptr::null_mut(),
            }
        }
        unsafe fn node_from_raw(ptr: *mut Node<T>) -> NodePtr<T> {
            if ptr.is_null() {
                None
            } else {
                Some(Box::from_raw(ptr))
            }
        }

        fn new_internal(ptr: NodePtr<T>) -> Self {
            AppendList(AtomicPtr::new(Self::node_into_raw(ptr)))
        }

        pub fn new() -> Self {
            Self::new_internal(None)
        }

        pub fn append(&self, value: T) {
            self.append_list(AppendList::new_internal(Some(Box::new(Node {
                value,
                next: AppendList::new(),
            }))));
        }

        unsafe fn append_ptr(&self, p: *mut Node<T>) {
            loop {
                match self
                    .0
                    .compare_exchange_weak(ptr::null_mut(), p, Ordering::AcqRel, Ordering::Acquire)
                {
                    Ok(_) => return,
                    Err(head) => {
                        if !head.is_null() {
                            return (*head).next.append_ptr(p);
                        }
                    }
                }
            }
        }

        pub fn append_list(&self, other: AppendList<T>) {
            let p = other.0.load(Ordering::Acquire);
            mem::forget(other);
            unsafe { self.append_ptr(p) };
        }

        pub fn iter(&self) -> AppendListIterator<'_, T> {
            AppendListIterator(&self.0)
        }

        pub fn iter_mut(&mut self) -> AppendListMutIterator<'_, T> {
            AppendListMutIterator(&mut self.0)
        }
    }

    impl<'a, T> IntoIterator for &'a AppendList<T> {
        type Item = &'a T;
        type IntoIter = AppendListIterator<'a, T>;

        fn into_iter(self) -> AppendListIterator<'a, T> {
            self.iter()
        }
    }

    impl<'a, T> IntoIterator for &'a mut AppendList<T> {
        type Item = &'a mut T;
        type IntoIter = AppendListMutIterator<'a, T>;

        fn into_iter(self) -> AppendListMutIterator<'a, T> {
            self.iter_mut()
        }
    }

    impl<T> Drop for AppendList<T> {
        fn drop(&mut self) {
            unsafe { Self::node_from_raw(mem::replace(self.0.get_mut(), ptr::null_mut())) };
        }
    }

    #[derive(Debug)]
    pub struct AppendListIterator<'a, T>(&'a AtomicPtr<Node<T>>);

    impl<'a, T: 'a> Iterator for AppendListIterator<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> Option<&'a T> {
            let p = self.0.load(Ordering::Acquire);
            if p.is_null() {
                None
            } else {
                unsafe {
                    self.0 = &(*p).next.0;
                    Some(&(*p).value)
                }
            }
        }
    }

    #[derive(Debug)]
    pub struct AppendListMutIterator<'a, T>(&'a mut AtomicPtr<Node<T>>);

    impl<'a, T: 'a> Iterator for AppendListMutIterator<'a, T> {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<&'a mut T> {
            let p = self.0.load(Ordering::Acquire);
            if p.is_null() {
                None
            } else {
                unsafe {
                    self.0 = &mut (*p).next.0;
                    Some(&mut (*p).value)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::UserDataMap;

    #[test]
    fn insert_twice() {
        let map = UserDataMap::new();

        assert_eq!(map.get::<usize>(), None);
        assert!(map.insert_if_missing(|| 42usize));
        assert!(!map.insert_if_missing(|| 43usize));
        assert_eq!(map.get::<usize>(), Some(&42));
    }
}
