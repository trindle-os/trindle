//! Switching the system's own swap files off, and on again, around Drives making room on the filesystem that holds them.

use std::os::unix::fs::MetadataExt;
use std::path::Path;

use super::{and_undone, at, kept, reading, replace, run, Outcome};
use crate::disks::plan::unescape;
use crate::toolkit::t;

const MOST: usize = 32;

const RESUME: &str = "/etc/default/grub.d/trindle-resume.cfg";

const KERNEL: &str = "/sys/power/resume_offset";

pub fn off(args: &[String]) -> Outcome {
    let (files, _) = checked(args, "swap-off")?;
    let active = kept(&at("/proc/swaps"))?;
    let mut done: Vec<&str> = Vec::new();
    for file in files.iter().filter(|f| is_on(&active, f)) {
        if let Err(why) = run(&["swapoff", file], None) {
            let undone = done.iter().try_for_each(|f| run(&["swapon", f], None));
            return Err(and_undone(t!("admin-swap-not-off", path = file.as_str(), why = why), undone));
        }
        done.push(file);
    }
    Ok(())
}

pub fn on(args: &[String]) -> Outcome {
    let (files, fstab) = checked(args, "swap-on")?;
    let active = kept(&at("/proc/swaps"))?;

    let failed: Vec<String> = files.iter().filter(|f| !is_on(&active, f))
        .filter_map(|f| run(&["swapon", f], None).err().map(|why| t!("admin-swap-not-on", path = f.as_str(), why = why)))
        .collect();
    if !failed.is_empty() {
        return Err(failed.join(" "));
    }

    if let [only] = listed(&fstab).as_slice() {
        if files.contains(only) {
            if let Err(why) = keep_resume(only) {
                println!("{why}");
            }
        }
    }
    Ok(())
}

fn keep_resume(file: &str) -> Outcome {
    let path = at(RESUME);
    let cfg = kept(&path)?;
    let Some(old) = cfg.split_whitespace().find_map(|w| w.trim_end_matches('"').strip_prefix("resume_offset=")) else { return Ok(()) };
    let now = reading(&["btrfs", "inspect-internal", "map-swapfile", "-r", file]);
    let now = now.trim();
    if now.is_empty() || !now.bytes().all(|b| b.is_ascii_digit()) {
        return Err(t!("admin-swap-resume-unread").into());
    }
    if now != old {
        replace(&path, cfg.replace(&format!("resume_offset={old}"), &format!("resume_offset={now}")).as_bytes(), 0o644)?;
        run(&["update-grub"], None)?;
    }
    follow_in_kernel(now)
}

fn follow_in_kernel(now: &str) -> Outcome {
    let path = at(KERNEL);
    let had = match std::fs::read_to_string(&path) {
        Ok(had) => had,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(format!("{} ({e})", t!("admin-swap-resume-restart"))),
    };
    if had.trim() == now {
        return Ok(());
    }
    std::fs::write(&path, now).map_err(|e| format!("{} ({e})", t!("admin-swap-resume-restart")))
}

fn checked(args: &[String], verb: &str) -> Result<(Vec<String>, String), String> {
    if args.is_empty() || args.len() > MOST {
        return Err(format!("usage: trindle-admin {verb} SWAPFILE..."));
    }
    let fstab = kept(&at("/etc/fstab"))?;
    for file in args {
        if !listed(&fstab).contains(file) {
            return Err(t!("admin-swap-not-listed", path = file.as_str()));
        }
        roots_own(Path::new(file)).map_err(|why| t!("admin-swap-not-safe", path = file.as_str(), why = why))?;
    }
    Ok((args.to_vec(), fstab))
}

fn listed(fstab: &str) -> Vec<String> {
    fstab.lines().map(str::trim).filter(|l| !l.starts_with('#')).filter_map(|l| {
        let f: Vec<&str> = l.split_whitespace().collect();
        (f.len() >= 3 && f[2] == "swap").then(|| unescape(f[0]))
    }).filter(|f| f.starts_with('/') && !f.starts_with("/dev/") && !f.split('/').any(|p| p == "..") && !f.chars().any(char::is_control))
    .collect()
}

fn roots_own(file: &Path) -> Result<(), String> {
    // SAFETY: asks the kernel for this process's own effective user id. It

    let me = unsafe { libc::geteuid() };
    let mut step = Some(file);
    while let Some(p) = step {
        let m = std::fs::symlink_metadata(at(p)).map_err(|e| e.to_string())?;
        let kind_ok = if p == file { m.file_type().is_file() && m.mode() & 0o077 == 0 } else { m.file_type().is_dir() && m.mode() & 0o022 == 0 };
        if !kind_ok || m.uid() != me {
            return Err(p.display().to_string());
        }
        step = p.parent();
    }
    Ok(())
}

