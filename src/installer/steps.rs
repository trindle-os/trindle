//! The half that runs as root: partition, copy the running system, settle it in.
//! Every step prints "percent text" so the window can follow along.

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};
use std::sync::mpsc::Receiver;

use zeroize::Zeroizing;

use super::plan::{part_name, whole_disk, Lock, Plan, Where, KEEP_FREE, LEAST};
use crate::common::run::text;
use crate::common::terms;
use crate::toolkit::t;

const TARGET: &str = "/run/trindle-install/target";
const TOP: &str = "/run/trindle-install/top";
const MAPPED: &str = "/dev/mapper/trindle";
const ESP_MB: u64 = 512;

const GREETD: &str = "[terminal]\nvt = 7\n\n[default_session]\ncommand = \"/usr/bin/trindle-greeter-session\"\nuser = \"_greetd\"\n";
const BOOT_MB: u64 = 1024;

const LET_GO_TRIES: u32 = 10;

type Step = Result<(), String>;

fn say(percent: u32, what: &str) {
    println!("{percent} {what}");
    let _ = std::io::stdout().flush();
}

fn problem(words: String) {
    println!("! {words}");
    let _ = std::io::stdout().flush();
}

fn run(argv: &[&str]) -> Step {
    run_input(argv, None)
}

fn run_input(argv: &[&str], input: Option<&str>) -> Step {
    run_with(argv, input, &[])
}

fn run_with(argv: &[&str], input: Option<&str>, ok: &[i32]) -> Step {
    use crate::admin::Fault;
    crate::admin::tool(argv, input, ok).map_err(|fault| match fault {
        Fault::NoStart(why) => t!("installer-run-no-start", program = argv[0], why = why),
        Fault::Stopped(why) => t!("installer-run-stopped", program = argv[0], why = why),
        Fault::Failed(err) => {
            let last = err.lines().rev().find(|l| !l.trim().is_empty()).unwrap_or("").trim();
            t!("installer-run-failed", program = argv[0], why = last)
        }
    })
}

fn chroot(argv: &[&str]) -> Step {
    let mut all = vec!["chroot", TARGET];
    all.extend_from_slice(argv);
    run(&all)
}

fn chroot_input(argv: &[&str], input: &str) -> Step {
    let mut all = vec!["chroot", TARGET];
    all.extend_from_slice(argv);
    run_input(&all, Some(input))
}

fn write(path: &str, body: &str) -> Step {
    let full = format!("{TARGET}{path}");
    if let Some(dir) = Path::new(&full).parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    std::fs::write(&full, body).map_err(|e| t!("installer-write-failed", path = path, why = e.to_string()))
}

fn uuid(dev: &str) -> String {
    text(&["blkid", "-s", "UUID", "-o", "value", dev])
}

fn uuid_of(dev: &str) -> Result<String, String> {
    let id = uuid(dev);
    if id.is_empty() { Err(t!("installer-uuid-missing", dev = dev)) } else { Ok(id) }
}

fn number_of(part: &str) -> String {
    let digits: String = part.chars().rev().take_while(|c| c.is_ascii_digit()).collect();
    digits.chars().rev().collect()
}

fn settle() {
    let _ = run(&["udevadm", "settle"]);
    let _ = run(&["partprobe"]);
    let _ = run(&["udevadm", "settle"]);
}

struct Places {
    esp: String,
    boot: Option<String>,
    root: String,
    locked: Option<String>,
    fresh_esp: bool,
}

impl Places {
    fn fs(&self) -> String {
        if self.locked.is_some() { MAPPED.to_string() } else { self.root.clone() }
    }
}

fn block_device(dev: &str) -> Step {
    let name = dev.strip_prefix("/dev/").filter(|n| !n.is_empty())
        .ok_or_else(|| t!("installer-not-a-disk", dev = dev))?;
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        return Err(t!("installer-not-a-disk", dev = dev));
    }
    if !std::fs::metadata(dev).is_ok_and(|m| {
        use std::os::unix::fs::FileTypeExt;
        m.file_type().is_block_device()
    }) {
        return Err(t!("installer-not-a-disk-here", dev = dev));
    }
    Ok(())
}

fn plain_word(refusal: &str, value: &str, extra: &str) -> Step {
    if value.len() > 64 || value.starts_with('-')
        || !value.chars().all(|c| c.is_ascii_alphanumeric() || extra.contains(c))
    {
        return Err(refusal.to_string());
    }
    Ok(())
}

fn erase_fault(disk: &str, listing: &str, live: &str) -> Option<String> {
    if disk == live {
        return Some(t!("installer-disk-is-stick", dev = disk));
    }
    let rows: Vec<Vec<&str>> = listing.lines().map(|l| l.split(' ').collect()).collect();
    if !rows.first().is_some_and(|r| r.first() == Some(&disk) && r.get(1) == Some(&"disk")) {
        return Some(t!("installer-not-a-disk", dev = disk));
    }
    let open_volume = |r: &Vec<&str>| r.get(1).is_some_and(|t| *t == "crypt" || *t == "lvm" || t.starts_with("raid"));
    if rows.iter().any(|r| r.get(2).is_some_and(|m| !m.is_empty()) || open_volume(r)) {
        return Some(t!("installer-disk-in-use", dev = disk));
    }
    None
}

fn holds_something(mountpoints: &str) -> bool {
    mountpoints.lines().any(|m| !m.trim().is_empty())
}

fn table_kind(listing: &str) -> Option<&str> {
    listing.lines().nth(1)?.split(':').nth(5)
}

const ESP_SHARE_LEAST: u64 = 50 * 1024 * 1024;

fn beside_fault(disk: &str) -> Step {
    if table_kind(&text(&["parted", "-sm", disk, "unit", "B", "print"])) != Some("gpt") {
        return Err(t!("installer-no-gpt", dev = disk));
    }
    existing_esp(disk).map(|_| ())
}

fn check(plan: &Plan) -> Step {
    if super::plan::short_of_memory() {
        return Err(t!("installer-memory-short").into());
    }
    if plan.lock != Lock::None && !super::plan::uefi() {
        return Err(t!("installer-lock-needs-uefi").into());
    }
    if plan.lock != Lock::None && plan.disk_password.is_empty() {
        return Err(t!("installer-disk-password-needed").into());
    }
    disk_fault(plan).inspect_err(|_| refuse(DISK_REFUSED))?;
    check_words(plan)
}

