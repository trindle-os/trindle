//! The Move your apps page: making a backup and putting one back.

use crate::store::{flat, refusal, go, Msg, Store};
use crate::common::run;
use crate::toolkit::chooser::{self, Ask};
use crate::toolkit::t;
use crate::toolkit::widget::{button, label, main_action, note, row, Node, Tone};
use crate::store::screen::{list_card, screen, title_row};
use std::cell::RefCell;
use super::job_rows::{begin, job_button, settle, update_row, updating};
use super::links::{more_about};
use super::{BackupPage, Waiting};

const HTTP_LINE: &str = "bk:http";

pub fn backup(s: &Store) -> Node<Msg> {
    let b = &s.backup;

    let mut tiles = Vec::new();
    let apps = t!("store-bk-apps-list", n = s.installed.len());

    let part = |head: &str, sub: &str, size: Option<&Option<String>>, on: bool, set: fn(&mut BackupPage, bool)| {
        let sub = match size {
            Some(Some(now)) => t!("store-bk-now", sub = sub, size = now),
            _ => sub.to_string(),
        };
        let switch = crate::toolkit::widget::switch(on, head.to_string(), move |on| go(move |s| set(&mut s.backup, !on)));
        update_row(s, "", head.to_string(), sub, vec![switch])
    };
    let file = crate::store::backup::default_file();

    let making = s.dials.get("bk:make").filter(|d| d.running());
    let make = match making {
        Some(d) => vec![d.round(button(crate::settings::ui::SAVE_ICON, t!("store-bk-making-tip")).key("bk:make")),
                        main_action(crate::settings::ui::SAVE_ICON, t!("store-bk-make")).off(true)],
        None => vec![main_action(crate::settings::ui::SAVE_ICON, t!("store-bk-make")).on_press(go(make_backup)).off_because(crate::common::ui::wait_while(updating(s)))],
    };
    let shown = button("folder-open-symbolic", t!("store-bk-show")).on_press(go(|_| {
        run::spawn(&[crate::common::apps::own_key("files").map_or("trindle-folders", |o| o.start_with()), &crate::common::files::home().display().to_string()]);
    }));
    tiles.push(list_card("document-save-symbolic", t!("store-bk-card"), vec![
        part(t!("store-bk-apps"), &t!("store-bk-apps-sub", list = apps),
             None, !b.no_apps, |p, off| p.no_apps = off),
        part(t!("store-bk-data"), t!("store-bk-data-sub"), Some(&b.data_size),
             !b.no_data, |p, off| p.no_data = off),
        part(t!("store-bk-appimages"), t!("store-bk-appimages-sub"), Some(&b.appimages_size), !b.no_appimages,
             |p, off| p.no_appimages = off),
        part(t!("store-bk-settings"), t!("store-bk-settings-sub"), None, !b.no_settings, |p, off| p.no_settings = off),
        update_row(s, "bk:make", t!("store-bk-where").into(),
                   t!("store-bk-where-sub", name = file.file_name().unwrap_or_default().to_string_lossy().to_string()),
                   vec![shown]),
    ]));

    let choose = job_button(s, "bk:back", "document-open-symbolic", t!("store-bk-choose"), choose_backup);
    tiles.push(list_card("document-open-symbolic", t!("store-bk-back-card"), vec![
        update_row(s, "bk:back", t!("store-bk-choose").into(),
                   t!("store-bk-choose-sub", flat = flat::name()), vec![choose]),
        note(t!("store-bk-back-note")),
    ]));
    if let Some((words, tone)) = s.said.get(HTTP_LINE).filter(|(w, _)| !w.is_empty()) {
        tiles.push(crate::store::help::beside(note(words.clone()).tone(*tone).key(HTTP_LINE), more_about(crate::store::help::BACKUP_HTTP)));
    }
    if let Some(w) = &b.waiting {
        tiles.push(http_card(w));
    }
    screen(title_row(crate::store::side::move_apps(), make), t!("store-bk-line"), tiles)
}

