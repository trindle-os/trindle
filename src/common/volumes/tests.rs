//! Tests for the volume list and the steps on a volume.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use zbus::zvariant::{ObjectPath, OwnedObjectPath, OwnedValue, Value};

use super::*;
use crate::toolkit::bus::testing::{own_daemon, until};
use crate::toolkit::bus::Asked;
use crate::toolkit::bus::Offer;

fn val<'a>(v: impl Into<Value<'a>>) -> OwnedValue {
    OwnedValue::try_from(v.into()).expect("a plain value")
}

fn path_val(p: &str) -> OwnedValue {
    val(ObjectPath::try_from(p.to_string()).expect("a path"))
}

fn bytes(s: &str) -> OwnedValue {
    let mut b = s.as_bytes().to_vec();
    b.push(0);
    val(b)
}

fn props(items: Vec<(&str, OwnedValue)>) -> Props {
    items.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
}

const D: &str = "/org/freedesktop/UDisks2/drives/";
const B: &str = "/org/freedesktop/UDisks2/block_devices/";

#[derive(Default, Clone)]
struct Fake(Objects);

impl Fake {
    fn drive(mut self, id: &str, bus: &str, removable: bool, media: &str) -> Fake {
        let p = props(vec![
            ("Vendor", val("Acme")), ("Model", val(format!("Stick {id}"))),
            ("Removable", val(removable)), ("Ejectable", val(false)), ("CanPowerOff", val(true)),
            ("MediaRemovable", val(removable)), ("ConnectionBus", val(bus)), ("Media", val(media)),
            ("Optical", val(false)),
        ]);
        self.0.insert(format!("{D}{id}"), HashMap::from([(DRIVE.to_string(), p)]));
        self
    }

    fn block(mut self, dev: &str, on: &str, label: &str, usage: &str, kind: &str, size: u64, system: bool,
             extra: Vec<(&str, Props)>) -> Fake {
        let b = props(vec![
            ("Device", bytes(&format!("/dev/{dev}"))), ("Drive", path_val(&format!("{D}{on}"))),
            ("IdLabel", val(label)), ("IdUsage", val(usage)), ("IdType", val(kind)), ("Size", val(size)),
            ("HintSystem", val(system)), ("HintIgnore", val(false)), ("CryptoBackingDevice", path_val("/")),
        ]);
        let mut o = HashMap::from([(BLOCK.to_string(), b)]);
        for (i, p) in extra {
            o.insert(i.to_string(), p);
        }
        self.0.insert(format!("{B}{dev}"), o);
        self
    }

    fn fs(mounts: &[&str]) -> (&'static str, Props) {
        let list: Vec<Vec<u8>> = mounts.iter().map(|m| { let mut b = m.as_bytes().to_vec(); b.push(0); b }).collect();
        (FILESYSTEM, props(vec![("MountPoints", val(list))]))
    }

    fn change(mut self, dev: &str, f: impl FnOnce(&mut Props)) -> Fake {
        f(self.0.get_mut(&format!("{B}{dev}")).and_then(|o| o.get_mut(BLOCK)).expect("a block"));
        self
    }

    fn list(&self) -> Vec<Volume> {
        volumes(&self.0, &Boot::default())
    }
}

fn stick() -> Fake {
    Fake::default().drive("usb", "usb", true, "")
        .block("sdx", "usb", "", "", "", 14_400_000_000, false, vec![])
        .block("sdx1", "usb", "HOLIDAY", "filesystem", "vfat", 14_399_000_000, false, vec![Fake::fs(&[])])
}

#[test]
fn a_plugged_in_stick_is_listed_by_its_label() {
    let v = stick().list();
    assert_eq!(v.len(), 1, "the whole disk, which holds no filesystem itself, is not a row: {v:?}");
    assert_eq!((v[0].name().as_str(), v[0].dev.as_str(), v[0].kind), ("HOLIDAY", "sdx1", Kind::Usb));
    assert_eq!(v[0].notice(), "USB drive connected: HOLIDAY");
    assert!(v[0].mounted().is_none() && !v[0].locked);
}

#[test]
fn a_part_with_no_label_is_called_by_its_size() {
    let v = stick().change("sdx1", |b| { b.insert("IdLabel".into(), val("")); }).list();
    assert_eq!(v[0].name(), "14.4 GB");
    assert_eq!(v[0].notice(), "USB drive connected: 14.4 GB");
    assert_eq!(size_words(512_000_000), "512 MB");
    assert_eq!(size_words(2_000_000_000_000), "2.0 TB");
}

