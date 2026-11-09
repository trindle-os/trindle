//! Pictures of Apps' failures for the help pages (apps, plugins): each feeds the real window the real state and words.

use super::*;
use crate::toolkit::t;

fn snap(name: &str, s: &mut Store) {
    let w = 1278u32;
    s.across = w as f32;
    s.tall = 692.0;
    let mut ui = Ui::new(s.whole(settings()), Theme::new(false, "#fbc711"));

    let h = (ui.natural_height(w as f32).ceil() as u32).clamp(692, 1000);
    let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    std::fs::create_dir_all(folder()).unwrap();
    pix.save_png(folder().join(format!("apps-{name}.png"))).unwrap();
}

fn updates_page() -> Store {
    let mut s = with_updates(stocked());
    s.view = View::Updates;
    s.plugins = plugin_page::made_up(true, vec![plugin(0, "1.0", &["network"])], Vec::new(), &[], None, &[]);
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
    s
}

fn detail_page(online: bool) -> Store {
    let mut s = with_updates(stocked());
    let a = app(2);
    s.online = online;
    s.facts.insert(a.id.clone(), Some(pages::install::AppFacts { download: "45 MB".into(), version: "3.2.1".into() }));
    s.said_about.insert(a.id.clone(), Vec::new());
    s.detail = Some(a);
    s
}

fn plugins_page(said: &[(&str, &str)], own: Option<&str>) -> Store {
    let mut s = stocked();
    s.view = View::Plugins;
    s.plugins = plugin_page::made_up(true, vec![plugin(0, "1.0", &["network"]), plugin(3, "1.0", &["network"])],
                                     vec![offer(0, "1.2", &["network"]), offer(2, "0.4", &["network"])],
                                     &[("org.example.Plugin0", (1, 0)), ("org.example.Plugin3", (1, 1))], own, said);
    s
}

fn shoot_plugin_failures() {

    let id = "org.example.Plugin2";
    let words = t!("plugins-install-check", why = t!("plugins-check-needs-name"));
    snap("plugin-check-refused", &mut plugins_page(&[(id, words.as_str())], None));
    let words = t!("store-plugins-not-running", why = t!("plugins-bg-no-session"));
    snap("plugin-not-running", &mut plugins_page(&[("org.example.Plugin0", words.as_str())], None));
    let words = t!("plugins-download-failed", why = "Could not resolve host");
    snap("plugin-download-failed", &mut plugins_page(&[(id, words.as_str())], None));
    let words = t!("plugins-catalogue-last-good", why = Refused::NotSigned.words());
    snap("plugin-list-refused", &mut plugins_page(&[], Some(&words)));
}

