//! The administrator's helper never loses a failure without a reason.

use std::collections::{BTreeMap, BTreeSet};

const ELSEWHERE: [&str; 3] = ["voice/setup.rs", "common/charge.rs", "common/rfkill.rs"];
const ONLY_TESTS: [&str; 2] = ["kept.rs", "stand_in.rs"];

const THROWN: &str = "thrown away";
const EMPTIED: &str = "read as nothing";
const YES_OR_NO: &str = "read as yes or no";

const REWORDED: &str = "said in other words";

const KEPT: &[(&str, &str, &str, &str)] = &[
    ("admin/mod.rs", "watched", THROWN, "reads the words of the tool's refusal; unread, the step still fails by how the tool ended, with plainer words"),
    ("admin/mod.rs", "watched", EMPTIED, "the same words, from the thread that read them"),
    ("admin/mod.rs", "read_line", YES_OR_NO, "a pipe that fails ends the reading as its end does; how the tool ended is what is said"),
    ("admin/mod.rs", "apt_share", EMPTIED, "a line of apt's that is not a figure is not progress; only the filling ring is fed from it"),
    ("admin/mod.rs", "reading", EMPTIED, "a command that only reads and cannot run says nothing: the rehearsal of the update and the look at DKMS come after it succeeded, and are not made to fail for it"),
    ("admin/accounts.rs", "valid_name_ok", YES_OR_NO, "the answer is the question: Settings asks whether a name would be taken, before it calls the helper"),
    ("admin/accounts.rs", "group_members", YES_OR_NO, "a group's name that cannot be a C string has no members; only ever asked of the fixed word sudo"),
    ("admin/accounts.rs", "regular_user", EMPTIED, "the lowest ordinary uid: where login.defs does not say, Debian's own 1000 stands, which only ever refuses more"),
    ("admin/accounts.rs", "read_password", EMPTIED, "not a failure: the line with or without its line end"),
    ("admin/accounts.rs", "read_password", REWORDED, "a line that cannot be read from the helper's input is no password given, and that is the refusal said"),
    ("admin/accounts.rs", "delete", EMPTIED, "who is asking: unknown, nobody is taken for the one being deleted, and deluser's own answer is carried"),
    ("admin/accounts.rs", "delete_checked", THROWN, "ends the person's sessions first; someone not logged in has none, and deluser's answer is the one carried"),
    ("admin/machine.rs", "charge_limit_with", THROWN, "switches the start-up service off; left on with its file gone it sets charging to full, which is what was asked"),
    ("admin/firewall.rs", "firewall_apply", EMPTIED, "who is let through, read at every start and after every change: a list that cannot be read lets no plugin through, so the firewall only keeps more out"),
    ("admin/packages.rs", "tidy", THROWN, "marks the programs Trindle replaced as no longer chosen; unmarked they only stay, and what is removed is carried"),
    ("admin/packages.rs", "installed_version", EMPTIED, "no version is an answer of its own, and the step that needs one refuses without it (apt_upgrade_debian_at)"),
    ("admin/packages.rs", "is_installed", YES_OR_NO, "a package dpkg cannot be asked about is left out of the tidying, which removes nothing by mistake"),
    ("admin/updates.rs", "driver_off", EMPTIED, "no file is no driver that failed: the hook writes it only when a build failed"),
    ("admin/tailscale.rs", "tailscale_on", EMPTIED, "who is asking: turned into the refusal on the same line"),
    ("admin/tailscale.rs", "tailscale_set", YES_OR_NO, "tried again while the service wakes; the last answer is the one carried"),
    ("admin/dns.rs", "dns_on", THROWN, "asks NetworkManager to read its files again, which one not running does as it starts; and clears a name the link made next would fail on, a failure that is carried"),
    ("admin/dns.rs", "dns_on", YES_OR_NO, "whether Tailscale is running and whether resolv.conf is there: each decides only which part is done"),
    ("admin/dns.rs", "dns_on", EMPTIED, "not a failure: no lines of NetworkManager's to write"),
    ("admin/dns.rs", "dns_off", YES_OR_NO, "whether each thing dns_on made is there to undo, and whether Tailscale is running to be told"),
    ("admin/dns.rs", "dns_off", EMPTIED, "the network's own list of servers, to stand in; with none readable an empty one is written and the network writes it again"),
    ("admin/dns.rs", "dns_off", THROWN, "asks NetworkManager to read its files again, which one not running does as it starts"),
    ("admin/dns.rs", "dns_allow", THROWN, "empties systemd-resolved's memory so the site opens at once; unemptied, it opens when the old answer runs out"),
    ("admin/sources.rs", "use_source", THROWN, "reads the lists again once a source that failed is taken away; the failure said is the one that caused it"),
    ("admin/sources.rs", "after", YES_OR_NO, "a second copy of the source a vendor's package adds: one that stays only has apt remark on it"),
    ("admin/sources.rs", "after", THROWN, "the lists read again after that, and a program's picture for the list of programs: it runs without"),
    ("admin/sources.rs", "before", THROWN, "asks a vendor's package not to add its own source; if it does, `after` takes it away"),
    ("admin/sources.rs", "remove", YES_OR_NO, "which services and packages are there to stop and purge; none found, none are, and the purge's answer is carried"),
    ("admin/sources.rs", "remove", THROWN, "stops a service of a program purged next, clears what it brought in and two files nothing reads once it is gone"),
    ("admin/snapshots.rs", "number", EMPTIED, "turned into the refusal on the same line"),
    ("admin/snapshots.rs", "root_place", EMPTIED, "which disk and subvolume the system is on: unknown, the step refuses in its own words and changes nothing"),
    ("admin/snapshots.rs", "unmount_top", THROWN, "lets go of the disk's top once the step is over; left mounted it is in nobody's way until the next start"),
    ("admin/snapshots.rs", "copy_snapshot", THROWN, "clears what an earlier restore left; it is followed by a step whose failure is carried"),
    ("admin/snapshots.rs", "undo", THROWN, "clears the copy made here once the restore has failed; the failure that caused it is the one carried"),
    ("admin/snapshots.rs", "finish", YES_OR_NO, "whether the old system and its snapshots are there to move; what is there is moved and a failure to move it is carried"),
    ("admin/snapshots.rs", "finish", THROWN, "the restored system's empty snapshots folder, which the real one is moved over; were it not empty that move fails and is carried"),
    ("admin/snapshots.rs", "put_in_place", YES_OR_NO, "whether this disk can swap two folders at once; if not, the older way runs and its failures are carried"),
    ("admin/snapshots.rs", "top", EMPTIED, "not a failure: a path made here always has a folder above it"),
    ("admin/snapshots.rs", "aside_name", EMPTIED, "not a failure: a path made here always has a last part"),
    ("admin/snapshots.rs", "forget_in", YES_OR_NO, "each system set aside that would not go is gathered, named and said at the end"),
    ("admin/snapshots.rs", "chrono_stamp", EMPTIED, "the date in a name: without the date program the name says `earlier`"),
    ("admin/sharing.rs", "group_id", EMPTIED, "a fixed word that is always a C string; with no group found one is made, and that failure is carried"),
    ("admin/sharing.rs", "setup", THROWN, "switches off Samba's servers nothing here uses; one not on the computer cannot be, and the file server is stopped at every start while nothing is shared"),
    ("admin/home_snapshots.rs", "now", EMPTIED, "a clock set before 1970 reads as 1970"),
    ("admin/home_snapshots.rs", "days_of", EMPTIED, "turned into the refusal on the same line"),
    ("admin/home_snapshots.rs", "folder_there", YES_OR_NO, "the answer is the question: is the folder there"),
    ("admin/home_snapshots.rs", "ensure_config", YES_OR_NO, "whether anything is where the snapshots' folder goes, which only chooses between the step and its refusal"),
    ("admin/home_snapshots.rs", "ensure_config", EMPTIED, "snapper's pattern for a config: with none to go by, the config is the lines Trindle sets, and snapper's own answers stand for the rest"),
    ("admin/home_snapshots.rs", "on", THROWN, "starts the daily timer for today; the first copy was taken just above and the timer starts itself at every start"),
    ("admin/home_snapshots.rs", "daily", YES_OR_NO, "a computer that cannot keep earlier versions has nothing to do each day, and that is no failure of the timer"),
    ("admin/home_room.rs", "open_folder", EMPTIED, "a folder that cannot be opened gives no figure, and Settings then shows none rather than a wrong one"),
    ("admin/home_room.rs", "without", YES_OR_NO, "not a failure: whether a search of a sorted list found the thing"),
    ("admin/home_room.rs", "only_in_snapshots", EMPTIED, "whether the snapshots lie inside the home folders, to pass them by in the walk"),
    ("admin/read_only.rs", "after_panic", YES_OR_NO, "a record that cannot be read is not this process's opening: the watchdog and the next start close the system whoever opened it"),
    ("admin/read_only.rs", "dpkg_at_work", YES_OR_NO, "no dpkg lock file is no dpkg at work; the watchdog then closes the system, which is its job"),
    ("admin/read_only.rs", "before_dpkg", EMPTIED, "a record of who opened it that cannot be had stops no dpkg: dpkg is opened for all the same, and its post-invoke or the watchdog closes it"),
    ("admin/read_only.rs", "folder", YES_OR_NO, "a place that is not a folder here (no /opt, no /usr/local, or a link) is not one there is to lock or keep writable"),
    ("admin/read_only.rs", "leave_open", THROWN, "the record of who opened it, cleared when the lock is switched off; a number left there is read only by this same process's fault hook"),
    ("admin/read_only.rs", "local_writable", YES_OR_NO, "the table of mounts unread, /usr was not bound afresh either, so the /usr/local there was stays as it is"),
    ("admin/read_only.rs", "relock", YES_OR_NO, "the table of mounts unread binds nothing more, and each close's answer is carried; a close that went is not tried again"),
    ("admin/shares.rs", "caller", EMPTIED, "turned into the refusal on the same line"),
    ("admin/shares.rs", "read_login", REWORDED, "the login is a password: what could not be read of it is said as that and no more, so nothing of it is ever printed"),
    ("admin/shares.rs", "unmount_all", REWORDED, "turned into the refusal on the same line: what is not a number is not a user id"),
    ("admin/shares.rs", "user_folder", YES_OR_NO, "a folder not there is made, and the making's failure is carried"),
    ("admin/shares.rs", "write", YES_OR_NO, "the same, for the folder only root can open"),
    ("admin/shares.rs", "write", THROWN, "clears a file an earlier run left under the same name; were it still there, making it new fails and is carried"),
    ("admin/shares.rs", "drop", THROWN, "removes the login's file once mount has read it; left, it is root's alone in a folder only root opens, in /run, emptied at every start"),
    ("admin/shares.rs", "mount", THROWN, "an empty folder left by a mount that failed, and the unit that closes shares at logging out, which logging out does another way as well"),
    ("admin/shares.rs", "nfs_mount", THROWN, "an empty folder left by a mount that failed, and the unit that closes shares at logging out, which logging out does another way as well"),
    ("admin/shares.rs", "unmount", THROWN, "the empty folder of a share that was closed as asked, and an answer nobody waits for any more"),
    ("admin/shares.rs", "detach", REWORDED, "a path with a NUL in it can name no mount: refused as that"),
    ("admin/shares.rs", "detach_all", YES_OR_NO, "whether the wait ended before every share answered; a share that did not is counted and said, with how many"),
    ("admin/shares.rs", "unmount", YES_OR_NO, "whether the umount ended within the wait; if not, the share is cut loose and a share that still would not close is said as not answering"),
    ("admin/shares.rs", "unmount", REWORDED, "a share that would not close in time is said as the computer not answering, which is all the person can act on"),
    ("admin/shares.rs", "detach_all", THROWN, "the empty folder of a share that has closed, and an answer nobody waits for any more after the time limit; a share that did not close is carried as the failure"),
    ("admin/shares.rs", "unmount_all", YES_OR_NO, "whether pkexec ran it, which decides who it is for"),
    ("voice/setup.rs", "run", EMPTIED, "not a failure: the first word of a command, for its error's words"),
    ("voice/setup.rs", "sha256", EMPTIED, "a fingerprint that could not be taken is the empty one, which matches nothing: the download is refused"),
    ("voice/setup.rs", "installed_package", YES_OR_NO, "a build tool dpkg cannot be asked about is taken as missing, put in, and removed again afterwards"),
    ("voice/setup.rs", "build_whisper", EMPTIED, "not a failure: a fixed path always has a folder"),
    ("voice/setup.rs", "build_whisper", YES_OR_NO, "whether this version is already built; unknown, it is built again"),
    ("voice/setup.rs", "build_whisper", THROWN, "the building's own leavings in the temporary folder"),
    ("voice/setup.rs", "build_in", EMPTIED, "how many cores to build on: unknown, two"),
    ("voice/setup.rs", "fetch_model", THROWN, "the download's own leavings in the temporary folder"),
    ("voice/setup.rs", "remove", EMPTIED, "not a failure: a fixed path always has a folder"),
    ("voice/setup.rs", "install", THROWN, "an old note of build tools that cannot be taken away is read again against what is installed, so it shows nothing false"),
    ("voice/setup.rs", "remove_tools", THROWN, "the same note, taken away after the tools: one left behind is read again against what is installed"),
    ("voice/setup.rs", "tools_left", EMPTIED, "no note that can be read is no tools left behind, which is all a missing note means"),
    ("voice/setup.rs", "main", THROWN, "the prompt and the wait for a key that keep a terminal open for reading, by hand only"),
    ("common/charge.rs", "batteries_in", YES_OR_NO, "which power supplies are batteries: one that cannot be read is not one, and with none the step fails in its own words"),
    ("common/charge.rs", "batteries_in", EMPTIED, "an entry of that list that cannot be read is no battery, as above"),
    ("common/charge.rs", "laptop_in", YES_OR_NO, "read by Settings to decide whether to show the page; nothing is changed by it"),
    ("common/charge.rs", "laptop_in", EMPTIED, "an entry of the list of power supplies that cannot be read is passed over; read for showing only"),
    ("common/charge.rs", "plugged_in", YES_OR_NO, "read for showing; nothing is changed by it"),
    ("common/charge.rs", "plugged_in", EMPTIED, "an entry of the list of power supplies that cannot be read is passed over; read for showing only"),
    ("common/charge.rs", "framework_held", EMPTIED, "whether a Framework's limit is waiting to be woken; unknown, nothing is woken and the batteries are told as on any laptop"),
    ("common/charge.rs", "framework_wake", THROWN, "wakes a Framework's limit; whether it is there to use is found next, and a laptop that takes the limit no way is said then"),
    ("common/charge.rs", "framework", YES_OR_NO, "whether this is a Framework; unknown, its firmware is not spoken to, as on any other make"),
    ("common/charge.rs", "tell", EMPTIED, "the firmware door's error is turned into the step's own error when there is no battery to tell, and carried back; the start point is no longer read, it is written, and a failed write is carried"),
    ("common/rfkill.rs", "state_in", YES_OR_NO, "read for showing which radios are blocked; nothing is changed by it"),
    ("common/rfkill.rs", "state_in", EMPTIED, "a radio whose state cannot be read is passed over; read for showing only"),
];

