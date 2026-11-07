//! Your plugins.

use std::collections::{BTreeMap, HashMap};
use std::time::Duration;

use crate::plugins::{self, Field, Kind, Plugin};
use crate::settings::ui::{confirm, dropdown, group, headline, in_words, line, on, page_with, Kit, Msg, Page, not_saved};

fn ask_again(key: String, id: String) -> Msg {
    on::<Plugins>(move |_, k| {
        let started = plugins::ask_again(&id);
        k.started(&key, started, t!("settings-plugins-window-failed"));
    })
}
use crate::toolkit::t;
use crate::toolkit::widget::{button, entry, main_action, note, switch, Length, Node};

const TYPING_PAUSE: Duration = Duration::from_millis(700);

pub fn here() -> bool {
    !plugins::installed().is_empty() || !plugins::rows::load_all().is_empty()
}

#[derive(Default)]
pub struct Plugins {
    all: Vec<Plugin>,
    values: HashMap<String, BTreeMap<String, String>>,
    round: u32,
    choices: HashMap<(String, String), Vec<(String, String)>>,
    notes: HashMap<(String, String), String>,
    standing: HashMap<String, (usize, usize)>,
    pub(super) rows: super::plugin_rows::Cards,
}

impl Plugins {
    fn read(&mut self) {
        self.all = plugins::installed();
        self.values = self.all.iter().map(|p| (p.id.clone(), plugins::values(p))).collect();
        self.choices = self.all.iter().flat_map(|p| p.fields.iter().filter(|f| matches!(f.kind, Kind::Choice(_)))
            .map(|f| ((p.id.clone(), f.name.clone()), plugins::choices_of(&p.id, f)))).collect();
        self.standing = self.all.iter().map(|p| (p.id.clone(), plugins::standing(p))).collect();
        self.reread_notes();
    }

    fn reread_notes(&mut self) {
        self.notes = self.all.iter().flat_map(|p| p.fields.iter().filter(|f| matches!(f.kind, Kind::Note(_)))
            .map(|f| ((p.id.clone(), f.name.clone()), plugins::note_of(&p.id, f)))).collect();
        for p in &self.all {
            let now = plugins::values(p);
            let kept = self.values.entry(p.id.clone()).or_default();
            for f in p.fields.iter().filter(|f| matches!(f.kind, Kind::Switch | Kind::Choice(_))) {
                if let Some(v) = now.get(&f.name) {
                    kept.insert(f.name.clone(), v.clone());
                }
            }
        }
    }

    fn reread_choices(&mut self) -> bool {
        let mut changed = false;
        for p in &self.all {
            for f in p.fields.iter().filter(|f| f.choices_from.is_some()) {
                let now = plugins::choices_of(&p.id, f);
                let slot = self.choices.entry((p.id.clone(), f.name.clone())).or_default();
                if *slot != now {
                    *slot = now;
                    changed = true;
                }
            }
        }
        changed
    }

    fn value(&self, id: &str, name: &str) -> String {
        self.values.get(id).and_then(|v| v.get(name)).cloned().unwrap_or_default()
    }

    fn save(&mut self, kit: &mut Kit, id: &str, name: &str, value: &str, put_back: bool) {
        self.values.entry(id.to_string()).or_default().insert(name.to_string(), value.to_string());
        let kept = kit.settle(&field_key(id, name), plugins::set_value(id, name, value), not_saved()).is_some();
        if !kept && put_back {
            if let Some(p) = self.all.iter().find(|p| p.id == id) {
                self.values.insert(id.to_string(), plugins::values(p));
            }
        }
    }

