//! The speakers and microphones PipeWire knows about, and which of them is
//! in use, read from pw-dump and changed with wpctl. Settings and the ring's
//! quick sound page both ask here, so there is one way of finding them and
//! one way of choosing one.

use serde_json::Value;

use super::run::capture;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Kind {
    Bluetooth,
    Hdmi,
    Usb,
    Headphones,
    Speakers,
    #[default]
    Other,
}

impl Kind {
    pub fn icon(self) -> &'static str {
        match self {
            Kind::Bluetooth => "bluetooth-active-symbolic",
            Kind::Hdmi => "video-display-symbolic",
            Kind::Headphones => "audio-headphones-symbolic",
            Kind::Usb | Kind::Speakers | Kind::Other => "audio-speakers-symbolic",
        }
    }
}

fn kind_of(props: &Value, name: &str) -> Kind {
    let has = |k: &str, v: &str| props[k].as_str().is_some_and(|s| s.eq_ignore_ascii_case(v));
    let in_name = |w: &str| name.to_lowercase().contains(w) || props["device.profile.name"].as_str().is_some_and(|p| p.to_lowercase().contains(w));
    if has("device.api", "bluez5") || has("device.bus", "bluetooth") || name.starts_with("bluez_") {
        Kind::Bluetooth
    } else if in_name("hdmi") || in_name("displayport") {
        Kind::Hdmi
    } else if has("device.form-factor", "headphone") || has("device.form-factor", "headset") || has("device.form-factor", "hands-free") {
        Kind::Headphones
    } else if has("device.bus", "usb") {
        Kind::Usb
    } else if has("device.form-factor", "speaker") || has("device.form-factor", "internal") || has("device.bus", "pci") {
        Kind::Speakers
    } else {
        Kind::Other
    }
}

pub struct Node {
    pub id: i64,
    pub kind: Kind,
    pub name: String,

    pub label: String,
}

#[derive(Default)]
pub struct Devices {
    pub outputs: Vec<Node>,
    pub inputs: Vec<Node>,

    pub output: Option<String>,
    pub input: Option<String>,

    pub bluetooth_silent: Vec<String>,
}

impl Devices {

    pub fn current_output(&self) -> Option<&Node> {
        current(&self.outputs, self.output.as_ref())
    }

    pub fn speakers(&self) -> Option<&Node> {
        let by = |k: Kind| self.outputs.iter().find(|n| n.kind == k);
        by(Kind::Speakers).or_else(|| by(Kind::Headphones)).or_else(|| by(Kind::Usb))
            .or_else(|| self.outputs.iter().find(|n| n.kind != Kind::Bluetooth && n.kind != Kind::Hdmi))
            .or_else(|| by(Kind::Hdmi))
    }

    pub fn current_input(&self) -> Option<&Node> {
        current(&self.inputs, self.input.as_ref())
    }
}

fn current<'a>(nodes: &'a [Node], name: Option<&String>) -> Option<&'a Node> {
    nodes.iter().find(|n| Some(&n.name) == name).or(nodes.first())
}

pub fn parse_nodes(dump: &[Value]) -> Devices {
    let mut found = Devices::default();

    let mut bluetooth: Vec<(i64, String)> = Vec::new();
    let mut sink_devices: Vec<i64> = Vec::new();
    for obj in dump {
        let kind = obj["type"].as_str().unwrap_or("");
        if kind == "PipeWire:Interface:Metadata" {
            if obj["props"]["metadata.name"].as_str() != Some("default") {
                continue;
            }
            for entry in obj["metadata"].as_array().into_iter().flatten() {
                let name = entry["value"]["name"].as_str().map(str::to_string);
                match entry["key"].as_str() {
                    Some("default.audio.sink") if entry["value"].is_object() => found.output = name,
                    Some("default.audio.source") if entry["value"].is_object() => found.input = name,
                    _ => {}
                }
            }
            continue;
        }
        if kind == "PipeWire:Interface:Device" {
            let p = &obj["info"]["props"];
            if p["device.api"].as_str() == Some("bluez5") {
                let name = ["device.description", "device.alias", "device.nick", "device.name"].iter()
                    .find_map(|k| p[*k].as_str().filter(|s| !s.is_empty())).unwrap_or_default();
                bluetooth.push((obj["id"].as_i64().unwrap_or(0), name.to_string()));
            }
            continue;
        }
        if kind != "PipeWire:Interface:Node" {
            continue;
        }
        let props = &obj["info"]["props"];
        let text = |k: &str| props[k].as_str().filter(|s| !s.is_empty()).map(str::to_string);
        let name = text("node.name").unwrap_or_default();
        let node = Node {
            id: obj["id"].as_i64().unwrap_or(0),
            kind: kind_of(props, &name),
            label: text("node.description").or_else(|| text("node.nick")).unwrap_or(name.clone()),
            name,
        };
        match props["media.class"].as_str() {
            Some("Audio/Sink") => {
                sink_devices.extend(props["device.id"].as_i64());
                found.outputs.push(node)
            }
            Some("Audio/Source") => found.inputs.push(node),
            _ => {}
        }
    }
    found.bluetooth_silent = bluetooth.into_iter().filter(|(id, _)| !sink_devices.contains(id)).map(|b| b.1).collect();
    found
}

