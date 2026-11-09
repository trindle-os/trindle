//! Tests and pictures: Flatpaks and AppImages side by side on Discover, every kind and results, with the selector.

#[cfg(test)]
use super::*;
use crate::store::formats::Show;
use crate::store::github::Found;
use crate::store::came_from::CameFrom;
use crate::toolkit::t;
use crate::toolkit::widget::Node;
use crate::store::pages::{appimage_warning, grid::SELECTOR, mixed::{self, Tile}};

fn found(name: &str, categories: &[&str]) -> Found {
    Found { name: name.into(), repo: format!("john-smith/{}", name.to_lowercase().replace(' ', "-")), summary: format!("{name} does a great deal"),
            categories: categories.iter().map(|c| c.to_string()).collect(), listed: true, ..Found::default() }
}

fn feed() -> Vec<Found> {
    let code = categories()[0].0;
    vec![found("Zebra Notes", &[code]), found("Photos Plus", &[code]), found("VLC", &[code]), found("Obsidian", &["Office"]),
         found("KeePassXC", &["Utility"]), found("Alpha Paint", &["Graphics"])]
}

fn mixed_store(name: &str) -> Store {
    let mut s = stocked();
    s.hub = Some(Rc::new(feed()));
    s.hub_asked = true;
    s.show = Show::Both;
    s.warning_off = false;
    s.conf = std::env::temp_dir().join(format!("trindle-mixed-{}-{name}", std::process::id())).join("trindle.conf");
    s
}

fn kinds(s: &Store) -> Vec<(bool, String)> {
    mixed::tiles(s).into_iter().map(|t| match t {
        Tile::Flat(a) => (true, a.name.clone()),
        Tile::Image(f) => (false, f.name.clone()),
    }).collect()
}

fn views() -> [View; 3] {
    [View::Home, View::Category(categories()[0].0.to_string()), View::Search("photos".into())]
}

#[test]
fn the_selector_filters_the_one_grid_on_discover_a_kind_and_results() {
    let mut s = mixed_store("filter");
    for view in views() {
        s.view = view.clone();
        for (show, flat, image) in [(Show::Both, true, true), (Show::Flatpaks, true, false), (Show::AppImages, false, true)] {
            s.show = show;
            let tiles = kinds(&s);
            let name = view.nav_key();
            assert_eq!(tiles.iter().any(|t| t.0), flat, "{name} {show:?}: Flatpaks");
            assert_eq!(tiles.iter().any(|t| !t.0), image, "{name} {show:?}: AppImages");
            assert_eq!(named(&places(&mut s, 1278), &[SELECTOR]).len(), 1, "{name} {show:?}: the selector is by the title");
        }
    }

    s.view = View::Installed;
    assert!(named(&places(&mut s, 1278), &[SELECTOR]).is_empty());
}

#[test]
fn both_kinds_share_one_order() {
    let mut s = mixed_store("order");
    s.view = View::Home;
    let home = kinds(&s);
    assert!(home[..2].iter().any(|t| t.0) && home[..2].iter().any(|t| !t.0), "Discover: both kinds from the start");
    s.view = views()[1].clone();
    let names: Vec<String> = kinds(&s).into_iter().map(|t| t.1.to_lowercase()).collect();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted, "a kind: one order by name");
    assert!(kinds(&s).iter().any(|t| t.0) && kinds(&s).iter().any(|t| !t.0));
    s.view = views()[2].clone();
    let found = kinds(&s);
    let starts: Vec<bool> = found.iter().map(|t| t.1.to_lowercase().starts_with("photos")).collect();
    assert!(starts.iter().any(|b| *b) && starts.windows(2).all(|w| w[0] || !w[1]), "results: names that start with the word come first: {found:?}");
    assert!(found.iter().any(|t| !t.0 && t.1 == "Photos Plus"), "an AppImage among them");
}

