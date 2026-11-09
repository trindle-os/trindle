//! trindle-ring: launcher, tray, tools and the open windows in one circle.
//! Stays resident; a second invocation hands its words to the running one.

pub mod battery_alert;
pub mod circles;
pub mod crashes;
pub mod ctx;
pub mod meter;
pub mod drives;
pub mod icons_run;
pub mod layout;
pub mod magnify;
pub mod kept;
pub mod label_window;
pub mod chosen;
pub mod mic;
pub mod lid;
pub mod updates;
pub mod using;
pub mod note_goes;
pub mod notes;
pub mod stack;
pub mod notice;
pub mod open_windows;
pub mod outside;
pub mod notify;
pub mod plugins;
pub mod quick;
pub mod sni;
pub mod store;
pub mod tray;
pub mod view;
pub mod windows;

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use ctx::{Ctx, Do};

use crate::common::{apps, run};
use crate::toolkit::t;
use crate::toolkit::bus::{Asked, Bus, Offer, Props, Served};
use crate::toolkit::event::{Flow, Loop};
use crate::toolkit::window::Display;

pub const BUS_NAME: &str = "org.trindle.Ring";
pub const BUS_PATH: &str = "/org/trindle/Ring";
pub(crate) const XML: &str = r#"<interface name="org.trindle.Ring"><method name="Run"><arg type="as" direction="in"/></method><method name="Tick"/><method name="CloseRing"/></interface>"#;

pub fn menu_button() -> f32 {
    crate::toolkit::theme::sheet().ring_button
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Job {

    Programs,

    Computer,

    Tools,

    Tray,
}

impl Job {

    pub fn share(self) -> f32 {
        let s = crate::toolkit::theme::sheet();
        match self {
            Job::Programs => s.ring_job_programs,
            Job::Computer => s.ring_job_computer,
            Job::Tools => s.ring_job_tools,
            Job::Tray => s.ring_job_tray,
        }
    }
}

pub fn tick_at_edge(bus: &Bus, lp: &Loop) {
    ask(bus, lp, "Tick");
}

pub fn give_way(bus: &Bus, lp: &Loop) {
    ask(bus, lp, "CloseRing");
}

fn ask(bus: &Bus, lp: &Loop, what: &str) {
    let hold = lp.hold();
    bus.call(BUS_NAME, BUS_PATH, BUS_NAME).never_start().method(
        what, &(), false, Some(Duration::from_secs(2)), lp, move |_| drop(hold));
}

const OPTIONS: &[&str] = &["say WORDS", "background", "show-desktop", "listen", "all", "power", "favourites", "restyle", "corners", "notes", "shortcuts"];

fn option_help(option: &str) -> &'static str {
    match option {
        "say WORDS" => t!("ring-cli-say"),
        "background" => t!("ring-cli-background"),
        "show-desktop" => t!("ring-cli-show-desktop"),
        "listen" => t!("ring-cli-listen"),
        "all" => t!("ring-cli-all"),
        "power" => t!("ring-cli-power"),
        "favourites" => t!("ring-cli-favourites"),
        "restyle" => t!("ring-cli-restyle"),
        "corners" => t!("ring-cli-corners"),
        "shortcuts" => t!("ring-cli-shortcuts"),
        _ => t!("ring-cli-notes"),
    }
}

#[derive(Default)]
struct Words {
    flags: Vec<String>,
    say: Option<String>,
}

impl Words {
    fn has(&self, flag: &str) -> bool {
        self.flags.iter().any(|f| f == flag)
    }
}

fn read(args: &[String]) -> Result<Words, String> {
    let mut asked = Words::default();
    let mut words = args.iter();
    while let Some(a) = words.next() {
        let Some(name) = a.strip_prefix("--") else { return Err(t!("ring-cli-unknown", option = a)) };
        if let Some(said) = name.strip_prefix("say=") {
            asked.say = Some(said.to_string());
        } else if name == "say" {
            asked.say = Some(words.next().ok_or_else(|| t!("ring-cli-missing-say").to_string())?.clone());
        } else if OPTIONS.contains(&name) {
            asked.flags.push(name.to_string());
        } else {
            return Err(t!("ring-cli-unknown", option = a));
        }
    }
    Ok(asked)
}

