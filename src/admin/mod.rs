//! trindle-admin: privileged helper for Settings, run only via pkexec.

mod accounts;
mod dns;
mod firewall;
mod games;
mod home_room;
mod home_snapshots;
mod machine;
mod packages;
mod protected;
mod read_only;
#[cfg(test)]
mod kept;
mod shares;
mod sharing;
mod snapshots;
mod sources;
#[cfg(test)]
mod stand_in;
mod tailscale;
mod updates;

pub use accounts::valid_name_ok;
pub(crate) use accounts::MAX_FACE;
pub use machine::valid_hostname;
pub use packages::valid_device_id;

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::toolkit::t;

type Outcome = Result<(), String>;

fn run(argv: &[&str], stdin: Option<&str>) -> Outcome {
    let _open = if read_only::writes_system(argv) { Some(read_only::open()?) } else { None };
    #[cfg(test)]
    if let Some(answer) = stand_in::run(argv) {
        return answer;
    }
    let mut cmd = Command::new(argv[0]);
    cmd.args(&argv[1..]).stdout(Stdio::null()).stderr(Stdio::piped());
    if stdin.is_some() {
        cmd.stdin(Stdio::piped());
    }
    let mut child = cmd.spawn().map_err(|e| t!("admin-tool-error", tool = argv[0], why = e.to_string()))?;

    let mut given = Ok(());
    if let (Some(text), Some(mut pipe)) = (stdin, child.stdin.take()) {
        given = pipe.write_all(text.as_bytes());
    }
    let out = child.wait_with_output().map_err(|e| e.to_string())?;
    if out.status.success() {
        given.map_err(|e| t!("admin-tool-stopped", tool = argv[0], why = e.to_string()))
    } else {
        let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
        Err(if err.is_empty() { t!("admin-tool-failed", tool = argv[0]) } else { err })
    }
}

fn apt_run(args: &[&str]) -> Outcome {
    let _open = if read_only::writes_system(&[&["apt-get"][..], args].concat()) { Some(read_only::open()?) } else { None };
    #[cfg(test)]
    if let Some(answer) = stand_in::run(&[&["apt-get"][..], args].concat()) {
        return answer;
    }
    let mut apt = Command::new("apt-get");
    apt.args(["-o", "APT::Status-Fd=1"]).args(args);
    watched(apt, "apt-get", &mut |share| println!("{}", crate::common::filling::within_line(share)))
}

fn watched(mut tool: Command, name: &str, heard: &mut dyn FnMut(f64)) -> Outcome {
    let mut child = tool.stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().map_err(|e| t!("admin-tool-error", tool = name, why = e.to_string()))?;
    let err = child.stderr.take();
    let said = std::thread::spawn(move || {
        let mut bytes = Vec::new();
        if let Some(mut e) = err {
            // Only the words of a refusal are lost.
            let _ = e.read_to_end(&mut bytes);
        }
        String::from_utf8_lossy(&bytes).into_owned()
    });
    if let Some(out) = child.stdout.take() {
        let mut last = 0.0;
        let mut out = std::io::BufReader::new(out);
        let mut line = Vec::new();
        while read_line(&mut out, &mut line) {

            if let Some(share) = apt_share(&String::from_utf8_lossy(&line)).filter(|s| *s >= last + 0.01) {
                last = share;
                heard(share);
            }
        }
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    let err = said.join().unwrap_or_default().trim().to_string();
    if status.success() {
        Ok(())
    } else {
        Err(if err.is_empty() { t!("admin-tool-failed", tool = name) } else { err })
    }
}

fn read_line(out: &mut impl std::io::BufRead, line: &mut Vec<u8>) -> bool {
    line.clear();
    out.read_until(b'\n', line).is_ok_and(|n| n > 0)
}

fn apt_share(line: &str) -> Option<f64> {

    let mut parts = line.split(':');
    let kind = parts.next()?;
    parts.next()?;
    let percent: f64 = parts.find_map(|p| p.trim().parse().ok())?;
    let share = (percent / 100.0).clamp(0.0, 1.0);
    match kind {
        "dlstatus" => Some(share / 2.0),
        "pmstatus" => Some(0.5 + share / 2.0),
        _ => None,
    }
}

fn at(path: impl AsRef<Path>) -> PathBuf {
    #[cfg(test)]
    return stand_in::at(path.as_ref());
    #[cfg(not(test))]
    path.as_ref().to_path_buf()
}

fn gone(path: &Path) -> Outcome {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(t!("admin-file-not-removed", path = path.display().to_string(), why = e.to_string())),
    }
}