#[test]
fn discover_has_the_popular_appimages_and_the_rest_the_whole_feed() {
    let mut s = mixed_store("popular");
    s.show = Show::AppImages;
    s.view = View::Home;
    let home: Vec<String> = kinds(&s).into_iter().map(|t| t.1).collect();
    assert_eq!(home, ["Obsidian", "VLC", "KeePassXC"], "the popular list's order; names the feed lacks are left out");
    s.view = View::Category(categories()[0].0.to_string());
    let kind: Vec<String> = kinds(&s).into_iter().map(|t| t.1).collect();
    assert_eq!(kind, ["Photos Plus", "VLC", "Zebra Notes"], "the whole feed, in the kind");
    s.view = View::Search("zebra".into());
    assert_eq!(kinds(&s).len(), 1, "and in a search");

    s.hub = None;
    s.view = View::Home;
    assert!(kinds(&s).is_empty());
}

#[test]
fn the_choice_is_one_setting_for_all_three_pages_and_is_kept() {
    let mut s = mixed_store("kept");
    assert_eq!(crate::store::formats::show_in(&s.conf), Show::Both, "starts on Both");
    s.view = View::Home;
    mixed::set_show(&mut s, Show::AppImages);
    assert_eq!(crate::store::formats::show_in(&s.conf), Show::AppImages, "kept in the file");
    for view in views() {
        s.view = view.clone();
        assert!(!kinds(&s).is_empty() && kinds(&s).iter().all(|t| !t.0), "{}: the choice made on Discover holds", view.nav_key());
    }
    s.view = views()[2].clone();
    mixed::set_show(&mut s, Show::Flatpaks);
    for view in views() {
        s.view = view.clone();
        assert!(kinds(&s).iter().all(|t| t.0), "{}: and the choice made on results", view.nav_key());
    }
    assert_eq!(crate::store::formats::show_in(&s.conf), Show::Flatpaks);
}

#[test]
fn offline_the_flatpaks_show_with_one_line_saying_why_no_appimages_are() {
    let offline = t!("store-browse-offline");
    let silent = t!("store-browse-failed");
    for (online, line) in [(false, offline), (true, silent)] {
        for view in views() {
            let mut s = mixed_store("offline");
            (s.hub, s.online, s.view) = (Some(Rc::new(Vec::new())), online, view.clone());
            let said = words(&mut s);
            assert_eq!(said.iter().filter(|w| w.as_str() == line).count(), 1, "{} online {online}: one line", view.nav_key());
            assert!(!kinds(&s).is_empty() && kinds(&s).iter().all(|t| t.0), "the Flatpaks show as they do");
            s.show = Show::Flatpaks;
            assert!(!words(&mut s).iter().any(|w| w == line), "Flatpaks only: nothing said of AppImages");
            s.show = Show::AppImages;
            let said = words(&mut s);
            assert_eq!(said.iter().filter(|w| w.as_str() == line).count(), 1, "AppImages only: the page's sentence, once");
            assert!(kinds(&s).is_empty());
        }
    }

    let mut s = mixed_store("fetching");
    (s.hub, s.show) = (None, Show::AppImages);
    assert!(words(&mut s).iter().any(|w| w.as_str() == t!("store-browse-fetching")));
}

