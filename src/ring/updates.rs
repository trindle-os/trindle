//! Updates, from the ring: a button in the middle that is always there.
//! Its name says how things stand in the fewest words: how many wait and
//! what pressing will do, "Up to date", or who is installing. Pressing it
//! opens Apps at its Updates page. When updates wait that can go in, and
//! the press was the person's own, Apps starts them there, as its own
//! Install all would (common/update_asked.rs carries the proof of the
//! person's press). The ring installs nothing itself. Firmware is never
//! put in unseen, since it can need the charger and a restart, so for
//! firmware the page opens at the firmware instead.

use std::cell::Cell;
use std::rc::{Rc, Weak};
use std::time::{Duration, Instant};

use crate::common::held::{self, Held};
use crate::common::run::{self, capture};
use crate::common::update_checks;
use crate::common::updating::{self, Who};
use crate::plugins::catalogue::{self, Offer};
use crate::plugins::{self, Plugin};
use crate::settings::ui::{UPDATE_ICON, UP_TO_DATE_ICON};
use crate::toolkit::event::Flow;
use crate::toolkit::widget::{button, Node};

use super::ctx::{msg, Ctx, Msg};
use crate::toolkit::t;

const EVERY_SECS: u64 = 3 * 60 * 60;

const FIRST_SECS: u64 = 90;

const STALE_SECS: u64 = 10 * 60;

const FIRMWARE_SECS: u64 = 24 * 60 * 60;

fn count_lines(text: &str, keep: impl Fn(&str) -> bool) -> usize {
    text.lines().filter(|l| keep(l)).count()
}

