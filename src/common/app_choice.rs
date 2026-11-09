//! Choosing an app to open something with: which apps are offered and in
//! what order, finding one by typing, and remembering the one used last.
//! Shared by the portal's app chooser and anything else that asks, such as
//! a file manager's Open with.

use super::apps::{self, Entry};

const KEEP: usize = 10;

fn record() -> std::path::PathBuf {
    super::files::config_path("app-choices")
}

pub fn recent(kind: &str) -> Vec<String> {
    recent_in(&read(&record()), kind)
}

pub const DAMAGED: &str = "Last-used apps";

pub fn read(file: &std::path::Path) -> String {
    super::damaged::read(file, DAMAGED, |l| l.split_once('\t').is_some())
}

fn recent_in(text: &str, kind: &str) -> Vec<String> {
    text.lines().filter_map(|l| l.split_once('\t')).filter(|(k, _)| *k == kind).map(|(_, id)| id.to_string()).collect()
}

pub fn remember(kind: &str, id: &str) {
    remember_in(&record(), kind, id);
}

pub(crate) fn remember_in(path: &std::path::Path, kind: &str, id: &str) {
    if kind.is_empty() || id.is_empty() || kind.contains(['\t', '\n']) || id.contains(['\t', '\n']) {
        return;
    }
    let was = read(path);
    if super::damaged::may_write(path, DAMAGED).is_err() {
        return;
    }
    let text = remembered(&was, kind, id);
    let _ = super::files::write_text(path, &text);
}

fn remembered(text: &str, kind: &str, id: &str) -> String {
    let mut out = format!("{kind}\t{id}\n");
    let mut kept = 1;
    for line in text.lines() {
        let Some((k, i)) = line.split_once('\t') else { continue };
        if k == kind && (i == id || kept >= KEEP) {
            continue;
        }
        kept += usize::from(k == kind);
        out.push_str(line);
        out.push('\n');
    }
    out
}

pub fn always(kind: &str, id: &str) -> Result<(), String> {
    apps::choose_opener(id, &[kind])
}

pub fn ordered(all: &[Entry], offered: &[String], kind: &str, first: &[String]) -> Vec<Entry> {
    let same = |a: &Entry, id: &str| a.id == id || a.key() == id;
    let mut pool: Vec<Entry> = match offered.is_empty() {
        true => apps::openers_of(all, kind),
        false => all.iter().filter(|a| offered.iter().any(|id| same(a, id))).cloned().collect(),
    };
    let mut out = Vec::new();
    let opener = (!kind.is_empty()).then(|| apps::chosen_of(all, kind)).flatten().map(|a| a.id);
    let wanted = first.iter().cloned().chain(recent(kind)).chain(opener);
    for id in wanted {
        if let Some(at) = pool.iter().position(|a| same(a, &id)) {
            out.push(pool.remove(at));
        }
    }
    pool.sort_by_key(|a| a.name.to_lowercase());
    out.extend(pool);
    out
}

pub fn matches(app: &Entry, query: &str) -> bool {
    let all = format!("{} {} {} {}", app.name, app.keywords.join(" "), app.program, app.key()).to_lowercase();
    query.to_lowercase().split_whitespace().all(|w| all.contains(w))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn app(id: &str, name: &str, kinds: &[&str]) -> Entry {
        Entry {
            id: format!("{id}.desktop"), name: name.into(), icon: None, exec: id.into(), try_exec: None,
            wm_class: String::new(), program: id.into(), kinds: kinds.iter().map(|k| k.to_string()).collect(),
            path: PathBuf::from(format!("/nowhere/{id}.desktop")), category: "", keywords: vec!["Viewer".into()],
            flatpak: None, appimage: None, terminal: false,
        }
    }

    #[test]
    fn the_last_used_is_kept_first_and_only_ten_are_kept() {
        let mut text = String::new();
        for n in 0..12 {
            text = remembered(&text, "image/png", &format!("app{n}.desktop"));
        }
        text = remembered(&text, "text/plain", "editor.desktop");
        text = remembered(&text, "image/png", "app5.desktop");
        let png = recent_in(&text, "image/png");
        assert_eq!(png.len(), 10);
        assert_eq!(png[0], "app5.desktop");
        assert_eq!(png[1], "app11.desktop");
        assert_eq!(png.iter().filter(|i| *i == "app5.desktop").count(), 1);
        assert_eq!(recent_in(&text, "text/plain"), vec!["editor.desktop"]);
    }

    #[test]
    fn apps_are_offered_with_the_wanted_ones_first() {
        let all = vec![app("zed", "Zed", &["text/plain"]), app("gimp", "GIMP", &["image/png"]), app("eog", "Eye of GNOME", &["image/*"])];
        let names = |v: Vec<Entry>| v.into_iter().map(|a| a.key().to_string()).collect::<Vec<_>>();
        let offered = vec!["zed".to_string(), "gimp".to_string(), "missing".to_string()];
        assert_eq!(names(ordered(&all, &offered, "", &["gimp".into()])), vec!["gimp", "zed"]);
        assert_eq!(names(ordered(&all, &[], "", &[])), Vec::<String>::new());
        let png = names(ordered(&all, &[], "image/png", &["eog.desktop".into()]));
        assert_eq!(png[0], "eog");
        assert_eq!(png.len(), 2);
        assert!(matches(&all[2], "eye gnome"));
        assert!(matches(&all[2], "VIEW"));
        assert!(!matches(&all[2], "gimp"));
    }
}