#[test]
fn the_warning_comes_every_time_until_its_tick_is_set() {
    use crate::toolkit::window::{with_named, with_press};
    use std::cell::Cell;
    let mut s = mixed_store("warning");
    let installed = Rc::new(Cell::new(0));
    let try_install = |s: &mut Store| { let n = installed.clone(); appimage_warning::before(s, move |_| n.set(n.get() + 1)); };
    let press = |s: &mut Store, tip: &str| {
        let over = appimage_warning::over(s);
        let msg = over.iter().flat_map(|n| n.presses()).find(|(t, _, _)| t == tip).map(|(_, _, m)| m.clone()).unwrap_or_else(|| panic!("no {tip} on the card"));
        with_named(true, Some(7), || with_press(Some(7), || (msg.0)(s)));
    };
    try_install(&mut s);
    assert!(appimage_warning::showing(&s) && installed.get() == 0, "the card first, nothing yet");
    let said: Vec<String> = appimage_warning::over(&s).iter().flat_map(|n| n.said()).collect();
    for part in [t!("store-appimage-warn-head"), t!("store-appimage-warn-box"), "tick: Don't show again", "button: OK", "button: Cancel"] {
        assert!(said.iter().any(|w| w == part), "the card says {part:?}: {said:?}");
    }
    press(&mut s, "Cancel");
    assert!(!appimage_warning::showing(&s) && installed.get() == 0, "Cancel installs nothing");
    try_install(&mut s);
    assert!(appimage_warning::showing(&s), "and it comes again");
    press(&mut s, "OK");
    assert!(!appimage_warning::showing(&s) && installed.get() == 1, "OK installs");
    assert!(!s.warning_off && !crate::store::formats::warning_off_in(&s.conf), "not ticked: still wanted");
    try_install(&mut s);
    assert!(appimage_warning::showing(&s), "so it comes again");
    appimage_warning::ticked(&mut s, true);
    press(&mut s, "OK");
    assert_eq!(installed.get(), 2);
    assert!(s.warning_off && crate::store::formats::warning_off_in(&s.conf), "the tick is kept");
    try_install(&mut s);
    assert!(!appimage_warning::showing(&s) && installed.get() == 3, "ticked: installs at once, no card");

    let mut other = mixed_store("warning-cancel");
    try_install(&mut other);
    appimage_warning::ticked(&mut other, true);
    press(&mut other, "Cancel");
    assert!(!other.warning_off && !crate::store::formats::warning_off_in(&other.conf));
}

#[test]
fn the_card_is_over_the_page_and_nothing_touches() {
    let mut s = mixed_store("layer");
    s.view = View::Home;
    let before = words(&mut s).len();
    appimage_warning::before(&mut s, |_| {});
    for w in [760, 1278, 1920] {
        s.across = w as f32;
        let mut ui = Ui::new(s.whole(settings()), Theme::new(false, "#fbc711"));
        ui.lay_at(w, 692);
        let wrong = ui.squeezed_or_touching();
        assert!(wrong.is_empty(), "at {w}: {wrong:?}");
    }
    assert!(words(&mut s).len() > before, "the card is added to the page");
}

fn installed_image(name: &str, file: &str, from: CameFrom) -> (Installed, CameFrom) {
    let path = std::path::PathBuf::from(format!("/home/test/.appimage/{file}"));
    (Installed { id: name.to_lowercase(), name: name.into(), version: String::new(), size: "120 MB".into(), source: Source::AppImage(path) }, from)
}

fn with_installed(s: &mut Store, items: Vec<(Installed, CameFrom)>) {
    let origins = items.iter().filter_map(|(i, f)| match &i.source {
        Source::AppImage(p) => Some((p.clone(), f.clone())),
        _ => None,
    }).collect();
    s.owned = pages::made_up::from(pages::made_up::installed(Some(items.into_iter().map(|i| i.0).collect()), 0, ""), origins);
}

fn tile_key(name: &str) -> String {
    format!("{}:gh:john-smith/{}", listing::TILE, name.to_lowercase().replace(' ', "-"))
}

fn card_press(s: &mut Store, find: impl Fn(&Node<Msg>) -> bool) -> Msg {
    use crate::toolkit::widget::What;
    fn walk(n: &Node<Msg>, find: &dyn Fn(&Node<Msg>) -> bool) -> Option<Msg> {
        if let What::Card(_, Some(m)) = &n.what {
            if find(n) {
                return Some(m.clone());
            }
        }
        n.items().iter().find_map(|i| walk(i, find))
    }
    walk(&s.whole(settings()), &find).expect("the card is on the page")
}

fn press(s: &mut Store, msg: &Msg) {
    use crate::toolkit::window::{with_named, with_press};
    with_named(true, Some(7), || with_press(Some(7), || (msg.0)(s)));
}

fn button_press(s: &mut Store, tip: &str) -> Option<Msg> {
    s.whole(settings()).presses().into_iter().find(|(t, off, _)| t == tip && !off).map(|(_, _, m)| m.clone())
}

fn pixels(s: &mut Store, w: u32, h: u32) -> tiny_skia::Pixmap {
    s.across = w as f32;
    let mut ui = Ui::new(s.whole(settings()), Theme::new(false, "#fbc711"));
    let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    pix
}