fn disk_fault(plan: &Plan) -> Step {
    match plan.target.as_ref().ok_or(t!("installer-no-disk-chosen"))? {
        Where::Erase { disk } => {
            block_device(disk)?;
            let listing = text(&["lsblk", "-rnpo", "PATH,TYPE,MOUNTPOINT", disk]);
            if let Some(fault) = erase_fault(disk, &listing, &super::plan::live_disk()) {
                return Err(fault);
            }
            if holds_install_image(&text(&["lsblk", "-rno", "FSTYPE", disk])) {
                return Err(t!("installer-disk-is-stick", dev = disk));
            }
        }
        Where::Beside { part, .. } => {
            block_device(part)?;
            block_device(&whole_disk(part))?;
            if whole_disk(part) == super::plan::live_disk() {
                return Err(t!("installer-disk-is-stick", dev = part));
            }
            if holds_something(&text(&["lsblk", "-rnpo", "MOUNTPOINT", part])) {
                return Err(t!("installer-disk-in-use", dev = part));
            }

            if text(&["blkid", "-s", "TYPE", "-o", "value", part]) == "ntfs" && crate::common::otheros::windows_asleep(part) {
                return Err(t!("installer-windows-asleep", part = part));
            }
            beside_fault(&whole_disk(part))?;
        }
    }

    let disk = plan.disk();
    if plan.disk_id.is_empty() || super::plan::disk_id(&disk) != plan.disk_id {
        refuse(CHANGED);
        return Err(t!("installer-disk-changed", dev = disk));
    }
    Ok(())
}

fn holds_install_image(fstypes: &str) -> bool {
    fstypes.lines().any(|f| f.trim() == "iso9660")
}

fn check_words(plan: &Plan) -> Step {
    plain_word(t!("installer-bad-language"), &plan.locale, "_.-")?;
    plain_word(t!("installer-bad-language"), &plan.trindle_language, "-")?;
    plain_word(t!("installer-bad-keyboard"), &plan.layout, "-")?;
    plain_word(t!("installer-bad-keyboard"), &plan.variant, "-_")?;
    plain_word(t!("installer-bad-timezone"), &plan.timezone, "/_+-")?;
    if plan.timezone.contains("..") || plan.timezone.starts_with('/') {
        return Err(t!("installer-bad-timezone").into());
    }
    if plan.disk_password.contains(['\n', '\r']) {
        return Err(t!("installer-bad-password").into());
    }
    Ok(())
}

fn check_answers(plan: &Plan) -> Step {
    if super::plan::username_fault(&plan.username).is_some() {
        return Err(t!("installer-bad-username").into());
    }
    if super::plan::hostname_fault(&plan.hostname).is_some() {
        return Err(t!("installer-bad-hostname").into());
    }
    if plan.fullname.contains([':', ',', '\n', '\r']) || plan.fullname.chars().any(char::is_control) {
        return Err(t!("installer-bad-fullname").into());
    }
    if plan.password.contains(['\n', '\r']) {
        return Err(t!("installer-bad-password").into());
    }
    Ok(())
}

pub const STOPPED: i32 = 4;

pub const STUCK: i32 = 5;

pub const CHANGED: i32 = 6;

pub const REFUSED: i32 = 7;
pub const DISK_REFUSED: i32 = 8;
static STOP: AtomicBool = AtomicBool::new(false);
static REFUSAL: AtomicI32 = AtomicI32::new(0);

fn refuse(code: i32) {
    let _ = REFUSAL.compare_exchange(0, code, Ordering::SeqCst, Ordering::SeqCst);
}

pub fn refusal() -> Option<i32> {
    Some(REFUSAL.load(Ordering::SeqCst)).filter(|&c| c != 0)
}
static COPYING: AtomicU32 = AtomicU32::new(0);

pub fn stop() {
    STOP.store(true, Ordering::SeqCst);
    let pid = COPYING.load(Ordering::SeqCst);
    if pid != 0 {
        let _ = run(&["kill", "-TERM", &pid.to_string()]);
    }
}

pub fn stopped() -> bool {
    STOP.load(Ordering::SeqCst)
}

fn go_on() -> Step {
    if stopped() { Err(t!("installer-stopped").into()) } else { Ok(()) }
}

pub fn let_go() -> bool {
    unmount();
    for _ in 0..LET_GO_TRIES {
        if !std::path::Path::new(MAPPED).exists() {
            break;
        }
        let _ = run(&["cryptsetup", "close", "trindle"]);
        settle();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    let mounts = std::fs::read_to_string("/proc/self/mounts").unwrap_or_default();
    !std::path::Path::new(MAPPED).exists() && !mounts.contains(TARGET) && !mounts.contains(TOP)
}

struct PowerHeld(Option<std::process::Child>);

impl PowerHeld {
    fn take() -> PowerHeld {
        let why = format!("--why={}", t!("installer-power-held"));
        let child = std::process::Command::new("systemd-inhibit")
            .args(["--what=shutdown:sleep:idle:handle-power-key:handle-suspend-key:handle-hibernate-key:handle-lid-switch",
                   "--who=Trindle", &why, "--mode=block", "sleep", "infinity"])
            .stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null())
            .spawn();
        PowerHeld(child.inspect_err(|e| eprintln!("trindle-installer: systemd-inhibit: {e}")).ok())
    }
}

impl Drop for PowerHeld {
    fn drop(&mut self) {
        if let Some(mut c) = self.0.take() {
            let _ = c.kill();
            let _ = c.wait();
        }
    }
}

fn one_installer() -> Result<std::fs::File, String> {
    use std::os::fd::AsRawFd;
    let file = std::fs::OpenOptions::new().create(true).write(true).truncate(false).open(crate::common::installing::INSTALL_LOCK)
        .map_err(|e| t!("installer-plan-unreadable-why", why = e.to_string()))?;
    // SAFETY: file owns a valid descriptor for the call.
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err(t!("installer-already-running").into());
    }
    Ok(file)
}

fn ready_to_change(plan: &Plan) -> Result<std::fs::File, String> {
    // SAFETY: asks the kernel for this process's own effective user id.
    if unsafe { libc::geteuid() } != 0 {
        return Err(t!("installer-needs-root").into());
    }

    std::env::set_var("PATH", "/usr/sbin:/usr/bin:/sbin:/bin");
    let only_one = one_installer()?;
    check(plan)?;
    Ok(only_one)
}

pub fn install(plan: &Plan, answers: &Receiver<String>) -> Step {

    let _only_one = ready_to_change(plan).inspect_err(|_| refuse(REFUSED))?;

    let _awake = PowerHeld::take();

    say(1, t!("installer-step-room"));
    let mut places = partition(plan)?;
    say(6, t!("installer-step-prepare"));
    go_on()?;
    lock_up(plan, &mut places)?;
    format(&places)?;
    mount(&places)?;
    go_on()?;
    let resume = if plan.swap { swap_file(plan).map_err(|why| problem(t!("installer-swap-failed", why = why))).ok() } else { None };
    say(10, t!("installer-step-copy"));
    copy_system()?;
    go_on()?;
    say(69, t!("installer-step-waiting"));
    let json = Zeroizing::new(answers.recv().map_err(|_| t!("installer-stopped").to_string())?);
    let full: Plan = serde_json::from_str(&json).map_err(|e| t!("installer-plan-unreadable-why", why = e.to_string()))?;
    if full.target != plan.target || full.lock != plan.lock || full.swap != plan.swap || full.swap_gib != plan.swap_gib
        || full.disk_id != plan.disk_id || full.disk_password != plan.disk_password {
        return Err(t!("installer-plan-unreadable").into());
    }
    check_words(&full)?;
    check_answers(&full)?;
    fill(&full, &places, resume)?;
    unmount();
    Ok(())
}

