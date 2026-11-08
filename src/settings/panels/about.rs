//! About: hostname (changed via trindle-admin) and hardware facts.

use std::path::PathBuf;

use crate::common::{boot, security_updates, update_checks};
use crate::common::files::{config_home, home, read_text as read};
use crate::common::run::{self, capture};
use crate::settings::ui::{asked, group, headline, line, not_saved, on, opens, page, Kit, Msg, Page, SAVE_ICON};
use crate::toolkit::t;
use crate::toolkit::theme::Palette;
use crate::toolkit::tiles::bars;
use crate::toolkit::widget::{button, column, entry, label, note, row, switch, word_button, Length, Node};

fn current_name() -> String {
    read("/etc/hostname").trim().to_string()
}

fn debian() -> String {
    read("/etc/os-release").lines().find_map(|l| l.strip_prefix("PRETTY_NAME="))
        .map(|v| v.trim_matches('"').to_string()).unwrap_or_else(|| t!("settings-about-unknown").to_string())
}

fn processor() -> String {
    let info = read("/proc/cpuinfo");
    let model = info.lines().find_map(|l| l.split_once(':').filter(|(k, _)| k.trim() == "model name").map(|(_, v)| v.trim().to_string()))
        .unwrap_or_else(|| std::env::consts::ARCH.to_string());
    let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1);
    t!("settings-about-processor", model = model, cores = cores)
}

fn meminfo(key: &str) -> u64 {
    read("/proc/meminfo").lines().find_map(|l| l.strip_prefix(key))
        .and_then(|v| v.split_whitespace().next()?.parse().ok()).unwrap_or(0)
}

fn memory() -> (f64, f64) {
    let (total, free) = (meminfo("MemTotal:"), meminfo("MemAvailable:"));
    (total as f64 / 1_000_000.0, total.saturating_sub(free) as f64 / 1_000_000.0)
}

fn disk() -> Option<(f64, f64)> {
    // SAFETY: statvfs is plain numbers, so all-zeroes is a valid starting

    let mut st: libc::statvfs = unsafe { std::mem::zeroed() };
    // SAFETY: a static NUL-terminated path, and a local struct the kernel fills.
    if unsafe { libc::statvfs(c"/".as_ptr(), &mut st) } != 0 {
        return None;
    }
    let gb = |blocks: u64| blocks as f64 * st.f_frsize as f64 / 1_000_000_000.0;
    Some((gb(st.f_blocks), gb(st.f_bavail)))
}

fn gb(n: f64) -> String {
    t!("kit-size-gb", size = trindle_toolkit::lang::format::decimal(n, if n < 10.0 { 1 } else { 0 }))
}

fn model() -> String {
    let get = |f: &str| read(format!("/sys/class/dmi/id/{f}")).trim().to_string();
    let (vendor, name, version) = (get("sys_vendor"), get("product_name"), get("product_version"));

    let model = if vendor.eq_ignore_ascii_case("lenovo") && !version.is_empty() { version } else { name };
    let text = format!("{vendor} {model}").trim().to_string();
    if text.is_empty() { t!("settings-about-unknown").to_string() } else { text }
}

fn graphics() -> String {
    let mut found: Vec<&str> = Vec::new();
    for card in crate::common::graphics::cards() {
        let maker = match card.driver.as_str() {
            "i915" | "xe" => t!("settings-about-gpu-intel"),
            "amdgpu" | "radeon" => t!("settings-about-gpu-amd"),
            "nouveau" | "nvidia" => t!("settings-about-gpu-nvidia"),
            "virtio_gpu" | "qxl" | "vmwgfx" | "bochs" => t!("settings-about-gpu-virtual"),
            _ if card.is_nvidia() => t!("settings-about-gpu-nvidia"),
            "" => continue,
            _ => t!("settings-about-gpu-other"),
        };
        if !found.contains(&maker) {
            found.push(maker);
        }
    }
    if found.is_empty() { t!("settings-about-unknown").to_string() } else { found.join(", ") }
}

