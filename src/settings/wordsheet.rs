//! The word sheet of "Your own words": Trindle's English with a box for a translation, given a check value on export and read as hostile text on import.

use std::fmt::Write as _;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::toolkit::lang;
use crate::toolkit::t;

pub const KIND: &str = "Trindle word sheet";
pub const VERSION: &str = "1";

pub const NOTES: [&str; 2] = [
    "# Change only the third column. Every other part of this sheet is locked.",
    "# A sheet changed anywhere else, or not made by Trindle, is rejected whole and nothing is installed.",
];
const DOMAIN: &str = "trindle-word-sheet/1";

pub const FILE_MOST: u64 = 8 << 20;
const LINE_MOST: usize = 16 << 10;

const WORDS_MOST: usize = 2000;
const WORDS_TIMES: usize = 8;
const WORDS_LEAST: usize = 120;

const PART: &str = "zz-sheet-";
const PART_MOST: usize = 400 << 10;

const NOT_MORE: &[&str] = &["<", ">", "://", "../", "~/", "file:", "javascript:", "\\", "&#", "&lt", "&gt", "`", "$(", "${", "%n", "\u{1b}", "NUMBER("];

pub const WAY_BACK: [&str; 8] = ["settings-region-title", "settings-words-heading", "settings-words-reset", "settings-words-reset-sub",
                                 "settings-words-reset-own-sub", "settings-language-heading", "settings-language-pick", "settings-language-auto"];

#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    pub key: String,
    pub english: String,
    pub marks: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Why {
    TooBig,
    NotText,
    NotPlain,
    LineLong,
    NotMine,
    Version,
    Language,
    CheckValue,
    Notes,
    Count,
    Key(String),
    English(String),
    Shape(String),
    Long(String),
    Characters(String),
    Links(String),
    Placeholders(String),
    NotSheet,

    NotSaved,
    NameLength,
    NamePlain,
    NameTaken,
}

impl Why {
    pub fn words(&self) -> String {
        match self {
            Why::TooBig => t!("settings-words-too-big").to_string(),
            Why::NotText => t!("settings-words-not-text").to_string(),
            Why::NotPlain => t!("settings-words-not-plain").to_string(),
            Why::LineLong => t!("settings-words-line-long").to_string(),
            Why::NotMine => t!("settings-words-not-mine").to_string(),
            Why::Version => t!("settings-words-version").to_string(),
            Why::Language => t!("settings-words-bad-language").to_string(),
            Why::CheckValue => t!("settings-words-check-value").to_string(),
            Why::Notes => t!("settings-words-notes").to_string(),
            Why::Count => t!("settings-words-count").to_string(),
            Why::Key(k) => t!("settings-words-key", key = k),
            Why::English(k) => t!("settings-words-english", key = k),
            Why::Shape(k) => t!("settings-words-shape", key = k),
            Why::Long(k) => t!("settings-words-long", key = k),
            Why::Characters(k) => t!("settings-words-characters", key = k),
            Why::Links(k) => t!("settings-words-links", key = k),
            Why::Placeholders(k) => t!("settings-words-placeholders", key = k),
            Why::NotSheet => t!("settings-words-not-sheet").to_string(),
            Why::NotSaved => t!("settings-words-not-saved").to_string(),
            Why::NameLength => t!("settings-words-name-length").to_string(),
            Why::NamePlain => t!("settings-words-name-plain").to_string(),
            Why::NameTaken => t!("settings-words-name-taken").to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Sheet {
    pub lang: String,
    pub words: BTreeMap<String, String>,
}

pub fn rows_of(files: &[(&str, &str)]) -> Vec<Row> {
    let mut all: BTreeMap<String, Row> = BTreeMap::new();
    for (_, text) in files {
        for (key, english) in lang::raw_messages(text) {
            if all.contains_key(&key) {
                continue;
            }
            if let Some(marks) = lang::placeholders(&english) {
                all.insert(key.clone(), Row { key, english, marks });
            }
        }
    }
    all.into_values().collect()
}

pub fn english_rows() -> Vec<Row> {
    let own = lang::folders().into_iter().next();
    let added: Vec<(String, String)> = lang::folders().into_iter()
        .filter(|d| Some(d) != own.as_ref())
        .flat_map(|d| lang::folder_files(&d.join("en")))
        .collect();
    let mut files = lang::english_files();
    files.extend(added.iter().map(|(n, t)| (n.as_str(), t.as_str())));
    rows_of(&files)
}

pub fn rows() -> &'static [Row] {
    static ALL: std::sync::OnceLock<Vec<Row>> = std::sync::OnceLock::new();
    ALL.get_or_init(english_rows)
}

pub fn tag_ok(tag: &str) -> bool {
    !tag.is_empty()
        && !tag.eq_ignore_ascii_case("x")
        && tag.len() <= 16
        && tag.split('-').all(|p| !p.is_empty() && p.len() <= 8 && p.chars().all(|c| c.is_ascii_alphanumeric()))
        && tag.chars().next().is_some_and(|c| c.is_ascii_alphabetic())
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\n', "\\n").replace('\t', "\\t")
}

fn unescape(s: &str) -> Option<String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        out.push(match chars.next()? {
            '\\' => '\\',
            'n' => '\n',
            't' => '\t',
            _ => return None,
        });
    }
    Some(out)
}

