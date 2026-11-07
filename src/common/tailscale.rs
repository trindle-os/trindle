//! Tailscale.

use std::path::Path;

use serde_json::Value;

pub const SERVICE: &str = "tailscaled.service";
const PROGRAM: &str = "/usr/bin/tailscale";
const ENABLED: &str = "/etc/systemd/system/multi-user.target.wants/tailscaled.service";

pub fn installed() -> bool {
    Path::new(PROGRAM).exists()
}

pub fn up(more: &[&str]) -> Vec<String> {
    let mut argv = vec!["tailscale".to_string(), "up".to_string()];
    let user = crate::common::files::user();
    if !user.is_empty() {
        argv.push(format!("--operator={user}"));
    }
    argv.push(format!("--accept-dns={}", !crate::common::dns::on()));
    argv.extend(more.iter().map(|m| m.to_string()));
    argv
}

pub fn on() -> bool {
    installed() && Path::new(ENABLED).exists()
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Device {
    pub name: String,
    pub address: String,
    pub system: String,
    pub online: bool,
    pub seen: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Status {
    pub state: String,
    pub sign_in: String,
    pub network: String,
    pub me: Device,
    pub devices: Vec<Device>,
}

impl Status {
    pub fn connected(&self) -> bool {
        self.state == "Running"
    }
    pub fn needs_sign_in(&self) -> bool {
        self.state == "NeedsLogin" || self.state == "NoState"
    }
}

fn device(v: &Value) -> Device {
    let text = |k: &str| v.get(k).and_then(Value::as_str).unwrap_or("").to_string();
    let address = v.get("TailscaleIPs").and_then(Value::as_array)
        .and_then(|ips| ips.iter().filter_map(Value::as_str).find(|ip| !ip.contains(':'))
            .or_else(|| ips.first().and_then(Value::as_str)))
        .unwrap_or("").to_string();
    let seen = text("LastSeen");
    Device {
        name: text("HostName"),
        address,
        system: text("OS"),
        online: v.get("Online").and_then(Value::as_bool).unwrap_or(false),
        seen: if seen.starts_with("0001") { String::new() } else { seen.chars().take(10).collect() },
    }
}

pub fn parse(json: &str) -> Option<Status> {
    let v: Value = serde_json::from_str(json).ok()?;
    let mut devices: Vec<Device> = v.get("Peer").and_then(Value::as_object)
        .map(|peers| peers.values().map(device).collect()).unwrap_or_default();
    devices.sort_by(|a, b| b.online.cmp(&a.online).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())));
    Some(Status {
        state: v.get("BackendState").and_then(Value::as_str).unwrap_or("").to_string(),
        sign_in: v.get("AuthURL").and_then(Value::as_str).unwrap_or("").to_string(),
        network: v.pointer("/CurrentTailnet/Name").and_then(Value::as_str).unwrap_or("").to_string(),
        me: v.get("Self").map(device).unwrap_or_default(),
        devices,
    })
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn reads_what_tailscale_says() {
        let s = parse(r#"{"BackendState":"Running","AuthURL":"","CurrentTailnet":{"Name":"john@example.com"},
            "Self":{"HostName":"laptop","TailscaleIPs":["100.64.0.1","fd7a:115c::1"],"OS":"linux","Online":true},
            "Peer":{"a":{"HostName":"phone","TailscaleIPs":["100.64.0.2"],"OS":"android","Online":false,
                         "LastSeen":"2026-10-01T09:00:00Z"},
                    "b":{"HostName":"Desktop","TailscaleIPs":["100.64.0.3"],"OS":"linux","Online":true,
                         "LastSeen":"0001-01-01T00:00:00Z"}}}"#).unwrap();
        assert!(s.connected());
        assert_eq!(s.network, "john@example.com");
        assert_eq!(s.me.address, "100.64.0.1");
        assert_eq!(s.devices.iter().map(|d| d.name.as_str()).collect::<Vec<_>>(), ["Desktop", "phone"]);
        assert_eq!(s.devices[0].seen, "");
        assert_eq!(s.devices[1].seen, "2026-10-01");
    }

    #[test]
    fn waiting_for_a_sign_in() {
        let s = parse(r#"{"BackendState":"NeedsLogin","AuthURL":"https://login.tailscale.com/a/abc","Self":{},"Peer":null}"#)
            .unwrap();
        assert!(s.needs_sign_in());
        assert_eq!(s.sign_in, "https://login.tailscale.com/a/abc");
        assert!(s.devices.is_empty());
        assert!(parse("not json").is_none());
    }
}
