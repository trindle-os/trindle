//! Tailscale.

use crate::common::run::{admin_error, admin_helper, capture, spawn};
use crate::common::tailscale::{self, Device, Status};
use crate::settings::panels::network::banded;
use crate::settings::ui::{confirm, group, headline, line, on, open_link, page, ran, Kit, Msg, Page, Raw, no_browser};
use crate::toolkit::t;
use crate::toolkit::widget::{main_action, switch, Node};

const SWITCH: &str = "tailscale:switch";
const SIGN_IN: &str = "tailscale:sign-in";
const SIGN_OUT: &str = "tailscale:sign-out";
const LOOK_EVERY: u32 = 5;
const LOOK_SIGNING: u32 = 1;

#[derive(Default)]
pub struct Tailscale {
    on: bool,
    busy: bool,
    asked: bool,
    status: Option<Status>,
    looks: u32,
    signing: bool,
    opened: String,
}

fn day(iso: &str) -> String {
    let mut parts = iso.splitn(3, '-');
    let (Some(y), Some(m), Some(d)) = (parts.next(), parts.next(), parts.next()) else { return iso.to_string() };
    match (y.parse::<i64>(), m.parse::<u32>(), d.parse::<u32>()) {
        (Ok(y), Ok(m @ 1..=12), Ok(d)) => {
            let at = crate::common::clock::moment_of_date(y, m, d);
            trindle_toolkit::lang::format::date(&at, trindle_toolkit::lang::format::Pattern::DayShortYear)
        }
        _ => iso.to_string(),
    }
}

fn system(os: &str) -> String {
    match os {
        "linux" => "Linux".into(),
        "windows" => "Windows".into(),
        "macOS" | "darwin" => "macOS".into(),
        "iOS" | "ios" => t!("settings-tailscale-system-ios").into(),
        "android" => "Android".into(),
        "freebsd" => "FreeBSD".into(),
        other => other.into(),
    }
}

fn about_device(d: &Device) -> String {
    let when = if d.online {
        t!("settings-tailscale-online").to_string()
    } else if d.seen.is_empty() {
        t!("settings-tailscale-not-seen").to_string()
    } else {
        t!("settings-tailscale-last-online", day = day(&d.seen))
    };
    let os = system(&d.system);
    [d.address.as_str(), os.as_str(), when.as_str()].iter().filter(|w| !w.is_empty())
        .copied().collect::<Vec<_>>().join(", ")
}

struct Cards {
    rows: Vec<Node<Msg>>,
    about: &'static str,
    list: Vec<Node<Msg>>,
    waits: Option<&'static str>,
}

impl Tailscale {
    fn look(&mut self, kit: &mut Kit) {
        self.on = tailscale::on();
        if !self.on {
            self.asked = true;
            self.status = None;
            return;
        }
        let back = kit.then::<Tailscale, (bool, String, String)>(|me, k, (_, out, _)| {
            me.asked = true;
            me.status = tailscale::parse(&out);
            let Some(s) = &me.status else { return };
            if s.connected() {
                me.signing = false;
            } else if me.signing && !s.sign_in.is_empty() && s.sign_in != me.opened {
                me.opened = s.sign_in.clone();
                let opened = open_link(&s.sign_in);
                k.started(SIGN_IN, opened, no_browser());
            }
        });
        capture(&["tailscale", "status", "--json"], None, move |ok, out, err| back((ok, out, err)));
    }

    fn set(&mut self, kit: &mut Kit, on: bool) {
        self.on = on;
        self.busy = true;
        let words = if on && !tailscale::installed() {
            t!("settings-tailscale-waiting-install")
        } else {
            t!("settings-tailscale-waiting")
        };
        kit.say(SWITCH, words, true, true);
        let back = kit.then::<Tailscale, (bool, String, String)>(move |me, k, (ok, _, err)| {
            me.busy = false;
            let err = admin_error(&err);
            k.done(SWITCH, if ok { None } else { Some(&err) }, if on { t!("settings-tailscale-done-on") } else { t!("settings-tailscale-done-off") });
            me.look(k);
        });
        let helper = admin_helper();
        capture(&["pkexec", &helper, if on { "tailscale-on" } else { "tailscale-off" }], None,
                move |ok, out, err| back((ok, out, err)));
    }

    fn sign_in(&mut self, kit: &mut Kit) {
        self.signing = true;
        self.opened.clear();
        let argv = tailscale::up(&[]);
        if spawn(&argv.iter().map(String::as_str).collect::<Vec<_>>()) {
            kit.say(SIGN_IN, t!("settings-tailscale-opening"), true, false);
        } else {
            self.signing = false;
            kit.done(SIGN_IN, Some(t!("settings-tailscale-not-started")), "");
        }
    }

