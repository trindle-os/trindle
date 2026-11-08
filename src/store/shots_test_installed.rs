//! Tests: installed.

#[cfg(test)]
use super::*;

#[test]
fn every_kind_on_installed_is_one_row_with_its_buttons_in_the_same_places() {
    for across in [820, 1100, 1278] {
        let mut s = with_every_kind(with_updates(stocked()));
        let all = places(&mut s, across);
        let column = |start: &str| -> Vec<(String, Spot)> { named(&all, &[start]) };
        let (removes, abouts, opens) = (column("remove:"), column("about:"), column("open:"));
        assert_eq!((removes.len(), abouts.len(), opens.len()), (14, 14, 14), "{across}");
        for want in DEB_ROWS.iter().chain(&PLUGIN_ROWS) {
            assert!(removes.iter().any(|(id, _)| id == &format!("remove:{want}")), "{across}: {want} is listed");
        }
        let first = removes[0].1;
        let gap = crate::toolkit::theme::sheet().gap;
        let step = removes[1].1.y - first.y;
        for (i, ((remove, about), open)) in removes.iter().zip(&abouts).zip(&opens).enumerate() {
            let what = &remove.0;
            assert_eq!((remove.1.x, remove.1.w, remove.1.h), (first.x, first.w, first.h), "{across}: {what}");
            assert!((remove.1.y - (first.y + step * i as f32)).abs() < 0.01, "{across}: {what} is as tall as the others");
            assert_eq!((about.1.x, about.1.y, about.1.w, about.1.h), (first.x + first.w + gap, remove.1.y, first.w, first.h), "{across}: {what}, About");
            assert_eq!((open.1.x, open.1.y, open.1.w, open.1.h), (first.x + 2.0 * (first.w + gap), remove.1.y, first.w, first.h), "{across}: {what}, Open");
        }

        let updates = named(&all, &["up:app:", "pl:"]);
        assert_eq!(updates.iter().map(|(id, _)| id.as_str()).collect::<Vec<_>>(), ["up:app:org.example.App1", "pl:org.example.Plugin0"], "{across}");
        for ((_, button), row) in updates.iter().zip(["remove:org.example.App1", "remove:plugin:org.example.Plugin0"]) {
            let own = removes.iter().find(|(id, _)| id == row).expect("its row").1;
            assert_eq!((button.x, button.y, button.w, button.h), (first.x + 3.0 * (first.w + gap), own.y, first.w, first.h), "{across}: {row}");
        }
        let buttons = s.whole(settings()).buttons();
        let count = |tip: &str, on: bool| buttons.iter().filter(|b| **b == (tip.to_string(), on)).count();
        assert_eq!((count("About", true), count("About", false)), (9, 5), "{across}: only an app in the catalogue has a page");
        assert_eq!((count("Open", true), count("Open", false)), (10, 1), "{across}: a program with no launcher has nothing to start");
        assert_eq!(count("Open its settings", true), 3, "{across}");
        assert_eq!((count("Update Notes", true), count("Update Weather", true)), (1, 1), "{across}: {buttons:?}");
        let said = words(&mut s);
        assert!(said.contains(&"14 things are installed: 9 apps, 2 from .deb files and 3 plugins. 3 have updates waiting.".to_string()), "{across}: {said:?}");
        for line in ["A .deb file \u{b7} 48 MB \u{b7} version 2.4-1", "A .deb file \u{b7} version 1.0",
                     "Plugin \u{b7} 2 MB \u{b7} version 1.0 \u{b7} version 1.2 is waiting", "Plugin \u{b7} under 1 MB \u{b7} version 2.0", "Plugin \u{b7} version 0.3"] {
            assert!(said.contains(&line.to_string()), "{across}: {line:?} in {said:?}");
        }

        s.held = crate::common::held::Held::of(&[crate::common::held::Thing::plugin("org.example.Plugin0")]);
        let held = places(&mut s, across);
        assert_eq!(named(&held, &["hold:"]), [("hold:pl:org.example.Plugin0".to_string(), updates[1].1)], "{across}");
        assert_eq!(named(&held, &["remove:", "about:", "open:", screen::HEADLINE]), named(&all, &["remove:", "about:", "open:", screen::HEADLINE]), "{across}");
        let said = words(&mut s);
        assert!(said.contains(&"Plugin \u{b7} 2 MB \u{b7} version 1.0 \u{b7} held back; version 1.2 is waiting".to_string()), "{across}: {said:?}");
        assert!(said.iter().any(|w| w.ends_with("2 have updates waiting.")), "{across}: {said:?}");
        s.held = Default::default();
        s.other = Some(crate::common::updating::Who::Ring);
        assert_eq!(named(&places(&mut s, across), &["remove:", "about:", "open:", "up:app:", "pl:"]), named(&all, &["remove:", "about:", "open:", "up:app:", "pl:"]), "{across}");
        assert!(s.whole(settings()).buttons().contains(&("Update Weather".to_string(), false)), "{across}: one update at a time");
    }
}

