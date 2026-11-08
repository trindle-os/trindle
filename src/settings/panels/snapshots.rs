//! Snapshots: list, restore and delete. Mechanics are in common/snapshots.rs.

use std::time::Duration;

use crate::common::run::{admin_error, admin_helper, capture};
use crate::common::home_snapshots::{self, keep_words, Standing, KEEP_CHOICES};
use crate::common::snapshot_title;
use crate::common::snapshots::{self, Snapshot};
use crate::settings::ui::{confirm, dropdown, group, headline, job, line, no_more, on, page, Kit, Msg, Page};
use crate::toolkit::t;
use crate::toolkit::widget::{button, switch, Node};

const EARLIER: &str = "earlier";
const KEEP: &str = "earlier-keep";
const CLEAR: &str = "earlier-clear";

const LOCKED: &str = "system-locked";

fn nearly_full() -> &'static str {
    t!("settings-snap-nearly-full")
}

fn when(date: &str) -> String {
    match home_snapshots::unix_of(date) {
        Some(unix) => when_unix(unix),
        None => t!("settings-snap-on", date = date),
    }
}

fn when_unix(unix: i64) -> String {
    let m = snapshot_title::moment_at(unix, crate::common::clock::Local::at(unix).utc_offset());
    let today = crate::common::clock::Local::now().day_number();
    if m.day_number == today {
        t!("settings-snap-today", time = m.time)
    } else if m.day_number == today - 1 {
        t!("settings-snap-yesterday", time = m.time)
    } else {
        t!("settings-snap-on", date = m.date)
    }
}

fn keep_choice(days: u32) -> String {
    let words = keep_words(days);
    let mut letters = words.chars();
    match letters.next() {
        Some(first) => first.to_uppercase().chain(letters).collect(),
        None => words,
    }
}

const RING_STAYS: Duration = Duration::from_millis(1500);

fn restart(kit: &mut Kit, key: &str) {

    if !crate::toolkit::window::persons_own(crate::common::person_only::SNAPSHOT_RESTART) {
        return;
    }
    let Some(bus) = kit.system_for(key) else { return };
    let back = kit.settles_quietly::<Snapshots, zbus::Message, crate::toolkit::bus::Failed>(
        key, t!("settings-snap-not-restart"), no_more);
    bus.call("org.freedesktop.login1", "/org/freedesktop/login1", "org.freedesktop.login1.Manager")
        .method("Reboot", &(true,), true, None, &kit.events, back);
}

fn confirm_job(kit: &Kit, key: &str, icon: &str, name: &'static str, f: impl Fn(&mut Snapshots, &mut Kit) + 'static) -> Node<Msg> {
    if kit.dials.contains_key(key) {
        job(kit, key, icon, name, on::<Snapshots>(|_, _| {}))
    } else {
        confirm::<Snapshots>(kit, key, icon, name, f)
    }
}

fn locked_now() -> bool {
    std::path::Path::new(snapshots::READ_ONLY_MARKER).exists()
}

#[derive(Default)]
pub struct Snapshots {

    available: bool,
    fs: String,
    home_separate: bool,
    booted: Option<u32>,
    aside: Vec<String>,

    list: Option<Result<Vec<Snapshot>, String>>,

    restored: Option<u32>,

    day: String,

    earlier: Standing,

    locked: bool,
}

impl Snapshots {
    fn draw(&mut self, kit: &mut Kit) {
        let back = kit.then::<Snapshots, (bool, String, String)>(|me, _, (ok, out, err)| {
            me.list = Some(if ok { Ok(snapshots::parse_list(&out)) } else { Err(admin_error(&err)) });
        });
        capture(&snapshots::list_argv(), None, move |ok, out, err| back((ok, out, err)));
    }

