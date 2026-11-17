//! American English (share/lang/en-US/) holds only the lines that differ from
//! the British English in share/lang/en/. The tests keep the two honest: any
//! English line with a British form must have an American one, and no
//! American line may keep a British form. `write_american_files` (ignored)
//! makes the files from the English; run it after changing English words.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const STEMS: &[(&str, &str)] = &[
    ("colour", "color"), ("favour", "favor"), ("behaviour", "behavior"), ("neighbour", "neighbor"), ("honour", "honor"),
    ("labour", "labor"), ("flavour", "flavor"), ("humour", "humor"), ("rumour", "rumor"), ("harbour", "harbor"),
    ("odour", "odor"), ("vapour", "vapor"), ("savour", "savor"), ("armour", "armor"), ("endeavour", "endeavor"),
    ("licence", "license"), ("defence", "defense"), ("offence", "offense"), ("pretence", "pretense"),
    ("centre", "center"), ("metre", "meter"), ("litre", "liter"), ("fibre", "fiber"), ("theatre", "theater"), ("calibre", "caliber"),
    ("catalogue", "catalog"), ("dialogue", "dialog"), ("analogue", "analog"), ("monologue", "monolog"),
    ("grey", "gray"), ("cancell", "cancel"), ("travell", "travel"), ("labell", "label"), ("modell", "model"),
    ("signall", "signal"), ("fuell", "fuel"), ("enrol", "enroll"), ("fulfil", "fulfill"), ("instalment", "installment"),
    ("skilful", "skillful"), ("wilful", "willful"), ("ageing", "aging"), ("judgement", "judgment"),
    ("acknowledgement", "acknowledgment"), ("aeroplane", "airplane"), ("petrol", "gas"), ("programme", "program"),
    ("aluminium", "aluminum"), ("mould", "mold"), ("sceptic", "skeptic"), ("artefact", "artifact"), ("cheque", "check"),
    ("kerb", "curb"), ("tyre", "tire"), ("plough", "plow"), ("storey", "story"), ("draught", "draft"),
    ("jewellery", "jewelry"), ("pyjamas", "pajamas"), ("manoeuvre", "maneuver"), ("paediatric", "pediatric"),
    ("anaesthetic", "anesthetic"), ("diarrhoea", "diarrhea"), ("whilst", "while"), ("amongst", "among"),
    ("learnt", "learned"), ("spelt", "spelled"), ("burnt", "burned"), ("towards", "toward"), ("postcode", "ZIP code"),
    ("anticlockwise", "counterclockwise"), ("biscuit", "cookie"), ("fortnight", "two weeks"), ("bin", "trash can"),
    ("tick", "check"),
];

const ENDINGS: &[&str] = &["", "s", "es", "ed", "ing", "ings", "ful", "fully", "less", "able", "ite", "ites", "er", "ers", "y", "d", "ment", "ments", "ist", "ists"];

const PHRASES: &[(&str, &str)] = &[
    ("a tick", "a check mark"), ("ticks", "check marks"), ("tick boxes", "check boxes"), ("tick box", "check box"),
    ("petrol pump", "gas pump"), ("full stop", "period"), ("post code", "ZIP code"),
];

const KEEP_ISE: &[&str] = &[
    "raise", "raised", "raises", "raising", "rise", "rises", "rising", "sunrise", "sunrises", "surprise", "surprised", "surprises", "surprising",
    "otherwise", "likewise", "clockwise", "wise", "promise", "promised", "promises", "exercise", "exercises", "exercised", "advise", "advised",
    "advises", "noise", "noises", "precise", "concise", "premise", "premises", "praise", "arise", "arises", "arising", "paradise", "compromise",
    "comprise", "comprises", "comprised", "disguise", "disguised", "revise", "revised", "revises", "revising", "supervise", "supervised",
    "supervises", "supervising", "devise", "devised", "enterprise", "expertise", "franchise", "merchandise", "improvise", "improvised",
    "televise", "demise", "despise", "excise", "incise", "treatise", "bruise", "bruised", "cruise", "cruises", "poise", "turquoise", "reprise",
    "advertise", "advertised", "advertises", "advertising", "chastise", "circumcise", "apprise", "apprised", "mortise", "chemise", "valise",
    "compromised", "compromises", "compromising", "exorcise", "tortoise", "porpoise", "guise", "cheerio",
];

fn word_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_' | '$' | '-' | '/')
}

