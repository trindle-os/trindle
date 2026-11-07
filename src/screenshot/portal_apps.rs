//! The AppChooser portal.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

use zbus::zvariant::{OwnedObjectPath, Value};

use super::portal::{answer, DONE};
use super::portal_ask::{ask, cancelled, get, Answer, Done, Kit, Options, ACCEPT_ICON, CANCEL_ICON};
use crate::common::app_choice;
use crate::common::apps::{self, Entry};
use crate::toolkit::bus::{Asked, Offer, Props};
use crate::toolkit::t;
use crate::toolkit::ui::{App, Shown};
use crate::toolkit::widget::{button, column, entry, item, label, list, main_action, row, scroll, space, switch, title, Length, Node, Tone};

pub(super) const INTERFACE: &str = "org.freedesktop.impl.portal.AppChooser";
const XML: &str = r#"<interface name="org.freedesktop.impl.portal.AppChooser">
    <method name="ChooseApplication">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="as" name="choices" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <method name="UpdateChoices">
      <arg type="o" name="handle" direction="in"/>
      <arg type="as" name="choices" direction="in"/>
    </method>
  </interface>"#;

const SIZE: (u32, u32) = (440, 520);
const LIST: f32 = 300.0;

#[derive(Clone, Debug)]
pub(super) enum Msg {
    Search(String),
    Pick(usize),
    Always(bool),
    Accept,
    Cancel,
}

pub(super) struct Chooser {
    heading: String,
    kind: String,
    apps: Vec<Entry>,
    first: Vec<String>,
    query: String,
    picked: Option<String>,
    always: bool,
    token: Option<String>,
    done: Done,
    reset: String,
}

impl Chooser {
    fn shown(&self) -> Vec<&Entry> {
        self.apps.iter().filter(|a| app_choice::matches(a, &self.query)).collect()
    }

    fn target(&self) -> Option<&Entry> {
        let shown = self.shown();
        self.picked.as_ref().and_then(|id| shown.iter().find(|a| &a.id == id).copied()).or_else(|| shown.first().copied())
    }

    fn replace(&mut self, offered: &[String]) {
        self.apps = app_choice::ordered(&apps::entries(), offered, &self.kind, &self.first);
    }

    fn answer(&self) -> Option<Answer> {
        let app = self.target()?;
        if !self.kind.is_empty() {
            app_choice::remember(&self.kind, &app.id);
            if self.always {
                let _ = app_choice::always(&self.kind, &app.id);
            }
        }
        let mut results = HashMap::from([("choice", Value::from(app.key().to_string()))]);
        if let Some(token) = &self.token {
            results.insert("activation_token", Value::from(token.clone()));
        }
        Some(answer(DONE, results))
    }
}

impl App for Chooser {
    type Msg = Msg;

    fn update(&mut self, msg: Msg) {
        match msg {
            Msg::Search(words) => self.query = words,
            Msg::Pick(i) => self.picked = self.shown().get(i).map(|a| a.id.clone()),
            Msg::Always(on) => self.always = on,
            Msg::Accept => {
                if let Some(outcome) = self.answer() {
                    (self.done)(outcome);
                }
            }
            Msg::Cancel => (self.done)(cancelled()),
        }
    }

