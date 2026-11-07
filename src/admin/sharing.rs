//! Sharing the person's own folders on the home network: root's part.
//!
//! `sharing-setup` is run the first time anyone shares a folder in Files.
//! It installs Debian's samba package, sets up Samba's shares for ordinary
//! people (their folder, the sambashare group, guests allowed), puts the
//! person who asked in that group, and keeps Samba's own servers from
//! starting by themselves. `smb-password` sets the asking person's own
//! Samba password, read from standard input. `sharing-apply`, run by
//! trindle-sharing.path whenever a share comes or goes and once at every
//! start, opens or shuts the firewall's door for shared folders and starts
//! or stops Samba's server, so nothing of Samba runs while nothing is shared.

use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use crate::common::{firewall, sharing};

use super::{at, gone, kept, replace, run, Outcome};
use crate::toolkit::t;

/// These take nothing on their command line: who they are for comes from
/// pkexec, and a password from standard input.
fn nothing_more(verb: &str, args: &[String]) -> Outcome {
    if args.is_empty() { Ok(()) } else { Err(format!("usage: trindle-admin {verb}")) }
}

/// The group's number, made first when Samba did not make it.
fn group_id() -> Result<u32, String> {
    let find = || {
        let name = std::ffi::CString::new(sharing::GROUP).ok()?;
        // SAFETY: getgrnam's entry is read at once, before anything else
        // touches the group database, and only when it is not null.
        unsafe {
            let g = libc::getgrnam(name.as_ptr());
            (!g.is_null()).then(|| (*g).gr_gid)
        }
    };
    if let Some(g) = find() {
        return Ok(g);
    }
    run(&["groupadd", "--system", sharing::GROUP], None)?;
    find().ok_or_else(|| t!("admin-sharing-group").into())
}

/// Samba's folder for people's shares: root's, the group's to write in,
/// each person's files their own (the sticky bit).
fn usershare_folder(gid: u32) -> Outcome {
    let dir = Path::new(sharing::USERSHARES);
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let c = std::ffi::CString::new(sharing::USERSHARES).map_err(|e| e.to_string())?;
    // SAFETY: a path with no nul in it, and plain numbers.
    if unsafe { libc::chown(c.as_ptr(), 0, gid) } != 0 {
        return Err(format!("{}: {}", sharing::USERSHARES, std::io::Error::last_os_error()));
    }
    std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o1770)).map_err(|e| e.to_string())
}

pub fn setup(args: &[String]) -> Outcome {
    nothing_more("sharing-setup", args)?;
    let me = super::shares::caller()?;
    if !sharing::installed() {
        println!("{}", crate::common::filling::step_line(1, 2, t!("admin-step-fetching-samba")));
        super::packages::apt_install(sharing::PACKAGES)?;
        println!("{}", crate::common::filling::step_line(2, 2, t!("admin-step-setting-up-sharing")));
    }
    // Samba's old name service and its domain controller are never wanted;
    // its file server starts only while something is shared.
    for unit in ["nmbd.service", "samba-ad-dc.service"] {
        // One that is not on this computer cannot be switched off, and
        // one that is and stays on is still behind the firewall.
        let _ = run(&["systemctl", "disable", "--now", unit], None);
    }
    // Left to start by itself, it is stopped again at every start while
    // nothing is shared (`apply`).
    let _ = run(&["systemctl", "disable", sharing::SERVICE], None);
    let gid = group_id()?;
    usershare_folder(gid)?;
    conf_made_ready(&at(sharing::SMB_CONF))?;
    run(&["usermod", "-aG", sharing::GROUP, &me.name], None)?;
    std::fs::create_dir_all("/etc/trindle").map_err(|e| e.to_string())?;
    replace(Path::new(sharing::SET_UP), b"", 0o644)?;
    apply(&[])
}

