//! Tests.

#[cfg(test)]
use super::*;

#[test]
fn every_tile_is_one_size_with_its_action_in_the_same_place() {
    for across in [820, 1100, 1278, 1720] {
        let mut s = with_updates(stocked());
        s.dials.insert("org.example.App2".into(), Dial::default());
        s.busy.insert("org.example.App2".into(), "install");
        for view in [View::Home, View::Category(categories()[0].0.to_string()), View::Search("app".into())] {
            s.view = view;
            let all = places(&mut s, across);
            let tiles: Vec<&(String, Spot)> = all.iter().filter(|(id, _)| id.starts_with(&format!("{}:", listing::TILE)) && !id.contains('/')).collect();
            assert!(tiles.len() >= 20, "{across}: {} tiles", tiles.len());
            let first = tiles[0].1;
            let action_in = |id: &str, tile: &Spot| {
                let key = format!("{}:{}", listing::TILE_ACTION, id.split_once(':').unwrap().1);
                let a = all.iter().find(|(id, r)| *id == key && r.y >= tile.y && r.y < tile.y + tile.h && r.x >= tile.x && r.x < tile.x + tile.w)
                    .unwrap_or_else(|| panic!("{key} is in its tile")).1;
                (a.x - tile.x, a.y - tile.y, a.w, a.h)
            };
            let want = action_in(&tiles[0].0, &first);
            assert_eq!(want.0 + want.2 + crate::toolkit::theme::sheet().card_padding, first.w, "{across}: the action is at the tile's far edge");
            let (mut lefts, mut tops) = (Vec::new(), Vec::new());
            for (id, tile) in &tiles {
                assert_eq!((tile.w, tile.h), (first.w, first.h), "{across}: {id} is the size of the first");
                assert_eq!(action_in(id, tile), want, "{across}: {id} has its action where the first has");
                if !lefts.contains(&tile.x) {
                    lefts.push(tile.x);
                }
                if !tops.contains(&tile.y) {
                    tops.push(tile.y);
                }
            }
            assert_eq!(tops.len(), tiles.len().div_ceil(lefts.len()), "{across}: {} tiles in {} columns", tiles.len(), lefts.len());
        }
    }
}

#[test]
fn every_installed_row_has_its_buttons_in_the_same_places() {
    for across in [820, 1278] {
        let mut s = with_updates(stocked());
        s.view = View::Installed;
        s.owned = pages::made_up::installed(Some(owned(12)), 0, "");
        let all = places(&mut s, across);
        let removes: Vec<Spot> = named(&all, &["remove:"]).into_iter().map(|(_, r)| r).collect();
        assert_eq!(removes.len(), 12);
        let step = removes[1].y - removes[0].y;
        for (i, r) in removes.iter().enumerate() {
            assert_eq!((r.x, r.w, r.h), (removes[0].x, removes[0].w, removes[0].h), "{across}: row {i}");
            assert!((r.y - (removes[0].y + step * i as f32)).abs() < 0.01, "{across}: row {i} is as tall as the others");
        }
        let update = named(&all, &["up:app:"]);
        assert_eq!(update.iter().map(|(id, _)| id.as_str()).collect::<Vec<_>>(), ["up:app:org.example.App1"], "{across}");
        let button = update[0].1;
        let gap = crate::toolkit::theme::sheet().gap;
        let own = all.iter().find(|(id, _)| id == "remove:org.example.App1").expect("its row").1;
        assert_eq!((button.y, button.w, button.h), (own.y, removes[0].w, removes[0].h), "{across}: on its own row, a round button like the others");
        assert_eq!(button.x, removes[0].x + 3.0 * (removes[0].w + gap), "{across}: the fourth place, at the edge");
        s.dials.insert(update_page::app_key("org.example.App1"), going(0.42));
        let during = places(&mut s, across);
        assert_eq!(named(&during, &["up:app:"])[0].1, button, "{across}: the line runs round the button where it stood");
        assert_eq!(named(&during, &["remove:"]), named(&all, &["remove:"]), "{across}");
        s.dials.clear();
        s.other = Some(crate::common::updating::Who::Ring);
        assert_eq!(named(&places(&mut s, across), &["remove:", "up:app:"]), named(&all, &["remove:", "up:app:"]), "{across}: held");
        s.other = None;
        s.updatable.clear();
        let after = places(&mut s, across);
        assert!(named(&after, &["up:app:"]).is_empty());
        assert_eq!(named(&after, &["remove:"]), named(&all, &["remove:"]), "{across}: updated, its row is as it was but for the button");
    }
}

const OWN: [&str; 5] = ["up:app:", "up:part:", "up:trindle", "up:debian", "fw:"];

