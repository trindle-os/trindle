//! The Access and Account portals.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use zbus::zvariant::{OwnedObjectPath, Value};

use super::portal::{answer, DONE};
use super::portal_ask::{app_name, ask, chosen, choices_of, get, picture_from, plain_label, Confirm, Kit, Options, Part, SIZE};
use crate::toolkit::bus::{Asked, Offer, Props};
use crate::toolkit::t;

pub(super) const ACCESS: &str = "org.freedesktop.impl.portal.Access";
const ACCESS_XML: &str = r#"<interface name="org.freedesktop.impl.portal.Access">
    <method name="AccessDialog">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="s" name="title" direction="in"/>
      <arg type="s" name="subtitle" direction="in"/>
      <arg type="s" name="body" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
  </interface>"#;

pub(super) const ACCOUNT: &str = "org.freedesktop.impl.portal.Account";
const ACCOUNT_XML: &str = r#"<interface name="org.freedesktop.impl.portal.Account">
    <method name="GetUserInformation">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="window" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
  </interface>"#;

const FACE: f32 = 96.0;

fn label_or(options: &Options, key: &str, fallback: &str) -> String {
    get::<String>(options, key).map(|w| plain_label(&w)).filter(|w| !w.trim().is_empty()).unwrap_or_else(|| fallback.into())
}

fn question(title: &str, subtitle: &str, body: &str, options: &Options) -> Vec<Part> {
    let mut parts = Vec::new();
    if let Some(icon) = get::<String>(options, "icon").filter(|i| !i.is_empty()) {
        parts.push(Part::Icon(icon));
    }
    parts.push(Part::Heading(title.into()));
    if !subtitle.is_empty() {
        parts.push(Part::Words(subtitle.into()));
    }
    if !body.is_empty() {
        parts.push(Part::Quiet(body.into()));
    }
    parts.extend(choices_of(options).into_iter().map(|(id, label, options, value)| Part::Pick { id, label, options, value }));
    parts
}

fn access(kit: &Kit, asked: Asked) {
    let Ok((handle, _app_id, parent, title, subtitle, body, options)) = asked.message().body()
        .deserialize::<(OwnedObjectPath, String, String, String, String, String, Options)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "AccessDialog takes a handle, an app, a window, three lines of words and options");
    };
    let parts = question(&title, &subtitle, &body, &options);
    let (deny, grant) = (label_or(&options, "deny_label", t!("screenshot-portal-deny")), label_or(&options, "grant_label", t!("screenshot-portal-allow")));
    ask(kit, asked, handle.as_str(), &parent, &title, SIZE, move |done| Confirm::new(parts, &deny, &grant, done, |parts| {
        let mut results = HashMap::new();
        let picked = chosen(parts);
        if !picked.is_empty() {
            results.insert("choices", Value::from(picked));
        }
        answer(DONE, results)
    }));
}

struct Me {
    id: String,
    name: String,
    picture: Option<PathBuf>,
}

impl Me {
    fn find() -> Me {
        let id = crate::common::files::user();
        let name = full_name().filter(|n| !n.trim().is_empty()).unwrap_or_else(|| id.clone());
        let home = crate::common::files::home();
        let picture = crate::common::faces::path_for(&id)
            .or_else(|| Some(PathBuf::from("/var/lib/AccountsService/icons").join(&id)).filter(|p| !id.is_empty() && p.is_file()))
            .or_else(|| Some(home.join(".face")).filter(|p| p.is_file()));
        Me { id, name, picture }
    }
}

const FACE_MOST: u64 = 16 << 20;

fn picture_bytes(path: &Path) -> Option<Vec<u8>> {
    use std::io::Read;
    use std::os::unix::fs::OpenOptionsExt;
    let file = std::fs::OpenOptions::new().read(true).custom_flags(libc::O_NONBLOCK | libc::O_CLOEXEC).open(path).ok()?;
    if !file.metadata().ok()?.is_file() {
        return None;
    }
    let mut bytes = Vec::new();
    file.take(FACE_MOST + 1).read_to_end(&mut bytes).ok()?;
    (bytes.len() as u64 <= FACE_MOST).then_some(bytes)
}

fn full_name() -> Option<String> {
    // SAFETY: getpwuid returns null or an entry that is read at once, before
    // anything else could call it; pw_gecos is null or NUL-terminated.
    unsafe {
        let p = libc::getpwuid(libc::getuid());
        if p.is_null() || (*p).pw_gecos.is_null() {
            return None;
        }
        let all = std::ffi::CStr::from_ptr((*p).pw_gecos).to_string_lossy().to_string();
        all.split(',').next().map(str::to_string)
    }
}

