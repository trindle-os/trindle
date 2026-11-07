//! Backup and restore of apps, their data and Trindle's settings.

use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::put_back;
use crate::toolkit::t;
use crate::common::files::{home, user};
use crate::common::run::{capture, on_thread};

fn outside_words() -> String {
    super::refusal::nothing(t!("store-because-outside"))
}

pub const MANIFEST: &str = "trindle-backup.json";
const FACE: &str = "trindle-face.png";
const HOME_PART: &str = "trindle/home.tar.gz";
const FACE_PART: &str = "trindle/face.png";

const SETTINGS: &[&str] = &[
    ".config/trindle",
    ".config/gtk-3.0/settings.ini",
    ".config/gtk-4.0/settings.ini",
    ".config/mimeapps.list",
];
const APP_DATA: &str = ".var/app";
const APPIMAGES: &str = ".appimage";

#[derive(Clone, Copy)]
pub struct Parts {
    pub apps: bool,
    pub data: bool,
    pub appimages: bool,
    pub settings: bool,
}

pub fn default_file() -> PathBuf {
    let date = crate::common::clock::Local::now().format("%Y-%m-%d");
    home().join(format!("trindle-backup-{date}.fmpack"))
}

pub fn size_of(rel: &'static str, done: impl FnOnce(String) + 'static) {
    let path = home().join(rel);
    if !path.exists() {
        crate::common::run::soon(move || done(t!("store-backup-nothing-yet").into()));
        return;
    }
    let p = path.display().to_string();
    capture(&["du", "-sh", "--exclude=cache", "--exclude=.cache", &p], None, move |_, out, _| {
        done(out.split_whitespace().next().map(|s| t!("store-backup-about", size = s)).unwrap_or_default());
    });
}

pub fn backup(parts: Parts, dest: PathBuf, done: impl FnOnce(Result<String, String>) + 'static) {
    on_thread(move || make_pack(parts, &dest).map_err(|e| {
        t!("store-backup-not-made", why = super::refusal::because(&e))
    }), done);
}

fn sh(args: &[&str]) -> Result<String, String> {
    let out = crate::common::gentle::program(&mut std::process::Command::new(args[0])).args(&args[1..]).stdin(std::process::Stdio::null()).output()
        .map_err(|e| e.to_string())?;
    if out.status.success() || (args[0] == "tar" && out.status.code() == Some(1)) {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(super::refusal::last_said(&String::from_utf8_lossy(&out.stderr)))
    }
}

fn make_pack(parts: Parts, dest: &Path) -> Result<String, String> {
    let stage = crate::common::files::private_temp("trindle-backup").map_err(|e| e.to_string())?;
    let Some(name) = dest.file_name() else { return Err(t!("store-because-no-name").into()) };
    let spare = dest.with_file_name(crate::common::files::spare_name(name, 0));
    let result = pack_into(parts, &spare, &stage);
    let _ = std::fs::remove_dir_all(&stage);
    let result = result.and_then(|size| std::fs::rename(&spare, dest).map(|()| size).map_err(|e| e.to_string()));
    if result.is_err() {
        let _ = std::fs::remove_file(&spare);
    }
    result
}