#[test]
fn memory_cards_and_discs_are_called_what_they_are() {
    let card = Fake::default().drive("sd", "sdio", true, "flash_sd")
        .block("mmcx1", "sd", "PHOTOS", "filesystem", "exfat", 64_000_000_000, false, vec![Fake::fs(&[])]);
    assert_eq!(card.list()[0].notice(), "Memory card connected: PHOTOS");
    let disc = Fake::default().drive("cd", "usb", true, "optical_cd_r")
        .block("srx", "cd", "MUSIC", "filesystem", "iso9660", 700_000_000, false, vec![Fake::fs(&[])]);
    assert_eq!(disc.list()[0].notice(), "Disc inserted: MUSIC");
}

#[test]
fn ordinary_system_partitions_never_show() {

    let internal = Fake::default().drive("nvme", "", false, "")
        .block("nvmex1", "nvme", "", "filesystem", "ext4", 500_000_000_000, true, vec![Fake::fs(&["/"])]);
    assert!(internal.list().is_empty());

    let unhinted = internal.change("nvmex1", |b| { b.insert("HintSystem".into(), val(false)); });
    assert!(unhinted.list().is_empty());

    assert!(stick().change("sdx1", |b| { b.insert("HintSystem".into(), val(true)); }).list().is_empty());
}

#[test]
fn swap_hidden_parts_and_parts_without_a_filesystem_are_left_out() {
    let swap = stick().block("sdx2", "usb", "", "other", "swap", 1_000_000_000, false,
                             vec![("org.freedesktop.UDisks2.Swapspace", props(vec![("Active", val(false))]))]);
    assert_eq!(swap.list().len(), 1, "swap is not offered");
    let hidden = stick().change("sdx1", |b| { b.insert("HintIgnore".into(), val(true)); });
    assert!(hidden.list().is_empty(), "a part UDisks says to ignore is not offered");
    let raw = stick().block("sdx3", "usb", "", "", "", 1_000_000, false, vec![]);
    assert_eq!(raw.list().len(), 1, "a part with nothing to open is not offered");
    let extended = stick().block("sdx4", "usb", "", "", "", 1_000_000, false,
                                 vec![(PARTITION, props(vec![("IsContainer", val(true))]))]);
    assert_eq!(extended.list().len(), 1);
}

#[test]
fn the_drive_the_system_runs_from_is_never_offered() {

    let live = stick().block("sdx2", "usb", "persistence", "filesystem", "ext4", 4_000_000_000, false, vec![Fake::fs(&[])]);
    assert_eq!(live.list().len(), 2);
    let info = "36 35 8:1 / /run/live/medium ro,relatime shared:1 - iso9660 /dev/sdx1 ro\n";
    let boot = boot_of(&live.0, info, "Filename Type Size Used Priority\n", |w| w.strip_prefix("/dev/").map(String::from), |_| Vec::new());
    assert!(boot.devs.contains("sdx1") && boot.drives.contains(&format!("{D}usb")), "{boot:?}");
    assert!(volumes(&live.0, &boot).is_empty(), "every part of the boot stick is hidden, not only the mounted one");

    let other = live.clone().drive("other", "usb", true, "")
        .block("sdy1", "other", "TRAVEL", "filesystem", "vfat", 8_000_000_000, false, vec![Fake::fs(&[])]);
    let v = volumes(&other.0, &boot);
    assert_eq!(v.len(), 1);
    assert_eq!(v[0].name(), "TRAVEL");
}

fn inside() -> Fake {
    let kind = |t: &str| (PARTITION, props(vec![("Type", val(t)), ("IsContainer", val(false))]));
    Fake::default().drive("nvme", "", false, "").drive("sata", "ata", false, "")
        .block("nvmex1", "nvme", "", "filesystem", "vfat", 500_000_000, true,
               vec![Fake::fs(&["/boot/efi"]), kind("c12a7328-f81f-11d2-ba4b-00a0c93ec93b")])
        .block("nvmex2", "nvme", "", "filesystem", "ext4", 1_000_000_000, true, vec![Fake::fs(&["/boot"]), kind("0fc63daf-8483-4772-8e79-3d69d8477de4")])
        .block("nvmex3", "nvme", "Trindle", "filesystem", "btrfs", 200_000_000_000, true,
               vec![Fake::fs(&["/home", "/", "/.snapshots"]), kind("0fc63daf-8483-4772-8e79-3d69d8477de4")])
        .block("nvmex4", "nvme", "", "other", "swap", 8_000_000_000, true,
               vec![("org.freedesktop.UDisks2.Swapspace", props(vec![("Active", val(true))]))])
        .block("nvmex5", "nvme", "WINRE", "filesystem", "ntfs", 900_000_000, true,
               vec![Fake::fs(&[]), kind("DE94BBA4-06D1-4D40-A16A-BFD50179D6AC")])
        .block("nvmex6", "nvme", "Recovery", "filesystem", "ntfs", 900_000_000, true, vec![Fake::fs(&[])])
        .block("sday1", "sata", "Films", "filesystem", "ext4", 2_000_000_000_000, true, vec![Fake::fs(&[])])
        .block("sday2", "sata", "", "filesystem", "ext4", 50_000_000_000, true, vec![Fake::fs(&["/var"])])
}

