//! Tests.

#[cfg(test)]
use super::*;

#[test]
fn apps_reads_ahead_once_in_sight_again_on_a_change_and_never_out_of_sight() {
    let mut s = store();
    s.hidden = true;
    assert!(!s.preload());
    s.apps_changed();
    assert_eq!((s.preloaded, s.stale, s.read_aheads), (false, false, 0));
    s.back_in_sight();
    s.preload();
    s.preload();
    assert_eq!((s.preloaded, s.read_aheads), (true, 1));
    s.apps_changed();
    assert_eq!(s.read_aheads, 2);
    s.hidden = true;
    s.apps_changed();
    s.apps_changed();
    assert_eq!((s.read_aheads, s.stale), (2, true));
    s.back_in_sight();
    assert_eq!((s.read_aheads, s.stale, s.hidden), (3, false, false));
    s.back_in_sight();
    assert_eq!(s.read_aheads, 3, "nothing changed, so nothing is read");
    let watched = folders_that_change();
    for end in ["flatpak", "flatpak/app", "flatpak/overrides"] {
        assert_eq!(watched.iter().filter(|f| f.ends_with(end)).count(), 2, "{end}: {watched:?}");
    }
    assert!(watched.contains(&boxed::folders()) && watched.contains(&crate::plugins::folders()));
    let source = crate::common::source::code(include_str!("launch.rs"));
    let watch = source.split_once("fn watch_what_is_installed(").unwrap().1.split_once("\n}\n").unwrap().0;
    assert!(watch.contains("events.watch(") && !watch.contains(".every("));
    assert!(source.contains("if shown.window.seen() {\n                shown.change_if(Store::preload);"));
    let ahead = crate::common::source::code(include_str!("reading.rs"));
    let ahead = ahead.split_once("fn read_ahead(").unwrap().1.split_once("\n    }\n").unwrap().0;
    for read in ["pages::installed::installed_opened(self)", "plugin_page::opened_by_itself(self)", "pages::sources_page::sources_opened(self)", "h.preload()"] {
        assert!(ahead.contains(read), "{read} is read ahead");
    }
}

#[test]
#[ignore = "reads this computer and the network, to print how long each page's reading takes"]
fn time_what_each_page_reads() {
    use crate::common::access;
    use std::time::Instant;
    let ms = |t: Instant| t.elapsed().as_secs_f64() * 1000.0;
    let mut rows: Vec<(String, f64)> = Vec::new();
    let t = Instant::now();
    let apps = access::flatpak_apps();
    for id in &apps {
        let _ = access::flatpak_allowed(id);
    }
    rows.push((format!("Permissions: Flatpak asked about each of {} apps", apps.len()), ms(t)));
    let t = Instant::now();
    let named = access::flatpak_apps_named(&access::flatpak_places());
    rows.push((format!("Permissions: its {} rows by name, from Flatpak's folders", named.len()), ms(t)));
    let t = Instant::now();
    let _ = crate::common::run::text(&["flatpak", "list", "--app", "--columns=application,name,origin,installation,size,version"]);
    let _ = crate::icons::appimages::found();
    let _ = boxed::added::added();
    let _: Vec<u64> = std::fs::read_dir(crate::plugins::folders()).into_iter().flatten().flatten().map(|e| boxed::added::size_of_tree(&e.path())).collect();
    rows.push(("Installed: the list".into(), ms(t)));
    let t = Instant::now();
    let _ = crate::plugins::catalogue::gather();
    rows.push(("Plugins: the catalogue".into(), ms(t)));
    let t = Instant::now();
    let _ = pages::sources_page::read_sources();
    rows.push(("Software sources".into(), ms(t)));
    for id in ["default-apps", "startup", "plugins"] {
        let events = Loop::new();
        let asks = events.post(|_: crate::settings::ui::Do| {});
        let mut h = crate::settings::Hosted::new(&events, asks, None, None);
        let t = Instant::now();
        h.show(id);
        rows.push((format!("{id} (read as it opens)"), ms(t)));
    }
    for (what, took) in rows {
        println!("{took:>8.1} ms  {what}");
    }
}

