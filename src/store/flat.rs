//! Restore backups (`*.fmpack`) and app lists made by Flat, in Flat's own order.

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::Value;

use super::put_back;
use crate::toolkit::t;
use crate::common::files::home;

const SYSTEM_REPO: &str = "/var/lib/flatpak/repo";

#[derive(Clone, Debug)]
pub struct Settings {
    blob: String,
    sha256: Option<String>,
    has_overrides: bool,
}

#[derive(Clone, Debug)]
pub struct FlatApp {
    pub id: String,
    pub name: String,
    remote: String,
    branch: Option<String>,

    pub settings: Option<Settings>,
}

#[derive(Clone, Debug)]
pub struct FlatFile {
    path: PathBuf,
    pub apps: Vec<FlatApp>,
    remotes: Vec<(String, String)>,
    global_override: bool,

    pub trindle: bool,

    left_out: Vec<String>,
}

pub(super) fn names_not_valid(n: usize) -> String {
    t!("store-flat-skipped", n = n)
}

impl FlatFile {

    pub fn addresses(&self) -> Vec<&str> {
        self.remotes.iter().filter(|r| r.0 != "flathub" && self.apps.iter().any(|a| a.remote == r.0)).map(|r| r.1.as_str()).collect()
    }
    pub fn with_settings(&self) -> usize {
        self.apps.iter().filter(|a| a.settings.is_some()).count()
    }
}

pub fn is_flat(path: &Path) -> bool {
    let name = path.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
    name.ends_with(".fmpack") || name.ends_with(".json")
}

pub(super) fn valid_id(id: &str) -> bool {

    let parts: Vec<&str> = id.split('.').collect();
    id.len() <= 255
        && parts.len() >= 3
        && id.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
        && parts.iter().all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'))
}

pub(super) fn valid_remote(r: &str) -> bool {
    !r.is_empty() && r.len() <= 64
        && r.chars().next().is_some_and(|c| c.is_ascii_alphanumeric())
        && r.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}

fn valid_branch(b: &str) -> bool {
    !b.is_empty() && b.len() <= 64
        && b.chars().next().is_some_and(|c| c.is_ascii_alphanumeric())
        && b.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}

fn valid_url(u: &str) -> bool {
    (u.starts_with("https://") || u.starts_with("http://")) && !u.chars().any(char::is_whitespace)
}

pub(super) fn secure(u: &str, ticked: bool) -> bool {
    super::link::allowed(u, ticked)
}

pub(super) fn not_secure(name: &str) -> String {
    t!("store-flat-not-secure", name = name)
}

fn text(v: &Value) -> Option<&str> {
    v.as_str().map(str::trim).filter(|s| !s.is_empty())
}

const LIST_MOST: u64 = crate::common::files::SMALL_MOST;

const NAME_MOST: usize = 120;

pub fn read(path: &Path) -> Result<FlatFile, String> {
    let is_pack = path.to_string_lossy().to_lowercase().ends_with(".fmpack");
    let json = read_json(path, is_pack)?;
    if json["format_version"].as_u64() != Some(1) {
        return Err(super::refusal::nothing(t!("store-because-newer-flat")));
    }

    let remotes: Vec<(String, String)> = json["remotes"].as_array().map(|a| a.iter().filter_map(|r| {
        let (n, u) = (text(&r["name"])?, text(&r["url"])?);
        (valid_remote(n) && valid_url(u)).then(|| (n.to_string(), u.to_string()))
    }).collect()).unwrap_or_default();

    let mut left = Left::default();
    let packed = if is_pack { packed_apps(&json, &mut left) } else { Vec::new() };
    let apps = listed_apps(&json, is_pack, packed, &mut left);
    if apps.is_empty() {
        return Err(super::refusal::nothing(t!("store-because-no-apps")));
    }
    let Left { bad_names, mut left_out } = left;
    if bad_names > 0 {
        left_out.insert(0, names_not_valid(bad_names));
    }
    Ok(FlatFile {
        left_out,
        path: path.to_path_buf(),
        apps,
        remotes,
        global_override: is_pack && json["has_global_override"].as_bool() == Some(true),
        trindle: is_pack && (json["trindle"]["home"].as_bool() == Some(true) || json["trindle"]["face"].as_bool() == Some(true)),
    })
}

