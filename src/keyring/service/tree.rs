//! The object tree: which paths are served, kept in line with the store, and what each path stands for.

use std::collections::{BTreeMap, BTreeSet, HashMap};
use serde::Serialize;
use zbus::zvariant::{DynamicType, OwnedValue, Value};
use crate::keyring::store::{DEFAULT_ALIAS, LOGIN};
use crate::toolkit::bus::{Failed, Offer};
use super::xml::xml_for;
use super::{ALIASES, COLLECTION, COLLECTIONS, INVALID, ITEM, Keyring, PORTAL_PATH, PROMPTS, ROOT, SERVICE, SESSIONS, Target, collection_path, item_path, op, path_safe};

impl Keyring {
    pub(super) fn serve(&self, path: &str, interfaces: &[&str]) -> Result<(), String> {
        let mut served = Vec::new();
        for name in interfaces {
            let xml = xml_for(name).ok_or_else(|| format!("no interface {name}"))?;
            let (call, get) = (self.me.clone(), self.me.clone());
            let (p1, i1, p2, i2) = (path.to_string(), name.to_string(), path.to_string(), name.to_string());
            let set: Option<Box<dyn FnMut(&str, OwnedValue) -> Result<(), Failed>>> = match *name {
                COLLECTION | ITEM => {
                    let (me, p, i) = (self.me.clone(), path.to_string(), name.to_string());
                    Some(Box::new(move |prop, value| match me.upgrade() {
                        Some(k) if k.set_property(&p, &i, prop, value) => Ok(()),
                        _ => Err(Failed::new(INVALID, "The property was not changed")),
                    }))
                }
                _ => None,
            };
            let offer = Offer {
                name: name.to_string(),
                xml: xml.into(),
                call: Box::new(move |asked| {
                    if let Some(k) = call.upgrade() {
                        k.call(&p1, &i1, asked);
                    }
                }),
                props: Box::new(move || get.upgrade().map(|k| k.props_for(&p2, &i2)).unwrap_or_default()),
                set,
            };
            served.push(self.bus.serve(path, offer).map_err(|e| e.to_string())?);
        }
        self.served.borrow_mut().insert(path.to_string(), served);
        Ok(())
    }

    pub(super) fn unserve(&self, path: &str) {
        let gone = self.served.borrow_mut().remove(path);
        drop(gone);
    }

    pub(super) fn emit<B: Serialize + DynamicType>(&self, path: &str, iface: &str, signal: &str, body: &B) {
        let _ = self.bus.emit(path, iface, signal, body);
    }

    pub(super) fn changed(&self, path: &str, iface: &str, wanted: &[&str]) {
        let all = self.props_for(path, iface);
        let values: HashMap<&str, Value<'_>> = wanted.iter()
            .filter_map(|p| all.get(*p).and_then(|v| v.try_clone().ok()).map(|v| (*p, Value::from(v)))).collect();
        let _ = self.bus.changed(path, iface, &values);
    }

    pub(super) fn shelves(&self) -> BTreeMap<String, Vec<u64>> {
        match self.store.borrow().as_ref() {
            Some(s) => s.index.collections.iter().filter(|(name, _)| path_safe(name))
                .map(|(name, shelf)| (name.clone(), shelf.items.keys().copied().collect())).collect(),
            None if self.damaged.get() => BTreeMap::new(),
            None => BTreeMap::from([(LOGIN.to_string(), Vec::new())]),
        }
    }

    pub(super) fn aliases(&self) -> BTreeMap<String, String> {
        match self.store.borrow().as_ref() {
            Some(s) => s.index.aliases.iter().filter(|(a, c)| path_safe(a) && path_safe(c))
                .map(|(a, c)| (a.clone(), c.clone())).collect(),
            None if self.damaged.get() => BTreeMap::new(),
            None => BTreeMap::from([(DEFAULT_ALIAS.to_string(), LOGIN.to_string())]),
        }
    }

