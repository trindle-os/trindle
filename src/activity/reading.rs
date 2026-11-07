//! What one look at the programs makes of them.

use std::collections::HashMap;
use std::rc::Rc;

use crate::common::apps::Entry;
use crate::toolkit::t;
use crate::toolkit::widget::{drawing, Node};

use super::{Activity, Info, Msg, orbit, procs, rings};
use super::headline::{Moment, sentence};
use super::procs::{Group, Kind, Member};

pub(super) const MOST_BUBBLES: usize = 14;

const OWN_MEMORY: f64 = 0.01;

const OWN_WORK: f64 = 0.5;

pub(super) const FLAT_OUT: f64 = 0.9;

pub(super) const MEMORY_FULL: f64 = 0.9;

pub(super) const RESTING_BELOW: f64 = 0.05;

pub(super) const SYSTEM: &str = "system";

const DESKTOP: &str = "trindle-desktop";

pub(super) fn heat(threads: f64, share: f64) -> f64 {
    ((threads / 2.0).min(1.0) * 0.75).max(share).clamp(0.0, 1.0)
}

pub(super) fn tidy(name: &str) -> String {
    let mut c = name.chars();
    match c.next() {
        Some(first) => crate::common::plain::line(&char::to_uppercase(first).chain(c).collect::<String>()),
        None => t!("activity-program-unnamed").into(),
    }
}

pub(super) fn percent(share: f64) -> String {
    let p = (share * 100.0).round();
    if p < 1.0 { t!("activity-under-one-percent").into() } else { format!("{p:.0}%") }
}

pub(super) fn threads_words(share: f64, threads: usize) -> String {
    let t = share * threads as f64;
    let n = t.round() as usize;
    if n >= threads {
        t!("activity-all-threads", n = threads)
    } else if t < 0.75 {
        t!("activity-less-than-one-thread", n = threads)
    } else {
        t!("activity-about-threads", n = n, total = threads)
    }
}

pub fn busy_words(busy: f64) -> String {
    if busy < 0.05 { t!("activity-resting").into() } else { t!("activity-busy-percent", percent = percent(busy)) }
}

pub(super) fn effort(threads: f64, share: f64) -> &'static str {
    if threads < RESTING_BELOW {
        t!("activity-effort-resting")
    } else if threads < OWN_WORK {
        t!("activity-effort-little")
    } else if share >= 0.75 {
        t!("activity-effort-flat-out")
    } else {
        t!("activity-effort-hard")
    }
}

pub(super) fn running_words(secs: u64) -> String {
    match secs {
        0..60 => t!("activity-under-a-minute").into(),
        60..3600 => t!("activity-minutes", n = secs / 60),
        3600..172_800 => t!("activity-hours", n = secs / 3600),
        _ => t!("activity-days", n = secs / 86_400),
    }
}

pub(super) fn title(g: &Group, apps: &[Entry]) -> String {
    match g.owner.kind {
        Kind::Linux => t!("activity-linux-itself").into(),
        Kind::Trindle => t!("activity-trindle-itself").into(),
        Kind::App | Kind::Other => g.owner.app.and_then(|i| apps.get(i)).map(|a| crate::common::plain::line(&a.name))
            .unwrap_or_else(|| tidy(&g.owner.name)),
    }
}

fn icon(g: &Group, apps: &[Entry]) -> Vec<String> {
    match (g.owner.kind, g.owner.app.and_then(|i| apps.get(i))) {
        (_, Some(a)) => a.icons(),
        (Kind::Linux, _) => vec!["computer".into()],
        (Kind::Trindle, _) => vec!["trindle".into()],
        _ => vec![crate::common::apps::generic_icon_for(&g.owner.name, &g.owner.name)],
    }
}

fn can_stop(g: &Group) -> bool {
    let here = std::process::id();
    g.owner.kind == Kind::App && g.members.iter().any(|m| m.mine) && !g.members.iter().any(|m| m.pid == here)
}

pub(super) fn those_left<'a>(groups: &'a [Group], own: &[&Group]) -> Vec<&'a Group> {
    groups.iter().filter(|g| !own.iter().any(|o| std::ptr::eq(*o, *g))).collect()
}

pub(super) fn own_bubble(g: &Group, memory: f64, threads: f64) -> bool {
    match g.owner.kind {
        Kind::App => true,
        Kind::Trindle => false,
        Kind::Linux => threads >= OWN_WORK,
        Kind::Other => threads >= OWN_WORK || (memory >= OWN_MEMORY && g.members.iter().any(|m| m.mine)),
    }
}