    fn take(&mut self, kit: &mut Kit) {
        kit.start("take", t!("settings-snap-taking")).expect(5.0);
        let back = kit.then::<Snapshots, (bool, String)>(|me, k, (ok, err)| {
            k.finish("take", ok, "");
            if ok {

                me.draw(k);
            } else {
                k.say("take", &admin_error(&err), false, false);
            }
        });
        capture(&["pkexec", &admin_helper(), "snapshot-create", "Taken in Settings"], None, move |ok, _, err| back((ok, err)));
    }

    fn free(&mut self, kit: &mut Kit) {
        kit.say("aside", t!("settings-snap-freeing"), true, true);
        let back = kit.then::<Snapshots, (bool, String)>(|_, k, (ok, err)| {
            if ok { k.say("aside", t!("settings-snap-freed"), true, true) } else { k.say("aside", &admin_error(&err), false, false) }
        });
        capture(&["pkexec", &admin_helper(), "snapshot-forget-set-aside"], None, move |ok, _, err| back((ok, err)));
    }

    fn restore(&mut self, kit: &mut Kit, n: u32) {
        let ring = format!("restore:{n}");
        kit.start(&ring, t!("settings-snap-restoring")).expect(10.0);
        let back = kit.then::<Snapshots, (bool, String)>(move |me, k, (ok, err)| {
            k.finish(&ring, ok, "");
            let row = format!("snap:{n}");
            if !ok {
                k.say(&row, &admin_error(&err), false, false);
                return;
            }
            k.say(&row, &t!("settings-snap-restored", n = n), true, true);
            me.restored = Some(n);
        });
        capture(&["pkexec", &admin_helper(), "snapshot-restore", &n.to_string()], None, move |ok, _, err| back((ok, err)));
    }

    fn delete(&mut self, kit: &mut Kit, n: u32) {
        let ring = format!("delete:{n}");
        kit.start(&ring, t!("settings-snap-deleting")).expect(5.0);
        let back = kit.then::<Snapshots, (bool, String)>(move |_, k, (ok, err)| {
            if ok {
                k.finish(&ring, true, t!("settings-snap-deleted"));
                k.after::<Snapshots>(RING_STAYS, |me, k| me.draw(k));
            } else {
                k.finish(&ring, false, "");
                k.say(&format!("snap:{n}"), &admin_error(&err), false, false);
            }
        });
        capture(&["pkexec", &admin_helper(), "snapshot-delete", &n.to_string()], None, move |ok, _, err| back((ok, err)));
    }

    fn look_at_earlier(&mut self, kit: &mut Kit) {
        self.earlier = home_snapshots::standing();
        self.say_if_full(kit);
    }

    fn say_if_full(&self, kit: &mut Kit) {
        if self.earlier.possible.is_ok() && self.earlier.setting.on && self.earlier.state.skipped_full.is_some() {
            kit.say(EARLIER, nearly_full(), false, true);
        }
    }

    fn earlier_switch(&mut self, kit: &mut Kit, on: bool) {
        self.earlier.setting.on = on;
        kit.say(EARLIER, t!("settings-snap-waiting"), true, true);
        let back = kit.then::<Snapshots, (bool, String)>(move |me, k, (ok, err)| {
            let err = admin_error(&err);
            k.done(EARLIER, if ok { None } else { Some(&err) }, if on { t!("settings-snap-copy-daily") } else { t!("settings-snap-no-more") });
            if ok {
                me.look_at_earlier(k);
            } else {
                me.earlier.setting.on = !on;
            }
        });
        let days = self.earlier.setting.days.to_string();
        let helper = admin_helper();
        let argv: Vec<&str> = if on {
            vec!["pkexec", &helper, "home-snapshots-on", &days]
        } else {
            vec!["pkexec", &helper, "home-snapshots-off"]
        };
        capture(&argv, None, move |ok, _, err| back((ok, err)));
    }