fn is_on(swaps: &str, file: &str) -> bool {
    swaps.lines().skip(1).any(|l| {
        let mut f = l.split_whitespace();
        f.next().is_some_and(|n| unescape(n) == file) && f.next() == Some("file")
    })
}

#[cfg(test)]
mod tests {
    use crate::admin::stand_in::StandIn;
    use std::os::unix::fs::PermissionsExt;

    const FSTAB: &str = "UUID=x / btrfs subvol=@rootfs 0 0\n# /old/swapfile none swap defaults 0 0\n/swap/swapfile none swap defaults 0 0\n";
    const ON: &str = "Filename\tType\tSize\tUsed\tPriority\n/dev/zram0 partition 100 0 100\n/swap/swapfile file 100 0 -2\n";
    const OFF: &str = "Filename\tType\tSize\tUsed\tPriority\n/dev/zram0 partition 100 0 100\n";

    fn computer(s: &StandIn, swaps: &str) {
        s.write("/etc/fstab", FSTAB.as_bytes());
        s.write("/proc/swaps", swaps.as_bytes());
        s.write("/swap/swapfile", b"");
        let mode = |p: &str, m: u32| std::fs::set_permissions(s.path(p), std::fs::Permissions::from_mode(m)).unwrap();
        mode("/swap/swapfile", 0o600);
        mode("/swap", 0o755);
        mode("/etc", 0o755);
        mode("/", 0o755);
    }

