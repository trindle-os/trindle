//! The helper's update of the system: Debian's packages, Trindle's own and the kernel drivers built along the way.

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
    let Some(headers) = headers_package(&reading(&["dpkg", "--print-architecture"])) else { return Ok(()) };
    if headers_wanted(installed_version("dkms"), installed_version(&headers)) {
        let mut argv = APT_QUIET.to_vec();
        argv.extend(["install", &headers]);
        apt_run(&argv)?;
    }
    Ok(())
}

fn headers_package(architecture: &str) -> Option<String> {
    let kind = architecture.trim();
    (!kind.is_empty() && kind.chars().all(|c| c.is_ascii_alphanumeric())).then(|| format!("linux-headers-{kind}"))
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
    tidy_after([headers, kept])
}

fn tidy_after(earlier: [Outcome; 2]) -> Outcome {
    use crate::common::filling::step_line;
    println!("{}", step_line(2, 2, t!("admin-step-tidying")));
    let tidied = tidy();
    let driver = driver_off();
    let said: Vec<String> = earlier.into_iter().chain([tidied, driver]).filter_map(Result::err).collect();
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
    fn the_headers_package_is_named_for_this_computers_kind() {
        assert_eq!(super::headers_package("arm64\n").as_deref(), Some("linux-headers-arm64"));
        assert_eq!(super::headers_package("amd64").as_deref(), Some("linux-headers-amd64"));
        assert_eq!(super::headers_package(""), None);
        assert_eq!(super::headers_package("a b; rm"), None);
    }

    #[test]
    fn the_update_installs_the_headers_only_for_a_driver_and_goes_on_without_them() {
        let installs = |s: &StandIn| s.ran().iter().filter(|c| c.contains("install linux-headers-amd64")).count();
        let quiet = "0 upgraded, 0 newly installed, 0 to remove and 0 not upgraded.\n";
        let s = StandIn::all_well("headers-wanted");
        s.prints("dpkg --print-architecture", "amd64\n");
        s.prints("dpkg-query -W dkms", "ii \t1:3.0");
        s.prints("apt-get -s", quiet);
        assert_eq!(super::apt_upgrade(), Ok(()));
        assert_eq!(installs(&s), 1, "{:?}", s.ran());
        drop(s);
        let s = StandIn::all_well("headers-not-wanted");
        s.prints("dpkg --print-architecture", "amd64\n");
        s.prints("dpkg-query -W dkms", "ii \t1:3.0");
        s.prints("dpkg-query -W linux-headers-amd64", "ii \t6.12");
        s.prints("apt-get -s", quiet);
        assert_eq!(super::apt_upgrade(), Ok(()));
        assert_eq!(installs(&s), 0, "they were there already");
        drop(s);
        let s = StandIn::all_well("headers-no-driver");
        s.prints("dpkg --print-architecture", "amd64\n");
        s.prints("apt-get -s", quiet);
        assert_eq!(super::apt_upgrade(), Ok(()));
        assert_eq!(installs(&s), 0, "no driver, no headers");
        drop(s);

        let s = StandIn::new("headers-fail", |argv| {
            if argv.contains(&"linux-headers-amd64") { Err("E: Unable to locate package linux-headers-amd64".into()) } else { Ok(()) }
        });
        s.prints("dpkg --print-architecture", "amd64\n");
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
            if argv.contains(&"autoremove") && !argv.contains(&"-s") { Err("E: the tidy-up broke".into()) } else { Ok(()) }
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
    fn debians_updates_alone_are_not_begun_without_knowing_trindles_version() {
        let s = StandIn::all_well("debian");
        let said = super::apt_upgrade_debian_at(None).unwrap_err();
        assert!(said.starts_with("Nothing was updated"), "{said}");
        assert!(s.ran().is_empty(), "apt was run though Trindle could not be held back");
        assert!(super::apt_upgrade_debian_at(Some("1.0".into())).is_ok());
        assert!(s.ran()[0].ends_with("--with-new-pkgs upgrade trindle=1.0"), "{:?}", s.ran());
    }
}
