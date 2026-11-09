

#[test]
#[ignore = "writes pictures when SITE_SHOTS names a folder"]
fn site_page_pictures() {
    let Some(dir) = std::env::var_os("SITE_SHOTS").map(std::path::PathBuf::from) else { return };
    std::fs::create_dir_all(&dir).unwrap();
    let page = |panel: &'static str, title: &'static str, body: Body| {
        let mut q = Quick::with(None);
        q.panel = Some(panel);
        q.head = ("preferences-system-notifications-symbolic", title);
        q.body = body;
        q
    };
    let icon_name = || "preferences-system-notifications-symbolic".to_string();
    let mut notes: Vec<(Note, String)> = [("Calendar", "Dentist at 3", "In one hour"), ("Firefox", "Download finished", "holiday-photos.zip"), ("Thunderbird", "New email from Anna", "Lunch on Sunday?")]
        .iter().enumerate().map(|(i, (app, summary, body))| (a_note(i as u32 + 1, app, summary, body), icon_name())).collect();
    notes[0].0.pinned = true;
    for n in notes.iter_mut().skip(1) {
        n.0.pinned = false;
    }
    let clips: Vec<clipboard::Item> = ["Remember to bring the charger", "14 Harbour Road, Whitby YO21 3PU", "Meet at the station at half past ten", "https://example.com/tour.html"]
        .iter().enumerate().map(|(i, t)| clipboard::Item { id: i as u64, time: 0, text: t.to_string(), picture: None, pinned: i == 1 }).collect();
    let level = |name: &'static str, out: bool| Level {
        name, target: "", out, value: 60.0, pct: "60%".into(), muted: false, touched: None,
        nodes: vec![(1, if out { "Built-in speakers" } else { "Built-in microphone" }.into()), (2, "Headphones".into())], chosen: 0,
    };
    let nets: Vec<Net> = ["Home network", "Cafe guest", "Neighbour's network"].iter().enumerate()
        .map(|(i, s)| Net { ssid: s.to_string(), signal: (90 - 20 * i) as u32, inuse: i == 0, secured: i != 1 }).collect();
    for (name, q) in [
        ("sound", page("sound", "Sound", Body::Sound(vec![level("Speakers", true), level("Microphone", false)]))),
        ("clipboard", page("clipboard", "Clipboard", Body::Clip(clips, String::new()))),
        ("notes", page("notes", "Notifications", Body::Notes(notes.clone()))),
        ("note", page("notes", "Notifications", Body::Whole(notes[2].0.clone(), icon_name()))),
        ("wifi", page("wifi", "Wi-Fi", Body::Wifi(Some(true), nets, HashSet::new()))),
        ("battery", page("power", "Battery", Body::Battery { big: "82%".into(), state: "About 5 hours 20 minutes left".into(), health: "Health 91% of new".into(), mode: Some("balanced".into()) })),
    ] {
        let mut q = q;
        q.set_size(436);
        on_disc(&q, q.disc().unwrap_or(436.0), 24.0, 1.0, true).save_png(dir.join(format!("page-{name}.png"))).unwrap();
    }
}