fn help(program: &str) {
    println!("{}\n  {}\n\n{}", t!("ring-cli-usage"), t!("ring-cli-command", program = program), t!("ring-cli-options"));
    let wide = OPTIONS.iter().map(|o| o.len()).max().unwrap_or(0) + 4;
    for o in OPTIONS {
        let h = option_help(o);
        let o = format!("--{o}");
        println!("  {o:wide$}{h}");
    }
}

pub fn main(args: Vec<String>) -> i32 {

    let marked = crate::common::renewed::not_traceable();
    let program = args.first().map(|a| a.rsplit('/').next().unwrap_or(a).to_string())
        .unwrap_or_else(|| "trindle-ring".into());
    let words: Vec<String> = args.iter().skip(1).cloned().collect();
    if !marked {
        eprintln!("{program}: this system would not mark the ring as closed to other programs");
    }
    if let Some(code) = check_words(&program, &words) {
        return code;
    }

    crate::toolkit::ui::drawn_by(crate::toolkit::ui::DrawnBy::Processor);

    kept::take();
    let lp = Loop::new();
    kept::hear_on(&lp);
    let Ok(bus) = Bus::session(&lp) else {
        let Some(_running) = start(&lp, None, &program, words) else { return 1 };
        lp.run();
        return 0;
    };
    run_with_bus(lp, bus, program, words)
}

fn check_words(program: &str, words: &[String]) -> Option<i32> {
    if words.iter().any(|w| w == "--help" || w == "-h") {
        help(program);
        return Some(0);
    }
    if let Err(e) = read(words) {
        eprintln!("{program}: {e}");
        return Some(1);
    }
    None
}

fn run_with_bus(lp: Loop, bus: Bus, program: String, words: Vec<String>) -> i32 {
    let code = Rc::new(std::cell::Cell::new(0));
    let kept = Rc::new(RefCell::new(None));
    let (ev, b, k, c) = (lp.clone(), bus.clone(), kept.clone(), code.clone());
    let first = RefCell::new(Some(words.clone()));

    let _lost = kept::when_lost(&bus, BUS_NAME, || {
        eprintln!("trindle-ring: the ring the desktop keeps has started; this one leaves");
        std::process::exit(0);
    });
    kept::hold_name(&bus, &lp, BUS_NAME, kept::kept(), move |ok| {
        if ok {
            match start(&ev, Some(b.clone()), &program, first.borrow_mut().take().unwrap_or_default()) {
                Some(running) => *k.borrow_mut() = Some(running),
                None => {
                    c.set(1);
                    ev.quit();
                }
            }
            return;
        }

        let quit = ev.clone();
        b.call(BUS_NAME, BUS_PATH, BUS_NAME).method("Run", &(words.clone(),), false,
                                                    Some(Duration::from_secs(2)), &ev, move |_| quit.quit());
    });
    lp.run();
    drop(kept);
    code.get()
}

struct Running {
    _ctx: Rc<Ctx>,
    _parts: Rc<Parts>,
    _served: Option<Served>,
}

struct Parts {
    ctx: Rc<Ctx>,
    ring: Rc<view::panel::Ring>,
    windows: Option<Rc<windows::Windows>>,
    notes: Rc<notes::Notes>,
    notice: Rc<notice::Notice>,
    drives: Option<Rc<drives::Drives>>,

    icons: RefCell<icons_run::Runs>,
}