fn read_json(path: &Path, is_pack: bool) -> Result<Value, String> {
    if is_pack {
        let list = match put_back::read_out(path, None, "manifest.json", LIST_MOST) {
            Ok(list) => list,
            Err(e) if e == put_back::too_big() => return Err(super::refusal::nothing(t!("store-because-damaged-list"))),
            Err(_) => return Err(super::refusal::nothing(t!("store-because-not-flat"))),
        };
        serde_json::from_slice(&list).map_err(|_| super::refusal::nothing(t!("store-because-damaged-list")))
    } else {
        use std::io::Read;
        let mut list = Vec::new();
        crate::common::files::open_file(path).and_then(|file| file.take(LIST_MOST + 1).read_to_end(&mut list)).map_err(|e| super::refusal::nothing(&t!("store-because-not-read", why = super::refusal::because(&e.to_string()))))?;
        Some(list).filter(|list| list.len() as u64 <= LIST_MOST).and_then(|list| serde_json::from_slice(&list).ok())
            .ok_or_else(|| super::refusal::nothing(t!("store-because-no-flat-list")))
    }
}

#[derive(Default)]
struct Left {
    bad_names: usize,
    left_out: Vec<String>,
}

type Packed = (String, String, String, Option<String>, Settings);

fn packed_apps(json: &Value, left: &mut Left) -> Vec<Packed> {
    let mut packed: Vec<Packed> = Vec::new();
    for a in json["apps"].as_array().cloned().unwrap_or_default() {
        let Some(id) = text(&a["id"]).filter(|i| valid_id(i)) else {

            if json["list"].as_array().is_none() {
                left.bad_names += 1;
            }
            continue;
        };
        let origin = text(&a["origin"]).unwrap_or("flathub");
        let branch = text(&a["branch"]);
        let blob = text(&a["blob"]).unwrap_or("");
        let sha = text(&a["sha256"]);
        let ok = valid_remote(origin)
            && branch.is_none_or(valid_branch)
            && (blob == format!("apps/{id}.tar.zst") || blob == format!("apps/{id}.tar.gz"))
            && sha.is_none_or(|s| s.len() == 64 && s.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
        if !ok {
            left.left_out.push(t!("store-flat-settings-checks", name = text(&a["name"]).unwrap_or(id).chars().take(NAME_MOST).collect::<String>()));
            continue;
        }
        let name = text(&a["name"]).unwrap_or(id).chars().take(NAME_MOST).collect();
        packed.push((id.to_string(), name, origin.to_string(), branch.map(String::from), Settings {
            blob: blob.to_string(),
            sha256: sha.map(String::from),
            has_overrides: a["has_overrides"].as_bool() == Some(true),
        }));
    }
    packed
}

fn listed_apps(json: &Value, is_pack: bool, packed: Vec<Packed>, left: &mut Left) -> Vec<FlatApp> {
    let list = if is_pack { json["list"].as_array().cloned() } else { json["apps"].as_array().cloned() };
    let mut apps: Vec<FlatApp> = Vec::new();
    match list {
        Some(entries) => {
            for e in entries {
                let Some(id) = text(&e["id"]).filter(|i| valid_id(i)) else {
                    left.bad_names += 1;
                    continue;
                };
                if apps.iter().any(|a| a.id == id) {
                    continue;
                }

                let named = text(&e["remote"]);
                if named.is_some_and(|r| !valid_remote(r)) {
                    left.left_out.push(t!("store-flat-source-name", name = text(&e["name"]).unwrap_or(id).chars().take(NAME_MOST).collect::<String>()));
                    continue;
                }
                let remote = named.unwrap_or("flathub");
                let pack = packed.iter().find(|p| p.0 == id);

                let wanted = pack.is_some() && (e.get("keep").is_none() || e["keep"].as_bool() == Some(true));
                apps.push(FlatApp {
                    id: id.to_string(),
                    name: text(&e["name"]).unwrap_or(id).chars().take(NAME_MOST).collect(),
                    remote: pack.map(|p| p.2.clone()).unwrap_or_else(|| remote.to_string()),
                    branch: pack.and_then(|p| p.3.clone()),
                    settings: if wanted { pack.map(|p| p.4.clone()) } else { None },
                });
            }
        }
        None => {
            for (id, name, origin, branch, s) in packed {
                apps.push(FlatApp { id, name, remote: origin, branch, settings: Some(s) });
            }
        }
    }
    apps
}

#[derive(Default, Debug)]
pub struct Outcome {
    pub installed: usize,
    pub already: usize,
    pub settings: usize,
    pub failed: Vec<String>,

    pub behind: Vec<String>,
}

impl Outcome {

    fn fail(&mut self, what: &str, why: &str) {
        let (words, behind) = super::refusal::apart(why);
        self.failed.push(t!("store-flat-failed-because", what = what, why = super::refusal::because(&words)));
        self.behind.extend(behind);
    }
}

fn run(args: &[&str]) -> Result<String, String> {
    ended(Command::new(args[0]).args(&args[1..]))
}

fn run_gently(args: &[&str]) -> Result<String, String> {
    ended(crate::common::gentle::program(&mut Command::new(args[0])).args(&args[1..]))
}

fn ended(program: &mut Command) -> Result<String, String> {
    let out = program.stdin(Stdio::null()).output().map_err(|e| e.to_string())?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        let err = String::from_utf8_lossy(&out.stderr);
        Err(err.trim().lines().last().unwrap_or("failed").to_string())
    }
}

