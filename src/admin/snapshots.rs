//! Snapshot commands run as root; see common/snapshots.rs.

use std::path::Path;

use super::{and_undone, kept, run, Outcome};
use crate::common::snapshots::CONFIG;
use crate::toolkit::t;

fn number(arg: &str) -> Result<u32, String> {
    arg.parse::<u32>().ok().filter(|n| *n > 0).ok_or_else(|| t!("admin-snapshot-number-invalid").into())
}

pub fn create(args: &[String]) -> Outcome {
    let [desc] = args else { return Err("usage: trindle-admin snapshot-create DESCRIPTION".into()) };
    let desc: String = desc.chars().filter(|c| !c.is_control()).take(80).collect();
    let desc = if desc.trim().is_empty() { t!("admin-snapshot-description").to_string() } else { desc };
    // important=yes exempts it from snapper's cleanup.
    run(&["snapper", "-c", CONFIG, "create", "--description", &desc, "--userdata", "important=yes"], None)
}

pub fn delete(args: &[String]) -> Outcome {
    let [n] = args else { return Err("usage: trindle-admin snapshot-delete NUMBER".into()) };
    let n = number(n)?;
    run(&["snapper", "-c", CONFIG, "delete", &n.to_string()], None)
}

/// Root filesystem UUID and subvolume, valid whether booted normally or from a snapshot.
fn root_place() -> Result<(String, String), String> {
    let cmdline = std::fs::read_to_string("/proc/cmdline").map_err(|e| e.to_string())?;
    let uuid = cmdline.split_whitespace().find_map(|w| w.strip_prefix("root=UUID="))
        .map(String::from)
        .or_else(|| {
            let out = std::process::Command::new("findmnt").args(["-no", "UUID", "/"]).output().ok()?;
            Some(String::from_utf8_lossy(&out.stdout).trim().to_string()).filter(|s| !s.is_empty())
        })
        .ok_or(t!("admin-disk-unknown"))?;
    let fstab = std::fs::read_to_string("/etc/fstab").unwrap_or_default();
    let sub = fstab.lines().map(str::trim).filter(|l| !l.starts_with('#'))
        .find(|l| l.split_whitespace().nth(1) == Some("/"))
        .and_then(|l| l.split_whitespace().nth(3))
        .and_then(|opts| opts.split(',').find_map(|o| o.strip_prefix("subvol=")))
        .map(|s| s.trim_start_matches('/').to_string())
        .filter(|s| !s.is_empty() && !s.contains(".."))
        .ok_or(t!("admin-system-subvolume"))?;
    if !uuid.chars().all(|c| c.is_ascii_hexdigit() || c == '-') {
        return Err(t!("admin-disk-unknown").into());
    }
    Ok((uuid, sub))
}

/// Takes effect at next boot. The current root is set aside, not deleted,
/// and its .snapshots moves into the new root.
pub fn restore(args: &[String]) -> Outcome {
    let [n] = args else { return Err("usage: trindle-admin snapshot-restore NUMBER".into()) };
    let n = number(n)?;
    let (uuid, sub) = root_place()?;
    let top = Path::new("/run/trindle-snapshot-top");
    std::fs::create_dir_all(top).map_err(|e| e.to_string())?;
    run(&["mount", "-o", "subvolid=5", &format!("/dev/disk/by-uuid/{uuid}"), &top.to_string_lossy()], None)?;
    let result = finish(top, &sub).and_then(|()| swap(top, &sub, n));
    unmount_top(top);
    result
}

/// `trindle-admin snapshot-finish-restore`: tidies up after a restore that
/// was cut short once the restored system was in place. Run at the start of
/// the restored system; harmless when nothing is pending.
pub fn finish_restore(args: &[String]) -> Outcome {
    if !args.is_empty() {
        return Err("usage: trindle-admin snapshot-finish-restore".into());
    }
    if !Path::new("/").join(PENDING_REL).exists() {
        return Ok(());
    }
    let (uuid, sub) = root_place()?;
    let top = Path::new("/run/trindle-snapshot-top");
    std::fs::create_dir_all(top).map_err(|e| e.to_string())?;
    run(&["mount", "-o", "subvolid=5", &format!("/dev/disk/by-uuid/{uuid}"), &top.to_string_lossy()], None)?;
    let done = finish(top, &sub);
    unmount_top(top);
    done
}