    fn sign_out(&mut self, kit: &mut Kit) {
        let back = kit.settles::<Tailscale, (), Raw>(SIGN_OUT, t!("settings-tailscale-sign-out-failed"), |me, k, _| me.look(k));
        capture(&["tailscale", "logout"], None, move |ok, _, err| back(ran(ok, &err)));
    }

    fn top(&self, kit: &Kit) -> Node<Msg> {
        headline(kit, self.top_words(tailscale::installed()))
    }

    fn top_words(&self, installed: bool) -> String {
        match (&self.status, self.on) {
            _ if !self.asked => t!("settings-tailscale-top-asking").to_string(),
            (_, false) if !installed => t!("settings-tailscale-top-not-installed").to_string(),
            (_, false) => t!("settings-tailscale-top-off").to_string(),
            (None, true) => t!("settings-tailscale-top-silent").to_string(),
            (Some(s), true) if s.connected() => {
                let network = if s.network.is_empty() { t!("settings-tailscale-network-fallback") } else { s.network.as_str() };
                t!("settings-tailscale-top-connected", network = network, name = &s.me.name)
            }
            (Some(s), true) if s.needs_sign_in() => t!("settings-tailscale-top-sign-in").to_string(),
            (Some(s), true) => t!("settings-tailscale-top-state", state = s.state.to_lowercase()),
        }
    }

    fn connected(&self, kit: &Kit, s: &Status, cards: &mut Cards) {
        let address = if s.me.address.is_empty() {
            t!("settings-tailscale-no-address").to_string()
        } else {
            t!("settings-tailscale-address", address = &s.me.address)
        };
        cards.rows.push(line(if s.me.name.is_empty() { t!("settings-tailscale-this-computer").to_string() } else { s.me.name.clone() })
            .icon("computer-symbolic").sub(address).node(kit));
        cards.waits = None;
        if !s.devices.is_empty() {
            cards.list = banded(s.devices.iter().map(|d| line(d.name.clone()).icon(device_icon(&d.system)).sub(about_device(d)).node(kit)).collect());
        }
    }

    fn not_connected(&self, kit: &Kit, s: &Status, cards: &mut Cards) {
        let (heading, sub, tip) = if s.needs_sign_in() {
            (t!("settings-tailscale-sign-in"), t!("settings-tailscale-sign-in-sub"), t!("settings-tailscale-sign-in"))
        } else {
            (t!("settings-tailscale-connect"), t!("settings-tailscale-connect-sub"), t!("settings-tailscale-connect"))
        };
        let press = main_action("network-transmit-receive-symbolic", tip)
            .on_press(on::<Tailscale>(|me, k| me.sign_in(k))).off_because(crate::common::ui::wait_while(self.signing && s.sign_in.is_empty()));
        cards.rows.push(line(heading).sub(sub).key(SIGN_IN).control(press).node(kit));
        cards.about = t!("settings-tailscale-about-sign-in");
    }
}

impl Page for Tailscale {
    fn id(&self) -> &'static str {
        "tailscale"
    }
    fn title(&self) -> &'static str {
        t!("settings-tailscale-title")
    }
    fn icon(&self) -> &'static str {
        "network-vpn-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-tailscale-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-tailscale-help")
    }
    fn shown(&mut self, kit: &mut Kit) {
        self.looks = 0;
        self.look(kit);
    }
    fn second(&mut self, kit: &mut Kit) -> bool {
        self.looks += 1;
        if self.busy || self.looks < if self.signing { LOOK_SIGNING } else { LOOK_EVERY } {
            return false;
        }
        self.looks = 0;
        self.look(kit);
        true
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let mut items = vec![self.top(kit)];
        let first = !tailscale::installed();
        let sub = if first { t!("settings-tailscale-switch-sub-first") } else { t!("settings-tailscale-switch-sub") };
        let toggle = switch(self.on, t!("settings-tailscale-switch"), |v| on::<Tailscale>(move |me, k| me.set(k, v))).off_because(crate::common::ui::wait_while(self.busy));
        let mut cards = Cards {
            rows: vec![line(t!("settings-tailscale-switch")).sub(sub).key(SWITCH).control(toggle).node(kit)],
            about: t!("settings-tailscale-about-plain"),
            list: vec![line(t!("settings-tailscale-no-devices")).sub(t!("settings-tailscale-no-devices-sub")).node(kit)],
            waits: Some(match (self.on, &self.status) {
                (false, _) => t!("settings-tailscale-waits-off"),
                (true, None) => t!("settings-tailscale-waits-looking"),
                (true, Some(s)) if s.needs_sign_in() => t!("settings-tailscale-waits-sign-in"),
                (true, Some(_)) => t!("settings-tailscale-waits-connect"),
            }),
        };
        if let (true, Some(s)) = (self.on, &self.status) {
            if s.connected() {
                self.connected(kit, s, &mut cards);
            } else {
                self.not_connected(kit, s, &mut cards);
            }
        }
        cards.rows.push(line(t!("settings-tailscale-firewall")).sub(t!("settings-tailscale-firewall-sub")).node(kit));
        if self.on && self.status.as_ref().is_some_and(Status::connected) {
            let out = confirm::<Tailscale>(kit, SIGN_OUT, "system-log-out-symbolic", t!("settings-tailscale-sign-out"), |me, k| me.sign_out(k));
            cards.rows.push(line(t!("settings-tailscale-sign-out")).sub(t!("settings-tailscale-sign-out-sub")).key(SIGN_OUT).control(out).node(kit));
            cards.about = t!("settings-tailscale-about-connected");
        }
        items.push(group(t!("settings-tailscale-card")).icon("network-vpn-symbolic").about(cards.about).rows(cards.rows).node(kit));
        items.push(group(t!("settings-tailscale-devices")).icon("network-workgroup-symbolic")
            .about(t!("settings-tailscale-devices-about"))
            .span(5, 8)
            .waits_for(cards.waits)
            .rows(cards.list)
            .node(kit));
        page(kit, self.title(), self.help(), items)
    }
}

