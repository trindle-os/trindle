//! Backup and restore of apps, their data and Trindle's settings.
//! Writes Flat's .fmpack; Trindle's part sits under trindle/, which Flat ignores.
//! A backup is a file anyone can write: all of one that is put on the disk
//! goes through `put_back`, which follows no link and leaves the folder it
//! is given by no route.

use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use super::put_back;
use crate::toolkit::t;
use crate::common::files::{home, user};
use crate::common::run::{capture, on_thread};

fn outside_words() -> String {
    super::refusal::nothing(t!("store-because-outside"))
}

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
    capture(&super::progress::english(&["du", "-sh", "--exclude=cache", "--exclude=.cache", &p]), None, move |_, out, _| {
        done(out.split_whitespace().next().map(|s| t!("store-backup-about", size = s)).unwrap_or_default());
    });
}

pub fn backup(parts: Parts, dest: PathBuf, done: impl FnOnce(Result<String, String>) + 'static) {
    on_thread(move || make_pack(parts, &dest).map_err(|e| {
        t!("store-backup-not-made", why = super::refusal::because(&e))
    }), done);
}

fn sh(args: &[&str]) -> Result<String, String> {

    let out = crate::common::run::parsed(crate::common::gentle::program(&mut std::process::Command::new(args[0]))).args(&args[1..]).stdin(std::process::Stdio::null()).output()
        .map_err(|e| e.to_string())?;

    if out.status.success() || (args[0] == "tar" && out.status.code() == Some(1)) {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(super::refusal::last_said(&String::from_utf8_lossy(&out.stderr)))
    }
}

fn private_file(path: &Path) -> std::io::Result<()> {
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
    std::fs::OpenOptions::new().write(true).create(true).truncate(true).mode(0o600).open(path)?;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
}

fn make_pack(parts: Parts, dest: &Path) -> Result<String, String> {
    let stage = crate::common::files::private_temp("trindle-backup").map_err(|e| e.to_string())?;

    let Some(name) = dest.file_name() else { return Err(t!("store-because-no-name").into()) };
    let spare = dest.with_file_name(crate::common::files::spare_name(name, 0));
    let result = private_file(&spare).map_err(|e| e.to_string()).and_then(|()| pack_into(parts, &spare, &stage));
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

#[cfg(test)]
pub(super) fn face_out(pack: &Path, how: Option<&str>, member: &str) -> Face {
    let listing = put_back::list_of(pack, how).map_err(|e| t!("store-refusal-not-read-from", why = e))?;
    face_from(pack, how, member, &listing)
}

pub(super) fn face_from(pack: &Path, how: Option<&str>, member: &str, listing: &str) -> Face {
    let why = |e: String| t!("store-refusal-not-read-from", why = e);
    if !put_back::has(listing, member) {
        return Ok(None);
    }
    let data = put_back::read_out(pack, how, member, crate::admin::MAX_FACE as u64).map_err(why)?;
    Ok(Some(data).filter(|data| !data.is_empty()))
}

pub(super) fn home_part_from(pack: &Path, listing: &str, work: &Path, home: &Path) -> Result<(), String> {
    (|| {
        if !put_back::has(listing, HOME_PART) {
            return Ok(());
        }
        let inner = put_back::taken_out(pack, HOME_PART, work)?;
        let listing = put_back::listed(&inner, "--gzip")?;
        let mut wanted: Vec<&str> = SETTINGS.to_vec();
        wanted.push(APPIMAGES);
        put_back::put_back(&inner, "--gzip", &listing, &wanted, home, put_back::Holds::FilesAndFolders)
    })().map_err(|e| if e == put_back::outside() { outside_words() } else { super::refusal::explained(e) })
}

pub fn rule_of(file: &Path) -> super::link::Rule {
    super::flat::read(file).map_or(super::link::Rule::Nothing, |f| super::link::rule(f.addresses()))
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

fn finish(done: impl FnOnce(Result<(), String>) + 'static) {
    capture(&["trindle-wallpaper"], None, move |_, _, _| {
        capture(&["trindle-icons", "--quiet"], None, move |_, _, _| done(Ok(())));
    });
}

#[cfg(test)]
pub(super) fn home_part_into(pack: &Path, work: &Path, home: &Path) -> Result<(), String> {
    let listing = put_back::list_of(pack, None).map_err(super::refusal::explained)?;
    home_part_from(pack, &listing, work, home)
}

#[cfg(test)]
mod tests {
    use super::with_face;

    #[test]
    fn a_backup_file_is_private_from_the_start() {
        use std::os::unix::fs::PermissionsExt;
        let dir = std::env::temp_dir().join(format!("trindle-backup-mode-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let (new, old) = (dir.join("new.fmpack"), dir.join("old.fmpack"));
        std::fs::write(&old, "x").unwrap();
        std::fs::set_permissions(&old, std::fs::Permissions::from_mode(0o644)).unwrap();
        super::private_file(&new).unwrap();
        super::private_file(&old).unwrap();
        for f in [&new, &old] {
            assert_eq!(std::fs::metadata(f).unwrap().permissions().mode() & 0o777, 0o600, "{}", f.display());
        }
        let _ = std::fs::remove_dir_all(&dir);
    }
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

        assert_eq!(with_face(Ok(()), Some(why.clone())), Err(format!("Everything was put back except your account picture. {why}")));
        let _ = std::fs::remove_dir_all(&top);
    }

    #[test]
    fn a_picture_that_was_not_put_back_is_said_with_the_rest() {
        assert_eq!(with_face(Ok(()), None), Ok(()));
        assert_eq!(with_face(Err("Nothing was put back, because the backup file could not be read.".into()), None), Err("Nothing was put back, because the backup file could not be read.".to_string()));
        let closed = admin_error("Error executing command as another user: Request dismissed");
        assert_eq!(closed, NO_PASSWORD);
        assert_eq!(with_face(Ok(()), Some(closed.clone())),
                   Err("Everything was put back except your account picture. That wasn't done, because no administrator's password was given.".to_string()));
        assert_eq!(with_face(Err("Settings and data were put back, but these apps were not installed, because Flatpak refused them: org.example.Notes.".into()), Some(closed)),
                   Err("Settings and data were put back, but these apps were not installed, because Flatpak refused them: org.example.Notes. Trindle didn't put back your account picture either. \
                        That wasn't done, because no administrator's password was given.".to_string()));
    }
}
