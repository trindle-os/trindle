//! Tests of the quick panels.

include!("../quick_site_shots_test.rs");

use super::*;
use super::battery::hours;
use super::bluetooth::{mac_of, parse_bt};
use super::clip_panel::forget_one;
use super::notifications::newest_first;
use super::shortcuts::shortcut_lines;
use super::sound::short_names;
use super::wifi::{parse_wifi, signal_level};

pub(in crate::ring) fn quick_code() -> &'static str {
    static CODE: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    CODE.get_or_init(|| {
        let files = [include_str!("battery.rs"), include_str!("bluetooth.rs"), include_str!("clip_panel.rs"), include_str!("notifications.rs"), include_str!("questions.rs"),
                     include_str!("shortcuts.rs"), include_str!("sound.rs"), include_str!("wifi.rs"), include_str!("mod.rs")];
        files.iter().map(|text| crate::common::source::code(text).split("\n#[cfg(test)]\nmod tests {").next().unwrap_or_default().replace("#[cfg(test)]\nmod tests;\n", "")).collect::<Vec<_>>().join("\n")
    })
}

#[test]
fn a_panel_keeps_its_size_when_something_is_said() {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    let panel = |said: &str| {
        let body = vec![row(vec![label("Headphones").width(Length::Fill), crate::toolkit::widget::word_button("Connect").key("connect")]).gap(ROW_GAP)];
        frame(row(vec![title("Bluetooth")]).width(Length::Shrink).key("head"), body, said, Some(340.0), None)
    };
    let height = |said: &str| Ui::new(panel(said), Theme::new(true, "#fbc711")).natural_height(340.0);
    let quiet = height("");
    assert_eq!(height("Connecting Headphones..."), quiet);
    assert_eq!(height("Joining Home..."), quiet);

    let mut ui = Ui::new(panel(""), Theme::new(true, "#fbc711"));
    ui.lay_at(340, 400);
    let place = |ui: &Ui<Msg>, key: &str| ui.boxes().into_iter().find(|b| b.0.ends_with(key)).map(|b| b.1);
    let (head, press) = (place(&ui, "head"), place(&ui, "connect"));
    assert!(press.is_some());
    ui.set_view(panel("Connecting Headphones..."));
    ui.lay_at(340, 400);
    assert_eq!((place(&ui, "head"), place(&ui, "connect")), (head, press));

    let long = "This went wrong in a way that takes a great many words to say, far more than two lines of this panel hold, \
                and then some more besides, and more again after that.";
    assert!(height(long) <= quiet + LINE_HEIGHT + 4.0, "at most one line more: {} against {quiet}", height(long));
}

#[test]
fn the_shortcut_list_is_read_row_by_row_with_headings() {
    use crate::common::key_list::{Group, Row};
    let row_of = |group, words: &str, keys: &str| Row { group, words: words.into(), keys: keys.into() };
    let rows = [row_of(Group::Trindle, "Take a screenshot", "Print, Ctrl+Alt+Shift+S"), row_of(Group::Plugin, "Flow: Switch computer", "Ctrl+Alt+S"),
                row_of(Group::Yours, "Runs foot", "Ctrl+Alt+G")];
    let said: Vec<String> = shortcut_lines(&rows).into_iter().flatten().collect();
    assert_eq!(said, ["Take a screenshot", "Print, Ctrl+Alt+Shift+S", "Plugins", "Flow: Switch computer", "Ctrl+Alt+S", "Yours", "Runs foot", "Ctrl+Alt+G"]);
    assert_eq!(shortcut_rows(&rows).len(), 5);
}

