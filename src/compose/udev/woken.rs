// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::{
    io,
    os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd},
};

use smithay::reexports::calloop::{generic::Generic, Interest, LoopHandle, Mode, PostAction};
use tracing::{info, warn};

use super::{console_switch, Compose, UdevData, SCREENS_BACK_LOG};

pub(super) struct Woken(OwnedFd);

impl Woken {
    pub(super) fn new() -> io::Result<Woken> {
        // SAFETY: plain call; the result is checked before it is owned.
        let fd = unsafe { libc::timerfd_create(libc::CLOCK_REALTIME, libc::TFD_CLOEXEC | libc::TFD_NONBLOCK) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        // SAFETY: a new descriptor that nothing else holds.
        let woken = Woken(unsafe { OwnedFd::from_raw_fd(fd) });
        woken.arm()?;
        Ok(woken)
    }

    fn arm(&self) -> io::Result<()> {
        let none = libc::timespec { tv_sec: 0, tv_nsec: 0 };
        let far = libc::itimerspec { it_interval: none, it_value: libc::timespec { tv_sec: libc::time_t::MAX, tv_nsec: 0 } };
        let flags = libc::TFD_TIMER_ABSTIME | libc::TFD_TIMER_CANCEL_ON_SET;
        // SAFETY: `far` lives for the call; the old value is not asked for.
        if unsafe { libc::timerfd_settime(self.0.as_raw_fd(), flags, &far, std::ptr::null_mut()) } < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    pub(super) fn heard(&self) -> io::Result<bool> {
        let mut ticks = [0u8; 8];
        // SAFETY: reads at most the buffer's own length into it.
        let read = unsafe { libc::read(self.0.as_raw_fd(), ticks.as_mut_ptr().cast(), ticks.len()) };
        if read >= 0 {

            self.arm()?;
            return Ok(false);
        }
        let err = io::Error::last_os_error();
        match err.raw_os_error() {
            Some(libc::ECANCELED) => self.arm().map(|()| true),
            Some(libc::EAGAIN) | Some(libc::EINTR) => Ok(false),
            _ => Err(err),
        }
    }
}

impl AsFd for Woken {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

pub(super) fn watch(handle: &LoopHandle<'static, Compose<UdevData>>) {
    let woken = match Woken::new() {
        Ok(woken) => woken,
        Err(err) => {
            warn!(?err, "cannot hear the computer wake; after a hibernation the screens may stay black until something moves");
            return;
        }
    };
    let added = handle.insert_source(Generic::new(woken, Interest::READ, Mode::Level), |_, woken, data| {
        match woken.heard() {
            Ok(true) => {
                info!(target: SCREENS_BACK_LOG, "the clock was set or the computer woke, drawing every screen afresh");
                console_switch::draw_afresh(data);
                Ok(PostAction::Continue)
            }
            Ok(false) => Ok(PostAction::Continue),

            Err(err) => {
                warn!(?err, "stopped listening for the computer waking");
                Ok(PostAction::Remove)
            }
        }
    });
    if let Err(err) = added {
        warn!(?err, "cannot hear the computer wake");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn readable(woken: &Woken) -> bool {
        let mut fd = libc::pollfd { fd: woken.0.as_raw_fd(), events: libc::POLLIN, revents: 0 };
        // SAFETY: one pollfd, no wait.
        assert!(unsafe { libc::poll(&mut fd, 1, 0) } >= 0);
        fd.revents & libc::POLLIN != 0
    }

    #[test]
    fn a_new_wake_watch_is_quiet_and_armed_on_the_wall_clock() {
        let woken = Woken::new().expect("a wake watch");

        assert!(!readable(&woken));
        assert!(!woken.heard().unwrap());
        let mut now = libc::itimerspec {
            it_interval: libc::timespec { tv_sec: 0, tv_nsec: 0 },
            it_value: libc::timespec { tv_sec: 0, tv_nsec: 0 },
        };
        // SAFETY: `now` lives for the call.
        assert_eq!(unsafe { libc::timerfd_gettime(woken.0.as_raw_fd(), &mut now) }, 0);

        assert!(now.it_value.tv_sec > 100 * 365 * 24 * 3600, "{}", now.it_value.tv_sec);
        assert_eq!(now.it_interval.tv_sec, 0);
        assert!(!readable(&woken));
    }
}