fn screens() -> String {
    let heads = crate::common::outputs::heads().unwrap_or_default();
    let list: Vec<String> = heads.iter().filter(|h| h.enabled).filter_map(|h| {
        let m = h.current_mode()?;
        Some(t!("settings-about-screen", width = m.width, height = m.height, hz = m.hz().unwrap_or(60.0).round()))
    }).collect();
    if list.is_empty() { t!("settings-about-unknown").to_string() } else { list.join(", ") }
}

fn kernel() -> String {
    // SAFETY: utsname is fixed-size character arrays, so all-zeroes is valid

    let mut u: libc::utsname = unsafe { std::mem::zeroed() };
    // SAFETY: a local struct the kernel fills in and terminates itself.
    if unsafe { libc::uname(&mut u) } != 0 {
        return t!("settings-about-unknown").to_string();
    }
    // SAFETY: uname succeeded, so `release` is a NUL-terminated string.
    let release = unsafe { std::ffi::CStr::from_ptr(u.release.as_ptr()) }.to_string_lossy().to_string();
    format!("Linux {release}")
}

struct Room {
    share: f64,
    figure: String,
    words: String,
}

#[derive(Default)]
pub struct About {
    said: String,
    memory: Option<Room>,
    disk: Option<Room>,

    on: String,

    name: String,

    updates_by_itself: bool,

    security: bool,
}

impl About {

    fn look(&mut self) {
        let (all, used) = memory();
        self.memory = (all > 0.0).then(|| Room { share: used / all, figure: t!("settings-about-in-use", amount = gb(used)),
                                                  words: t!("settings-about-of", amount = gb(all)) });
        self.disk = disk().filter(|(all, _)| *all > 0.0).map(|(all, free)| Room {
            share: (all - free) / all, figure: t!("settings-about-free", amount = gb(free)),
            words: t!("settings-about-of", amount = gb(all)) });
        self.on = switched_on_for();
        self.updates_by_itself = update_checks::on();
        self.security = security_updates::switch_on();
        let version = crate::store::updates::shown(env!("CARGO_PKG_VERSION"));
        self.said = if self.disk.as_ref().is_some_and(|d| d.share >= 0.95) {
            t!("settings-about-said-full", version = version, on = &self.on)
        } else {
            t!("settings-about-said", version = version, on = &self.on)
        };

        if let Some(waits) = crate::common::renewed::said() {
            self.said.push(' ');
            self.said.push_str(&waits);
        }
    }

    fn rename(&mut self, kit: &mut Kit) {
        let name = self.name.trim().to_lowercase();
        if name == current_name() {
            return;
        }
        if !crate::admin::valid_hostname(&name) {
            kit.say("name", t!("settings-about-name-bad"), false, false);
            return;
        }
        kit.say("name", t!("settings-about-name-waiting"), true, true);
        let helper = run::admin_helper();
        let back = kit.settles::<About, (), String>("name", t!("settings-about-name-failed"), |_, k, done| {

            if done.is_some() {
                k.say("name", t!("settings-about-name-done"), true, false);
            }
        });
        capture(&["pkexec", &helper, "hostname", &name], None, move |ok, _, err| back(asked(ok, &err)));
    }

    fn name_card(&self, kit: &Kit) -> Node<Msg> {
        let typed = entry(self.name.clone(), t!("settings-about-name-entry"), |text: String| {
            on::<About>(move |me, _| me.name = text.chars().take(63).collect())
        }).on_submit(on::<About>(|me, k| me.rename(k))).width(Length::Fixed(200.0));
        let rename = button(SAVE_ICON, t!("settings-about-rename")).on_press(on::<About>(|me, k| me.rename(k)));
        let name = line(t!("settings-about-name")).sub(t!("settings-about-name-sub"))
            .key("name").controls(vec![typed, rename]).node(kit);
        group(t!("settings-about-name-group")).icon("network-workgroup-symbolic")
            .about(t!("settings-about-name-about"))
            .rows(vec![name])
            .node(kit)
    }

