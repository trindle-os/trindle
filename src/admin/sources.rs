//! Software source commands run as root; see common/sources.rs.

use std::path::{Path, PathBuf};

use super::accounts::regular_user;
use super::{and_undone, at, gone, replace, run, Outcome};
use crate::common::sources::{self, Known, KEYRINGS, LISTS};
use crate::toolkit::t;

const MAX_KEY: usize = 1_000_000;

fn step(n: usize, of: usize, words: &str) {
    println!("{}", crate::common::filling::step_line(n, of, words));
}

fn get_key(url: &str) -> Result<(Vec<u8>, bool), String> {
    let out = std::process::Command::new("curl")
        .args(["-fsSL", "--proto", "=https", "--proto-redir", "=https", "--max-time", "60", "--max-filesize", &MAX_KEY.to_string(), url])
        .output().map_err(|e| e.to_string())?;
    if !out.status.success() || out.stdout.is_empty() {
        return Err(t!("admin-key-fetch", url = url));
    }
    let data = out.stdout;
    let armoured = data.starts_with(b"-----BEGIN PGP PUBLIC KEY BLOCK-----");
    if !armoured && data.first().is_none_or(|b| b & 0x80 == 0) {
        return Err(t!("admin-key-invalid").into());
    }
    Ok((data, armoured))
}

fn fetch_key(id: &str, url: &str) -> Result<PathBuf, String> {
    let (data, armoured) = get_key(url)?;
    std::fs::create_dir_all(at(KEYRINGS)).map_err(|e| e.to_string())?;
    let path = at(KEYRINGS).join(format!("trindle-{id}.{}", if armoured { "asc" } else { "gpg" }));
    replace(&path, &data, 0o644)?;
    Ok(path)
}

fn write_source(id: &str, uri: &str, suite: &str, components: &str, key: &Path) -> Outcome {
    let text = format!("# Added by Trindle, Apps, Software sources. Removed there too.\n\
                        Types: deb\nURIs: {uri}\nSuites: {suite}\nComponents: {components}\n\
                        Architectures: {}\nSigned-By: {}\n", sources::arch(), key.display());
    replace(&at(sources::file_for(id)), text.as_bytes(), 0o644)
}

fn forget(id: &str) -> Outcome {
    let mut done = gone(&at(sources::file_for(id)));
    for ext in ["asc", "gpg"] {
        done = done.and(gone(&at(KEYRINGS).join(format!("trindle-{id}.{ext}"))));
    }
    done
}

struct Was(Vec<(PathBuf, Option<Vec<u8>>)>);

impl Was {

    fn of(id: &str) -> Result<Was, String> {
        let mut files = vec![at(sources::file_for(id))];
        files.extend(["asc", "gpg"].map(|ext| at(KEYRINGS).join(format!("trindle-{id}.{ext}"))));
        files.into_iter().map(|file| match std::fs::read(&file) {
            Ok(held) => Ok((file, Some(held))),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok((file, None)),
            Err(e) => Err(t!("admin-source-file-unreadable", path = file.display().to_string(), why = e.to_string())),
        }).collect::<Result<_, _>>().map(Was)
    }

    #[allow(clippy::manual_try_fold)]
    fn put_back(&self) -> Outcome {
        self.0.iter().fold(Ok(()), |done, (file, held)| done.and(match held {
            Some(held) => replace(file, held, 0o644).map_err(|e| t!("admin-file-not-put-back", path = file.display().to_string(), why = e.to_string())),
            None => gone(file),
        }))
    }

    fn held_any(&self) -> bool {
        self.0.iter().any(|(_, held)| held.is_some())
    }
}

fn add_source(id: &str, uri: &str, suite: &str, components: &str, key_url: &str, of: usize) -> Outcome {
    let was = Was::of(id)?;
    step(1, of, t!("admin-step-fetching-key"));
    let key = fetch_key(id, key_url)?;
    step(2, of, t!("admin-step-adding-source"));
    use_source(id, uri, suite, components, &key, &was)
}

fn use_source(id: &str, uri: &str, suite: &str, components: &str, key: &Path, was: &Was) -> Outcome {
    let done = write_source(id, uri, suite, components, key)
        .and_then(|_| run(&["apt-get", "-q", "update"], None));
    if let Err(e) = done {
        let taken = was.put_back();
        // The lists are only read again without it; what failed is said.
        let _ = run(&["apt-get", "-q", "update"], None);
        return Err(match taken {
            Ok(()) if was.held_any() => t!("admin-source-unusable", why = &e),
            Ok(()) => t!("admin-source-unusable-removed", why = &e),
            Err(left) => and_undone(t!("admin-source-unusable", why = &e), Err(left)),
        });
    }
    Ok(())
}

fn install(packages: &[&str]) -> Outcome {
    super::updates::no_questions();
    let mut argv = vec!["apt-get", "-q", "-y", "-o", "Dpkg::Options::=--force-confdef", "-o",
                        "Dpkg::Options::=--force-confold", "install"];
    argv.extend(packages);
    run(&argv, None)
}

