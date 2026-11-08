//! The properties of every object, read and set.

use std::collections::HashMap;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};
use crate::keyring::store::LOGIN;
use crate::toolkit::bus::Props;
use super::{COLLECTION, ITEM, Keyring, PORTAL, SERVICE, Target, collection_path, item_path, op, props};

impl Keyring {

    pub(super) fn props_for(&self, path: &str, iface: &str) -> Props {
        let locked = self.locked();
        let pairs = match (self.target(path), iface) {
            (Some(Target::Portal), PORTAL) => vec![("version", Value::from(1u32))],
            (Some(Target::Service), SERVICE) => vec![("Collections", Value::from(self.shelves().keys()
                .map(|c| op(&collection_path(c))).collect::<Vec<_>>()))],
            (Some(Target::Collection(c)), COLLECTION) => {
                let items: Vec<OwnedObjectPath> = self.shelves().get(&c).into_iter().flatten()
                    .map(|id| op(&item_path(&c, *id))).collect();
                let guard = self.store.borrow();
                let shelf = guard.as_ref().and_then(|s| s.index.collections.get(&c));
                let label = shelf.map_or(if c == LOGIN { "Login" } else { "" }, |sh| &sh.label).to_string();
                let (created, modified) = shelf.map_or((0, 0), |sh| (sh.created, sh.modified));
                vec![
                    ("Items", Value::from(items)),
                    ("Label", Value::from(label)),
                    ("Locked", Value::from(locked)),
                    ("Created", Value::from(created)),
                    ("Modified", Value::from(modified)),
                ]
            }
            (Some(Target::Item(c, id)), ITEM) => {
                let guard = self.store.borrow();
                let item = guard.as_ref().and_then(|s| s.item(&c, id).ok());
                let attributes: HashMap<String, String> = item.map(|i| i.attributes.iter()
                    .map(|(k, v)| (k.clone(), v.clone())).collect()).unwrap_or_default();
                vec![
                    ("Locked", Value::from(locked)),
                    ("Attributes", Value::from(attributes)),
                    ("Label", Value::from(item.map_or("", |i| &i.label).to_string())),
                    ("Type", Value::from(item.map_or("", |i| &i.content_type).to_string())),
                    ("Created", Value::from(item.map_or(0, |i| i.created))),
                    ("Modified", Value::from(item.map_or(0, |i| i.modified))),
                ]
            }
            _ => Vec::new(),
        };
        props(pairs)
    }

    pub(super) fn set_property(&self, path: &str, iface: &str, prop: &str, value: OwnedValue) -> bool {
        let done = match (self.target(path), iface, prop) {
            (Some(Target::Collection(c)), COLLECTION, "Label") => match String::try_from(value) {
                Ok(label) => self.change(|s| s.set_collection_label(&c, &label)).is_ok(),
                Err(_) => false,
            },
            (Some(Target::Item(c, id)), ITEM, "Label" | "Attributes" | "Type") => {
                let ok = match prop {
                    "Label" => String::try_from(value)
                        .is_ok_and(|l| self.change(|s| s.update(&c, id, |i| i.label = l)).is_ok()),
                    "Type" => String::try_from(value)
                        .is_ok_and(|t| self.change(|s| s.update(&c, id, |i| i.content_type = t)).is_ok()),
                    _ => HashMap::<String, String>::try_from(value).is_ok_and(|a| {
                        self.change(|s| s.update(&c, id, |i| i.attributes = a.into_iter().collect())).is_ok()
                    }),
                };
                if ok {
                    self.item_changed(&c, id);
                }
                return ok;
            }
            _ => false,
        };
        if done {
            self.changed(path, iface, &[prop]);
        }
        done
    }
}
