//! Tests of the ScreenCast portal.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use cast_capture::{Screen, Window};
use zbus::zvariant::{OwnedObjectPath, OwnedValue};

use super::{offer, sender_of, wait_for, Cast, FRAME_GAP, INTERFACE, Kept, MONITOR, SESSION, Session, Wants};
use super::desktop::{desktop_layout, kept, place, recall};
use super::methods::{restore_of, started};
use super::super::cast_capture;
use super::super::cast_pick::Choice;
use super::super::portal::{DONE, FAILED};
use super::super::portal_ask::Options;

fn screen(name: &str, place: [i32; 4], pixels: (i32, i32)) -> Screen {
    Screen { name: name.into(), description: String::new(), place, pixels }
}

#[test]
fn restore_data_given_out_reads_back_through_the_bus() {
    let screens = vec![screen("eDP-1", [0, 0, 10, 10], (10, 10))];
    let s = Session {
        path: String::new(), app_name: String::new(), app_pid: Cell::new(None),
        wants: RefCell::new(Wants { persist: 2, ..Wants::default() }), casts: RefCell::new(Vec::new()),
        served: RefCell::new(None), picker: RefCell::new(None), request: RefCell::new(None),
        started: Cell::new(false), closed: Cell::new(false),
    };
    let cast = Rc::new(Cast::new(Choice::Screen(screens[0].clone()), Vec::new(), false, (10, 10), None));
    cast.node.set(Some(7));
    let (code, results) = started(&s, &[cast], &screens);
    assert_eq!(code, DONE);
    let restore = results.get("restore_data").expect("kept").try_clone().expect("plain");
    let options = HashMap::from([("restore_data".to_string(), restore)]);
    let msg = zbus::Message::method_call("/x", "M").and_then(|b| b.build(&(options,))).expect("message");
    let (back,): (Options,) = msg.body().deserialize().expect("read");
    assert_eq!(restore_of(&back), Some(vec![("screen".to_string(), "eDP-1".to_string(), String::new(), String::new())]));
    let none: Options = HashMap::new();
    assert_eq!(restore_of(&none), None);
}

#[test]
fn pictures_wait_their_turn_and_no_longer() {
    let gap = Duration::from_millis(16);
    let now = Instant::now();
    assert_eq!(wait_for(None, gap, now), Duration::ZERO, "the first goes at once");
    assert_eq!(wait_for(Some(now), gap, now), gap);
    assert_eq!(wait_for(Some(now), gap, now + Duration::from_millis(10)), Duration::from_millis(6));
    assert_eq!(wait_for(Some(now), gap, now + Duration::from_millis(40)), Duration::ZERO);
    let cast = Cast::new(Choice::Window(cast_capture::Window { id: "1".into(), title: String::new(), app_id: String::new() }, None),
                         Vec::new(), false, (2235, 1482), None);
    assert_eq!(cast.gap.get(), FRAME_GAP);
    assert!(cast.held.borrow().is_none() && !cast.pacing.get());
}

#[test]
fn the_sender_is_read_from_the_handle() {
    assert_eq!(sender_of("/org/freedesktop/portal/desktop/session/1_42/brave1").as_deref(), Some(":1.42"));
    assert_eq!(sender_of("/x"), None);
}

#[test]
fn the_desktop_is_every_screen_at_the_sharpest_scale() {
    let screens = [screen("A", [0, 100, 1920, 1080], (1920, 1080)), screen("B", [1920, 0, 1920, 1080], (3840, 2160))];
    let (size, at) = desktop_layout(&screens);
    assert_eq!(size, (7680, 2360));
    assert_eq!(at, vec![(0, 200, 3840, 2160), (3840, 0, 3840, 2160)]);
}

