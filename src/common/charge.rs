//! Stopping charging at 80%: kinder to the battery of a laptop that spends
//! most of its time plugged in. The laptop's own firmware does the stopping;
//! Linux only tells it where. Not every laptop lets it be told, and on one
//! that does not, nothing here is offered.
//!
//! The choice is kept in STATE, and trindle-charge-limit.service tells the
//! firmware again at every start, since some laptops forget.

use std::path::{Path, PathBuf};

use crate::toolkit::t;

pub const LIMIT: u32 = 80;
/// Present, holding the limit, while it is on.
pub const STATE: &str = "/etc/trindle/charge-limit";
pub const SERVICE: &str = "trindle-charge-limit.service";

const SUPPLIES: &str = "/sys/class/power_supply";
const END: &str = "charge_control_end_threshold";
const START: &str = "charge_control_start_threshold";
const FWK_PARAM: &str = "/sys/module/cros_charge_control/parameters/probe_with_fwk_charge_control";
const PLATFORM: &str = "/sys/bus/platform/devices";

/// The batteries whose charging can be stopped short.
pub fn batteries() -> Vec<PathBuf> {
    batteries_in(Path::new(SUPPLIES))
}

fn batteries_in(root: &Path) -> Vec<PathBuf> {
    let Ok(dir) = std::fs::read_dir(root) else { return Vec::new() };
    let mut out: Vec<PathBuf> = dir.flatten().map(|e| e.path())
        .filter(|p| std::fs::read_to_string(p.join("type")).is_ok_and(|t| t.trim() == "Battery"))
        .filter(|p| p.join(END).exists())
        .collect();
    out.sort();
    out
}

/// A computer with a battery of its own, rather than a mouse's or a
/// keyboard's, which the kernel lists too.
pub fn laptop() -> bool {
    laptop_in(Path::new(SUPPLIES))
}

fn laptop_in(root: &Path) -> bool {
    let Ok(dir) = std::fs::read_dir(root) else { return false };
    dir.flatten().map(|e| e.path()).any(|p| {
        std::fs::read_to_string(p.join("type")).is_ok_and(|t| t.trim() == "Battery")
            && !std::fs::read_to_string(p.join("scope")).is_ok_and(|s| s.trim() == "Device")
    })
}

/// Running on its battery: a laptop with no charger plugged in.
pub fn on_battery() -> bool {
    laptop() && !plugged_in(Path::new(SUPPLIES))
}

/// UPower's battery state, put right: some laptops, a Framework among them,
/// say "discharging" while on the charger whenever they let a full battery
/// settle. With the charger in, that is "plugged in, not charging" (5).
pub fn true_state(upower: u32) -> u32 {
    match upower {
        2 | 6 if !on_battery() => 5,
        s => s,
    }
}

fn plugged_in(root: &Path) -> bool {
    let Ok(dir) = std::fs::read_dir(root) else { return true };
    dir.flatten().map(|e| e.path()).any(|p| {
        std::fs::read_to_string(p.join("type")).is_ok_and(|t| matches!(t.trim(), "Mains" | "USB"))
            && std::fs::read_to_string(p.join("online")).is_ok_and(|o| o.trim() == "1")
    })
}

pub fn supported() -> bool {
    !batteries().is_empty() || framework_held().is_some()
}

/// A Framework laptop's charge limit, which the kernel finds but leaves
/// unused until told, in case the Framework's own setup screen has one set.
/// Its name, while it is waiting.
fn framework_held() -> Option<String> {
    let param = std::fs::read_to_string(FWK_PARAM).ok()?;
    if param.trim() != "N" {
        return None;
    }
    std::fs::read_dir(PLATFORM).ok()?.flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .find(|n| n.starts_with("cros-charge-control") && !Path::new(PLATFORM).join(n).join("driver").exists())
}

/// Puts the Framework's charge limit to work without waiting for a restart;
/// trindle-charge.conf does the same at every start after this.
fn framework_wake() {
    let Some(name) = framework_held() else { return };
    // Whether the limit is there to use is found next (`batteries`), and
    // a laptop that takes it no way at all is said then.
    let _ = std::fs::write(FWK_PARAM, "Y\n");
    // As above.
    let _ = std::fs::write("/sys/bus/platform/drivers/cros-charge-control/bind", name);
}

/// A Framework laptop's firmware keeps a charge limit of its own (the one
/// its setup screen shows), and goes by that one whatever the kernel's
/// limit says: left at 100 it charges to the top. So on a Framework the
/// firmware is told directly as well, through the kernel's door to it.
const FWK_DOOR: &str = "/dev/cros_ec";
const VENDOR: &str = "/sys/class/dmi/id/sys_vendor";
/// The Framework firmware's own "charge limit" command, and its "set" mode.
const FWK_LIMIT: u32 = 0x3E03;
const FWK_SET: u8 = 1 << 1;
/// The kernel's request to pass one command to the firmware and bring back
/// its answer: _IOWR(0xEC, 0, the 20-byte heading below).
const EC_XCMD: libc::c_ulong = 0xC014_EC00;
const EC_HEAD: usize = 20;

fn framework() -> bool {
    std::fs::read_to_string(VENDOR).is_ok_and(|v| v.trim() == "Framework")
}