fn like(from: &str, to: &str) -> String {
    if from.len() > 1 && from.chars().all(|c| !c.is_lowercase()) {
        to.to_uppercase()
    } else if from.chars().next().is_some_and(char::is_uppercase) {
        let mut chars = to.chars();
        chars.next().map(|f| f.to_uppercase().chain(chars).collect()).unwrap_or_default()
    } else {
        to.to_string()
    }
}

fn american_word(lower: &str) -> Option<String> {
    for (british, american) in STEMS {
        if let Some(rest) = lower.strip_prefix(british) {
            if ENDINGS.contains(&rest) {
                return Some(format!("{american}{rest}"));
            }
        }
    }
    if KEEP_ISE.contains(&lower) {
        return None;
    }
    for (end, new) in [("isations", "izations"), ("isation", "ization"), ("ised", "ized"), ("ises", "izes"), ("ising", "izing"), ("ise", "ize"),
                       ("ysed", "yzed"), ("yses", "yzes"), ("ysing", "yzing"), ("yse", "yze")] {
        if let Some(stem) = lower.strip_suffix(end) {
            if stem.len() >= 3 {
                return Some(format!("{stem}{new}"));
            }
        }
    }
    None
}

fn words(line: &str) -> Vec<(usize, usize)> {
    let mut found = Vec::new();
    let mut start = None;
    for (i, c) in line.char_indices().chain(std::iter::once((line.len(), ' '))) {
        match (word_char(c), start) {
            (true, None) => start = Some(i),
            (false, Some(s)) => {
                found.push((s, i));
                start = None;
            }
            _ => {}
        }
    }
    found
}

fn prose_start(line: &str) -> usize {
    let head = line.trim_start();
    let named = head.starts_with('.') || (!line.starts_with([' ', '\t']) && !head.starts_with(['#', '[', '*', '}', '{']));
    match (named, line.find(" = ")) {
        (true, Some(at)) => at + 3,
        _ => 0,
    }
}

fn american(line: &str) -> String {
    let from = prose_start(line);
    let (head, body) = line.split_at(from);
    let mut text = body.to_string();
    for (british, new) in PHRASES {
        let mut at = 0;
        while let Some(found) = text.to_lowercase()[at..].find(british) {
            let (a, b) = (at + found, at + found + british.len());
            let clear = |i: usize, back: bool| {
                let c = if back { text[..i].chars().next_back() } else { text[i..].chars().next() };
                !c.is_some_and(word_char)
            };
            if text.is_char_boundary(a) && text.is_char_boundary(b) && clear(a, true) && clear(b, false) {
                let put = like(&text[a..b], new);
                text.replace_range(a..b, &put);
                at = a + put.len();
            } else {
                at = b;
            }
        }
    }
    let mut out = String::new();
    let mut last = 0;
    for (a, b) in words(&text) {
        let word = &text[a..b];
        if word.contains(['_', '$', '-', '/']) || word.starts_with(|c: char| c.is_ascii_digit()) {
            continue;
        }
        if let Some(new) = american_word(&word.to_lowercase()) {
            out.push_str(&text[last..a]);
            out.push_str(&like(word, &new));
            last = b;
        }
    }
    out.push_str(&text[last..]);
    format!("{head}{out}")
}

fn blocks(text: &str) -> Vec<(String, Vec<String>)> {
    let mut out: Vec<(String, Vec<String>)> = Vec::new();
    let mut open = false;
    for line in text.lines() {
        let key = line.split_once(" =").map(|(k, _)| k).filter(|k| !k.is_empty() && !line.starts_with([' ', '\t', '#', '-']) && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'));
        if let Some(key) = key {
            out.push((key.to_string(), vec![line.to_string()]));
            open = true;
        } else if open && (line.starts_with([' ', '\t']) || line.trim().is_empty()) {
            out.last_mut().unwrap().1.push(line.to_string());
        } else {
            open = false;
        }
    }
    for (_, lines) in &mut out {
        while lines.last().is_some_and(|l| l.trim().is_empty()) {
            lines.pop();
        }
    }
    out
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("share/lang")
}

fn files(tag: &str) -> BTreeMap<String, String> {
    let mut all = BTreeMap::new();
    for entry in std::fs::read_dir(root().join(tag)).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|x| x == "ftl") {
            all.insert(entry.file_name().to_string_lossy().into_owned(), std::fs::read_to_string(&path).unwrap());
        }
    }
    all
}

fn messages(tag: &str) -> BTreeMap<String, Vec<String>> {
    files(tag).values().flat_map(|t| blocks(t)).collect()
}