#[test]
fn a_quick_list_keeps_its_order_when_a_row_changes_state() {
    let mut order = KeptOrder::default();
    let read = |out: &str, order: &mut KeptOrder| {
        let (_, mut devices, _) = parse_bt(out);
        order.keep(&mut devices, |d| d.mac.clone());
        devices.into_iter().map(|d| (d.name, d.connected)).collect::<Vec<_>>()
    };
    let all = "on\n#all\nDevice AA:01 Mouse\nDevice AA:02 Headphones\nDevice AA:03 Keyboard\n#paired\nDevice AA:01 Mouse\n\
               Device AA:02 Headphones\nDevice AA:03 Keyboard\n#connected\n";
    let first = read(&format!("{all}Device AA:02 Headphones\n"), &mut order);
    assert_eq!(first, [("Headphones".to_string(), true), ("Keyboard".to_string(), false), ("Mouse".to_string(), false)]);

    let next = read(&format!("{all}Device AA:01 Mouse\n"), &mut order);
    assert_eq!(next, [("Headphones".to_string(), false), ("Keyboard".to_string(), false), ("Mouse".to_string(), true)]);

    let mut order = KeptOrder::default();
    let read = |out: &str, order: &mut KeptOrder| {
        let (_, _, mut nets) = parse_wifi(out);
        order.keep(&mut nets, |n| n.ssid.clone());
        nets.into_iter().map(|n| n.ssid).collect::<Vec<_>>()
    };
    assert_eq!(read("enabled\n#saved\n#near\n*:80:WPA2:Home\n :60:WPA2:Cafe\n :40::Open\n", &mut order), ["Home", "Cafe", "Open"]);
    assert_eq!(read("enabled\n#saved\n#near\n :80:WPA2:Home\n*:95:WPA2:Cafe\n :99::Open\n :99::New\n", &mut order),
               ["Home", "Cafe", "Open", "New"]);
}

#[test]
fn quick_short_names_drop_shared_words() {
    let names = vec!["Built-in Audio Analog Stereo".to_string(), "Built-in Audio HDMI".to_string()];
    assert_eq!(short_names(&names), vec!["Analog Stereo", "HDMI"]);
    let long = vec!["a".repeat(40)];
    assert_eq!(short_names(&long)[0].chars().count(), 30);
}

#[test]
fn quick_hours_read_well() {
    assert_eq!(hours(600), "10 min");
    assert_eq!(hours(3600), "1 h");
    assert_eq!(hours(5400), "1 h 30 min");
}

#[test]
fn quick_bluetooth_list_parses() {
    let out = "on\n#all\nDevice AA:BB Phone\nDevice CC:DD CC-DD\nDevice EE:FF Keyboard\n#paired\nDevice EE:FF Keyboard\n\
               #connected\nDevice AA:BB Phone\n";
    let (on, devices, names) = parse_bt(out);
    assert!(on);
    assert_eq!(names.len(), 3);
    assert_eq!(devices.iter().map(|d| d.name.as_str()).collect::<Vec<_>>(), vec!["Phone", "Keyboard"]);
    assert!(devices[0].connected && devices[1].paired);
    assert!(!parse_bt("#all\n").0);
}

#[test]
fn quick_wifi_list_parses() {
    let out = "enabled\n#saved\nHome\\:Net\n#near\n:40:WPA2:Cafe\n*:60:WPA2:Home\\:Net\n:80:WPA2:Cafe\n:30:--:Open\n:20::\n";
    let (on, saved, nets) = parse_wifi(out);
    assert!(on);
    assert!(saved.contains("Home:Net"));
    assert_eq!(nets.iter().map(|n| n.ssid.as_str()).collect::<Vec<_>>(), vec!["Home:Net", "Cafe", "Open"]);
    assert_eq!(nets[1].signal, 80);
    assert!(!nets[2].secured);
}

#[test]
fn quick_signal_and_pages() {
    assert_eq!(signal_level(76), "excellent");
    assert_eq!(signal_level(51), "good");
    assert_eq!(signal_level(26), "ok");
    assert_eq!(signal_level(25), "weak");
    assert_eq!(settings_page("notes"), "ring");
    assert_eq!(settings_page("brightness"), "display");
    assert_eq!(settings_page("audio"), "audio");
    assert_eq!(mac_of("/org/bluez/hci0/dev_AA_BB"), "AA:BB");
}

#[test]
fn the_notifications_list_starts_with_the_newest() {
    let plain = |id: u32, summary: &str| {
        let mut n = a_note(id, "Mail", summary, "");
        n.pinned = false;
        n
    };
    let mut pinned = plain(1, "old pinned");
    pinned.pinned = true;
    let list = vec![plain(4, "newest"), plain(3, "middle"), pinned, plain(2, "oldest")];
    let ids: Vec<u32> = newest_first(list).iter().map(|n| n.id).collect();
    assert_eq!(ids, vec![1, 4, 3, 2], "pinned first, then newest to oldest");
}

fn a_note(id: u32, app: &str, summary: &str, body: &str) -> Note {
    Note { id, app: app.into(), icon: String::new(), desktop: String::new(), summary: summary.into(), body: body.into(),
           at: crate::common::clock::Local::now(), urgent: false, pinned: id % 2 == 0, handoff: String::new(), drive: String::new(),
           allow: String::new() }
}

