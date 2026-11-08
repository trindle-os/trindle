//! Tests: pages.

#[cfg(test)]
use super::*;

#[test]
fn the_wallpaper_page_is_laid_out_as_asked() {
    for across in [NARROWEST, 560, 700, 900, 1100, 1280, 1600, 1920] {
        let s = showing(Box::new(panels::wallpaper::Wallpaper::made_up()), 900.0);
        let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
        ui.lay_at(across, 900);
        let cards = ui.puzzle_cards_named();
        let card = |name: &str| cards.iter().find(|c| c.0.as_deref() == Some(crate::settings::ui::card_key(name).as_str()))
            .unwrap_or_else(|| panic!("{across}: no card called {name}")).1;

        let shown = cards.iter().find(|c| c.0.is_none()).unwrap_or_else(|| panic!("{across}: no picture")).1;
        let (paper, changing, pictures) = (card("Wallpaper"), card("Changing pictures"), card("Pictures"));
        let (left, right) = (cards.iter().map(|c| c.1.x).fold(f32::MAX, f32::min), cards.iter().map(|c| c.1.x + c.1.w).fold(0.0, f32::max));
        let whole = |r: crate::toolkit::widget::Rect| (r.x - left).abs() < 1.0 && (r.x + r.w - right).abs() < 1.0;
        assert!(whole(changing), "{across}: Changing pictures does not cross the page: {changing:?} in {left}..{right}");
        assert!(whole(pictures), "{across}: Pictures does not cross the page: {pictures:?}");
        assert!(changing.y >= shown.y + shown.h && changing.y >= paper.y + paper.h && pictures.y >= changing.y + changing.h, "{across}: the cards are out of order");
        if (shown.y - paper.y).abs() < 1.0 {

            assert!((shown.h - paper.h).abs() < 1.0, "{across}: {} and {} tall", shown.h, paper.h);
            assert!((shown.x - left).abs() < 1.0 && (paper.x + paper.w - right).abs() < 1.0, "{across}: a gap beside the top row: {shown:?} {paper:?}");
            assert!(across >= 700, "{across}: the top two are side by side in a window too narrow for it");
        } else {

            assert!(whole(shown) && whole(paper), "{across}: {shown:?} {paper:?}");
            assert!(across < 1100, "{across}: the top two are not side by side");
        }

        let boxes = ui.boxes();
        let control = |key: &str| boxes.iter().find(|b| b.0.ends_with(key)).unwrap_or_else(|| panic!("{across}: no control {key}")).1;
        let (every, order, next) = (control("dropdown:every"), control("dropdown:order"), control(panels::wallpaper::NEXT));
        if order.y < every.y {

            assert!(order.x > every.x && next.x > every.x, "{across}: Order and Next picture are the right-hand column: {every:?} {order:?} {next:?}");
            assert!(next.y > order.y, "{across}: Next picture is under Order, in its column");
            assert!(every.x + every.w <= changing.x + changing.w / 2.0 + 1.0 && order.x >= changing.x + changing.w / 2.0 - 1.0 && next.x >= changing.x + changing.w / 2.0 - 1.0,
                    "{across}: each column keeps to its half: {every:?} {order:?} {next:?}");
        } else {
            assert!(every.y < order.y && order.y < next.y, "{across}: one column, in order: {every:?} {order:?} {next:?}");
        }
        for inside in [every, order, next] {
            assert!(inside.x >= changing.x && inside.x + inside.w <= changing.x + changing.w + 0.5, "{across}: a control leaves its card");
            assert!(inside.y >= changing.y && inside.y + inside.h <= changing.y + changing.h + 0.5, "{across}: a control leaves its card");
        }

        let foot = [every, order, next].iter().map(|r| r.y + r.h).fold(0.0, f32::max);
        assert!(changing.y + changing.h - foot < 60.0, "{across}: empty room at the foot of Changing pictures: {}", changing.y + changing.h - foot);
    }
    let said = showing(Box::new(panels::wallpaper::Wallpaper::made_up()), 900.0).view().said();
    for words in ["Change the picture now and then", "Change every", "Order", "Next picture now", "Wallpaper", "Picture", "Fit", "Colour",
                  "Every picture in a folder, in turn.", "How long each picture stays.", "Shuffled, or by file name.", "Skips to the next one at once."] {
        assert!(said.iter().any(|s| s == words), "{words:?} is gone from the page");
    }
}

#[test]
fn the_notes_about_memory_sit_with_the_picture() {
    for across in [NARROWEST, 900, 1280, 1920] {
        let s = showing(Box::new(panels::wallpaper::Wallpaper::with_memory()), 900.0);
        let said = s.view().said();
        for words in ["This picture is bigger than your screen and uses extra memory.",
                      "Moving wallpapers use more memory and battery."] {
            assert!(said.iter().any(|w| w == words), "{across}: {words:?} is not on the page");
        }
        let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
        ui.lay_at(across, 900);
        let cards = ui.puzzle_cards_named();
        let shown = cards.iter().find(|c| c.0.is_none()).unwrap_or_else(|| panic!("{across}: no picture")).1;
        let paper = cards.iter().find(|c| c.0.as_deref() == Some(crate::settings::ui::card_key("Wallpaper").as_str())).unwrap().1;
        if (shown.y - paper.y).abs() < 1.0 {
            assert!((shown.h - paper.h).abs() < 1.0, "{across}: {} and {} tall", shown.h, paper.h);
        }
    }
}