fn pack_into(parts: Parts, dest: &Path, stage: &Path) -> Result<String, String> {
    let err = |e: std::io::Error| e.to_string();
    std::fs::create_dir_all(stage.join("apps")).map_err(err)?;
    std::fs::create_dir_all(stage.join("overrides/app")).map_err(err)?;
    let apps = flatpak_apps();
    let zstd = sh(&["bash", "-c", "printf x | tar --zstd -cf - -T /dev/null >/dev/null 2>&1"]).is_ok();
    let overrides = home().join(".local/share/flatpak/overrides");
    let packed = if parts.data { pack_data(stage, &apps, zstd, &overrides)? } else { Vec::new() };
    let global = std::fs::read_to_string(overrides.join("global")).ok();
    if let Some(t) = &global {
        std::fs::write(stage.join("overrides/global"), t).map_err(err)?;
    }

    let list: Vec<Value> = apps.iter()
        .filter(|a| parts.apps || packed.iter().any(|p| p["id"] == a["id"]))
        .map(|a| json!({"id": a["id"], "name": a["name"], "remote": a["origin"],
                        "keep": packed.iter().any(|p| p["id"] == a["id"])}))
        .collect();
    let remotes = flatpak_remotes();

    std::fs::create_dir_all(stage.join("trindle")).map_err(err)?;
    let has_home = pack_home(&parts, stage)?;
    let face = PathBuf::from(crate::common::faces::FACES).join(format!("{}.png", user()));
    let has_face = parts.settings && face.is_file() && std::fs::copy(&face, stage.join(FACE_PART)).is_ok();

    let manifest = json!({
        "format_version": 1,
        "created": crate::common::clock::utc_iso(crate::common::clock::Local::now().unix),
        "source_host": std::fs::read_to_string("/etc/hostname").unwrap_or_default().trim(),
        "source_distro": "Trindle",
        "compression": if zstd { "zstd" } else { "gzip" },
        "has_global_override": global.is_some(),
        "remotes": remotes,
        "list": list,
        "apps": packed,
        "made_by": "Trindle",
        "trindle": {"version": env!("CARGO_PKG_VERSION"), "home": has_home, "face": has_face},
    });
    std::fs::write(stage.join("manifest.json"), serde_json::to_string_pretty(&manifest).unwrap_or_default())
        .map_err(err)?;
    let mut members = vec!["manifest.json", "apps", "overrides"];
    if has_home || has_face {
        members.push("trindle");
    }
    let mut args = vec!["tar", "-cf"];
    let d = lossy(dest);
    let st = lossy(stage);
    args.push(&d);
    args.extend(["-C", &st]);
    args.extend(members);
    sh(&args)?;
    Ok(std::fs::metadata(dest).map(|m| crate::store::github::size_text(m.len())).unwrap_or_default())
}

fn lossy(p: &Path) -> String {
    p.to_string_lossy().to_string()
}

fn flatpak_apps() -> Vec<Value> {
    let mut apps: Vec<Value> = Vec::new();
    for l in sh(&["flatpak", "list", "--app", "--columns=name,application,version,branch,arch,origin,installation"])
        .unwrap_or_default().lines()
    {
        let c: Vec<&str> = l.split('\t').map(str::trim).collect();
        let Some(id) = c.get(1).filter(|i| !i.is_empty()) else { continue };
        let scope = if c.get(6) == Some(&"user") { "user" } else { "system" };
        if let Some(prev) = apps.iter_mut().find(|a| a["id"] == *id) {
            if scope == "user" {
                prev["scope"] = json!("user");
            }
            continue;
        }
        apps.push(json!({"id": id, "name": c.first().filter(|n| !n.is_empty()).unwrap_or(id),
                         "branch": c.get(3).filter(|b| !b.is_empty()).unwrap_or(&"stable"),
                         "arch": c.get(4).unwrap_or(&"x86_64"), "origin": c.get(5).unwrap_or(&"flathub"),
                         "scope": scope}));
    }
    apps
}

fn pack_data(stage: &Path, apps: &[Value], zstd: bool, overrides: &Path) -> Result<Vec<Value>, String> {
    let (flag, ext) = if zstd { ("--zstd", "tar.zst") } else { ("--gzip", "tar.gz") };
    let var_app = home().join(APP_DATA);
    let mut packed: Vec<Value> = Vec::new();
    for a in apps {
        let id = a["id"].as_str().unwrap_or_default().to_string();
        if !var_app.join(&id).is_dir() {
            continue;
        }
        let blob = format!("apps/{id}.{ext}");
        let file = stage.join(&blob);
        sh(&["tar", flag, "-cpf", &lossy(&file), &format!("--exclude={id}/cache"), "-C", &lossy(&var_app), &id])?;
        let sha = sh(&["sha256sum", &lossy(&file)])?.split_whitespace().next().unwrap_or_default().to_string();
        let own = std::fs::read_to_string(overrides.join(&id)).ok();
        if let Some(t) = &own {
            std::fs::write(stage.join("overrides/app").join(&id), t).map_err(|e| e.to_string())?;
        }
        let mut m = a.clone();
        m["blob"] = json!(blob);
        m["blob_bytes"] = json!(std::fs::metadata(&file).map(|m| m.len()).unwrap_or(0));
        m["sha256"] = json!(sha);
        m["cache_included"] = json!(false);
        m["has_overrides"] = json!(own.is_some());
        m["overrides_scope"] = if own.is_some() { json!("user") } else { Value::Null };
        packed.push(m);
    }
    Ok(packed)
}

