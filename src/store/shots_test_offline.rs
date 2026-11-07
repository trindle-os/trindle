//! Tests.

#[cfg(test)]
use super::*;

#[test]
fn the_headline_is_one_height_whatever_it_says_at_every_width() {
    for across in [820, 1278, 1720] {
        let mut tall = None;
        let mut check = |what: &str, s: &mut Store| {
            let all = places(s, across);
            let line = named(&all, &[screen::HEADLINE]);
            assert_eq!(line.len(), 1, "{across}, {what}: the page has its one headline");
            let h = line[0].1.h;
            assert_eq!(*tall.get_or_insert(h), h, "{across}, {what}: the headline is as tall as on every other page");
            line[0].1
        };
        for view in every_view() {
            let mut s = with_updates(stocked());
            s.view = view.clone();
            check(view.nav_key().as_str(), &mut s);
        }
        let mut s = with_updates(stocked());
        s.view = View::Updates;
        s.plugins = plugin_page::made_up(true, vec![plugin(0, "1.0", &["network"])], vec![offer(0, "1.2", &["network"])], &[], None, &[]);
        s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
        let first = |s: &mut Store| named(&places(s, across), &["up:app:"])[0].1;
        let (line, card) = (check("found", &mut s), first(&mut s));
        let mut sentences = vec![words(&mut s)[..].iter().find(|w| w.contains("updates are ready")).cloned().unwrap()];
        let mut same = |what: &str, s: &mut Store| {
            assert_eq!(check(what, s), line, "{across}, {what}: the headline is where it was");
            assert_eq!(first(s), card, "{across}, {what}: the first row is where it was");
            sentences.push(words(s).into_iter().find(|w| w.ends_with('.') && w.len() > 20).unwrap_or_default());
        };
        s.held = held_one_of_each();
        same("held back", &mut s);
        s.online = false;
        same("held back, no internet", &mut s);
        s.online = true;
        s.other = Some(crate::common::updating::Who::Ring);
        same("the ring installing", &mut s);
        s.other = None;
        s.held = Default::default();
        s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS).with_one("A Very Long Name For An App That Goes On And On");
        s.dials.insert(update_page::app_key("org.example.App1"), going(0.42));
        same("one going in", &mut s);
        s.dials.clear();
        s.upd = update_page::made_up(&["apps", "system", "firmware"], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
        same("checking", &mut s);
        let lengths: Vec<usize> = sentences.iter().map(String::len).collect();
        assert!(lengths.iter().max().unwrap() > &(3 * lengths.iter().min().unwrap()), "{across}: short and long sentences were both tried: {sentences:?}");
    }
}

#[test]
fn only_the_pages_of_apps_to_choose_from_wait_for_the_catalogue() {
    for (how, mut s) in missing() {
        for view in every_view() {
            let needs = matches!(view, View::Home | View::Category(_) | View::Search(_));
            s.view = view.clone();
            assert_eq!(s.awaiting(), needs, "{how}: {}", view.nav_key());
            let said = words(&mut s);
            let stands_in = said.iter().any(|w| w.starts_with("Fetching the list of apps") || w == "Apps could not reach Flathub."
                                            || w.starts_with("This computer is not connected to the internet. Apps tries again"));
            assert_eq!(stands_in, needs, "{how}: {}", view.nav_key());
            if !needs {
                assert_eq!(s.page_key(), view.nav_key(), "{how}: the page is itself");
                continue;
            }
            assert_eq!(s.page_key(), "loading", "{how}");
            let works = said.iter().find(|w| w.contains("on the left, work without")).unwrap_or_else(|| panic!("{how}: says which pages work"));
            for page in pages::grid::open_without() {
                assert!(works.contains(page), "{how}: {page} is named");
                assert!(said.iter().any(|w| w == page || w.starts_with(&format!("{page} ("))), "{how}: {page} is in the side bar");
            }
            let title = s.grid_title();
            assert!(said.contains(&title), "{how}: {title}");
        }
    }
    let mut s = without_catalogue(false);
    s.view = View::GitHub;
    assert!(words(&mut s).contains(&pages::github_page::github_offline().to_string()));
    s.online = true;
    assert!(!words(&mut s).contains(&pages::github_page::github_offline().to_string()));
    let named: Vec<String> = pages::grid::open_without().iter().map(|p| p.to_string()).collect();
    assert_eq!(named, ["Installed", "Updates", "Plugins", "Software sources", super::side::move_apps()]);
}

