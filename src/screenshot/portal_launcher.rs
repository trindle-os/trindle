//! The DynamicLauncher portal.

use std::collections::HashMap;
use std::rc::Rc;

use zbus::zvariant::{OwnedObjectPath, OwnedValue, Value};

use super::portal::{answer, DONE};
use super::portal_ask::{app_name, ask, get, picture_from, Confirm, Kit, Options, Part, SIZE};
use crate::toolkit::bus::{Asked, Offer, Props};
use crate::toolkit::t;

pub(super) const INTERFACE: &str = "org.freedesktop.impl.portal.DynamicLauncher";
const XML: &str = r#"<interface name="org.freedesktop.impl.portal.DynamicLauncher">
    <method name="PrepareInstall">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="s" name="name" direction="in"/>
      <arg type="v" name="icon_v" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
    <method name="RequestInstallToken">
      <arg type="s" name="app_id" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
    </method>
    <property name="SupportedLauncherTypes" type="u" access="read"/>
    <property name="version" type="u" access="read"/>
  </interface>"#;

const TYPES: u32 = 1 | 2;
const WEB: u32 = 2;
const ICON: f32 = 64.0;

fn icon_bytes(v: &Value) -> Option<Vec<u8>> {
    match v {
        Value::Value(inner) => icon_bytes(inner),
        Value::Structure(s) => match s.fields() {
            [Value::Str(kind), data] if kind.as_str() == "bytes" => icon_bytes(data),
            _ => None,
        },
        Value::Array(_) => Vec::<u8>::try_from(v.try_clone().ok()?).ok(),
        _ => None,
    }
}

fn parts(who: &str, name: &str, icon: Option<&[u8]>, options: &Options) -> Vec<Part> {
    let heading = match who.is_empty() {
        true => t!("screenshot-launcher-ask").to_string(),
        false => t!("screenshot-launcher-asks", who = who),
    };
    let mut parts = vec![Part::Heading(heading)];
    if let Some(px) = icon.and_then(|b| picture_from(b, ICON as u32 * 2)) {
        parts.push(Part::Picture { px: Rc::new(px), size: (ICON, ICON), cover: false, tip: t!("screenshot-launcher-icon").into() });
    }
    let editable = get::<bool>(options, "editable_name").unwrap_or(true);
    parts.push(Part::Name { text: name.trim().to_string(), editable });
    let kind = get::<u32>(options, "launcher_type").unwrap_or(1);
    if kind & WEB != 0 {
        if let Some(target) = get::<String>(options, "target").filter(|t| !t.is_empty()) {
            parts.push(Part::Quiet(t!("screenshot-launcher-opens", target = &target)));
        }
    }
    parts
}

fn name_of(parts: &[Part]) -> String {
    parts.iter().find_map(|p| match p {
        Part::Name { text, .. } => Some(text.trim().to_string()),
        _ => None,
    }).unwrap_or_default()
}

fn prepare(kit: &Kit, asked: Asked) {
    let Ok((handle, app_id, parent, name, icon, options)) = asked.message().body()
        .deserialize::<(OwnedObjectPath, String, String, String, OwnedValue, Options)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "PrepareInstall takes a handle, an app, a window, a name, an icon and options");
    };
    let bytes = icon_bytes(&Value::from(icon.try_clone().unwrap_or_else(|_| OwnedValue::from(0u8))));
    let parts = parts(&app_name(&app_id), &name, bytes.as_deref(), &options);
    ask(kit, asked, handle.as_str(), &parent, t!("screenshot-launcher-title"), SIZE, move |done| {
        Confirm::new(parts, t!("screenshot-portal-cancel"), t!("screenshot-launcher-add"), done, move |parts| {
            let icon = icon.try_clone().map(Value::from).unwrap_or_else(|_| Value::from(""));
            answer(DONE, HashMap::from([
                ("name", Value::from(name_of(parts))),
                ("icon", Value::Value(Box::new(icon))),
            ]))
        })
    });
}

pub fn offer(kit: &Kit) -> Offer {
    let kit = kit.clone();
    Offer {
        name: INTERFACE.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| match asked.member().as_str() {
            "PrepareInstall" => prepare(&kit, asked),
            "RequestInstallToken" => asked.reply(&(2u32,)),
            _ => asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "Not part of DynamicLauncher"),
        }),
        props: Box::new(|| {
            let mut props = Props::new();
            props.insert("SupportedLauncherTypes".into(), OwnedValue::from(TYPES));
            props.insert("version".into(), OwnedValue::from(1u32));
            props
        }),
        set: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::portal::FAILED;
    use super::super::portal_ask::testkit::{call, props, Reply, Rig};
    use zbus::zvariant::{ObjectPath, Structure};

    fn png() -> Vec<u8> {
        let mut out = std::io::Cursor::new(Vec::new());
        image::RgbaImage::from_pixel(4, 4, image::Rgba([1, 2, 3, 255])).write_to(&mut out, image::ImageFormat::Png).expect("png");
        out.into_inner()
    }

    fn gicon(bytes: &[u8]) -> Value<'static> {
        Value::from(Structure::from(("bytes", Value::from(bytes.to_vec()))))
    }

    #[test]
    fn a_launcher_shows_its_name_icon_and_address() {
        let bytes = png();
        assert_eq!(icon_bytes(&gicon(&bytes)), Some(bytes.clone()));
        assert_eq!(icon_bytes(&Value::Value(Box::new(gicon(&bytes)))), Some(bytes.clone()));
        assert_eq!(icon_bytes(&Value::from("firefox")), None);
        let options: Options = HashMap::from([
            ("launcher_type".to_string(), OwnedValue::from(2u32)),
            ("target".to_string(), Value::from("https://example.com").try_into().expect("v")),
            ("editable_name".to_string(), OwnedValue::from(false)),
        ]);
        let shown = parts("", " Example ", Some(&bytes), &options);
        assert!(matches!(&shown[0], Part::Heading(_)));
        assert!(matches!(&shown[1], Part::Picture { .. }));
        assert!(matches!(&shown[2], Part::Name { text, editable: false } if text == "Example"));
        assert!(matches!(&shown[3], Part::Quiet(t) if t == "Opens https://example.com"));
        assert_eq!(name_of(&shown), "Example");
    }

    #[test]
    fn no_token_is_given_and_with_no_screen_nothing_is_added() {
        let rig = Rig::new();
        let _l = rig.a.serve(super::super::portal::PATH, offer(&rig.kit())).expect("serve");
        let none: HashMap<&str, Value> = HashMap::new();
        let r: (u32,) = call(&rig, INTERFACE, "RequestInstallToken", &("org.example.App", &none)).expect("reply");
        assert_eq!(r.0, 2);
        let handle = ObjectPath::try_from("/org/freedesktop/portal/desktop/request/1_1/l1").expect("path");
        let r: Reply = call(&rig, INTERFACE, "PrepareInstall", &(&handle, "org.example.App", "", "Example", gicon(&png()), &none)).expect("reply");
        assert_eq!(r.0, FAILED);
        assert!(r.1.is_empty());
        let p = props(&rig, INTERFACE).expect("props");
        assert_eq!(u32::try_from(p.get("SupportedLauncherTypes").expect("types").try_clone().expect("v")).ok(), Some(3));
        assert_eq!(rig.stay.busy.get(), 0);
    }
}
