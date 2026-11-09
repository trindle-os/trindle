//! The Secret Service methods programs call, matched by object, interface and method.

use std::collections::{BTreeMap, HashMap};
use zbus::Message;
use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};
use zeroize::Zeroizing;
use crate::keyring::store::Item;
use crate::toolkit::bus::Asked;
use crate::toolkit::t;
use super::{ATTRIBUTES, Asker, COLLECTION, FAILED, Fault, GNOME, INVALID, IS_LOCKED, ITEM, Keyring, LABEL_COLLECTION, LABEL_ITEM, MOST_HELD, NOT_SUPPORTED, NO_SESSION, NO_SUCH, PORTAL, PROMPT, SERVICE, SESSION, SESSIONS, Secret, Target, Work, answer, args, collection_path, fault, item_path, name_for, none, op, path_safe};

#[derive(Clone, Copy)]
struct Call<'a> {
    sender: &'a str,
    iface: &'a str,
    method: &'a str,
    msg: &'a Message,
}

impl Keyring {
    pub(super) fn call(&self, path: &str, iface: &str, asked: Asked) {
        if let Some(held) = self.held_back.borrow_mut().as_mut() {
            if held.len() < MOST_HELD {
                held.push((path.to_string(), iface.to_string(), asked));
            } else {
                asked.fail(FAILED, t!("keyring-starting"));
            }
            return;
        }
        let (msg, member, sender) = (asked.message().clone(), asked.member(), asked.sender());
        let mut asked = Some(asked);
        let done = match self.target(path) {
            None => Err(fault(NO_SUCH, "No such object")),
            Some(t) => self.dispatch(&Call { sender: &sender, iface, method: &member, msg: &msg }, t, &mut asked),
        };
        if let (Err(Fault(name, words)), Some(a)) = (done, asked.take()) {
            a.fail(name, &words);
        }
    }

    fn dispatch(&self, call: &Call, target: Target, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let Call { sender, iface, method, msg } = *call;
        match (target, iface, method) {
            (Target::Service, _, _) => self.service_method(call, asked),
            (Target::Portal, PORTAL, "RetrieveSecret") => self.retrieve_secret(msg, asked),
            (Target::Collection(c), _, _) => self.collection_method(&c, call, asked),
            (Target::Item(c, id), _, _) => self.item_method(&c, id, call, asked),
            (Target::Session(path), SESSION, "Close") => {
                self.sessions.borrow_mut().remove(&path);
                self.unserve(&path);
                answer(asked, &());
                Ok(())
            }
            (Target::Prompt(path), PROMPT, "Prompt") => {
                self.show_prompt(path, Asker::Bus(sender.to_string()));
                answer(asked, &());
                Ok(())
            }
            (Target::Prompt(path), PROMPT, "Dismiss") => {
                self.asking.borrow_mut().retain(|p| *p != path);
                self.complete(&path, false);

                if self.asking.borrow().is_empty() {
                    self.close_window();
                }
                answer(asked, &());
                Ok(())
            }
            _ => Err(fault(NOT_SUPPORTED, "Not a method this object has")),
        }
    }

    fn service_method(&self, call: &Call, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let Call { sender, iface, method, msg } = *call;
        match (iface, method) {
            (SERVICE, "OpenSession") => self.open_session(sender, msg, asked),
            (SERVICE, "CreateCollection") => self.create_collection_call(msg, asked),
            (SERVICE, "SearchItems") => self.search_all(msg, asked),
            (SERVICE, "Unlock") => self.unlock_call(msg, asked),
            (SERVICE, "Lock") => self.lock_objects(msg, asked),
            (SERVICE, "GetSecrets") => self.get_secrets(msg, asked),
            (SERVICE, "ReadAlias") => {
                let name: String = args(msg, "s")?;
                let found = self.aliases().get(&name).map_or_else(none, |c| op(&collection_path(c)));
                answer(asked, &(found,));
                Ok(())
            }
            (SERVICE, "SetAlias") => self.set_alias(msg, asked),
            (GNOME, "ChangeWithMasterPassword") => {
                let (_, original, master): (OwnedObjectPath, Secret, Secret) = args(msg, "A secret is (oayays)")?;
                let Some(a) = asked.take() else { return Ok(()) };
                self.change_password(Zeroizing::new(original.2), Zeroizing::new(master.2), a);
                Ok(())
            }
            (GNOME, "UnlockWithMasterPassword") => {
                let (_, master): (OwnedObjectPath, Secret) = args(msg, "A secret is (oayays)")?;
                let Some(a) = asked.take() else { return Ok(()) };
                self.unlock_with_password(Zeroizing::new(master.2), a);
                Ok(())
            }
            _ => Err(fault(NOT_SUPPORTED, "Not a method this object has")),
        }
    }

