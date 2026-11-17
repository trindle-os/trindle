//! Apps started at login, each with an optional delay; can be disabled without removal.

use std::path::{Path, PathBuf};

use crate::common::apps;
use crate::common::autostart::{self, DELAYS};
use crate::settings::ui::{app_icon, confirm, dropdown, group, headline, line, on, page, Kit, Msg, Page, not_saved, no_chooser};
use crate::toolkit::bus::Listen;
use crate::toolkit::chooser::{self, Ask};
use crate::toolkit::widget::{button, column, entry, row, scroll, switch, Length, Node};
use crate::toolkit::t;

fn not_added() -> &'static str {
    t!("settings-startup-not-added")
}

fn delay_label(s: u32) -> String {
    match s {
        0 => t!("settings-startup-delay-0").into(),
        60 => t!("settings-startup-delay-60").into(),
        120 => t!("settings-startup-delay-120").into(),
        s => t!("settings-startup-delay-seconds", n = s),
    }
}

fn desktop_file(id: &str) -> Option<PathBuf> {
    fn look(base: &Path, dir: &Path, id: &str) -> Option<PathBuf> {
        for e in std::fs::read_dir(dir).ok()?.flatten() {
            let p = e.path();
            if p.is_dir() {
                if let Some(found) = look(base, &p, id) {
                    return Some(found);
                }
            } else if p.strip_prefix(base).ok().is_some_and(|rel| rel.to_string_lossy().replace('/', "-") == id) {
                return Some(p);
            }
        }
        None
    }
    apps::app_dirs().iter().find_map(|base| look(base, base, id))
}

fn cut(words: &str, most: usize) -> String {
    if words.chars().count() <= most {
        return words.to_string();
    }
    let kept: String = words.chars().take(most - 1).collect();
    format!("{}\u{2026}", kept.trim_end())
}

fn with_icon(picture: Node<Msg>, line: Node<Msg>) -> Node<Msg> {
    row(vec![picture, line.width(Length::Fill)]).gap(12.0).width(Length::Fill)
}

#[derive(Default, PartialEq)]
enum Adding {
    #[default]
    Nothing,
    Apps,
    Command,
}

#[derive(Default)]
pub struct Startup {
    entries: Vec<autostart::Entry>,
    adding: Adding,

    apps: Vec<apps::Entry>,
    find: String,
    command: String,

    chooser: Option<Listen>,

    turn: u32,
}

impl Startup {
    fn read(&mut self) {
        let icons = crate::common::apps::shared_icons();
        self.entries = autostart::list();
        for e in &mut self.entries {
            if !e.icon.is_empty() && icons.find(&e.icon, 32, 1).is_none() {
                e.icon.clear();
            }
        }
    }

    fn close_adding(&mut self) {
        self.adding = Adding::Nothing;
        self.find.clear();
        self.command.clear();
    }

    fn top(&self) -> Node<Msg> {
        let on: Vec<&autostart::Entry> = self.entries.iter().filter(|e| e.enabled).collect();
        let off = self.entries.len() - on.len();
        let mut words = match on.as_slice() {
            [] => t!("settings-startup-top-none").to_string(),
            [one] => t!("settings-startup-top-one", name = &one.name),
            many => t!("settings-startup-top-many", n = many.len()),
        };
        match off {
            0 => {}
            n => {
                words.push(' ');
                words.push_str(&t!("settings-startup-top-off", n = n));
            }
        }
        headline(words)
    }

    fn row(&self, kit: &Kit, e: &autostart::Entry) -> Node<Msg> {
        let key = format!("start:{}", e.path.display());
        let icon = if e.icon.is_empty() { crate::common::apps::generic_icon_for(&e.name, &e.name) } else { e.icon.clone() };
        let (p, k) = (e.path.clone(), key.clone());
        let starts = switch(e.enabled, t!("settings-startup-switch"), move |want| {
            let (p, k) = (p.clone(), k.clone());
            on::<Startup>(move |me, kit| {
                kit.settle(&k, autostart::set_enabled(&p, want), not_saved());
                me.read();
            })
        });

        let (sub, controls) = if e.system {
            (t!("settings-startup-system", command = cut(&e.exec, 30)), vec![starts])
        } else {
            (cut(&e.exec, 40), vec![self.delay_list(e, &key), starts, self.stop_button(e, &key)])
        };
        with_icon(app_icon(vec![icon], 28.0), line(e.name.clone()).sub(sub).controls(controls).key(key).node(kit))
    }