#[test]
fn each_tab_of_installed_shows_its_kind_and_the_search_finds_every_kind() {
    let mut s = with_every_kind(with_updates(stocked()));
    assert_eq!(pages::installed::installed_tabs(&s), [("All", 14), ("Flathub", 6), ("Flatpak files", 2), ("AppImages", 1), ("Deb files", 2), ("Plugins", 3)]);
    let kind_of = |key: &str| if key.starts_with("deb:") { 4 } else if key.starts_with("plugin:") { 5 } else {
        match key.trim_start_matches("org.example.App").parse::<usize>().unwrap() % 5 { 3 => 2, 4 => 3, _ => 1 }
    };
    for (tab, (name, count)) in pages::installed::installed_tabs(&s).into_iter().enumerate() {
        s.owned = every_kind(tab, "");
        let shown = rows_shown(&mut s, 1278);
        assert_eq!(shown.len(), count, "{name}: {shown:?}");
        assert!(tab == 0 || shown.iter().all(|key| kind_of(key) == tab), "{name}: {shown:?}");
    }

    for (typed, found) in [("edit", vec!["deb:example-editor"]), ("smith", vec!["deb:example-tool"]), ("weather", vec!["plugin:org.example.Plugin0"]),
                           ("Plugin1", vec!["plugin:org.example.Plugin1"]), ("pho", vec!["org.example.App0"]),
                           ("example-", vec!["deb:example-editor", "deb:example-tool"])] {
        s.owned = every_kind(0, typed);
        assert_eq!(rows_shown(&mut s, 1278), found, "{typed}");
        let said = words(&mut s);
        assert!(said.contains(&format!("{} of 14 things match \u{201c}{typed}\u{201d}.", found.len())), "{typed}: {said:?}");
    }

    s.owned = every_kind(5, "t");
    assert_eq!(rows_shown(&mut s, 1278), ["plugin:org.example.Plugin2", "plugin:org.example.Plugin1", "plugin:org.example.Plugin0"], "Tides, Timer, Weather");
}

#[test]
fn remove_on_installed_asks_a_flatpak_by_card_and_the_others_twice_and_takes_each_its_own_way() {
    use updates::Removal;
    let kinds = [("org.example.App0", Removal::Flatpak { system: false, delete_data: false }, "org.example.App0"),
                 ("org.example.App3", Removal::Flatpak { system: false, delete_data: false }, "org.example.App3"),
                 ("org.example.App4", Removal::AppImage(PathBuf::from("/nonexistent/4.AppImage")), "org.example.App4"),
                 ("deb:example-editor", Removal::Deb, "example-editor"),
                 ("plugin:org.example.Plugin1", Removal::Plugin, "org.example.Plugin1")];
    for across in [820, 1278] {
        for (key, way, id) in &kinds {
            removal_by_its_own_way(across, key, way, id);
        }
    }
}

