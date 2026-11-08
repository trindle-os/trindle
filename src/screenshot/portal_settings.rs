//! The desktop's settings for programs that ask the portal for them: dark or
//! light, fonts, icons, the cursor and the rest that GTK and libadwaita
//! programs read as they start. GTK's portal answered these before, which
//! woke a 36 MB program for every app opened and kept it running for days;
//! the answers come from Trindle's own files here, and each change to them
//! is told to every program.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use zbus::zvariant::{OwnedValue, Value as Variant};

use crate::common::files;
use crate::common::look::{self, Entry, Value};
use crate::toolkit::bus::{Asked, Bus, Offer, Props};
use crate::toolkit::event::{Flow, Loop};
use crate::toolkit::watch::Watch;

pub const INTERFACE: &str = "org.freedesktop.impl.portal.Settings";
pub const XML: &str = r#"<interface name="org.freedesktop.impl.portal.Settings">
    <method name="ReadAll">
      <arg type="as" name="namespaces" direction="in"/>
      <arg type="a{sa{sv}}" name="value" direction="out"/>
    </method>
    <method name="Read">
      <arg type="s" name="namespace" direction="in"/>
      <arg type="s" name="key" direction="in"/>
      <arg type="v" name="value" direction="out"/>
    </method>
    <signal name="SettingChanged">
      <arg type="s" name="namespace"/>
      <arg type="s" name="key"/>
      <arg type="v" name="value"/>
    </signal>
    <property name="version" type="u" access="read"/>
  </interface>"#;
pub const VERSION: u32 = 1;

const NOT_FOUND: &str = "org.freedesktop.portal.Error.NotFound";

pub struct Settings {
    now: RefCell<Vec<Entry>>,

    qt: RefCell<String>,
}

impl Settings {
    pub fn new() -> Self {
        Self { now: RefCell::new(look::now()), qt: RefCell::new(look::write_qt()) }
    }

    fn everything(&self, wanted: &[String]) -> HashMap<String, HashMap<String, Variant<'static>>> {
        let mut all: HashMap<String, HashMap<String, Variant<'static>>> = HashMap::new();
        for (ns, key, value) in self.now.borrow().iter().filter(|e| wanted_namespace(e.0, wanted)) {
            all.entry(ns.to_string()).or_default().insert(key.to_string(), variant(value));
        }
        all
    }

    pub fn answer(&self, asked: Asked) {
        match asked.member().as_str() {
            "ReadAll" => {
                let wanted: Vec<String> = asked.message().body().deserialize().unwrap_or_default();
                let all = self.everything(&wanted);
                asked.reply(&all);
            }
            "Read" => {
                let (namespace, key): (String, String) = asked.message().body().deserialize().unwrap_or_default();
                let found = self.now.borrow().iter().find(|e| e.0 == namespace && e.1 == key).map(|e| variant(&e.2));
                match found {
                    Some(v) => asked.reply(&v),
                    None => asked.fail(NOT_FOUND, "Requested setting not found"),
                }
            }
            m => {
                let m = m.to_string();
                asked.fail("org.freedesktop.DBus.Error.UnknownMethod", &m)
            }
        }
    }

    pub fn offer(self: &Rc<Self>, mut asked: impl FnMut() + 'static) -> Offer {
        let me = self.clone();
        Offer {
            name: INTERFACE.into(),
            xml: XML.into(),
            call: Box::new(move |a| {
                asked();
                me.answer(a)
            }),
            props: Box::new(|| Props::from([("version".to_string(), OwnedValue::from(VERSION))])),
            set: None,
        }
    }

    pub fn watch(self: &Rc<Self>, bus: &Bus, events: &Loop, path: &'static str) {
        let Ok(mut watch) = Watch::new() else { return };
        for folder in [files::trindle_config(), files::config_home().join("gtk-4.0")] {
            let _ = std::fs::create_dir_all(&folder);
            let _ = watch.folder(&folder);
        }
        let fd = watch.fd();
        let (me, bus) = (self.clone(), bus.clone());
        events.watch(fd, move |hung_up| {
            if hung_up {
                return Flow::Stop;
            }
            if watch.changed().is_empty() {
                return Flow::Continue;
            }
            let after = look::now();
            let qt = look::write_qt();
            let changed = look::changed(&me.now.borrow(), &after);
            for (ns, key, value) in &changed {
                let _ = bus.emit(path, INTERFACE, "SettingChanged", &(*ns, *key, variant(value)));
            }

            let told = changed.iter().any(|e| e.0 == look::KDE_GENERAL);
            if qt != *me.qt.borrow() && !told {
                if let Some((ns, key, value)) = after.iter().find(|e| e.0 == look::KDE_GENERAL) {
                    let _ = bus.emit(path, INTERFACE, "SettingChanged", &(*ns, *key, variant(value)));
                }
            }
            *me.qt.borrow_mut() = qt;
            *me.now.borrow_mut() = after;
            Flow::Continue
        });
    }
}

fn variant(value: &Value) -> Variant<'static> {
    match value {
        Value::S(s) => Variant::from(s.clone()),
        Value::B(b) => Variant::from(*b),
        Value::I(i) => Variant::from(*i),
        Value::U(u) => Variant::from(*u),
        Value::D(d) => Variant::from(*d),
    }
}

fn wanted_namespace(namespace: &str, wanted: &[String]) -> bool {
    if wanted.is_empty() || (wanted.len() == 1 && wanted[0].is_empty()) {
        return true;
    }
    wanted.iter().any(|w| match w.strip_suffix('*') {
        Some(start) => namespace.starts_with(start),
        None => namespace == w,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_value_keeps_its_type() {
        let kind = |v: Value| variant(&v).value_signature().to_string();
        assert_eq!(<&str>::try_from(&variant(&Value::S("Adwaita".into()))).ok(), Some("Adwaita"));
        assert_eq!(kind(Value::U(1)), "u");
        assert_eq!(kind(Value::I(24)), "i");
        assert_eq!(kind(Value::D(1.0)), "d");
        assert_eq!(kind(Value::B(true)), "b");
    }

    #[test]
    fn the_namespaces_asked_for_are_the_ones_answered() {
        let ask = |list: &[&str]| list.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(wanted_namespace("org.gnome.desktop.interface", &[]));
        assert!(wanted_namespace("org.gnome.desktop.interface", &ask(&[""])));
        assert!(wanted_namespace("org.gnome.desktop.interface", &ask(&["org.gnome.*"])));
        assert!(wanted_namespace(look::APPEARANCE, &ask(&[look::APPEARANCE])));
        assert!(!wanted_namespace("org.gnome.desktop.interface", &ask(&[look::APPEARANCE])));
        assert!(!wanted_namespace("org.gnome.desktop.interfaceX", &ask(&["org.gnome.desktop.interface"])));
    }
}
