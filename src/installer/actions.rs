//! What the installer does when a button is pressed.

use std::cell::Cell;
use std::io::{BufRead, BufReader, Read, Write};
use std::sync::mpsc::{channel, Receiver};
use std::process::{Child, Command, Stdio};
use std::rc::Rc;
use std::time::Duration;

use super::{ABOUT_YOU, DISK_PASSWORD, READY, this_machine, steps, FRAME, Installer, Machine, Msg, PAGE_COUNT, RUNNING, SLIDE, roll, tour};
use super::pages::{half_way, matching};
use super::plan::{self, Lock, Plan, Where};
use crate::common::run;
use crate::common::ui::DISARM_SECONDS;
use crate::toolkit::colour::Colour;
use crate::toolkit::event::{Flow, Loop, Post};
use crate::toolkit::t;
use zeroize::Zeroizing;

impl Installer {
    pub(super) fn new(name: String, accent: Colour, machine: Machine, events: Loop, asks: Post<Msg>, wake: Post<()>) -> Self {
        let leave = machine.disks.iter().flat_map(|d| d.parts.iter()).filter(|p| p.shrinkable())
            .map(|p| (p.dev.clone(), half_way(p))).collect();
        Installer {
            name,
            accent,
            plan: Plan::default().with_swap(),
            page: 0,
            machine,
            find: Default::default(),
            tried: String::new(),
            disk_one: Zeroizing::default(),
            disk_two: Zeroizing::default(),
            suggested: 0,
            full: String::new(),
            user: String::new(),
            host: String::new(),
            one: Zeroizing::default(),
            two: Zeroizing::default(),
            user_theirs: false,
            leave,
            doing: t!("installer-getting-ready").into(),
            share: 0.0,
            failed: false,
            problems: Vec::new(),
            helper: None,
            stopping: false,
            stuck: false,
            armed: false,
            arms: 0,
            terms: false,
            reading_terms: false,
            roll_at: Rc::new(Cell::new(0.0)),
            roll_to: Rc::new(Cell::new(0.0)),
            rolling: Rc::new(Cell::new(false)),
            slide: 0,
            touring: Rc::new(Cell::new(false)),
            cache: Rc::new(tour::Cache::default()),
            events,
            asks,
            wake,
        }
    }

    pub(super) fn go(&mut self, by: i32) {
        if self.failed {
            self.show(READY);
            return;
        }
        if self.page == ABOUT_YOU && by > 0 {
            self.send_answers();
            return;
        }
        let mut now = self.page as i32 + by;
        if now == DISK_PASSWORD as i32 && self.plan.lock == Lock::None {
            now += by.signum();
        }
        self.show(now.clamp(0, PAGE_COUNT as i32 - 1) as usize);
    }

    pub(super) fn show(&mut self, n: usize) {
        self.page = n;
        self.armed = false;
        self.failed = false;
        if n != RUNNING {
            self.touring.set(false);
        }
        self.roll();
    }

    pub(super) fn roll(&self) {
        let to = self.page as f32;
        self.roll_to.set(to);
        if !crate::common::graphics::moving() {
            self.roll_at.set(to);
            return;
        }
        if self.rolling.replace(true) {
            return;
        }
        let (at, to, rolling, wake) = (self.roll_at.clone(), self.roll_to.clone(), self.rolling.clone(), self.wake.clone());
        self.events.every(FRAME, move || {
            let flow = match roll::step(at.get(), to.get()) {
                Some(next) => {
                    at.set(next);
                    Flow::Continue
                }
                None => {
                    at.set(to.get());
                    rolling.set(false);
                    Flow::Stop
                }
            };
            wake.send(());
            flow
        });
    }

