mod page_shots_test;
use crate::common::info_popup_tests::{shuts_like_every_other, THE_I, UNDER_THE_I};
use crate::toolkit::ui::App;
use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;
use std::rc::Rc;
use std::time::{Duration, Instant};
use super::plan::Health;
use zbus::zvariant::{OwnedValue, Value};
use crate::common::{drive_leave, hidden_drives};
use crate::toolkit::bus::{Asked, Bus, Offer, Props, Served};
use crate::toolkit::event::Loop;
use crate::toolkit::input::{Input, Keysym};
use crate::toolkit::tiles::worry;
use crate::toolkit::widget::{Node, Rect, Tone};
use crate::toolkit::{t, theme};
use super::{Act, Do, Drives, GB, Msg, SIZE, plan, room, spot, steps};
use super::actions::format_key;
use super::actions::fs_opts;
use super::map::MAP_BAR;
use super::map::MAP_GAP;
use super::map::MAP_ROW;
use super::map::SEG_LEAST;
use super::map::map_caption;
use super::map::map_spacing;
use super::map::segments;
use super::model::Caps;
use super::model::Drive;
use super::model::Vol;
use super::model::build;
use super::model::headline;
use super::steps::Carried;
use super::steps::STANDING_GAP;
use super::steps::TAKE_BACK_TRIES;
use super::steps::changes;
use super::steps::mount;
use super::steps::step;
use super::steps::unmount;
use super::udisks::BLOCK;
use super::udisks::DRIVE;
use super::udisks::FILESYSTEM;
use super::udisks::Objects;
use super::udisks::Opts;
use super::udisks::PARTITION;
use super::udisks::PARTITION_TABLE;
use super::udisks::UD;
use crate::toolkit::theme::Theme;
use crate::toolkit::ui::Ui;
mod layout;
mod mounts;
mod changes;
mod stopping;

fn sample() -> Drives {
    let events = Loop::new();
    let asks = events.post(|_: Do| {});
    let mut d = Drives::new(&events, asks);
    let g = 1_000_000_000u64;
    let part = |name: &str, fs: &str, size: u64, at: u64, full: Option<(u64, u64)>, system: bool| Vol {
        path: format!("/b/{name}"), dev: name.into(), label: name.into(), fs_type: fs.into(), usage: "filesystem".into(),
        uuid: format!("uuid-{name}"), size, has_fs: true,
        mounts: if full.is_some() { vec![format!("/mnt/{name}")] } else { Vec::new() },
        full, part: Some((at, size, false)), fstab: None, system, part_type: String::new(), unlocked: false,
        key: format!("fs:uuid-{name}"),
    };
    d.drives = vec![
        Drive {
            path: "/d/nvme".into(), name: "Samsung SSD 970".into(), size: 500 * g, removable: false, can_power_off: false,
            optical: false, whole: Some("/b/nvme".into()),
            vols: vec![
                part("EFI", "vfat", g, 0, Some((g / 10, 9 * g / 10)), true),
                part("System", "ext4", 120 * g, g, Some((100 * g, 20 * g)), true),
                part("Data", "ext4", 300 * g, 121 * g, Some((120 * g, 180 * g)), false),
                part("Swap", "swap", 16 * g, 421 * g, None, true),
            ],
            gaps: vec![(437 * g, 63 * g)],
            health: Some(Health { failing: false, troubles: Vec::new(), kelvin: 311.0, hours: 1234 }),
            system: true, key: "drive:Samsung-SSD-970-0001".into(),
        },
        Drive {
            path: "/d/usb".into(), name: "SanDisk Cruzer".into(), size: 32 * g, removable: true, can_power_off: true,
            optical: false, whole: Some("/b/sdb".into()),
            vols: vec![part("Photos", "exfat", 32 * g, 0, Some((30 * g, 2 * g)), false)],
            gaps: Vec::new(), health: None, system: false, key: "drive:SanDisk-Cruzer-0002".into(),
        },
        Drive {
            path: "/d/old".into(), name: "WD Elements".into(), size: 2000 * g, removable: true, can_power_off: true,
            optical: false, whole: Some("/b/sdc".into()),
            vols: vec![part("Backup", "ntfs", 2000 * g, 0, None, false)],
            gaps: Vec::new(),
            health: Some(Health { failing: false, troubles: vec!["3 damaged spots".into()], kelvin: 305.0, hours: 40_000 }),
            system: false, key: "drive:WD-Elements-0003".into(),
        },
    ];
    for k in plan::kinds() {
        d.caps.insert(k.id.into(), Caps { format: true, resize: 0, check: true, repair: true });
    }
    d.caps.insert("swap".into(), Caps::default());
    d.loaded = true;
    d.sum_up();
    d
}

