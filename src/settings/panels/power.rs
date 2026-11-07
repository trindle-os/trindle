//! Power.

use std::collections::HashMap;

use zbus::zvariant::{OwnedValue, Value};
use zbus::Message;

use crate::common::files::{set_setting, setting};
use crate::common::pin;
use crate::common::run::{admin_error, admin_helper, capture, spawn};
use crate::session::idle;
use crate::settings::ui::{confirm, confirm_by_hand, dropdown, group, headline, kept, line, no_more, on, page, Kit, Msg, Page, not_saved, SAVE_ICON};
use crate::toolkit::bus::{get_bool, get_f64, get_i64, get_str, get_u32, Failed, Listen, Props};
use crate::toolkit::theme::Palette;
use crate::toolkit::tiles::sketch;
use crate::toolkit::widget::{button, column, entry, note, row, switch, Length, Node, Secret, Tone};
use crate::toolkit::t;

const UPOWER: &str = "org.freedesktop.UPower";
const DISPLAY_DEVICE: &str = "/org/freedesktop/UPower/devices/DisplayDevice";
const PROFILES: &str = "org.freedesktop.UPower.PowerProfiles";
const PROFILES_PATH: &str = "/org/freedesktop/UPower/PowerProfiles";
const LOGIN: &str = "org.freedesktop.login1";
const LOGIN_PATH: &str = "/org/freedesktop/login1";
const LOGIN_IFACE: &str = "org.freedesktop.login1.Manager";

fn times() -> Vec<(&'static str, &'static str)> {
    vec![
        ("0", t!("settings-power-time-never")),
        ("1", t!("settings-power-time-1m")),
        ("2", t!("settings-power-time-2m")),
        ("5", t!("settings-power-time-5m")),
        ("10", t!("settings-power-time-10m")),
        ("15", t!("settings-power-time-15m")),
        ("30", t!("settings-power-time-30m")),
        ("45", t!("settings-power-time-45m")),
        ("60", t!("settings-power-time-1h")),
        ("90", t!("settings-power-time-90m")),
        ("120", t!("settings-power-time-2h")),
    ]
}

fn lock_times() -> Vec<(&'static str, &'static str)> {
    vec![
        ("0", t!("settings-power-time-only-sleep")),
        ("1", t!("settings-power-time-1m")),
        ("5", t!("settings-power-time-5m")),
        ("10", t!("settings-power-time-10m")),
        ("15", t!("settings-power-time-15m")),
        ("30", t!("settings-power-time-30m")),
        ("45", t!("settings-power-time-45m")),
        ("60", t!("settings-power-time-1h")),
        ("90", t!("settings-power-time-90m")),
        ("120", t!("settings-power-time-2h")),
    ]
}

fn session() -> [(&'static str, &'static str, &'static str); 3] {
    [
        (t!("settings-power-suspend"), "Suspend", "system-suspend-symbolic"),
        (t!("settings-power-restart"), "Reboot", "system-reboot-symbolic"),
        (t!("settings-power-shut-down"), "PowerOff", "system-shutdown-symbolic"),
    ]
}

fn lid_choices() -> [(&'static str, &'static str); 5] {
    [
        ("suspend", t!("settings-power-lid-sleep")),
        ("hibernate", t!("settings-power-lid-hibernate")),
        ("lock", t!("settings-power-lid-lock")),
        ("nothing", t!("settings-power-lid-nothing")),
        ("poweroff", t!("settings-power-lid-shut-down")),
    ]
}

const WARN_LEVELS: &[u32] = &[0, 5, 10, 15, 20, 25, 30];

fn timers() -> [(&'static str, &'static str, &'static str); 4] {
    [
        ("dim_after", t!("settings-power-timer-dim"), t!("settings-power-timer-dim-about")),
        ("screensaver_after", t!("settings-power-timer-clock"), t!("settings-power-timer-clock-about")),
        ("blank_after", t!("settings-power-timer-off"), t!("settings-power-timer-off-about")),
        ("suspend_after", t!("settings-power-timer-suspend"), t!("settings-power-timer-suspend-about")),
    ]
}

const KEYS: &[&str] = &[
    "dim_after", "dim_after_battery", "screensaver_after", "screensaver_after_battery", "blank_after",
    "blank_after_battery", "suspend_after", "suspend_after_battery", "lid_close", "lid_close_battery",
    "battery_warn", "lock", "lock_after", "lock_on_suspend", "pin_minutes",
];

const MODE: &str = "power:mode";
const LIMIT: &str = "power:limit";
const LOCK: &str = "power:lock";
const PIN: &str = "power:pin";
const AWAKE: &str = "power:awake";
const LOCK_NOW: &str = "power:lock-now";

#[derive(Clone, Copy)]
enum When {
    Battery,
    Charger,
}

impl When {
    fn caption(self) -> &'static str {
        match self {
            When::Battery => t!("settings-power-when-battery"),
            When::Charger => t!("settings-power-when-charger"),
        }
    }

    fn named(self, title: &str) -> String {
        match self {
            When::Battery => t!("settings-power-named-battery", title = title),
            When::Charger => t!("settings-power-named-charger", title = title),
        }
    }
}

fn profile_name(p: &str) -> String {
    match p {
        "power-saver" => t!("settings-power-profile-saver").into(),
        "balanced" => t!("settings-power-profile-balanced").into(),
        "performance" => t!("settings-power-profile-performance").into(),
        other => other.into(),
    }
}

fn state_text(state: u32) -> &'static str {
    match state {
        1 => t!("settings-power-state-charging"),
        2 | 6 => t!("settings-power-state-battery"),
        3 => t!("settings-power-state-empty"),
        4 => t!("settings-power-state-full"),
        5 => t!("settings-power-state-plugged"),
        _ => "",
    }
}