fn removal_by_its_own_way(across: u32, key: &str, way: &updates::Removal, id: &str) {
    use crate::toolkit::input::{Button, Input};
    use updates::Removal;
    let mut s = with_every_kind(with_updates(stocked()));
    let noted = Rc::new(std::cell::RefCell::new(Vec::new()));
    s.owned.removals = Some(noted.clone());
    s.across = across as f32;
    let mut ui = Ui::new(s.whole(settings()), Theme::new(false, "#fbc711"));
    let before = ui.places_at((across, 4000));
    let button = format!("remove:{key}");
    let at = named(&before, &[button.as_str()])[0].1;
    let (x, y) = (at.x + at.w / 2.0, at.y + at.h / 2.0);
    let now = std::time::Instant::now();
    let click = |ui: &mut Ui<Msg>| {
        ui.input(&Input::Press { button: Button::Left, x, y }, now);
        ui.input(&Input::Release { button: Button::Left, x, y }, now).messages
    };
    let sent = if matches!(way, Removal::Flatpak { .. }) {

        let opened = click(&mut ui);
        assert_eq!(opened.len(), 1, "{across}, {key}: the press opens the card");
        for msg in opened {
            s.update(msg);
        }
        assert!(noted.borrow().is_empty() && s.owned.ask.is_some(), "{across}, {key}: nothing is removed by the first press");
        let mut ui = Ui::new(s.whole(settings()), Theme::new(false, "#fbc711"));
        let keep = named(&ui.places_at((across, 4000)), &["remove-keep"])[0].1;
        let (x, y) = (keep.x + keep.w / 2.0, keep.y + keep.h / 2.0);
        ui.input(&Input::Press { button: Button::Left, x, y }, now);
        ui.input(&Input::Release { button: Button::Left, x, y }, now).messages
    } else {
        assert!(click(&mut ui).is_empty() && noted.borrow().is_empty(), "{across}, {key}: the first press only asks");
        click(&mut ui)
    };
    assert_eq!(sent.len(), 1, "{across}, {key}: the answer removes");
    for msg in sent {
        s.update(msg);
    }
    assert_eq!(*noted.borrow(), [(way.clone(), id.to_string())], "{across}, {key}: its own removal, once");

    let after = places(&mut s, across);
    let rows = named(&before, &["remove:"]);
    let step = rows[1].1.y - rows[0].1.y;
    let want: Vec<(String, Spot)> = rows.iter().filter(|(id, _)| *id != button)
        .map(|(id, r)| (id.clone(), if r.y > at.y { Spot { y: r.y - step, ..*r } } else { *r })).collect();
    assert_eq!(named(&after, &["remove:"]), want, "{across}, {key}: the list closed up and nothing else moved");

    let stands = |all: &[(String, Spot)]| -> Vec<(f32, f32, f32)> {
        named(all, &[screen::HEADLINE, pages::installed::INSTALLED_TABS]).iter().map(|(_, r)| (r.x, r.y, r.h)).collect()
    };
    assert_eq!(stands(&after), stands(&before), "{across}, {key}");
    let said = words(&mut s);
    assert!(said.iter().any(|w| w.starts_with("13 things are installed")), "{across}, {key}: {said:?}");

    s.view = View::Plugins;
    let plugins = s.whole(settings()).buttons();
    let gone = *way == Removal::Plugin;
    assert_eq!(plugins.contains(&("Install Timer".to_string(), true)), gone, "{across}, {key}: {plugins:?}");
    assert_eq!(words(&mut s).iter().any(|w| w.starts_with("2 plugins are installed")), gone, "{across}, {key}");
}

#[test]
fn installed_lists_every_kind_without_the_catalogue() {
    for across in [820, 1278] {
        let mut with = with_every_kind(with_updates(stocked()));
        let want = named(&places(&mut with, across), &["remove:", "about:", "open:", "up:app:", "pl:", pages::installed::INSTALLED_TABS]);
        for (how, s) in missing() {
            let mut s = with_every_kind(s);
            let said = words(&mut s);
            assert!(said.contains(&"14 things are installed: 9 apps, 2 from .deb files and 3 plugins. 3 have updates waiting.".to_string()), "{how}: {said:?}");
            for name in ["Editor", "John Smith's Tool", "Weather", "Timer", "Tides", "Photos"] {
                assert!(said.contains(&name.to_string()), "{how}: {name} is listed");
            }
            assert_eq!(named(&places(&mut s, across), &["remove:", "about:", "open:", "up:app:", "pl:", pages::installed::INSTALLED_TABS]), want, "{across}, {how}");
            let buttons = s.whole(settings()).buttons();
            let count = |tip: &str, on: bool| buttons.iter().filter(|b| **b == (tip.to_string(), on)).count();
            assert_eq!((count("About", true), count("About", false)), (0, 14), "{how}");
            assert_eq!((count("Remove", true), count("Open", true), count("Open its settings", true)), (14, 10, 3), "{how}: {buttons:?}");
        }
    }
}