fn sheet_places(d: &Drives, size: (u32, u32), scale: f32) -> (Vec<(String, Rect)>, Rect, Node<Msg>) {
    use crate::toolkit::widget::space;
    let view = || crate::toolkit::widget::layers(vec![space(), d.sheet().expect("a box of changes is open")]);
    let mut ui = Ui::new(view(), Theme::chosen());
    let places = ui.places_at_scale(((size.0 as f32 * scale) as u32, (size.1 as f32 * scale) as u32), scale);
    let sheet = places.iter().find(|(k, _)| k.starts_with("sheet:")).map(|p| p.1).expect("the box is laid out");
    (places, sheet, view())
}

fn is_a_change(m: &Msg) -> bool {
    matches!(m, Msg::Act(Act::Rename { .. } | Act::Folder { .. } | Act::Resize { .. } | Act::Room { .. } | Act::Check { .. }
        | Act::Repair { .. } | Act::Format { .. } | Act::Delete { .. } | Act::Make { .. }))
}

const SECOND: &str = "Example NVMe 2TB";

const D: &str = "/org/freedesktop/UDisks2/drives/";

const B: &str = "/org/freedesktop/UDisks2/block_devices/";

const EFI: &str = "c12a7328-f81f-11d2-ba4b-00a0c93ec93b";

const BOOT: &str = "bc13c2ff-59e6-4262-a352-b275fd6f7172";

const LINUX: &str = "0fc63daf-8483-4772-8e79-3d69d8477de4";

fn val<'a>(v: impl Into<Value<'a>>) -> OwnedValue {
    OwnedValue::try_from(v.into()).expect("a plain value")
}

fn path_val(p: &str) -> OwnedValue {
    val(zbus::zvariant::ObjectPath::try_from(p.to_string()).expect("a path"))
}

fn props(items: Vec<(&str, OwnedValue)>) -> Props {
    items.into_iter().map(|(k, v)| (k.to_string(), v)).collect()
}

#[derive(Default)]
struct Fake(Objects);

impl Fake {
    fn drive(mut self, id: &str, model: &str, size: u64, usb: bool) -> Fake {
        let p = props(vec![
            ("Vendor", val("")), ("Model", val(model)), ("Size", val(size)), ("Id", val(format!("{}-{id}", model.replace(' ', "-")))),
            ("Removable", val(usb)), ("Ejectable", val(false)), ("MediaRemovable", val(usb)), ("CanPowerOff", val(usb)),
            ("ConnectionBus", val(if usb { "usb" } else { "" })), ("Optical", val(false)),
        ]);
        self.0.insert(format!("{D}{id}"), HashMap::from([(DRIVE.to_string(), p)]));
        self
    }

    fn whole(mut self, dev: &str, on: &str, size: u64, table: &str) -> Fake {
        let mut o = HashMap::from([(BLOCK.to_string(), Self::block(dev, on, "", "", "", "", size, false))]);
        if !table.is_empty() {
            o.insert(PARTITION_TABLE.to_string(), props(vec![("Type", val(table))]));
        }
        self.0.insert(format!("{B}{dev}"), o);
        self
    }

    fn block(dev: &str, on: &str, label: &str, usage: &str, kind: &str, uuid: &str, size: u64, ignore: bool) -> Props {
        let mut device = format!("/dev/{dev}").into_bytes();
        device.push(0);
        props(vec![
            ("Device", val(device)), ("Drive", path_val(&format!("{D}{on}"))), ("IdLabel", val(label)), ("IdUsage", val(usage)),
            ("IdType", val(kind)), ("IdUUID", val(uuid)), ("Size", val(size)), ("HintIgnore", val(ignore)),
            ("CryptoBackingDevice", path_val("/")),
        ])
    }