fn duration(seconds: i64) -> String {
    if seconds <= 0 || seconds > 48 * 60 * 60 {
        return String::new();
    }
    let (h, m) = (seconds / 60 / 60, seconds / 60 % 60);
    if h > 0 { t!("settings-power-hours-minutes", h = h, m = m) } else { t!("settings-power-minutes", m = m) }
}

#[derive(Clone)]
struct Charge {
    percent: f64,
    state: u32,
    left: String,
}

fn sentence(charge: Option<&Charge>, mode: &str) -> String {
    let Some(c) = charge else {
        return if mode.is_empty() {
            t!("settings-power-top-no-battery").into()
        } else {
            t!("settings-power-top-no-battery-mode", mode = mode)
        };
    };
    let pct = c.percent;
    match c.state {
        1 if c.left.is_empty() => t!("settings-power-top-charging", percent = pct),
        1 => t!("settings-power-top-charging-left", percent = pct, left = &c.left),
        4 => t!("settings-power-top-full").into(),
        5 => t!("settings-power-top-plugged", percent = pct),
        _ if c.left.is_empty() => t!("settings-power-top-battery", percent = pct),
        _ => t!("settings-power-top-battery-left", percent = pct, left = &c.left),
    }
}

fn battery_tone(pct: f64, state: u32) -> Tone {
    match state {
        1 | 4 => Tone::Plain,
        _ if pct <= 20.0 => Tone::Warn,
        _ => Tone::Calm,
    }
}

#[derive(Default)]
pub struct Power {
    charge: Option<Charge>,
    profiles: Vec<String>,
    active: String,
    mode_name: String,
    read: u8,
    awake: bool,
    limit_on: bool,
    can_hibernate: bool,
    laptop: bool,
    limit_supported: bool,
    pin_set: bool,
    pin: String,
    values: HashMap<String, String>,
    hearing: Option<Listen>,
}

impl Page for Power {
    fn id(&self) -> &'static str {
        "power"
    }
    fn title(&self) -> &'static str {
        t!("settings-power-title")
    }
    fn icon(&self) -> &'static str {
        "battery-good-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-power-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-power-help")
    }

    fn view(&self, kit: &Kit) -> Node<Msg> {
        let words = if self.read == 3 {
            sentence(self.charge.as_ref(), &self.mode_name)
        } else {
            t!("settings-power-top-looking").into()
        };
        page(kit, self.title(), self.help(), vec![
            headline(kit, words),
            self.battery_card(kit),
            self.screen_card(kit),
            self.sleep_card(kit),
            self.locking_card(kit),
            self.session_card(kit),
        ])
    }

    fn shown(&mut self, kit: &mut Kit) {
        self.laptop = crate::common::charge::laptop();
        self.limit_supported = crate::common::charge::supported();
        self.limit_on = crate::common::charge::on();
        self.awake = idle::keep_awake();
        self.load_values();
        if let Some(bus) = kit.system.clone() {
            if self.hearing.is_none() {
                let tell = kit.heard::<Power>(|me, k, _| me.load_battery(k));
                self.hearing = bus
                    .listen(Some(UPOWER), "org.freedesktop.DBus.Properties", Some("PropertiesChanged"), Some(DISPLAY_DEVICE),
                            move |_| tell("changed"))
                    .ok();
            }
            if self.laptop {
                let back = kit.then::<Power, Result<Message, Failed>>(|me, _, r| {
                    me.can_hibernate = r.ok()
                        .and_then(|m| m.body().deserialize::<(String,)>().ok())
                        .is_some_and(|(s,)| s == "yes");
                });
                bus.call(LOGIN, LOGIN_PATH, LOGIN_IFACE).method("CanHibernate", &(), false, None, &kit.events, back);
            }
        }
        self.load_battery(kit);
        self.load_profiles(kit);
    }

    fn hidden(&mut self, _kit: &mut Kit) {
        self.hearing = None;
    }

    fn conf_changed(&mut self, _kit: &mut Kit) {
        self.load_values();
    }

    fn second(&mut self, _kit: &mut Kit) -> bool {
        let awake = idle::keep_awake();
        let changed = awake != self.awake;
        self.awake = awake;
        changed
    }
}

impl Power {
    fn load_values(&mut self) {
        self.values = KEYS.iter().map(|k| (k.to_string(), setting(k))).collect();
        self.pin_set = pin::is_set();
    }