pub(in crate::ring) fn pages() -> Vec<(&'static str, Quick)> {
    const LONG: &str = "Pneumonoultramicroscopicsilicovolcanoconiosis-and-then-some-more.example.com";
    let page = |panel: &'static str, title: &'static str, body: Body| {
        let mut q = Quick::with(None);
        q.panel = Some(panel);
        q.head = ("preferences-system-notifications-symbolic", title);
        q.body = body;
        q
    };
    let icon_name = || "preferences-system-notifications-symbolic".to_string();
    let notes: Vec<(Note, String)> = (0..9).map(|i| {
        (a_note(i, "Example", &format!("John Smith sent message {i} with a long heading"), "A body that goes on for some while, to the row's end and past it"), icon_name())
    }).collect();
    let whole = a_note(4, "Example", "The build has finished and its heading is a long one", &format!("All 12 steps passed. {LONG} The pictures are in the folder."));

    let (mut drive, mut crash, mut plugin) = (whole.clone(), whole.clone(), whole.clone());
    drive.drive = "/org/example/drive".into();
    crash.handoff = "something to copy".into();
    plugin.allow = "example-plugin".into();
    let mut long_whole = whole.clone();
    long_whole.app = format!("{LONG} {LONG}");
    long_whole.summary = LONG.into();
    let level = |name: &'static str, out: bool| Level {
        name, target: "", out, value: 60.0, pct: "60%".into(), muted: false, touched: None,
        nodes: vec![(1, "Built-in Audio Analogue Stereo".into()), (2, "John Smith's Headphones Hands-Free".into())], chosen: 0,
    };
    let devs: Vec<Dev> = (0..8).map(|i| Dev { mac: format!("AA:BB:CC:DD:EE:0{i}"), name: format!("John Smith's Headphones number {i}"), paired: i < 3, connected: i == 0 }).collect();
    let nets: Vec<Net> = (0..8).map(|i| Net { ssid: format!("Example network with a long name {i}"), signal: 90 - 10 * i, inuse: i == 0, secured: i % 2 == 0 }).collect();
    let clips: Vec<clipboard::Item> = (0..8).map(|i| clipboard::Item { id: i, time: 0, text: format!("{LONG} copied text {i}"), picture: None, pinned: i == 1 }).collect();
    let mut said = page("notes", "Notifications", Body::Notes(notes.clone()));
    said.said = "This went wrong in a way that takes a great many words to say, far more than one line of this panel holds.".into();
    let asking = |code: Option<&str>, entry: Option<(&'static str, bool)>, words: &'static str| {
        let mut q = page("wifi", "Wi-Fi", Body::Wifi(Some(true), nets.clone(), HashSet::new()));
        q.ask = Some(Ask { icon: "network-wireless-symbolic", title: "Example network with a long name".into(), words, code: code.map(str::to_string),
                           entry, yes: "Join", no: "Cancel", text: Zeroizing::new(String::new()), done: None });
        q
    };
    vec![
        ("notes", page("notes", "Notifications", Body::Notes(notes))),
        ("notes-none", page("notes", "Notifications", Body::Notes(vec![]))),
        ("notes-said", said),
        ("note", page("notes", "Notifications", Body::Whole(whole, icon_name()))),
        ("note-long", page("notes", "Notifications", Body::Whole(long_whole, icon_name()))),
        ("note-drive", page("notes", "Notifications", Body::Whole(drive, icon_name()))),
        ("note-crash", page("notes", "Notifications", Body::Whole(crash, icon_name()))),
        ("note-plugin", page("notes", "Notifications", Body::Whole(plugin, icon_name()))),
        ("clipboard", page("clipboard", "Clipboard", Body::Clip(clips, String::new()))),
        ("sound", page("sound", "Sound", Body::Sound(vec![level("Speakers", true), level("Microphone", false)]))),
        ("bluetooth", page("bluetooth", "Bluetooth", Body::Bt(Some(true), devs))),
        ("bluetooth-off", page("bluetooth", "Bluetooth", Body::Bt(Some(false), vec![]))),
        ("wifi", page("wifi", "Wi-Fi", Body::Wifi(Some(true), nets.clone(), HashSet::new()))),
        ("battery", page("power", "Battery", Body::Battery { big: "82%".into(), state: "About 5 hours 20 minutes left".into(),
                                                             health: "Health 91% of new".into(), mode: Some("balanced".into()) })),
        ("battery-charging", page("power", "Battery", Body::Battery { big: "78%".into(), state: "Charging: full in about 25 min".into(),
                                                             health: "Health: good. It holds 82% of what it did when new. Charged 244 times.".into(), mode: Some("balanced".into()) })),
        ("brightness", page("brightness", "Brightness", Body::Bright(70.0, "70%".into()))),
        ("question-password", asking(None, Some(("Password", true)), "Type the password for this network.")),
        ("question-code", asking(Some("123456"), None, "Is this the code John Smith's Headphones show?")),
    ]
}

