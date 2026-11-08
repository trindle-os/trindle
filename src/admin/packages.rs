//! The helper's steps for programs: running apt with its progress, installing, removing and tidying, and firmware.

use std::process::{Command, Stdio};

use super::{apt_run, Outcome, protected, run};
use super::updates::no_questions;
use crate::toolkit::t;

pub(super) fn apt_refresh() -> Outcome {
    run(&["apt-get", "-q", "update"], None)
}

const REPLACED: &[&str] = &["labwc", "cage", "swaybg", "swayidle", "wlopm", "brightnessctl", "wlsunset",
                            "wl-clipboard", "wlr-randr", "grim",

                            "file-roller", "7zip", "zip", "unrar-free",

                            "rfkill"];

pub(super) fn tidy() -> Outcome {
    let installed = protected::installed_among(REPLACED)?;
    if !installed.is_empty() {
        let mut argv = vec!["apt-mark", "auto"];
        argv.extend(installed.iter().map(String::as_str));
        // A program still marked as chosen only stays.
        let _ = run(&argv, None);
    }
    protected::mark_chosen()?;
    let simulate = protected::autoremove_argv(true);
    let simulate: Vec<&str> = simulate.iter().map(String::as_str).collect();
    let going = protected::would_go(&protected::capture(&simulate)?);
    let argv = protected::autoremove_argv(false);
    let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
    run(&argv, None)?;
    if !going.is_empty() {
        println!("{}", t!("admin-tidied-away", list = going.join(", ")));
    }
    Ok(())
}

pub(super) fn installed_version(package: &str) -> Option<String> {
    #[cfg(test)]
    if let Some(answer) = super::stand_in::said(&["dpkg-query", "-W", package]) {
        return installed_version_in(&answer.unwrap_or_default());
    }
    let out = Command::new("dpkg-query").args(["-W", "-f=${db:Status-Abbrev}\t${Version}", package])
        .stderr(Stdio::null()).output().ok().filter(|o| o.status.success())?;
    installed_version_in(&String::from_utf8_lossy(&out.stdout))
}

fn installed_version_in(said: &str) -> Option<String> {
    let (state, version) = said.split_once('\t')?;
    let version = version.trim();
    let plain = !version.is_empty() && version.bytes().all(|b| b.is_ascii_alphanumeric() || b".+-~:".contains(&b));
    (state.starts_with("ii") && plain).then(|| version.to_string())
}

pub(super) fn app_remove(args: &[String]) -> Outcome {
    use crate::common::removal;
    let [package] = args else { return Err("usage: trindle-admin app-remove PACKAGE".into()) };
    if !crate::store::deb::valid_name(package) {
        return Err(t!("admin-package-invalid").into());
    }
    if !is_installed(package) {
        return Err(t!("admin-package-not-installed").into());
    }
    let sim = Command::new("apt-get").args(["-s", "-q", "purge", "--", package]).output().map_err(|e| t!("admin-tool-error", tool = "apt-get", why = e.to_string()))?;
    if !sim.status.success() {
        let err = String::from_utf8_lossy(&sim.stderr).trim().to_string();
        return Err(if err.is_empty() { t!("admin-tool-failed", tool = "apt-get") } else { err });
    }
    if let Some(why) = removal::refusal_of_all(package, &String::from_utf8_lossy(&sim.stdout), removal::facts) {
        return Err(why.into());
    }
    std::env::set_var("DEBIAN_FRONTEND", "noninteractive");
    apt_run(&["-q", "-y", "purge", "--", package])
}

fn is_installed(package: &str) -> bool {
    std::process::Command::new("dpkg").args(["-s", package]).stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null()).status().is_ok_and(|s| s.success())
}

pub fn valid_device_id(id: &str) -> bool {
    id.len() == 40 && id.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub(super) fn firmware_update(args: &[String]) -> Outcome {
    let [id] = args else { return Err("usage: trindle-admin firmware-update DEVICE_ID".into()) };
    if !valid_device_id(id) {
        return Err(t!("admin-device-invalid").into());
    }
    run(&["fwupdmgr", "update", id, "--assume-yes", "--no-reboot-check", "--no-unreported-check",
          "--no-metadata-check", "--no-remote-check", "--no-security-fix"], None)
}

pub(super) fn apt_install(what: &[&str]) -> Outcome {
    no_questions();
    let mut argv = vec!["-q", "-y", "--no-install-recommends", "-o", "Dpkg::Options::=--force-confdef",
                        "-o", "Dpkg::Options::=--force-confold", "install"];
    argv.extend(what);
    apt_run(&argv).or_else(|first| {

        apt_refresh().map_err(|lists| t!("admin-install-and-lists", first = &first, lists = lists))?;
        apt_run(&argv)
    })
}

#[cfg(test)]
mod tests {
    use super::valid_device_id;
    use crate::admin::stand_in::StandIn;

    #[test]
    fn the_version_to_keep_is_only_ever_a_whole_installed_one() {
        use super::installed_version_in as read;
        assert_eq!(read("ii \t1:0.9.0~beta1+local2"), Some("1:0.9.0~beta1+local2".into()));
        assert_eq!(read("ii \t1.0.16\n"), Some("1.0.16".into()));
        assert_eq!(read("rc \t1.0.16"), None, "removed, with only its settings left");
        assert_eq!(read("iU \t1.0.16"), None, "half put in");
        assert_eq!(read("ii \t"), None);
        assert_eq!(read("ii \t1.0 --allow-downgrades"), None, "nothing but a version reaches apt");
        assert_eq!(read(""), None);
    }

    #[test]
    fn device_ids() {
        assert!(valid_device_id("1e4fa9cea0b89e613370cf9657ffa0b6d3f37fec"));
        assert!(!valid_device_id("1E4FA9CEA0B89E613370CF9657FFA0B6D3F37FEC"));
        assert!(!valid_device_id("1e4fa9cea0b89e613370cf9657ffa0b6d3f37fe"));
        assert!(!valid_device_id("1e4fa9cea0b89e613370cf9657ffa0b6d3f37fec0"));

        assert!(!valid_device_id("b5f7dcc1-568c-50f8-a4dd-e39d1f93fda1"));
        assert!(!valid_device_id("--allow-older-000000000000000000000000000"));
        assert!(!valid_device_id("1e4fa9cea0b89e613370cf9657ffa0b6d3f37fe "));
        assert!(!valid_device_id("1e4fa9cea0b89e613370cf9657ffa0b6d3f37f\n"));
        assert!(!valid_device_id(""));
    }

    #[test]
    fn an_install_refused_twice_says_why_it_was_refused_first() {
        let s = StandIn::new("apt-install", |argv| {
            if argv.contains(&"install") { Err("E: Unable to locate package made-up-package".into()) } else { Err("E: The lists could not be fetched.".into()) }
        });
        assert_eq!(super::apt_install(&["made-up-package"]), Err("E: Unable to locate package made-up-package; E: The lists could not be fetched.".to_string()));
        assert_eq!(s.ran().len(), 2, "{:?}", s.ran());
        drop(s);

        let mut tries = 0;
        let s = StandIn::new("apt-install-again", move |argv| {
            tries += usize::from(argv.contains(&"install"));
            if argv.contains(&"install") && tries == 1 { Err("E: Unable to locate package made-up-package".into()) } else { Ok(()) }
        });
        assert_eq!(super::apt_install(&["made-up-package"]), Ok(()));
        assert_eq!(s.ran().len(), 3);
    }
}