fn remote_names(scope: &str) -> Vec<(String, String)> {
    run(&["flatpak", "remotes", scope, "--columns=name,url"]).unwrap_or_default().lines().filter_map(|l| {
        let mut c = l.split('\t');
        Some((c.next()?.trim().to_string(), c.next().unwrap_or("").trim().to_string()))
    }).collect()
}

fn address_for(name: &str, recorded: Option<&str>, system: &[(String, String)], ticked: bool) -> Result<String, String> {
    let url = recorded.map(String::from).or_else(|| system.iter().find(|r| r.0 == name).map(|r| r.1.clone()))
        .filter(|u| valid_url(u))
        .ok_or_else(|| t!("store-flat-no-address", name = name))?;
    if !secure(&url, ticked) {
        return Err(not_secure(name));
    }
    Ok(url)
}

fn user_remote(name: &str, recorded: Option<&str>, ticked: bool) -> Result<String, String> {
    let user = remote_names("--user");
    if user.iter().any(|r| r.0 == name) {
        return Ok(name.to_string());
    }
    let system = remote_names("--system");
    let local = if system.iter().any(|r| r.0 == name) { format!("{name}-user") } else { name.to_string() };
    if user.iter().any(|r| r.0 == local) {
        return Ok(local);
    }
    if name == "flathub" {
        let add = super::catalogue::add_flathub(&local);
        run(&add.iter().map(String::as_str).collect::<Vec<_>>())?;
        return Ok(local);
    }
    let url = address_for(name, recorded, &system, ticked)?;

    let key = format!("{SYSTEM_REPO}/{name}.trustedkeys.gpg");
    let key = Some(key).filter(|k| Path::new(k).is_file());
    let args = remote_add_argv(&local, &url, key.as_deref());
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    run(&argv)?;
    Ok(local)
}

pub(super) fn remote_add_argv(name: &str, url: &str, key: Option<&str>) -> Vec<String> {
    let mut args = vec!["flatpak".to_string(), "remote-add".into(), "--if-not-exists".into(), "--user".into()];
    if let Some(key) = key {
        args.push(format!("--gpg-import={key}"));
    }

    args.push("--".into());
    args.push(name.to_string());
    args.push(url.to_string());
    args
}

pub(super) fn install_argv(remote: &str, what: &[String]) -> Vec<String> {

    let mut args = vec!["flatpak".to_string(), "install".into(), "--user".into(), "--noninteractive".into(), "-y".into(), "--".into(),
                        remote.to_string()];
    args.extend(what.iter().cloned());
    args
}

fn installed(id: &str) -> bool {
    Command::new("flatpak").args(["info", id]).stdout(Stdio::null()).stderr(Stdio::null())
        .status().is_ok_and(|s| s.success())
}

