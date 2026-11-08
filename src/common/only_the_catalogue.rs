//! Nobody installs a plugin themselves. A
//! plugin is submitted, tried by Trindle's own people and added to
//! Trindle's signed catalogue, and the catalogue is the only way one is
//! ever installed. The other ways in were taken out whole: a folder added
//! in Apps, a typed command that installed or checked a maker's folder,
//! and an assistant's two tools. This holds the source and what
//! ships beside it to that, so none comes back by a side door.

use std::path::Path;

use super::source::code;

fn gone() -> Vec<String> {
    [
        ["install_", "plugin"], ["check_", "plugin"], ["trindle-plugin ", "install"], ["trindle-plugin ", "check"],
        ["plugin-", "catalogues"], ["folder you ", "added"], ["folder of ", "plugins"], ["From::", "Folder"],
        ["FromA", "Folder"], ["install_", "asked"], ["install_", "told"], ["install_", "came"],
        ["may_be_", "asked"], ["may_take_", "the_place"], ["By::", "Typed"], ["By::", "Asked"], ["Came::", "Hand"],
        ["add_", "place("], ["clashes_with_", "installed"],

        ["from your ", "folder"], ["Remove it first to ", "use"], ["its source could not be ", "saved"],
        ["Where it came from ", "could not be noted"], ["catalogue.ini could not ", "be read"], ["not from Trindle's ", "catalogue"],
        ["installed from ", "a folder"], ["from a folder you ", "added"],
    ].iter().map(|pieces| pieces.concat()).collect()
}