pub fn parse_dump(text: &str) -> Option<Vec<Value>> {
    serde_json::from_str::<Value>(text).ok().and_then(|v| v.as_array().cloned())
}

pub fn list(done: impl FnOnce(Result<Devices, String>) + 'static) {
    capture(&["pw-dump"], None, move |ok, out, err| {
        match parse_dump(&out).filter(|_| ok) {
            Some(dump) => done(Ok(parse_nodes(&dump))),
            None => done(Err(if err.trim().is_empty() { "pw-dump gave no answer.".to_string() } else { err.trim().to_string() })),
        }
    });
}

pub fn set_default(id: i64, done: impl FnOnce(Option<String>) + 'static) {
    capture(&["wpctl", "set-default", &id.to_string()], None, move |ok, _, err| {
        let err = err.trim();
        done((!ok).then(|| if err.is_empty() { "Refused".to_string() } else { err.to_string() }));
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const DUMP: &str = r#"[
      { "id": 30, "type": "PipeWire:Interface:Metadata",
        "props": { "metadata.name": "default" },
        "metadata": [
          { "subject": 0, "key": "default.audio.sink", "type": "Spa:String:JSON", "value": { "name": "alsa_output.usb-headset" } },
          { "subject": 0, "key": "default.audio.source", "type": "Spa:String:JSON", "value": { "name": "alsa_input.pci-analog" } },
          { "subject": 0, "key": "default.configured.audio.sink", "value": { "name": "ignored" } }
        ] },
      { "id": 31, "type": "PipeWire:Interface:Metadata",
        "props": { "metadata.name": "settings" },
        "metadata": [ { "key": "default.audio.sink", "value": { "name": "not-this-one" } } ] },
      { "id": 41, "type": "PipeWire:Interface:Node",
        "info": { "props": { "media.class": "Audio/Sink", "node.name": "alsa_output.pci-analog",
                             "node.description": "Built-in Audio Analog Stereo" } } },
      { "id": 42, "type": "PipeWire:Interface:Node",
        "info": { "props": { "media.class": "Audio/Sink", "node.name": "alsa_output.usb-headset",
                             "node.description": "", "node.nick": "USB Headset" } } },
      { "id": 43, "type": "PipeWire:Interface:Node",
        "info": { "props": { "media.class": "Audio/Source", "node.name": "alsa_input.pci-analog",
                             "node.description": "Built-in Audio Analog Stereo" } } },
      { "id": 44, "type": "PipeWire:Interface:Node",
        "info": { "props": { "media.class": "Audio/Source", "node.name": "bare.name" } } },
      { "id": 45, "type": "PipeWire:Interface:Node",
        "info": { "props": { "media.class": "Stream/Output/Audio", "node.name": "firefox" } } },
      { "id": 46, "type": "PipeWire:Interface:Client", "info": { "props": {} } }
    ]"#;

    #[test]
    fn sorts_speakers_from_microphones() {
        let d = parse_nodes(&parse_dump(DUMP).unwrap());
        let ids = |n: &[Node]| n.iter().map(|n| n.id).collect::<Vec<_>>();
        assert_eq!(ids(&d.outputs), vec![41, 42]);
        assert_eq!(ids(&d.inputs), vec![43, 44]);
    }

    #[test]
    fn names_each_as_people_would() {
        let d = parse_nodes(&parse_dump(DUMP).unwrap());
        assert_eq!(d.outputs[0].label, "Built-in Audio Analog Stereo");

        assert_eq!(d.outputs[1].label, "USB Headset");
        assert_eq!(d.inputs[1].label, "bare.name");
    }

    #[test]
    fn knows_which_is_in_use() {
        let d = parse_nodes(&parse_dump(DUMP).unwrap());
        assert_eq!(d.output.as_deref(), Some("alsa_output.usb-headset"));
        assert_eq!(d.input.as_deref(), Some("alsa_input.pci-analog"));
        assert_eq!(d.current_output().map(|n| n.id), Some(42));
        assert_eq!(d.current_input().map(|n| n.id), Some(43));
    }

    #[test]
    fn first_is_in_use_when_none_is_named() {
        let d = parse_nodes(&parse_dump(r#"[
          { "id": 7, "type": "PipeWire:Interface:Node",
            "info": { "props": { "media.class": "Audio/Sink", "node.name": "a" } } },
          { "id": 8, "type": "PipeWire:Interface:Node",
            "info": { "props": { "media.class": "Audio/Sink", "node.name": "b" } } }
        ]"#).unwrap());
        assert_eq!(d.output, None);
        assert_eq!(d.current_output().map(|n| n.id), Some(7));
        assert!(d.current_input().is_none());
    }

    const WITH_BT: &str = r#"[
      { "id": 30, "type": "PipeWire:Interface:Metadata", "props": { "metadata.name": "default" },
        "metadata": [ { "key": "default.audio.sink", "value": { "name": "bluez_output.AA.1" } } ] },
      { "id": 50, "type": "PipeWire:Interface:Device",
        "info": { "props": { "device.api": "bluez5", "device.description": "Sony headphones" } } },
      { "id": 51, "type": "PipeWire:Interface:Device",
        "info": { "props": { "device.api": "bluez5", "device.description": "Car kit" } } },
      { "id": 52, "type": "PipeWire:Interface:Device", "info": { "props": { "device.api": "alsa" } } },
      { "id": 41, "type": "PipeWire:Interface:Node",
        "info": { "props": { "media.class": "Audio/Sink", "node.name": "alsa_output.pci.hdmi-stereo",
                             "node.description": "HDMI", "device.profile.name": "hdmi-stereo", "device.bus": "pci" } } },
      { "id": 42, "type": "PipeWire:Interface:Node",
        "info": { "props": { "media.class": "Audio/Sink", "node.name": "alsa_output.pci.analog-stereo",
                             "node.description": "Built-in", "device.bus": "pci", "device.form-factor": "internal" } } },
      { "id": 43, "type": "PipeWire:Interface:Node",
        "info": { "props": { "media.class": "Audio/Sink", "node.name": "bluez_output.AA.1", "device.id": 50,
                             "node.description": "Sony headphones", "device.api": "bluez5" } } }
    ]"#;

    #[test]
    fn a_bluetooth_output_is_told_and_the_speakers_are_offered() {
        let d = parse_nodes(&parse_dump(WITH_BT).unwrap());
        let kinds = d.outputs.iter().map(|n| n.kind).collect::<Vec<_>>();
        assert_eq!(kinds, vec![Kind::Hdmi, Kind::Speakers, Kind::Bluetooth]);
        assert_eq!(d.current_output().map(|n| n.kind), Some(Kind::Bluetooth));
        assert_eq!(d.speakers().map(|n| n.id), Some(42), "the built-in speakers, not the screen");
    }

    #[test]
    fn a_connected_device_without_a_sink_is_found() {
        let d = parse_nodes(&parse_dump(WITH_BT).unwrap());
        assert_eq!(d.bluetooth_silent, vec!["Car kit".to_string()]);
    }

    #[test]
    fn no_bluetooth_means_nothing_to_say_about_it() {
        let d = parse_nodes(&parse_dump(DUMP).unwrap());
        assert!(d.bluetooth_silent.is_empty());
        assert!(d.outputs.iter().all(|n| n.kind != Kind::Bluetooth));
    }

    #[test]
    fn nonsense_is_not_a_list() {
        assert!(parse_dump("").is_none());
        assert!(parse_dump("{}").is_none());
        assert!(parse_nodes(&parse_dump("[]").unwrap()).outputs.is_empty());
    }
}