/// Lets go of the whole disk mounted at `top`. Left mounted, it is in
/// nobody's way until the computer is next started, and a step that comes
/// before then mounts it there again.
fn unmount_top(top: &Path) {
    // As above: what the step did is already done, and is what is said.
    let _ = run(&["umount", &top.to_string_lossy()], None);
    // An empty folder in /run, gone at the next start.
    let _ = std::fs::remove_dir(top);
}

#[cfg(test)]
thread_local! {
    /// For a test: a move is refused when the name of what is moved starts
    /// with one of these, as a disk in trouble would refuse it. With a
    /// stroke in it, the folder it is in must start with the first part too.
    static REFUSED: std::cell::RefCell<Vec<&'static str>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Moves a subvolume, or a folder in one, to another name on the same disk.
fn moved(from: &Path, to: &Path) -> std::io::Result<()> {
    #[cfg(test)]
    if REFUSED.with(|r| r.borrow().iter().any(|start| {
        let starts = |path: Option<&Path>, with: &str| path.and_then(Path::file_name).is_some_and(|n| n.to_string_lossy().starts_with(with));
        match start.split_once('/') {
            Some((folder, name)) => starts(from.parent(), folder) && starts(Some(from), name),
            None => starts(Some(from), start),
        }
    })) {
        return Err(std::io::Error::other("the disk would not"));
    }
    std::fs::rename(from, to)
}

/// Swaps two folders in one step, so the system is never without a name:
/// a power cut leaves either the old system or the restored one under it.
fn exchanged(a: &Path, b: &Path) -> std::io::Result<()> {
    #[cfg(test)]
    REFUSED.with(|r| {
        let r = r.borrow();
        if r.contains(&"exchange") {
            Err(std::io::Error::other("the disk would not"))
        } else if r.contains(&"!exchange") {
            Err(std::io::ErrorKind::Unsupported.into())
        } else {
            Ok(())
        }
    })?;
    use std::os::unix::ffi::OsStrExt;
    let c = |p: &Path| std::ffi::CString::new(p.as_os_str().as_bytes()).map_err(std::io::Error::other);
    let (a, b) = (c(a)?, c(b)?);
    // SAFETY: two valid NUL-ended paths that outlive the call.
    let done = unsafe { libc::syscall(libc::SYS_renameat2, libc::AT_FDCWD, a.as_ptr(), libc::AT_FDCWD, b.as_ptr(), libc::RENAME_EXCHANGE) };
    if done == 0 { Ok(()) } else { Err(std::io::Error::last_os_error()) }
}

/// The disk or kernel cannot swap two folders at once.
fn cannot_exchange(e: &std::io::Error) -> bool {
    e.kind() == std::io::ErrorKind::Unsupported
        || matches!(e.raw_os_error(), Some(libc::EINVAL | libc::ENOSYS | libc::EOPNOTSUPP))
}

fn swap(top: &Path, sub: &str, n: u32) -> Outcome {
    let current = top.join(sub);
    let snap = current.join(format!(".snapshots/{n}/snapshot"));
    if !snap.is_dir() {
        return Err(t!("admin-snapshot-missing", n = n));
    }
    let stamp = chrono_stamp();
    let fresh = top.join(format!("{sub}-restoring"));
    let aside = top.join(format!("{sub}-before-restore-{stamp}"));
    copy_snapshot(&snap, &fresh)?;
    Swap { sub, current: &current, fresh: &fresh, aside: &aside }.put_in_place()
}

fn path_text(p: &Path) -> String {
    p.to_string_lossy().to_string()
}

/// The snapshot copied to a subvolume of its own, ready to take the present system's place.
fn copy_snapshot(snap: &Path, fresh: &Path) -> Outcome {
    if fresh.exists() {
        // Were it still there, the copy made next fails, and that is said.
        let _ = run(&["btrfs", "subvolume", "delete", &path_text(fresh)], None);
    }
    run(&["btrfs", "subvolume", "snapshot", &path_text(snap), &path_text(fresh)], None)?;
    // The copy's .snapshots is an empty dir; the real one is moved in below.
    // Were it still there and not empty, that move fails, and that is said.
    let _ = std::fs::remove_dir(fresh.join(".snapshots"));
    Ok(())
}

/// The three places a restore moves between: the present system, the copy of the snapshot, and where the present system is set aside.
struct Swap<'a> {
    sub: &'a str,
    current: &'a Path,
    fresh: &'a Path,
    aside: &'a Path,
}