#[test]
fn a_page_of_every_drive_has_the_disks_inside_too_without_the_computers_own_parts() {
    let f = inside().drive("usb", "usb", true, "")
        .block("sdx1", "usb", "HOLIDAY", "filesystem", "vfat", 14_399_000_000, false, vec![Fake::fs(&[])]);

    let offered = f.list();
    assert_eq!(offered.iter().map(|v| v.name()).collect::<Vec<_>>(), ["HOLIDAY"]);
    assert!(!offered[0].fixed && !offered[0].system);

    let every = all(&f.0, &Boot::default());
    let named: Vec<(String, bool, bool)> = every.iter().map(|v| (v.name(), v.fixed, v.system)).collect();
    assert_eq!(named, [("Trindle".to_string(), true, true), ("Films".to_string(), true, false), ("HOLIDAY".to_string(), false, false)], "{every:?}");

    assert!(every.iter().filter(|v| v.fixed).all(|v| !v.can_eject && !v.can_power_off));

    let home = inside().block("nvmex7", "nvme", "", "filesystem", "ext4", 300_000_000_000, true, vec![Fake::fs(&["/home"])]);
    let every = all(&home.0, &Boot::default());
    assert!(every.iter().any(|v| v.mounts == ["/home"] && v.fixed && !v.system), "{every:?}");

    let live = stick().change("sdx1", |b| { b.insert("IdLabel".into(), val("LIVE")); });
    let mut on = live.clone();
    on.0.get_mut(&format!("{B}sdx1")).unwrap().insert(FILESYSTEM.to_string(), Fake::fs(&["/run/live/medium"]).1);
    let boot = Boot { devs: ["sdx1".to_string()].into(), drives: [format!("{D}usb")].into() };
    assert!(all(&on.0, &boot).is_empty());
}

#[test]
fn a_swap_partition_in_use_hides_its_drive() {
    let f = stick();
    let boot = boot_of(&f.0, "", "Filename Type Size Used Priority\n/dev/sdx1 partition 1024 0 -2\n",
                       |w| w.strip_prefix("/dev/").map(String::from), |_| Vec::new());
    assert!(volumes(&f.0, &boot).is_empty());
}

#[test]
fn every_part_of_a_drive_with_several_is_a_row() {
    let f = stick().block("sdx2", "usb", "BACKUPS", "filesystem", "ext4", 9_000_000_000, false, vec![Fake::fs(&["/run/user/media/BACKUPS"])]);
    let v = f.list();
    assert_eq!(v.iter().map(Volume::name).collect::<Vec<_>>(), vec!["HOLIDAY", "BACKUPS"]);
    assert_eq!(v[1].mounted(), Some("/run/user/media/BACKUPS"));
    let parts: Vec<&Volume> = v.iter().collect();
    assert_eq!(best(&parts).map(Volume::name).as_deref(), Some("HOLIDAY"), "the biggest part is the one opened");
}

#[test]
fn an_encrypted_part_is_locked_until_opened_and_then_shown_once() {
    let locked = stick().block("sdx2", "usb", "", "crypto", "crypto_LUKS", 4_000_000_000, false,
                               vec![(ENCRYPTED, props(vec![("HintEncryptionType", val("luks"))]))]);
    let v = locked.list();
    assert_eq!(v.len(), 2);
    let l = &v[1];
    assert!(l.locked && l.container.as_deref() == Some(&*format!("{B}sdx2")), "{l:?}");

    let open = locked.block("dm-x", "usb", "SECRET", "filesystem", "ext4", 3_900_000_000, false, vec![Fake::fs(&["/run/user/media/SECRET"])])
        .change("dm-x", |b| { b.insert("CryptoBackingDevice".into(), path_val(&format!("{B}sdx2"))); });
    let v = open.list();
    assert_eq!(v.len(), 2, "{v:?}");
    let o = &v[1];
    assert!(!o.locked && o.id == format!("{B}dm-x") && o.label == "SECRET" && o.mounted().is_some(), "{o:?}");
    assert!(o.container.is_some(), "so it can be locked again");
}