#[test]
fn screens_are_placed_and_the_rest_is_black() {
    let (w, h, stride) = (4u32, 2u32, 16u32);
    let mut out = vec![0u8; (stride * h) as usize];
    let src = vec![9u8; 2 * 2 * 4];
    place(&mut out, stride, (w, h), &src, (2, 2), (1, 0, 2, 2));
    assert_eq!(&out[0..4], &[0, 0, 0, 0]);
    assert_eq!(&out[4..12], &[9; 8]);
    assert_eq!(&out[12..16], &[0, 0, 0, 0]);
    let mut out = vec![0u8; (stride * h) as usize];
    place(&mut out, stride, (w, h), &src, (2, 2), (2, 0, 4, 4));
    assert_eq!(&out[8..16], &[9; 8]);
    assert_eq!(&out[16 + 8..32], &[9; 8]);
}

#[test]
fn a_kept_choice_comes_back_only_while_its_things_are_there() {
    let screens = vec![screen("HDMI-A-1", [0, 0, 10, 10], (10, 10)), screen("eDP-1", [10, 0, 10, 10], (10, 10))];
    let windows = vec![
        Window { id: "a".into(), title: "Notes".into(), app_id: "org.example.Notes".into() },
        Window { id: "b".into(), title: "Inbox".into(), app_id: "org.example.Mail".into() },
    ];
    let keep: Vec<Kept> = vec![kept(&Choice::Screen(screens[1].clone())), kept(&Choice::Window(windows[0].clone(), None)), kept(&Choice::Desktop)];
    let back = recall(&keep, &screens, &windows).expect("all there");
    assert_eq!(back.len(), 3);
    assert!(matches!(&back[1], Choice::Window(w, _) if w.id == "a"));
    let renamed = vec![Window { id: "c".into(), title: "Notes - draft".into(), app_id: "org.example.Notes".into() }];
    assert!(recall(&keep[1..2], &screens, &renamed).is_some());
    assert!(recall(&keep[0..1], &screens[..1], &windows).is_none());
    assert!(recall(&keep[2..3], &screens[..1], &windows).is_none());
    assert!(recall(&[], &screens, &windows).is_none());
}

#[test]
fn the_interface_answers_and_a_session_opens_and_closes() {
    use super::super::portal_ask::testkit::{call, props, Reply, Rig};
    use crate::toolkit::bus::get_u32;
    let rig = Rig::new();
    let _offered = rig.a.serve(super::super::portal::PATH, offer(&rig.kit())).expect("served");
    let p = props(&rig, INTERFACE).expect("props");
    assert_eq!((get_u32(&p, "version"), get_u32(&p, "AvailableSourceTypes"), get_u32(&p, "AvailableCursorModes")), (5, 3, 7));
    let session = OwnedObjectPath::try_from("/org/freedesktop/portal/desktop/session/1_9/t1").unwrap();
    let handle = OwnedObjectPath::try_from("/org/freedesktop/portal/desktop/request/1_9/t1").unwrap();
    let got: Option<Reply> = call(&rig, INTERFACE, "CreateSession", &(handle.clone(), session.clone(), "", Options::new()));
    assert_eq!(got.map(|r| r.0), Some(DONE));
    let mut options = Options::new();
    options.insert("types".into(), OwnedValue::from(MONITOR));
    options.insert("persist_mode".into(), OwnedValue::from(2u32));
    let got: Option<Reply> = call(&rig, INTERFACE, "SelectSources", &(handle.clone(), session.clone(), "", options));
    assert_eq!(got.map(|r| r.0), Some(DONE));
    let got: Option<Reply> = call(&rig, INTERFACE, "Start", &(handle.clone(), session.clone(), "", "", Options::new()));
    assert_eq!(got.map(|r| r.0), Some(FAILED));
    assert_eq!(rig.stay.busy.get(), 1, "an open session keeps the portal");
    let done = std::rc::Rc::new(std::cell::Cell::new(false));
    let d = done.clone();
    rig.b.call(&rig.a.unique_name(), session.as_str(), SESSION).method("Close", &(), false, None, &rig.lp, move |_| d.set(true));
    let d = done.clone();
    assert!(crate::toolkit::bus::testing::until(&rig.lp, move || d.get()));
    assert_eq!(rig.stay.busy.get(), 0, "a closed session lets the portal go");
}
