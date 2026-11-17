//! The page before a Flatpak reference or bundle is installed: what the program will be able to reach, and Install. Nothing installs until Install is pressed.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::common::run;
use crate::store::dropped::{self, Reference};
use crate::store::flatpak_peek::{self, Line};
use crate::store::screen::{list_card, screen, title_row};
use crate::store::{go, Msg, Store};
use crate::toolkit::t;
use crate::toolkit::theme;
use crate::toolkit::widget::{button, label, main_action, note, row, spinner, Length, Node, Tone};
use super::installed::installed_opened;
use super::install::pressed_install;
use super::pictures::icon;

#[derive(Clone)]
enum Source {

    Ref(PathBuf),

    Bundle(PathBuf),

    Mine(PathBuf),

    Copy(PathBuf),
}

enum Reach {
    Reading,
    Known(Vec<Line>),
    Unread,
}

enum Step {
    Waiting,
    Installing,
    Failed(String),
}

pub(crate) struct Page {
    id: u32,
    source: Source,
    name: String,
    host: Option<String>,

    about: Option<Reference>,
    commit: Option<String>,
    reach: Reach,
    step: Step,

    key: Option<PathBuf>,

    warned: bool,

    away: bool,
}

enum Seen {
    Have,
    Ref(Option<(Vec<Line>, String)>),
    Bundle { copy: Option<Result<PathBuf, ()>>, found: Option<(String, Option<Vec<Line>>)>, have: bool },
}

fn next_id() -> u32 {
    static COUNT: AtomicU32 = AtomicU32::new(0);
    COUNT.fetch_add(1, Ordering::Relaxed)
}

pub(crate) fn offer_ref(s: &mut Store, file: PathBuf) {
    let Some((id, about)) = queue_ref(s, file) else { return };
    if s.installed.contains_key(&about.name) {
        return have(s, id, &about.name);
    }
    let name = about.name.clone();
    let heard = s.then(move |s: &mut Store, seen: Seen| match seen {
        Seen::Have => have(s, id, &name),
        Seen::Ref(Some((lines, commit))) => reached(s, id, None, Some(lines), Some(commit)),
        _ => reached(s, id, None, None, None),
    });
    run::on_thread(move || {
        if flatpak_peek::is_installed(&about.name) {
            return Seen::Have;
        }
        Seen::Ref(flatpak_peek::read_ref(&about).and_then(|seen| Some((flatpak_peek::lines_of(&seen.metadata)?, seen.commit))))
    }, heard);
}

fn have(s: &mut Store, id: u32, name: &str) {
    let Some(at) = s.flatpaks.iter().position(|p| p.id == id) else { return };
    let_go(&s.flatpaks.remove(at).source);
    match s.apps.get(name).cloned() {
        Some(a) => s.open_detail(a),
        None => {
            let waiting = std::mem::take(&mut s.flatpaks);
            s.select_nav("installed");
            s.flatpaks.extend(waiting);
        }
    }
    s.top = Some((t!("store-fp-have", name = name), Tone::Plain));
}

fn queue_ref(s: &mut Store, file: PathBuf) -> Option<(u32, Reference)> {
    let Some(about) = dropped::reference(&file) else {
        s.top = Some((t!("store-add-bad-ref").into(), Tone::Problem));
        return None;
    };
    let id = next_id();
    let key = Some(file.canonicalize().unwrap_or_else(|_| file.clone()));
    if s.flatpaks.iter().any(|p| p.key == key || p.about.as_ref().is_some_and(|a| a.name == about.name)) {
        return None;
    }
    let Some(copy) = flatpak_peek::private_ref(&about) else {
        s.top = Some((t!("store-fp-copy-failed").into(), Tone::Problem));
        return None;
    };
    s.flatpaks.push(Page { id, source: Source::Ref(copy), name: about.name.clone(), host: Some(about.host.clone()), about: Some(about.clone()), commit: None, reach: Reach::Reading, step: Step::Waiting, key, warned: false, away: false });
    Some((id, about))
}

