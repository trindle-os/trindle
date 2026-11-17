//! Tests of the files Trindle ships or builds with outside src/: system files, tools, packaging and documents.

use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    std::fs::read_to_string(root().join(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"))
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-shipped-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn gate_ships() -> bool {
    root().join(GATE).is_file()
}

const GATE: &str = "tools/release/release-check.sh";

fn release_check(conf: &Path, args: &[&str]) -> (bool, String) {
    let out = Command::new("bash").arg(root().join(GATE)).args(args)
        .env("TRINDLE_RELEASE_CONF", conf).env("LC_ALL", "C").output().unwrap();
    (out.status.success(), format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr)))
}

#[test]
fn the_release_gate_reads_private_words_from_outside_the_repository() {
    if !gate_ships() {
        return;
    }
    let script = read(GATE);
    for private in ["/home/", "Claude", "ibis-porgy", "forgejo", "lightmorphic", "8-Claude"] {
        assert!(!script.contains(private), "release-check.sh holds {private}");
    }
    let dir = scratch("release-words");
    let (clean, leak) = (dir.join("clean"), dir.join("leak"));
    std::fs::create_dir_all(&clean).unwrap();
    std::fs::create_dir_all(&leak).unwrap();
    std::fs::write(clean.join("a"), "nothing here").unwrap();
    std::fs::write(leak.join("a"), "built by johnsmith").unwrap();
    std::fs::write(dir.join("words"), "# a comment\njohnsmith\n").unwrap();
    let conf = dir.join("check.conf");
    let (ok, said) = release_check(&conf, &["scan", clean.to_str().unwrap()]);
    assert!(!ok && said.contains("no settings file"), "{said}");
    std::fs::write(&conf, format!("words={}\n", dir.join("words").display())).unwrap();
    assert!(release_check(&conf, &["scan", clean.to_str().unwrap()]).0);
    let (ok, said) = release_check(&conf, &["scan", leak.to_str().unwrap()]);
    assert!(!ok && said.contains("hits in 1 files"), "{said}");
    std::fs::write(&conf, "words=relative/words\n").unwrap();
    assert!(!release_check(&conf, &["scan", clean.to_str().unwrap()]).0, "a relative path is refused");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_release_gate_refuses_a_signature_by_another_key() {
    if !gate_ships() {
        return;
    }
    if Command::new("gpg").arg("--version").output().is_err() {
        return;
    }
    let dir = scratch("release-sig");
    let own = dir.join("own");
    std::fs::create_dir_all(&own).unwrap();
    std::fs::write(dir.join("SHA256SUMS"), "0  trindle-0.9.1-beta.4-amd64.iso\n").unwrap();
    let gpg = |args: &[&str]| Command::new("gpg").arg("--homedir").arg(&own).args(["--batch", "--quiet", "--passphrase", ""]).args(args)
        .output().unwrap().status.success();
    assert!(gpg(&["--pinentry-mode", "loopback", "--quick-gen-key", "John Smith <john@example.com>", "ed25519", "sign", "never"]));
    let sums = dir.join("SHA256SUMS");
    assert!(gpg(&["--pinentry-mode", "loopback", "--detach-sign", "-o", dir.join("SHA256SUMS.sig").to_str().unwrap(), sums.to_str().unwrap()]));
    let _ = Command::new("gpgconf").arg("--homedir").arg(&own).args(["--kill", "all"]).status();
    let script = root().join(GATE);
    let out = Command::new("bash").arg("-c").arg("source <(sed '/^case ${1/,$d' \"$1\"); here=$2; signed_by_archive_key \"$3\"")
        .args(["_", script.to_str().unwrap(), root().to_str().unwrap(), dir.to_str().unwrap()]).output().unwrap();
    assert!(!out.status.success(), "a signature by another key passed");
    assert!(String::from_utf8_lossy(&out.stderr).contains("not a good signature by the archive key"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_release_gate_refuses_an_image_named_for_another_version() {
    if !gate_ships() {
        return;
    }
    let dir = scratch("release-version");
    let iso = dir.join("trindle-0.0.1-amd64.iso");
    std::fs::write(&iso, "").unwrap();
    let (ok, said) = release_check(&dir.join("none"), &["versions", iso.to_str().unwrap(), "none.deb"]);
    assert!(!ok && said.contains(&format!("trindle-{}-amd64.iso", env!("CARGO_PKG_VERSION"))), "{said}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_release_gate_compares_the_whole_export() {
    if !gate_ships() {
        return;
    }
    let script = read(GATE);
    assert!(script.contains("find \"$clean\" -mindepth 1 -maxdepth 1 ! -name target -exec rm -rf {} +"));
    assert!(script.contains("git -C \"$here\" archive HEAD | tar -x -C \"$tmp\""));
    assert!(!script.contains("0.9.0~beta"), "the package version is read, not written in");
    assert!(script.contains("has no line in SHA256SUMS"));
}

#[test]
fn the_boot_menu_stays_hidden_where_there_are_no_efi_folders() {
    let dir = scratch("grub-efi");
    let cfg = read("system/grub/trindle.cfg").replace("/boot/efi/EFI", dir.to_str().unwrap());
    let style = |cfg: &str| {
        let out = Command::new("sh").arg("-c").arg(format!("{cfg}\necho \"$GRUB_TIMEOUT_STYLE $GRUB_TIMEOUT\"")).output().unwrap();
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };
    assert_eq!(style(&cfg), "hidden 1");
    std::fs::create_dir_all(dir.join("debian")).unwrap();
    assert_eq!(style(&cfg), "hidden 1");
    std::fs::create_dir_all(dir.join("Microsoft")).unwrap();
    assert_eq!(style(&cfg), "menu 5");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_readme_names_every_program_that_runs_as_root() {
    let readme = read("README.md");
    for entry in std::fs::read_dir(root().join("system/systemd")).unwrap().flatten() {
        if !entry.file_name().to_string_lossy().ends_with(".service") {
            continue;
        }
        let unit = std::fs::read_to_string(entry.path()).unwrap();
        if unit.lines().any(|l| l.starts_with("User=")) {
            continue;
        }
        for line in unit.lines().filter(|l| l.starts_with("Exec")) {
            let Some(program) = line.split_whitespace().next().and_then(|p| p.rsplit('/').next()) else { continue };
            if program.starts_with("trindle-") {
                assert!(readme.contains(&format!("`{program}")), "{program} runs as root ({}) and the README does not say so", entry.path().display());
            }
        }
    }
    for (name, text) in [("README.md", readme.as_str()), ("SECURITY.md", &read("SECURITY.md"))] {
        assert!(!text.contains("Only `trindle-admin` runs as root") && !text.contains("only `trindle-admin` runs as root"), "{name}");
    }
}

#[test]
fn no_document_says_the_update_switch_stops_security_fixes() {
    for name in ["README.md", "SECURITY.md", "packages.txt"] {
        let text = read(name);
        for wrong in ["also stops Debian's daily security fixes", "unless Check for updates is switched off", "(Debian's daily security fixes and"] {
            assert!(!text.contains(wrong), "{name} says the switch stops the security fixes: {wrong}");
        }
    }
    let packages = read("packages.txt");
    assert!(!packages.contains("install.sh") && !packages.contains("authorization") && !packages.contains("two Trindle icon sets"));
}

#[test]
fn the_image_build_writes_what_the_download_page_names() {
    let build = read("tools/iso/build.sh");
    assert!(!build.contains("version-suffix") && !build.contains("target/pkg.sh") && !build.contains("cp -r \"$here/share\""));
    assert!(build.contains("dpkg-deb -f \"$main_deb\" Version") && build.contains("a test image needs a package built with TRINDLE_VERSION_SUFFIX"));
    assert!(build.contains("sha256sum \"$name\" > SHA256SUMS") && build.contains("name=\"trindle-$(sed -n"));
    let page = read("docs/verify-download.md");
    assert!(page.contains("trindle-<version>-amd64.iso") && page.contains("download it from `system/apt/`"));
    assert!(!page.contains("beta.1-amd64.iso"));
}

#[test]
fn no_tool_needs_a_script_from_the_build_folder() {
    for name in ["tools/iso/build.sh", "tools/real-polkit-check.sh", "tools/themes-preview.sh", "tools/site/strip.py", GATE] {
        if name == GATE && !gate_ships() {
            continue;
        }
        let text = read(name);
        for local in ["target/pkg.sh", "devlibs/env.sh", "rebuild.sh"] {
            assert!(!text.contains(local), "{name} needs {local}, which is not in the repository");
        }
    }
}

#[test]
fn the_catalogue_list_holds_no_computers_paths() {
    let sums = read("tools/catalogue-release.sums");
    for line in sums.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
        let path = line.split_whitespace().nth(2).unwrap_or("");
        assert!(!path.is_empty() && !path.starts_with('/') && !path.contains(".."), "{line}");
    }
    let scan_id = ["org.", "trindle.Scan"].concat();
    assert!(!(sums.contains(&scan_id) && sums.contains("Not built, not listed: trindle-scan")));
    assert!(read("tools/catalogue-release.sh").contains("TRINDLE_PLUGIN_SRC"));
}

#[test]
fn the_package_build_does_no_work_it_throws_away() {
    assert!(!read("tools/system-files.sh").contains("--names"), "the links are made by the postinst alone");
    assert!(!read("tools/deb/build.sh").contains("usr/bin"));
    let lock = read("tools/lock.sh");
    assert!(!lock.contains("0\\.9\\.0"), "lock.sh names one version of trindle-common");
}

#[test]
fn removing_trindle_remakes_the_boot_menu_and_purge_clears_its_state() {
    let dir = scratch("postrm");
    let bin = dir.join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    let log = dir.join("calls");
    for tool in ["update-grub", "dpkg-divert", "rm"] {
        std::fs::write(bin.join(tool), format!("#!/bin/sh\necho \"{tool} $*\" >> {}\n", log.display())).unwrap();
        Command::new("chmod").arg("755").arg(bin.join(tool)).status().unwrap();
    }
    let script = read("tools/deb/postrm").replace("/usr/sbin/update-grub", &bin.join("update-grub").display().to_string()).replace("/boot/grub", "/");
    let path = format!("{}:/usr/bin:/bin", bin.display());
    for verb in ["remove", "purge"] {
        let _ = std::fs::remove_file(&log);
        let ok = Command::new("sh").arg("-c").arg(&script).arg("postrm").arg(verb).env("PATH", &path).status().unwrap().success();
        let calls = std::fs::read_to_string(&log).unwrap_or_default();
        assert!(ok && calls.contains("update-grub"), "{verb}: {calls}");
        assert_eq!(calls.contains("rm -rf /var/lib/trindle /etc/trindle"), verb == "purge", "{verb}: {calls}");
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn the_archive_offers_only_amd64() {
    for name in ["system/apt/trindle.sources", "tools/apt/build.sh", "tools/apt/publish.sh"] {
        assert!(!read(name).contains("arm64"), "{name} offers arm64");
    }
}

#[test]
fn the_vendored_smithay_changes_are_all_recorded() {
    let notes = read("vendor/smithay/TRINDLE.md");
    assert!(notes.contains("`README.md` lacks one line"));
    let numbers: Vec<u32> = notes.lines().filter_map(|l| l.strip_prefix("Change ")).filter_map(|l| l.split([',', ':']).next()?.parse().ok()).collect();
    assert_eq!(numbers, (1..=numbers.len() as u32).collect::<Vec<_>>(), "the changes are numbered once, in order");
    assert!(!read("Cargo.toml").contains("Smithay 0.7.0 with one change"));
}

#[test]
fn shipped_words_say_what_is_true_now() {
    assert!(!read("tools/deb/preinst").contains("Ubuntu"));
    assert!(!read(&["system/polkit/org.", "trindle.admin.policy"].concat()).contains("Change users and languages"));
    assert!(!read("system/pam.d/trindle-lock").starts_with("# Not a back door"));
    let dated = |t: &str| ["October 2026", "November 2026"].iter().any(|m| t.contains(m));
    for name in ["system/polkit/60-trindle.rules", "system/apt/hardware-packages.txt", "tools/apt/publish.sh", "docs/apt-key-rotation.md"] {
        assert!(!dated(&read(name)), "{name} carries dated history");
    }
    let rotation = read("docs/apt-key-rotation.md");
    assert!(!rotation.contains("project session") && !rotation.contains("backed up"));
    assert!(!read("tools/apt/publish.sh").contains("GitHub Pages"));
    assert!(!read("CHANGELOG.md").contains("The first version published"));
    assert!(!read("tools/site/strip.py").contains("gi.write_text"), "the stripper never writes to the working tree");
    let polkit_check = read("tools/real-polkit-check.sh");
    assert!(polkit_check.lines().take_while(|l| l.starts_with('#')).count() < 25, "the header retells the test");
}

#[test]
fn the_shell_tools_name_their_variables() {
    for name in ["tools/apt/simulate-upgrade.sh", "tools/renewal-check.sh"] {
        let text = read(name);
        for one in 'A'..='Z' {
            assert!(!text.contains(&format!("${one} ")) && !text.contains(&format!("${one}/")) && !text.lines().any(|l| l.trim_start().starts_with(&format!("{one}="))),
                "{name} has a one-letter variable {one}");
        }
    }
    assert!(read("tools/idle-cost.sh").contains("nothing was measured"));
}

#[test]
fn the_test_machine_types_on_a_british_keyboard() {
    let testbox = read("tools/testbox.py");
    assert!(testbox.contains("\"#\": \"backslash\"") && !testbox.contains("\"#\": \"3\""));
    assert!(!testbox.contains("open(a[0], \"rb\").read()") && !testbox.contains("return open(WHO).read()"));
}

#[test]
fn the_checks_run_on_debian_13_with_pinned_actions() {
    let ci = read(".github/workflows/ci.yml");
    assert!(ci.contains("container: debian:13"));
    for line in ci.lines().filter_map(|l| l.trim().strip_prefix("- uses: ")) {
        let pin = line.rsplit('@').next().unwrap_or("");
        assert!(pin.len() == 40 && pin.bytes().all(|b| b.is_ascii_hexdigit()), "{line} is not pinned to a commit");
    }
}

#[test]
fn the_build_stops_without_the_english_words() {
    let build = read("build.rs");
    assert!(!build.contains("unwrap_or_default") && build.contains("holds no .ftl files"));
}

#[test]
fn a_session_has_room_for_more_than_128_file_watchers() {
    let conf = read("system/sysctl/90-trindle-watches.conf");
    let n: u32 = conf.lines().find_map(|l| l.strip_prefix("fs.inotify.max_user_instances = ")).expect("the setting").parse().unwrap();
    assert!(n >= 1024, "{n}");
    assert!(read("tools/system-files.sh").contains("usr/lib/sysctl.d/90-trindle-watches.conf"), "the file goes into the package");
}

#[test]
fn every_kernel_setting_trindle_ships_takes_effect_when_installed() {
    let postinst = read("tools/deb/postinst");
    let applied = postinst.lines().find(|l| l.trim_start().starts_with("systemd-sysctl ")).expect("the settings applied");
    let files = read("tools/system-files.sh");
    let shipped: Vec<&str> = files.split("usr/lib/sysctl.d/").skip(1).filter_map(|r| r.split('"').next()).collect();
    assert!(!shipped.is_empty());
    for name in shipped {
        assert!(applied.contains(name), "{name} waits for a restart");
    }
}

#[test]
fn every_shipped_language_has_its_fonts_package() {
    use crate::toolkit::lang::{script, tags_of};
    let packages: Vec<String> = read("packages.txt").lines()
        .filter_map(|l| l.split('#').next()).flat_map(|l| l.split_whitespace().map(str::to_string)).collect();
    let readme = read("README.md");
    for tag in crate::common::shipped_languages::tags() {
        let Some(s) = script::script_of(&tags_of(&tag)) else { continue };
        assert!(packages.iter().any(|p| p == s.package), "{tag} is written in {} and needs {}", s.name, s.package);
        assert!(readme.contains(&format!("| `{}` |", s.package)), "the README does not list {}", s.package);
    }
    for tag in ["zh-Hans", "ur", "bn", "hi", "ar"] {
        assert!(script::script_of(&tags_of(tag)).is_some(), "{tag} has no script in the toolkit's table");
    }
}
