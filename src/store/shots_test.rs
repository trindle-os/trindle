//! Pictures of every page of Apps in every state, drawn from made-up data,
//! for looking at the layout: run with `-- --ignored shoot_every_apps_page`.
//! TRINDLE_SHOTS is the folder and TRINDLE_SHOTS_NAME
//! what each file's name starts with ("apps"). Each is the whole window,
//! as tall as the page is long, at a usual width and at a narrow one.
//! Nothing here asks the network, a command or the session for anything.

use std::path::PathBuf;

use std::rc::Rc;
mod page_shots_test;
use super::catalogue::{categories, App, Catalogue, POPULAR};
use super::updates::{Installed, Source};
use super::*;
use super::launch::folders_that_change;
use super::side::{Place, SIDE};
use crate::plugins::catalogue::{Entry, Offer, Refused};
use crate::plugins::Plugin;
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::{App as Shows, Ui};
use crate::toolkit::widget::Length;
mod shots_test_pages;
mod shots_test_side_bar;
mod shots_test_rows;
mod shots_test_held_back;
mod shots_test_offline;
mod shots_test_installed;
mod shots_test_mixed;

const SIZES: [(&str, u32, u32); 2] = [("", 1278, 692), ("-narrow", 820, 692)];

const NAMES: [&str; 24] = ["Photos", "Notes", "Music", "Maps", "Clock", "Paint", "Letters", "Films", "Recipes", "Budget", "Diary", "Sketch",
                           "Chess", "Radio", "Weather", "Scanner", "Sums", "Birds", "Stars", "Trains", "Typing", "Podcasts", "Garden",
                           "A Very Long Name For An App That Goes On And On"];

const LINES: [&str; 6] = ["Look after your pictures", "Write things down", "Play your music and find more of it, wherever it is kept",
                          "", "Find the way", "Does a great many things for a great many people, all at once and all day long"];

const COLOURS: [[u8; 3]; 6] = [[0x22, 0x95, 0xf1], [0xfe, 0x97, 0x00], [0x9b, 0x26, 0xae], [0x01, 0x95, 0x87], [0xe8, 0x20, 0x7e], [0x79, 0x56, 0x49]];

fn folder() -> PathBuf {
    let dir = PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));

    if dir.is_absolute() { dir } else { std::env::current_dir().unwrap().join(dir) }
}

fn icon(n: usize) -> PathBuf {
    let dir = folder().join("made-up-icons");
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join(format!("{}.png", n % COLOURS.len()));
    if !file.is_file() {
        let c = COLOURS[n % COLOURS.len()];
        let img = image::RgbaImage::from_fn(128, 128, |x, y| {
            let (dx, dy) = (x as f32 - 63.5, y as f32 - 63.5);
            if dx * dx + dy * dy < 60.0 * 60.0 { image::Rgba([c[0], c[1], c[2], 255]) } else { image::Rgba([0, 0, 0, 0]) }
        });
        img.save(&file).unwrap();
    }
    file
}

fn app(n: usize) -> App {
    let name = if n < NAMES.len() { NAMES[n].to_string() } else { format!("{} {}", NAMES[n % NAMES.len()], n / NAMES.len() + 1) };
    App {
        id: format!("org.example.App{n}"),
        name,
        summary: LINES[n % LINES.len()].to_string(),
        description: "It keeps what you make in one tidy place. ".repeat(6),
        developer: if n % 3 == 0 { String::new() } else { "John Smith".into() },
        licence: "GPL-3.0-or-later".into(),
        categories: vec![categories()[n % 3].0.to_string()],

        icon: (n % 7 != 6).then(|| icon(n)),
        homepage: "https://example.com".into(),
        keywords: String::new(),
        screenshots: vec![(format!("https://example.com/{n}/0.png"), "The main window".into())],
        ..Default::default()
    }
}

fn catalogue() -> Catalogue {
    let mut all: Catalogue = (0..200).map(|n| (format!("org.example.App{n}"), app(n))).collect();
    for (n, popular) in POPULAR.iter().enumerate() {
        all.insert(popular.to_string(), App { id: popular.to_string(), ..app(n) });
    }
    all
}

fn store() -> Store {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut s = Store::new(&events, asks, None);
    s.online = true;
    s
}

fn stocked() -> Store {
    let mut s = store();
    s.apps = Rc::new(catalogue());
    for n in [1, 4, 9] {
        s.installed.insert(format!("org.example.App{n}"), ("user".into(), "3.2.1".into()));
    }
    s
}

