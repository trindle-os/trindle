//! Tests: whole.

#[cfg(test)]
use super::*;

#[test]
fn every_page_is_whole_for_a_screen_reader() {
    let mut wrong = Vec::new();
    for pages in [every_page(), made_up_pages()] {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages, current: 0, search: String::new(), tall: 692.0 };
        for i in 0..s.pages.len() {
            s.current = i;
            for (w, h) in [(1278, 692), (1920, 1080), (760, 480)] {
                let mut ui = Ui::new(s.view(), Theme::new(true, "#fbc711"));
                ui.lay_at(w, h);
                let kept = ui.kept_from_a_reader();
                if !kept.is_empty() {
                    wrong.push(format!("{} at {w} by {h}: {} parts, the first {}", s.pages[i].id(), kept.len(), kept[0]));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "a screen reader could not reach every part of these pages:\n{}", wrong.join("\n"));
}

#[test]
fn nothing_on_any_page_touches_its_neighbour() {
    use crate::toolkit::ui::Labels;
    let mut wrong = Vec::new();
    for pages in [every_page(), made_up_pages()] {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages, current: 0, search: String::new(), tall: 692.0 };
        for i in 0..s.pages.len() {
            s.current = i;
            for w in [760, 1278, 1480, 1920] {
                for labels in [Labels::Brief, Labels::Always] {
                    let mut ui = Ui::new(s.view(), Theme::new(true, "#fbc711"));
                    ui.set_labels(labels);
                    ui.lay_at(w, 692);
                    for found in ui.squeezed_or_touching() {
                        wrong.push(format!("{} at {w}: {found}", s.pages[i].id()));
                    }
                }
            }
        }
    }
    wrong.dedup();
    assert!(wrong.is_empty(), "{} things touch or are squeezed:\n{}", wrong.len(), wrong.join("\n"));
}

#[test]
fn no_two_cards_on_a_page_share_a_name() {
    for pages in [every_page(), made_up_pages()] {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages, current: 0, search: String::new(), tall: 692.0 };
        for i in 0..s.pages.len() {
            s.current = i;
            let mut ui = Ui::new(s.view(), Theme::new(true, "#fbc711"));
            ui.lay_at(1278, 692);
            let names: Vec<String> = ui.puzzle_cards_named().into_iter().filter_map(|c| c.0).collect();
            for (at, n) in names.iter().enumerate() {
                assert!(!names[..at].contains(n), "{}: two cards are called {n}", s.pages[i].id());
            }
        }
    }
}

#[test]
fn every_page_has_a_made_up_state() {
    let made: Vec<&str> = made_up_pages().iter().map(|p| p.id()).collect();
    let all: Vec<&str> = every_page().iter().map(|p| p.id()).collect();
    for id in &all {
        assert_eq!(made.iter().filter(|m| *m == id).count(), 1, "{id} is among the made-up pages once");
    }
    assert_eq!(made.len(), all.len());
}

#[test]
fn every_full_page_has_the_house_shape() {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages: made_up_pages(), current: 0,
                           search: String::new(), tall: 800.0 };
    let fonts = crate::toolkit::text::Fonts::new();
    let mut wrong = Vec::new();
    for i in 0..s.pages.len() {
        s.current = i;
        let tree = s.view();
        let id = s.pages[i].id();
        for w in [1280.0, 1920.0] {
            let found = by_choice(id, crate::toolkit::widget::shape::check(&fonts, &tree, w, 800.0), w);
            if found.wide > 1 {
                wrong.push(format!("{id} at {w}: {} cards wider than three quarters of the page", found.wide));
            }
            for b in &found.bars {
                wrong.push(format!("{id} at {w}: a card thinner than 3:1, {b}"));
            }
            for l in &found.long {
                wrong.push(format!("{id} at {w}: a long line: {l}"));
            }
            if found.cards > 3 && !found.puzzle {
                wrong.push(format!("{id} at {w}: {} cards not in a puzzle", found.cards));
            }
        }
        for w in [1280.0, 900.0, 400.0] {
            for (path, spare) in crate::toolkit::widget::loose_tiles(&fonts, &tree, w, 800.0) {
                wrong.push(format!("{id} at {w}: tile {path:?} has {spare} spare"));
            }
        }
    }
    wrong.dedup();
    assert!(wrong.is_empty(), "{} faults:\n{}", wrong.len(), wrong.join("\n"));
}

#[test]
#[ignore]
fn shoot_the_layout_of_every_page() {
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));
    let prefix = std::env::var("TRINDLE_SHOT_PREFIX").unwrap_or_else(|_| "settings-".into());
    let only = std::env::var("TRINDLE_SHOT_PAGES").unwrap_or_default();
    std::fs::create_dir_all(&out).unwrap();
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let pages = if std::env::var_os("TRINDLE_SHOT_EMPTY").is_some() { every_page() } else { made_up_pages() };
    let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages, current: 0, search: String::new(), tall: 692.0 };
    for i in 0..s.pages.len() {
        let id = s.pages[i].id();
        if !only.is_empty() && !only.split(',').any(|o| o == id) {
            continue;
        }
        s.current = i;
        s.kit.current = id.to_string();
        s.kit.icon = s.pages[i].icon();
        for (w, h, tail) in [(1278u32, 692u32, ""), (700, 692, "-narrow"), (1278, 1500, "-tall")] {
            s.tall = h as f32;
            let mut ui = Ui::new(s.view(), Theme::new(std::env::var_os("TRINDLE_SHOT_LIGHT").is_some(), "#fbc711"));
            let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            pix.save_png(out.join(format!("{prefix}{id}{tail}.png"))).unwrap();
        }
    }
}
