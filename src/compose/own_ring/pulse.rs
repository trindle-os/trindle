//! Reading the pulse the ring and the screenshot tool write while a panel of theirs holds the keyboard.

use smithay::reexports::calloop::{
    generic::Generic, Interest, LoopHandle, Mode, PostAction, RegistrationToken,
};

use super::Kept;

impl Kept {

    pub(super) fn hear_shot(&mut self, number: u64) -> PostAction {
        let Some(shot) = self.shots.iter_mut().find(|s| s.number == number) else { return PostAction::Remove };
        let Some(pipe) = shot.pulse.as_mut() else { return PostAction::Remove };
        let looked = crate::ring_pulse::read(pipe, shot.heard, crate::ring_pulse::now());
        shot.heard = looked.heard;
        if looked.closed {

            shot.pulse = None;
            shot.hearing = None;
            return PostAction::Remove;
        }
        PostAction::Continue
    }

    pub(super) fn hear(&mut self, now: u64) -> bool {
        let Some(pipe) = self.pulse.as_mut() else { return true };
        let looked = crate::ring_pulse::read(pipe, self.heard, now);
        if looked.full {
            tracing::warn!("the ring's pulse had not been read for hours and its pipe was full: the ring counts as heard from now, and is judged from here");
        }
        self.heard = looked.heard;
        if looked.closed {
            self.pulse = None;
        }
        looked.closed
    }

    pub(super) fn quiet(&mut self, now: u64) -> bool {
        self.hear(now);
        crate::ring_pulse::silent(self.heard, now)
    }
}

pub(super) fn hear_on<D: 'static>(handle: &LoopHandle<'static, D>, pipe: &std::fs::File, kept: fn(&mut D) -> &mut Kept) -> Option<RegistrationToken> {
    let pipe = pipe.try_clone().ok()?;
    let source = Generic::new(pipe, Interest::READ, Mode::Level);
    handle
        .insert_source(source, move |_, _, data| {
            let kept = kept(data);
            if kept.hear(crate::ring_pulse::now()) {
                kept.hearing = None;
                return Ok(PostAction::Remove);
            }
            Ok(PostAction::Continue)
        })
        .ok()
}

#[cfg(test)]
impl Kept {

    pub(crate) fn running_now(&self) -> Option<(u64, u32)> {
        self.running.as_ref().map(|r| (r.0, r.1.id()))
    }

    pub(crate) fn counted(&self) -> usize {
        self.starts.len()
    }

    pub(crate) fn teller(&self) -> Option<smithay::reexports::calloop::channel::Sender<super::Told>> {
        self.told.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::os::fd::AsRawFd;
    use std::time::Duration;

    use super::super::launch::pipe;
    use super::*;

    #[test]
    fn the_rings_pulse_is_read_without_waiting_and_a_ring_gone_quiet_is_known() {
        use std::io::Write;
        let (read, write) = pipe().unwrap();
        // SAFETY: fcntl only sets a flag on a file this owns.
        assert_eq!(unsafe { libc::fcntl(read.as_raw_fd(), libc::F_SETFL, libc::O_NONBLOCK) }, 0);
        let (mut read, mut write) = (std::fs::File::from(read), std::fs::File::from(write));
        let newest_pulse = |read: &mut std::fs::File, heard| crate::ring_pulse::read(read, heard, crate::ring_pulse::now()).heard;

        assert_eq!(newest_pulse(&mut read, None), None);
        assert!(!crate::ring_pulse::silent(None, crate::ring_pulse::now()));

        let now = crate::ring_pulse::now();
        for at in [now - 2000, now - 1000, now] {
            write.write_all(&at.to_le_bytes()).unwrap();
        }
        let heard = newest_pulse(&mut read, None);
        assert_eq!(heard, Some(now));
        assert!(!crate::ring_pulse::silent(heard, now + 1000));

        let heard = newest_pulse(&mut read, heard);
        assert_eq!(heard, Some(now));
        assert!(crate::ring_pulse::silent(heard, now + crate::ring_pulse::SILENT_AFTER + 1));

        drop(write);
        assert_eq!(newest_pulse(&mut read, heard), Some(now));
    }

    #[test]
    fn a_healthy_ring_left_open_for_three_hours_with_no_key_is_not_ended_at_the_next_key() {
        let (read, write) = pipe().unwrap();
        for end in [&read, &write] {
            // SAFETY: fcntl only sets a flag on a file this owns.
            assert_eq!(unsafe { libc::fcntl(end.as_raw_fd(), libc::F_SETFL, libc::O_NONBLOCK) }, 0);
        }
        let mut write = std::fs::File::from(write);
        let mut events = smithay::reexports::calloop::EventLoop::<Kept>::try_new().unwrap();
        let mut kept = Kept { pulse: Some(std::fs::File::from(read)), ..Kept::default() };
        kept.hearing = hear_on(&events.handle(), kept.pulse.as_ref().unwrap(), |kept| kept);
        assert!(kept.hearing.is_some());

        let mut clock = 1_000_000u64;
        for _ in 0..3 * 3600 {
            clock += crate::ring_pulse::PULSE_EVERY;
            assert_eq!(crate::ring_pulse::write(&mut write, clock), crate::ring_pulse::Wrote::Sent, "a pulse was refused: the pipe was let fill");
            events.dispatch(Some(Duration::ZERO), &mut kept).unwrap();
            assert_eq!(kept.heard, Some(clock), "the pulse was not read as it was written");
        }

        assert!(!kept.quiet(clock + 500), "a healthy ring was taken for one that had stopped answering");

        events.dispatch(Some(Duration::from_millis(20)), &mut kept).unwrap();
        assert_eq!(kept.heard, Some(clock));

        assert!(kept.quiet(clock + crate::ring_pulse::SILENT_AFTER + 1));

        drop(write);

        let give_up = std::time::Instant::now() + Duration::from_secs(10);
        while kept.hearing.is_some() && std::time::Instant::now() < give_up {
            events.dispatch(Some(Duration::from_millis(5)), &mut kept).unwrap();
        }
        assert!(kept.hearing.is_none() && kept.pulse.is_none());
        events.dispatch(Some(Duration::ZERO), &mut kept).unwrap();
    }
}