pub fn backup_opened(s: &mut Store) {
    crate::store::backup::size_of(".var/app", s.then(|s: &mut Store, size: String| s.backup.data_size = Some(size)));
    crate::store::backup::size_of(".appimage", s.then(|s: &mut Store, size: String| s.backup.appimages_size = Some(size)));
}

fn make_backup(s: &mut Store) {
    let b = &s.backup;
    let parts = crate::store::backup::Parts { apps: !b.no_apps, data: !b.no_data, appimages: !b.no_appimages,
                                       settings: !b.no_settings };
    begin(s, "bk:make", |d| d.expect(60.0));
    s.say("bk:make", t!("store-bk-making"), Tone::Plain);
    s.working += 1;
    let file = crate::store::backup::default_file();
    let f = file.clone();
    crate::store::backup::backup(parts, file, s.then(move |s: &mut Store, res: Result<String, String>| {
        s.job_over();
        settle(s, "bk:make", res.is_ok(), "");
        match res {
            Ok(size) => s.say("bk:make", t!("store-bk-made", path = f.display().to_string(), size = size), Tone::Plain),
            Err(e) => s.say("bk:make", e, Tone::Problem),
        }
    }));
}

pub(super) fn choose_backup(s: &mut Store) {

    if !crate::toolkit::window::persons_own(crate::common::person_only::APPS_BACKUP_BACK) {
        return;
    }
    let Some(bus) = s.bus.clone() else {
        s.say("bk:back", t!("store-bk-no-bus"), Tone::Problem);
        return;
    };
    let ask = Ask {
        title: t!("store-bk-choose-title").into(),
        folder: None,
        name: None,
        filters: vec![(t!("store-bk-filter").into(),
                       ["*.tar.gz", "*.fmpack", "flat-apps.json", "my-apps.json", "flatmorphic-apps.json"]
                           .iter().map(|p| p.to_string()).collect())],
    };
    let chosen = s.then(|s: &mut Store, picked: Option<std::path::PathBuf>| {
        s.chooser = None;
        match picked {
            Some(path) => ask_backup(s, path),
            None if crate::toolkit::chooser::failed() => s.say("bk:back", crate::toolkit::chooser::did_not_answer(), Tone::Problem),
            None => {}
        }
    });
    s.chooser = chooser::open(&bus, &s.events, ask, chosen);
    if s.chooser.is_none() {
        s.say("bk:back", t!("store-bk-no-answer"), Tone::Problem);
    }
}

fn restoring(s: &mut Store, line: &str, step: Option<(usize, usize)>) {
    if let Some((n, of)) = step {
        if let Some(d) = s.dials.get_mut("bk:back") {
            d.step(n, of, "");
        }
    }
    s.say("bk:back", line, Tone::Plain);
}

fn ask_backup(s: &mut Store, path: std::path::PathBuf) {
    use crate::store::link::Rule;
    s.backup.waiting = None;
    s.say(HTTP_LINE, "", Tone::Plain);
    let looked = path.clone();
    run::on_thread(move || crate::store::backup::rule_of(&looked), s.then(move |s: &mut Store, rule: Rule| match rule {
        Rule::Nothing => restore_backup(s, path, false),
        Rule::Line => {
            s.say(HTTP_LINE, t!("store-bk-http-line"), Tone::Warn);
            restore_backup(s, path, false);
        }
        Rule::Card => s.backup.waiting = Some(Waiting { path, ticked: false }),
    }));
}