pub fn check_value(tag: &str, rows: &[Row]) -> String {
    let mut h = Sha256::new();
    let mut field = |s: &str| {
        h.update((s.len() as u64).to_le_bytes());
        h.update(s.as_bytes());
    };
    field(DOMAIN);
    field(tag);
    field(&rows.len().to_string());
    for r in rows {
        field(&r.key);
        field(&r.english);
        field(&r.marks.join("\u{1}"));
    }
    h.finalize().iter().fold(String::new(), |mut s, b| { let _ = write!(s, "{b:02x}"); s })
}

use crate::common::expand::hidden;

fn joins(c: char) -> bool {
    matches!(c as u32, 0x600..=0x6ff | 0x750..=0x77f | 0x8a0..=0x8ff | 0x900..=0xdff | 0x1000..=0x109f | 0x1780..=0x17ff | 0xfb50..=0xfdff | 0xfe70..=0xfefc)
        && !hidden(c)
}

fn unfit(text: &str, allowed: &[char]) -> bool {
    let chars: Vec<char> = text.chars().collect();
    chars.iter().enumerate().any(|(i, &c)| {
        if allowed.contains(&c) {
            return false;
        }
        if matches!(c, '\u{200c}' | '\u{200d}') {
            let before = i.checked_sub(1).map(|j| chars[j]);
            let after = chars.get(i + 1).copied();
            let after_ok = if c == '\u{200c}' {
                after.is_some_and(joins)
            } else {
                after.is_none_or(|a| joins(a) || (!hidden(a) && !matches!(a, '\u{200c}' | '\u{200d}')))
            };
            return !(before.is_some_and(joins) && after_ok);
        }
        hidden(c)
    })
}

pub fn check_words(row: &Row, words: &str) -> Result<String, Why> {
    let words = words.trim();
    if words.is_empty() {
        return Ok(String::new());
    }
    let key = &row.key;

    if words.chars().count() > WORDS_MOST.min((row.english.chars().count() * WORDS_TIMES).max(WORDS_LEAST)) {
        return Err(Why::Long(key.clone()));
    }
    if unfit(words, &['\n', '\u{200e}', '\u{200f}']) || words.contains('\u{fffd}') {
        return Err(Why::Characters(key.clone()));
    }

    let (theirs, english) = (words.replace("->", ""), row.english.replace("->", ""));
    for bad in NOT_MORE {
        if theirs.matches(bad).count() > english.matches(bad).count() {
            return Err(Why::Links(key.clone()));
        }
    }
    if new_path(words, &row.english) {
        return Err(Why::Links(key.clone()));
    }
    match lang::placeholders(words) {
        Some(marks) if same_marks(&marks, &row.marks) => Ok(words.to_string()),
        _ => Err(Why::Placeholders(key.clone())),
    }
}

fn paths(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut run = String::new();
    for c in text.chars().chain(std::iter::once(' ')) {
        if c.is_ascii_alphanumeric() || "._-+/~@%=:".contains(c) {
            run.push(c);
            continue;
        }
        let path = run.trim_end_matches(['.', ',', ':', '/', '-']);
        if run.starts_with(['/', '~']) && path.len() > 1 {
            out.push(path.to_string());
        }
        run.clear();
    }
    out
}

fn new_path(words: &str, english: &str) -> bool {
    let known = paths(english);
    paths(words).iter().any(|p| !known.contains(p))
}

fn same_marks(theirs: &[String], english: &[String]) -> bool {
    let plain = |m: &[String]| m.iter().filter(|x| !x.starts_with('?')).cloned().collect::<Vec<_>>();
    let shown = |var: &str| english.iter().any(|e| e.strip_prefix('$').or_else(|| e.strip_prefix('?')) == Some(var));
    plain(theirs) == plain(english) && theirs.iter().filter_map(|x| x.strip_prefix('?')).all(shown)
}

