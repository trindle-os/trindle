//! The helper's steps for people: names, passwords, the administrator group, deleting a person and their picture.

use std::ffi::{CStr, CString};
use std::io::Read;

use super::{Outcome, at, gone, replace, run};
use crate::common::faces::FACES;
use crate::toolkit::t;

pub(crate) const MAX_FACE: usize = 2_000_000;
const PNG: &[u8] = b"\x89PNG\r\n\x1a\n";

pub fn valid_name_ok(name: &str) -> bool {
    valid_name(name).is_ok()
}

pub(super) fn valid_name(name: &str) -> Outcome {
    let first_ok = name.chars().next().is_some_and(|c| c.is_ascii_lowercase() || c == '_');
    let rest_ok = name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-');
    if !first_ok || !rest_ok {
        return Err(t!("admin-login-invalid").into());
    }
    if name.len() > 32 {
        return Err(t!("admin-login-too-long").into());
    }
    Ok(())
}

pub(super) fn name_of(uid: u32) -> Option<String> {
    // SAFETY: as above. pw_name is set for every entry getpwuid returns, and

    unsafe {
        let p = libc::getpwuid(uid);
        (!p.is_null()).then(|| CStr::from_ptr((*p).pw_name).to_string_lossy().to_string())
    }
}

fn group_members(group: &str) -> Vec<String> {
    let Ok(c) = CString::new(group) else { return Vec::new() };
    let mut out = Vec::new();
    // SAFETY: gr_mem is an array of string pointers ending in a null one, which

    unsafe {
        let g = libc::getgrnam(c.as_ptr());
        if g.is_null() {
            return out;
        }
        let mut m = (*g).gr_mem;
        while !m.is_null() && !(*m).is_null() {
            out.push(CStr::from_ptr(*m).to_string_lossy().to_string());
            m = m.add(1);
        }
    }
    out
}

pub(super) fn regular_user(name: &str) -> Outcome {
    let uid = crate::common::files::uid_of(name).ok_or(t!("admin-user-none", name = name))?;
    let min = std::fs::read_to_string("/etc/login.defs").unwrap_or_default().lines()
        .filter_map(|l| {
            let mut w = l.split_whitespace();
            (w.next() == Some("UID_MIN")).then(|| w.next().and_then(|v| v.parse::<u32>().ok())).flatten()
        })
        .next().unwrap_or(1000);
    if uid < min {
        return Err(t!("admin-user-system", name = name));
    }
    Ok(())
}

pub(super) fn read_password() -> Result<String, String> {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).map_err(|_| t!("admin-password-none"))?;
    let pw = line.strip_suffix('\n').unwrap_or(&line).to_string();
    if line.is_empty() {
        return Err(t!("admin-password-none").into());
    }
    if pw.chars().count() < 8 {
        return Err(t!("admin-password-short", n = 8));
    }
    if pw.contains(['\n', '\r']) {
        return Err(t!("admin-password-line-break").into());
    }
    Ok(pw)
}

fn set_password(name: &str, pw: &str) -> Outcome {
    run(&["chpasswd"], Some(&format!("{name}:{pw}\n")))
}

pub(super) fn add(args: &[String]) -> Outcome {
    let [name, full, admin] = args else { return Err("usage: trindle-admin add NAME 'FULL NAME' yes|no".into()) };
    valid_name(name)?;
    if crate::common::files::uid_of(name).is_some() {
        return Err(t!("admin-user-exists", name = name));
    }
    let full: String = full.chars().filter(|c| !matches!(c, ':' | ',' | '\n')).take(80).collect();
    let pw = read_password()?;
    run(&["adduser", "--disabled-password", "--comment", &full, name], None)?;
    set_password(name, &pw)?;
    if admin == "yes" {
        run(&["usermod", "-aG", "sudo", name], None)?;
    }
    Ok(())
}

