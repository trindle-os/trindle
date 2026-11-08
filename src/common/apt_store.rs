//! The store computers running Trindle update from is signed by the one
//! key they check it against, and nothing else is ever put up.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use super::needs;

struct Keys {
    top: PathBuf,
    home: PathBuf,
}

impl Keys {
    fn new() -> Keys {
        use std::os::unix::fs::DirBuilderExt;

        static MADE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
        let n = MADE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let top = Path::new(env!("CARGO_MANIFEST_DIR")).join("target").join(format!("apt-store-test-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        std::fs::create_dir_all(&top).unwrap();
        let home = top.join("keys");
        std::fs::DirBuilder::new().mode(0o700).create(&home).unwrap();
        Keys { top, home }
    }

    fn gpg(&self, home: &Path, args: &[&str]) -> Output {
        Command::new(needs::program("gpg")).arg("--homedir").arg(home).args(["--batch", "--no-tty", "--quiet"]).args(args)
            .current_dir(&self.top).output().unwrap()
    }

    fn make(&self, name: &str) -> String {
        let made = self.gpg(&self.home, &["--pinentry-mode", "loopback", "--passphrase", "", "--quick-generate-key", name, "ed25519", "sign", "never"]);
        assert!(made.status.success(), "{}", String::from_utf8_lossy(&made.stderr));
        let listed = self.gpg(&self.home, &["--list-keys", "--with-colons", &format!("={name}")]);
        let listed = String::from_utf8_lossy(&listed.stdout).into_owned();
        listed.lines().find_map(|l| l.strip_prefix("fpr:")).map(|rest| rest.trim_matches(':').to_string()).expect("the key just made")
    }

    fn script(&self, name: &str, repo: &Path, shipped: &Path) -> Output {
        self.script_said(name, repo, shipped, true)
    }

    fn script_said(&self, name: &str, repo: &Path, shipped: &Path, test: bool) -> Output {
        let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("tools/apt").join(name);
        let mut run = Command::new(needs::program("sh"));
        run.arg(script).arg(repo)
            .env("GNUPGHOME", &self.home).env("HOME", &self.top)
            .env("TRINDLE_APT_EMPTY", "1").env("TRINDLE_APT_KEYRING", shipped)
            .env("TRINDLE_APT_KEY_FILE", self.top.join("no-login")).env("RCLONE", self.top.join("no-sender"))
            .env_remove("TRINDLE_APT_KEY").env_remove("TRINDLE_APT_TEST").current_dir(&self.top);
        if test {
            run.env("TRINDLE_APT_TEST", "1");
        }
        run.output().unwrap()
    }

    fn signed_by(&self, shipped: &Path, what: &[&str]) -> Option<String> {
        use std::os::unix::fs::DirBuilderExt;
        let check = self.top.join("check");
        let _ = std::fs::remove_dir_all(&check);
        std::fs::DirBuilder::new().mode(0o700).create(&check).unwrap();
        assert!(self.gpg(&check, &["--import", &shipped.to_string_lossy()]).status.success());
        let mut args = vec!["--status-fd", "1", "--verify"];
        args.extend(what);
        let said = self.gpg(&check, &args);
        String::from_utf8_lossy(&said.stdout).lines()
            .find_map(|l| l.strip_prefix("[GNUPG:] VALIDSIG ").and_then(|rest| rest.split_whitespace().last().map(String::from)))
    }
}

impl Drop for Keys {
    fn drop(&mut self) {
        for home in [self.home.clone(), self.top.join("check")] {
            // The agent gpg started for a folder of this test's own.
            let _ = Command::new("gpgconf").arg("--homedir").arg(home).args(["--kill", "all"]).output();
        }
        let _ = std::fs::remove_dir_all(&self.top);
    }
}

fn wanted_its_login(out: &Output) -> bool {
    String::from_utf8_lossy(&out.stderr).contains("no key at")
}

fn found_it_good(out: &Output) -> bool {
    !out.status.success() && !wanted_its_login(out) && String::from_utf8_lossy(&out.stderr).contains("stand-in")
}

#[test]
fn a_stand_in_key_is_only_ever_the_tests_and_nothing_goes_up_with_one() {
    needs::programs(&["gpg", "apt-ftparchive", "sh", "gzip", "cmp"]);
    let k = Keys::new();
    let own = k.make("Trindle Archive");
    let shipped = k.top.join("shipped.gpg");
    std::fs::write(&shipped, k.gpg(&k.home, &["--export", &own]).stdout).unwrap();
    let repo = k.top.join("repo");
    let said = |out: &Output| String::from_utf8_lossy(&out.stderr).into_owned();

    let built = k.script_said("build.sh", &repo, &shipped, false);
    assert!(!built.status.success() && said(&built).contains("TRINDLE_APT_KEYRING"), "{}", said(&built));
    assert!(!repo.join("dists/stable/InRelease").exists(), "a store was signed for a key that is not the shipped one");

    let built = k.script("build.sh", &repo, &shipped);
    assert!(built.status.success(), "{}", said(&built));
    for name in ["verify.sh", "publish.sh"] {
        let out = k.script_said(name, &repo, &shipped, false);
        assert!(!out.status.success() && !wanted_its_login(&out) && said(&out).contains("TRINDLE_APT_KEYRING"), "{name}: {}", said(&out));
    }

    let out = k.script("publish.sh", &repo, &shipped);
    assert!(found_it_good(&out) && said(&out).contains("Nothing was put up"), "{}", said(&out));
}

fn path(p: &Path) -> String {
    p.to_string_lossy().into_owned()
}

fn build_and_check_the_signing(k: &Keys, shipped: &Path, own: &str) -> (PathBuf, PathBuf, PathBuf, PathBuf) {

    let repo = k.top.join("repo");
    let built = k.script("build.sh", &repo, shipped);
    assert!(built.status.success(), "{}", String::from_utf8_lossy(&built.stderr));
    let dist = repo.join("dists/stable");
    let (release, detached, inline) = (dist.join("Release"), dist.join("Release.gpg"), dist.join("InRelease"));
    assert_eq!(k.signed_by(shipped, &[&path(&detached), &path(&release)]), Some(own.to_string()), "Release.gpg is the shipped key's");
    assert_eq!(k.signed_by(shipped, &[&path(&inline)]), Some(own.to_string()), "InRelease is the shipped key's");
    assert_eq!(std::fs::read(repo.join("trindle-archive-keyring.gpg")).unwrap(), std::fs::read(shipped).unwrap(),
               "the public half put in the store is the one it was signed for");
    (repo, release, detached, inline)
}

fn the_key_trindle_ships_is_named_in_the_notes(k: &Keys) {

    let real = Path::new(env!("CARGO_MANIFEST_DIR")).join("system/apt/trindle-archive-keyring.gpg");
    let shown = k.gpg(&k.top.join("check"), &["--show-keys", "--with-colons", &path(&real)]);
    let shown = String::from_utf8_lossy(&shown.stdout).into_owned();
    let prints: Vec<&str> = shown.lines().filter_map(|l| l.strip_prefix("fpr:")).map(|rest| rest.trim_matches(':')).collect();
    assert_eq!(prints, ["DC239DC34F02A6A6D569E83B6D17DFB7011DD6AA"]);
}

#[test]
fn the_update_store_is_signed_by_the_shipped_key_and_nothing_else_goes_up() {
    needs::programs(&["gpg", "apt-ftparchive", "sh", "gzip", "cmp"]);
    let k = Keys::new();

    let other = k.make("Old Trindle Archive (not the one)");
    let own = k.make("Trindle Archive");
    assert_ne!(other, own);
    let shipped = k.top.join("shipped.gpg");
    let export = k.gpg(&k.home, &["--export", &own]);
    std::fs::write(&shipped, &export.stdout).unwrap();
    let other_half = k.top.join("other.gpg");
    std::fs::write(&other_half, k.gpg(&k.home, &["--export", &other]).stdout).unwrap();

    let (repo, release, detached, inline) = build_and_check_the_signing(&k, &shipped, &own);

    let good = k.script("publish.sh", &repo, &shipped);
    assert!(found_it_good(&good), "{}", String::from_utf8_lossy(&good.stderr));

    let refused = |what: &str| {
        let out = k.script("publish.sh", &repo, &shipped);
        let said = String::from_utf8_lossy(&out.stderr).into_owned();
        assert!(!out.status.success() && !wanted_its_login(&out) && !found_it_good(&out) && said.contains("is not signed by Trindle's archive key. Nothing was put up"), "{what}: {said}");
    };
    let (was_release, was_detached, was_inline) = (std::fs::read(&release).unwrap(), std::fs::read(&detached).unwrap(), std::fs::read(&inline).unwrap());
    let put_right = || {
        for (at, was) in [(&release, &was_release), (&detached, &was_detached), (&inline, &was_inline)] {
            std::fs::write(at, was).unwrap();
        }
    };

    let mut changed = was_release.clone();
    changed.extend_from_slice(b"Changed: after signing\n");
    std::fs::write(&release, &changed).unwrap();
    refused("the list is not the one signed");
    put_right();

    for at in [&detached, &inline] {
        std::fs::remove_file(at).unwrap();
    }
    let by = format!("{other}!");
    assert!(k.gpg(&k.home, &["--yes", "--local-user", &by, "--armor", "--detach-sign", "-o", &path(&detached), &path(&release)]).status.success());
    assert!(k.gpg(&k.home, &["--yes", "--local-user", &by, "--clearsign", "-o", &path(&inline), &path(&release)]).status.success());
    assert_eq!(k.signed_by(&other_half, &[&path(&inline)]), Some(other.clone()), "the stand-in for a wrong key did sign it");
    refused("signed by another key");
    put_right();

    let elsewhere = k.top.join("other-list");
    std::fs::write(&elsewhere, b"Origin: Trindle\nSuite: stable\n").unwrap();
    std::fs::remove_file(&inline).unwrap();
    assert!(k.gpg(&k.home, &["--yes", "--local-user", &format!("{own}!"), "--clearsign", "-o", &path(&inline), &path(&elsewhere)]).status.success());
    refused("the inline copy is not the list");
    put_right();

    std::fs::remove_file(&detached).unwrap();
    refused("no Release.gpg");
    put_right();

    assert!(found_it_good(&k.script("publish.sh", &repo, &shipped)));

    the_key_trindle_ships_is_named_in_the_notes(&k);
}
