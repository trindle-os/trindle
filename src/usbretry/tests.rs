//! Fake /sys in a scratch folder.

use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

use super::*;

type Writes = Rc<RefCell<Vec<(PathBuf, String)>>>;

struct Fake {
    root: PathBuf,
    places: Places,
}

impl Fake {
    fn new() -> Fake {
        let root = crate::common::files::private_temp("usbretry").unwrap();
        let places = Places { sys: root.join("sys"), log: root.join("run/log") };
        fs::create_dir_all(places.sys.join("bus/usb/devices")).unwrap();
        Fake { root, places }
    }

    fn port(&self, hub: &str, port: &str, switch: bool, over: u64) -> PathBuf {
        let dir = self.places.sys.join("bus/usb/devices").join(hub).join(port);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("over_current_count"), format!("{over}\n")).unwrap();
        if switch {
            fs::write(dir.join("disable"), "0\n").unwrap();
        }
        dir
    }

    fn child(&self, name: &str) {
        fs::create_dir_all(self.places.sys.join("bus/usb/devices").join(name)).unwrap();
    }

    fn retrier(&self) -> (Retrier, Writes) {
        let writes: Writes = Rc::default();
        let w = writes.clone();
        let r = Retrier::with_io(
            self.places.clone(),
            Box::new(move |p, v| {
                w.borrow_mut().push((p.to_path_buf(), v.to_string()));
                Ok(())
            }),
            Box::new(|_| {}),
        );
        (r, writes)
    }
}