pub fn names(apps: &[Entry]) -> procs::Names {
    let list: Vec<procs::AppNames> = apps.iter().map(|a| procs::AppNames {
        key: a.key().to_string(),
        exec: procs::exec_program(&a.exec),
        try_exec: a.try_exec.clone(),
        wm_class: a.wm_class.clone(),
    }).collect();
    procs::Names::new(&list)
}

pub(super) fn icon_picture(names: Vec<String>, size: f32) -> Node<Msg> {
    drawing((size, size), "", move |canvas, pen| {
        let side = canvas.width().min(canvas.height());
        let whole = pen.scale.round().max(1.0) as u32;
        let px = (side as f32 / whole as f32).round() as u32;
        for n in &names {
            if pen.icons.draw(canvas, n, 0, 0, px, whole, pen.theme.fg).is_some() {
                break;
            }
        }
    })
}

struct Reading {
    all: u64,
    threads: usize,
    total_kb: u64,
}

struct Part<'a> {
    title: String,
    icon: Vec<String>,
    kb: u64,
    ticks: u64,
    members: &'a [Member],
    can_stop: bool,
}

struct Bubbles {
    blobs: Vec<orbit::Blob>,
    infos: HashMap<String, Info>,
    members: HashMap<String, Vec<Member>>,
}

impl Reading {
    fn share(&self, ticks: u64) -> f64 {
        if self.all > 0 { (ticks as f64 / self.all as f64).clamp(0.0, 1.0) } else { 0.0 }
    }

    fn of_memory(&self, kb: u64) -> f64 {
        kb as f64 / self.total_kb as f64
    }

    fn in_threads(&self, ticks: u64) -> f64 {
        self.share(ticks) * self.threads as f64
    }

    fn info(&self, clock: &procs::Clock, part: Part) -> Info {
        let Part { title, icon, kb, ticks, members, can_stop } = part;
        let (s, t) = (self.share(ticks), self.in_threads(ticks));
        let oldest = members.iter().map(|m| m.start).min().unwrap_or(0);
        Info {
            title, icon, heat: heat(t, s), effort: effort(t, s), working: t >= RESTING_BELOW,
            work: (percent(s), threads_words(s, self.threads)),
            holding: (rings::size(kb * 1024), t!("activity-share-of-memory", share = percent(self.of_memory(kb)))),
            running: running_words(clock.running(oldest)), parts: members.len(), can_stop,
        }
    }

    fn own_groups<'a>(&self, groups: &'a [Group]) -> Vec<&'a Group> {
        let mut own: Vec<&Group> = groups.iter()
            .filter(|g| own_bubble(g, self.of_memory(g.kb), self.in_threads(g.ticks))).collect();
        own.sort_by(|a, b| {
            let weight = |g: &Group| self.of_memory(g.kb) + 2.0 * self.share(g.ticks);
            weight(b).total_cmp(&weight(a))
        });
        own.truncate(MOST_BUBBLES);
        own
    }

    fn bubbles(&self, apps: &[Entry], clock: &procs::Clock, own: &[&Group], rest: &[&Group]) -> Bubbles {
        let mut blobs = Vec::new();
        let mut infos = HashMap::new();
        let mut members = HashMap::new();
        for g in own {
            let part = Part { title: title(g, apps), icon: icon(g, apps), kb: g.kb, ticks: g.ticks, members: &g.members, can_stop: can_stop(g) };
            let i = self.info(clock, part);
            blobs.push(orbit::Blob {
                key: g.owner.key.clone(), title: i.title.clone(), icon: i.icon.clone(), memory: self.of_memory(g.kb),
                figure: rings::size(g.kb * 1024), threads: self.in_threads(g.ticks), heat: i.heat, quiet: false,
            });
            infos.insert(g.owner.key.clone(), i);
            if can_stop(g) {
                members.insert(g.owner.key.clone(), g.members.clone());
            }
        }
        let (desk, other): (Vec<&Group>, Vec<&Group>) = rest.iter().partition(|g| g.owner.kind == Kind::Trindle);
        for (key, title, icon, part) in [
            (DESKTOP, t!("activity-trindle-desktop"), "trindle", desk),
            (SYSTEM, t!("activity-linux-and-other"), "computer", other),
        ] {
            if part.is_empty() {
                continue;
            }
            let (kb, ticks) = part.iter().fold((0, 0), |(k, t), g| (k + g.kb, t + g.ticks));
            let parts: Vec<Member> = part.iter().flat_map(|g| g.members.iter().cloned()).collect();
            let i = self.info(clock, Part { title: title.into(), icon: vec![icon.into()], kb, ticks, members: &parts, can_stop: false });
            blobs.push(orbit::Blob {
                key: key.into(), title: i.title.clone(), icon: i.icon.clone(), memory: self.of_memory(kb),
                figure: rings::size(kb * 1024), threads: self.in_threads(ticks), heat: i.heat, quiet: true,
            });
            infos.insert(key.into(), i);
        }
        Bubbles { blobs, infos, members }
    }
}

