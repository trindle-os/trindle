//! Reading and checking a plugin's manifest, and where plugins are kept.

use std::path::{Path, PathBuf};

use crate::common::files::data_home;
use crate::common::firewall::Door;
use crate::common::keyfile::{command_words, KeyFile};
use crate::store::boxed;
use crate::toolkit::t;

use super::shortcuts;
use super::fields::{CHOICE_TEXT_MAX, CHOICES_MAX, Field, good_choices_file, is_plain, Kind, show_when};
use super::menu::menu_items;
use super::permissions::{self, HOME_NETWORK, PERMISSIONS};
use super::ports::listen_ports;
use super::shortcuts::Shortcut;

pub const INTERFACE: i32 = 1;

pub(super) const MANIFEST: &str = "plugin.ini";

pub const NAME_MAX: usize = 48;
pub const VERSION_MAX: usize = 32;
pub const SUMMARY_MAX: usize = 200;

#[derive(Clone, Debug)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub version: String,

    pub summary: String,

    pub asks: Vec<String>,

    pub ring: Option<(PathBuf, String, Vec<String>)>,
    pub fields: Vec<Field>,
    pub background: Option<Vec<String>>,

    pub listen: Vec<Door>,

    pub shortcuts: Vec<Shortcut>,
}

pub fn folders() -> PathBuf {
    data_home().join("trindle/plugins")
}

pub fn folder(id: &str) -> PathBuf {
    folders().join(id)
}

pub fn home_of(id: &str) -> PathBuf {
    folder(id).join("home")
}

pub(crate) fn good_id(id: &str) -> bool {
    !id.is_empty() && id.len() <= 100 && !id.starts_with(['.', '-']) && !id.contains("..")
        && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
}

pub(super) fn good_name(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

pub(super) fn list(kf: &KeyFile, group: &str, key: &str) -> Vec<String> {
    kf.list(group, key).map(|l| l.iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
        .unwrap_or_default()
}

pub(super) fn text(kf: &KeyFile, group: &str, key: &str) -> String {
    kf.string(group, key).map(|s| s.trim().to_string()).unwrap_or_default()
}

fn inside(top: &Path, path: &Path) -> bool {
    match (top.canonicalize(), path.canonicalize()) {
        (Ok(t), Ok(p)) => p.starts_with(t),
        _ => false,
    }
}

pub(super) fn command(at: &Path, group: &str, run: &str, problems: &mut Vec<String>) -> Option<Vec<String>> {
    let Some(argv) = command_words(run) else {
        problems.push(t!("plugins-run-unreadable", group = group));
        return None;
    };
    let program = argv.first()?;
    let path = at.join(program);
    use std::os::unix::fs::PermissionsExt;
    if program.starts_with('/') || program.split('/').any(|p| p == "..") || !inside(at, &path) {
        problems.push(t!("plugins-run-outside", group = group, program = program));
    } else if !path.metadata().is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0) {
        problems.push(t!("plugins-run-missing", group = group, program = program));
    } else {
        return Some(argv);
    }
    None
}

pub const PIN_GROUP: &str = "PinUnlock";

fn pin_unlock(at: &Path, kf: &KeyFile, id: &str, asks: &[String], problems: &mut Vec<String>) -> Option<Vec<String>> {
    if !kf.has_group(PIN_GROUP) {
        return None;
    }
    if id != crate::common::own_hands::FLOW || !asks.iter().any(|a| a == "control") {
        problems.push(t!("plugins-pin-group-only-flow", group = PIN_GROUP));
        return None;
    }
    let run = text(kf, PIN_GROUP, "Run");
    if run.is_empty() {
        problems.push(t!("plugins-pin-group-needs-run", group = PIN_GROUP));
        return None;
    }
    command(at, PIN_GROUP, &run, problems)
}

pub fn good_computer(id: &str) -> bool {
    !id.is_empty() && id.len() <= 64 && !id.starts_with(['.', '-'])
        && id.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_'))
}

pub fn pin_command_in(folders: &Path, id: &str) -> Option<Vec<String>> {
    if !crate::common::own_hands::is_flow_in(folders, id) {
        return None;
    }
    let at = folders.join(id);
    let p = check(&at.join("app")).ok()?;
    let allowed = boxed::allowed_at(&at, PERMISSIONS);
    if !allowed.iter().any(|a| a == "control") || !p.asks.iter().any(|a| a == "control") {
        return None;
    }
    let kf = KeyFile::load(&at.join("app").join(MANIFEST)).ok()?;
    pin_unlock(&at.join("app"), &kf, &p.id, &p.asks, &mut Vec::new())
}

pub fn pin_command(id: &str) -> Option<Vec<String>> {
    pin_command_in(&folders(), id)
}

fn links_out(top: &Path, dir: &Path, problems: &mut Vec<String>) {
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let p = e.path();
        let Ok(kind) = e.file_type() else { continue };
        if kind.is_symlink() {
            let target = std::fs::read_link(&p).unwrap_or_default();
            if target.is_absolute() || !inside(top, &p) {
                problems.push(t!("plugins-links-outside", path = p.strip_prefix(top).unwrap_or(&p).display().to_string()));
            }
        } else if kind.is_dir() {
            links_out(top, &p, problems);
        }
    }
}

