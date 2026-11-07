//! Crash reports.

use std::collections::HashSet;
use std::path::PathBuf;

use serde_json::{Map, Value};

use crate::common::files::home;
use crate::settings::panels::network::banded;
use crate::settings::ui::{confirm, group, headline, line, on, page, Kit, Msg, Page};
use crate::toolkit::t;
use crate::toolkit::theme::Palette;
use crate::toolkit::widget::{button, card as boxed, entry, label, switch, Length, Node};

const ADD_ON: &str = "/usr/bin/trindle-crash-reports";
fn starts_on(key: &str) -> bool {
    key != "computer"
}
fn switches() -> [(&'static str, &'static str, &'static str, &'static str, Palette); 3] {
    [
        ("crashes", t!("settings-crash-crashes"), t!("settings-crash-crashes-short"), t!("settings-crash-crashes-long"), Palette::Orange),
        ("errors", t!("settings-crash-errors"), t!("settings-crash-errors-short"), t!("settings-crash-errors-long"), Palette::Teal),
        ("computer", t!("settings-crash-computer"), t!("settings-crash-computer-short"), t!("settings-crash-computer-long"), Palette::Blue),
    ]
}

const NAME_WIDTH: f32 = 200.0;
const SENT_LINE: &str = "sent";

pub fn here() -> bool {
    std::path::Path::new(ADD_ON).exists()
}

fn file() -> PathBuf {
    home().join(".config/trindle/crash-reports.json")
}

fn sent_file() -> PathBuf {
    home().join(".local/state/trindle/crash-reports/sent.jsonl")
}

fn load() -> Map<String, Value> {
    std::fs::read_to_string(file()).ok()
        .and_then(|t| serde_json::from_str::<Value>(&t).ok())
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default()
}

fn save(key: &str, on: bool) -> Result<(), String> {
    save_in(&file(), key, on)
}

fn load_kept(path: &std::path::Path) -> Result<Map<String, Value>, String> {
    let text = crate::common::files::read_small(path).map_err(|e| t!("settings-crash-nothing-changed-because", why = e.to_string()))?;
    if text.trim().is_empty() {
        return Ok(Map::new());
    }
    serde_json::from_str::<Value>(&text).ok().and_then(|v| v.as_object().cloned())
        .ok_or_else(|| t!("settings-crash-file-not-as-wrote").to_string())
}

pub(crate) fn save_in(path: &std::path::Path, key: &str, on: bool) -> Result<(), String> {
    let folder = path.parent().ok_or_else(|| t!("settings-crash-no-settings-folder").to_string())?;
    std::fs::create_dir_all(folder).map_err(|_| t!("settings-crash-folder-not-made"))?;
    let mut now = load_kept(path)?;
    now.insert(key.into(), Value::Bool(on));
    let text = serde_json::to_string_pretty(&Value::Object(now)).map_err(|_| t!("settings-crash-not-written"))?;
    crate::common::files::write_text(path, &text).map_err(|_| t!("settings-crash-not-written").into())
}

fn sent() -> Vec<(i64, u64, Value)> {
    let text = std::fs::read_to_string(sent_file()).unwrap_or_default();
    let mut all: Vec<_> = text.lines().filter_map(|l| serde_json::from_str::<Value>(l).ok()).filter_map(|v| {
        Some((v["when"].as_i64()?, v["answer"].as_u64()?, v.get("report")?.clone()))
    }).collect();
    all.reverse();
    all
}

fn what(report: &Value) -> String {
    let (crash, program) = (&report["crash"], report["crash"]["program"].as_str().or_else(|| report["program"].as_str()).unwrap_or("?"));
    match (crash["signal"].as_str().or_else(|| report["signal"].as_str()), report["code"].as_str()) {
        (Some(signal), _) => t!("settings-crash-what-crash", signal = signal, program = program),
        (_, Some(code)) => t!("settings-crash-what-error", code = code, program = program),
        _ => program.to_string(),
    }
}

fn answer_words(answer: u64) -> &'static str {
    match answer {
        200..=299 => t!("settings-crash-taken"),
        403 => t!("settings-crash-tracker-closed"),
        413 => t!("settings-crash-too-big"),
        422 => t!("settings-crash-refused"),
        429 => t!("settings-crash-too-many"),
        _ => t!("settings-crash-not-sent"),
    }
}

fn when(at: i64) -> String {
    crate::common::clock::Local::at(at).line(trindle_toolkit::lang::format::Pattern::DayShortClock)
}

#[derive(Default)]
pub struct CrashReports {
    switches: Map<String, Value>,
    sent: Vec<(i64, u64, Value)>,
    open: HashSet<usize>,
    report_offer: bool,
    report_github: String,
    report_username: String,
}

impl CrashReports {
    fn fill(&mut self) {
        self.report_offer = crate::report::offered();
        self.report_github = crate::report::github();
        self.report_username = crate::report::username();
        self.switches = load();
        self.sent = sent();
        self.open.clear();
    }