fn install(app: &FlatApp, remote: &str) -> Result<(), String> {
    let from = |what: String| {
        let args = install_argv(remote, &[what]);
        run_gently(&args.iter().map(String::as_str).collect::<Vec<_>>()).map(|_| ())
    };
    if let Some(b) = &app.branch {
        if from(format!("{}//{b}", app.id)).is_ok() {
            return Ok(());
        }
    }
    from(app.id.clone())
}

fn running() -> Vec<String> {
    run(&["flatpak", "ps", "--columns=application"]).unwrap_or_default().lines().map(|l| l.trim().to_string()).collect()
}

fn stays_inside(listing: &str, id: &str) -> bool {
    listing.lines().filter(|l| !l.is_empty()).all(|l| {
        let l = l.trim_end_matches('/');
        (l == id || l.starts_with(&format!("{id}/"))) && !l.split('/').any(|p| p == "..")
    })
}

fn put_settings_back(file: &FlatFile, app: &FlatApp, s: &Settings, work: &Path) -> Result<(), String> {
    settings_into(file, app, s, work, &home().join(".var/app"))?;
    if s.has_overrides {
        overrides_back(&file.path, &format!("overrides/app/{}", app.id), &overrides_home().join(&app.id))?;
    }
    Ok(())
}

pub(super) fn settings_into(file: &FlatFile, app: &FlatApp, s: &Settings, work: &Path, var_app: &Path) -> Result<(), String> {

    let blob = put_back::taken_out(&file.path, &s.blob, work)?;
    let blob_s = blob.to_string_lossy().to_string();
    let result = (|| {
        if let Some(want) = &s.sha256 {
            let got = run_gently(&["sha256sum", &blob_s])?;
            if got.split_whitespace().next() != Some(want.as_str()) {
                return Err(t!("store-because-checksum").to_string());
            }
        }
        let how = if s.blob.ends_with(".zst") { "--zstd" } else { "--gzip" };
        let listing = put_back::listed(&blob, how)?;
        if !stays_inside(&listing, &app.id) {
            return Err(put_back::outside().to_string());
        }
        if !crate::common::files::is_one_name(app.id.as_bytes()) {
            return Err(put_back::outside().to_string());
        }
        std::fs::create_dir_all(var_app).map_err(|e| e.to_string())?;

        let own = var_app.join(&app.id);
        let aside = var_app.join(crate::common::files::spare_name(std::ffi::OsStr::new(&app.id), 0));
        let kept = std::fs::rename(&own, &aside).is_ok();
        let put = put_back::put_back(&blob, how, &listing, &[&app.id], var_app, put_back::Holds::Anything);
        if kept {
            if put.is_ok() {
                let _ = std::fs::remove_dir_all(&aside);
            } else {
                let _ = std::fs::remove_dir_all(&own);
                if let Err(e) = crate::common::files::rename_new(&aside, &own) {
                    return Err(t!("store-flat-kept-as", why = put.err().unwrap_or_default(), path = aside.display().to_string(), error = e.to_string()));
                }
            }
        }
        put
    })();
    let _ = std::fs::remove_file(&blob);
    result
}

fn overrides_home() -> PathBuf {
    home().join(".local/share/flatpak/overrides")
}

fn overrides_back(pack: &Path, member: &str, to: &Path) -> Result<(), String> {
    let text = put_back::read_out(pack, None, member, LIST_MOST)?;
    let text = String::from_utf8_lossy(&text);
    if let Some(dir) = to.parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    crate::common::files::write_text(to, &text).map_err(|e| e.to_string())
}

