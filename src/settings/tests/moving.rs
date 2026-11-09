//! Tests: moving.

#[cfg(test)]
use super::*;

#[test]
fn opening_a_dropdown_moves_nothing_on_any_page() {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let (w, h) = (1278u32, 692u32);
    let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages: pages_with_dropdowns(), current: 0, search: String::new(), tall: h as f32 };
    let now = std::time::Instant::now();
    let (mut found, mut with) = (Dropdowns { wrong: Vec::new(), opened: 0 }, Vec::new());
    for i in 0..s.pages.len() {
        s.current = i;
        let id = s.pages[i].id();
        let mut ui = Ui::new(s.view(), Theme::new(true, "#fbc711"));
        ui.lay_at(w, h);
        let all = ui.dropdowns();
        if !all.is_empty() {
            with.push(id);
        }
        for (dd, usable) in all {
            if usable {
                try_dropdown(&s, &mut ui, id, &dd, (now, w, h), &mut found);
            } else {

                ui.keyboard_to(&dd);
                ui.input(&dropdown_key(crate::toolkit::input::Keysym::space), now);
                if ui.open_list().is_some() {
                    found.wrong.push(format!("{id} {dd}: it opened though it cannot be used"));
                }
            }
        }
    }
    let Dropdowns { wrong, opened } = found;
    assert!(wrong.is_empty(), "{} faults:\n{}", wrong.len(), wrong.join("\n"));

    for id in ["ring", "power", "datetime"] {
        assert!(with.contains(&id), "{id} has a dropdown to try; pages with one: {with:?}");
    }
    assert!(opened >= 6, "only {opened} dropdowns were found, on {with:?}");
}

struct Dropdowns {
    wrong: Vec<String>,
    opened: i32,
}

fn dropdown_key(sym: crate::toolkit::input::Keysym) -> crate::toolkit::input::Input {
    crate::toolkit::input::Input::Key { sym, text: None, mods: crate::toolkit::input::Mods::default(), repeat: false }
}

fn pages_with_dropdowns() -> Vec<Box<dyn Page>> {
    let mut pages = panels::all();
    for p in [
        Box::new(panels::crash_reports::CrashReports::default()) as Box<dyn Page>,
        Box::new(panels::plugins::Plugins::default()),
    ] {
        if !pages.iter().any(|q| q.id() == p.id()) {
            pages.push(p);
        }
    }
    pages
}

fn try_dropdown(s: &Settings, ui: &mut Ui<Msg>, id: &str, dd: &str, (now, w, h): (std::time::Instant, u32, u32), found: &mut Dropdowns) {
    use crate::toolkit::input::{Input, Keysym, Mods};
    let wrong = &mut found.wrong;
    ui.keyboard_to(dd);
    let before = ui.boxes();
    let reply = ui.input(&dropdown_key(Keysym::space), now);
    if !reply.messages.is_empty() {
        wrong.push(format!("{id} {dd}: opening it told the page something"));
    }
    let Some((list, own)) = ui.open_list() else {
        wrong.push(format!("{id} {dd}: Space did not open it"));
        return;
    };
    found.opened += 1;

    ui.set_view(s.view());
    ui.lay_at(w, h);
    if ui.boxes() != before {
        wrong.push(format!("{id} {dd}: something moved when it opened"));
    }
    if list.x < 0.0 || list.y < 0.0 || list.x + list.w > w as f32 || list.y + list.h > h as f32 {
        wrong.push(format!("{id} {dd}: its list leaves the window, {list:?}"));
    }
    let (under, over) = (list.y >= own.y + own.h, list.y + list.h <= own.y);
    if !(under || over) || (list.y - (own.y + own.h)).abs().min((own.y - (list.y + list.h)).abs()) > 5.0 {
        wrong.push(format!("{id} {dd}: its list is not against its box, {list:?} {own:?}"));
    }

    for sym in [Keysym::Down, Keysym::End, Keysym::Home] {
        ui.input(&dropdown_key(sym), now);
    }
    ui.input(&Input::Key { sym: Keysym::a, text: Some("a".into()), mods: Mods::default(), repeat: false }, now);
    if ui.boxes() != before {
        wrong.push(format!("{id} {dd}: something moved while its list was used"));
    }
    let reply = ui.input(&dropdown_key(Keysym::Escape), now);
    if ui.open_list().is_some() || !reply.messages.is_empty() || ui.boxes() != before {
        wrong.push(format!("{id} {dd}: Escape did not close it and leave all as it was"));
    }
}