fn bubble_notes(blobs: &[orbit::Blob], infos: &HashMap<String, Info>) -> Vec<orbit::Note> {
    let busiest = blobs.iter().filter(|b| b.threads >= OWN_WORK).max_by(|a, b| a.threads.total_cmp(&b.threads));
    let mut by_memory: Vec<&orbit::Blob> = blobs.iter().collect();
    by_memory.sort_by(|a, b| b.memory.total_cmp(&a.memory));
    let holding = |key: &str| infos.get(key).map(|i| t!("activity-holding", amount = &i.holding.0, how = &i.holding.1)).unwrap_or_default();
    let mut notes = Vec::new();
    if let Some(b) = busiest {
        let hardest_most = by_memory.first().is_some_and(|g| g.key == b.key);
        let work = infos.get(&b.key).map(|i| if hardest_most {
            t!("activity-working-hardest-most", effort = i.effort, work = &i.work.1)
        } else {
            t!("activity-working-hardest", effort = i.effort, work = &i.work.1)
        }).unwrap_or_default();
        notes.push(orbit::Note { key: b.key.clone(), heading: t!("activity-note-working-hardest").into(), name: b.title.clone(), words: work });
    }
    let headings = [t!("activity-note-most-memory"), t!("activity-note-next-memory")];
    let ranked = headings.iter().zip(&by_memory).filter(|(_, g)| busiest.is_none_or(|b| b.key != g.key));
    for (heading, g) in ranked.take(2 - notes.len()) {
        notes.push(orbit::Note {
            key: g.key.clone(), heading: heading.to_string(), name: g.title.clone(), words: holding(&g.key),
        });
    }
    notes
}

impl Activity {
    pub(super) fn show(&mut self, look: procs::Look, (total_kb, available_kb): (u64, u64)) {
        let procs::Look { busy, all, threads, each, groups } = look;
        self.looked = true;
        let reading = Reading { all, threads, total_kb };
        let busy = reading.share(busy);
        let trindle_kb: u64 = groups.iter().filter(|g| g.owner.kind == Kind::Trindle).map(|g| g.kb).sum();
        self.rings.show(trindle_kb, busy, threads);
        let o = Rc::downgrade(&self.orbit);
        self.rings.ease(&self.events, move || {
            if let Some(o) = o.upgrade() {
                o.redraw();
            }
        });
        self.orbit.set_threads(&each);

        let used = total_kb.saturating_sub(available_kb) as f64 / total_kb as f64;
        let own = reading.own_groups(&groups);
        let rest = those_left(&groups, &own);
        let Bubbles { blobs, infos, members } = reading.bubbles(&self.apps, &procs::Clock::now(), &own, &rest);
        let free = available_kb.min(total_kb);
        self.orbit.set_notes(bubble_notes(&blobs, &infos), t!("activity-memory-free", amount = rings::size(free * 1024)));
        self.order = blobs.iter().map(|b| b.key.clone()).collect();
        self.infos = infos;
        self.members = members;
        self.orbit.set_bubbles(blobs);
        self.say_how_it_is(&reading, &groups, busy, used);
    }

    fn say_how_it_is(&mut self, reading: &Reading, groups: &[Group], busy: f64, used: f64) {
        let mut hardest: Vec<(String, f64)> = groups.iter().filter(|g| reading.share(g.ticks) >= 0.005)
            .map(|g| (title(g, &self.apps), reading.share(g.ticks))).collect();
        hardest.sort_by(|a, b| b.1.total_cmp(&a.1));
        hardest.truncate(2);
        let biggest = groups.iter().filter(|g| g.owner.kind != Kind::Trindle).max_by_key(|g| g.kb)
            .map(|g| title(g, &self.apps));
        let (mood, words) = sentence(&Moment { busy, memory: used, threads: reading.threads, busiest: &hardest, biggest: biggest.as_deref() });
        if self.headline.offer(mood, words.clone()) {
            self.was = std::mem::replace(&mut self.said, words);
            self.fade_in();
        }
    }
}
