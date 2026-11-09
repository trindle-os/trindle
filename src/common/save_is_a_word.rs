//! Save, Save as and Replace are words with no icon, everywhere: in a
//! question, in the file chooser, in a toolbar, in the bar of Text that
//! replaces words, and beside the boxes of a Settings page. The first test
//! here let a worded button keep its icon, so the file chooser's Save
//! kept its floppy beside the word, chosen on a line that names no
//! button (which the test once missed).

use std::path::Path;

const ROUND: [(&str, &str, &str); 0] = [];

const SAVING: [&str; 3] = ["SAVE_ICON", "media-floppy", "document-save"];

const ICONS: [(&str, &str, &str); 16] = [
    ("toolkit/common/ui.rs", "pub const SAVE_ICON", "the one name the floppy has"),
    ("settings/ui.rs", "pub use crate::common::ui::SAVE_ICON", "the same name, for Settings' pages"),
    ("disks/mod.rs", "const SAVE_ICON", "Drives' own name for it"),
    ("disks/rows.rs", "t!(\"disks-rename-tip\")", "keeps a drive's new name"),
    ("disks/rows.rs", "t!(\"disks-use-folder\")", "keeps where a drive is opened"),
    ("settings/panels/about.rs", "use crate::settings::ui::", "the name brought in"),
    ("settings/panels/about.rs", "t!(\"settings-about-rename\")", "keeps the computer's new name"),
    ("settings/panels/datetime.rs", "use crate::settings::ui::", "the name brought in"),
    ("settings/panels/datetime.rs", "t!(\"settings-datetime-set-date-time\")", "sets the clock"),
    ("settings/panels/power.rs", "use crate::settings::ui::", "the name brought in"),
    ("settings/panels/power.rs", "t!(\"settings-power-pin-keep\")", "keeps a new PIN"),
    ("store/side.rs", "(\"backup\".into(), move_apps().into()", "the backup page's line in Apps' sidebar"),
    ("store/pages/github_key.rs", "t!(\"store-gh-key-keep\")", "keeps a key typed in"),
    ("store/pages/backup_page.rs", "t!(\"store-bk-making-tip\")", "the ring that runs while a backup is made"),
    ("store/pages/backup_page.rs", "t!(\"store-bk-make\")", "makes a backup, on two lines: waiting and ready"),
    ("store/pages/backup_page.rs", "list_card(\"document-save-symbolic\", t!(\"store-bk-card\")", "the card that says what a backup holds"),
];

fn named_save(line: &str) -> bool {
    let makes = ["button(", "main_action(", "main_word("].iter().any(|b| line.contains(b));
    let named = ["\"Save", "\"Replace"].iter().any(|word| {
        line.match_indices(word).any(|(at, _)| line[at + word.len()..].starts_with(['"', ' ', ':']))
    });
    makes && named
}

fn save_with_an_icon(line: &str) -> bool {
    let line = line.trim_start();
    if line.starts_with("//") || !named_save(line) {
        return false;
    }
    let plain = |maker: &str| line.match_indices(maker).any(|(at, _)| !line[..at].ends_with(|c: char| c.is_ascii_alphanumeric() || c == '_'));
    plain("button(") || plain("main_action(") || SAVING.iter().any(|icon| line.contains(icon))
}

fn scan(dir: &Path, src: &Path, prefix: &str, found: &mut Vec<(String, String)>) {
    let Ok(list) = std::fs::read_dir(dir) else { return };
    for entry in list.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan(&path, src, prefix, found);
            continue;
        }

        let file = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        if file == "save_is_a_word.rs" || file == "tests.rs" || file.ends_with("_tests.rs") || !file.ends_with(".rs") {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(&path) else { continue };
        let name = format!("{prefix}{}", path.strip_prefix(src).unwrap_or(&path).to_string_lossy());

        let lines: Vec<&str> = text.lines().collect();
        let tests = (0..lines.len()).find(|&i| lines[i].starts_with("#[cfg(test)]") && lines.get(i + 1).is_some_and(|next| next.contains("mod tests") && next.trim_end().ends_with('{')));
        found.extend(lines[..tests.unwrap_or(lines.len())].iter().filter(|l| !l.trim_start().starts_with("//")).map(|l| (name.clone(), l.trim().to_string())));
    }
}

pub(super) fn all_lines() -> Vec<(String, String)> {
    let mut found = Vec::new();
    for (src, prefix) in crate::common::source::trees() {
        scan(&src, &src, prefix, &mut found);
    }
    found
}

