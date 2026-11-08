//! Permissions: what each Flathub app, each program installed in its own
//! folder and each plugin may use, as round icons that turn it on and off;
//! and the Thunderbolt and USB4 devices allowed to connect without asking.

use std::path::{Path, PathBuf};

use crate::common::apps::{self, Entry};
use crate::common::keyfile::KeyFile;
use crate::common::{access, files, run, x11_rooms};
use crate::plugins;
use crate::settings::ui::{app_icon, confirm, dropdown, group, headline, in_use_key, line, logo, on, page, Cause, Kit, Msg, Page};

const PICTURE: f32 = 24.0;

fn not_changed() -> &'static str {
    t!("settings-permissions-not-changed")
}

fn restart_plugin(kit: &mut Kit, id: &str) {
    let started = run::spawn(&["trindle-plugin", "restart", id]);
    kit.started(&said_key(Kind::Plugin, id), started,
                t!("settings-permissions-plugin-not-restarted"));
}
use crate::store::{boxed, loose};
use crate::thunder::{self, sysfs::Level, Listing};
use crate::toolkit::t;
use crate::toolkit::widget::{button, note, row, spinner, switch, Length, Node, Tone};

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Flathub,
    Deb,
    Plugin,

    Appimage,

    Program,
}

enum Picture {
    App(Vec<String>),
    File(PathBuf),
    Nothing,
}

struct Owner {

    id: String,
    name: String,
    sub: String,
    picture: Picture,

    known: Vec<(&'static str, &'static str)>,
    on: Vec<String>,
}

const CODES_KEY: &str = "permissions:codes-unread";

const MOST_CODES_SHOWN: usize = 200;

const MOST_DRAWN: usize = 40;

#[derive(Debug, PartialEq)]
struct Codes {
    id: String,
    name: String,

    list: Vec<(String, String)>,
}

fn drawn(text: &str) -> String {
    use crate::common::expand::{hidden, hides_in_words};
    let mut clean = String::new();
    for c in text.chars() {
        let joiner = hidden(c) && !hides_in_words(c);
        if !hides_in_words(c) && (!joiner || clean.chars().last().is_some_and(|p| !p.is_ascii() && !hidden(p))) {
            clean.push(c);
        }
    }
    if clean.chars().count() > MOST_DRAWN {
        format!("{}…", clean.chars().take(MOST_DRAWN).collect::<String>())
    } else {
        clean
    }
}

fn read_codes(folders: &Path, name_of: &dyn Fn(&str) -> Option<String>) -> std::io::Result<Vec<Codes>> {
    let read = match std::fs::read_dir(folders) {
        Ok(read) => read,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e),
    };
    let mut ids: Vec<String> = read.flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .filter(|id| crate::common::summon::good(id) && crate::common::summon::record_says(folders, id, &[crate::common::expand::WORD]))
        .collect();
    ids.sort();
    Ok(ids.into_iter().filter_map(|id| {
        let list: Vec<(String, String)> = crate::common::expand::read(&folders.join(&id)).into_iter().map(|e| (e.trigger, e.words)).collect();
        let name = name_of(&id).map(|n| drawn(&n)).filter(|n| !n.trim().is_empty()).unwrap_or_else(|| id.clone());
        (!list.is_empty()).then_some(Codes { id, name, list })
    }).collect())
}

type Found = Vec<(String, Vec<&'static str>)>;

fn ask_flatpak() -> Found {
    access::flatpak_apps().into_iter().map(|id| { let on = access::flatpak_allowed(&id); (id, on) }).collect()
}

fn apps_named() -> Vec<String> {
    access::flatpak_apps_named(&access::flatpak_places())
}

fn owners_of(found: Found) -> Vec<Owner> {
    let all = apps::entries();
    let known: Vec<(&'static str, &'static str)> = access::FLATPAK.iter().map(|p| (p.key, p.words)).collect();
    let mut out: Vec<Owner> = found.into_iter().map(|(id, on)| {
        let (name, icons) = flathub_app(&all, &id);
        Owner { id, name, sub: String::new(), picture: Picture::App(icons), known: known.clone(),
                on: on.iter().map(|s| s.to_string()).collect() }
    }).collect();
    out.sort_by_key(|o| o.name.to_lowercase());
    out
}

#[derive(Default)]
pub struct Permissions {

    tab: Option<usize>,

    flathub: Option<Vec<Owner>>,

    fresh: bool,

    answered: bool,

    turn: u64,

    #[cfg(test)]
    asker: Option<fn() -> Found>,
    #[cfg(test)]
    namer: Option<fn() -> Vec<String>>,
    debs: Vec<Owner>,
    plugins: Vec<Owner>,
    appimages: Vec<Owner>,
    programs: Vec<Owner>,

    #[cfg(test)]
    loose_top: Option<PathBuf>,

    thunder: Option<(Listing, Option<String>)>,

    older: bool,

    pub(super) outside: super::outside_card::Outside,

    codes: Vec<Codes>,

    codes_unread: bool,

    #[cfg(test)]
    plugins_top: Option<PathBuf>,
}

fn loose_images() -> &'static str {
    t!("settings-permissions-loose-images")
}
fn loose_programs() -> &'static str {
    t!("settings-permissions-loose-programs")
}

fn loose_words(top: &str) -> String {
    let mut out = top.to_string();
    for key in boxed::ALLOWED {
        let a = access::ask(key);
        out.push(' ');
        out.push_str(&t!("settings-permissions-title-and-words", title = access::title(key), words = access::off_line(a.without)));
    }
    out
}

fn added_images(top: &Path) -> Vec<String> {
    crate::icons::appimages::found().iter().filter_map(|file| {
        let stem = crate::common::apps::appimage_stem(file.file_name()?.to_str()?).to_string();
        let id = loose::id_for(&stem);
        let at = top.join(format!("appimage-{id}"));
        loose::prepare(&at, loose::APPIMAGE_STARTS).ok()?;
        loose::remember_name(&at, &stem);
        Some(id)
    }).collect()
}

pub fn older_why() -> &'static str {
    t!("settings-permissions-older-why")
}
const OLDER_KEY: &str = "permissions:older-programs";

fn lets_in(level: Level) -> &'static str {
    match level {
        Level::DpOnly => t!("settings-permissions-lets-in-screens"),
        Level::UsbOnly | Level::NoPcie => t!("settings-permissions-lets-in-usb"),
        _ => t!("settings-permissions-lets-in-all"),
    }
}