#[test]
fn the_tabs_of_installed_are_a_strip_only_where_they_fit() {
    let strip = |s: &mut Store| !words(s).contains(&"dropdown: Show".to_string());
    let mut s = with_every_kind(with_updates(stocked()));

    let names: Vec<String> = pages::installed::installed_tabs(&s).iter().map(|(name, n)| format!("{name} {n}")).collect();
    let free = |words: Vec<String>| {
        use crate::toolkit::widget::{choice, row, space};
        let mut ui = Ui::new(row(vec![choice::<Msg>(words, 0, "Show", |_| go(|_| {})).key("strip"), space()]), Theme::new(false, "#fbc711"));
        named(&ui.places_at((4000, 200)), &["strip"])[0].1.w
    };
    let whole = free(names.clone());
    assert!(look::strip_room(&names) >= whole, "the room reckoned for the strip, {}, is not less than it takes, {whole}", look::strip_room(&names));

    for n in [0, 7, 88, 1234] {
        let other: Vec<String> = pages::installed::installed_tabs(&s).iter().map(|(name, _)| format!("{name} {n}")).collect();
        let takes = free(other.clone());
        assert!(look::strip_room(&other) >= takes && look::strip_room(&other) < takes * 1.25, "{n}: {} for {takes}", look::strip_room(&other));
    }
    let k = crate::toolkit::theme::sheet();
    let (mut strips, mut drops) = (0, 0);
    for across in (700..=1500).step_by(20) {
        let tabs = named(&places(&mut s, across), &[pages::installed::INSTALLED_TABS])[0].1;
        assert!(tabs.x + tabs.w <= across as f32 - k.page_padding - k.card_padding, "{across}: the tabs stay inside their card");
        if strip(&mut s) {
            strips += 1;
            assert_eq!(tabs.w, whole, "{across}: a strip shows every tab whole");
        } else {
            drops += 1;
        }
    }
    assert!(strips > 0 && drops > 0, "a strip where there is room ({strips}) and a dropdown where there is not ({drops})");
    assert!({ s.across = 820.0; !strip(&mut s) }, "at 820 six tabs do not fit, so they are a dropdown");
    assert!({ s.across = 1278.0; strip(&mut s) }, "at 1278 they are a strip");

    s.across = 1278.0;
    let tall = s.tall;
    assert!(Shows::resized(&mut s, 820.0, tall), "Installed is drawn again when the window's width changes");
}

#[test]
#[ignore]
fn shoot_ten_programs_with_no_icon() {
    std::fs::create_dir_all(folder()).unwrap();
    let names = ["Plain Notes", "Tide Tables", "Garden Log", "Bird Counter", "Sums", "Quiet Clock", "Recipe Box", "Star Chart", "Typing Tutor", "Radio Set"];
    for light in [false, true] {
        let mut s = store();
        s.view = View::Installed;
        let some: Vec<Installed> = names.iter().enumerate().map(|(i, name)| Installed {
            id: format!("org.example.Unknown{i}"), name: name.to_string(), version: "1.0".into(), size: "12 MB".into(),
            source: Source::Flatpak { origin: "flathub".into(), installation: "user".into() },
        }).collect();
        s.owned = pages::made_up::installed(Some(some), 0, "");
        s.across = 1278.0;
        s.tall = 692.0;
        let mut ui = Ui::new(s.whole(settings()), Theme::new(light, "#fbc711"));
        let h = (ui.natural_height(1278.0).ceil() as u32).clamp(692, 4000);
        let mut pix = tiny_skia::Pixmap::new(1278, h).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(folder().join(format!("390-apps-unknown-{}.png", if light { "light" } else { "dark" }))).unwrap();
    }
}

#[test]
fn every_button_apps_refuses_during_a_job_is_greyed_out_and_says_why() {
    let refused = |tip: &str, plugins: bool| ["Update", "Hold back", "Release", "Check", "Restart now"].iter().any(|w| tip.starts_with(w)) || (tip.starts_with("Install") && tip.contains("update"))
        || (plugins && tip.starts_with("Install"));
    let mut seen = 0;
    for state in ["idle", "one-update", "ring", "plugin-going", "checking"] {
        for (view, detail) in every_view().into_iter().map(|v| (v, false)).chain([(View::Plugins, false), (View::Installed, true)]) {
            let mut s = with_updates(stocked());
            s.plugins = plugin_page::made_up(true, vec![plugin(0, "1.0", &["network"]), plugin(1, "2.0", &[])],
                                             vec![offer(0, "1.2", &["network"]), offer(1, "2.0", &[])], &[], None, &[]);
            s.view = view.clone();
            if detail { s.detail = Some(app(1)); }
            s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
            match state {
                "one-update" => {
                    s.dials.insert(update_page::app_key("org.example.App4"), going(0.42));
                    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS).with_one("Notes");
                }
                "ring" => s.other = Some(crate::common::updating::Who::Ring),
                "plugin-going" => { s.dials.insert("pl:org.example.Plugin0".into(), going(0.4)); }
                "checking" => s.upd = update_page::made_up(&["apps"], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS),
                _ => {}
            }
            let plugins = matches!(view, View::Plugins);
            for (tip, off, why) in s.whole(settings()).shut() {
                if !refused(&tip, plugins) {
                    continue;
                }
                seen += 1;
                let waiting = update_page::busy(&s);
                assert_eq!((off, why.as_deref()), if waiting { (true, Some("Wait for the job to finish.")) } else { (false, None) },
                           "{state} / {}{}: {tip}", view.nav_key(), if detail { " detail" } else { "" });
            }
        }
    }
    assert!(seen > 50, "{seen}");
}