fn flatpak_remotes() -> Vec<Value> {
    let mut remotes: Vec<Value> = Vec::new();
    for l in sh(&["flatpak", "remotes", "--columns=name,url"]).unwrap_or_default().lines() {
        if let Some((n, u)) = l.split_once('\t') {
            if !remotes.iter().any(|r| r["name"] == n.trim()) {
                remotes.push(json!({"name": n.trim(), "url": u.trim()}));
            }
        }
    }
    remotes
}

fn pack_home(parts: &Parts, stage: &Path) -> Result<bool, String> {
    let mut wanted: Vec<&str> = Vec::new();
    if parts.settings {
        wanted.extend(SETTINGS);
    }
    if parts.appimages {
        wanted.push(APPIMAGES);
    }
    let wanted: Vec<String> = wanted.into_iter().filter(|p| home().join(p).exists()).map(String::from).collect();
    let has_home = !wanted.is_empty();
    if has_home {
        let mut args: Vec<String> = vec!["tar".into(), "-czf".into(), lossy(&stage.join(HOME_PART)),
            "--exclude=.config/trindle/github-key".into(), "-C".into(), lossy(&home())];
        args.extend(wanted);
        let argv: Vec<&str> = args.iter().map(String::as_str).collect();
        sh(&argv)?;
    }
    Ok(has_home)
}

pub fn restore_trindle_part(pack: PathBuf, say: impl Fn(&str) + 'static, done: impl FnOnce(Result<(), String>) + 'static) {
    say(t!("store-backup-step-trindle"));
    let Ok(work) = crate::common::files::private_temp("trindle-restore") else {
        return done(Err(super::refusal::nothing(t!("store-because-no-unpack"))));
    };
    let w = work.clone();
    on_thread(move || -> Result<Face, String> {
        let listing = put_back::list_of(&pack, None).map_err(super::refusal::explained)?;
        home_part_from(&pack, &listing, &w, &home())?;
        Ok(face_from(&pack, None, FACE_PART, &listing))
    }, move |res| {
        let _ = std::fs::remove_dir_all(&work);
        match res {
            Err(e) => done(Err(e)),
            Ok(Ok(Some(data))) => send_face(data, move |face| finish(move |res| done(with_face(res, face)))),
            Ok(Ok(None)) => finish(done),
            Ok(Err(why)) => finish(move |res| done(with_face(res, Some(why)))),
        }
    });
}

type Face = Result<Option<Vec<u8>>, String>;

pub(super) fn face_out(pack: &Path, how: Option<&str>, member: &str) -> Face {
    let listing = put_back::list_of(pack, how).map_err(|e| t!("store-refusal-not-read-from", why = e))?;
    face_from(pack, how, member, &listing)
}

pub(super) fn face_from(pack: &Path, how: Option<&str>, member: &str, listing: &str) -> Face {
    let why = |e: String| t!("store-refusal-not-read-from", why = e);
    if !put_back::holds_in(listing, member) {
        return Ok(None);
    }
    let data = put_back::read_out(pack, how, member, crate::admin::MAX_FACE as u64).map_err(why)?;
    Ok(Some(data).filter(|data| !data.is_empty()))
}