    fn value(&self, key: &str) -> String {
        self.values.get(key).cloned().unwrap_or_else(|| setting(key))
    }

    fn save(&mut self, kit: &mut Kit, line: &str, key: &str, value: &str, restart: bool) -> bool {
        if kit.settle_quietly(line, set_setting(key, value), not_saved()).is_none() {
            return false;
        }
        self.values.insert(key.to_string(), value.to_string());
        if restart {
            idle::restart();
        }
        true
    }

    fn warn_level(&self) -> u32 {
        self.value("battery_warn").parse().unwrap_or(10)
    }

    fn load_battery(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.system.clone() else {
            self.got_battery(Err(Failed { name: String::new(), text: String::new() }));
            return;
        };
        let back = kit.then::<Power, Result<Props, Failed>>(|me, _, r| me.got_battery(r));
        bus.call(UPOWER, DISPLAY_DEVICE, "org.freedesktop.UPower.Device").get_all(&kit.events, back);
    }

    fn got_battery(&mut self, r: Result<Props, Failed>) {
        self.read |= 1;
        let Some(props) = r.ok().filter(|p| get_bool(p, "IsPresent")) else {
            self.charge = None;
            return;
        };
        let percent = get_f64(&props, "Percentage").round();
        let state = crate::common::charge::true_state(get_u32(&props, "State"));
        let left = duration(get_i64(&props, if state == 2 { "TimeToEmpty" } else { "TimeToFull" }));
        self.charge = Some(Charge { percent, state, left });
    }

    fn load_profiles(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.system.clone() else {
            self.got_profiles(Err(Failed { name: String::new(), text: String::new() }));
            return;
        };
        let back = kit.then::<Power, Result<Props, Failed>>(|me, _, r| me.got_profiles(r));
        bus.call(PROFILES, PROFILES_PATH, PROFILES).get_all(&kit.events, back);
    }

    fn got_profiles(&mut self, r: Result<Props, Failed>) {
        self.read |= 2;
        self.profiles.clear();
        self.mode_name.clear();
        let Ok(props) = r else { return };
        self.profiles = props
            .get("Profiles")
            .and_then(|v| v.try_clone().ok())
            .and_then(|v| Vec::<HashMap<String, OwnedValue>>::try_from(v).ok())
            .unwrap_or_default()
            .iter()
            .map(|p| get_str(p, "Profile"))
            .collect();
        if self.profiles.is_empty() {
            return;
        }
        self.active = get_str(&props, "ActiveProfile");
        self.mode_name = profile_name(&self.active);
    }

    fn choose_profile(&mut self, kit: &mut Kit, id: String) {
        let Some(bus) = kit.system_for(MODE) else { return };
        let chosen = id.clone();
        let back = kit.settles::<Power, (), Failed>(MODE, t!("settings-power-mode-failed"), move |me, _, done| {
            if done.is_some() {
                me.mode_name = profile_name(&chosen);
                me.active = chosen;
            }
        });
        bus.call(PROFILES, PROFILES_PATH, PROFILES).set("ActiveProfile", Value::from(id.as_str()), true, &kit.events, back);
    }

    fn mode_line(&self, kit: &Kit) -> Option<Node<Msg>> {
        if self.profiles.is_empty() {
            return None;
        }
        let names: Vec<String> = self.profiles.iter().map(|p| profile_name(p)).collect();
        let chosen = self.profiles.iter().position(|p| *p == self.active);
        let ids = self.profiles.clone();
        let pick = dropdown(kit, MODE, names, chosen, t!("settings-power-mode"), move |i| {
            let id = ids[i].clone();
            on::<Power>(move |me, k| me.choose_profile(k, id.clone()))
        });
        Some(line(t!("settings-power-mode"))
            .sub(t!("settings-power-mode-about"))
            .key(MODE)
            .below(pick)
            .node(kit))
    }

    fn timed(&self, kit: &Kit, line_key: &'static str, key: String, title: &str, caption: Option<When>,
             options: Vec<(&'static str, &'static str)>, restart: bool) -> Node<Msg> {
        let now = self.value(&key);
        let chosen = options.iter().position(|(v, _)| *v == now).or(Some(0));
        let names: Vec<String> = options.iter().map(|(_, l)| l.to_string()).collect();
        let values: Vec<&'static str> = options.iter().map(|(v, _)| *v).collect();
        let dd_key = format!("power:dd:{key}");
        let named = caption.map_or_else(|| title.to_string(), |c| c.named(title));
        let pick = dropdown(kit, &dd_key, names, chosen, &named, move |i| {
            let (key, value) = (key.clone(), values[i]);
            on::<Power>(move |me, k| {
                if me.save(k, line_key, &key, value, restart) {
                    k.say(line_key, t!("settings-power-saved"), true, false);
                }
            })
        });
        match caption {
            Some(when) => column(vec![note(when.caption()), pick]).gap(4.0),
            None => pick,
        }
    }

