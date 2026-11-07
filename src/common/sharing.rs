//! Sharing the person's own folders with other computers on the home network, through Samba's shares for ordinary people ("usershares").

use std::path::{Path, PathBuf};

pub const USERSHARES: &str = "/var/lib/samba/usershares";
pub const GROUP: &str = "sambashare";
pub const SMB_CONF: &str = "/etc/samba/smb.conf";
pub const SMBD: &str = "/usr/sbin/smbd";
pub const SERVICE: &str = "smbd.service";
pub const PACKAGES: &[&str] = &["samba"];
pub const SET_UP: &str = "/etc/trindle/sharing-set-up";
pub const OPEN: &str = "/run/trindle-sharing";

const NEEDED: [(&str, &str); 5] = [
    ("usershare path", USERSHARES),
    ("usershare max shares", "100"),
    ("usershare allow guests", "yes"),
    ("usershare owner only", "yes"),
    ("map to guest", "bad user"),
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shared {
    pub name: String,
    pub path: PathBuf,
    pub write: bool,
    pub guests: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Problem {
    NoSamba,
    NotSetUp,
    Other(String),
}

pub fn installed() -> bool {
    Path::new(SMBD).exists()
}

pub fn delete_argv(name: &str) -> Vec<String> {
    vec!["net".into(), "usershare".into(), "delete".into(), name.to_string()]
}

pub fn info_argv() -> Vec<String> {
    vec!["net".into(), "usershare".into(), "info".into()]
}

pub fn parse_info(text: &str) -> Vec<Shared> {
    let mut out: Vec<Shared> = Vec::new();
    for line in text.lines().map(str::trim) {
        if let Some(name) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) {
            out.push(Shared { name: name.to_string(), path: PathBuf::new(), write: false, guests: false });
            continue;
        }
        let (Some(s), Some((key, value))) = (out.last_mut(), line.split_once('=')) else { continue };
        match key.trim() {
            "path" => s.path = PathBuf::from(value),
            "usershare_acl" => s.write = value.split(',').any(|e| e.rsplit(':').next().is_some_and(|r| matches!(r.trim(), "F" | "C" | "f" | "c"))),
            "guest_ok" => s.guests = value.trim().eq_ignore_ascii_case("y"),
            _ => {}
        }
    }
    out.retain(|s| !s.name.is_empty() && s.path.is_absolute());
    out
}

pub fn problem_of(err: &str) -> Problem {
    let l = err.to_lowercase();
    if l.contains("usershares are currently disabled") || l.contains("cannot open usershare directory")
        || l.contains("permission denied") || (l.contains("group") && l.contains("does not exist")) {
        return Problem::NotSetUp;
    }
    Problem::Other(words_of(err))
}

pub fn words_of(err: &str) -> String {
    let l = err.to_lowercase();
    if l.contains("directories we own") {
        return "Only folders that belong to you can be shared.".into();
    }
    if l.contains("valid system user name") {
        return "That name is someone's login on this computer, so a share cannot have it. Choose another.".into();
    }
    if l.contains("not a valid share name") || l.contains("invalid char") {
        return "A share's name cannot hold any of % < > * ? | / \\ + = ; : \" or a comma.".into();
    }
    if l.contains("guest") && (l.contains("not allowed") || l.contains("allow guests")) {
        return "This computer's Samba settings do not allow shares without a password.".into();
    }
    if l.contains("maximum number") {
        return "This computer already shares as many folders as it allows.".into();
    }
    if l.contains("is not a directory") || l.contains("cannot stat") || l.contains("no such file") {
        return "That folder is no longer there.".into();
    }
    let last = err.trim().lines().last().unwrap_or("").trim();
    let last = last.strip_prefix("net usershare add:").or_else(|| last.strip_prefix("net usershare delete:")).unwrap_or(last).trim();
    if last.is_empty() { "Samba did not say why.".into() } else { format!("Samba said: {last}") }
}

pub fn quote(word: &str) -> String {
    format!("'{}'", word.replace('\'', "'\\''"))
}

pub fn in_group(argv: Vec<String>, through_sg: bool) -> Vec<String> {
    if !through_sg {
        return argv;
    }
    let line = argv.iter().map(|w| quote(w)).collect::<Vec<_>>().join(" ");
    vec!["sg".into(), GROUP.into(), "-c".into(), line]
}

#[cfg(not(test))]
fn group_entry(user: &str) -> Option<(u32, bool)> {
    let name = std::ffi::CString::new(GROUP).ok()?;
    // SAFETY: getgrnam's entry is read at once, before anything else touches
    // the group database, and only after it is known not to be null. gr_mem
    // is a list of strings ending in a null pointer, which stops the walk.
    unsafe {
        let g = libc::getgrnam(name.as_ptr());
        if g.is_null() {
            return None;
        }
        let mut listed = false;
        let mut m = (*g).gr_mem;
        while !m.is_null() && !(*m).is_null() {
            listed |= std::ffi::CStr::from_ptr(*m).to_bytes() == user.as_bytes();
            m = m.add(1);
        }
        Some(((*g).gr_gid, listed))
    }
}

#[cfg(not(test))]
fn holds(gid: u32) -> bool {
    // SAFETY: asks for the count first with no list, then fills a list of
    // that size; both calls only write into memory given to them.
    unsafe {
        if libc::getegid() == gid {
            return true;
        }
        let n = libc::getgroups(0, std::ptr::null_mut());
        if n <= 0 {
            return false;
        }
        let mut list = vec![0 as libc::gid_t; n as usize];
        let got = libc::getgroups(n, list.as_mut_ptr());
        got > 0 && list[..got as usize].contains(&gid)
    }
}

#[cfg(not(test))]
pub fn through_sg() -> bool {
    match group_entry(&crate::common::files::user()) {
        Some((gid, listed)) => listed && !holds(gid),
        None => false,
    }
}

pub fn conf_ready(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let section = |l: &str| l.trim().starts_with('[');
    let start = lines.iter().position(|l| l.trim().eq_ignore_ascii_case("[global]"));
    let have = |key: &str| -> bool {
        let Some(s) = start else { return false };
        lines[s + 1..].iter().take_while(|l| !section(l)).any(|l| {
            let t = l.trim();
            !t.starts_with(['#', ';']) && t.split_once('=').is_some_and(|(k, _)| same_key(k, key))
        })
    };
    let missing: Vec<String> = NEEDED.iter().filter(|(k, _)| !have(k)).map(|(k, v)| format!("   {k} = {v}")).collect();
    let mut out: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
    match start {
        Some(s) => {
            for (i, m) in missing.into_iter().enumerate() {
                out.insert(s + 1 + i, m);
            }
        }
        None => {
            let mut head = vec!["[global]".to_string()];
            head.extend(missing);
            head.push(String::new());
            out.splice(0..0, head);
        }
    }
    out.join("\n") + "\n"
}

fn same_key(a: &str, b: &str) -> bool {
    let squash = |s: &str| s.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase();
    squash(a) == squash(b)
}

pub fn workgroup(conf: &str) -> String {
    let mut global = false;
    for l in conf.lines().map(str::trim) {
        if l.starts_with('[') {
            global = l.eq_ignore_ascii_case("[global]");
        } else if global && !l.starts_with(['#', ';']) {
            if let Some((k, v)) = l.split_once('=') {
                if same_key(k, "workgroup") && !v.trim().is_empty() {
                    return v.trim().to_uppercase();
                }
            }
        }
    }
    "WORKGROUP".into()
}

pub fn hand_made(conf: &str) -> bool {
    conf.lines().map(str::trim).filter_map(|l| l.strip_prefix('[').and_then(|l| l.strip_suffix(']')))
        .any(|s| !["global", "printers", "print$", "homes"].contains(&s.trim().to_lowercase().as_str()))
}

pub fn anything_shared(dir: &Path, conf: &str) -> bool {
    let usershare = std::fs::read_dir(dir).into_iter().flatten().flatten()
        .any(|e| !e.file_name().to_string_lossy().starts_with('.') && e.file_type().is_ok_and(|t| t.is_file()));
    usershare || hand_made(conf)
}

pub fn shared_now() -> bool {
    anything_shared(Path::new(USERSHARES), &std::fs::read_to_string(SMB_CONF).unwrap_or_default())
}

pub fn host_name() -> String {
    let mut buf = [0u8; 256];
    // SAFETY: gethostname writes at most the length given into buf.
    let r = unsafe { libc::gethostname(buf.as_mut_ptr().cast(), buf.len()) };
    if r != 0 {
        return String::new();
    }
    let end = buf.iter().position(|b| *b == 0).unwrap_or(buf.len());
    let full = String::from_utf8_lossy(&buf[..end]).into_owned();
    full.split('.').next().unwrap_or("").to_string()
}

#[cfg(not(test))]
mod real {
    use super::*;
    use crate::common::run::captured;

    fn net(argv: Vec<String>) -> (bool, String, String) {
        captured(&in_group(argv, through_sg()), None)
    }

    pub fn list() -> Result<Vec<Shared>, Problem> {
        if !crate::common::run::have("net") {
            return Err(if installed() { Problem::NotSetUp } else { Problem::NoSamba });
        }
        let (ok, out, err) = net(info_argv());
        if ok { Ok(parse_info(&out)) } else { Err(problem_of(&err)) }
    }

    pub fn remove(name: &str) -> Result<(), String> {
        let (ok, _, err) = net(delete_argv(name));
        if ok { Ok(()) } else { Err(words_of(&err)) }
    }

}

#[cfg(not(test))]
pub use real::*;

#[cfg(test)]
pub mod pretend {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    pub struct Computer {
        pub installed: bool,
        pub set_up: bool,
        pub shares: Vec<Shared>,
    }

    thread_local! {
        pub static HERE: RefCell<Computer> = RefCell::new(Computer::default());
    }

    pub fn with<T>(f: impl FnOnce(&mut Computer) -> T) -> T {
        HERE.with(|c| f(&mut c.borrow_mut()))
    }

    pub fn list() -> Result<Vec<Shared>, Problem> {
        with(|c| {
            match (c.installed, c.set_up) {
                (false, _) => Err(Problem::NoSamba),
                (true, false) => Err(Problem::NotSetUp),
                _ => Ok(c.shares.clone()),
            }
        })
    }

    pub fn remove(name: &str) -> Result<(), String> {
        with(|c| {
            let before = c.shares.len();
            c.shares.retain(|x| !x.name.eq_ignore_ascii_case(name));
            if c.shares.len() < before { Ok(()) } else { Err(words_of("net usershare delete: cannot delete share")) }
        })
    }

}

#[cfg(test)]
pub use pretend::{list, remove};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usershare_command_lines() {
        assert_eq!(delete_argv("photos"), ["net", "usershare", "delete", "photos"]);
        assert_eq!(info_argv(), ["net", "usershare", "info"]);
        let sg = in_group(vec!["net".into(), "usershare".into(), "add".into(), "Mine".into(), "/home/john/It's mine".into(), String::new()], true);
        assert_eq!(sg[..3], ["sg", "sambashare", "-c"]);
        assert_eq!(sg[3], "'net' 'usershare' 'add' 'Mine' '/home/john/It'\\''s mine' ''");
        assert_eq!(in_group(delete_argv("a"), false), delete_argv("a"));
    }

    #[test]
    fn shares_are_read_from_net() {
        let got = parse_info("[photos]\npath=/home/john/Photos\ncomment=\nusershare_acl=Everyone:R,\nguest_ok=n\n\n\
                              [music]\npath=/home/john/Music\ncomment=\nusershare_acl=Everyone:F,\nguest_ok=y\n\n[odd]\ncomment=\n");
        assert_eq!(got, [Shared { name: "photos".into(), path: "/home/john/Photos".into(), write: false, guests: false },
                         Shared { name: "music".into(), path: "/home/john/Music".into(), write: true, guests: true }]);
        assert!(parse_info("").is_empty());
    }

    #[test]
    fn nets_complaints_are_said_plainly() {
        assert_eq!(problem_of("net usershare: cannot open usershare directory /var/lib/samba/usershares. Error Permission denied"), Problem::NotSetUp);
        assert_eq!(problem_of("net usershare: usershares are currently disabled"), Problem::NotSetUp);
        assert_eq!(words_of("net usershare add: cannot share path /etc as we are restricted to only sharing directories we own."),
                   "Only folders that belong to you can be shared.");
        assert!(words_of("net usershare add: share name john is already a valid system user name.").contains("login"));
        assert!(words_of("net usershare add: guest_ok=y requested but the \"usershare allow guests\" parameter is not enabled by this server.")
            .contains("without a password"));
        assert_eq!(words_of("net usershare add: something new\n"), "Samba said: something new");
        assert!(!words_of("x").contains('\u{2014}'));
    }

    #[test]
    fn sambas_settings_are_added_only_where_missing() {
        let debian = "# Samba's own\n[global]\n   workgroup = HOME\n   usershare allow guests = no\n;  usershare max shares = 0\n\n[printers]\n   comment = All Printers\n";
        let got = conf_ready(debian);
        assert!(got.contains("usershare allow guests = no"), "the person's own setting stays");
        assert!(!got.contains("usershare allow guests = yes"));
        assert!(got.contains("usershare path = /var/lib/samba/usershares"));
        assert!(got.contains("   usershare max shares = 100"));
        assert!(got.contains("map to guest = bad user"));
        assert!(got.find("usershare path").unwrap() < got.find("[printers]").unwrap());
        assert_eq!(conf_ready(&got), got, "nothing is added twice");
        let fresh = conf_ready("");
        assert!(fresh.starts_with("[global]\n   usershare path"));
        assert_eq!(workgroup(debian), "HOME");
        assert_eq!(workgroup(""), "WORKGROUP");
    }

    #[test]
    fn something_is_shared_when_a_usershare_or_a_hand_made_share_is_there() {
        let dir = std::env::temp_dir().join(format!("trindle-usershares-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        assert!(!anything_shared(&dir, "[global]\n[printers]\n[print$]\n"));
        assert!(anything_shared(&dir, "[global]\n[Media]\npath = /srv/media\n"));
        std::fs::write(dir.join(".lock"), "").unwrap();
        assert!(!anything_shared(&dir, ""));
        std::fs::write(dir.join("photos"), "path=/home/john/Photos\n").unwrap();
        assert!(anything_shared(&dir, ""));
        assert!(!anything_shared(Path::new("/nonexistent/usershares"), ""));
        std::fs::remove_dir_all(&dir).unwrap();
    }

}
