//! Tests: colours.

#[cfg(test)]
use super::*;

#[test]
fn a_page_inside_apps_keeps_its_place_when_its_i_opens() {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut hosted = Hosted::new(&events, asks, None, None);
    let (w, h) = (1000u32, 500u32);
    for i in 0..hosted.s.pages.len() {
        hosted.s.current = i;
        let name = format!("page:{}", hosted.s.pages[i].title());
        let mut ui = Ui::new(hosted.view(), Theme::new(true, "#fbc711"));
        ui.lay_at(w, h);
        ui.scroll_area(&name, 1e9);
        let (boxes, places) = (ui.boxes(), ui.places());
        for open in [true, false] {
            hosted.s.kit.info = open.then(|| ("page".to_string(), "What this page is for.".to_string(), crate::toolkit::widget::Rect::default()));
            ui.set_view(hosted.view());
            ui.lay_at(w, h);
            assert_eq!(ui.places(), places, "{name}, its i {open}");
            let now_boxes = ui.boxes();
            assert!(boxes.iter().all(|b| now_boxes.contains(b)), "{name}, its i {open}");
        }
    }
}

#[test]
fn no_page_uses_more_than_two_colours() {
    use crate::toolkit::colour::Colour;
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut pages = panels::all();
    pages.extend([
        Box::new(panels::crash_reports::CrashReports::default()) as Box<dyn Page>,
        Box::new(panels::plugins::Plugins::default()),
    ]);
    let mut s = Settings { kit: Kit::new(events.clone(), asks, None, None), pages, current: 0, search: String::new(), tall: 800.0 };
    let accent = Colour::parse("#fbc711").unwrap();
    let status = [theme::good(), theme::problem()];
    let offered: Vec<Colour> = panels::appearance::colours::highlights().iter().filter_map(|(hex, _)| Colour::parse(hex)).collect();
    let mut wrong = Vec::new();
    for i in 0..s.pages.len() {
        s.current = i;
        let id = s.pages[i].id();
        s.kit.icon = s.pages[i].icon();
        let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
        let mut pix = tiny_skia::Pixmap::new(1280, 800).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        let found = hues_in(&pix, accent, &status, &offered);
        if found.len() > 2 {
            wrong.push(format!("{id}: {} colours besides the highlight (slices {found:?})", found.len()));
        }
    }
    wrong.dedup();
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn only_the_pickers_own_colours_are_let_through_the_colour_count() {
    use crate::toolkit::colour::Colour;
    let (blue, other) = (Colour::parse("#2295f1").unwrap(), Colour::parse("#2296f1").unwrap());
    let accent = Colour::parse("#fbc711").unwrap();
    let fill = |c: Colour| {
        let mut pix = tiny_skia::Pixmap::new(40, 40).unwrap();
        pix.fill(c.to_skia());
        pix
    };
    assert!(hues_in(&fill(blue), accent, &[], &[]).len() == 1);
    assert!(hues_in(&fill(blue), accent, &[], &[blue]).is_empty());
    assert_eq!(hues_in(&fill(other), accent, &[], &[blue]).len(), 1);
}

#[test]
#[ignore]
fn shoot_every_page() {
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));
    let size = std::env::var("TRINDLE_SHOT_SIZE").unwrap_or_else(|_| "1280x800".into());
    let (w, h) = size.split_once('x').and_then(|(a, b)| Some((a.parse::<f32>().ok()?, b.parse::<f32>().ok()?))).unwrap_or((1280.0, 800.0));
    let scale: f32 = std::env::var("TRINDLE_SHOT_SCALE").ok().and_then(|v| v.parse().ok()).unwrap_or(1.0);
    let only = std::env::var("TRINDLE_SHOT_PAGES").unwrap_or_default();
    let prefix = std::env::var("TRINDLE_SHOT_PREFIX").unwrap_or_default();
    std::fs::create_dir_all(&out).unwrap();
    let events = Loop::new();

    let heard: Rc<RefCell<Vec<Do>>> = Rc::default();
    let h2 = heard.clone();
    let asks = events.post(move |d: Do| h2.borrow_mut().push(d));
    let system = std::env::var_os("TRINDLE_SHOT_SYSTEM").and_then(|_| crate::toolkit::bus::Bus::system(&events).ok());
    let mut pages = panels::all();
    pages.extend([
        Box::new(panels::crash_reports::CrashReports::default()) as Box<dyn Page>,
        Box::new(panels::plugins::Plugins::default()),
    ]);
    let mut s = Settings { kit: Kit::new(events.clone(), asks, None, system), pages, current: 0, search: String::new(), tall: h };
    for i in 0..s.pages.len() {
        let id = s.pages[i].id();
        if !only.is_empty() && !only.split(',').any(|o| o == id) {
            continue;
        }
        s.current = i;
        s.kit.current = id.to_string();
        s.kit.icon = s.pages[i].icon();
        s.pages[i].shown(&mut s.kit);
        for _ in 0..12 {
            let quit = events.clone();
            events.after(Duration::from_millis(300), move || quit.quit());
            events.run();
            let got: Vec<Do> = heard.borrow_mut().drain(..).collect();
            for d in got {
                if let Do::Then(f) = d {
                    f(&mut s);
                }
            }
        }
        let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
        let mut pix = tiny_skia::Pixmap::new((w * scale) as u32, (h * scale) as u32).unwrap();
        ui.paint(&mut pix.as_mut(), scale);
        pix.save_png(out.join(format!("{prefix}{id}.png"))).unwrap();
    }
}
