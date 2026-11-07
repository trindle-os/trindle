//! Firmware.

use serde::Deserialize;

use crate::common::run::{self, capture};

#[derive(Clone, Debug, PartialEq)]
pub struct Firmware {
    pub id: String,
    pub name: String,
    pub current: String,
    pub newest: String,
    pub size: u64,
}

#[derive(Deserialize)]
struct Answer {
    #[serde(rename = "Devices", default)]
    devices: Vec<Device>,
}

#[derive(Deserialize)]
struct Device {
    #[serde(rename = "DeviceId", default)]
    id: String,
    #[serde(rename = "Name", default)]
    name: String,
    #[serde(rename = "Vendor", default)]
    vendor: String,
    #[serde(rename = "Version", default)]
    version: String,
    #[serde(rename = "Releases", default)]
    releases: Vec<Release>,
}

#[derive(Deserialize)]
struct Release {
    #[serde(rename = "Version", default)]
    version: String,
    #[serde(rename = "Size", default)]
    size: u64,
}

pub fn parse_updates(json: &str) -> Vec<Firmware> {
    let Some(start) = json.find('{') else { return Vec::new() };
    let Ok(answer) = serde_json::from_str::<Answer>(&json[start..]) else { return Vec::new() };
    answer.devices.into_iter().filter(|d| crate::admin::valid_device_id(&d.id)).filter_map(|d| {
        let first = d.releases.first()?;
        let (newest, size) = (first.version.clone(), first.size);
        let name = if d.vendor.is_empty() || d.name.to_lowercase().starts_with(&d.vendor.to_lowercase()) {
            d.name
        } else {
            format!("{} {}", d.vendor, d.name)
        };
        Some(Firmware { id: d.id, name, current: d.version, newest, size })
    }).collect()
}

#[derive(Deserialize)]
struct Remotes {
    #[serde(rename = "Remotes", default)]
    remotes: Vec<Remote>,
}

#[derive(Deserialize)]
struct Remote {
    #[serde(rename = "Kind", default)]
    kind: String,
    #[serde(rename = "Enabled", default)]
    enabled: bool,
    #[serde(rename = "Mtime", default)]
    fetched: i64,
    #[serde(rename = "RefreshInterval", default)]
    every: i64,
}

pub fn stale(json: &str, now: i64) -> bool {
    const GRACE: i64 = 24 * 60 * 60;
    let Some(start) = json.find('{') else { return false };
    let Ok(r) = serde_json::from_str::<Remotes>(&json[start..]) else { return false };
    r.remotes.iter().filter(|r| r.enabled && r.kind == "download")
        .any(|r| r.fetched < 0 || r.fetched + r.every.max(0) + GRACE < now)
}

pub fn waiting(done: impl FnOnce(Vec<Firmware>) + 'static) {
    if !run::have("fwupdmgr") {
        crate::common::run::soon(move || done(Vec::new()));
        return;
    }
    capture(&["fwupdmgr", "get-updates", "--json", "--no-authenticate"], None,
            move |_, out, _| done(parse_updates(&out)));
}

pub fn refresh_if_stale(then: impl FnOnce() + 'static) {
    if !run::have("fwupdmgr") {
        return;
    }
    capture(&["fwupdmgr", "get-remotes", "--json"], None, move |ok, out, _| {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64).unwrap_or(0);
        if !ok || !stale(&out, now) {
            return;
        }
        capture(&["fwupdmgr", "refresh", "--no-authenticate"], None, move |ok, _, _| {
            if ok {
                then();
            }
        });
    });
}

#[cfg(test)]
mod tests {
    use super::{parse_updates, stale, Firmware};

    const ONE: &str = r#"{
  "Devices" : [
    {
      "Name" : "System Firmware",
      "DeviceId" : "1e4fa9cea0b89e613370cf9657ffa0b6d3f37fec",
      "Guid" : [ "b5f7dcc1-568c-50f8-a4dd-e39d1f93fda1" ],
      "Flags" : [ "internal", "updatable", "require-ac", "needs-reboot" ],
      "Vendor" : "Framework",
      "Version" : "0.0.3.18",
      "Releases" : [
        { "Version" : "0.0.3.20", "Name" : "Laptop 13 AMD Ryzen 7040", "Size" : 36130020 },
        { "Version" : "0.0.3.19" }
      ]
    }
  ]
}"#;

    #[test]
    fn reads_a_device_and_its_newest_release() {
        assert_eq!(parse_updates(ONE), vec![Firmware {
            id: "1e4fa9cea0b89e613370cf9657ffa0b6d3f37fec".into(),
            name: "Framework System Firmware".into(),
            current: "0.0.3.18".into(),
            newest: "0.0.3.20".into(),
            size: 36130020,
        }]);
    }

    #[test]
    fn skips_what_comes_before_the_answer() {
        assert_eq!(parse_updates(&format!("Idle…: 0%\n{ONE}")).len(), 1);
    }

    #[test]
    fn a_name_that_already_says_whose_is_left_alone() {
        let json = r#"{"Devices":[{"Name":"Logitech Unifying Receiver","Vendor":"Logitech",
            "DeviceId":"0123456789abcdef0123456789abcdef01234567","Version":"1","Releases":[{"Version":"2"}]}]}"#;
        assert_eq!(parse_updates(json)[0].name, "Logitech Unifying Receiver");
    }

    #[test]
    fn nothing_waiting_is_an_empty_list() {
        assert!(parse_updates(r#"{ "Devices" : [] }"#).is_empty());
        assert!(parse_updates("").is_empty());
        assert!(parse_updates("No updatable devices").is_empty());
        assert!(parse_updates("{ not json").is_empty());
    }

    #[test]
    fn a_device_with_no_release_or_a_strange_id_is_left_out() {
        let json = r#"{"Devices":[
            {"Name":"A","DeviceId":"0123456789abcdef0123456789abcdef01234567","Releases":[]},
            {"Name":"B","DeviceId":"--force","Releases":[{"Version":"2"}]},
            {"Name":"C","Releases":[{"Version":"2"}]}]}"#;
        assert!(parse_updates(json).is_empty());
    }

    #[test]
    fn stale_only_when_well_past_its_time() {
        let day = 86_400;
        let remotes = |fetched: i64, enabled: bool, kind: &str| format!(
            r#"{{"Remotes":[{{"Id":"lvfs","Kind":"{kind}","Enabled":{enabled},"Mtime":{fetched},"RefreshInterval":{day}}}]}}"#);
        let now = 1_790_248_433;
        assert!(!stale(&remotes(now - day, true, "download"), now));
        assert!(stale(&remotes(now - 3 * day, true, "download"), now));
        assert!(stale(&remotes(-1, true, "download"), now));
        assert!(!stale(&remotes(-1, false, "download"), now));
        assert!(!stale(&remotes(-1, true, "directory"), now));
        assert!(!stale("", now));
    }
}