pub(super) fn home_part_from(pack: &Path, listing: &str, work: &Path, home: &Path) -> Result<(), String> {
    (|| {
        if !put_back::holds_in(listing, HOME_PART) {
            return Ok(());
        }
        let inner = put_back::taken_out(pack, HOME_PART, work)?;
        let listing = put_back::listed(&inner, "--gzip")?;
        let mut wanted: Vec<&str> = SETTINGS.to_vec();
        wanted.push(APPIMAGES);
        put_back::put_back(&inner, "--gzip", &listing, &wanted, home, put_back::Holds::FilesAndFolders)
    })().map_err(|e| if e == put_back::outside() { outside_words() } else { super::refusal::explained(e) })
}

pub(super) fn old_into(file: &Path, home: &Path) -> Result<(), String> {
    let listing = put_back::listed(file, "--gzip").map_err(|e| super::refusal::with_behind(&super::refusal::nothing(t!("store-because-unreadable")), &[e]))?;
    let mut wanted: Vec<&str> = SETTINGS.to_vec();
    wanted.push(APPIMAGES);
    put_back::put_back(file, "--gzip", &listing, &wanted, home, put_back::Holds::FilesAndFolders)
        .and_then(|()| put_back::put_back(file, "--gzip", &listing, &[APP_DATA], home, put_back::Holds::Anything))
        .map_err(|e| if e == put_back::outside() { outside_words() } else { super::refusal::explained(e) })
}

pub fn read_manifest(file: &Path, done: impl FnOnce(Result<Value, String>) + 'static) {
    let file = file.to_path_buf();
    on_thread(move || manifest_of(&file), done);
}

fn manifest_of(file: &Path) -> Result<Value, String> {
    let read = put_back::read_out(file, Some("--gzip"), MANIFEST, crate::common::files::SMALL_MOST);
    let manifest = read.ok().and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok()).filter(|v| v["made_by"] == "Trindle");
    manifest.ok_or_else(|| super::refusal::nothing(t!("store-because-not-trindle")))
}

fn addresses(manifest: &Value) -> Vec<&str> {
    let used = |name: &str| manifest["flatpaks"].as_array().map(Vec::as_slice).unwrap_or_default().iter().any(|x| x["origin"].as_str() == Some(name));
    manifest["remotes"].as_array().map(Vec::as_slice).unwrap_or_default().iter()
        .filter_map(|r| Some((r["name"].as_str()?, r["url"].as_str()?)))
        .filter(|(name, _)| *name != "flathub" && super::flat::valid_remote(name) && used(name))
        .map(|(_, url)| url).collect()
}

pub fn rule_of(file: &Path) -> super::link::Rule {
    if super::flat::is_flat(file) {
        return super::flat::read(file).map_or(super::link::Rule::Nothing, |f| super::link::rule(f.addresses()));
    }
    manifest_of(file).map_or(super::link::Rule::Nothing, |m| super::link::rule(addresses(&m)))
}

pub fn restore(file: PathBuf, manifest: Value, ticked: bool, say: impl Fn(&str) + 'static,
               done: impl FnOnce(Result<(), String>) + 'static) {
    let say: std::rc::Rc<dyn Fn(&str)> = std::rc::Rc::new(say);
    say(t!("store-backup-step-data"));
    on_thread(move || {
        old_into(&file, &home())?;
        Ok(face_out(&file, Some("--gzip"), FACE))
    }, move |res: Result<Face, String>| match res {
        Err(e) => done(Err(e)),
        Ok(Ok(Some(data))) => send_face(data, move |face| reinstall(manifest, ticked, say, move |res| done(with_face(res, face)))),
        Ok(Ok(None)) => reinstall(manifest, ticked, say, done),
        Ok(Err(why)) => reinstall(manifest, ticked, say, move |res| done(with_face(res, Some(why)))),
    });
}

fn send_face(data: Vec<u8>, then: impl FnOnce(Option<String>) + 'static) {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let helper = crate::common::run::admin_helper();
    let user = user();
    on_thread(move || {
        let child = Command::new("pkexec").args([&helper, "face", &user])
            .stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::piped()).spawn();
        let mut child = match child {
            Ok(child) => child,
            Err(e) => return Some(t!("store-refusal-pkexec", why = e.to_string())),
        };
        if let Some(mut input) = child.stdin.take() {
            let _ = input.write_all(&data);
        }
        match child.wait_with_output() {
            Ok(out) if out.status.success() => None,
            Ok(out) => Some(crate::common::run::admin_error(&String::from_utf8_lossy(&out.stderr))),
            Err(e) => Some(e.to_string()),
        }
    }, then);
}

