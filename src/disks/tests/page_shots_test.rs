//! Pictures of Drives' failures for the help page: each feeds the real window the real state and words.

use super::*;
use crate::disks::{help, Ring};

fn snap(name: &str, d: &Drives) {
    let (w, h) = (1280u32, 960u32);
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots-a".into()));
    std::fs::create_dir_all(&out).unwrap();
    let mut ui = Ui::new(d.view(), Theme::new(false, "#fbc711"));
    let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    pix.save_png(out.join(format!("drives-{name}.png"))).unwrap();
}

fn read_only_world() -> room::World {
    room::World { mountinfo: "36 1 259:2 /@rootfs / ro,relatime shared:1 - btrfs /dev/nvme9n1p2 ro,ssd\n".into(), ..made_up_world() }
}

fn made(mut d: Drives) -> Drives {
    let g = 1_000_000_000u64;
    d.rings = vec![Ring { name: t!("disks-ring-system").into(), icon: "computer-symbolic", share: 0.3, free: 700 * g, whole: 1000 * g, words: "system" }];
    d.headline = headline(&d.drives, Some((300 * g, 700 * g)));
    d
}

fn open(d: &mut Drives, part: &str) {
    d.pressed_at = Rect { x: 400.0, y: 300.0, w: 34.0, h: 34.0 };
    d.update(Msg::Toggle(part.into()));
}

#[test]
#[ignore = "makes the pictures of Drives' failures for the help page"]
fn shoot_drives_failures() {

    let base = sample();
    let mut d = Drives::new(&base.events, base.asks.clone());
    d.missing = Some(("The drive service", "udisks2"));
    snap("service-missing", &d);

    let mut d = made(sample());
    d.say("/d/nvme", "/b/Data", t!("disks-no-password").into(), false);
    snap("no-password", &d);
    let mut d = made(sample());
    let steps = [plan::Did { what: "unmounting Photos".into(), changes: true, mounts: None },
                 plan::Did { what: "resizing what is on Photos".into(), changes: true, mounts: None }];
    let words = plan::run_stopped(&steps, 1, &plan::Stopped { why: "Something is still using it.".into(), refused: false });
    d.say("/d/usb", "/b/Photos", words, false);
    snap("part-way", &d);

    let mut d = made(room_window("btrfs", false));
    d.world = read_only_world;
    let (_, part) = system_part(&d);
    open(&mut d, &part);
    snap("room-refused", &d);

    let mut d = made(sample());
    d.texts.insert("/b/Photos:folder".into(), "holiday photos".into());
    d.folder("/d/usb", "/b/Photos");
    open(&mut d, "/b/Photos");
    snap("name-refused", &d);

    let mut d = made(sample());
    d.drives[2].name = "Old disk".into();
    d.drives[2].health = Some(Health { failing: true, troubles: vec!["has made itself read-only".into()], kelvin: 305.0, hours: 40_000 });
    d.headline = headline(&d.drives, None);
    snap("failing", &d);

    let mut d = made(sample());
    let g = 1_000_000_000u64;
    let ring = |name: &str, icon, words, used: u64, free: u64| Ring { name: name.into(), icon, share: used as f64 / (used + free) as f64, free, whole: used + free, words };
    d.rings = vec![ring(t!("disks-ring-system"), "computer-symbolic", "system", 60 * g, 39 * g),
                   ring(t!("disks-ring-files"), "folder-symbolic", "files", 599 * g, 400_000_000)];
    d.drives.truncate(1);
    d.headline = headline(&d.drives, Some((599 * g, 400_000_000)));
    snap("nearly-full", &d);

    let mut d = made(sample());
    std::fs::remove_dir_all(d.hidden_file.parent().unwrap()).ok();
    std::fs::create_dir_all(d.hidden_file.parent().unwrap().parent().unwrap()).unwrap();
    std::fs::write(d.hidden_file.parent().unwrap(), b"in the way").unwrap();
    d.update(Msg::Hide { drive: "/d/usb".into(), row: String::new(), key: "drive:SanDisk-Cruzer-0002".into(), hide: true });
    snap("hide-failed", &d);
    std::fs::remove_file(d.hidden_file.parent().unwrap()).unwrap();
}

#[test]
#[ignore = "makes the more pictures of Drives' failures for the help page"]
fn shoot_drives_failures_more() {
    let said = |d: &mut Drives, drive: &str, row: &str, words: String| d.say(drive, row, words, false);

    let mut d = made(made_up_window("shots"));
    let key = d.drives.iter().find(|x| x.name == SECOND).map(|x| x.key.clone()).unwrap();
    d.update(Msg::Hide { drive: String::new(), row: String::new(), key, hide: true });
    snap("hidden", &d);
    let _ = std::fs::remove_dir_all(d.hidden_file.parent().unwrap());

    let mut d = made(room_window("btrfs", false));
    let (_, part) = system_part(&d);
    open(&mut d, &part);
    snap("room-slider", &d);
    let mut d = made(room_window("ext4", false));
    d.world = made_up_world;
    let (_, part) = system_part(&d);
    open(&mut d, &part);
    snap("room-other-kind", &d);
    let mut d = made(room_window("btrfs", false));
    d.world = || room::World { swaps: "Filename\tType\tSize\tUsed\tPriority\n/swapfile file 100 0 -2\n".into(), ..made_up_world() };
    let (_, part) = system_part(&d);
    open(&mut d, &part);
    snap("room-swap-file", &d);

    let mut d = made(sample());
    said(&mut d, "/d/old", "/b/Backup", plan::no_service().into());
    snap("no-service", &d);
    let mut d = made(sample());
    said(&mut d, "/d/usb", "/b/Photos", plan::one_at_a_time().into());
    snap("one-at-a-time", &d);
    let mut d = made(sample());
    said(&mut d, "/d/nvme", "/b/Data", plan::refusal_words(crate::common::refused::Refused::Wrong));
    snap("password-wrong", &d);

    let mut d = made(sample());
    let step = |what: &str| plan::Did { what: what.into(), changes: true, mounts: None };
    let steps = [step("unmounting Photos"), step("checking Photos"), step("resizing what is on Photos"), step("mounting Photos")];
    let words = plan::run_stopped(&steps, 2, &plan::Stopped { why: t!("disks-busy").into(), refused: false });
    said(&mut d, "/d/usb", "/b/Photos", words);
    snap("part-way-lists", &d);
    let mut d = made(sample());
    said(&mut d, "/d/usb", "/b/Photos", t!("disks-no-room").into());
    snap("no-room", &d);
    let mut d = made(sample());
    said(&mut d, "/d/usb", "/b/Photos", plan::folder_not_kept("/mnt/photos", t!("disks-new-part-missing")));
    snap("folder-not-kept", &d);

    snap("needs-a-look", &made(sample()));

    let mut d = made(room_window("btrfs", false));
    let (dp, vp) = system_part(&d);
    d.say_on(&dp, &format!("{vp}:room"), t!("disks-room-too-little", used = "299.0 GB"), false, help::ROOM);
    open(&mut d, &vp);
    snap("room-too-little", &d);
    let mut d = made(room_window("btrfs", false));
    let (dp, vp) = system_part(&d);
    d.say_on(&dp, &format!("{vp}:room"), t!("disks-room-stopped", what = t!("disks-room-check-unmounted")), false, help::ROOM);
    open(&mut d, &vp);
    snap("room-stopped", &d);
    let mut d = made(sample());
    said(&mut d, "/d/nvme", "/b/Data", t!("disks-standing").into());
    snap("password-standing", &d);
}