fn device_icon(os: &str) -> &'static str {
    match os {
        "iOS" | "ios" | "android" => "phone-symbolic",
        _ => "computer-symbolic",
    }
}

#[cfg(test)]
impl Tailscale {
    pub(crate) fn flipped() -> Vec<Tailscale> {
        let off = Tailscale { on: false, ..Tailscale::made_up() };
        let mut signing = Tailscale::made_up();
        if let Some(s) = signing.status.as_mut() {
            s.state = "NeedsLogin".into();
        }
        let looking = Tailscale { status: None, ..Tailscale::made_up() };
        vec![Tailscale::made_up(), off, signing, looking]
    }

    pub(crate) fn made_up() -> Tailscale {
        let device = |name: &str, address: &str, system: &str, online: bool| Device {
            name: name.into(), address: address.into(), system: system.into(), online, seen: "2026-10-01".into(),
        };
        let status = Status {
            state: "Running".into(), sign_in: String::new(), network: "example.ts.net".into(),
            me: device("johns-laptop", "100.64.0.1", "linux", true),
            devices: vec![device("desk", "100.64.0.2", "linux", true), device("phone", "100.64.0.3", "android", true),
                          device("old-laptop", "100.64.0.4", "windows", false)],
        };
        Tailscale { on: true, asked: true, status: Some(status), ..Tailscale::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::{about_device as about, day};
    use crate::common::tailscale::Device;

    #[test]
    fn the_top_line_follows_not_installed_off_signed_out_and_connected() {
        let [connected, off, signing, looking] = <[super::Tailscale; 4]>::try_from(super::Tailscale::flipped()).ok().unwrap();
        assert_eq!(off.top_words(false), "Tailscale is not installed. Switch it on below to install it.");
        assert_eq!(off.top_words(true), "Tailscale is off, so it uses no memory.");
        assert_eq!(signing.top_words(true), "Tailscale is on. Sign in below to join your private network.");
        assert_eq!(looking.top_words(true), "Tailscale is not answering. It may still be starting.");
        assert_eq!(connected.top_words(true), "Connected to example.ts.net as johns-laptop.");
        assert_eq!(super::Tailscale::default().top_words(true), "Asking Tailscale\u{2026}");
    }

    #[test]
    fn dates_and_lines() {
        assert_eq!(day("2026-10-01"), "1 Oct 2026");
        assert_eq!(day("odd"), "odd");
        let d = Device { name: "phone".into(), address: "100.64.0.2".into(), system: "android".into(),
                         online: false, seen: "2026-10-01".into() };
        assert_eq!(about(&d), "100.64.0.2, Android, last online 1 Oct 2026");
        assert_eq!(about(&Device { online: true, ..Default::default() }), "online now");
    }

    #[test]
    fn a_device_line_follows_the_language_and_falls_back_word_by_word() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let french = "settings-tailscale-online = en ligne\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            assert_eq!(about(&Device { online: true, ..Default::default() }), "en ligne");
            let seen = Device { seen: String::new(), ..Default::default() };
            assert_eq!(about(&seen), "not seen yet", "a word French lacks is English");
        });
    }

    #[test]
    fn a_last_seen_day_is_written_the_languages_way() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let made_up = "kit-date-month-short-oct = Blu\nkit-date-pattern-day-short-year = %Y %b %-d\n";
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!(day("2026-10-01"), "2026 Blu 1");
            assert_eq!(day("2026-12-31"), "2026 Dec 31");
            assert_eq!(day("odd"), "odd");
        });
    }
}