fn upgradable(line: &str) -> bool {
    line.contains("[upgradable")
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Waiting {

    pub system: usize,

    pub apps: usize,

    pub firmware: usize,

    pub plugins: usize,

    pub look: usize,
}

impl Waiting {
    pub fn total(&self) -> usize {
        self.system + self.apps + self.firmware + self.plugins + self.look
    }

    pub fn only_a_look(&self) -> bool {
        self.look > 0 && self.total() == self.look
    }

    fn look_words(&self) -> String {
        t!("ring-updates-look", n = self.look)
    }

    pub fn spoken(&self) -> String {
        let (n, put) = (self.total(), self.total() - self.look);
        if n == 0 {
            t!("ring-updates-none").into()
        } else if self.look == 0 || self.worth_a_look() {
            t!("ring-updates-spoken-all", n = n)
        } else if put == 0 {
            t!("ring-updates-spoken-look", n = n, look = self.look_words())
        } else {
            t!("ring-updates-spoken-mixed", n = n, put = put, look = self.look_words())
        }
    }

    pub fn worth_a_look(&self) -> bool {
        self.firmware > 0
    }

    pub fn words(&self) -> String {
        let n = self.total();
        if self.worth_a_look() {
            if n == self.firmware {
                t!("ring-updates-see-firmware", n = n)
            } else {
                t!("ring-updates-see-among", n = n)
            }
        } else if self.look == 0 {
            t!("ring-updates-install", n = n)
        } else if self.only_a_look() {
            t!("ring-updates-see-plugin", n = n)
        } else {
            let put = n - self.look;
            t!("ring-updates-install-and-look", put = put, look = self.look_words())
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pending {
    pub id: String,

    pub look: bool,
}

fn pending(installed: &[Plugin], offers: &[Offer]) -> Vec<Pending> {
    catalogue::updates_for(installed, offers).into_iter()
        .map(|(mine, offer)| Pending { look: !catalogue::said_to_ask_more(&mine, &offer).is_empty(), id: mine.id })
        .collect()
}

fn plugins_pending(net: bool) -> Vec<Pending> {
    let installed = plugins::installed();
    if installed.is_empty() {
        return Vec::new();
    }
    let offers = if net { catalogue::offers_to_count(catalogue::INDEX_FRESH) } else { catalogue::offers_kept() };
    pending(&installed, &offers)
}

fn flatpak_args(net: bool) -> Vec<&'static str> {
    let mut args = vec!["flatpak", "remote-ls", "--updates", "--user", "--columns=ref"];
    if !net {
        args.push("--cached");
    }
    args
}

fn ask_firmware(asked: Option<Instant>, last: usize) -> bool {
    last > 0 || asked.is_none_or(|t| t.elapsed().as_secs() >= FIRMWARE_SECS)
}

fn counted(apt: &str, flatpaks: &str, firmware: &[String], plugins: &[Pending], held: &Held) -> Waiting {
    let waits = |look: bool| plugins.iter().filter(|p| p.look == look && !held.plugin(&p.id)).count();
    Waiting {
        system: count_lines(apt, upgradable),
        apps: held::flatpaks_to_update(flatpaks, held).len(),
        firmware: firmware.iter().filter(|id| !held.firmware(id)).count(),
        plugins: waits(false),
        look: waits(true),
    }
}

pub fn waiting(_ctx: &Rc<Ctx>, firmware: Option<Vec<String>>, net: bool, done: impl FnOnce(Waiting, Vec<String>) + 'static) {
    capture(&["apt", "list", "--upgradable"], None, move |_, apt, _| {
        capture(&flatpak_args(net), None, move |_, apps, _| {

            run::on_thread(move || plugins_pending(net), move |plugins| {
                let answer = move |firmware: Vec<String>| {
                    let w = counted(&apt, &apps, &firmware, &plugins, &Held::read());
                    done(w, firmware);
                };
                match firmware {
                    Some(firmware) => answer(firmware),
                    None => crate::store::firmware::waiting(move |list| answer(list.into_iter().map(|f| f.id).collect())),
                }
            });
        });
    });
}

pub struct Updates {
    ctx: Rc<Ctx>,
    last: Waiting,
    looked: Cell<Option<Instant>>,
    firmware_asked: Option<Instant>,

    firmware: Vec<String>,

    held_at: Option<std::time::SystemTime>,

    other: Option<Who>,

    off: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Stand {

    UpToDate,

    Waiting,

    Elsewhere(Who),
}

fn stand(waiting: Waiting, other: Option<Who>) -> Stand {
    match other {
        Some(who) => Stand::Elsewhere(who),
        None if waiting.total() == 0 => Stand::UpToDate,
        None => Stand::Waiting,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Opens {

    AndStarts,

    AtFirmware,

    AtPlugins,

    Alone,
}

fn opens(waiting: Waiting, other: Option<Who>, persons_own: bool) -> Opens {
    if other.is_some() {
        Opens::Alone
    } else if waiting.worth_a_look() {
        Opens::AtFirmware
    } else if persons_own && waiting.total() > waiting.look {
        Opens::AndStarts
    } else if waiting.only_a_look() {
        Opens::AtPlugins
    } else {
        Opens::Alone
    }
}

fn named(waiting: Waiting, other: Option<Who>) -> String {
    match stand(waiting, other) {
        Stand::UpToDate => t!("ring-updates-up-to-date").into(),
        Stand::Waiting => waiting.words(),
        Stand::Elsewhere(who) => who.short().into(),
    }
}

fn unchecked(waiting: Waiting, other: Option<Who>, off: bool) -> bool {
    off && stand(waiting, other) == Stand::UpToDate
}

impl Updates {
    pub fn new(ctx: &Rc<Ctx>) -> Updates {
        let first = Rc::downgrade(ctx);
        ctx.lp.after(Duration::from_secs(FIRST_SECS), move || look_later(&first));
        let every = Rc::downgrade(ctx);
        ctx.lp.every(Duration::from_secs(EVERY_SECS), move || {
            look_later(&every);
            Flow::Continue
        });
        Updates { ctx: ctx.clone(), last: Waiting::default(), looked: Cell::new(None), firmware_asked: None, firmware: Vec::new(),
                  held_at: held::changed_at(), other: None, off: !update_checks::on() }
    }

    pub fn look(&mut self) {
        self.looked.set(Some(Instant::now()));
        let keep = self.firmware.clone();
        let firmware = if ask_firmware(self.firmware_asked, keep.len()) {
            self.firmware_asked = Some(Instant::now());
            None
        } else {
            Some(keep)
        };
        let ctx = self.ctx.clone();
        self.off = !update_checks::on();
        let net = update_checks::may_ask_network(false, !self.off);
        waiting(&self.ctx, firmware, net, move |w, devices| ctx.face(move |f| (f.updates.last, f.updates.firmware) = (w, devices)));
    }

    pub fn notice(&mut self) -> bool {
        let off = !update_checks::on();
        let switched = off != self.off;
        self.off = off;
        let other = updating::other();
        let held_at = held::changed_at();
        let changed = other != self.other;
        let finished = changed && other.is_none();
        let held_changed = held_at != self.held_at;
        (self.other, self.held_at) = (other, held_at);
        if finished || held_changed {
            self.look();
        }
        changed || held_changed || switched
    }

    pub fn other(&self) -> Option<Who> {
        self.other
    }

    pub fn button(&self, size: f32) -> Option<Node<Msg>> {

        if self.looked.get().is_some_and(|t| t.elapsed().as_secs() > STALE_SECS) {
            self.looked.set(Some(Instant::now()));
            self.ctx.face(|f| f.updates.look());
        }
        let unchecked = unchecked(self.last, self.other, self.off);
        let icon = if stand(self.last, self.other) == Stand::UpToDate && !unchecked { UP_TO_DATE_ICON } else { UPDATE_ICON };
        let name = if unchecked { t!("ring-updates-off").into() } else { named(self.last, self.other) };
        Some(button(icon, name).size(size).on_press(msg(|f| f.updates.press())))
    }

    fn press(&mut self) {
        let own = super::kept::own_press();
        match opens(self.last, self.other, own) {
            Opens::AtFirmware => self.ctx.face(|f| f.open_own("store", &["--updates=firmware"])),
            Opens::AtPlugins => self.ctx.face(|f| f.open_own("store", &["--updates=plugins"])),
            Opens::Alone => self.ctx.face(|f| f.open_own("store", &["--updates"])),
            Opens::AndStarts => {

                if crate::common::update_asked::leave() {
                    self.ctx.face(|f| f.open_own("store", &["--updates", "--update-now"]));
                } else {
                    self.ctx.face(|f| f.open_own("store", &["--updates"]));
                }
            }
        }
    }
}

fn look_later(ctx: &Weak<Ctx>) {
    if let Some(ctx) = ctx.upgrade() {
        ctx.face(|f| f.updates.look());
    }
}

#[cfg(test)]
mod tests {
    use super::{ask_firmware, named, opens, stand, Opens, Pending, Stand, Waiting, FIRMWARE_SECS};
    use crate::common::updating::{self, Lock, Who};
    use std::time::Instant;

    #[test]
    fn fwupd_is_asked_once_a_day_unless_it_offered_something() {
        assert!(ask_firmware(None, 0));
        assert!(!ask_firmware(Some(Instant::now()), 0));
        assert!(ask_firmware(Some(Instant::now()), 1));
        let long_ago = Instant::now().checked_sub(std::time::Duration::from_secs(FIRMWARE_SECS));
        if let Some(t) = long_ago {
            assert!(ask_firmware(Some(t), 0));
        }
    }

    #[test]
    fn the_button_is_always_there_and_its_name_says_how_things_stand() {
        let path = updating::tests::temp("ring-button");
        updating::pretend(Some(path.clone()), false);
        let waiting = Waiting { system: 3, apps: 2, ..Waiting::default() };
        assert_eq!(stand(waiting, updating::other()), Stand::Waiting);
        assert_eq!(named(waiting, updating::other()), "Install 5 updates");

        assert_eq!(stand(Waiting::default(), None), Stand::UpToDate);
        assert_eq!(named(Waiting::default(), None), "Up to date");

        let apps = Lock::take(&path, Who::Apps).expect("free");
        assert_eq!(stand(waiting, updating::other()), Stand::Elsewhere(Who::Apps));
        assert_eq!(named(waiting, updating::other()), "Updating in Apps");
        assert_eq!(named(Waiting::default(), updating::other()), "Updating in Apps", "whether or not anything is counted");
        assert!(updating::begin(Who::Ring).is_err(), "and the ring starts nothing itself");
        drop(apps);
        assert_eq!(stand(waiting, updating::other()), Stand::Waiting);

        updating::pretend(Some(path), true);
        assert_eq!(named(waiting, updating::other()), "Debian is updating");
        updating::pretend(None, false);
    }

    #[test]
    fn the_press_always_opens_apps_and_starts_updates_only_when_it_may() {
        let w = |system, apps, firmware, plugins, look| Waiting { system, apps, firmware, plugins, look };
        let nothing = Waiting::default();
        let some = w(3, 2, 0, 0, 0);

        assert_eq!(opens(nothing, None, true), Opens::Alone);

        assert_eq!(opens(some, None, true), Opens::AndStarts);
        assert_eq!(opens(some, None, false), Opens::Alone, "a made-up press opens the page and starts nothing");

        assert_eq!(opens(w(0, 0, 0, 2, 0), None, true), Opens::AndStarts);
        assert_eq!(opens(w(0, 0, 0, 0, 2), None, true), Opens::AtPlugins, "only plugins that ask for more: Apps opens at them, starts nothing");
        assert_eq!(opens(w(1, 0, 0, 0, 2), None, true), Opens::AndStarts, "the one that can go in does, the rest stay");

        assert_eq!(opens(w(0, 0, 1, 0, 0), None, true), Opens::AtFirmware);
        assert_eq!(opens(w(4, 1, 1, 0, 0), None, true), Opens::AtFirmware);

        for who in [Who::Apps, Who::Boxes, Who::System, Who::Unknown] {
            assert_eq!(opens(some, Some(who), true), Opens::Alone);
            assert_eq!(opens(w(0, 0, 1, 0, 0), Some(who), true), Opens::Alone);
        }

        assert_eq!(opens(Waiting::default(), None, true), Opens::Alone);
    }

    #[test]
    fn the_ring_installs_nothing_and_asks_only_for_a_persons_press() {
        let source = crate::common::source::code(include_str!("updates.rs")).split_once("#[cfg(test)]").unwrap().0;
        for gone in ["updating::begin(", "\"-y\"", "\"apt-upgrade\"", "fetch_update_unseen", "Dial"] {
            assert!(!source.contains(gone), "the ring no longer installs: {gone}");
        }
        let press = source.split_once("fn press(").unwrap().1.split_once("\n    }\n").unwrap().0;
        assert!(press.contains("kept::own_press()"));
        assert!(press.find("kept::own_press()").unwrap() < press.find("update_asked::leave()").unwrap());
        assert_eq!(press.matches("update_asked::leave()").count(), 1);
    }

    #[test]
    fn what_is_held_back_is_not_counted_and_never_installed() {
        use crate::common::held::{self, Held, Thing};
        const APT: &str = "Listing...\nbash/stable 5.2 amd64 [upgradable from: 5.1]\ntrindle/stable 2 amd64 [upgradable from: 1]\n";
        const FLATPAKS: &str = "app/org.example.Notes/x86_64/stable\napp/org.example.Paint/x86_64/stable\nruntime/org.example.Platform/x86_64/25.08\n";
        let devices = ["abc".to_string(), "def".to_string()];
        let none = Held::default();
        assert_eq!(super::counted(APT, FLATPAKS, &devices, &[], &none), Waiting { system: 2, apps: 3, firmware: 2, ..Waiting::default() });
        let kept = Held::of(&[Thing::app("org.example.Notes"), Thing::part("org.example.Platform", "25.08"), Thing::firmware("abc"),
                              Thing::plugin("org.example.Paint")]);
        let w = super::counted(APT, FLATPAKS, &devices, &[], &kept);
        assert_eq!(w, Waiting { system: 2, apps: 1, firmware: 1, ..Waiting::default() }, "a plugin of an app's name holds no app");

        assert_eq!(held::names_for_update(FLATPAKS, &kept), Some(vec!["org.example.Paint/x86_64/stable".to_string()]));

        let kept = Held::of(&[Thing::firmware("abc"), Thing::firmware("def")]);
        let w = super::counted(APT, FLATPAKS, &devices, &[], &kept);
        assert_eq!((w.firmware, w.worth_a_look(), w.words()), (0, false, "Install 5 updates".to_string()));

        let kept = Held::of(&[Thing::app("org.example.Notes"), Thing::app("org.example.Paint"), Thing::part("org.example.Platform", "25.08"),
                              Thing::firmware("abc"), Thing::firmware("def")]);
        let w = super::counted("Listing...\n", FLATPAKS, &devices, &[], &kept);
        assert_eq!(w.total(), 0);
        assert_eq!(named(w, None), "Up to date");
        assert_eq!(held::names_for_update(FLATPAKS, &kept), None);
    }

    #[test]
    fn firmware_counts_with_the_rest() {
        assert_eq!(Waiting { system: 3, apps: 2, firmware: 1, plugins: 2, look: 1 }.total(), 9);
        assert_eq!(Waiting::default().total(), 0);
    }

    #[test]
    fn only_firmware_sends_the_button_to_apps() {
        let w = |system, apps, firmware| Waiting { system, apps, firmware, ..Waiting::default() };
        assert!(!w(4, 1, 0).worth_a_look());
        assert!(w(0, 0, 1).worth_a_look());
        assert!(w(4, 1, 1).worth_a_look());
        assert!(!Waiting { plugins: 2, look: 1, ..Waiting::default() }.worth_a_look(), "plugins never send it to the firmware");
    }

    #[test]
    fn the_name_says_what_pressing_will_do() {
        let w = |system, apps, firmware| Waiting { system, apps, firmware, ..Waiting::default() };
        assert_eq!(w(1, 0, 0).words(), "Install 1 update");
        assert_eq!(w(3, 2, 0).words(), "Install 5 updates");
        assert_eq!(w(0, 0, 1).words(), "See the firmware update");
        assert_eq!(w(0, 0, 2).words(), "See 2 firmware updates");
        assert_eq!(w(3, 0, 1).words(), "See 4 updates, firmware among them");
    }

    use crate::plugins::catalogue::{Entry, Offer};
    use crate::plugins::Plugin;

    fn mine(id: &str, version: &str, asks: &[&str]) -> Plugin {
        Plugin { id: id.into(), name: id.rsplit('.').next().unwrap_or(id).into(), version: version.into(), summary: String::new(),
                 asks: asks.iter().map(|a| a.to_string()).collect(), ring: None, fields: Vec::new(), background: None, listen: Vec::new(),
                 shortcuts: Vec::new() }
    }

    fn offer(id: &str, version: &str, asks: &[&str]) -> Offer {
        Offer { entry: Entry { id: id.into(), name: id.into(), version: version.into(), summary: String::new(),
                               permissions: asks.iter().map(|a| a.to_string()).collect(), icon: None, icon_sha256: None, archive: "a.tar.zst".into(), sha256: "0".repeat(64) },
                icon: None }
    }

    #[test]
    fn a_plugins_update_is_counted_with_the_rest() {
        use crate::common::held::{Held, Thing};
        let installed = [mine("org.example.One", "1.0", &["network"]), mine("org.example.Two", "2.0", &[]), mine("org.example.Three", "3.0", &["network"]),
                         mine("org.example.Four", "4.0", &[])];
        let offers = [offer("org.example.One", "1.1", &["network"]), offer("org.example.Two", "2.0", &[]),
                      offer("org.example.Three", "3.1", &["network", "files"]), offer("org.example.Five", "5.0", &[])];
        let found = super::pending(&installed, &offers);
        assert_eq!(found, [Pending { id: "org.example.One".into(), look: false }, Pending { id: "org.example.Three".into(), look: true }],
                   "a newer version of an installed plugin, and whether it asks for more");
        let none = Held::default();

        let w = super::counted("Listing...\n", "", &[], &found[..1], &none);
        assert_eq!((w.total(), w.plugins, w.look), (1, 1, 0));
        assert_eq!(stand(w, None), Stand::Waiting);
        assert_eq!(w.words(), "Install 1 update");

        const APT: &str = "Listing...\nbash/stable 5.2 amd64 [upgradable from: 5.1]\n";
        let w = super::counted(APT, "app/org.example.Notes/x86_64/stable\n", &[], &found, &none);
        assert_eq!(w, Waiting { system: 1, apps: 1, firmware: 0, plugins: 1, look: 1 });
        assert_eq!(w.total(), 4);

        let kept = Held::of(&[Thing::plugin("org.example.One"), Thing::plugin("org.example.Three")]);
        let w = super::counted(APT, "", &[], &found, &kept);
        assert_eq!((w.total(), w.plugins, w.look), (1, 0, 0));

        let w = super::counted(APT, "", &["abc".to_string()], &found, &none);
        assert_eq!(w.words(), "See 4 updates, firmware among them");
    }

    #[test]
    fn a_plugin_that_asks_for_more_is_left_waiting_and_said_to_need_a_look() {
        use crate::common::held::Held;
        let look = |plugins, look| Waiting { plugins, look, ..Waiting::default() };

        assert!(look(0, 1).only_a_look() && look(0, 2).only_a_look());
        assert_eq!(look(0, 1).words(), "See the plugin update, which asks for more than before");
        assert_eq!(look(0, 2).words(), "See 2 plugin updates, which ask for more than before");
        assert_eq!(stand(look(0, 1), None), Stand::Waiting, "it still waits, so the button says so");

        assert!(!look(1, 1).only_a_look());
        assert_eq!(look(1, 1).words(), "Install 1 update; 1 plugin update needs a look in Apps");
        assert_eq!(Waiting { system: 2, apps: 1, firmware: 0, plugins: 0, look: 2 }.words(), "Install 3 updates; 2 plugin updates need a look in Apps");
        assert!(!Waiting::default().only_a_look());

        assert_eq!(Waiting::default().spoken(), "Everything is up to date");
        assert_eq!(look(2, 0).spoken(), "2 waiting: the updates button beside Apps puts them in");
        assert_eq!(look(0, 1).spoken(), "1 waiting: 1 plugin update needs a look in Apps");
        assert_eq!(look(1, 2).spoken(), "3 waiting: the updates button beside Apps puts 1 in, and 2 plugin updates need a look in Apps");

        let installed = [mine("org.example.One", "1.0", &["network"]), mine("org.example.Three", "3.0", &["network"])];
        let offers = [offer("org.example.One", "1.1", &["network"]), offer("org.example.Three", "3.1", &["network", "microphone"])];
        let looks: Vec<(String, bool)> = super::pending(&installed, &offers).into_iter().map(|p| (p.id, p.look)).collect();
        assert_eq!(looks, [("org.example.One".to_string(), false), ("org.example.Three".to_string(), true)]);
        let _ = Held::default();
    }

    #[test]
    fn looking_for_plugins_asks_no_more_than_the_index() {
        let source = crate::common::source::code(include_str!("updates.rs")).split_once("#[cfg(test)]").unwrap().0;
        let look = source.split_once("fn plugins_pending(").unwrap().1.split_once("\n}\n").unwrap().0;
        assert!(look.find("installed.is_empty()").unwrap() < look.find("catalogue::offers_to_count(catalogue::INDEX_FRESH)").unwrap());
        assert!(!source.contains("catalogue::gather("), "the ring never fetches the pictures");
        let waiting = source.split_once("pub fn waiting(").unwrap().1.split_once("\n}\n").unwrap().0;
        assert!(waiting.contains("plugins_pending(net)"), "found at the same look as the rest, from the kept index while it is fresh");
    }

    #[test]
    fn with_looking_by_itself_off_a_look_asks_the_network_nothing() {
        use super::{flatpak_args, unchecked};
        assert!(!flatpak_args(true).contains(&"--cached"));
        assert!(flatpak_args(false).contains(&"--cached"), "Flatpak answers from what it last fetched");
        let source = crate::common::source::code(include_str!("updates.rs")).split_once("#[cfg(test)]").unwrap().0;
        let plugins = source.split_once("fn plugins_pending(").unwrap().1.split_once("\n}\n").unwrap().0;
        assert!(plugins.contains("if net { catalogue::offers_to_count(") && plugins.contains("else { catalogue::offers_kept() }"));

        let look = source.split_once("pub fn look(").unwrap().1.split_once("\n    }\n").unwrap().0;
        assert!(look.contains("may_ask_network(false, !self.off)"));
        assert_eq!(source.matches("waiting(&self.ctx,").count(), 1, "one look, and the timers, the stale look and the held look all go through it");
        let new = source.split_once("pub fn new(").unwrap().1.split_once("\n    }\n").unwrap().0;
        assert!(new.contains("look_later"), "the timers only call look, which decides");

        let none = Waiting::default();
        assert!(unchecked(none, None, true) && !unchecked(none, None, false));
        assert!(!unchecked(Waiting { system: 1, ..none }, None, true), "what is known to wait is still said");
        assert_eq!(crate::toolkit::t!("ring-updates-off"), "Checking is off");
    }
}
