//! Event sounds (login, logout, shutdown, charger, low battery): the ones that
//! come with Trindle, ones of your own, or off.

use std::path::PathBuf;

use crate::common::copies::Put;
use crate::common::files::{set_setting, setting};
use crate::common::run;
use crate::common::sounds::{self, MOMENTS, OFF, SOUNDS};
use crate::settings::ui::{confirm, dropdown, group, headline, line, no_more, on, page, ran, replace_rows, Clashes, Kit, Msg, Page, Raw, not_saved, no_chooser};
use crate::toolkit::bus::Listen;
use crate::toolkit::chooser::{self, Ask};
use crate::toolkit::widget::{button, slider, Length, Node};
use std::time::Duration;
use crate::toolkit::t;

const ADD_KEY: &str = "sounds:add";

const ASK_KEY: &str = "sounds:replace";

const VOLUME_SETTLE_MS: u64 = 400;

#[derive(Default)]
pub struct Sounds {

    volumes: Vec<u32>,

    moved: u32,

    own: Vec<(String, String)>,

    asking: Option<Listen>,

    taken: Clashes,
}

fn options(own: &[(String, String)]) -> Vec<(String, String)> {
    let mut out = vec![(OFF.to_string(), t!("settings-sounds-off").to_string())];
    out.extend(SOUNDS.iter().map(|id| (id.to_string(), sounds::sound_name(id).to_string())));
    out.extend(own.iter().cloned());
    out
}

type Copied = (String, std::io::Result<Put>);

fn copied_in(files: &[PathBuf], dir: &std::path::Path) -> Vec<Copied> {
    files.iter().filter_map(|path| {
        let named = path.file_name()?;
        Some((named.to_string_lossy().to_string(), crate::common::copies::copy_in(path, dir, named)))
    }).collect()
}

fn volume_key(group: &str) -> String {
    format!("sounds:volume:{group}")
}

fn volume_line(kit: &Kit, at: usize, title: &'static str, percent: u32) -> Node<Msg> {
    let group = sounds::VOLUMES[at].0;
    line(title).sub(format!("{percent}%")).key(volume_key(group))
        .control(slider(percent as f32, 0.0, sounds::VOLUME_FULL as f32, sounds::VOLUME_STEP as f32, title, move |v| {
            on::<Sounds>(move |me, k| me.louder(k, at, v))
        }).width(Length::Fixed(220.0)))
        .node(kit)
}

fn moment_key(moment: &str) -> String {
    format!("sounds:{moment}")
}

fn hear(kit: &mut Kit, key: &str, id: &str, percent: u32) {
    let Some(path) = sounds::file(id) else {
        kit.say(key, t!("settings-sounds-gone"), false, false);
        return;
    };
    play(kit, key, &path, percent);
}

fn play(kit: &mut Kit, key: &str, path: &std::path::Path, percent: u32) {
    let back = kit.settles_quietly::<Sounds, (), Raw>(key, t!("settings-sounds-would-not-play"), no_more);
    let args = sounds::command(path, percent);
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    run::capture(&args, None, move |ok, _, err| back(ran(ok, &err)));
}

impl Sounds {

    fn top(&self, kit: &Kit) -> Node<Msg> {
        let total = MOMENTS.len();
        let on = MOMENTS.iter().filter(|(m, _)| sounds::chosen(m).is_some()).count();
        let said = match on {
            0 => t!("settings-sounds-top-none").to_string(),
            n if n == total => t!("settings-sounds-top-all").to_string(),
            n => t!("settings-sounds-top-some", n = n, total = total),
        };
        headline(kit, said)
    }

    fn choose(&mut self, kit: &mut Kit, moment: &str, id: &str) {
        let key = moment_key(moment);
        if kit.settle(&key, set_setting(&format!("sound_{moment}"), id), not_saved()).is_some() && id != OFF {
            hear(kit, &key, id, sounds::volume_of(moment));
        }
    }

