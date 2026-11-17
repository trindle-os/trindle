//! Tests: changes.

#[cfg(test)]
use super::*;

#[test]
fn room_is_offered_for_btrfs_and_another_kind_says_why_not() {
    let d = room_window("btrfs", false);
    let (_, vol) = system_part(&d);
    let v = d.vol(&d.drives[0].path, &vol).unwrap().1.clone();
    let (facts, ids) = d.room_facts(&v, false);
    assert!(ids.is_some());
    assert!(matches!(room::offer(&facts), room::Offer::Make { .. }), "{facts:?}");
    let kinds = d.kinds();
    assert_eq!(d.part_extras(&d.drives[0], &v, &kinds).len(), 1, "the system part has this row and no other");
    for (kind, words) in [("ext4", "isn't in use"), ("xfs", t!("disks-room-xfs-fixed"))] {
        let d = room_window(kind, false);
        let v = d.drives[0].vols.iter().find(|v| v.part.is_some()).unwrap().clone();
        let (facts, _) = d.room_facts(&v, false);
        let room::Offer::Not(why) = room::offer(&facts) else { panic!("{kind} gets a label") };
        assert!(why.contains(kind) && why.contains(words), "{why}");
        assert_eq!(d.part_extras(&d.drives[0], &v, &d.kinds()).len(), 1, "only the label, no way to erase anything");
    }

    let mut other = room_window("btrfs", false);
    other.world = || room::World { snapper: false, ..made_up_world() };
    let v = other.drives[0].vols.iter().find(|v| v.part.is_some()).unwrap().clone();
    let (facts, _) = other.room_facts(&v, false);
    assert_eq!(room::offer(&facts), room::Offer::Not(room::no_snapshot().into()));
}

#[test]
fn making_room_takes_a_snapshot_first_then_every_step_asks_and_is_checked() {
    for (encrypted, methods) in [
        (false, vec!["Verify", "Resize", "Verify", "Resize", "Verify"]),
        (true, vec!["Verify", "Resize", "Verify", "Resize", "Verify", "Resize", "Verify"]),
    ] {
        let mut d = room_window("btrfs", encrypted);
        let (dp, vp) = system_part(&d);
        d.gbs.insert(format!("{vp}:room"), 100.0);
        let act = Act::Room { drive: dp.clone(), vol: vp.clone() };
        d.act(act.clone());
        assert!(d.snaps.is_empty() && d.ran.is_empty(), "the first press only arms it");
        d.act(act);
        assert_eq!(d.snaps.len(), 1, "a snapshot is asked for");
        assert!(d.snaps[0].contains("nvme9n1p2"));
        assert!(d.ran.is_empty(), "nothing is sent to UDisks before the snapshot is known");
        assert!(d.snapping && d.busy == 1);

        add_a_part_that_can_grow(&mut d);
        let (other, part) = ("/d/grow".to_string(), "/b/Grow".to_string());
        d.act(Act::Delete { drive: other.clone(), vol: part.clone() });
        d.act(Act::Delete { drive: other.clone(), vol: part });
        assert_eq!(d.told(&other, "").map(|(w, _)| w).as_deref(), Some(plan::one_at_a_time()));
        d.handle(Do::Snapped { drive: dp.clone(), vol: vp.clone(), row: format!("{vp}:room"), want: 100 * G, desc: d.snaps[0].clone(), res: Ok(()) });
        assert!(!d.snapping);
        let ran = d.ran.last().expect("the steps were sent");
        assert_eq!(ran.methods, methods, "encrypted: {encrypted}");
        assert!(ran.asked, "a password is asked for before the steps");
    }
}

