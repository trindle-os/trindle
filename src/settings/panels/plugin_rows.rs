//! Cards for the plugins that ship a rows file (plugins/rows.rs).

use serde_json::Value;
use trindle_toolkit::lang;

use crate::common::run;
use crate::plugins::rows::{self, Act, Done, Kind, Row, Rows, Status, Words};
use crate::settings::panels::plugins::Plugins;
use crate::settings::ui::{dropdown, group, line, on, Kit, Msg};
use crate::toolkit::t;
use crate::toolkit::widget::{button, switch, Node};

const LOOK_EVERY: u32 = 5;
const ICON: &str = "application-x-addon-symbolic";
const DO_ICON: &str = "go-next-symbolic";

struct Card {
    rows: Rows,
    words: Words,
    status: Option<Status>,
    asked: bool,
}

#[derive(Default)]
pub struct Cards {
    all: Vec<Card>,
    looks: u32,
}

fn key_of(id: &str, row: usize) -> String {
    format!("rows:{id}:{row}")
}

impl Cards {
    pub fn read(&mut self) {
        let tags = lang::choice();
        self.all = rows::load_all().into_iter().map(|rows| Card { words: rows.words(&tags), rows, status: None, asked: false }).collect();
    }

    pub fn names(&self) -> Vec<String> {
        self.all.iter().map(|c| c.rows.name.clone()).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.all.is_empty()
    }

    pub fn refresh(&self, kit: &Kit) {
        for card in &self.all {
            let (id, rows) = (card.rows.id.clone(), card.rows.clone());
            let back = kit.then::<Plugins, Option<Status>>(move |me, _, status| me.rows.heard(&id, status));
            run::on_thread(move || rows.status(), back);
        }
    }

    pub fn second(&mut self, kit: &Kit) {
        self.looks += 1;
        if self.looks >= LOOK_EVERY {
            self.looks = 0;
            self.refresh(kit);
        }
    }

    fn heard(&mut self, id: &str, status: Option<Status>) {
        if let Some(card) = self.all.iter_mut().find(|c| c.rows.id == id) {
            card.asked = true;
            card.status = status;
        }
    }

    fn act(&mut self, kit: &mut Kit, id: &str, ri: usize, what: Act) {
        if matches!(what, Act::Switch(false)) && !crate::toolkit::window::persons_own(crate::common::person_only::PLUGIN_OFF) {
            return;
        }
        let Some(card) = self.all.iter_mut().find(|c| c.rows.id == id) else { return };
        let Some(row) = card.rows.rows.get(ri) else { return };
        if let Some(status) = card.status.as_mut() {
            match (&row.kind, what) {
                (Kind::Switch { field, .. }, Act::Switch(now)) => {
                    status.insert(field.clone(), Value::Bool(now));
                }
                (Kind::Choice { field, options, .. }, Act::Pick(i)) => {
                    if let Some(o) = options.get(i) {
                        status.insert(field.clone(), o.value.parse::<u64>().map_or_else(|_| Value::String(o.value.clone()), Value::from));
                    }
                }
                _ => {}
            }
        }
        let launch = matches!(row.kind, Kind::Launch { .. });
        let (rows, id) = (card.rows.clone(), id.to_string());
        let back = kit.then::<Plugins, Done>(move |me, k, done| me.rows.done(k, &id, ri, launch, done));
        run::on_thread(move || rows.act(ri, what), back);
    }

    fn done(&mut self, kit: &mut Kit, id: &str, ri: usize, launch: bool, done: Done) {
        let key = key_of(id, ri);
        let name = self.all.iter().find(|c| c.rows.id == id).map(|c| c.rows.name.clone()).unwrap_or_default();
        match done {
            Done::Kept if launch => kit.hush(&key),
            Done::Kept => kit.say(&key, t!("settings-plugins-rows-saved"), true, false),
            Done::Cancelled => kit.hush(&key),
            Done::Refused => kit.say(&key, &t!("settings-plugins-rows-refused", name = &name), false, false),
            Done::Silent => kit.say(&key, &t!("settings-plugins-rows-silent", name = &name), false, false),
            Done::Failed(None) => kit.say(&key, &t!("settings-plugins-rows-failed", name = &name), false, false),
            Done::Failed(Some(why)) => kit.say(&key, &t!("settings-plugins-rows-failed-why", name = &name, why = &why), false, false),
        }
        self.refresh(kit);
    }

    pub fn nodes(&self, kit: &Kit) -> Vec<Node<Msg>> {
        self.all.iter().map(|c| c.node(kit)).collect()
    }
}

impl Card {
    fn node(&self, kit: &Kit) -> Node<Msg> {
        let (id, name) = (&self.rows.id, &self.rows.name);
        let sentence = match (&self.status, self.asked) {
            (Some(s), _) => self.rows.sentence(s, &self.words),
            (None, false) => Some(t!("settings-plugins-rows-asking", name = name)),
            (None, true) => Some(t!("settings-plugins-rows-silent", name = name)),
        };
        let mut items: Vec<Node<Msg>> = sentence.into_iter().map(|w| line(w).key(format!("rows:{id}")).node(kit)).collect();
        if let Some(status) = &self.status {
            let shown = self.rows.rows.iter().enumerate().filter(|(_, r)| r.shows(status));
            items.extend(shown.filter_map(|(ri, row)| self.row(kit, ri, row, status)));
        }
        group(name.clone()).icon(ICON).rows(items).span(4, 12).known_as(format!("rows:{id}")).node(kit)
    }

