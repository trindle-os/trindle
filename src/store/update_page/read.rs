//! Reading what waits: Flatpak's list and apt's, and parsing them.

use crate::store::{updates, Store};
use crate::common::held::{self};
use crate::common::run;
use super::check::{could_not};
use super::words::no_flathub;
use super::{APPS, Flat};

pub(crate) fn read_flatpaks(s: &mut Store, cached: bool, then: impl FnOnce(&mut Store) + 'static) {
    let cached = cached || !s.online;
    let back = s.then(move |s: &mut Store, (ok, out): (bool, String)| {
        if flatpaks_heard(s, cached, ok, &out) {
            return read_flatpaks(s, true, then);
        }
        then(s);
    });
    let mut argv = vec!["flatpak", "remote-ls", "--user", "--updates", "--columns=ref,origin,version,download-size,name"];
    if cached {
        argv.push("--cached");
    }
    run::capture(&argv, None, move |ok, out, _| back((ok, out)));
}

pub(super) fn flatpaks_heard(s: &mut Store, cached: bool, ok: bool, out: &str) -> bool {
    if ok {
        waiting_flatpaks(s, parse_flat_updates(out));
        if !cached {
            s.upd.failed.remove(APPS);
        }
        return false;
    }
    if !cached {
        could_not(s, APPS, no_flathub());
    }
    !cached
}

fn waiting_flatpaks(s: &mut Store, found: Vec<Flat>) {
    s.updatable = found.iter().filter(|f| f.app).map(|f| f.id.clone()).collect();
    s.upd.sizes.extend(found.iter().filter(|f| f.app).map(|f| (f.id.clone(), f.size)));
    s.upd.flats = found;
}

pub(super) fn read_system(s: &mut Store, then: impl FnOnce(&mut Store) + 'static) {
    updates::trindle_waiting(s.then(move |s: &mut Store, v: Option<String>| {
        s.trindle_waiting = v;
        updates::debian_upgradable(s.then(move |s: &mut Store, list: Vec<String>| {
            s.debian = list;
            let mut names = s.debian.clone();
            if s.trindle_waiting.is_some() {
                names.push("trindle".into());
            }
            if names.is_empty() {
                then(s);
                return;
            }
            let back = s.then(move |s: &mut Store, out: String| {
                s.upd.sizes.extend(parse_package_sizes(&out));
                then(s);
            });
            let mut argv = vec!["apt-cache", "show", "--no-all-versions"];
            argv.extend(names.iter().map(String::as_str));
            run::capture(&argv, None, move |_, out, _| back(out));
        }));
    }));
}

pub(super) fn parse_flat_updates(out: &str) -> Vec<Flat> {
    out.lines().filter_map(|l| {
        let (app, id, arch, branch) = held::flat_ref(l)?;
        let mut cols = l.split('\t').skip(1);
        Some(Flat {
            id: id.to_string(), arch: arch.to_string(), branch: branch.to_string(), app,
            origin: cols.next().unwrap_or("").trim().to_string(),
            version: cols.next().unwrap_or("").trim().to_string(),
            size: cols.next().map(bytes_from).unwrap_or(0),
            name: cols.next().unwrap_or("").trim().to_string(),
        })
    }).collect()
}

pub(super) fn bytes_from(text: &str) -> u64 {
    let text = text.replace('\u{a0}', " ");
    let mut words = text.split_whitespace();
    let Some(figure) = words.next().and_then(|f| f.parse::<f64>().ok()) else { return 0 };
    let unit = match words.next().unwrap_or("bytes") {
        "kB" | "KB" => 1e3,
        "MB" => 1e6,
        "GB" => 1e9,
        _ => 1.0,
    };
    (figure * unit) as u64
}

pub(super) fn parse_package_sizes(out: &str) -> Vec<(String, u64)> {
    let mut found = Vec::new();
    let mut name = None;
    for line in out.lines() {
        if let Some(p) = line.strip_prefix("Package:") {
            name = Some(p.trim().to_string());
        } else if let Some(n) = line.strip_prefix("Size:") {
            if let (Some(p), Ok(n)) = (name.take(), n.trim().parse::<u64>()) {
                found.push((p, n));
            }
        }
    }
    found
}