fn permission_rules(id: &str, asks: &[String], problems: &mut Vec<String>) {
    for (i, a) in asks.iter().enumerate() {
        if asks[..i].contains(a) {
            problems.push(format!("Permission {a} is named twice."));
        }
    }
    let has = |k: &str| asks.iter().any(|a| a == k);
    if has(super::permissions::ACCESSIBILITY) && id != crate::common::own_hands::SCREEN_READER {
        problems.push(t!("plugins-accessibility-only-reader").into());
    }
    if has(HOME_NETWORK) && has("network") {
        problems.push(format!("Ask for {HOME_NETWORK} or network, not both: network reaches the home network as well as the internet."));
    }
}

pub fn check(at: &Path) -> Result<Plugin, Vec<String>> {
    let mut problems = Vec::new();
    let kf = match KeyFile::load(&at.join(MANIFEST)) {
        Ok(kf) => kf,
        Err(e) => return Err(vec![t!("plugins-check-no-manifest", file = MANIFEST, why = e.to_string())]),
    };
    if !kf.has_group("Plugin") {
        return Err(vec![t!("plugins-check-no-plugin-part", file = MANIFEST)]);
    }
    let About { id, name, version, summary, asks } = check_about(&kf, &mut problems);
    let ring = check_ring(at, &kf, &mut problems);
    let shortcuts = shortcuts::parse(at, &kf, &shortcuts::kept_by_trindle, &mut problems);
    pin_unlock(at, &kf, &id, &asks, &mut problems);
    let fields = check_fields(&kf, &mut problems);

    let mut background = None;
    if kf.has_group("Background") {
        if !asks.iter().any(|a| a == "background") {
            problems.push(t!("plugins-background-needs-permission").into());
        }
        background = command(at, "Background", &text(&kf, "Background", "Run"), &mut problems);
    }

    let listen = listen_ports(&kf, &asks, &mut problems);

    if ring.is_none() && fields.is_empty() && background.is_none() && shortcuts.is_empty()
        && !asks.iter().any(|a| a == "notifications" || a == permissions::ACCESSIBILITY) {
        problems.push(t!("plugins-hooks-nothing").into());
    }
    links_out(at, at, &mut problems);
    if problems.is_empty() {
        Ok(Plugin { id, name, version, summary, asks, ring, fields, background, listen, shortcuts })
    } else {
        Err(problems)
    }
}

struct About {
    id: String,
    name: String,
    version: String,
    summary: String,
    asks: Vec<String>,
}

fn check_about(kf: &KeyFile, problems: &mut Vec<String>) -> About {
    match kf.integer("Plugin", "Interface") {
        Some(n) if n == i64::from(INTERFACE) => {}
        Some(n) => problems.push(t!("plugins-check-interface-other", n = n, known = INTERFACE)),
        None => problems.push(t!("plugins-check-interface-missing", known = INTERFACE)),
    }
    let id = text(kf, "Plugin", "Id");
    if !good_id(&id) {
        problems.push(t!("plugins-check-id").into());
    }
    let name = text(kf, "Plugin", "Name");
    if name.is_empty() {
        problems.push(t!("plugins-check-needs-name").into());
    } else if !is_plain(&name, NAME_MAX) {
        problems.push(t!("plugins-plain-line", group = "Plugin", key = "Name", max = NAME_MAX));
    }
    let version = text(kf, "Plugin", "Version");
    if version.is_empty() {
        problems.push(t!("plugins-check-needs-version").into());
    } else if !is_plain(&version, VERSION_MAX) {
        problems.push(t!("plugins-plain-line", group = "Plugin", key = "Version", max = VERSION_MAX));
    }
    let summary = text(kf, "Plugin", "Summary");
    if summary.contains('\n') {
        problems.push(t!("plugins-check-summary-one-line").into());
    } else if !is_plain(&summary, SUMMARY_MAX) {
        problems.push(t!("plugins-plain-line", group = "Plugin", key = "Summary", max = SUMMARY_MAX));
    }
    let asks = list(kf, "Plugin", "Permissions");
    for a in &asks {
        if !PERMISSIONS.contains(&a.as_str()) {
            let known: Vec<&str> = PERMISSIONS.to_vec();
            problems.push(t!("plugins-check-permission-unknown", name = a, list = known.join(", ")));
        }
    }
    permission_rules(&id, &asks, problems);
    About { id, name, version, summary, asks }
}