pub fn restore(file: &FlatFile, ticked: bool, say: impl Fn(String, Option<(usize, usize)>)) -> Outcome {
    let mut out = Outcome::default();
    out.failed.extend(file.left_out.iter().cloned());
    let Ok(work) = crate::common::files::private_temp("trindle-flat") else {
        out.failed.push(t!("store-flat-nowhere").to_string());
        return out;
    };

    if file.global_override {
        if let Err(e) = overrides_back(&file.path, "overrides/global", &overrides_home().join("global")) {
            out.fail(t!("store-flat-every-app"), &e);
        }
    }

    let open = running();
    let total = file.apps.len();
    for (i, app) in file.apps.iter().enumerate() {
        let at = i + 1;
        if installed(&app.id) {

            out.already += 1;
            continue;
        }
        say(t!("store-flat-step-installing", name = &app.name, at = at, total = total), Some((at, total)));
        let recorded = file.remotes.iter().find(|r| r.0 == app.remote).map(|r| r.1.as_str());
        let remote = match user_remote(&app.remote, recorded, ticked) {
            Ok(r) => r,
            Err(e) => {
                out.fail(&app.name, &e);
                continue;
            }
        };
        if let Err(e) = install(app, &remote) {
            out.failed.push(t!("store-flat-flatpak-refused", name = &app.name, why = super::refusal::last_said(&e)));
            continue;
        }
        out.installed += 1;
        let Some(s) = &app.settings else { continue };
        if open.iter().any(|o| o == &app.id) {
            let _ = run(&["flatpak", "kill", &app.id]);
            std::thread::sleep(std::time::Duration::from_millis(1500));
        }
        say(t!("store-flat-step-settings", name = &app.name, at = at, total = total), Some((at, total)));
        match put_settings_back(file, app, s, &work) {
            Ok(()) => out.settings += 1,
            Err(e) => out.fail(&t!("store-flat-app-settings", name = &app.name), &e),
        }
    }
    let _ = std::fs::remove_dir_all(&work);
    out
}

pub fn name() -> &'static str {
    t!("store-flat-name")
}