    pub(super) fn missing(&self) -> Option<&'static str> {
        let p = &self.plan;
        match self.page {
            0 => {
                if self.machine.short_of_memory {
                    Some(t!("installer-memory-short"))
                } else if p.locale.is_empty() {
                    Some(t!("installer-missing-language"))
                } else if p.layout.is_empty() {
                    Some(t!("installer-missing-keyboard"))
                } else if p.timezone.is_empty() {
                    Some(t!("installer-missing-time"))
                } else {
                    None
                }
            }
            1 => p.target.is_none().then_some(t!("installer-missing-disk")),
            2 => (p.lock != Lock::None && !self.machine.uefi).then_some(t!("installer-lock-needs-uefi")),
            DISK_PASSWORD => (p.lock != Lock::None && p.disk_password.is_empty())
                .then_some(t!("installer-missing-disk-password")),
            ABOUT_YOU => {
                if plan::username_fault(&p.username).is_some() {
                    Some(t!("installer-missing-username"))
                } else if plan::hostname_fault(&p.hostname).is_some() {
                    Some(t!("installer-missing-hostname"))
                } else if p.password.is_empty() {
                    Some(t!("installer-missing-password"))
                } else {
                    None
                }
            }
            READY => (!self.terms).then_some(t!("terms-needed")),
            _ => None,
        }
    }

    pub(super) fn erases(&self) -> bool {
        matches!(self.plan.target, Some(Where::Erase { .. }))
    }

    pub(super) fn arm_words(&self) -> &'static str {
        if self.page > READY {
            t!("installer-halt-sure")
        } else if self.erases() { t!("installer-arm-erase") } else { t!("installer-arm") }
    }

    fn choose_language(&mut self, locale: &str) {
        self.plan.locale = locale.to_string();
        let (layout, variant) = plan::layout_for(locale);
        let key = format!("{layout}\t{variant}");
        if self.machine.keyboards.iter().any(|(k, _)| *k == key) {
            self.choose_keyboard(&key);
        }
        let zones: Vec<String> = self.machine.zones.iter().map(|(z, _)| z.clone()).collect();
        let zone = plan::timezone_for(locale, &zones);
        if zones.contains(&zone) {
            self.plan.timezone = zone;
        }
    }

    fn choose_keyboard(&mut self, key: &str) {
        let (layout, variant) = key.split_once('\t').unwrap_or((key, ""));
        self.plan.layout = layout.to_string();
        self.plan.variant = variant.to_string();
        use_keyboard(layout, variant);
    }

    pub(super) fn pick(&mut self, field: usize, at: usize) {
        let items = match field {
            0 => &self.machine.locales,
            1 => &self.machine.keyboards,
            _ => &self.machine.zones,
        };
        let Some(key) = matching(items, &self.find[field]).get(at).map(|(k, _)| k.clone()) else { return };
        match field {
            0 => self.choose_language(&key),
            1 => self.choose_keyboard(&key),
            _ => self.plan.timezone = key,
        }
    }

    pub(super) fn beside(&mut self, part: String) {
        let leave = self.leave.get(&part).copied().unwrap_or(0);
        self.plan.target = Some(Where::Beside { part, leave });
    }

    pub(super) fn disk_typed(&mut self) {
        let fine = !self.disk_one.is_empty() && self.disk_one == self.disk_two;
        self.plan.disk_password = if fine { self.disk_one.to_string() } else { String::new() };
    }

    pub(super) fn you_typed(&mut self) {
        self.plan.fullname = self.full.clone();
        self.plan.username = self.user.clone();
        self.plan.hostname = self.host.clone();
        self.plan.password = if self.one == self.two { self.one.to_string() } else { String::new() };
    }

    pub(super) fn install(&mut self) {
        if !self.terms {
            return;
        }
        if !self.arm() {
            self.start();
        }
    }

    fn arm(&mut self) -> bool {
        if self.armed {
            return false;
        }
        self.armed = true;
        self.arms += 1;
        let (n, asks) = (self.arms, self.asks.clone());
        self.events.after(Duration::from_secs(DISARM_SECONDS.into()), move || {
            asks.send(Msg::Disarm(n));
        });
        true
    }

    pub(super) fn stop(&mut self) {
        if !self.arm() {
            self.armed = false;
            self.stopping = true;
            self.doing = t!("installer-halting").into();
            self.helper = None;
        }
    }

    pub(super) fn stopped(&mut self, free: bool) {
        self.stopping = false;
        self.helper = None;
        let mut kept = Plan::default().with_swap();
        kept.locale = std::mem::take(&mut self.plan.locale);
        kept.layout = std::mem::take(&mut self.plan.layout);
        kept.variant = std::mem::take(&mut self.plan.variant);
        kept.timezone = std::mem::take(&mut self.plan.timezone);
        self.plan = kept;
        self.terms = false;
        self.reading_terms = false;
        self.disk_one = Zeroizing::new(String::new());
        self.disk_two = Zeroizing::new(String::new());
        self.one = Zeroizing::new(String::new());
        self.two = Zeroizing::new(String::new());
        (self.full, self.user, self.host) = Default::default();
        self.user_theirs = false;
        self.leave.clear();
        self.problems.clear();
        self.share = 0.0;
        if free {
            self.machine = this_machine();
            self.show(1);
        } else {
            self.stuck = true;
            self.fail(t!("installer-halt-stuck"));
        }
    }

    fn send_answers(&mut self) {
        if let Some(helper) = &self.helper {
            let _ = helper.send(Zeroizing::new(serde_json::to_string(&self.plan).unwrap_or_default()));
        }
        self.show(RUNNING);
        self.tour();
    }

    pub(super) fn start(&mut self) {
        self.show(ABOUT_YOU);
        self.doing = t!("installer-getting-ready").into();
        self.share = 0.0;
        let json = serde_json::to_string(&self.plan).unwrap_or_default();
        let helper: &[&str] = if run::have("sudo") {
            &["sudo", "-n", "trindle-installer", "--run"]
        } else {
            &["pkexec", "trindle-installer", "--run"]
        };
        let child = Command::new(helper[0]).args(&helper[1..])
            .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn();
        match child {
            Ok(child) => {
                let asks = self.asks.clone();
                let (helper, lines) = channel();
                let _ = helper.send(Zeroizing::new(json));
                self.helper = Some(helper);
                std::thread::spawn(move || follow(child, lines, asks));
            }
            Err(e) => self.fail(&t!("installer-no-start", why = e.to_string())),
        }
    }

    pub(super) fn fail(&mut self, why: &str) {
        self.helper = None;
        self.armed = false;
        self.doing = why.to_string();
        self.failed = true;
        self.share = 1.0;
    }

    pub(super) fn tour(&mut self) {
        self.touring.set(false);
        let touring = Rc::new(Cell::new(true));
        self.touring = touring.clone();
        let asks = self.asks.clone();
        self.events.every(SLIDE, move || {
            if !touring.get() {
                return Flow::Stop;
            }
            asks.send(Msg::NextSlide);
            Flow::Continue
        });
    }

    pub(super) fn suggest(&mut self) {
        let words = plan::suggest_passphrase();
        self.disk_one = Zeroizing::new(words.clone());
        self.disk_two = Zeroizing::new(words);
        self.suggested += 1;
        self.disk_typed();
    }

    pub(super) fn full_name(&mut self, name: String) {
        if !self.user_theirs {
            self.user = plan::username_from(&name);
        }
        self.full = name;
        self.you_typed();
    }

    pub(super) fn sign_in_name(&mut self, name: String) {
        self.user = name;
        self.user_theirs = true;
        self.you_typed();
    }

    pub(super) fn progress(&mut self, line: &str) {
        if let Some(why) = line.strip_prefix("! ") {
            self.problems.push(why.trim().to_string());
            return;
        }
        if let Some((percent, what)) = line.trim().split_once(' ') {
            if let Ok(n) = percent.parse::<f32>() {
                if !self.failed {
                    self.share = crate::common::gentle::share(n.clamp(0.0, 100.0) as u64, 100, None, false);
                    self.doing = what.to_string();
                }
            }
        }
    }
}

