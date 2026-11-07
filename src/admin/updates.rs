//! The helper's update of the system.

use super::{apt_run, Outcome, at, reading};
use super::packages::{installed_version, tidy};
use crate::toolkit::t;

pub(super) fn apt_upgrade() -> Outcome {
    apt_upgrade_keeping(None)
}

pub(super) fn no_questions() {
    #[cfg(not(test))]
    std::env::set_var("DEBIAN_FRONTEND", "noninteractive");
}

const TRINDLE: &str = "trindle";
const APT_QUIET: [&str; 6] = ["-q", "-y", "-o", "Dpkg::Options::=--force-confdef", "-o", "Dpkg::Options::=--force-confold"];

pub(super) fn apt_upgrade_debian() -> Outcome {
    apt_upgrade_debian_at(installed_version(TRINDLE))
}

fn apt_upgrade_debian_at(version: Option<String>) -> Outcome {
    if version.is_none() {
        return Err(t!("admin-update-version-unknown").into());
    }
    apt_upgrade_keeping(version)
}

fn headers_for_drivers() -> Outcome {
    if headers_wanted(installed_version("dkms"), installed_version("linux-headers-amd64")) {
        let mut argv = APT_QUIET.to_vec();
        argv.extend(["install", "linux-headers-amd64"]);
        apt_run(&argv)?;
    }
    Ok(())
}

fn headers_wanted(dkms: Option<String>, headers: Option<String>) -> bool {
    dkms.is_some() && headers.is_none()
}

fn apt_upgrade_keeping(trindle_at: Option<String>) -> Outcome {
    use crate::common::filling::step_line;
    no_questions();
    println!("{}", step_line(1, 2, t!("admin-step-installing-updates")));
    let headers = headers_for_drivers();
    let keep = trindle_at.map(|v| format!("{TRINDLE}={v}"));
    let mut argv = APT_QUIET.to_vec();
    argv.extend(["--with-new-pkgs", "upgrade"]);
    argv.extend(keep.as_deref());
    apt_run(&argv)?;
    let kept = kept_back(&argv, keep.is_some());
    println!("{}", step_line(2, 2, t!("admin-step-tidying")));
    let tidied = tidy();
    let driver = driver_off();
    let mut said = Vec::new();
    for ended in [headers, kept, tidied, driver] {
        if let Err(e) = ended {
            said.push(e);
        }
    }
    if said.is_empty() { Ok(()) } else { Err(said.join(" ")) }
}

const DRIVERS_FAILED: &str = "/var/lib/trindle/dkms-failed";

fn driver_feature(driver: &str) -> Option<String> {
    driver.starts_with("v4l2loopback").then(|| t!("admin-driver-virtual-camera").to_string())
}

fn driver_off() -> Outcome {
    let text = std::fs::read_to_string(at(DRIVERS_FAILED)).unwrap_or_default();
    let names: Vec<&str> = text.lines().map(str::trim)
        .filter(|n| n.len() < 64 && n.starts_with(|c: char| c.is_ascii_alphanumeric()) && n.bytes().all(|b| b.is_ascii_alphanumeric() || b"-_.+".contains(&b))).collect();
    let known = reading(&["dkms", "status"]);
    let names: Vec<&str> = names.into_iter().filter(|n| *n == "dkms" || known.lines().any(|l| l.split(['/', ',', ':']).next().is_some_and(|d| d.trim() == *n))).collect();
    if names.is_empty() {
        return Ok(());
    }
    let mut said: Vec<String> = names.iter().map(|n| match driver_feature(n) {
        Some(feature) => t!("admin-driver-off-known", driver = *n, feature = feature).to_string(),
        None => t!("admin-driver-off", driver = *n).to_string(),
    }).collect();
    said.push(t!("admin-driver-off-rest").to_string());
    Err(said.join(" "))
}

fn kept_back(upgrade_args: &[&str], keeps_trindle: bool) -> Outcome {
    let mut argv = vec!["apt-get", "-s", "-q"];
    argv.extend(upgrade_args.iter().filter(|a| !a.starts_with("trindle=")));
    let said = reading;
    let mut left = kept_back_in(&said(&argv));
    if left.is_empty() {
        return Ok(());
    }
    let held = said(&["apt-mark", "showhold"]);
    left.retain(|p| !(held.split_whitespace().any(|h| h == p) || (keeps_trindle && p == TRINDLE)));
    if left.is_empty() { Ok(()) } else { Err(t!("admin-update-kept-back", list = left.join(", "))) }
}