impl Drop for Fake {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn names_are_strict() {
    for ok in ["usb3-port1", "3-1-port2", "3-1.4-port3", "1-10.2.3-port12"] {
        assert!(valid_port(ok), "{ok}");
    }
    for bad in ["", "port1", "usb-port1", "usb3-port", "3-1-port", "../x-port1", "3-1/../-port1", "3-1-port1x",
        "a-1-port1", "3--port1", "3-1..2-port1", "usb3-port1\n", "3-1.-port1"] {
        assert!(!valid_port(bad), "{bad:?}");
    }
}

#[test]
fn reads_the_kernels_words() {
    let a = "3,812,5000;usb 3-1-port2: unable to enumerate USB device\n SUBSYSTEM=usb\n";
    assert_eq!(failed_port(a).as_deref(), Some("3-1-port2"));
    let b = "3,812,5000;usb usb3-port1: unable to enumerate USB device";
    assert_eq!(failed_port(b).as_deref(), Some("usb3-port1"));
    let c = "3,812,5000;3-1.4-port3: unable to enumerate USB device";
    assert_eq!(failed_port(c).as_deref(), Some("3-1.4-port3"));
    assert_eq!(failed_port("3,1,5;usb 3-1: device descriptor read/64, error -71"), None);
    assert_eq!(failed_port("3,1,5;usb ../../etc: unable to enumerate USB device"), None);
    assert_eq!(failed_port("3,1,5;hello unable to enumerate USB device"), None);
    assert_eq!(failed_port("3,1,5;usb 3-1-port2: unable to enumerate USB device later"), None);
}

#[test]
fn switches_off_then_on_once_and_notes_it() {
    let f = Fake::new();
    let dir = f.port("3-1:1.0", "3-1-port2", true, 0);
    let (mut r, writes) = f.retrier();
    assert_eq!(r.consider("3-1-port2"), Ok(Outcome::Nothing));
    assert_eq!(
        *writes.borrow(),
        vec![(dir.join("disable"), "1".to_string()), (dir.join("disable"), "0".to_string())]
    );
    let log = read_log(&f.places.log);
    assert_eq!(log, vec![Entry { port: "3-1-port2".into(), worked: false, controller: false }]);
    assert!(log[0].words().contains("did not come back after a restart"));
    assert!(log[0].words().contains("Switch the computer off and on"));
}

#[test]
fn never_a_second_time() {
    let f = Fake::new();
    f.port("3-1:1.0", "3-1-port2", true, 0);
    let (mut r, writes) = f.retrier();
    assert!(r.consider("3-1-port2").is_ok());
    for _ in 0..5 {
        assert_eq!(r.consider("3-1-port2"), Err(Skip::Done));
    }
    assert_eq!(writes.borrow().len(), 2);
    let (mut again, writes2) = f.retrier();
    assert_eq!(again.consider("3-1-port2"), Err(Skip::Done));
    assert!(writes2.borrow().is_empty());
}

#[test]
fn only_a_few_ports_in_all() {
    let f = Fake::new();
    for n in 1..=(MOST + 3) {
        f.port("3-0:1.0", &format!("usb3-port{n}"), true, 0);
    }
    let (mut r, writes) = f.retrier();
    for n in 1..=(MOST + 3) {
        let got = r.consider(&format!("usb3-port{n}"));
        if n <= MOST {
            assert!(got.is_ok());
        } else {
            assert_eq!(got, Err(Skip::TooMany));
        }
    }
    assert_eq!(writes.borrow().len(), MOST * 2);
}

#[test]
fn leaves_alone_what_is_not_safe() {
    let f = Fake::new();
    f.port("3-0:1.0", "usb3-port1", true, 0);
    f.child("3-1");
    f.port("3-0:1.0", "usb3-port2", true, 2);
    f.port("3-0:1.0", "usb3-port3", false, 0);
    let (mut r, writes) = f.retrier();
    assert_eq!(r.consider("usb3-port1"), Err(Skip::Busy));
    assert_eq!(r.consider("usb3-port2"), Err(Skip::Overcurrent));
    assert_eq!(r.consider("usb3-port3"), Err(Skip::NoSwitch));
    assert_eq!(r.consider("usb3-port9"), Err(Skip::NotFound));
    assert_eq!(r.consider("../../etc-port1"), Err(Skip::Done));
    assert!(writes.borrow().is_empty());
    assert!(read_log(&f.places.log).is_empty());
}

#[test]
fn says_when_a_device_came_up() {
    let f = Fake::new();
    f.port("3-1:1.0", "3-1-port2", true, 0);
    let sys = f.places.sys.clone();
    let naps = Rc::new(RefCell::new(0u32));
    let n = naps.clone();
    let mut r = Retrier::with_io(
        f.places.clone(),
        Box::new(|_, _| Ok(())),
        Box::new(move |_| {
            *n.borrow_mut() += 1;
            if *n.borrow() == 4 {
                fs::create_dir_all(sys.join("bus/usb/devices/3-1.2")).unwrap();
            }
        }),
    );
    assert_eq!(r.consider("3-1-port2"), Ok(Outcome::Worked));
    let log = read_log(&f.places.log);
    assert!(log[0].worked);
    assert!(log[0].words().contains("A device came up"));
}

#[test]
fn a_switch_that_fails_still_ends_on_and_never_loops() {
    let f = Fake::new();
    f.port("3-1:1.0", "3-1-port2", true, 0);
    let writes: Writes = Rc::default();
    let w = writes.clone();
    let mut r = Retrier::with_io(
        f.places.clone(),
        Box::new(move |p, v| {
            w.borrow_mut().push((p.to_path_buf(), v.to_string()));
            Err(io::Error::other("no"))
        }),
        Box::new(|_| {}),
    );
    assert_eq!(r.consider("3-1-port2"), Ok(Outcome::Nothing));
    assert_eq!(writes.borrow().iter().map(|x| x.1.as_str()).collect::<Vec<_>>(), ["1", "0"]);
    assert_eq!(r.consider("3-1-port2"), Err(Skip::Done));
}

#[test]
fn watching_the_log_tries_each_named_port_once() {
    let f = Fake::new();
    f.port("3-0:1.0", "usb3-port1", true, 0);
    let kmsg = f.root.join("kmsg");
    fs::write(&kmsg, "3,1,5;usb usb3-port1: unable to enumerate USB device").unwrap();
    let (mut r, writes) = f.retrier();
    watch(&mut r, &kmsg).unwrap();
    watch(&mut r, &kmsg).unwrap();
    assert_eq!(writes.borrow().len(), 2);
}

fn controller(f: &Fake, driver: &str) -> PathBuf {
    let pci = f.places.sys.join("devices/pci0000:00/0000:e5:00.3");
    let iface = pci.join("usb3/3-0:1.0");
    fs::create_dir_all(iface.join("usb3-port1")).unwrap();
    fs::write(iface.join("usb3-port1/disable"), "0\n").unwrap();
    fs::write(iface.join("usb3-port1/over_current_count"), "0\n").unwrap();
    let drv = f.places.sys.join(format!("bus/pci/drivers/{driver}"));
    fs::create_dir_all(&drv).unwrap();
    std::os::unix::fs::symlink(&drv, pci.join("driver")).unwrap();
    std::os::unix::fs::symlink(&iface, f.places.sys.join("bus/usb/devices/3-0:1.0")).unwrap();
    pci
}

fn showing_up_after_bind(f: &Fake, appear: bool) -> (Retrier, Writes) {
    let writes: Writes = Rc::default();
    let w = writes.clone();
    let sys = f.places.sys.clone();
    let r = Retrier::with_io(
        f.places.clone(),
        Box::new(move |p, v| {
            w.borrow_mut().push((p.to_path_buf(), v.to_string()));
            if appear && p.ends_with("xhci_hcd/bind") {
                fs::create_dir_all(sys.join("bus/usb/devices/3-1")).unwrap();
            }
            Ok(())
        }),
        Box::new(|_| {}),
    );
    (r, writes)
}

#[test]
fn restarts_an_idle_controller_when_the_port_still_gives_nothing() {
    let f = Fake::new();
    controller(&f, "xhci_hcd");
    let (mut r, writes) = showing_up_after_bind(&f, true);
    assert_eq!(r.consider("usb3-port1"), Ok(Outcome::WorkedAfterController));
    let w: Vec<String> =
        writes.borrow().iter().map(|(p, v)| format!("{} {v}", p.file_name().unwrap().to_string_lossy())).collect();
    assert_eq!(w, ["disable 1", "disable 0", "unbind 0000:e5:00.3", "bind 0000:e5:00.3"]);
    let log = read_log(&f.places.log);
    assert!(log[0].worked && log[0].controller);
    assert!(log[0].words().contains("restarted its USB controller"));
}

#[test]
fn a_controller_still_dead_after_that_says_to_switch_off_and_on() {
    let f = Fake::new();
    controller(&f, "xhci_hcd");
    let (mut r, _) = showing_up_after_bind(&f, false);
    assert_eq!(r.consider("usb3-port1"), Ok(Outcome::NothingAfterController));
    let log = read_log(&f.places.log);
    assert!(!log[0].worked && log[0].controller);
    assert_eq!(
        log[0].words(),
        "USB port usb3-port1 did not start. Trindle restarted its USB controller and nothing came up. Switch the computer off and on to bring it back."
    );
}

fn desktop(f: &Fake) -> PathBuf {
    let pci = controller(f, "xhci_hcd");
    for (dev, class) in [("3-2", "07"), ("3-3", "ff")] {
        let iface = pci.join(format!("usb3/{dev}/{dev}:1.0"));
        fs::create_dir_all(&iface).unwrap();
        fs::write(iface.join("bInterfaceClass"), format!("{class}\n")).unwrap();
    }
    pci
}

#[test]
fn the_desktops_bus_with_a_printer_and_a_dongle_is_restarted() {
    let f = Fake::new();
    desktop(&f);
    let (mut r, writes) = showing_up_after_bind(&f, true);
    assert_eq!(r.consider("usb3-port1"), Ok(Outcome::WorkedAfterController));
    let w: Vec<String> = writes.borrow().iter().map(|(_, v)| v.clone()).collect();
    assert_eq!(w, ["1", "0", "0000:e5:00.3", "0000:e5:00.3"]);
}

#[test]
fn a_controller_with_a_disk_on_it_is_never_restarted() {
    for how in 0..3 {
        let f = Fake::new();
        let pci = desktop(&f);
        let iface = pci.join("usb3/3-4/3-4:1.0");
        fs::create_dir_all(&iface).unwrap();
        match how {
            0 => fs::write(iface.join("bInterfaceClass"), "08\n").unwrap(),
            1 => {
                fs::write(iface.join("bInterfaceClass"), "ff\n").unwrap();
                let drv = f.places.sys.join("bus/usb/drivers/uas");
                fs::create_dir_all(&drv).unwrap();
                std::os::unix::fs::symlink(&drv, iface.join("driver")).unwrap();
            }
            _ => fs::create_dir_all(iface.join("host6/target6:0:0/6:0:0:0/block/sda")).unwrap(),
        }
        assert!(!controller_safe(&pci), "{how}");
        let (mut r, writes) = showing_up_after_bind(&f, true);
        assert_eq!(r.consider("usb3-port1"), Ok(Outcome::Nothing), "{how}");
        assert_eq!(writes.borrow().len(), 2, "{how}");
        let log = read_log(&f.places.log);
        assert!(!log[0].controller);
    }
}

#[test]
fn only_a_pci_xhci_controller_is_restarted_and_only_once() {
    let f = Fake::new();
    controller(&f, "ehci-pci");
    let (mut r, writes) = showing_up_after_bind(&f, true);
    assert_eq!(r.consider("usb3-port1"), Ok(Outcome::Nothing));
    assert_eq!(writes.borrow().len(), 2);
    assert!(valid_pci("0000:e5:00.3"));
    for bad in ["", "0000:e5:00.8", "0000:e5:00:3", "../../x/0000:e5", "0000:E5:00.3", "0000:e5:00.3\n"] {
        assert!(!valid_pci(bad), "{bad:?}");
    }
    let g = Fake::new();
    let pci = controller(&g, "xhci_hcd");
    fs::create_dir_all(pci.join("usb3/3-0:1.0/usb3-port2")).unwrap();
    fs::write(pci.join("usb3/3-0:1.0/usb3-port2/disable"), "0\n").unwrap();
    let (mut r, writes) = showing_up_after_bind(&g, false);
    assert_eq!(r.consider("usb3-port1"), Ok(Outcome::NothingAfterController));
    assert_eq!(r.consider("usb3-port2"), Ok(Outcome::Nothing));
    let unbinds = writes.borrow().iter().filter(|(p, _)| p.ends_with("unbind")).count();
    assert_eq!(unbinds, 1);
}