const LINES: &str = "admin/kept_lines.txt";

fn lost(line: &str) -> Vec<&'static str> {
    let code = line.trim();
    if code.starts_with("//") {
        return Vec::new();
    }
    let has = |parts: &[&str]| code.contains(&parts.concat());
    let mut found = Vec::new();
    if has(&["let ", "_ ="]) || has(&[".ok", "();"]) {
        found.push(THROWN);
    } else if has(&[".ok", "()"]) || has(&["unwrap_or", "_default()"]) || has(&[".unwrap_or", "("]) || has(&["unwrap_or", "_else("])
        || has(&[".flat", "ten()"]) || has(&["map_while(", "Result::ok)"]) {
        found.push(EMPTIED);
    }
    if has(&[".is_", "ok()"]) || has(&[".is_", "err()"]) || has(&["is_ok", "_and("]) || has(&["let ", "Ok("]) || has(&["Err", "(_)"]) {
        found.push(YES_OR_NO);
    }
    if has(&["_err(", "|_|"]) || has(&[".or_", "else(|_|"]) {
        found.push(REWORDED);
    }
    found
}

fn defines(line: &str) -> Option<&str> {
    let rest = line.trim_start();
    let rest = rest.strip_prefix("pub(crate) ").or_else(|| rest.strip_prefix("pub(super) ")).or_else(|| rest.strip_prefix("pub ")).unwrap_or(rest);
    let name = rest.strip_prefix("fn ")?;
    Some(&name[..name.find(['(', '<']).unwrap_or(name.len())])
}