#[test]
fn the_biggest_part_with_no_passphrase_is_opened_first() {
    let mut big = stick().list();
    let mut locked = big[0].clone();
    locked.locked = true;
    locked.size = 90_000_000_000;
    locked.id = "locked".into();
    big.push(locked);
    let parts: Vec<&Volume> = big.iter().collect();
    assert_eq!(best(&parts).map(|v| v.id.as_str()), Some(&*format!("{B}sdx1")));
    assert_eq!(best(&parts[1..]).map(|v| v.id.as_str()), Some("locked"), "with nothing else, the locked one");
}

struct Service {
    bus: Bus,
    objects: Rc<RefCell<Objects>>,
    calls: Rc<RefCell<Vec<String>>>,
    _served: Vec<crate::toolkit::bus::Served>,
}

fn wire(objects: &Objects) -> HashMap<OwnedObjectPath, HashMap<String, Props>> {
    objects.iter().map(|(p, o)| {
        let o = o.iter().map(|(i, props)| {
            let copy: Props = props.iter().filter_map(|(k, v)| v.try_clone().ok().map(|v| (k.clone(), v))).collect();
            (i.clone(), copy)
        }).collect();
        (OwnedObjectPath::try_from(p.as_str()).expect("a path"), o)
    }).collect()
}

fn offer(iface: &str, call: impl FnMut(Asked) + 'static) -> Offer {
    Offer { name: iface.into(), xml: format!("<interface name=\"{iface}\"/>"), call: Box::new(call), props: Box::new(Props::new), set: None }
}

fn service(lp: &Loop, address: &str) -> Service {
    let bus = Bus::at(lp, address).expect("the service's bus");
    let objects: Rc<RefCell<Objects>> = Rc::new(RefCell::new(Objects::new()));
    let calls = Rc::new(RefCell::new(Vec::new()));
    let named = Rc::new(std::cell::Cell::new(false));
    let n = named.clone();
    bus.own(UD, false, lp, move |ok| n.set(ok));
    assert!(until(lp, move || named.get()), "the fake owns the name");
    let mut served = Vec::new();
    let o = objects.clone();
    served.push(bus.serve(UD_PATH, offer(OBJECT_MANAGER, move |a: Asked| {
        if a.member() == "GetManagedObjects" {
            a.reply(&(wire(&o.borrow()),));
        }
    })).expect("served"));
    {
        let dev = "sdx1";
        let c = calls.clone();
        served.push(bus.serve(&format!("{B}{dev}"), offer(FILESYSTEM, move |a: Asked| {
            c.borrow_mut().push(format!("{} sdx1", a.member()));
            match a.member().as_str() {
                "Mount" => a.reply(&("/run/user/1000/media/HOLIDAY".to_string(),)),
                _ => a.reply(&()),
            }
        })).expect("served"));
    }
    let c = calls.clone();
    served.push(bus.serve(&format!("{D}usb"), offer(DRIVE, move |a: Asked| {
        c.borrow_mut().push(format!("{} usb", a.member()));
        a.reply(&());
    })).expect("served"));
    Service { bus, objects, calls, _served: served }
}

impl Service {
    fn set(&self, fake: &Fake, signal: &str, changed: &str) {
        *self.objects.borrow_mut() = fake.0.clone();
        let p = OwnedObjectPath::try_from(changed).expect("a path");
        if signal == "InterfacesAdded" {
            let all = wire(&fake.0).remove(&p).unwrap_or_default();
            self.bus.emit(UD_PATH, OBJECT_MANAGER, signal, &(p, all)).expect("signal");
        } else {
            self.bus.emit(UD_PATH, OBJECT_MANAGER, signal, &(p, vec![BLOCK.to_string()])).expect("signal");
        }
    }
}

fn plugged(lists: &Rc<RefCell<Vec<Vec<Volume>>>>) -> usize {
    lists.borrow().last().map_or(0, Vec::len)
}

