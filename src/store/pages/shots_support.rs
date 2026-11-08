//! Made-up page states for the failure pictures (page_shots_test.rs); private fields of the pages are reached from here.

use super::*;

pub(in crate::store) fn backup_waiting() -> BackupPage {
    BackupPage { waiting: Some(Waiting { path: std::path::PathBuf::from("/home/john/trindle-backup-2026-10-01.fmpack"), ticked: false }), ..made_up::backup() }
}

pub(in crate::store) fn source_form(uri: &str, suite: &str, key: &str, package: &str) -> SourcesPage {
    let mut p = made_up::sources();
    p.known.truncate(1);
    p.others.clear();
    p.form = SourceForm { uri: uri.into(), suite: suite.into(), comps: "main".into(), key: key.into(), package: package.into() };
    p
}