#[test]
fn discover_takes_one_of_each_in_turn() {
    let mut s = mixed_store("turns");
    s.view = View::Home;
    let all = kinds(&s);
    let images = all.iter().filter(|t| !t.0).count();
    assert_eq!(images, 3);
    assert!(all.iter().filter(|t| t.0).count() > images, "more Flatpaks than AppImages");
    for (n, t) in all.iter().take(2 * images).enumerate() {
        assert_eq!(t.0, n % 2 == 0, "place {n} of {all:?}");
    }
    assert!(all[2 * images..].iter().all(|t| t.0), "the Flatpaks go on alone");

    let mut few = (*s.apps).clone();
    few.retain(|id, _| !POPULAR.contains(&id.as_str()) || id == POPULAR[0]);
    s.apps = Rc::new(few);
    let all = kinds(&s);
    assert_eq!(all.iter().map(|t| t.0).collect::<Vec<_>>(), [true, false, false, false], "one Flatpak, then the AppImages on alone");

    s.show = Show::AppImages;
    assert_eq!(kinds(&s).len(), 3);
}

#[test]
fn the_headline_counts_what_is_chosen_and_names_where() {
    let count = |n: usize| t!("store-grid-apps-count", n = n, count = crate::common::thousands(n as u64));
    let said = |s: &mut Store, starts: &str| words(s).iter().filter(|w| w.starts_with(starts)).count();
    let mut s = mixed_store("headline");
    let (flat, listed) = (s.apps.len(), s.hub.as_ref().unwrap().len());
    s.view = View::Home;
    for (show, want) in [(Show::Both, format!("{} to choose from on Flathub and AppImageHub; ", count(flat + listed))),
                         (Show::Flatpaks, format!("{} to choose from on Flathub; ", count(flat))),
                         (Show::AppImages, format!("{} to choose from on AppImageHub.", count(listed)))] {
        s.show = show;
        assert_eq!(said(&mut s, &want), 1, "{show:?}: {want}");
        assert_eq!(words(&mut s).iter().filter(|w| w.contains(" to choose from")).count(), 1, "{show:?}: one headline");
    }

    s.show = Show::Both;
    s.hub = None;
    assert_eq!(said(&mut s, &format!("{} to choose from on Flathub; ", count(flat))), 1);
    s.hub = Some(Rc::new(feed()));

    for view in [views()[1].clone(), views()[2].clone()] {
        s.view = view.clone();
        for show in Show::ALL {
            s.show = show;
            let n = kinds(&s).len();
            assert!(n > 0, "{} {show:?}", view.nav_key());
            assert_eq!(said(&mut s, &format!("{}.", count(n))), 1, "{} {show:?}: {n}", view.nav_key());
        }
    }

    s.view = views()[1].clone();
    let by = |s: &mut Store, show| { s.show = show; kinds(s).len() };
    let (both, flatpaks, appimages) = (by(&mut s, Show::Both), by(&mut s, Show::Flatpaks), by(&mut s, Show::AppImages));
    assert!(flatpaks > 0 && appimages > 0 && both == flatpaks + appimages);
}