#[test]
fn every_update_row_has_its_own_button_in_the_same_place() {
    for across in [820, 1278] {
        let mut s = with_updates(stocked());
        s.view = View::Updates;
        s.plugins = plugin_page::made_up(true, vec![plugin(0, "1.0", &["network"])], vec![offer(0, "1.2", &["network"])], &[], None, &[]);
        s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
        let all = places(&mut s, across);
        let found = named(&all, &OWN);
        beside_each_update_is_a_hold_back(&all, across);
        holding_back_moves_nothing(&mut s, &all, across);
        the_held_back_card_is_steady(&mut s, across);
        let plugins = every_update_button_is_the_one_design(&all, &found, across);
        let first = found[0].1;
        one_going_in_moves_nothing(&mut s, &found, &plugins, first, across);
        the_first_app_leaves(&mut s, &found, first, across);
    }
}

fn beside_each_update_is_a_hold_back(all: &[(String, Spot)], across: u32) {
    let gap = crate::toolkit::theme::sheet().gap;
    let holds = named(all, &["hold:"]);
    assert_eq!(holds.iter().map(|(id, _)| id.as_str()).collect::<Vec<_>>(),
               ["hold:up:app:org.example.App1", "hold:up:app:org.example.App4", "hold:up:part:org.example.Platform:25.08", "hold:fw:abc",
                "hold:pl:org.example.Plugin0"], "{across}: Trindle's row and Debian's have none");
    for (id, hold) in &holds {
        let own = all.iter().find(|(key, _)| Some(key.as_str()) == id.strip_prefix("hold:")).unwrap_or_else(|| panic!("{id} has its Update")).1;
        assert_eq!((hold.x + hold.w + gap, hold.y, hold.w, hold.h), (own.x, own.y, own.w, own.h), "{across}: {id} is beside its Update, the same size");
    }
}

fn holding_back_moves_nothing(s: &mut Store, all: &[(String, Spot)], across: u32) {
    let free = named(all, &["up:", "fw:", "pl:", "hold:", screen::HEADLINE]);
    s.held = held_one_of_each();
    let kept = places(s, across);
    assert_eq!(named(&kept, &["up:", "fw:", "pl:", "hold:", screen::HEADLINE]), free, "{across}: holding back moved something");
    let page = s.whole(settings());
    assert!(page.said().contains(&"13 updates are ready, 153 MB in all; 4 are held back. ".trim().to_string()), "{across}: {:?}", page.said());
    assert!(page.said().iter().any(|w| w == "Updates (13)"), "{across}: the side bar counts what can go in");
    let buttons = page.buttons();
    for (tip, can) in [("Update Notes", false), ("Update Clock", true), ("Release Notes", true), ("Hold back Clock", true), ("Install 13 updates", true),
                       ("Update Example Platform 25.08 (org.example.Platform)", false), ("Release Example keyboard", true), ("Update Weather", false)] {
        assert!(buttons.contains(&(tip.to_string(), can)), "{across}: {tip} {can}: {buttons:?}");
    }
    s.held = Default::default();
}

fn the_held_back_card_is_steady(s: &mut Store, across: u32) {
    use crate::common::held::{Held, Thing};
    let all_held = Held::of(&[Thing::app("org.example.App1"), Thing::app("org.example.App9"), Thing::part("org.example.Platform", "25.08"),
                              Thing::part("org.example.Sdk", "24.08"), Thing::firmware("abc"), Thing::plugin("org.example.Plugin0")]);
    s.held = all_held.clone();
    s.upd = std::mem::take(&mut s.upd).opened_with(&s.held);
    let opened = places(s, across);
    let card = named(&opened, &["hold:held:"]);
    assert_eq!(card.iter().map(|(id, _)| id.as_str()).collect::<Vec<_>>(),
               ["hold:held:up:app:org.example.App1", "hold:held:up:app:org.example.App9", "hold:held:up:part:org.example.Platform:25.08",
                "hold:held:up:part:org.example.Sdk:24.08", "hold:held:fw:abc", "hold:held:pl:org.example.Plugin0"], "{across}");
    let step = card[1].1.y - card[0].1.y;
    for (i, (id, r)) in card.iter().enumerate() {
        assert_eq!((r.x, r.w, r.h), (card[0].1.x, card[0].1.w, card[0].1.h), "{across}: {id} is where the first is, the same round button");
        assert!((r.y - (card[0].1.y + step * i as f32)).abs() < 0.01, "{across}: {id}'s row is as tall as the others");
    }
    let own = named(&opened, &["up:app:"]);
    assert!((own[1].1.y - own[0].1.y - step).abs() < 0.01, "{across}: a row is one height in every card");
    let still = |all: &[(String, Spot)]| named(all, &["up:", "fw:", "pl:", "hold:", screen::HEADLINE]);
    for (what, held) in [("one released", Held::of(&[Thing::app("org.example.App9"), Thing::part("org.example.Platform", "25.08"),
                                                  Thing::part("org.example.Sdk", "24.08"), Thing::firmware("abc"), Thing::plugin("org.example.Plugin0")])),
                         ("only one left", Held::of(&[Thing::part("org.example.Sdk", "24.08")])), ("every one released", Held::default()),
                         ("held again", all_held.clone())] {
        s.held = held;
        assert_eq!(still(&places(s, across)), still(&opened), "{across}: {what} moved something");
    }
    s.held = Held::default();
    let said = s.whole(settings()).said();
    assert!(said.iter().any(|w| w == "Held back") && said.iter().any(|w| w == "Nothing is held back."), "{across}: the card stays and says so: {said:?}");
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
}