    fn louder(&mut self, kit: &mut Kit, at: usize, v: f32) {
        let (group, moments) = sounds::VOLUMES[at];
        let key = volume_key(group);
        let percent = sounds::clamp_volume(v.round() as i64);
        if kit.settle(&key, set_setting(&sounds::volume_key(group), &percent.to_string()), not_saved()).is_none() {
            return;
        }
        self.volumes[at] = percent;
        self.moved += 1;
        let round = self.moved;
        kit.after::<Sounds>(Duration::from_millis(VOLUME_SETTLE_MS), move |me, k| {
            if me.moved != round {
                return;
            }
            if let Some(id) = moments.iter().find_map(|m| sounds::chosen(m)) {
                hear(k, &key, &id, percent);
            }
        });
    }

    fn ask(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.session_for(ADD_KEY) else { return };
        let globs = ["*.wav", "*.ogg", "*.oga", "*.opus", "*.flac", "*.mp3"].iter().map(|g| g.to_string()).collect();
        let ask = Ask { title: t!("settings-sounds-chooser-title").into(), filters: vec![(t!("settings-sounds-chooser-filter").into(), globs)], ..Ask::default() };
        let back = kit.then::<Sounds, Vec<PathBuf>>(|me, k, paths| {
            me.asking = None;
            if !paths.is_empty() {
                me.add(k, &paths);
            } else {
                k.unanswered(ADD_KEY);
            }
        });
        self.asking = chooser::open_many(&bus, &kit.events, ask, back);
        kit.started(ADD_KEY, self.asking.is_some(), no_chooser());
    }

    fn add(&mut self, kit: &mut Kit, files: &[PathBuf]) {
        let dir = sounds::own_dir();
        if kit.settle_quietly(ADD_KEY, std::fs::create_dir_all(&dir), t!("settings-sounds-folder-not-made")).is_none() {
            return;
        }
        let mut wanted = Vec::new();
        for path in files {
            let Some(named) = path.file_name() else { continue };
            let name = named.to_string_lossy().to_string();
            if !sounds::playable(&name) {
                kit.say(ADD_KEY, &t!("settings-sounds-not-playable", name = &name), false, false);
                continue;
            }
            wanted.push(path.clone());
        }
        if wanted.is_empty() {
            return;
        }
        let back = kit.then::<Sounds, Vec<Copied>>(|me, kit, copied| me.added(kit, copied));
        run::on_thread(move || copied_in(&wanted, &dir), back);
    }

    fn added(&mut self, kit: &mut Kit, copied: Vec<Copied>) {
        let added = newly(&copied);
        let mut asked = Vec::new();
        for (name, result) in copied {
            if let Some(Put::Taken(t)) = kit.settle_quietly(ADD_KEY, result, &t!("settings-sounds-not-copied", name = &name)) {
                asked.push(t);
            }
        }
        self.taken.add_batch(asked);
        if added > 0 {
            kit.say(ADD_KEY, &t!("settings-sounds-added", n = added), true, false);
            self.own = sounds::own();
        }
    }

    fn skip(&mut self) {
        self.taken.answer();
    }

    fn replace(&mut self, kit: &mut Kit) {
        let covered = self.taken.answer();
        if covered.is_empty() {
            return;
        }
        let back = kit.then::<Sounds, Vec<(String, std::io::Result<()>)>>(|me, k, done| {
            let total = done.len();
            let mut mended = Vec::new();
            for (name, result) in done {
                if k.settle_quietly(ASK_KEY, result, &t!("settings-sounds-not-replaced", name = &name)).is_some() {
                    mended.push(name);
                }
            }
            if let [one] = mended.as_slice() {
                k.say(ASK_KEY, &t!("settings-sounds-replaced-one", name = one), true, false);
            } else if !mended.is_empty() {
                k.say(ASK_KEY, &t!("settings-sounds-replaced-some", n = mended.len(), total = total), true, false);
            }
            me.own = sounds::own();
        });
        run::on_thread(move || covered.iter().map(|t| (t.name.clone(), crate::common::copies::replace(t))).collect(), back);
    }