    fn timer_line(&self, kit: &Kit, key: &'static str, title: &str, sub: &str) -> Node<Msg> {
        let l = line(title).sub(sub).key(key);
        if !self.laptop {
            return l.control(self.timed(kit, key, key.to_string(), title, None, times(), true)).node(kit);
        }
        l.below(row(vec![
            self.timed(kit, key, format!("{key}_battery"), title, Some(When::Battery), times(), true),
            self.timed(kit, key, key.to_string(), title, Some(When::Charger), times(), true),
        ]).gap(PAIR_GAP)).node(kit)
    }

    fn screen_card(&self, kit: &Kit) -> Node<Msg> {
        let awake = line(t!("settings-power-awake"))
            .sub(t!("settings-power-awake-about"))
            .key(AWAKE)
            .control(switch(self.awake, t!("settings-power-awake"), |v| on::<Power>(move |me, k| {
                idle::set_keep_awake(v);
                me.awake = idle::keep_awake();
                k.settle_quietly(AWAKE, kept(me.awake == v), if v { t!("settings-power-no-awake") } else { t!("settings-power-no-sleep") });
            })))
            .node(kit);
        let mut rows = vec![awake];
        for &(key, title, sub) in &timers()[..2] {
            rows.push(self.timer_line(kit, key, title, sub));
        }
        group(t!("settings-power-screen-title")).icon("video-display-symbolic")
            .about(t!("settings-power-screen-about"))
            .rows(rows)
            .node(kit)
    }

