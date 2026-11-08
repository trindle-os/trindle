//! Long disk work in Trindle goes through the gentle rule, read from the code itself.

use crate::common::gentle::argv;
use crate::common::no_writing_in_place::{all_code, code};

const LONG: &[&str] = &["toolkit/common/file_work/work/bin.rs", "toolkit/common/file_work/work/copy.rs", "toolkit/common/file_work/work/crew.rs", "toolkit/common/file_work/work/finish.rs", "toolkit/common/file_work/work/leave.rs", "toolkit/common/file_work/work/looks.rs", "toolkit/common/file_work/work/mod.rs", "toolkit/common/file_work/work/plan.rs", "toolkit/common/file_work/work/run.rs", "toolkit/common/file_work/work/said.rs", "toolkit/common/file_work/work/side.rs", "toolkit/common/file_work/work/undo.rs", "toolkit/common/file_work/work/walk.rs"];

const STARTS: [&str; 3] = ["thread::spawn(", "thread::Builder", "s.spawn("];

const NOT_DISK: &[(&str, usize, &str)] = &[];

const POURS: [&str; 4] = ["io::copy(", "fs::copy(", "copy_file_range(", "sendfile("];

const SHORT: &[(&str, usize, &str)] = &[
    ("installer/steps.rs", 1, "one small settings file"),
    ("mcp/tools.rs", 2, "Trindle's own small settings files"),
    ("screenshot/mod.rs", 1, "one picture of the screen"),
    ("store/backup.rs", 1, "the person's own small picture"),
    ("store/fence_net.rs", 4, "the bytes of a live connection passed on as they come, between an AI program and a site on the person's list; nothing is on a drive"),
];

const PROGRAMS: [&str; 7] = ["\"tar\"", "\"rsync\"", "\"cp\"", "\"dd\"", "\"sha256sum\"", "\"unzip\"", "\"dpkg-deb\""];

const STARTED: &[(&str, usize, &str)] = &[
    ("common/packs.rs", 2, "unzip and tar, both started through program by the one unpacking (gently)"),
    ("common/kinds.rs", 1, "a kind of file's name, and nothing started"),
    ("installer/steps.rs", 1, "the copy of the whole system, started through program"),
    ("plugins/catalogue.rs", 2, "unpacking an archive and listing it first, both started through program"),
    ("plugins/install.rs", 1, "started through program"),
    ("store/backup.rs", 5, "packing and summing go through sh, which starts them through program"),
    ("store/deb.rs", 2, "reads the few lines a package says of itself and the list of its files, and unpacks nothing"),
    ("store/flat.rs", 1, "the sum of a backup's file, through run_gently, which starts it through program"),
    ("store/parts.rs", 2, "unpacking a package is started through program; the other reads the few lines it says of itself"),
    ("store/put_back.rs", 1, "all unpacking and reading of a backup, started through program (gentle_tar)"),
    ("voice/setup.rs", 2, "unpacking goes through run, which starts it through program; the other sums one file"),
];

const SHOWN: &[(&str, &str)] = &[
    ("installer/pages.rs", "installer/actions.rs"),
];

const NOT_WORK: &[(&str, &str)] = &[
    ("settings/panels/about.rs", "how full the disk is"),
    ("ring/battery_alert.rs", "how full the battery is"),
    ("store/app_page.rs", "what people thought of an app"),
];

fn said(line: &str) -> &str {
    line.split("//").next().unwrap_or(line)
}

fn lines_with(text: &str, any: &[&str]) -> usize {
    code(text).into_iter().map(said).filter(|l| any.iter().any(|a| l.contains(a))).count()
}

#[test]
fn long_disk_work_is_started_only_through_the_gentle_rule() {
    let files = all_code();
    let mut wrong = Vec::new();
    let find = |name: &str| files.iter().find(|(f, _)| f == name).map(|(_, t)| t.as_str());
    for name in LONG {
        let Some(text) = find(name) else {
            wrong.push(format!("{name} is listed in LONG and is not there"));
            continue;
        };
        let plain = lines_with(text, &STARTS);
        let allowed = NOT_DISK.iter().find(|(f, ..)| f == name).map_or(0, |(_, n, _)| *n);
        if plain != allowed {
            wrong.push(format!("{name} starts {plain} threads that are not gentle, and {allowed} are listed in NOT_DISK"));
        }
    }

    for (name, _) in files.iter().filter(|(f, _)| f.starts_with("toolkit/common/file_work/work/") && !LONG.contains(&f.as_str())) {
        wrong.push(format!("{name} is part of the file work and is not listed in LONG"));
    }
    for (name, text) in files.iter().filter(|(f, _)| f != "toolkit/common/file_work/gentle/mod.rs") {
        let pours = lines_with(text, &POURS);
        let listed = SHORT.iter().find(|(f, ..)| f == name).map_or(0, |(_, n, _)| *n);
        if pours != listed {
            wrong.push(format!("{name} moves bytes in bulk on {pours} lines outside common::gentle, and {listed} are listed in SHORT"));
        }
        let programs = lines_with(text, &PROGRAMS);
        let listed = STARTED.iter().find(|(f, ..)| f == name).map_or(0, |(_, n, _)| *n);
        if programs != listed {
            wrong.push(format!("{name} names a program that does long disk work on {programs} lines, and {listed} are listed in STARTED"));
        }
        let rings = lines_with(text, &["standard_ring(Some(", "standard_ring_with(Some("]);
        let shown = SHOWN.iter().find(|(f, _)| f == name);
        if rings > 0 && shown.is_none() && !NOT_WORK.iter().any(|(f, _)| f == name) && !name.starts_with("toolkit/") {
            wrong.push(format!("{name} shows a ring that fills and is in neither SHOWN nor NOT_WORK"));
        }
        if let Some((_, by)) = shown {
            if !find(by).is_some_and(|t| t.contains("gentle::share(")) {
                wrong.push(format!("{name} shows how far disk work has got, and {by} does not work it out with gentle::share"));
            }
        }
    }
    for listed in SHORT.iter().map(|s| s.0).chain(STARTED.iter().map(|s| s.0)).chain(SHOWN.iter().map(|s| s.0)).chain(NOT_DISK.iter().map(|s| s.0)) {
        if find(listed).is_none() {
            wrong.push(format!("{listed} is listed and is not there"));
        }
    }
    let said_why = SHORT.iter().all(|s| s.1 > 0 && !s.2.is_empty()) && NOT_DISK.iter().all(|s| s.1 > 0 && !s.2.is_empty()) && STARTED.iter().all(|s| s.1 > 0 && !s.2.is_empty());
    assert!(said_why, "each one listed says how many and why");
    assert!(files.len() > 200, "Trindle's code was not found");
    assert!(wrong.is_empty(), "long disk work goes through common::gentle (see the top of that file): {wrong:#?}");
}