    fn view(&self) -> Node<Msg> {
        let shown = self.shown();
        let target = self.target().map(|a| a.id.clone());
        let chosen = target.as_ref().and_then(|id| shown.iter().position(|a| &a.id == id));
        let rows = shown.iter().map(|a| item(a.name.clone()).icon(a.icon.clone().unwrap_or_else(|| crate::common::apps::generic_icon_for(&a.id, &a.name)))).collect();
        let mut all = vec![
            title(self.heading.clone()),
            entry(self.query.clone(), t!("screenshot-apps-search"), Msg::Search).width(Length::Fill).on_submit(Msg::Accept).first(),
        ];
        all.push(match shown.is_empty() {
            true => label(t!("screenshot-apps-none")).height(Length::Fixed(LIST)),
            false => scroll(format!("apps:{}", self.query), list(rows, chosen, Msg::Pick).on_open(|_| Msg::Accept)).height(Length::Fixed(LIST)),
        });
        if !self.reset.is_empty() {
            all.push(label(self.reset.clone()).tone(Tone::Problem));
        }
        if !self.kind.is_empty() {
            let words = t!("screenshot-apps-always");
            all.push(row(vec![label(words), space(), switch(self.always, words, Msg::Always)]).gap(12.0));
        }
        all.push(row(vec![
            space(),
            button(CANCEL_ICON, t!("screenshot-portal-cancel")).on_press(Msg::Cancel),
            main_action(ACCEPT_ICON, t!("screenshot-portal-open")).on_press(Msg::Accept).off(target.is_none()),
        ]).gap(8.0));
        column(all).gap(12.0).padding(16.0).on_escape(Msg::Cancel)
    }
}

thread_local! {
    static OPEN: RefCell<HashMap<String, Weak<Shown<Chooser>>>> = RefCell::new(HashMap::new());
}

fn heading_for(options: &Options) -> String {
    let file = get::<String>(options, "filename").filter(|f| !f.is_empty())
        .or_else(|| get::<String>(options, "uri").and_then(|u| u.rsplit('/').next().map(str::to_string)).filter(|f| !f.is_empty()))
        .map(|f| crate::toolkit::chooser::from_uri(&format!("file:///{f}")).and_then(|p| p.file_name().map(|n| n.to_string_lossy().into_owned())).unwrap_or(f));
    match file {
        Some(name) => t!("screenshot-apps-open-with", name = name),
        None => t!("screenshot-apps-choose").into(),
    }
}

fn choose(kit: &Kit, asked: Asked) {
    let Ok((handle, _app_id, parent, offered, options)) = asked.message().body()
        .deserialize::<(OwnedObjectPath, String, String, Vec<String>, Options)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "ChooseApplication takes a handle, an app, a window, choices and options");
    };
    let kind = get::<String>(&options, "content_type").unwrap_or_default();
    let first: Vec<String> = get::<String>(&options, "last_choice").filter(|c| !c.is_empty()).into_iter().collect();
    let heading = heading_for(&options);
    let apps = app_choice::ordered(&apps::entries(), &offered, &kind, &first);
    let reset = crate::common::damaged::take_notes().join(" ");
    let token = get::<String>(&options, "activation_token").filter(|t| !t.is_empty());
    let key = handle.as_str().to_string();
    let shown = ask(kit, asked, &key, &parent, &heading.clone(), SIZE, move |done| Chooser {
        heading, kind, apps, first, query: String::new(), picked: None, always: false, token, done, reset,
    });
    OPEN.with(|open| {
        let mut open = open.borrow_mut();
        open.retain(|_, w| w.strong_count() > 0);
        if let Some(shown) = shown {
            open.insert(key, Rc::downgrade(&shown));
        }
    });
}

fn update_choices(asked: Asked) {
    let Ok((handle, offered)) = asked.message().body().deserialize::<(OwnedObjectPath, Vec<String>)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "UpdateChoices takes a handle and choices");
    };
    let shown = OPEN.with(|open| open.borrow().get(handle.as_str()).and_then(Weak::upgrade));
    if let Some(shown) = shown {
        shown.change(|c| c.replace(&offered));
    }
    asked.reply(&());
}