    fn hardware(&self, kit: &Kit) -> Node<Msg> {
        group(t!("settings-about-hardware")).icon("computer-symbolic")
            .about(t!("settings-about-hardware-about"))
            .rows(vec![
                line(model()).sub(t!("settings-about-hw-computer")).node(kit),
                line(processor()).sub(t!("settings-about-hw-processor")).node(kit),
                line(graphics()).sub(t!("settings-about-hw-graphics")).node(kit),
                line(screens()).sub(t!("settings-about-hw-screen")).node(kit),
            ])
            .node(kit)
    }

    fn software(&self, kit: &Kit) -> Node<Msg> {
        group(t!("settings-about-software")).icon("applications-system-symbolic")
            .about(t!("settings-about-software-about"))
            .rows(vec![
                line(debian()).sub(t!("settings-about-sw-debian")).node(kit),
                line(kernel()).sub(t!("settings-about-sw-kernel")).node(kit),
                line(t!("settings-about-sw-compositor")).sub(t!("settings-about-sw-compositor-sub")).node(kit),
                line(t!("settings-about-sw-apps", n = crate::common::apps::entries().len())).sub(t!("settings-about-sw-apps-sub")).node(kit),
            ])
            .node(kit)
    }

    fn set_updates(&mut self, kit: &mut Kit, now: bool) {
        self.updates_by_itself = now;
        Self::set_switch(kit, update_checks::KEY, "update-checks", update_checks::set, now, |me, v| me.updates_by_itself = v);
    }

    fn set_security(&mut self, kit: &mut Kit, now: bool) {
        self.security = now;
        Self::set_switch(kit, security_updates::KEY, "security-updates", security_updates::set, now, |me, v| me.security = v);
    }

    fn set_switch(kit: &mut Kit, key: &'static str, command: &'static str, keep: fn(bool) -> std::io::Result<()>, now: bool,
                  put: fn(&mut About, bool)) {
        kit.say(key, t!("settings-about-updates-waiting"), true, true);
        let back = kit.then::<About, (bool, String)>(move |me, k, (ok, err)| {
            if ok && k.settle(key, keep(now), not_saved()).is_some() {
                return;
            }
            if !ok {
                k.say(key, &crate::common::run::admin_error(&err), false, false);
            } else {

                let helper = crate::common::run::admin_helper();
                let undone = k.then::<About, (bool, String)>(move |_, k, (ok, err)| {
                    if !ok {
                        k.say(key, &crate::common::run::admin_error(&err), false, false);
                    }
                });
                capture(&["pkexec", &helper, command, if now { "off" } else { "on" }], None, move |ok, _, err| undone((ok, err)));
            }
            put(me, !now);
        });
        let helper = crate::common::run::admin_helper();
        capture(&["pkexec", &helper, command, if now { "on" } else { "off" }], None, move |ok, _, err| back((ok, err)));
    }

    fn updates(&self, kit: &Kit) -> Node<Msg> {
        let key = update_checks::KEY;
        let by_itself = line(t!("settings-about-updates-by-itself")).sub(t!("settings-about-updates-by-itself-sub")).key(key)
            .control(switch(self.updates_by_itself, t!("settings-about-updates-by-itself"), move |now| on::<About>(move |me, k| me.set_updates(k, now))))
            .node(kit);
        let security = line(t!("settings-about-security")).sub(if self.security { t!("settings-about-security-sub") } else { t!("settings-about-security-off") })
            .key(security_updates::KEY)
            .control(switch(self.security, t!("settings-about-security"), move |now| on::<About>(move |me, k| me.set_security(k, now))))
            .node(kit);
        group(t!("settings-about-updates")).icon("software-update-available-symbolic")
            .about(t!("settings-about-updates-about"))
            .rows(vec![by_itself, security])
            .node(kit)
    }