impl Swap<'_> {
    fn aside_name(&self) -> String {
        self.aside.file_name().unwrap_or_default().to_string_lossy().to_string()
    }

    fn undo(&self, why: String) -> Outcome {
        // What is left is cleared away when the next restore begins.
        let _ = run(&["btrfs", "subvolume", "delete", &path_text(self.fresh)], None);
        Err(why)
    }

    /// Where the present system is, should it not go back under its name.
    fn stranded(&self, e: std::io::Error) -> String {
        t!("admin-restore-stranded", aside = &self.aside_name(), sub = self.sub, why = e.to_string())
    }

    fn put_in_place(&self) -> Outcome {
        // Record the set-aside subvolume so Settings can reclaim the space, and
        // tell the restored system to list the snapshots in the boot menu
        // again. Both are written before anything is moved: without the first
        // the room the present system holds could never be had back in
        // Settings, so a restore that cannot note it is not begun.
        if let Err(e) = noted(self.current, self.fresh, &self.aside_name()) {
            return self.undo(t!("admin-restore-not-ready", why = e));
        }
        // The two systems trade names in one step. Moving the present one
        // away first would leave the computer with no system under its name
        // for as long as a power cut could come.
        match exchanged(self.current, self.fresh) {
            Ok(()) => return self.tidy_up(),
            Err(e) if !cannot_exchange(&e) => return self.undo(t!("admin-restore-in-place", why = e.to_string())),
            Err(_) => {}
        }
        if let Err(e) = moved(self.current, self.aside) {
            return self.undo(t!("admin-restore-aside", why = e.to_string()));
        }
        if let Err(e) = moved(&self.aside.join(".snapshots"), &self.fresh.join(".snapshots")) {
            let back = moved(self.aside, self.current).map_err(|e| self.stranded(e));
            return self.undo(and_undone(t!("admin-restore-snapshots-across", why = e.to_string()), back));
        }
        match moved(self.fresh, self.current) {
            Ok(()) => finish(self.top(), self.sub),
            Err(e) => self.roll_back(e),
        }
    }

    fn top(&self) -> &Path {
        self.current.parent().unwrap_or(self.current)
    }

    /// The restored system is in place; the old one lies under the copy's
    /// name. Its name and snapshots are put right, and if that is cut short
    /// the mark in the restored system has it finished at the next start.
    fn tidy_up(&self) -> Outcome {
        finish(self.top(), self.sub).map_err(|e| t!("admin-restore-finish-later", why = e))
    }

    /// The copy could not take the present system's place. Roll back. The
    /// snapshots are in the copy by now, and go back into the present
    /// system; and whatever became of them, the present system goes back
    /// under its own name, since the computer starts from that name and
    /// from no other. Snapshots that would not go back are said with where
    /// they now lie, which nothing else names.
    fn roll_back(&self, e: std::io::Error) -> Outcome {
        let held = self.fresh.join(".snapshots");
        let snapshots = moved(&held, &self.aside.join(".snapshots")).map_err(|e| t!("admin-path-error", path = held.display().to_string(), why = e.to_string()));
        let named = moved(self.aside, self.current).map_err(|e| self.stranded(e));
        let back = match (named, snapshots) {
            (Err(name), Err(snapshots)) => Err(t!("admin-both-failures", first = name, second = snapshots)),
            (named, snapshots) => named.and(snapshots),
        };
        let why = t!("admin-restore-in-place", why = e.to_string());
        match back {
            // With all as it was, the copy holds nothing and is cleared away.
            Ok(()) => self.undo(why),
            // Otherwise it is left: the snapshots may still be in it.
            back => Err(and_undone(why, back)),
        }
    }
}

/// Writes into the copy about to become the system the list of systems set
/// aside, with `aside` added to what the present system's list holds, and
/// the mark that has the boot menu made again.
fn noted(current: &Path, fresh: &Path, aside: &str) -> Outcome {
    let mut names: Vec<String> = kept(&current.join(SET_ASIDE_REL))?
        .lines().map(String::from).filter(|l| !l.trim().is_empty()).collect();
    names.push(aside.to_string());
    let list = fresh.join(SET_ASIDE_REL);
    if let Some(dir) = list.parent() {
        std::fs::create_dir_all(dir).map_err(|e| t!("admin-path-error", path = dir.display().to_string(), why = e.to_string()))?;
    }
    std::fs::write(&list, names.join("\n") + "\n").map_err(|e| t!("admin-path-error", path = list.display().to_string(), why = e.to_string()))?;
    let pending = fresh.join(PENDING_REL);
    std::fs::write(&pending, aside).map_err(|e| t!("admin-path-error", path = pending.display().to_string(), why = e.to_string()))?;
    let mark = fresh.join(GRUB_REFRESH_REL);
    std::fs::write(&mark, "").map_err(|e| t!("admin-path-error", path = mark.display().to_string(), why = e.to_string()))
}

