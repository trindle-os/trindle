//! Tests for the Wallpaper page.

use super::*;

fn page_code() -> String {
    [include_str!("looking.rs"), include_str!("cards.rs"), include_str!("../wallpaper.rs")].iter()
        .map(|text| crate::common::source::code(text).split("\n#[cfg(test)]").next().unwrap_or_default()).collect::<Vec<_>>().join("\n")
}

#[test]
fn nothing_on_the_pages_own_thread_looks_at_the_disk() {
    const ASIDE: [&str; 10] = ["decode", "look", "showing_words", "said_words", "rotating_source", "open_film", "step_film", "next_frame", "screen_size", "vet"];
    let code = page_code();
    let code = code.split("\n#[cfg(test)]").next().unwrap();
    let mut own = String::new();
    for (i, part) in code.split("\nfn ").enumerate() {
        let name = part.split(['(', '<']).next().unwrap_or_default();
        if i > 0 && ASIDE.contains(&name) {
            let end = part.find("\n}\n").map_or(part.len(), |at| at + 3);
            own.push_str(&part[end..]);
        } else {
            own.push_str(part);
        }
        own.push('\n');
    }
    for looks in [".is_dir()", ".is_file()", ".exists()", "metadata(", "read_dir(", "looked_in(", "pictures_in(", "pictures_used(",
                  "showing()", "limits::over", "moving::info(", "Player::open(", "Clip::open(", "first_frame(", "picture::load(", "next_small("] {
        assert!(!own.contains(looks), "the Wallpaper page calls {looks} on its own thread");
    }
    for name in ASIDE {
        let called = own.matches(&format!("{name}(")).count();
        let handed = own.matches(&format!("on_thread(move || {name}(")).count();
        assert_eq!(called, handed, "{name} is called {called} times on the page's own thread, and handed to a thread {handed} times");
    }
    assert!(own.contains("on_thread(move || look(") && own.contains("on_thread(move || open_film(") && own.contains("on_thread(move || rotating_source("));
}

#[test]
fn a_picture_is_named_as_people_say_it() {
    assert_eq!(shown_name(std::path::Path::new("/x/2026-10-the-moving-mosaic.webm")), "The moving mosaic");
    assert_eq!(shown_name(std::path::Path::new("/x/2026-09-the-cafe-at-night.jpg")), "The cafe at night");
    assert_eq!(shown_name(std::path::Path::new("/x/holiday_beach.png")), "Holiday beach");
    assert_eq!(shown_name(std::path::Path::new("/x/2026.jpg")), "2026");
}

#[test]
fn every_way_a_wallpaper_is_chosen_asks_the_one_rule() {
    for (file, code, asks) in [
        ("settings/panels/wallpaper.rs", page_code().as_str(), "limits::over("),
        ("session/wallpaper.rs", crate::common::source::code(include_str!("../../../session/wallpaper.rs")), "wallpaper_limits::over("),
        ("screenshot/portal_wallpaper.rs", crate::common::source::code(include_str!("../../../screenshot/portal_wallpaper.rs")), "wallpaper_limits::over("),
    ] {
        assert!(code.contains(asks), "{file} does not ask the wallpaper size rule");
    }
    let own = page_code();
    let own = own.split("\n#[cfg(test)]").next().unwrap();
    assert_eq!(own.matches("None => self.set(kit, \"wallpaper\", &path").count(), 1, "a picture is set without being vetted");
}
