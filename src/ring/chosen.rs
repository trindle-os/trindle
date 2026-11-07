//! Which tray icons the ring's centre row shows.

use std::cell::RefCell;
use std::path::Path;

use crate::common::files::{read_env, trindle_conf, update_env};
use crate::toolkit::t;

use super::ctx::msg;
use super::sni::menu::{menu_line_in, menu_words, Part};

pub const MOST: usize = 4;
const KEY: &str = "ring_tray_shown";
const NONE: &str = "none";

pub fn plugin_id(id: &str) -> String {
    format!("plugin:{}", clean(id))
}

pub fn program_id(id: &str, service: &str, path: &str) -> String {
    let own = if !id.is_empty() { id } else if !service.starts_with(':') { service } else { path };
    format!("program:{}", clean(own))
}

fn clean(id: &str) -> String {
    id.chars().map(|c| if c == ',' || c.is_control() { '_' } else { c }).take(120).collect()
}

#[derive(Default, Debug, PartialEq)]
pub struct Choice {
    chosen: Option<Vec<String>>,
    appeared: Vec<String>,
}

impl Choice {
    pub fn read(saved: &str) -> Choice {
        let chosen = match saved {
            "" => None,
            NONE => Some(Vec::new()),
            list => Some(list.split(',').filter(|i| !i.is_empty()).map(String::from).collect()),
        };
        Choice { chosen, appeared: Vec::new() }
    }

    fn text(&self) -> Option<String> {
        self.chosen.as_ref().map(|c| if c.is_empty() { NONE.to_string() } else { c.join(",") })
    }

    pub fn shown(&mut self, present: &[String]) -> Vec<String> {
        for id in present {
            if !self.appeared.contains(id) {
                self.appeared.push(id.clone());
            }
        }
        let mut order: Vec<&String> = present.iter().collect();
        order.sort_by_key(|id| self.appeared.iter().position(|a| a == *id));
        order.into_iter().filter(|id| self.chosen.as_ref().is_none_or(|c| c.contains(*id))).take(MOST).cloned().collect()
    }

    pub fn settle(&mut self, present: &[String]) -> bool {
        let now = self.shown(present);
        if self.chosen.is_none() && now.len() == MOST {
            self.chosen = Some(now);
            return true;
        }
        false
    }

    pub fn show(&mut self, id: &str, present: &[String]) -> bool {
        let mut now = self.shown(present);
        if now.iter().any(|i| i == id) {
            return true;
        }
        if now.len() >= MOST {
            return false;
        }
        now.push(id.to_string());
        self.chosen = Some(now);
        true
    }

    pub fn take_out(&mut self, id: &str, present: &[String]) {
        let mut now = self.shown(present);
        now.retain(|i| i != id);
        self.chosen = Some(now);
    }

    pub fn load_from(file: &Path) -> Choice {
        Choice::read(read_env(file).get(KEY).map_or("", String::as_str))
    }

    pub fn save_to(&self, file: &Path) -> std::io::Result<()> {
        match self.text() {
            Some(text) => update_env(file, &[(KEY, Some(&text))]),
            None => Ok(()),
        }
    }
}

thread_local! {
    static CHOICE: RefCell<Choice> = RefCell::new(Choice::default());
}

pub fn start() {
    CHOICE.with(|c| *c.borrow_mut() = Choice::load_from(&trindle_conf()));
}

fn save(c: &Choice) {
    let _ = c.save_to(&trindle_conf());
}

fn present() -> Vec<String> {
    super::sni::tray().map(|t| t.ids()).unwrap_or_default()
}

pub fn shown() -> Vec<String> {
    let present = present();
    CHOICE.with(|c| c.borrow_mut().shown(&present))
}

pub fn settle() {
    let present = present();
    CHOICE.with(|c| {
        let mut c = c.borrow_mut();
        if c.settle(&present) {
            save(&c);
        }
    });
}