    fn search_all(&self, msg: &Message, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let wanted: HashMap<String, String> = args(msg, "a{ss}")?;
        let found = self.search(None, wanted);
        let locked = self.locked();
        let none = Vec::<OwnedObjectPath>::new();
        answer(asked, &if locked { (none, found) } else { (found, none) });
        Ok(())
    }

    fn lock_objects(&self, msg: &Message, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let objects: Vec<OwnedObjectPath> = args(msg, "ao")?;
        if let Some(s) = self.store.borrow_mut().as_mut() {
            s.lock();
        }
        self.lock_changed();
        answer(asked, &(objects, none()));
        Ok(())
    }

    fn get_secrets(&self, msg: &Message, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let (items, session): (Vec<OwnedObjectPath>, OwnedObjectPath) = args(msg, "(aoo)")?;
        self.check_session(&session)?;
        let mut out: HashMap<OwnedObjectPath, Secret> = HashMap::new();
        for item in items {
            if let Some(Target::Item(c, id)) = self.target(item.as_str()) {
                if let Ok(secret) = self.secret(&c, id, &session) {
                    out.insert(item, secret);
                }
            }
        }
        answer(asked, &(out,));
        Ok(())
    }

    fn set_alias(&self, msg: &Message, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let (name, collection): (String, OwnedObjectPath) = args(msg, "(so)")?;
        if !path_safe(&name) {
            return Err(fault(INVALID, "An alias is letters, digits and underscores"));
        }
        let to = match self.target(collection.as_str()) {
            Some(Target::Collection(c)) => Some(c),
            _ if collection.as_str() == "/" => None,
            _ => return Err(fault(NO_SUCH, "No such collection")),
        };
        self.change(|s| {
            match to {
                Some(c) => s.index.aliases.insert(name, c),
                None => s.index.aliases.remove(&name),
            };
            Ok(())
        })?;
        self.sync();
        answer(asked, &());
        Ok(())
    }

    fn collection_method(&self, c: &str, call: &Call, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let Call { iface, method, msg, .. } = *call;
        match (iface, method) {
            (COLLECTION, "Delete") => {
                self.change(|s| s.remove_collection(c))?;
                self.sync();
                answer(asked, &(none(),));
                Ok(())
            }
            (COLLECTION, "SearchItems") => {
                let wanted: HashMap<String, String> = args(msg, "a{ss}")?;
                answer(asked, &(self.search(Some(c), wanted),));
                Ok(())
            }
            (COLLECTION, "CreateItem") => self.create_item(c, msg, asked),
            _ => Err(fault(NOT_SUPPORTED, "Not a method this object has")),
        }
    }

    fn create_item(&self, c: &str, msg: &Message, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let (mut props, secret, replace): (HashMap<String, OwnedValue>, Secret, bool) =
            args(msg, "A secret is (oayays)")?;
        self.check_session(&secret.0)?;
        let attributes: BTreeMap<String, String> = props.remove(ATTRIBUTES)
            .and_then(|v| HashMap::<String, String>::try_from(v).ok()).unwrap_or_default().into_iter().collect();
        let item = Item {
            label: props.remove(LABEL_ITEM).and_then(|v| String::try_from(v).ok()).unwrap_or_default(),
            attributes,
            secret: secret.2,
            content_type: secret.3,
            created: 0,
            modified: 0,
        };
        let before = self.shelves().get(c).cloned().unwrap_or_default();
        let id = self.change(|s| s.put(c, item, replace))?;
        self.sync();
        if before.contains(&id) {
            self.item_changed(c, id);
        }
        answer(asked, &(op(&item_path(c, id)), none()));
        Ok(())
    }

    fn item_method(&self, c: &str, id: u64, call: &Call, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let Call { iface, method, msg, .. } = *call;
        match (iface, method) {
            (ITEM, "Delete") => {
                self.change(|s| s.remove(c, id))?;
                self.sync();
                answer(asked, &(none(),));
                Ok(())
            }
            (ITEM, "GetSecret") => {
                let session: OwnedObjectPath = args(msg, "o")?;
                self.check_session(&session)?;
                answer(asked, &(self.secret(c, id, &session)?,));
                Ok(())
            }
            (ITEM, "SetSecret") => {
                let secret: Secret = args(msg, "A secret is (oayays)")?;
                self.check_session(&secret.0)?;
                self.change(|s| s.update(c, id, |i| {
                    i.secret = secret.2;
                    i.content_type = secret.3;
                }))?;
                self.item_changed(c, id);
                answer(asked, &());
                Ok(())
            }
            _ => Err(fault(NOT_SUPPORTED, "Not a method this object has")),
        }
    }

