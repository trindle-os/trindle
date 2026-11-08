// Origin: began from anvil, Smithay's example compositor (MIT, Copyright (c) 2017 Victor Berger and Victoria Brekenfeld), changed by Trindle. Licence text: THIRD-PARTY.txt.

use smithay::output::Output;

use super::*;

impl Rig {

    pub fn lock_by(&mut self, who: usize) {
        let manager = self.programs[who].program.lock_manager.clone().expect("no lock manager");
        let qh = self.programs[who].queue.handle();
        self.programs[who].program.lock_object = Some(manager.lock(&qh, ()));
        self.pump();
        let screens: Vec<Output> = self.state.space.outputs().cloned().collect();
        for output in &screens {
            self.state.lock_frame_shown(output);
        }
        self.pump();
        assert!(self.programs[who].program.held, "the lock did not hold");
    }

    pub fn lock_surfaces(&mut self, who: usize) {
        let p = &self.programs[who];
        let qh = p.queue.handle();
        let lock = p.program.lock_object.clone().expect("no lock");
        let compositor = p.program.compositor.as_ref().expect("the compositor's global");
        for output in &p.program.outputs {
            let surface = compositor.create_surface(&qh, ());
            let _ = lock.get_lock_surface(&surface, output, &qh, ());
        }
        self.pump();
    }

    pub fn sees_unlock(&self, who: usize) -> bool {
        self.programs[who].program.unlock.is_some()
    }

    pub fn asks_to_unlock(&mut self, who: usize, pin: &str) -> Option<Result<(), String>> {
        let way = self.programs[who].program.unlock.clone()?;
        way.with_pin(pin.into());
        self.pump();
        self.programs[who].program.said.pop()
    }

    pub fn unlock_straight(&mut self, who: usize, pin: &str) -> Result<(), String> {
        use crate::protocols::unlock::UnlockHandler;
        let client = self.programs[who].client.clone();
        self.state.unlock_with_pin(&client, pin)
    }

    pub fn lock_ended(&self, who: usize) -> bool {
        !self.state.is_locked() && self.programs[who].program.finished
    }
}