fn without_catalogue(online: bool) -> Store {
    let mut s = with_updates(store());
    (s.failed, s.online) = (Some("Flathub did not answer.".into()), online);
    for n in [1, 4, 9] {
        s.installed.insert(format!("org.example.App{n}"), ("user".into(), "3.2.1".into()));
    }
    s.owned = pages::made_up::installed(Some(owned(9)), 0, "");
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
    s
}

fn owned(n: usize) -> Vec<Installed> {
    (0..n).map(|i| {
        let source = match i % 5 {
            3 => Source::Flatpak { origin: "example-origin".into(), installation: "user".into() },
            4 => Source::AppImage(PathBuf::from(format!("/nonexistent/{i}.AppImage"))),
            _ => Source::Flatpak { origin: "flathub".into(), installation: "user".into() },
        };
        let appimage = matches!(source, Source::AppImage(_));
        Installed {
            id: format!("org.example.App{i}"), name: app(i).name,
            version: if appimage { String::new() } else { ["3.2.1", "46.1", "2026.10.1-beta.3"][i % 3].into() },
            size: if appimage { String::new() } else { ["45.2 MB", "1.3 GB", "880 kB"][i % 3].into() }, source,
        }
    }).collect()
}

fn with_updates(mut s: Store) -> Store {
    s.updatable = vec!["org.example.App1".into(), "org.example.App4".into()];
    s.debian = (0..11).map(|i| format!("example-package-{i}")).collect();
    s.trindle_waiting = Some("9.9.9".into());
    s.firmware = vec![firmware::Firmware { id: "abc".into(), name: "Example keyboard".into(), current: "1.2".into(), newest: "1.3".into(), size: 2_400_000 }];
    s
}

const SIZES_KNOWN: [(&str, u64); 5] = [("org.example.App1", 45_200_000), ("org.example.App4", 120_000_000), ("trindle", 31_000_000),
                                       ("example-package-0", 1_500_000), ("example-package-1", 300_000)];

const FLATS: [(&str, bool, &str, u64); 3] = [("org.example.App1", true, "3.3", 45_200_000), ("org.example.App4", true, "", 120_000_000),
                                             ("org.example.Platform", false, "", 88_000_000)];

const DEVICE: &str = "fw:abc";

fn going(share: f64) -> Dial {
    let mut d = Dial::default();
    d.within(share);
    d
}

fn plugin(n: usize, version: &str, asks: &[&str]) -> Plugin {
    Plugin { id: format!("org.example.Plugin{n}"), name: ["Weather", "Timer", "Tides", "Lamp"][n % 4].into(), version: version.into(),
             summary: ["The weather where you are and for the days ahead, in the ring and in a notification each morning.", "A kitchen timer in the ring.", "", "Turns a lamp on and off."][n % 4].into(),
             asks: asks.iter().map(|a| a.to_string()).collect(), ring: None, fields: Vec::new(), background: None, listen: Vec::new(),
             shortcuts: Vec::new() }
}

fn offer(n: usize, version: &str, asks: &[&str]) -> Offer {
    let p = plugin(n, version, asks);
    Offer {
        entry: Entry { id: p.id, name: p.name, version: p.version, summary: p.summary, permissions: p.asks, icon: None, icon_sha256: None,
                       archive: "a.tar.zst".into(), sha256: "0".repeat(64) },
        icon: Some(icon(n)),
    }
}

fn paint<M: Clone + 'static>(name: &str, tree: impl Fn() -> crate::toolkit::widget::Node<M>, width: u32, least: u32) {
    let start = std::env::var("TRINDLE_SHOTS_NAME").unwrap_or_else(|_| "apps".into());
    let mut ui = Ui::new(tree(), Theme::new(false, "#fbc711"));
    let h = (ui.natural_height(width as f32).ceil() as u32).clamp(least, 4000);
    let mut pix = tiny_skia::Pixmap::new(width, h).unwrap();
    crate::toolkit::theme::drawn::page(name);
    ui.paint(&mut pix.as_mut(), 1.0);
    if !crate::toolkit::theme::drawn::noting() {
        pix.save_png(folder().join(format!("{start}-{name}.png"))).unwrap();
    }
}

const LIMIT_SAID: &str = "GitHub has stopped answering for now: it answers 60 questions an hour from one address without a key. \
                          It will answer again in about 23 minutes. A GitHub key raises that to 5,000: press the key at the top right of Search GitHub.";