    fn sent_card(&self, kit: &Kit) -> Node<Msg> {
        let clear = confirm::<CrashReports>(kit, SENT_LINE, "user-trash-symbolic", t!("settings-crash-clear"), |me, k| {
            let result = match std::fs::remove_file(sent_file()) {
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => Some(t!("settings-crash-not-cleared")),
                _ => None,
            };
            k.done(SENT_LINE, result, t!("settings-crash-removed"));
            me.fill();
        });
        let mut sent = Vec::new();
        let mut rows = vec![line(t!("settings-crash-kept-here")).sub(t!("settings-crash-newest-first")).control(clear).key(SENT_LINE).node(kit)];
        if self.sent.is_empty() {
            rows.push(line(t!("settings-crash-nothing-yet")).sub(t!("settings-crash-listed-here")).node(kit));
        }
        for (i, (at, answer, report)) in self.sent.iter().enumerate() {
            let times: u64 = report["count"].as_u64().filter(|c| *c > 1).unwrap_or(0);
            let open = self.open.contains(&i);
            let show = button("view-reveal-symbolic", t!("settings-crash-show-sent")).on(open).on_press(on::<CrashReports>(move |me, _| {
                if !me.open.remove(&i) {
                    me.open.insert(i);
                }
            }));
            let mut l = line(what(report)).sub(t!("settings-crash-sent-sub", n = times, when = when(*at), answer = answer_words(*answer))).control(show);
            if open {
                l = l.below(boxed(label(serde_json::to_string_pretty(report).unwrap_or_default()).width(Length::Fill))
                    .sunk().width(Length::Fill));
            }
            sent.push(l.node(kit));
        }
        rows.extend(banded(sent));
        let card = group(t!("settings-crash-sent-so-far")).icon("mail-send-symbolic")
            .about(t!("settings-crash-sent-about"))
            .rows(rows);
        if self.sent.is_empty() { card.node(kit) } else { card.span(5, 8).node(kit) }
    }

    fn report_sum(&self) -> String {
        if self.report_offer { t!("settings-crash-sum-asked").into() } else { t!("settings-crash-sum-nothing").into() }
    }

    fn report_card(&self, kit: &Kit) -> Node<Msg> {
        let offer = self.offer_line(kit);
        let about = button("web-browser-symbolic", t!("settings-crash-about-community"))
            .on_press(crate::settings::ui::link("report:about", crate::report::send::COMMUNITY_URL.to_string()));
        let about_line = line(t!("settings-crash-go-to-bm"))
            .sub(t!("settings-crash-community"))
            .key("report:about").control(about).node(kit);
        let username = self.username_line(kit);
        let github = self.github_line(kit);
        let credit = line(t!("settings-crash-join"))
            .sub(t!("settings-crash-join-sub")).node(kit);
        let fixed = line(t!("settings-crash-fixed"))
            .sub(t!("settings-crash-fixed-sub")).node(kit);
        group(t!("settings-crash-reports-you-send")).icon("mail-send-symbolic")
            .about(t!("settings-crash-reports-about"))
            .rows(vec![offer, about_line, credit, fixed, username, github])
            .node(kit)
    }

    fn offer_line(&self, kit: &Kit) -> Node<Msg> {
        line(t!("settings-crash-ask"))
        .sub(t!("settings-crash-ask-sub"))
        .key("report:offer")
        .control(switch(self.report_offer, t!("settings-crash-ask"), |turned| {
            on::<CrashReports>(move |me, kit| {
                let result = crate::common::files::set_setting(crate::report::OFFER_KEY, if turned { "on" } else { "off" });
                if result.is_err() {
                    kit.done("report:offer", Some(t!("settings-crash-not-written")), t!("settings-crash-saved"));
                    return;
                }
                me.report_offer = turned;
                if turned {
                    kit.done("report:offer", None, t!("settings-crash-saved"));
                } else {
                    let n = crate::report::forget_pending(&crate::report::pending_dir());
                    let words = if n == 0 { t!("settings-crash-off-none").to_string() } else { t!("settings-crash-off-deleted", n = n) };
                    kit.say("report:offer", &words, true, true);
                }
            })
        })).node(kit)
    }

    fn username_line(&self, kit: &Kit) -> Node<Msg> {
        let name = entry(self.report_username.clone(), t!("settings-crash-username"), |t: String| {
            on::<CrashReports>(move |me, kit| {
                me.report_username = t.clone();
                match crate::report::name_fault(&t) {
                    Some(why) => kit.say("report:username", why, false, true),
                    None => {
                        let result = crate::common::files::set_setting(crate::report::USERNAME_KEY, t.trim())
                            .and_then(|()| save("username", !t.trim().is_empty()).map_err(|_| std::io::Error::other("switch")));
                        kit.done("report:username", result.err().map(|_| t!("settings-crash-not-written")), t!("settings-crash-saved"));
                    }
                }
            })
        }).width(Length::Fixed(NAME_WIDTH));
        line(t!("settings-crash-your-username")).sub(t!("settings-crash-username-sub"))
            .key("report:username").controls(vec![name]).node(kit)
    }

