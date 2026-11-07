//! Date and time.

use std::time::Duration;

use crate::common::clock::Local;
use crate::common::files::{set_setting, setting};
use crate::common::run::capture;
use crate::settings::ui::{dropdown, group, headline, line, no_more, on, page, Cause, Kit, Msg, Page, not_saved, SAVE_ICON};
use crate::toolkit::bus::{get_bool, get_str, Failed, Props};
use crate::toolkit::t;
use crate::toolkit::theme::Palette;
use crate::toolkit::tiles::{picture, sketch};
use crate::toolkit::widget::{button, entry, row, Length, Node};

const NAME: &str = "org.freedesktop.timedate1";
const PATH: &str = "/org/freedesktop/timedate1";
const IFACE: &str = "org.freedesktop.timedate1";
const ASK: Duration = Duration::from_secs(60);
fn clocks() -> [(&'static str, &'static str); 3] {
    [("%H:%M", t!("settings-datetime-24")), ("%I:%M %p", t!("settings-datetime-12")), ("%H:%M:%S", t!("settings-datetime-24s"))]
}

fn parts() -> [(&'static str, i32, i32); 5] {
    [(t!("settings-datetime-day"), 1, 31), (t!("settings-datetime-month"), 1, 12), (t!("settings-datetime-year"), 2000, 2100),
     (t!("settings-datetime-hour"), 0, 23), (t!("settings-datetime-minute"), 0, 59)]
}

struct Clock {
    automatic: bool,
    in_step: bool,
    zone: String,
    can_ntp: bool,
}

fn zone_words(zone: &str) -> (String, String) {
    let plain = |s: &str| s.replace('_', " ");
    match zone.rsplit_once('/') {
        Some((region, city)) => (plain(city), plain(region.split('/').next().unwrap_or(region))),
        None => (plain(zone), String::new()),
    }
}

const HOUR: i64 = 3600;

fn offset_words(secs: i64) -> String {
    if secs == 0 {
        return "GMT".into();
    }
    let (sign, secs) = if secs < 0 { ('-', -secs) } else { ('+', secs) };
    let (h, m) = (secs / HOUR, secs % HOUR / 60);
    if m == 0 { format!("GMT{sign}{h}") } else { format!("GMT{sign}{h}:{m:02}") }
}

fn offset_secs(z: &str) -> Option<i64> {
    let z = z.trim();
    let (sign, digits) = match z.as_bytes().first() {
        Some(b'+') => (1, &z[1..]),
        Some(b'-') => (-1, &z[1..]),
        _ => return None,
    };
    if digits.len() != 4 {
        return None;
    }
    let h: i64 = digits[..2].parse().ok()?;
    let m: i64 = digits[2..].parse().ok()?;
    Some(sign * (h * HOUR + m * 60))
}

fn written(unix: i64, offset: i64, pattern: &str) -> String {
    trindle_toolkit::lang::format::format(&crate::common::clock::moment_at(unix, offset), pattern)
}

fn to_unix(v: [i32; 5]) -> Option<i64> {
    // SAFETY: tm is plain data, so all zeroes is a valid value to fill in.
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    tm.tm_mday = v[0];
    tm.tm_mon = v[1] - 1;
    tm.tm_year = v[2] - 1900;
    tm.tm_hour = v[3];
    tm.tm_min = v[4];
    tm.tm_isdst = -1;
    // SAFETY: tm is a live local, filled in above.
    let t = unsafe { libc::mktime(&mut tm) };
    (t != -1 && tm.tm_mday == v[0] && tm.tm_mon == v[1] - 1).then_some(t as i64)
}

#[derive(Default)]
pub struct Datetime {
    state: Option<Result<Clock, String>>,
    ntp: bool,
    zones: Vec<String>,
    zone: String,
    offset: Option<i64>,
    unix: i64,
    format: String,
    hand: [String; 5],
}

fn no_time() -> &'static str {
    t!("settings-datetime-no-time")
}

fn time_refused(e: &Failed) -> String {
    if e.name.contains("AutomaticTimeSync") || e.text.to_lowercase().contains("automatic time") {
        t!("settings-datetime-keeps-right").to_string()
    } else {
        e.words(no_time())
    }
}