pub(super) fn password(args: &[String]) -> Outcome {
    let [name] = args else { return Err("usage: trindle-admin password NAME".into()) };
    valid_name(name)?;
    regular_user(name)?;
    let pw = read_password()?;
    set_password(name, &pw)
}

pub(super) fn admin(args: &[String]) -> Outcome {
    let [name, yes] = args else { return Err("usage: trindle-admin admin NAME yes|no".into()) };
    valid_name(name)?;
    regular_user(name)?;
    if yes == "yes" {
        return run(&["usermod", "-aG", "sudo", name], None);
    }

    if !group_members("sudo").iter().any(|m| m != name) {
        return Err(t!("admin-user-only-admin", name = name));
    }
    run(&["gpasswd", "-d", name, "sudo"], None)
}

pub(super) fn delete(args: &[String]) -> Outcome {
    let [name] = args else { return Err("usage: trindle-admin delete NAME".into()) };
    valid_name(name)?;
    regular_user(name)?;
    let caller = std::env::var("PKEXEC_UID").ok().and_then(|u| u.parse().ok()).and_then(name_of);
    if caller.as_deref() == Some(name.as_str()) {
        return Err(t!("admin-delete-self").into());
    }
    delete_checked(name)
}

fn delete_checked(name: &str) -> Outcome {
    // Someone not logged in has nothing to end.
    let _ = run(&["loginctl", "terminate-user", name], None);
    run(&["deluser", "--remove-home", name], None)?;
    gone(&at(FACES).join(format!("{name}.png")))
        .map_err(|e| t!("admin-delete-picture-kept", name = name, why = e))
}

pub(super) fn face(args: &[String]) -> Outcome {
    let (name, remove) = match args {
        [name] => (name, false),
        [name, r] if r == "remove" => (name, true),
        _ => return Err("usage: trindle-admin face NAME [remove]".into()),
    };
    valid_name(name)?;
    regular_user(name)?;
    set_face(name, remove, &mut std::io::stdin())
}

fn set_face(name: &str, remove: bool, picture: &mut dyn Read) -> Outcome {
    let path = at(FACES).join(format!("{name}.png"));
    if remove {
        return gone(&path);
    }
    let mut data = Vec::new();
    picture.take(MAX_FACE as u64 + 1).read_to_end(&mut data).map_err(|e| e.to_string())?;
    if data.len() > MAX_FACE {
        return Err(t!("admin-picture-large").into());
    }
    if !data.starts_with(PNG) {
        return Err(t!("admin-picture-not-ours").into());
    }
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::create_dir_all(at(FACES)).map_err(|e| e.to_string())?;

        std::fs::set_permissions(at(FACES), std::fs::Permissions::from_mode(0o755))
            .map_err(|e| t!("admin-picture-folder", why = e.to_string()))?;
    }
    replace(&path, &data, 0o644)
}

#[cfg(test)]
mod tests {
    use super::valid_name;
    use crate::admin::stand_in::StandIn;

    #[test]
    fn names() {
        assert!(valid_name("morgan").is_ok());
        assert!(valid_name("_x-1").is_ok());
        assert!(valid_name("Morgan").is_err());
        assert!(valid_name("1abc").is_err());
        assert!(valid_name("a\nroot").is_err());
        assert!(valid_name("").is_err());
        assert!(valid_name(&"a".repeat(33)).is_err());
    }

    #[test]
    fn a_picture_that_cannot_be_taken_away_is_said() {
        let s = StandIn::all_well("face");
        s.stuck(&format!("{}/john.png", super::FACES));
        let said = super::set_face("john", true, &mut std::io::empty()).unwrap_err();
        assert!(said.contains("could not be removed"), "{said}");

        assert!(super::set_face("jane", true, &mut std::io::empty()).is_ok());
        let said = super::delete_checked("john").unwrap_err();
        assert!(said.starts_with("john was deleted, but their picture is still kept for the login screen"), "{said}");
        assert!(s.ran().contains(&"deluser --remove-home john".to_string()));
    }
}