#[test]
fn the_pictures_fill_their_card_at_wide_windows() {
    for across in [900, 1280, 1600, 1920, 2560, 3200, 3840] {
        let s = showing(Box::new(panels::wallpaper::Wallpaper::made_up()), 900.0);
        let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
        ui.lay_at(across, 1200);
        let cards = ui.puzzle_cards_named();
        let card = cards.iter().find(|c| c.0.as_deref() == Some(crate::settings::ui::card_key("Pictures").as_str())).unwrap().1;
        let boxes = ui.boxes();
        let tiles: Vec<_> = boxes.iter().filter(|b| b.0.contains("tile:") && !b.0.ends_with("/0")).map(|b| b.1).collect();
        assert_eq!(tiles.len(), 8, "{across}: every picture has a tile");
        let foot = tiles.iter().map(|t| t.y + t.h).fold(0.0, f32::max);
        let wide = tiles.iter().map(|t| t.x + t.w).fold(0.0, f32::max);
        assert!(foot <= card.y + card.h && wide <= card.x + card.w, "{across}: the tiles leave their card");

        let spare = card.y + card.h - foot;
        assert!(spare <= card.h / 5.0, "{across}: {spare} of {} left empty under the pictures", card.h);
    }
}

#[test]
fn every_page_has_the_house_shape() {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut pages = panels::all();
    pages.extend([
        Box::new(panels::crash_reports::CrashReports::default()) as Box<dyn Page>,
        Box::new(panels::plugins::Plugins::default()),
    ]);
    let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages, current: 0, search: String::new(), tall: 800.0 };
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
    }
    wrong.dedup();
    assert!(wrong.is_empty(), "{} faults:\n{}", wrong.len(), wrong.join("\n"));
}

#[test]
fn a_band_never_moves_a_rows_words_or_buttons_sideways() {
    use crate::toolkit::widget::{button, card, column, label, row, space, Length, Node};
    type Banding = fn(Vec<Node<Msg>>) -> Vec<Node<Msg>>;
    let ways: [(&str, Banding); 4] = [
        ("the toolkit's banded_list", crate::toolkit::widget::banded_list),
        ("the Settings people lists", panels::users::banded_rows),
        ("the store's lists", crate::store::listing::rows),
        ("Network's lists", panels::network::banded),
    ];
    for (name, way) in ways {
        let rows: Vec<Node<Msg>> = (0..9).map(|i| {
            row(vec![label(format!("Row {i}")).key(format!("t{i}")), space(),
                     button("user-trash-symbolic", "Delete").key(format!("b{i}"))]).gap(8.0).width(Length::Fill)
        }).collect();
        let mut ui = Ui::new(card(column(way(rows)).gap(0.0)).width(Length::Fixed(600.0)), Theme::new(false, "#fbc711"));
        ui.lay_at(700, 900);
        let boxes = ui.boxes();
        for part in ["t", "b"] {
            let xs: Vec<f32> = (0..9).map(|i| {
                let id = format!("{part}{i}");
                boxes.iter().find(|b| b.0 == id).unwrap_or_else(|| panic!("{name}: no {id}")).1.x
            }).collect();
            assert!(xs.iter().all(|x| (x - xs[0]).abs() < 0.5), "{name}: the {part} column is not straight: {xs:?}");
        }
    }
}

#[test]
fn the_snapshots_columns_are_straight() {
    let s = showing(Box::new(panels::snapshots::Snapshots::made_up()), 900.0);
    let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
    ui.lay_at(1280, 900);
    let boxes = ui.boxes();
    let xs = |part: &str| -> Vec<f32> {
        boxes.iter().filter(|b| if part.starts_with("twice:") { b.0.starts_with(part) }
                                else { b.0.starts_with("card:Snapshots/0.") && b.0.ends_with(part) && !b.0.ends_with("/0.0.0") })
            .map(|b| b.1.x).collect()
    };
    for part in [".0.0.0", ".0.0.1", "twice:restore:", "twice:delete:"] {
        let found = xs(part);
        assert!(found.len() >= 5, "{part}: only {} found", found.len());
        assert!(found.iter().all(|x| (x - found[0]).abs() < 0.5), "{part} is not straight: {found:?}");
    }
}

#[test]
fn the_updates_switch_is_there_on_and_off_and_moves_nothing() {
    let said = |page: Box<dyn Page>| showing(page, 900.0).view().said();
    let on = said(Box::new(panels::about::About::made_up()));
    let off = said(Box::new(panels::about::About::made_up_not_looking()));
    for words in [&on, &off] {
        assert!(words.iter().any(|w| w == "Updates"), "the card");
        assert!(words.iter().any(|w| w == "Check for updates"), "the switch");
        assert!(words.iter().any(|w| w == "Security updates"), "the security switch");
    }
    assert_eq!(on, off);
}

#[test]
fn the_security_switch_warns_when_off() {
    let said = |page: Box<dyn Page>| showing(page, 900.0).view().said();
    let on = said(Box::new(panels::about::About::made_up()));
    let off = said(Box::new(panels::about::About::made_up_not_secure()));
    assert!(on.iter().any(|w| w.contains("install by themselves every day")));
    assert!(!on.iter().any(|w| w.contains("no longer install")));
    assert!(off.iter().any(|w| w.contains("no longer install by themselves")));
}

#[test]
#[ignore]
fn shoot_the_snapshots_bands() {
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/bands-h496".into()));
    let tag = std::env::var("TRINDLE_TAG").unwrap_or_else(|_| "after".into());
    std::fs::create_dir_all(&out).unwrap();
    for light in [false, true] {
        let s = showing(Box::new(panels::snapshots::Snapshots::made_up()), 700.0);
        let mut ui = Ui::new(s.view(), Theme::new(light, "#fbc711"));
        let mut pix = tiny_skia::Pixmap::new(1280, 700).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        pix.save_png(out.join(format!("{tag}-snapshots-{}.png", if light { "light" } else { "dark" }))).unwrap();
    }
}