    fn delay_list(&self, e: &autostart::Entry, key: &str) -> Node<Msg> {
        let chosen = DELAYS.iter().position(|d| *d == e.delay);
        let options = DELAYS.iter().map(|d| delay_label(*d)).collect();
        let (p, k) = (e.path.clone(), key.to_string());
        dropdown(&format!("delay:{}", e.path.display()), options, chosen, &t!("settings-startup-delay-for", name = &e.name), move |i| {
            let (p, k, d) = (p.clone(), k.clone(), DELAYS[i]);
            on::<Startup>(move |me, kit| {
                kit.settle(&k, autostart::set_delay(&p, d), not_saved());

                me.read();
            })
        })
    }

    fn stop_button(&self, e: &autostart::Entry, key: &str) -> Node<Msg> {
        let p = e.path.clone();
        confirm::<Startup>(&format!("stop:{}", e.path.display()), "user-trash-symbolic", t!("settings-startup-stop"), {
            let k = key.to_string();
            move |me, kit| {

                kit.settle_quietly(&k, autostart::remove(&p), t!("settings-startup-not-removed"));
                me.read();
            }
        })
    }

    fn app_picker(&self, kit: &Kit) -> Vec<Node<Msg>> {
        let find = entry(self.find.clone(), t!("settings-startup-find"), |t| on::<Startup>(move |me, _| me.find = t.clone()))
            .grab(self.turn);
        let words = self.find.to_lowercase();
        let rows = self.apps.iter().filter(|a| a.name.to_lowercase().contains(&words)).map(|a| {
            let key = format!("app:{}", a.id);
            let (id, k) = (a.id.clone(), key.clone());
            let add = button("list-add-symbolic", t!("settings-startup-add-this")).on_press(on::<Startup>(move |me, kit| {
                let Some(path) = desktop_file(&id) else {
                    kit.say(&k, t!("settings-startup-gone"), false, false);
                    return;
                };
                if kit.settle_quietly(&k, autostart::add_app(&path, 0), not_added()).is_some() {
                    me.close_adding();
                    me.read();
                }
            }));
            with_icon(app_icon(a.icons(), 24.0), line(a.name.clone()).control(add).key(key).node(kit))
        }).collect();
        vec![find, scroll("startup:adding", column(rows)).height(Length::Fixed(280.0))]
    }

    fn command_line(&self, kit: &Kit) -> Node<Msg> {
        let typed = entry(self.command.clone(), t!("settings-startup-command-example"),
                          |t| on::<Startup>(move |me, _| me.command = t.clone()))
            .on_submit(on::<Startup>(|me, kit| me.add_command(kit)))
            .grab(self.turn);
        let browse = button("document-open-symbolic", t!("settings-startup-browse"))
            .on_press(on::<Startup>(|me, kit| me.choose_program(kit)));
        let add = button("list-add-symbolic", t!("settings-startup-add-this")).on_press(on::<Startup>(|me, kit| me.add_command(kit)));
        line(t!("settings-startup-command"))
            .controls(vec![browse, add]).below(typed).key("command").node(kit)
    }

    fn add_command(&mut self, kit: &mut Kit) {
        let command = self.command.trim().to_string();
        if command.is_empty() {
            kit.say("command", t!("settings-startup-command-empty"), false, false);
            return;
        }
        if kit.settle_quietly("command", autostart::add_command(&command, 0), not_added()).is_some() {
            self.close_adding();
            self.read();
        }
    }

    fn choose_program(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.session_for("command") else { return };
        let ask = Ask { title: t!("settings-startup-chooser-title").into(), ..Default::default() };
        let chosen = kit.then::<Startup, Option<PathBuf>>(|me, k, picked| {
            me.chooser = None;
            if picked.is_none() {
                k.unanswered("command");
            }
            if let Some(path) = picked {
                let p = path.display().to_string();
                if !p.contains(['\n', '\r']) {
                    me.command = quoted_exec(&p);
                }
            }
        });
        self.chooser = chooser::open(&bus, &kit.events, ask, chosen);
        kit.started("command", self.chooser.is_some(), no_chooser());
    }
}