pub fn kept_back_in(said: &str) -> Vec<String> {
    let mut lines = said.lines().skip_while(|l| !l.starts_with("The following packages have been kept back"));
    lines.next();
    lines.take_while(|l| l.starts_with(' ')).flat_map(|l| l.split_whitespace()).map(String::from)
        .filter(|n| n.bytes().all(|b| b.is_ascii_alphanumeric() || b"+-.:".contains(&b))).collect()
}

pub(super) fn apt_upgrade_trindle() -> Outcome {
    use crate::common::filling::step_line;
    no_questions();
    println!("{}", step_line(1, 2, t!("admin-step-installing-trindle")));
    let mut argv = APT_QUIET.to_vec();
    argv.extend(["--only-upgrade", "--no-install-recommends", "install", TRINDLE]);
    apt_run(&argv)?;
    println!("{}", step_line(2, 2, t!("admin-step-tidying")));
    tidy()
}

#[cfg(test)]
mod tests {
    use crate::admin::stand_in::StandIn;

    #[test]
    fn packages_apt_keeps_back_are_named_and_never_a_silent_success() {
        let said = "Reading package lists...\nThe following packages have been kept back:\n  firmware-amd-graphics firmware-misc-nonfree\n  linux-image-amd64 trindle\nThe following packages will be upgraded:\n  libfoo\n0 upgraded, 0 newly installed";
        assert_eq!(super::kept_back_in(said), ["firmware-amd-graphics", "firmware-misc-nonfree", "linux-image-amd64", "trindle"]);
        assert!(super::kept_back_in("0 upgraded, 0 newly installed").is_empty());
        let stand = StandIn::all_well("kept-back");
        stand.prints("apt-get -s", said);
        stand.prints("apt-mark showhold", "firmware-misc-nonfree\n");
        let err = super::apt_upgrade_debian_at(Some("1.0".into())).unwrap_err();
        assert_eq!(err, crate::toolkit::t!("admin-update-kept-back", list = "firmware-amd-graphics, linux-image-amd64"));
        assert!(stand.ran().iter().any(|c| c.contains("NeverAutoRemove")), "the tidy-up still runs");
        assert!(crate::common::help_pages::classify(&err).is_some(), "the words open their own help");
    }

    #[test]
    fn the_kernel_headers_come_with_the_first_update_after_a_driver_is_built_by_dkms() {
        let v = || Some("1".to_string());
        assert!(super::headers_wanted(v(), None));
        assert!(!super::headers_wanted(None, None), "no driver to build, no headers");
        assert!(!super::headers_wanted(v(), v()), "already there");
    }

    #[test]
    fn the_update_installs_the_headers_only_for_a_driver_and_goes_on_without_them() {
        let installs = |s: &StandIn| s.ran().iter().filter(|c| c.contains("install linux-headers-amd64")).count();
        let quiet = "0 upgraded, 0 newly installed, 0 to remove and 0 not upgraded.\n";
        let s = StandIn::all_well("headers-wanted");
        s.prints("dpkg-query -W dkms", "ii \t1:3.0");
        s.prints("apt-get -s", quiet);
        assert_eq!(super::apt_upgrade(), Ok(()));
        assert_eq!(installs(&s), 1, "{:?}", s.ran());
        drop(s);
        let s = StandIn::all_well("headers-not-wanted");
        s.prints("dpkg-query -W dkms", "ii \t1:3.0");
        s.prints("dpkg-query -W linux-headers-amd64", "ii \t6.12");
        s.prints("apt-get -s", quiet);
        assert_eq!(super::apt_upgrade(), Ok(()));
        assert_eq!(installs(&s), 0, "they were there already");
        drop(s);
        let s = StandIn::all_well("headers-no-driver");
        s.prints("apt-get -s", quiet);
        assert_eq!(super::apt_upgrade(), Ok(()));
        assert_eq!(installs(&s), 0, "no driver, no headers");
        drop(s);
        let s = StandIn::new("headers-fail", |argv| {
            if argv.contains(&"linux-headers-amd64") { Err("E: Unable to locate package linux-headers-amd64".into()) } else { Ok(()) }
        });
        s.prints("dpkg-query -W dkms", "ii \t1:3.0");
        s.prints("apt-get -s", quiet);
        let err = super::apt_upgrade().unwrap_err();
        assert!(err.contains("linux-headers-amd64"), "{err}");
        assert!(s.ran().iter().any(|c| c.contains("upgrade")), "the upgrade went on: {:?}", s.ran());
        assert!(s.ran().iter().any(|c| c.contains("NeverAutoRemove")), "{:?}", s.ran());
    }