    fn remove(&mut self, kit: &mut Kit, id: &str) {
        let Some(name) = id.strip_prefix("own:") else { return };
        let key = format!("sounds:{id}");
        if kit.settle_quietly(&key, std::fs::remove_file(sounds::own_dir().join(name)), t!("settings-sounds-not-removed")).is_none() {
            return;
        }

        for &(moment, _) in MOMENTS {
            if setting(&format!("sound_{moment}")) == id {
                kit.settle_quietly(&moment_key(moment), set_setting(&format!("sound_{moment}"), ""),
                                   t!("settings-sounds-not-set-back"));
            }
        }
        self.own = sounds::own();
    }

    fn moment_rows(&self, kit: &Kit) -> (Vec<Node<Msg>>, Vec<Node<Msg>>) {
        let choices = options(&self.own);
        let names: Vec<String> = choices.iter().map(|(_, name)| name.clone()).collect();
        let moment_line = |moment: &'static str, title: &'static str| {
            let key = moment_key(moment);
            let current = sounds::chosen(moment).unwrap_or_else(|| OFF.to_string());
            let at = choices.iter().position(|(id, _)| *id == current);
            let ids: Vec<String> = choices.iter().map(|(id, _)| id.clone()).collect();
            let pick = dropdown(kit, &key, names.clone(), at, title, move |i| {
                let id = ids[i].clone();
                on::<Sounds>(move |me, k| me.choose(k, moment, &id))
            });
            let heard_under = key.clone();
            let hear = button("audio-volume-high-symbolic", t!("settings-sounds-hear")).on_press(on::<Sounds>(move |_, k| {
                if let Some(id) = sounds::chosen(moment) {
                    hear(k, &heard_under, &id, sounds::volume_of(moment));
                }
            }));
            let (name, sub) = moment_words(moment, title);
            line(name).sub(sub).key(key).control(pick).control(hear).node(kit)
        };
        let (power, rest): (Vec<_>, Vec<_>) = MOMENTS.iter().partition(|(m, _)| POWER.contains(m));
        let shown = |at: usize| self.volumes.get(at).copied().unwrap_or_else(|| sounds::volume(sounds::VOLUMES[at].0));
        let at = |group: &str| sounds::VOLUMES.iter().position(|v| v.0 == group).unwrap_or(0);
        let mut coming: Vec<Node<Msg>> = Vec::new();
        for &&(m, _) in &rest {
            if m == "notification" {
                coming.push(volume_line(kit, at("coming"), t!("settings-sounds-volume-coming"), shown(at("coming"))));
            }
            coming.push(moment_line(m, sounds::moment_label(m)));
        }
        coming.push(volume_line(kit, at("notification"), t!("settings-sounds-volume-notifications"), shown(at("notification"))));
        let mut power: Vec<Node<Msg>> = power.iter().map(|&&(m, _)| moment_line(m, sounds::moment_label(m))).collect();
        power.push(volume_line(kit, at("power"), t!("settings-sounds-volume-power"), shown(at("power"))));
        (coming, power)
    }