fn http_card(w: &Waiting) -> Node<Msg> {
    let go_on = crate::toolkit::widget::main_word(t!("store-bk-http-go")).off(!w.ticked).on_press(go(|s| {
        if let Some(w) = s.backup.waiting.take().filter(|w| w.ticked) {
            restore_backup(s, w.path, true);
        }
    }));
    let cancel = crate::toolkit::widget::word_button(t!("store-bk-http-cancel")).on_press(go(|s| s.backup.waiting = None));
    list_card("dialog-warning-symbolic", t!("store-bk-http-title"), vec![
        crate::store::help::beside(note(t!("store-bk-http-card")).tone(Tone::Warn).key("bk:http-card"), more_about(crate::store::help::BACKUP_HTTP)),
        row(vec![
            crate::toolkit::widget::switch(w.ticked, t!("store-bk-http-tick"), |on| go(move |s| {
                if let Some(w) = s.backup.waiting.as_mut() {
                    w.ticked = on;
                }
            })),
            label(t!("store-bk-http-tick")),
        ]).gap(8.0).centred(),
        row(vec![go_on, cancel]).gap(8.0),
    ])
}

fn restore_backup(s: &mut Store, path: std::path::PathBuf, ticked: bool) {

    begin(s, "bk:back", |d| d.expect(60.0));
    s.working += 1;
    if crate::store::flat::is_flat(&path) {
        return restore_flat(s, path, ticked);
    }
    s.say("bk:back", t!("store-bk-reading"), Tone::Plain);
    let reading = path.clone();
    crate::store::backup::read_manifest(&reading, s.then(move |s: &mut Store, m: Result<serde_json::Value, String>| match m {
        Err(e) => {
            s.job_over();
            settle(s, "bk:back", false, "");
            s.say("bk:back", e, Tone::Problem);
        }
        Ok(manifest) => {
            let say = s.heard(|s, line| restoring(s, line, None));
            crate::store::backup::restore(path, manifest, ticked, say, s.then(|s: &mut Store, res: Result<(), String>| {
                s.job_over();
                settle(s, "bk:back", res.is_ok(), "");
                match res {
                    Ok(()) => s.say("bk:back", t!("store-bk-logout"), Tone::Plain),
                    Err(e) => s.say("bk:back", e, Tone::Problem),
                }
            }));
        }
    }));
}

fn restore_flat(s: &mut Store, path: std::path::PathBuf, ticked: bool) {
    s.say("bk:back", t!("store-bk-reading"), Tone::Plain);

    let p = path.clone();
    let line = s.heard_step(|s, line, step| restoring(s, &line, step));
    let end = RefCell::new(Some(s.then(move |s: &mut Store, (last, trindle, good): (String, bool, bool)| {
        let refresh = s.then(|s: &mut Store, ()| s.refresh_installed(|_| {}));
        if trindle {
            let say = s.heard(|s, line| restoring(s, line, None));
            crate::store::backup::restore_trindle_part(path, say, s.then(move |s: &mut Store, res: Result<(), String>| {
                s.job_over();
                settle(s, "bk:back", good && res.is_ok(), "");
                match res {
                    Ok(()) => s.say("bk:back", t!("store-bk-logout-after", last = &last),
                                    if good { Tone::Plain } else { Tone::Problem }),
                    Err(e) => s.say("bk:back", refusal::after(&last, &e),
                                    Tone::Problem),
                }
                refresh(());
            }));
        } else {
            s.job_over();
            settle(s, "bk:back", good, "");
            s.say("bk:back", last, if good { Tone::Plain } else { Tone::Problem });
            run::capture(&["trindle-icons", "--quiet"], None, move |_, _, _| refresh(()));
        }
    })));
    let post = s.events.post(move |m: Result<(String, Option<(usize, usize)>), (String, bool, bool)>| match m {
        Ok(l) => line(l),
        Err(last) => {
            if let Some(end) = end.borrow_mut().take() {
                end(last);
            }
        }
    });
    std::thread::spawn(move || {
        let file = match crate::store::flat::read(&p) {
            Ok(f) => f,
            Err(e) => {
                post.send(Err((e, false, false)));
                return;
            }
        };
        post.send(Ok((t!("store-bk-found", apps = file.apps.len(), with = file.with_settings()), None)));
        let out = crate::store::flat::restore(&file, ticked, |line, step| {
            post.send(Ok((line, step)));
        });
        post.send(Err((flat::summary(&out), file.trindle, out.failed.is_empty())));
    });
}
