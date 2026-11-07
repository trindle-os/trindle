//! Files Trindle keeps as copies of the toolkit's, so a clean clone builds and packages without the toolkit's folder beside it.

use std::path::Path;

const COPIES: [(&str, &str); 3] = [
    ("share/style.css", "share/style.css"),
    ("src/said.rs", "src/said.rs"),
    ("src/compose/protocols/own_press.xml", "protocols/own_press.xml"),
];

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

#[test]
fn every_copy_is_the_toolkits_word_for_word() {
    let (here, toolkit) = (crate::common::source::root(), crate::common::source::toolkit_root());
    for (copy, original) in COPIES {
        assert!(read(&here.join(copy)) == read(&toolkit.join(original)),
                "{copy} differs from the toolkit's {original}: copy it again (cp {} {copy})", toolkit.join(original).display());
    }
}

#[test]
fn the_themes_are_the_toolkits_every_one() {
    let (here, toolkit) = (crate::common::source::root().join("share/themes"), crate::common::source::toolkit_root().join("share/themes"));
    let names = |dir: &Path| {
        let mut all: Vec<String> = std::fs::read_dir(dir).unwrap().flatten().filter_map(|e| e.file_name().into_string().ok()).collect();
        all.sort();
        all
    };
    assert_eq!(names(&here), names(&toolkit), "share/themes holds other files than the toolkit's: copy them again (cp {}/*.css share/themes/)", toolkit.display());
    for name in names(&toolkit) {
        assert!(read(&here.join(&name)) == read(&toolkit.join(&name)), "share/themes/{name} differs from the toolkit's: copy it again");
    }
}

#[test]
fn the_lock_names_the_commit_cargo_toml_names() {
    let here = crate::common::source::root();
    let manifest = read(&here.join("Cargo.toml"));
    let rev_of = |name: &str| {
        let rev = manifest.lines().find_map(|l| l.strip_prefix(name)?.strip_prefix(" = ")?.split("rev = \"").nth(1)?.split('"').next().map(String::from)).unwrap_or_else(|| panic!("Cargo.toml names the commit of {name}"));
        assert_eq!(rev.len(), 40, "a commit in full, not a name that moves");
        rev
    };
    let lock = read(&here.join("Cargo.lock"));
    let patched = std::fs::read_to_string(here.join(".cargo/config.toml")).is_ok_and(|c| c.contains("[patch"));
    let mut seen = std::collections::BTreeSet::new();
    let mut lines = lock.lines();
    while let Some(line) = lines.next() {
        let (repo, rev) = match line {
            "name = \"trindle-toolkit\"" | "name = \"trindle-common\"" => ("trindle-toolkit", rev_of("trindle-toolkit")),
            "name = \"trindle-help\"" => ("trindle-help", rev_of("trindle-help")),
            _ => continue,
        };
        seen.insert(line);
        let source = lines.nth(1).and_then(|l| l.strip_prefix("source = ")).map(|s| s.trim_matches('"').to_string());
        match source {
            Some(s) => assert!(s == format!("git+https://github.com/trindle-os/{repo}?rev={rev}#{rev}"), "Cargo.lock names {s}, not {rev}: run tools/lock.sh"),
            None => assert!(patched, "Cargo.lock holds no source for {line}, so a clean clone's --locked build would fail: run tools/lock.sh"),
        }
    }
    assert_eq!(seen.len(), 3, "the toolkit's two packages and Help are in Cargo.lock");
}