impl Datetime {
    fn refresh(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.system.as_ref() else {
            self.got(kit, Err(t!("settings-datetime-no-bus").to_string()));
            return;
        };
        let back = kit.then::<Datetime, Result<Props, String>>(|me, k, r| me.got(k, r));
        bus.call(NAME, PATH, IFACE).get_all(&kit.events, move |r| back(r.map_err(|e| e.words(t!("settings-datetime-not-answering")))));
    }

    fn got(&mut self, kit: &mut Kit, res: Result<Props, String>) {
        let props = match res {
            Ok(p) => p,
            Err(e) => {
                kit.say("timedate", &e, false, false);
                self.state = Some(Err(e));
                return;
            }
        };
        let clock = Clock {
            automatic: get_bool(&props, "NTP"),
            in_step: get_bool(&props, "NTPSynchronized"),
            zone: get_str(&props, "Timezone"),
            can_ntp: props.get("CanNTP").and_then(|v| bool::try_from(v).ok()).unwrap_or(true),
        };
        self.ntp = clock.automatic;
        if clock.zone != self.zone {
            self.zone = clock.zone.clone();
            self.ask_offset(kit);
        }
        if !clock.automatic {
            let now = Local::now();
            self.hand = [now.day(), now.month(), now.year(), now.hour(), now.minute()].map(|n| n.to_string());
        }
        self.state = Some(Ok(clock));
        if self.zones.is_empty() {
            self.list_zones(kit);
        }
    }

    fn list_zones(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.system.as_ref() else { return };
        let back = kit.then::<Datetime, Vec<String>>(|me, _k, zones| me.zones = zones);
        bus.call(NAME, PATH, IFACE).method("ListTimezones", &(), false, None, &kit.events, move |r| {
            let zones = r.ok().and_then(|m| {
                let body = m.body();
                body.deserialize::<Vec<String>>().ok().or_else(|| body.deserialize::<(Vec<String>,)>().ok().map(|a| a.0))
            });
            back(zones.unwrap_or_default());
        });
    }

    fn ask_offset(&self, kit: &mut Kit) {
        if self.zone.is_empty() {
            return;
        }
        let zone = self.zone.clone();
        let tz = format!("TZ=:{zone}");
        let back = kit.then::<Datetime, (bool, String)>(move |me, _k, (ok, out)| {
            if ok && me.zone == zone {
                me.offset = offset_secs(&out).or(me.offset);
            }
        });
        capture(&["env", &tz, "date", "+%z"], None, move |ok, out, _| back((ok, out)));
    }

    fn set_ntp(&mut self, kit: &mut Kit, v: bool) {
        let Some(bus) = kit.system_for("ntp") else { return };
        self.ntp = v;
        let could_not = if v { t!("settings-datetime-ntp-on-failed") } else { t!("settings-datetime-ntp-off-failed") };
        let back = kit.settles::<Datetime, zbus::Message, Failed>("ntp", could_not, |me, k, _| me.refresh(k));
        bus.call(NAME, PATH, IFACE).method("SetNTP", &(v, true), true, Some(ASK), &kit.events, back);
    }

    fn set_zone(&mut self, kit: &mut Kit, zone: String) {
        if zone == self.zone {
            return;
        }
        let Some(bus) = kit.system_for("zone") else { return };
        self.zone = zone.clone();
        let back = kit.settles::<Datetime, zbus::Message, Failed>("zone", t!("settings-datetime-zone-failed"), |me, k, done| match done {
            Some(_) => {
                crate::session::nightlight::restart();
                me.moved_zone(k);
            }
            None => me.refresh(k),
        });
        bus.call(NAME, PATH, IFACE).method("SetTimezone", &(zone.as_str(), true), true, Some(ASK), &kit.events, back);
    }

    fn moved_zone(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.system.as_ref() else { return };
        let back = kit.then::<Datetime, Props>(|me, k, props| {
            let zone = get_str(&props, "Timezone");
            if let Some(Ok(clock)) = me.state.as_mut() {
                clock.zone = zone.clone();
            }
            me.zone = zone;
            me.ask_offset(k);
        });
        bus.call(NAME, PATH, IFACE).get_all(&kit.events, move |r| {
            if let Ok(props) = r {
                back(props);
            }
        });
    }