#[test]
fn a_drive_comes_and_goes_on_a_private_bus_and_is_mounted() {
    let d = own_daemon();

    assert!(d.1.starts_with("unix:") && d.1.contains("dbus-"), "{}", d.1);
    assert_ne!(std::env::var("DBUS_SESSION_BUS_ADDRESS").ok().as_deref(), Some(d.1.as_str()));
    let lp = Loop::new();
    let fake = service(&lp, &d.1);
    let client = Bus::at(&lp, &d.1).expect("the client's bus");
    let lists: Rc<RefCell<Vec<Vec<Volume>>>> = Rc::new(RefCell::new(Vec::new()));

    let l = lists.clone();
    let watching = watch(&client, &lp, move |list| l.borrow_mut().push(list.to_vec()));
    let l = lists.clone();
    assert!(until(&lp, move || !l.borrow().is_empty()));
    assert_eq!(plugged(&lists), 0);

    let with = stick().block("sdx1", "usb", "HOLIDAY", "filesystem", "vfat", 14_399_000_000, false, vec![Fake::fs(&[])]);
    fake.set(&with, "InterfacesAdded", &format!("{B}sdx1"));
    let l = lists.clone();
    assert!(until(&lp, move || plugged(&l) == 1));
    let listed = watching.now();
    assert_eq!(listed[0].notice(), "USB drive connected: HOLIDAY");
    let n = lists.borrow().len();

    let got: Rc<RefCell<Option<Result<String, String>>>> = Rc::new(RefCell::new(None));
    let g = got.clone();
    mount(&client, &lp, &listed[0], move |r| *g.borrow_mut() = Some(r));
    let g = got.clone();
    assert!(until(&lp, move || g.borrow().is_some()));
    assert_eq!(got.borrow().clone(), Some(Ok("/run/user/1000/media/HOLIDAY".to_string())));
    assert_eq!(fake.calls.borrow().clone(), vec!["Mount sdx1"]);

    fake.set(&Fake::default(), "InterfacesRemoved", &format!("{B}sdx1"));
    let l = lists.clone();
    assert!(until(&lp, move || plugged(&l) == 0 && l.borrow().len() > n));
    drop(watching);
}

#[test]
fn only_news_of_a_job_is_ignored_whichever_shape_it_comes_in() {
    let job = OwnedObjectPath::try_from("/org/freedesktop/UDisks2/jobs/3").expect("a path");
    let block = OwnedObjectPath::try_from(format!("{B}sdx1")).expect("a path");
    let added = |p: &OwnedObjectPath| zbus::Message::signal(UD_PATH, OBJECT_MANAGER, "InterfacesAdded")
        .expect("a signal").build(&(p.clone(), HashMap::<String, Props>::new())).expect("a message");
    let removed = |p: &OwnedObjectPath| zbus::Message::signal(UD_PATH, OBJECT_MANAGER, "InterfacesRemoved")
        .expect("a signal").build(&(p.clone(), vec![BLOCK.to_string()])).expect("a message");
    assert!(is_job(&added(&job)) && is_job(&removed(&job)));
    assert!(!is_job(&added(&block)) && !is_job(&removed(&block)), "a drive coming or going is news");
}

fn known() -> Fake {
    let mut f = inside().drive("usb", "usb", true, "")
        .block("sdx1", "usb", "HOLIDAY", "filesystem", "vfat", 14_399_000_000, false, vec![Fake::fs(&[])])
        .block("sdx2", "usb", "WORK", "filesystem", "ext4", 1_000_000_000, false, vec![Fake::fs(&[])])
        .change("sdx1", |b| { b.insert("IdUUID".into(), val("1111-AAAA")); })
        .change("sdx2", |b| { b.insert("IdUUID".into(), val("2222-bbbb")); })
        .change("sday1", |b| { b.insert("IdUUID".into(), val("3333-cccc")); });
    for (id, known_by) in [("usb", "Acme-Stick-usb-0001"), ("sata", "Acme-Stick-sata-0002"), ("nvme", "Acme-Stick-nvme-0003")] {
        f.0.get_mut(&format!("{D}{id}")).and_then(|o| o.get_mut(DRIVE)).expect("a drive").insert("Id".into(), val(known_by));
    }
    f
}

fn names(list: &[Volume]) -> Vec<String> {
    list.iter().map(|v| v.name()).collect()
}

