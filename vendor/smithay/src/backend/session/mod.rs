//! Abstraction of different session APIs.

use rustix::fs::OFlags;
use std::{
    cell::RefCell,
    os::unix::io::OwnedFd,
    path::Path,
    rc::Rc,
    sync::{Arc, Mutex},
};

pub trait Session {
    type Error: AsErrno;

    fn open(&mut self, path: &Path, flags: OFlags) -> Result<OwnedFd, Self::Error>;
    fn close(&mut self, fd: OwnedFd) -> Result<(), Self::Error>;

    fn change_vt(&mut self, vt: i32) -> Result<(), Self::Error>;

    fn is_active(&self) -> bool;
    fn seat(&self) -> String;
}

#[derive(Copy, Clone, Debug)]
pub enum Event {
    PauseSession,
    ActivateSession,
}

impl Session for () {
    type Error = ();

    fn open(&mut self, _path: &Path, _flags: OFlags) -> Result<OwnedFd, Self::Error> {
        Err(())
    }
    fn close(&mut self, _fd: OwnedFd) -> Result<(), Self::Error> {
        Err(())
    }

    fn change_vt(&mut self, _vt: i32) -> Result<(), Self::Error> {
        Err(())
    }

    fn is_active(&self) -> bool {
        false
    }
    fn seat(&self) -> String {
        String::from("seat0")
    }
}

impl<S: Session> Session for Rc<RefCell<S>> {
    type Error = S::Error;

    fn open(&mut self, path: &Path, flags: OFlags) -> Result<OwnedFd, Self::Error> {
        self.borrow_mut().open(path, flags)
    }

    fn close(&mut self, fd: OwnedFd) -> Result<(), Self::Error> {
        self.borrow_mut().close(fd)
    }

    fn change_vt(&mut self, vt: i32) -> Result<(), Self::Error> {
        self.borrow_mut().change_vt(vt)
    }

    fn is_active(&self) -> bool {
        self.borrow().is_active()
    }

    fn seat(&self) -> String {
        self.borrow().seat()
    }
}

impl<S: Session> Session for Arc<Mutex<S>> {
    type Error = S::Error;

    fn open(&mut self, path: &Path, flags: OFlags) -> Result<OwnedFd, Self::Error> {
        self.lock().unwrap().open(path, flags)
    }

    fn close(&mut self, fd: OwnedFd) -> Result<(), Self::Error> {
        self.lock().unwrap().close(fd)
    }

    fn change_vt(&mut self, vt: i32) -> Result<(), Self::Error> {
        self.lock().unwrap().change_vt(vt)
    }

    fn is_active(&self) -> bool {
        self.lock().unwrap().is_active()
    }

    fn seat(&self) -> String {
        self.lock().unwrap().seat()
    }
}

pub trait AsErrno: ::std::fmt::Debug {
    fn as_errno(&self) -> Option<i32>;
}

impl AsErrno for () {
    fn as_errno(&self) -> Option<i32> {
        None
    }
}

#[cfg(feature = "backend_session_libseat")]
pub mod libseat;