    fn shown_fields<'a>(&self, p: &'a Plugin) -> Vec<&'a Field> {
        let none = BTreeMap::new();
        let values = self.values.get(&p.id).unwrap_or(&none);
        p.fields.iter().filter(|f| {
            let offered = self.choices.get(&(p.id.clone(), f.name.clone())).map(Vec::as_slice).unwrap_or(&[]);
            let said = !matches!(f.kind, Kind::Note(_)) || self.notes.get(&(p.id.clone(), f.name.clone())).is_some_and(|n| !n.is_empty());
            said && plugins::field_shown(p, f, values, offered)
        }).collect()
    }

    fn says<'a>(&self, p: &'a Plugin) -> Says<'a> {
        let (asks, allowed) = self.standing.get(&p.id).copied().unwrap_or((0, 0));
        let needs = plugins::needs_words(&p.name, asks, allowed);
        let about = match &needs {
            Some(_) => t!("settings-plugins-version", v = &p.version),
            None => t!("settings-plugins-version-settled", v = &p.version, settled = plugins::settled_words(asks)),
        };
        let fields = self.shown_fields(p);
        let none = fields.is_empty().then(|| no_settings_words(&p.name, p.fields.is_empty()));
        Says { change: needs.is_none() && asks > 0, needs, about, fields, none }
    }

    fn card(&self, kit: &Kit, p: &Plugin) -> Node<Msg> {
        let (asks, allowed) = self.standing.get(&p.id).copied().unwrap_or((0, 0));
        let names: Vec<String> = p.asks.iter().filter(|a| plugins::PERMISSIONS.contains(&a.as_str()))
            .map(|a| crate::common::access::ask(a).name.to_string()).collect();
        let longer = if names.is_empty() {
            t!("settings-plugins-asks-nothing", name = &p.name)
        } else {
            t!("settings-plugins-asks-for", name = &p.name, list = in_words(&names), allowed = allowed, asks = asks)
        };
        let says = self.says(p);
        let mut rows = Vec::new();
        if let Some(words) = &says.needs {
            rows.push(self.needs_row(kit, p, words));
        }
        rows.push(self.about_row(kit, p, &says));
        if let Some(words) = says.none {
            rows.push(note(words));
        }
        for f in says.fields {
            rows.extend(self.field_row(kit, p, f));
        }
        group(p.name.clone()).known_as(format!("plugin:{}", p.id)).icon("application-x-addon-symbolic").about(longer).rows(rows).span(CARD_SPAN.0, CARD_SPAN.1).node(kit)
    }

    fn needs_row(&self, kit: &Kit, p: &Plugin, words: &str) -> Node<Msg> {
        let id = p.id.clone();
        let asks_key = format!("plugin-asks:{}", p.id);
        line(words.to_string()).sub(t!("settings-plugins-needs-more")).key(asks_key.clone())
            .control(main_action(plugins::ALLOW_ICON, plugins::allow_word()).on_press(ask_again(asks_key, id)))
            .node(kit)
    }

    fn about_row(&self, kit: &Kit, p: &Plugin, says: &Says) -> Node<Msg> {
    let key = format!("plugin:{}", p.id);
    let mut about = line(t!("settings-plugins-about-it")).sub(says.about.clone()).key(key.clone());
    if says.change {
        let id = p.id.clone();
        about = about.control(button(plugins::ALLOW_ICON, t!("settings-plugins-change")).on_press(ask_again(key.clone(), id)));
    }
    let id = p.id.clone();
    about = about.control(confirm::<Plugins>(kit, &key, "user-trash-symbolic", t!("settings-plugins-remove"), move |me, k| {
        if k.settle_quietly(&format!("plugin:{id}"), plugins::remove(&id), t!("settings-plugins-not-removed")).is_some() {
            me.read();
        }
    }));
    about.node(kit)
    }

    fn field_row(&self, kit: &Kit, p: &Plugin, f: &Field) -> Option<Node<Msg>> {
        let now = self.value(&p.id, &f.name);
        let key = field_key(&p.id, &f.name);
        let (id, name) = (p.id.clone(), f.name.clone());
        if let Kind::Note(_) = &f.kind {
            let words = self.notes.get(&(p.id.clone(), f.name.clone())).cloned().unwrap_or_default();
            return Some(if f.label.is_empty() { note(words).key(key) } else { line(f.label.clone()).sub(words).key(key).node(kit) });
        }
        let control = match &f.kind {
            Kind::Switch => switch(now == "true", f.label.clone(), move |on_now| {
                let (id, name) = (id.clone(), name.clone());
                on::<Plugins>(move |me, k| me.save(k, &id, &name, if on_now { "true" } else { "false" }, true))
            }),
            Kind::Choice(_) => {
                let offered = self.choices.get(&(p.id.clone(), f.name.clone())).cloned().unwrap_or_default();
                let chosen = offered.iter().position(|(value, _)| *value == now);
                let labels: Vec<String> = offered.iter().map(|(_, label)| label.clone()).collect();
                dropdown(kit, &key, labels, chosen, &f.label, move |i| {
                    let (id, name, value) = (id.clone(), name.clone(), offered[i].0.clone());
                    on::<Plugins>(move |me, k| me.save(k, &id, &name, &value, true))
                })
            }
            Kind::Text => entry(now, text_hint(&f.label), move |text| {
                let (id, name) = (id.clone(), name.clone());
                on::<Plugins>(move |me, k| me.typed(k, &id, &name, text.clone()))
            }).spoken(f.label.clone()).width(Length::Fixed(TEXT_WIDTH)),
            Kind::Note(_) => return None,
        };
        Some(line(f.label.clone()).key(key).control(control).node(kit))
    }

    fn typed(&mut self, kit: &mut Kit, id: &str, name: &str, text: String) {
        self.values.entry(id.to_string()).or_default().insert(name.to_string(), text);
        self.round = self.round.wrapping_add(1);
        let (mine, id, name) = (self.round, id.to_string(), name.to_string());
        kit.after::<Plugins>(TYPING_PAUSE, move |me, k| {
            if me.round == mine {
                let value = me.value(&id, &name);
                me.save(k, &id, &name, &value, false);
            }
        });
    }
}