/// One command for the firmware as the kernel takes it: five numbers
/// (version, command, bytes out, bytes back, result), then the bytes: the
/// mode, where to stop and where to start charging again. A start of 0
/// leaves the firmware not charging until the battery is empty.
fn fwk_limit_command(top: u8) -> Vec<u8> {
    let out = [FWK_SET, top, top.saturating_sub(5)];
    let mut msg = Vec::with_capacity(EC_HEAD + out.len());
    for n in [0, FWK_LIMIT, out.len() as u32, 2, 0] {
        msg.extend_from_slice(&n.to_ne_bytes());
    }
    msg.extend_from_slice(&out);
    msg
}

/// Tells a Framework's firmware where to stop. Nothing is done on any other
/// make, whose firmware could mean something else by the same command.
fn framework_tell(top: u8) -> Result<(), String> {
    use std::os::fd::AsRawFd;
    if !framework() || !Path::new(FWK_DOOR).exists() {
        return Ok(());
    }
    let door = std::fs::OpenOptions::new().read(true).write(true).open(FWK_DOOR)
        .map_err(|e| t!("charge-no-firmware-door", error = e.to_string()))?;
    let mut msg = fwk_limit_command(top);
    // SAFETY: the request reads and writes only within `msg`, which holds
    // the 20-byte heading and the 3 bytes it says follow (the 2 bytes of
    // answer fit in those), and lives until the call returns.
    let got = unsafe { libc::ioctl(door.as_raw_fd(), EC_XCMD, msg.as_mut_ptr()) };
    let result = u32::from_ne_bytes([msg[16], msg[17], msg[18], msg[19]]);
    if got < 0 || result != 0 {
        return Err(t!("charge-firmware-refused").to_string());
    }
    Ok(())
}

pub fn on() -> bool {
    Path::new(STATE).exists()
}

/// Tells each battery's firmware where to stop: `LIMIT`, or all the way to
/// the top. Where there is also a point at which charging starts again, it
/// is kept below the stopping point, which the firmware insists on.
pub fn apply(limit: bool) -> Result<(), String> {
    framework_wake();
    // Either way of telling is enough; only a laptop that takes neither
    // has failed.
    let told = framework_tell(if limit { LIMIT as u8 } else { 100 });
    tell(&batteries(), limit, told)
}

/// Tells each of `batteries`; `told` is how telling the firmware by its
/// own door went, which matters only where there is no battery to tell.
fn tell(batteries: &[PathBuf], limit: bool, told: Result<(), String>) -> Result<(), String> {
    if batteries.is_empty() {
        return Err(told.err().unwrap_or_else(|| t!("charge-not-allowed").to_string()));
    }
    for b in batteries {
        let (end, start) = (b.join(END), b.join(START));
        let write = |p: &Path, v: u32| std::fs::write(p, format!("{v}\n"))
            .map_err(|e| t!("charge-could-not-set", path = p.display().to_string(), error = e.to_string()));
        if limit {
            // Charging begins again 5 below the stop; left at 0 it never would.
            if start.exists() {
                write(&start, LIMIT - 5)?;
            }
            write(&end, LIMIT)?;
        } else {
            write(&end, 100)?;
            if start.exists() {
                // Left where it was, charging would not begin again until
                // the battery had run down to it: not the full battery
                // that was asked for.
                write(&start, 0)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_framework_command_is_laid_out_as_the_kernel_takes_it() {
        let msg = fwk_limit_command(80);
        assert_eq!(msg.len(), EC_HEAD + 3);
        let word = |i: usize| u32::from_ne_bytes([msg[i], msg[i + 1], msg[i + 2], msg[i + 3]]);
        assert_eq!([word(0), word(4), word(8), word(12), word(16)], [0, 0x3E03, 3, 2, 0]);
        assert_eq!(&msg[EC_HEAD..], [2, 80, 75]);
    }

    #[test]
    fn only_batteries_that_can_be_told_are_found() {
        let root = std::env::temp_dir().join(format!("trindle-charge-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for (name, kind, can) in [("BAT0", "Battery", true), ("BAT1", "Battery", false), ("AC", "Mains", false)] {
            let d = root.join(name);
            std::fs::create_dir_all(&d).unwrap();
            std::fs::write(d.join("type"), format!("{kind}\n")).unwrap();
            if can {
                std::fs::write(d.join(END), "100\n").unwrap();
            }
        }
        assert_eq!(batteries_in(&root), vec![root.join("BAT0")]);
        assert!(laptop_in(&root));
        std::fs::write(root.join("AC").join("online"), "0\n").unwrap();
        assert!(!plugged_in(&root));
        std::fs::write(root.join("AC").join("online"), "1\n").unwrap();
        assert!(plugged_in(&root));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn a_battery_that_would_not_charge_to_the_top_again_is_said() {
        let dir = std::env::temp_dir().join(format!("trindle-charge-full-{}", std::process::id()));
        // This test's own folder, by its name.
        let _ = std::fs::remove_dir_all(&dir);
        let battery = dir.join("BAT0");
        std::fs::create_dir_all(&battery).unwrap();
        std::fs::write(battery.join(END), "80\n").unwrap();
        std::fs::write(battery.join(START), "75\n").unwrap();
        assert_eq!(tell(std::slice::from_ref(&battery), false, Ok(())), Ok(()));
        assert_eq!(std::fs::read_to_string(battery.join(START)).unwrap(), "0\n");
        // The point where charging starts again cannot be written: a
        // folder stands where it is.
        std::fs::remove_file(battery.join(START)).unwrap();
        std::fs::create_dir(battery.join(START)).unwrap();
        let said = tell(std::slice::from_ref(&battery), false, Ok(())).unwrap_err();
        assert!(said.starts_with("Could not set") && said.contains(START), "{said}");
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
