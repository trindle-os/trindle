//! Files of `[Group]` headings and `key=value` lines, as desktop entries, plugin.ini and GTK's settings.ini are written.

use std::path::Path;

enum Line {
    Group(String),
    Entry { key: String, value: String },
    Other(String),
}

pub const LOAD_MOST: u64 = 4 << 20;

#[derive(Default)]
pub struct KeyFile {
    lines: Vec<Line>,
}

impl KeyFile {
    pub fn parse(text: &str) -> KeyFile {
        let lines = text.lines().map(|line| {
            let t = line.trim();
            if t.starts_with('[') && t.ends_with(']') {
                Line::Group(t[1..t.len() - 1].to_string())
            } else if let (false, Some((key, value))) = (t.starts_with('#'), line.split_once('=')) {
                Line::Entry { key: key.trim().to_string(), value: value.trim_start().to_string() }
            } else {
                Line::Other(line.to_string())
            }
        }).collect();
        KeyFile { lines }
    }

    pub fn load(path: &Path) -> std::io::Result<KeyFile> {
        use std::io::Read;
        let mut text = String::new();
        super::files::open_settings(path)?.take(LOAD_MOST + 1).read_to_string(&mut text)?;
        if text.len() as u64 > LOAD_MOST {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "the file is too big to be read whole"));
        }
        Ok(KeyFile::parse(&text))
    }

    pub fn parts(text: &str) -> Vec<(String, KeyFile)> {
        let mut all: Vec<(String, KeyFile)> = Vec::new();
        for line in KeyFile::parse(text).lines {
            match line {
                Line::Group(group) => all.push((group.clone(), KeyFile { lines: vec![Line::Group(group)] })),
                other => {
                    if let Some((_, part)) = all.last_mut() {
                        part.lines.push(other);
                    }
                }
            }
        }
        all
    }

    pub fn load_kept(path: &Path) -> std::io::Result<KeyFile> {
        super::files::read_small(path).map(|t| KeyFile::parse(&t))
    }

    pub fn load_or_empty(path: &Path) -> KeyFile {
        KeyFile::load(path).unwrap_or_default()
    }

    pub fn has_group(&self, group: &str) -> bool {
        self.lines.iter().any(|l| matches!(l, Line::Group(g) if g == group))
    }

    fn find(&self, group: &str, key: &str) -> Option<usize> {
        let mut inside = false;
        let mut found = None;
        for (i, line) in self.lines.iter().enumerate() {
            match line {
                Line::Group(g) => inside = g == group,
                Line::Entry { key: k, .. } if inside && k == key => found = Some(i),
                _ => {}
            }
        }
        found
    }

    fn raw(&self, group: &str, key: &str) -> Option<&str> {
        match &self.lines[self.find(group, key)?] {
            Line::Entry { value, .. } => Some(value),
            _ => None,
        }
    }

    pub fn string(&self, group: &str, key: &str) -> Option<String> {
        self.raw(group, key).map(|v| unescape(v, None).concat())
    }

    pub fn list(&self, group: &str, key: &str) -> Option<Vec<String>> {
        self.raw(group, key).map(|v| unescape(v, Some(';')))
    }

    pub fn integer(&self, group: &str, key: &str) -> Option<i64> {
        self.raw(group, key)?.trim().parse().ok()
    }

    pub fn set(&mut self, group: &str, key: &str, value: &str) {
        let entry = Line::Entry { key: key.to_string(), value: escape(value) };
        if let Some(at) = self.find(group, key) {
            self.lines[at] = entry;
            return;
        }
        let mut inside = false;
        let mut end = None;
        for (i, line) in self.lines.iter().enumerate() {
            match line {
                Line::Group(g) => {
                    inside = g == group;
                    if inside {
                        end = Some(i + 1);
                    }
                }
                Line::Entry { .. } if inside => end = Some(i + 1),
                _ => {}
            }
        }
        match end {
            Some(at) => self.lines.insert(at, entry),
            None => {
                if self.lines.last().is_some_and(|l| !matches!(l, Line::Other(t) if t.trim().is_empty())) {
                    self.lines.push(Line::Other(String::new()));
                }
                self.lines.push(Line::Group(group.to_string()));
                self.lines.push(entry);
            }
        }
    }

    pub fn text(&self) -> String {
        self.lines.iter().map(|l| match l {
            Line::Group(g) => format!("[{g}]\n"),
            Line::Entry { key, value } => format!("{key}={value}\n"),
            Line::Other(t) => format!("{t}\n"),
        }).collect()
    }
}

fn unescape(value: &str, split: Option<char>) -> Vec<String> {
    let mut items = vec![String::new()];
    let mut chars = value.chars();
    while let Some(c) = chars.next() {
        let cur = items.last_mut().expect("never empty");
        match c {
            '\\' => match chars.next() {
                Some('s') => cur.push(' '),
                Some('n') => cur.push('\n'),
                Some('t') => cur.push('\t'),
                Some('r') => cur.push('\r'),
                Some(other) => cur.push(other),
                None => {}
            },
            c if Some(c) == split => items.push(String::new()),
            c => cur.push(c),
        }
    }
    if split.is_some() {
        items.retain(|i| !i.is_empty());
    }
    items
}

