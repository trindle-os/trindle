//! What is on the other disks, found by looking inside them, so a computer
//! with two drives says which one holds Windows, Ubuntu or Bazzite and
//! nobody replaces the wrong one.

use std::path::Path;
use std::process::Command;

use super::plan;

const LOOK_IN: &str = "/run/trindle-look";

const EFI_FOLDERS: &[(&str, &str)] = &[
    ("microsoft", "Windows"), ("ubuntu", "Ubuntu"), ("fedora", "Fedora"), ("debian", "Debian"),
    ("trindle", "Trindle"), ("opensuse", "openSUSE"), ("pop", "Pop!_OS"), ("manjaro", "Manjaro"),
    ("arch", "Arch Linux"), ("endeavouros", "EndeavourOS"), ("neon", "KDE neon"), ("elementary", "elementary OS"),
    ("zorin", "Zorin OS"), ("centos", "CentOS"), ("rocky", "Rocky Linux"), ("almalinux", "AlmaLinux"),
    ("nixos", "NixOS"),
];

const SUBVOLUMES: &[&str] = &["", "@", "root", "@root", "rootfs", "@rootfs"];

pub fn main() -> i32 {
    let json = crate::common::run::text(&["lsblk", "-J", "-b", "-o", "PATH,TYPE,SIZE,MODEL,RM,FSTYPE,LABEL,FSUSED,MOUNTPOINT"]);
    let _ = std::fs::create_dir_all(LOOK_IN);
    for (n, part) in plan::parse_disks(&json).iter().flat_map(|d| d.parts.iter()).enumerate() {
        let Some(extra) = careful_options(&part.fstype) else { continue };
        if mounted(&part.dev) {
            continue;
        }
        let at = format!("{LOOK_IN}/{n}");
        if std::fs::create_dir_all(&at).is_err() {
            continue;
        }
        let options = format!("ro,noexec,nosuid,nodev{extra}");
        let ok = Command::new("timeout").args(["15", "mount", "-o", &options, &part.dev, &at])
            .stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null())
            .status().is_ok_and(|s| s.success());
        if ok {
            match name_inside(Path::new(&at)) {
                Some(Found::System(name)) => println!("{}:{name}", part.dev),
                Some(Found::Guess(name)) => println!("{}:{name}:efi", part.dev),
                None => {}
            }
            let gone = Command::new("umount").arg(&at).status().is_ok_and(|s| s.success());
            if !gone {
                let _ = Command::new("umount").args(["-l", &at]).status();
            }
        }
        let _ = std::fs::remove_dir(&at);
    }
    let _ = std::fs::remove_dir(LOOK_IN);
    0
}

fn careful_options(fstype: &str) -> Option<&'static str> {
    match fstype {
        "ext4" | "ext3" | "ext2" => Some(",noload"),
        "btrfs" => Some(",nologreplay"),
        "xfs" => Some(",norecovery"),
        "ntfs" | "vfat" | "f2fs" => Some(""),
        _ => None,
    }
}

fn mounted(dev: &str) -> bool {
    let mounts = std::fs::read_to_string("/proc/self/mounts").unwrap_or_default();
    mounts.lines().any(|l| l.split_whitespace().next() == Some(dev))
}

#[derive(Debug, PartialEq)]
pub enum Found {

    System(String),

    Guess(String),
}

pub fn name_inside(root: &Path) -> Option<Found> {
    for sub in SUBVOLUMES {
        let base = root.join(sub);
        for file in ["etc/os-release", "usr/lib/os-release"] {
            if let Some(name) = os_release(&base.join(file)) {
                return Some(Found::System(name));
            }
        }

        if let Some(name) = ostree(&base) {
            return Some(Found::System(name));
        }
    }

    if let Some(name) = boot_entry(&root.join("loader/entries")) {
        return Some(Found::System(name));
    }
    if root.join("Windows/System32").is_dir() {
        return Some(Found::System("Windows".into()));
    }
    let names = efi_names(root);
    (!names.is_empty()).then(|| Found::Guess(names.join(" and ")))
}

fn small_text(path: &Path) -> Option<String> {
    use std::io::Read;
    use std::os::unix::fs::OpenOptionsExt;
    let file = std::fs::OpenOptions::new().read(true).custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK).open(path).ok()?;
    if !file.metadata().ok()?.is_file() {
        return None;
    }
    let mut text = String::new();
    file.take(TEXT_MOST).read_to_string(&mut text).ok()?;
    Some(text)
}

const TEXT_MOST: u64 = 64 * 1024;

fn os_release(path: &Path) -> Option<String> {
    let text = small_text(path)?;
    let value = |key: &str| text.lines()
        .find_map(|l| l.strip_prefix(key)?.strip_prefix('='))
        .map(|v| v.trim().trim_matches('"').trim_matches('\'').to_string())
        .filter(|v| !v.is_empty());
    value("PRETTY_NAME").or_else(|| value("NAME")).map(|n| plain(&n))
}