#[test]
#[ignore = "makes the pictures of Apps' failures for the help pages"]
fn shoot_apps_failures() {

    let mut s = store();
    (s.failed, s.online) = (Some("Flathub did not answer.".into()), false);
    snap("discover-offline", &mut s);
    snap("picture-offline", &mut detail_page(false));

    let mut s = detail_page(true);
    s.said.insert(app(2).id, (t!("store-flatpak-refused").into(), Tone::Problem));
    snap("install-refused", &mut s);

    let mut s = updates_page();
    s.said.insert(update_page::app_key("org.example.App4"), (t!("store-not-updated-said", why = "Not enough disk space"), Tone::Problem));
    snap("update-not-updated", &mut s);
    let mut s = updates_page();
    s.said.insert(update_page::app_key("org.example.App1"), (t!("store-upd-flatpak-said", words = t!("store-upd-held-not-told"), why = "Unable to load metadata"), Tone::Problem));
    snap("held-not-told", &mut s);
    let mut s = updates_page();
    let because = t!("common-updating-because", why = t!("common-refused-why-cancelled"));
    s.said.insert(DEVICE.into(), (t!("common-updating-not-installed", why = because), Tone::Problem));
    snap("firmware-failed", &mut s);

    let mut s = stocked();
    s.view = View::GitHub;
    s.github = pages::made_up::github("photos", Vec::new(), Vec::new(), true);
    s.github = pages::made_up::github_key(std::mem::take(&mut s.github), None, false, Some(LIMIT_SAID));
    snap("github-limit", &mut s);
    let at = key_button_at(&mut s, 1278, 692);
    s.github = pages::made_up::github_key(std::mem::take(&mut s.github), Some(at), false, None);
    s.said.insert("gh:key".into(), (t!("store-gh-key-refused").into(), Tone::Problem));
    snap("github-key-refused", &mut s);

    let mut s = stocked();
    s.view = View::Backup;
    s.backup = pages::made_up::backup();
    s.said.insert("bk:make".into(), (t!("store-backup-not-made", why = t!("store-because-space")), Tone::Problem));
    snap("backup-not-made", &mut s);
    s.said.clear();
    s.said.insert("bk:back".into(), (refusal::nothing(t!("store-because-not-trindle")), Tone::Problem));
    s.behind.insert("bk:back".into(), vec!["/home/john/Downloads/holiday.fmpack".into()]);
    snap("backup-nothing-put-back", &mut s);
    s.said.clear();
    s.backup = pages::shots_support::backup_waiting();
    snap("backup-http-card", &mut s);

    let mut s = stocked();
    s.view = View::Sources;
    s.sources = pages::shots_support::source_form("http://apt.example.org/debian", "trixie", "https://apt.example.org/key.gpg", "example-tool");
    s.said.insert("src:custom".into(), (t!("store-src-bad-address").into(), Tone::Problem));
    snap("source-refused", &mut s);

    let events = Loop::new();
    let d = deb::Deb { name: "example-driver".into(), version: "1.2".into(), summary: "An example driver".into(),
                       about: "It shows how a package looks before it is installed. ".repeat(3), installed_kib: Some(48_000), maker: "John Smith".into(),
                       arch: "amd64".into() };
    let p = deb::made_up(&events, Some(Ok((d, deb::State::Service))), Some((origin::Origin::default(), origin::Verdict::Unknown)), None, false);
    paint("package-refused", || p.view(), deb::SIZE.0, deb::SIZE.1);

    shoot_plugin_failures();
}

#[test]
#[ignore = "makes the more pictures of Apps' pages for the help pages"]
fn shoot_apps_failures_more() {
    updates_failures();
    offline_and_problem_pictures();
    github_pictures();
    backup_pictures();
    package_pictures();
}

fn updates_failures() {
    let held_updates = || {
        let mut s = updates_page();
        s.held = held_one_of_each();
        s.upd = std::mem::take(&mut s.upd).opened_with(&s.held);
        s
    };
    snap("updates-found", &mut updates_page());
    snap("updates-held-back", &mut held_updates());
    let mut s = stocked();
    s.view = View::Updates;
    s.upd = update_page::made_up(&[], 0, false, &[], &[("apps", "Flathub could not be reached.")], &[]);
    snap("updates-could-not-check", &mut s);
    let mut s = stocked();
    s.view = View::Updates;
    s.upd = update_page::made_up(&[], 0, true, &[], &[], &[]);
    s.said.insert("up:system".into(), ("example-package-3 could not be set up.".into(), Tone::Problem));
    snap("updates-restart-and-a-failure", &mut s);
}

fn offline_and_problem_pictures() {
    let mut s = without_catalogue(false);
    s.view = View::Installed;
    snap("offline-installed", &mut s);
    let mut s = without_catalogue(false);
    s.view = View::GitHub;
    snap("offline-github", &mut s);

    let mut s = stocked();
    s.search = "photos".into();
    s.view = View::Search("photos".into());
    s.top = Some(("org.example.Nothing is not in the Flathub catalogue.".into(), Tone::Problem));
    snap("results-with-a-problem-said", &mut s);
    let mut all = with_every_kind(with_updates(stocked()));
    all.owned = pages::made_up::said(every_kind(0, ""), "deb:example-editor", "It could not be removed: John Smith's folder is in use.");
    snap("installed-remove-failed", &mut all);
}

fn github_pictures() {
    let found = |n: usize| github::Found { repo: format!("john-smith/example-{n}"), name: NAMES[n].into(), summary: LINES[n % LINES.len()].into(), stars: Some(120 * n as u64), ..Default::default() };
    let mut s = stocked();
    s.view = View::GitHub;
    s.github = pages::made_up::github("photos", (1..3).map(found).collect(), (3..5).map(found).collect(), true);
    snap("github-results", &mut s);
    s.github = pages::made_up::github("photos", Vec::new(), Vec::new(), true);
    let at = key_button_at(&mut s, 1278, 692);
    s.github = pages::made_up::github_key(std::mem::take(&mut s.github), Some(at), true, None);
    snap("github-key-stored-open", &mut s);
    s.github = pages::made_up::github("photos", Vec::new(), Vec::new(), true);
    s.github = pages::made_up::github_key(std::mem::take(&mut s.github), Some(at), false, None);
    snap("github-key-open", &mut s);
}

