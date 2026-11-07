//! A drive plugged in.

use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};
use std::time::Duration;

use crate::common::volumes::{self, Volume, Watching};

use super::ctx::Ctx;
use super::notify;
use crate::toolkit::t;

const RECHECK: Duration = Duration::from_secs(2);

#[derive(Debug, Default, PartialEq)]
pub struct Change {
    pub arrived: Vec<String>,
    pub gone: Vec<String>,
    pub held: bool,
}

#[derive(Default)]
pub struct Roll {
    seen: Option<HashSet<String>>,
}

impl Roll {
    pub fn update(&mut self, list: &[Volume], locked: bool) -> Change {
        let mut now: Vec<&str> = Vec::new();
        for v in list {
            if !now.contains(&v.drive.as_str()) {
                now.push(&v.drive);
            }
        }
        let Some(seen) = self.seen.as_mut() else {
            self.seen = Some(now.iter().map(|d| d.to_string()).collect());
            return Change::default();
        };
        let gone: Vec<String> = seen.iter().filter(|d| !now.contains(&d.as_str())).cloned().collect();
        for d in &gone {
            seen.remove(d);
        }
        let new: Vec<&str> = now.into_iter().filter(|d| !seen.contains(*d)).collect();
        if locked {
            return Change { arrived: Vec::new(), gone, held: !new.is_empty() };
        }
        let arrived: Vec<String> = new.iter().map(|d| d.to_string()).collect();
        seen.extend(arrived.iter().cloned());
        Change { arrived, gone, held: false }
    }
}

pub struct Drives {
    ctx: Rc<Ctx>,
    me: Weak<Drives>,
    roll: RefCell<Roll>,
    list: RefCell<Vec<Volume>>,
    notes: RefCell<HashMap<String, u32>>,
    waiting: Cell<bool>,
    watching: RefCell<Option<Watching>>,
}

pub fn start(ctx: &Rc<Ctx>) -> Option<Rc<Drives>> {
    let bus = ctx.system.clone()?;
    let drives = Rc::new_cyclic(|me| Drives {
        ctx: ctx.clone(),
        me: me.clone(),
        roll: RefCell::new(Roll::default()),
        list: RefCell::new(Vec::new()),
        notes: RefCell::new(HashMap::new()),
        waiting: Cell::new(false),
        watching: RefCell::new(None),
    });
    let w = Rc::downgrade(&drives);
    let watching = volumes::watch(&bus, &ctx.lp, move |list| {
        if let Some(d) = w.upgrade() {
            *d.list.borrow_mut() = list.to_vec();
            d.settle();
        }
    });
    *drives.watching.borrow_mut() = Some(watching);
    Some(drives)
}

fn locked() -> bool {
    crate::common::locked::path().exists()
}

impl Drives {
    fn settle(&self) {
        let list = self.list.borrow().clone();
        let change = self.roll.borrow_mut().update(&list, locked());
        for drive in &change.gone {
            if let Some(id) = self.notes.borrow_mut().remove(drive) {
                notify::take_back(id);
            }
        }
        for drive in &change.arrived {
            self.announce(&list, drive);
        }
        if change.held && !self.waiting.replace(true) {
            let me = self.me.clone();
            self.ctx.lp.after(RECHECK, move || {
                if let Some(d) = me.upgrade() {
                    d.waiting.set(false);
                    d.settle();
                }
            });
        }
    }

    fn announce(&self, list: &[Volume], drive: &str) {
        let parts: Vec<&Volume> = list.iter().filter(|v| v.drive == drive).collect();
        let Some(v) = volumes::best(&parts).cloned() else { return };
        let id = notify::say_drive(&v.notice(), drive);
        self.notes.borrow_mut().insert(drive.to_string(), id);
        if opens_drives() && !v.locked {
            self.open(drive);
        }
    }

    pub fn open(&self, drive: &str) {
        if locked() {
            return;
        }
        let list = self.list.borrow().clone();
        let parts: Vec<&Volume> = list.iter().filter(|v| v.drive == drive).collect();
        let Some(v) = volumes::best(&parts).cloned() else {
            notify::say_here(t!("ring-drive-gone"), "");
            return;
        };
        let Some(bus) = self.ctx.system.clone() else { return };
        let lp = self.ctx.lp.clone();
        let name = v.name();
        if v.locked {
            volumes::ask_and_mount(&bus, &lp, &self.ctx.display, &v, move |r| opened(&name, r));
            return;
        }
        volumes::mount(&bus, &lp, &v, move |r| opened(&name, r));
    }
}