fn with_face(res: Result<(), String>, face: Option<String>) -> Result<(), String> {
    match (res, face) {
        (res, None) => res,
        (Ok(()), Some(why)) => Err(t!("store-refusal-picture-only", why = why)),
        (Err(e), Some(why)) => {
            let stop = if e.trim_end().ends_with(['.', '!', '?']) { "" } else { "." };
            Err(t!("store-refusal-picture-too", words = format!("{}{stop}", e.trim_end()), why = why))
        }
    }
}

fn reinstall(manifest: Value, ticked: bool, say: std::rc::Rc<dyn Fn(&str)>, done: impl FnOnce(Result<(), String>) + 'static) {
    let sorted = by_remote(&manifest, ticked);
    if sorted.groups.is_empty() && sorted.refused.is_empty() {
        return finish(done);
    }
    for (name, url) in &sorted.add {
        let args = super::flat::remote_add_argv(name, url, None);
        let _ = std::process::Command::new(&args[0]).args(&args[1..]).status();
    }
    let count: usize = sorted.groups.values().map(Vec::len).sum();
    say(&t!("store-backup-step-installing", count = count));
    install_groups(sorted.groups.into_iter().collect(), sorted.refused, Vec::new(), Vec::new(), done);
}

#[derive(Default)]
struct ByRemote {
    add: Vec<(String, String)>,
    groups: std::collections::BTreeMap<String, Vec<String>>,
    refused: Vec<String>,
}

fn by_remote(manifest: &Value, ticked: bool) -> ByRemote {
    let mut sorted = ByRemote::default();
    let mut insecure: Vec<&str> = Vec::new();
    for r in manifest["remotes"].as_array().map(Vec::as_slice).unwrap_or_default() {
        if let (Some(name), Some(url)) = (r["name"].as_str(), r["url"].as_str()) {
            if name == "flathub" || !super::flat::valid_remote(name) {
                continue;
            }
            if super::flat::secure(url, ticked) {
                sorted.add.push((name.to_string(), url.to_string()));
            } else {
                insecure.push(name);
            }
        }
    }
    for x in manifest["flatpaks"].as_array().map(Vec::as_slice).unwrap_or_default() {
        let Some(id) = x["id"].as_str().filter(|id| super::flat::valid_id(id)) else { continue };
        let origin = x["origin"].as_str().unwrap_or("flathub");
        if !super::flat::valid_remote(origin) {
            sorted.refused.push(id.to_string());
            continue;
        }
        if insecure.contains(&origin) {
            sorted.refused.push(t!("store-backup-app-why", id = id, why = super::flat::not_secure(origin)));
        } else {
            sorted.groups.entry(origin.to_string()).or_default().push(id.to_string());
        }
    }
    sorted
}

fn install_groups(mut groups: Vec<(String, Vec<String>)>, refused: Vec<String>, failed: Vec<String>, said: Vec<String>,
                  done: impl FnOnce(Result<(), String>) + 'static) {
    let Some((origin, ids)) = groups.pop() else {
        return if failed.is_empty() && refused.is_empty() { finish(done) } else {
            done(Err(not_installed(&refused, &failed, &said)))
        };
    };
    let args = super::flat::install_argv(&origin, &ids);
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    capture(&crate::common::gentle::argv(&argv), None, move |ok, _, err| {
        let (mut failed, mut said) = (failed, said);
        if !ok {
            failed.extend(ids);
            said.push(super::refusal::last_said(&err));
        }
        install_groups(groups, refused, failed, said, done);
    });
}

fn not_installed(refused: &[String], failed: &[String], said: &[String]) -> String {
    let (failed_list, refused_list) = (failed.join(", "), refused.join(", "));
    let words = if failed.is_empty() {
        t!("store-not-installed-refused", refused = refused_list)
    } else if refused.is_empty() {
        t!("store-not-installed-failed", failed = failed_list)
    } else {
        t!("store-not-installed-both", failed = failed_list, refused = refused_list)
    };
    super::refusal::with_behind(&words, said)
}

