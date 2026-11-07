//! This computer as it is announced while one of its folders is shared.

use std::net::Ipv4Addr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use super::{ANONYMOUS, DISCOVERY_TO, GET, HTTP_PORT, OUR_TYPES, WSD, WSDP};
use super::message::{envelope, escape, new_urn, same_type, urn_of};
use super::xml::Node;

pub(super) struct Host {
    pub(super) urn: String,
    name: String,
    workgroup: String,
    pub(super) on: AtomicBool,
    instance: u64,
    number: AtomicU64,
}

impl Host {
    pub(super) fn new(urn: String, name: String, workgroup: String) -> Host {
        let instance = SystemTime::now().duration_since(UNIX_EPOCH).map_or(1, |d| d.as_secs());
        Host { urn, name, workgroup, on: AtomicBool::new(false), instance, number: AtomicU64::new(0) }
    }

    fn xaddr(&self, me: Ipv4Addr) -> String {
        format!("http://{me}:{HTTP_PORT}/{}", self.urn.trim_start_matches("urn:uuid:"))
    }

    fn sequence(&self) -> String {
        let n = self.number.fetch_add(1, Ordering::Relaxed) + 1;
        format!("<wsd:AppSequence InstanceId=\"{}\" MessageNumber=\"{n}\"/>", self.instance)
    }

    fn about(&self, me: Option<Ipv4Addr>) -> String {
        let addrs = me.map(|m| format!("<wsd:XAddrs>{}</wsd:XAddrs>", self.xaddr(m))).unwrap_or_default();
        format!("<wsa:EndpointReference><wsa:Address>{}</wsa:Address></wsa:EndpointReference>\
                 <wsd:Types>{OUR_TYPES}</wsd:Types>{addrs}<wsd:MetadataVersion>1</wsd:MetadataVersion>", self.urn)
    }

    pub(super) fn hello(&self, me: Ipv4Addr) -> String {
        let body = format!("<wsd:Hello>{}</wsd:Hello>", self.about(Some(me)));
        envelope(DISCOVERY_TO, &format!("{WSD}/Hello"), &new_urn(), &self.sequence(), &body)
    }

    pub(super) fn bye(&self) -> String {
        let body = format!("<wsd:Bye><wsa:EndpointReference><wsa:Address>{}</wsa:Address></wsa:EndpointReference></wsd:Bye>", self.urn);
        envelope(DISCOVERY_TO, &format!("{WSD}/Bye"), &new_urn(), &self.sequence(), &body)
    }

    pub(super) fn matches(&self, me: Ipv4Addr, relates: &str, probe: bool) -> String {
        let (outer, inner) = if probe { ("ProbeMatches", "ProbeMatch") } else { ("ResolveMatches", "ResolveMatch") };
        let body = format!("<wsd:{outer}><wsd:{inner}>{}</wsd:{inner}></wsd:{outer}>", self.about(Some(me)));
        let extra = format!("<wsa:RelatesTo>{}</wsa:RelatesTo>{}", escape(relates), self.sequence());
        envelope(ANONYMOUS, &format!("{WSD}/{outer}"), &new_urn(), &extra, &body)
    }

    fn metadata(&self, relates: &str) -> String {
        let device = format!(
            "<wsx:MetadataSection Dialect=\"{WSDP}/ThisDevice\"><wsdp:ThisDevice>\
             <wsdp:FriendlyName>{name}</wsdp:FriendlyName><wsdp:FirmwareVersion>1.0</wsdp:FirmwareVersion>\
             <wsdp:SerialNumber>1</wsdp:SerialNumber></wsdp:ThisDevice></wsx:MetadataSection>\
             <wsx:MetadataSection Dialect=\"{WSDP}/ThisModel\"><wsdp:ThisModel>\
             <wsdp:Manufacturer>Trindle</wsdp:Manufacturer><wsdp:ModelName>Trindle</wsdp:ModelName>\
             <pnpx:DeviceCategory>Computers</pnpx:DeviceCategory></wsdp:ThisModel></wsx:MetadataSection>\
             <wsx:MetadataSection Dialect=\"{WSDP}/Relationship\"><wsdp:Relationship Type=\"{WSDP}/host\">\
             <wsdp:Host><wsa:EndpointReference><wsa:Address>{urn}</wsa:Address></wsa:EndpointReference>\
             <wsdp:Types>pub:Computer</wsdp:Types><wsdp:ServiceId>{urn}</wsdp:ServiceId>\
             <pub:Computer>{name}/Workgroup:{group}</pub:Computer></wsdp:Host></wsdp:Relationship></wsx:MetadataSection>",
            name = escape(&self.name), urn = self.urn, group = escape(&self.workgroup));
        let extra = format!("<wsa:RelatesTo>{}</wsa:RelatesTo>", escape(relates));
        envelope(ANONYMOUS, &format!("{GET}Response"), &new_urn(), &extra, &format!("<wsx:Metadata>{device}</wsx:Metadata>"))
    }

    pub(super) fn reply_to(&self, path: &str, body: &[u8]) -> Option<String> {
        if path.trim_start_matches('/') != self.urn.trim_start_matches("urn:uuid:") {
            return None;
        }
        let root = Node::parse(body)?;
        let header = root.child("Header")?;
        (header.text_of("Action") == GET && header.text_of("To") == self.urn).then(|| self.metadata(&header.text_of("MessageID")))
    }
}

pub(super) fn wants_us(body: &Node) -> bool {
    let Some(probe) = body.child("Probe") else { return false };
    let types = probe.text_of("Types");
    let ours = types.split_whitespace().all(|t| same_type(t, "wsdp:Device") || same_type(t, "pub:Computer"));
    ours && probe.text_of("Scopes").is_empty()
}

pub(super) fn lasting_urn() -> String {
    use sha2::{Digest, Sha256};
    let id = std::fs::read_to_string("/etc/machine-id").unwrap_or_default();
    if id.trim().is_empty() {
        return new_urn();
    }
    let sum = Sha256::digest(format!("trindle-wsdd {}", id.trim()).as_bytes());
    let mut b = [0u8; 16];
    b.copy_from_slice(&sum[..16]);
    urn_of(b)
}
