//! `chip-reseal UUID`: seals the disk's key to the TPM chip again, after the
//! chip stopped opening the disk at start (Settings, Hardware). The disk
//! password comes on stdin. The new chip key is added and proved before the
//! old one goes, and the slot the password opens is never touched.

use std::io::Read;
use std::os::unix::fs::{FileTypeExt, OpenOptionsExt};
use std::process::{Command, Stdio};
use std::time::Duration;

use trindle_shared::luks_chip::{chip_listed, chip_slots, free_slot, read_header, refused_disk, valid_uuid, POLICY, REFUSED, TPMS};
use zeroize::Zeroizing;

use super::{and_undone, at, gone, kept, replace, Outcome};
use crate::toolkit::t;

const LOCK: &str = "/run/trindle-chip.lock";

const TRIES: &str = "/run/trindle-chip-tries";
const CRYPTTAB: &str = "/etc/crypttab";
const CLEVIS: &str = "/usr/bin/clevis";

const FREE_TRIES: u32 = 3;
const WAIT: Duration = Duration::from_secs(60);

const MOST_PASSWORD: u64 = 1024;

const PROVE: &str = r#"clevis luks pass -d "$1" -s "$2" | cryptsetup open --test-passphrase --key-slot "$2" --key-file=- "$1""#;

pub(super) fn reseal(args: &[String]) -> Outcome {
    let [uuid] = args else { return Err("usage: trindle-admin chip-reseal UUID".into()) };
    if !valid_uuid(uuid) {
        return Err("usage: trindle-admin chip-reseal UUID".into());
    }
    let _held = hold()?;
    let password = password(&mut std::io::stdin())?;
    reseal_with(uuid, &password)
}

fn hold() -> Result<std::fs::File, String> {
    use std::os::unix::io::AsRawFd;
    let file = std::fs::OpenOptions::new().write(true).create(true).truncate(false).mode(0o600)
        .custom_flags(libc::O_NOFOLLOW).open(at(LOCK)).map_err(|e| e.to_string())?;
    // SAFETY: flock on a descriptor this function owns.
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err(t!("admin-chip-busy").into());
    }
    Ok(file)
}

fn password(input: &mut impl Read) -> Result<Zeroizing<String>, String> {
    let mut bytes = Zeroizing::new(Vec::new());
    input.take(MOST_PASSWORD + 2).read_to_end(&mut bytes).map_err(|_| t!("admin-password-none"))?;
    if bytes.last() == Some(&b'\n') {
        bytes.pop();
    }
    if bytes.is_empty() {
        return Err(t!("admin-password-none").into());
    }
    if bytes.len() as u64 > MOST_PASSWORD || bytes.contains(&b'\n') || bytes.contains(&0) {
        return Err(t!("admin-chip-wrong-password").into());
    }
    let text = std::str::from_utf8(&bytes).map_err(|_| t!("admin-chip-wrong-password"))?;
    Ok(Zeroizing::new(text.to_string()))
}

fn this_disk(uuid: &str) -> Result<(String, Vec<u32>, Option<u32>), String> {
    // SAFETY: reads this process's own user id; cannot fail.
    let owner = unsafe { libc::geteuid() };
    let noted = refused_disk(&at(REFUSED), owner).ok_or_else(|| t!("admin-chip-not-needed"))?;
    let listed = kept(&at(CRYPTTAB))?.lines()
        .any(|l| l.split_whitespace().nth(1).is_some_and(|s| s.eq_ignore_ascii_case(&format!("UUID={uuid}"))));
    let device = format!("/dev/disk/by-uuid/{uuid}");
    let disk = std::fs::File::open(at(&device)).map_err(|_| t!("admin-chip-changed"))?;
    let meta = disk.metadata().map_err(|e| e.to_string())?;
    let is_disk = meta.file_type().is_block_device() || cfg!(test) && meta.is_file();
    let header = read_header(disk).filter(|h| is_disk && h.uuid == uuid).ok_or_else(|| t!("admin-chip-changed"))?;
    let slots = chip_slots(&header.json);
    if noted != uuid || !listed || slots.is_empty() {
        return Err(t!("admin-chip-changed").into());
    }
    Ok((device, slots, free_slot(&header.json)))
}