struct Says<'a> {
    needs: Option<String>,
    about: String,
    change: bool,
    fields: Vec<&'a Field>,
    none: Option<String>,
}

const CARD_SPAN: (u8, u8) = (4, 12);

const TEXT_WIDTH: f32 = 220.0;
const TEXT_HINT_MAX: usize = 22;

fn text_hint(label: &str) -> String {
    if label.chars().count() <= TEXT_HINT_MAX { label.to_string() } else { t!("settings-plugins-type-here").to_string() }
}

fn no_settings_words(name: &str, none_at_all: bool) -> String {
    if none_at_all {
        t!("settings-plugins-no-settings", name = name)
    } else {
        t!("settings-plugins-no-settings-now", name = name)
    }
}

fn field_key(id: &str, name: &str) -> String {
    format!("plugin:{id}:{name}")
}

impl Page for Plugins {
    fn id(&self) -> &'static str {
        "plugins"
    }
    fn title(&self) -> &'static str {
        t!("settings-plugins-title")
    }
    fn icon(&self) -> &'static str {
        "application-x-addon-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-plugins-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-plugins-help")
    }
    fn shown(&mut self, kit: &mut Kit) {
        self.read();
        self.rows.read();
        self.rows.refresh(kit);
    }
    fn second(&mut self, kit: &mut Kit) -> bool {
        self.rows.second(kit);
        self.reread_choices();
        self.reread_notes();
        self.standing = self.all.iter().map(|p| (p.id.clone(), plugins::standing(p))).collect();
        true
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let mut names: Vec<String> = self.all.iter().map(|p| p.name.clone()).collect();
        names.extend(self.rows.names());
        let words = t!("settings-plugins-installed", n = names.len(), list = in_words(&names));
        let mut items = vec![headline(kit, words)];
        items.extend(self.all.iter().map(|p| self.card(kit, p)));
        items.extend(self.rows.nodes(kit));
        if self.all.is_empty() && self.rows.is_empty() {
            items.push(group(t!("settings-plugins-group")).rows(vec![line(t!("settings-plugins-none")).sub(t!("settings-plugins-none-sub")).node(kit)]).node(kit));
        }
        let permissions = button("changes-prevent-symbolic", t!("settings-plugins-open-permissions")).key("plugins:permissions")
            .on_press(on::<Plugins>(|_, k| k.go_to("permissions")));
        page_with(kit, self.title(), self.help(), vec![permissions], items)
    }
}