pub(crate) fn offer_bundle(s: &mut Store, file: PathBuf, shown: String) {
    offer_bundle_as(s, Source::Bundle(file), shown);
}

pub(crate) fn offer_opened_bundle(s: &mut Store, file: PathBuf) {
    let shown = file.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    offer_bundle_as(s, Source::Mine(file), shown);
}

fn offer_bundle_as(s: &mut Store, source: Source, shown: String) {
    let (Source::Bundle(file) | Source::Mine(file)) = source.clone() else { return };
    let mine = matches!(source, Source::Mine(_));
    let Some(id) = queue_bundle(s, source, &shown) else { return };
    let heard = s.then(move |s: &mut Store, seen: Seen| heard_bundle(s, id, seen));
    run::on_thread(move || {
        let copy = mine.then(|| flatpak_peek::private_copy(&file).ok_or(()));
        let from = match &copy {
            Some(Ok(c)) => c.clone(),
            Some(Err(())) => return Seen::Bundle { copy, found: None, have: false },
            None => file,
        };
        let found = flatpak_peek::read_bundle(&from).map(|b| (b.id, flatpak_peek::lines_of(&b.metadata)));
        let have = found.as_ref().is_some_and(|(name, _)| flatpak_peek::is_installed(name));
        Seen::Bundle { copy, found, have }
    }, heard);
}

fn heard_bundle(s: &mut Store, id: u32, seen: Seen) {
    let Seen::Bundle { copy, found, have: there } = seen else { return };
    match (s.flatpaks.iter_mut().find(|p| p.id == id), copy) {
        (Some(p), Some(Ok(c))) => p.source = Source::Copy(c),
        (Some(p), Some(Err(()))) => {
            p.step = Step::Failed(t!("store-add-ref-failed", name = &p.name, why = t!("store-fp-copy-failed")));
            p.reach = Reach::Unread;
            return;
        }
        (None, Some(Ok(c))) => return let_go(&Source::Copy(c)),
        _ => {}
    }
    match found {
        Some((name, _)) if there => have(s, id, &name),
        Some((name, lines)) => reached(s, id, Some(name), lines, None),
        None => reached(s, id, None, None, None),
    }
}

fn queue_bundle(s: &mut Store, source: Source, shown: &str) -> Option<u32> {
    let (Source::Bundle(file) | Source::Mine(file) | Source::Copy(file)) = &source else { return None };
    let key = Some(file.canonicalize().unwrap_or_else(|_| file.clone()));
    if s.flatpaks.iter().any(|p| p.key == key) {
        return None;
    }
    let id = next_id();
    let name = shown.strip_suffix(".flatpak").unwrap_or(shown).chars().filter(|c| !c.is_control()).take(80).collect();
    s.flatpaks.push(Page { id, source, name, host: None, about: None, commit: None, reach: Reach::Reading, step: Step::Waiting, key, warned: false, away: false });
    Some(id)
}

fn reached(s: &mut Store, id: u32, name: Option<String>, lines: Option<Vec<Line>>, commit: Option<String>) {
    let twin = name.as_ref().is_some_and(|n| s.flatpaks.iter().any(|p| p.id != id && (p.name == *n || p.about.as_ref().is_some_and(|a| a.name == *n))));
    if twin {
        s.flatpaks.retain(|p| p.id != id || { let_go(&p.source); false });
        return;
    }
    let Some(p) = s.flatpaks.iter_mut().find(|p| p.id == id) else { return };
    p.commit = commit;
    if let Some(n) = name {
        p.name = n;
    }
    p.reach = lines.map_or(Reach::Unread, Reach::Known);
}

pub(crate) fn showing(s: &Store) -> bool {
    front(s).is_some()
}

fn front(s: &Store) -> Option<&Page> {
    if s.detail.is_some() || s.image.is_some() {
        return None;
    }
    s.flatpaks.iter().find(|p| !p.away)
}

pub(crate) fn cancel_first(s: &mut Store) {
    if let Some(id) = front(s).map(|p| p.id) {
        cancel(s, id);
    }
}