    fn args(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn the_systems_swap_file_goes_off_and_on_and_only_when_it_needs_to() {
        let s = StandIn::all_well("swap");
        computer(&s, ON);
        super::off(&args(&["/swap/swapfile"])).unwrap();
        s.write("/proc/swaps", OFF.as_bytes());
        super::off(&args(&["/swap/swapfile"])).unwrap();
        super::on(&args(&["/swap/swapfile"])).unwrap();
        s.write("/proc/swaps", ON.as_bytes());
        super::on(&args(&["/swap/swapfile"])).unwrap();
        assert_eq!(s.ran(), ["swapoff /swap/swapfile", "swapon /swap/swapfile"]);
    }

    #[test]
    fn waking_from_hibernation_follows_a_swap_file_that_moved() {
        let cfg = "GRUB_CMDLINE_LINUX=\"$GRUB_CMDLINE_LINUX resume=UUID=x resume_offset=533760\"\n";
        {
            let s = StandIn::all_well("swap-resume");
            computer(&s, OFF);
            s.write(super::RESUME, cfg.as_bytes());
            s.prints("btrfs inspect-internal map-swapfile -r /swap/swapfile", "533760\n");
            super::on(&args(&["/swap/swapfile"])).unwrap();
            assert_eq!(s.ran(), ["swapon /swap/swapfile"], "not moved, nothing written");
        }
        let s = StandIn::all_well("swap-resume-moved");
        computer(&s, OFF);
        s.write(super::RESUME, cfg.as_bytes());
        s.prints("btrfs inspect-internal map-swapfile -r /swap/swapfile", "81920\n");
        s.write(super::KERNEL, b"533760\n");
        super::on(&args(&["/swap/swapfile"])).unwrap();
        assert_eq!(std::fs::read_to_string(s.path(super::RESUME)).unwrap(), cfg.replace("533760", "81920"));
        assert_eq!(s.ran(), ["swapon /swap/swapfile", "update-grub"]);
        assert_eq!(std::fs::read_to_string(s.path(super::KERNEL)).unwrap(), "81920", "hibernating works before a restart");
        drop(s);

        let s = StandIn::all_well("swap-resume-unread");
        computer(&s, OFF);
        s.write(super::RESUME, cfg.as_bytes());
        s.prints("btrfs inspect-internal map-swapfile -r /swap/swapfile", "ERROR: not a swap file\n");
        assert_eq!(super::keep_resume("/swap/swapfile").unwrap_err(), crate::toolkit::t!("admin-swap-resume-unread"));
        super::on(&args(&["/swap/swapfile"])).unwrap();
        assert_eq!(s.ran(), ["swapon /swap/swapfile"]);
        assert_eq!(std::fs::read_to_string(s.path(super::RESUME)).unwrap(), cfg);
    }

    #[test]
    fn the_running_kernel_follows_the_swap_file_too() {
        let cfg = "GRUB_CMDLINE_LINUX=\"$GRUB_CMDLINE_LINUX resume=UUID=x resume_offset=81920\"\n";
        let s = StandIn::all_well("swap-resume-kernel");
        computer(&s, OFF);
        s.write(super::RESUME, cfg.as_bytes());
        s.write(super::KERNEL, b"533760\n");
        s.prints("btrfs inspect-internal map-swapfile -r /swap/swapfile", "81920\n");
        super::on(&args(&["/swap/swapfile"])).unwrap();
        assert_eq!(s.ran(), ["swapon /swap/swapfile"], "the start-up line was right already");
        assert_eq!(std::fs::read_to_string(s.path(super::KERNEL)).unwrap(), "81920");
        drop(s);

        let s = StandIn::all_well("swap-resume-no-kernel");
        computer(&s, OFF);
        s.write(super::RESUME, cfg.as_bytes());
        s.prints("btrfs inspect-internal map-swapfile -r /swap/swapfile", "81920\n");
        assert!(super::keep_resume("/swap/swapfile").is_ok());
        assert!(!s.path(super::KERNEL).exists());
    }

    #[test]
    fn only_a_swap_file_of_the_systems_own_is_touched() {
        let s = StandIn::all_well("swap-own");
        computer(&s, ON);
        s.write("/home/john/file", b"");
        for file in ["/home/john/file", "/old/swapfile", "swap/swapfile", "/swap/../swap/swapfile", "/swap/swapfile\n"] {
            assert!(super::on(&args(&[file])).is_err(), "{file:?}");
            assert!(super::off(&args(&[file])).is_err(), "{file:?}");
        }
        assert!(super::on(&[]).is_err());
        assert!(super::on(&vec!["/swap/swapfile".to_string(); super::MOST + 1]).is_err());

        let mode = |p: &str, m: u32| std::fs::set_permissions(s.path(p), std::fs::Permissions::from_mode(m)).unwrap();
        mode("/swap/swapfile", 0o644);
        assert!(super::on(&args(&["/swap/swapfile"])).unwrap_err().contains("/swap/swapfile"));
        mode("/swap/swapfile", 0o600);
        mode("/swap", 0o777);
        assert!(super::on(&args(&["/swap/swapfile"])).unwrap_err().contains("/swap"));
        mode("/swap", 0o755);
        std::fs::remove_file(s.path("/swap/swapfile")).unwrap();
        std::os::unix::fs::symlink(s.path("/home/john/file"), s.path("/swap/swapfile")).unwrap();
        assert!(super::on(&args(&["/swap/swapfile"])).is_err());
        assert!(s.ran().is_empty(), "{:?}", s.ran());
    }

    #[test]
    fn a_swap_file_that_will_not_go_off_puts_the_others_back_on() {
        let s = StandIn::failing("swap-fail", "swapoff /swap/two", "swapoff: Cannot allocate memory");
        computer(&s, &format!("{ON}/swap/two file 100 0 -3\n"));
        s.write("/etc/fstab", format!("{FSTAB}/swap/two none swap defaults 0 0\n").as_bytes());
        s.write("/swap/two", b"");
        std::fs::set_permissions(s.path("/swap/two"), std::fs::Permissions::from_mode(0o600)).unwrap();
        let said = super::off(&args(&["/swap/swapfile", "/swap/two"])).unwrap_err();
        assert!(said.contains("/swap/two") && said.contains("Cannot allocate memory"), "{said}");
        assert_eq!(s.ran(), ["swapoff /swap/swapfile", "swapoff /swap/two", "swapon /swap/swapfile"]);
    }

    #[test]
    fn every_swap_file_is_tried_when_they_go_back_on() {
        let s = StandIn::failing("swap-on-fail", "swapon /swap/swapfile", "swapon: busy");
        computer(&s, OFF);
        s.write("/etc/fstab", format!("{FSTAB}/swap/two none swap defaults 0 0\n").as_bytes());
        s.write("/swap/two", b"");
        std::fs::set_permissions(s.path("/swap/two"), std::fs::Permissions::from_mode(0o600)).unwrap();
        let said = super::on(&args(&["/swap/swapfile", "/swap/two"])).unwrap_err();
        assert!(said.contains("/swap/swapfile") && said.contains("busy"), "{said}");
        assert_eq!(s.ran(), ["swapon /swap/swapfile", "swapon /swap/two"]);
    }
}