fn on_disc(q: &Quick, across: f32, room: f32, scale: f32, disc: bool) -> tiny_skia::Pixmap {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    use crate::toolkit::widget::{layers, placed};

    let room = if disc { room.max(crate::ring::view::paint::DISC_ROOM) } else { room };
    let side = across + 2.0 * room;
    let middle = |n: Node<Msg>| placed(n, |(w, h), (aw, ah)| ((aw - w) / 2.0, (ah - h) / 2.0));
    let mut all = Vec::new();
    if disc {
        all.push(middle(crate::ring::view::paint::disc(across, |t| t.raised)));
        if q.asking() {
            all.push(middle(crate::ring::view::paint::disc(q.pop_size(across as i32) as f32, |t| t.raised_more)));
        }
    }
    all.push(middle(q.view()));
    let theme = Theme::new(false, "");
    let px = (side * scale).round() as u32;
    let mut pix = tiny_skia::Pixmap::new(px, px).unwrap();
    if disc {
        pix.fill(theme.bg.to_skia());
    }
    let mut ui = Ui::new(layers(all).width(Length::Fixed(side)).height(Length::Fixed(side)), theme);
    ui.see_through = true;
    ui.paint(&mut pix.as_mut(), scale);
    pix
}

const DISCS: [f32; 7] = [340.0, 356.0, 372.0, 404.0, 436.0, 520.0, 700.0];
const SCALES: [f32; 4] = [1.0, 1.25, 1.5, 2.0];

#[test]
fn nothing_on_a_page_is_drawn_outside_its_disc() {
    let room = 40.0;
    let mut out: Vec<String> = Vec::new();
    for across in DISCS {
        for scale in SCALES {
            for (name, mut q) in pages() {
                q.set_size(across as i32);

                let on = if q.asking() { q.pop_size(across as i32) as f32 } else { q.disc().unwrap_or(across) };
                let pix = on_disc(&q, across, room, scale, false);
                let (side, c, r) = (pix.width() as usize, pix.width() as f32 / 2.0, on * scale / 2.0);
                let stray = pix.pixels().iter().enumerate().filter(|(i, p)| {
                    let (x, y) = ((i % side) as f32 + 0.5 - c, (i / side) as f32 + 0.5 - c);
                    p.alpha() > 0 && (x * x + y * y).sqrt() > r + 0.5
                }).count();
                if stray > 0 {
                    out.push(format!("{name} on a {across} disc at {scale}: {stray} pixels outside"));
                }
            }
        }
    }
    assert!(out.is_empty(), "{} of {} pages reach outside their disc:\n{}", out.len(), DISCS.len() * SCALES.len() * pages().len(), out.join("\n"));
}

#[test]
fn every_page_has_the_rings_whole_disc() {
    for (name, mut q) in pages() {
        q.set_size(436);
        assert_eq!(q.disc(), Some(436.0), "{name}");
    }
}

#[test]
fn the_rings_pages_draw_only_pairs_the_table_holds() {
    use crate::toolkit::theme::drawn;
    drawn::none(drawn::in_every_look("the ring's pages", 100, || {
        for (name, mut q) in pages() {
            q.set_size(436);
            drawn::page(name);
            let _ = on_disc(&q, 436.0, 24.0, 1.0, true);
        }
    }));
}