    fn row(&self, kit: &Kit, ri: usize, row: &Row, status: &Status) -> Option<Node<Msg>> {
        let label = self.words.say(&row.label, None)?;
        let id = self.rows.id.clone();
        let key = key_of(&id, ri);
        let mut item = line(label.clone()).key(key.clone());
        if let Some(sub) = row.sub.as_ref().and_then(|s| self.words.say(s, None)) {
            item = item.sub(sub);
        }
        let control = match &row.kind {
            Kind::Switch { .. } => switch(row.kind.is_on(status), label, move |now| {
                let id = id.clone();
                on::<Plugins>(move |me, k| me.rows.act(k, &id, ri, Act::Switch(now)))
            }),
            Kind::Choice { options, .. } => {
                let shown: Vec<(usize, String)> = options.iter().enumerate().filter_map(|(i, o)| Some((i, self.words.say(&o.text, None)?))).collect();
                let chosen = row.kind.chosen(status).and_then(|c| shown.iter().position(|(i, _)| *i == c));
                let picks: Vec<usize> = shown.iter().map(|(i, _)| *i).collect();
                dropdown(kit, &key, shown.into_iter().map(|(_, w)| w).collect(), chosen, &label, move |n| {
                    let (id, i) = (id.clone(), picks[n]);
                    on::<Plugins>(move |me, k| me.rows.act(k, &id, ri, Act::Pick(i)))
                })
            }
            Kind::Send { .. } | Kind::Launch { .. } => button(DO_ICON, label).on_press(on::<Plugins>(move |me, k| me.rows.act(k, &id, ri, Act::Press))),
        };
        Some(item.control(control).node(kit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::event::Loop;
    use crate::settings::ui::Do;

    const FILE: &str = r#"
name = "Thing"
words = "thing"
socket = "/run/trindle-thing/user.sock"
status = "status"
[[say]]
test = "!on"
text = "t-off"
[[say]]
text = "t-on"
name = "who"
[[row]]
kind = "switch"
label = "t-on-label"
field = "on"
send_on = "on"
send_off = "off"
[[row]]
kind = "choice"
show = "paired"
label = "t-delay"
sub = "t-delay-sub"
field = "delay"
send = "delay {}"
options = [ { value = 10, text = "t-ten" }, { value = 30, text = "t-thirty" }, { value = 60, text = "t-not-in-words" } ]
[[row]]
kind = "button"
show = "paired"
label = "t-forget"
send = "unpair"
[[row]]
kind = "button"
label = "t-missing-from-words"
send = "unpair"
"#;

    fn card(status: &str) -> Card {
        let top = std::env::temp_dir().join(format!("trindle-plugin-rows-ui-{}", std::process::id()));
        std::fs::create_dir_all(top.join("share/lang/en")).unwrap();
        std::fs::write(top.join("share/lang/en/thing.ftl"), "t-off = Off.\nt-on = Paired with { $name }.\nt-on-label = Use it\nt-delay = Lock after\n\
                                                              t-delay-sub = Once it is away.\nt-ten = 10 seconds\nt-thirty = 30 seconds\nt-forget = Forget\n").unwrap();
        let roots = rows::Roots { share: top.join("share"), run: top.join("run"), lib: top.join("lib"), pkexec: "pkexec".into() };
        let rows = Rows::parse("thing", FILE, &roots).unwrap();
        Card { words: rows.words(&[]), rows, status: serde_json::from_str(status).ok(), asked: true }
    }

    fn said(card: Card) -> Vec<String> {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let kit = Kit::new(events.clone(), asks, None, None);
        let cards = Cards { all: vec![card], looks: 0 };
        cards.nodes(&kit).iter().flat_map(|n| n.said()).collect()
    }

    #[test]
    fn the_card_is_drawn_from_the_rows_in_the_plugins_own_words() {
        let on = said(card(r#"{"on":true,"paired":true,"delay":30,"who":"John's phone"}"#));
        for want in ["Thing", "Paired with John's phone.", "Use it", "Lock after", "Once it is away.", "Forget"] {
            assert!(on.iter().any(|w| w == want), "{want:?} is not on the card: {on:?}");
        }
        assert!(!on.iter().any(|w| w.contains("t-")), "{on:?}");
        let off = said(card(r#"{"on":false,"paired":false}"#));
        assert!(off.iter().any(|w| w == "Off.") && !off.iter().any(|w| w == "Lock after" || w == "Forget"), "{off:?}");
    }

    #[test]
    fn a_daemon_that_does_not_answer_is_said_with_no_rows() {
        let none = said(card("null"));
        assert!(none.iter().any(|w| w == "Thing is not answering. Its service may be stopped."), "{none:?}");
        assert!(!none.iter().any(|w| w == "Use it"), "{none:?}");
    }
}
