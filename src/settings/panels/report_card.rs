//! The Problem reports card on About: the master switch, and the optional community username and GitHub name, kept in the very settings the report window uses; off also empties the Reporting add-on.

use std::path::Path;

use crate::common::files::set_setting;
use crate::report::{github, github_fault, name_fault, offered, pending_dir, username, GITHUB_KEY, OFFER_KEY, USERNAME_KEY};
use crate::settings::panels::about::About;
use crate::common::run::{captured, on_thread};
use crate::settings::ui::{group, line, link, not_saved, on, ran, Cause, Kit, Msg, Raw};
use crate::toolkit::t;
use crate::toolkit::widget::{button, entry, switch, Length, Node};

const OFFER: &str = "reports:offer";
const NAME: &str = "reports:name";
const GITHUB: &str = "reports:github";
const COMMUNITY_LINE: &str = "reports:community";

const ADDON: &str = "/usr/bin/trindle-reporting";

type Keep = fn(&str, &str) -> std::io::Result<()>;

type Forgot = Option<Result<(), Raw>>;
type Forget = fn(Box<dyn FnOnce(Forgot)>);

pub struct Reports {
    offer: bool,
    username: String,
    github: String,

    keep: Keep,

    forget_addon: Forget,
}

impl Default for Reports {
    fn default() -> Self {
        Reports { offer: false, username: String::new(), github: String::new(), keep: set_setting, forget_addon }
    }
}

fn forget_addon(done: Box<dyn FnOnce(Forgot)>) {
    on_thread(|| forget_with(ADDON), done);
}

fn forget_with(program: &str) -> Forgot {
    if !Path::new(program).exists() {
        return None;
    }
    let (ok, _, complaint) = captured(&[program.to_string(), "forget".to_string()], None);
    Some(ran(ok, &complaint))
}

fn addon_answered(kit: &mut Kit, said: Forgot) {
    if let Some(Err(e)) = said {
        kit.say(OFFER, &e.words(t!("settings-about-reports-addon-kept")), false, true);
    }
}

fn forget_waiting(dir: &Path) -> (usize, usize) {
    let files: Vec<_> = std::fs::read_dir(dir).into_iter().flatten().flatten()
        .map(|f| f.path()).filter(|p| p.extension().is_some_and(|e| e == "json")).collect();
    let gone = files.iter().filter(|p| std::fs::remove_file(p).is_ok()).count();
    (gone, files.len() - gone)
}

impl Reports {
    pub fn read() -> Reports {
        Reports { offer: offered(), username: username(), github: github(), ..Reports::default() }
    }

    fn set_offer(&mut self, kit: &mut Kit, now: bool, dir: &Path) {
        self.offer = now;
        if kit.settle_quietly(OFFER, (self.keep)(OFFER_KEY, if now { "on" } else { "off" }), not_saved()).is_none() {
            self.offer = !now;
            return;
        }
        if now {
            return;
        }
        (self.forget_addon)(Box::new(kit.then::<About, Forgot>(|_, k, said| addon_answered(k, said))));
        match forget_waiting(dir) {
            (0, 0) => {}
            (n, 0) => kit.say(OFFER, &t!("settings-about-reports-deleted", n = n), true, true),
            (_, n) => kit.say(OFFER, &t!("settings-about-reports-kept", n = n), false, true),
        }
    }

    fn typed(&mut self, kit: &mut Kit, line: &'static str, text: String) {
        let (key, fault) = if line == NAME { (USERNAME_KEY, name_fault(&text)) } else { (GITHUB_KEY, github_fault(&text)) };
        match fault {
            Some(why) => kit.say(line, why, false, true),
            None => {
                kit.settle(line, (self.keep)(key, text.trim()), not_saved());
            }
        }
        if line == NAME { self.username = text } else { self.github = text }
    }