fn lets_in_short(level: Level) -> &'static str {
    match level {
        Level::DpOnly => t!("settings-permissions-short-screens"),
        Level::UsbOnly | Level::NoPcie => t!("settings-permissions-short-usb"),
        _ => t!("settings-permissions-short-firmware"),
    }
}

const THUNDER: &str = "permissions:thunder";

fn said_key(kind: Kind, id: &str) -> String {
    let kind = match kind {
        Kind::Flathub => "flathub",
        Kind::Deb => "deb",
        Kind::Plugin => "plugin",
        Kind::Appimage => "appimage",
        Kind::Program => "program",
    };
    format!("permissions:{kind}:{id}")
}

fn launcher(path: &Path) -> Option<(String, Vec<String>)> {
    let file = KeyFile::load(path).ok()?;
    let name = file.string("Desktop Entry", "Name").unwrap_or_default();
    Some((name, file.string("Desktop Entry", "Icon").into_iter().collect()))
}

fn flathub_app(all: &[Entry], id: &str) -> (String, Vec<String>) {
    let file = format!("{id}.desktop");
    if let Some(e) = all.iter().find(|e| e.id == file) {
        return (e.name.clone(), e.icons());
    }
    apps::app_dirs().iter().map(|d| d.join(&file)).find(|p| p.is_file())
        .and_then(|p| launcher(&p)).filter(|(name, _)| !name.is_empty())
        .unwrap_or_else(|| (id.to_string(), Vec::new()))
}

impl Permissions {

    fn flatpak_asker(&self) -> fn() -> Found {
        #[cfg(test)]
        if let Some(f) = self.asker {
            return f;
        }
        ask_flatpak
    }

    fn apps_namer(&self) -> fn() -> Vec<String> {
        #[cfg(test)]
        if let Some(f) = self.namer {
            return f;
        }
        apps_named
    }

    fn loose_folder(&self) -> PathBuf {
        #[cfg(test)]
        if let Some(top) = &self.loose_top {
            return top.clone();
        }
        loose::root()
    }

    fn plugins_folder(&self) -> PathBuf {
        #[cfg(test)]
        if let Some(top) = &self.plugins_top {
            return top.clone();
        }
        plugins::folders()
    }

    fn read(&mut self, kit: &mut Kit) {
        self.older = x11_rooms::allowed(&files::setting(x11_rooms::SETTING));
        self.outside.read();
        self.read_thunder(kit);
        if !self.fresh {
            self.ask(kit);
        }
        self.read_own();
        if self.codes_unread {
            kit.say(CODES_KEY, t!("settings-permissions-codes-unread"), false, true);
        } else {
            kit.hush(CODES_KEY);
        }
    }

    fn ask(&mut self, kit: &mut Kit) {
        self.turn += 1;
        if self.flathub.is_none() {
            let named = self.apps_namer()();
            self.flathub = Some(owners_of(named.into_iter().map(|id| (id, Vec::new())).collect()));
        }
        let turn = self.turn;
        let back = kit.then::<Permissions, Found>(move |me, _, found| me.heard(turn, found));
        run::on_thread(self.flatpak_asker(), back);
    }

    fn heard(&mut self, turn: u64, found: Found) {
        if turn != self.turn {
            return;
        }
        (self.fresh, self.answered) = (true, true);
        self.flathub = Some(owners_of(found));
    }

    fn read_own(&mut self) {
        let all = apps::entries();
        self.debs = boxed::installed().into_iter().map(|id| {
            let name = all.iter().find(|e| e.id.trim_end_matches(".desktop").eq_ignore_ascii_case(&id)).map_or_else(|| id.clone(), |e| e.name.clone());
            let icons = boxed::launchers::launchers_of(&id).first().and_then(|p| launcher(p)).map(|(_, i)| i).unwrap_or_default();
            Owner { sub: boxed::version(&id).map(|v| t!("settings-permissions-version", v = v)).unwrap_or_default(),
                    picture: Picture::App(icons), known: boxed::ALLOWED.iter().map(|k| (*k, *k)).collect(), on: boxed::allowed(&id),
                    id, name }
        }).collect();
        self.debs.sort_by_key(|o| o.name.to_lowercase());
        let top = self.loose_folder();

        let real = !cfg!(test) && top == loose::root();
        let added = if real { added_images(&top) } else { Vec::new() };
        let (mut images, mut programs) = (Vec::new(), Vec::new());
        for b in loose::listed_in(&top) {
            let known = boxed::ALLOWED.iter().map(|k| (*k, *k)).collect();
            let apart = real && b.kind == "appimage" && !added.contains(&b.id);
            let sub = if apart { t!("settings-permissions-not-added").to_string() } else { String::new() };
            let o = Owner { sub, picture: Picture::Nothing, known, on: b.allowed, id: b.id, name: b.name };
            if b.kind == "appimage" { images.push(o) } else { programs.push(o) }
        }
        (self.appimages, self.programs) = (images, programs);
        let top = self.plugins_folder();
        match read_codes(&top, &|id| plugins::find(id).map(|p| p.name)) {
            Ok(codes) => (self.codes, self.codes_unread) = (codes, false),
            Err(_) => self.codes_unread = true,
        }
        self.plugins = plugins::installed().into_iter().map(|p| {

            let known: Vec<(&'static str, &'static str)> = plugins::PERMISSIONS.iter().map(|k| (*k, *k))
                .filter(|(k, _)| p.asks.iter().any(|a| a == k)).collect();
            let mut sub = if known.is_empty() { t!("settings-permissions-version-asks-nothing", v = &p.version) } else { t!("settings-permissions-version", v = &p.version) };

            let mut notes: Vec<&str> = Vec::new();
            for note in known.iter().filter_map(|(k, _)| access::row_note(k)) {
                if !notes.contains(&note) {
                    notes.push(note);
                }
            }
            if !notes.is_empty() {
                sub = format!("{sub}; {}.", notes.join("; "));
            }

            if !p.listen.is_empty() {
                sub = t!("settings-permissions-sub-listen", sub = &sub, detail = access::listen_detail(&p.listen));
            }
            let picture = match &p.ring {
                Some((icon, _, _)) => Picture::File(icon.clone()),
                None => Picture::Nothing,
            };
            Owner { on: plugins::allowed(&p), id: p.id, name: p.name, sub, picture, known }
        }).collect();
    }