#[test]
fn a_swap_file_is_switched_off_for_making_room_and_back_on_however_it_ends() {
    let swapfile = vec!["/swap/swapfile".to_string()];
    let mut rig = Rig::new();
    let room = room_window("btrfs", false);
    let d = &mut rig.d;
    (d.drives, d.objects, d.caps) = (room.drives, room.objects, room.caps);
    d.world = || room::World { swaps: Some("Filename Type\n/swap/swapfile file 100 0 -2\n".into()), ..made_up_world() };
    let (dp, vp) = system_part(d);
    let row = format!("{vp}:room");
    d.gbs.insert(row.clone(), 100.0);
    let v = d.vol(&dp, &vp).unwrap().1.clone();
    assert!(matches!(room::offer(&d.room_facts(&v, false).0), room::Offer::Make { .. }), "a swap file is no refusal");
    d.act(Act::Room { drive: dp.clone(), vol: vp.clone() });
    d.act(Act::Room { drive: dp.clone(), vol: vp.clone() });
    d.handle(Do::Snapped { drive: dp.clone(), vol: vp.clone(), row: row.clone(), want: 100 * G, desc: String::new(), res: Ok(()) });
    assert_eq!(d.swaps, [(false, swapfile.clone())]);
    assert!(d.ran.is_empty(), "nothing is resized while the swap file is on");
    assert_eq!(d.told(&dp, &row).map(|(w, _)| w).as_deref(), Some("Switching off the swap file…"));

    add_a_part_that_can_grow(d);
    d.act(Act::Delete { drive: "/d/grow".into(), vol: "/b/Grow".into() });
    d.act(Act::Delete { drive: "/d/grow".into(), vol: "/b/Grow".into() });
    assert_eq!(d.told("/d/grow", "").map(|(w, _)| w).as_deref(), Some(plan::one_at_a_time()));
    d.handle(Do::SwappedOff { drive: dp.clone(), vol: vp.clone(), row: row.clone(), want: 100 * G, files: swapfile.clone(), res: Ok(String::new()) });
    assert_eq!(d.ran.last().map(|r| r.methods.clone()), Some(vec!["Verify", "Resize", "Verify", "Resize", "Verify"]));
    assert!(d.swapped.contains_key(&spot(&dp, &row)), "the run began");

    assert!(rig.until(|d| d.swaps.len() == 2), "the swap file is asked back on as the run ends");
    let d = &mut rig.d;
    assert_eq!(d.swaps.last(), Some(&(true, swapfile.clone())));
    assert_eq!(d.told(&dp, &row).map(|(w, _)| w).as_deref(), Some("Switching the swap file back on…"));
    assert!(d.busy == 1 && d.snapping, "the work is not over until the swap file is back");
    let ended: Result<String, String> = Err("Example reason.".into());
    d.handle(Do::SwappedOn { drive: dp.clone(), row: row.clone(), res: ended.clone(), standing: false });
    assert_eq!(d.told(&dp, &row), Some(("Example reason.".to_string(), Tone::Problem)));
    assert!(d.busy == 0 && !d.snapping && d.swapped.is_empty());
}

#[test]
fn a_swap_file_left_off_or_not_switched_off_is_never_silent() {
    let swapfile = vec!["/swap/swapfile".to_string()];
    let mut d = room_window("btrfs", false);
    d.world = || room::World { swaps: Some("Filename Type\n/swap/swapfile file 100 0 -2\n".into()), ..made_up_world() };
    let (dp, vp) = system_part(&d);
    let row = format!("{vp}:room");
    d.gbs.insert(row.clone(), 100.0);
    let press = |d: &mut Drives| {
        d.act(Act::Room { drive: dp.clone(), vol: vp.clone() });
        d.act(Act::Room { drive: dp.clone(), vol: vp.clone() });
        d.handle(Do::Snapped { drive: dp.clone(), vol: vp.clone(), row: row.clone(), want: 100 * G, desc: String::new(), res: Ok(()) });
    };
    press(&mut d);
    d.handle(Do::SwappedOff { drive: dp.clone(), vol: vp.clone(), row: row.clone(), want: 100 * G, files: swapfile.clone(), res: Err("swapoff: Cannot allocate memory".into()) });
    let (said, tone) = d.told(&dp, &row).expect("said");
    assert!(said.contains("Cannot allocate memory") && said.contains("Nothing was resized") && tone == Tone::Problem, "{said}");
    assert!(d.ran.is_empty() && d.busy == 0 && d.swaps.len() == 1);

    press(&mut d);
    d.objects.get_mut(&vp).unwrap().get_mut(FILESYSTEM).unwrap().insert("MountPoints".into(), val(Vec::<Vec<u8>>::new()));
    d.handle(Do::SwappedOff { drive: dp.clone(), vol: vp.clone(), row: row.clone(), want: 100 * G, files: swapfile.clone(), res: Ok(String::new()) });
    assert!(d.ran.is_empty());
    assert_eq!(d.swaps.last(), Some(&(true, swapfile.clone())));
    assert_eq!(d.busy, 1);
    d.handle(Do::SwappedOn { drive: dp.clone(), row: row.clone(), res: Err(t!("disks-snapshot-changed").into()), standing: false });
    assert_eq!(d.busy, 0);

    let off = Err("Couldn't switch the swap file /swap/swapfile back on: swapon: busy".to_string());
    for ended in [Ok("Made 100 GB of room".to_string()), Err("Example reason.".to_string())] {
        let news = |back| crate::disks::actions::swap_news(ended.clone(), back);
        let said = news(off.clone()).expect_err("red");
        assert!(said.starts_with(&plan::sentence(ended.as_ref().unwrap_or_else(|e| e))), "{said}");
        assert!(said.contains("stays off until you restart") && said.ends_with("swapon: busy."), "{said}");
        assert_eq!(news(Ok(String::new())), ended);

        let said = news(Ok(t!("admin-swap-resume-unread").into())).expect_err("red");
        assert!(said.ends_with(t!("admin-swap-resume-unread")) && !said.contains("stays off"), "{said}");
    }
}