fn key_button_at(s: &mut Store, w: u32, h: u32) -> crate::toolkit::widget::Rect {
    let mut ui = Ui::new(s.whole(settings()), Theme::new(false, "#fbc711"));
    ui.places_at((w, h)).into_iter().find(|(name, _)| name == "gh:key-open").map(|(_, spot)| spot).expect("the key button is on the page")
}

fn settings() -> Vec<Place> {
    ["Default apps", "Startup apps", "More settings", "Permissions", "Your plugins", "Crash reports"].iter().enumerate()
        .map(|(i, name)| (format!("set:made-up-{i}"), name.to_string(), "preferences-system-symbolic")).collect()
}

fn shoot(name: &str, s: &mut Store) {

    let sizes = if crate::toolkit::theme::drawn::noting() { &SIZES[..1] } else { &SIZES[..] };
    for (end, w, h) in sizes.iter().copied() {
        s.across = w as f32;
        s.tall = h as f32;
        paint(&format!("{name}{end}"), || s.whole(settings()), w, h);
    }
}

fn every_apps_page() {
    offline_before_the_catalogue();
    offline_pages_without_catalogue();
    no_flathub_pages();
    discover_pages();
    let mut s = installed_pages();
    installed_every_kind();
    installed_update_states(&mut s);
    updates_nothing_found();
    let mut s = updates_found_and_held();
    updates_held_with_nothing_newer(&mut s);
    updates_only_held_back(&mut s);
    updates_going_in(&mut s);
    updates_installing_and_restart();
    plugin_pages();
    app_detail_pages();
    shots_test_mixed::mixed_and_image_pages();
    other_ways_to_get_apps();
    package_windows();
}

fn offline_before_the_catalogue() {

    let mut s = store();
    s.loading = true;
    shoot("loading", &mut s);
    let mut s = store();
    s.failed = Some("Flathub did not answer.".into());
    shoot("flathub-failed", &mut s);
    s.online = false;
    shoot("offline", &mut s);
    s.view = View::Category(categories()[0].0.to_string());
    shoot("offline-kind", &mut s);
    s.search = "photos".into();
    s.view = View::Search("photos".into());
    shoot("offline-results", &mut s);
}

fn offline_pages_without_catalogue() {

    let mut s = without_catalogue(false);
    s.view = View::Installed;
    shoot("offline-installed", &mut s);
    s.owned = pages::made_up::installed(Some(Vec::new()), 0, "");
    shoot("offline-installed-nothing", &mut s);
    let mut s = without_catalogue(false);
    s.view = View::Updates;
    shoot("offline-updates-known", &mut s);
    let mut s = store();
    (s.failed, s.online, s.view) = (Some("Flathub did not answer.".into()), false, View::Updates);
    s.upd = update_page::made_up(&[], 0, false, &[], &[], &[]);
    shoot("offline-updates-nothing-known", &mut s);
    for (name, view) in [("offline-plugins", View::Plugins), ("offline-github", View::GitHub), ("offline-sources", View::Sources),
                         ("offline-move-your-apps", View::Backup)] {
        let mut s = without_catalogue(false);
        s.plugins = plugin_page::made_up(true, vec![plugin(0, "1.0", &["network"])], Vec::new(), &[], None, &[]);
        s.sources = pages::made_up::sources();
        s.backup = pages::made_up::backup();
        s.view = view;
        shoot(name, &mut s);
    }
}

fn no_flathub_pages() {

    let mut s = without_catalogue(true);
    s.view = View::Installed;
    shoot("no-flathub-installed", &mut s);
    s.view = View::Updates;
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[("apps", "Flathub could not be reached.")], &[]).with_flats(&FLATS);
    shoot("no-flathub-updates", &mut s);
}

fn discover_pages() {

    let mut s = with_updates(stocked());
    shoot("discover", &mut s);
    let mut s = stocked();
    shoot("discover-nothing-waiting", &mut s);
    s.view = View::Category(categories()[0].0.to_string());
    shoot("kind", &mut s);
    s.view = View::Category("Nothing".into());
    shoot("kind-empty", &mut s);
    for (name, words) in [("results-many", "app"), ("results-few", "photos"), ("results-one", "trains 2"), ("results-none", "zzzz")] {
        s.search = words.into();
        s.view = View::Search(words.into());
        shoot(name, &mut s);
    }
    s.top = Some(("org.example.Nothing is not in the Flathub catalogue.".into(), Tone::Problem));
    shoot("results-with-a-problem-said", &mut s);
}