fn findings(text: &str) -> (Vec<(String, Vec<&'static str>, String)>, Vec<usize>) {
    let (mut all, mut bare) = (Vec::new(), Vec::new());
    let end = text.find("\nmod tests {").unwrap_or(text.len());
    let mut function = String::new();
    let mut above = "";
    for (n, line) in text[..end].lines().enumerate() {
        if let Some(f) = defines(line) {
            function = f.to_string();
        }
        let ways = lost(line);
        if ways.contains(&THROWN) && !above.starts_with("//") {
            bare.push(n + 1);
        }
        if !ways.is_empty() {
            all.push((function.clone(), ways, line.trim().to_string()));
        }
        if !line.trim().is_empty() {
            above = line.trim();
        }
    }
    (all, bare)
}

fn files() -> Vec<String> {
    let mut all: Vec<String> = std::fs::read_dir(crate::common::source::path_of("admin")).unwrap().flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with(".rs") && !ONLY_TESTS.contains(&n.as_str()))
        .map(|n| format!("admin/{n}")).collect();
    all.extend(ELSEWHERE.iter().map(|f| f.to_string()));
    all.sort();
    all
}

#[test]
fn each_way_of_losing_a_failure_is_seen() {
    let line = |parts: &[&str]| parts.concat();
    assert_eq!(lost(&line(&["    let ", "_ = run(&[\"systemctl\", \"enable\", unit], None);"])), [THROWN]);
    assert_eq!(lost(&line(&["    std::fs::remove_file(path)", ".ok", "();"])), [THROWN]);
    assert_eq!(lost(&line(&["let text = std::fs::read_to_string(hosts).", "unwrap_or", "_default();"])), [EMPTIED]);
    assert_eq!(lost(&line(&["let out = Command::new(\"date\").output()", ".ok", "()?;"])), [EMPTIED]);
    assert_eq!(lost(&line(&["for line in lines.", "map_while(", "Result::ok) {"])), [EMPTIED]);
    assert_eq!(lost(&line(&["if std::fs::remove_file(&p)", ".is_", "ok() {"])), [YES_OR_NO]);
    assert_eq!(lost(&line(&["if ", "let ", "Ok(out) = Command::new(\"systemctl\").output() {"])), [YES_OR_NO]);
    assert_eq!(lost(&line(&["let n = text.parse()", ".ok", "().filter(|n| run(n)", ".is_", "ok());"])), [EMPTIED, YES_OR_NO]);
    assert!(lost("    run(&[\"systemctl\", \"enable\", unit], None)?;").is_empty());
    assert!(lost("    gone(&at(charge::STATE)).map_err(|e| format!(\"It will come back: {e}\"))?;").is_empty());
    assert!(lost(&line(&["    // ", "let ", "_ = old();"])).is_empty());
    let bare = line(&["fn step() {\n    let ", "_ = run(&[\"a\"], None);\n}\n"]);
    let thrown = |what: &str| ("step".to_string(), vec![THROWN], line(&["let ", "_ = ", what, "();"]));
    assert_eq!(findings(&bare), (vec![("step".to_string(), vec![THROWN], line(&["let ", "_ = run(&[\"a\"], None);"]))], vec![2]));
    let said = line(&["fn step() {\n    // Only tidying.\n    let ", "_ = run(&[\"a\"], None);\n}\n"]);
    assert_eq!(findings(&said), (vec![("step".to_string(), vec![THROWN], line(&["let ", "_ = run(&[\"a\"], None);"]))], Vec::new()));
    assert_eq!(lost(&line(&["    apt_run(&argv)", ".or_", "else(|_| {"])), [REWORDED]);
    assert_eq!(lost(&line(&["    rfkill::unblock(radio).map", "_err(", "|_| rfkill::refused_words(radio))"])), [REWORDED]);
    assert_eq!(lost(&line(&["    text.parse().map", "_err(", "|_| \"That is not a number.\")?;"])), [REWORDED]);
    assert!(lost("    apt_run(&argv).or_else(|first| again(first))").is_empty());
    assert!(lost("    run(&[\"a\"], None).map_err(|e| format!(\"It failed: {e}\"))?;").is_empty());
    assert!(lost("    write_source(id).and_then(|_| run(&[\"apt-get\", \"update\"], None))").is_empty());
    let twice = line(&["fn step() {\n    // Only tidying.\n    let ", "_ = a();\n    // And this.\n    let ", "_ = b();\n}\n"]);
    assert_eq!(findings(&twice).0, [thrown("a"), thrown("b")]);
}