    pub(super) fn sync(&self) {
        let shelves = self.shelves();
        let mut wanted: BTreeMap<String, &[&str]> = BTreeMap::new();
        for (name, items) in &shelves {
            wanted.insert(collection_path(name), &[COLLECTION]);
            for id in items {
                wanted.insert(item_path(name, *id), &[ITEM]);
            }
        }
        for alias in self.aliases().keys() {
            wanted.insert(format!("{ALIASES}{alias}"), &[COLLECTION]);
        }
        let had: BTreeSet<String> = self.served.borrow().keys()
            .filter(|p| p.starts_with(COLLECTIONS) || p.starts_with(ALIASES)).cloned().collect();
        let mut touched = BTreeSet::new();
        for gone in had.iter().filter(|p| !wanted.contains_key(*p)) {
            self.unserve(gone);
            self.announce(gone, "Deleted", &mut touched);
        }
        for (path, ifaces) in wanted.iter().filter(|(p, _)| !had.contains(*p)) {
            if let Err(e) = self.serve(path, ifaces) {
                eprintln!("trindle-keyring: {path}: {e}");
                continue;
            }
            self.announce(path, "Created", &mut touched);
        }
        for collection in touched {
            self.changed(&collection, COLLECTION, &["Items"]);
        }
    }

    fn announce(&self, path: &str, what: &str, touched: &mut BTreeSet<String>) {
        let Some(rest) = path.strip_prefix(COLLECTIONS) else { return };
        match rest.split_once('/') {
            Some((c, _)) => {
                let collection = collection_path(c);
                self.emit(&collection, COLLECTION, &format!("Item{what}"), &(op(path),));
                touched.insert(collection);
            }
            None => {
                self.emit(ROOT, SERVICE, &format!("Collection{what}"), &(op(path),));
                self.changed(ROOT, SERVICE, &["Collections"]);
            }
        }
    }

    pub(super) fn item_changed(&self, collection: &str, id: u64) {
        let path = item_path(collection, id);
        self.emit(&collection_path(collection), COLLECTION, "ItemChanged", &(op(&path),));
        self.changed(&path, ITEM, &["Label", "Attributes", "Modified"]);
    }

    pub(super) fn lock_changed(&self) {
        self.sync();
        for (name, items) in self.shelves() {
            let path = collection_path(&name);
            self.changed(&path, COLLECTION, &["Locked"]);
            self.emit(ROOT, SERVICE, "CollectionChanged", &(op(&path),));
            for id in items {
                self.changed(&item_path(&name, id), ITEM, &["Locked", "Label", "Attributes"]);
            }
        }

        if !self.locked() && !self.asking.borrow().is_empty() {
            self.finish_all(true);
        }
        self.tell();
    }

    pub(super) fn tell(&self) {
        if let Some(told) = self.told.borrow().as_ref() {
            told();
        }
    }

    pub(super) fn target(&self, path: &str) -> Option<Target> {
        if path == ROOT {
            return Some(Target::Service);
        }
        if path == PORTAL_PATH {
            return Some(Target::Portal);
        }
        if path.starts_with(SESSIONS) {
            return self.sessions.borrow().contains_key(path).then(|| Target::Session(path.to_string()));
        }
        if path.starts_with(PROMPTS) {
            return self.prompts.borrow().contains_key(path).then(|| Target::Prompt(path.to_string()));
        }
        if let Some(alias) = path.strip_prefix(ALIASES) {
            return self.aliases().get(alias).map(|c| Target::Collection(c.clone()));
        }
        let rest = path.strip_prefix(COLLECTIONS)?;
        let shelves = self.shelves();
        match rest.split_once('/') {
            None => shelves.contains_key(rest).then(|| Target::Collection(rest.to_string())),
            Some((c, id)) => {
                let id: u64 = id.parse().ok()?;
                shelves.get(c)?.contains(&id).then(|| Target::Item(c.to_string(), id))
            }
        }
    }
}