    fn more(&self, kit: &Kit) -> Node<Msg> {
        let open = |icon: &str, heading: &str, words: &str, name: &'static str, program: &'static str| {
            let key = format!("about:open:{program}");
            line(heading).sub(words).key(key.clone())
                .control(button(icon.to_string(), name).on_press(opens(key, program, name)))
                .node(kit)
        };
        group(t!("settings-about-more")).icon("utilities-system-monitor-symbolic")
            .about(t!("settings-about-more-about"))
            .rows(vec![
                open("go-next-symbolic", t!("settings-about-activity"), t!("settings-about-activity-sub"), t!("settings-about-activity-name"),
                     "trindle-activity"),
                open("go-next-symbolic", t!("settings-about-drives"), t!("settings-about-drives-sub"), t!("settings-about-drives-name"),
                     "trindle-drives"),
            ])
            .node(kit)
    }
}

fn room(kit: &Kit, icon: &str, name: &str, r: &Option<Room>) -> Option<Node<Msg>> {
    let r = r.as_ref()?;

    let used = (r.share.clamp(0.0, 1.0) * 100.0).round() as f32 / 100.0;
    Some(group(name).icon(icon)
        .vis(crate::toolkit::tiles::standard_ring(Some(used), 72.0))
        .rows(vec![line(r.figure.clone()).sub(r.words.clone()).node(kit)])

        .span(3, 4)
        .node(kit))
}

fn starts_art(times: Vec<f32>) -> Node<Msg> {
    let most = times.iter().copied().fold(1.0_f32, f32::max);
    bars(Palette::Teal, times.iter().map(|t| t / most).collect())
}

fn downloads() -> PathBuf {
    crate::common::files::user_folder_for(&read(config_home().join("user-dirs.dirs")), "DOWNLOAD", &home()).unwrap_or_else(home)
}

fn save_times(kit: &mut Kit) {
    let body: String = std::iter::once("Date and time,Seconds\n".to_string())
        .chain(boot::history().iter().map(|s| format!("{},{:.1}\n", when(s.at, "%Y-%m-%d %H:%M:%S"), s.took)))
        .collect();
    let dir = downloads();
    let path = dir.join("Trindle start-up times.csv");
    let saved = crate::common::files::write_text(&path, &body);
    if kit.settle("start-times", saved, t!("settings-about-times-failed")).is_some() {
        kit.say("start-times", &t!("settings-about-times-saved", file = "Trindle start-up times.csv"), true, false);
    }
}

fn start_times(kit: &Kit) -> Option<Node<Msg>> {
    let all = boot::history();
    if all.is_empty() {
        return None;
    }

    let save = word_button(t!("settings-about-times-save")).spoken(t!("settings-about-times-save-spoken")).on_press(on::<About>(|_, k| save_times(k)));
    let head = line(t!("settings-about-times-keep")).sub(t!("settings-about-times-keep-sub")).key("start-times").control(save).node(kit);

    let list: Vec<Node<Msg>> = all.iter().rev().take(10).enumerate().map(|(i, s)| {
        let r = row(vec![label(boot::words(s.took)).width(Length::Fixed(110.0)), note(crate::common::clock::Local::at(s.at).line(trindle_toolkit::lang::format::Pattern::Stamp))]).gap(12.0);
        if i % 2 == 1 { r.banded() } else { crate::toolkit::widget::card(r).snug().flat() }
    }).collect();
    let times: Vec<f32> = all.iter().rev().take(10).rev().map(|s| s.took as f32).collect();
    let last = t!("settings-about-times-chart");
    let chart_sentence = note(last);
    Some(group(t!("settings-about-times")).icon("document-open-recent-symbolic")
        .about(t!("settings-about-times-about"))
        .vis(column(vec![chart_sentence, starts_art(times), column(list).gap(0.0)]).gap(12.0))
        .rows(vec![head])
        .span(6, 12)
        .node(kit))
}

