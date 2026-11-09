//! Mounting and unmounting Windows and Samba shares for the person who
//! asked, with mount.cifs from Debian's cifs-utils. Only ever a cifs mount,
//! only ever in that person's own folder under /run/media, which nobody
//! else can enter, owned by them. The login comes on standard input and
//! reaches mount.cifs in a credentials file in a folder only root can
//! enter, on /run, which lives in memory; the file is removed as soon as
//! mount.cifs has read it, whatever happened.

use std::ffi::CString;
use std::io::{Read, Write};
use std::os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::common::smb::{self, Login};
use crate::toolkit::t;

type Outcome = Result<(), String>;

const PRIVATE: &str = "/run/trindle-admin";

const MAX_INPUT: u64 = 2048;

pub(super) struct Person {
    pub(super) name: String,
    pub(super) uid: u32,
    pub(super) gid: u32,
}

fn person_by_uid(uid: u32) -> Result<Person, String> {
    let name = super::accounts::name_of(uid).ok_or(t!("admin-caller-unknown"))?;
    super::accounts::valid_name(&name)?;
    super::accounts::regular_user(&name)?;
    // SAFETY: getpwuid's entry is read at once, before anything else

    let gid = unsafe {
        let p = libc::getpwuid(uid);
        if p.is_null() {
            return Err(t!("admin-caller-unknown").into());
        }
        (*p).pw_gid
    };
    Ok(Person { name, uid, gid })
}

pub(super) fn caller() -> Result<Person, String> {
    let uid = std::env::var("PKEXEC_UID").ok().and_then(|u| u.parse().ok()).ok_or(t!("admin-pkexec-only"))?;
    person_by_uid(uid)
}

fn mount_args(args: &[String]) -> Result<(String, String, Option<std::net::IpAddr>), String> {
    let (server, share, address) = match args {
        [server, share] => (server, share, None),
        [server, share, address] => (server, share, Some(address)),
        _ => return Err("usage: trindle-admin smb-mount SERVER SHARE [ADDRESS]".into()),
    };
    if !smb::valid_server(server) {
        return Err(t!("admin-share-computer-invalid").into());
    }
    if !smb::valid_share(share) {
        return Err(t!("admin-share-name-invalid").into());
    }
    let ip = match address {
        Some(a) => Some(smb::valid_address(a).ok_or(t!("admin-share-address-invalid"))?),
        None => None,
    };
    Ok((server.clone(), share.clone(), ip))
}

fn nfs_args(args: &[String]) -> Result<(String, String), String> {
    let [server, export] = args else { return Err("usage: trindle-admin nfs-mount SERVER EXPORT".into()) };
    if !smb::valid_server(server) {
        return Err(t!("admin-share-computer-invalid").into());
    }
    if !smb::valid_export(export) {
        return Err(t!("admin-share-path-invalid").into());
    }
    Ok((server.clone(), export.clone()))
}

fn unmount_target(args: &[String], user: &str, mounts: &str) -> Result<PathBuf, String> {
    let [at] = args else { return Err("usage: trindle-admin smb-unmount FOLDER".into()) };
    let at = PathBuf::from(at);
    if !smb::in_user_folder(&at, user) {
        return Err(t!("admin-share-not-yours").into());
    }
    if !smb::mounted(mounts, &smb::user_folder(user)).iter().any(|m| m.at == at) {
        return Err(t!("admin-share-not-open").into());
    }
    Ok(at)
}

fn user_folder(p: &Person) -> Result<PathBuf, String> {
    let media = Path::new(smb::MEDIA);
    let mine = smb::user_folder(&p.name);
    let mut dir = std::fs::DirBuilder::new();
    dir.mode(0o755);
    for (at, mode, gid) in [(media, 0o755, 0), (mine.as_path(), 0o750, p.gid)] {
        match std::fs::symlink_metadata(at) {
            Ok(m) if m.is_dir() && !m.file_type().is_symlink() && m.uid() == 0 => {}
            Ok(_) => return Err(t!("admin-share-folder-changed", path = at.display().to_string())),
            Err(_) => dir.create(at).map_err(|e| t!("admin-path-error", path = at.display().to_string(), why = e.to_string()))?,
        }
        chown(at, 0, gid)?;
        std::fs::set_permissions(at, std::fs::Permissions::from_mode(mode)).map_err(|e| e.to_string())?;
    }
    Ok(mine)
}