pub(super) fn follow(mut child: Child, lines: Receiver<Zeroizing<String>>, asks: Post<Msg>) {
    if let Some(mut stdin) = child.stdin.take() {
        std::thread::spawn(move || {
            for line in lines {
                if stdin.write_all(line.as_bytes()).and_then(|_| stdin.write_all(b"\n")).and_then(|_| stdin.flush()).is_err() {
                    return;
                }
            }
        });
    }
    let errors = child.stderr.take().map(|mut e| std::thread::spawn(move || {
        let mut said = String::new();
        let _ = e.read_to_string(&mut said);
        said
    }));
    if let Some(out) = child.stdout.take() {
        for line in BufReader::new(out).lines().map_while(Result::ok) {
            asks.send(Msg::Line(line));
        }
    }
    let said = errors.and_then(|h| h.join().ok()).unwrap_or_default();
    let result = match child.wait() {
        Ok(status) if status.code() == Some(steps::STOPPED) => {
            asks.send(Msg::Stopped(true));
            return;
        }
        Ok(status) if status.code() == Some(steps::STUCK) => {
            asks.send(Msg::Stopped(false));
            return;
        }
        Ok(status) if status.success() => Ok(()),
        Ok(status) if said.trim().is_empty() => Err(status.to_string()),
        Ok(_) => Err(said),
        Err(e) => Err(e.to_string()),
    };
    asks.send(Msg::Finished(result));
}

pub(super) fn plain(err: &str) -> String {
    let e = err.trim();
    if e.contains("exit status: 1") || e.is_empty() {
        t!("installer-stopped").into()
    } else {
        e.lines().last().unwrap_or(e).to_string()
    }
}

fn use_keyboard(layout: &str, variant: &str) {
    let file = crate::common::files::user_environment();
    let changes: [(&str, Option<&str>); 2] = [
        ("XKB_DEFAULT_LAYOUT", Some(layout)),
        ("XKB_DEFAULT_VARIANT", (!variant.is_empty()).then_some(variant)),
    ];
    let _ = crate::common::files::update_env(&file, &changes);
}