    fn earlier_keep(&mut self, kit: &mut Kit, days: u32) {
        let was = self.earlier.setting.days;
        if days == was {
            return;
        }
        self.earlier.setting.days = days;
        kit.say(KEEP, t!("settings-snap-waiting"), true, true);
        let back = kit.then::<Snapshots, (bool, String)>(move |me, k, (ok, err)| {
            let err = admin_error(&err);
            k.done(KEEP, if ok { None } else { Some(&err) }, t!("settings-snap-kept"));
            if ok {
                me.look_at_earlier(k);
            } else {
                me.earlier.setting.days = was;
            }
        });
        capture(&["pkexec", &admin_helper(), "home-snapshots-keep", &days.to_string()], None, move |ok, _, err| back((ok, err)));
    }

    fn earlier_clear(&mut self, kit: &mut Kit) {
        kit.start(CLEAR, t!("settings-snap-clearing")).expect(10.0);
        let back = kit.then::<Snapshots, (bool, String)>(|me, k, (ok, err)| {
            if ok {
                k.finish(CLEAR, true, t!("settings-snap-cleared"));
                me.look_at_earlier(k);
            } else {
                k.finish(CLEAR, false, "");
                k.say(CLEAR, &admin_error(&err), false, false);
            }
        });
        capture(&["pkexec", &admin_helper(), "home-snapshots-clear"], None, move |ok, _, err| back((ok, err)));
    }

    fn locked_switch(&mut self, kit: &mut Kit, on: bool) {
        self.locked = on;
        kit.say(LOCKED, t!("settings-snap-waiting"), true, true);
        let back = kit.then::<Snapshots, (bool, String)>(move |me, k, (ok, err)| {
            let err = admin_error(&err);
            k.done(LOCKED, if ok { None } else { Some(&err) }, if on { t!("settings-snap-locked-now") } else { t!("settings-snap-unlocked-now") });
            me.locked = locked_now();
        });
        let helper = admin_helper();
        capture(&["pkexec", &helper, "system-read-only", if on { "on" } else { "off" }], None, move |ok, _, err| back((ok, err)));
    }

    fn locked_line(&self, kit: &Kit) -> Node<Msg> {
        let l = line(t!("settings-snap-locked"));
        if self.fs != "btrfs" {
            return l.sub(t!("admin-system-needs-btrfs")).key(LOCKED).node(kit);
        }
        l.sub(t!("settings-snap-locked-sub"))
            .control(switch(self.locked, t!("settings-snap-locked"), |v| on::<Snapshots>(move |me, k| me.locked_switch(k, v))))
            .key(LOCKED).node(kit)
    }

    fn earlier_card(&self, kit: &Kit) -> Node<Msg> {
        let e = &self.earlier;
        let why = e.possible.as_ref().err().map(|w| w.words());
        let n = e.kept.len();
        let copies = t!("settings-snap-copies", n = n);
        let room = match (n, e.state.room) {
            (0, _) => t!("settings-snap-first-soon").to_string(),
            (_, Some((bytes, at))) => {
                let measured = when_unix(at);
                t!("settings-snap-they-take", size = crate::common::volumes::size_words(bytes), when = measured)
            }
            (_, None) => t!("settings-snap-measured-daily").to_string(),
        };
        let clear = if n == 0 {

            button("user-trash-symbolic", t!("settings-snap-clear-all"))
        } else {
            confirm_job(kit, CLEAR, "user-trash-symbolic", t!("settings-snap-clear-all"), |me, k| me.earlier_clear(k))
        };
        let chosen = KEEP_CHOICES.iter().position(|d| *d == e.setting.days);
        group(t!("settings-snap-earlier")).icon("document-open-recent-symbolic")
            .about(t!("settings-snap-earlier-about"))
            .waits_for(why.as_deref())
            .rows(vec![
                line(t!("settings-snap-keep-earlier"))
                    .sub(t!("settings-snap-deleted-stay", time = keep_words(e.setting.days)))
                    .control(switch(e.setting.on, t!("settings-snap-keep-earlier"),
                                    |v| on::<Snapshots>(move |me, k| me.earlier_switch(k, v))))
                    .key(EARLIER).node(kit),
                line(t!("settings-snap-keep-each"))
                    .control(dropdown(kit, KEEP, KEEP_CHOICES.iter().map(|d| keep_choice(*d)).collect(), chosen,
                                      t!("settings-snap-how-long"),
                                      |i| on::<Snapshots>(move |me, k| me.earlier_keep(k, KEEP_CHOICES[i]))))
                    .key(KEEP).node(kit),
                line(copies).sub(room).control(clear).key(CLEAR).node(kit),
            ])
            .node(kit)
    }

