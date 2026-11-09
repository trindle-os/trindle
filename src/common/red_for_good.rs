//! Red is for an act that cannot be undone, and for a failure, and for
//! nothing else (the rule for every design). "Wipe securely" was once drawn in the highlight,
//! as "Delete permanently" and "Empty the Trash" were too: each a
//! screen's main action, and each said under it "This cannot be undone".

use super::save_is_a_word::all_lines;

const MARK: &str = ".cannot_be_undone()";

const FOR_GOOD: [(&str, &str, &str); 6] = [
    ("store/pages/remove_ask.rs", "main_word(t!(\"store-flatpak-wipe\")).cannot_be_undone()", "Remove a Flatpak and delete its saved data: the data is gone for good"),
    ("store/pages/remove_ask.rs", "main_word(t!(\"store-flatpak-data-only\")).cannot_be_undone()", "Delete a Flatpak's saved data only: the data is gone for good"),
    ("ring/view/menu.rs", "main_word(t!(\"store-flatpak-wipe\")).cannot_be_undone()", "Remove a Flatpak from the ring and delete its saved data: the data is gone for good"),
    ("ring/view/menu.rs", "main_word(t!(\"store-flatpak-data-only\")).cannot_be_undone()", "Delete a Flatpak's saved data only, from the ring: the data is gone for good"),
    ("screenshot/file_chooser/draw.rs", "main_action(lent::SHRED_ICON, t!(\"screenshot-chooser-delete\")).cannot_be_undone()", "Delete a file in the file chooser: it is gone for good"),
    ("installer/pages.rs", "install.cannot_be_undone()", "Install over a whole disk: everything on it is replaced"),
];

const TELLS: [&str; 2] = ["cannot be undone", "erase and install"];

const NO_BUTTON: [(&str, &str); 4] = [
    ("settings/help_topics.rs", "Help's own words about erasing a drive: a page to read, with nothing to press"),
    ("settings/panels/snapshots.rs", "the words behind the card's i; clearing the copies is a round remove button that asks twice, and whether those turn red is not yet decided"),
    ("common/no_writing_in_place.rs", "a list for a test, which says what the wipe tells the person"),
    ("common/red_for_good.rs", "this file, which names the words"),
];

#[test]
fn every_red_main_action_is_on_the_list_with_what_cannot_be_undone() {
    let marked: Vec<(String, String)> = all_lines().into_iter()
        .filter(|(file, line)| line.contains(MARK) && !file.starts_with("toolkit/") && file != "common/red_for_good.rs").collect();
    let listed = |file: &str, line: &str| FOR_GOOD.iter().any(|(f, words, _)| *f == file && line.contains(words));
    let unlisted: Vec<_> = marked.iter().filter(|(file, line)| !listed(file, line)).collect();
    assert!(
        unlisted.is_empty(),
        "a main action is marked as one that cannot be undone, and so drawn red, and is not on the list in common/red_for_good.rs. \
         Red is only for an act that cannot be undone: add it there with what that is, or take the mark away: {unlisted:#?}"
    );
    for (file, words, what) in FOR_GOOD {
        let lines: Vec<_> = marked.iter().filter(|(f, line)| f == file && line.contains(words)).collect();
        assert_eq!(lines.len(), 1, "{file}: the main action for \"{what}\" is no longer marked on a line with {words}");
    }
}

#[test]
fn every_file_that_says_an_act_cannot_be_undone_marks_its_main_action() {
    let all = all_lines();
    let mut wrong = Vec::new();
    let mut says: Vec<&str> = all.iter().filter(|(_, line)| TELLS.iter().any(|t| line.to_lowercase().contains(t))).map(|(f, _)| f.as_str()).collect();
    says.dedup();
    for file in says {
        let marks = all.iter().any(|(f, line)| f == file && line.contains(MARK));
        let excused = NO_BUTTON.iter().any(|(f, _)| *f == file);
        if !marks && !excused {
            wrong.push(file);
        }
    }
    assert!(wrong.is_empty(), "these files tell a person that something cannot be undone, and no main action in them is marked with {MARK}, so the button is the highlight and not red: {wrong:?}");

    let words = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("share/lang/en/installer.ftl")).unwrap_or_default();
    assert!(TELLS.iter().any(|t| words.to_lowercase().contains(t)), "the installer's file no longer tells a person that a whole disk is erased");
    assert!(all.iter().any(|(f, line)| f == "installer/pages.rs" && line.contains(MARK)), "the installer no longer marks its main action");
}