fn installed_pages() -> Store {

    let mut s = with_updates(stocked());
    s.view = View::Installed;
    s.owned = pages::made_up::installed(None, 0, "");
    shoot("installed-looking", &mut s);
    s.owned = pages::made_up::installed(Some(Vec::new()), 0, "");
    shoot("installed-nothing", &mut s);
    s.owned = pages::made_up::installed(Some(owned(9)), 0, "");
    shoot("installed", &mut s);
    s.owned = pages::made_up::installed(Some(owned(40)), 0, "");
    shoot("installed-many", &mut s);
    s.owned = pages::made_up::installed(Some(owned(9)), 3, "");
    shoot("installed-appimages", &mut s);
    s.owned = pages::made_up::installed(Some(owned(9)), 0, "pho");
    shoot("installed-narrowed", &mut s);
    s.owned = pages::made_up::installed(Some(owned(9)), 0, "zzzz");
    shoot("installed-no-match", &mut s);
    s
}

fn installed_every_kind() {

    let mut all = with_every_kind(with_updates(stocked()));
    shoot("installed-every-kind", &mut all);
    for (tab, name) in [(4, "installed-deb-files"), (5, "installed-plugins")] {
        all.owned = every_kind(tab, "");
        shoot(name, &mut all);
    }
    all.owned = every_kind(0, "t");
    shoot("installed-every-kind-narrowed", &mut all);
    all.owned = every_kind(0, "");
    all.held = held_one_of_each();
    shoot("installed-every-kind-held-back", &mut all);
    all.held = Default::default();
    all.dials.insert("pl:org.example.Plugin0".into(), going(0.42));
    shoot("installed-every-kind-plugin-updating", &mut all);
    all.dials.clear();
    all.owned = pages::made_up::said(every_kind(0, ""), "deb:example-editor", "It could not be removed: John Smith's folder is in use.");
    shoot("installed-every-kind-remove-failed", &mut all);
    let mut all = with_every_kind(without_catalogue(false));
    shoot("installed-every-kind-offline", &mut all);
}

fn installed_update_states(s: &mut Store) {
    s.owned = pages::made_up::installed(Some(owned(9)), 0, "");
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
    shoot("installed-update-waiting", s);
    s.other = Some(crate::common::updating::Who::Ring);
    shoot("installed-held-by-the-ring", s);
    s.other = None;
    s.held = held_one_of_each();
    shoot("installed-held-back", s);
    s.held = Default::default();
    s.said.insert(update_page::app_key("org.example.App1"), ("Not updated. Flatpak said: error: the remote could not be reached".into(), Tone::Problem));
    shoot("installed-update-failed", s);
    s.said.clear();
    s.dials.insert(update_page::app_key("org.example.App1"), going(0.42));
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS).with_one("Notes");
    shoot("installed-one-updating", s);
    s.dials.clear();
    s.dials.insert("up:apps".into(), Dial::default());
    s.upd = update_page::made_up(&[], 1, false, &SIZES_KNOWN, &[], &[("org.example.App1", 0.42), ("org.example.App4", 0.0)]);
    shoot("installed-updating", s);
}

fn updates_nothing_found() {

    let mut s = stocked();
    s.view = View::Updates;
    s.upd = update_page::made_up(&[], 0, false, &[], &[], &[]);
    shoot("updates-none", &mut s);
    s.upd = update_page::made_up(&["apps", "system", "firmware"], 0, false, &[], &[], &[]);
    shoot("updates-checking", &mut s);
    s.upd = update_page::made_up(&[], 0, false, &[], &[("apps", "Flathub could not be reached.")], &[]);
    shoot("updates-could-not-check", &mut s);
}

fn updates_found_and_held() -> Store {
    let mut s = with_updates(stocked());
    s.view = View::Updates;
    s.plugins = plugin_page::made_up(true, vec![plugin(0, "1.0", &["network"])], vec![offer(0, "1.2", &["network"])], &[], None, &[]);
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
    shoot("updates-found", &mut s);
    s.other = Some(crate::common::updating::Who::Ring);
    shoot("updates-held-by-the-ring", &mut s);
    s.other = None;

    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS).with_part_name("org.example.Platform", "Example Platform");
    shoot("updates-shared-part-named", &mut s);

    s.held = held_one_of_each();
    s.upd = std::mem::take(&mut s.upd).opened_with(&s.held);
    shoot("updates-held-back", &mut s);
    s
}