fn reseal_with(uuid: &str, password: &str) -> Outcome {
    let (device, old, free) = this_disk(uuid)?;
    if !chip_listed(&at(TPMS)) {
        return Err(t!("admin-chip-none").into());
    }
    if !at(CLEVIS).exists() {
        return Err(t!("admin-chip-no-clevis").into());
    }
    let opened = checked(&device, password)?;
    if old.iter().any(|s| prove(&device, *s).is_ok()) {
        return gone(&at(REFUSED));
    }
    let new = free.ok_or_else(|| t!("admin-chip-no-slot"))?;
    let new_slot = new.to_string();
    super::run(&["clevis", "luks", "bind", "-y", "-k", "-", "-s", &new_slot, "-d", &device, "tpm2", POLICY], Some(password))
        .map_err(|_| t!("admin-chip-not-sealed"))?;
    if prove(&device, new).is_err() {
        let undone = super::run(&["clevis", "luks", "unbind", "-f", "-d", &device, "-s", &new_slot], None);
        return Err(and_undone(t!("admin-chip-not-sealed").into(), undone));
    }
    gone(&at(REFUSED))?;

    for slot in old.iter().filter(|s| **s != opened && **s != new) {
        super::run(&["clevis", "luks", "unbind", "-f", "-d", &device, "-s", &slot.to_string()], None)
            .map_err(|_| t!("admin-chip-old-kept"))?;
    }
    Ok(())
}

fn checked(device: &str, password: &str) -> Result<u32, String> {
    let tries = at(TRIES);
    let (count, last) = std::fs::symlink_metadata(&tries).ok().filter(|m| m.is_file() && m.len() < 16)
        .map(|m| (std::fs::read_to_string(&tries).ok().and_then(|s| s.trim().parse::<u32>().ok()).unwrap_or(FREE_TRIES), m.modified().ok()))
        .unwrap_or((0, None));
    let waited = last.and_then(|l| l.elapsed().ok()).is_none_or(|e| e >= WAIT);
    if count >= FREE_TRIES && !waited {
        return Err(t!("admin-chip-wait").into());
    }
    match answered(&["cryptsetup", "open", "--test-passphrase", "-v", "--key-file=-", device], password) {
        Ok(said) => {
            gone(&tries)?;
            opened_slot(&said).ok_or_else(|| t!("admin-chip-not-sealed").into())
        }
        Err((Some(2), _)) => {
            replace(&tries, format!("{}\n", count.saturating_add(1)).as_bytes(), 0o600)?;
            Err(t!("admin-chip-wrong-password").into())
        }
        Err((_, why)) => Err(why),
    }
}

fn opened_slot(said: &str) -> Option<u32> {
    said.lines().find_map(|l| l.trim().strip_prefix("Key slot ")?.strip_suffix(" unlocked.")?.parse().ok())
}

fn prove(device: &str, slot: u32) -> Outcome {
    super::run(&["sh", "-c", PROVE, "sh", device, &slot.to_string()], None)
}

fn answered(argv: &[&str], input: &str) -> Result<String, (Option<i32>, String)> {
    #[cfg(test)]
    if let Some(answer) = super::stand_in::run(argv) {
        return match answer {
            Ok(()) => Ok(super::stand_in::said(argv).and_then(Result::ok).unwrap_or_default()),
            Err(e) => Err((e.strip_prefix("exit ").and_then(|c| c.parse().ok()), e)),
        };
    }
    let mut child = Command::new(argv[0]).args(&argv[1..]).env("LC_ALL", "C")
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
        .spawn().map_err(|e| (None, t!("admin-tool-error", tool = argv[0], why = e.to_string())))?;
    if let Some(mut pipe) = child.stdin.take() {
        // A tool that stopped early ends badly and is said below.
        let _ = std::io::Write::write_all(&mut pipe, input.as_bytes());
    }
    let out = child.wait_with_output().map_err(|e| (None, e.to_string()))?;
    if out.status.success() {
        return Ok(String::from_utf8_lossy(&out.stdout).into_owned());
    }
    let err = String::from_utf8_lossy(&out.stderr).trim().to_string();
    Err((out.status.code(), if err.is_empty() { t!("admin-tool-failed", tool = argv[0]) } else { err }))
}

#[path = "chip_tests.rs"]
#[cfg(test)]
mod chip_tests;