fn chown(at: &Path, uid: u32, gid: u32) -> Outcome {
    let c = CString::new(at.as_os_str().as_encoded_bytes()).map_err(|e| e.to_string())?;
    // SAFETY: lchown reads the NUL-terminated path and changes nothing else;

    if unsafe { libc::lchown(c.as_ptr(), uid, gid) } != 0 {
        return Err(format!("{}: {}", at.display(), std::io::Error::last_os_error()));
    }
    Ok(())
}

struct Credentials(PathBuf);

impl Credentials {
    fn write(login: &Login) -> Result<Credentials, String> {
        let mut dir = std::fs::DirBuilder::new();
        dir.mode(0o700);
        let private = Path::new(PRIVATE);
        match std::fs::symlink_metadata(private) {
            Ok(m) if m.is_dir() && !m.file_type().is_symlink() && m.uid() == 0 => {}
            Ok(_) => return Err(t!("admin-share-private-changed", path = PRIVATE)),
            Err(_) => dir.create(private).map_err(|e| t!("admin-path-error", path = PRIVATE, why = e.to_string()))?,
        }
        std::fs::set_permissions(private, std::fs::Permissions::from_mode(0o700)).map_err(|e| e.to_string())?;
        let path = private.join(format!("cifs-{}", std::process::id()));
        // Were one still there, making it new fails next, and that is said.
        let _ = std::fs::remove_file(&path);
        let mut f = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600).custom_flags(libc::O_NOFOLLOW)
            .open(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let kept = Credentials(path);
        f.write_all(login.credentials().as_bytes()).map_err(|e| e.to_string())?;
        Ok(kept)
    }
}

impl Drop for Credentials {
    fn drop(&mut self) {

        // in /run, which is emptied when the computer is switched off.
        let _ = std::fs::remove_file(&self.0);
    }
}

fn fresh_folder(mine: &Path, server: &str, share: &str) -> Result<PathBuf, String> {
    fresh_named(mine, smb::folder_name(server, share))
}

fn fresh_named(mine: &Path, base: String) -> Result<PathBuf, String> {
    let mut dir = std::fs::DirBuilder::new();
    dir.mode(0o700);
    for n in 1..100 {
        let name = if n == 1 { base.clone() } else { format!("{base} ({n})") };
        let at = mine.join(name);
        match dir.create(&at) {
            Ok(()) => return Ok(at),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(t!("admin-path-error", path = at.display().to_string(), why = e.to_string())),
        }
    }
    Err(t!("admin-share-too-many").into())
}

fn quiet(argv: &[String]) -> Result<(), String> {
    let out = Command::new(&argv[0]).args(&argv[1..]).env_clear().env("PATH", "/usr/sbin:/usr/bin:/sbin:/bin").env("LC_ALL", "C")
        .stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).output()
        .map_err(|e| if e.kind() == std::io::ErrorKind::NotFound {
            let package = if argv[0] == "mount.nfs" { "nfs-common" } else { "cifs-utils" };
            t!("admin-tool-missing", tool = &argv[0], package = package)
        } else {
            t!("admin-tool-error", tool = &argv[0], why = e.to_string())
        })?;
    if out.status.success() {
        return Ok(());
    }
    let said = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    Err(if said.trim().is_empty() { t!("admin-tool-failed", tool = &argv[0]) } else { said.trim().to_string() })
}

fn read_login() -> Result<Option<Login>, String> {
    let mut text = zeroize::Zeroizing::new(String::new());
    std::io::stdin().take(MAX_INPUT).read_to_string(&mut text).map_err(|_| t!("admin-login-unreadable"))?;
    Login::read(&text)
}

const MOUNTS: &str = "/proc/self/mounts";

fn mounts_in(list: &Path) -> Result<String, String> {
    std::fs::read_to_string(list).map_err(|e| t!("admin-mounts-unreadable", why = e.to_string()))
}