fn check_ring(at: &Path, kf: &KeyFile, problems: &mut Vec<String>) -> Option<(PathBuf, String, Vec<String>)> {
    if !kf.has_group("Ring") {
        return None;
    }
    let mut ring = None;
    let icon = text(kf, "Ring", "Icon");
    let path = at.join(&icon);
    let label = text(kf, "Ring", "Label");
    if icon.is_empty() || !(icon.ends_with(".svg") || icon.ends_with(".png")) {
        problems.push(t!("plugins-ring-needs-icon").into());
    } else if !path.is_file() || !inside(at, &path) {
        problems.push(t!("plugins-ring-icon-missing", icon = &icon));
    }
    if label.is_empty() {
        problems.push(t!("plugins-ring-needs-label").into());
    } else if !is_plain(&label, NAME_MAX) {
        problems.push(t!("plugins-plain-line", group = "Ring", key = "Label", max = NAME_MAX));
    }
    let run = text(kf, "Ring", "Run");
    match command(at, "Ring", &run, problems) {
        Some(argv) => ring = Some((path, label, argv)),
        None if run.is_empty() => problems.push(t!("plugins-ring-needs-run").into()),
        None => {}
    }
    menu_items(at, kf, problems);
    ring
}

fn check_fields(kf: &KeyFile, problems: &mut Vec<String>) -> Vec<Field> {
    let mut fields = Vec::new();
    if !kf.has_group("Settings") {
        return fields;
    }
    let names = list(kf, "Settings", "Fields");
    if names.is_empty() {
        problems.push(t!("plugins-settings-needs-fields").into());
    }
    let all_names = names.clone();
    for (index, n) in names.into_iter().enumerate() {
        let group = format!("Field {n}");
        if !good_name(&n) {
            problems.push(t!("plugins-field-bad-name", name = &n));
            continue;
        }
        if !kf.has_group(&group) {
            problems.push(t!("plugins-field-no-part", name = &n, group = &group));
            continue;
        }
        let label = text(kf, &group, "Label");
        let is_note = text(kf, &group, "Kind") == "note";
        if label.is_empty() && !is_note {
            problems.push(t!("plugins-needs-label", group = &group));
        } else if !label.is_empty() && !is_plain(&label, NAME_MAX) {
            problems.push(t!("plugins-plain-line", group = &group, key = "Label", max = NAME_MAX));
        }
        let default = text(kf, &group, "Default");
        let Some((kind, choices_from)) = field_kind(kf, &group, &default, problems) else { continue };
        if kind == Kind::Switch && !matches!(default.as_str(), "" | "true" | "false") {
            problems.push(t!("plugins-switch-default", group = &group));
        }
        let show_when = show_when(&group, &n, index, &all_names, &fields, &text(kf, &group, "ShowWhen"), problems);
        fields.push(Field { name: n, label, kind, default, choices_from, show_when });
    }
    fields
}

fn field_kind(kf: &KeyFile, group: &str, default: &str, problems: &mut Vec<String>) -> Option<(Kind, Option<String>)> {
    let mut choices_from = None;
    let kind = match text(kf, group, "Kind").as_str() {
        "switch" => Kind::Switch,
        "text" => Kind::Text,
        "choice" => {
            let choices = list(kf, group, "Choices");
            if choices.len() > CHOICES_MAX || choices.iter().any(|c| !is_plain(c, CHOICE_TEXT_MAX)) {
                problems.push(t!("plugins-choices-limits", group = group, most = CHOICES_MAX, chars = CHOICE_TEXT_MAX));
            }
            let file = text(kf, group, "ChoicesFrom");
            if !file.is_empty() && !good_choices_file(&file) {
                problems.push(t!("plugins-choices-from-name", group = group));
            } else if file.is_empty() && choices.len() < 2 {
                problems.push(t!("plugins-choice-needs-choices", group = group));
            }
            if !choices.is_empty() && !default.is_empty() && !choices.iter().any(|c| c == default) && file.is_empty() {
                problems.push(t!("plugins-choice-default", group = group));
            }
            choices_from = (!file.is_empty() && good_choices_file(&file)).then_some(file);
            Kind::Choice(choices)
        }
        "note" => {
            let file = text(kf, group, "TextFrom");
            if !good_choices_file(&file) {
                problems.push(t!("plugins-note-needs-textfrom", group = group));
                return None;
            }
            if !default.is_empty() {
                problems.push(t!("plugins-note-default", group = group));
            }
            Kind::Note(file)
        }
        other => {
            problems.push(t!("plugins-field-kind", group = group, kind = format!("{other:?}")));
            return None;
        }
    };
    Some((kind, choices_from))
}

#[cfg(test)]
mod tests;