fn partition(plan: &Plan) -> Result<Places, String> {
    let separate_boot = plan.lock != Lock::None;
    match plan.target.clone().unwrap() {
        Where::Erase { disk } => {

            run(&["wipefs", "-a", &disk])?;
            let esp_end = 3 + ESP_MB;
            let mut argv: Vec<String> = ["parted", "-s", "-a", "optimal", &disk, "mklabel", "gpt",
                "mkpart", "bios", "1MiB", "3MiB", "set", "1", "bios_grub", "on",
                "mkpart", "boot", "fat32", "3MiB", &format!("{esp_end}MiB"), "set", "2", "esp", "on"]
                .iter().map(|s| s.to_string()).collect();
            let root_start = if separate_boot { esp_end + BOOT_MB } else { esp_end };
            if separate_boot {
                argv.extend(["mkpart", "start", "ext4", &format!("{esp_end}MiB"), &format!("{root_start}MiB")]
                    .iter().map(|s| s.to_string()));
            }
            argv.extend(["mkpart", "trindle", "btrfs", &format!("{root_start}MiB"), "100%"]
                .iter().map(|s| s.to_string()));
            let args: Vec<&str> = argv.iter().map(String::as_str).collect();
            run(&args)?;
            settle();
            Ok(Places {
                esp: part_name(&disk, 2),
                boot: separate_boot.then(|| part_name(&disk, 3)),
                root: part_name(&disk, if separate_boot { 4 } else { 3 }),
                locked: None,
                fresh_esp: true,
            })
        }
        Where::Beside { part, leave } => {
            let disk = whole_disk(&part);

            let esp = existing_esp(&disk)?;
            shrink(&part, leave)?;
            settle();
            say(4, t!("installer-step-place"));
            let mut made = Vec::new();
            let mut start = end_of(&part)? + 1;

            let room_end = next_start(&text(&["parted", "-sm", &disk, "unit", "B", "print"]), start).map(|n| n - 1);
            if separate_boot {
                let boot = add_part(&disk, "start", "ext4", start, Some(start + BOOT_MB * 1024 * 1024))?;

                start = end_of(&boot)? + 1;
                made.push(boot);
            }
            made.push(add_part(&disk, "trindle", "btrfs", start, room_end)?);
            Ok(Places {
                esp,
                boot: separate_boot.then(|| made[0].clone()),
                root: made.last().unwrap().clone(),
                locked: None,
                fresh_esp: false,
            })
        }
    }
}

fn next_start(listing: &str, from: u64) -> Option<u64> {
    listing.lines()
        .filter(|l| l.starts_with(|c: char| c.is_ascii_digit()))
        .filter_map(|l| l.split(':').nth(1)?.trim_end_matches('B').parse::<u64>().ok())
        .filter(|s| *s >= from)
        .min()
}

fn add_part(disk: &str, name: &str, fs: &str, start: u64, end: Option<u64>) -> Result<String, String> {
    let before = partition_list(disk);
    let stop = end.map(|e| format!("{e}B")).unwrap_or_else(|| "100%".into());
    run(&["parted", "-s", "-a", "optimal", disk, "mkpart", name, fs, &format!("{start}B"), &stop])?;
    settle();
    partition_list(disk).into_iter().find(|p| !before.contains(p))
        .ok_or_else(|| t!("installer-new-part-missing").to_string())
}

fn partition_list(disk: &str) -> Vec<String> {
    text(&["lsblk", "-lno", "PATH,TYPE", disk]).lines()
        .filter(|l| l.trim_end().ends_with("part"))
        .filter_map(|l| l.split_whitespace().next().map(String::from))
        .collect()
}

fn edge_of(part: &str, field: usize) -> Result<u64, String> {
    let disk = whole_disk(part);
    let n = number_of(part);
    text(&["parted", "-sm", &disk, "unit", "B", "print"]).lines()
        .find(|l| l.starts_with(&format!("{n}:")))
        .and_then(|l| l.split(':').nth(field).map(|e| e.trim_end_matches('B').to_string()))
        .and_then(|e| e.parse::<u64>().ok())
        .ok_or_else(|| t!("installer-size-unreadable", part = part))
}

fn end_of(part: &str) -> Result<u64, String> {
    edge_of(part, 2)
}

fn existing_esp(disk: &str) -> Result<String, String> {
    esp_in(&text(&["lsblk", "-lnbo", "PATH,SIZE,FSTYPE,PARTTYPENAME", disk]))
}

fn esp_in(listing: &str) -> Result<String, String> {
    let found = listing.lines()
        .find(|l| l.contains("EFI System") || (l.contains("vfat") && l.to_lowercase().contains("efi")))
        .ok_or(t!("installer-no-boot-partition"))?;
    let mut words = found.split_whitespace();
    let (dev, size) = (words.next().unwrap_or_default(), words.next().and_then(|n| n.parse::<u64>().ok()).unwrap_or(0));
    if size < ESP_SHARE_LEAST {
        return Err(t!("installer-boot-partition-small").into());
    }
    Ok(dev.to_string())
}

fn kept(leave: u64, size: u64) -> Result<u64, String> {
    if size < KEEP_FREE + LEAST {
        return Err(t!("installer-disks-none").into());
    }
    Ok(leave.clamp(KEEP_FREE, size - LEAST))
}

fn shrink(part: &str, leave: u64) -> Step {
    let size = text(&["lsblk", "-bndo", "SIZE", part]).trim().parse().unwrap_or(0);
    let leave = kept(leave, size)?;
    let fs = text(&["blkid", "-s", "TYPE", "-o", "value", part]);
    say(2, t!("installer-step-check"));
    match fs.as_str() {
        "ntfs" => {
            run(&["ntfsresize", "--no-progress-bar", "--force", "--info", part])?;
            say(3, t!("installer-step-smaller"));
            run(&["ntfsresize", "--no-progress-bar", "--force", "--size", &format!("{leave}"), part])?;
            let _ = run(&["ntfsfix", "-d", part]);
        }
        "ext2" | "ext3" | "ext4" => {

            run_with(&["e2fsck", "-f", "-y", part], None, &[1])?;
            say(3, t!("installer-step-smaller"));
            run(&["resize2fs", part, &format!("{}K", leave / 1024)])?;
        }
        "" => return Err(t!("installer-cannot-shrink-unknown").into()),
        other => return Err(t!("installer-cannot-shrink", fs = other)),
    }
    let n = number_of(part);

    let size_kib = (leave + 32 * 1024 * 1024) / 1024;
    run_input(&["sfdisk", "--no-reread", "-N", &n, &whole_disk(part)], Some(&format!(", {size_kib}KiB\n")))
}

fn lock_up(plan: &Plan, places: &mut Places) -> Step {
    if plan.lock == Lock::None {
        return Ok(());
    }
    say(7, t!("installer-step-lock"));
    let key = &plan.disk_password;
    if key.is_empty() {
        return Err(t!("installer-disk-password-needed").into());
    }
    run_input(&["cryptsetup", "luksFormat", "--type", "luks2", "--batch-mode", "--key-file", "-",
                "--pbkdf", "argon2id", "--label", "trindle", &places.root], Some(key))?;
    run_input(&["cryptsetup", "open", "--key-file", "-", &places.root, "trindle"], Some(key))?;
    places.locked = Some(places.root.clone());
    Ok(())
}