#[test]
fn an_installed_row_held_back_has_release_where_its_update_was() {
    use crate::common::held::{Held, Thing};
    for across in [820, 1278] {
        let mut s = with_updates(stocked());
        s.view = View::Installed;
        s.owned = pages::made_up::installed(Some(owned(9)), 0, "");
        s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
        let before = places(&mut s, across);
        let update = named(&before, &["up:app:org.example.App1"])[0].1;
        assert!(words(&mut s).iter().any(|w| w.ends_with("2 have updates waiting.")));
        s.held = Held::of(&[Thing::app("org.example.App1")]);
        let after = places(&mut s, across);
        assert!(named(&after, &["up:app:org.example.App1"]).is_empty(), "{across}: no Update for it");
        assert_eq!(named(&after, &["hold:"]), [("hold:up:app:org.example.App1".to_string(), update)], "{across}: Release is where Update was");
        assert_eq!(named(&after, &["remove:", "up:app:org.example.App4", screen::HEADLINE]), named(&before, &["remove:", "up:app:org.example.App4", screen::HEADLINE]),
                   "{across}: nothing else moved");
        let said = words(&mut s);
        assert!(said.iter().any(|w| w.ends_with("held back; version 3.3 is waiting")), "{across}: {said:?}");
        assert!(said.iter().any(|w| w.ends_with("1 has an update waiting.")), "{across}: {said:?}");
        let buttons = s.whole(settings()).buttons();
        assert!(buttons.contains(&("Release Notes".to_string(), true)) && !buttons.iter().any(|(tip, _)| tip == "Update Notes"), "{across}: {buttons:?}");
        s.updatable.clear();
        s.upd = update_page::made_up(&[], 0, false, &[], &[], &[]);
        assert_eq!(named(&places(&mut s, across), &["hold:"]), [("hold:up:app:org.example.App1".to_string(), update)], "{across}");
        assert!(words(&mut s).iter().any(|w| w.ends_with("version 46.1 \u{00b7} held back")), "{across}: {:?}", words(&mut s));
        s.other = Some(crate::common::updating::Who::Ring);
        assert!(s.whole(settings()).buttons().contains(&("Release Notes".to_string(), false)));
    }
}

#[test]
fn an_app_s_page_held_back_has_release_where_update_would_be() {
    use crate::common::held::{Held, Thing};
    for across in [820, 1278] {
        let mut s = with_updates(stocked());
        let a = app(1);
        s.detail = Some(a.clone());
        s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
        const PLACES: [&str; 5] = [app_page::ACTION_PLACE, app_page::MAIN_ACTION, app_page::OPEN, app_page::WEBSITE, "remove:"];
        let before = named(&places(&mut s, across), &PLACES);
        assert_eq!(before.len(), 5, "{across}: {before:?}");
        assert!(s.whole(settings()).buttons().contains(&("Update".to_string(), true)));
        s.held = Held::of(&[Thing::app(&a.id)]);
        assert_eq!(named(&places(&mut s, across), &PLACES), before, "{across}: Release is where Update was, and no button has moved");
        let said = words(&mut s);
        assert!(said.iter().any(|w| w == "Held back at version 3.2.1; version 3.3 is waiting"), "{across}: {said:?}");
        let buttons = s.whole(settings()).buttons();
        assert!(buttons.contains(&("Release".to_string(), true)) && buttons.contains(&("Open".to_string(), true)), "{across}: {buttons:?}");
        assert!(!buttons.iter().any(|(tip, _)| tip == "Update"), "{across}: {buttons:?}");
        s.updatable.clear();
        s.upd = update_page::made_up(&[], 0, false, &[], &[], &[]);
        assert!(words(&mut s).iter().any(|w| w == "Installed, held back at version 3.2.1"), "{across}");
        assert!(s.whole(settings()).buttons().contains(&("Release".to_string(), true)));
        s.other = Some(crate::common::updating::Who::Ring);
        assert!(s.whole(settings()).buttons().contains(&("Release".to_string(), false)), "{across}: off while updates go in elsewhere");
    }
}

#[test]
fn a_plugin_held_back_has_release_where_its_update_was() {
    use crate::common::held::{Held, Thing};
    for across in [820, 1278] {
        let mut s = stocked();
        s.view = View::Plugins;
        s.plugins = plugin_page::made_up(true, vec![plugin(0, "1.0", &["network"]), plugin(1, "2.0", &[])],
                                         vec![offer(0, "1.2", &["network"]), offer(1, "2.0", &[])], &[], None, &[]);
        let before = places(&mut s, across);
        let update = named(&before, &["pl:org.example.Plugin0"])[0].1;
        assert!(words(&mut s).contains(&"2 plugins are installed, and 1 has an update.".to_string()));
        s.held = Held::of(&[Thing::plugin("org.example.Plugin0")]);
        let after = places(&mut s, across);
        assert_eq!(named(&after, &["hold:"]), [("hold:pl:org.example.Plugin0".to_string(), update)], "{across}");
        assert_eq!(named(&after, &["plugin:", "pl-remove:", screen::HEADLINE]), named(&before, &["plugin:", "pl-remove:", screen::HEADLINE]), "{across}: no card moved");
        let said = words(&mut s);
        assert!(said.contains(&"2 plugins are installed; 1 update is held back.".to_string()), "{across}: {said:?}");
        assert!(said.contains(&"Held back at version 1.0; version 1.2 is waiting.".to_string()), "{across}: {said:?}");
        let buttons = s.whole(settings()).buttons();
        assert!(buttons.contains(&("Release Weather".to_string(), true)) && !buttons.iter().any(|(tip, _)| tip.starts_with("Update to")), "{across}: {buttons:?}");
    }
}