#[test]
fn no_snapshot_or_a_changed_state_means_nothing_is_resized() {
    let mut d = room_window("btrfs", false);
    let (dp, vp) = system_part(&d);
    let row = format!("{vp}:room");
    d.gbs.insert(row.clone(), 100.0);
    d.act(Act::Room { drive: dp.clone(), vol: vp.clone() });
    d.act(Act::Room { drive: dp.clone(), vol: vp.clone() });
    d.handle(Do::Snapped { drive: dp.clone(), vol: vp.clone(), row: row.clone(), want: 100 * G, desc: String::new(), res: Err("You didn't enter a password, so nothing changed.".into()) });
    assert!(d.ran.is_empty() && !d.snapping);
    let (said, _) = d.told(&dp, &row).expect("said");
    assert!(said.contains("Nothing was resized"), "{said}");

    d.act(Act::Room { drive: dp.clone(), vol: vp.clone() });
    d.act(Act::Room { drive: dp.clone(), vol: vp.clone() });
    d.objects.get_mut(&vp).unwrap().get_mut(FILESYSTEM).unwrap().insert("MountPoints".into(), val(Vec::<Vec<u8>>::new()));
    d.handle(Do::Snapped { drive: dp.clone(), vol: vp.clone(), row: row.clone(), want: 100 * G, desc: String::new(), res: Ok(()) });
    assert!(d.ran.is_empty(), "nothing is sent when things changed");
    let (said, _) = d.told(&dp, &row).expect("said");
    assert!(said.contains("Something changed"), "{said}");
    assert_eq!(d.busy, 0);
}

#[test]
fn making_room_waits_for_another_change() {
    let mut d = room_window("btrfs", false);
    let (dp, vp) = system_part(&d);
    let row = format!("{vp}:room");
    d.gbs.insert(row.clone(), 100.0);
    d.changing.set(true);
    d.act(Act::Room { drive: dp.clone(), vol: vp.clone() });
    d.act(Act::Room { drive: dp.clone(), vol: vp.clone() });
    assert!(d.snaps.is_empty() && d.ran.is_empty());
    assert_eq!(d.told(&dp, &row).map(|(w, _)| w).as_deref(), Some(plan::one_at_a_time()));
}

fn add_a_part_that_can_grow(d: &mut Drives) {

    let g = G;
    d.drives.push(Drive {
        path: "/d/grow".into(), name: "Example Grow".into(), size: 30 * g, removable: true, can_power_off: true, optical: false,
        whole: Some("/b/sdg".into()),
        vols: vec![Vol {
            path: "/b/Grow".into(), dev: "sdg1".into(), label: "Grow".into(), fs_type: "ext4".into(), usage: "filesystem".into(),
            uuid: "uuid-Grow".into(), size: 10 * g, has_fs: true, mounts: Vec::new(), full: None, part: Some((0, 10 * g, false)),
            fstab: None, system: false, part_type: String::new(), unlocked: false, key: "fs:uuid-Grow".into(),
        }],
        gaps: vec![(10 * g, 20 * g)], health: None, system: false, key: "drive:Example-Grow-0005".into(),
    });
}