fn after(k: &Known, user: &str, started: bool) -> Outcome {
    let mut removed = false;
    for f in k.vendor_files {
        let p = at(LISTS).join(f);

        if p.exists() && std::fs::remove_file(&p).is_ok() {
            removed = true;
        }
    }
    if removed {
        // The lists are read again at the next look for updates.
        let _ = run(&["apt-get", "-q", "update"], None);
    }
    let set_up = match k.id {

        "tailscale" if !started => Ok(()),
        "tailscale" => super::tailscale::tailscale_set(&[format!("--operator={user}").as_str()])
            .map_err(|e| t!("admin-source-tailscale-operator", user = user, why = e)),
        "syncthing" => run(&["systemctl", "enable", "--now", &format!("syncthing@{user}.service")], None)
            .map_err(|e| t!("admin-source-syncthing", user = user, why = e)),

        "claude" => run(&["usermod", "-aG", "kvm", user], None)
            .map_err(|e| t!("admin-source-claude-kvm", user = user, why = e)),
        "docker" => run(&["usermod", "-aG", k.id, user], None)
            .map_err(|e| t!("admin-source-docker", user = user, why = e)),
        _ => Ok(()),
    };
    set_up.map_err(|e| t!("admin-source-set-up-failed", name = k.name, why = e))
}

fn before(k: &Known) {
    if k.id == "claude" {
        // Should it add one all the same, `after` takes it away again.
        let _ = replace(Path::new("/etc/default/claude-desktop"), b"CLAUDE_DESKTOP_ADD_REPO=\"false\"\n", 0o644);
    }
}

pub fn add(args: &[String]) -> Outcome {
    let [id, user] = args else { return Err("usage: trindle-admin source-add ID USER".into()) };
    add_for(id, user, true)
}

pub fn add_unstarted(id: &str, user: &str) -> Outcome {
    add_for(id, user, false)
}

fn add_for(id: &str, user: &str, started: bool) -> Outcome {
    let k = sources::known(id).ok_or(t!("admin-source-unknown"))?;
    regular_user(user)?;
    let arch = sources::arch();
    if !k.archs.contains(&arch.as_str()) {
        return Err(t!("admin-source-arch", name = k.name, arch = &arch));
    }
    before(k);
    add_source(k.id, k.uri, k.suite, k.components, k.key, 4)?;
    step(3, 4, t!("admin-step-installing-it"));
    install(k.packages)?;
    step(4, 4, t!("admin-step-setting-up"));
    after(k, user, started)
}

pub fn custom(args: &[String]) -> Outcome {
    let (uri, suite, components, key, package) = match args {
        [u, s, c, k] => (u, s, c, k, None),
        [u, s, c, k, p] => (u, s, c, k, Some(p)),
        _ => return Err("usage: trindle-admin source-custom URI SUITE COMPONENTS KEY-URI [PACKAGE]".into()),
    };
    if !sources::valid_uri(uri) || !sources::valid_uri(key) {
        return Err(t!("admin-source-address").into());
    }
    if !sources::valid_words(suite) || !sources::valid_words(components) {
        return Err(t!("admin-source-words").into());
    }
    if package.is_some_and(|p| !sources::valid_package(p)) {
        return Err(t!("admin-source-package").into());
    }
    let id = sources::custom_id(uri, suite);
    let of = if package.is_some() { 3 } else { 2 };
    add_source(&id, uri, suite.trim(), components.trim(), key, of)?;
    if let Some(p) = package {
        step(3, 3, t!("admin-step-installing-it"));
        install(&[p.as_str()])?;
    }
    Ok(())
}

