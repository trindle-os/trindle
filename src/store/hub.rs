//! Where AppImages are listed from.

use std::path::PathBuf;

use serde_json::Value;

use super::github::{self, Found};
use crate::common::run::{self, capture};

const FEED: &str = "https://appimage.github.io/feed.json";
const ICONS: &str = "https://appimage.github.io/database/";
const DAY: u64 = 24 * 3600;

pub fn fetch(done: impl FnOnce(Vec<Found>) + 'static) {
    github::tidy();
    let file = github::cache().join("appimagehub.json");
    let fresh = std::fs::metadata(&file).and_then(|m| m.modified()).ok()
        .and_then(|t| t.elapsed().ok()).is_some_and(|age| age.as_secs() < DAY);
    let read = move |path: PathBuf| run::on_thread(move || parse(&std::fs::read_to_string(path).unwrap_or_default()), done);
    if !crate::common::update_checks::may_fetch_catalogue(file.exists(), fresh, crate::common::update_checks::on()) {
        return read(file);
    }
    let _ = std::fs::create_dir_all(github::cache());
    let tmp = file.with_extension("new");
    let t = tmp.display().to_string();
    capture(&["curl", "-fsSL", "--proto", "=https", "--proto-redir", "=https", "--max-time", "60",
              "--max-filesize", run::MAX_CATALOGUE, "-o", &t, FEED], None, move |ok, _, _| {
        if ok {
            let _ = std::fs::rename(&tmp, &file);
        }
        read(file);
    });
}

fn strip_tags(html: &str) -> String {
    let mut out = String::new();
    let mut inside = false;
    for c in html.chars() {
        match c {
            '<' => inside = true,
            '>' => { inside = false; out.push(' '); }
            c if !inside => out.push(c),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn parse(text: &str) -> Vec<Found> {
    let v: Value = serde_json::from_str(text).unwrap_or(Value::Null);
    let Some(items) = v["items"].as_array() else { return Vec::new() };
    items.iter().filter_map(|i| {
        let links = i["links"].as_array();
        let link = |kind: &str| links.and_then(|l| l.iter().find(|l| l["type"] == kind)).and_then(|l| l["url"].as_str());
        let repo = link("GitHub").map(|u| u.trim_matches('/').to_string())
            .filter(|r| github::repo_of_url(&format!("https://github.com/{r}")).as_deref() == Some(r.as_str()))
            .unwrap_or_default();
        let site = link("Download").filter(|u| repo.is_empty() && crate::toolkit::link::is_web(u) && u.starts_with("https://"))
            .unwrap_or_default().to_string();
        if repo.is_empty() && site.is_empty() {
            return None;
        }
        let summary = strip_tags(i["description"].as_str().unwrap_or("")).chars().take(github::SUMMARY_MOST).collect();
        let icon = i["icons"].as_array().and_then(|a| a.first()).and_then(Value::as_str)
            .filter(|p| !p.contains("..") && p.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '.' | '-' | '_')))
            .map(|p| format!("{ICONS}{p}")).unwrap_or_default();
        let categories = i["categories"].as_array().map(|a| a.iter().filter_map(Value::as_str).map(String::from).collect()).unwrap_or_default();
        Some(Found { repo, site, icon, categories, summary, name: i["name"].as_str().unwrap_or("").to_string(),
                     listed: true, ..Default::default() })
    }).collect()
}

pub fn matching<'a>(listed: &'a [Found], words: &[String]) -> Vec<&'a Found> {
    let mut found: Vec<&Found> = listed.iter().filter(|f| {
        let hay = format!("{} {} {}", f.name, f.repo, f.summary).to_lowercase();
        words.iter().all(|w| hay.contains(w.as_str()))
    }).collect();
    let first = words.first().map(String::as_str).unwrap_or("");
    found.sort_by_key(|f| (!f.name.to_lowercase().starts_with(first), f.name.to_lowercase()));
    found
}

pub fn search(listed: &[Found], words: &[String]) -> Vec<Found> {
    matching(listed, words).into_iter().cloned().collect()
}

const POPULAR: [&str; 30] = [
    "Obsidian", "VLC", "Audacity", "Bitwarden", "Joplin", "Cura", "Shotcut", "LMMS", "KeePassXC", "Darktable",
    "Heroic", "Bottles", "LosslessCut", "Cryptomator", "Firefox", "Thunderbird", "FreeTube", "Zettlr", "draw.io", "ONLYOFFICE",
    "SMPlayer", "Luanti", "OpenRA", "Session", "Mailspring", "Museeks", "TreeSheets", "CherryTree", "KeeWeb", "KDiskMark",
];

pub fn popular(listed: &[Found]) -> Vec<&Found> {
    POPULAR.iter().filter_map(|name| listed.iter().find(|f| f.name.eq_ignore_ascii_case(name))).collect()
}

pub fn in_kind(f: &Found, code: &str) -> bool {
    f.categories.iter().any(|c| c == code || (code == "AudioVideo" && (c == "Audio" || c == "Video")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_list() {
        let text = r#"{"items":[{"name":"Thing","description":"<p>Does <b>stuff</b></p>","categories":["Audio","Utility"],
            "icons":["Thing/icons/128x128/thing.png"],"links":[{"type":"GitHub","url":"john-smith/thing"}]},
            {"name":"Elsewhere","links":[{"type":"Download","url":"https://example.org/get"}],"icons":["../x.png"]},
            {"name":"Plain","links":[{"type":"Download","url":"http://example.org/get"}]},
            {"name":"Nowhere","links":[]}]}"#;
        let found = parse(text);
        assert_eq!(found.len(), 2);
        assert_eq!((found[0].repo.as_str(), found[0].summary.as_str()), ("john-smith/thing", "Does stuff"));
        assert_eq!(found[0].icon, "https://appimage.github.io/database/Thing/icons/128x128/thing.png");
        assert_eq!((found[1].repo.as_str(), found[1].site.as_str(), found[1].icon.as_str()), ("", "https://example.org/get", ""));
        assert!(in_kind(&found[0], "AudioVideo") && in_kind(&found[0], "Utility") && !in_kind(&found[1], "Utility"));
        assert_eq!(search(&found, &["stuff".into()]).len(), 1);
    }

    #[test]
    fn the_popular_list_is_whatever_of_it_the_feed_has_that_day() {
        let entry = |name: &str| Found { name: name.into(), repo: "john-smith/thing".into(), listed: true, ..Default::default() };
        let feed = [entry("Zzz"), entry("kdiskmark"), entry("Obsidian")];
        let names: Vec<&str> = popular(&feed).iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, ["Obsidian", "kdiskmark"], "in the list's order, in any case, and no name the feed lacks");
        assert!(popular(&[]).is_empty());
        let mut seen = std::collections::HashSet::new();
        assert!(POPULAR.iter().all(|n| seen.insert(n.to_lowercase())), "a name is listed once");
    }
}
