//! trindle-nightlight: warms the screens in the evening and cools them in the morning.

use std::cell::RefCell;
use std::f64::consts::PI;
use std::rc::Rc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use wayland_client::globals::{registry_queue_init, GlobalList, GlobalListContents};
use wayland_client::protocol::{wl_output, wl_registry};
use wayland_client::{Connection, Dispatch, EventQueue, Proxy, QueueHandle};
use crate::common::wire::night::{
    trindle_night_manager_v1::TrindleNightManagerV1,
    trindle_night_v1::{self, TrindleNightV1},
};

use crate::common::clock::Local;
use crate::common::files::setting;
use crate::common::run::spawn;
use crate::toolkit::event::{Flow, Loop};

const DAYTIME: u32 = 6500;

pub const WARMEST: u32 = 3000;
pub const LEAST_WARM: u32 = 5500;
const DEFAULT_WARMTH: u32 = 4000;
const DEFAULT_FROM: &str = "20:00";
const DEFAULT_TO: &str = "07:00";

pub const ON: &str = "nightlight";
pub const WHEN: &str = "nightlight_when";
pub const FROM: &str = "nightlight_from";
pub const TO: &str = "nightlight_to";
pub const WARMTH: &str = "nightlight_warmth";

pub const BY_SUN: &str = "sun";
pub const BY_TIMES: &str = "times";

pub fn evening_times() -> Vec<String> {
    half_hours(16 * 60, 22 * 60 + 30)
}

pub fn morning_times() -> Vec<String> {
    half_hours(4 * 60, 10 * 60)
}

fn half_hours(first: u32, last: u32) -> Vec<String> {
    (first..=last).step_by(30).map(|m| format!("{:02}:{:02}", m / 60, m % 60)).collect()
}

pub fn from_time(saved: &str) -> String {
    listed(saved, evening_times(), DEFAULT_FROM)
}

pub fn to_time(saved: &str) -> String {
    listed(saved, morning_times(), DEFAULT_TO)
}

fn listed(saved: &str, list: Vec<String>, default: &str) -> String {
    if list.iter().any(|t| t == saved.trim()) { saved.trim().to_string() } else { default.to_string() }
}

pub fn warmth(saved: &str) -> u32 {
    saved.trim().parse::<u32>().unwrap_or(DEFAULT_WARMTH).clamp(WARMEST, LEAST_WARM)
}

pub fn zone() -> String {
    std::fs::read_link("/etc/localtime").ok()
        .and_then(|p| p.to_str().and_then(|s| s.split_once("zoneinfo/")).map(|(_, z)| z.to_string()))
        .or_else(|| std::fs::read_to_string("/etc/timezone").ok().map(|s| s.trim().to_string()))
        .unwrap_or_default()
}

pub fn place() -> Option<(f64, f64)> {
    let read = |name: &str| std::fs::read_to_string(format!("/usr/share/zoneinfo/{name}")).unwrap_or_default();
    let tabs = read("zone1970.tab") + &read("zone.tab");
    place_for(&zone(), &tabs, &read("tzdata.zi"))
}

fn place_for(zone: &str, tabs: &str, links: &str) -> Option<(f64, f64)> {
    let find = |name: &str| {
        tabs.lines().filter(|l| !l.starts_with('#'))
            .find(|l| l.split('\t').nth(2) == Some(name))
            .and_then(|l| l.split('\t').nth(1))
            .and_then(coordinates)
    };
    find(zone).or_else(|| {

        let target = links.lines().find_map(|l| match l.split_whitespace().collect::<Vec<_>>()[..] {
            ["L", target, alias] if alias == zone => Some(target),
            _ => None,
        })?;
        find(target)
    })
}

fn coordinates(text: &str) -> Option<(f64, f64)> {
    let split = text.get(1..)?.find(['+', '-'])? + 1;
    let (lat, lon) = text.split_at(split);
    Some((degrees(lat, 2)?, degrees(lon, 3)?))
}