    pub fn card(&self, kit: &Kit) -> Node<Msg> {
        let switched = line(t!("settings-about-reports-offer")).key(OFFER)
            .control(switch(self.offer, t!("settings-about-reports-offer"), |now| on::<About>(move |me, k| me.reports.set_offer(k, now, &pending_dir()))))
            .node(kit);
        let name = line(t!("settings-about-reports-name")).key(NAME)
            .control(entry(self.username.clone(), t!("settings-about-reports-name"), |text: String| on::<About>(move |me, k| me.reports.typed(k, NAME, text.clone())))
                .width(Length::Fixed(200.0)))
            .node(kit);
        let github = line(t!("settings-about-reports-github")).key(GITHUB)
            .control(entry(self.github.clone(), t!("settings-about-reports-github"), |text: String| on::<About>(move |me, k| me.reports.typed(k, GITHUB, text.clone())))
                .width(Length::Fixed(200.0)))
            .node(kit);
        let community = line(t!("settings-about-reports-community")).key(COMMUNITY_LINE)
            .control(button("go-next-symbolic", t!("settings-about-reports-community")).on_press(link(COMMUNITY_LINE, crate::report::send::COMMUNITY_URL)))
            .node(kit);
        group(t!("settings-about-reports")).icon("mail-send-symbolic").about(t!("settings-about-reports-about"))
            .rows(vec![switched, name, github, community]).node(kit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static KEPT: Mutex<Vec<(String, String)>> = Mutex::new(Vec::new());

    fn record(key: &str, value: &str) -> std::io::Result<()> {
        KEPT.lock().unwrap().push((key.into(), value.into()));
        Ok(())
    }

    fn kit() -> Kit {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: crate::settings::ui::Do| {});
        Kit::new(events, asks, None, None)
    }

    fn page() -> Reports {
        KEPT.lock().unwrap().clear();
        Reports { keep: record, forget_addon: |_| record("addon", "forget").unwrap(), ..Reports::default() }
    }

    fn kept() -> Vec<(String, String)> {
        std::mem::take(&mut *KEPT.lock().unwrap())
    }

    #[test]
    fn the_boxes_and_the_switch_write_the_settings_the_report_window_reads() {
        let (mut r, mut k) = (page(), kit());
        r.typed(&mut k, NAME, " john-smith ".into());
        r.typed(&mut k, GITHUB, "john-smith2".into());
        r.set_offer(&mut k, true, Path::new("/nonexistent"));
        r.set_offer(&mut k, false, Path::new("/nonexistent"));
        assert_eq!(kept(), [
            (crate::report::USERNAME_KEY.to_string(), "john-smith".to_string()), (crate::report::GITHUB_KEY.to_string(), "john-smith2".to_string()),
            (crate::report::OFFER_KEY.to_string(), "on".to_string()), (crate::report::OFFER_KEY.to_string(), "off".to_string()),
            ("addon".to_string(), "forget".to_string())]);
        r.keep = |_, _| Err(std::io::Error::other("full"));
        r.set_offer(&mut k, false, Path::new("/nonexistent"));
        assert!(kept().is_empty(), "the add-on was emptied though the switch stayed on");
        r.keep = record;
        assert_eq!(k.said(NAME), None);

        r.typed(&mut k, NAME, "john@example.com".into());
        assert_eq!(k.said(NAME), Some((crate::report::no_at(), true)));
        r.typed(&mut k, GITHUB, "has space".into());
        assert!(k.said(GITHUB).is_some_and(|(_, bad)| bad));
        assert!(kept().is_empty(), "a refused name was saved");

        r.typed(&mut k, NAME, String::new());
        assert_eq!(kept(), [(crate::report::USERNAME_KEY.to_string(), String::new())]);
        assert_eq!(k.said(NAME), None, "the red words stay after the box is cleared");
    }

    #[test]
    fn switching_off_deletes_the_waiting_reports_and_says_how_many() {
        let dir = std::env::temp_dir().join(format!("trindle-report-card-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        for n in 0..2 {
            std::fs::write(dir.join(format!("{n}.json")), "{}").unwrap();
        }
        std::fs::write(dir.join("keep.txt"), "").unwrap();
        let (mut r, mut k) = (Reports { keep: |_, _| Ok(()), forget_addon: |_| (), ..Reports::default() }, kit());
        r.set_offer(&mut k, false, &dir);
        let left = std::fs::read_dir(&dir).unwrap().count();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(left, 1);
        assert_eq!(k.said(OFFER), Some(("Off. 2 waiting reports deleted.", false)));
    }

    #[test]
    fn a_failed_forget_by_the_add_on_is_said_in_red_and_a_missing_add_on_says_nothing() {
        assert!(forget_with("/nonexistent/trindle-reporting").is_none());
        assert!(matches!(forget_with("/bin/false"), Some(Err(_))));
        let mut k = kit();
        addon_answered(&mut k, None);
        addon_answered(&mut k, Some(Ok(())));
        assert_eq!(k.said(OFFER), None);
        addon_answered(&mut k, Some(Err(Raw("rm: cannot remove: Permission denied".into()))));
        let (words, bad) = k.said(OFFER).unwrap();
        assert!(bad);
        assert!(words.starts_with(t!("settings-about-reports-addon-kept")) && words.len() > t!("settings-about-reports-addon-kept").len(),
                "the reason is missing: {words}");
    }
}