#[test]
fn every_flatpak_install_is_started_gentle() {
    let mut wrong = Vec::new();
    let mut seen = 0;
    for (name, text) in all_code().iter().filter(|(f, _)| f != "toolkit/common/file_work/gentle/mod.rs" && f != "store/flat.rs") {
        let installs = code(text).into_iter().map(said).filter(|l| l.contains("\"flatpak\"") && l.contains("\"install\"")).count();
        let gentle = lines_with(text, &["gentle::argv("]);
        seen += installs;
        if installs > gentle {
            wrong.push(format!("{name} installs with Flatpak on {installs} lines and starts {gentle} of them gentle"));
        }
    }
    assert!(seen >= 4, "the installs were not found ({seen})");
    assert!(wrong.is_empty(), "a Flatpak install is started at the usual priority: {wrong:#?}");
    assert_eq!(argv(&["flatpak", "install"]), ["ionice", "-c", "3", "nice", "-n", "19", "flatpak", "install"]);
}

const OUTS: [&str; 12] = ["settle(", "all_out(", "all_out_of(", "folder_out(", "sync_all(", "sync_data(", "syncfs(", "fsync(", "fdatasync(", "written(", "finish_now(", ".flush()"];

/// The ways an answer is thrown away.
const THROWS: [&str; 3] = ["let _ =", ".ok();", "drop("];

const WIRES: [&str; 2] = ["conn.flush()", "queue.flush()"];

const THROWN: &[(&str, usize, &str)] = &[
    ("common/clipboard.rs", 1, "the word copied, said to the program that asked, which may be gone; nothing is on a drive"),
    ("installer/steps.rs", 2, "a line of progress or a problem said to the installer's window; nothing is on a drive"),
    ("session/idle.rs", 1, "a connection to the compositor; nothing is on a drive"),
    ("session/wayland_loop.rs", 1, "a connection to the compositor; nothing is on a drive"),
    ("unlock/keys.rs", 1, "a refusal said as the program ends; nothing is on a drive"),
];

fn thrown(text: &str) -> usize {
    code(text).into_iter().map(said).filter(|l| OUTS.iter().any(|o| l.contains(o)) && THROWS.iter().any(|t| l.contains(t)) && !WIRES.iter().any(|w| l.contains(w))).count()
}

#[test]
fn no_answer_of_a_drive_is_thrown_away_uncounted() {
    let files = all_code();
    let mut wrong = Vec::new();
    for (name, text) in &files {
        let found = thrown(text);
        let listed = THROWN.iter().find(|(f, ..)| f == name).map_or(0, |(_, n, _)| *n);
        if found != listed {
            wrong.push(format!("{name} throws away {found} answers of a drive or a flush, and {listed} are listed in THROWN"));
        }
    }
    for (listed, n, why) in THROWN {
        if !files.iter().any(|(f, _)| f == listed) {
            wrong.push(format!("{listed} is listed and is not there"));
        }
        assert!(*n > 0 && !why.is_empty(), "each one listed says how many and why");
    }
    assert!(files.len() > 200, "Trindle's code was not found");
    assert!(wrong.is_empty(), "a write the drive refuses is never counted as done: {wrong:#?}");

    let seen = |line: &str| thrown(&format!("fn f() {{\n    {line}\n}}\n"));
    for line in ["let _ = held.settle(&|n| {", "let _ = gentle::all_out(folder);", "file.sync_all().ok();", "drop(out.finish_now());", "let _ = folder_out(dir);", "let _ = out.flush();", "let _ = gentle::all_out_of(&dir);"] {
        assert_eq!(seen(line), 1, "{line}");
    }
    for line in ["gentle::all_out(folder)?;", "let lost = held.settle(&reached);", "let _ = self.conn.flush();", "// let _ = file.sync_all();", "out.finish_now().map(drop)"] {
        assert_eq!(seen(line), 0, "{line}");
    }
}

#[test]
fn the_scan_counts_code_and_not_comments_or_tests() {
    let text = "fn f() {\n    // io::copy(a, b)\n    io::copy(a, b); // fs::copy(\n    std::thread::spawn(|| ());\n}\n\n#[cfg(test)]\nmod tests {\n    fn t() { io::copy(a, b); }\n}\n";
    assert_eq!(lines_with(text, &POURS), 1);
    assert_eq!(lines_with(text, &STARTS), 1);
}