fn degrees(text: &str, whole: usize) -> Option<f64> {
    let (sign, digits) = text.split_at(1);
    let sign = match sign { "+" => 1.0, "-" => -1.0, _ => return None };
    if !digits.chars().all(|c| c.is_ascii_digit()) || ![whole + 2, whole + 4].contains(&digits.len()) {
        return None;
    }
    let part = |from: usize, to: usize| digits.get(from..to).and_then(|p| p.parse::<f64>().ok()).unwrap_or(0.0);
    Some(sign * (part(0, whole) + part(whole, whole + 2) / 60.0 + part(whole + 2, whole + 4) / 3600.0))
}

pub struct Choice {
    pub on: bool,
    pub by_sun: bool,
    pub from: String,
    pub to: String,
    pub warmth: String,
}

impl Choice {
    fn saved() -> Choice {
        Choice {
            on: setting(ON) == "on",
            by_sun: setting(WHEN) != BY_TIMES,
            from: setting(FROM),
            to: setting(TO),
            warmth: setting(WARMTH),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum When {

    Sun(f64, f64),

    Times(i64, i64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Plan {
    warm: u32,
    when: When,
}

fn plan(choice: &Choice, place: Option<(f64, f64)>) -> Option<Plan> {
    if !choice.on {
        return None;
    }
    let seconds = |t: String| t.split_once(':')
        .and_then(|(h, m)| Some(h.parse::<i64>().ok()? * 3600 + m.parse::<i64>().ok()? * 60)).unwrap_or(0);
    let when = match place.filter(|_| choice.by_sun) {
        Some((lat, lon)) => When::Sun(lat, lon),
        None => When::Times(seconds(to_time(&choice.to)), seconds(from_time(&choice.from))),
    };
    Some(Plan { warm: warmth(&choice.warmth), when })
}

const FADE: i64 = 10 * 60;

const STEP_KELVIN: i64 = 10;

const LOOK_AGAIN: i64 = 60;

const TWILIGHT: f64 = -6.0;
const DAYLIGHT: f64 = 3.0;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Stops {
    dawn: i64,
    sunrise: i64,
    sunset: i64,
    night: i64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Day {
    Normal(Stops),

    NoSunset,
    NoSunrise,
}

fn start_of_day(now: i64, offset: i64) -> i64 {
    now - (now - offset).rem_euclid(86_400)
}

fn today(plan: &Plan, now: i64, utc_offset: i64) -> Day {
    match plan.when {
        When::Times(morning, evening) => {
            let day = start_of_day(now, -utc_offset);
            Day::Normal(Stops { dawn: day + morning - FADE, sunrise: day + morning,
                                sunset: day + evening, night: day + evening + FADE })
        }

        When::Sun(lat, lon) => sun_day(start_of_day(now, (-lon * 240.0) as i64), lat),
    }
}

fn sun_day(day: i64, latitude: f64) -> Day {
    let (year, yday) = date(day);
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let orbit = 2.0 * PI / if leap { 366.0 } else { 365.0 } * yday as f64;
    let eqtime = 4.0 * (0.000075 + 0.001868 * orbit.cos() - 0.032077 * orbit.sin()
        - 0.014615 * (2.0 * orbit).cos() - 0.040849 * (2.0 * orbit).sin());
    let decl = 0.006918 - 0.399912 * orbit.cos() + 0.070257 * orbit.sin() - 0.006758 * (2.0 * orbit).cos()
        + 0.000907 * (2.0 * orbit).sin() - 0.002697 * (3.0 * orbit).cos() + 0.00148 * (3.0 * orbit).sin();
    let lat = latitude.to_radians();

    let hour_angle = |height: f64| {
        let zenith = (90.833 - height).to_radians();
        (zenith.cos() / (lat.cos() * decl.cos()) - lat.tan() * decl.tan()).acos().abs()
    };
    let (twilight, daylight) = (hour_angle(TWILIGHT), hour_angle(DAYLIGHT));
    if twilight.is_nan() || daylight.is_nan() {

        return if (lat >= 0.0) == (decl >= 0.0) { Day::NoSunset } else { Day::NoSunrise };
    }
    let at = |angle: f64| day + ((4.0 * PI + angle * 4.0 - eqtime) * 60.0).to_degrees() as i64;
    Day::Normal(Stops { dawn: at(-twilight), sunrise: at(-daylight), sunset: at(daylight), night: at(twilight) })
}

fn date(seconds: i64) -> (i64, i64) {
    let days = seconds.div_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let year = yoe + era * 400 + i64::from(mp >= 10);

    let y = year - 1;
    let (era, yoe) = (y.div_euclid(400), y.rem_euclid(400));
    let new_year = era * 146_097 + yoe * 365 + yoe / 4 - yoe / 100 + 306 - 719_468;
    (year, days - new_year)
}

fn position(day: &Day, now: i64) -> f64 {
    let s = match day {
        Day::NoSunset => return 1.0,
        Day::NoSunrise => return 0.0,
        Day::Normal(s) => s,
    };
    let along = |from: i64, to: i64| if to == from { 1.0 } else { ((now - from) as f64 / (to - from) as f64).clamp(0.0, 1.0) };
    if now < s.dawn {
        0.0
    } else if now < s.sunrise {
        along(s.dawn, s.sunrise)
    } else if now < s.sunset {
        1.0
    } else if now < s.night {
        1.0 - along(s.sunset, s.night)
    } else {
        0.0
    }
}

fn kelvin(plan: &Plan, day: &Day, now: i64) -> u32 {
    let warm = plan.warm as f64;
    (warm + (DAYTIME as f64 - warm) * position(day, now)) as u32
}

fn unwarmed(kelvin: u32) -> bool {
    kelvin >= DAYTIME
}

fn wait(plan: &Plan, day: &Day, now: i64) -> i64 {
    let Day::Normal(s) = day else { return LOOK_AGAIN };
    let step = |from: i64, to: i64| ((to - from) * STEP_KELVIN / (DAYTIME - plan.warm).max(1) as i64).max(1);
    let until = if now < s.dawn {
        s.dawn - now
    } else if now < s.sunrise {
        step(s.dawn, s.sunrise)
    } else if now < s.sunset {
        s.sunset - now
    } else if now < s.night {
        step(s.sunset, s.night)
    } else {
        LOOK_AGAIN
    };
    until.clamp(1, LOOK_AGAIN)
}

fn whitepoint(temp: u32) -> (f64, f64, f64) {
    if temp == DAYTIME {
        return (1.0, 1.0, 1.0);
    }
    let t = temp as f64;
    let daylight = |t: f64| {
        let x = if t <= 7000.0 {
            0.244063 + 0.09911e3 / t + 2.9678e6 / t.powi(2) - 4.6070e9 / t.powi(3)
        } else {
            0.237040 + 0.24748e3 / t + 1.9018e6 / t.powi(2) - 2.0064e9 / t.powi(3)
        };
        (x, -3.0 * x * x + 2.870 * x - 0.275)
    };
    let filament = |t: f64| {
        if t <= 4000.0 {
            let x = -0.2661239e9 / t.powi(3) - 0.2343589e6 / t.powi(2) + 0.8776956e3 / t + 0.179910;
            let y = if t <= 2222.0 {
                -1.1064814 * x.powi(3) - 1.34811020 * x * x + 2.18555832 * x - 0.20219683
            } else {
                -0.9549476 * x.powi(3) - 1.37418593 * x * x + 2.09137015 * x - 0.16748867
            };
            (x, y)
        } else {
            let x = -3.0258469e9 / t.powi(3) + 2.1070379e6 / t.powi(2) + 0.2226347e3 / t + 0.240390;
            (x, 3.0817580 * x.powi(3) - 5.87338670 * x * x + 3.75112997 * x - 0.37001483)
        }
    };
    let (x, y) = if t >= 25_000.0 {
        daylight(25_000.0)
    } else if t >= 4000.0 {
        daylight(t)
    } else if t >= 2500.0 {
        let ((x1, y1), (x2, y2)) = (daylight(t), filament(t));
        let blend = ((PI * (4000.0 - t) / 1500.0).cos() + 1.0) / 2.0;
        (x1 * blend + x2 * (1.0 - blend), y1 * blend + y2 * (1.0 - blend))
    } else {
        filament(t.max(1667.0))
    };
    let z = 1.0 - x - y;

    let c = |v: f64| v.clamp(0.0, 1.0).powf(1.0 / 2.2);
    let (r, g, b) = (c(3.2404542 * x - 1.5371385 * y - 0.4985314 * z),
                     c(-0.9692660 * x + 1.8760108 * y + 0.0415560 * z),
                     c(0.0556434 * x - 0.2040259 * y + 1.0572252 * z));
    let most = r.max(g).max(b);
    (r / most, g / most, b / most)
}

fn shares(temp: u32) -> [u16; 3] {
    let (r, g, b) = whitepoint(temp);
    [r, g, b].map(|w| (w * f64::from(u16::MAX)).round() as u16)
}

fn earlier() -> Vec<u32> {
    use std::os::unix::fs::MetadataExt;
    let me = std::process::id();
    let Some(mine) = started(me) else { return Vec::new() };
    // SAFETY: getuid only reads this process's user id and cannot fail.
    let uid = unsafe { libc::getuid() };
    let Ok(dir) = std::fs::read_dir("/proc") else { return Vec::new() };
    dir.flatten().filter_map(|e| {
        let pid = e.file_name().to_str()?.parse::<u32>().ok().filter(|p| *p != me)?;
        (e.metadata().ok()?.uid() == uid && started_as_us(pid) && started(pid)? < mine).then_some(pid)
    }).collect()
}

fn started(pid: u32) -> Option<(u64, u32)> {
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;

    let ticks = stat.rsplit_once(')')?.1.split_whitespace().nth(19)?.parse().ok()?;
    Some((ticks, pid))
}

fn started_as_us(pid: u32) -> bool {
    std::fs::read(format!("/proc/{pid}/cmdline")).ok()
        .and_then(|c| c.split(|b| *b == 0).next().map(|a| a.to_vec()))
        .is_some_and(|a| a.rsplit(|b| *b == b'/').next() == Some(b"trindle-nightlight".as_slice()))
}

fn stop_running() {
    let pids = earlier();
    for pid in &pids {
        // SAFETY: a signal to a process of this account just checked to be a

        unsafe { libc::kill(*pid as i32, libc::SIGTERM) };
    }
    for _ in 0..40 {
        if !pids.iter().any(|p| started_as_us(*p)) {
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

pub fn restart() {
    spawn(&["trindle-nightlight"]);
}

fn now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

fn utc_offset() -> i64 {
    Local::now().utc_offset()
}

struct Screen {

    name: u32,
    output: wl_output::WlOutput,
    control: Option<TrindleNightV1>,

    shown: Option<u32>,
}

struct State {
    manager: TrindleNightManagerV1,
    screens: Vec<Screen>,

    kelvin: u32,
}

impl State {
    fn add(&mut self, registry: &wl_registry::WlRegistry, name: u32, version: u32, qh: &QueueHandle<State>) {
        let output: wl_output::WlOutput = registry.bind(name, version.min(3), qh, ());
        let control = self.manager.tint(&output, qh, name);
        self.screens.push(Screen { name, output, control: Some(control), shown: None });
    }

    fn remove(&mut self, name: u32) {
        let Some(i) = self.screens.iter().position(|s| s.name == name) else { return };
        let screen = self.screens.remove(i);
        if let Some(c) = screen.control {
            c.destroy();
        }
        if screen.output.version() >= 3 {
            screen.output.release();
        }
    }

    fn show(&mut self) {
        let kelvin = self.kelvin;
        let [red, green, blue] = shares(kelvin).map(u32::from);
        for screen in &mut self.screens {
            let Some(control) = &screen.control else { continue };
            if screen.shown != Some(kelvin) {
                control.warm(red, green, blue);
                screen.shown = Some(kelvin);
            }
        }
    }
}

pub fn main(_args: Vec<String>) -> i32 {

    stop_running();
    let Some(plan) = plan(&Choice::saved(), place()) else { return 0 };

    let Ok(conn) = Connection::connect_to_env() else {
        eprintln!("trindle-nightlight: no desktop to warm");
        return 1;
    };
    let Ok((globals, mut queue)) = registry_queue_init::<State>(&conn) else { return 1 };
    let qh = queue.handle();
    let Some(state) = first_state(&plan, &globals, &mut queue, &qh) else { return 1 };

    let queue = Rc::new(RefCell::new(queue));
    let state = Rc::new(RefCell::new(state));
    let events = Loop::new();
    super::wayland_loop::dispatch_on(&events, &queue, &state, &conn);
    tick(plan, state.clone(), conn.clone(), events.clone());
    let ml = events.clone();
    events.on_signal(libc::SIGTERM, move || {
        ml.quit();
        Flow::Stop
    });

    let st = state.clone();
    crate::common::renewed::watch(&events, "trindle-nightlight", move || {
        st.try_borrow().is_ok_and(|s| unwarmed(s.kelvin)).then_some(crate::common::renewed::Renew::InPlace)
    });
    let _ = conn.flush();
    events.run();

    for screen in state.borrow_mut().screens.drain(..) {
        if let Some(c) = screen.control {
            c.destroy();
        }
    }
    let _ = conn.flush();
    0
}

fn first_state(plan: &Plan, globals: &GlobalList, queue: &mut EventQueue<State>, qh: &QueueHandle<State>) -> Option<State> {
    let Ok(manager) = globals.bind::<TrindleNightManagerV1, _, _>(qh, 1..=1, ()) else {
        eprintln!("trindle-nightlight: this desktop cannot change the screens' colours");
        return None;
    };
    let when = now();
    let mut state = State { manager, screens: Vec::new(), kelvin: kelvin(plan, &today(plan, when, utc_offset()), when) };
    for g in globals.contents().clone_list() {
        if g.interface == "wl_output" {
            state.add(globals.registry(), g.name, g.version, qh);
        }
    }

    let _ = queue.roundtrip(&mut state);
    Some(state)
}

fn tick(plan: Plan, state: Rc<RefCell<State>>, conn: Connection, events: Loop) {
    let when = now();
    let day = today(&plan, when, utc_offset());
    {
        let mut st = state.borrow_mut();
        st.kelvin = kelvin(&plan, &day, when);
        st.show();
    }
    let _ = conn.flush();
    let after = Duration::from_secs(wait(&plan, &day, when) as u64);
    let again = events.clone();
    events.after(after, move || tick(plan, state, conn, again));
}

impl Dispatch<TrindleNightV1, u32> for State {
    fn event(state: &mut Self, control: &TrindleNightV1, event: trindle_night_v1::Event, name: &u32,
             _: &Connection, _: &QueueHandle<Self>) {
        let Some(screen) = state.screens.iter_mut().find(|s| s.name == *name) else { return };

        let trindle_night_v1::Event::Refused { .. } = event;
        eprintln!("trindle-nightlight: a screen's colours cannot be changed");
        control.destroy();
        screen.control = None;
    }
}

impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(state: &mut Self, registry: &wl_registry::WlRegistry, event: wl_registry::Event, _: &GlobalListContents,
             _: &Connection, qh: &QueueHandle<Self>) {

        match event {
            wl_registry::Event::Global { name, interface, version } if interface == "wl_output" => {
                state.add(registry, name, version, qh);
            }
            wl_registry::Event::GlobalRemove { name } => state.remove(name),
            _ => {}
        }
    }
}

impl Dispatch<TrindleNightManagerV1, ()> for State {
    fn event(_: &mut Self, _: &TrindleNightManagerV1, _: <TrindleNightManagerV1 as Proxy>::Event, _: &(),
             _: &Connection, _: &QueueHandle<Self>) {}
}

impl Dispatch<wl_output::WlOutput, ()> for State {
    fn event(_: &mut Self, _: &wl_output::WlOutput, _: wl_output::Event, _: &(), _: &Connection, _: &QueueHandle<Self>) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABS: &str = "# comment\nGB,GG,IM,JE\t+513030-0000731\tEurope/London\nIN\t+2232+08822\tAsia/Kolkata\n\
                        AR\t-3436-05827\tAmerica/Argentina/Buenos_Aires\n";
    const LINKS: &str = "L Etc/GMT GMT\nL Asia/Kolkata Asia/Calcutta\n";

    fn choice(on: bool, by_sun: bool, from: &str, to: &str, warmth: &str) -> Choice {
        Choice { on, by_sun, from: from.into(), to: to.into(), warmth: warmth.into() }
    }

    #[test]
    fn finds_the_zone_and_its_old_names() {
        let (lat, lon) = place_for("Europe/London", TABS, LINKS).unwrap();
        assert!((lat - 51.508).abs() < 0.01 && (lon + 0.125).abs() < 0.01);
        let (lat, lon) = place_for("America/Argentina/Buenos_Aires", TABS, LINKS).unwrap();
        assert!((lat + 34.6).abs() < 0.01 && (lon + 58.45).abs() < 0.01);
        assert_eq!(place_for("Asia/Calcutta", TABS, LINKS), place_for("Asia/Kolkata", TABS, LINKS));
        assert!(place_for("Asia/Kolkata", TABS, LINKS).is_some());
        assert_eq!(place_for("Etc/UTC", TABS, LINKS), None);
        assert_eq!(coordinates("+51-001x"), None);
    }

    #[test]
    fn makes_the_plan() {
        assert_eq!(plan(&choice(false, true, "20:00", "07:00", "4000"), Some((51.5, -0.12))), None);
        let london = place_for("Europe/London", TABS, LINKS);
        assert_eq!(plan(&choice(true, true, "20:00", "07:00", "4000"), london).unwrap(),
                   Plan { warm: 4000, when: When::Sun(london.unwrap().0, london.unwrap().1) });
        assert_eq!(plan(&choice(true, false, "21:30", "06:00", "3500"), Some((51.5, -0.12))).unwrap(),
                   Plan { warm: 3500, when: When::Times(6 * 3600, 21 * 3600 + 1800) });

        assert_eq!(plan(&choice(true, true, "03:00", "", "100"), None).unwrap(),
                   Plan { warm: 3000, when: When::Times(7 * 3600, 20 * 3600) });
    }

    #[test]
    fn offers_half_hours() {
        let evening = evening_times();
        assert_eq!((evening.first().unwrap().as_str(), evening.last().unwrap().as_str()), ("16:00", "22:30"));
        assert_eq!(morning_times()[..3], ["04:00", "04:30", "05:00"]);
    }

    #[test]
    fn knows_the_date() {
        assert_eq!(date(0), (1970, 0));

        assert_eq!(date(1_709_251_200), (2024, 60));
        assert_eq!(date(1_767_139_200 + 3600), (2025, 364));
    }

    #[test]
    fn warms_between_the_set_times() {
        let plan = Plan { warm: 4000, when: When::Times(7 * 3600, 20 * 3600) };

        let midnight = 1_750_464_000 - 3600;
        let at = |h: i64, m: i64| {
            let now = midnight + h * 3600 + m * 60;
            kelvin(&plan, &today(&plan, now, 3600), now)
        };
        assert_eq!(at(3, 0), 4000);
        assert_eq!(at(6, 50), 4000);
        assert_eq!(at(6, 55), 5250);
        assert_eq!(at(7, 0), 6500);
        assert_eq!(at(12, 0), 6500);
        assert_eq!(at(20, 0), 6500);
        assert_eq!(at(20, 5), 5250);
        assert_eq!(at(20, 10), 4000);
        assert_eq!(at(23, 59), 4000);

        let now = midnight + 20 * 3600 + 60;
        assert_eq!(wait(&plan, &today(&plan, now, 3600), now), 600 * 10 / 2500);
        let now = midnight + 19 * 3600 + 59 * 60 + 30;
        assert_eq!(wait(&plan, &today(&plan, now, 3600), now), 30);
    }

    #[test]
    fn night_light_renews_by_day_at_every_warmth_and_never_while_the_screens_are_warm() {

        let midnight = 1_750_464_000 - 3600;
        let (mut by_day, mut warm) = (0, 0);
        for warmth in (WARMEST..=LEAST_WARM).step_by(50) {
            for when in [When::Times(7 * 3600, 20 * 3600), When::Sun(51.5, -0.13)] {
                let plan = Plan { warm: warmth, when };
                for minutes in (0..24 * 60).step_by(5) {
                    let now = midnight + minutes * 60;
                    let kelvin = kelvin(&plan, &today(&plan, now, 3600), now);
                    assert!((warmth..=DAYTIME).contains(&kelvin), "{kelvin} at {minutes} minutes, warmth {warmth}");
                    assert_eq!(unwarmed(kelvin), kelvin == DAYTIME);
                    let midday = (9 * 60..17 * 60).contains(&minutes);
                    let night = !(4 * 60..23 * 60).contains(&minutes);
                    assert!(!midday || unwarmed(kelvin), "at {minutes} minutes of the day, warmth {warmth}, the screens count as warm ({kelvin}), so night light would never renew");
                    assert!(!night || !unwarmed(kelvin), "at {minutes} minutes, in the night, the screens count as their daytime colour");
                    by_day += usize::from(unwarmed(kelvin));
                    warm += usize::from(!unwarmed(kelvin));
                }
            }
        }
        assert!(by_day > 0 && warm > 0);

        let noon = 1_750_507_200;
        let north = Plan { warm: 4000, when: When::Sun(78.2, 15.6) };
        assert!(unwarmed(kelvin(&north, &today(&north, noon, 3600), noon)));
        assert!(!unwarmed(kelvin(&north, &today(&north, noon - 182 * 86_400, 3600), noon - 182 * 86_400)));
    }

    #[test]
    fn follows_the_sun() {

        let plan = Plan { warm: 4000, when: When::Sun(51.5, -0.13) };
        let noon = 1_750_507_200;
        let Day::Normal(s) = today(&plan, noon, 3600) else { panic!("London has nights") };
        let bst = |t: i64| (t + 3600).rem_euclid(86_400) / 60;
        assert!((3 * 60 + 30..4 * 60 + 20).contains(&bst(s.dawn)), "dawn {}", bst(s.dawn));
        assert!((4 * 60 + 50..5 * 60 + 30).contains(&bst(s.sunrise)), "sunrise {}", bst(s.sunrise));
        assert!((20 * 60 + 40..21 * 60 + 20).contains(&bst(s.sunset)), "sunset {}", bst(s.sunset));
        assert!((21 * 60 + 50..22 * 60 + 40).contains(&bst(s.night)), "night {}", bst(s.night));
        assert_eq!(kelvin(&plan, &today(&plan, noon, 3600), noon), 6500);
        let midnight = noon + 12 * 3600 - 3600;
        assert_eq!(kelvin(&plan, &today(&plan, midnight, 3600), midnight), 4000);

        assert_eq!(today(&Plan { warm: 4000, when: When::Sun(78.2, 15.6) }, noon, 3600), Day::NoSunset);
        assert_eq!(today(&Plan { warm: 4000, when: When::Sun(78.2, 15.6) }, noon - 182 * 86_400, 3600), Day::NoSunrise);
    }

    #[test]
    fn colours_warm_the_right_way() {
        assert_eq!(whitepoint(6500), (1.0, 1.0, 1.0));
        for t in [5500, 4000, 3500, 3000] {
            let (r, g, b) = whitepoint(t);
            assert!(r == 1.0 && g < 1.0 && b < g, "{t}: {r} {g} {b}");
        }

        assert!(whitepoint(3000).2 < whitepoint(4000).2 && whitepoint(4000).2 < whitepoint(5500).2);

        for (t, g, b) in [(3000, 0.716621, 0.426973), (3500, 0.780462, 0.530576), (4000, 0.836038, 0.626376),
                          (5500, 0.950514, 0.874692)] {
            let (r, gg, bb) = whitepoint(t);
            assert!(r == 1.0 && (gg - g).abs() < 1e-5 && (bb - b).abs() < 1e-5, "{t}: {gg} {bb}");
        }
    }

    #[test]
    fn shares_are_the_white_point_in_sixteen_bits() {
        assert_eq!(shares(6500), [65535; 3]);
        let [r, g, b] = shares(3000);
        assert_eq!(r, 65535);

        assert!((i32::from(g) - (0.716621 * 65535.0) as i32).abs() <= 1, "{g}");
        assert!((i32::from(b) - (0.426973 * 65535.0) as i32).abs() <= 1, "{b}");

        assert!(shares(3000)[2] < shares(4000)[2] && shares(4000)[2] < shares(5500)[2]);
        assert!(shares(3000).iter().all(|&v| v > u16::MAX / 10));
    }
}