impl Page for About {
    fn id(&self) -> &'static str {
        "about"
    }
    fn title(&self) -> &'static str {
        t!("settings-about-title")
    }
    fn icon(&self) -> &'static str {
        "computer-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-about-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-about-help")
    }
    fn shown(&mut self, _kit: &mut Kit) {
        self.look();
        self.name = current_name();
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {

        let mut items = vec![headline(kit, self.said.clone()), self.hardware(kit)];
        items.extend(room(kit, "media-removable-symbolic", t!("settings-about-memory"), &self.memory));
        items.extend(room(kit, "drive-harddisk-symbolic", t!("settings-about-disk"), &self.disk));
        items.push(self.software(kit));
        items.push(self.updates(kit));
        items.push(self.name_card(kit));
        items.push(self.more(kit));
        items.extend(start_times(kit));
        page(kit, self.title(), self.help(), items)
    }
}

fn switched_on_for() -> String {
    let up: f64 = read("/proc/uptime").split_whitespace().next().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    uptime(up)
}

fn uptime(seconds: f64) -> String {
    let minutes = (seconds / 60.0) as u64;
    match minutes {
        0..=59 => t!("settings-about-up-minutes", n = minutes.max(1)),
        60..=2879 => t!("settings-about-up-hours", n = (minutes / 60).max(1)),
        _ => t!("settings-about-up-days", n = minutes / 1440),
    }
}

fn when(at: i64, format: &str) -> String {
    crate::common::clock::Local::at(at).format(format)
}

#[cfg(test)]
impl About {

    pub(crate) fn made_up() -> About {
        About { said: "Trindle 2026.10, switched on for 3 hours.".into(),
                memory: Some(Room { share: 0.31, figure: "5.0 GB in use".into(), words: "of 16 GB".into() }),
                disk: Some(Room { share: 0.42, figure: "290 GB free".into(), words: "of 500 GB".into() }),
                on: "3 hours".into(), name: "johns-laptop".into(), updates_by_itself: true, security: true }
    }

    pub(crate) fn made_up_not_looking() -> About {
        About { updates_by_itself: false, ..About::made_up() }
    }

    pub(crate) fn made_up_not_secure() -> About {
        About { security: false, ..About::made_up() }
    }
}

#[cfg(test)]
mod tests {
    use super::uptime;

    #[test]
    fn uptime_reads_the_way_people_say_it() {
        assert_eq!(uptime(30.0), "1 minute");
        assert_eq!(uptime(90.0 * 60.0), "1 hour");
        assert_eq!(uptime(3.0 * 3600.0), "3 hours");
        assert_eq!(uptime(3.0 * 86400.0), "3 days");
    }

    #[test]
    fn the_about_page_follows_the_language_and_falls_back_word_by_word() {
        use crate::settings::ui::Page;
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let french = "settings-about-title = A propos de cet ordinateur\nsettings-about-up-hours =\n    { $n ->\n        [one] { $n } heure\n       *[other] { $n } heures\n    }\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
            assert_eq!(super::About::default().title(), "A propos de cet ordinateur");
            assert_eq!(uptime(3.0 * 3600.0), "3 heures");
            assert_eq!(uptime(30.0), "1 minute");
        });
    }

    #[test]
    fn sizes_and_starts_follow_the_language() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        assert_eq!((super::gb(3.2), super::gb(16.0)), ("3.2 GB".to_string(), "16 GB".to_string()));
        let made_up = "kit-number-decimal = ,\nkit-size-gb = GO { $size }\nkit-date-pattern-stamp = %e/%m/%Y %H.%M\n";
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!((super::gb(3.2), super::gb(16.0)), ("GO 3,2".to_string(), "GO 16".to_string()));
            let said = crate::common::clock::Local::utc(1_790_924_589).line(trindle_toolkit::lang::format::Pattern::Stamp);
            assert_eq!(said, " 2/10/2026 07.03");
        });
    }
}
