//! Nothing in the file chooser's acts, Text or the plugins' code waits on a name.

use std::collections::BTreeMap;

use super::no_writing_in_place::{all_code, code};

const WHERE: [&str; 5] = ["screenshot/file_chooser/lent.rs", "toolkit/common/file_work/", "text/", "plugins/", "store/plugin_page.rs"];

const LEFT: [(&str, usize, &str); 2] = [
    ("toolkit/common/file_work/gentle/mod.rs", 1, "a folder opened only to ask its drive to keep what it was given, which cannot be a pipe: the opening refuses anything but a folder"),
    ("plugins/launcher.rs", 2, "the ring's own launcher files in the applications folder, read to tell its own from another program's; plain text files Trindle wrote, never a pipe the person named"),
];

fn ways() -> [String; 7] {
    [["File::", "open("].concat(), ["fs::", "read("].concat(), ["fs::", "read_to_string("].concat(), ["ImageReader::", "open("].concat(),
     ["image::", "open("].concat(), ["fs::", "copy("].concat(), ["image_", "dimensions("].concat()]
}

fn opened_to_read() -> String {
    [".read", "(true)"].concat()
}

fn never_waits() -> [String; 3] {
    [["O_NON", "BLOCK"].concat(), ["O_", "PATH"].concat(), ["create_", "new("].concat()]
}

fn kernels() -> String {
    ["/pr", "oc/"].concat()
}

const NEAR: usize = 2;

fn count(text: &str) -> usize {
    let lines = code(text);
    let (ways, opened, safe, kernel) = (ways(), opened_to_read(), never_waits(), kernels());
    (0..lines.len())
        .filter(|&i| !lines[i].trim_start().starts_with("//") && !lines[i].contains(&kernel))
        .filter(|&i| {
            let near = &lines[i.saturating_sub(NEAR)..(i + NEAR + 1).min(lines.len())];
            ways.iter().any(|w| lines[i].contains(w.as_str()))
                || (lines[i].contains(&opened) && !near.iter().any(|l| safe.iter().any(|s| l.contains(s.as_str()))))
        })
        .count()
}

#[test]
fn an_opening_by_name_is_counted_and_one_that_cannot_wait_is_not() {
    let [open, read, text, picture, image, copy, size] = &ways();
    let (opened, [quick, named, new], kernel) = (opened_to_read(), never_waits(), kernels());
    let plain = format!("let f = {open}p)?;\n// {open}p)\nlet b = std::{read}p)?;\nlet t = {text}p)?;\nlet i = {picture}p)?;\nlet j = {image}p)?;\n\
                         let g = OpenOptions::new(){opened}.open(p)?;\nstd::{copy}from, to)?;\nlet d = image::{size}p)?;\n#[cfg(test)]\nmod tests {{\n    {open}p);\n}}\n");
    assert_eq!(count(&plain), 8);
    let safe = format!("let m = {text}\"{kernel}self/mounts\")?;\nlet f = OpenOptions::new(){opened}.custom_flags(libc::{quick}).open(p)?;\n\
                        let g = OpenOptions::new(){opened}\n    .custom_flags(libc::{named})\n    .open(p)?;\nlet h = File::options(){opened}.{new}true).open(p)?;\n\
                        let k = crate::common::files::open_file(p)?;\n");
    assert_eq!(count(&safe), 0);
}

#[test]
fn nothing_in_folders_text_or_plugins_opens_a_name_to_read_it_but_by_the_one_opener() {
    let found: BTreeMap<String, usize> = all_code().into_iter()
        .filter(|(file, _)| WHERE.iter().any(|w| file.starts_with(w)))
        .map(|(file, text)| (file, count(&text)))
        .filter(|(_, n)| *n > 0)
        .collect();
    let listed: BTreeMap<String, usize> = LEFT.iter().map(|(file, n, _)| (file.to_string(), *n)).collect();
    assert_eq!(listed.len(), LEFT.len(), "a file is listed twice");
    assert!(LEFT.iter().all(|(_, _, why)| !why.is_empty()), "each one listed says why");
    let wrong: Vec<String> = found.iter().filter(|(file, n)| listed.get(*file) != Some(n))
        .map(|(file, n)| format!("{file} opens a name to read it {n} times, and {} are listed", listed.get(file).copied().unwrap_or(0)))
        .chain(listed.keys().filter(|file| !found.contains_key(*file)).map(|file| format!("{file} is listed and no longer opens any")))
        .collect();
    assert!(wrong.is_empty(), "use common::files::open_file, read_file or read_string, or list it in LEFT with its reason: {wrong:#?}");
}