fn scan(dir: &Path, top: &Path, gone: &[String], found: &mut Vec<String>) {
    let Ok(list) = std::fs::read_dir(dir) else { return };
    for entry in list.flatten() {
        let path = entry.path();
        if path.file_name().is_some_and(|n| n == ".git") { continue; }
        if path.is_dir() {
            scan(&path, top, gone, found);
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        for word in gone.iter().filter(|word| text.contains(word.as_str())) {
            found.push(format!("{word} in {}", path.strip_prefix(top).unwrap_or(&path).display()));
        }
    }
}

#[test]
fn no_word_of_another_way_in_is_left() {
    let top = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut found = Vec::new();
    for part in ["src", "spec", "docs", "share", "system", "tools", "config", "tests", "../trindle-site"] {
        scan(&top.join(part), top, &gone(), &mut found);
    }
    let toolkit = crate::common::source::toolkit_root();
    for part in ["src", "share", "docs"] {
        scan(&toolkit.join(part), toolkit, &gone(), &mut found);
    }
    found.sort();
    assert!(found.is_empty(), "another way to install a plugin than the catalogue, or words of one:\n{}", found.join("\n"));
}

#[test]
fn the_catalogue_is_the_only_way_a_plugin_is_installed() {

    for taken_out in ["install", "check"] {
        let asked = ["trindle-plugin", taken_out, "/nonexistent/plugin"].map(String::from).to_vec();
        assert_eq!(crate::plugins::main(asked), 2, "trindle-plugin {taken_out} is still a command");
    }

    let tools: Vec<&str> = crate::mcp::TOOLS.iter().map(|t| t.name).filter(|name| name.contains("plugin")).collect();
    assert!(tools.is_empty(), "an assistant's tool for plugins: {tools:?}");

    let before_tests = |text: &'static str| text.split_once("#[cfg(test)]").map_or(text, |(before, _)| before);
    let install = before_tests(code(include_str!("../plugins/install.rs")));
    assert!(install.contains("\npub(super) fn put_in(root: &Path, from: &Path) -> Result<Installed, String> {"), "the one install, and private");
    let plugins = [
        code(include_str!("../plugins/mod.rs")),
        install,
        code(include_str!("../plugins/background.rs")),
        code(include_str!("../plugins/fields.rs")),
        code(include_str!("../plugins/manifest.rs")),
        code(include_str!("../plugins/menu.rs")),
        code(include_str!("../plugins/notes.rs")),
        code(include_str!("../plugins/permissions.rs")),
        code(include_str!("../plugins/ports.rs")),
    ]
    .map(before_tests);
    assert_eq!(plugins.iter().map(|text| text.matches("put_in(").count()).sum::<usize>(), 1, "the plugins' own files call their install");
    assert!(plugins.iter().all(|text| !text.contains("pub fn install(") && !text.contains("pub fn install_")), "an install anything outside the plugins' files could call");
    let catalogue = before_tests(code(include_str!("../plugins/catalogue.rs")));
    assert_eq!(catalogue.matches("put_in(").count(), 2);
    for caller in ["install_offer", "update_unseen"] {
        let body = catalogue.split_once(&format!("\nfn {caller}(")).expect(caller).1.split_once("\n}\n").expect(caller).0;
        let (checked, put) = (body.find("fetched(offer, get)?").expect(caller), body.find("put_in(root, &at)").expect(caller));
        assert!(checked < put, "{caller} installs what was fetched and checked, after it was");
    }
    for other in [code(include_str!("../plugins/shortcuts.rs")), code(include_str!("../plugins/log.rs"))] {
        assert!(!other.contains("put_in("));
    }

    for set in ["env::var", "setting(", "config_path("] {
        assert!(!catalogue.contains(set), "the catalogue reads {set}, so it could be pointed somewhere else");
    }
    assert_eq!(catalogue.matches("Trust::own()").count(), 3, "each reading of the address (Apps', the ring's, and the kept copy read when looking by itself is off) is held to the built-in keys");
}

#[test]
fn the_release_has_no_way_to_make_sign_or_add_a_plugin() {
    let signing: Vec<String> = [["Ed25519", "KeyPair"], ["generate_", "pkcs8"], ["from_", "pkcs8"], ["new_key_", "main"], ["sign_", "main"], ["pair.", "sign("]]
        .iter().map(|p| p.concat()).collect();
    fn walk(dir: &Path, top: &Path, signing: &[String], found: &mut Vec<String>) {
        let Ok(list) = std::fs::read_dir(dir) else { return };
        for entry in list.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, top, signing, found);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let name = path.strip_prefix(top).unwrap_or(&path).display().to_string();
                if name.ends_with("plugins/catalogue/publisher.rs") || name.ends_with("common/only_the_catalogue.rs") {
                    continue;
                }
                let Ok(text) = std::fs::read_to_string(&path) else { continue };
                let text: &'static str = Box::leak(text.into_boxed_str());
                let release = code(text).split("#[cfg(test)]").next().unwrap_or_default();
                for word in signing {

                    if name.ends_with("plugins/mod.rs") && word.ends_with("_main") {
                        continue;
                    }
                    if release.contains(word.as_str()) {
                        found.push(format!("{word} in {name}"));
                    }
                }
            }
        }
    }
    let top = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut found = Vec::new();
    walk(&top.join("src"), top, &signing, &mut found);
    found.sort();
    assert!(found.is_empty(), "code that makes or signs outside the publisher's file:\n{}", found.join("\n"));

    let gate = "#[cfg(any(test, feature = \"publisher\"))]";
    let catalogue = code(include_str!("../plugins/catalogue.rs"));
    assert!(catalogue.contains(&format!("{gate}\npub mod publisher;")), "the publisher's file is not behind its feature");
    let plugins = code(include_str!("../plugins/mod.rs")).split("#[cfg(test)]").next().unwrap_or_default();
    for command in ["catalogue", "new-key", "sign"] {
        let arm = plugins.find(&format!("(\"{command}\", ")).unwrap_or_else(|| panic!("the {command} arm"));
        assert!(plugins[..arm].trim_end().ends_with(gate), "{command} is a command in the release");
    }
    assert!(!plugins.contains("catalogue FOLDER") && !plugins.contains("new-key KEYFILE"), "the release says it can make a catalogue");

    let cargo = include_str!("../../Cargo.toml");
    let features = cargo.split_once("[features]").expect("the features").1.split("\n[").next().unwrap_or_default();
    assert!(!features.contains("default"), "a feature is on by default: {features}");
    assert!(features.contains("test-catalogue = [\"publisher\"]"));
    let real = catalogue.split_once("#[cfg(not(feature = \"test-catalogue\"))]\npub const KEYS").expect("the real keys").1.split(';').next().unwrap_or_default();
    let test = catalogue.split_once("#[cfg(feature = \"test-catalogue\")]\npub const KEYS").expect("the test keys").1.split(';').next().unwrap_or_default();
    let key = |s: &str| s.split('"').nth(1).unwrap_or_default().to_string();
    assert!(!key(real).is_empty() && !key(test).is_empty() && key(real) != key(test), "the test build must have a key of its own");
}