#[test]
fn every_button_on_every_page_stands_out_from_the_disc() {
    use crate::toolkit::colour::Colour;
    use crate::toolkit::theme::{contrast, Theme};
    use crate::toolkit::ui::Ui;
    use crate::toolkit::widget::{layers, placed};

    const LEAST: f32 = 1.15;
    let across = 436.0_f32;
    let middle = |n: Node<Msg>| placed(n, |(w, h), (aw, ah)| ((aw - w) / 2.0, (ah - h) / 2.0));
    let colour_at = |pix: &tiny_skia::Pixmap, x: f32, y: f32| {
        let p = pix.pixel(x as u32, y as u32).unwrap().demultiply();
        Colour { r: f32::from(p.red()) / 255.0, g: f32::from(p.green()) / 255.0, b: f32::from(p.blue()) / 255.0, a: 1.0 }
    };
    let (mut checked, mut faults) = (0, Vec::new());
    for light in [false, true] {
        for (name, mut q) in pages() {
            q.set_size(across as i32);
            let disc = |on: bool| {
                let mut all = vec![middle(crate::ring::view::paint::disc(across, |t| t.raised))];
                if q.asking() {
                    all.push(middle(crate::ring::view::paint::disc(q.pop_size(across as i32) as f32, |t| t.raised_more)));
                }
                if on {
                    all.push(middle(q.view()));
                }
                let mut ui = Ui::new(layers(all).width(Length::Fixed(across)).height(Length::Fixed(across)), Theme::new(light, ""));
                let mut pix = tiny_skia::Pixmap::new(across as u32, across as u32).unwrap();
                ui.see_through = true;
                ui.paint(&mut pix.as_mut(), 1.0);
                (ui, pix)
            };
            let (ui, pix) = disc(true);
            let (_, bare) = disc(false);
            let boxes = ui.boxes();
            for (id, kind) in ui.pressables() {
                if kind != "button" {
                    continue;
                }
                let Some((_, r, clip)) = boxes.iter().find(|b| b.0 == id) else { continue };
                if r.w < 8.0 || clip.is_some_and(|c| r.x < c.x || r.y < c.y || r.x + r.w > c.x + c.w || r.y + r.h > c.y + c.h) {
                    continue;
                }

                let (y, inside, outside) = (r.y + r.h / 2.0, r.x + r.w * 0.12, (r.x - 1.5).max(0.0));
                let best = [inside, outside].iter().map(|&x| contrast(colour_at(&pix, x, y), colour_at(&bare, x, y))).fold(0.0_f32, f32::max);
                if best < LEAST {
                    faults.push(format!("{name} ({}): {id} at ({:.0}, {y:.0}) stands {best:.2} from the disc", if light { "light" } else { "dark" }, r.x));
                }
                checked += 1;
            }
        }
    }
    assert!(checked >= 60, "only {checked} buttons were checked");
    assert!(faults.is_empty(), "{} buttons do not stand out:\n{}", faults.len(), faults.join("\n"));
}

#[test]
#[ignore]
fn shoot_ring_pages() {
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-b".into()));
    let prefix = std::env::var("TRINDLE_SHOT_PREFIX").unwrap_or_default();
    std::fs::create_dir_all(&out).unwrap();
    for across in [340.0, 436.0, 520.0] {
        for (name, mut q) in pages() {
            q.set_size(across as i32);
            on_disc(&q, q.disc().unwrap_or(across), 24.0, 2.0, true).save_png(out.join(format!("{prefix}ring-page-{name}-{across}.png"))).unwrap();
        }
    }
}

fn pointing(q: &Quick, across: u32) -> crate::toolkit::ui::Ui<Msg> {
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    let mut ui = Ui::new(q.view().width(Length::Fixed(across as f32)), Theme::new(false, ""));
    let mut pix = tiny_skia::Pixmap::new(across, across + 200).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    ui
}

#[test]
fn every_line_cut_short_on_every_page_shows_whole_on_pointing() {
    let (mut all, mut out) = (0, Vec::new());
    for (name, mut q) in pages() {
        q.set_size(340);
        let mut ui = pointing(&q, 340);
        let (cut, unsaid) = ui.cut_unsaid();
        all += cut;
        out.extend(unsaid.into_iter().map(|(whole, label)| format!("{name}: {whole:?} shows {label:?}")));
    }
    assert!(all >= 12, "only {all} lines that can be pointed at were cut on all the pages");
    assert!(out.is_empty(), "{} of {all} cut lines do not show whole:\n{}", out.len(), out.join("\n"));
}