#[test]
fn installed_shows_what_is_on_the_computer_without_the_catalogue() {
    for across in [820, 1278] {
        let mut with = with_updates(stocked());
        with.view = View::Installed;
        with.owned = pages::made_up::installed(Some(owned(9)), 0, "");
        with.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
        let want = places(&mut with, across);
        assert_eq!(named(&want, &["remove:"]).len(), 9);
        for (how, mut s) in missing() {
            s.view = View::Installed;
            let said = words(&mut s);
            assert!(said.contains(&"9 programs are installed, 6 from Flathub, 2 from Flatpak files and 1 AppImage. 2 have updates waiting.".to_string()),
                    "{how}: {said:?}");
            for i in 0..9 {
                assert!(said.contains(&app(i).name), "{how}: {} is listed", app(i).name);
            }
            assert!(said.iter().any(|w| w.contains("Flathub") && w.contains("45.2 MB") && w.contains("version 3.2.1")), "{how}");
            assert!(said.iter().any(|w| w.contains("version 3.3 is waiting")), "{how}: what Flatpak already knew is said");
            assert_eq!(places(&mut s, across), want, "{across}, {how}: every row is where it is with the catalogue");
            let buttons = s.whole(settings()).buttons();
            assert!(buttons.contains(&("Update Notes".to_string(), true)), "{how}: {buttons:?}");
            let was = s.page_key();
            s.arrived(catalogue());
            assert!(s.page_key() == was && s.view == View::Installed, "{how}: the page is not left for Discover");
            assert_eq!(places(&mut s, across), want, "{across}, {how}: the catalogue arriving moved something");
        }
    }
}

#[test]
fn updates_shows_what_is_known_without_the_catalogue() {
    const OWN: [&str; 6] = ["up:app:", "up:part:", "up:trindle", "up:debian", "fw:", "pl:"];
    for across in [820, 1278] {
        let mut with = with_updates(stocked());
        with.view = View::Updates;
        with.owned = pages::made_up::installed(Some(owned(9)), 0, "");
        with.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
        let want = named(&places(&mut with, across), &OWN);
        assert_eq!(want.len(), 6);
        for (how, mut s) in missing() {
            s.view = View::Updates;
            let said = words(&mut s);
            assert!(said.contains(&"Notes".to_string()) && said.contains(&"Clock".to_string()), "{how}: each app by its own name");
            assert!(said.contains(&"Trindle".to_string()) && said.contains(&"Example keyboard".to_string()) && said.contains(&"example-package-0".to_string()), "{how}");
            let headline = said.iter().find(|w| w.starts_with("16 updates are ready")).unwrap_or_else(|| panic!("{how}: {said:?}"));
            assert_eq!(headline.contains("not connected to the internet"), !s.online, "{how}: {headline}");
            assert!(!said.iter().any(|w| w.starts_with("Could not check")), "{how}");
            let got = named(&places(&mut s, across), &OWN);
            let by = got[0].1.y - want[0].1.y;
            assert_eq!(by, 0.0, "{across}, {how}: the page is not where it is with the catalogue");
            assert_eq!(got, lowered(&want, by), "{across}, {how}: every button is where it is with the catalogue");
            s.online = true;
            s.arrived(catalogue());
            assert!(s.page_key() == "updates" && s.view == View::Updates, "{how}: the page is not left for Discover");
            assert_eq!(named(&places(&mut s, across), &OWN), want, "{across}, {how}: arriving moved something");
        }
    }
}

#[test]
fn the_spec_says_what_every_page_does_without_the_internet() {
    let Ok(spec) = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/spec/trindle.md")) else { return };
    let part = spec.split_once("### Apps without the internet").expect("the spec has the part").1;
    let part = part.split_once("\n### ").map_or(part, |(p, _)| p);
    let s = store();
    let side = s.whole(settings()).said();
    let pages: Vec<&String> = side.iter().take_while(|w| *w != "Settings for apps")
        .chain(side.iter().skip_while(|w| *w != "Categories").take(categories().len() + 6))
        .filter(|w| !["", "Apps", "Categories", "More ways to get apps"].contains(&w.as_str())).collect();
    assert_eq!(pages.len(), 8 + categories().len(), "{pages:?}");
    for page in pages {
        let name = if categories().iter().any(|(_, n, _)| n == page) { "a category" } else { page.as_str() };
        assert!(part.contains(&format!("**{name}**")) || part.contains(&format!("**{}**", name.replace("a category", "A category"))),
                "the spec does not say what {page} does without the internet");
    }
    for other in ["**The pages from Settings**", "**A package's window**", "**An app's page**", "**Results**"] {
        assert!(part.contains(other), "{other}");
    }
}