fn every_button_of_every_part(d: &mut Drives) -> Vec<Act> {
    let mut acts: Vec<Act> = Vec::new();
    for drive in &d.drives {
        let dp = drive.path.clone();
        acts.push(Act::Off { drive: dp.clone(), method: "Eject" });
        acts.push(Act::Off { drive: dp.clone(), method: "PowerOff" });
        if let Some(whole) = &drive.whole {
            acts.push(Act::Format { drive: dp.clone(), target: whole.clone(), whole: true });
        }
        for &(offset, size) in &drive.gaps {
            acts.push(Act::Make { drive: dp.clone(), offset, size });
        }
        for v in &drive.vols {
            let vp = v.path.clone();
            acts.push(Act::Open(format!("/mnt/{}", v.dev)));
            acts.push(Act::Mount { drive: dp.clone(), vol: vp.clone(), on: true });
            acts.push(Act::Mount { drive: dp.clone(), vol: vp.clone(), on: false });
            acts.push(Act::Rename { drive: dp.clone(), vol: vp.clone() });
            acts.push(Act::Folder { drive: dp.clone(), vol: vp.clone() });
            acts.push(Act::Resize { drive: dp.clone(), vol: vp.clone() });
            acts.push(Act::Check { drive: dp.clone(), vol: vp.clone() });
            acts.push(Act::Repair { drive: dp.clone(), vol: vp.clone() });
            acts.push(Act::Format { drive: dp.clone(), target: vp.clone(), whole: false });
            acts.push(Act::Delete { drive: dp.clone(), vol: vp.clone() });
            d.texts.insert(format!("{vp}:name"), "Renamed".into());
            d.texts.insert(format!("{vp}:folder"), format!("example-{}", v.dev.to_lowercase()));
            d.gbs.insert(format!("{vp}:size"), 25.0);
        }
    }
    acts
}

fn press_every_button(d: &mut Drives, acts: Vec<Act>) -> (HashSet<&'static str>, Vec<String>) {
    let free: [&str; 4] = ["Mount", "Unmount", "Eject", "PowerOff"];
    let mut tried: HashSet<&'static str> = HashSet::new();
    let mut wrong = Vec::new();
    for act in acts {
        let Some(ask) = must_ask(&act) else { continue };
        let from = d.ran.len();

        d.act(act.clone());
        d.act(act.clone());
        for ran in &d.ran[from..] {
            let changes = ran.methods.iter().any(|m| !free.contains(m));
            tried.extend(ran.methods.iter().copied());
            if changes && !ran.asked {
                wrong.push(format!("{:?} went unasked", ran.methods));
            }
            if ask != changes {
                wrong.push(format!("{:?}: a button {} asked for did this", ran.methods, if ask { "to be" } else { "never" }));
            }
        }
    }
    (tried, wrong)
}

fn changes_made_at_start_up_and_for_a_new_folder_are_asked(d: &mut Drives, tried: &mut HashSet<&'static str>) {

    let from = d.ran.len();
    d.update(Msg::Startup { drive: "/d/usb".into(), vol: "/b/Photos".into(), on: true });
    assert!(d.ran.len() > from && d.ran[from..].iter().all(|r| r.asked), "the fstab is a change");
    tried.extend(d.ran[from..].iter().flat_map(|r| r.methods.iter().copied()));

    let from = d.ran.len();
    d.busy = 1;
    d.carry_known(carried("/mnt/example"), "uuid-new");
    assert!(d.ran.len() > from && d.ran[from..].iter().all(|r| r.asked && r.methods == ["AddConfigurationItem"]));
}

fn a_change_in_progress_refuses_another(d: &mut Drives) {

    d.changing.set(true);
    let from = d.ran.len();
    d.act(Act::Delete { drive: "/d/usb".into(), vol: "/b/Photos".into() });
    assert!(d.ran.len() > from);
    assert_eq!(d.told("/d/usb", "").map(|(w, _)| w).as_deref(), Some(plan::one_at_a_time()));

    d.act(Act::Mount { drive: "/d/old".into(), vol: "/b/Backup".into(), on: true });
    assert_eq!(d.told("/d/old", "/b/Backup").map(|(w, _)| w).as_deref(), Some(plan::no_service()));
}