    #[allow(clippy::too_many_arguments)]
    fn part(mut self, dev: &str, on: &str, label: &str, usage: &str, kind: &str, at: u64, size: u64, table_kind: &str,
            ignore: bool, mounts: Option<&[&str]>) -> Fake {
        let uuid = if usage.is_empty() { String::new() } else { format!("uuid-of-{label}{}", if label.is_empty() { at.to_string() } else { String::new() }) };
        let mut o = HashMap::from([
            (BLOCK.to_string(), Self::block(dev, on, label, usage, kind, &uuid, size, ignore)),
            (PARTITION.to_string(), props(vec![("Offset", val(at)), ("Size", val(size)), ("IsContainer", val(false)),
                                                ("Type", val(table_kind)), ("UUID", val(format!("table-{on}-{at}")))])),
        ]);
        if let Some(mounts) = mounts {
            let list: Vec<Vec<u8>> = mounts.iter().map(|m| { let mut b = m.as_bytes().to_vec(); b.push(0); b }).collect();
            o.insert(FILESYSTEM.to_string(), props(vec![("MountPoints", val(list))]));
        }
        self.0.insert(format!("{B}{dev}"), o);
        self
    }
}

const G: u64 = 1_000_000_000;

const THIRD: u64 = 2000 * G - 2_602_000_000;

fn made_up(second_dev: &str) -> Objects {
    let m = 1_000_000u64;
    Fake::default()
        .drive("0001", "Example SSD 500", 500 * G, false)
        .whole("nvme9n1", "0001", 500 * G, "gpt")
        .part("nvme9n1p1", "0001", "", "filesystem", "vfat", m, 512 * m, EFI, true, Some(&["/boot/efi"]))
        .part("nvme9n1p2", "0001", "system", "filesystem", "ext4", 513 * m, 499 * G, LINUX, false, Some(&["/"]))
        .drive("0002", SECOND, 2000 * G, false)
        .whole(second_dev, "0002", 2000 * G, "gpt")
        .part(&format!("{second_dev}p1"), "0002", "", "filesystem", "vfat", m, 600 * m, EFI, true, Some(&[]))
        .part(&format!("{second_dev}p2"), "0002", "other_boot", "filesystem", "ext4", 601 * m, 2 * G, BOOT, true, Some(&[]))
        .part(&format!("{second_dev}p3"), "0002", "other_system", "filesystem", "btrfs", 2601 * m, THIRD, LINUX, false, Some(&[]))
        .drive("0003", "Example Stick", 64 * G, true)
        .whole("sdx", "0003", 64 * G, "gpt")
        .part("sdx1", "0003", "", "crypto", "crypto_LUKS", m, 32 * G, LINUX, false, None)
        .part("sdx2", "0003", "", "", "", 32 * G + m, 8 * G, LINUX, false, None)
        .drive("0004", "Example Blank", 16 * G, true)
        .whole("sdy", "0004", 16 * G, "")
        .0
}

fn made_up_drives(second_dev: &str) -> Vec<Drive> {
    let system_devs = HashSet::from(["nvme9n1p1".to_string(), "nvme9n1p2".to_string()]);
    let system_drives = HashSet::from([format!("{D}0001")]);
    build(&made_up(second_dev), &system_devs, &system_drives, |at| (at == "/").then_some((100 * G, 399 * G)))
}

fn made_up_window(tag: &str) -> Drives {
    let mut d = sample();
    d.drives = made_up_drives("nvme1n1");
    d.hidden_file = std::env::temp_dir().join(format!("trindle-drives-{tag}-{}", std::process::id())).join(hidden_drives::FILE);
    let _ = std::fs::remove_dir_all(d.hidden_file.parent().unwrap());
    d.sum_up();
    d
}

fn named<'a>(drives: &'a [Drive], name: &str) -> &'a Drive {
    drives.iter().find(|d| d.name == name).unwrap_or_else(|| panic!("{name} is listed"))
}

fn picture(d: &Drives) -> Vec<u8> {
    let mut ui = Ui::new(d.view(), Theme::new(true, "#fbc711"));
    let mut pix = tiny_skia::Pixmap::new(1280, 1600).unwrap();
    ui.paint(&mut pix.as_mut(), 1.0);
    pix.data().to_vec()
}

const SIZE_OF_SYSTEM: u64 = 499 * G;

fn made_up_world() -> room::World {
    room::World {
        mountinfo: "36 1 259:2 /@rootfs / rw,relatime shared:1 - btrfs /dev/nvme9n1p2 rw,ssd\n".into(),
        swaps: "Filename\tType\tSize\tUsed\tPriority\n/dev/zram0 partition 100 0 100\n".into(),
        devices: |_| Some(1),
        used: |_| Some(100 * G),
        snapper: true,
    }
}