    fn read_thunder(&mut self, kit: &mut Kit) {
        let back = kit.then::<Permissions, (Listing, Option<String>)>(|me, _, found| me.thunder = Some(found));
        run::on_thread(thunder::listing, back);
    }

    fn forget_thunder(&mut self, kit: &mut Kit, id: String) {
        let back = kit.then::<Permissions, Option<()>>(move |me, k, err| {
            if err.is_some() {
                k.say(THUNDER, t!("settings-permissions-not-forgotten"), false, false);
            }
            me.read_thunder(k);
        });
        run::on_thread(move || thunder::forget(&id).err().map(|_| ()), back);
    }

    fn thunder_card(&self, kit: &Kit) -> Node<Msg> {
        let card = |about: &str, rows: Vec<Node<Msg>>| group(t!("settings-permissions-thunder")).icon("thunderbolt-symbolic")
            .about(t!("settings-permissions-thunder-about", about = about))
            .rows(rows)
            .node(kit);
        let Some((listing, problem)) = &self.thunder else {
            return card("", vec![line(t!("settings-permissions-ports")).sub(t!("settings-permissions-reading-ports")).node(kit),
                                 line(t!("settings-permissions-devices-allowed")).sub(t!("settings-permissions-reading")).node(kit),
                                 line(t!("settings-permissions-memory-apart")).sub(t!("settings-permissions-reading")).node(kit)]);
        };
        if listing.domains.is_empty() {
            return card("", vec![line(t!("settings-permissions-no-ports")).sub(t!("settings-permissions-nothing-to-ask")).node(kit),
                                 line(t!("settings-permissions-devices-allowed")).sub(t!("settings-permissions-none-plug")).node(kit)]);
        }
        if listing.domains.iter().all(|d| !d.level.asks()) {
            let level = listing.domains[0].level;
            return card(lets_in(level), vec![line(t!("settings-permissions-nothing-ask")).sub(lets_in_short(level)).node(kit),
                                             line(t!("settings-permissions-devices-allowed")).sub(t!("settings-permissions-every-one")).node(kit)]);
        }
        let iommu = listing.domains.iter().all(|d| d.iommu);
        let about = if iommu {
            t!("settings-permissions-iommu")
        } else {
            t!("settings-permissions-no-iommu")
        };
        let mut rows = vec![line(t!("settings-permissions-asked-before"))
            .sub(t!("settings-permissions-forget-one"))
            .key(THUNDER).node(kit)];
        if let Some(p) = problem {
            rows.push(note(t!("settings-permissions-devices-not-read", why = p)).tone(Tone::Problem));
        } else if listing.devices.is_empty() {
            rows.push(line(t!("settings-permissions-no-device")).node(kit));
        }
        for (id, vendor, name) in &listing.devices {
            let key = format!("{THUNDER}:{id}");
            let who = id.clone();
            let forget = confirm::<Permissions>(kit, &key, "user-trash-symbolic", t!("settings-permissions-forget-device"),
                                                move |me, k| me.forget_thunder(k, who.clone()));
            let name = if name.is_empty() { t!("settings-permissions-a-device").to_string() } else { name.clone() };
            let sub = if vendor.is_empty() { t!("settings-permissions-may-always").to_string() } else { t!("settings-permissions-made-by", vendor = vendor) };
            rows.push(row(vec![crate::settings::ui::icon("thunderbolt-symbolic", 24.0),
                               line(name).sub(sub).key(key).control(forget).node(kit).width(Length::Fill)])
                .gap(12.0).width(Length::Fill));
        }
        card(about, rows)
    }

    fn owners(&mut self, kind: Kind) -> Option<&mut Vec<Owner>> {
        match kind {
            Kind::Flathub => self.flathub.as_mut(),
            Kind::Deb => Some(&mut self.debs),
            Kind::Plugin => Some(&mut self.plugins),
            Kind::Appimage => Some(&mut self.appimages),
            Kind::Program => Some(&mut self.programs),
        }
    }

    fn set(&mut self, kind: Kind, id: &str, key: &str, allow: bool) {
        let Some(o) = self.owners(kind).and_then(|all| all.iter_mut().find(|o| o.id == id)) else { return };
        o.on.retain(|k| k != key);
        if allow {
            o.on.push(key.to_string());
        }
    }

    fn toggle(&mut self, kit: &mut Kit, kind: Kind, id: String, key: &'static str) {
        if !crate::toolkit::window::persons_own(crate::common::person_only::SETTINGS_PERMISSIONS) {
            return;
        }
        let Some(allow) = self.owners(kind).and_then(|all| all.iter().find(|o| o.id == id))
            .map(|o| !o.on.iter().any(|k| k == key)) else { return };
        self.set(kind, &id, key, allow);
        match kind {
            Kind::Flathub => {

                self.turn += 1;
                self.fresh = false;
                let back = kit.then::<Permissions, Option<String>>({
                    let id = id.clone();
                    move |me, k, err| me.settled(k, kind, &id, key, allow, err)
                });
                run::on_thread(move || access::flatpak_set(&id, key, allow).err(), back);
            }
            Kind::Deb => {
                let err = boxed::allow(&id, key, allow).err().map(|e| e.words(not_changed()));
                self.settled(kit, kind, &id, key, allow, err);
            }
            Kind::Appimage | Kind::Program => {

                let top = self.loose_folder();
                let which = if kind == Kind::Appimage { "appimage" } else { "program" };
                let err = loose::allow_in(&top, which, &id, key, allow).err().map(|e| e.words(not_changed()));
                self.settled(kit, kind, &id, key, allow, err);
            }
            Kind::Plugin if matches!(key, "listen" | "network") && plugins::find(&id).is_some_and(|p| !p.listen.is_empty()) => {

                let back = kit.then::<Permissions, Option<String>>({
                    let id = id.clone();
                    move |me, k, err| {
                        let changed = err.is_none();
                        me.settled(k, kind, &id, key, allow, err);
                        if changed {
                            restart_plugin(k, &id);
                        }
                    }
                });
                run::on_thread(move || plugins::set_allowed(&id, key, allow).err(), back);
            }
            Kind::Plugin => {
                let err = plugins::allow(&id, key, allow).err().map(|e| e.words(not_changed()));
                let changed = err.is_none();
                self.settled(kit, kind, &id, key, allow, err);
                if changed {
                    restart_plugin(kit, &id);
                }
            }
        }
    }