/// Samba's settings with what sharing by ordinary people needs. They are
/// read honestly first: settings that cannot be read, a file someone wrote
/// in another alphabet's old encoding among them, are never written over.
fn conf_made_ready(file: &Path) -> Outcome {
    let conf = kept(file)?;
    let ready = sharing::conf_ready(&conf);
    if ready != conf {
        if let Some(dir) = file.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        replace(file, ready.as_bytes(), 0o644)?;
    }
    Ok(())
}

/// The person's own Samba password, given twice on smbpasswd's input as it
/// asks, never on a command line.
pub fn password(args: &[String]) -> Outcome {
    nothing_more("smb-password", args)?;
    let me = super::shares::caller()?;
    if !sharing::installed() {
        return Err(t!("admin-sharing-not-set-up").into());
    }
    let pw = zeroize::Zeroizing::new(super::accounts::read_password()?);
    let twice = zeroize::Zeroizing::new(format!("{}\n{}\n", pw.as_str(), pw.as_str()));
    run(&["smbpasswd", "-s", "-a", &me.name], Some(twice.as_str()))?;
    run(&["smbpasswd", "-e", &me.name], None)
}

/// The firewall, Samba's server and the mark for trindle-wsdd, as what is
/// shared and the sharing switch say.
pub fn apply(args: &[String]) -> Outcome {
    nothing_more("sharing-apply", args)?;
    let shared = sharing::shared_now();
    super::firewall::firewall_apply()?;
    if shared && firewall::sharing() {
        replace(&at(sharing::OPEN), b"", 0o644)?;
    } else {
        gone(&at(sharing::OPEN))
            .map_err(|e| t!("admin-sharing-still-shown", why = e))?;
    }
    // A Samba set up by hand is left to run as its owner set it.
    if sharing::installed() && Path::new(sharing::SET_UP).exists() {
        let how = if shared { "start" } else { "stop" };
        run(&["systemctl", "--no-block", how, sharing::SERVICE], None)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn nothing_but_the_verb_is_taken() {
        assert!(nothing_more("smb-password", &[]).is_ok());
        // A user name or a password on the command line is refused: who it
        // is for comes from pkexec, and the password from standard input.
        assert!(nothing_more("smb-password", &strings(&["root"])).is_err());
        assert!(nothing_more("smb-password", &strings(&["john", "secret123"])).is_err());
        assert!(nothing_more("sharing-setup", &strings(&["--now"])).is_err());
        assert_eq!(nothing_more("sharing-apply", &strings(&["x"])).unwrap_err(), "usage: trindle-admin sharing-apply");
    }

    #[test]
    fn each_verb_checks_its_arguments_before_anything_else() {
        // These fail on the arguments alone, before asking who called or
        // touching the computer.
        assert!(setup(&strings(&["extra"])).unwrap_err().starts_with("usage:"));
        assert!(password(&strings(&["john"])).unwrap_err().starts_with("usage:"));
        assert!(apply(&strings(&["on"])).unwrap_err().starts_with("usage:"));
    }

    #[test]
    fn sambas_settings_that_cannot_be_read_are_not_written_over() {
        let s = crate::admin::stand_in::StandIn::all_well("smb-conf");
        let was: &[u8] = b"[global]\n   workgroup = CAF\xc9\n";
        s.write(sharing::SMB_CONF, was);
        let file = s.path(sharing::SMB_CONF);
        assert!(conf_made_ready(&file).unwrap_err().starts_with("Nothing was changed, because"));
        assert_eq!(std::fs::read(&file).unwrap(), was);
        // Settings that read are made ready, and one not there yet is made.
        s.write(sharing::SMB_CONF, b"[global]\n   workgroup = HOME\n");
        assert!(conf_made_ready(&file).is_ok());
        assert!(std::fs::read_to_string(&file).unwrap().contains("workgroup = HOME"));
    }

    #[test]
    fn a_computer_still_shown_on_the_network_is_said() {
        let s = crate::admin::stand_in::StandIn::all_well("sharing-open");
        s.stuck(sharing::OPEN);
        // Shared or not, the mark can be neither made nor taken away.
        assert!(apply(&[]).is_err());
    }
}