use trindle_shared::luks_chip::POLICY as CHIP_POLICY;

fn arrange_chip_binding(plan: &Plan, places: &Places) -> Step {
    if plan.lock != Lock::Chip {
        return Ok(());
    }
    say(93, t!("installer-step-chip"));
    if !chip_or_password(TARGET, || teach_chip(plan, places))? {
        problem(t!("installer-chip-not-set-up").into());
    }
    Ok(())
}

const CHIP_UNIT: &str = "[Unit]\n\
                    Description=Teach this computer's chip to open the disk\n\
                    After=multi-user.target\n\
                    ConditionPathExists=/etc/trindle/tpm-key\n\
                    \n\
                    [Service]\n\
                    Type=oneshot\n\
                    ExecStart=/etc/trindle/chip.sh\n\
                    ExecStopPost=/bin/rm -f /etc/trindle/tpm-key\n\
                    \n\
                    [Install]\n\
                    WantedBy=multi-user.target\n";

const CHIP_FILES: [&str; 4] = ["/etc/trindle/tpm-key", "/etc/trindle/chip.sh", "/etc/systemd/system/trindle-chip.service",
                               "/etc/systemd/system/multi-user.target.wants/trindle-chip.service"];

fn chip_or_password(root: &str, teach: impl FnOnce() -> Step) -> Result<bool, String> {
    if teach().is_ok() {
        return Ok(true);
    }
    for path in CHIP_FILES {
        gone(&format!("{root}{path}"))?;
    }
    Ok(false)
}

fn teach_chip(plan: &Plan, places: &Places) -> Step {
    let part = places.locked.as_ref().ok_or_else(String::new)?;

    let part = format!("/dev/disk/by-uuid/{}", uuid_of(part)?);

    run_input(&["clevis", "encrypt", "tpm2", CHIP_POLICY], Some("trindle"))?;

    write_private("/etc/trindle/tpm-key", &plan.disk_password, 0o600)?;
    write_private("/etc/trindle/chip.sh", &chip_script(&part, &format!("'{CHIP_POLICY}'")), 0o700)?;
    write("/etc/systemd/system/trindle-chip.service", CHIP_UNIT)?;
    chroot(&["systemctl", "enable", "trindle-chip.service"])
}

fn chip_script(part: &str, policy: &str) -> String {
    format!("#!/bin/sh\n\
             set -e\n\
             [ -s /etc/trindle/tpm-key ] || exit 0\n\
             trap 'shred -u /etc/trindle/tpm-key 2>/dev/null || rm -f /etc/trindle/tpm-key; \
             systemctl disable trindle-chip.service 2>/dev/null || true' EXIT\n\
             trap 'exit 1' HUP INT TERM\n\
             clevis luks list -d {part} | grep -q tpm2 || \
             clevis luks bind -y -k /etc/trindle/tpm-key -d {part} tpm2 {policy}\n\
             update-initramfs -u -k all\n")
}

fn write_private(path: &str, body: &str, mode: u32) -> Step {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    let full = format!("{TARGET}{path}");
    if let Some(dir) = Path::new(&full).parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let mut f = std::fs::OpenOptions::new().write(true).create(true).truncate(true).mode(mode)
        .open(&full)
        .map_err(|e| t!("installer-write-failed", path = path, why = e.to_string()))?;
    f.write_all(body.as_bytes()).map_err(|e| t!("installer-write-failed", path = path, why = e.to_string()))
}

pub fn has_chip() -> bool {
    std::fs::read_to_string("/sys/class/tpm/tpm0/tpm_version_major").map(|v| v.trim() == "2").unwrap_or(false)
}

fn format(places: &Places) -> Step {
    if places.fresh_esp {
        run(&["mkfs.vfat", "-F", "32", "-n", "TRINDLE", &places.esp])?;
    }
    if let Some(boot) = &places.boot {
        run(&["mkfs.ext4", "-F", "-L", "trindle-start", boot])?;
    }
    run(&["mkfs.btrfs", "-f", "-L", "trindle", &places.fs()])?;
    std::fs::create_dir_all(TOP).map_err(|e| e.to_string())?;
    run(&["mount", &places.fs(), TOP])?;
    let sub = |name: &str| run(&["btrfs", "subvolume", "create", &format!("{TOP}/{name}")]);

    let made = sub("@rootfs").and_then(|_| sub("@home")).and_then(|_| sub("@swap"));
    let _ = run(&["umount", TOP]);
    made
}

fn mount(places: &Places) -> Step {
    let fs = places.fs();
    std::fs::create_dir_all(TARGET).map_err(|e| e.to_string())?;
    run(&["mount", "-o", "subvol=@rootfs,compress=zstd:1", &fs, TARGET])?;
    let at = |sub: &str, dir: &str| -> Step {
        std::fs::create_dir_all(format!("{TARGET}{dir}")).map_err(|e| e.to_string())?;
        run(&["mount", "-o", &format!("subvol={sub},compress=zstd:1"), &fs, &format!("{TARGET}{dir}")])
    };
    at("@home", "/home")?;
    at("@swap", "/swap")?;
    if let Some(boot) = &places.boot {
        std::fs::create_dir_all(format!("{TARGET}/boot")).map_err(|e| e.to_string())?;
        run(&["mount", boot, &format!("{TARGET}/boot")])?;
    }
    std::fs::create_dir_all(format!("{TARGET}/boot/efi")).map_err(|e| e.to_string())?;
    run(&["mount", &places.esp, &format!("{TARGET}/boot/efi")])
}

fn unmount() {
    for m in ["/dev/pts", "/dev", "/proc", "/sys", "/run", "/boot/efi", "/boot", "/swap", "/home", ""] {
        let _ = run(&["umount", "-l", &format!("{TARGET}{m}")]);
    }
    let _ = run(&["umount", "-l", TOP]);
}

fn copy_system() -> Step {
    let excludes = ["/proc/*", "/sys/*", "/dev/*", "/run/*", "/tmp/*", "/mnt/*", "/media/*",
                    "/lost+found", "/var/log/*", "/var/cache/apt/archives/*.deb", "/swapfile",
                    "/etc/sudoers.d/live", "/usr/local/share/applications/trindle-installer.desktop",
                    "/home/*"];
    let mut argv = vec!["rsync".to_string(), "-aHAXx".into(), "--info=progress2".into(), "--no-inc-recursive".into()];
    for e in excludes {
        argv.push(format!("--exclude={e}"));
    }
    argv.push("/".into());
    argv.push(format!("{TARGET}/"));
    let args: Vec<&str> = argv.iter().map(String::as_str).collect();

    let mut child = crate::common::gentle::program(&mut Command::new(args[0])).args(&args[1..])
        .stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()
        .map_err(|e| t!("installer-copy-no-start", why = e.to_string()))?;
    COPYING.store(child.id(), Ordering::SeqCst);
    let errors = child.stderr.take().map(|e| std::thread::spawn(move || last_lines(e)));

    if stopped() {
        stop();
    }
    if let Some(out) = child.stdout.take() {
        let mut reader = BufReader::new(out);
        let mut line = Vec::new();

        while reader.read_until(b'\r', &mut line).map(|n| n > 0).unwrap_or(false) {
            let s = String::from_utf8_lossy(&line).to_string();
            line.clear();
            if let Some(p) = s.split_whitespace().find(|w| w.ends_with('%')) {
                if let Ok(n) = p.trim_end_matches('%').parse::<u32>() {
                    say(10 + n.min(100) * 58 / 100, t!("installer-step-copy"));
                }
            }
        }
    }
    let status = child.wait().map_err(|e| e.to_string())?;
    COPYING.store(0, Ordering::SeqCst);
    let said = errors.and_then(|t| t.join().ok()).unwrap_or_default();

    match status.code() {
        Some(0) | Some(24) => Ok(()),
        _ => Err(t!("installer-copy-failed", why = said.as_str())),
    }
}