#[test]
fn a_change_is_made_only_to_what_the_model_has_now() {
    let mut d = sample();
    let from = d.ran.len();
    let system = (d.drives[0].path.clone(), d.drives[0].vols[0].path.clone());
    let refused = [
        Act::Delete { drive: system.0.clone(), vol: system.1.clone() },
        Act::Format { drive: system.0.clone(), target: d.drives[0].whole.clone().unwrap(), whole: true },

        Act::Format { drive: "/d/usb".into(), target: "/b/sdc".into(), whole: true },
        Act::Format { drive: "/d/usb".into(), target: "/b/Backup".into(), whole: false },
        Act::Format { drive: "/d/gone".into(), target: "/b/sdb".into(), whole: true },
        Act::Rename { drive: system.0.clone(), vol: system.1.clone() },
    ];
    for act in refused {
        d.act(act.clone());
        d.act(act);
    }
    assert_eq!(d.ran.len(), from, "nothing was sent to UDisks");
    let erase = Act::Format { drive: "/d/usb".into(), target: "/b/sdb".into(), whole: true };
    d.act(erase.clone());
    d.act(erase);
    assert!(d.ran.len() > from, "what is there is still changed");
}

#[test]
fn what_was_typed_for_a_drive_that_went_is_forgotten() {
    let mut d = sample();
    d.texts.insert("/b/Photos:name".into(), "Mine".into());
    d.texts.insert("/b/Gone:name".into(), "Not mine".into());
    d.picks.insert("/d/usb:wipe:kind".into(), 2);
    d.picks.insert("/d/gone:wipe:kind".into(), 2);
    d.gbs.insert("/b/Gone:size".into(), 3.0);
    d.armed = Some("resize:/b/Gone".into());
    d.forget_gone();
    assert!(d.texts.contains_key("/b/Photos:name") && !d.texts.contains_key("/b/Gone:name"));
    assert!(d.picks.contains_key("/d/usb:wipe:kind") && !d.picks.contains_key("/d/gone:wipe:kind"));
    assert!(d.gbs.is_empty() && d.armed.is_none());
}

#[test]
fn no_change_to_a_drive_runs_unasked() {
    assert_eq!(plan::FREE, ["Mount", "Unmount", "Eject", "PowerOff"], "nothing joins the free steps unnoticed");
    let mut d = sample();
    add_a_part_that_can_grow(&mut d);
    d.caps.insert("ext4".into(), Caps { format: true, resize: plan::OFFLINE_GROW | plan::ONLINE_GROW, check: true, repair: true });
    let acts = every_button_of_every_part(&mut d);
    let (mut tried, wrong) = press_every_button(&mut d, acts);
    changes_made_at_start_up_and_for_a_new_folder_are_asked(&mut d, &mut tried);
    assert_eq!(wrong, Vec::<String>::new());
    for method in ["SetLabel", "AddConfigurationItem", "Resize", "Check", "Repair", "Format", "CreatePartitionAndFormat", "Delete",
                   "Mount", "Unmount", "Eject", "PowerOff"] {
        assert!(tried.contains(method), "{method} was tried: {tried:?}");
    }

    assert!(changes(&[step("/b/Photos", BLOCK, "SomethingNew", (Opts::new(),))]));
    assert!(!changes(&[unmount("/b/Photos"), mount("/b/Photos")]));
    a_change_in_progress_refuses_another(&mut d);
}

#[test]
fn nothing_is_asked_of_udisks_any_other_way() {
    let cut = |source: &'static str| &source[..source.find("\nmod tests {").or_else(|| source.find("\n#[cfg(test)]")).unwrap_or(source.len())];
    let called = |source: &str| -> Vec<String> {
        source.split(".method(").skip(1).map(|rest| rest.split([',', ')']).next().unwrap_or_default().trim().to_string()).collect()
    };
    let own = drives_code();
    let own = own.as_str();
    let mut by_name = called(own);
    by_name.sort();
    assert_eq!(by_name, ["\"GetManagedObjects\"", "\"Mount\"", "\"Show\"", "\"SmartUpdate\"", "method", "method"],
               "reading, the mount put back after no password, this program's own Show, the four questions of what UDisks can do, and `step`");
    assert_eq!(own.matches("chain(").count(), 4, "`chain` itself, its own next step, and the two starts in `run`");
    let reads = ["\"GetManagedObjects\"", "\"Mount\"", "\"Unlock\"", "method"];
    let changing = ["\"Format\"", "\"Delete\"", "\"Resize\"", "\"SetLabel\"", "\"SetFlags\"", "\"SetName\"", "\"SetType\"", "\"SetUUID\"",
                    "\"Repair\"", "\"Check\"", "\"TakeOwnership\"", "\"CreatePartition\"", "\"CreatePartitionAndFormat\"",
                    "\"AddConfigurationItem\"", "\"UpdateConfigurationItem\"", "\"RemoveConfigurationItem\"", "\"OpenForRestore\"",
                    "\"OpenForBenchmark\"", "\"OpenDevice\"", "\"ChangePassphrase\"", "\"SecurityEraseUnit\"", "\"SetConfiguration\"",
                    "\"FormatNamespace\"", "\"SanitizeStart\"", "\"SmartSetEnabled\""];
    for (name, source) in [
        ("src/common/volumes.rs", cut(crate::common::source::code(include_str!("../../common/volumes.rs")))),
        ("src/ring/drives.rs", cut(crate::common::source::code(include_str!("../../ring/drives.rs")))),
    ] {
        for call in called(source) {
            assert!(reads.contains(&call.as_str()), "{name} asks {call}");
        }
        for word in changing {
            assert!(!source.contains(word), "{name} names {word}");
        }
    }
}

