//! Tests of the package's own files: the system's names, the diverted files, the kernel hook and the maintainer scripts.

#[cfg(test)]
mod tests {
    #[test]
    fn the_system_names_itself_trindle_based_on_debian_and_apt_still_sees_debian() {
        let field = |k: &str| include_str!("../../system/identity/os-release").lines()
            .find_map(|l| l.strip_prefix(k)?.strip_prefix('=')).map(|v| v.trim_matches('"').to_string()).unwrap_or_default();
        assert_eq!((field("NAME"), field("ID"), field("ID_LIKE"), field("VERSION_CODENAME")), ("Trindle".into(), "trindle".into(), "debian".into(), "trixie".into()));
        assert!(field("PRETTY_NAME").starts_with("Trindle @VERSION@ (based on Debian") && field("VERSION_ID") == "@VERSION@");
        assert!(field("HOME_URL").contains("trindle.org") && field("BUG_REPORT_URL").contains("trindle"));
        assert!(include_str!("../../system/identity/issue").starts_with("Trindle \\n \\l"));
        assert_eq!(include_str!("../../system/identity/issue.net").trim(), "Trindle");
        let (pre, rm, files) = (include_str!("../../tools/deb/preinst"), include_str!("../../tools/deb/postrm"), include_str!("../../tools/system-files.sh"));
        assert!(!include_str!("../../tools/deb/prerm").contains("/etc/issue"), "the divert comes off after our copies are removed, never before");
        for f in ["/usr/lib/os-release", "/etc/issue", "/etc/issue.net"] {
            assert!(pre.contains(&format!("--divert {f}.distrib {f}")) && rm.contains(f), "{f} is diverted and given back");
        }
        assert!(files.contains("$dest/usr/lib/os-release") && files.contains("$dest/etc/issue.net"));
        assert!(pre.contains("\"$id\" != trindle"), "an installed Trindle is not warned about");
        for apt in ["system/apt/trindle.sources", "system/apt/trindle-backports.sources", "system/apt/trindle-contrib.sources"] {
            let text = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(apt)).unwrap();
            assert!(!text.contains("ID=trindle") && !text.contains("os-release"), "{apt} never reads the system's ID");
        }
    }

    #[test]
    fn debians_contrib_section_is_on_for_the_graphics_driver_and_the_package_ships_it() {
        let text = include_str!("../../system/apt/trindle-contrib.sources");
        for stanza in text.split("\n\n") {
            let components: Vec<&str> = stanza.lines().filter(|l| l.starts_with("Components:")).collect();
            assert_eq!(components, ["Components: contrib"], "only contrib, so a computer that has non-free is not told twice");
            assert!(stanza.contains("Signed-By: /usr/share/keyrings/debian-archive-keyring.pgp") && stanza.contains("debian.org/debian"));
        }
        assert_eq!(text.matches("Types: deb\n").count(), 2, "Debian and its security updates");
        assert!(include_str!("../../tools/system-files.sh").contains("system/apt/trindle-contrib.sources"));
    }

    #[test]
    fn the_kernel_hook_runs_debians_and_never_fails_and_the_package_diverts_it() {
        let hook = include_str!("../../system/kernel/dkms-hook");
        assert!(hook.contains("dkms.distrib") && hook.trim_end().ends_with("exit 0"));
        assert!(include_str!("../../tools/deb/preinst").contains("--divert /etc/kernel/postinst.d/dkms.distrib /etc/kernel/postinst.d/dkms"));
        assert!(include_str!("../../tools/deb/prerm").contains("dpkg-divert --package trindle --quiet --remove"));
        assert!(include_str!("../../tools/system-files.sh").contains("system/kernel/dkms-hook"));

        let dir = std::env::temp_dir().join(format!("trindle-dkms-hook-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let state = dir.join("failed");
        let mut script = hook.to_string();
        for (from, to) in [
            ("/etc/kernel/postinst.d/dkms.distrib", dir.join("d").to_str().unwrap().to_string()),
            ("/var/lib/trindle/dkms-failed", state.to_str().unwrap().to_string()),
            ("mkdir -p /var/lib/trindle", "true".to_string()),
            ("dkms status -k \"$1\"", "printf 'v4l2loopback/0.15.0, 7.2.9, x86_64: built\\nother/1, 7.2.9, x86_64: installed\\n'".to_string()),
        ] {
            assert!(script.contains(from), "the hook no longer has {from}: the test would run its real paths");
            script = script.replace(from, &to);
        }
        assert!(!script.contains("/var/lib") && !script.contains("/etc/kernel"), "a real path is left in the hook under test");
        std::fs::write(dir.join("d"), "#!/bin/sh\nexit 11\n").unwrap();
        std::fs::write(dir.join("hook"), script).unwrap();
        for f in ["d", "hook"] {
            std::process::Command::new("chmod").arg("755").arg(dir.join(f)).status().unwrap();
        }
        let st = std::process::Command::new("sh").arg(dir.join("hook")).arg("7.2.9").status().unwrap();
        assert!(st.success(), "a driver that will not build must not fail the kernel");
        assert_eq!(std::fs::read_to_string(&state).unwrap().trim(), "v4l2loopback");
        std::fs::write(dir.join("d"), "#!/bin/sh\nexit 0\n").unwrap();
        assert!(std::process::Command::new("sh").arg(dir.join("hook")).arg("7.2.9").status().unwrap().success());
        assert!(!state.exists(), "cleared once a kernel builds all its drivers");

        std::fs::write(&state, "v4l2loopback\n").unwrap();
        std::fs::remove_file(dir.join("d")).unwrap();
        assert!(std::process::Command::new("sh").arg(dir.join("hook")).arg("7.2.9").status().unwrap().success());
        assert!(!state.exists(), "an old failure outlived the hook it came from");
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn login_screen_step(dir: &std::path::Path, link_to: &std::path::Path) -> (Vec<String>, String) {
        let postinst = include_str!("../../tools/deb/postinst");
        let lines: Vec<&str> = postinst.lines().collect();
        let at = lines.iter().position(|l| l.contains("/etc/systemd/system/display-manager.service")).expect("a login-screen step");
        let start = (0..at).rev().find(|&i| lines[i] == "if [ -d /run/systemd/system ]; then").expect("its systemd check");
        let end = (at..lines.len()).find(|&i| lines[i] == "fi").expect("its end");
        let log = dir.join("calls");
        let dm = dir.join("display-manager.service");
        for old in [&log, &dm] {
            let _ = std::fs::remove_file(old);
        }
        let bin = dir.join("bin");
        std::fs::create_dir_all(&bin).unwrap();
        std::fs::create_dir_all(dir.join("run")).unwrap();
        for tool in ["systemctl", "deb-systemd-invoke", "invoke-rc.d", "service", "loginctl", "pkill", "kill"] {
            let f = bin.join(tool);
            std::fs::write(&f, format!("#!/bin/sh\necho \"{tool} $*\" >> '{}'\n", log.display())).unwrap();
            std::process::Command::new("chmod").arg("755").arg(&f).status().unwrap();
        }
        std::os::unix::fs::symlink(link_to, &dm).unwrap();
        let mut script = lines[start..=end].join("\n");
        for (from, to) in [("/etc/systemd/system/display-manager.service", dm.display().to_string()), ("/run/systemd/system", dir.join("run").display().to_string())] {
            script = script.replace(from, &to);
        }
        assert!(!script.contains("/etc/systemd") && !script.contains("/run/systemd"), "a real path is left in the step under test");
        let path = format!("{}:{}", bin.display(), std::env::var("PATH").unwrap_or_default());
        let out = std::process::Command::new("sh").arg("-ec").arg(&script).env("PATH", path).output().unwrap();
        assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
        let calls = std::fs::read_to_string(&log).unwrap_or_default().lines().map(String::from).collect();
        (calls, String::from_utf8_lossy(&out.stderr).into_owned())
    }

    #[test]
    fn another_login_screen_is_switched_for_the_next_start_and_never_stopped() {
        let dir = std::env::temp_dir().join(format!("trindle-dm-switch-{}", std::process::id()));
        let unit = dir.join("lightdm.service");
        std::fs::create_dir_all(&dir).unwrap();

        let (calls, said) = login_screen_step(&dir, &unit);
        assert_eq!(calls, ["systemctl enable greetd.service"]);
        assert!(said.is_empty() && !dir.join("display-manager.service").exists(), "{said}");
        std::fs::write(&unit, "").unwrap();
        let (calls, said) = login_screen_step(&dir, &unit);
        assert!(!calls.iter().any(|c| ["stop", "restart", "kill", "isolate", "terminate"].iter().any(|w| c.contains(w))), "a running login screen was stopped: {calls:?}");
        assert!(calls.contains(&"systemctl disable lightdm.service".to_string()), "{calls:?}");
        assert!(calls.contains(&"systemctl enable greetd.service".to_string()), "{calls:?}");
        assert!(said.contains("trindle: lightdm.service keeps running until the computer restarts"), "the person is not told: {said}");

        let greetd = dir.join("greetd.service");
        std::fs::write(&greetd, "").unwrap();
        let (calls, said) = login_screen_step(&dir, &greetd);
        assert_eq!(calls, ["systemctl enable greetd.service"]);
        assert!(said.is_empty(), "{said}");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_package_scripts_stop_and_restart_only_trindles_own_services() {
        for (name, text) in [
            ("preinst", include_str!("../../tools/deb/preinst")),
            ("postinst", include_str!("../../tools/deb/postinst")),
            ("prerm", include_str!("../../tools/deb/prerm")),
            ("postrm", include_str!("../../tools/deb/postrm")),
        ] {
            for line in text.lines().map(str::trim).filter(|l| !l.starts_with('#')) {
                let words: Vec<&str> = line.split_whitespace().collect();
                for (i, w) in words.iter().enumerate() {
                    if !["systemctl", "deb-systemd-invoke", "invoke-rc.d", "service"].contains(w) {
                        continue;
                    }
                    let Some(verb) = words.get(i + 1) else { continue };
                    if ["stop", "restart", "try-restart", "reload-or-restart", "kill", "isolate"].contains(verb) {
                        let unit = words.get(i + 2).copied().unwrap_or("");
                        assert!(unit.starts_with("trindle-") || unit == "systemd-journald", "{name} stops or restarts {unit}: {line}");
                    }
                }
            }
        }
    }
}
