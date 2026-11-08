//! The introspection XML of every object the Secret Service offers.

use super::{COLLECTION, GNOME, ITEM, PORTAL, PROMPT, SERVICE, SESSION};

const SERVICE_XML: &str = r#"<interface name="org.freedesktop.Secret.Service">
  <method name="OpenSession">
    <arg type="s" name="algorithm" direction="in"/>
    <arg type="v" name="input" direction="in"/>
    <arg type="v" name="output" direction="out"/>
    <arg type="o" name="result" direction="out"/>
  </method>
  <method name="CreateCollection">
    <arg type="a{sv}" name="properties" direction="in"/>
    <arg type="s" name="alias" direction="in"/>
    <arg type="o" name="collection" direction="out"/>
    <arg type="o" name="prompt" direction="out"/>
  </method>
  <method name="SearchItems">
    <arg type="a{ss}" name="attributes" direction="in"/>
    <arg type="ao" name="unlocked" direction="out"/>
    <arg type="ao" name="locked" direction="out"/>
  </method>
  <method name="Unlock">
    <arg type="ao" name="objects" direction="in"/>
    <arg type="ao" name="unlocked" direction="out"/>
    <arg type="o" name="prompt" direction="out"/>
  </method>
  <method name="Lock">
    <arg type="ao" name="objects" direction="in"/>
    <arg type="ao" name="locked" direction="out"/>
    <arg type="o" name="Prompt" direction="out"/>
  </method>
  <method name="GetSecrets">
    <arg type="ao" name="items" direction="in"/>
    <arg type="o" name="session" direction="in"/>
    <arg type="a{o(oayays)}" name="secrets" direction="out"/>
  </method>
  <method name="ReadAlias">
    <arg type="s" name="name" direction="in"/>
    <arg type="o" name="collection" direction="out"/>
  </method>
  <method name="SetAlias">
    <arg type="s" name="name" direction="in"/>
    <arg type="o" name="collection" direction="in"/>
  </method>
  <property name="Collections" type="ao" access="read"/>
  <signal name="CollectionCreated"><arg type="o" name="collection"/></signal>
  <signal name="CollectionDeleted"><arg type="o" name="collection"/></signal>
  <signal name="CollectionChanged"><arg type="o" name="collection"/></signal>
</interface>"#;

const COLLECTION_XML: &str = r#"<interface name="org.freedesktop.Secret.Collection">
  <method name="Delete">
    <arg type="o" name="prompt" direction="out"/>
  </method>
  <method name="SearchItems">
    <arg type="a{ss}" name="attributes" direction="in"/>
    <arg type="ao" name="results" direction="out"/>
  </method>
  <method name="CreateItem">
    <arg type="a{sv}" name="properties" direction="in"/>
    <arg type="(oayays)" name="secret" direction="in"/>
    <arg type="b" name="replace" direction="in"/>
    <arg type="o" name="item" direction="out"/>
    <arg type="o" name="prompt" direction="out"/>
  </method>
  <property name="Items" type="ao" access="read"/>
  <property name="Label" type="s" access="readwrite"/>
  <property name="Locked" type="b" access="read"/>
  <property name="Created" type="t" access="read"/>
  <property name="Modified" type="t" access="read"/>
  <signal name="ItemCreated"><arg type="o" name="item"/></signal>
  <signal name="ItemDeleted"><arg type="o" name="item"/></signal>
  <signal name="ItemChanged"><arg type="o" name="item"/></signal>
</interface>"#;

const ITEM_XML: &str = r#"<interface name="org.freedesktop.Secret.Item">
  <method name="Delete">
    <arg type="o" name="Prompt" direction="out"/>
  </method>
  <method name="GetSecret">
    <arg type="o" name="session" direction="in"/>
    <arg type="(oayays)" name="secret" direction="out"/>
  </method>
  <method name="SetSecret">
    <arg type="(oayays)" name="secret" direction="in"/>
  </method>
  <property name="Locked" type="b" access="read"/>
  <property name="Attributes" type="a{ss}" access="readwrite"/>
  <property name="Label" type="s" access="readwrite"/>
  <property name="Type" type="s" access="readwrite"/>
  <property name="Created" type="t" access="read"/>
  <property name="Modified" type="t" access="read"/>
</interface>"#;

const SESSION_XML: &str = r#"<interface name="org.freedesktop.Secret.Session">
  <method name="Close"/>
</interface>"#;

const PROMPT_XML: &str = r#"<interface name="org.freedesktop.Secret.Prompt">
  <method name="Prompt">
    <arg type="s" name="window-id" direction="in"/>
  </method>
  <method name="Dismiss"/>
  <signal name="Completed">
    <arg type="b" name="dismissed"/>
    <arg type="v" name="result"/>
  </signal>
</interface>"#;

const GNOME_XML: &str = r#"<interface name="org.gnome.keyring.InternalUnsupportedGuiltRiddenInterface">
  <method name="ChangeWithMasterPassword">
    <arg type="o" name="collection" direction="in"/>
    <arg type="(oayays)" name="original" direction="in"/>
    <arg type="(oayays)" name="master" direction="in"/>
  </method>
  <method name="UnlockWithMasterPassword">
    <arg type="o" name="collection" direction="in"/>
    <arg type="(oayays)" name="master" direction="in"/>
  </method>
</interface>"#;

const PORTAL_XML: &str = r#"<interface name="org.freedesktop.impl.portal.Secret">
  <method name="RetrieveSecret">
    <arg type="o" name="handle" direction="in"/>
    <arg type="s" name="app_id" direction="in"/>
    <arg type="h" name="fd" direction="in"/>
    <arg type="a{sv}" name="options" direction="in"/>
    <arg type="u" name="response" direction="out"/>
    <arg type="a{sv}" name="results" direction="out"/>
  </method>
  <property name="version" type="u" access="read"/>
</interface>"#;

pub(super) fn xml_for(name: &str) -> Option<&'static str> {
    match name {
        SERVICE => Some(SERVICE_XML),
        COLLECTION => Some(COLLECTION_XML),
        ITEM => Some(ITEM_XML),
        SESSION => Some(SESSION_XML),
        PROMPT => Some(PROMPT_XML),
        GNOME => Some(GNOME_XML),
        PORTAL => Some(PORTAL_XML),
        _ => None,
    }
}