fn updates_held_with_nothing_newer(s: &mut Store) {

        use crate::common::held::{Held, Thing};
        let before = s.held.clone();
        let quiet = [Thing::app("org.example.App9"), Thing::part("org.example.Sdk", "24.08"), Thing::firmware("0a1b2c"), Thing::plugin("org.example.Plugin0")];
        s.held = Held::of(&[&[Thing::app("org.example.App1")][..], &quiet[..]].concat());
        s.upd = std::mem::take(&mut s.upd).opened_with(&s.held);
        shoot("updates-held-back-nothing-newer", s);
        s.held = Held::of(&[Thing::app("org.example.App1"), Thing::part("org.example.Sdk", "24.08"), Thing::plugin("org.example.Plugin0")]);
        shoot("updates-held-back-two-released", s);
        s.held = Default::default();
        shoot("updates-held-back-all-released", s);
        s.held = before;
        s.upd = std::mem::take(&mut s.upd).opened_with(&s.held);
}

fn updates_only_held_back(s: &mut Store) {

        let (debian, trindle) = (std::mem::take(&mut s.debian), s.trindle_waiting.take());
        s.held = crate::common::held::Held::of(&[crate::common::held::Thing::app("org.example.App1"), crate::common::held::Thing::app("org.example.App4"),
                                                 crate::common::held::Thing::part("org.example.Platform", "25.08"),
                                                 crate::common::held::Thing::firmware("abc"), crate::common::held::Thing::plugin("org.example.Plugin0")]);
        s.upd = std::mem::take(&mut s.upd).opened_with(&s.held);
        shoot("updates-only-held-back", s);
        (s.debian, s.trindle_waiting) = (debian, trindle);
}

fn updates_going_in(s: &mut Store) {
    s.held = Default::default();
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);

    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS).with_one("Notes");
    s.dials.insert(update_page::app_key("org.example.App1"), going(0.42));
    shoot("updates-one-app-going-in", s);
    s.dials.clear();
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS).with_one("Debian's packages");
    s.dials.insert("up:debian".into(), going(0.42));
    shoot("updates-debian-going-in", s);
    s.dials.clear();
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS).with_one("Example keyboard");
    s.dials.insert(DEVICE.into(), going(0.42));
    shoot("updates-firmware-going-in", s);
    s.dials.clear();

    s.updatable.remove(0);
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS[1..]);
    s.said.insert(update_page::app_key("org.example.App4"), ("Not updated. Flatpak said: error: the remote could not be reached".into(), Tone::Problem));
    s.said.insert(DEVICE.into(), ("The device must be plugged in to the charger.".into(), Tone::Problem));
    s.said.insert("up:trindle".into(), ("Trindle's own source could not be reached.".into(), Tone::Problem));
    shoot("updates-after-one-and-failures", s);
    s.said.clear();
}

fn updates_installing_and_restart() {
    let mut s = with_updates(stocked());
    s.view = View::Updates;
    s.plugins = plugin_page::made_up(true, vec![plugin(0, "1.0", &["network"])], vec![offer(0, "1.2", &["network"])], &[], None, &[]);
    s.upd = update_page::made_up(&[], 2, false, &SIZES_KNOWN, &[], &[("org.example.App1", 1.0), ("org.example.App4", 0.42)]);
    for key in ["up:apps", "up:system"] {
        let mut d = Dial::default();
        d.within(0.42);
        s.dials.insert(key.into(), d);
    }
    shoot("updates-installing", &mut s);
    s.dials.clear();
    let mut s = stocked();
    s.view = View::Updates;
    s.upd = update_page::made_up(&[], 0, true, &[], &[], &[]);
    s.said.insert("up:system".into(), ("example-package-3 could not be set up.".into(), Tone::Problem));
    shoot("updates-restart-and-a-failure", &mut s);
}