fn start(lp: &Loop, session: Option<Bus>, program: &str, words: Vec<String>) -> Option<Running> {
    let display = connect(lp, program)?;
    let system = Bus::system(lp).ok();

    let slot: Rc<RefCell<Option<Rc<Parts>>>> = Rc::new(RefCell::new(None));
    let s = slot.clone();
    let tell = lp.post(move |d: Do| {
        let parts = s.borrow().clone();
        if let Some(p) = parts {
            p.answer(d);
        }
    });
    let ctx = Ctx::new(lp.clone(), display, session.clone(), system, tell);

    kept::pulse_while_keys_held(&ctx.lp, &ctx.display);

    start_services(&ctx);
    let notes = notes::start(&ctx);
    notes.bring_back();
    let notice = notice::start(&ctx);
    crashes::watch(&ctx);
    plugins::start(&ctx);
    {
        let n = notes.clone();
        notify::on_new(move |note| n.announce(note));
        let n = notes.clone();
        notify::on_gone(move |id| n.withdraw(id));
        let n = notes.clone();
        notify::on_pinned(move |id, pinned| n.pinned_elsewhere(id, pinned));
    }
    battery_alert::start(&ctx);

    outside::start(lp);

    let drives = drives::start(&ctx);
    let windows = windows::Windows::start(lp);
    let ring = view::panel::Ring::start(&ctx, windows.clone());
    keep_up_to_date(lp, &ctx, &ring);

    std::mem::forget(lp.hold());

    let parts = Rc::new(Parts { ctx: ctx.clone(), ring, windows, notes, notice, drives, icons: RefCell::default() });
    *slot.borrow_mut() = Some(parts.clone());
    let served = session.as_ref().and_then(|bus| serve_commands(bus, &ctx));
    parts.answer(Do::Command(words));
    Some(Running { _ctx: ctx, _parts: parts, _served: served })
}

fn connect(lp: &Loop, program: &str) -> Option<Rc<Display>> {
    let no_display = || eprintln!("{program}: no Wayland display to open; run it inside a Trindle session");
    if std::env::var_os("WAYLAND_DISPLAY").is_none() && std::env::var_os("WAYLAND_SOCKET").is_none() {
        no_display();
        return None;
    }
    let gone = lp.clone();
    let Some(display) = Display::connect(lp, move || gone.quit()) else {
        no_display();
        return None;
    };
    if !display.has_layers() {
        eprintln!("{program}: this compositor does not offer the layer-shell protocol; run it inside a Trindle session");
        return None;
    }
    Some(display)
}

fn start_services(ctx: &Rc<Ctx>) {

    chosen::start();
    sni::start(ctx);
    notify::start(ctx);

    if !matches!(crate::common::files::setting("ring_place").as_str(), "" | "none") {
        let _ = crate::common::files::set_setting("ring_place", "none");
    }

    crate::corner::start(ctx);
    crate::polkit::start(ctx);

    crate::thunder::ask::start(&ctx.lp, &ctx.display);
    lid::start(ctx);
}

fn keep_up_to_date(lp: &Loop, ctx: &Rc<Ctx>, ring: &Rc<view::panel::Ring>) {
    {
        let c = ctx.clone();
        apps::watch(lp, move || c.tell(Do::AppsChanged));
    }

    lp.every(Duration::from_secs(crate::common::memory::TRIM_SECS.into()), || {
        crate::common::memory::give_back();
        Flow::Continue
    });
    lp.after(Duration::from_secs(8), tell_about_virtual_machine);

    {
        let weak = Rc::downgrade(ring);

        let how = if kept::kept() { crate::common::renewed::Renew::Leave } else { crate::common::renewed::Renew::InPlaceAs(&["--background"]) };
        crate::common::renewed::watch(lp, "trindle-ring", move || weak.upgrade().is_none_or(|r| r.resting()).then_some(how));
    }
}

fn serve_commands(bus: &Bus, ctx: &Rc<Ctx>) -> Option<Served> {
    let c = ctx.clone();
    bus.serve(BUS_PATH, Offer {
        name: BUS_NAME.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| {
            match asked.member().as_str() {
                "Run" => {
                    let args = asked.message().body().deserialize::<(Vec<String>,)>().map(|a| a.0).unwrap_or_default();
                    c.tell(Do::Command(args));
                }

                "Tick" => c.tell(Do::Tick),

                "CloseRing" => c.tell(Do::CloseRing),
                _ => {}
            }
            asked.reply(&());
        }),
        props: Box::new(Props::new),
        set: None,
    }).ok()
}

impl Parts {

    fn make_icons(&self, id: Option<u64>) {
        let Some(id) = id else { return };
        let (back, late) = (self.ctx.clone(), self.ctx.clone());
        run::capture(&["trindle-icons", "--quiet"], None, move |_, _, _| back.tell(Do::IconsMade(id)));
        run::every(icons_run::WAIT, move || {
            late.tell(Do::IconsLate(id));
            false
        });
    }

    fn apps_afresh(&self) {
        self.ctx.reread_apps();
        self.ring.reload_icons();
        self.ring.redraw();
    }

