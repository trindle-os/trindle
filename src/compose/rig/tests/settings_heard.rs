//! The compositor hears its settings saved at once, through inotify, even when their folder is made later.

#[cfg(test)]
use crate::rig::*;
use std::time::Duration;

#[test]
fn a_setting_saved_is_taken_up_at_once() {
    alone("rig::tests::settings_heard::a_setting_saved_is_taken_up_at_once", |dir| {
        let mut rig = Rig::new(dir);
        let conf = crate::common::files::trindle_conf();
        assert!(!conf.exists(), "the test began with settings saved");
        rig.state.watch_settings();
        assert_eq!(rig.state.config.workspaces, 1);
        let folder = conf.parent().expect("a folder");
        std::fs::create_dir_all(folder).unwrap();
        let beside = folder.join("trindle.conf.new");
        std::fs::write(&beside, "workspaces=on\n").unwrap();
        std::fs::rename(&beside, &conf).unwrap();

        let heard = rig.until(Duration::from_millis(400), |r| r.state.config.workspaces > 1);
        assert!(heard, "a setting saved was not taken up at once");
    });
}