fn last_lines(from: impl std::io::Read) -> String {
    let mut last = String::new();
    let mut reader = BufReader::new(from);
    let mut line = Vec::new();
    while reader.read_until(b'\n', &mut line).map(|n| n > 0).unwrap_or(false) {

        line.truncate(4096);
        let text = String::from_utf8_lossy(&line);
        if !text.trim().is_empty() {
            last = text.trim_end().to_string();
        }
        line.clear();
    }
    last
}

struct Resume {
    offset: String,
}

fn swap_file(plan: &Plan) -> Result<Resume, String> {
    let gib = plan.swap_size();
    if !super::plan::swap_sizes(super::plan::memory_gib()).contains(&gib) {
        return Err(t!("installer-plan-unreadable").into());
    }
    let free = text(&["df", "--output=avail", "-B1", &format!("{TARGET}/swap")])
        .lines().nth(1).and_then(|l| l.trim().parse::<u64>().ok()).unwrap_or(0);
    if !super::plan::swap_fits(free, gib) {
        return Err(t!("installer-swap-no-room").into());
    }
    let file = format!("{TARGET}/swap/swapfile");
    run(&["btrfs", "filesystem", "mkswapfile", "--size", &format!("{gib}g"), &file])?;
    let offset = text(&["btrfs", "inspect-internal", "map-swapfile", "-r", &file]).trim().to_string();
    if offset.is_empty() || !offset.chars().all(|c| c.is_ascii_digit()) {
        return Err(t!("installer-swap-unreadable").into());
    }
    Ok(Resume { offset })
}

fn resume_settings(places: &Places, resume: &Resume) -> Step {
    let r = uuid_of(&places.fs())?;
    write("/etc/initramfs-tools/conf.d/resume", "RESUME=none\n")?;
    write("/etc/default/grub.d/trindle-resume.cfg",
          &format!("# Where a hibernated computer's memory is, on the swap file.\n\
                    GRUB_CMDLINE_LINUX=\"$GRUB_CMDLINE_LINUX resume=UUID={r} resume_offset={}\"\n", resume.offset))
}

fn fill(plan: &Plan, places: &Places, resume: Option<Resume>) -> Step {
    say(70, t!("installer-step-settings"));
    fstab(places, resume.is_some())?;

    if let Some(r) = resume.as_ref().filter(|_| plan.swap_hibernates()) {
        resume_settings(places, r)?;
    }
    settings(plan)?;
    for d in ["dev", "dev/pts", "proc", "sys", "run"] {
        std::fs::create_dir_all(format!("{TARGET}/{d}")).map_err(|e| e.to_string())?;
        run(&["mount", "--bind", &format!("/{d}"), &format!("{TARGET}/{d}")])?;
    }
    let brand = super::brand::brand();
    go_on()?;
    remove_live_account()?;
    people(plan)?;
    say(80, t!("installer-step-live-parts"));
    live_parts(&brand, &plan.username)?;
    say(84, t!("installer-step-startup"));
    crypttab(places)?;

    if chroot(&["setupcon", "--force", "--save-only"]).is_err() && places.locked.is_some() {
        problem(t!("installer-start-keyboard-failed").into());
    }

    let _ = std::fs::copy(format!("{TARGET}/usr/share/trindle/initramfs-small.conf"),
                          format!("{TARGET}/etc/initramfs-tools/conf.d/trindle-small"));
    chroot(&["update-initramfs", "-u", "-k", "all"])?;
    go_on()?;
    say(90, t!("installer-step-boot"));
    boot(&plan.disk(), places)?;
    arrange_chip_binding(plan, places)?;
    say(96, t!("installer-step-snapshots"));
    snapshots();
    say(99, t!("installer-step-finishing"));
    if let Err(why) = chroot(&["systemctl", "enable", "greetd.service"]) {
        problem(t!("installer-login-screen-failed", why = why));
    }
    if let Some(after) = &brand.after {
        let _ = chroot(&["sh", "-c", after]);
    }
    Ok(())
}

fn fstab(places: &Places, swap: bool) -> Step {
    let (r, e) = (uuid(&places.fs()), uuid(&places.esp));
    if r.is_empty() || e.is_empty() {
        return Err(t!("installer-fstab-unidentified").into());
    }
    let mut body = format!(
        "# Written by the Trindle installer.\n\
         UUID={r} /     btrfs subvol=@rootfs,compress=zstd:1,noatime 0 0\n\
         UUID={r} /home btrfs subvol=@home,compress=zstd:1,noatime   0 0\n\
         UUID={r} /swap btrfs subvol=@swap,noatime                   0 0\n");
    if swap {
        body.push_str("/swap/swapfile none swap defaults 0 0\n");
    }
    if let Some(boot) = &places.boot {
        body.push_str(&format!("UUID={} /boot ext4 defaults 0 2\n", uuid_of(boot)?));
    }
    body.push_str(&format!("UUID={e} /boot/efi vfat umask=0077 0 1\n"));
    write("/etc/fstab", &body)
}

fn crypttab(places: &Places) -> Step {
    let Some(part) = &places.locked else { return Ok(()) };
    write("/etc/crypttab", &format!("trindle UUID={} none luks,discard\n", uuid_of(part)?))?;

    write("/etc/default/grub.d/trindle-lock.cfg", "GRUB_ENABLE_CRYPTODISK=y\n")
}