#[test]
fn a_save_or_replace_button_with_an_icon_is_told_from_one_of_its_word_alone() {
    assert!(save_with_an_icon("    button(SAVE_ICON, \"Save\").on_press(Msg::Answer(Answer::Save)),"));
    assert!(save_with_an_icon("quiet.push(button(REPLACE, \"Replace: the old one goes\").on_press(a));"));
    assert!(save_with_an_icon("for (icon, name) in [(crate::settings::ui::SAVE_ICON, \"Save\")] { foot.push(button(icon, name)); }"));

    assert!(save_with_an_icon("    main_action(SAVE_ICON, \"Save\").on_press(Msg::Answer(Answer::Save)),"));
    assert!(save_with_an_icon("    (main_action(REPLACE, \"Replace\").on_press(answer(Answer::Replace)), \"The same answer.\")"));
    assert!(save_with_an_icon("    button(\"document-save-as-symbolic\", \"Save as (Ctrl+Shift+S)\").on_press(Msg::SaveAs),"));
    assert!(!save_with_an_icon("    main_word(\"Save\").on_press(Msg::Answer(Answer::Save)),"));
    assert!(!save_with_an_icon("    word_button(\"Replace\").on_press(answer(Answer::Replace))"));
    assert!(!save_with_an_icon("    word_button(\"Save as\").spoken(\"Save as (Ctrl+Shift+S)\").on_press(Msg::SaveAs),"));
    assert!(!save_with_an_icon("    button(SAVE_ICON, \"Keep this PIN\").on_press(keep),"));
    assert!(!save_with_an_icon("    kit.say(&key, \"Saved\", true, false);"));
    assert!(!save_with_an_icon("    // button(SAVE_ICON, \"Save\")"));
}

#[test]
fn no_save_or_replace_choice_has_an_icon() {
    let found: Vec<(String, String)> = all_lines().into_iter().filter(|(_, line)| save_with_an_icon(line)).collect();
    let listed = |file: &str, line: &str| ROUND.iter().any(|(f, words, _)| *f == file && line.contains(words));
    let bare: Vec<_> = found.iter().filter(|(file, line)| !listed(file, line)).collect();
    assert!(bare.is_empty(), "a save or replace choice has an icon; give it its word alone (word_button, or main_word for a screen's one main action): {bare:#?}");

    for (file, words, why) in ROUND {
        assert_eq!(found.iter().filter(|(f, line)| f == file && line.contains(words)).count(), 1, "{file}: {words} ({why})");
    }
}

#[test]
fn an_icon_for_saving_is_named_only_where_the_list_says() {
    let found: Vec<(String, String)> = all_lines().into_iter().filter(|(_, line)| SAVING.iter().any(|icon| line.contains(icon))).collect();
    let listed = |file: &str, line: &str| ICONS.iter().any(|(f, words, _)| *f == file && line.contains(words));
    let unlisted: Vec<_> = found.iter().filter(|(file, line)| !listed(file, line)).collect();
    assert!(
        unlisted.is_empty(),
        "an icon for saving is named on a line that is not on the list in common/save_is_a_word.rs. Save, Save as and Replace take no icon; \
         if this is another button, add its line to ICONS with what it is: {unlisted:#?}"
    );
    for (file, words, what) in ICONS {
        assert!(found.iter().any(|(f, line)| f == file && line.contains(words)), "{file}: {words} ({what}) is on the list and gone from the source");
    }
}

#[test]
fn a_file_left_out_as_tests_is_built_for_tests_alone() {
    fn find(dir: &Path, tests: &mut Vec<String>, mods: &mut Vec<(String, bool)>) {
        let Ok(list) = std::fs::read_dir(dir) else { return };
        for entry in list.flatten() {
            let path = entry.path();
            if path.is_dir() {
                find(&path, tests, mods);
                continue;
            }
            let file = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
            if let Some(stem) = file.strip_suffix(".rs").filter(|s| s.ends_with("_tests")) {
                tests.push(stem.to_string());
            }
            let Ok(text) = std::fs::read_to_string(&path) else { continue };
            let lines: Vec<&str> = text.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                let line = line.trim();
                let name = line.strip_prefix("mod ").or_else(|| line.strip_prefix("pub mod ")).or_else(|| line.strip_prefix("pub(crate) mod "));
                if let Some(name) = name.and_then(|n| n.strip_suffix(';')).filter(|n| n.ends_with("_tests")) {

                    let above = lines[..i].iter().rev().map(|l| l.trim()).find(|l| !l.starts_with("#[cfg_attr("));
                    mods.push((name.to_string(), matches!(above, Some(l) if l == "#[cfg(test)]" || l == "#[cfg(any(test, feature = \"testing\"))]" || l.starts_with("#[cfg(all(test, "))));
                }
            }
        }
    }
    let (mut tests, mut mods) = (Vec::new(), Vec::new());
    for (src, _) in crate::common::source::trees() {
        find(&src, &mut tests, &mut mods);
    }
    assert!(!tests.is_empty(), "no file of tests was found");
    for stem in &tests {
        let declared: Vec<bool> = mods.iter().filter(|(n, _)| n == stem).map(|(_, marked)| *marked).collect();
        assert!(!declared.is_empty() && declared.iter().all(|m| *m),
                "{stem}.rs is left out of the scan as tests and is not declared under #[cfg(test)]");
    }
}