fn serve_the_fakes(fake: &Bus, log: &Rc<RefCell<Vec<String>>>) -> Vec<impl Sized> {
    let offer = |iface: &str, call: Box<dyn FnMut(Asked)>| Offer {
        name: iface.into(), xml: format!("<interface name=\"{iface}\"/>"), call, props: Box::new(Props::new), set: None,
    };
    let mut served = Vec::new();
    let l = log.clone();
    served.push(fake.serve(drive_leave::AUTHORITY.1, offer(drive_leave::AUTHORITY.2, Box::new(move |a: Asked| {
        l.borrow_mut().push("polkit: what leave is kept?".into());
        type Kept = (String, String, (String, HashMap<String, OwnedValue>), u64, u64);
        a.reply(&(Vec::<Kept>::new(),));
    }))).expect("served"));
    for iface in [FILESYSTEM, BLOCK, PARTITION] {
        let l = log.clone();
        served.push(fake.serve("/b/Photos", offer(iface, Box::new(move |a: Asked| {
            l.borrow_mut().push(a.member());
            match a.member().as_str() {
                "Mount" => a.reply(&("/mnt/Photos".to_string(),)),

                "Delete" => a.fail("org.freedesktop.UDisks2.Error.NotAuthorizedDismissed", "Not authorized to perform operation"),
                _ => a.reply(&()),
            }
        }))).expect("served"));
    }
    served
}

fn password_steps(d: &mut Drives, log: &Rc<RefCell<Vec<String>>>, heard: &Rc<RefCell<Vec<Result<String, String>>>>, wait: &impl Fn(usize)) {

    d.act(Act::Mount { drive: "/d/usb".into(), vol: "/b/Photos".into(), on: true });
    wait(1);
    assert_eq!(log.borrow().clone(), vec!["Mount"]);
    assert_eq!(heard.borrow()[0], Ok("Mounted".to_string()));
    log.borrow_mut().clear();

    let format = Act::Format { drive: "/d/usb".into(), target: "/b/Photos".into(), whole: false };
    d.act(format.clone());
    assert!(log.borrow().is_empty() && !d.changing.get(), "the first press only arms it");
    d.act(format);
    assert!(d.changing.get());
    wait(2);
    assert_eq!(log.borrow().clone(), vec!["polkit: what leave is kept?", "Format", "polkit: what leave is kept?"]);
    assert_eq!(heard.borrow()[1], Ok("Formatted".to_string()));
    assert!(!d.changing.get(), "the next change may start");
    log.borrow_mut().clear();

    d.act(Act::Delete { drive: "/d/usb".into(), vol: "/b/Photos".into() });
    wait(3);
    assert_eq!(heard.borrow()[2], Err(plan::no_password().to_string()));
    assert_eq!(log.borrow().clone(), vec!["polkit: what leave is kept?", "Unmount", "Delete", "Mount", "polkit: what leave is kept?"]);
    assert!(!d.changing.get());
    assert_eq!(plan::no_password(), "You didn't enter a password, so nothing changed.");
}

fn only_a_closed_box_is_no_password() {

    use crate::common::refused::Refused;
    for (refusal, said) in [
        ("GDBus.Error:org.freedesktop.UDisks2.Error.NotAuthorizedDismissed: The authentication dialog was dismissed", Refused::Cancelled),
        ("org.freedesktop.UDisks2.Error.NotAuthorized: Not authorized to perform operation", Refused::Unclear),
        ("org.freedesktop.UDisks2.Error.NotAuthorizedCanObtain: Not authorized to perform operation", Refused::Unasked),
    ] {
        assert_eq!(plan::plain_error(refusal), plan::refusal_words(said), "{refusal}");
        assert!(plan::plain_error(refusal).ends_with(", so nothing changed."), "{refusal}");
    }
    assert_eq!(plan::refusal_words(Refused::Cancelled), plan::no_password());
}