pub fn offer(kit: &Kit) -> Offer {
    let kit = kit.clone();
    Offer {
        name: INTERFACE.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| match asked.member().as_str() {
            "ChooseApplication" => choose(&kit, asked),
            "UpdateChoices" => update_choices(asked),
            _ => asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "Not part of AppChooser"),
        }),
        props: Box::new(Props::new),
        set: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use super::super::portal::FAILED;
    use super::super::portal_ask::testkit::{call, Reply, Rig};
    use zbus::zvariant::ObjectPath;

    fn app(id: &str, name: &str) -> Entry {
        Entry {
            id: format!("{id}.desktop"), name: name.into(), icon: None, exec: id.into(), try_exec: None,
            wm_class: String::new(), program: id.into(), kinds: vec![], path: PathBuf::from("/nowhere"),
            category: "", keywords: vec![], flatpak: None, appimage: None, terminal: false,
        }
    }

    #[test]
    fn the_chooser_opens_what_is_picked_or_the_first_in_sight() {
        let got: Rc<RefCell<Option<Answer>>> = Rc::new(RefCell::new(None));
        let g = got.clone();
        let mut c = Chooser {
            heading: "Choose an app".into(), kind: String::new(), apps: vec![app("gimp", "GIMP"), app("org.gnome.eog", "Image Viewer")],
            first: vec![], query: String::new(), picked: None, always: false, token: Some("t1".into()),
            done: Rc::new(move |a| *g.borrow_mut() = Some(a)), reset: String::new(),
        };
        assert_eq!(c.target().map(|a| a.key()), Some("gimp"));
        c.update(Msg::Pick(1));
        c.update(Msg::Search("view".into()));
        assert_eq!(c.shown().len(), 1);
        c.update(Msg::Search("nothing like it".into()));
        assert!(c.target().is_none());
        c.update(Msg::Accept);
        assert!(got.borrow().is_none(), "nothing in sight, nothing opened");
        c.update(Msg::Search(String::new()));
        c.update(Msg::Accept);
        let (code, results) = got.borrow_mut().take().expect("answered");
        assert_eq!(code, DONE);
        assert_eq!(results.get("choice"), Some(&Value::from("org.gnome.eog")));
        assert_eq!(results.get("activation_token"), Some(&Value::from("t1")));
    }

    #[test]
    fn the_chooser_of_an_app_draws_only_pairs_the_table_holds() {
        use crate::toolkit::theme::{drawn, Theme};
        drawn::none(drawn::in_every_look("the chooser of an app", 6, || {
            for (state, kind, query, picked, always) in [("no kind of file", "", "", None, false), ("a picture, one picked", "image/png", "", Some(1), true),
                                                         ("searching", "image/png", "view", None, false), ("nothing found", "", "nothing like it", None, false)] {
                let mut c = Chooser {
                    heading: "Open Sea view.png with".into(), kind: kind.into(), apps: vec![app("gimp", "GIMP"), app("org.gnome.eog", "Image Viewer")],
                    first: vec![], query: query.into(), picked: None, always, token: None, done: Rc::new(|_| {}), reset: String::new(),
                };
                if let Some(i) = picked {
                    c.update(Msg::Pick(i));
                }
                drawn::window(state, c.view(), Theme::chosen(), SIZE, false);
            }
        }));
    }

    #[test]
    fn the_heading_names_the_file() {
        let options: Options = HashMap::from([("uri".to_string(), Value::from("file:///home/john/Sea%20view.png").try_into().expect("v"))]);
        assert_eq!(heading_for(&options), "Open Sea view.png with");
        assert_eq!(heading_for(&Options::new()), "Choose an app");
    }

    #[test]
    fn with_no_screen_it_fails_and_updates_are_taken() {
        let rig = Rig::new();
        let _c = rig.a.serve(super::super::portal::PATH, offer(&rig.kit())).expect("serve");
        let handle = ObjectPath::try_from("/org/freedesktop/portal/desktop/request/1_1/c1").expect("path");
        let none: HashMap<&str, Value> = HashMap::new();
        let r: Reply = call(&rig, INTERFACE, "ChooseApplication", &(&handle, "", "", vec!["gimp"], &none)).expect("reply");
        assert_eq!(r.0, FAILED);
        let r: Option<()> = call(&rig, INTERFACE, "UpdateChoices", &(&handle, vec!["gimp"]));
        assert!(r.is_some());
        assert_eq!(rig.stay.busy.get(), 0);
    }
}
