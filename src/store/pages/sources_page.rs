//! The Software sources page.

use crate::store::{go, updates, Msg, Store};
use crate::common::files::user;
use crate::common::run;
use crate::common::sources::{self, KNOWN, LISTS};
use crate::toolkit::t;
use crate::toolkit::widget::{button, card, column, entry, label, note, row, Length, Node, Tone};
use crate::toolkit::theme;
use crate::store::screen::{list_card, screen, title_row};
use std::cell::RefCell;
use std::rc::Rc;
use super::job_rows::{banded, begin, job_button, pictured_row, settle, update_row};
use super::links::{link_button, open_link};
use super::pictures::{icon};
use super::{SourceForm};

pub fn sources(s: &Store) -> Node<Msg> {
    let p = &s.sources;
    if !p.loaded {
        return screen(title_row(t!("store-nav-sources"), Vec::new()), t!("store-looking"), vec![card(crate::settings::ui::loading())]);
    }
    let added = p.known.iter().filter(|(_, ours, _)| *ours).count();
    let head = t!("store-src-line", n = added);
    let rows = p.known.iter().map(|&(k, ours, here)| known_row(s, k, ours, here)).collect();
    let mut tiles = vec![list_card("network-server-symbolic", t!("store-src-ready"), banded(rows))];
    tiles.push(custom_card(s));
    if !p.others.is_empty() {
        tiles.push(others_card(s));
    }
    screen(title_row(t!("store-nav-sources"), Vec::new()), head, tiles)
}

fn known_row(s: &Store, k: &'static sources::Known, ours: bool, here: bool) -> Node<Msg> {
    let sub = match (ours, here) {
        (true, true) => t!("store-src-added", what = k.what),
        (true, false) => t!("store-src-no-program", what = k.what),
        (false, true) => t!("store-src-other-way", what = k.what),
        (false, false) => k.what.to_string(),
    };
    let key = format!("src:{}", k.id);
    let mut buttons = Vec::new();
    if ours {
        buttons.push(remove_source(s, &key, t!("store-src-remove-both"), k.id));
    }
    if here {
        if let Some(page) = k.page {
            buttons.push(link_button(&t!("store-src-open", name = k.name), page));
        }
    }
    if ours {
        if k.id == "tailscale" && here {
            buttons.push(button("network-vpn-symbolic", t!("store-src-tailscale-sign-in")).on_press(go(tailscale_sign_in)));
        }
    } else if !here {
        let id = k.id;
        buttons.push(source_button(s, &key, crate::settings::ui::INSTALL_ICON, &t!("store-src-add", name = k.name),
                                   move |s| source_job(s, &format!("src:{id}"), vec!["source-add".into(), id.into(), user()], t!("store-added"))));
    }
    pictured_row(s, Some(icon(k.icon.to_string(), theme::apps_list().row_icon)), &key, k.name.to_string(), sub, buttons)
}

fn custom_card(s: &Store) -> Node<Msg> {
    let f = &s.sources.form;
    let field = |head: &str, sub: &str, hint: &str, text: &str, set: fn(&mut SourceForm, String)| {
        column(vec![label(head.to_string()), note(sub.to_string()),
                    entry(text.to_string(), hint, move |t| go(move |s| set(&mut s.sources.form, t.clone())))
                        .width(Length::Fill)]).gap(4.0)
    };
    let add = source_button(s, "src:custom", crate::settings::ui::INSTALL_ICON, t!("store-src-add-this"), add_custom);
    list_card("list-add-symbolic", t!("store-src-another"), vec![
        note(t!("store-src-another-note")),
        field(t!("store-src-address"), t!("store-src-address-sub"),
              "https://apt.example.org/debian", &f.uri, |f, t| f.uri = t),
        field(t!("store-src-suite"), t!("store-src-suite-sub"), "trixie", &f.suite, |f, t| f.suite = t),
        field(t!("store-src-components"), t!("store-src-components-sub"), "main", &f.comps, |f, t| f.comps = t),
        field(t!("store-src-key"), t!("store-src-key-sub"), "https://apt.example.org/key.gpg", &f.key, |f, t| f.key = t),
        field(t!("store-src-program"), t!("store-src-program-sub"), t!("store-src-program-hint"), &f.package,
              |f, t| f.package = t),
        row(vec![s.said.get("src:custom").map_or_else(|| note("").width(Length::Fill), |(w, t)| if *t == Tone::Problem { crate::store::help::red(w.clone(), s.more("src:custom")) } else { note(w.clone()).tone(*t).width(Length::Fill) }), add])
            .gap(12.0).centred(),
    ])
}

fn others_card(s: &Store) -> Node<Msg> {
    let rows = s.sources.others.iter().map(|(file, addr)| {
        let mut buttons = Vec::new();
        if let Some(id) = file.strip_prefix("trindle-").and_then(|f| f.strip_suffix(".sources")) {
            buttons.push(remove_source(s, &format!("src:{id}"), t!("store-src-remove-this"), id));
        }
        update_row(s, &format!("src:{file}"), file.clone(), addr.clone(), buttons)
    }).collect();
    list_card("drive-harddisk-symbolic", t!("store-src-already"), banded(rows))
}

fn sources_busy(s: &Store) -> bool {
    s.dials.iter().any(|(k, d)| k.starts_with("src:") && d.running())
}