const KEPT_MOST: u64 = 4 << 20;

fn kept(path: &Path) -> Result<String, String> {
    crate::common::files::read_within(path, KEPT_MOST)
        .map_err(|e| t!("admin-file-not-read", why = e.to_string(), path = path.display().to_string()))
}

fn and_undone(why: String, undone: Outcome) -> String {
    match undone {
        Ok(()) => why,
        Err(e) => t!("admin-undone-failed", why = &why, problem = e),
    }
}

fn replace(path: &Path, data: &[u8], mode: u32) -> Outcome {
    use std::os::unix::fs::OpenOptionsExt;
    let tmp = path.with_extension("trindle-new");
    let mut f = std::fs::OpenOptions::new().write(true).create(true).truncate(true).mode(mode)
        .open(&tmp).map_err(|e| e.to_string())?;
    std::io::Write::write_all(&mut f, data).map_err(|e| e.to_string())?;
    drop(f);
    std::fs::rename(&tmp, path).map_err(|e| e.to_string())
}

fn reading(argv: &[&str]) -> String {
    #[cfg(test)]
    if let Some(answer) = stand_in::said(argv) {
        return answer.unwrap_or_default();
    }
    Command::new(argv[0]).args(&argv[1..]).env("LC_ALL", "C").stderr(Stdio::null()).output().map(|o| String::from_utf8_lossy(&o.stdout).into_owned()).unwrap_or_default()
}

pub fn main(args: Vec<String>) -> i32 {
    std::env::set_var("PATH", "/usr/sbin:/usr/bin:/sbin:/bin");
    // SAFETY: asks the kernel for this process's own effective user id. It

    if unsafe { libc::geteuid() } != 0 {
        eprintln!("trindle-admin must be run through pkexec.");
        return 1;
    }
    let Some(verb) = args.get(1) else {
        eprintln!("usage: trindle-admin add|password|admin|delete|face|locale|hostname ...");
        return 1;
    };

    let shown = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        shown(info);
        read_only::after_panic();
    }));
    let rest = &args[2..];
    let outcome = if read_only::CHANGES.contains(&verb.as_str()) { read_only::around(|| command(verb, rest)) } else { command(verb, rest) };
    match outcome {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("{e}");
            1
        }
    }
}

fn command(verb: &str, rest: &[String]) -> Outcome {
    match verb {
        "add" => accounts::add(rest),
        "password" => accounts::password(rest),
        "admin" => accounts::admin(rest),
        "delete" => accounts::delete(rest),
        "face" => accounts::face(rest),
        "locale" => machine::locale(rest),
        "locale-remove" => machine::locale_remove(rest),
        "hostname" => machine::hostname(rest),
        "apt-refresh" => packages::apt_refresh(),
        "apt-upgrade" => updates::apt_upgrade(),
        "apt-upgrade-debian" => updates::apt_upgrade_debian(),
        "apt-upgrade-trindle" => updates::apt_upgrade_trindle(),
        "source-add" => sources::add(rest),
        "source-custom" => sources::custom(rest),
        "source-remove" => sources::remove(rest),
        "charge-limit" => machine::charge_limit(rest),
        "update-checks" => machine::update_checks(rest),
        "security-updates" => machine::security_updates(rest),
        "charge-apply" => machine::charge_apply(),
        "radio-unblock" => machine::radio_unblock(rest),
        "firewall-sharing" => firewall::firewall_sharing(rest),
        "firewall-apply" => firewall::firewall_apply(),
        "firewall-plugin" => firewall::firewall_plugin(rest),
        "firewall-plugin-close" => firewall::firewall_plugin_close(rest),
        "firmware-update" => packages::firmware_update(rest),
        "tailscale-on" => tailscale::tailscale_on(),
        "tailscale-off" => tailscale::tailscale_off(),
        "app-remove" => packages::app_remove(rest),
        "dns-on" => dns::dns_on(),
        "dns-off" => dns::dns_off(),
        "dns-allow" => dns::dns_allow(rest),
        "dns-provider" => dns::dns_provider(rest),
        "plugin-catalogue-restart" => firewall::plugin_catalogue_restart(),
        other => system_command(other, rest),
    }
}