#[test]
fn pressing_anything_on_any_page_moves_nothing() {
    let (w, h) = (1278u32, 692u32);
    let now = std::time::Instant::now();
    let mut found = Pressing { wrong: Vec::new(), pressed: 0, chosen: 0 };
    let mut scrolled = Vec::new();

    for (full, pages) in [(false, every_page()), (true, made_up_pages())] {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages, current: 0, search: String::new(), tall: h as f32 };
        for i in 0..s.pages.len() {
            s.current = i;
            let id = s.pages[i].id();
            let name = format!("page:{}", s.pages[i].title());
            let mut ui = Ui::new(s.view(), Theme::new(true, "#fbc711"));
            ui.lay_at(w, h);

            let Some(most) = ui.places().iter().find(|p| p.0 == name).map(|p| p.2) else {
                found.wrong.push(format!("{id}: its page is not a scrolling area named {name}"));
                continue;
            };
            ui.scroll_area(&name, most);
            ui.scroll_area("side", 1e9);
            if most > 0.0 {
                scrolled.push((id, full));
            }
            for (widget, kind) in ui.pressables() {
                press_one(&s, &mut ui, id, (&widget, kind), (now, w, h), &mut found);
            }
            let places = shut_and_open_the_i(&mut s, &mut ui, id, (&name, most), (w, h), &mut found.wrong);
            the_next_page_starts_at_its_top(&mut s, &mut ui, (id, i), (w, h), &places, &mut found.wrong);
        }
    }
    let Pressing { mut wrong, pressed, chosen } = found;
    wrong.dedup();
    assert!(wrong.is_empty(), "{} faults:\n{}", wrong.len(), wrong.join("\n"));
    assert!(pressed >= 60, "only {pressed} things were pressed");
    assert!(chosen >= 6, "only {chosen} answers were chosen from dropdowns");
    assert!(scrolled.len() >= 8, "only {scrolled:?} were long enough to scroll");
}

struct Pressing {
    wrong: Vec<String>,
    pressed: i32,
    chosen: usize,
}

fn press_one(s: &Settings, ui: &mut Ui<Msg>, id: &str, (widget, kind): (&str, &str), (now, w, h): (std::time::Instant, u32, u32), found: &mut Pressing) {
    use crate::toolkit::input::Keysym;
    let wrong = &mut found.wrong;

    ui.keyboard_to(widget);
    let (boxes, places) = (ui.boxes(), ui.places());
    let keys: &[Keysym] = match kind {
        "dropdown" => &[Keysym::space, Keysym::Down, Keysym::Return],
        "choice" | "slider" => &[Keysym::Right, Keysym::Left],
        _ => &[Keysym::space],
    };
    let mut told = 0;
    for &sym in keys {
        told += ui.input(&dropdown_key(sym), now).messages.len();
        if kind != "dropdown" && (ui.boxes() != boxes || ui.places() != places) {
            wrong.push(format!("{id} {kind} {widget}: something moved as it was pressed"));
        }
    }
    if kind == "dropdown" {
        if ui.open_list().is_some() {
            wrong.push(format!("{id} {widget}: Enter did not close its list"));
            ui.input(&dropdown_key(Keysym::Escape), now);
        }
        found.chosen += told;
    }
    found.pressed += 1;

    ui.set_view(s.view());
    ui.lay_at(w, h);
    if ui.places() != places {
        wrong.push(format!("{id} {kind} {widget}: the page scrolled, {places:?} to {:?}", ui.places()));
    }
    if ui.boxes() != boxes {
        wrong.push(format!("{id} {kind} {widget}: something moved when the page was drawn again"));
    }
}

fn shut_and_open_the_i(s: &mut Settings, ui: &mut Ui<Msg>, id: &str, (name, most): (&str, f32), (w, h): (u32, u32), wrong: &mut Vec<String>) -> Vec<(String, f32, f32)> {
    ui.scroll_area(name, most);
    let (boxes, places) = (ui.boxes(), ui.places());
    for open in [true, false, true, false] {
        s.kit.info = open.then(|| ("page".to_string(), "What this page is for.".to_string(), crate::toolkit::widget::Rect { x: 400.0, y: 60.0, w: 20.0, h: 20.0 }));
        ui.set_view(s.view());
        ui.lay_at(w, h);
        if ui.places() != places {
            wrong.push(format!("{id}: the page scrolled when its i was {}, {places:?} to {:?}", if open { "opened" } else { "shut" }, ui.places()));
        }
        let now_boxes = ui.boxes();
        if let Some(b) = boxes.iter().find(|b| !now_boxes.contains(b)) {
            wrong.push(format!("{id}: {} moved when its i was {}", b.0, if open { "opened" } else { "shut" }));
        }
    }
    places
}