    fn answer(&self, d: Do) {
        match d {
            Do::Command(words) => self.command(&words),
            Do::Tick => self.notice.show(t!("settings-common-done")),
            Do::Notice(words) => self.notice.show(&words),
            Do::CloseRing => self.ring.close(),
            Do::Redraw => self.ring.redraw(),
            Do::ShowNote(id) => self.ring.show_note(id),
            Do::Face(f) => self.ring.change(f),

            Do::AppsChanged => {
                let run = self.icons.borrow_mut().changed();
                self.make_icons(run);
            }
            Do::IconsLate(id) => {
                let (read, run) = self.icons.borrow_mut().late(id);
                if read {
                    self.apps_afresh();
                }
                self.make_icons(run);
            }
            Do::IconsMade(id) => {
                let run = self.icons.borrow_mut().back(id);
                self.apps_afresh();
                self.make_icons(run);
            }
            Do::OpenDrive(drive) => {
                if let Some(d) = &self.drives {
                    d.open(&drive);
                }
            }
        }
    }

    fn restyle(&self) {
        self.ring.restyle();
        self.notes.restyle();
        self.notice.restyle();
    }

    fn command(&self, words: &[String]) {
        let Ok(asked) = read(words) else { return };
        if !self.command_without_opening(&asked) {
            self.command_for_ring(&asked);
        }
    }

    fn command_without_opening(&self, asked: &Words) -> bool {
        if asked.has("favourites") {
            run::spawn(&["trindle-settings", "--panel", "ring"]);
            return true;
        }
        if asked.has("notes") {
            self.notes.take_keyboard();
            return true;
        }
        let r = &self.ring;
        if asked.has("shortcuts") {
            r.toggle_shortcuts();
            return true;
        }
        if asked.has("corners") {
            crate::corner::start(&self.ctx);
            return true;
        }
        if asked.has("background") {
            if crate::common::restart::restarted_just_now() {
                notify::say_here(t!("ring-restarted"), t!("ring-restarted-why"));
            }
            return true;
        }
        if asked.has("restyle") {
            self.restyle();
            return true;
        }
        false
    }

    fn command_for_ring(&self, asked: &Words) {
        let r = &self.ring;
        let open = || if !r.is_open() {
            r.open_fresh();
        };
        if asked.has("listen") {
            open();
            r.listen_now(crate::voice::MAX_SECONDS as u32);
            return;
        }
        if let Some(words) = &asked.say {
            open();
            r.heard_words(words);
            return;
        }
        if asked.has("power") {
            open();
            r.show_power_choice();
            return;
        }
        if asked.has("all") {
            open();
            r.show_all();
            return;
        }
        if asked.has("show-desktop") {
            if r.is_open() {
                r.close();
            }
            if let Some(w) = &self.windows {
                w.show_desktop();
            }
            return;
        }
        if r.is_open() {
            r.close();
        } else {
            r.open_fresh();
        }
    }
}

fn tell_about_virtual_machine() {
    let live = crate::common::boot::from_stick();
    let told = crate::common::files::trindle_config().join("virtual-machine-told");
    if live || told.exists() {
        return;
    }
    let Some(machine) = crate::common::graphics::virtual_machine() else { return };
    let (heading, body) = crate::common::graphics::vm_note(&machine);
    notify::say_as("Trindle", "dialog-information-symbolic", heading, &body);
    let _ = crate::common::files::write_text(&told, "");
}

#[cfg(test)]
pub(crate) mod shots {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    use crate::toolkit::widget::Node;

    pub fn save<M: Clone + 'static>(name: &str, node: Node<M>, (w, h): (u32, u32)) {
        let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-b".into()));
        std::fs::create_dir_all(&out).unwrap();
        let theme = Theme::new(false, "");
        let scale = 2.0;
        let mut pix = tiny_skia::Pixmap::new((w as f32 * scale) as u32, (h as f32 * scale) as u32).unwrap();
        pix.fill(theme.bg.to_skia());
        let mut ui = Ui::new(node, theme);
        ui.paint(&mut pix.as_mut(), scale);
        pix.save_png(out.join(format!("{name}.png"))).unwrap();
    }
}