#[test]
fn an_appimage_tile_opens_its_page_and_back_returns_to_where_it_was() {
    for view in views() {
        let mut s = mixed_store("opens");
        s.view = view.clone();
        crate::store::github::asked_so_far();
        let grid = pixels(&mut s, 1278, 692);
        assert!(crate::store::github::asked_so_far().is_empty(), "{}: drawing a grid asks GitHub nothing", view.nav_key());
        let name = kinds(&s).into_iter().find(|t| !t.0).expect("an AppImage on the page").1;
        let press_tile = card_press(&mut s, |n| n.key.as_deref() == Some(&tile_key(&name)));
        press(&mut s, &press_tile);
        let mut asked = crate::store::github::asked_so_far();
        asked.sort();
        let repo = format!("/repos/john-smith/{}", name.to_lowercase().replace(' ', "-"));
        assert_eq!(asked, [repo.clone(), format!("{repo}/releases?per_page=5")], "{}: two questions, on opening", view.nav_key());
        assert_eq!(s.image.as_ref().map(|f| f.name.as_str()), Some(name.as_str()));
        assert!(s.view == view, "the place it was opened from is still the view");
        let said = words(&mut s);
        assert!(said.contains(&name) && said.iter().any(|w| w == "Facts") && said.iter().any(|w| w == "About"), "{said:?}");
        assert!(said.iter().any(|w| w == &t!("store-back-results").to_string() || w == &pages::links::back_name(&view)), "the way back names the place");

        let (page, side) = (pixels(&mut s, 1278, 692), crate::store::side::side_width() as u32);
        assert!(page.data().chunks(1278 * 4).zip(grid.data().chunks(1278 * 4)).all(|(a, b)| a[..side as usize * 4] == b[..side as usize * 4]),
                "{}: the side bar looks as it did", view.nav_key());

        let back = pages::links::back_name(&view);
        let leave = card_press(&mut s, |n| n.said() == [back.clone()]);
        press(&mut s, &leave);
        assert!(s.image.is_none() && s.view == view);
        assert!(words(&mut s).iter().all(|w| w != "Facts"));
    }

    let mut s = mixed_store("leaves");
    s.view = View::Home;
    pages::image_page::open(&mut s, found("Photos Plus", &[]));
    s.select_nav("installed");
    assert!(s.image.is_none() && s.view == View::Installed);
    pages::image_page::open(&mut s, found("Photos Plus", &[]));
    s.typed("zebra".into());
    assert!(s.image.is_none() && matches!(s.view, View::Search(_)));

    crate::store::github::asked_so_far();
    pages::image_page::open(&mut s, Found { name: "Elsewhere".into(), site: "https://example.org/get".into(), listed: true, ..Found::default() });
    assert!(crate::store::github::asked_so_far().is_empty());
}

fn answered(s: &mut Store, f: &Found) {
    let release = github::Release { tag: "v1.2".into(), published: Some(1_780_000_000), files: vec![
        github::Download { name: format!("{}-1.2-x86_64.AppImage", f.name), url: "https://example.org/a".into(), size: 123_000_000, kind: github::Kind::AppImage },
        github::Download { name: format!("{}.flatpak", f.name), url: "https://example.org/f".into(), size: 98_000_000, kind: github::Kind::Flatpak }] };
    let project = github::Project { stars: Some(4210), pushed: Some(1_780_100_000), created: Some(1_500_000_000) };
    s.github = pages::made_up::looked_at(pages::GitHubPage::default(), &f.repo, project, Some(release));
}

#[test]
fn an_appimage_page_keeps_its_places_in_every_state() {
    use crate::store::app_page::{ACTION_PLACE, FACTS_PLACE, MAIN_ACTION};
    use crate::store::pages::image_page::MAIN_LINK;
    let f = found("Photos Plus", &[]);
    let site = Found { name: "Elsewhere".into(), site: "https://example.org/get".into(), listed: true, ..Found::default() };
    for across in [820u32, 1278, 1720] {
        let mut s = mixed_store("places");
        s.image = Some(f.clone());
        let held = [ACTION_PLACE, FACTS_PLACE];
        let first = places(&mut s, across);
        let want: Vec<Spot> = held.iter().map(|k| first.iter().find(|(id, _)| id == k).unwrap_or_else(|| panic!("{k}")).1).collect();
        let mut states: Vec<Store> = Vec::new();
        for step in 0..5 {
            let mut t = mixed_store("places");
            t.image = Some(f.clone());
            match step {
                0 => answered(&mut t, &f),
                1 => {
                    answered(&mut t, &f);
                    t.dials.insert("gh:john-smith/photos-plus".into(), Dial::default());
                }
                2 => {
                    answered(&mut t, &f);
                    let mut d = Dial::default();
                    d.finish(true, "");
                    t.dials.insert("gh:john-smith/photos-plus".into(), d);
                }
                3 => {
                    answered(&mut t, &f);
                    with_installed(&mut t, vec![installed_image("Photos Plus", "Photos_Plus-1.2.AppImage", CameFrom::GitHub("john-smith/photos-plus".into()))]);
                }
                _ => {
                    t.said.insert("gh:john-smith/photos-plus".into(), ("The release could not be fetched. ".repeat(5), Tone::Problem));
                }
            }
            states.push(t);
        }
        for (n, t) in states.iter_mut().enumerate() {
            let all = places(t, across);
            for (k, w) in held.iter().zip(&want) {
                assert_eq!(all.iter().find(|(id, _)| id == k).unwrap().1, *w, "{across} wide, state {n}: {k}");
            }
            let pill = all.iter().find(|(id, _)| id == MAIN_ACTION).map(|(_, r)| *r);
            if n == 1 || n == 2 {
                assert!(pill.is_none(), "{n}: the ring or the spinner stands in the pill's room");
            } else {
                assert_eq!(pill, Some(want[0]), "{across} wide, state {n}: the pill fills the room");
            }
        }

        let mut t = mixed_store("places");
        t.image = Some(site.clone());
        let all = places(&mut t, across);
        assert!(all.iter().all(|(id, _)| id != MAIN_ACTION), "no pill without GitHub");
        let tips: Vec<String> = t.whole(settings()).presses().into_iter().map(|(tip, _, _)| tip).collect();
        assert!(tips.contains(&t!("store-from-site-open").to_string()) && !tips.iter().any(|tip| tip.starts_with("Install")), "{tips:?}");
        let link = all.iter().find(|(id, _)| id == MAIN_LINK).map(|(_, r)| *r).expect("the link is in the main room");
        assert!(link.x >= want[0].x && link.x + link.w <= want[0].x + want[0].w + 0.5 && link.y >= want[0].y, "{across}: inside the main room");
    }
}