/// In the restored system until the old one has its name and its snapshots
/// back; holds the name it is to have.
const PENDING_REL: &str = "var/lib/trindle/restore-pending";

/// Finishes a restore whose swap is done: the old system, lying under the
/// copy's name, is renamed to what the mark says and its snapshots are moved
/// into the restored system. Each step is skipped when already done, so it
/// can be run again after any cut, and the mark goes last.
fn finish(top: &Path, sub: &str) -> Outcome {
    let current = top.join(sub);
    let mark = current.join(PENDING_REL);
    let Ok(aside) = std::fs::read_to_string(&mark) else { return Ok(()) };
    let aside = aside.trim();
    // Only a name restore produced, so the mark cannot point elsewhere.
    if !aside.starts_with(&format!("{sub}-before-restore-")) || aside.contains('/') || aside.contains("..") {
        return Err(t!("admin-restore-not-ready", why = mark.display().to_string()));
    }
    let (aside, old) = (top.join(aside), top.join(format!("{sub}-restoring")));
    if old.exists() && !aside.exists() {
        moved(&old, &aside).map_err(|e| t!("admin-restore-aside", why = e.to_string()))?;
    }
    if aside.join(".snapshots").exists() {
        // An empty one is all a copy has; the real ones are moved over it.
        let _ = std::fs::remove_dir(current.join(".snapshots"));
        moved(&aside.join(".snapshots"), &current.join(".snapshots"))
            .map_err(|e| t!("admin-restore-snapshots-across", why = e.to_string()))?;
    }
    std::fs::remove_file(&mark).map_err(|e| t!("admin-path-error", path = mark.display().to_string(), why = e.to_string()))
}

const SET_ASIDE_REL: &str = "var/lib/trindle/set-aside";
const GRUB_REFRESH_REL: &str = "var/lib/trindle/grub-refresh";

pub fn forget_set_aside(args: &[String]) -> Outcome {
    if !args.is_empty() {
        return Err("usage: trindle-admin snapshot-forget-set-aside".into());
    }
    let (uuid, sub) = root_place()?;
    let list = Path::new("/").join(SET_ASIDE_REL);
    let top = Path::new("/run/trindle-snapshot-top");
    std::fs::create_dir_all(top).map_err(|e| e.to_string())?;
    run(&["mount", "-o", "subvolid=5", &format!("/dev/disk/by-uuid/{uuid}"), &top.to_string_lossy()], None)?;
    let done = forget_in(top, &list, &sub);
    unmount_top(top);
    done
}

/// Removes every system set aside that `list` names under `top`, and
/// leaves in the list only those that would not go.
fn forget_in(top: &Path, list: &Path, sub: &str) -> Outcome {
    let names: Vec<String> = kept(list)?
        .lines().map(str::trim).map(String::from)
        // Only names restore produced, so nothing else can be deleted this way.
        .filter(|n| n.starts_with(&format!("{sub}-before-restore-")) && !n.contains('/') && !n.contains(".."))
        .collect();
    let mut failed = Vec::new();
    for n in &names {
        let p = top.join(n);
        if p.exists() && run(&["btrfs", "subvolume", "delete", &p.to_string_lossy()], None).is_err() {
            failed.push(n.clone());
        }
    }
    // A list left naming what has gone would have Settings offer the same
    // room again for ever.
    let listed = crate::common::files::write_text(list, &failed.join("\n"))
        .map_err(|e| t!("admin-restore-list", why = e.to_string()));
    if failed.is_empty() { listed } else { Err(t!("admin-restore-not-removed", names = failed.join(", "))) }
}

