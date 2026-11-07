//! Snapshots of the home folder, the part that runs as root; the rules and
//! the exact commands are in common/home_snapshots.rs. Each step takes at
//! most a keep time, checked against the few there are: never a path, a
//! config's name or a snapshot's number from the caller.

use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use super::{at, kept, replace, run, Outcome};
use crate::common::home_snapshots::{self as home, Setting, State};
use crate::toolkit::t;

fn now() -> i64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

fn run_all(commands: &[Vec<String>]) -> Outcome {
    for c in commands {
        run(&c.iter().map(String::as_str).collect::<Vec<_>>(), None)?;
    }
    Ok(())
}

/// A keep time, only ever one of those Settings offers.
fn days_of(arg: &str) -> Result<u32, String> {
    arg.parse::<u32>().ok().filter(|d| home::KEEP_CHOICES.contains(d))
        .ok_or_else(|| t!("admin-home-keep-invalid").into())
}

fn folder() -> std::path::PathBuf {
    at(home::SNAPSHOTS)
}

/// Whether the snapshots' folder is there as a real folder.
fn folder_there() -> bool {
    std::fs::symlink_metadata(folder()).is_ok_and(|m| m.is_dir())
}

/// The snapshots' folder is made root's and open to pass through, each time
/// anything is done, so a change made by hand does not last.
fn secure_folder() -> Outcome {
    if !folder_there() {
        return Err(t!("admin-home-folder-missing").into());
    }
    std::os::unix::fs::chown(folder(), Some(0), Some(0)).map_err(|e| e.to_string())?;
    std::fs::set_permissions(folder(), std::fs::Permissions::from_mode(home::FOLDER_MODE)).map_err(|e| e.to_string())
}

