//! The Flathub catalog, read from Flatpak's local appstream file.

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::common::files::home;
use crate::common::run::{capture, spawn};
use crate::toolkit::t;

pub const REMOTE: &str = "flathub";
const FLATHUB_URL: &str = "https://dl.flathub.org/repo/flathub.flatpakrepo";
const STALE: u64 = 24 * 3600;

pub fn categories() -> [(&'static str, &'static str, &'static str); 10] {
    [
        ("AudioVideo", t!("store-cat-audio-video"), "applications-multimedia-symbolic"),
        ("Graphics", t!("store-cat-graphics"), "applications-graphics-symbolic"),
        ("Office", t!("store-cat-office"), "x-office-document-symbolic"),
        ("Network", t!("store-cat-network"), "network-workgroup-symbolic"),
        ("Development", t!("store-cat-development"), "applications-engineering-symbolic"),
        ("Education", t!("store-cat-education"), "accessories-dictionary-symbolic"),
        ("Game", t!("store-cat-game"), "applications-games-symbolic"),
        ("Science", t!("store-cat-science"), "applications-science-symbolic"),
        ("Utility", t!("store-cat-utility"), "applications-utilities-symbolic"),
        ("System", t!("store-cat-system"), "applications-system-symbolic"),
    ]
}

pub const POPULAR: &[&str] = &[
    "org.mozilla.firefox", "org.mozilla.thunderbird", "org.libreoffice.LibreOffice", "org.videolan.VLC",
    "org.gimp.GIMP", "org.inkscape.Inkscape", "com.obsproject.Studio", "org.audacityteam.Audacity",
    "org.blender.Blender", "org.kde.krita", "org.signal.Signal", "com.spotify.Client",
    "com.visualstudio.code", "md.obsidian.Obsidian", "org.chromium.Chromium", "com.github.tchx84.Flatseal",
    "org.darktable.Darktable", "org.shotcut.Shotcut", "org.kde.kdenlive", "com.bitwarden.desktop",
];

#[derive(Default, Clone, Debug)]
pub struct App {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub description: String,
    pub developer: String,
    pub licence: String,
    pub categories: Vec<String>,
    pub icon: Option<PathBuf>,
    pub homepage: String,
    pub keywords: String,

    pub screenshots: Vec<(String, String)>,

    pub released: Option<u64>,

    pub repo: String,
}

impl App {
    pub fn matches(&self, words: &[String]) -> bool {
        let hay = format!("{} {} {} {}", self.name, self.summary, self.id, self.keywords).to_lowercase();
        words.iter().all(|w| hay.contains(w.as_str()))
    }
}

pub type Catalogue = HashMap<String, App>;

fn places() -> Vec<PathBuf> {
    let arch = std::env::consts::ARCH;
    vec![
        home().join(format!(".local/share/flatpak/appstream/{REMOTE}/{arch}/active")),
        PathBuf::from(format!("/var/lib/flatpak/appstream/{REMOTE}/{arch}/active")),
    ]
}

fn find_place() -> Option<PathBuf> {
    places().into_iter().find(|p| p.join("appstream.xml.gz").exists() || p.join("appstream.xml").exists())
}

fn age(place: &Path) -> u64 {
    std::fs::metadata(place.join("appstream.xml.gz")).and_then(|m| m.modified()).ok()
        .and_then(|t| t.elapsed().ok()).map(|d| d.as_secs()).unwrap_or(STALE + 1)
}

fn squash(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn attr(e: &BytesStart, name: &[u8]) -> Option<String> {
    e.attributes().flatten().find(|a| a.key.as_ref() == name)
        .map(|a| a.normalized_value(quick_xml::XmlVersion::Implicit1_0).map(|v| v.to_string()).unwrap_or_default())
}

#[derive(PartialEq, Clone, Copy)]
enum Into {
    Nothing,
    Id,
    Name,
    Summary,
    Paragraph,
    Item,
    Developer,
    DeveloperName,
    Licence,
    Category,
    Keyword,
    Homepage,
    Source,
    Icon,
    Shot,
    Caption,
}

fn released(e: &BytesStart) -> Option<u64> {
    if matches!(attr(e, b"type").as_deref(), Some("development" | "snapshot")) {
        return None;
    }
    attr(e, b"timestamp").and_then(|t| t.trim().parse().ok())
        .or_else(|| attr(e, b"date").and_then(|d| crate::common::home_snapshots::unix_of(&format!("{}T00:00:00", d.get(..10)?)))
            .and_then(|n| u64::try_from(n).ok()))
}

fn best_size(sizes: &[(u32, String)]) -> Option<String> {
    sizes.iter().min_by_key(|(w, _)| (*w as i64 - 752).abs()).map(|(_, u)| u.clone())
}

pub fn parse(place: &Path) -> Result<Catalogue, String> {
    let gz = place.join("appstream.xml.gz");
    let reader: Box<dyn BufRead> = if gz.exists() {
        Box::new(BufReader::new(flate2::read::GzDecoder::new(std::fs::File::open(&gz).map_err(|e| e.to_string())?)))
    } else {
        Box::new(BufReader::new(std::fs::File::open(place.join("appstream.xml")).map_err(|e| e.to_string())?))
    };
    let mut xml = Reader::from_reader(reader);
    let mut buf = Vec::new();
    let mut parser = Parser::new(place);
    loop {
        let ev = xml.read_event_into(&mut buf).map_err(|e| e.to_string())?;
        match ev {
            Event::Eof => break,
            Event::Start(ref e) => parser.start(e, false),
            Event::Empty(ref e) => parser.start(e, true),
            Event::Text(ref t) => parser.text(t),

            Event::GeneralRef(ref r) => parser.reference(r),
            Event::CData(ref t) => parser.cdata(t),
            Event::End(ref e) => parser.end(e.local_name().as_ref()),
            _ => {}
        }
        buf.clear();
    }
    Ok(parser.apps)
}

#[derive(Default)]
struct Seen {
    name: bool,
    summary: bool,
    description: bool,
    developer: bool,
}

#[derive(Default)]
struct Shot {
    sizes: Vec<(u32, String)>,
    caption: String,
    width: u32,
}

struct Parser<'a> {
    place: &'a Path,
    apps: Catalogue,
    app: Option<App>,

    path: Vec<Vec<u8>>,

    langs: Vec<bool>,
    into: Into,
    text: String,
    paragraphs: Vec<String>,
    in_description: bool,
    developer_name: String,
    icons: Vec<String>,
    seen: Seen,
    shot: Shot,
}

impl<'a> Parser<'a> {
    fn new(place: &'a Path) -> Parser<'a> {
        Parser {
            place, apps: Catalogue::new(), app: None, path: Vec::new(), langs: Vec::new(), into: Into::Nothing, text: String::new(),
            paragraphs: Vec::new(), in_description: false, developer_name: String::new(), icons: Vec::new(), seen: Seen::default(),
            shot: Shot::default(),
        }
    }

    fn wants_text(&self) -> bool {
        self.app.is_some() && self.into != Into::Nothing
    }

    fn text(&mut self, t: &quick_xml::events::BytesText) {
        if self.wants_text() {
            self.text.push_str(&t.decode().map(|s| s.to_string()).unwrap_or_default());
        }
    }

    fn reference(&mut self, r: &quick_xml::events::BytesRef) {
        if self.wants_text() {
            match r.resolve_char_ref() {
                Ok(Some(c)) => self.text.push(c),
                _ => self.text.push_str(match r.decode().as_deref().unwrap_or("") {
                    "amp" => "&",
                    "lt" => "<",
                    "gt" => ">",
                    "quot" => "\"",
                    "apos" => "'",
                    _ => "",
                }),
            }
        }
    }

    fn cdata(&mut self, t: &quick_xml::events::BytesCData) {
        if self.wants_text() {
            self.text.push_str(&String::from_utf8_lossy(t.as_ref()));
        }
    }

    fn start(&mut self, e: &BytesStart, empty: bool) {
        let name = e.local_name().as_ref().to_vec();
        if self.app.is_none() {
            if name == b"component" && !empty {
                self.begin_component(e);
            }
            return;
        }

        if name == b"release" && self.path.last().map(Vec::as_slice) == Some(b"releases") {
            if let Some(a) = self.app.as_mut() {
                a.released = a.released.max(released(e));
            }
        }
        if empty {
            return;
        }
        let lang = e.attributes().flatten().any(|a| a.key.as_ref() == b"xml:lang");
        let translated = lang || self.langs.iter().any(|l| *l);
        self.into = if translated { Into::Nothing } else { self.next_into(e, &name) };
        if !self.in_description || matches!(name.as_slice(), b"p" | b"li") {
            self.text.clear();
        }
        self.path.push(name);
        self.langs.push(lang);
    }

    fn begin_component(&mut self, e: &BytesStart) {
        let kind = attr(e, b"type").unwrap_or_default();
        if ["desktop-application", "desktop", "console-application"].contains(&kind.as_str()) {
            self.app = Some(App::default());
            self.path.clear();
            self.langs.clear();
            self.paragraphs.clear();
            self.developer_name.clear();
            self.icons.clear();
            self.shot.sizes.clear();
            self.shot.caption.clear();
            self.seen = Seen::default();
        }
    }

    fn next_into(&mut self, e: &BytesStart, name: &[u8]) -> Into {
        let parent = self.path.last().map(|p| p.as_slice());
        if self.in_description {
            return match name {
                b"p" => Into::Paragraph,
                b"li" => Into::Item,
                _ => self.into,
            };
        }
        if self.path.is_empty() {
            return match name {
                b"id" => Into::Id,
                b"name" if !self.seen.name => Into::Name,
                b"summary" if !self.seen.summary => Into::Summary,
                b"description" if !self.seen.description => {
                    self.in_description = true;
                    Into::Nothing
                }
                b"developer_name" => Into::DeveloperName,
                b"project_license" => Into::Licence,
                b"url" if attr(e, b"type").as_deref() == Some("homepage") => Into::Homepage,
                b"url" if attr(e, b"type").as_deref() == Some("vcs-browser") => Into::Source,
                b"icon" if attr(e, b"type").as_deref() == Some("cached") && attr(e, b"scale").is_none() => Into::Icon,
                _ => Into::Nothing,
            };
        }
        match (parent, name) {
            (Some(b"developer"), b"name") if !self.seen.developer => Into::Developer,
            (Some(b"categories"), b"category") => Into::Category,
            (Some(b"keywords"), b"keyword") => Into::Keyword,
            (Some(b"screenshot"), b"image") if attr(e, b"type").as_deref() == Some("thumbnail") => {
                self.shot.width = attr(e, b"width").and_then(|w| w.parse().ok()).unwrap_or(0);
                Into::Shot
            }
            (Some(b"screenshot"), b"caption") => Into::Caption,
            _ => Into::Nothing,
        }
    }

    fn end(&mut self, name: &[u8]) {
        if self.app.is_none() {
            return;
        }
        if self.path.is_empty() && name == b"component" {
            self.finish_component();
            return;
        }
        let depth = self.path.len();
        let translated = self.langs.iter().any(|l| *l);
        self.path.pop();
        self.langs.pop();
        let done = squash(&self.text);
        if !translated {
            self.keep(name, depth, done);
        }
        if name == b"screenshot" {
            self.close_screenshot();
        }
        let closes_text = match self.into {
            Into::Paragraph => name == b"p",
            Into::Item => name == b"li",
            _ => true,
        };
        if closes_text {
            self.into = Into::Nothing;
            self.text.clear();
        }
        if self.in_description && depth == 1 && name == b"description" {
            self.in_description = false;
            if !translated {
                self.seen.description = true;
            }
        }
    }

    fn keep(&mut self, name: &[u8], depth: usize, done: String) {
        let Some(a) = self.app.as_mut() else { return };
        match self.into {
            Into::Id if depth == 1 => a.id = done,
            Into::Name => { a.name = done; self.seen.name = true; }
            Into::Summary => { a.summary = done; self.seen.summary = true; }
            Into::Paragraph if name == b"p" => { if !done.is_empty() { self.paragraphs.push(done); } }
            Into::Item if name == b"li" => { if !done.is_empty() { self.paragraphs.push(format!("  \u{2022} {done}")); } }
            Into::Developer => { a.developer = done; self.seen.developer = true; }
            Into::DeveloperName if self.developer_name.is_empty() => self.developer_name = done,
            Into::Licence => a.licence = done,
            Into::Category if !done.is_empty() => a.categories.push(done),
            Into::Keyword => { if !a.keywords.is_empty() { a.keywords.push(' '); } a.keywords.push_str(&done); }
            Into::Homepage if a.homepage.is_empty() => a.homepage = done,
            Into::Source if a.repo.is_empty() => a.repo = super::github::repo_of_url(&done).unwrap_or_default(),
            Into::Icon => self.icons.push(done),
            Into::Shot if done.starts_with("https://") => self.shot.sizes.push((self.shot.width, done)),
            Into::Caption => self.shot.caption = done,
            _ => {}
        }
    }

    fn close_screenshot(&mut self) {
        if let (Some(url), Some(a)) = (best_size(&self.shot.sizes), self.app.as_mut()) {
            if a.screenshots.len() < 8 {
                a.screenshots.push((url, std::mem::take(&mut self.shot.caption)));
            }
        }
        self.shot.sizes.clear();
        self.shot.caption.clear();
    }

    fn finish_component(&mut self) {
        let mut a = self.app.take().unwrap_or_default();

        if let Some(bare) = a.id.strip_suffix(".desktop").filter(|b| b.split('.').count() >= 3) {
            a.id = bare.to_string();
        }
        if a.name.is_empty() {
            a.name = a.id.clone();
        }
        if a.developer.is_empty() {
            a.developer = self.developer_name.clone();
        }
        a.description = self.paragraphs.join("\n\n");
        if a.repo.is_empty() {
            a.repo = super::github::repo_of_url(&a.homepage).unwrap_or_default();
        }
        for size in ["128x128", "64x64"] {

            let plain = |i: &&String| !i.is_empty() && !i.contains('/') && *i != ".." && *i != ".";
            if let Some(f) = self.icons.iter().filter(plain).map(|i| self.place.join("icons").join(size).join(i)).find(|f| f.is_file()) {
                a.icon = Some(f);
                break;
            }
        }
        if !a.id.is_empty() {
            self.apps.insert(a.id.clone(), a);
        }
    }
}

pub fn flathub_repo() -> String {
    let local = crate::common::files::share("flatpak").join("flathub.flatpakrepo");
    if local.is_file() { local.to_string_lossy().into_owned() } else { FLATHUB_URL.to_string() }
}

pub fn add_flathub(name: &str) -> Vec<String> {
    ["flatpak", "remote-add", "--user", "--if-not-exists", name].iter().map(|s| s.to_string())
        .chain([flathub_repo()]).collect()
}

pub fn load(done: impl FnOnce(Option<Catalogue>, String) + 'static) {
    fn read(place: PathBuf, done: impl FnOnce(Option<Catalogue>, String) + 'static) {
        crate::common::run::on_thread(move || parse(&place), move |result| match result {
            Ok(apps) => done(Some(apps), String::new()),
            Err(e) => done(None, t!("store-catalogue-unreadable", why = e.to_string())),
        });
    }
    if let Some(place) = find_place() {
        if !crate::common::update_checks::may_fetch_catalogue(true, age(&place) < STALE, crate::common::update_checks::on()) {
            read(place, done);
            return;
        }
    }

    let add = add_flathub(REMOTE);
    let add: Vec<&str> = add.iter().map(String::as_str).collect();
    capture(&add, None, move |_ok, _out, _err| {
        capture(&["flatpak", "update", "--user", "--appstream", REMOTE], None, move |_ok, _out, err| {
            match find_place() {
                Some(p) => read(p, done),
                None => done(None, t!("store-catalogue-not-fetched", why = err.trim())),
            }
        });
    });
}

pub fn installed(done: impl FnOnce(HashMap<String, (String, String)>) + 'static) {
    capture(&["flatpak", "list", "--app", "--columns=application,installation,version"], None, move |_ok, out, _err| {
        let mut result = HashMap::new();
        for line in out.lines() {
            let parts: Vec<&str> = line.split('\t').collect();
            if let Some(id) = parts.first().filter(|s| !s.is_empty()) {
                result.insert(id.to_string(), (parts.get(1).unwrap_or(&"user").to_string(),
                                               parts.get(2).unwrap_or(&"").to_string()));
            }
        }
        done(result);
    });
}

pub fn open_app(id: &str) {
    let argv = crate::common::apps::with_own_x11(vec!["flatpak".into(), "run".into(), id.into()]);
    spawn(&argv.iter().map(String::as_str).collect::<Vec<_>>());
}

#[cfg(test)]
mod tests {
    #[test]
    fn reads_when_an_app_was_last_released_and_its_project() {
        let dir = std::env::temp_dir().join(format!("trindle-catalogue-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("appstream.xml"), r#"<components>
            <component type="desktop-application"><id>org.example.One</id><name>One</name>
              <url type="homepage">https://example.org</url><url type="vcs-browser">https://github.com/john-smith/one</url>
              <releases><release version="2" timestamp="200"><description><p>Two</p></description></release>
                <release version="3" timestamp="900" type="development"/><release version="1" timestamp="100"/></releases></component>
            <component type="desktop-application"><id>org.example.Two</id><name>Two</name>
              <url type="homepage">https://github.com/john-smith/two/</url>
              <releases><release version="1" date="1970-01-02"/></releases></component>
            <component type="desktop-application"><id>org.example.Three</id><name>Three</name></component>
            </components>"#).unwrap();
        let apps = super::parse(&dir).unwrap();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!((apps["org.example.One"].released, apps["org.example.One"].repo.as_str()), (Some(200), "john-smith/one"));
        assert_eq!((apps["org.example.Two"].released, apps["org.example.Two"].repo.as_str()), (Some(86_400), "john-smith/two"));
        assert_eq!((apps["org.example.Three"].released, apps["org.example.Three"].repo.as_str()), (None, ""));
    }

    #[test]
    fn reads_a_component() {
        let dir = std::env::temp_dir().join(format!("trindle-store-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("appstream.xml"), r#"<?xml version="1.0"?>
<components>
  <component type="desktop-application">
    <id>org.example.Thing.desktop</id>
    <name>Thing</name>
    <name xml:lang="de">Ding</name>
    <summary>Does &amp; things</summary>
    <description>
      <p>First <em>part</em>.</p>
      <p xml:lang="de">Erster.</p>
      <ul><li>One</li><li>Two</li></ul>
    </description>
    <developer><name>Some One</name></developer>
    <project_license>GPL-3.0</project_license>
    <categories><category>Utility</category><category>Office</category></categories>
    <url type="homepage">https://example.org</url>
    <keywords><keyword>stuff</keyword></keywords>
    <screenshots>
      <screenshot type="default">
        <caption xml:lang="de">Ding</caption>
        <caption>The main window</caption>
        <image type="source" width="1920">https://example.org/big.png</image>
        <image type="thumbnail" width="1248">https://example.org/1248.png</image>
        <image type="thumbnail" width="752">https://example.org/752.png</image>
        <image type="thumbnail" width="224">https://example.org/224.png</image>
      </screenshot>
    </screenshots>
  </component>
  <component type="runtime"><id>org.example.Runtime</id></component>
</components>"#).unwrap();
        let apps = super::parse(&dir).unwrap();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(apps.len(), 1);
        let a = &apps["org.example.Thing"];
        assert_eq!(a.name, "Thing");
        assert_eq!(a.summary, "Does & things");
        assert_eq!(a.description, "First part.\n\n  \u{2022} One\n\n  \u{2022} Two");
        assert_eq!(a.developer, "Some One");
        assert_eq!(a.categories, vec!["Utility", "Office"]);
        assert_eq!(a.homepage, "https://example.org");
        assert_eq!(a.keywords, "stuff");
        assert_eq!(a.screenshots, vec![("https://example.org/752.png".to_string(), "The main window".to_string())]);
    }
}