    fn set_time(&mut self, kit: &mut Kit) {
        let mut v = [0i32; 5];
        for (i, (_, lo, hi)) in parts().iter().enumerate() {
            match self.hand[i].trim().parse::<i32>() {
                Ok(n) if (*lo..=*hi).contains(&n) => v[i] = n,
                _ => {
                    kit.say("hand", t!("settings-datetime-not-real"), false, false);
                    return;
                }
            }
        }
        let Some(when) = to_unix(v) else {
            kit.say("hand", t!("settings-datetime-not-real"), false, false);
            return;
        };
        let Some(bus) = kit.system_for("hand") else { return };
        let back = kit.settles::<Datetime, _, String>("hand", no_time(), no_more);
        bus.call(NAME, PATH, IFACE).method("SetTime", &(when * 1_000_000, false, true), true, Some(ASK), &kit.events,
                                           move |r| back(r.map_err(|e| time_refused(&e))));
    }

    fn offset(&self) -> i64 {
        self.offset.unwrap_or_else(|| Local::at(self.unix).utc_offset())
    }

    fn show(&self, pattern: &str) -> String {
        written(self.unix, self.offset(), pattern)
    }

    fn head(&self, kit: &Kit) -> Node<Msg> {
        let time = self.show(&self.format.replace(":%S", ""));
        let date = self.show(trindle_toolkit::lang::format::Pattern::DayMonth.text());
        let words: String = match self.state.as_ref() {
            None => t!("settings-datetime-it-is", time = &time, date = &date),
            Some(Err(_)) => t!("settings-datetime-service-down").into(),
            Some(Ok(c)) => match (c.automatic, c.in_step) {
                (true, true) => t!("settings-datetime-it-is-right", time = &time, date = &date),
                (true, false) => t!("settings-datetime-it-is-checks", time = &time, date = &date),
                (false, _) => t!("settings-datetime-it-is-hand", time = &time, date = &date),
            },
        };
        headline(kit, words)
    }

    fn face(&self) -> Node<Msg> {
        let secs = (self.unix + self.offset()).rem_euclid(86_400);
        let (h, m, s) = ((secs / HOUR) as f32, (secs % HOUR / 60) as f32, (secs % 60) as f32);
        let time = self.show(&self.format.replace(":%S", ""));
        let date = self.show(trindle_toolkit::lang::format::Pattern::DayMonth.text());
        let (city, _) = zone_words(&self.zone);
        let zone = if self.zone.is_empty() { offset_words(self.offset()) } else { t!("settings-datetime-city-offset", city = &city, offset = offset_words(self.offset())) };
        let tip = t!("settings-datetime-tip", time = &time, date = &date, zone = &zone);
        sketch(tip, (360.0, 200.0), crate::toolkit::theme::bento().picture_least, move |sk| {
            let page = sk.page();
            sk.rect((8.0, 8.0, 344.0, 184.0), 10.0, page);
            let (cx, cy, r) = (100.0, 100.0, 74.0);
            let (boxed, words, quiet, lit) = (sk.boxed(), sk.words(), sk.quiet(), sk.highlight());
            sk.circle((cx, cy), r, boxed);
            sk.ring((cx, cy), r, 6.0, Palette::Blue.colour());
            let at = |turn: f32, len: f32| {
                let a = turn * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
                (cx + len * a.cos(), cy + len * a.sin())
            };
            for i in 0..12 {
                let t = i as f32 / 12.0;
                let long = i % 3 == 0;
                sk.line(at(t, if long { 52.0 } else { 58.0 }), at(t, 64.0), if long { 4.0 } else { 2.5 }, quiet);
            }
            sk.line((cx, cy), at((h % 12.0 + m / 60.0) / 12.0, 36.0), 7.0, words);
            sk.line((cx, cy), at((m + s / 60.0) / 60.0, 54.0), 5.0, lit);
            sk.line((cx, cy), at(s / 60.0, 58.0), 2.0, words);
            sk.circle((cx, cy), 6.0, words);
            sk.text(&time, (264.0, 82.0), 34.0, words);
            sk.text(&date, (264.0, 116.0), 13.0, quiet);
            sk.text(&zone, (264.0, 138.0), 13.0, quiet);
        })
    }