fn opened(name: &str, got: Result<String, String>) {
    match got {
        Ok(at) => {
            if let Some(files) = crate::common::apps::own_key("files") {
                super::windows::Windows::raise_key("files");
                crate::common::run::spawn(&[files.start_with(), &at]);
            }
        }
        Err(why) if why.is_empty() => {}
        Err(why) => notify::say_here(&t!("ring-drive-not-opened", name = name), &why),
    }
}

fn opens_drives() -> bool {
    #[cfg(test)]
    return false;
    #[cfg(not(test))]
    crate::common::files::read_env(&crate::common::files::config_path("folders.conf")).get("open_drives").is_some_and(|v| v.trim() == "yes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::volumes::Kind;

    fn vol(drive: &str, dev: &str) -> Volume {
        Volume {
            id: format!("/block/{dev}"), dev: dev.into(), label: String::new(), size: 1_000_000_000,
            drive: drive.into(), drive_name: "Stick".into(), kind: Kind::Usb, mounts: Vec::new(), locked: false,
            container: None, can_eject: false, can_power_off: true, fixed: false, system: false, key: String::new(), drive_key: String::new(),
        }
    }

    #[test]
    fn what_was_plugged_in_already_is_not_news() {
        let mut roll = Roll::default();
        assert_eq!(roll.update(&[vol("/d/a", "sda1")], false), Change::default());
        assert_eq!(roll.update(&[vol("/d/a", "sda1")], false), Change::default());
    }

    #[test]
    fn a_new_drive_is_told_once_and_a_removed_one_goes() {
        let mut roll = Roll::default();
        roll.update(&[], false);
        let two = [vol("/d/a", "sda1"), vol("/d/a", "sda2")];
        let c = roll.update(&two, false);
        assert_eq!(c.arrived, vec!["/d/a".to_string()], "two parts, one drive, one notice");
        assert_eq!(roll.update(&two, false), Change::default());
        let c = roll.update(&[], false);
        assert_eq!(c.gone, vec!["/d/a".to_string()]);
        assert_eq!(roll.update(&two, false).arrived.len(), 1);
    }

    #[test]
    fn nothing_new_is_told_while_the_screen_is_locked() {
        let mut roll = Roll::default();
        roll.update(&[], false);
        let one = [vol("/d/b", "sdb1")];
        let c = roll.update(&one, true);
        assert!(c.arrived.is_empty() && c.held, "{c:?}");
        let c = roll.update(&one, true);
        assert!(c.arrived.is_empty() && c.held);
        let c = roll.update(&one, false);
        assert_eq!(c.arrived, vec!["/d/b".to_string()]);
        assert!(!c.held);
    }

    #[test]
    fn a_hidden_drive_is_never_news_and_comes_back_when_shown() {
        use std::collections::BTreeSet;
        let mut stick = vol("/d/h", "sdh1");
        (stick.key, stick.drive_key) = ("fs:1111-AAAA".into(), "drive:Example-Stick-0001".into());
        let other = vol("/d/a", "sda1");
        let plugged = vec![stick.clone(), other.clone()];
        let hidden = BTreeSet::from(["drive:Example-Stick-0001".to_string()]);
        let mut roll = Roll::default();
        roll.update(&[], false);
        let listed = volumes::unhidden(plugged.clone(), &hidden);
        let c = roll.update(&listed, false);
        assert_eq!(c.arrived, vec!["/d/a".to_string()], "only the other drive is told of");
        assert!(volumes::best(&listed.iter().filter(|v| v.drive == "/d/h").collect::<Vec<_>>()).is_none(), "there is nothing of it to open");
        let c = roll.update(&volumes::unhidden(plugged, &BTreeSet::new()), false);
        assert_eq!(c.arrived, vec!["/d/h".to_string()]);
    }

    #[test]
    fn one_plugged_and_pulled_out_while_locked_is_never_told() {
        let mut roll = Roll::default();
        roll.update(&[], false);
        roll.update(&[vol("/d/c", "sdc1")], true);
        let c = roll.update(&[], true);
        assert_eq!(c, Change::default());
        assert_eq!(roll.update(&[], false), Change::default());
    }
}