    fn sleep_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows: Vec<Node<Msg>> = timers()[2..].iter().map(|&(key, title, sub)| self.timer_line(kit, key, title, sub)).collect();
        if self.laptop {
            let pair = [("lid_close_battery", When::Battery), ("lid_close", When::Charger)]
                .iter()
                .map(|&(key, when)| self.lid(kit, key, when))
                .collect();
            rows.push(line(t!("settings-power-lid"))
                .sub(t!("settings-power-lid-about"))
                .key("lid_close")
                .below(row(pair).gap(PAIR_GAP))
                .node(kit));
        }
        group(t!("settings-power-sleep-title")).icon("system-suspend-symbolic")
            .about(t!("settings-power-sleep-about"))
            .rows(rows)
            .node(kit)
    }

    fn battery_card(&self, kit: &Kit) -> Node<Msg> {
        let level = self.warn_level();
        let names: Vec<String> = WARN_LEVELS.iter().map(|l| if *l == 0 { t!("settings-power-never").to_string() } else { format!("{l}%") }).collect();
        let chosen = WARN_LEVELS.iter().position(|l| *l == level).or(Some(0));
        let warn = dropdown(kit, "power:dd:battery_warn", names, chosen, t!("settings-power-warn-name"), |i| {
            let v = WARN_LEVELS[i].to_string();
            on::<Power>(move |me, k| {
                if me.save(k, "battery_warn", "battery_warn", &v, false) {
                    k.say("battery_warn", t!("settings-power-saved"), true, false);
                }
            })
        });
        let mut rows: Vec<Node<Msg>> = self.mode_line(kit).into_iter().collect();
        rows.push(line(t!("settings-power-warn"))
            .sub(t!("settings-power-warn-about"))
            .key("battery_warn")
            .control(warn)
            .node(kit));
        if self.limit_supported {
            rows.push(line(t!("settings-power-limit"))
                .sub(t!("settings-power-limit-about"))
                .key(LIMIT)
                .control(switch(self.limit_on, t!("settings-power-limit"), |v| on::<Power>(move |me, k| me.charge_limit(k, v))))
                .node(kit));
        }
        let drawn = self.charge.is_some() || (self.laptop && self.read != 3);
        let mut pictures = Vec::new();
        if drawn {
            pictures.push(battery_art(self.charge.as_ref().cloned(), self.read == 3));
        }
        if self.profiles.is_empty() {
            pictures.push(limits_art(level, self.limit_supported && self.limit_on));
        }
        let mut card = group(if drawn { t!("settings-power-card-battery") } else if self.profiles.is_empty() { t!("settings-power-card-looking-after") } else { t!("settings-power-mode") })
            .icon(if drawn { "battery-good-symbolic" } else { "battery-caution-symbolic" })
            .span(5, 8);
        if !pictures.is_empty() {
            card = card.vis(column(pictures).gap(12.0));
        }
        card.about(t!("settings-power-battery-about"))
            .rows(rows)
            .node(kit)
    }

    fn lid(&self, kit: &Kit, key: &'static str, when: When) -> Node<Msg> {
        let choices: Vec<(&'static str, &'static str)> =
            lid_choices().into_iter().filter(|(id, _)| *id != "hibernate" || self.can_hibernate).collect();
        let now = self.value(key);
        let chosen = choices.iter().position(|(v, _)| *v == now).or(Some(0));
        let names: Vec<String> = choices.iter().map(|(_, l)| l.to_string()).collect();
        let pick = dropdown(kit, &format!("power:dd:{key}"), names, chosen, when.caption(), move |i| {
            let value = choices[i].0;
            on::<Power>(move |me, k| {
                if me.save(k, "lid_close", key, value, false) {
                    k.say("lid_close", t!("settings-power-saved"), true, false);
                }
            })
        });
        column(vec![note(when.caption().to_string()), pick]).gap(4.0)
    }

    fn charge_limit(&mut self, kit: &mut Kit, on: bool) {
        self.limit_on = on;
        kit.say(LIMIT, t!("settings-power-limit-waiting"), true, true);
        let back = kit.then::<Power, (bool, String, String)>(move |me, k, (ok, _, err)| {
            let err = admin_error(&err);
            k.done(LIMIT, if ok { None } else { Some(&err) }, if on { t!("settings-power-limit-on") } else { t!("settings-power-limit-off") });
            if !ok {
                me.limit_on = !on;
            }
        });
        let helper = admin_helper();
        capture(&["pkexec", &helper, "charge-limit", if on { "on" } else { "off" }], None,
                move |ok, out, err| back((ok, out, err)));
    }

    fn locking_card(&self, kit: &Kit) -> Node<Msg> {
        let lock_on = self.value("lock") == "on";
        let mut rows = vec![line(t!("settings-power-lock"))
            .sub(t!("settings-power-lock-about"))
            .key(LOCK)
            .control(switch(lock_on, t!("settings-power-lock"), |now| on::<Power>(move |me, k| {
                if me.save(k, LOCK, "lock", if now { "on" } else { "off" }, false) {
                    if !now {
                        me.forget_pin(k, LOCK);
                    }
                    idle::restart();
                }
            })))
            .node(kit)];
        let off = !lock_on;
        rows.extend(self.lock_rows(kit, off));
        group(t!("settings-power-lock-title")).icon("system-lock-screen-symbolic")
            .about(t!("settings-power-lock-card-about"))
            .rows(rows)
            .node(kit)
    }

    fn lock_rows(&self, kit: &Kit, off: bool) -> Vec<Node<Msg>> {
        let mut rows = Vec::new();
        let mut after = self.value("lock_after");
        if after.is_empty() {
            after = "10".into();
        }
        let lock_times = lock_times();
        let chosen = lock_times.iter().position(|(v, _)| *v == after).or(Some(0));
        let names: Vec<String> = lock_times.iter().map(|(_, l)| l.to_string()).collect();
        let values: Vec<&'static str> = lock_times.iter().map(|(v, _)| *v).collect();
        let pick = dropdown(kit, "power:dd:lock_after", names, chosen, t!("settings-power-lock-after-name"), move |i| {
            let v = values[i];
            on::<Power>(move |me, k| {
                if me.save(k, "lock_after", "lock_after", v, true) {
                    k.say("lock_after", t!("settings-power-saved"), true, false);
                }
            })
        });
        rows.push(line(t!("settings-power-lock-after-name"))
            .sub(t!("settings-power-lock-after-about"))
            .key("lock_after")
            .control(pick)
            .node(kit).off(off));
        rows.push(line(t!("settings-power-lock-sleep"))
            .sub(t!("settings-power-lock-sleep-about"))
            .key("lock_on_suspend")
            .control(switch(self.value("lock_on_suspend") != "no", t!("settings-power-lock-sleep"), |v| on::<Power>(move |me, k| {
                if me.save(k, "lock_on_suspend", "lock_on_suspend", if v { "yes" } else { "no" }, true) {
                    k.say("lock_on_suspend", t!("settings-power-saved"), true, false);
                }
            })))
            .node(kit).off(off));
        rows.push(line(t!("settings-power-lock-now"))
            .sub(t!("settings-power-lock-now-about"))
            .key(LOCK_NOW)
            .control(button("system-lock-screen-symbolic", t!("settings-power-lock-now")).on_press(on::<Power>(|_, k| {
                let started = spawn(&["trindle-lock"]);
                k.started(LOCK_NOW, started, t!("settings-power-lock-now-failed"));
            })))
            .node(kit).off(off));
        rows.extend(self.pin_rows(kit).into_iter().map(|r| r.off(off)));
        rows
    }

    fn pin_rows(&self, kit: &Kit) -> Vec<Node<Msg>> {
        let mut rows = Vec::new();
        if !self.pin_set {
            rows.push(line(t!("settings-power-pin-add"))
                .sub(t!("settings-power-pin-add-about"))
                .key(PIN)
                .controls(self.pin_entry())
                .node(kit));
        } else {
            let minutes = self.value("pin_minutes").parse().unwrap_or_else(|_| pin::minutes());
            let names: Vec<String> = pin::WINDOWS.iter().map(|m| pin::window_label(*m)).collect();
            let chosen = pin::WINDOWS.iter().position(|m| *m == minutes).or(Some(0));
            let pick = dropdown(kit, "power:dd:pin_minutes", names, chosen, t!("settings-power-pin-works-name"), |i| {
                let v = pin::WINDOWS[i].to_string();
                on::<Power>(move |me, k| {
                    if me.save(k, "pin_minutes", "pin_minutes", &v, false) {
                        k.say("pin_minutes", t!("settings-power-saved"), true, false);
                    }
                })
            });
            rows.push(line(t!("settings-power-pin-works-name"))
                .sub(t!("settings-power-pin-works-about"))
                .key("pin_minutes")
                .control(pick)
                .node(kit));
            let mut controls = self.pin_entry();
            controls.push(confirm::<Power>(kit, PIN, "user-trash-symbolic", t!("settings-power-pin-stop"), |me, k| me.forget_pin(k, PIN)));
            rows.push(line(t!("settings-power-pin-kept"))
                .sub(t!("settings-power-pin-kept-about"))
                .key(PIN)
                .controls(controls)
                .node(kit));
        }
        rows
    }

    fn pin_entry(&self) -> Vec<Node<Msg>> {
        let keep = on::<Power>(|me, k| me.keep_pin(k));
        vec![
            entry(self.pin.clone(), t!("settings-power-pin-new"), |t| {
                let clean: String = t.chars().filter(|c| c.is_ascii_digit()).take(pin::LENGTH).collect();
                on::<Power>(move |me, _| me.pin = clean.clone())
            })
            .key("power:pin")
            .secret_of(Secret::Pin)
            .on_submit(keep.clone())
            .width(Length::Fixed(110.0)),
            button(SAVE_ICON, t!("settings-power-pin-keep")).on_press(keep),
        ]
    }

    fn forget_pin(&mut self, kit: &mut Kit, line: &str) {
        pin::remove();
        self.pin_set = pin::is_set();
        kit.settle_quietly(line, kept(!self.pin_set), t!("settings-power-pin-not-removed"));
    }

    fn keep_pin(&mut self, kit: &mut Kit) {
        if !pin::valid(&self.pin) {
            kit.say(PIN, &t!("settings-power-pin-digits", n = pin::LENGTH), false, false);
            return;
        }
        if let Err(e) = pin::set(&self.pin) {
            kit.say(PIN, &e, false, false);
            return;
        }
        self.pin.clear();
        self.pin_set = true;
        idle::restart();
        kit.say(PIN, t!("settings-power-saved"), true, false);
    }

    fn session_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows = vec![line(t!("settings-power-log-out"))
            .sub(t!("settings-power-log-out-about"))
            .control(confirm_by_hand::<Power>(kit, "power:logout", "system-log-out-symbolic", t!("settings-power-log-out"), crate::common::person_only::SETTINGS_POWER, |me, k| me.way_out(k, None)))
            .node(kit)];
        rows.extend(session().iter().map(|&(title, method, icon)| {
            let key = format!("power:{method}");
            line(title)
                .key(key.clone())
                .control(confirm_by_hand::<Power>(kit, &key, icon, title, crate::common::person_only::SETTINGS_POWER, move |me, k| me.way_out(k, Some(method))))
                .node(kit)
        }));
        group(t!("settings-power-session-title")).icon("system-shutdown-symbolic")
            .about(t!("settings-power-session-about"))
            .rows(rows)
            .node(kit)
    }

    fn way_out(&mut self, kit: &mut Kit, method: Option<&'static str>) {
        if !crate::toolkit::window::persons_own(crate::common::person_only::SETTINGS_POWER) {
            return;
        }
        match method {
            None => crate::common::sounds::play_then("logout", crate::common::compositor::end_session),
            Some(method) => self.logind(kit, method),
        }
    }

    fn logind(&mut self, kit: &mut Kit, method: &'static str) {
        let key = format!("power:{method}");
        if kit.system.is_none() {
            kit.say(&key, t!("settings-power-no-bus"), false, false);
            return;
        }
        if method == "Suspend" {
            send_logind(kit, method);
        } else {
            let go = kit.then::<Power, ()>(move |_, k, ()| send_logind(k, method));
            crate::common::sounds::play_then("logout", move || go(()));
        }
    }
}

