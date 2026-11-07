//! Installing and removing a Flathub app.

use crate::store::catalogue::{App, REMOTE};
use crate::store::reason::last_line;
use crate::store::{update_page, progress, Store};
use crate::common::filling::Dial;
use crate::common::run;
use crate::toolkit::t;
use crate::toolkit::widget::Tone;
use std::cell::RefCell;
use super::Pipe3;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct AppFacts {
    pub download: String,
    pub version: String,
}

fn parse_remote_info(out: &str) -> Option<AppFacts> {
    let field = |name: &str| out.lines().find_map(|l| l.trim().strip_prefix(name).map(|v| v.trim().replace('\u{a0}', " ")))
        .unwrap_or_default();
    let facts = AppFacts { download: field("Download:"), version: field("Version:") };
    (facts != AppFacts::default()).then_some(facts)
}

pub fn fetch_facts(s: &mut Store, a: &App) {
    if s.facts.contains_key(&a.id) {
        return;
    }
    s.facts.insert(a.id.clone(), None);
    let id = a.id.clone();
    let back = s.then(move |s: &mut Store, out: String| {
        s.facts_heard.insert(id.clone());
        s.facts.insert(id, parse_remote_info(&out));
    });
    run::capture(&["flatpak", "remote-info", "--user", REMOTE, &a.id], None, move |_, out, _| back(out));
}

pub(crate) fn pressed_install(s: &mut Store, install: impl FnOnce(&mut Store)) {
    if !crate::toolkit::window::persons_own(crate::common::person_only::APPS_INSTALL) {
        return;
    }
    install(s);
}

pub(crate) fn install(s: &mut Store, a: &App) {
    let id = a.id.clone();
    s.said.remove(&id);
    s.busy.insert(id.clone(), "install");
    s.dials.insert(id.clone(), Dial::default());
    s.working += 1;
    let job = RefCell::new(progress::Job::default());
    let i = id.clone();
    let on_line = s.heard(move |s, line| match job.borrow_mut().heard(line) {
        Some(at) => {
            if let Some(d) = s.dials.get_mut(&i) {
                d.within(at.share);
            }
        }
        None => {
            let line = line.trim();
            if !line.is_empty() {
                s.trouble.insert(i.clone(), line.to_string());
            }
        }
    });
    let app = a.clone();
    let done = s.then(move |s: &mut Store, ok: bool| {
        s.job_over();
        if !ok && !s.jobs.contains_key(&app.id) {
            s.busy.remove(&app.id);
            s.dials.remove(&app.id);
            return;
        }
        let e = s.trouble.remove(&app.id).unwrap_or_default();
        finished(s, &app, ok, &e, t!("store-done-installed-note"));
    });
    if let Some(p) = run::stream_stoppable(&run::line_by_line(&crate::common::gentle::argv(&["flatpak", "install", "--user", "-y", REMOTE, &id])), on_line, done) {
        s.jobs.insert(id, p);
    }
}

pub(crate) fn give_up(s: &mut Store, id: &str) {
    if let Some(job) = s.jobs.remove(id) {
        job.force_exit();
    }
}

pub(crate) fn uninstall(s: &mut Store, a: &App, wipe: bool) {
    s.said.remove(&a.id);
    s.busy.insert(a.id.clone(), "uninstall");
    s.dials.insert(a.id.clone(), Dial::default());
    let system = s.installed.get(&a.id).is_some_and(|(place, _)| place == "system");
    let place = if system { "--system" } else { "--user" };
    let app = a.clone();
    let argv = crate::common::removal::flatpak_uninstall(Some(place), &a.id, wipe);
    let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
    run::capture(&argv, None,
                 s.then(move |s: &mut Store, (ok, _o, e): (bool, String, String)| finished(s, &app, ok, &e, t!("store-done-removed"))).pipe3());
}

fn finished(s: &mut Store, a: &App, ok: bool, err: &str, success: &'static str) {
    let removed = ok && s.busy.get(&a.id) == Some(&"uninstall");
    let words = if !ok { t!("store-done-not") } else if removed { t!("store-done-removed") } else { t!("store-done-installed") };
    if let Some(d) = s.dials.get_mut(&a.id) {
        d.finish(ok, words);
    }
    s.dials.remove(&a.id);
    s.busy.remove(&a.id);
    s.jobs.remove(&a.id);
    if ok {
        run::capture(&["trindle-icons", "--quiet"], None, |_, _, _| {});
    }
    let said = if ok { (success.to_string(), Tone::Plain) } else { (last_line(err, t!("store-flatpak-refused")), Tone::Problem) };
    s.said.insert(a.id.clone(), said);
    if removed {
        update_page::hold::removed(s, &a.id);
    }
    s.refresh_installed(|_| {});
}