    fn open_session(&self, sender: &str, msg: &Message, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let (algorithm, _): (String, OwnedValue) = args(msg, "(sv)")?;
        if algorithm != "plain" {
            return Err(fault(NOT_SUPPORTED, "Only plain sessions: the bus is private to this login"));
        }
        let path = format!("{SESSIONS}s{}", self.next());
        self.serve(&path, &[SESSION]).map_err(|e| Fault(FAILED, e))?;
        self.sessions.borrow_mut().insert(path.clone(), sender.to_string());
        answer(asked, &(Value::from(""), op(&path)));
        Ok(())
    }

    pub(super) fn watch_programs(&self) {
        let me = self.me.clone();
        let watch = self.bus.listen(Some("org.freedesktop.DBus"), "org.freedesktop.DBus",
            Some("NameOwnerChanged"), Some("/org/freedesktop/DBus"),
            move |signal| {
                let Ok((name, _, new)) = signal.body().deserialize::<(String, String, String)>() else { return };
                let Some(k) = me.upgrade() else { return };
                if !new.is_empty() || !name.starts_with(':') {
                    return;
                }
                let gone: Vec<String> = k.sessions.borrow().iter()
                    .filter(|(_, owner)| **owner == name).map(|(p, _)| p.clone()).collect();
                for path in gone {
                    k.sessions.borrow_mut().remove(&path);
                    k.unserve(&path);
                }
            });
        *self.watching.borrow_mut() = watch.ok();
    }

    fn check_session(&self, session: &OwnedObjectPath) -> Result<(), Fault> {
        if self.sessions.borrow().contains_key(session.as_str()) {
            Ok(())
        } else {
            Err(fault(NO_SESSION, "No such session"))
        }
    }

    fn secret(&self, collection: &str, id: u64, session: &OwnedObjectPath) -> Result<Secret, Fault> {
        let guard = self.store.borrow();
        let s = guard.as_ref().ok_or_else(|| fault(IS_LOCKED, "The keyring is locked"))?;
        let item = s.item(collection, id)?;
        let kind = if item.content_type.is_empty() { "text/plain" } else { &item.content_type };
        Ok((session.clone(), Vec::new(), item.secret.clone(), kind.to_string()))
    }

    fn search(&self, collection: Option<&str>, wanted: HashMap<String, String>) -> Vec<OwnedObjectPath> {
        let wanted: BTreeMap<String, String> = wanted.into_iter().collect();
        let guard = self.store.borrow();
        let Some(s) = guard.as_ref() else { return Vec::new() };
        s.search(collection, &wanted).into_iter().filter(|(c, _)| path_safe(c))
            .map(|(c, id)| op(&item_path(&c, id))).collect()
    }

    fn create_collection_call(&self, msg: &Message, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let (mut props, alias): (HashMap<String, OwnedValue>, String) = args(msg, "(a{sv}s)")?;
        if !alias.is_empty() && !path_safe(&alias) {
            return Err(fault(INVALID, "An alias is letters, digits and underscores"));
        }
        if let Some(c) = self.aliases().get(&alias) {
            answer(asked, &(op(&collection_path(c)), none()));
            return Ok(());
        }
        let label = props.remove(LABEL_COLLECTION).and_then(|v| String::try_from(v).ok()).unwrap_or_default();
        if !self.locked() {
            let path = self.create_collection(&label, &alias)?;
            answer(asked, &(path, none()));
            return Ok(());
        }
        if self.damaged.get() {
            return Err(fault(FAILED, "The keyring file is damaged"));
        }
        let prompt = self.new_prompt(Work::Create { label, alias })?;
        answer(asked, &(none(), prompt));
        Ok(())
    }

    pub(super) fn create_collection(&self, label: &str, alias: &str) -> Result<OwnedObjectPath, Fault> {
        let name = self.change(|s| {
            let name = name_for(label, |n| s.index.collections.contains_key(n));
            s.add_collection(&name, label)?;
            if !alias.is_empty() {
                s.index.aliases.insert(alias.to_string(), name.clone());
            }
            Ok(name)
        })?;
        self.sync();
        Ok(op(&collection_path(&name)))
    }

    fn unlock_call(&self, msg: &Message, asked: &mut Option<Asked>) -> Result<(), Fault> {
        let objects: Vec<OwnedObjectPath> = args(msg, "ao")?;
        if self.locked() && self.try_login_key() {
            self.lock_changed();
        }
        if !self.locked() {
            answer(asked, &(objects, none()));
            return Ok(());
        }
        if self.damaged.get() {
            return Err(fault(FAILED, "The keyring file is damaged"));
        }
        let prompt = self.new_prompt(Work::Unlock(objects))?;
        answer(asked, &(Vec::<OwnedObjectPath>::new(), prompt));
        Ok(())
    }
}