fn plain(name: &str) -> String {
    name.split(" (").next().unwrap_or(name).trim().to_string()
}

fn ostree(base: &Path) -> Option<String> {
    for os in std::fs::read_dir(base.join("ostree/deploy")).ok()?.flatten() {
        let Ok(versions) = std::fs::read_dir(os.path().join("deploy")) else { continue };
        for v in versions.flatten().filter(|v| v.path().is_dir()) {
            if let Some(name) = os_release(&v.path().join("usr/lib/os-release"))
                .or_else(|| os_release(&v.path().join("etc/os-release"))) {
                return Some(name);
            }
        }
    }
    None
}

fn boot_entry(dir: &Path) -> Option<String> {
    let mut files: Vec<_> = std::fs::read_dir(dir).ok()?.flatten().map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "conf")).collect();
    files.sort();
    files.iter().rev().find_map(|f| {
        let text = small_text(f)?;
        let title = text.lines().find_map(|l| l.strip_prefix("title"))?.trim();
        (!title.is_empty()).then(|| plain(title))
    })
}

fn efi_names(root: &Path) -> Vec<String> {
    let Ok(dir) = std::fs::read_dir(root.join("EFI")).or_else(|_| std::fs::read_dir(root.join("efi"))) else {
        return Vec::new();
    };
    let mut folders: Vec<String> = dir.flatten().map(|e| e.file_name().to_string_lossy().to_lowercase()).collect();
    folders.sort();
    let mut out: Vec<String> = Vec::new();
    for f in folders {
        if let Some((_, name)) = EFI_FOLDERS.iter().find(|(k, _)| *k == f) {
            if !out.iter().any(|o| o == name) {
                out.push(name.to_string());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tree(files: &[(&str, &str)]) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("trindle-look-{}-{}", std::process::id(),
            files.first().map(|f| f.0.replace('/', "_")).unwrap_or_default()));
        let _ = std::fs::remove_dir_all(&root);
        for (path, text) in files {
            let p = root.join(path);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, text).unwrap();
        }
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    #[test]
    fn a_pipe_on_the_disk_is_not_waited_for() {
        let root = tree(&[("pipe_dir/x", "")]);
        let pipe = root.join("pipe_dir/os-release");
        let c = std::ffi::CString::new(pipe.to_string_lossy().as_bytes()).unwrap();
        // SAFETY: a valid C string path and a plain mode.
        assert_eq!(unsafe { libc::mkfifo(c.as_ptr(), 0o600) }, 0);
        let (tell, hear) = std::sync::mpsc::channel();
        std::thread::spawn(move || tell.send(small_text(&pipe)).ok());
        assert_eq!(hear.recv_timeout(std::time::Duration::from_secs(5)), Ok(None));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn bazzite_is_found_in_its_ostree_folder() {
        let root = tree(&[("root/ostree/deploy/default/deploy/abc123.0/usr/lib/os-release",
                           "NAME=\"Bazzite\"\nPRETTY_NAME=\"Bazzite 42 (FROM Fedora Kinoite)\"\n")]);
        assert_eq!(name_inside(&root), Some(Found::System("Bazzite 42".into())));
    }

    #[test]
    fn bazzites_boot_partition_names_it_too() {
        let root = tree(&[("loader/entries/ostree-1.conf", "title Bazzite 42 (FROM Fedora Kinoite) (ostree:0)\nversion 1\n")]);
        assert_eq!(name_inside(&root), Some(Found::System("Bazzite 42".into())));
    }

    #[test]
    fn ubuntu_on_btrfs_is_found_in_its_subvolume() {
        let root = tree(&[("@/etc/os-release", "PRETTY_NAME=\"Ubuntu 24.04.1 LTS\"\n")]);
        assert_eq!(name_inside(&root), Some(Found::System("Ubuntu 24.04.1 LTS".into())));
    }

    #[test]
    fn windows_and_start_up_files_are_recognised() {
        let root = tree(&[("Windows/System32/x", "")]);
        assert_eq!(name_inside(&root), Some(Found::System("Windows".into())));
        let esp = tree(&[("EFI/fedora/shimx64.efi", ""), ("EFI/BOOT/BOOTX64.EFI", ""), ("EFI/Microsoft/Boot/x", "")]);
        assert_eq!(name_inside(&esp), Some(Found::Guess("Fedora and Windows".into())));
    }

    #[test]
    fn only_careful_mounts_are_made() {
        assert_eq!(careful_options("ext4"), Some(",noload"));
        assert_eq!(careful_options("crypto_LUKS"), None);
        assert_eq!(careful_options("swap"), None);
    }
}