pub fn export(tag: &str, rows: &[Row], words: &BTreeMap<String, String>) -> String {
    let mut out = String::new();
    out.push_str(KIND);
    out.push('\n');
    out.push_str(&format!("version: {VERSION}\nlanguage: {tag}\ncheck value: {}\nrows: {}\n", check_value(tag, rows), rows.len()));
    for n in NOTES {
        out.push_str(n);
        out.push('\n');
    }
    out.push('\n');
    for r in rows {
        let mine = words.get(&r.key).and_then(|w| check_words(r, w).ok()).unwrap_or_default();
        out.push_str(&format!("{}\t{}\t{}\n", r.key, escape(&r.english), escape(&mine)));
    }
    out
}

pub fn import(bytes: &[u8], rows: &[Row]) -> Result<Sheet, Why> {
    if bytes.len() as u64 > FILE_MOST {
        return Err(Why::TooBig);
    }
    let text = std::str::from_utf8(bytes).map_err(|_| Why::NotText)?;
    if unfit(text, &['\n', '\t', '\u{200e}', '\u{200f}']) {
        return Err(Why::NotPlain);
    }
    let all: Vec<&str> = text.split('\n').collect();
    if all.iter().any(|l| l.len() > LINE_MOST) {
        return Err(Why::LineLong);
    }
    let mut lines = all.into_iter();
    let tag = read_head(&mut lines, rows)?;
    let words = read_rows(&mut lines, rows)?;

    match (lines.next(), lines.next()) {
        (Some(""), None) | (None, _) => Ok(Sheet { lang: tag, words }),
        _ => Err(Why::Count),
    }
}

fn read_head(lines: &mut std::vec::IntoIter<&str>, rows: &[Row]) -> Result<String, Why> {
    if lines.next() != Some(KIND) {
        return Err(Why::NotMine);
    }
    if lines.next() != Some(&format!("version: {VERSION}")) {
        return Err(Why::Version);
    }
    let tag = lines.next().and_then(|l| l.strip_prefix("language: ")).filter(|t| tag_ok(t)).ok_or(Why::Language)?.to_string();
    let given = lines.next().and_then(|l| l.strip_prefix("check value: ")).ok_or(Why::CheckValue)?;
    if given != check_value(&tag, rows) {
        return Err(Why::CheckValue);
    }
    if lines.next() != Some(&format!("rows: {}", rows.len())) {
        return Err(Why::Count);
    }
    for n in NOTES {
        if lines.next() != Some(n) {
            return Err(Why::Notes);
        }
    }
    if lines.next() != Some("") {
        return Err(Why::Notes);
    }
    Ok(tag)
}

fn read_rows(lines: &mut std::vec::IntoIter<&str>, rows: &[Row]) -> Result<BTreeMap<String, String>, Why> {
    let mut words = BTreeMap::new();
    for r in rows {
        let line = lines.next().ok_or(Why::Count)?;
        let mut cells = line.split('\t');
        let (key, english, mine) = match (cells.next(), cells.next(), cells.next(), cells.next()) {
            (Some(k), Some(e), Some(m), None) => (k, e, m),
            _ => return Err(Why::Shape(r.key.clone())),
        };
        if key != r.key {
            return Err(Why::Key(r.key.clone()));
        }
        if english != escape(&r.english) {
            return Err(Why::English(r.key.clone()));
        }
        let mine = unescape(mine).ok_or_else(|| Why::Shape(r.key.clone()))?;
        let kept = check_words(r, &mine)?;
        if kept != mine {
            return Err(Why::Shape(r.key.clone()));
        }
        if !kept.is_empty() {
            words.insert(r.key.clone(), kept);
        }
    }
    Ok(words)
}

pub fn import_file(path: &Path, rows: &[Row]) -> Result<Sheet, Why> {
    use std::io::Read;
    use std::os::unix::fs::OpenOptionsExt;
    let file = std::fs::OpenOptions::new().read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK | libc::O_CLOEXEC).open(path).map_err(|_| Why::NotSheet)?;
    let meta = file.metadata().map_err(|_| Why::NotSheet)?;
    if !meta.is_file() {
        return Err(Why::NotSheet);
    }
    if meta.len() > FILE_MOST {
        return Err(Why::TooBig);
    }
    let mut bytes = Vec::new();
    file.take(FILE_MOST + 1).read_to_end(&mut bytes).map_err(|_| Why::NotSheet)?;
    import(&bytes, rows)
}

pub fn base() -> PathBuf {
    crate::common::files::config_home().join("trindle").join("lang")
}