    fn clock(&self, kit: &Kit) -> Node<Msg> {
        let ring = t!("settings-datetime-ring-about");
        let card = group(t!("settings-datetime-the-clock")).icon("preferences-system-time-symbolic");
        let c = match self.state.as_ref() {
            None => return card.about(ring).rows(vec![self.ring_line(kit)]).node(kit),
            Some(Err(_)) => {
                return card.about(ring).rows(vec![line(t!("settings-datetime-service-down-line")).key("timedate").node(kit),
                                                  self.ring_line(kit)]).node(kit);
            }
            Some(Ok(c)) => c,
        };
        let sub = if c.can_ntp { t!("settings-datetime-from-internet") } else { t!("settings-datetime-needs-timesyncd") };
        let auto = line(t!("settings-datetime-set-auto")).sub(sub).key("ntp")
            .control(crate::toolkit::widget::switch(self.ntp, t!("settings-datetime-set-auto"),
                                                    |v| on::<Datetime>(move |me, k| me.set_ntp(k, v))));
        let mut zone = line(t!("settings-datetime-time-zone")).key("zone");
        let (city, region) = zone_words(&c.zone);
        let offset = offset_words(self.offset());
        zone = zone.sub(if region.is_empty() { t!("settings-datetime-city-offset", city = &city, offset = &offset) } else { t!("settings-datetime-city-region-offset", city = &city, region = &region, offset = &offset) });
        if !self.zones.is_empty() {
            let zones = self.zones.clone();
            let at = zones.iter().position(|z| *z == self.zone);
            zone = zone.control(dropdown(kit, "zone", self.zones.clone(), at, t!("settings-datetime-time-zone"), move |i| {
                let z = zones[i].clone();
                on::<Datetime>(move |me, k| me.set_zone(k, z.clone()))
            }));
        }
        let mut rows = vec![auto.node(kit), zone.node(kit)];
        if !c.automatic {
            let mut parts: Vec<Node<Msg>> = parts().iter().enumerate().map(|(i, (name, _, _))| {
                entry(self.hand[i].clone(), *name, move |t| on::<Datetime>(move |me, _k| me.hand[i] = t.clone()))
                    .on_submit(on::<Datetime>(|me, k| me.set_time(k)))
                    .width(Length::Fixed(if i == 2 { 88.0 } else { 64.0 }))
            }).collect();
            parts.push(button(SAVE_ICON, t!("settings-datetime-set-date-time")).on_press(on::<Datetime>(|me, k| me.set_time(k))));
            rows.push(line(t!("settings-datetime-by-hand")).sub(t!("settings-datetime-by-hand-sub")).key("hand")
                .below(row(parts).gap(8.0)).node(kit));
        }
        let about = if c.can_ntp { t!("settings-datetime-about") } else { t!("settings-datetime-about-no-ntp") };
        rows.push(self.ring_line(kit));
        card.about(about).rows(rows).node(kit)
    }

    fn ring_line(&self, kit: &Kit) -> Node<Msg> {
        let options: Vec<String> = clocks().iter().map(|(fmt, what)| t!("settings-datetime-clock-option", time = self.show(fmt), what = *what)).collect();
        let at = clocks().iter().position(|(fmt, _)| *fmt == self.format);
        let pick = dropdown(kit, "clock_format", options, at, t!("settings-datetime-show-as"), |i| {
            on::<Datetime>(move |me, k| {
                let v = clocks()[i].0;
                if k.settle("clock_format", set_setting("clock_format", v), not_saved()).is_some() {
                    me.format = v.to_string();
                }
            })
        });
        line(t!("settings-datetime-rings-clock")).sub(t!("settings-datetime-rings-clock-sub")).key("clock_format").control(pick).node(kit)
    }
}