#[test]
fn browse_appimages_warns_before_add_and_says_where_each_comes_from() {
    let mut s = stocked();
    (s.view, s.hub, s.hub_asked) = (View::Browse, Some(Rc::new(listed())), true);
    let said = s.whole(settings()).said();
    let at = |w: &str| said.iter().position(|x| x.contains(w)).unwrap_or_else(|| panic!("{w} not in {said:?}"));
    assert!(at("nobody reviews or scans them") < at("button: Add to Trindle's apps"));
    let warning = "AppImageHub only lists projects: nobody reviews or scans them, and they need not be signed. \
                   Trindle runs each AppImage in a box with a private home folder: at first it may use the internet and sound, \
                   and sees none of your files.";
    assert!(said.iter().any(|w| w == warning), "the whole warning, in plain ink, before any Add: {said:?}");
    assert!(said.iter().any(|w| w == "2 AppImages are listed on AppImageHub."), "{said:?}");
    assert!(said.iter().any(|w| w.starts_with("On GitHub: john-smith/example")), "{said:?}");
    assert!(said.iter().any(|w| w.starts_with("Not on GitHub: from a website, example.org")), "{said:?}");
    at("Where an AppImage comes from is a hint, not a check.");
    assert_eq!(said.iter().filter(|w| *w == "button: Add to Trindle's apps").count(), 1, "a website's row has no Add");
    at("button: Open its website");
    assert!(!said.iter().any(|w| w.starts_with("Last updated")), "nothing is asked of GitHub before See details");

    let sound = categories().iter().position(|(c, _, _)| *c == "AudioVideo").unwrap() + 1;
    s.browse = browse::made_up(sound);
    let said = s.whole(settings()).said();
    assert!(said.iter().any(|w| w.starts_with("Not on GitHub")) && !said.iter().any(|w| w.starts_with("On GitHub")), "{said:?}");

    s.browse = browse::made_up(0);
    let project = github::Project { stars: Some(1200), pushed: Some(0), created: Some(0) };
    let release = github::Release { tag: "v1".into(), files: Vec::new(), published: Some(86_400) };
    s.github = pages::made_up::looked_at(std::mem::take(&mut s.github), "john-smith/example", project, Some(release));
    let said = s.whole(settings()).said();
    assert!(said.iter().any(|w| w.starts_with("On GitHub: john-smith/example, 1,200 stars")), "{said:?}");
    assert!(said.iter().any(|w| w.starts_with("Last updated 2 January 1970. Not updated for more than six months")), "{said:?}");
    assert!(said.iter().any(|w| w == "Created 1 January 1970"), "{said:?}");
}

#[test]
fn an_installed_appimage_says_where_it_came_from() {
    let mut s = stocked();
    s.view = View::Installed;
    let image = |n: &str| Installed { id: format!("trindle-{n}"), name: n.into(), version: String::new(), size: String::new(),
                                      source: Source::AppImage(PathBuf::from(format!("/nonexistent/{n}.AppImage"))) };
    let list = vec![image("one"), image("two"), image("three")];
    s.owned = pages::made_up::from(pages::made_up::installed(Some(list), 0, ""), vec![
        (PathBuf::from("/nonexistent/one.AppImage"), came_from::CameFrom::GitHub("john-smith/one".into())),
        (PathBuf::from("/nonexistent/two.AppImage"), came_from::CameFrom::Site("https://example.org/two".into())),
    ]);
    let said = s.whole(settings()).said();
    for w in ["On GitHub: john-smith/one", "Not on GitHub: from a website, example.org", "Source not known",
              "button: Open on GitHub", "button: Open its website", "Where an AppImage comes from is a hint, not a check."] {
        assert!(said.iter().any(|x| x.contains(w)), "{w} not in {said:?}");
    }
}