    fn settled(&mut self, kit: &mut Kit, kind: Kind, id: &str, key: &str, allow: bool, err: Option<String>) {
        if err.is_some() {
            self.set(kind, id, key, !allow);
        }

        let said = err.as_deref().map(|e| if kind == Kind::Plugin { e } else { not_changed() });
        kit.done(&said_key(kind, id), said, t!("settings-permissions-saved"));
    }

    fn row(&self, kit: &Kit, kind: Kind, o: &Owner) -> Node<Msg> {

        let waiting = kind == Kind::Flathub && !self.answered;
        let toggles = o.known.iter().map(|&(key, words)| {
            let id = o.id.clone();
            button(access::icon(words), access::title(words)).on(o.on.iter().any(|k| k == key))
                .on_press(on::<Permissions>(move |me, k| me.toggle(k, kind, id.clone(), key))).off(waiting)
        }).collect();
        let text = line(o.name.clone()).sub(o.sub.clone()).key(said_key(kind, &o.id)).controls(toggles).node(kit)
            .width(Length::Fill);

        let name = format!("row:{}", said_key(kind, &o.id));
        let picture = match &o.picture {
            Picture::App(_) if waiting => spinner().size(PICTURE),
            Picture::App(icons) => app_icon(icons.clone(), PICTURE),
            Picture::File(path) => logo(path.clone(), PICTURE),
            Picture::Nothing => return text.key(name),
        };
        row(vec![picture, text]).gap(12.0).width(Length::Fill).key(name)
    }

    fn older(&mut self, kit: &mut Kit, allow: bool) {
        self.older = allow;
        let err = files::set_setting(x11_rooms::SETTING, if allow { "yes" } else { "no" }).err();
        if err.is_some() {
            self.older = !allow;
        }
        kit.done(OLDER_KEY, err.map(|_| not_changed()), t!("settings-permissions-saved"));
    }

    fn codes_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows = Vec::new();
        for c in &self.codes {
            let id = c.id.clone();
            let off = button(access::icon("expand"), t!("settings-permissions-codes-off", name = &c.name)).on(true)
                .on_press(on::<Permissions>(move |me, k| me.toggle(k, Kind::Plugin, id.clone(), "expand")));
            let sub = t!("settings-permissions-codes-count", n = c.list.len() as i64, id = &c.id);
            rows.push(line(c.name.clone()).sub(sub).key(said_key(Kind::Plugin, &c.id)).control(off).node(kit));
            for (trigger, words) in c.list.iter().take(MOST_CODES_SHOWN) {
                rows.push(line(drawn(trigger)).sub(t!("settings-permissions-code-types", words = drawn(words), name = &c.name))
                    .key(format!("code:{}:{trigger}", c.id)).node(kit));
            }
            if c.list.len() > MOST_CODES_SHOWN {
                rows.push(line(t!("settings-permissions-codes-more", n = (c.list.len() - MOST_CODES_SHOWN) as i64)).node(kit));
            }
        }
        if self.codes_unread {
            rows.push(line(t!("settings-permissions-codes-unread-line")).key(CODES_KEY).node(kit));
        } else if rows.is_empty() {
            rows.push(line(t!("settings-permissions-codes-none")).sub(t!("settings-permissions-codes-none-sub")).node(kit));
        }
        group(t!("settings-permissions-codes")).icon(access::icon("expand")).about(t!("settings-permissions-codes-about"))
            .rows(rows).node(kit)
    }

    fn older_card(&self, kit: &Kit) -> Node<Msg> {
        let title = t!("settings-permissions-allow-older");
        let sw = switch(self.older, title, |b| on::<Permissions>(move |me, k| me.older(k, b)));
        group(t!("settings-permissions-older")).icon("preferences-desktop-display-symbolic")
            .rows(vec![line(title).sub(older_why()).key(OLDER_KEY).control(sw).node(kit)])
            .node(kit)
    }

    fn card(&self, kit: &Kit, heading: &str, icon: &str, about: &str, kind: Kind, owners: &[Owner], empty: &str) -> Node<Msg> {
        let rows = if owners.is_empty() {
            vec![line(empty).node(kit)]
        } else {
            super::users::banded_rows(owners.iter().map(|o| self.row(kit, kind, o)).collect())
        };
        let mut about = about.to_string();
        for o in owners {
            for w in o.known.iter().filter_map(|(k, _)| access::warning(k)) {
                about.push(' ');
                about.push_str(&t!("settings-permissions-title-and-words", title = &o.name, words = w));
            }
        }
        let card = group(heading).icon(icon).about(about).rows(rows);
        let (least, most) = if owners.len() > 3 { (5, 8) } else { crate::toolkit::theme::bento().card };
        card.node(kit).span_grows(least, most, crate::toolkit::theme::bento().words)
    }

}