    fn own_rows(&self, kit: &Kit) -> Vec<Node<Msg>> {
        let add = button("list-add-symbolic", t!("settings-sounds-add")).on_press(on::<Sounds>(|me, k| me.ask(k)));
        let folder = button("folder-symbolic", t!("settings-sounds-open-folder")).on_press(on::<Sounds>(|_, k| {
            let dir = sounds::own_dir();
            if k.settle_quietly(ADD_KEY, std::fs::create_dir_all(&dir), t!("settings-sounds-folder-not-made")).is_some() {
                let opened = run::spawn(&["xdg-open", &dir.to_string_lossy()]);
                k.started(ADD_KEY, opened, t!("settings-sounds-folder-not-opened"));
            }
        }));
        let mut mine = Vec::new();
        if let Some(t) = self.taken.first() {
            let (a, b) = (t.from.clone(), t.there.clone());
            let hear_new = button("audio-volume-high-symbolic", t!("settings-sounds-hear-new"))
                .on_press(on::<Sounds>(move |_, k| play(k, ASK_KEY, &a, sounds::VOLUME_FULL)));
            let hear_old = button("audio-volume-high-symbolic", t!("settings-sounds-hear-old"))
                .on_press(on::<Sounds>(move |_, k| play(k, ASK_KEY, &b, sounds::VOLUME_FULL)));
            mine.extend(replace_rows(kit, ASK_KEY, &self.taken, vec![hear_new], vec![hear_old],
                                     on::<Sounds>(|me, _| me.skip()), on::<Sounds>(|me, k| me.replace(k)),
                                     |all| on::<Sounds>(move |me, _| me.taken.set_same(all))));
        }
        mine.push(line(t!("settings-sounds-add-heading"))
            .sub(t!("settings-sounds-add-formats"))
            .key(ADD_KEY).control(add).control(folder).node(kit));
        for (id, name) in &self.own {
            let key = format!("sounds:{id}");
            let playing = id.clone();
            let heard_under = key.clone();
            let hear = button("audio-volume-high-symbolic", t!("settings-sounds-hear"))
                .on_press(on::<Sounds>(move |_, k| hear(k, &heard_under, &playing, sounds::VOLUME_FULL)));
            let gone = id.clone();
            let remove = confirm::<Sounds>(kit, &key, "user-trash-symbolic", t!("settings-sounds-remove"), move |me, k| me.remove(k, &gone));
            mine.push(line(name.clone()).key(key).control(hear).control(remove).node(kit));
        }

        if self.own.is_empty() {
            mine.push(line(t!("settings-sounds-none-yet")).sub(t!("settings-sounds-none-yet-about")).node(kit));
        }
        mine
    }
}

impl Page for Sounds {
    fn id(&self) -> &'static str {
        "sounds"
    }
    fn title(&self) -> &'static str {
        t!("settings-sounds-title")
    }
    fn icon(&self) -> &'static str {
        "audio-x-generic-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-sounds-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-sounds-help")
    }
    fn shown(&mut self, _kit: &mut Kit) {
        self.own = sounds::own();
        self.volumes = sounds::VOLUMES.iter().map(|(g, _)| sounds::volume(g)).collect();
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let (coming, power) = self.moment_rows(kit);
        let mine = self.own_rows(kit);
        page(kit, self.title(), self.help(), vec![
            self.top(kit),
            group(t!("settings-sounds-coming-title")).icon("system-log-out-symbolic")
                .about(t!("settings-sounds-coming-about"))
                .rows(coming)
                .span(3, 4)
                .node(kit),
            group(t!("settings-sounds-power-title")).icon("battery-full-charging-symbolic")
                .about(t!("settings-sounds-power-about"))
                .rows(power)
                .span(3, 4)
                .node(kit),
            group(t!("settings-sounds-own-title")).icon("folder-symbolic")
                .about(t!("settings-sounds-own-about"))
                .rows(mine)
                .span(3, 4)
                .node(kit),
        ])
    }
}

fn newly(copied: &[Copied]) -> usize {
    copied.iter().filter(|(_, got)| matches!(got, Ok(Put::Made(_)))).count()
}

fn moment_words(moment: &str, title: &'static str) -> (&'static str, &'static str) {
    match moment {
        "login" => (title, t!("settings-sounds-moment-login-about")),
        "logout" => (t!("settings-sounds-moment-logout-short"), t!("settings-sounds-moment-logout-about")),
        "plug" => (title, t!("settings-sounds-moment-plug-about")),
        "unplug" => (title, t!("settings-sounds-moment-unplug-about")),
        "battery" => (title, t!("settings-sounds-moment-battery-about")),
        _ => (t!("settings-sounds-moment-notification-short"), t!("settings-sounds-moment-notification-about")),
    }
}