impl Page for Datetime {
    fn id(&self) -> &'static str {
        "datetime"
    }
    fn title(&self) -> &'static str {
        t!("settings-datetime-title")
    }
    fn icon(&self) -> &'static str {
        "preferences-system-time-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-datetime-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-datetime-help")
    }
    fn shown(&mut self, kit: &mut Kit) {
        self.unix = Local::now().unix;
        self.format = setting("clock_format");
        self.refresh(kit);
        self.ask_offset(kit);
    }
    fn second(&mut self, kit: &mut Kit) -> bool {
        self.unix = Local::now().unix;
        if self.unix % 60 == 0 {
            self.ask_offset(kit);
        }
        true
    }
    fn conf_changed(&mut self, _kit: &mut Kit) {
        self.format = setting("clock_format");
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        page(kit, self.title(), self.help(), vec![
            self.head(kit),
            picture(self.icon(), t!("settings-datetime-time-now"), self.face()),
            self.clock(kit),
        ])
    }
}

#[cfg(test)]
impl Datetime {
    pub(crate) fn made_up() -> Datetime {
        Datetime { state: Some(Ok(Clock { automatic: true, in_step: true, zone: "Europe/London".into(), can_ntp: true })),
                   ntp: true, zones: vec!["Europe/London".into(), "Europe/Paris".into(), "America/New_York".into()],
                   zone: "Europe/London".into(), offset: Some(3600), unix: 1_791_627_000, format: "%H:%M".into(),
                   hand: Default::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zones_read_as_a_city_and_a_part_of_the_world() {
        assert_eq!(zone_words("Europe/London"), ("London".to_string(), "Europe".to_string()));
        assert_eq!(zone_words("America/Argentina/Buenos_Aires"), ("Buenos Aires".to_string(), "America".to_string()));
        assert_eq!(zone_words("UTC"), ("UTC".to_string(), String::new()));
    }

    #[test]
    fn offsets_read_from_greenwich() {
        assert_eq!(offset_words(0), "GMT");
        assert_eq!(offset_words(3600), "GMT+1");
        assert_eq!(offset_words(-5 * 3600), "GMT-5");
        assert_eq!(offset_words(5 * 3600 + 1800), "GMT+5:30");
        assert_eq!(offset_secs("+0530"), Some(5 * 3600 + 1800));
        assert_eq!(offset_secs("-0100"), Some(-3600));
    }

    #[test]
    fn a_change_the_time_service_cannot_take_is_said_and_nothing_stays_changed() {
        use crate::settings::ui::stand::Stand;
        use crate::settings::ui::no_system_bus;
        let mut s = Stand::new(Box::new(Datetime::made_up()));
        let before = s.page::<Datetime, _>(|d, _| (d.ntp, d.zone.clone()));
        s.page::<Datetime, _>(|d, k| d.set_ntp(k, !before.0));
        assert_eq!(s.red("ntp").as_deref(), Some(no_system_bus()));
        s.page::<Datetime, _>(|d, k| d.set_zone(k, "Pacific/Auckland".into()));
        assert_eq!(s.red("zone").as_deref(), Some(no_system_bus()));
        assert_eq!(s.page::<Datetime, _>(|d, _| (d.ntp, d.zone.clone())), before, "neither looks changed");
        let automatic = Failed::new("org.freedesktop.timedate1.AutomaticTimeSyncEnabled", "Automatic time synchronization is enabled");
        assert_eq!(time_refused(&automatic), "The clock keeps itself right just now. Turn that off above, then set the time by hand.");
        assert_eq!(time_refused(&Failed::new("org.freedesktop.DBus.Error.Failed", "EINVAL")), "The time could not be set.");
    }

    #[test]
    fn the_clock_and_date_follow_the_language() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let unix = 1_790_924_589;
        assert_eq!(written(unix, 3600, "%A %-d %B, %I:%M %p"), "Friday 2 October, 08:03 AM");
        assert_eq!(written(unix + 43_200, 3600, "%I:%M %p"), "08:03 PM");
        let made_up = "kit-date-weekday-fri = Frodag\nkit-date-month-oct = Blumar\nkit-date-month-short-oct = Blu\n\
                       kit-date-pattern-day-month = %A, %B %-d\nkit-date-pattern-day-short-year = %Y %b %-d\n\
                       kit-date-pm = EVE\nkit-number-decimal = ,\n";
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!(written(unix, 3600, crate::toolkit::lang::format::Pattern::DayMonth.text()), "Frodag, Blumar 2");
            assert_eq!(written(unix + 43_200, 3600, "%I:%M %p"), "08:03 EVE");
        });
    }
}