fn settings(plan: &Plan) -> Step {
    let locale = if plan.locale.is_empty() { "en_GB.UTF-8".to_string() } else { plan.locale.clone() };
    let gen = std::fs::read_to_string(format!("{TARGET}/etc/locale.gen")).unwrap_or_default();
    let wanted = format!("{locale} UTF-8");
    let body: String = gen.lines()
        .map(|l| if l.trim_start_matches("# ").starts_with(&wanted) { wanted.clone() } else { l.to_string() })
        .collect::<Vec<_>>().join("\n");
    write("/etc/locale.gen", &format!("{body}\n{wanted}\n"))?;
    write("/etc/default/locale", &format!("LANG={locale}\n"))?;
    let layout = if plan.layout.is_empty() { "gb" } else { &plan.layout };
    write("/etc/default/keyboard", &format!(
        "XKBMODEL=\"pc105\"\nXKBLAYOUT=\"{layout}\"\nXKBVARIANT=\"{}\"\nXKBOPTIONS=\"\"\nBACKSPACE=\"guess\"\n",
        plan.variant))?;
    if plan.terms {
        write(terms::SYSTEM_RECORD, &terms::record_text(terms::VERSION, &terms::today()))?;
    }
    write("/etc/hostname", &format!("{}\n", plan.hostname))?;
    write("/etc/hosts", &format!(
        "127.0.0.1\tlocalhost\n127.0.1.1\t{}\n::1\tlocalhost ip6-localhost ip6-loopback\n", plan.hostname))?;
    let zone = if plan.timezone.is_empty() { "UTC".to_string() } else { plan.timezone.clone() };
    write("/etc/timezone", &format!("{zone}\n"))?;
    let _ = std::fs::remove_file(format!("{TARGET}/etc/localtime"));
    std::os::unix::fs::symlink(format!("/usr/share/zoneinfo/{zone}"), format!("{TARGET}/etc/localtime"))
        .map_err(|e| t!("installer-timezone-failed", why = e.to_string()))?;

    write("/etc/skel/.config/trindle/environment",
          &format!("XKB_DEFAULT_LAYOUT={layout}\nXKB_DEFAULT_VARIANT={}\n", plan.variant))?;

    let conf = "/etc/skel/.config/trindle/trindle.conf";
    let old = std::fs::read_to_string(format!("{TARGET}{conf}")).unwrap_or_default();
    let mut lines: Vec<String> = old.lines().filter(|l| !l.trim_start().starts_with("language=")).map(str::to_string).collect();
    lines.extend(plan.language_setting());
    if lines.is_empty() && old.is_empty() {
        return Ok(());
    }
    write(conf, &(lines.join("\n") + "\n"))
}

fn people(plan: &Plan) -> Step {
    say(74, t!("installer-step-account"));
    chroot(&["locale-gen"])?;
    let groups = "sudo,audio,video,input,netdev,plugdev,lpadmin,cdrom,dialout";
    chroot(&["useradd", "-m", "-s", "/bin/bash", "-c", &plan.fullname, "-G", groups, &plan.username])?;
    chroot_input(&["chpasswd"], &format!("{}:{}\n", plan.username, plan.password))?;

    chroot(&["passwd", "-l", "root"])
}

fn remove_live_account() -> Step {
    let passwd = || std::fs::read_to_string(format!("{TARGET}/etc/passwd")).unwrap_or_default();
    let Some(live_user) = live_account(&passwd()) else { return Ok(()) };

    let tried = chroot(&["userdel", "-r", &live_user]);
    match (live_account(&passwd()), tried) {
        (Some(_), Err(why)) => Err(why),
        (Some(_), Ok(())) => Err(t!("installer-live-account-left", user = live_user)),
        (None, _) => Ok(()),
    }
}

fn live_account(passwd: &str) -> Option<String> {
    passwd.lines().find(|l| l.split(':').nth(2) == Some("1000"))
        .and_then(|l| l.split(':').next()).filter(|u| !u.is_empty()).map(str::to_string)
}

fn live_parts(brand: &super::brand::Brand, username: &str) -> Step {

    write("/etc/machine-id", "")?;
    gone(&format!("{TARGET}/var/lib/dbus/machine-id"))?;

    if let Ok(dir) = std::fs::read_dir(format!("{TARGET}/etc/ssh")) {
        for e in dir.flatten() {
            if e.file_name().to_string_lossy().starts_with("ssh_host_") {
                gone(&e.path().to_string_lossy())?;
            }
        }
    }
    for f in ["/etc/sudoers.d/live", "/etc/xdg/autostart/trindle-installer.desktop",
              "/usr/local/share/applications/trindle-installer.desktop",
              "/etc/skel/.config/autostart/trindle-live-welcome.desktop",
              "/etc/skel/.config/autostart/trindle-welcome.desktop"] {
        gone(&format!("{TARGET}{f}"))?;
    }

    let homes = ["/etc/skel".to_string(), format!("/home/{username}")];
    for home in &homes {

        for welcome in ["trindle-live-welcome", "trindle-welcome"] {
            gone(&format!("{TARGET}{home}/.config/autostart/{welcome}.desktop"))?;
        }
        for (file, drop) in [(".config/trindle/favourites", "trindle-installer"),
                             (".config/trindle/trindle.conf", "lock="),
                             (".config/trindle/trindle.conf", "clipboard_forget=")] {
            let path = format!("{TARGET}{home}/{file}");
            if let Ok(body) = std::fs::read_to_string(&path) {
                let kept: Vec<&str> = body.lines().filter(|l| !l.contains(drop)).collect();
                let _ = std::fs::write(&path, kept.join("\n") + "\n");
            }
        }
    }

    write("/etc/trindle/greetd.toml", GREETD)?;
    if !brand.remove.is_empty() {
        let mut argv = vec!["apt-get", "purge", "-y"];
        argv.extend(brand.remove.iter().map(String::as_str));
        let _ = chroot(&argv);
    }
    let mut tidy = crate::admin::protected::autoremove_argv(false);
    tidy.insert(1, "--purge".into());
    let _ = chroot(&tidy.iter().map(String::as_str).collect::<Vec<_>>());

    if let Err(why) = chroot(&["apt-mark", "unhold", "trindle"]) {
        problem(t!("installer-updates-held", why = why));
    }
    Ok(())
}

fn gone(path: &str) -> Step {
    match std::fs::remove_file(path) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(t!("installer-write-failed", path = path, why = e.to_string())),
        _ => Ok(()),
    }
}

fn boot(disk: &str, places: &Places) -> Step {
    let efi = Path::new("/sys/firmware/efi").exists();
    let mut efi_ok = false;
    let mut bios_ok = false;
    if efi {

        efi_ok = chroot(&["grub-install", "--target=x86_64-efi", "--efi-directory=/boot/efi",
                         "--bootloader-id=Trindle", "--no-nvram", "--recheck"]).is_ok();

        if places.fresh_esp {
            let _ = chroot(&["grub-install", "--target=x86_64-efi", "--efi-directory=/boot/efi",
                             "--removable", "--no-nvram", "--recheck"]);
        }
        name_in_firmware(disk, &places.esp);
    }
    if Path::new(&format!("{TARGET}/usr/lib/grub/i386-pc")).exists() {
        bios_ok = chroot(&["grub-install", "--target=i386-pc", "--recheck", disk]).is_ok();
    }

    if !(if efi { efi_ok } else { bios_ok }) {
        return Err(t!("installer-boot-failed").into());
    }

    let _ = chroot(&["os-prober"]);
    chroot(&["update-grub"])
}

pub const FIRMWARE_NAME: &str = "Trindle";