pub(super) fn menu_part(id: &str, aligned: bool) -> Part {
    let id = id.to_string();
    let present = present();
    let (in_row, room) = CHOICE.with(|c| {
        let now = c.borrow_mut().shown(&present);
        (now.contains(&id), now.len() < MOST)
    });
    if in_row {
        return menu_line_in(t!("ring-tray-take-out").into(), aligned, msg(move |f| {
            CHOICE.with(|c| {
                let mut c = c.borrow_mut();
                c.take_out(&id, &present);
                save(&c);
            });
            f.close_beside();
            f.quick.hide();
        }));
    }
    if !room {
        return menu_words(t!("ring-tray-full").into());
    }
    menu_line_in(t!("ring-tray-show").into(), aligned, msg(move |f| {
        CHOICE.with(|c| {
            let mut c = c.borrow_mut();
            if c.show(&id, &present) {
                save(&c);
            }
        });
        f.close_beside();
        f.quick.hide();
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(n: usize) -> Vec<String> {
        (0..n).map(|i| program_id(&format!("app{i}"), ":1.5", "/x")).collect()
    }

    fn scratch(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-chosen-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn an_icons_id_is_its_own_and_does_not_change_between_runs() {
        assert_eq!(program_id("nm-applet", ":1.42", "/StatusNotifierItem"), program_id("nm-applet", ":1.97", "/StatusNotifierItem"));
        assert_eq!(program_id("", "org.example.Flow", "/a"), "program:org.example.Flow");
        assert_eq!(program_id("", ":1.42", "/MenuBar"), "program:/MenuBar");
        assert_ne!(program_id("a", "s", "/p"), plugin_id("a"));
        assert!(!plugin_id("org.example,One\n").contains([',', '\n']));
    }

    #[test]
    fn with_nothing_chosen_the_first_four_to_appear_are_shown_and_stay() {
        let mut c = Choice::default();
        let all = ids(6);
        assert_eq!(c.shown(&all[..2]), all[..2]);
        assert_eq!(c.shown(&all), all[..4]);
        let without: Vec<String> = all.iter().filter(|i| **i != all[1]).cloned().collect();
        assert_eq!(c.shown(&without), [all[0].clone(), all[2].clone(), all[3].clone(), all[4].clone()]);
        assert_eq!(c.shown(&all), all[..4]);
        assert!(c.settle(&all));
        assert!(!c.settle(&all));
        assert_eq!(c.shown(&all), all[..4]);
    }

    #[test]
    fn never_more_than_four_and_a_fifth_waits_until_one_is_taken_out() {
        let mut c = Choice::default();
        let all = ids(6);
        assert_eq!(c.shown(&all).len(), MOST);
        assert!(!c.show(&all[5], &all));
        assert_eq!(c.shown(&all), all[..4]);
        c.take_out(&all[0], &all);
        assert_eq!(c.shown(&all), all[1..4]);
        assert!(c.show(&all[5], &all));
        assert_eq!(c.shown(&all), [all[1].clone(), all[2].clone(), all[3].clone(), all[5].clone()]);
        assert!(c.show(&all[1], &all));
        assert_eq!(c.shown(&all).len(), MOST);
    }

    #[test]
    fn every_icon_can_be_taken_out() {
        let mut c = Choice::default();
        let all = ids(2);
        for i in &all {
            c.take_out(i, &all);
        }
        assert!(c.shown(&all).is_empty());
        assert_eq!(c.text().as_deref(), Some("none"));
        assert_eq!(Choice::read("none").shown(&all), Vec::<String>::new());
    }

    #[test]
    fn the_choice_survives_a_restart() {
        let dir = scratch("restart");
        let file = dir.join("trindle.conf");
        std::fs::write(&file, "clock_format=%H:%M\n").unwrap();
        let all = ids(6);
        let mut c = Choice::load_from(&file);
        assert_eq!(c.shown(&all), all[..4]);
        c.take_out(&all[0], &all);
        assert!(c.show(&all[4], &all));
        c.save_to(&file).unwrap();
        let mut again = Choice::load_from(&file);
        assert_eq!(again.shown(&all), [all[1].clone(), all[2].clone(), all[3].clone(), all[4].clone()]);
        assert!(std::fs::read_to_string(&file).unwrap().contains("clock_format=%H:%M"));
        let fresh = dir.join("fresh.conf");
        Choice::default().save_to(&fresh).unwrap();
        assert!(!fresh.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