fn plugin_pages() {

    let mut s = stocked();
    s.view = View::Plugins;
    shoot("plugins-looking", &mut s);
    s.plugins = plugin_page::made_up(true, Vec::new(), Vec::new(), &[], None, &[]);
    shoot("plugins-none", &mut s);
    s.plugins = plugin_page::made_up(true, Vec::new(), vec![offer(0, "1.2", &["network", "notifications"]), offer(1, "2.0", &[]), offer(2, "0.4", &["network"])],
                                     &[], None, &[]);
    shoot("plugins-on-offer", &mut s);

    let refused = Refused::NoSignature.words();
    s.plugins = plugin_page::made_up(
        true,
        vec![plugin(0, "1.0", &["network", "notifications"]), plugin(1, "2.0", &[]), plugin(3, "1.0", &["network"])],
        vec![offer(0, "1.2", &["network", "notifications"]), offer(1, "2.0", &[]), offer(2, "0.4", &["network"])],
        &[("org.example.Plugin0", (2, 0)), ("org.example.Plugin1", (0, 0)), ("org.example.Plugin3", (1, 1))],
        Some(&refused),
        &[("org.example.Plugin2", "The archive is not the one the catalogue lists.")]);
    shoot("plugins", &mut s);
    s.dials.insert("pl:org.example.Plugin2".into(), Dial::default());
    shoot("plugins-installing", &mut s);
    s.dials.clear();
    s.held = held_one_of_each();
    shoot("plugins-held-back", &mut s);
    s.held = Default::default();
    s.other = Some(crate::common::updating::Who::Ring);
    shoot("plugins-held-by-the-ring", &mut s);
}

fn app_detail_pages() {

    let mut s = with_updates(stocked());
    let a = app(1);
    s.detail = Some(a.clone());
    s.facts.insert(a.id.clone(), Some(pages::install::AppFacts { download: "45 MB".into(), version: "3.2.1".into() }));
    s.reviews.insert(a.id.clone(), Some(reviews::Ratings { stars: [3, 5, 12, 40, 90], total: 150 }));
    s.said_about.insert(a.id.clone(), Vec::new());
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
    shoot("app-installed-update-waiting", &mut s);
    s.other = Some(crate::common::updating::Who::Ring);
    shoot("app-update-held-by-the-ring", &mut s);
    s.other = None;
    s.held = held_one_of_each();
    shoot("app-held-back", &mut s);
    s.held = Default::default();
    s.said.insert(update_page::app_key(&a.id), ("Not updated. Flatpak said: error: the remote could not be reached".into(), Tone::Problem));
    shoot("app-update-failed", &mut s);
    s.said.clear();
    s.dials.insert(update_page::app_key(&a.id), going(0.42));
    shoot("app-updating", &mut s);
    s.dials.clear();
    s.updatable.clear();
    s.upd = update_page::made_up(&[], 0, false, &[], &[], &[]);
    shoot("app-installed-up-to-date", &mut s);
    let mut s = with_updates(stocked());
    s.detail = Some(a.clone());
    s.facts.insert(a.id.clone(), Some(pages::install::AppFacts { download: "45 MB".into(), version: "3.2.1".into() }));
    s.reviews.insert(a.id.clone(), Some(reviews::Ratings { stars: [3, 5, 12, 40, 90], total: 150 }));
    s.said_about.insert(a.id.clone(), Vec::new());
    let b = app(2);
    s.detail = Some(b.clone());
    shoot("app-not-installed-still-asking", &mut s);
    s.dials.insert(b.id.clone(), { let mut d = Dial::default(); d.within(0.42); d });
    s.busy.insert(b.id.clone(), "install");
    shoot("app-installing", &mut s);
    s.dials.clear();
    s.busy.clear();
}

fn other_ways_to_get_apps() {

    let mut s = stocked();
    s.view = View::GitHub;
    shoot("github-empty", &mut s);
    let found = |n: usize| github::Found { repo: format!("john-smith/example-{n}"), name: NAMES[n].into(), summary: LINES[n % LINES.len()].into(), stars: Some(120 * n as u64), ..Default::default() };
    s.github = pages::made_up::github("photos", (1..4).map(found).collect(), (4..7).map(found).collect(), true);
    shoot("github-results", &mut s);

    for (end, w, h) in SIZES {
        s.across = w as f32;
        s.tall = h as f32;
        let at = key_button_at(&mut s, w, h);
        for (name, stored) in [("github-key-open", false), ("github-key-stored-open", true)] {
            s.github = pages::made_up::github_key(std::mem::take(&mut s.github), Some(at), stored, None);
            paint(&format!("{name}{end}"), || s.whole(settings()), w, h);
        }
        s.github = pages::made_up::github_key(std::mem::take(&mut s.github), None, true, None);
        paint(&format!("github-key-stored{end}"), || s.whole(settings()), w, h);
        s.github = pages::made_up::github_key(std::mem::take(&mut s.github), None, false, Some(LIMIT_SAID));
        paint(&format!("github-limit{end}"), || s.whole(settings()), w, h);
        s.github = pages::made_up::github_key(std::mem::take(&mut s.github), None, false, None);
    }
    s.view = View::Sources;
    shoot("sources-looking", &mut s);
    s.sources = pages::made_up::sources();
    shoot("sources", &mut s);
    s.view = View::Backup;
    s.backup = pages::made_up::backup();
    shoot("move-your-apps", &mut s);
}