fn quoted_exec(path: &str) -> String {
    let mut out = String::from("\"");
    for c in path.chars() {
        match c {
            '"' | '`' | '$' | '\\' => {
                out.push('\\');
                out.push(c);
            }
            '%' => out.push_str("%%"),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

impl Page for Startup {
    fn id(&self) -> &'static str {
        "startup"
    }
    fn title(&self) -> &'static str {
        t!("settings-startup-title")
    }
    fn icon(&self) -> &'static str {
        "system-run-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-startup-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-startup-help")
    }

    fn shown(&mut self, _kit: &mut Kit) {
        self.read();
    }

    fn view(&self, kit: &Kit) -> Node<Msg> {
        let mut items = vec![self.top()];
        let rows: Vec<Node<Msg>> = crate::toolkit::widget::banded_list(self.entries.iter().map(|e| self.row(kit, e)).collect());

        let mut about = t!("settings-startup-about")
            .to_string();
        for e in self.entries.iter().filter(|e| e.exec.chars().count() > 40) {
            about.push(' ');
            about.push_str(&t!("settings-startup-runs", name = &e.name, command = &e.exec));
        }

        let rows = if rows.is_empty() {
            vec![line(t!("settings-startup-none-title")).node(kit)]
        } else {
            rows
        };
        items.push(group(t!("settings-startup-list-title")).icon("system-run-symbolic").about(about).span(5, 8).rows(rows).grows().node(kit));
        let open_apps = button("view-app-grid-symbolic", t!("settings-startup-open-apps")).on(self.adding == Adding::Apps)
            .on_press(on::<Startup>(|me, _| {
                me.apps = apps::entries();
                me.find.clear();
                me.adding = Adding::Apps;
                me.turn += 1;
            }));
        let open_command = button("utilities-terminal-symbolic", t!("settings-startup-open-command"))
            .on(self.adding == Adding::Command)
            .on_press(on::<Startup>(|me, _| {
                me.adding = Adding::Command;
                me.turn += 1;
            }));
        let mut add = Vec::new();
        add.push(line(t!("settings-startup-app-heading")).control(open_apps).node(kit));
        if self.adding == Adding::Apps {
            add.extend(self.app_picker(kit));
        }
        add.push(line(t!("settings-startup-cmd-heading")).control(open_command).node(kit));
        if self.adding == Adding::Command {
            add.push(self.command_line(kit));
        }
        let adding = t!("settings-startup-add-about");
        items.push(group(t!("settings-startup-add-title")).icon("list-add-symbolic").about(adding).rows(add).node(kit));

        page(kit, self.title(), self.help(), items)
    }
}

#[cfg(test)]
impl Startup {

    pub(crate) fn flipped() -> Vec<Startup> {
        let none = Startup { entries: Vec::new(), ..Startup::made_up() };
        let mut one = Startup::made_up();
        one.entries.truncate(1);
        vec![Startup::made_up(), none, one]
    }

    pub(crate) fn made_up() -> Startup {
        let entry = |name: &str, exec: &str, delay: u32, enabled: bool| autostart::Entry {
            path: PathBuf::from(format!("/made-up/{name}.desktop")), name: name.into(), icon: String::new(), exec: exec.into(),
            delay, enabled, system: false,
        };
        Startup { entries: vec![entry("Notes", "notes --hidden", 0, true), entry("Sync", "sync --no-browser", 30, true),
                                entry("Chat", "chat", 10, false),
                                autostart::Entry { system: true, ..entry("Keyring", "gnome-keyring-daemon --start", 0, true) }],
                  ..Startup::default() }
    }
}

#[cfg(test)]
mod quoting_tests {
    use super::quoted_exec;
    use crate::common::keyfile::command_words;

    #[test]
    fn a_path_with_odd_characters_stays_one_word_of_the_command() {
        for p in ["/usr/bin/plain", "/home/john/My Games/run", "/home/john/it's/run", "/home/john/a\"b/run", "/home/john/$HOME/run", "/home/john/`id`/run", "/home/john/back\\slash/run", "/home/john/tab\there/run", "/home/john/100%/run"] {
            let line = quoted_exec(p);
            let want = p.replace("%%", "%");
            assert_eq!(command_words(&line.replace("%%", "%")), Some(vec![want.clone()]), "{p} became {line}");
        }
    }
}