pub fn as_ftl(words: &BTreeMap<String, String>) -> Vec<String> {
    let mut files = vec![String::new()];
    for (key, w) in words {
        let mut lines = w.lines();

        let first = lines.next().unwrap_or("");
        let mut one = if first.is_empty() { format!("{key} =\n") } else { format!("{key} = {first}\n") };
        for l in lines {
            one.push_str("    ");
            one.push_str(l);
            one.push('\n');
        }
        if files.last().is_some_and(|f| f.len() + one.len() > PART_MOST) {
            files.push(String::new());
        }
        if let Some(f) = files.last_mut() {
            f.push_str(&one);
        }
    }
    files
}

pub fn install(base: &Path, tag: &str, words: &BTreeMap<String, String>) -> std::io::Result<()> {
    if !tag_ok(tag) {
        return Err(std::io::Error::other("language tag"));
    }
    let dir = base.join(tag);
    std::fs::create_dir_all(&dir)?;
    let parts = if words.is_empty() { Vec::new() } else { as_ftl(words) };
    for (i, text) in parts.iter().enumerate() {
        crate::common::files::write_whole_private(&dir.join(format!("{PART}{i:03}.ftl")), text.as_bytes())?;
    }
    for (path, i) in kept_parts(&dir)? {
        if i >= parts.len() {
            std::fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn kept_parts(dir: &Path) -> std::io::Result<Vec<(PathBuf, usize)>> {
    let mut out = Vec::new();
    for e in std::fs::read_dir(dir)?.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if let Some(i) = name.strip_prefix(PART).and_then(|n| n.strip_suffix(".ftl")).and_then(|n| n.parse::<usize>().ok()) {
            out.push((e.path(), i));
        }
    }
    Ok(out)
}

fn back_up(base: &Path, tag: &str) -> std::io::Result<()> {
    let dir = base.join(tag);
    let Ok(read) = std::fs::read_dir(&dir) else { return Ok(()) };
    for e in read.flatten() {
        let name = e.file_name().to_string_lossy().into_owned();
        if name.starts_with(PART) && name.ends_with(".ftl.bak") {
            std::fs::remove_file(e.path())?;
        }
    }
    for (path, _) in kept_parts(&dir)? {
        let bak = PathBuf::from(format!("{}.bak", path.display()));
        crate::common::files::write_whole_private(&bak, &std::fs::read(&path)?)?;
    }
    Ok(())
}

pub fn take(path: &Path, rows: &[Row], base: &Path) -> Result<String, Why> {
    let got = import_file(path, rows)?;
    let shipped = shipped(&got.lang, rows, base);
    let words: BTreeMap<String, String> = got.words.into_iter().filter(|(k, w)| shipped.get(k).map(|s| s.trim()) != Some(w.trim()) && !WAY_BACK.contains(&k.as_str())).collect();
    if let Some(r) = rows.iter().find(|r| words.get(&r.key).is_some_and(|w| w.chars().count() > room(r.english.chars().count()))) {
        return Err(Why::Long(r.key.clone()));
    }
    back_up(base, &got.lang).map_err(|_| Why::NotSaved)?;
    install(base, &got.lang, &words).map_err(|_| Why::NotSaved)?;
    Ok(got.lang)
}

pub fn room(english: usize) -> usize {
    match english {
        0..=6 => english * 2 + 4,
        7..=12 => english * 2 + 6,
        _ => english * 3,
    }
}

pub fn reset(base: &Path, tag: &str) -> std::io::Result<()> {
    back_up(base, tag)?;
    install(base, tag, &BTreeMap::new())
}

pub fn shipped(tag: &str, rows: &[Row], base: &Path) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    if !tag_ok(tag) {
        return out;
    }
    if own::is_own(tag) {
        return rows.iter().map(|r| (r.key.clone(), r.english.clone())).collect();
    }
    for dir in lang::folders().iter().rev().filter(|d| d.as_path() != base) {
        for (_, text) in lang::folder_files(&dir.join(tag)) {
            out.extend(lang::raw_messages(&text));
        }
    }
    out
}

pub fn in_use(tag: &str, rows: &[Row], base: &Path) -> BTreeMap<String, String> {
    let mut out = shipped(tag, rows, base);
    out.extend(installed(base, tag));
    out
}

pub fn installed(base: &Path, tag: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    if !tag_ok(tag) {
        return out;
    }
    for (name, text) in lang::folder_files(&base.join(tag)) {
        if name.starts_with(PART) {
            out.extend(lang::raw_messages(&text));
        }
    }
    out
}

pub mod own;

#[cfg(test)]
mod tests;