fn source_button(s: &Store, key: &str, icon: &str, tip: &str, press: impl Fn(&mut Store) + 'static) -> Node<Msg> {
    if s.dials.contains_key(key) || !sources_busy(s) {
        return job_button(s, key, icon, tip, press);
    }
    button(icon, tip)
}

fn remove_source(s: &Store, key: &str, tip: &str, id: &str) -> Node<Msg> {
    if s.dials.contains_key(key) || sources_busy(s) {
        return source_button(s, key, "user-trash-symbolic", tip, |_| {});
    }
    let (k, id) = (key.to_string(), id.to_string());
    crate::toolkit::widget::remove("user-trash-symbolic", tip).key(format!("source:{k}"))
        .on_press(go(move |s| source_job(s, &k, vec!["source-remove".into(), id.clone()], t!("store-done-removed"))))
}

fn add_custom(s: &mut Store) {
    let f = s.sources.form.clone();
    let (u, st, c, k, p) = (f.uri.trim().to_string(), f.suite.trim().to_string(), f.comps.trim().to_string(),
                            f.key.trim().to_string(), f.package.trim().to_string());
    let wrong = if !sources::valid_uri(&u) || !sources::valid_uri(&k) {
        Some(t!("store-src-bad-address"))
    } else if !sources::valid_words(&st) || !sources::valid_words(&c) {
        Some(t!("store-src-bad-words"))
    } else if !p.is_empty() && !sources::valid_package(&p) {
        Some(t!("store-src-bad-package"))
    } else {
        None
    };
    if let Some(w) = wrong {
        s.say("src:custom", w.to_string(), Tone::Problem);
        return;
    }
    let mut args = vec!["source-custom".to_string(), u, st, c, k];
    if !p.is_empty() {
        args.push(p);
    }
    source_job(s, "src:custom", args, t!("store-added"));
}

fn source_job(s: &mut Store, key: &str, args: Vec<String>, done: &'static str) {
    s.said.remove(key);
    begin(s, key, |d| {
        d.by_steps();
        d.say(t!("store-src-starting"));
    });
    s.working += 1;
    let k = key.to_string();
    let on_line = s.heard(move |s, line| {
        if let Some(d) = s.dials.get_mut(&k) {
            d.heard(line);
        }
    });
    let k = key.to_string();
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    updates::admin_watching(&argv, on_line, s.then(move |s: &mut Store, res: Result<(), String>| {
        s.job_over();
        match res {
            Ok(()) => {
                settle(s, &k, true, done);
                if k == "src:custom" {
                    s.sources.form = SourceForm::default();
                }
                sources_opened(s);
            }
            Err(e) => {
                settle(s, &k, false, t!("store-done-not"));
                s.say(&k, e, Tone::Problem);
            }
        }
    }));
}

fn tailscale_sign_in(s: &mut Store) {
    let key = "src:tailscale";
    s.say(key, t!("store-src-opening"), Tone::Calm);
    let last = Rc::new(RefCell::new(String::new()));
    let l = last.clone();
    let on_line = s.heard(move |s, line| {
        if let Some(url) = line.split_whitespace().find(|w| w.starts_with("https://login.tailscale.com/")) {
            open_link(s, url);
            s.say(key, t!("store-src-opened"), Tone::Calm);
        }
        if !line.trim().is_empty() {
            *l.borrow_mut() = line.to_string();
        }
    });
    let argv = crate::common::tailscale::up(&["--timeout=300s"]);
    run::stream(&argv.iter().map(String::as_str).collect::<Vec<_>>(), on_line, s.then(move |s: &mut Store, ok: bool| {
        if ok {
            s.say(key, t!("store-src-connected"), Tone::Plain);
        } else {
            s.say(key, last.borrow().clone(), Tone::Problem);
        }
    }));
}

fn deb_installed(package: &str) -> bool {
    std::process::Command::new("dpkg-query").args(["-W", "-f=${Status}", package]).output()
        .is_ok_and(|o| String::from_utf8_lossy(&o.stdout).contains("install ok installed"))
}

fn other_sources() -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = std::fs::read_dir(LISTS).map(|d| d.flatten().filter_map(|e| {
        let name = e.file_name().to_string_lossy().to_string();
        if !(name.ends_with(".list") || name.ends_with(".sources")) {
            return None;
        }
        if name.starts_with("trindle-") && !name.starts_with("trindle-custom-") {
            return None;
        }
        let text = std::fs::read_to_string(e.path()).unwrap_or_default();
        let addr = text.lines().find_map(|l| {
            let l = l.trim();
            if let Some(u) = l.strip_prefix("URIs:") {
                return u.split_whitespace().next().map(String::from);
            }
            if l.starts_with("deb ") {
                return l.split_whitespace().find(|w| w.starts_with("http")).map(String::from);
            }
            None
        }).unwrap_or_default();
        Some((name, addr))
    }).collect()).unwrap_or_default();
    out.sort();
    out
}

pub(crate) fn read_sources() -> (Vec<(&'static sources::Known, bool, bool)>, Vec<(String, String)>) {
    let arch = sources::arch();
    let known: Vec<_> = KNOWN.iter().filter(|k| k.archs.contains(&arch.as_str()))
        .map(|k| (k, sources::file_for(k.id).exists(), deb_installed(k.packages[0]))).collect();
    (known, other_sources())
}

pub fn sources_opened(s: &mut Store) {
    run::on_thread(read_sources, s.then(|s: &mut Store, (known, others)| {
        s.sources.known = known;
        s.sources.others = others;
        s.sources.loaded = true;
    }));
}