fn the_next_page_starts_at_its_top(s: &mut Settings, ui: &mut Ui<Msg>, (id, i): (&str, usize), (w, h): (u32, u32), places: &[(String, f32, f32)], wrong: &mut Vec<String>) {
    s.current = (i + 1) % s.pages.len();
    let next = format!("page:{}", s.pages[s.current].title());
    ui.set_view(s.view());
    ui.lay_at(w, h);
    if ui.places().iter().any(|p| p.0 == next && p.1 != 0.0) {
        wrong.push(format!("{id}: the page after it did not start at its top"));
    }
    if ui.places().iter().find(|p| p.0 == "side").map(|p| p.1) != places.iter().find(|p| p.0 == "side").map(|p| p.1) {
        wrong.push(format!("{id}: the side bar moved when the page changed"));
    }
}

#[test]
fn a_press_never_changes_the_cards_on_a_page() {
    use crate::toolkit::widget::Rect;
    let mut wrong: Vec<String> = Vec::new();
    let mut flips = 0;
    for (w, h) in [(1278u32, 692u32), (700, 692)] {
        for states in flipped_pages() {
            let events = Loop::new();
            let asks = events.post(|_: Do| {});
            let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages: states, current: 0,
                                   search: String::new(), tall: h as f32 };
            let id = s.pages[0].id();
            s.kit.current = id.to_string();
            let mut ui = Ui::new(s.view(), Theme::new(true, "#fbc711"));
            ui.lay_at(w, h);

            let cards = |ui: &Ui<Msg>| {
                let all = ui.puzzle_cards_named();
                let top = all.iter().map(|c| c.1.y).fold(f32::MAX, f32::min);
                all.into_iter().map(|(name, r)| (name, Rect { y: r.y - top, ..r })).collect::<Vec<_>>()
            };
            let first = cards(&ui);
            let names: Vec<&String> = first.iter().filter_map(|c| c.0.as_ref()).collect();
            for (i, n) in names.iter().enumerate() {
                if names[..i].contains(n) {
                    wrong.push(format!("{id}: two cards are called {n}"));
                }
            }
            let mut was = first.clone();

            for at in (1..s.pages.len()).chain([0]) {
                assert_eq!(s.pages[at].id(), id, "the states are all of one page");
                s.current = at;
                ui.set_view(s.view());
                ui.lay_at(w, h);
                let now = cards(&ui);
                flips += 1;
                cards_after_a_press(id, w, at, &was, &now, &mut wrong);
                was = now;
            }
            if was != first {
                wrong.push(format!("{id} at {w}: back as it was, the cards are not where they were"));
            }
        }
    }
    wrong.dedup();
    assert!(wrong.is_empty(), "{} faults:\n{}", wrong.len(), wrong.join("\n"));
    assert!(flips >= 40, "only {flips} states were tried");
}

type Cards = [(Option<String>, crate::toolkit::widget::Rect)];

fn cards_after_a_press(id: &str, w: u32, at: usize, was: &Cards, now: &Cards, wrong: &mut Vec<String>) {
    use crate::toolkit::widget::Rect;
    let beside = |a: &Rect, b: &Rect| a.x < b.x + b.w - 0.5 && b.x < a.x + a.w - 0.5;
    let keys = |c: &[(Option<String>, Rect)]| c.iter().map(|c| c.0.clone()).collect::<Vec<_>>();
    if keys(now) != keys(was) {
        wrong.push(format!("{id} at {w}, state {at}: the cards changed, {:?} to {:?}", keys(was), keys(now)));
        return;
    }
    for i in 0..was.len() {
        let (name, a, b) = (was[i].0.clone().unwrap_or_else(|| format!("card {i}")), was[i].1, now[i].1);
        if (a.x, a.w) != (b.x, b.w) {
            wrong.push(format!("{id} at {w}, state {at}: {name} left its column, {a:?} to {b:?}"));
        }
        let pushed = (0..was.len()).any(|k| was[k].1.y < a.y && beside(&was[k].1, &a)
            && (was[k].1.h != now[k].1.h || was[k].1.y != now[k].1.y));
        if a.y != b.y && !pushed {
            wrong.push(format!("{id} at {w}, state {at}: {name} moved though nothing over it changed, {a:?} to {b:?}"));
        }
    }
}

#[test]
#[ignore]
fn shoot_every_flipped_state() {
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));
    let prefix = std::env::var("TRINDLE_SHOT_PREFIX").unwrap_or_else(|_| "settings-".into());
    std::fs::create_dir_all(&out).unwrap();
    let (w, h) = (1278u32, 1000u32);
    for states in flipped_pages() {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages: states, current: 0,
                               search: String::new(), tall: h as f32 };
        let id = s.pages[0].id();
        s.kit.current = id.to_string();
        s.kit.icon = s.pages[0].icon();
        let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
        ui.lay_at(w, h);
        for at in 1..s.pages.len() {
            s.current = at;
            ui.set_view(s.view());
            let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
            pix.save_png(out.join(format!("{prefix}{id}-flipped-{at}.png"))).unwrap();
        }
    }
}