fn finish(done: impl FnOnce(Result<(), String>) + 'static) {
    use std::process::{Command, Stdio};
    let _ = Command::new("trindle-wallpaper").stdout(Stdio::null()).stderr(Stdio::null()).status();
    capture(&["trindle-icons", "--quiet"], None, move |_, _, _| done(Ok(())));
}

#[cfg(test)]
pub(super) fn home_part_into(pack: &Path, work: &Path, home: &Path) -> Result<(), String> {
    let listing = put_back::list_of(pack, None).map_err(super::refusal::explained)?;
    home_part_from(pack, &listing, work, home)
}

#[cfg(test)]
mod tests {
    use super::with_face;
    use crate::common::run::{admin_error, NO_PASSWORD};

    #[test]
    fn a_picture_the_backup_holds_and_cannot_give_up_is_said() {
        use super::{face_out, FACE_PART};
        let tar = crate::common::needs::program("tar");
        let top = std::env::temp_dir().join(format!("trindle-backup-face-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        std::fs::create_dir_all(top.join("stage/trindle")).unwrap();
        let pack = |name: &str, picture: Option<&[u8]>| {
            let _ = std::fs::remove_file(top.join("stage").join(FACE_PART));
            if let Some(picture) = picture {
                std::fs::write(top.join("stage").join(FACE_PART), picture).unwrap();
            }
            let pack = top.join(name);
            assert!(std::process::Command::new(&tar).arg("-cf").arg(&pack).arg("-C").arg(top.join("stage")).arg("trindle").status().unwrap().success());
            pack
        };
        assert_eq!(face_out(&pack("none.fmpack", None), None, FACE_PART), Ok(None));
        assert_eq!(face_out(&pack("empty.fmpack", Some(b"")), None, FACE_PART), Ok(None));
        assert_eq!(face_out(&pack("one.fmpack", Some(b"\x89PNG picture")), None, FACE_PART), Ok(Some(b"\x89PNG picture".to_vec())));
        let big = pack("big.fmpack", Some(&vec![7u8; crate::admin::MAX_FACE + 1]));
        let why = face_out(&big, None, FACE_PART).unwrap_err();
        assert!(why.contains(super::put_back::too_big()), "{why}");
        let whole = std::fs::read(&big).unwrap();
        std::fs::write(top.join("cut.fmpack"), &whole[..whole.len() / 2]).unwrap();
        assert!(face_out(&top.join("cut.fmpack"), None, FACE_PART).is_err());
        assert_eq!(with_face(Ok(()), Some(why.clone())), Err(format!("Put back, except your account picture. {why}")));
        let _ = std::fs::remove_dir_all(&top);
    }

    #[test]
    fn apps_not_installed_say_why_and_keep_flatpaks_words_behind() {
        let e = super::not_installed(&[], &["org.example.Notes".into()], &["error: No remote refs found".into()]);
        let (words, behind) = super::super::refusal::apart(&e);
        assert_eq!(words, "Settings and data were put back, but these apps were not installed, because Flatpak refused them: org.example.Notes.");
        assert_eq!(behind, vec!["error: No remote refs found".to_string()]);
        let (words, _) = super::super::refusal::apart(&super::not_installed(&["x (the y remote is not secure)".into()], &[], &[]));
        assert_eq!(words, "Settings and data were put back, but these apps were not installed: x (the y remote is not secure).");
    }

    #[test]
    fn a_picture_that_was_not_put_back_is_said_with_the_rest() {
        assert_eq!(with_face(Ok(()), None), Ok(()));
        assert_eq!(with_face(Err("Nothing was put back, because the backup file could not be read.".into()), None), Err("Nothing was put back, because the backup file could not be read.".to_string()));
        let closed = admin_error("Error executing command as another user: Request dismissed");
        assert_eq!(closed, NO_PASSWORD);
        assert_eq!(with_face(Ok(()), Some(closed.clone())),
                   Err("Put back, except your account picture. Not done: no administrator's password was given.".to_string()));
        assert_eq!(with_face(Err("Settings and data were put back, but these apps were not installed, because Flatpak refused them: org.example.Notes.".into()), Some(closed)),
                   Err("Settings and data were put back, but these apps were not installed, because Flatpak refused them: org.example.Notes. Your account picture was not put back either. \
                        Not done: no administrator's password was given.".to_string()));
    }

    #[test]
    fn a_backups_remote_at_an_address_that_is_not_secure_is_refused_and_said() {
        let manifest = serde_json::json!({
            "remotes": [{"name": "flathub", "url": "https://dl.flathub.org/repo/"},
                        {"name": "good", "url": "https://example.com/repo"},
                        {"name": "bad", "url": "http://example.org/repo"}],
            "flatpaks": [{"id": "org.example.One", "origin": "flathub"}, {"id": "org.example.Two", "origin": "good"},
                         {"id": "org.example.Three", "origin": "bad"}, {"id": "org.example.Four"}],
        });
        let sorted = super::by_remote(&manifest, false);
        assert_eq!(sorted.add, [("good".to_string(), "https://example.com/repo".to_string())]);
        assert_eq!(sorted.refused, ["org.example.Three (the bad remote is not encrypted and was not accepted, so it was not used)"]);
        assert!(!sorted.groups.contains_key("bad"));
        assert_eq!(sorted.groups["flathub"], ["org.example.One", "org.example.Four"]);
        assert_eq!(sorted.groups["good"], ["org.example.Two"]);
    }

    #[test]
    fn an_http_remote_waits_for_the_tick_only_over_the_internet() {
        let manifest = serde_json::json!({
            "remotes": [{"name": "home", "url": "http://192.168.1.5/repo"}, {"name": "far", "url": "http://example.org/repo"},
                        {"name": "safe", "url": "https://example.org/repo"}, {"name": "idle", "url": "http://example.net/repo"}],
            "flatpaks": [{"id": "org.example.One", "origin": "home"}, {"id": "org.example.Two", "origin": "far"},
                         {"id": "org.example.Three", "origin": "safe"}],
        });
        let before = super::by_remote(&manifest, false);
        assert_eq!(before.refused.len(), 1);
        assert!(before.refused[0].starts_with("org.example.Two "));
        assert_eq!(before.groups["home"], ["org.example.One"]);
        let after = super::by_remote(&manifest, true);
        assert!(after.refused.is_empty());
        assert_eq!(after.groups["far"], ["org.example.Two"]);
        assert_eq!(super::addresses(&manifest), ["http://192.168.1.5/repo", "http://example.org/repo", "https://example.org/repo"]);
        assert_eq!(crate::store::link::rule(super::addresses(&manifest)), crate::store::link::Rule::Card);
    }

    #[test]
    fn a_backups_names_that_are_not_names_never_reach_flatpak() {
        let manifest = serde_json::json!({
            "remotes": [{"name": "--no-gpg-verify", "url": "https://example.com/repo"},
                        {"name": "good", "url": "https://example.com/repo"},
                        {"name": "a b", "url": "https://example.org/repo"}],
            "flatpaks": [{"id": "--from=https://example.com/x.flatpakref", "origin": "flathub"},
                         {"id": "org.example.One", "origin": "--system"},
                         {"id": "org.example.Two", "origin": "good"},
                         {"id": "org.example.Three", "origin": "a b"},
                         {"id": "../x", "origin": "good"}, {"id": "org.example.Four"}],
        });
        let sorted = super::by_remote(&manifest, false);
        assert_eq!(sorted.add, [("good".to_string(), "https://example.com/repo".to_string())]);
        let groups: Vec<(&str, &Vec<String>)> = sorted.groups.iter().map(|(remote, ids)| (remote.as_str(), ids)).collect();
        assert_eq!(groups, [("flathub", &vec!["org.example.Four".to_string()]), ("good", &vec!["org.example.Two".to_string()])]);
        assert_eq!(sorted.refused, ["org.example.One", "org.example.Three"]);
    }
}
