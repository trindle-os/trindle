//! What a test needs of the computer it is run on, asked for in one way.

use std::path::{Path, PathBuf};

pub use crate::toolkit::common::needs::{file, file_with, program, programs};

const LEFT: &[(&str, &str, &str)] = &[(
    "settings/panels/thanks.rs",
    "the_website_thanks_the_same_projects_as_the_app",
    "the website is a repository of its own, trindle-site, beside this one; a clone of Trindle alone has no page to compare, and says so",
)];

fn ways() -> Vec<String> {
    let asks = ["can_pdf", "can_video", "can_play", "bwrap", "ffmpeg", "ffprobe", "pdftoppm"];
    let mut all: Vec<String> = asks.iter().map(|a| [a, "()"].concat()).collect();
    for found in ["find", "have"] {
        all.push(["run::", found, "("].concat());
    }
    for started in ["spawn", "output", "status"] {
        all.push([".", started, "().ok", "()"].concat());
        all.push([".", started, "() else"].concat());
    }
    all.push(["skipped", ":"].concat());
    all
}

fn defines(line: &str) -> Option<&str> {
    let rest = line.trim_start();
    let rest = rest.strip_prefix("pub(crate) ").or_else(|| rest.strip_prefix("pub(super) ")).or_else(|| rest.strip_prefix("pub ")).unwrap_or(rest);
    let name = rest.strip_prefix("fn ")?;
    Some(&name[..name.find(['(', '<']).unwrap_or(name.len())])
}

fn asks(line: &str, way: &str) -> bool {
    line.match_indices(way).any(|(at, _)| {
        let before = line[..at].chars().next_back();
        way.starts_with('.') || !before.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_')
    })
}

fn quiet_in(text: &str, whole: bool, ways: &[String]) -> Vec<(String, usize)> {
    let mut found = Vec::new();
    let (mut testing, mut cfg, mut ignored, mut ignore_next) = (whole, false, false, false);
    let mut function = String::new();
    for (n, line) in text.lines().enumerate() {
        let code = line.trim();
        if !testing {
            testing = cfg && (code.starts_with("mod ") || code.starts_with("pub mod ") || code.starts_with("pub(crate) mod ")) && code.ends_with('{');
            cfg = code == "#[cfg(test)]";
            continue;
        }
        if code.starts_with("#[ignore") {
            ignore_next = true;
        }
        if let Some(name) = defines(line) {
            function = name.to_string();
            ignored = std::mem::take(&mut ignore_next);
        }
        if code.starts_with("//") || ignored {
            continue;
        }
        if ways.iter().any(|w| asks(code, w)) {
            found.push((function.clone(), n + 1));
        }
    }
    found
}

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn only_tests(all: &[(PathBuf, String)]) -> Vec<PathBuf> {
    let mut only = Vec::new();
    for (path, text) in all {
        let dir = path.parent().unwrap();

        let stem = path.file_stem().unwrap().to_string_lossy();
        let home = if stem == "mod" || stem == "main" || stem == "lib" { dir.to_path_buf() } else { dir.join(&*stem) };
        let mut cfg = false;
        for line in text.lines().map(str::trim) {
            let named = line.strip_prefix("pub(crate) mod ").or_else(|| line.strip_prefix("pub mod ")).or_else(|| line.strip_prefix("mod "))
                .and_then(|m| m.strip_suffix(';'));
            if let Some(name) = named.filter(|_| cfg) {
                only.extend([home.join(format!("{name}.rs")), home.join(name).join("mod.rs"), dir.join(format!("{name}.rs"))]);
            }
            cfg = line == "#[cfg(test)]";
        }
    }
    only
}

#[test]
fn each_quiet_way_out_is_seen_and_the_loud_way_is_not() {
    let ways = ways();
    let line = |parts: &[&str]| parts.concat();
    let test = |body: &str| format!("fn real() {{}}\n#[cfg(test)]\nmod tests {{\n    #[test]\n    fn one() {{\n{body}\n    }}\n}}\n");
    let seen = |body: String| quiet_in(&test(&body), false, &ways).into_iter().map(|f| f.0).collect::<Vec<_>>();
    assert_eq!(seen(line(&["if !walled::", "can_play", "() { return; }"])), ["one"]);
    assert_eq!(seen(line(&["let Some(_) = ", "bwrap", "() else { return };"])), ["one"]);
    assert_eq!(seen(line(&["if run::", "find", "(\"nft\").is_none() { return; }"])), ["one"]);
    assert_eq!(seen(line(&["let out = Command::new(\"nft\").", "output", "().ok", "()?;"])), ["one"]);
    assert_eq!(seen(line(&["let Ok(out) = Command::new(\"nft\").", "output", "() else { return };"])), ["one"]);
    assert_eq!(seen(line(&["eprintln!(\"", "skipped", ": nft cannot be used here\");"])), ["one"]);

    assert!(seen("crate::common::needs::programs(&[\"unshare\", \"nft\"]);".into()).is_empty());
    assert!(seen(line(&["let x = needs_", "ffmpeg", "();"])).is_empty());
    assert!(seen(line(&["// ", "bwrap", "() is asked for elsewhere"])).is_empty());
    let ignored = line(&["#[cfg(test)]\nmod tests {\n    #[test]\n    #[ignore]\n    fn by_hand() {\n        if !", "can_play", "() { return; }\n    }\n}\n"]);
    assert!(quiet_in(&ignored, false, &ways).is_empty());
    let real = line(&["fn real() {\n    let _ = ", "bwrap", "();\n}\n"]);
    assert!(quiet_in(&real, false, &ways).is_empty());
    assert_eq!(quiet_in(&real, true, &ways).len(), 1, "a file that is only tests is tests from its first line");
}

#[test]
fn no_test_passes_by_leaving_when_a_program_is_missing() {
    let mut files = Vec::new();
    let mut names = std::collections::HashMap::new();
    for (tree, prefix) in crate::common::source::trees() {
        let before = files.len();
        sources(&tree, &mut files);
        for f in &files[before..] {
            names.insert(f.clone(), format!("{prefix}{}", f.strip_prefix(&tree).unwrap().to_string_lossy()));
        }
    }
    let all: Vec<(PathBuf, String)> = files.into_iter().filter_map(|f| Some((f.clone(), std::fs::read_to_string(&f).ok()?))).collect();
    let only = only_tests(&all);
    let ways = ways();
    let mut found = std::collections::BTreeSet::new();
    let mut quiet = Vec::new();
    for (path, text) in &all {
        let name = names[path].clone();
        for (function, line) in quiet_in(text, only.contains(path), &ways) {
            if LEFT.iter().any(|(f, func, _)| *f == name && *func == function) {
                found.insert((name.clone(), function));
            } else {
                quiet.push(format!("{name}:{line} in {function}"));
            }
        }
    }
    assert!(quiet.is_empty(), "these tests may pass having checked nothing, where a program is missing; \
            ask for the program with common::needs::program, so the test fails and names it: {quiet:#?}");
    let stale: Vec<_> = LEFT.iter().filter(|(f, func, _)| !found.contains(&(f.to_string(), func.to_string()))).collect();
    assert!(stale.is_empty(), "these are on the list of places left as they are, and are no longer there: {stale:#?}");
}