    fn head(&self, kit: &Kit) -> Node<Msg> {
        let all = match &self.list {
            None => return headline(kit, t!("settings-snap-looking")),
            Some(Err(_)) => return headline(kit, t!("settings-snap-not-read")),
            Some(Ok(all)) => all,
        };
        let newest = all.iter().max_by_key(|s| s.number);
        let n = all.len();
        if let Some(b) = self.booted {
            headline(kit, t!("settings-snap-started-from", n = b))
        } else if let Some(s) = newest {
            headline(kit, t!("settings-snap-go-back", n = n, when = when(&s.date)))
        } else {
            headline(kit, t!("settings-snap-none-yet"))
        }
    }

    fn row(&self, kit: &Kit, s: &Snapshot) -> Node<Msg> {
        let mut what = s.what();
        if s.important && s.kind == "single" {
            what = t!("settings-snap-kept-until-deleted", what = &what);
        }
        let n = s.number;
        let mut controls = vec![
            confirm_job(kit, &format!("restore:{n}"), "document-revert-symbolic", t!("settings-snap-restore-this"),
                        move |me, k| me.restore(k, n)),
            confirm_job(kit, &format!("delete:{n}"), "user-trash-symbolic", t!("settings-snap-delete-this"),
                        move |me, k| me.delete(k, n)),
        ];
        if self.restored == Some(n) {
            controls.push(button("system-reboot-symbolic", t!("settings-snap-restart-now"))
                .on_press(on::<Snapshots>(move |_, k| restart(k, &format!("snap:{n}")))));
        }
        line(format!("{}  {}", n, snapshot_title::local_words(&s.date).unwrap_or_default())).sub(what).controls(controls)
            .key(format!("snap:{n}")).node(kit)
    }

    fn unavailable(&self, kit: &Kit) -> Node<Msg> {
        let (sub, how, why) = if self.fs == "btrfs" {
            (t!("settings-snap-not-set-up-sub").to_string(), t!("settings-snap-reinstall"),
             t!("settings-snap-not-set-up-about").to_string())
        } else {
            let fs = if self.fs.is_empty() { t!("settings-snap-another-kind").to_string() } else { self.fs.clone() };
            (t!("settings-snap-need-btrfs", fs = &fs).to_string(), t!("settings-snap-choose-btrfs"),
             t!("settings-snap-need-btrfs-about", fs = &fs))
        };
        page(kit, self.title(), self.help(), vec![
            headline(kit, t!("settings-snap-no-snapshots")),
            group(t!("settings-snap-no-snapshots-title")).icon("dialog-information-symbolic").about(why)
                .rows(vec![line(t!("settings-snap-no-snapshots-line")).sub(sub).node(kit), line(t!("settings-snap-to-have")).sub(how).node(kit),
                           self.locked_line(kit)])
                .node(kit),
            self.earlier_card(kit),
        ])
    }

    fn list_card(&self, kit: &Kit) -> Node<Msg> {
        let rows = match &self.list {
            None => vec![line(t!("settings-snap-looking-short")).node(kit)],
            Some(Err(err)) => vec![line(t!("settings-snap-not-read-line")).sub(err.clone()).node(kit)],
            Some(Ok(all)) if all.is_empty() => vec![line(t!("settings-snap-none-line"))
                .sub(t!("settings-snap-none-line-sub")).node(kit)],
            Some(Ok(all)) => super::users::banded_rows(all.iter().map(|s| self.row(kit, s)).collect()),
        };
        group(t!("settings-snap-title")).icon("document-revert-symbolic")
            .about(t!("settings-snap-list-about"))
            .span(5, 8)
            .rows(rows)
            .node(kit)
    }