#[test]
fn an_installed_appimage_shows_open_on_its_tile_and_its_page() {
    use crate::store::pages::image_tile::{standing, started_so_far, Standing};
    let mut s = mixed_store("installed");
    s.view = View::Home;
    let obsidian = found("Obsidian", &[]);
    let file = std::path::PathBuf::from("/home/test/.appimage/Obsidian-1.5-x86_64.AppImage");
    assert_eq!(standing(&s, &obsidian), Standing::Absent);
    assert!(button_press(&mut s, "Install Obsidian").is_some() && button_press(&mut s, "Open Obsidian").is_none());
    with_installed(&mut s, vec![
        installed_image("Obsidian", "Obsidian-1.5-x86_64.AppImage", CameFrom::GitHub("John-Smith/obsidian".into())),
        installed_image("VLC", "VLC-3.0.AppImage", CameFrom::GitHub("somebody-else/vlc".into())),
        installed_image("Keepassxc", "KeePassXC-2.7.AppImage", CameFrom::NotKnown),
    ]);
    assert_eq!(standing(&s, &obsidian), Standing::Here(file.clone()), "the same project, in any case");
    assert_eq!(standing(&s, &found("VLC", &[])), Standing::Absent, "another project of the same name is not it");
    assert!(matches!(standing(&s, &found("KeePassXC", &[])), Standing::Here(_)), "one added by hand, by its name");
    assert_eq!(standing(&s, &Found { name: "Elsewhere".into(), site: "https://example.org/get".into(), ..Found::default() }), Standing::Absent);

    assert!(button_press(&mut s, "Open Obsidian").is_some() && button_press(&mut s, "Install Obsidian").is_none());
    assert!(button_press(&mut s, "Install VLC").is_some() && button_press(&mut s, "Open VLC").is_none());
    started_so_far();
    let open = button_press(&mut s, "Open Obsidian").unwrap();
    (open.0)(&mut s);
    assert_eq!(started_so_far(), std::slice::from_ref(&file), "the tile's Open starts the file Installed lists");

    answered(&mut s, &obsidian);
    s.image = Some(obsidian.clone());
    let tips: Vec<String> = s.whole(settings()).presses().into_iter().map(|(tip, _, _)| tip).collect();
    assert!(tips.iter().any(|t| t == "Open") && !tips.iter().any(|t| t == "Install"), "{tips:?}");
    assert!(words(&mut s).iter().any(|w| w == "Installed"), "and says it is installed");
    (button_press(&mut s, "Open").unwrap().0)(&mut s);
    assert_eq!(started_so_far(), std::slice::from_ref(&file), "the page's Open too");

    s.image = None;
    s.view = View::Installed;
    let opens: Vec<Msg> = s.whole(settings()).presses().into_iter().filter(|(t, off, _)| t == t!("store-app-open-round") && !off).map(|(_, _, m)| m.clone()).collect();
    assert!(!opens.is_empty(), "Installed has Open");
    for m in opens {
        (m.0)(&mut s);
    }
    assert!(started_so_far().contains(&file), "Installed starts it the same way");
}