fn name_in_firmware(disk: &str, esp: &str) {
    let list = Command::new("efibootmgr").output().map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default();
    for number in firmware_entries_named(&list, FIRMWARE_NAME) {
        let _ = run(&["efibootmgr", "--quiet", "--bootnum", &number, "--delete-bootnum"]);
    }

    let dir = format!("{TARGET}/boot/efi/EFI/Trindle");
    let loader = if Path::new(&format!("{dir}/shimx64.efi")).exists() {
        r"\EFI\Trindle\shimx64.efi"
    } else {
        r"\EFI\Trindle\grubx64.efi"
    };
    if let Err(why) = run(&["efibootmgr", "--quiet", "--create", "--disk", disk, "--part", &number_of(esp),
                            "--label", FIRMWARE_NAME, "--loader", loader]) {
        problem(t!("installer-firmware-entry-failed", why = why));
    }
}

fn firmware_entries_named(list: &str, name: &str) -> Vec<String> {
    list.lines().filter_map(|line| {
        let rest = line.strip_prefix("Boot")?;
        let (number, label) = rest.split_once(['*', ' '])?;
        if number.len() != 4 || !number.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }
        let label = label.trim_start_matches(['*', ' ']);
        let label = label.split('\t').next().unwrap_or(label).trim_end();
        (label == name).then(|| number.to_string())
    }).collect()
}

fn snapshots() {
    let made = chroot(&["snapper", "--no-dbus", "-c", "root", "create-config", "/"]).and_then(|()| {
        chroot(&["snapper", "--no-dbus", "-c", "root", "set-config", "TIMELINE_CREATE=no",
                 "NUMBER_LIMIT=10", "NUMBER_LIMIT_IMPORTANT=5", "ALLOW_GROUPS=sudo", "SYNC_ACL=yes"])
    });
    if let Err(why) = made {
        problem(t!("installer-snapshots-failed", why = why));
    }
    if !home_snapshots() {
        problem(t!("installer-home-snapshots-failed").into());
    }
    if let Err(why) = chroot(&["systemctl", "enable", "trindle-snapshots-grub.path"]).and_then(|()| chroot(&["update-grub"])) {
        problem(t!("installer-snapshots-menu-failed", why = why));
    }
}