fn differences() -> BTreeMap<String, Vec<(String, Vec<String>)>> {
    let mut out = BTreeMap::new();
    for (name, text) in files("en") {
        let changed: Vec<_> = blocks(&text).into_iter().filter_map(|(key, lines)| {
            let new: Vec<String> = lines.iter().map(|l| american(l)).collect();
            (new != lines).then_some((key, new))
        }).collect();
        if !changed.is_empty() {
            out.insert(name, changed);
        }
    }
    out
}

#[test]
#[ignore = "writes share/lang/en-US; run it by name after changing English words"]
fn write_american_files() {
    let dir = root().join("en-US");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    for (name, changed) in differences() {
        let mut text = String::from("# American English: only the lines that differ from English. The rest is English.\n");
        for (_, lines) in changed {
            text.push('\n');
            text.push_str(&lines.join("\n"));
            text.push('\n');
        }
        std::fs::write(dir.join(name), text).unwrap();
    }
}

#[test]
fn the_conversion_makes_the_usual_changes_and_leaves_names_and_placeholders_alone() {
    assert_eq!(american("a-key = Pick a colour for your favourite licence."), "a-key = Pick a color for your favorite license.");
    assert_eq!(american("k = Colour { $colour } and { settings-colour-x }"), "k = Color { $colour } and { settings-colour-x }");
    assert_eq!(american("k = Minimise, maximise and recognise it. Raise the sunrise."), "k = Minimize, maximize and recognize it. Raise the sunrise.");
    assert_eq!(american("k = Cancelled. Centre it. Press the bin; it shows a tick."), "k = Canceled. Center it. Press the trash can; it shows a check mark.");
    assert_eq!(american("    .colour = Greyed catalogues"), "    .colour = Grayed catalogs");
    assert_eq!(american("k = /usr/bin and tick boxes"), "k = /usr/bin and check boxes");
    assert_eq!(american("k = COLOUR"), "k = COLOR");
}

#[test]
fn american_english_has_no_british_spelling() {
    let mut left = Vec::new();
    for (key, lines) in messages("en-US") {
        for line in lines {
            if american(&line) != line {
                left.push(format!("{key}: {}", line.trim()));
            }
        }
    }
    assert!(left.is_empty(), "British forms in share/lang/en-US:\n{}", left.join("\n"));
}

#[test]
fn every_american_line_is_a_line_english_has() {
    let english = messages("en");
    let stray: Vec<String> = messages("en-US").into_keys().filter(|k| !english.contains_key(k)).collect();
    assert!(stray.is_empty(), "en-US has keys English does not: {stray:?}");
}

#[test]
fn every_english_line_with_a_british_form_has_an_american_line() {
    let us = messages("en-US");
    let missing: Vec<String> = differences().into_values().flatten().map(|(key, _)| key).filter(|k| !us.contains_key(k)).collect();
    assert!(missing.is_empty(), "English lines with a British form and no line in share/lang/en-US (run: cargo test --release --bin trindle write_american_files -- --ignored): {missing:?}");
}

#[test]
fn american_lines_keep_the_placeholders_of_the_english() {
    let (english, us) = (messages("en"), messages("en-US"));
    let holes = |lines: &[String]| {
        let mut all: Vec<String> = lines.iter().flat_map(|l| l.split('$').skip(1).map(|r| r.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect::<String>()).collect::<Vec<_>>()).collect();
        all.sort();
        all
    };
    for (key, lines) in &us {
        assert_eq!(holes(lines), holes(&english[key]), "{key}");
    }
}

#[test]
fn a_folder_of_differences_passes_the_missing_words_check() {
    use crate::toolkit::lang::check::{check, domains_at};
    let report = check(&domains_at(&PathBuf::from(env!("CARGO_MANIFEST_DIR"))));
    let ours: Vec<_> = report.problems.iter().filter(|p| p.contains("en-US")).collect();
    assert!(ours.is_empty(), "{ours:?}");
    let (done, total) = report.coverage.get("en-US").copied().expect("en-US is checked as a language");
    assert!(done > 0 && done < total, "{done} of {total}");
}

#[test]
fn american_english_is_chosen_for_a_us_computer_and_falls_back_to_english() {
    use crate::toolkit::lang::{load, tags_of};
    assert_eq!(tags_of("en_US.UTF-8"), ["en-US", "en"]);
    let bundle = load(&tags_of("en_US.UTF-8"), &[root()]);
    let (key, lines) = differences().into_values().flatten().next().expect("some line differs");
    let first = lines[0].split_once(" = ").map(|(_, v)| v.to_string()).unwrap_or_default();
    assert!(bundle.get(&key).is_some(), "{key} {first}");
    assert!(bundle.get("settings-region-looking").is_some(), "lines that do not differ come from English");
}