fn cancel(s: &mut Store, id: u32) {
    let Some(at) = s.flatpaks.iter().position(|p| p.id == id && !matches!(p.step, Step::Installing)) else { return };
    let gone = s.flatpaks.remove(at);
    let_go(&gone.source);
}

pub(crate) fn forget(s: &mut Store) {
    let (keep, gone): (Vec<Page>, Vec<Page>) = std::mem::take(&mut s.flatpaks).into_iter().partition(|p| matches!(p.step, Step::Installing));
    s.flatpaks = keep;
    s.flatpaks.iter_mut().for_each(|p| p.away = true);
    gone.iter().for_each(|p| let_go(&p.source));
}

fn let_go(source: &Source) {
    match source {
        Source::Bundle(file) => {
            let _ = std::fs::remove_file(file);
        }
        Source::Copy(file) | Source::Ref(file) => {
            let _ = std::fs::remove_file(file);
            let _ = file.parent().map(std::fs::remove_dir);
        }
        Source::Mine(_) => {}
    }
}

fn press(s: &mut Store, id: u32) {
    pressed_install(s, |s| install(s, id));
}

fn install(s: &mut Store, id: u32) {
    let Some(plan) = begin(s, id) else { return };
    let (name, source) = (plan.name, plan.source);
    let done = s.then(move |s: &mut Store, (ok, err): (bool, String)| {
        if installed(s, id, &name, &source, ok, &err) {
            installed_opened(s);
        }
    });
    run::on_thread(move || run_steps(&plan.steps, &plan.undo, STEP_SECONDS), done);
}

struct Plan {
    name: String,
    source: Source,
    steps: Vec<Vec<String>>,
    undo: Vec<String>,
}

const STEP_SECONDS: &str = "1800";

fn timed(argv: &[String], seconds: &str) -> Vec<String> {
    ["timeout", "-k", "3", seconds].iter().map(|w| w.to_string()).chain(argv.iter().cloned()).collect()
}

fn run_steps(steps: &[Vec<String>], undo: &[String], seconds: &str) -> (bool, String) {
    steps_with(steps, undo, &mut |argv| {
        let (ok, _, err) = run::captured(&timed(argv, seconds), None);
        (ok, err)
    })
}

fn steps_with(steps: &[Vec<String>], undo: &[String], run: &mut dyn FnMut(&[String]) -> (bool, String)) -> (bool, String) {
    for (i, argv) in steps.iter().enumerate() {
        let (ok, err) = run(argv);
        if ok {
            continue;
        }
        if i > 0 && !undo.is_empty() && !run(undo).0 {
            return (false, format!("{} {}", why_of(&err), t!("store-fp-undo-failed")));
        }
        return (false, err);
    }
    (true, String::new())
}

fn why_of(err: &str) -> String {
    let last = err.trim().lines().last().unwrap_or("");
    let clean: String = last.chars().filter(|c| !c.is_control() && !matches!(c, '\u{200b}'..='\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')).collect();
    let clean = clean.trim();
    if clean.chars().count() > WHY_MOST {
        format!("{}…", clean.chars().take(WHY_MOST).collect::<String>().trim_end())
    } else {
        clean.to_string()
    }
}

const WHY_MOST: usize = 200;

fn begin(s: &mut Store, id: u32) -> Option<Plan> {
    let p = s.flatpaks.iter_mut().find(|p| p.id == id && !matches!(p.step, Step::Installing) && !matches!(p.reach, Reach::Reading))?;
    if matches!(p.source, Source::Mine(_)) {
        return None;
    }
    if matches!(p.reach, Reach::Unread) && !p.warned {
        p.warned = true;
        return None;
    }
    p.step = Step::Installing;
    s.working += 1;
    let (mut steps, mut undo) = (Vec::new(), Vec::new());
    match &p.source {
        Source::Ref(file) => {
            steps.push(dropped::install_argv(file));
            if let (Some(about), Some(commit)) = (&p.about, &p.commit) {
                steps.push(flatpak_peek::pin_argv(about, commit));
                undo = crate::common::removal::flatpak_uninstall(Some("--user"), &flatpak_peek::target(about), false);
            }
        }
        Source::Bundle(file) | Source::Mine(file) | Source::Copy(file) => steps.push(flatpak_peek::bundle_install_argv(file)),
    }
    Some(Plan { name: p.name.clone(), source: p.source.clone(), steps, undo })
}