#[test]
fn a_change_asks_afresh_and_a_password_not_given_changes_nothing() {
    use crate::toolkit::bus::testing::{own_daemon, until};
    let bus = own_daemon();

    assert!(bus.1.starts_with("unix:") && bus.1.contains("dbus-"), "{}", bus.1);
    assert_ne!(std::env::var("DBUS_SESSION_BUS_ADDRESS").ok().as_deref(), Some(bus.1.as_str()));
    let lp = Loop::new();
    let fake = Bus::at(&lp, &bus.1).expect("the fakes' bus");
    let owned = Rc::new(Cell::new(0));
    for name in [UD, drive_leave::AUTHORITY.0] {
        let o = owned.clone();
        fake.own(name, false, &lp, move |ok| o.set(o.get() + u32::from(ok)));
    }
    let o = owned.clone();
    assert!(until(&lp, move || o.get() == 2), "the fakes own their names");
    let log: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
    let served = serve_the_fakes(&fake, &log);
    let heard: Rc<RefCell<Vec<Result<String, String>>>> = Rc::new(RefCell::new(Vec::new()));
    let h = heard.clone();
    let asks = lp.post(move |d: Do| {
        if let Do::Done { res, .. } = d {
            h.borrow_mut().push(res);
        }
    });
    let mut d = sample();
    (d.events, d.asks) = (lp.clone(), asks);
    d.bus = Some(Bus::at(&lp, &bus.1).expect("the window's bus"));
    d.session = Some("c7".into());
    let wait = |n: usize| {
        let h = heard.clone();
        assert!(until(&lp, move || h.borrow().len() == n), "answer {n} came");
    };
    password_steps(&mut d, &log, &heard, &wait);
    only_a_closed_box_is_no_password();
    drop(served);
}

#[test]
fn one_btrfs_mounted_in_three_places_is_one_filesystem() {
    let mut d = room_window("btrfs", false);
    let (_, vol) = system_part(&d);
    let mut places = Vec::new();
    for dir in ["/home", "/", "/swap"] {
        places.push(dir.as_bytes().iter().copied().chain([0]).collect::<Vec<u8>>());
    }
    d.objects.get_mut(&vol).unwrap().get_mut(FILESYSTEM).unwrap().insert("MountPoints".into(), val(places));
    let v = d.vol(&d.drives[0].path, &vol).unwrap().1.clone();
    let (facts, _) = d.room_facts(&v, false);
    assert_eq!(facts.mounts.len(), 3);
    assert!(matches!(room::offer(&facts), room::Offer::Make { .. }), "{facts:?}");
    d.world = || room::World {
        swaps: Some("Filename\tType\tSize\tUsed\tPriority\n/swap/swapfile file 100 0 -2\n".into()),
        mountinfo: "36 1 259:2 /@rootfs / rw,relatime - btrfs /dev/nvme9n1p2 rw\n37 36 259:2 /@home /home rw,relatime - btrfs /dev/nvme9n1p2 rw\n38 36 259:2 /@swap /swap rw,relatime - btrfs /dev/nvme9n1p2 rw\n".into(),
        ..made_up_world()
    };
    let (facts, _) = d.room_facts(&v, false);
    assert_eq!(facts.swap_files, Some(vec!["/swap/swapfile".to_string()]), "the swap file on the third mount is found");
    assert!(matches!(room::offer(&facts), room::Offer::Make { .. }));
}

#[test]
fn a_stick_is_closed_without_eject_when_there_is_no_eject_program() {
    let parts = vec!["/org/freedesktop/UDisks2/block_devices/sdb1".to_string()];
    let drive = "/org/freedesktop/UDisks2/drives/stick";
    let methods = |method, can| crate::disks::actions::closing_steps(drive, &parts, method, can).iter().map(|s| s.method).collect::<Vec<_>>();
    assert_eq!(methods("Eject", true), ["Unmount", "Eject"]);
    assert_eq!(methods("Eject", false), ["Unmount"]);
    assert_eq!(methods("PowerOff", false), ["Unmount", "PowerOff"]);
}