pub fn mount(args: &[String]) -> Outcome {
    let (server, share, ip) = mount_args(args)?;
    let p = caller()?;
    let login = read_login()?;
    let mine = user_folder(&p)?;
    let mounts = mounts_in(Path::new(MOUNTS))?;

    if let Some(m) = smb::mounted(&mounts, &mine).into_iter()
        .find(|m| m.server.eq_ignore_ascii_case(&server) && m.share.eq_ignore_ascii_case(&share)) {
        println!("{}", m.at.display());
        return Ok(());
    }
    let at = fresh_folder(&mine, &server, &share)?;
    let outcome = {
        let creds = match &login {
            Some(l) => Some(Credentials::write(l)?),
            None => None,
        };
        let options = smb::options(p.uid, p.gid, ip, creds.as_ref().map(|c| c.0.as_path()));
        quiet(&smb::mount_argv(&server, &share, &at, &options))

    };
    if let Err(e) = outcome {
        // An empty folder of root's, which shows as no place anywhere.
        let _ = std::fs::remove_dir(&at);
        return Err(e);
    }

    // is open as asked whether or not this one could be set.
    let _ = quiet(&["systemctl".into(), "start".into(), smb::tidy_unit(p.uid)]);
    println!("{}", at.display());
    Ok(())
}

pub fn nfs_mount(args: &[String]) -> Outcome {
    let (server, export) = nfs_args(args)?;
    let p = caller()?;
    let mine = user_folder(&p)?;
    let mounts = mounts_in(Path::new(MOUNTS))?;

    if let Some(m) = smb::mounted(&mounts, &mine).into_iter().find(|m| m.is_nfs() && m.server.eq_ignore_ascii_case(&server) && m.share == export) {
        println!("{}", m.at.display());
        return Ok(());
    }
    let at = fresh_named(&mine, smb::nfs_folder_name(&server, &export))?;
    if let Err(e) = quiet(&smb::nfs_mount_argv(&server, &export, &at)) {
        // An empty folder of root's, which shows as no place anywhere.
        let _ = std::fs::remove_dir(&at);
        return Err(e);
    }
    // As for a Windows share: closed by itself when the person logs out.
    let _ = quiet(&["systemctl".into(), "start".into(), smb::tidy_unit(p.uid)]);
    println!("{}", at.display());
    Ok(())
}

pub fn unmount(args: &[String]) -> Outcome {
    let p = caller()?;
    let mounts = mounts_in(Path::new(MOUNTS))?;
    let at = unmount_target(args, &p.name, &mounts)?;

    let (tx, rx) = std::sync::mpsc::channel();
    let target = at.clone();
    std::thread::spawn(move || {
        // Nobody listening any more means the wait ended, and the answer is not wanted.
        let _ = tx.send(quiet(&["umount".into(), "-i".into(), target.to_string_lossy().into_owned()]));
    });
    match rx.recv_timeout(std::time::Duration::from_secs(8)) {
        Ok(r) => r.map_err(|e| {
            if e.contains("busy") { t!("admin-share-busy").to_string() } else { e }
        })?,
        Err(_) => detach_all(vec![at.clone()], STOP_LIMIT, detach).map_err(|_| t!("admin-share-not-answering").to_string())?,
    }
    // The share is closed, as asked; its empty folder shows nowhere.
    let _ = std::fs::remove_dir(&at);
    Ok(())
}

pub fn unmount_all(args: &[String]) -> Outcome {
    let p = match (std::env::var("PKEXEC_UID").is_ok(), args) {
        (true, []) => caller()?,
        (false, [uid]) => person_by_uid(uid.parse().map_err(|_| t!("admin-user-id-invalid"))?)?,
        _ => return Err("usage: trindle-admin smb-unmount-all".into()),
    };
    let mine = smb::user_folder(&p.name);
    let mounts = mounts_in(Path::new(MOUNTS))?;
    let at: Vec<PathBuf> = smb::mounted(&mounts, &mine).into_iter().map(|m| m.at).collect();
    detach_all(at, STOP_LIMIT, detach)
}

pub const STOP_LIMIT: std::time::Duration = std::time::Duration::from_secs(6);