pub fn remove(args: &[String]) -> Outcome {
    let [id] = args else { return Err("usage: trindle-admin source-remove ID".into()) };
    step(1, 2, t!("admin-step-removing"));
    if let Some(k) = sources::known(id) {
        if k.id == "syncthing" {
            if let Ok(out) = std::process::Command::new("systemctl")
                .args(["list-units", "--plain", "--no-legend", "syncthing@*"]).output()
            {
                for unit in String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| l.split_whitespace().next()) {
                    // The program is purged next, and that is what is said.
                    let _ = run(&["systemctl", "disable", "--now", unit], None);
                }
            }
        }

        super::updates::no_questions();
        let installed: Vec<&str> = k.packages.iter().copied()
            .filter(|p| run(&["dpkg-query", "-W", "-f=${Status}", p], None).is_ok())
            .collect();
        if !installed.is_empty() {
            let mut argv = vec!["apt-get", "-q", "-y", "purge"];
            argv.extend(installed);
            run(&argv, None)?;
            let mut tidy = super::protected::autoremove_argv(false);
            tidy.insert(tidy.len() - 1, "--purge".into());
            let tidy: Vec<&str> = tidy.iter().map(String::as_str).collect();
            // What the program brought in with it goes at the next update's tidying.
            let _ = run(&tidy, None);
        }
        if k.id == "claude" {
            // A line of settings nothing reads once the program has gone.
            let _ = std::fs::remove_file("/etc/default/claude-desktop");
        }
    } else if !(id.starts_with("custom-") && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')) {
        return Err(t!("admin-source-not-ours").into());
    }
    forget(id).map_err(|e| t!("admin-source-still-there", why = e))?;
    step(2, 2, t!("admin-step-reading-lists"));
    run(&["apt-get", "-q", "update"], None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::admin::stand_in::StandIn;

    const ID: &str = "custom-abc";

    #[test]
    fn a_source_that_cannot_be_taken_away_is_not_called_removed() {
        let s = StandIn::all_well("source-remove");
        s.stuck(&sources::file_for(ID).to_string_lossy());
        let said = remove(&[ID.to_string()]).unwrap_err();
        assert!(said.starts_with("The source is still there"), "{said}");
        assert!(s.ran().is_empty(), "the lists were read again as though it had gone");
    }

    #[test]
    fn a_source_neither_used_nor_taken_away_is_said_as_both() {
        let s = StandIn::failing("source-add", "apt-get -q update", "E: The repository is not signed.");
        s.write(&format!("{LISTS}/other.sources"), b"");
        let was = Was::of(ID).unwrap();
        s.stuck(&format!("{KEYRINGS}/trindle-{ID}.gpg"));
        let key = s.path(&format!("{KEYRINGS}/trindle-{ID}.asc"));
        let said = use_source(ID, "https://example.com/apt", "stable", "main", &key, &was).unwrap_err();
        assert!(!said.contains("taken away again"), "{said}");
        assert!(said.contains("not signed") && said.contains("did not work either"), "{said}");
    }

    #[test]
    fn a_source_that_could_not_be_used_is_taken_away_again() {
        let s = StandIn::failing("source-back", "apt-get -q update", "E: The repository is not signed.");
        s.write(&format!("{LISTS}/other.sources"), b"");
        let key = s.path(&format!("{KEYRINGS}/trindle-{ID}.asc"));
        let said = use_source(ID, "https://example.com/apt", "stable", "main", &key, &Was::of(ID).unwrap()).unwrap_err();
        assert!(said.starts_with("The source could not be used, so it was taken away again."), "{said}");
        assert!(!s.path(&sources::file_for(ID).to_string_lossy()).exists());
    }

    #[test]
    fn a_source_added_again_that_cannot_be_used_leaves_the_one_that_worked() {
        let s = StandIn::failing("source-again", "apt-get -q update", "E: The repository is not signed.");
        let file = s.path(&sources::file_for(ID).to_string_lossy());
        let (asc, gpg) = (s.path(&format!("{KEYRINGS}/trindle-{ID}.asc")), s.path(&format!("{KEYRINGS}/trindle-{ID}.gpg")));
        s.write(&sources::file_for(ID).to_string_lossy(), b"Types: deb\nURIs: https://example.com/working\n");
        s.write(&format!("{KEYRINGS}/trindle-{ID}.asc"), b"the key that works");
        let was = Was::of(ID).unwrap();

        std::fs::write(&asc, b"another key").unwrap();
        std::fs::write(&gpg, b"\x99another").unwrap();
        let said = use_source(ID, "https://example.com/other", "stable", "main", &gpg, &was).unwrap_err();

        assert_eq!(said, "The source could not be used. E: The repository is not signed.");
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "Types: deb\nURIs: https://example.com/working\n");
        assert_eq!(std::fs::read(&asc).unwrap(), b"the key that works");
        assert!(!gpg.exists(), "the key that came with the one refused is gone");

        std::fs::remove_file(&asc).unwrap();
        s.stuck(&format!("{KEYRINGS}/trindle-{ID}.asc"));
        assert!(Was::of(ID).err().is_some_and(|e| e.starts_with("Nothing was changed, because")));
    }

    #[test]
    fn setting_up_that_fails_once_the_program_is_in_is_said() {
        for (id, tool) in [("tailscale", "tailscale set"), ("syncthing", "systemctl enable"), ("claude", "usermod"), ("docker", "usermod")] {
            let k = sources::known(id).expect("one of the ready-made sources");
            let s = StandIn::failing("source-after", tool, "it would not");
            let said = after(k, "john", true).unwrap_err();
            assert!(said.contains("is installed, but its setting up did not finish") && said.ends_with("it would not"), "{id}: {said}");
            drop(s);
            let _s = StandIn::all_well("source-after");
            assert!(after(k, "john", true).is_ok(), "{id}");
        }

        let s = StandIn::failing("source-after", "tailscale set", "it would not");
        assert!(after(sources::known("tailscale").unwrap(), "john", false).is_ok());
        assert!(s.ran().is_empty());
    }
}
