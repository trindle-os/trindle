//! The Lockdown portal.

use zbus::zvariant::OwnedValue;

use crate::toolkit::bus::{Asked, Offer, Props};

pub(super) const INTERFACE: &str = "org.freedesktop.impl.portal.Lockdown";
const LOCKS: [&str; 7] = [
    "disable-printing",
    "disable-save-to-disk",
    "disable-application-handlers",
    "disable-location",
    "disable-camera",
    "disable-microphone",
    "disable-sound-output",
];
const XML: &str = r#"<interface name="org.freedesktop.impl.portal.Lockdown">
    <property name="disable-printing" type="b" access="readwrite"/>
    <property name="disable-save-to-disk" type="b" access="readwrite"/>
    <property name="disable-application-handlers" type="b" access="readwrite"/>
    <property name="disable-location" type="b" access="readwrite"/>
    <property name="disable-camera" type="b" access="readwrite"/>
    <property name="disable-microphone" type="b" access="readwrite"/>
    <property name="disable-sound-output" type="b" access="readwrite"/>
  </interface>"#;

pub fn offer() -> Offer {
    Offer {
        name: INTERFACE.into(),
        xml: XML.into(),
        call: Box::new(|asked: Asked| asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "Lockdown has no methods")),
        props: Box::new(|| LOCKS.iter().map(|k| (k.to_string(), OwnedValue::from(false))).collect::<Props>()),
        set: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::bus::get_bool;
    use super::super::portal_ask::testkit::{props, Rig};

    #[test]
    fn nothing_is_locked_down() {
        let rig = Rig::new();
        let _s = rig.a.serve(super::super::portal::PATH, offer()).expect("serve");
        let got = props(&rig, INTERFACE).expect("props");
        assert_eq!(got.len(), LOCKS.len());
        assert!(LOCKS.iter().all(|k| got.contains_key(*k) && !get_bool(&got, k)));
    }
}