fn every_update_button_is_the_one_design(all: &[(String, Spot)], found: &[(String, Spot)], across: u32) -> Vec<(String, Spot)> {
    let names: Vec<&str> = found.iter().map(|(id, _)| id.as_str()).collect();
    assert_eq!(names, ["up:app:org.example.App1", "up:app:org.example.App4", "up:part:org.example.Platform:25.08", "up:trindle", "up:debian", DEVICE],
               "{across}: one for each thing that can go in alone, and one for Debian's packages together");
    let plugins = named(all, &["pl:"]);
    assert_eq!(plugins.len(), 1, "{across}: the plugin's, in the same design");
    let first = found[0].1;
    for (id, r) in found.iter().chain(&plugins) {
        assert_eq!((r.w, r.h), (first.w, first.h), "{across}: {id} is the one round button");
    }
    for card in [&found[..3], &found[3..5]] {
        for (id, r) in card {
            assert_eq!(r.x, card[0].1.x, "{across}: {id}");
        }
    }
    let step = found[1].1.y - found[0].1.y;
    assert!((found[2].1.y - found[1].1.y - step).abs() < 0.01, "{across}: the rows are one height");
    plugins
}

fn one_going_in_moves_nothing(s: &mut Store, found: &[(String, Spot)], plugins: &[(String, Spot)], first: Spot, across: u32) {
    for (key, name) in [(found[0].0.clone(), "Notes"), ("up:debian".to_string(), "Debian's packages"), (DEVICE.to_string(), "Example keyboard")] {
        s.dials.insert(key.clone(), going(0.42));
        s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS).with_one(name);
        let during = places(s, across);
        let by = named(&during, &OWN)[0].1.y - first.y;
        assert_eq!(by, 0.0, "{across}: {key} going in moved the page");
        assert_eq!(named(&during, &OWN), lowered(found, by), "{across}: {key} going in");
        assert_eq!(named(&during, &["pl:"]), lowered(plugins, by), "{across}: {key} going in");
        s.dials.clear();
    }
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS);
    s.other = Some(crate::common::updating::Who::Ring);
    let held = named(&places(s, across), &OWN);
    let by = held[0].1.y - first.y;
    assert_eq!(by, 0.0, "{across}: held by the ring moved the page");
    assert_eq!(held, lowered(found, by), "{across}: held by the ring");
    s.other = None;
}

fn the_first_app_leaves(s: &mut Store, found: &[(String, Spot)], first: Spot, across: u32) {
    let mut ui = Ui::new(s.whole(settings()), Theme::new(false, "#fbc711"));
    assert_eq!(named(&ui.places_at((across, 4000)), &OWN), found, "{across}: the open window is laid out as a new one is");
    s.updatable.remove(0);
    s.upd = update_page::made_up(&[], 0, false, &SIZES_KNOWN, &[], &[]).with_flats(&FLATS[1..]);
    ui.set_view(s.whole(settings()));
    let after = named(&ui.places_at((across, 4000)), &OWN);
    assert_eq!(after.len(), found.len() - 1);
    let by = after[0].1.y - first.y;
    assert_eq!(by, 0.0, "{across}: one going moved the page");
    assert_eq!(after[0], (found[1].0.clone(), Spot { y: first.y + by, ..first }), "{across}: the next app has the place the first had");
    assert_eq!(after[1].1.y, found[1].1.y + by, "{across}: and the row after it has closed up by one row");
    for ((id, now), (_, was)) in after.iter().zip(&found[1..]) {
        assert_eq!((now.x, now.w, now.h), (was.x, was.w, was.h), "{across}: {id} has not moved sideways");
    }
}