fn account(kit: &Kit, asked: Asked) {
    let Ok((handle, app_id, parent, options)) = asked.message().body().deserialize::<(OwnedObjectPath, String, String, Options)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "GetUserInformation takes a handle, an app, a window and options");
    };
    let me = Me::find();
    let who = app_name(&app_id);
    let heading = if who.is_empty() { t!("screenshot-account-asks-unknown").to_string() } else { t!("screenshot-account-asks", who = &who) };
    let mut parts = vec![Part::Heading(heading.clone())];
    if let Some(reason) = get::<String>(&options, "reason").filter(|r| !r.trim().is_empty()) {
        parts.push(Part::Quiet(reason));
    }
    let px = me.picture.as_ref().and_then(|p| picture_bytes(p)).and_then(|b| picture_from(&b, FACE as u32 * 2));
    if let Some(px) = px {
        parts.push(Part::Picture { px: Rc::new(px), size: (FACE, FACE), cover: true, tip: t!("screenshot-account-picture").into() });
    }
    parts.push(Part::Words(me.name.clone()));
    parts.push(Part::Quiet(t!("screenshot-account-signed-in", id = &me.id)));
    ask(kit, asked, handle.as_str(), &parent, &heading, SIZE, move |done| Confirm::new(parts, t!("screenshot-portal-deny"), t!("screenshot-portal-allow"), done, move |_| {
        let image = me.picture.as_deref().map(crate::session::folders::file_uri).unwrap_or_default();
        answer(DONE, HashMap::from([
            ("id", Value::from(me.id.clone())),
            ("name", Value::from(me.name.clone())),
            ("image", Value::from(image)),
        ]))
    }));
}

pub fn offer_access(kit: &Kit) -> Offer {
    let kit = kit.clone();
    Offer {
        name: ACCESS.into(),
        xml: ACCESS_XML.into(),
        call: Box::new(move |asked: Asked| match asked.member().as_str() {
            "AccessDialog" => access(&kit, asked),
            _ => asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "Not part of Access"),
        }),
        props: Box::new(Props::new),
        set: None,
    }
}

pub fn offer_account(kit: &Kit) -> Offer {
    let kit = kit.clone();
    Offer {
        name: ACCOUNT.into(),
        xml: ACCOUNT_XML.into(),
        call: Box::new(move |asked: Asked| match asked.member().as_str() {
            "GetUserInformation" => account(&kit, asked),
            _ => asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "Not part of Account"),
        }),
        props: Box::new(Props::new),
        set: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::portal::FAILED;
    use super::super::portal_ask::testkit::{call, Reply, Rig};
    use zbus::zvariant::ObjectPath;

    #[test]
    fn a_face_picture_is_read_only_if_it_is_a_plain_file_of_a_fair_size() {
        let dir = std::env::temp_dir().join(format!("trindle-face-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let plain = dir.join("face");
        std::fs::write(&plain, b"picture").unwrap();
        assert_eq!(picture_bytes(&plain).unwrap(), b"picture");
        std::os::unix::fs::symlink(&plain, dir.join("link")).unwrap();
        assert_eq!(picture_bytes(&dir.join("link")).unwrap(), b"picture", "a link the picture is kept by is followed");
        let pipe = std::ffi::CString::new(dir.join("pipe").to_str().unwrap()).unwrap();
        // SAFETY: a NUL-terminated name that lives to the end of the call.
        assert_eq!(unsafe { libc::mkfifo(pipe.as_ptr(), 0o600) }, 0);
        assert!(picture_bytes(&dir.join("pipe")).is_none(), "a pipe was read");
        let big = std::fs::File::create(dir.join("big")).unwrap();
        big.set_len(FACE_MOST + 1).unwrap();
        assert!(picture_bytes(&dir.join("big")).is_none(), "more than the most was taken");
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_question_shows_what_the_program_sent() {
        let options: Options = HashMap::from([
            ("icon".to_string(), Value::from("camera-web-symbolic").try_into().expect("v")),
            ("deny_label".to_string(), Value::from("_Not now").try_into().expect("v")),
            ("choices".to_string(), Value::from(vec![
                ("remember", "Remember", Vec::<(&str, &str)>::new(), "maybe"),
                ("res", "Size", vec![("s", "Small"), ("l", "Large")], "l"),
            ]).try_into().expect("v")),
        ]);
        let parts = question("Use the camera?", "", "Pictures stay here.", &options);
        assert!(matches!(&parts[0], Part::Icon(i) if i == "camera-web-symbolic"));
        assert!(matches!(&parts[1], Part::Heading(t) if t == "Use the camera?"));
        assert!(matches!(&parts[2], Part::Quiet(_)), "an empty subtitle is left out");
        assert_eq!(chosen(&parts), vec![("remember".into(), "false".into()), ("res".into(), "l".into())]);
        assert_eq!(label_or(&options, "deny_label", "Deny"), "Not now");
        assert_eq!(label_or(&options, "grant_label", "Allow"), "Allow");
    }

    #[test]
    fn with_no_screen_both_answer_failed_and_tell_nothing() {
        let rig = Rig::new();
        let kit = rig.kit();
        let _a = rig.a.serve(super::super::portal::PATH, offer_access(&kit)).expect("serve");
        let _b = rig.a.serve(super::super::portal::PATH, offer_account(&kit)).expect("serve");
        let handle = ObjectPath::try_from("/org/freedesktop/portal/desktop/request/1_1/t1").expect("path");
        let none: HashMap<&str, Value> = HashMap::new();
        let r: Reply = call(&rig, ACCESS, "AccessDialog", &(&handle, "org.example.App", "", "Title", "", "", &none)).expect("reply");
        assert_eq!(r.0, FAILED);
        let r: Reply = call(&rig, ACCOUNT, "GetUserInformation", &(&handle, "org.example.App", "", &none)).expect("reply");
        assert_eq!(r.0, FAILED);
        assert!(r.1.is_empty());
        assert_eq!(rig.stay.busy.get(), 0);
    }
}