fn send_logind(kit: &mut Kit, method: &'static str) {
    let key = format!("power:{method}");
    let Some(bus) = kit.system_for(&key) else { return };
    let back = kit.settles_quietly::<Power, Message, Failed>(&key, t!("settings-power-would-not"), no_more);
    bus.call(LOGIN, LOGIN_PATH, LOGIN_IFACE).method(method, &(true,), true, None, &kit.events, back);
}

fn battery_art(charge: Option<Charge>, read: bool) -> Node<Msg> {
    let tip = match &charge {
        _ if !read => t!("settings-power-art-looking-tip").to_string(),
        Some(c) => t!("settings-power-art-tip", percent = c.percent, state = state_text(c.state)),
        None => String::new(),
    };
    sketch(tip, (420.0, 150.0), crate::toolkit::theme::bento().picture_least, move |s| {
        let (words, quiet, boxed, glass) = (s.words(), s.quiet(), s.boxed(), s.page());
        if !read {
            s.rect((8.0, 22.0, 196.0, 108.0), 18.0, boxed);
            s.rect((20.0, 34.0, 172.0, 84.0), 10.0, glass);
            s.rect((204.0, 54.0, 16.0, 44.0), 6.0, boxed);
            s.text(t!("settings-power-art-looking"), (326.0, 76.0), 15.0, quiet);
            return;
        }
        let Some(c) = &charge else { return };
        let share = (c.percent / 100.0).clamp(0.0, 1.0) as f32;
        let fill = match battery_tone(c.percent, c.state) {
            Tone::Warn => Palette::Orange.colour(),
            _ => s.highlight(),
        };
        s.rect((8.0, 22.0, 196.0, 108.0), 18.0, boxed);
        s.rect((20.0, 34.0, 172.0, 84.0), 10.0, glass);
        s.rect((204.0, 54.0, 16.0, 44.0), 6.0, boxed);
        if share > 0.0 {
            s.rect((20.0, 34.0, (172.0 * share).max(20.0), 84.0), 10.0, fill);
        }
        if c.state == 1 {
            let bolt = [(112.0, 38.0), (82.0, 80.0), (102.0, 80.0), (94.0, 114.0), (128.0, 68.0), (108.0, 68.0), (118.0, 38.0)];
            s.shape(&bolt, words);
        }
        s.text(&format!("{}%", c.percent), (326.0, 46.0), 40.0, words);
        let what = state_text(c.state);
        s.text(if what.is_empty() { t!("settings-power-art-battery") } else { what }, (326.0, 92.0), 14.0, quiet);
        if !c.left.is_empty() {
            let left = t!("settings-power-art-left", left = &c.left, state = if c.state == 2 { "empty" } else { "full" });
            s.text(&left, (326.0, 116.0), 14.0, quiet);
        }
    })
}