#[test]
fn the_page_s_install_shows_the_warning_first() {
    let mut s = mixed_store("page-warning");
    let f = found("Photos Plus", &[]);
    answered(&mut s, &f);
    s.image = Some(f);
    let install = button_press(&mut s, "Install").expect("the page has Install");
    press(&mut s, &install);
    assert!(appimage_warning::showing(&s), "the card comes first");
    assert!(s.dials.is_empty() && s.working == 0, "nothing started");
}

#[test]
fn a_tile_in_the_middle_of_a_mixed_grid_behaves_while_it_goes_in() {
    let key = "gh:john-smith/vlc";
    let action = format!("{}:{key}", listing::TILE_ACTION);
    let mut s = mixed_store("going");
    s.view = View::Home;
    let before = places(&mut s, 1278);
    let tiles = |all: &[(String, Spot)]| -> Vec<(String, Spot)> { all.iter().filter(|(id, _)| id.starts_with("apps-tile")).cloned().collect() };
    assert!(tiles(&before).iter().any(|(id, _)| *id == action), "the tile's action is keyed for its place");
    let order: Vec<bool> = kinds(&s).iter().map(|t| t.0).collect();
    let at = kinds(&s).iter().position(|t| t.1 == "VLC").unwrap();
    assert!(order[..at].iter().any(|f| *f) && order[at..].iter().any(|f| *f), "VLC is in the middle of the Flatpaks");
    s.dials.insert(key.into(), Dial::default());
    assert!(s.moving(), "the window keeps moving the ring");
    assert!(button_press(&mut s, "Install VLC").is_none(), "no second Install while it goes in");
    assert!(button_press(&mut s, "Install Obsidian").is_some(), "the others are as they were");
    let during = places(&mut s, 1278);
    for (id, r) in tiles(&before) {
        assert_eq!(during.iter().find(|(i, _)| *i == id).map(|(_, r)| *r), Some(r), "{id} has not moved");
    }
    assert!(during.iter().any(|(id, _)| *id == action), "and its button is where it was");

    s.dials.get_mut(key).unwrap().finish(true, "");
    assert!(!s.moving());
    assert!(button_press(&mut s, "Install VLC").is_none());

    with_installed(&mut s, vec![installed_image("VLC", "VLC-3.0.AppImage", CameFrom::GitHub("john-smith/vlc".into()))]);
    assert!(button_press(&mut s, "Open VLC").is_some() && button_press(&mut s, "Install VLC").is_none());

    let mut t = mixed_store("failed");
    t.view = View::Home;
    let mut d = Dial::default();
    d.finish(false, "");
    t.dials.insert(key.into(), d);
    assert!(button_press(&mut t, "Install VLC").is_some());

    let f = found("VLC", &[]);
    s.owned = Default::default();
    s.dials.insert(key.into(), Dial::default());
    s.image = Some(f);
    assert!(words(&mut s).iter().any(|w| w == &t!("store-app-state-downloading").to_string()));
}

fn iconed(s: &mut Store, rows: &mut [Found]) {
    for (n, f) in rows.iter_mut().enumerate() {
        f.icon = format!("https://example.org/icons/{n}.png");
        s.shots.insert(f.icon.clone(), Some(Rc::new(image::open(icon(n)).unwrap())));
    }
}

