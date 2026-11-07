//! "accessibility".

use std::os::unix::fs::{DirBuilderExt, FileTypeExt, MetadataExt};
use std::path::{Path, PathBuf};

pub const KEYS_FOLDER: &str = "trindle-screen-reader";
pub const VOICE_FOLDER: &str = "trindle-voice";
pub const BUS_ENV: &str = "AT_SPI_BUS_ADDRESS";

fn mine(m: &std::fs::Metadata) -> bool {
    // SAFETY: asks the kernel for this process's own user id. It reads nothing and cannot fail.
    m.uid() == unsafe { libc::getuid() }
}

pub fn bus_socket(runtime: &Path) -> Option<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(runtime.join("at-spi")).ok()?.flatten()
        .filter(|e| e.file_name().to_string_lossy().starts_with("bus"))
        .filter(|e| e.path().symlink_metadata().is_ok_and(|m| m.file_type().is_socket() && mine(&m)))
        .map(|e| e.path()).collect();
    found.sort();
    found.into_iter().next()
}

pub fn keys_folder(runtime: &Path) -> Option<PathBuf> {
    private_folder(runtime, KEYS_FOLDER)
}

pub fn voice_folder(runtime: &Path) -> Option<PathBuf> {
    private_folder(runtime, VOICE_FOLDER)
}

fn private_folder(runtime: &Path, name: &str) -> Option<PathBuf> {
    let at = runtime.join(name);
    if std::fs::symlink_metadata(&at).is_err() {
        std::fs::DirBuilder::new().mode(0o700).create(&at).ok()?;
    }
    let m = std::fs::symlink_metadata(&at).ok()?;
    (m.is_dir() && mine(&m) && m.mode() & 0o022 == 0).then_some(at)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::net::UnixListener;

    fn runtime(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("trindle-a11y-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("at-spi")).unwrap();
        dir
    }

    #[test]
    fn the_bus_is_a_socket_of_the_persons_named_bus_and_nothing_else() {
        let r = runtime("bus");
        assert_eq!(bus_socket(&r), None);
        std::fs::write(r.join("at-spi/bus_0"), "").unwrap();
        std::fs::write(r.join("at-spi/other"), "").unwrap();
        let _held = UnixListener::bind(r.join("at-spi/other-sock")).unwrap();
        assert_eq!(bus_socket(&r), None, "a file named bus is not a socket, a socket not named bus is not the bus");
        std::fs::remove_file(r.join("at-spi/bus_0")).unwrap();
        let held = UnixListener::bind(r.join("at-spi/bus_1")).unwrap();
        std::os::unix::fs::symlink(r.join("at-spi/bus_1"), r.join("at-spi/bus_0")).unwrap();
        assert_eq!(bus_socket(&r), Some(r.join("at-spi/bus_1")), "a link to a socket is not taken");
        drop(held);
        let _ = std::fs::remove_dir_all(&r);
    }

    #[test]
    fn the_key_folder_is_made_private_and_a_link_or_open_folder_is_refused() {
        let r = runtime("keys");
        let at = keys_folder(&r).unwrap();
        assert_eq!(std::fs::metadata(&at).unwrap().mode() & 0o777, 0o700);
        assert_eq!(keys_folder(&r), Some(at.clone()), "found again");
        std::fs::set_permissions(&at, std::os::unix::fs::PermissionsExt::from_mode(0o777)).unwrap();
        assert_eq!(keys_folder(&r), None, "a folder others may write");
        std::fs::remove_dir(&at).unwrap();
        std::fs::create_dir(r.join("elsewhere")).unwrap();
        std::os::unix::fs::symlink(r.join("elsewhere"), &at).unwrap();
        assert_eq!(keys_folder(&r), None, "a link");
        assert_eq!(voice_folder(&r).unwrap(), r.join("trindle-voice"));
        assert_eq!(std::fs::metadata(r.join("trindle-voice")).unwrap().mode() & 0o777, 0o700);
        let _ = std::fs::remove_dir_all(&r);
    }
}