#[test]
fn the_github_key_is_a_button_whose_box_floats_over_the_page() {
    use crate::toolkit::input::{Input, Keysym, Mods};
    let found = |n: usize| github::Found { repo: format!("john-smith/example-{n}"), name: NAMES[n].into(), summary: LINES[n % LINES.len()].into(), stars: Some(120 * n as u64), ..Default::default() };
    for across in [820u32, 1278] {
        let mut s = stocked();
        s.view = View::GitHub;
        s.github = pages::made_up::github("photos", (1..4).map(found).collect(), (4..7).map(found).collect(), true);
        s.tall = 692.0;
        let closed = places(&mut s, across);
        let said = words(&mut s);
        assert!(said.contains(&"button: GitHub key".to_string()), "{across}: {said:?}");
        for gone in ["GitHub key", "A key is stored", "No key is stored", "Only if GitHub stops answering", "button: Keep the key"] {
            assert!(!said.iter().any(|w| w == gone), "{across}: {gone:?} is on the page with the box shut");
        }
        assert!(!said.iter().any(|w| w.starts_with("You almost certainly")), "{across}");
        let key = closed.iter().find(|(id, _)| id == "gh:key-open").expect("the key button").1;
        let sheet = crate::toolkit::theme::sheet();
        let edge = across as f32 - (key.x + key.w);
        assert!(edge > 0.0 && edge <= 2.0 * sheet.page_padding + sheet.card_padding + 24.0, "{across}: the key button is at the far end, {edge} from the edge");
        let rows = |all: &[(String, Spot)]| -> Vec<(String, Spot)> { all.iter().filter(|(id, _)| id.starts_with("gh:john")).cloned().collect() };
        assert_eq!(rows(&closed).len(), 6, "{across}: {:?}", rows(&closed));
        assert!(rows(&closed).iter().all(|(_, r)| key.y + key.h < r.y), "{across}: the button is above the rows, in the heading of the first card");
        s.github = pages::made_up::github_key(std::mem::take(&mut s.github), Some(key), false, None);
        assert!(pages::github_key::key_box_open(&s));
        let open = places(&mut s, across);
        let still = ["gh:key-open", screen::HEADLINE];
        assert_eq!(named(&open, &still), named(&closed, &still), "{across}: opening the box moved the page");
        assert_eq!(rows(&open), rows(&closed), "{across}: opening the box moved a row");
        let typed = open.iter().find(|(id, _)| id == "gh:key-box").expect("the box to paste the key in").1;
        assert!(typed.y > key.y + key.h && typed.x >= 8.0 && typed.x + typed.w <= across as f32 - 8.0, "{across}: {typed:?} under {key:?}");
        let said = words(&mut s);
        for there in ["GitHub key", "No key is stored", "button: Keep the key", "button: Make a key on GitHub", "secret: Paste the key"] {
            assert!(said.iter().any(|w| w == there), "{across}: {there:?} is in the box: {said:?}");
        }
        assert!(said.iter().any(|w| w.starts_with("You almost certainly do not need one.") && w.contains("kept on this computer")), "{across}");
        assert!(!said.iter().any(|w| w == "button: Remove the key"), "{across}: there is none to remove");
        s.github = pages::made_up::github_key(std::mem::take(&mut s.github), Some(key), true, None);
        let said = words(&mut s);
        for there in ["A key is stored", "button: Remove the key", "button: GitHub key: one is stored"] {
            assert!(said.iter().any(|w| w == there), "{across}: {there:?}: {said:?}");
        }
        let escape = Input::Key { sym: Keysym::Escape, text: None, mods: Mods::default(), repeat: false };
        let shut = Shows::intercept(&s, &escape, false).expect("Escape shuts the box");
        Shows::update(&mut s, shut);
        assert!(!pages::github_key::key_box_open(&s) && Shows::intercept(&s, &escape, false).is_none(), "{across}");
        assert_eq!(rows(&places(&mut s, across)), rows(&closed), "{across}: shutting it moved the page");
        s.github = pages::made_up::github_key(std::mem::take(&mut s.github), Some(key), true, None);
        pages::github_key::close_key_box(&mut s);
        assert!(!pages::github_key::key_box_open(&s));
        s.github = pages::made_up::github_key(std::mem::take(&mut s.github), Some(key), true, None);
        s.view = View::Sources;
        assert!(!pages::github_key::key_box_open(&s) && pages::github_key::key_box(&s).is_none(), "{across}: the box belongs to its page");
    }
    let source = crate::common::source::code(include_str!("github/tests.rs"));
    assert!(source.contains("press the key at the top right of Search GitHub") && !source.contains(&["GitHub key ", "below"].concat()));
}