    fn take_card(&self, kit: &Kit) -> Node<Msg> {
        let take = match self.booted {

            Some(_) => button("camera-photo-symbolic", t!("settings-snap-take-now")),
            None => job(kit, "take", "camera-photo-symbolic", t!("settings-snap-take-now"), on::<Snapshots>(|me, k| me.take(k))),
        };
        let mut now = vec![line(t!("settings-snap-take-now"))
            .sub(if self.booted.is_some() { t!("settings-snap-not-while") } else { t!("settings-snap-kept-until-you") })
            .control(take).key("take").node(kit), self.locked_line(kit)];

        if !self.aside.is_empty() {
            now.push(line(t!("settings-snap-set-aside", n = self.aside.len()))
                .sub(t!("settings-snap-free-space"))
                .control(confirm::<Snapshots>(kit, "aside", "user-trash-symbolic", t!("settings-snap-free-them"), |me, k| me.free(k)))
                .key("aside").node(kit));
        }
        group(t!("settings-snap-take-one")).icon("camera-photo-symbolic")
            .about(t!("settings-snap-take-about"))
            .rows(now)
            .node(kit)
    }

    fn care_card(&self, kit: &Kit) -> Option<Node<Msg>> {
        let mut care = Vec::new();
        if let Some(n) = self.booted {
            care.push(line(t!("settings-snap-started-line", n = n)).sub(t!("settings-snap-nothing-kept")).node(kit));
        }
        if !self.home_separate {
            care.push(line(t!("settings-snap-files-too")).sub(t!("settings-snap-files-back")).node(kit));
        }
        if care.is_empty() {
            return None;
        }
        Some(group(t!("settings-snap-before")).icon("dialog-warning-symbolic")
            .about(t!("settings-snap-before-about"))
            .rows(care)
            .node(kit))
    }
}

impl Page for Snapshots {
    fn id(&self) -> &'static str {
        "snapshots"
    }
    fn title(&self) -> &'static str {
        t!("settings-snap-title")
    }
    fn icon(&self) -> &'static str {
        "document-revert-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-snap-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-snap-help")
    }

    fn shown(&mut self, kit: &mut Kit) {
        self.look_at_earlier(kit);
        self.available = snapshots::available();
        self.fs = snapshots::root_fs();
        self.locked = locked_now();
        if !self.available {
            return;
        }
        self.home_separate = snapshots::home_separate();
        self.booted = snapshots::booted_snapshot();
        self.aside = snapshots::set_aside();
        self.draw(kit);
    }

    fn second(&mut self, _kit: &mut Kit) -> bool {
        let day = crate::common::clock::Local::now().format("%Y-%m-%d");
        let turned = !self.day.is_empty() && day != self.day;
        self.day = day;
        turned
    }

    fn view(&self, kit: &Kit) -> Node<Msg> {
        if !self.available {
            return self.unavailable(kit);
        }

        let mut items = vec![self.head(kit)];
        items.push(self.list_card(kit));
        items.push(self.take_card(kit));
        items.push(self.earlier_card(kit));

        items.extend(self.care_card(kit));
        page(kit, self.title(), self.help(), items)
    }
}

#[cfg(test)]
impl Snapshots {

    pub(crate) fn made_up() -> Snapshots {
        let snap = |number: u32, kind: &str, date: &str, description: &str, important: bool| Snapshot {
            number, kind: kind.into(), date: date.into(), description: description.into(), important,
        };
        Snapshots { available: true, fs: "btrfs".into(), home_separate: true, booted: None, aside: vec!["old".into()],
                    list: Some(Ok(vec![
                        snap(12, "single", "2026-10-09 18:02:11", "Taken in Settings", true),
                        snap(11, "post", "2026-10-08 09:14:40", "apt", false),
                        snap(10, "pre", "2026-10-08 09:14:03", "apt", false),
                        snap(9, "post", "2026-10-05 20:31:12", "apt", false),
                        snap(8, "pre", "2026-10-05 20:30:44", "apt", false),
                    ])),
                    restored: None, day: String::new(), earlier: made_up_earlier(), locked: true }
    }
}

