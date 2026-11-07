//! Tests of the room of bubbles.

use super::*;
use crate::toolkit::input::Input;
use crate::toolkit::input::Keysym;
use super::motion::{GAP, REST, settle, thread_at};

#[test]
fn the_rim_names_the_thread_under_the_pointer() {
    let g = room(400.0, 400.0);
    let on_rim = g.rim + BAR_FULL / 2.0;
    assert_eq!(thread_at(&g, g.cx + 1.0, g.cy - on_rim, 8), Some(0));
    assert_eq!(thread_at(&g, g.cx - 1.0, g.cy - on_rim, 8), Some(7));
    assert_eq!(thread_at(&g, g.cx + on_rim, g.cy + 1.0, 4), Some(1));
    assert_eq!(thread_at(&g, g.cx, g.cy, 8), None);
    assert_eq!(thread_at(&g, g.cx, g.cy - on_rim - 60.0, 8), None);
    assert_eq!(thread_at(&g, g.cx + 1.0, g.cy - on_rim, 0), None);
}

fn overlap(items: &[(f64, f64, f64)]) -> f64 {
    let mut worst: f64 = 0.0;
    for (i, a) in items.iter().enumerate() {
        for b in &items[i + 1..] {
            worst = worst.max(a.2 + b.2 - (a.0 - b.0).hypot(a.1 - b.1));
        }
    }
    worst
}

#[test]
fn bubbles_settle_apart_and_inside_the_room() {
    let mut items: Vec<(f64, f64, f64)> = (0..14).map(|i| {
        let r = [0.35, 0.22, 0.18, 0.12, 0.1][i.min(4)];
        let a = i as f64 * 2.4;
        (0.3 * a.cos(), 0.3 * a.sin(), r)
    }).collect();
    let mut rounds = 0;
    while settle(&mut items) > REST && rounds < 1000 {
        rounds += 1;
    }
    assert!(rounds < 1000, "never came to rest");
    assert!(overlap(&items) < 0.01, "still overlapping by {}", overlap(&items));
    assert!(items.iter().all(|(x, y, r)| x.hypot(*y) + r <= 1.0 + 1e-9));
}

#[test]
fn a_new_bubble_comes_in_from_its_own_side() {
    assert_eq!(arrival("app:3", 0.1), arrival("app:3", 0.1));
    assert_ne!(arrival("app:3", 0.1), arrival("app:4", 0.1));
    let (x, y) = arrival("firefox", 0.2);
    assert!((x.hypot(y) - (1.0 - GAP - 0.2)).abs() < 1e-9);
}

fn blob(key: &str) -> Blob {
    Blob { key: key.into(), title: key.to_ascii_uppercase(), icon: Vec::new(), memory: 0.1, figure: "1 GB".into(),
           threads: 0.0, heat: 0.0, quiet: false }
}

fn press(o: &Orbit, sym: Keysym, shift: bool) -> Option<bool> {
    let mods = crate::toolkit::input::Mods { shift, ..Default::default() };
    o.key(&Input::Key { sym, text: None, mods, repeat: false })
}

#[test]
fn the_keyboard_steps_through_the_bubbles_and_opens_their_cards() {
    let o = Orbit::new(&Loop::new());
    o.set_bubbles(vec![blob("a"), blob("b"), blob("c")]);
    assert_eq!(o.key(&Input::Focus(true)), Some(false));
    assert_eq!(o.spoken(), "A, 1 GB");
    assert_eq!(press(&o, Keysym::Tab, false), Some(false));
    assert_eq!(press(&o, Keysym::Right, false), Some(false));
    assert_eq!(o.spoken(), "C, 1 GB");
    assert_eq!(press(&o, Keysym::Tab, false), None, "Tab past the last leaves the room");
    assert_eq!(press(&o, Keysym::Down, false), Some(false), "an arrow stays at the end");
    assert_eq!(press(&o, Keysym::ISO_Left_Tab, true), Some(false));
    assert_eq!(press(&o, Keysym::Return, false), Some(true));
    assert_eq!(o.picked().as_deref(), Some("b"));
    assert_eq!(o.spoken(), "B, 1 GB, card open");
    assert_eq!(press(&o, Keysym::space, false), Some(true));
    assert_eq!(o.picked(), None, "a second press puts the card away");
    assert_eq!(press(&o, Keysym::Left, false), Some(false));
    assert_eq!(press(&o, Keysym::Left, false), Some(false), "an arrow stays at the start");
    assert_eq!(press(&o, Keysym::ISO_Left_Tab, true), None, "Shift Tab before the first leaves the room");
    assert_eq!(press(&o, Keysym::Escape, false), None, "Escape is the window's");
    o.pointer(Pointer::Down(0.0, 0.0));
    assert!(o.keyed.borrow().is_none(), "the pointer takes the ring away");
    o.key(&Input::Focus(false));
    assert_eq!(o.spoken(), "Programs, each as big as the memory it holds");
}
