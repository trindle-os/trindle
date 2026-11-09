// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use std::io::Write;
use std::time::Duration;

use crate::state::ClientState;

use super::*;

pub fn beat(pulse: &mut std::fs::File) {
    pulse.write_all(&crate::ring_pulse::now().to_le_bytes()).unwrap();
}

pub fn there(pid: u32) -> bool {
    // SAFETY: kill with signal 0 sends nothing; it only says whether the

    unsafe { libc::kill(pid as libc::pid_t, 0) == 0 }
}

pub struct KeptRing {
    pub who: usize,
    pub number: u64,
    pub pid: u32,
    pub pulse: std::fs::File,

    pub hand: std::fs::File,
}

impl Rig {
    pub fn kept_ring(&mut self) -> KeptRing {
        self.state.keep_ring().expect("a ring is kept");
        let (number, pulse, hand) = self.state.start_ring_for_rig().expect("the ring's stand-in starts");
        let (_, pid) = self.state.ring.running_now().unwrap();
        let told = self.state.ring.teller().unwrap();
        let who = self.join(ClientState { ours: true, left: Some((told, number)), ..ClientState::default() });
        KeptRing { who, number, pid, pulse, hand }
    }
}

impl KeptRing {

    pub fn turn(&mut self, out: &[&str]) {
        if !out.is_empty() {
            beat(&mut self.pulse);
        }
    }
}

const BEAT: Duration = Duration::from_millis(900);

pub fn a_while(rig: &mut Rig, ring: &mut KeptRing, out: &[&str]) {
    let turns = crate::ring_pulse::SILENT_AFTER / BEAT.as_millis() as u64 + 1;
    for _ in 0..turns {
        std::thread::sleep(BEAT);
        ring.turn(out);
        rig.pump();
    }
}