    fn github_line(&self, kit: &Kit) -> Node<Msg> {
        let typed = entry(self.report_github.clone(), t!("settings-crash-github-name"), |t: String| {
            on::<CrashReports>(move |me, kit| {
                me.report_github = t.clone();
                match crate::report::github_fault(&t) {
                    Some(why) => kit.say("report:github", why, false, true),
                    None => {
                        let result = crate::common::files::set_setting(crate::report::GITHUB_KEY, t.trim())
                            .and_then(|()| save("github", !t.trim().is_empty()).map_err(|_| std::io::Error::other("switch")));
                        kit.done("report:github", result.err().map(|_| t!("settings-crash-not-written")), t!("settings-crash-saved"));
                    }
                }
            })
        }).width(Length::Fixed(NAME_WIDTH));
        line(t!("settings-crash-github-line")).sub(t!("settings-crash-github-about"))
            .key("report:github").controls(vec![typed]).node(kit)
    }

    fn turned_on(&self) -> Vec<&'static str> {
        switches().iter().filter(|s| self.switches.get(s.0).and_then(Value::as_bool).unwrap_or_else(|| starts_on(s.0))).map(|s| s.0).collect()
    }

    fn sum_up(&self) -> String {
        let on: Vec<String> = switches().iter().filter(|s| self.turned_on().contains(&s.0) && s.0 != "computer")
            .map(|s| s.0.to_string()).collect();
        if on.is_empty() {
            return t!("settings-crash-sum-none").into();
        }
        let what = if on.len() == 2 { "both" } else { on[0].as_str() };
        t!("settings-crash-sum-reported", which = what)
    }
}

impl Page for CrashReports {
    fn id(&self) -> &'static str {
        "crash-reports"
    }
    fn title(&self) -> &'static str {
        t!("settings-crash-title")
    }
    fn icon(&self) -> &'static str {
        "dialog-warning-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-crash-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-crash-help")
    }
    fn shown(&mut self, _: &mut Kit) {
        self.fill();
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let flips: Vec<Node<Msg>> = switches().iter().map(|&(key, heading, short, _, _)| {
            let on_now = self.switches.get(key).and_then(Value::as_bool).unwrap_or_else(|| starts_on(key));
            let k = format!("switch:{key}");
            line(heading).sub(short).key(k.clone()).control(switch(on_now, heading, move |turned| {
                let k = k.clone();
                on::<CrashReports>(move |me, kit| {
                    let result = save(key, turned);
                    if result.is_ok() {
                        me.switches.insert(key.into(), Value::Bool(turned));
                    }
                    kit.done(&k, result.err().as_deref(), t!("settings-crash-saved"));
                })
            })).node(kit)
        }).collect();
        let what_is_sent = group(t!("settings-crash-what-is-sent")).icon("dialog-warning-symbolic")
            .about(t!("settings-crash-what-about", switches = switches().iter().map(|s| s.3).collect::<Vec<_>>().join(" ")))
            .rows(flips)
            .node(kit);
        if !here() {
            return page(kit, self.title(), self.help(), vec![headline(kit, self.report_sum()), self.report_card(kit)]);
        }
        page(kit, self.title(), self.help(), vec![
            headline(kit, self.sum_up()),
            self.report_card(kit),
            what_is_sent,
            self.sent_card(kit),
        ])
    }
}

#[cfg(test)]
impl CrashReports {
    pub(crate) fn made_up() -> CrashReports {
        let crash = serde_json::json!({ "kind": "crash", "program": "trindle-ring", "signal": "panic" });
        let error = serde_json::json!({ "kind": "error", "program": "trindle-settings", "code": "E_SAVE_FAILED", "count": 3 });
        CrashReports { sent: vec![(1_791_627_000, 202, crash), (1_791_540_000, 202, error)], ..CrashReports::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_notes_are_kept_before_a_yes_and_the_computer_description_starts_off() {
        assert!(starts_on("crashes") && starts_on("errors") && !starts_on("computer"));
    }

    #[test]
    fn a_report_is_named_plainly() {
        let crash = serde_json::json!({ "kind": "crash", "program": "trindle-ring", "signal": "panic" });
        assert_eq!(what(&crash), "Crash (panic) in trindle-ring");
        let error = serde_json::json!({ "kind": "error", "program": "trindle-ring", "code": "E_SAVE_FAILED" });
        assert_eq!(what(&error), "Error E_SAVE_FAILED in trindle-ring");
        assert_eq!(answer_words(202), "Taken");
        assert!(answer_words(422).starts_with("Not sent"));
    }
}