/// Sees that snapper has the home config. Where there is none: made by
/// snapper when the folder is not there either, and written here when the
/// snapshots are already on the disk, which snapper will not do.
fn ensure_config(days: u32) -> Outcome {
    let config = at(home::SNAPPER_CONFIG);
    let set_up = config.exists();
    if std::fs::symlink_metadata(folder()).is_ok() && !folder_there() {
        return Err(t!("admin-home-folder-blocked").into());
    }
    if !set_up && folder_there() {
        // Both are read before anything is written: snapper's list of
        // configs names the system's own too, and written out from a bad
        // read it would lose that one.
        let list_file = at(home::SNAPPER_LIST);
        let list = kept(&list_file)?;
        // With no pattern to go by, the config is the few lines Trindle
        // sets itself, and snapper's own answers stand for the rest.
        let template = std::fs::read_to_string(at(home::SNAPPER_TEMPLATE)).unwrap_or_default();
        if let Some(dir) = config.parent() {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        replace(&config, home::adopted_config(&template, days).as_bytes(), 0o640)?;
        if let Some(with) = home::listed(&list) {
            replace(&list_file, with.as_bytes(), 0o644)?;
        }
        return run_all(&home::setup_commands(true, days));
    }
    run_all(&home::setup_commands(set_up, days))
}

fn write_setting(setting: Setting) -> Outcome {
    std::fs::create_dir_all(at("/etc/trindle")).map_err(|e| e.to_string())?;
    replace(&at(home::SETTING), setting.text().as_bytes(), 0o644)
}

/// The setting as kept, read honestly, for a step that changes one half of
/// it and writes both: from a bad read the other half would be lost.
fn setting_kept() -> Result<Setting, String> {
    Ok(Setting::parse(&kept(&at(home::SETTING))?))
}

fn write_state(state: &State) -> Outcome {
    if let Some(dir) = Path::new(home::STATE).parent() {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    replace(Path::new(home::STATE), state.text().as_bytes(), 0o644)
}

/// The room only the snapshots hold (admin/home_room.rs). Asking where
/// every file lies on the disk takes a while on a large home, so it is done
/// only by the daily look, which runs at the lowest priority.
fn measure() -> Option<u64> {
    let numbers: Vec<u32> = home::kept_in(&folder()).iter().map(|k| k.number).collect();
    if numbers.is_empty() {
        return Some(0);
    }
    super::home_room::only_in_snapshots(Path::new(home::HOME), &folder(), &numbers)
}

/// One look: old copies go, the day's is taken if it should be, and how it
/// went is left for Settings.
fn look(setting: Setting, measuring: bool) -> Outcome {
    let at = now();
    let plan = home::plan(setting, &home::kept_in(&folder()), at, home::room_on(Path::new(home::HOME)));
    let mut state = State::read();
    let tidied = match home::delete_command(&plan.delete, false) {
        Some(c) => run_all(&[c]),
        None => Ok(()),
    };
    let taken = if plan.take { run_all(&[home::take_command()]) } else { Ok(()) };
    if plan.skipped_full {
        state.skipped_full = Some(at);
    } else if plan.take || !setting.on {
        state.skipped_full = None;
    }
    if measuring {
        if let Some(bytes) = measure() {
            state.room = Some((bytes, at));
        }
    }
    write_state(&state)?;
    taken.and(tidied)
}

/// Switches earlier versions on: the config if there is none, the folder's
/// permissions, the first copy, and the timer for the days after.
pub fn on(args: &[String]) -> Outcome {
    let [days] = args else { return Err("usage: trindle-admin home-snapshots-on DAYS".into()) };
    let setting = Setting { on: true, days: days_of(days)? };
    home::possible_here().map_err(|why| why.words())?;
    ensure_config(setting.days)?;
    secure_folder()?;
    look(setting, false)?;
    write_setting(setting)?;
    // It starts by itself at the next start; this is for today, when the
    // first copy has already been taken above.
    let _ = run(&["systemctl", "start", home::TIMER], None);
    Ok(())
}

/// Switches them off: no more are taken, and those there go as their time
/// comes, or at once with clear.
pub fn off(args: &[String]) -> Outcome {
    if !args.is_empty() {
        return Err("usage: trindle-admin home-snapshots-off".into());
    }
    write_setting(Setting { on: false, ..setting_kept()? })
}

/// Changes how long each copy is kept, and lets go at once of those a
/// shorter time no longer covers.
pub fn keep(args: &[String]) -> Outcome {
    let [days] = args else { return Err("usage: trindle-admin home-snapshots-keep DAYS".into()) };
    let setting = Setting { days: days_of(days)?, ..setting_kept()? };
    write_setting(setting)?;
    if !folder_there() {
        return Ok(());
    }
    ensure_config(setting.days)?;
    secure_folder()?;
    look(setting, false)
}

/// Deletes every snapshot of the home folders and waits for the room.
pub fn clear(args: &[String]) -> Outcome {
    if !args.is_empty() {
        return Err("usage: trindle-admin home-snapshots-clear".into());
    }
    if !folder_there() {
        return Ok(());
    }
    ensure_config(Setting::read().days)?;
    let all: Vec<u32> = home::kept_in(&folder()).iter().map(|k| k.number).collect();
    if let Some(c) = home::delete_command(&all, true) {
        run_all(&[c])?;
    }
    write_state(&State { room: Some((0, now())), ..State::read() })
}

/// From trindle-home-snapshots.service, once a day and a while after each
/// start. It never fails the unit over a computer where there is nothing
/// to do.
pub fn daily(args: &[String]) -> Outcome {
    if !args.is_empty() {
        return Err("usage: trindle-admin home-snapshots-daily".into());
    }
    if !folder_there() || home::possible_here().is_err() {
        return Ok(());
    }
    // With the config gone, as after the system was put back to before it
    // was made, the setting has gone too and reads as off: the snapshots
    // still on the disk are let go as their time comes.
    let setting = Setting { on: Path::new(home::SNAPPER_CONFIG).exists() && Setting::read().on, ..Setting::read() };
    ensure_config(setting.days)?;
    secure_folder()?;
    look(setting, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(a: &[&str]) -> Vec<String> {
        a.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn each_step_checks_its_arguments_before_anything_else() {
        assert!(on(&[]).unwrap_err().starts_with("usage:"));
        assert!(on(&strings(&["7", "/"])).unwrap_err().starts_with("usage:"));
        assert!(keep(&[]).unwrap_err().starts_with("usage:"));
        assert!(off(&strings(&["now"])).unwrap_err().starts_with("usage:"));
        assert!(clear(&strings(&["root"])).unwrap_err().starts_with("usage:"));
        assert!(daily(&strings(&["x"])).unwrap_err().starts_with("usage:"));
    }

    #[test]
    fn only_the_keep_times_on_offer_are_taken() {
        assert_eq!(days_of("3"), Ok(3));
        assert_eq!(days_of("30"), Ok(30));
        for no in ["0", "1", "8", "365", "-7", "7 ", "seven", "", "7;reboot", "../7"] {
            assert!(days_of(no).is_err(), "{no}");
            // Refused before anything is looked at or run.
            assert_eq!(on(&strings(&[no])).unwrap_err(), "That is not a time earlier versions can be kept for.");
            assert_eq!(keep(&strings(&[no])).unwrap_err(), "That is not a time earlier versions can be kept for.");
        }
    }

    #[test]
    fn snappers_list_that_cannot_be_read_is_not_written_over() {
        let s = crate::admin::stand_in::StandIn::all_well("snapper-list");
        std::fs::create_dir_all(s.path(home::SNAPSHOTS)).unwrap();
        let was: &[u8] = b"SNAPPER_CONFIGS=\"root\"\n# \xff\n";
        s.write(home::SNAPPER_LIST, was);
        assert!(ensure_config(7).unwrap_err().starts_with("Nothing was changed, because"));
        assert_eq!(std::fs::read(s.path(home::SNAPPER_LIST)).unwrap(), was);
        assert!(!s.path(home::SNAPPER_CONFIG).exists() && s.ran().is_empty());
    }

    #[test]
    fn a_setting_that_cannot_be_read_is_not_written_over() {
        let s = crate::admin::stand_in::StandIn::all_well("home-setting");
        let was: &[u8] = b"on 30\xff\n";
        s.write(home::SETTING, was);
        assert!(off(&[]).unwrap_err().starts_with("Nothing was changed, because"));
        assert!(keep(&strings(&["7"])).unwrap_err().starts_with("Nothing was changed, because"));
        assert_eq!(std::fs::read(s.path(home::SETTING)).unwrap(), was);
        // One that reads keeps the half that was not changed.
        s.write(home::SETTING, b"on 30\n");
        assert!(off(&[]).is_ok());
        assert_eq!(std::fs::read_to_string(s.path(home::SETTING)).unwrap(), "off 30\n");
    }
}