fn package_windows() {

    let events = Loop::new();
    let d = deb::Deb { name: "example".into(), version: "1.2".into(), summary: "An example program".into(),
                       about: "It shows how a package looks before it is installed. ".repeat(4), installed_kib: Some(48_000), maker: "John Smith".into(),
                       arch: "amd64".into() };
    let from = origin::Origin { site: Some("dl.example.com".into()), referrer: None };
    let cases: Vec<(&str, deb::Package)> = vec![
        ("reading", deb::made_up(&events, None, None, None, false)),
        ("new", deb::made_up(&events, Some(Ok((d.clone(), deb::State::New))), Some((from.clone(), origin::Verdict::Clear)), None, false)),
        ("installing", deb::made_up(&events, Some(Ok((d.clone(), deb::State::New))), Some((from.clone(), origin::Verdict::Clear)), None, true)),
        ("installed", deb::made_up(&events, Some(Ok((d.clone(), deb::State::New))), Some((from.clone(), origin::Verdict::Clear)), Some("1.2".into()), false)),
        ("older-installed", deb::made_up(&events, Some(Ok((d.clone(), deb::State::New))), Some((from.clone(), origin::Verdict::Listed("example.com".into()))), Some("1.0".into()), false)),
        ("trindle", deb::made_up(&events, Some(Ok((d.clone(), deb::State::Trindle))), Some((origin::Origin::default(), origin::Verdict::Unknown)), None, false)),
        ("service", deb::made_up(&events, Some(Ok((d.clone(), deb::State::Service))), Some((origin::Origin::default(), origin::Verdict::Unknown)), None, false)),
        ("elsewhere", deb::made_up(&events, Some(Ok((d, deb::State::Elsewhere("arm64".into())))), Some((origin::Origin::default(), origin::Verdict::Unknown)), None, false)),
        ("not-a-package", deb::made_up(&events, Some(Err("This file is not a Debian package that can be installed. It may not have finished downloading.".into())), None, None, false)),
    ];
    for (name, p) in cases {
        paint(&format!("package-{name}"), || p.view(), deb::SIZE.0, deb::SIZE.1);
    }
}

type Spot = crate::toolkit::widget::Rect;

fn places(s: &mut Store, across: u32) -> Vec<(String, Spot)> {
    s.across = across as f32;
    let mut ui = Ui::new(s.whole(settings()), Theme::new(false, "#fbc711"));
    ui.places_at((across, 4000))
}

fn named(all: &[(String, Spot)], starts: &[&str]) -> Vec<(String, Spot)> {
    all.iter().filter(|(id, _)| !id.contains('/') && starts.iter().any(|w| id.starts_with(w))).cloned().collect()
}

fn lowered(all: &[(String, Spot)], by: f32) -> Vec<(String, Spot)> {
    all.iter().map(|(id, r)| (id.clone(), Spot { y: r.y + by, ..*r })).collect()
}

fn held_one_of_each() -> crate::common::held::Held {
    use crate::common::held::{Held, Thing};
    Held::of(&[Thing::app("org.example.App1"), Thing::part("org.example.Platform", "25.08"), Thing::firmware("abc"), Thing::plugin("org.example.Plugin0")])
}

fn words(s: &mut Store) -> Vec<String> {
    s.whole(settings()).said()
}

fn every_view() -> Vec<View> {
    vec![View::Home, View::Installed, View::Updates, View::Plugins, View::Browse, View::GitHub, View::Sources, View::Backup,
         View::Category(categories()[0].0.to_string()), View::Search("photos".into())]
}