fn limits_art(warn: u32, limit: bool) -> Node<Msg> {
    let top = if limit { 0.8 } else { 1.0 };
    let tip = match warn {
        0 => t!("settings-power-limits-tip", top = (top * 100.0) as u32),
        w => t!("settings-power-limits-tip-warn", top = (top * 100.0) as u32, warn = w),
    };
    sketch(tip, (240.0, 40.0), 40.0, move |s| {
        let (track, hl) = (s.track(), s.highlight());
        s.rect((0.0, 12.0, 240.0, 16.0), 8.0, track);
        s.rect((0.0, 12.0, 240.0 * top, 16.0), 8.0, hl);
        if warn > 0 {
            let x = 240.0 * warn as f32 / 100.0;
            s.rect((x - 2.0, 4.0, 4.0, 32.0), 2.0, Palette::Orange.colour());
        }
    })
}

const PAIR_GAP: f32 = 24.0;

#[cfg(test)]
impl Power {
    pub(crate) fn flipped() -> Vec<Power> {
        let mut unlocked = Power::made_up();
        unlocked.values.insert("lock".into(), "off".into());
        let pin = Power { pin_set: true, ..Power::made_up() };
        vec![Power::made_up(), unlocked, pin]
    }

    pub(crate) fn made_up() -> Power {
        let values = [
            ("dim_after", "5"), ("dim_after_battery", "2"), ("screensaver_after", "0"), ("screensaver_after_battery", "0"),
            ("blank_after", "10"), ("blank_after_battery", "5"), ("suspend_after", "30"), ("suspend_after_battery", "15"),
            ("lid_close", "suspend"), ("lid_close_battery", "suspend"), ("battery_warn", "10"), ("lock", "on"),
            ("lock_after", "10"), ("lock_on_suspend", "yes"), ("pin_minutes", "60"),
        ];
        Power { charge: Some(Charge { percent: 82.0, state: 2, left: "4 h 10 min".into() }),
                profiles: vec!["power-saver".into(), "balanced".into(), "performance".into()], active: "balanced".into(),
                mode_name: "Balanced".into(), read: 3, laptop: true, limit_supported: true, can_hibernate: true,
                values: values.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(), ..Power::default() }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_low_battery_is_orange_and_never_red() {
        for pct in 0..=100 {
            for state in 0..=5 {
                assert!(!matches!(battery_tone(f64::from(pct), state), Tone::Problem), "{pct}% in state {state}");
            }
        }
        assert!(matches!(battery_tone(5.0, 2), Tone::Warn));
    }
    use super::*;

    fn kit() -> Kit {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: crate::settings::ui::Do| {});
        Kit::new(events, asks, None, None)
    }

    fn pictures(n: &Node<Msg>) -> usize {
        let own = match &n.what {
            crate::toolkit::widget::What::Drawing { size, .. } if size.0 > 48.0 => 1,
            _ => 0,
        };
        own + n.items().iter().map(pictures).sum::<usize>()
    }

    #[test]
    fn the_time_pies_are_gone_and_the_battery_pictures_stay() {
        let (p, kit) = (Power::made_up(), kit());
        assert_eq!(pictures(&p.screen_card(&kit)), 0, "Screen has no picture");
        assert_eq!(pictures(&p.sleep_card(&kit)), 0, "Screen off and sleep has no picture");
        assert_eq!(pictures(&p.battery_card(&kit)), 1, "the battery");
        let plain = Power { profiles: Vec::new(), ..Power::made_up() };
        assert_eq!(pictures(&plain.battery_card(&kit)), 2, "the battery and the charge bar");
        let said = p.view(&kit).said();
        for &(_, line, _) in &timers() {
            assert!(said.iter().any(|w| w == line), "{line} is still set on the page: {said:?}");
        }
    }