fn home_snapshots() -> bool {
    use crate::common::home_snapshots as home;
    use std::os::unix::fs::PermissionsExt;
    for command in home::setup_commands(false, home::KEEP_DEFAULT) {
        if chroot(&command.iter().map(String::as_str).collect::<Vec<_>>()).is_err() {
            return false;
        }
    }
    let folder = format!("{TARGET}{}", home::SNAPSHOTS);
    std::fs::symlink_metadata(&folder).is_ok_and(|m| m.is_dir())
        && std::fs::set_permissions(&folder, std::fs::Permissions::from_mode(home::FOLDER_MODE)).is_ok()
        && write(home::SETTING, &home::Setting { on: true, days: home::KEEP_DEFAULT }.text()).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_exit_code_listed_as_fine_is_not_a_failure() {
        assert!(run_with(&["sh", "-c", "exit 1"], None, &[1]).is_ok());
        assert!(run(&["sh", "-c", "exit 1"]).is_err());
        assert!(run_with(&["sh", "-c", "exit 4"], None, &[1]).is_err());
    }

    #[test]
    fn a_child_that_writes_a_lot_of_errors_is_not_left_waiting() {
        let mut child = Command::new("sh").args(["-c", "head -c 1000000 /dev/zero | tr '\\0' e >&2; echo >&2; echo last >&2; echo out"])
            .stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        let errors = child.stderr.take().map(|e| std::thread::spawn(move || last_lines(e)));
        let mut out = String::new();
        std::io::Read::read_to_string(&mut child.stdout.take().unwrap(), &mut out).unwrap();
        assert!(child.wait().unwrap().success());
        assert_eq!(errors.unwrap().join().unwrap(), "last");
    }

    #[test]
    fn the_new_partition_stops_where_the_next_one_starts() {
        let listing = "BYT;\n/dev/sda:900B:scsi:512:512:gpt:Disk:;\n1:10B:99B:90B:fat32::;\n2:100B:499B:400B:ntfs::;\n3:600B:899B:300B:ext4::;\n";
        assert_eq!(next_start(listing, 500), Some(600));
        assert_eq!(next_start(listing, 900), None);
    }

    #[test]
    fn sharing_a_disk_needs_a_gpt_table_and_a_boot_partition_big_enough() {
        let gpt = "BYT;\n/dev/sda:900B:scsi:512:512:gpt:Disk:;\n";
        let mbr = "BYT;\n/dev/sda:900B:scsi:512:512:msdos:Disk:;\n";
        assert_eq!(table_kind(gpt), Some("gpt"));
        assert_eq!(table_kind(mbr), Some("msdos"));
        assert_eq!(table_kind(""), None);
        let big = "/dev/sda1 104857600 vfat EFI System\n/dev/sda2 5000000000 ntfs Microsoft basic data\n";
        assert_eq!(esp_in(big), Ok("/dev/sda1".to_string()));
        assert_eq!(esp_in("/dev/sda1 1048576 vfat EFI System\n"), Err(t!("installer-boot-partition-small").to_string()));
        assert_eq!(esp_in("/dev/sda2 5000000000 ntfs Microsoft basic data\n"), Err(t!("installer-no-boot-partition").to_string()));
    }

    #[test]
    #[allow(clippy::field_reassign_with_default)]
    fn answers_sent_after_the_copy_are_checked_like_the_first() {
        let mut plan = Plan::default();
        plan.locale = "en_GB.UTF-8".into();
        plan.layout = "gb".into();
        plan.timezone = "Europe/London".into();
        assert!(check_words(&plan).is_ok());
        plan.layout = "gb\"\nEVIL=1".into();
        assert!(check_words(&plan).is_err());
        plan.layout = "gb".into();
        plan.timezone = "../../etc/passwd".into();
        assert!(check_words(&plan).is_err());
    }

    #[test]
    fn a_stick_is_known_by_the_install_image_on_it() {
        assert!(holds_install_image("iso9660\nvfat\n"));
        assert!(!holds_install_image("ntfs\nvfat\n"));
    }

    #[test]
    fn a_tool_that_stops_before_it_has_its_input_is_not_called_done() {
        let much = "x".repeat(1 << 20);
        assert!(run_input(&["true"], Some(&much)).is_err());
        assert!(run_input(&["sh", "-c", "cat >/dev/null"], Some(&much)).is_ok());
    }

    #[test]
    fn a_mounted_or_swapped_partition_is_in_use() {
        assert!(holds_something("/mnt/old\n"));
        assert!(holds_something("[SWAP]\n"));
        assert!(!holds_something("\n"));
        assert!(!holds_something(""));
    }

    #[test]
    fn what_is_kept_stays_between_its_floor_and_what_leaves_room() {
        let size = 100 * super::super::plan::GIB;
        assert_eq!(kept(0, size), Ok(KEEP_FREE));
        assert_eq!(kept(size, size), Ok(size - LEAST));
        assert_eq!(kept(50 * super::super::plan::GIB, size), Ok(50 * super::super::plan::GIB));
        assert!(kept(0, 0).is_err());
        assert!(kept(0, KEEP_FREE + LEAST - 1).is_err());
    }

    #[test]
    fn only_the_entries_with_exactly_that_name_are_found() {
        let list = "BootCurrent: 0001\nTimeout: 1 seconds\nBootOrder: 0003,0001,0000\n\
                    Boot0000* UEFI OS\tHD(1,GPT,abcd)/\\EFI\\BOOT\\BOOTX64.EFI\n\
                    Boot0001* Trindle\tHD(1,GPT,abcd)/\\EFI\\Trindle\\shimx64.efi\n\
                    Boot0003  Trindle\tHD(1,GPT,ef01)/\\EFI\\Trindle\\shimx64.efi\n\
                    Boot0004* Trindle backup\tHD(1,GPT,abcd)/x\n\
                    Boot00A2* Windows Boot Manager\tHD(2,GPT,1234)/\\EFI\\Microsoft\\bootmgfw.efi\n";
        assert_eq!(firmware_entries_named(list, FIRMWARE_NAME), ["0001", "0003"]);
        assert!(firmware_entries_named(list, "Nothing").is_empty());
    }

    #[test]
    fn erasing_needs_a_whole_unused_disk_that_is_not_the_stick() {
        assert_eq!(erase_fault("/dev/vdb", "/dev/vdb disk \n", "/dev/sr0"), None);
        assert_eq!(erase_fault("/dev/nvme0n1", "/dev/nvme0n1 disk \n/dev/nvme0n1p1 part \n", ""), None);
        assert_eq!(erase_fault("/dev/sdb", "/dev/sdb disk \n/dev/sdb1 part \n", "/dev/sdb").as_deref(),
                   Some(t!("installer-disk-is-stick", dev = "/dev/sdb").as_str()));
        assert_eq!(erase_fault("/dev/vda1", "/dev/vda1 part /run/testbox\n", "").as_deref(),
                   Some(t!("installer-not-a-disk", dev = "/dev/vda1").as_str()));
        assert_eq!(erase_fault("/dev/sr0", "/dev/sr0 rom \n", "").as_deref(),
                   Some(t!("installer-not-a-disk", dev = "/dev/sr0").as_str()));
        assert!(erase_fault("/dev/sda", "", "").is_some());
        for in_use in ["/dev/sda disk \n/dev/sda1 part /boot/efi\n",
                       "/dev/sda disk \n/dev/sda2 part [SWAP]\n",
                       "/dev/sda disk \n/dev/sda3 part \n/dev/mapper/home crypt /home\n",
                       "/dev/sda disk \n/dev/sda3 part \n/dev/mapper/old crypt \n",
                       "/dev/sda disk \n/dev/sda3 part \n/dev/mapper/vg-root lvm \n",
                       "/dev/sda disk \n/dev/sda3 part \n/dev/md0 raid1 \n"] {
            assert_eq!(erase_fault("/dev/sda", in_use, "").as_deref(),
                       Some(t!("installer-disk-in-use", dev = "/dev/sda").as_str()));
        }
    }

    #[test]
    fn the_disk_password_is_gone_whenever_the_chip_will_not_bind() {
        for (case, clevis) in [("fails", "exit 1"), ("stopped", "kill -TERM $PPID; sleep 1; exit 1"),
                               ("binds", "exit 0")] {
            let dir = std::env::temp_dir().join(format!("trindle-chip-{case}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(dir.join("bin")).unwrap();
            let key = dir.join("tpm-key");
            std::fs::write(&key, "a disk password").unwrap();
            let fake = |name: &str, body: &str| {
                use std::os::unix::fs::PermissionsExt;
                let p = dir.join("bin").join(name);
                std::fs::write(&p, format!("#!/bin/sh\n{body}\n")).unwrap();
                std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
            };
            fake("clevis", &format!("[ \"$2\" = list ] && exit 0\n{clevis}"));
            fake("update-initramfs", "exit 0");
            fake("systemctl", "exit 0");
            let script = chip_script("/dev/disk/by-uuid/x", "'{}'")
                .replace("/etc/trindle/tpm-key", key.to_str().unwrap());
            let out = Command::new("sh").arg("-c").arg(&script)
                .env("PATH", format!("{}:/usr/bin:/bin", dir.join("bin").display()))
                .output().unwrap();
            assert_eq!(out.status.success(), case == "binds", "{case}");
            assert!(!key.exists(), "the password was left behind when clevis {case}");
            let _ = std::fs::remove_dir_all(&dir);
        }
    }

    #[test]
    fn the_package_rewrites_the_chip_script_as_the_installer_writes_it() {
        let postinst = include_str!("../../tools/deb/postinst");
        let body = postinst.split_once("cat > \"$chip\" <<CHIP\n").unwrap().1.split_once("\nCHIP\n").unwrap().0;
        assert_eq!(format!("{body}\n"), chip_script("$part", &format!("'{CHIP_POLICY}'")));
    }

    #[test]
    fn a_chip_that_cannot_be_taught_leaves_the_password_alone() {
        let top = std::env::temp_dir().join(format!("trindle-unchipped-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        let at = |f: &str| top.join(f.trim_start_matches('/'));
        for f in CHIP_FILES {
            std::fs::create_dir_all(at(f).parent().unwrap()).unwrap();
            std::fs::write(at(f), "x").unwrap();
        }
        let r = top.to_str().unwrap();
        assert_eq!(chip_or_password(r, || Ok(())), Ok(true));
        assert!(CHIP_FILES.iter().all(|f| at(f).exists()));
        assert_eq!(chip_or_password(r, || Err("no chip".into())), Ok(false));
        assert!(CHIP_FILES.iter().all(|f| !at(f).exists()), "something of the chip was left");
        assert_eq!(chip_or_password(r, || Err("no chip".into())), Ok(false), "nothing to remove is not a fault");
        std::fs::create_dir_all(at("/etc/trindle/tpm-key/stuck")).unwrap();
        assert!(chip_or_password(r, || Err("no chip".into())).is_err(), "a password copy that would not go was let through");
        let _ = std::fs::remove_dir_all(&top);
    }

    #[test]
    fn the_stick_s_account_is_found_by_its_number() {
        let passwd = "admin:x:0:0::/home/admin:/bin/bash\nuser:x:1000:1000::/home/user:/bin/bash\n";
        assert_eq!(live_account(passwd).as_deref(), Some("user"));
        assert_eq!(live_account("admin:x:0:0::/home/admin:/bin/bash\n"), None);
        assert_eq!(live_account(":x:1000:1000::/home/x:/bin/sh\n"), None);
    }

    #[test]
    fn only_tidying_is_left_unsaid() {
        let tidy = ["udevadm", "partprobe", "kill", "cryptsetup", "umount", "ntfsfix", "grub-install", "os-prober",
                    "efibootmgr", "sh", "&argv", "&tidy"];
        let source = include_str!("steps.rs");
        let code = source.split("#[cfg(test)]").next().unwrap();
        let unsaid: Vec<&str> = code.lines().filter(|l| l.contains("let _ = chroot(") || l.contains("let _ = run("))
            .filter(|l| !tidy.iter().any(|t| l.contains(&format!("[\"{t}\"")) || l.contains(&format!("({t}"))))
            .collect();
        assert!(unsaid.is_empty(), "failures nobody is told of: {unsaid:#?}");
    }
}