fn backup_pictures() {
    let mut s = stocked();
    s.view = View::Backup;
    s.backup = pages::made_up::backup();
    snap("move-your-apps", &mut s);
    s.said.insert("bk:back".into(), (refusal::nothing(t!("store-because-in-the-way")), Tone::Problem));
    s.behind.insert("bk:back".into(), vec!["/home/john/.config/example.conf".into(), "/home/john/.local/share/example".into()]);
    s.behind_open.insert("bk:back".into());
    snap("backup-behind-open", &mut s);
    s.said.insert("bk:back".into(), (t!("store-not-installed-failed", failed = "Notes, Clock"), Tone::Problem));
    s.behind_open.clear();
    s.behind.clear();
    snap("backup-not-installed", &mut s);
    let mut s = without_catalogue(false);
    s.view = View::Backup;
    s.backup = pages::made_up::backup();
    snap("offline-move-your-apps", &mut s);
}

fn package_pictures() {
    let events = Loop::new();
    let d = deb::Deb { name: "example".into(), version: "1.2".into(), summary: "An example program".into(),
                       about: "It shows how a package looks before it is installed. ".repeat(3), installed_kib: Some(48_000), maker: "John Smith".into(),
                       arch: "amd64".into() };
    let none = || Some((origin::Origin::default(), origin::Verdict::Unknown));
    let from = origin::Origin { site: Some("dl.example.com".into()), referrer: None };
    let p = deb::made_up(&events, Some(Ok((d.clone(), deb::State::New))), Some((from, origin::Verdict::Clear)), None, false);
    paint("package-new", || p.view(), deb::SIZE.0, deb::SIZE.1);
    let p = deb::made_up(&events, Some(Err(t!("store-deb-not-finished").into())), None, None, false);
    paint("package-not-a-package", || p.view(), deb::SIZE.0, deb::SIZE.1);
    let p = deb::made_up(&events, Some(Ok((d.clone(), deb::State::Trindle))), none(), None, false);
    paint("package-trindle", || p.view(), deb::SIZE.0, deb::SIZE.1);
    let p = deb::made_up(&events, Some(Ok((d, deb::State::Elsewhere("arm64".into())))), none(), None, false);
    paint("package-elsewhere", || p.view(), deb::SIZE.0, deb::SIZE.1);
}

#[test]
#[ignore = "makes the more pictures of the Plugins page for the help pages"]
fn shoot_plugin_failures_more() {
    let id = "org.example.Plugin2";
    let named = |name: &str, words: String, own: Option<&str>| snap(name, &mut plugins_page(&[(id, words.as_str())], own));
    let check = |why: String| t!("plugins-install-check", why = why);
    named("plugin-check-interface", check(t!("plugins-check-interface-missing", known = 1)), None);
    named("plugin-check-permission", check(t!("plugins-check-permission-unknown", name = "wallet", list = "network, notifications, background")), None);
    named("plugin-check-hooks", check(t!("plugins-hooks-nothing").into()), None);
    named("plugin-check-links", check(t!("plugins-links-outside", path = "bin/run")), None);
    snap("plugin-not-running-slow", &mut plugins_page(&[("org.example.Plugin0", t!("store-plugins-not-running", why = t!("plugins-bg-slow", n = 15)).as_str())], None));
    named("plugin-check-listen", check(t!("plugins-listen-needs-network").into()), None);
    named("plugin-swap-not-running", t!("plugins-swap-not-running", why = t!("plugins-install-hash"), not = t!("plugins-bg-no-session")), None);
    named("plugin-install-note", t!("plugins-install-note", why = "Permission denied (os error 13)"), None);
    named("plugin-hash", t!("plugins-install-hash").into(), None);
    named("plugin-copy-failed", t!("plugins-copy-failed").into(), None);
    snap("plugin-list-older", &mut plugins_page(&[], Some(&t!("plugins-catalogue-last-good", why = Refused::Older { is: 3, after: 5 }.words()))));
    snap("plugin-list-paused", &mut plugins_page(&[], Some(&Refused::NotRemembered.words())));
}