#[cfg(test)]
fn made_up_earlier() -> Standing {
    use crate::common::home_snapshots::{Kept, Setting, State};
    let day = 86_400;
    let first = home_snapshots::unix_of("2026-10-03 03:10:00").unwrap_or(0);
    Standing {
        possible: Ok(()),
        setting: Setting { on: true, days: 7 },
        kept: (0..7).map(|i| Kept { number: i + 1, taken: first + i64::from(i) * day, ours: true }).collect(),
        state: State { skipped_full: None, room: Some((3_200_000_000, first + 6 * day)) },
    }
}

#[cfg(test)]
mod tests {
    use super::when;

    #[test]
    fn an_old_snapshot_says_its_day() {
        assert_eq!(when("2020-03-05 10:00:00"), "on 5 Mar");
        assert_eq!(when("rubbish"), "on rubbish");
    }

    use super::{made_up_earlier, nearly_full, Snapshots, CLEAR, EARLIER};
    use crate::common::home_snapshots::{NotHere, Setting, Standing, State};
    use crate::settings::ui::{stand::Stand, Page};

    fn said_with(earlier: Standing, then: impl FnOnce(&mut Snapshots, &mut crate::settings::ui::Kit)) -> (Vec<String>, Stand) {
        let mut stand = Stand::new(Box::new(Snapshots { earlier, ..Snapshots::made_up() }));
        let said = stand.page::<Snapshots, _>(|me, k| {
            then(me, k);
            me.view(k).said()
        });
        (said, stand)
    }

    fn holds(said: &[String], words: &str) -> bool {
        said.iter().any(|s| s == words)
    }

    fn lines_stay(said: &[String]) {
        let at = |w: &str| said.iter().position(|s| s == w).unwrap_or_else(|| panic!("{w} is missing from {said:?}"));
        let (switch, keep) = (at("switch: Keep earlier versions of your files"), at("Keep each day's copy for"));
        assert!(at("Earlier versions") < switch && switch < keep, "{said:?}");
        assert!(said[keep..].iter().any(|s| s.contains("Clear them all now") || s == "Clearing"), "{said:?}");
    }

    #[test]
    fn earlier_versions_switched_on_says_what_it_costs_and_what_it_holds() {
        let (said, stand) = said_with(made_up_earlier(), |me, k| me.say_if_full(k));
        lines_stay(&said);
        assert!(holds(&said, "Deleted files stay for a week before their room is freed."), "{said:?}");
        assert!(holds(&said, "7 copies"), "{said:?}");
        assert!(said.iter().any(|s| s.starts_with("They take 3.2 GB, measured on 9 Oct")), "{said:?}");
        assert!(holds(&said, "button: Clear them all now"));
        assert_eq!(stand.red(EARLIER), None);

        let month = Standing { setting: Setting { on: true, days: 30 }, ..made_up_earlier() };
        let (said, _) = said_with(month, |_, _| {});
        assert!(holds(&said, "Deleted files stay for a month before their room is freed."), "{said:?}");
    }

    #[test]
    fn earlier_versions_switched_off_keeps_every_line() {
        let off = Standing { setting: Setting { on: false, days: 7 }, kept: Vec::new(), state: State::default(), possible: Ok(()) };
        let (said, stand) = said_with(off, |me, k| me.say_if_full(k));
        lines_stay(&said);
        assert!(holds(&said, "No copies yet"), "{said:?}");
        assert!(holds(&said, "The first is taken within a day."), "{said:?}");
        assert_eq!(stand.red(EARLIER), None);

        let (said, _) = said_with(Standing::default(), |_, _| {});
        lines_stay(&said);
    }