fn chrono_stamp() -> String {
    std::process::Command::new("date").arg("+%Y-%m-%d-%H%M").output().ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty()).unwrap_or_else(|| "earlier".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::admin::stand_in::StandIn;

    /// A stand-in btrfs that makes the copy as a plain folder, with a file
    /// in the way of the note when `blocked`.
    fn copying(tag: &str, blocked: bool) -> StandIn {
        StandIn::new(tag, move |argv| {
            if argv.starts_with(&["btrfs", "subvolume", "snapshot"]) {
                std::fs::create_dir_all(argv[4]).map_err(|e| e.to_string())?;
                if blocked {
                    std::fs::write(Path::new(argv[4]).join("var"), b"").map_err(|e| e.to_string())?;
                }
            }
            Ok(())
        })
    }

    fn a_system(s: &StandIn) -> std::path::PathBuf {
        let top = s.path("/top");
        std::fs::create_dir_all(top.join("@/.snapshots/3/snapshot")).unwrap();
        std::fs::write(top.join("@/mark"), b"present").unwrap();
        top
    }

    fn set_aside(top: &Path) -> Vec<String> {
        std::fs::read_dir(top).unwrap().flatten().map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|n| n.starts_with("@-before-restore-")).collect()
    }

    #[test]
    fn a_restore_that_cannot_note_what_it_sets_aside_changes_nothing() {
        let s = copying("restore-blocked", true);
        let top = a_system(&s);
        let said = swap(&top, "@", 3).unwrap_err();
        assert!(said.starts_with("Nothing was changed: the snapshot could not be made ready."), "{said}");
        assert_eq!(std::fs::read(top.join("@/mark")).unwrap(), b"present");
        assert!(set_aside(&top).is_empty());
    }

    #[test]
    fn a_restore_notes_what_it_set_aside_in_the_system_put_back() {
        let s = copying("restore", false);
        let top = a_system(&s);
        assert_eq!(swap(&top, "@", 3), Ok(()));
        let aside = set_aside(&top);
        assert_eq!(aside.len(), 1);
        assert_eq!(std::fs::read_to_string(top.join("@").join(SET_ASIDE_REL)).unwrap(), format!("{}\n", aside[0]));
        assert!(top.join("@").join(GRUB_REFRESH_REL).is_file());
        assert!(top.join("@/.snapshots/3/snapshot").is_dir() && top.join(&aside[0]).join("mark").is_file());
    }

    /// Moves refused for as long as this is kept.
    struct Refusing;

    fn refusing(starts: &[&'static str]) -> Refusing {
        REFUSED.with(|r| *r.borrow_mut() = starts.to_vec());
        Refusing
    }

    impl Drop for Refusing {
        fn drop(&mut self) {
            REFUSED.with(|r| r.borrow_mut().clear());
        }
    }

    /// The fault: the snapshot would not go into place, the snapshots were
    /// looked for where the system no longer was, and so the system was
    /// left under the name it had been set aside as, and would not start.
    #[test]
    fn a_snapshot_that_cannot_be_put_in_place_leaves_the_system_under_its_own_name() {
        let s = copying("restore-not-placed", false);
        let top = a_system(&s);
        let no = refusing(&["exchange"]);
        let said = swap(&top, "@", 3).unwrap_err();
        drop(no);
        assert_eq!(said, "Could not put the snapshot in place: the disk would not");
        assert_eq!(std::fs::read(top.join("@/mark")).unwrap(), b"present", "the present system is under its own name");
        assert!(top.join("@/.snapshots/3/snapshot").is_dir(), "and has its snapshots again");
        assert!(set_aside(&top).is_empty(), "nothing is left set aside");
        // With all put back, the copy that was made for the restore is
        // cleared away, not left to take up room.
        let cleared = format!("btrfs subvolume delete {}", top.join("@-restoring").display());
        assert_eq!(s.ran().iter().filter(|ran| **ran == cleared).count(), 1, "{:?}", s.ran());
        // A restore tried again afterwards goes through.
        assert_eq!(swap(&top, "@", 3), Ok(()));
    }

    /// When the system will not go back under its name either, that is
    /// said with the name it has, and the snapshots are still put back in it.
    #[test]
    fn a_system_left_under_another_name_is_said_with_that_name() {
        let s = copying("restore-stranded", false);
        let top = a_system(&s);
        let no = refusing(&["!exchange", "@-restoring", "@-before-restore-"]);
        let said = swap(&top, "@", 3).unwrap_err();
        drop(no);
        let aside = set_aside(&top);
        assert_eq!(aside.len(), 1);
        assert!(said.starts_with("Could not put the snapshot in place: the disk would not Putting things back as they were did not work either: \
                                  the present system is now named @-before-restore-"), "{said}");
        assert!(said.contains(&format!("named {} and not @, and the computer will not start until it is named @ again", aside[0])), "{said}");
        assert!(top.join(&aside[0]).join(".snapshots/3/snapshot").is_dir(), "its snapshots went back into it all the same");
        // The snapshots that would not go back are said too, and the name is still put right.
        let s = copying("restore-snapshots-left", false);
        let top = a_system(&s);
        let no = refusing(&["!exchange", "@-restoring", "@-restoring/.snapshots"]);
        let said = swap(&top, "@", 3).unwrap_err();
        drop(no);
        // Where they are now is said, since nothing else names the place.
        assert_eq!(said, format!("Could not put the snapshot in place: the disk would not Putting things back as they were did not work either: \
                                  {}: the disk would not", top.join("@-restoring/.snapshots").display()));
        assert_eq!(std::fs::read(top.join("@/mark")).unwrap(), b"present");
        assert!(set_aside(&top).is_empty() && top.join("@-restoring/.snapshots/3/snapshot").is_dir());
        // The copy holds the snapshots still, so it is not cleared away.
        assert!(!s.ran().iter().any(|ran| ran.starts_with("btrfs subvolume delete")), "{:?}", s.ran());
    }

    /// The fault: the present system was moved away before the restored one
    /// was moved in, so a power cut between the two left no system under its name.
    #[test]
    fn the_system_never_lacks_its_name_during_a_restore() {
        let s = copying("restore-exchange", false);
        let top = a_system(&s);
        assert_eq!(swap(&top, "@", 3), Ok(()));
        let aside = set_aside(&top);
        assert_eq!(aside.len(), 1);
        assert!(top.join(&aside[0]).join("mark").is_file() && !top.join("@/mark").exists(), "the restored system is under the name");
        assert!(top.join("@/.snapshots/3/snapshot").is_dir() && !top.join(&aside[0]).join(".snapshots").exists());
        assert!(!top.join("@").join(PENDING_REL).exists() && !top.join("@-restoring").exists());
    }

    /// A cut after the swap: the restored system is under the name and starts;
    /// the old one lies under the copy's name until the next start finishes it.
    #[test]
    fn a_restore_cut_short_after_the_swap_is_finished_later() {
        let s = copying("restore-cut", false);
        let top = a_system(&s);
        let no = refusing(&["@-restoring"]);
        let said = swap(&top, "@", 3).unwrap_err();
        drop(no);
        assert!(said.starts_with("The snapshot is in place"), "{said}");
        assert!(!top.join("@/mark").exists() && top.join("@-restoring/mark").is_file(), "the name is the restored system's");
        assert!(top.join("@").join(PENDING_REL).is_file() && top.join("@-restoring/.snapshots/3").is_dir());
        assert_eq!(finish(&top, "@"), Ok(()));
        let aside = set_aside(&top);
        assert_eq!(aside.len(), 1);
        assert!(top.join(&aside[0]).join("mark").is_file() && top.join("@/.snapshots/3/snapshot").is_dir());
        assert!(!top.join("@-restoring").exists() && !top.join("@").join(PENDING_REL).exists());
        // Run again, nothing is pending.
        assert_eq!(finish(&top, "@"), Ok(()));
    }

    #[test]
    fn a_mark_that_names_something_else_is_not_followed() {
        let s = copying("restore-mark", false);
        let top = a_system(&s);
        std::fs::create_dir_all(top.join("@/var/lib/trindle")).unwrap();
        std::fs::write(top.join("@").join(PENDING_REL), "../etc").unwrap();
        assert!(finish(&top, "@").is_err());
        assert!(top.join("@/.snapshots/3/snapshot").is_dir());
    }

    #[test]
    fn a_list_of_what_was_set_aside_is_read_honestly_and_kept_up_to_date() {
        let s = StandIn::all_well("forget");
        let top = s.path("/top");
        std::fs::create_dir_all(&top).unwrap();
        // One that cannot be read is not emptied.
        let was: &[u8] = b"@-before-restore-2026-10-01-0900\n\xff\n";
        s.write("/list", was);
        assert!(forget_in(&top, &s.path("/list"), "@").unwrap_err().starts_with("Nothing was changed, because"));
        assert_eq!(std::fs::read(s.path("/list")).unwrap(), was);
        // One that cannot be written is said, and is whole as it was.
        s.write("/full", b"@-before-restore-2026-10-01-0900\n");
        crate::common::files::FULL.set(true);
        let said = forget_in(&top, &s.path("/full"), "@");
        crate::common::files::FULL.set(false);
        let said = said.unwrap_err();
        assert_eq!(std::fs::read(s.path("/full")).unwrap(), b"@-before-restore-2026-10-01-0900\n");
        assert!(said.starts_with("The list of what was set aside could not be brought up to date"), "{said}");
    }
}