fn installed(s: &mut Store, id: u32, name: &str, source: &Source, ok: bool, err: &str) -> bool {
    s.job_over();
    let here = s.flatpaks.iter().position(|p| p.id == id && !p.away);
    if ok {
        let_go(source);
        s.flatpaks.retain(|p| p.id != id);
        s.top = Some((t!("store-add-installed-ref", name = name), Tone::Plain));
        return true;
    }
    let why = Some(why_of(err)).filter(|w| !w.is_empty()).unwrap_or_else(|| t!("store-add-ref-refused").to_string());
    match here {
        Some(at) => s.flatpaks[at].step = Step::Failed(t!("store-add-ref-failed", name = name, why = why)),
        None => {
            s.flatpaks.retain(|p| p.id != id);
            let_go(source);
            s.top = Some((t!("store-add-ref-failed", name = name, why = why), Tone::Problem));
        }
    }
    false
}

pub(crate) fn view(s: &Store) -> Option<Node<Msg>> {
    front(s).map(laid)
}

fn laid(p: &Page) -> Node<Msg> {
    let id = p.id;
    let busy = matches!(p.step, Step::Installing);
    let main = if busy {
        row(vec![spinner(), label(t!("store-app-busy-installing")).cut()]).gap(crate::store::look::busy_gap())
    } else {
        let install = main_action(crate::settings::ui::INSTALL_ICON, t!("store-fp-install")).width(Length::Fixed(crate::store::look::action_room()));
        row(vec![if matches!(p.reach, Reach::Reading) { install } else { install.on_press(go(move |s| press(s, id))) }])
    };
    let no = button("window-close-symbolic", t!("store-fp-cancel")).key("flatpak-cancel");
    let no = if busy { no } else { no.on_press(go(move |s| cancel(s, id))) };
    let words = match &p.host {
        Some(host) => t!("store-fp-headline-from", name = &p.name, host = host),
        None => t!("store-fp-headline", name = &p.name),
    };
    screen(title_row(p.name.clone(), vec![no, main]), words, vec![list_card("security-high-symbolic", t!("store-fp-card"), body(p))])
}

fn body(p: &Page) -> Vec<Node<Msg>> {
    let adds = p.about.as_ref().and_then(|a| a.runtime_repo.as_deref()).and_then(dropped::https_host);
    let adds = adds.map(|host| line_row(&Line { icon: "network-server-symbolic", words: t!("store-fp-adds-source", host = host) }));
    let reach = match &p.reach {
        Reach::Reading => vec![row(vec![spinner(), label(t!("store-fp-reading"))]).gap(8.0)],
        Reach::Unread if p.warned => vec![note(t!("store-fp-unread")).tone(Tone::Problem), note(t!("store-fp-unread-again")).tone(Tone::Problem)],
        Reach::Unread => vec![note(t!("store-fp-unread")).tone(Tone::Problem)],
        Reach::Known(lines) if lines.is_empty() => vec![note(t!("store-fp-none"))],
        Reach::Known(lines) => lines.iter().map(line_row).collect(),
    };
    let mut all: Vec<Node<Msg>> = adds.into_iter().chain(reach).collect();
    if let Step::Failed(why) = &p.step {
        all.push(note(why.clone()).tone(Tone::Problem));
    }
    all
}

fn line_row(l: &Line) -> Node<Msg> {
    row(vec![icon(l.icon.to_string(), theme::apps_list().row_icon), label(l.words.clone()).width(Length::Fill)]).gap(12.0).centred()
}

#[cfg(test)]
mod tests;