pub fn summary(out: &Outcome) -> String {

    let mut sentences = vec![t!("store-flat-summary-installed", n = out.installed)];
    if out.settings > 0 {
        sentences.push(t!("store-flat-summary-settings", n = out.settings));
    }
    if out.already > 0 {
        sentences.push(t!("store-flat-summary-already", n = out.already));
    }
    let mut line = sentences.join(" ");
    if !out.failed.is_empty() {
        let list = out.failed.join(", ");
        let list = if list.ends_with('.') { list } else { format!("{list}.") };
        line.push(' ');
        line.push_str(&t!("store-flat-summary-failed", list = list));
    }
    super::refusal::with_behind(&line, &out.behind)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_remotes_and_folders_are_checked() {
        assert!(valid_id("com.brave.Browser"));
        assert!(!valid_id("-rf.brave.Browser"));
        assert!(!valid_id("brave.Browser"));
        assert!(!valid_remote("../x"));
        assert!(valid_branch("stable"));
        assert!(!valid_branch("a/b"));
        assert!(stays_inside("com.a.B/\ncom.a.B/config/x\n", "com.a.B"));
        assert!(!stays_inside("com.a.B/\ncom.a.B/../../.bashrc\n", "com.a.B"));
        assert!(!stays_inside("com.a.Bx/y\n", "com.a.B"));
    }

    #[test]
    fn nothing_from_a_backup_is_read_by_flatpak_as_an_option() {
        let after_the_end = |args: &[String], word: &str| {
            let end = args.iter().position(|a| a == "--").unwrap_or_else(|| panic!("no -- in {args:?}"));
            args.iter().position(|a| a == word).is_some_and(|at| at > end)
        };
        let args = install_argv("--system", &["--from=https://example.com/x.flatpakref".to_string(), "org.example.One//stable".to_string()]);
        for word in ["--system", "--from=https://example.com/x.flatpakref", "org.example.One//stable"] {
            assert!(after_the_end(&args, word), "{word} in {args:?}");
        }
        let args = remote_add_argv("--no-gpg-verify", "https://example.com/repo", Some("/var/lib/flatpak/repo/example.trustedkeys.gpg"));
        for word in ["--no-gpg-verify", "https://example.com/repo"] {
            assert!(after_the_end(&args, word), "{word} in {args:?}");
        }
        assert!(!after_the_end(&args, "--gpg-import=/var/lib/flatpak/repo/example.trustedkeys.gpg"), "Trindle's own option is still one");

        for (file, text) in [("store/flat.rs", crate::common::source::code(include_str!("flat.rs"))), ("store/backup.rs", crate::common::source::code(include_str!("backup.rs")))] {
            let code = text.split_once("#[cfg(test)]").unwrap().0;
            for word in [["\"inst", "all\""].concat(), ["\"remote", "-add\""].concat()] {
                let made_in = |name: &str| code.split_once(name).is_some_and(|(_, rest)| rest.split_once("\n}\n").unwrap().0.contains(&word));
                let by_hand = code.matches(&word).count() - usize::from(made_in("fn install_argv(")) - usize::from(made_in("fn remote_add_argv("));
                assert_eq!(by_hand, 0, "{file} puts {word} together by hand, not through install_argv or remote_add_argv");
            }
        }
    }

    #[test]
    fn what_apps_are_allowed_that_cannot_be_put_back_is_a_failure() {
        let tar = crate::common::needs::program("tar");
        let top = std::env::temp_dir().join(format!("trindle-flat-overrides-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        std::fs::create_dir_all(top.join("stage/overrides/app")).unwrap();
        std::fs::write(top.join("stage/overrides/global"), "[Context]\nfilesystems=!home;\n").unwrap();
        let pack = top.join("backup.fmpack");
        assert!(Command::new(&tar).arg("-cf").arg(&pack).arg("-C").arg(top.join("stage")).arg("overrides").status().unwrap().success());

        let to = top.join("home/overrides/global");
        assert_eq!(overrides_back(&pack, "overrides/global", &to), Ok(()));
        assert_eq!(std::fs::read_to_string(&to).unwrap(), "[Context]\nfilesystems=!home;\n");

        assert!(overrides_back(&pack, "overrides/app/org.example.One", &top.join("home/overrides/org.example.One")).is_err());
        assert!(!top.join("home/overrides/org.example.One").exists());

        std::fs::write(top.join("blocked"), "").unwrap();
        assert!(overrides_back(&pack, "overrides/global", &top.join("blocked/overrides/global")).is_err());

        let code = crate::common::source::code(include_str!("flat.rs")).split_once("#[cfg(test)]").unwrap().0;
        let restore = code.split_once("pub fn restore(").unwrap().1.split_once("\n}\n").unwrap().0;
        assert!(restore.contains("if let Err(e) = overrides_back(") && restore.contains("out.failed.push("));
        let _ = std::fs::remove_dir_all(&top);
    }

    #[test]
    fn only_long_disk_work_is_started_gentle() {
        let nice = crate::common::needs::program("nice");
        let own = String::from_utf8_lossy(&Command::new(&nice).output().unwrap().stdout).trim().to_string();
        let nice = nice.to_string_lossy();
        assert_eq!(run(&[&nice]).unwrap().trim(), own, "a short question was started gentle");
        assert_eq!(run_gently(&[&nice]).unwrap().trim(), "19", "long disk work was not");

        let code = crate::common::source::code(include_str!("flat.rs")).split_once("#[cfg(test)]").unwrap().0;
        assert_eq!(code.matches("run_gently(&").count(), 2);
        assert!(code.contains("run_gently(&[\"sha256sum\","));
        assert!(code.contains("run_gently(&args.iter()"), "an install is not started gentle");
    }

    #[test]
    fn what_apps_are_allowed_is_not_read_whole_when_it_is_too_long() {
        let tar = crate::common::needs::program("tar");
        let top = std::env::temp_dir().join(format!("trindle-flat-long-overrides-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        std::fs::create_dir_all(top.join("stage/overrides")).unwrap();
        std::fs::write(top.join("stage/overrides/global"), vec![b'#'; LIST_MOST as usize + 1]).unwrap();
        let pack = top.join("backup.fmpack");
        assert!(Command::new(&tar).arg("-cf").arg(&pack).arg("-C").arg(top.join("stage")).arg("overrides").status().unwrap().success());
        assert_eq!(overrides_back(&pack, "overrides/global", &top.join("home/global")), Err(put_back::too_big().to_string()));
        assert!(!top.join("home/global").exists());
        let _ = std::fs::remove_dir_all(&top);
    }

    #[test]
    fn a_list_too_long_to_be_one_is_not_read() {
        let tar = crate::common::needs::program("tar");
        let dir = std::env::temp_dir().join(format!("trindle-flat-long-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("stage")).unwrap();
        let list = |pad: usize| format!(r#"{{"format_version":1,{}"apps":[{{"id":"org.example.One","name":"One","remote":"flathub"}}],"list":[{{"id":"org.example.One"}}]}}"#, " ".repeat(pad));
        let most = crate::common::files::SMALL_MOST as usize;
        for (pad, read_whole) in [(0, true), (most, false)] {
            let f = dir.join("flat-apps.json");
            std::fs::write(&f, list(pad)).unwrap();
            assert_eq!(read(&f).is_ok(), read_whole, "a list of apps {pad} bytes longer");
            std::fs::write(dir.join("stage/manifest.json"), list(pad)).unwrap();
            let pack = dir.join("backup.fmpack");
            assert!(Command::new(&tar).arg("-cf").arg(&pack).arg("-C").arg(dir.join("stage")).arg("manifest.json").status().unwrap().success());
            assert_eq!(read(&pack).is_ok(), read_whole, "a backup's list {pad} bytes longer");
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_flat_app_list_is_read() {
        let dir = std::env::temp_dir().join(format!("trindle-flat-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("flat-apps.json");
        std::fs::write(&f, r#"{"format_version":1,"apps":[{"id":"com.brave.Browser","name":"Brave","remote":"flathub","keep":true},{"id":"bad"}]}"#).unwrap();
        let read = read(&f).unwrap();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(read.apps.len(), 1);
        assert_eq!(read.apps[0].name, "Brave");
        assert!(read.apps[0].settings.is_none());
    }

    #[test]
    fn what_a_backup_holds_and_is_not_used_is_said() {
        let dir = std::env::temp_dir().join(format!("trindle-flat-left-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let f = dir.join("flat-apps.json");
        std::fs::write(&f, r#"{"format_version":1,"apps":[
            {"id":"com.brave.Browser","name":"Brave"},
            {"id":"bad"},{"id":"-rf.x.Y"},
            {"id":"org.example.Notes","name":"Notes","remote":"../evil"}]}"#).unwrap();
        let read = read(&f).unwrap();
        let _ = std::fs::remove_dir_all(&dir);
        assert_eq!(read.apps.iter().map(|a| a.id.as_str()).collect::<Vec<_>>(), ["com.brave.Browser"], "no app falls back to Flathub");

        let two = ["2 apps skip", "ped: their names were not valid."].concat();
        assert_eq!(read.left_out, [two.as_str(), "Notes (because the name of its source is not valid)"]);
        assert_eq!(names_not_valid(1), ["1 app skip", "ped: its name was not valid."].concat());

        let source = crate::common::source::code(include_str!("flat.rs"));
        let restore = source.split_once("pub fn restore(").unwrap().1.split_once("\n}\n").unwrap().0;
        assert!(restore.contains("out.failed.extend(file.left_out.iter().cloned())"));
    }

    #[test]
    fn a_restore_summary_is_whole_sentences() {
        let two = ["2 apps skip", "ped: their names were not valid."].concat();
        let out = Outcome { installed: 3, settings: 2, already: 1, failed: vec![two.clone()], behind: Vec::new() };
        let said = super::summary(&out);
        assert_eq!(said, format!("Installed 3 apps. Settings put back for 2. 1 already here, left as it was. Not put back: {two}"));
        assert!(!said.contains("; "));
    }

    #[test]
    fn an_http_remote_over_the_internet_is_added_only_after_the_tick() {
        let system = vec![("fromsystem".to_string(), "http://example.org/repo".to_string())];
        assert_eq!(address_for("example", Some("https://example.com/repo"), &system, false).as_deref(), Ok("https://example.com/repo"));

        for (name, recorded) in [("example", Some("http://example.com/repo")), ("fromsystem", None)] {
            assert_eq!(address_for(name, recorded, &system, false), Err(not_secure(name)), "{name}");
            assert!(address_for(name, recorded, &system, true).is_ok(), "{name} ticked");
        }

        assert_eq!(address_for("home", Some("http://192.168.1.4/repo"), &system, false).as_deref(), Ok("http://192.168.1.4/repo"));
        assert_eq!(not_secure("example"), "the example remote is not encrypted and was not accepted, so it was not used");

        assert!(address_for("nowhere", None, &system, false).unwrap_err().contains("does not say where it is"));
        assert!(address_for("odd", Some("ftp://example.com/repo"), &system, false).unwrap_err().contains("does not say where it is"));
    }
}