fn room_window(kind: &str, encrypted: bool) -> Drives {
    let m = 1_000_000u64;
    let header = 16 * 1024 * 1024u64;
    let mut fake = Fake::default().drive("0001", "Example SSD 500", 500 * G, false).whole("nvme9n1", "0001", 500 * G, "gpt");
    fake = if encrypted {
        fake.part("nvme9n1p2", "0001", "", "crypto", "crypto_LUKS", 513 * m, SIZE_OF_SYSTEM, LINUX, false, None)
    } else {
        fake.part("nvme9n1p2", "0001", "system", "filesystem", kind, 513 * m, SIZE_OF_SYSTEM, LINUX, false, Some(&["/"]))
    };
    let mut objects = fake.0;
    let (fs_at, fs_size) = if encrypted {
        let mut block = Fake::block("dm-0", "0001", "system", "filesystem", kind, "uuid-of-system", SIZE_OF_SYSTEM - header, false);
        block.insert("CryptoBackingDevice".into(), path_val(&format!("{B}nvme9n1p2")));
        block.insert("Drive".into(), path_val("/"));
        let mut mount = b"/".to_vec();
        mount.push(0);
        let o = HashMap::from([(BLOCK.to_string(), block), (FILESYSTEM.to_string(), props(vec![("MountPoints", val(vec![mount]))]))]);
        objects.insert(format!("{B}dm-0"), o);
        (format!("{B}dm-0"), SIZE_OF_SYSTEM - header)
    } else {
        (format!("{B}nvme9n1p2"), SIZE_OF_SYSTEM)
    };
    objects.get_mut(&fs_at).unwrap().get_mut(FILESYSTEM).unwrap().insert("Size".into(), val(fs_size));
    let mut d = sample();
    let system_devs = HashSet::from(["nvme9n1p2".to_string()]);
    d.drives = build(&objects, &system_devs, &HashSet::from([format!("{D}0001")]), |at| (at == "/").then_some((100 * G, 399 * G)));
    d.objects = objects;
    let online = plan::ONLINE_SHRINK | plan::ONLINE_GROW | plan::OFFLINE_GROW;
    for k in ["btrfs", "ext4", "xfs"] {
        d.caps.insert(k.into(), Caps { format: true, resize: if k == "btrfs" { online } else { 0 }, check: true, repair: true });
    }
    d.world = made_up_world;
    d.sum_up();
    d
}

fn system_part(d: &Drives) -> (String, String) {
    let drive = &d.drives[0];
    (drive.path.clone(), drive.vols.iter().find(|v| v.part.is_some()).unwrap().path.clone())
}

fn must_ask(a: &Act) -> Option<bool> {
    match a {
        Act::Open(_) => None,
        Act::Off { .. } | Act::Mount { .. } => Some(false),
        Act::Rename { .. } | Act::Folder { .. } | Act::Resize { .. } | Act::Room { .. } | Act::Check { .. } | Act::Repair { .. }
        | Act::Format { .. } | Act::Delete { .. } | Act::Make { .. } => Some(true),
    }
}

fn carried(dir: &str) -> Carried {
    Carried { drive: "/d/usb".into(), row: "/b/Photos".into(), made: "/b/Photos".into(), dir: dir.into(), startup: false, fs: "ext4".into() }
}

struct Rig {
    d: Drives,
    lp: Loop,
    told: Rc<RefCell<VecDeque<Do>>>,

    log: Rc<RefCell<Vec<String>>>,

    fails: Rc<RefCell<HashMap<&'static str, (&'static str, &'static str)>>>,

    polkit_down: Rc<Cell<bool>>,

    down_at: Rc<Cell<Option<&'static str>>>,

    grants: Rc<Cell<bool>>,

    hold_at: Rc<Cell<Option<&'static str>>>,
    held: Rc<RefCell<Option<Asked>>>,
    _served: Vec<Served>,
    _fake: Bus,
    _bus: crate::toolkit::bus::testing::Daemon,
}

fn fakes_on_their_own_bus() -> (crate::toolkit::bus::testing::Daemon, Loop, Bus) {
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
    (bus, lp, fake)
}

fn offer(iface: &str, call: Box<dyn FnMut(Asked)>) -> Offer {
    Offer {
        name: iface.into(), xml: format!("<interface name=\"{iface}\"/>"), call, props: Box::new(Props::new), set: None,
    }
}