fn missing() -> [(&'static str, Store); 3] {
    let mut fetching = without_catalogue(true);
    (fetching.failed, fetching.loading) = (None, true);
    [("fetching", fetching), ("no answer", without_catalogue(true)), ("no internet", without_catalogue(false))]
}

fn programs() -> Vec<boxed::added::Program> {
    vec![
        boxed::added::Program { package: "example-editor".into(), name: "Editor".into(), version: "2.4-1".into(), size: 48_300_000,
                         launcher: Some(PathBuf::from("/nonexistent/example-editor.desktop")), icon: None },
        boxed::added::Program { package: "example-tool".into(), name: "John Smith's Tool".into(), version: "1.0".into(), size: 0,
                         launcher: None, icon: Some(icon(3).to_string_lossy().into_owned()) },
    ]
}

fn with_every_kind(mut s: Store) -> Store {
    s.view = View::Installed;
    s.owned = every_kind(0, "");
    s.plugins = plugin_page::made_up(true, vec![plugin(0, "1.0", &["network"]), plugin(1, "2.0", &[]), plugin(2, "0.3", &[])],
                                     vec![offer(0, "1.2", &["network"]), offer(1, "2.0", &[])], &[], None, &[]);
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
    s
}

fn every_kind(tab: usize, find: &str) -> pages::InstalledPage {
    pages::made_up::with_debs(pages::made_up::installed(Some(owned(9)), tab, find), programs(),
                              &[("org.example.Plugin0", 2_400_000), ("org.example.Plugin1", 310_000)])
}

const DEB_ROWS: [&str; 2] = ["deb:example-editor", "deb:example-tool"];

const PLUGIN_ROWS: [&str; 3] = ["plugin:org.example.Plugin0", "plugin:org.example.Plugin1", "plugin:org.example.Plugin2"];

fn rows_shown(s: &mut Store, across: u32) -> Vec<String> {
    named(&places(s, across), &["remove:"]).into_iter().map(|(id, _)| id.trim_start_matches("remove:").to_string()).collect()
}

    #[test]
    fn the_card_removes_a_flatpak_with_its_data_only_when_asked() {
        use updates::{Installed, Removal, Source};
        let item = Installed { id: "org.example.Thing".into(), name: "Thing".into(), version: String::new(), size: String::new(),
                               source: Source::Flatpak { origin: "flathub".into(), installation: "user".into() } };
        for wipe in [false, true] {
            let mut s = store();
            let noted = Rc::new(std::cell::RefCell::new(Vec::new()));
            s.owned.removals = Some(noted.clone());
            super::pages::remove_ask::ask_removing(&mut s, None, Some(item.clone()));
            assert!(noted.borrow().is_empty(), "asking deletes nothing");
            super::pages::remove_ask::answer(&mut s, wipe);
            assert_eq!(*noted.borrow(), [(Removal::Flatpak { system: false, delete_data: wipe }, item.id.clone())]);
            assert!(s.owned.ask.is_none(), "the question is for one removal only");
        }
    }

    #[test]
    fn the_card_says_how_much_data_and_only_deleting_it_is_red() {
        use updates::{Installed, Source};
        let item = Installed { id: "org.example.Thing".into(), name: "Thing".into(), version: String::new(), size: String::new(),
                               source: Source::Flatpak { origin: "flathub".into(), installation: "user".into() } };
        let mut s = stocked();
        s.view = View::Installed;
        s.owned = pages::made_up::installed(Some(vec![item.clone()]), 0, "");
        for (bytes, want) in [(None, "Counting its saved data..."), (Some(0), "It has no saved data."), (Some(5_000_000), "Its saved data takes")] {
            super::pages::remove_ask::ask_removing(&mut s, None, Some(item.clone()));
            s.owned.ask.as_mut().unwrap().bytes = bytes;
            let card = super::pages::remove_ask::remove_card(s.owned.ask.as_ref().unwrap());
            assert_eq!(card.for_good(), ["Remove and delete its data", "Delete its data only"]);
            assert!(words(&mut s).iter().any(|w| w.starts_with(want)), "{bytes:?}: {want}");
        }

        let mut apps: Vec<_> = s.apps.values().cloned().collect();
        apps.sort_by(|a, b| a.id.cmp(&b.id));
        let app = apps[0].clone();
        let other = apps.iter().find(|a| a.id != app.id).cloned().expect("another app");
        s.installed.insert(app.id.clone(), ("user".into(), "1".into()));
        super::pages::remove_ask::ask_removing(&mut s, Some(app.clone()), None);
        assert!(app_page::gather(&s, &app).ask.is_some());
        assert!(app_page::gather(&s, &other).ask.is_none());
    }

fn listed() -> Vec<github::Found> {
    vec![
        github::Found { repo: "john-smith/example".into(), name: "Example".into(), summary: "Edits pictures".into(),
                        categories: vec!["Graphics".into()], icon: "https://appimage.github.io/database/Example/icon.png".into(),
                        listed: true, ..Default::default() },
        github::Found { site: "https://example.org/get".into(), name: "Elsewhere".into(), summary: "Plays songs".into(),
                        categories: vec!["Audio".into()], listed: true, ..Default::default() },
    ]
}