impl Page for Permissions {
    fn id(&self) -> &'static str {
        "permissions"
    }
    fn title(&self) -> &'static str {
        t!("settings-permissions-title")
    }
    fn icon(&self) -> &'static str {
        "security-high-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-permissions-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-permissions-help")
    }
    fn shown(&mut self, kit: &mut Kit) {
        self.read(kit);
    }

    fn preload(&mut self, kit: &mut Kit) {
        self.fresh = false;
        self.read(kit);
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let (flathub_about, debs_about, plugins_about) =
            (t!("settings-permissions-flathub-about"), t!("settings-permissions-debs-about"), t!("settings-permissions-plugins-about"));
        let n = self.flathub.as_ref().map_or(0, Vec::len) + self.debs.len() + self.plugins.len()
            + self.appimages.len() + self.programs.len();
        let words = match (&self.flathub, n) {
            (None, _) => t!("settings-permissions-reading-apps").to_string(),
            (Some(_), n) if !self.answered && n > 0 => t!("settings-permissions-reading-each", n = n),
            (_, 0) => t!("settings-permissions-no-apps").to_string(),
            (_, 1) => t!("settings-permissions-one-app").to_string(),
            (_, n) => t!("settings-permissions-many-apps", n = n),
        };
        let mut items = vec![headline(kit, words)];

        let flathub = self.flathub.as_deref().unwrap_or(&[]);
        let kinds: [(&str, &str, &str, Kind, &[Owner], &str); 5] = [
            (t!("settings-permissions-flathub-apps"), "system-software-install-symbolic", flathub_about, Kind::Flathub, flathub,
             if self.flathub.is_some() { t!("settings-permissions-none-installed") } else { t!("settings-permissions-reading-apps") }),
            (t!("settings-permissions-own-folder"), "folder-symbolic", debs_about, Kind::Deb, &self.debs, t!("settings-permissions-none-installed")),
            (t!("settings-permissions-plugins"), "application-x-addon-symbolic", plugins_about, Kind::Plugin, &self.plugins, t!("settings-permissions-none-installed")),
            (t!("settings-permissions-appimages"), "application-x-executable-symbolic", &loose_words(loose_images()), Kind::Appimage, &self.appimages, t!("settings-permissions-none-run")),
            (t!("settings-permissions-from-folders"), "folder-open-symbolic", &loose_words(loose_programs()), Kind::Program, &self.programs, t!("settings-permissions-none-run")),
        ];

        let at = self.tab.filter(|t| *t < kinds.len()).unwrap_or_else(|| kinds.iter().position(|k| !k.4.is_empty()).unwrap_or(0));
        let names = kinds.iter().map(|(heading, _, _, _, owners, _)| if owners.is_empty() { heading.to_string() } else { format!("{heading} ({})", owners.len()) }).collect();
        items.push(dropdown(kit, "permissions-kind", names, Some(at), t!("settings-permissions-kind-tip"),
                                              |i| on::<Permissions>(move |p, _| p.tab = Some(i))));
        let (heading, icon, words, kind, owners, none) = kinds[at];
        items.push(self.card(kit, heading, icon, words, kind, owners, none));
        items.push(self.codes_card(kit));
        items.push(self.older_card(kit));
        items.extend(self.outside.cards(kit));
        items.push(self.thunder_card(kit));
        items.push(group(t!("common-in-use-key")).icon("dialog-information-symbolic").about(t!("common-in-use-where")).rows(in_use_key()).node(kit));
        page(kit, self.title(), self.help(), items)
    }
}

#[cfg(test)]
fn boxed_one(id: &str, name: &str, on: &[&str]) -> Owner {
    Owner { id: id.into(), name: name.into(), sub: String::new(), picture: Picture::Nothing,
            known: boxed::ALLOWED.iter().map(|k| (*k, *k)).collect(), on: on.iter().map(|s| s.to_string()).collect() }
}

#[cfg(test)]
impl Permissions {