    #[test]
    fn earlier_versions_not_possible_here_says_the_one_reason() {
        for why in [NotHere::PartOfSystem, NotHere::NotBtrfs("ext4".into()), NotHere::Apart, NotHere::NoSnapper] {
            let not = Standing { possible: Err(why.clone()), ..Standing::default() };
            let (said, stand) = said_with(not, |me, k| me.say_if_full(k));
            lines_stay(&said);
            assert_eq!(said.iter().filter(|s| **s == why.words()).count(), 1, "{said:?}");
            assert_eq!(stand.red(EARLIER), None);
        }

        let stale = Standing { possible: Err(NotHere::ReadOnly), setting: Setting { on: true, days: 7 },
                               state: State { skipped_full: Some(1), room: None }, kept: Vec::new() };
        let (_, stand) = said_with(stale, |me, k| me.say_if_full(k));
        assert_eq!(stand.red(EARLIER), None);
    }

    #[test]
    fn a_nearly_full_disk_is_said_in_red_under_the_switch() {
        let full = Standing { state: State { skipped_full: Some(1_791_000_000), room: None }, ..made_up_earlier() };
        let (said, stand) = said_with(full, |me, k| me.say_if_full(k));
        lines_stay(&said);
        assert_eq!(stand.red(EARLIER).as_deref(), Some(nearly_full()));
        assert!(holds(&said, nearly_full()), "{said:?}");
        assert!(holds(&said, "Their room is measured once a day."), "{said:?}");

        let off = Standing { setting: Setting { on: false, days: 7 },
                             state: State { skipped_full: Some(1), room: None }, ..made_up_earlier() };
        let (_, stand) = said_with(off, |me, k| me.say_if_full(k));
        assert_eq!(stand.red(EARLIER), None);
    }

    #[test]
    fn clearing_shows_its_ring_in_the_buttons_place() {
        let (said, _) = said_with(made_up_earlier(), |_, k| {
            k.start(CLEAR, "Clearing");
        });
        lines_stay(&said);
        assert!(holds(&said, "Clearing"), "{said:?}");
        assert!(!holds(&said, "button: Clear them all now"), "{said:?}");

        let none = Standing { kept: Vec::new(), ..made_up_earlier() };
        let (said, _) = said_with(none, |_, _| {});
        assert!(holds(&said, "button: Clear them all now"), "{said:?}");
    }

    #[test]
    fn the_system_files_switch_is_offered_only_on_btrfs() {
        let (said, _) = said_with(made_up_earlier(), |_, _| {});
        assert!(holds(&said, "switch: System files locked"), "{said:?}");
        assert!(holds(&said, "Only installs and updates unlock them."), "{said:?}");
        let mut stand = Stand::new(Box::new(Snapshots { available: false, fs: "ext4".into(), ..Snapshots::made_up() }));
        let said = stand.page::<Snapshots, _>(|me, k| me.view(k).said());
        assert!(holds(&said, "System files locked") && holds(&said, "Needs Btrfs."), "{said:?}");
        assert!(!said.iter().any(|s| s.starts_with("switch: System files")), "{said:?}");
    }

    #[test]
    fn the_keep_times_read_as_choices() {
        assert_eq!(super::KEEP_CHOICES.map(super::keep_choice), ["3 days", "A week", "Two weeks", "A month"]);
    }

    #[test]
    fn restart_now_never_does_nothing() {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: crate::settings::ui::Do| {});
        let mut kit = crate::settings::ui::Kit::new(events.clone(), asks, None, None);
        use crate::toolkit::window::{with_named, with_press};

        with_named(true, Some(7), || with_press(Some(8), || super::restart(&mut kit, "snap:3")));
        with_named(true, None, || with_press(Some(7), || super::restart(&mut kit, "snap:3")));
        assert_eq!(kit.said("snap:3"), None, "Restart now was run for a press that was not the person's own");
        with_named(true, Some(7), || with_press(Some(7), || super::restart(&mut kit, "snap:3")));
        assert_eq!(kit.said("snap:3"), Some((crate::settings::ui::no_system_bus(), true)));
    }
}
