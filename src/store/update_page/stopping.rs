//! Stopping an update that runs as the person.

use crate::toolkit::t;
use crate::store::{go, Msg, Store};
use crate::common::run;
use crate::common::stopping::Stop;
use crate::toolkit::widget::{button, Node};
use super::{STOP_ICON, UpdatesPage};

pub(crate) fn stop_key(key: &str) -> String {
    format!("stop:{key}")
}

impl UpdatesPage {
    pub(crate) fn stop_button(&self, key: &str) -> Option<Node<Msg>> {
        let stop = self.stops.get(key)?;
        let key = key.to_string();
        Some(button(STOP_ICON, t!("store-upd-stop")).key(stop_key(&key)).on_press(go(move |s| stop_update(s, &key)))
            .off_because((!stop.offered()).then(|| t!("store-upd-stopping"))))
    }
}

pub(crate) fn stop_update(s: &mut Store, key: &str) {
    if let Some(stop) = s.upd.stops.get(key) {
        stop.press();
    }
}

pub(crate) fn stopped(s: &mut Store, key: &str, stop: &Stop, ok: bool) -> bool {
    stop.release();
    s.upd.stops.remove(key);
    let was = stop.pressed() && !ok;
    if was {
        s.dials.remove(key);
        s.said.remove(key);
        s.upd.stopped.insert(key.to_string());
    }
    was
}

pub(crate) fn start_stoppable(stop: &Stop, argv: &[&str], on_line: impl Fn(&str) + 'static, done: impl FnOnce(bool) + 'static) {
    #[cfg(test)]
    let stand_in = super::tests::stand_in();
    #[cfg(test)]
    let argv: &[&str] = &stand_in.as_deref().map_or_else(|| argv.to_vec(), |script| vec!["sh", "-c", script]);
    if let Some(proc) = run::stream_group(argv, on_line, done) {
        stop.hold(proc);
    }
}