const POWER: [&str; 3] = ["plug", "unplug", "battery"];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chosen_sounds_are_copied_in_by_a_function_of_its_own() {
        let dir = std::env::temp_dir().join(format!("trindle-sounds-in-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let (from, to) = (dir.join("from"), dir.join("to"));
        std::fs::create_dir_all(&from).unwrap();
        std::fs::create_dir_all(&to).unwrap();
        std::fs::write(from.join("bell.wav"), b"ring").unwrap();
        let copied = copied_in(&[from.join("bell.wav"), from.join("gone.wav")], &to);
        assert_eq!(copied.iter().map(|(name, r)| (name.as_str(), r.is_ok())).collect::<Vec<_>>(), [("bell.wav", true), ("gone.wav", false)]);
        assert_eq!(std::fs::read(to.join("bell.wav")).unwrap(), b"ring");
        std::fs::remove_dir_all(&dir).unwrap();
        let code = crate::common::source::code(include_str!("sounds.rs"));
        let code = code.split("\n#[cfg(test)]").next().unwrap();
        assert_eq!(code.matches("copy_in(").count(), 1, "a sound is copied in from a second place");
        assert!(code.contains("run::on_thread(move || copied_in(&wanted, &dir), back);"));
    }

    #[test]
    fn the_switch_covers_the_rest_of_a_batch_of_sounds() {
        use crate::settings::ui::stand::Stand;
        use crate::settings::ui::stand::made_up_taken;
        let mut s = Stand::new(Box::new(Sounds::default()));
        let left = |s: &mut Stand| s.page::<Sounds, _>(|p, _| p.taken.more() + usize::from(!p.taken.is_empty()));
        s.page::<Sounds, _>(|p, _| p.taken.add_batch(["a.wav", "b.wav", "c.wav"].map(made_up_taken)));
        s.page::<Sounds, _>(|p, _| p.skip());
        assert_eq!(left(&mut s), 2, "one answer with the switch off covered more than one sound");
        s.page::<Sounds, _>(|p, _| {
            p.taken.set_same(true);
            p.skip();
        });
        assert_eq!(left(&mut s), 0, "the switch did not cover the rest of the batch");
    }

    #[test]
    #[ignore]
    fn shoot_the_question_with_the_switch() {
        use crate::settings::ui::stand::{made_up_taken, shoot_both};
        let mut page = Sounds::default();
        page.taken.add_batch(["bell.wav", "chime.ogg", "ping.flac", "drum.mp3"].map(made_up_taken));
        shoot_both(Box::new(page), "376-sounds", (1280.0, 900.0));
    }

    #[test]
    fn a_sound_that_is_already_there_is_not_counted_as_added() {
        let dir = std::env::temp_dir().join(format!("trindle-settings-sounds-newly-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("own")).unwrap();
        let files = [dir.join("ping.wav")];
        std::fs::write(&files[0], b"RIFF a ping").unwrap();
        assert_eq!(newly(&copied_in(&files, &dir.join("own"))), 1);

        assert_eq!(newly(&copied_in(&files, &dir.join("own"))), 0, "a sound that was there all along was counted as added");
        assert_eq!(std::fs::read_dir(dir.join("own")).unwrap().count(), 1);

        std::fs::write(&files[0], b"RIFF another ping").unwrap();
        let asked = copied_in(&files, &dir.join("own"));
        assert_eq!(newly(&asked), 0);
        assert!(matches!(asked[0].1, Ok(Put::Taken(_))), "{:?}", asked[0].1);
        assert_eq!(std::fs::read_dir(dir.join("own")).unwrap().count(), 1);
        assert_eq!(std::fs::read(dir.join("own/ping.wav")).unwrap(), b"RIFF a ping", "the sound there was written over unasked");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