fn system_command(verb: &str, rest: &[String]) -> Outcome {
    match verb {
        "snapshot-create" => snapshots::create(rest),
        "snapshot-delete" => snapshots::delete(rest),
        "snapshot-restore" => snapshots::restore(rest),
        "snapshot-forget-set-aside" => snapshots::forget_set_aside(rest),
        "snapshot-finish-restore" => snapshots::finish_restore(rest),
        "home-snapshots-on" => home_snapshots::on(rest),
        "home-snapshots-off" => home_snapshots::off(rest),
        "home-snapshots-keep" => home_snapshots::keep(rest),
        "home-snapshots-clear" => home_snapshots::clear(rest),
        "home-snapshots-daily" => home_snapshots::daily(rest),
        "split-lock-slowing" => games::split_lock(rest),
        "games-drive" => games::drive(rest),
        "games-drive-forget" => games::forget(rest),
        "voice-install" => crate::voice::setup::install(),
        "voice-remove" => crate::voice::setup::remove(),
        "voice-tools-remove" => crate::voice::setup::remove_tools(),
        "smb-mount" => shares::mount(rest),
        "nfs-mount" => shares::nfs_mount(rest),
        "smb-unmount" => shares::unmount(rest),
        "smb-unmount-all" => shares::unmount_all(rest),
        "sharing-setup" => sharing::setup(rest),
        "sharing-apply" => sharing::apply(rest),
        "smb-password" => sharing::password(rest),
        "system-lock" => read_only::lock(rest),
        "system-read-only" => read_only::switch(rest),
        "system-run" => read_only::system_run(rest),
        "dpkg-pre-invoke" => read_only::dpkg_pre_invoke(rest),
        "dpkg-post-invoke" => read_only::dpkg_post_invoke(rest),
        other => Err(format!("trindle-admin: unknown command {other}")),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_tool_that_stops_before_it_has_its_input_is_not_called_done() {

        let much = "x".repeat(1 << 20);
        let said = super::run(&["true"], Some(&much)).unwrap_err();
        assert!(said.starts_with("true stopped before it was given everything"), "{said}");
        assert!(super::run(&["sh", "-c", "cat >/dev/null"], Some(&much)).is_ok());
    }

    #[test]
    fn a_line_that_is_not_text_does_not_end_the_listening() {
        let mut tool = std::process::Command::new("sh");
        tool.args(["-c", r"printf '\377\n'; printf 'pmstatus:x:50:y\n'"]);
        let mut heard = Vec::new();
        assert!(super::watched(tool, "sh", &mut |share| heard.push(share)).is_ok());
        assert_eq!(heard, [0.75], "what came after the line that is not text was still read");
    }

    #[test]
    fn apt_progress_is_half_fetching_and_half_putting_in() {
        let near = |got: Option<f64>, want: f64| got.is_some_and(|g| (g - want).abs() < 0.001);
        assert!(near(super::apt_share("dlstatus:3:50:Retrieving file 3 of 7"), 0.25));
        assert!(near(super::apt_share("pmstatus:libc6:amd64:20:Installing libc6 (amd64)"), 0.6));
        assert!(near(super::apt_share("pmstatus:dpkg-exec:60.0000:Running dpkg"), 0.8));
        assert!(near(super::apt_share("dlstatus:1:250:odd"), 0.5));
        assert_eq!(super::apt_share("Get:1 http://deb.debian.org trixie InRelease"), None);
        assert_eq!(super::apt_share("pmerror:foo:30:it broke"), None);
        assert_eq!(super::apt_share(""), None);
    }
}