    pub(crate) fn made_up() -> Permissions {
        let known: Vec<(&'static str, &'static str)> = access::FLATPAK.iter().map(|p| (p.key, p.words)).collect();
        let app = |id: &str, name: &str, on: &[&str]| Owner {
            id: id.into(), name: name.into(), sub: String::new(), picture: Picture::App(Vec::new()), known: known.clone(),
            on: on.iter().map(|s| s.to_string()).collect(),
        };
        let first = known.first().map(|k| k.0).unwrap_or("");
        let asked: Vec<(&'static str, &'static str)> = plugins::PERMISSIONS.iter().take(3).map(|k| (*k, *k)).collect();
        let plugin = Owner { id: "org.example.Pair".into(), name: "Pair".into(), sub: "Version 1.2".into(),
                             picture: Picture::Nothing, on: asked.iter().take(1).map(|k| k.0.to_string()).collect(), known: asked };
        let domains = vec![crate::thunder::sysfs::Domain { number: "0".into(), level: Level::User, iommu: true }];
        let dock = ("aaaa-1".to_string(), "Dock Makers".to_string(), "Big Dock".to_string());
        Permissions { flathub: Some(vec![app("org.example.Browser", "Browser", &[first]), app("org.example.Notes", "Notes", &[])]),
                      debs: Vec::new(), plugins: vec![plugin], fresh: true,
                      appimages: vec![boxed_one("krita", "Krita", &["network", "sound", "x11"])],
                      programs: vec![boxed_one("gimp", "gimp", &["network", "sound", "files"])], answered: true,
                      thunder: Some((Listing { domains, devices: vec![dock] }, None)), ..Permissions::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::ui::Do;
    use crate::thunder::sysfs::Domain;
    use crate::toolkit::event::Loop;

    #[test]
    fn the_page_ends_with_the_card_that_says_what_the_dots_mean() {
        use crate::common::in_use::Kind;
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let kit = Kit::new(events.clone(), asks, None, None);
        let words = Permissions::made_up().view(&kit).said();
        let head = words.iter().rposition(|w| w == t!("common-in-use-key")).expect("the card");
        for kind in Kind::ALL {
            assert!(words[head..].contains(&kind.spoken(&[])), "{kind:?}");
        }
    }

    fn code_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-codes-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn plugin_with(dir: &Path, id: &str, record: &str, file: &str) {
        std::fs::create_dir_all(dir.join(id).join("home")).unwrap();
        std::fs::write(dir.join(id).join("allowed"), record).unwrap();
        std::fs::write(dir.join(id).join("home").join(crate::common::expand::FILE), file).unwrap();
    }

    const TWO: &str = r#"{"version":1,"snippets":[{"trigger":"tn#","replace":"Trindle"},{"trigger":"addr;","replace":"1 High Street"}]}"#;

    #[test]
    fn the_list_has_every_active_code_under_its_own_plugin_and_no_other() {
        let dir = code_dir("list");
        plugin_with(&dir, "org.example.One", "network\nexpand\n", TWO);
        plugin_with(&dir, "org.example.Two", "expand\n", r#"{"version":1,"snippets":[{"trigger":"tn#","replace":"Other"}]}"#);
        plugin_with(&dir, "org.example.Off", "network\n", TWO);
        plugin_with(&dir, "org.example.Bad", "expand\n", r#"{"version":1,"snippets":[{"trigger":"ab","replace":"short"},{"trigger":"go#","replace":"x\ny"}]}"#);
        plugin_with(&dir, "..sneaky", "expand\n", TWO);
        let name = |id: &str| match id {
            "org.example.One" => Some("Fine\u{202e}Snippets\n".to_string()),
            "org.example.Two" => Some("Fine Snippets".to_string()),
            _ => None,
        };
        let all = read_codes(&dir, &name).unwrap();
        let ids: Vec<&str> = all.iter().map(|c| c.id.as_str()).collect();
        assert_eq!(ids, ["org.example.One", "org.example.Two"], "only plugins allowed expand, with codes that count");
        assert_eq!(all[0].list.len(), 2);
        assert_eq!(all[0].name, "FineSnippets", "direction marks and control characters are not drawn");
        assert_eq!(all[1].list, [("tn#".to_string(), "Other".to_string())], "the same code is shown under each owner");
        assert_eq!(drawn("a\u{200b}b\u{2028}c\u{feff}d\u{e0041}e"), "abcde", "invisible characters are not left to hide text");
        assert_eq!(drawn("\u{1F468}\u{200d}\u{1F469}"), "\u{1F468}\u{200d}\u{1F469}", "a joiner in an emoji stays");
        let long = "x".repeat(200);
        assert!(drawn(&long).chars().count() <= MOST_DRAWN + 1);
        let unnamed = read_codes(&dir, &|_| Some("\u{7}".into())).unwrap();
        assert_eq!(unnamed[0].name, "org.example.One", "a name that draws as nothing falls back to the folder's name");
    }

    #[test]
    fn invisible_text_is_not_drawn_and_an_unreadable_folder_is_not_an_empty_one() {
        for hidden in ["\u{200b}", "\u{200d}", "\u{2060}", "\u{ad}", "\u{feff}", "\u{e0041}", "\u{200e}", "\u{7}"] {
            assert_eq!(drawn(&format!("a{hidden}b")), "ab", "{hidden:?}");
            assert!(drawn(hidden).is_empty());
        }
        let dir = code_dir("hidden");
        plugin_with(&dir, "org.example.Blank", "expand\n", TWO);
        let blank = read_codes(&dir, &|_| Some("\u{200b}\u{2060}\u{feff}".into())).unwrap();
        assert_eq!(blank[0].name, "org.example.Blank", "a name of invisible text falls back to the folder's name");
        let not_a_folder = dir.join("org.example.Blank").join("allowed");
        assert!(read_codes(&not_a_folder, &|_| None).is_err(), "a file where the folder should be");
        assert_eq!(read_codes(&dir.join("nothing-here"), &|_| None).unwrap().len(), 0, "no folder yet means no plugins");
    }

    #[test]
    fn a_linked_file_or_a_withdrawn_permission_lists_nothing() {
        let dir = code_dir("links");
        plugin_with(&dir, "org.example.Real", "expand\n", TWO);
        std::fs::create_dir_all(dir.join("org.example.Linked").join("home")).unwrap();
        std::fs::write(dir.join("org.example.Linked").join("allowed"), "expand\n").unwrap();
        std::os::unix::fs::symlink(dir.join("org.example.Real").join("home").join(crate::common::expand::FILE),
                                   dir.join("org.example.Linked").join("home").join(crate::common::expand::FILE)).unwrap();
        assert_eq!(read_codes(&dir, &|_| None).unwrap().iter().map(|c| c.id.as_str()).collect::<Vec<_>>(), ["org.example.Real"]);
        std::fs::write(dir.join("org.example.Real").join("allowed"), "network\n").unwrap();
        assert!(read_codes(&dir, &|_| None).unwrap().is_empty());
    }

    #[test]
    fn the_page_draws_each_code_and_a_made_up_press_cannot_switch_them_off() {
        use crate::toolkit::window::{with_named, with_press};
        let dir = code_dir("page");
        plugin_with(&dir, "org.example.One", "expand\n", TWO);
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut kit = Kit::new(events.clone(), asks, None, None);
        let mut page = Permissions { plugins_top: Some(dir.clone()), ..Permissions::default() };
        assert!(page.codes_card(&kit).said().iter().any(|w| w == t!("settings-permissions-codes-none")));
        page.codes = read_codes(&dir, &|_| Some("Snippets".into())).unwrap();
        let words = page.codes_card(&kit).said().join("|");
        for want in ["tn#", "addr;", "Snippets", "Trindle", "1 High Street"] {
            assert!(words.contains(want), "{want} missing from {words}");
        }
        page.plugins.push(Owner { id: "org.example.One".into(), name: "Snippets".into(), sub: String::new(), picture: Picture::Nothing,
                                  known: vec![("expand", "expand")], on: vec!["expand".into()] });
        with_named(true, Some(7), || with_press(Some(8), || page.toggle(&mut kit, Kind::Plugin, "org.example.One".into(), "expand")));
        assert_eq!(page.plugins[0].on, ["expand"], "a press that was not the person's own switched codes off");
    }

    #[test]
    fn a_long_list_is_cut_and_the_rest_counted() {
        let dir = code_dir("long");
        let many: Vec<String> = (0..300).map(|i| format!(r#"{{"trigger":"c{i:04}","replace":"x"}}"#)).collect();
        plugin_with(&dir, "org.example.Many", "expand\n", &format!(r#"{{"version":1,"snippets":[{}]}}"#, many.join(",")));
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let kit = Kit::new(events.clone(), asks, None, None);
        let page = Permissions { codes: read_codes(&dir, &|_| None).unwrap(), ..Permissions::default() };
        assert_eq!(page.codes[0].list.len(), 300);
        let words = page.codes_card(&kit).said();
        assert!(words.iter().any(|w| w == "c0199") && !words.iter().any(|w| w == "c0200"));
        assert!(words.iter().any(|w| w.contains("100 more codes")));
    }

    fn two_named() -> Vec<String> {
        vec!["org.example.Browser".into(), "org.example.Notes".into()]
    }

    fn two_found() -> Found {
        vec![("org.example.Browser".into(), vec!["network"]), ("org.example.Notes".into(), Vec::new())]
    }

    #[test]
    fn a_made_up_press_on_a_permission_changes_nothing() {
        use crate::toolkit::window::{with_named, with_press};
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut kit = Kit::new(events.clone(), asks, None, None);
        let mut page = Permissions::made_up();
        page.debs.push(Owner { id: "org.example.Package".into(), name: "Package".into(), sub: String::new(), picture: Picture::Nothing,
                               known: vec![("network", "network")], on: Vec::new() });
        let all = |page: &mut Permissions| -> Vec<Vec<String>> {
            [Kind::Flathub, Kind::Deb, Kind::Plugin].iter().flat_map(|k| page.owners(*k).unwrap().iter().map(|o| o.on.clone()).collect::<Vec<_>>()).collect()
        };
        let before = all(&mut page);
        for (kind, id, key) in [(Kind::Flathub, "org.example.Notes", "network"), (Kind::Deb, "org.example.Package", "network"),
                                (Kind::Plugin, "org.example.Pair", "network")] {
            with_named(true, Some(7), || with_press(Some(8), || page.toggle(&mut kit, kind, id.to_string(), key)));
            with_named(true, None, || with_press(Some(7), || page.toggle(&mut kit, kind, id.to_string(), key)));
            with_named(true, Some(7), || page.toggle(&mut kit, kind, id.to_string(), key));
        }
        assert_eq!(before, all(&mut page), "a permission was changed by a press that was not the person's own");
    }

    fn made_up_asker() -> Permissions {
        Permissions { asker: Some(two_found), namer: Some(two_named), thunder: Some((Listing { domains: Vec::new(), devices: Vec::new() }, None)),
                      ..Permissions::default() }
    }

    fn icons(p: &Permissions, kit: &Kit) -> Vec<(String, bool)> {
        let titles: Vec<String> = access::FLATPAK.iter().map(|k| access::title(k.words).to_string()).collect();
        p.view(kit).buttons().into_iter().filter(|(tip, _)| titles.contains(tip)).collect()
    }

    fn ahead(p: &mut Permissions, kit: &mut Kit) {
        p.preload(kit);
        (p.debs, p.plugins) = (Vec::new(), Vec::new());
    }

    fn open(p: &mut Permissions, kit: &mut Kit) {
        p.shown(kit);
        (p.debs, p.plugins) = (Vec::new(), Vec::new());
    }

    #[test]
    fn read_ahead_the_page_is_whole_when_first_opened_and_asks_nothing_more() {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut kit = Kit::new(events.clone(), asks, None, None);
        let mut p = made_up_asker();
        ahead(&mut p, &mut kit);
        assert_eq!((p.turn, p.fresh), (1, false), "reading ahead asks once");
        p.heard(1, two_found());
        assert!(p.fresh && p.answered);

        open(&mut p, &mut kit);
        assert_eq!(p.turn, 1, "opening the page asks Flatpak nothing");
        let said = p.view(&kit).said();
        assert!(said.iter().any(|w| w == "org.example.Browser") && said.iter().any(|w| w == "org.example.Notes"), "{said:?}");
        assert!(!said.iter().any(|w| w.starts_with("Reading")), "{said:?}");
        let all = icons(&p, &kit);
        assert_eq!(all.len(), 2 * access::FLATPAK.len());
        assert!(all.iter().all(|(_, can)| *can), "{all:?}");

        p.hidden(&mut kit);
        open(&mut p, &mut kit);
        assert_eq!(p.turn, 1);
    }

    #[test]
    fn a_change_to_an_app_reads_again_and_keeps_what_shows_meanwhile() {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut kit = Kit::new(events.clone(), asks, None, None);
        let mut p = made_up_asker();
        ahead(&mut p, &mut kit);
        p.heard(1, two_found());

        ahead(&mut p, &mut kit);
        assert_eq!((p.turn, p.fresh), (2, false));
        assert_eq!(p.flathub.as_ref().map(Vec::len), Some(2), "the list does not empty while it is read again");
        assert!(icons(&p, &kit).iter().all(|(_, can)| *can), "and its icons stay live");

        let mut three = two_found();
        three.push(("org.example.Paint".into(), vec!["home"]));

        p.heard(1, Vec::new());
        assert_eq!(p.flathub.as_ref().map(Vec::len), Some(2));
        p.heard(2, three);
        assert_eq!((p.flathub.as_ref().map(Vec::len), p.fresh), (Some(3), true));

        ahead(&mut p, &mut kit);
        let asked = p.turn;
        p.turn += 1;
        p.fresh = false;
        p.heard(asked, Vec::new());
        assert_eq!(p.flathub.as_ref().map(Vec::len), Some(3), "an answer older than the change is dropped");
        open(&mut p, &mut kit);
        assert_eq!(p.turn, asked + 2, "and the page asks again when next it reads");
    }

    #[test]
    fn opened_before_the_answer_the_rows_are_in_place_and_nothing_jumps() {
        use crate::toolkit::theme::Theme;
        use crate::toolkit::ui::Ui;
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut kit = Kit::new(events.clone(), asks, None, None);
        let mut p = made_up_asker();
        open(&mut p, &mut kit);
        assert_eq!((p.turn, p.answered), (1, false));
        let said = p.view(&kit).said();
        assert!(said.iter().any(|w| w == "org.example.Browser") && said.iter().any(|w| w == "org.example.Notes"), "{said:?}");
        assert!(said.iter().any(|w| w.starts_with("Reading what each of ") && w.ends_with(" may use…")), "{said:?}");
        let waiting = icons(&p, &kit);
        assert_eq!(waiting.len(), 2 * access::FLATPAK.len());
        assert!(waiting.iter().all(|(_, can)| !*can), "what is not known yet cannot be pressed: {waiting:?}");
        let rows = |p: &Permissions, kit: &Kit| -> Vec<(String, crate::toolkit::widget::Rect)> {
            let mut ui = Ui::new(p.view(kit), Theme::new(false, crate::common::style::DEFAULT_ACCENT));
            ui.places_at((1040, 2000)).into_iter().filter(|(id, _)| id.starts_with("row:permissions:flathub:")).collect()
        };
        let before = rows(&p, &kit);

        let names: std::collections::BTreeSet<&str> = before.iter().map(|(id, _)| id.split('/').next().unwrap_or_default()).collect();
        assert_eq!(names.len(), 2, "{before:?}");
        assert!(before.len() >= 2 * (2 + access::FLATPAK.len()), "{before:?}");
        p.heard(1, two_found());
        assert_eq!(rows(&p, &kit), before, "the answer moved a row, or something in one");
        assert!(icons(&p, &kit).iter().all(|(_, can)| *can));
        assert!(!p.view(&kit).said().iter().any(|w| w.starts_with("Reading")));
    }

    #[test]
    fn flatpak_s_apps_are_named_from_its_folders_with_nothing_run() {
        let dir = std::env::temp_dir().join(format!("trindle-permissions-named-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (own, all) = (dir.join("own"), dir.join("all"));
        for (place, app) in [(&own, "org.example.Notes"), (&own, "org.example.Browser"), (&all, "org.example.Browser"), (&all, "org.example.Paint")] {
            std::fs::create_dir_all(place.join("app").join(app)).unwrap();
        }
        std::fs::create_dir_all(own.join("app/.hidden.thing")).unwrap();
        std::fs::write(own.join("app/org.example.File"), "not a folder").unwrap();
        let named = access::flatpak_apps_named(&[own, all, dir.join("missing")]);
        assert_eq!(named, ["org.example.Browser", "org.example.Notes", "org.example.Paint"], "each once, in order");
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn two_boxes(tag: &str) -> PathBuf {
        let top = std::env::temp_dir().join(format!("trindle-permissions-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        loose::prepare(&top.join("appimage-krita"), loose::APPIMAGE_STARTS).unwrap();
        loose::prepare(&top.join("program-gimp"), loose::PROGRAM_STARTS).unwrap();
        std::fs::write(top.join("appimage-krita/name"), "Krita\n").unwrap();
        top
    }

    fn loose_page(top: &Path) -> Permissions {
        let mut p = Permissions { loose_top: Some(top.to_path_buf()), ..Permissions::default() };
        p.read_own();
        (p.debs, p.plugins) = (Vec::new(), Vec::new());
        p
    }

    #[test]
    fn the_page_lists_appimage_and_folders_boxes_with_every_switch() {
        let top = two_boxes("list");
        let p = loose_page(&top);
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let kit = Kit::new(events.clone(), asks, None, None);
        let mut p = p;
        let tab = |p: &mut Permissions, kind: usize| {
            p.tab = Some(kind);
            let view = p.view(&kit);
            (view.said(), view.buttons())
        };
        let (said, buttons) = tab(&mut p, 3);
        assert!(said.iter().any(|w| w == "Krita") && said.iter().any(|w| w == "AppImages"), "{said:?}");
        for key in boxed::ALLOWED {
            let tip = access::title(key);
            assert_eq!(buttons.iter().filter(|(t, can)| *t == tip && *can).count(), 1, "{key}: one switch per AppImage");
        }
        let (said, buttons) = tab(&mut p, 4);
        assert!(said.iter().any(|w| w == "gimp") && said.iter().any(|w| w == "Started from Folders"), "{said:?}");
        for key in boxed::ALLOWED {
            let tip = access::title(key);
            assert_eq!(buttons.iter().filter(|(t, can)| *t == tip && *can).count(), 1, "{key}: one switch per program");
        }
        assert_eq!(p.appimages[0].on, ["network", "sound", "x11"]);
        assert_eq!(p.programs[0].on, ["network", "sound"]);
        let all = loose_words(loose_images());
        for key in boxed::ALLOWED {
            assert!(all.contains(&access::off_line(access::ask(key).without)), "{key} has no Off: line");
        }
        let _ = std::fs::remove_dir_all(&top);
    }

    #[test]
    fn a_grant_changes_the_boxs_next_start_and_a_made_up_press_is_refused() {
        use crate::toolkit::window::{with_named, with_press};
        let top = two_boxes("grant");
        let mut p = loose_page(&top);
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut kit = Kit::new(events.clone(), asks, None, None);
        let args = |at: &Path| {
            let allowed = boxed::allowed_at(at, boxed::ALLOWED);
            boxed::walls::walls(at, &allowed, &boxed::walls::Gates::default(), &["x".to_string()]).join(" ")
        };
        let (image, program) = (top.join("appimage-krita"), top.join("program-gimp"));
        assert!(args(&image).contains("--share-net") && !args(&image).contains("--bind-try"));

        with_named(true, Some(7), || with_press(Some(8), || p.toggle(&mut kit, Kind::Appimage, "krita".into(), "files")));
        with_named(true, None, || with_press(Some(7), || p.toggle(&mut kit, Kind::Program, "gimp".into(), "network")));
        assert_eq!(boxed::allowed_at(&image, boxed::ALLOWED), ["network", "sound", "x11"]);
        assert_eq!(boxed::allowed_at(&program, boxed::ALLOWED), ["network", "sound"]);
        assert!(!p.appimages[0].on.iter().any(|k| k == "files"));

        with_named(true, Some(7), || with_press(Some(7), || {
            p.toggle(&mut kit, Kind::Appimage, "krita".into(), "files");
            p.toggle(&mut kit, Kind::Program, "gimp".into(), "network");
        }));
        assert!(args(&image).contains("--bind-try") && args(&image).contains("--share-net"), "{}", args(&image));
        assert!(!args(&program).contains("--share-net"), "{}", args(&program));
        assert!(p.appimages[0].on.iter().any(|k| k == "files") && !p.programs[0].on.iter().any(|k| k == "network"));

        let again = loose_page(&top);
        assert_eq!(again.appimages[0].on, p.appimages[0].on);

        for (kind, id) in [("appimage", "../program-gimp"), ("program", "nobody"), ("other", "gimp"), ("appimage", "")] {
            assert!(loose::allow_in(&top, kind, id, "files", true).is_err(), "{kind} {id}");
        }
        assert!(loose::allow_in(&top, "program", "gimp", "wheel", true).is_err(), "a grant that does not exist");
        let _ = std::fs::remove_dir_all(&top);
    }

    #[test]
    fn the_thunderbolt_card_says_what_the_ports_do() {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let kit = Kit::new(events.clone(), asks, None, None);
        let mut p = Permissions::default();
        let _ = p.thunder_card(&kit);
        let dock = ("aaaa-1".to_string(), "Dock Makers".to_string(), "Big Dock".to_string());
        for level in [None, Some(Level::None), Some(Level::DpOnly), Some(Level::UsbOnly), Some(Level::User), Some(Level::Secure)] {
            let domains: Vec<Domain> = level.into_iter().map(|level| Domain { number: "0".into(), level, iommu: true }).collect();
            p.thunder = Some((Listing { domains, devices: vec![dock.clone()] }, None));
            let _ = p.thunder_card(&kit);
        }
        let user = vec![Domain { number: "0".into(), level: Level::User, iommu: false }];
        p.thunder = Some((Listing { domains: user, devices: Vec::new() }, Some("not running".into())));
        let _ = p.thunder_card(&kit);
        for l in [Level::None, Level::DpOnly, Level::UsbOnly, Level::NoPcie] {
            assert!(lets_in(l).contains("nothing to ask"), "{l:?}");
            assert!(!lets_in(l).contains('\u{2014}'));
        }
    }
}