fn serve_polkit(fake: &Bus, log: &Rc<RefCell<Vec<String>>>, down: &Rc<Cell<bool>>, kept: &Rc<RefCell<Vec<String>>>) -> Served {
    let (l, down, k) = (log.clone(), down.clone(), kept.clone());
    fake.serve(drive_leave::AUTHORITY.1, offer(drive_leave::AUTHORITY.2, Box::new(move |a: Asked| {
        if down.get() {
            l.borrow_mut().push("polkit: no answer".into());
            return a.fail("org.freedesktop.DBus.Error.Failed", "polkit is not answering");
        }
        if a.member() == "RevokeTemporaryAuthorizationById" {
            let id = a.message().body().deserialize::<(String,)>().map(|(id,)| id).unwrap_or_default();
            k.borrow_mut().retain(|kept| *kept != id);
            l.borrow_mut().push("polkit: leave taken back".into());
            return a.reply(&());
        }
        l.borrow_mut().push("polkit: what leave is kept?".into());
        type Kept = (String, String, (String, HashMap<String, OwnedValue>), u64, u64);
        let list: Vec<Kept> = k.borrow().iter()
            .map(|id| (id.clone(), format!("{}modify-device", drive_leave::UDISKS), ("unix-session".to_string(), HashMap::new()), 0, 0))
            .collect();
        a.reply(&(list,));
    }))).expect("served")
}

impl Rig {
    fn new() -> Rig {
        let (bus, lp, fake) = fakes_on_their_own_bus();
        let log: Rc<RefCell<Vec<String>>> = Rc::default();
        let fails: Rc<RefCell<HashMap<&'static str, (&'static str, &'static str)>>> = Rc::default();
        let (polkit_down, down_at) = (Rc::new(Cell::new(false)), Rc::new(Cell::new(None)));
        let grants = Rc::new(Cell::new(false));
        let kept: Rc<RefCell<Vec<String>>> = Rc::default();
        let hold_at: Rc<Cell<Option<&'static str>>> = Rc::default();
        let held: Rc<RefCell<Option<Asked>>> = Rc::default();
        let mut served = Vec::new();
        served.push(serve_polkit(&fake, &log, &polkit_down, &kept));
        let given = Rc::new(Cell::new(0u32));
        for (path, iface) in [("/b/Photos", FILESYSTEM), ("/b/Photos", BLOCK), ("/b/Photos", PARTITION), ("/b/sdb", BLOCK), ("/b/sdb", PARTITION_TABLE)] {
            let (l, f, down, at) = (log.clone(), fails.clone(), polkit_down.clone(), down_at.clone());
            let (grants, k, given, hold_at, held) = (grants.clone(), kept.clone(), given.clone(), hold_at.clone(), held.clone());
            served.push(fake.serve(path, offer(iface, Box::new(move |a: Asked| {
                let step = a.member();
                l.borrow_mut().push(step.clone());
                if at.get() == Some(step.as_str()) {
                    down.set(true);
                }
                let failing = f.borrow().get(step.as_str()).copied();
                if grants.get() && failing.is_none() && !plan::FREE.contains(&step.as_str()) && k.borrow().is_empty() {
                    l.borrow_mut().push("password box".into());
                    given.set(given.get() + 1);
                    k.borrow_mut().push(format!("leave-{}", given.get()));
                }
                if hold_at.get() == Some(step.as_str()) {
                    *held.borrow_mut() = Some(a);
                    return;
                }
                match (failing, step.as_str()) {
                    (Some((name, text)), _) => a.fail(name, text),
                    (None, "Mount") => a.reply(&("/mnt/Photos".to_string(),)),
                    (None, "CreatePartitionAndFormat") => a.reply(&(zbus::zvariant::ObjectPath::try_from("/b/sdb1").expect("a path"),)),
                    (None, _) => a.reply(&()),
                }
            }))).expect("served"));
        }
        let told: Rc<RefCell<VecDeque<Do>>> = Rc::default();
        let t = told.clone();
        let asks = lp.post(move |d: Do| t.borrow_mut().push_back(d));
        let mut d = sample();
        (d.events, d.asks) = (lp.clone(), asks);
        d.bus = Some(Bus::at(&lp, &bus.1).expect("the window's bus"));
        d.session = Some("c7".into());
        Rig { d, lp, told, log, fails, polkit_down, down_at, grants, hold_at, held, _served: served, _fake: fake, _bus: bus }
    }