#[test]
fn what_is_hidden_is_left_out_of_every_list_and_comes_back() {
    let f = known();
    let none = BTreeSet::new();
    let offered = unhidden(f.list(), &none);
    let every = unhidden(all(&f.0, &Boot::default()), &none);
    assert_eq!(names(&offered), ["HOLIDAY", "WORK"]);
    assert_eq!(names(&every), ["Trindle", "Films", "HOLIDAY", "WORK"]);
    assert_eq!((offered[0].key.as_str(), offered[0].drive_key.as_str()), ("fs:1111-AAAA", "drive:Acme-Stick-usb-0001"));

    let whole = BTreeSet::from(["drive:Acme-Stick-usb-0001".to_string()]);
    assert!(unhidden(f.list(), &whole).is_empty());
    assert_eq!(names(&unhidden(all(&f.0, &Boot::default()), &whole)), ["Trindle", "Films"]);

    let part = BTreeSet::from(["fs:2222-bbbb".to_string()]);
    assert_eq!(names(&unhidden(f.list(), &part)), ["HOLIDAY"]);

    let second = BTreeSet::from(["drive:Acme-Stick-sata-0002".to_string()]);
    assert_eq!(names(&unhidden(all(&f.0, &Boot::default()), &second)), ["Trindle", "HOLIDAY", "WORK"]);

    assert_eq!(unhidden(all(&f.0, &Boot::default()), &none), every);

    let mut moved = Fake(f.0.clone());
    for (was, now) in [("sdx1", "sdz1"), ("sdx2", "sdz2")] {
        let mut o = moved.0.remove(&format!("{B}{was}")).expect("a part");
        o.get_mut(BLOCK).expect("a block").insert("Device".into(), bytes(&format!("/dev/{now}")));
        moved.0.insert(format!("{B}{now}"), o);
    }
    assert_eq!(moved.list().iter().map(|v| v.dev.as_str()).collect::<Vec<_>>(), ["sdz1", "sdz2"]);
    assert!(unhidden(moved.list(), &whole).is_empty());
    assert_eq!(names(&unhidden(moved.list(), &part)), ["HOLIDAY"]);

    let system = BTreeSet::from(["drive:Acme-Stick-nvme-0003".to_string()]);
    assert!(names(&unhidden(all(&f.0, &Boot::default()), &system)).contains(&"Trindle".to_string()));

    let nameless = stick().list();
    assert_eq!((nameless[0].key.as_str(), nameless[0].drive_key.as_str()), ("", ""));
    assert_eq!(unhidden(nameless, &BTreeSet::from([String::new()])).len(), 1);
}

#[test]
fn the_reader_alone_honours_the_file_of_hidden_drives_as_it_changes() {
    let d = own_daemon();

    assert!(d.1.starts_with("unix:") && d.1.contains("dbus-"), "{}", d.1);
    assert_ne!(std::env::var("DBUS_SESSION_BUS_ADDRESS").ok().as_deref(), Some(d.1.as_str()));
    let dir = std::env::temp_dir().join(format!("trindle-volumes-hidden-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let file = dir.join(hidden_drives::FILE);
    let lp = Loop::new();
    let fake = service(&lp, &d.1);
    *fake.objects.borrow_mut() = known().0;
    let client = Bus::at(&lp, &d.1).expect("the client's bus");
    let lists: Rc<RefCell<Vec<Vec<Volume>>>> = Rc::new(RefCell::new(Vec::new()));
    let l = lists.clone();
    let watching = super::watching(&client, &lp, true, &file, move |list| l.borrow_mut().push(list.to_vec()));
    let shows = |want: &'static [&'static str]| {
        let l = lists.clone();
        assert!(until(&lp, move || l.borrow().last().is_some_and(|list| names(list) == want)), "{want:?}: {:?}", lists.borrow().last().map(|l| names(l)));
    };

    assert!(!file.exists());
    shows(&["Trindle", "Films", "HOLIDAY", "WORK"]);

    hidden_drives::set_in(&file, "drive:Acme-Stick-sata-0002", true).unwrap();
    shows(&["Trindle", "HOLIDAY", "WORK"]);
    hidden_drives::set_in(&file, "fs:1111-AAAA", true).unwrap();
    shows(&["Trindle", "WORK"]);

    hidden_drives::set_in(&file, "drive:Acme-Stick-sata-0002", false).unwrap();
    shows(&["Trindle", "Films", "WORK"]);
    std::fs::remove_file(&file).unwrap();
    shows(&["Trindle", "Films", "HOLIDAY", "WORK"]);
    drop(watching);
    std::fs::remove_dir_all(&dir).unwrap();
}