    #[test]
    fn a_tidy_up_that_fails_does_not_hide_the_other_news() {
        let s = StandIn::new("tidy-fails", |argv| {
            if argv.contains(&["auto", "remove"].concat().as_str()) && !argv.contains(&"-s") { Err("E: the tidy-up broke".into()) } else { Ok(()) }
        });
        s.write(super::DRIVERS_FAILED, b"v4l2loopback\n");
        s.prints("dkms status", "v4l2loopback/0.15.0, 7.2.9, x86_64: built\n");
        s.prints("apt-get -s", "The following packages have been kept back:\n  firmware-misc-nonfree\n0 upgraded");
        let err = super::apt_upgrade().unwrap_err();
        assert!(err.contains("the tidy-up broke"), "{err}");
        assert!(err.contains("v4l2loopback") && err.contains("firmware-misc-nonfree"), "the other news was dropped: {err}");
    }

    #[test]
    fn an_update_that_keeps_nothing_back_is_quiet() {
        let stand = StandIn::all_well("kept-back-none");
        stand.prints("apt-get -s", "0 upgraded, 0 newly installed, 0 to remove and 0 not upgraded.\n");
        assert_eq!(super::apt_upgrade(), Ok(()));
    }

    #[test]
    fn a_driver_that_will_not_build_never_stops_the_update_and_is_named_with_its_feature() {
        let stand = StandIn::all_well("dkms-off");
        stand.write(super::DRIVERS_FAILED, b"v4l2loopback\nnvidia-current\n--bad\ngone-since\n");
        stand.prints("dkms status", "v4l2loopback/0.15.0, 7.2.9, x86_64: built\nnvidia-current/1, 7.2.9, x86_64: built\n");
        stand.prints("apt-get -s", "0 upgraded, 0 newly installed, 0 to remove and 0 not upgraded.\n");
        let err = super::apt_upgrade().unwrap_err();
        assert!(err.contains("virtual camera") && err.contains("v4l2loopback") && err.contains("nvidia-current"), "{err}");
        assert!(!err.contains("--bad"), "{err}");
        assert!(!err.contains("gone-since"), "a driver DKMS no longer has is not one that would not build: {err}");
        assert!(stand.ran().iter().any(|c| c.contains("NeverAutoRemove")), "the tidy-up and the rest still ran");
        assert!(crate::common::help_pages::classify(&err).is_some(), "the words open their own help");
        stand.write(super::DRIVERS_FAILED, b"");
        assert_eq!(super::apt_upgrade(), Ok(()));
    }

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
        for apt in ["system/apt/trindle.sources", "system/apt/trindle-backports.sources"] {
            let text = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(apt)).unwrap();
            assert!(!text.contains("ID=trindle") && !text.contains("os-release"), "{apt} never reads the system's ID");
        }
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

    #[test]
    fn debians_updates_alone_are_not_begun_without_knowing_trindles_version() {
        let s = StandIn::all_well("debian");
        let said = super::apt_upgrade_debian_at(None).unwrap_err();
        assert!(said.starts_with("Nothing was updated"), "{said}");
        assert!(s.ran().is_empty(), "apt was run though Trindle could not be held back");
        assert!(super::apt_upgrade_debian_at(Some("1.0".into())).is_ok());
        assert!(s.ran()[0].ends_with("--with-new-pkgs upgrade trindle=1.0"), "{:?}", s.ran());
    }
}