    fn until(&mut self, ready: impl Fn(&Drives) -> bool) -> bool {
        for _ in 0..400 {
            loop {
                let next = self.told.borrow_mut().pop_front();
                match next {
                    Some(Do::Objects(_)) => {}
                    Some(told) => self.d.handle(told),
                    None => break,
                }
            }
            if ready(&self.d) {
                return true;
            }
            let t = self.told.clone();
            if !crate::toolkit::bus::testing::until(&self.lp, move || !t.borrow().is_empty()) {
                return ready(&self.d);
            }
        }
        false
    }

    fn said(&mut self, drive: &'static str, row: &str) -> Option<(String, bool)> {
        let key = spot(drive, row);
        let at = row.to_string();
        assert!(self.until(move |d| !d.working.contains_key(&key) && d.told(drive, &at).is_some()), "the row said how it ended");
        self.d.told(drive, row).map(|(words, tone)| (words, matches!(tone, Tone::Problem)))
    }
}

impl Rig {

    fn wait(&self, long: Duration) {
        let over = Rc::new(Cell::new(false));
        let o = over.clone();
        self.lp.after(long, move || o.set(true));
        assert!(crate::toolkit::bus::testing::until(&self.lp, move || over.get()));
    }

    fn let_go(&self) {
        let a = self.held.borrow_mut().take().expect("a step was held");
        match a.member().as_str() {
            "CreatePartitionAndFormat" => a.reply(&(zbus::zvariant::ObjectPath::try_from("/b/sdb1").expect("a path"),)),
            _ => a.reply(&()),
        }
    }
}

fn notes(d: &Drives) -> Vec<String> {
    let mut all: Vec<PathBuf> = std::fs::read_dir(&d.noted).map(|r| r.flatten().map(|e| e.path()).collect()).unwrap_or_default();
    all.sort();
    all.iter().filter_map(|p| std::fs::read_to_string(p).ok()).collect()
}

const DISMISSED: (&str, &str) = ("org.freedesktop.UDisks2.Error.NotAuthorizedDismissed", "The authentication dialog was dismissed");

const NOT_AUTHORIZED: (&str, &str) = ("org.freedesktop.UDisks2.Error.NotAuthorized", "Not authorized to perform operation");

fn every_state(w: u32, h: u32, out: Option<&std::path::Path>) {
    let shoot = |name: &str, d: &Drives| {
        crate::toolkit::theme::drawn::page(name);
        let mut ui = Ui::new(d.view(), Theme::chosen());
        let mut pix = tiny_skia::Pixmap::new(w, h).unwrap();
        ui.paint(&mut pix.as_mut(), 1.0);
        if let Some(out) = out {
            pix.save_png(out.join(format!("drives-{name}-{w}.png"))).unwrap();
        }
    };
    let mut d = sample();
    shoot("full", &d);

    let mut second = made_up_window("shots");
    shoot("second", &second);

    let key = second.drives.iter().find(|d| d.name == SECOND).map(|d| d.key.clone()).unwrap();
    second.update(Msg::Hide { drive: String::new(), row: String::new(), key, hide: true });
    shoot("second-hidden", &second);
    let _ = std::fs::remove_dir_all(second.hidden_file.parent().unwrap());
    d.pressed_at = Rect { x: 400.0, y: 300.0, w: 34.0, h: 34.0 };
    d.update(Msg::Toggle("/b/Data".into()));
    shoot("changes", &d);
    d.update(Msg::Help);
    shoot("help", &d);
    let mut lone = sample();
    lone.drives.truncate(1);
    lone.sum_up();
    shoot("lone", &lone);
    let loading = Drives::new(&d.events, d.asks.clone());
    shoot("loading", &loading);
    let mut missing = Drives::new(&d.events, d.asks.clone());
    missing.missing = Some(("The drive service", "udisks2"));
    shoot("missing", &missing);
    let mut empty = Drives::new(&d.events, d.asks.clone());
    empty.loaded = true;
    empty.sum_up();
    shoot("empty", &empty);
}

fn drives_code() -> String {
    let files = [
        include_str!("mod.rs"),
        include_str!("actions.rs"),
        include_str!("app.rs"),
        include_str!("cards.rs"),
        include_str!("job.rs"),
        include_str!("map.rs"),
        include_str!("model.rs"),
        include_str!("refresh.rs"),
        include_str!("rows.rs"),
        include_str!("steps.rs"),
        include_str!("udisks.rs"),
        include_str!("view.rs"),
    ];
    files.iter().map(|f| crate::common::source::code(f).replace("#[cfg(test)]\nmod tests;\n", "")).collect::<Vec<_>>().join("\n")
}
