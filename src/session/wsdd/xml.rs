//! The little XML the finding reads: a piece with its names stripped of prefixes, and a computer's description.

use std::collections::BTreeMap;

use quick_xml::events::Event;

use super::WSDP;
use super::message::same_type;

pub(super) fn metadata_of(root: &Node) -> Option<(BTreeMap<String, String>, Option<Vec<String>>)> {
    let meta = root.path(&["Body", "Metadata"])?;
    let mut props: BTreeMap<String, String> = BTreeMap::new();
    let mut types = None;
    for section in meta.kids.iter().filter(|k| k.name == "MetadataSection") {
        let dialect = section.attr("Dialect").unwrap_or("");
        match dialect.strip_prefix(WSDP).unwrap_or("") {
            "/ThisDevice" | "/ThisModel" => {
                let inner = &dialect[WSDP.len() + 1..];
                if let Some(p) = section.child(inner) {
                    for k in &p.kids {
                        props.insert(k.name.clone(), k.text.trim().to_string());
                    }
                }
            }
            "/Relationship" => {
                let host_type = format!("{WSDP}/host");
                let rel = section.kids.iter().find(|k| k.name == "Relationship" && k.attr("Type") == Some(host_type.as_str()));
                if let Some(h) = rel.and_then(|r| r.child("Host")) {
                    let list: Vec<String> = h.text_of("Types").split_whitespace().map(String::from).collect();
                    if list.iter().any(|t| same_type(t, "pub:Computer")) {
                        let computer = h.text_of("Computer");
                        let (name, group) = computer.split_once('/').unwrap_or((&computer, ""));
                        props.insert("DisplayName".into(), name.to_string());
                        props.insert("BelongsTo".into(), group.to_string());
                    }
                    types = Some(list);
                }
            }
            _ => {}
        }
    }
    Some((props, types))
}

#[derive(Default)]
pub(super) struct Node {
    pub(super) name: String,
    attrs: Vec<(String, String)>,
    pub(super) text: String,
    pub(super) kids: Vec<Node>,
}

pub(super) const MOST_DEPTH: usize = 32;

impl Node {
    pub(super) fn parse(data: &[u8]) -> Option<Node> {
        let mut reader = quick_xml::Reader::from_reader(data);
        let mut stack: Vec<Node> = vec![Node::default()];
        let open = |e: &quick_xml::events::BytesStart| Node {
            name: String::from_utf8_lossy(e.local_name().as_ref()).into_owned(),
            attrs: e
                .attributes()
                .flatten()
                .map(|a| {
                    let key = String::from_utf8_lossy(a.key.local_name().as_ref()).into_owned();
                    let value = a
                        .normalized_value(quick_xml::XmlVersion::Implicit1_0)
                        .map(|v| v.into_owned())
                        .unwrap_or_default();
                    (key, value)
                })
                .collect(),
            ..Node::default()
        };
        loop {
            match reader.read_event().ok()? {

                Event::Start(_) if stack.len() > MOST_DEPTH => return None,
                Event::Start(e) => stack.push(open(&e)),
                Event::Empty(e) => {
                    let node = open(&e);
                    stack.last_mut()?.kids.push(node);
                }
                Event::End(_) => {
                    let node = stack.pop()?;
                    stack.last_mut()?.kids.push(node);
                }
                Event::Text(t) => stack.last_mut()?.text.push_str(&t.decode().ok()?),
                Event::CData(t) => stack.last_mut()?.text.push_str(&String::from_utf8_lossy(&t)),
                Event::GeneralRef(r) => {
                    let c = match r.resolve_char_ref().ok()? {
                        Some(c) => c,
                        None => match r.decode().ok()?.as_ref() {
                            "amp" => '&',
                            "lt" => '<',
                            "gt" => '>',
                            "quot" => '"',
                            "apos" => '\'',
                            _ => continue,
                        },
                    };
                    stack.last_mut()?.text.push(c);
                }
                Event::Eof => break,
                _ => {}
            }
        }
        let mut top = stack.pop()?;
        (stack.is_empty() && top.kids.len() == 1).then(|| top.kids.remove(0))
    }

    pub(super) fn child(&self, name: &str) -> Option<&Node> {
        self.kids.iter().find(|k| k.name == name)
    }

    pub(super) fn path(&self, names: &[&str]) -> Option<&Node> {
        names.iter().try_fold(self, |n, name| n.child(name))
    }

    pub(super) fn text_of(&self, name: &str) -> String {
        self.child(name).map(|n| n.text.trim().to_string()).unwrap_or_default()
    }

    fn attr(&self, name: &str) -> Option<&str> {
        self.attrs.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
    }
}