fn detach(at: &Path) -> std::io::Result<()> {
    let c = CString::new(at.as_os_str().as_encoded_bytes()).map_err(|_| std::io::ErrorKind::InvalidInput)?;
    // SAFETY: umount2 only reads the NUL-ended path, which lives for the call.
    if unsafe { libc::umount2(c.as_ptr(), libc::MNT_DETACH | libc::UMOUNT_NOFOLLOW) } == 0 {
        Ok(())
    } else {
        Err(std::io::Error::last_os_error())
    }
}

fn detach_all(at: Vec<PathBuf>, limit: std::time::Duration, close: fn(&Path) -> std::io::Result<()>) -> Outcome {
    let (tx, rx) = std::sync::mpsc::channel();
    let n = at.len();
    for path in at {
        let tx = tx.clone();
        std::thread::spawn(move || {
            let r = close(&path).map(|()| {
                // The empty folder of a share that has closed, as in `unmount`.
                let _ = std::fs::remove_dir(&path);
            });
            // Nobody listening any more means the wait ended, and the answer is not wanted.
            let _ = tx.send((path, r));
        });
    }
    drop(tx);
    let end = std::time::Instant::now() + limit;
    let mut last = Ok(());
    let mut done = 0;
    while done < n {
        let Some(wait) = end.checked_duration_since(std::time::Instant::now()) else { break };
        match rx.recv_timeout(wait) {
            Ok((path, r)) => {
                done += 1;
                if let Err(e) = r {
                    last = Err(format!("{} could not be closed: {e}", path.display()));
                }
            }
            Err(_) => break,
        }
    }
    if done < n {
        last = Err(format!("{} shared folders did not answer and were left; they are dropped when the computer stops.", n - done));
    }
    last
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn only_a_share_is_mounted() {
        let (server, share, ip) = mount_args(&strings(&["NAS", "My Photos"])).unwrap();
        assert_eq!((server.as_str(), share.as_str(), ip), ("NAS", "My Photos", None));
        let (_, _, ip) = mount_args(&strings(&["NAS", "Photos", "192.168.1.20"])).unwrap();
        assert_eq!(ip, Some("192.168.1.20".parse().unwrap()));
        for bad in [&["NAS"][..], &["NAS", "a", "1.2.3.4", "x"], &["-o", "x"], &["NAS", "../../etc"], &["NAS", "a/b"],
                    &["//NAS", "x"], &["NAS", "x", "nas"], &["NAS", "x", "fe80::1"], &["NAS,ip=1.2.3.4", "x"], &["NAS", "a\nb"], &["NAS", "x,uid=0"]] {
            assert!(mount_args(&strings(bad)).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn only_the_persons_own_share_is_unmounted() {
        let mounts = "//NAS/Photos /run/media/jo/Photos\\040on\\040NAS cifs rw 0 0\n\
                      /dev/sdb1 /run/media/jo/STICK vfat rw 0 0\n\
                      //NAS/Work /run/media/ann/Work\\040on\\040NAS cifs rw 0 0\n\
                      /dev/sda1 / ext4 rw 0 0\n";
        let ok = unmount_target(&strings(&["/run/media/jo/Photos on NAS"]), "jo", mounts).unwrap();
        assert_eq!(ok, Path::new("/run/media/jo/Photos on NAS"));
        for bad in ["/run/media/jo/STICK", "/run/media/ann/Work on NAS", "/", "/run/media/jo", "/run/media/jo/../ann/Work on NAS",
                    "/run/media/jo/Nothing on NAS", "run/media/jo/Photos on NAS", "/run/media/jo//Photos on NAS"] {
            assert!(unmount_target(&strings(&[bad]), "jo", mounts).is_err(), "{bad}");
        }
        assert!(unmount_target(&strings(&[]), "jo", mounts).is_err());
        assert!(unmount_target(&strings(&["/run/media/jo/Photos on NAS", "x"]), "jo", mounts).is_err());
    }

    #[test]
    fn a_folder_of_its_own_is_found() {
        let dir = std::env::temp_dir().join(format!("trindle-admin-shares-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let one = fresh_folder(&dir, "NAS", "Photos").unwrap();
        let two = fresh_folder(&dir, "NAS", "Photos").unwrap();
        assert_eq!(one, dir.join("Photos on NAS"));
        assert_eq!(two, dir.join("Photos on NAS (2)"));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn a_list_of_mounts_that_cannot_be_read_is_not_taken_for_empty() {
        let said = mounts_in(Path::new("/no/such/folder/mounts")).unwrap_err();
        assert!(said.starts_with("What is mounted on this computer could not be read"), "{said}");
        assert!(mounts_in(Path::new(MOUNTS)).is_ok());
    }

    #[test]
    fn only_an_export_is_nfs_mounted() {
        let (server, export) = nfs_args(&strings(&["nas", "/srv/media"])).unwrap();
        assert_eq!((server.as_str(), export.as_str()), ("nas", "/srv/media"));
        for bad in [&["nas"][..], &["nas", "/a", "x"], &["-o", "/a"], &["nas", "a"], &["nas", "/a,vers=3"], &["nas", "/a/../b"], &["nas:/x", "/a"],
                    &["nas,x", "/a"], &["fe80::1", "/a"], &["nas", "/a b\nc"], &["nas", "/a:/b"], &["", "/a"], &["nas", ""]] {
            assert!(nfs_args(&strings(bad)).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn an_nfs_mount_is_closed_like_a_share_and_only_the_persons_own() {
        let mounts = "nas:/srv/media /run/media/jo/media\\040on\\040nas nfs4 rw 0 0\n\
                      nas:/srv/work /run/media/ann/work\\040on\\040nas nfs4 rw 0 0\n";
        let ok = unmount_target(&strings(&["/run/media/jo/media on nas"]), "jo", mounts).unwrap();
        assert_eq!(ok, Path::new("/run/media/jo/media on nas"));
        assert!(unmount_target(&strings(&["/run/media/ann/work on nas"]), "jo", mounts).is_err());
        assert!(unmount_target(&strings(&["/run/media/jo/work on nas"]), "jo", mounts).is_err());
    }

    #[test]
    fn nfs_is_never_let_through_without_a_password() {

        let rules = include_str!("../../system/polkit/60-trindle.rules");
        assert!(rules.contains("smb-mount "));
        assert!(!rules.contains("nfs-mount"));
    }

    #[test]
    fn an_nfs_folder_is_found_with_a_number_when_taken() {
        let dir = std::env::temp_dir().join(format!("trindle-admin-nfs-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let one = fresh_named(&dir, smb::nfs_folder_name("nas", "/srv/media")).unwrap();
        let two = fresh_named(&dir, smb::nfs_folder_name("nas", "/srv/media")).unwrap();
        assert_eq!((one, two), (dir.join("media on nas"), dir.join("media on nas (2)")));
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn closing_never_waits_on_a_share_that_does_not_answer() {
        fn stuck(_: &Path) -> std::io::Result<()> {
            std::thread::sleep(std::time::Duration::from_secs(120));
            Ok(())
        }
        fn fine(_: &Path) -> std::io::Result<()> { Ok(()) }
        let started = std::time::Instant::now();
        let r = detach_all(vec!["/nonexistent/a".into(), "/nonexistent/b".into()], std::time::Duration::from_millis(300), stuck);
        assert!(started.elapsed() < std::time::Duration::from_secs(2), "{:?}", started.elapsed());
        assert!(r.unwrap_err().contains("2 shared folders did not answer"));
        assert!(detach_all(vec!["/nonexistent/a".into()], std::time::Duration::from_secs(5), fine).is_ok());
        assert!(detach_all(vec![], std::time::Duration::from_millis(10), stuck).is_ok());
        assert!(STOP_LIMIT < std::time::Duration::from_secs(10));
    }

    #[test]
    fn a_share_that_will_not_close_keeps_its_folder() {
        fn refuses(_: &Path) -> std::io::Result<()> { Err(std::io::ErrorKind::TimedOut.into()) }
        let dir = std::env::temp_dir().join(format!("439-keep-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("file"), "x").unwrap();
        let r = detach_all(vec![dir.clone()], std::time::Duration::from_secs(2), refuses);
        assert!(r.is_err());
        assert!(dir.join("file").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn the_shares_unit_is_given_a_short_stop() {
        let unit = include_str!("../../system/systemd/trindle-shares@.service");
        assert!(unit.contains("TimeoutStopSec=10"));
    }
}