fn escape(value: &str) -> String {
    let mut out = String::new();
    for (i, c) in value.chars().enumerate() {
        match c {
            ' ' if i == 0 => out.push_str("\\s"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            '\\' => out.push_str("\\\\"),
            c => out.push(c),
        }
    }
    out
}

pub fn command_words(line: &str) -> Option<Vec<String>> {
    let mut words = Vec::new();
    let mut word: Option<String> = None;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            c if c.is_whitespace() => {
                if let Some(w) = word.take() {
                    words.push(w);
                }
            }
            '#' if word.is_none() => {
                for c in chars.by_ref() {
                    if c == '\n' {
                        break;
                    }
                }
            }
            '\'' => {
                let w = word.get_or_insert_with(String::new);
                loop {
                    match chars.next()? {
                        '\'' => break,
                        c => w.push(c),
                    }
                }
            }
            '"' => {
                let w = word.get_or_insert_with(String::new);
                loop {
                    match chars.next()? {
                        '"' => break,
                        '\\' => match chars.next()? {
                            '\n' => {}
                            c @ ('$' | '`' | '"' | '\\') => w.push(c),
                            c => {
                                w.push('\\');
                                w.push(c);
                            }
                        },
                        c => w.push(c),
                    }
                }
            }
            '\\' => match chars.next() {
                Some('\n') => {}
                Some(c) => word.get_or_insert_with(String::new).push(c),
                None => word.get_or_insert_with(String::new).push('\\'),
            },
            c => word.get_or_insert_with(String::new).push(c),
        }
    }
    words.extend(word);
    (!words.is_empty()).then_some(words)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_lone_bracket_is_not_a_group_and_does_not_panic() {
        let f = KeyFile::parse("[\n[]\n[A]\n]\n");
        assert_eq!(f.text(), "[\n[]\n[A]\n]\n");
    }

    const PLUGIN: &str = "# A plugin\n[Plugin]\nName = Weather now\nInterface=1\nAsks=network;sound\\;loud;\n\n[Settings]\nCity=\\sLondon\\nUK\n";

    #[test]
    fn reads_values_lists_and_numbers() {
        let kf = KeyFile::parse(PLUGIN);
        assert!(kf.has_group("Plugin") && kf.has_group("Settings") && !kf.has_group("Ring"));
        assert_eq!(kf.string("Plugin", "Name").as_deref(), Some("Weather now"));
        assert_eq!(kf.integer("Plugin", "Interface"), Some(1));
        assert_eq!(kf.integer("Plugin", "Name"), None);
        assert_eq!(kf.list("Plugin", "Asks").unwrap(), ["network", "sound;loud"]);
        assert_eq!(kf.string("Settings", "City").as_deref(), Some(" London\nUK"));
        assert_eq!(kf.string("Settings", "Name"), None);
        assert_eq!(KeyFile::parse("[A]\nx=a;b").list("A", "x").unwrap(), ["a", "b"]);
    }

    #[test]
    fn writes_back_keeping_every_other_line() {
        let mut kf = KeyFile::parse(PLUGIN);
        kf.set("Plugin", "Name", " Rain\\Sun");
        kf.set("Plugin", "Version", "2");
        kf.set("Ring", "Show", "yes");
        let mut bare = KeyFile::parse("[A]\n\n[B]\nx=1\n");
        bare.set("A", "y", "2");
        assert_eq!(bare.text(), "[A]\ny=2\n\n[B]\nx=1\n");
        assert_eq!(kf.string("Plugin", "Name").as_deref(), Some(" Rain\\Sun"));
        let text = kf.text();
        assert!(text.starts_with("# A plugin\n[Plugin]\nName=\\sRain\\\\Sun\nInterface=1\n"));
        assert!(text.contains("sound\\;loud;\nVersion=2\n\n[Settings]"));
        assert!(text.ends_with("City=\\sLondon\\nUK\n\n[Ring]\nShow=yes\n"));
        let mut empty = KeyFile::default();
        empty.set("Settings", "gtk-font-name", "Trindle Sans 11");
        assert_eq!(empty.text(), "[Settings]\ngtk-font-name=Trindle Sans 11\n");
    }

    #[test]
    fn a_file_too_big_to_be_one_of_these_is_not_loaded() {
        let dir = std::env::temp_dir().join(format!("trindle-keyfile-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let (small, big) = (dir.join("small.ini"), dir.join("big.ini"));
        std::fs::write(&small, PLUGIN).unwrap();
        assert_eq!(KeyFile::load(&small).unwrap().string("Plugin", "Name").as_deref(), Some("Weather now"));
        let mut text = PLUGIN.to_string();
        text.push_str(&"# padding\n".repeat(LOAD_MOST as usize / 10 + 1));
        std::fs::write(&big, &text).unwrap();
        assert!(KeyFile::load(&big).is_err());
        assert!(!KeyFile::load_or_empty(&big).has_group("Plugin"));
        assert!(KeyFile::load(&dir).is_err() && KeyFile::load(&dir.join("none.ini")).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_file_is_read_group_by_group() {
        let parts = KeyFile::parts("x=before\n[A]\nx=1\n# note\n[B]\nx=2\ny=3\n[A]\nx=4\n");
        let read: Vec<(&str, Option<String>, Option<String>)> = parts.iter().map(|(g, kf)| (g.as_str(), kf.string(g, "x"), kf.string(g, "y"))).collect();
        assert_eq!(read, [("A", Some("1".into()), None), ("B", Some("2".into()), Some("3".into())), ("A", Some("4".into()), None)]);
        assert!(!parts[0].1.has_group("B") && parts[1].1.string("A", "x").is_none());
        assert!(KeyFile::parts("x=1\n").is_empty());
    }

    #[test]
    fn commands_split_as_a_shell_would() {
        assert_eq!(command_words("run --city 'New York' \"say \\\"hi\\\"\" a\\ b").unwrap(),
                   ["run", "--city", "New York", "say \"hi\"", "a b"]);
        assert_eq!(command_words("bin/x '' # note").unwrap(), ["bin/x", ""]);
        assert_eq!(command_words("x\"y\"z \"a\\nb\"").unwrap(), ["xyz", "a\\nb"]);
        assert!(command_words("  ").is_none());
        assert!(command_words("x 'open").is_none());
    }
}