#[test]
fn a_thing_copied_shows_whole_on_pointing_with_its_line_breaks() {
    const LINES: &str = "Dear John Smith,\n\n    Thank you for the letter.\nYours";
    const LONG: &str = "A sentence copied from a page that runs on for a good deal longer than one row of the list of things copied";
    let item = |id: u64, text: &str, picture: Option<&str>| clipboard::Item { id, time: 0, text: text.into(), picture: picture.map(str::to_string), pinned: false };
    let mut q = Quick::with(None);
    q.panel = Some("clipboard");
    q.head = ("edit-paste-symbolic", "Clipboard");
    q.body = Body::Clip(vec![item(1, LINES, None), item(2, "Short", None), item(3, LONG, None), item(4, "", Some("1.png")), item(5, "\nTwo words\n", None)], String::new());
    q.set_size(436);
    let mut ui = pointing(&q, 436);
    let mut seen: Vec<String> = Vec::new();
    let mut x = 30.0;
    while x < 436.0 {
        let mut y = 0.0;
        while y < 636.0 {
            if let Some(label) = ui.label_over(x, y) {
                if !seen.contains(&label) {
                    seen.push(label);
                }
            }
            y += 3.0;
        }
        x += 12.0;
    }
    for whole in [LINES, LONG] {
        assert!(seen.iter().any(|l| l == whole), "{whole:?} is not among {seen:?}");
    }
    for none in ["Short", "A picture", "Two words", "\nTwo words\n"] {
        assert!(!seen.iter().any(|l| l == none), "{none:?} is a label: {seen:?}");
    }
    for name in ["Copy this again", forget_one()] {
        assert!(seen.iter().any(|l| l == name), "{name:?} is not among {seen:?}");
    }
}

#[test]
#[ignore]
fn shoot_clipboard_labels() {
    use crate::toolkit::input::Input;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{tip_rest, Ui};
    use crate::toolkit::widget::{layers, placed};
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-b".into()));
    std::fs::create_dir_all(&out).unwrap();
    let letter: String = (1..=40).map(|i| format!("Line {i} of a letter to John Smith at john.smith@example.com")).collect::<Vec<_>>().join("\n");
    let texts = ["Meet at four", "A sentence copied from a page that runs on for a good deal longer than one row of the list of things copied has room for",
                 "Dear John Smith,\n\n    Thank you for the letter.\nYours", letter.as_str(), "07700 900123"];
    let items: Vec<clipboard::Item> = texts.iter().enumerate()
        .map(|(i, t)| clipboard::Item { id: i as u64, time: 0, text: t.to_string(), picture: None, pinned: i == 0 }).collect();
    for (row, name) in [(0, "clipboard-short-item"), (1, "clipboard-item-cut-on-its-line"), (2, "clipboard-item-of-a-few-lines"), (3, "clipboard-long-item-at-the-line-limit")] {
        let mut q = Quick::with(None);
        q.panel = Some("clipboard");
        q.head = ("edit-paste-symbolic", "Clipboard");
        q.body = Body::Clip(items.clone(), String::new());
        q.set_size(436);
        let (across, room, scale) = (436.0_f32, 60.0_f32, 2.0_f32);
        let side = across + 2.0 * room;
        let middle = |n: Node<Msg>| placed(n, |(w, h), (aw, ah)| ((aw - w) / 2.0, (ah - h) / 2.0));
        let theme = Theme::new(false, "");
        let px = (side * scale).round() as u32;
        let mut pix = tiny_skia::Pixmap::new(px, px).unwrap();
        pix.fill(theme.bg.to_skia());
        let mut ui = Ui::new(layers(vec![middle(crate::ring::view::paint::disc(across, |t| t.raised)), middle(q.view())])
                                 .width(Length::Fixed(side)).height(Length::Fixed(side)), theme);
        ui.see_through = true;
        ui.paint(&mut pix.as_mut(), scale);

        let x = px as f32 * 0.4;
        let mut rows: Vec<(f32, f32)> = Vec::new();
        let (mut y, mut from) = (0.0, None);
        while y < px as f32 {
            match (ui.pressable_at(x, y), from) {
                (true, None) => from = Some(y),
                (false, Some(top)) => {
                    rows.push((x, (top + y) / 2.0));
                    from = None;
                }
                _ => {}
            }
            y += 2.0;
        }
        let (x, y) = rows[row];
        let now = std::time::Instant::now();
        ui.input(&Input::PointerIn { x: x / scale, y: y / scale }, now);
        ui.tick(now + tip_rest());
        pix.fill(Theme::new(false, "").bg.to_skia());
        ui.paint(&mut pix.as_mut(), scale);
        pix.save_png(out.join(format!("{name}.png"))).unwrap();
    }
}