pub(super) fn mixed_and_image_pages() {
    const LONG: &str = "A private and flexible note-taking app that works on top of a folder of plain files, with links between notes and a graph of them";
    let mut rows: Vec<Found> = ["Obsidian", "VLC", "Audacity", "Bitwarden", "Joplin"].iter().map(|n| Found { summary: LONG.into(), stars: Some(4210), ..found(n, &[]) }).collect();
    let mut s = mixed_store("pages");
    iconed(&mut s, &mut rows);
    s.hub = Some(Rc::new(rows.clone()));
    s.view = View::Home;
    shoot("discover-mixed", &mut s);
    let f = rows[0].clone();
    let key = "gh:john-smith/obsidian";
    s.image = Some(f.clone());
    shoot("image-page-looking", &mut s);
    answered(&mut s, &f);
    shoot("image-page-not-installed", &mut s);
    s.dials.insert(key.into(), { let mut d = Dial::default(); d.within(0.42); d });
    shoot("image-page-installing", &mut s);
    s.dials.clear();
    with_installed(&mut s, vec![installed_image("Obsidian", "Obsidian-1.5-x86_64.AppImage", CameFrom::GitHub("john-smith/obsidian".into()))]);
    shoot("image-page-installed", &mut s);
    s.view = View::Search("o".into());
    shoot("image-page-from-results", &mut s);
    s.view = View::Home;
    s.owned = Default::default();
    s.said.insert(key.into(), ("The release could not be fetched: GitHub did not answer.".into(), Tone::Problem));
    shoot("image-page-failed", &mut s);
    s.said.clear();
    s.github = pages::made_up::unreleased(pages::GitHubPage::default(), "john-smith/obsidian");
    shoot("image-page-nothing-to-install", &mut s);
    s.image = Some(Found { name: "Elsewhere".into(), site: "https://example.org/get".into(), summary: LONG.into(), listed: true, ..Found::default() });
    shoot("image-page-not-on-github", &mut s);
    s.view = View::Home;
    s.image = None;
    s.dials.insert("gh:john-smith/vlc".into(), { let mut d = Dial::default(); d.within(0.42); d });
    with_installed(&mut s, vec![installed_image("Joplin", "Joplin-2.9.AppImage", CameFrom::GitHub("john-smith/joplin".into()))]);
    shoot("discover-mixed-going-in", &mut s);
}

#[test]
#[ignore = "makes the pictures of the Flatpaks and AppImages side by side"]
fn shoot_flatpaks_and_appimages_side_by_side() {
    mixed_and_image_pages();
    let mut named_feed: Vec<Found> = ["Obsidian", "VLC", "Audacity", "Bitwarden", "Joplin", "Cura", "KeePassXC", "Darktable"].iter().enumerate()
        .map(|(n, name)| Found { summary: LINES[n % LINES.len()].into(), ..found(name, &[categories()[0].0]) }).collect();
    let shot = |name: &str, s: &mut Store, full: bool| {
        let w = 1278u32;
        s.across = w as f32;
        s.tall = 692.0;
        let mut ui = Ui::new(s.whole(settings()), Theme::new(false, "#fbc711"));
        let h = (ui.natural_height(w as f32).ceil() as u32).clamp(692, if full { 2600 } else { 692 });
        let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        std::fs::create_dir_all(folder()).unwrap();
        pix.save_png(folder().join(format!("apps-{name}.png"))).unwrap();
    };
    let mut s = mixed_store("shots");
    iconed(&mut s, &mut named_feed);
    s.hub = Some(Rc::new(named_feed));
    for (name, show) in [("discover-both", Show::Both), ("discover-flatpaks", Show::Flatpaks), ("discover-appimages", Show::AppImages)] {
        s.show = show;
        s.view = View::Home;
        shot(name, &mut s, true);
    }
    s.show = Show::Both;
    s.view = View::Search("o".into());
    shot("results-both", &mut s, true);
    s.view = View::Home;
    appimage_warning::before(&mut s, |_| {});
    shot("discover-warning", &mut s, false);
    appimage_warning::ticked(&mut s, true);
    shot("discover-warning-ticked", &mut s, false);
    appimage_warning::cancel(&mut s);
    (s.hub, s.online) = (Some(Rc::new(Vec::new())), false);
    shot("discover-offline-both", &mut s, true);
    s.show = Show::AppImages;
    shot("discover-offline-appimages", &mut s, false);
}