    #[test]
    fn the_locking_lines_stay_greyed_with_locking_off() {
        let kit = kit();
        let on = Power::made_up().locking_card(&kit);
        let off = Power::flipped().remove(1).locking_card(&kit);
        assert_eq!(on.said(), off.said(), "the same lines either way");
        assert!(off.buttons().iter().filter(|(tip, _)| !tip.starts_with("About")).all(|(_, usable)| !usable), "{:?}", off.buttons());
        assert!(on.buttons().iter().any(|(tip, usable)| tip == t!("settings-power-lock-now") && *usable));
    }

    #[test]
    fn the_battery_in_one_sentence() {
        let on = Charge { percent: 82.0, state: 2, left: "4 h 10 min".into() };
        assert_eq!(sentence(Some(&on), ""), "Battery at 82%, about 4 h 10 min left.");
        let low = Charge { percent: 9.0, state: 2, left: String::new() };
        assert_eq!(sentence(Some(&low), ""), "Battery at 9%.");
        let charging = Charge { percent: 40.0, state: 1, left: "1 h 5 min".into() };
        assert_eq!(sentence(Some(&charging), ""), "Charging, 40%, full in 1 h 5 min.");
        assert_eq!(sentence(None, "Balanced"), "Plugged in, in Balanced mode: no battery to look after.");
    }

    #[test]
    fn the_look_pages_follow_the_language_and_fall_back_word_by_word() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let french = "settings-power-time-never = Jamais\nsettings-power-pin-minutes =\n    { $n ->\n        [one] { $n } minute (fr)\n       *[other] { $n } minutes (fr)\n    }\nsettings-power-top-full = La batterie est pleine.\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            let times = times();
            assert_eq!((times[0].1, times[1].1), ("Jamais", "1 minute"), "a word French lacks is English");
            assert_eq!(crate::common::pin::window_label(45), "45 minutes (fr)");
            assert_eq!(crate::common::pin::window_label(75), "1 hour 15 minutes (fr)", "the hours are English, the minutes French");
            let full = Charge { percent: 100.0, state: 4, left: String::new() };
            assert_eq!(sentence(Some(&full), ""), "La batterie est pleine.");
        });
    }

    #[test]
    fn a_wild_guess_at_time_left_is_not_given() {
        assert_eq!(duration(90 * 60), "1 h 30 min");
        assert_eq!(duration(930 * 60 * 60), "");
    }

    #[test]
    fn only_the_persons_own_press_leaves_the_session_from_this_page() {
        use crate::toolkit::window::{with_named, with_press};
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: crate::settings::ui::Do| {});
        let mut kit = Kit::new(events.clone(), asks, None, None);
        let mut page = Power::made_up();
        for (_, method, _) in session() {
            let key = format!("power:{method}");
            with_named(true, Some(7), || with_press(Some(8), || page.way_out(&mut kit, Some(method))));
            with_named(true, None, || with_press(Some(7), || page.way_out(&mut kit, Some(method))));
            with_named(true, Some(7), || page.way_out(&mut kit, Some(method)));
            assert_eq!(kit.said(&key), None, "{method} was run for a press that was not the person's own");
            with_named(true, Some(7), || with_press(Some(7), || page.way_out(&mut kit, Some(method))));
            assert!(kit.said(&key).is_some(), "{method} was refused the person's own press");
        }
        let code = crate::common::source::code(include_str!("power.rs"));
        let code = code.split("\nmod tests {").next().unwrap();
        assert_eq!(code.matches("me.way_out(k, ").count(), 2, "Log out, and the three logind is asked for");
        assert_eq!(code.matches("confirm_by_hand::<Power>(").count(), 2, "a way out of the session can be armed by a made-up press");
        assert_eq!(code.matches("crate::common::person_only::SETTINGS_POWER").count(), 3, "the arming and the running are not one act");
        assert_eq!(code.matches("end_session").count(), 1, "logging out is run in one place");
        assert_eq!(code.matches(".logind(").count(), 1, "logind is asked from one place");
        assert_eq!(code.matches("send_logind(").count(), 3, "the function, and its two calls in logind");
        let way_out = code.split("fn way_out(").nth(1).unwrap().split("fn logind(").next().unwrap();
        assert!(way_out.contains("end_session") && way_out.contains("self.logind(kit, method)"));
    }

    #[test]
    fn a_session_button_never_does_nothing() {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: crate::settings::ui::Do| {});
        let mut kit = Kit::new(events.clone(), asks, None, None);
        for (_, method, _) in session() {
            send_logind(&mut kit, method);
            assert_eq!(kit.said(&format!("power:{method}")), Some((crate::settings::ui::no_system_bus(), true)));
        }
    }
}