#[cfg(test)]
impl Plugins {
    pub(crate) fn made_up() -> Plugins {
        let field = |name: &str, label: &str, kind: Kind, default: &str| Field {
            name: name.into(), label: label.into(), kind, default: default.into(), choices_from: None, show_when: None,
        };
        let plugin = |id: &str, name: &str, fields: Vec<Field>| Plugin {
            id: id.into(), name: name.into(), version: "1.2".into(), summary: String::new(),
            asks: ["network", "notifications"].iter().map(|a| a.to_string()).collect(),
            ring: None, background: None, listen: Vec::new(), shortcuts: Vec::new(), fields,
        };
        let sides = Kind::Choice(vec!["Left".into(), "Right".into()]);
        let all = vec![
            plugin("org.example.Pair", "Pair", vec![field("name", "Name the other computer sees", Kind::Text, ""),
                                                    field("start", "Start when you log in", Kind::Switch, "true"),
                                                    field("side", "Where the other computer is", sides, "Left")]),
            plugin("org.example.Weather", "Weather", vec![field("town", "Town", Kind::Text, "Anytown")]),
        ];
        let mut me = Plugins::default();
        for (i, p) in all.iter().enumerate() {
            me.values.insert(p.id.clone(), p.fields.iter().map(|f| (f.name.clone(), f.default.clone())).collect());
            for f in &p.fields {
                if let Kind::Choice(c) = &f.kind {
                    me.choices.insert((p.id.clone(), f.name.clone()), c.iter().map(|c| (c.clone(), c.clone())).collect());
                }
            }
            me.standing.insert(p.id.clone(), (p.asks.len(), if i == 0 { p.asks.len() } else { 0 }));
        }
        me.all = all;
        me
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::ui::Do;
    use crate::settings::Settings;
    use crate::toolkit::event::Loop;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::{App as _, Ui};

    fn field(name: &str, label: &str, kind: Kind, default: &str, from: Option<&str>, when: Option<(&str, &str)>) -> Field {
        Field { name: name.into(), label: label.into(), kind, default: default.into(), choices_from: from.map(String::from),
                show_when: when.map(|(o, v)| (o.to_string(), v.to_string())) }
    }

    fn choice(of: &[&str]) -> Kind {
        Kind::Choice(of.iter().map(|c| c.to_string()).collect())
    }

    fn pair() -> Plugin {
        let asks = ["network", "notifications", "background"].iter().map(|a| a.to_string()).collect();
        Plugin {
            id: "org.example.Pair".into(), name: "Pair".into(), version: "1.2".into(), summary: String::new(), asks,
            ring: None, background: None, listen: Vec::new(), shortcuts: Vec::new(),
            fields: vec![
                field("role", "This computer", choice(&["Controls the other computer", "Is controlled by the other computer"]),
                      "Controls the other computer", None, None),
                field("name", "The name the other computer sees, which may be long", Kind::Text, "", None, None),
                field("start", "Start when you log in", Kind::Switch, "true", None, None),
                field("side", "Where the other computer is", choice(&["Left", "Right"]), "Left", None,
                      Some(("role", "Controls the other computer"))),
                field("found", "The other computer", choice(&[]), "", Some("found.txt"), Some(("role", "Controls the other computer"))),
                field("desk", "Its desktop", choice(&[]), "", Some("desks.txt"), Some(("role", "Controls the other computer"))),
                field("fast", "Move the pointer faster on the other computer", Kind::Switch, "", None,
                      Some(("role", "Controls the other computer"))),
                field("quiet", "Say nothing when it joins", Kind::Switch, "", None, Some(("start", "true"))),
            ],
        }
    }

    fn page_of(all: Vec<Plugin>, allowed: usize) -> Plugins {
        let mut me = Plugins::default();
        for p in &all {
            let values = p.fields.iter().map(|f| (f.name.clone(), f.default.clone())).collect();
            me.values.insert(p.id.clone(), values);
            for f in p.fields.iter().filter(|f| matches!(f.kind, Kind::Choice(_))) {
                let own = match &f.kind {
                    Kind::Choice(c) => c.iter().map(|c| (c.clone(), c.clone())).collect(),
                    _ => Vec::new(),
                };
                me.choices.insert((p.id.clone(), f.name.clone()), own);
            }
            me.standing.insert(p.id.clone(), (p.asks.len(), allowed.min(p.asks.len())));
        }
        me.all = all;
        me
    }

    fn names(says: &Says) -> Vec<String> {
        says.fields.iter().map(|f| f.name.clone()).collect()
    }

    fn set(me: &mut Plugins, name: &str, value: &str) {
        me.values.get_mut("org.example.Pair").unwrap().insert(name.into(), value.into());
    }

    #[test]
    fn the_card_follows_the_other_field_at_once_and_keeps_what_is_hidden() {
        let p = pair();
        let mut me = page_of(vec![p.clone()], 3);
        assert_eq!(names(&me.says(&p)), ["role", "name", "start", "side", "fast", "quiet"]);
        set(&mut me, "side", "Right");
        set(&mut me, "role", "Is controlled by the other computer");
        assert_eq!(names(&me.says(&p)), ["role", "name", "start", "quiet"]);
        assert_eq!(me.value("org.example.Pair", "side"), "Right", "a hidden field keeps its value");
        set(&mut me, "role", "Controls the other computer");
        assert_eq!(names(&me.says(&p)), ["role", "name", "start", "side", "fast", "quiet"]);
        assert_eq!(me.value("org.example.Pair", "side"), "Right");
        set(&mut me, "start", "false");
        assert!(!names(&me.says(&p)).contains(&"quiet".to_string()));
    }

    #[test]
    fn a_list_the_plugin_fills_shows_once_it_has_two_to_choose_from() {
        let p = pair();
        let mut me = page_of(vec![p.clone()], 3);
        let key = ("org.example.Pair".to_string(), "found".to_string());
        assert!(!names(&me.says(&p)).contains(&"found".to_string()));
        me.choices.insert(key.clone(), vec![("a".into(), "Desk".into())]);
        assert!(!names(&me.says(&p)).contains(&"found".to_string()), "one is no choice");
        me.choices.insert(key.clone(), vec![("a".into(), "Desk".into()), ("b".into(), "Laptop".into())]);
        assert!(names(&me.says(&p)).contains(&"found".to_string()));
        me.choices.remove(&key);
        assert!(!names(&me.says(&p)).contains(&"found".to_string()), "gone again with its file");
    }

    #[test]
    fn a_card_short_of_permission_says_so_first_and_quietly_when_it_has_all() {
        let p = pair();
        let none = page_of(vec![p.clone()], 0);
        let says = none.says(&p);
        assert_eq!(says.needs.as_deref(), Some("Pair needs your permission to work: 0 of 3 allowed."));
        assert_eq!(says.about, "Version 1.2.");
        assert!(!says.change);
        let some = page_of(vec![p.clone()], 2);
        assert_eq!(some.says(&p).needs.as_deref(), Some("Pair has only some of the permission it asks for: 2 of 3 allowed."));
        let all = page_of(vec![p.clone()], 3);
        let says = all.says(&p);
        assert_eq!(says.needs, None);
        assert_eq!(says.about, "Version 1.2. All 3 things it asks for are allowed.");
        assert!(says.change);
        let mut plain = pair();
        plain.asks.clear();
        let me = page_of(vec![plain.clone()], 0);
        let says = me.says(&plain);
        assert_eq!((says.needs, says.change, says.about.as_str()), (None, false, "Version 1.2. It asks for nothing."));
    }

    #[test]
    fn the_card_of_a_plugin_short_of_permission_leads_with_the_words_and_one_allow_button() {
        let seen = |allowed: usize| {
            let me = page_of(vec![pair()], allowed);
            let events = Loop::new();
            let asks = events.post(|_: Do| {});
            let view = me.view(&Kit::new(events.clone(), asks, None, None));
            (view.said(), view.buttons())
        };
        let allow = |b: &[(String, bool)]| b.iter().filter(|(tip, _)| tip == "Allow").count();
        for (allowed, words) in [(0, "Pair needs your permission"), (2, "Pair has only some of the permission")] {
            let (said, buttons) = seen(allowed);
            let at = |w: &str| said.iter().position(|x| x.starts_with(w)).unwrap_or_else(|| panic!("{w} not in {said:?}"));
            assert!(at(words) < at("Version 1.2."), "the words come before the rest of the card");
            assert!(at(words) < at("button: Allow"));
            assert_eq!(allow(&buttons), 1, "one Allow button: {buttons:?}");
            assert!(buttons.iter().any(|(tip, can)| tip == "Allow" && *can));
        }
        let (said, buttons) = seen(3);
        assert!(!said.iter().any(|w| w.contains("needs your permission") || w.contains("only some of the permission")), "{said:?}");
        assert_eq!(allow(&buttons), 0, "{buttons:?}");
    }

    #[test]
    fn an_empty_text_box_never_shows_a_label_cut_short() {
        assert_eq!(text_hint("Town"), "Town");
        assert_eq!(text_hint(&"W".repeat(TEXT_HINT_MAX)), "W".repeat(TEXT_HINT_MAX));
        assert_eq!(text_hint(&"W".repeat(TEXT_HINT_MAX + 1)), "Type here");
    }

    #[test]
    fn a_card_with_no_settings_to_show_says_so() {
        let mut bare = pair();
        bare.fields.clear();
        let me = page_of(vec![bare.clone()], 3);
        assert_eq!(me.says(&bare).none.as_deref(), Some("Pair has no settings of its own."));
        let mut hidden = pair();
        hidden.fields.retain(|f| f.name == "found");
        hidden.fields[0].show_when = None;
        let me = page_of(vec![hidden.clone()], 3);
        let says = me.says(&hidden);
        assert!(says.fields.is_empty());
        assert_eq!(says.none.as_deref(), Some("Pair has no settings to show as things stand."));
        let p = pair();
        assert_eq!(page_of(vec![p.clone()], 3).says(&p).none, None);
    }

    fn draw(me: Plugins, width: f32, height: f32, scale: f32) -> tiny_skia::Pixmap {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let mut kit = Kit::new(events.clone(), asks, None, None);
        kit.current = "plugins".into();
        let s = Settings { kit, pages: vec![Box::new(me)], current: 0, search: String::new(), tall: height };
        let mut ui = Ui::new(s.view(), Theme::new(false, "#fbc711"));
        let mut pix = tiny_skia::Pixmap::new((width * scale) as u32, (height * scale) as u32).unwrap();
        ui.paint(&mut pix.as_mut(), scale);
        pix
    }

    #[test]
    fn how_plugins_work_is_behind_the_i_and_permissions_is_in_the_title_row() {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let kit = Kit::new(events.clone(), asks, None, None);
        let me = Plugins::made_up();
        let view = me.view(&kit);
        let said = view.said();
        assert!(!said.iter().any(|w| w == "How plugins work"), "{said:?}");
        for gone in ["Where they show", "Walled off", "Their own settings", "Removing one"] {
            assert!(!said.iter().any(|w| w == gone), "{gone} is no longer a line on the page");
        }
        for words in ["the ring", "settings", "notifications", "login", "walled off", "Permissions", "bin", "removes"] {
            assert!(me.help().contains(words), "the i says: {words}");
        }
        assert!(view.buttons().contains(&("Open Permissions".to_string(), true)));
        let s = Settings { kit, pages: vec![Box::new(me)], current: 0, search: String::new(), tall: 692.0 };
        let mut ui = Ui::new(s.view(), Theme::new(true, "#fbc711"));
        ui.lay_at(1278, 692);
        let boxes = ui.boxes();
        let button = boxes.iter().find(|b| b.0.ends_with("plugins:permissions")).expect("the button is on the page").1;
        let cards = ui.puzzle_cards_named();
        let first = cards.iter().filter(|c| c.0.as_ref().is_some_and(|k| k.starts_with("card:"))).map(|c| c.1.y).fold(f32::MAX, f32::min);
        assert!(button.y + button.h <= first, "above every card: {button:?}, the first card at {first}");
        let names: Vec<String> = cards.into_iter().filter_map(|c| c.0).collect();
        assert_eq!(names, ["card:plugin:org.example.Pair", "card:plugin:org.example.Weather"]);
        let none = Plugins::default();
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        let said = none.view(&Kit::new(events.clone(), asks, None, None)).said();
        assert!(said.contains(&"None installed".to_string()));
    }

    #[test]
    fn every_kind_of_field_draws_in_every_state() {
        let mut long = pair();
        for f in &mut long.fields {
            f.label = "W".repeat(plugins::NAME_MAX);
        }
        long.name = "M".repeat(plugins::NAME_MAX);
        long.id = "org.example.Long".into();
        for allowed in [0, 2, 3] {
            for (w, h) in [(1280.0, 800.0), (400.0, 800.0)] {
                let mut me = page_of(vec![pair(), long.clone()], allowed);
                me.choices.insert(("org.example.Pair".into(), "found".into()),
                                  (0..plugins::CHOICES_MAX).map(|n| (format!("v{n}"), "L".repeat(plugins::CHOICE_TEXT_MAX))).collect());
                let _ = draw(me, w, h, 1.0);
            }
        }
        let mut bare = pair();
        bare.fields.clear();
        let _ = draw(page_of(vec![bare], 0), 1280.0, 800.0, 1.0);
    }

    #[test]
    #[ignore]
    fn shoot_plugin_cards() {
        let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/style-check".into()));
        std::fs::create_dir_all(&out).unwrap();
        let found = |me: &mut Plugins| {
            me.choices.insert(("org.example.Pair".into(), "found".into()),
                              vec![("a".into(), "Desk in the study".into()), ("b".into(), "Laptop".into())]);
        };
        let mut bare = pair();
        bare.id = "org.example.Bare".into();
        bare.name = "Bare".into();
        bare.fields.clear();
        let controls = page_of(vec![pair()], 0);
        let mut controlled = page_of(vec![pair(), bare.clone()], 0);
        set(&mut controlled, "role", "Is controlled by the other computer");
        let mut filled = page_of(vec![pair(), bare], 3);
        found(&mut filled);
        let mut narrow = page_of(vec![pair()], 2);
        found(&mut narrow);
        for (name, me, w, h) in [("controls-none-allowed", controls, 1280.0, 1000.0), ("controlled-none-allowed", controlled, 1280.0, 1000.0),
                                 ("all-allowed-list-filled", filled, 1280.0, 1000.0), ("narrow-some-allowed", narrow, 640.0, 1400.0)] {
            draw(me, w, h, 1.0).save_png(out.join(format!("showwhen-card-{name}.png"))).unwrap();
        }
    }
}