#[test]
fn a_failure_lost_in_the_place_of_another_is_seen() {
    let line = |parts: &[&str]| parts.concat();
    let before = line(&["fn step() {\n    // Only tidying.\n    let ", "_ = run(&[\"systemctl\", \"daemon-reload\"], None);\n}\n"]);
    let after = line(&["fn step() {\n    // Only tidying.\n    let ", "_ = run(&[\"apt-get\", \"-y\", \"install\", name], None);\n}\n"]);
    assert_ne!(findings(&before).0, findings(&after).0);
}

fn more_in<'a>(one: &'a [(String, String, String)], other: &[(String, String, String)]) -> Vec<&'a (String, String, String)> {
    let mut left: BTreeMap<&(String, String, String), usize> = BTreeMap::new();
    for line in other {
        *left.entry(line).or_insert(0) += 1;
    }
    one.iter().filter(|line| match left.get_mut(line) {
        Some(n) if *n > 0 => {
            *n -= 1;
            false
        }
        _ => true,
    }).collect()
}

#[test]
fn the_helper_drops_no_failure_that_is_not_on_the_list() {
    let (mut found, mut bare) = (BTreeSet::new(), Vec::new());
    let mut lines: Vec<(String, String, String)> = Vec::new();
    for file in files() {
        let text = std::fs::read_to_string(crate::common::source::path_of(&file)).unwrap();
        let (all, uncommented) = findings(&text);
        for (function, ways, line) in all {
            for way in ways {
                found.insert((file.clone(), function.clone(), way));
            }
            lines.push((file.clone(), function, line));
        }
        bare.extend(uncommented.into_iter().map(|n| format!("{file}:{n}")));
    }
    let listed: BTreeSet<(String, String, &str)> = KEPT.iter().map(|(f, func, way, _)| (f.to_string(), func.to_string(), *way)).collect();
    assert_eq!(listed.len(), KEPT.len(), "a place is on the list twice");
    let new: Vec<_> = found.difference(&listed).collect();
    assert!(new.is_empty(), "the helper loses a failure here: carry it back and say it, or add the place to KEPT \
            in admin/kept.rs with why losing it changes nothing the person was told: {new:#?}");
    let gone: Vec<_> = listed.difference(&found).collect();
    assert!(gone.is_empty(), "these are on the list in admin/kept.rs and no longer in the helper: {gone:#?}");
    assert!(bare.is_empty(), "a result thrown away has no comment over it saying why: {bare:#?}");
    for (file, function, _, why) in KEPT {
        assert!(why.len() > 20, "{file}, {function}: the reason is to say why, not only that");
    }
    let kept = std::fs::read_to_string(crate::common::source::path_of(LINES)).unwrap();
    let said: Vec<(String, String, String)> = kept.lines().filter(|l| !l.is_empty() && !l.starts_with('#')).map(|l| {
        let mut parts = l.splitn(3, '\t');
        let mut part = || parts.next().unwrap_or_else(|| panic!("{LINES}: a line is to be a file, a function and a line of code, with a tab between: {l:?}")).to_string();
        (part(), part(), part())
    }).collect();
    let (new, gone) = (more_in(&lines, &said), more_in(&said, &lines));
    assert!(new.is_empty() && gone.is_empty(), "the helper loses a failure on a line that has not been looked at, or a line that was is gone or changed. \
            Decide each new line as any other, then bring {LINES} up to date.\nIn the helper and not on the list: {new:#?}\nOn the list and not in the helper: {gone:#?}");
}
