//! What Settings' Printers page and the Print portal's window share: reading
//! the printers and the queue from CUPS, the choices a print window offers,
//! how they become CUPS options, and sending a document as one job.

use std::io::Read;
use std::os::fd::OwnedFd;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use crate::toolkit::t;

pub struct Printer {
    pub name: String,
    pub state: String,
    pub disabled: bool,
}

pub fn parse_printers(out: &str) -> Vec<Printer> {
    let mut printers = Vec::new();
    for line in out.lines() {
        let Some(rest) = line.strip_prefix("printer ") else { continue };
        let Some((name, rest)) = rest.split_once(' ') else { continue };
        if name.is_empty() {
            continue;
        }
        let state = if let Some(after) = rest.strip_prefix("is ") {
            let word: String = after.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
            if word.is_empty() {
                continue;
            }
            word
        } else if rest.starts_with("now printing") {
            "printing".to_string()
        } else {
            continue;
        };
        printers.push(Printer { name: name.to_string(), state, disabled: line.contains("disabled") });
    }
    printers
}

pub fn parse_default(out: &str) -> Option<String> {
    let (_, after) = out.split_once("system default destination: ")?;
    let name = after.split_whitespace().next()?;
    Some(name.to_string())
}

#[derive(Debug, PartialEq)]
pub struct Queued {

    pub id: String,
    pub printer: String,
    pub user: String,
    pub bytes: u64,

    pub when: String,
}

pub fn parse_jobs(out: &str) -> Vec<Queued> {
    out.lines().filter_map(|line| {
        let mut parts = line.split_whitespace();
        let id = parts.next()?;
        let user = parts.next()?;
        let bytes = parts.next()?.parse().ok()?;
        let when = parts.collect::<Vec<_>>().join(" ");
        let (printer, number) = id.rsplit_once('-')?;
        if printer.is_empty() || number.is_empty() || !number.bytes().all(|b| b.is_ascii_digit()) || id.starts_with('-') {
            return None;
        }
        Some(Queued { id: id.into(), printer: printer.into(), user: user.into(), bytes, when })
    }).collect()
}

pub fn is_job_id(id: &str) -> bool {
    id.rsplit_once('-').is_some_and(|(printer, number)| {
        !printer.is_empty() && !id.starts_with('-') && !number.is_empty() && number.bytes().all(|b| b.is_ascii_digit())
            && id.bytes().all(|b| b.is_ascii_graphic())
    })
}

pub fn is_queue_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 127 && !name.starts_with('-')
        && name.bytes().all(|b| b.is_ascii_graphic() && b != b'/' && b != b'#')
}

pub const MOST_COPIES: u32 = 99;

const MOST_PAGE: u32 = 9999;
const MOST_PARTS: usize = 32;

pub fn bad_range() -> &'static str {
    t!("printers-bad-range")
}

pub fn bad_copies() -> &'static str {
    t!("printers-bad-copies")
}

pub fn parse_ranges(text: &str) -> Result<Vec<(u32, u32)>, &'static str> {
    let mut out = Vec::new();
    for part in text.split(',') {
        let part = part.trim();
        let (a, b) = part.split_once('-').map_or((part, part), |(a, b)| (a.trim(), b.trim()));
        let page = |s: &str| -> Result<u32, &'static str> {
            if s.is_empty() || s.len() > 5 || !s.bytes().all(|b| b.is_ascii_digit()) {
                return Err(bad_range());
            }
            s.parse::<u32>().ok().filter(|n| (1..=MOST_PAGE).contains(n)).ok_or(bad_range())
        };
        let (a, b) = (page(a)?, page(b)?);
        if a > b {
            return Err(bad_range());
        }
        out.push((a, b));
        if out.len() > MOST_PARTS {
            return Err(bad_range());
        }
    }
    Ok(out)
}

pub fn parse_copies(text: &str) -> Result<u32, &'static str> {
    let t = text.trim();
    if t.is_empty() || t.len() > 3 || !t.bytes().all(|b| b.is_ascii_digit()) {
        return Err(bad_copies());
    }
    t.parse::<u32>().ok().filter(|n| (1..=MOST_COPIES).contains(n)).ok_or(bad_copies())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sides {
    One,
    LongEdge,
    ShortEdge,
}

impl Sides {
    pub const ALL: [Sides; 3] = [Sides::One, Sides::LongEdge, Sides::ShortEdge];

    pub fn label(self) -> &'static str {
        match self {
            Sides::One => t!("printers-sides-one"),
            Sides::LongEdge => t!("printers-sides-long"),
            Sides::ShortEdge => t!("printers-sides-short"),
        }
    }

    fn cups(self) -> &'static str {
        match self {
            Sides::One => "one-sided",
            Sides::LongEdge => "two-sided-long-edge",
            Sides::ShortEdge => "two-sided-short-edge",
        }
    }

    fn gtk(self) -> &'static str {
        match self {
            Sides::One => "simplex",
            Sides::LongEdge => "horizontal",
            Sides::ShortEdge => "vertical",
        }
    }

    fn from_gtk(word: &str) -> Option<Sides> {
        Sides::ALL.into_iter().find(|s| s.gtk() == word)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Paper {
    A4,
    A3,
    A5,
    Letter,
    Legal,
}

impl Paper {
    pub const ALL: [Paper; 5] = [Paper::A4, Paper::A3, Paper::A5, Paper::Letter, Paper::Legal];

    pub fn label(self) -> &'static str {
        match self {
            Paper::A4 => t!("printers-paper-a4"),
            Paper::A3 => t!("printers-paper-a3"),
            Paper::A5 => t!("printers-paper-a5"),
            Paper::Letter => t!("printers-paper-letter"),
            Paper::Legal => t!("printers-paper-legal"),
        }
    }

    fn plain(self) -> &'static str {
        match self {
            Paper::A4 => "A4",
            Paper::A3 => "A3",
            Paper::A5 => "A5",
            Paper::Letter => "Letter",
            Paper::Legal => "Legal",
        }
    }

    pub fn mm(self) -> (f64, f64) {
        match self {
            Paper::A4 => (210.0, 297.0),
            Paper::A3 => (297.0, 420.0),
            Paper::A5 => (148.0, 210.0),
            Paper::Letter => (215.9, 279.4),
            Paper::Legal => (215.9, 355.6),
        }
    }

    pub fn gtk(self) -> &'static str {
        match self {
            Paper::A4 => "iso_a4",
            Paper::A3 => "iso_a3",
            Paper::A5 => "iso_a5",
            Paper::Letter => "na_letter",
            Paper::Legal => "na_legal",
        }
    }

    fn cups(self) -> &'static str {
        match self {
            Paper::A4 => "iso_a4_210x297mm",
            Paper::A3 => "iso_a3_297x420mm",
            Paper::A5 => "iso_a5_148x210mm",
            Paper::Letter => "na_letter_8.5x11in",
            Paper::Legal => "na_legal_8.5x14in",
        }
    }

    pub fn from_program(name: Option<&str>, size_mm: Option<(f64, f64)>) -> Option<Paper> {
        if let Some(name) = name {
            if let Some(p) = Paper::ALL.into_iter().find(|p| name == p.gtk() || name.eq_ignore_ascii_case(p.plain())) {
                return Some(p);
            }
        }
        let (w, h) = size_mm?;
        let (w, h) = if w <= h { (w, h) } else { (h, w) };
        Paper::ALL.into_iter().find(|p| {
            let (pw, ph) = p.mm();
            (pw - w).abs() < 2.0 && (ph - h).abs() < 2.0
        })
    }

    pub fn usual(language: &str) -> Paper {
        let place = language.split('.').next().unwrap_or("");
        match place.split_once('_').map(|(_, c)| c) {
            Some("US") | Some("CA") | Some("MX") | Some("PH") | Some("CL") | Some("CO") | Some("VE") => Paper::Letter,
            _ => Paper::A4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

impl Orientation {
    pub const ALL: [Orientation; 2] = [Orientation::Portrait, Orientation::Landscape];

    pub fn label(self) -> &'static str {
        match self {
            Orientation::Portrait => t!("printers-orientation-portrait"),
            Orientation::Landscape => t!("printers-orientation-landscape"),
        }
    }

    fn cups(self) -> &'static str {
        match self {
            Orientation::Portrait => "3",
            Orientation::Landscape => "4",
        }
    }

    pub fn gtk(self) -> &'static str {
        match self {
            Orientation::Portrait => "portrait",
            Orientation::Landscape => "landscape",
        }
    }

    fn from_gtk(word: &str) -> Option<Orientation> {
        match word {
            "portrait" | "reverse_portrait" => Some(Orientation::Portrait),
            "landscape" | "reverse_landscape" => Some(Orientation::Landscape),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Choices {
    pub printer: String,
    pub copies: u32,

    pub pages: Option<Vec<(u32, u32)>>,
    pub sides: Sides,
    pub colour: bool,
    pub paper: Paper,
    pub orientation: Orientation,
}

fn ranges_for_cups(ranges: &[(u32, u32)]) -> String {
    ranges.iter().map(|&(a, b)| if a == b { a.to_string() } else { format!("{a}-{b}") }).collect::<Vec<_>>().join(",")
}

fn ranges_for_program(ranges: &[(u32, u32)]) -> String {
    ranges.iter().map(|&(a, b)| if a == b { (a - 1).to_string() } else { format!("{}-{}", a - 1, b - 1) }).collect::<Vec<_>>().join(",")
}

impl Choices {

    pub fn lp_args(&self) -> Vec<String> {
        let mut a = vec!["-d".to_string(), self.printer.clone(), "-n".into(), self.copies.to_string()];
        if let Some(ranges) = &self.pages {
            a.extend(["-P".to_string(), ranges_for_cups(ranges)]);
        }
        for option in [
            format!("sides={}", self.sides.cups()),
            format!("print-color-mode={}", if self.colour { "color" } else { "monochrome" }),
            format!("media={}", self.paper.cups()),
            format!("orientation-requested={}", self.orientation.cups()),
        ] {
            a.extend(["-o".to_string(), option]);
        }
        a
    }

    pub fn for_program(&self) -> Vec<(&'static str, String)> {
        let mut s = vec![
            ("printer", self.printer.clone()),
            ("n-copies", self.copies.to_string()),
            ("duplex", self.sides.gtk().into()),
            ("use-color", self.colour.to_string()),
            ("paper-format", self.paper.gtk().into()),
            ("paper-width", format!("{:.1}", self.paper.mm().0)),
            ("paper-height", format!("{:.1}", self.paper.mm().1)),
            ("orientation", self.orientation.gtk().into()),
        ];
        match &self.pages {
            Some(r) => {
                s.push(("print-pages", "ranges".into()));
                s.push(("page-ranges", ranges_for_program(r)));
            }
            None => s.push(("print-pages", "all".into())),
        }
        s
    }

    pub fn from_program(base: Choices, get: impl Fn(&str) -> Option<String>) -> Choices {
        let mut c = base;
        if let Some(p) = get("printer").filter(|p| is_queue_name(p)) {
            c.printer = p;
        }
        if let Some(n) = get("n-copies").and_then(|n| parse_copies(&n).ok()) {
            c.copies = n;
        }
        if let Some(s) = get("duplex").and_then(|w| Sides::from_gtk(&w)) {
            c.sides = s;
        }
        if let Some(on) = get("use-color").and_then(|w| w.parse::<bool>().ok()) {
            c.colour = on;
        }
        let size = get("paper-width").zip(get("paper-height")).and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?)));
        if let Some(p) = Paper::from_program(get("paper-format").as_deref(), size) {
            c.paper = p;
        }
        if let Some(o) = get("orientation").and_then(|w| Orientation::from_gtk(&w)) {
            c.orientation = o;
        }
        c.pages = match get("print-pages").as_deref() {
            Some("ranges") => get("page-ranges").and_then(|r| {
                let counted: Result<Vec<(u32, u32)>, _> = parse_ranges(&r.split(',').map(|p| {

                    let (a, b) = p.split_once('-').unwrap_or((p, p));
                    let up = |s: &str| s.trim().parse::<u32>().map(|n| (n + 1).to_string()).unwrap_or_default();
                    format!("{}-{}", up(a), up(b))
                }).collect::<Vec<_>>().join(","));
                counted.ok()
            }),
            _ => None,
        };
        c
    }
}

const SEND_LIMIT: Duration = Duration::from_secs(15 * 60);

pub fn job_title(title: &str) -> String {
    let t: String = title.chars().filter(|c| !c.is_control()).take(100).collect();
    if t.trim().is_empty() { "Document".to_string() } else { t }
}

pub fn explain(err: &str) -> String {
    let lower = err.to_lowercase();
    if err.contains("Forbidden") || lower.contains("not authorized") || lower.contains("permission") {
        return t!("printers-not-allowed").to_string();
    }
    let e = err.trim().trim_start_matches("lp:").trim();
    if e.is_empty() { t!("printers-refused").to_string() } else { e.to_string() }
}

pub fn send(choices: &Choices, title: &str, document: OwnedFd) -> Result<String, String> {
    if !is_queue_name(&choices.printer) {
        return Err(t!("printers-bad-name").into());
    }
    let listed = Command::new("lpstat").args(["-p"]).stdin(Stdio::null()).output()
        .map_err(|_| t!("printers-not-installed").to_string())?;
    let known = parse_printers(&String::from_utf8_lossy(&listed.stdout));
    match known.iter().find(|p| p.name == choices.printer) {
        None => return Err(t!("printers-gone", name = &choices.printer)),
        Some(p) if p.disabled => return Err(t!("printers-paused", name = &choices.printer)),
        Some(_) => {}
    }
    let mut child = Command::new("lp")
        .args(choices.lp_args())
        .args(["-t", &job_title(title)])
        .stdin(Stdio::from(document))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| t!("printers-not-installed").to_string())?;
    let start = Instant::now();
    let status = loop {
        match child.try_wait() {
            Ok(Some(s)) => break s,
            Ok(None) if start.elapsed() < SEND_LIMIT => std::thread::sleep(Duration::from_millis(50)),
            _ => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(t!("printers-too-slow").into());
            }
        }
    };
    let mut out = String::new();
    let mut err = String::new();
    if let Some(o) = child.stdout.take() {
        let _ = o.take(4096).read_to_string(&mut out);
    }
    if let Some(e) = child.stderr.take() {
        let _ = e.take(4096).read_to_string(&mut err);
    }
    if !status.success() {
        return Err(explain(&err));
    }

    Ok(out.split_whitespace().skip_while(|w| *w != "is").nth(1).unwrap_or("").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn choices() -> Choices {
        Choices { printer: "Office".into(), copies: 2, pages: Some(vec![(1, 3), (5, 5)]), sides: Sides::LongEdge, colour: false,
                  paper: Paper::A4, orientation: Orientation::Landscape }
    }

    #[test]
    fn the_choices_become_cups_options() {
        assert_eq!(choices().lp_args(), [
            "-d", "Office", "-n", "2", "-P", "1-3,5",
            "-o", "sides=two-sided-long-edge", "-o", "print-color-mode=monochrome",
            "-o", "media=iso_a4_210x297mm", "-o", "orientation-requested=4",
        ]);
        let all = Choices { pages: None, sides: Sides::One, colour: true, paper: Paper::Letter, orientation: Orientation::Portrait, copies: 1, ..choices() };
        let a = all.lp_args();
        assert!(!a.contains(&"-P".to_string()), "every page asks for no range");
        assert!(a.contains(&"sides=one-sided".to_string()) && a.contains(&"print-color-mode=color".to_string()));
        assert!(a.contains(&"media=na_letter_8.5x11in".to_string()) && a.contains(&"orientation-requested=3".to_string()));
        let short = Choices { sides: Sides::ShortEdge, ..choices() }.lp_args();
        assert!(short.contains(&"sides=two-sided-short-edge".to_string()));
    }

    #[test]
    fn the_printer_is_named_once_and_only_as_the_destination() {
        let a = choices().lp_args();
        assert_eq!(a.iter().filter(|w| *w == "-d").count(), 1);
        assert_eq!(a[0], "-d");
        assert_eq!(a[1], "Office");
    }

    #[test]
    fn page_ranges_are_read_and_bad_ones_refused() {
        assert_eq!(parse_ranges("1-3, 5"), Ok(vec![(1, 3), (5, 5)]));
        assert_eq!(parse_ranges(" 7 "), Ok(vec![(7, 7)]));
        assert_eq!(parse_ranges("2 - 4,9"), Ok(vec![(2, 4), (9, 9)]));
        for bad in ["", " ", ",", "1,", ",1", "0", "0-2", "3-1", "a", "1-a", "-3", "1-", "1--2", "1-2-3", "+1", "1.5", "10000",
                    "99999999999999999999", "1;2", "1 2", "-d evil", "\u{0661}"] {
            assert_eq!(parse_ranges(bad), Err(bad_range()), "{bad:?}");
        }
        let many = (1..=40).map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        assert_eq!(parse_ranges(&many), Err(bad_range()));
    }

    #[test]
    fn copies_are_one_to_ninety_nine() {
        assert_eq!(parse_copies("1"), Ok(1));
        assert_eq!(parse_copies(" 99 "), Ok(99));
        for bad in ["", "0", "100", "-1", "1.5", "x", "0001", "+3"] {
            assert_eq!(parse_copies(bad), Err(bad_copies()), "{bad:?}");
        }
    }

    #[test]
    fn what_a_program_sent_comes_back_and_what_is_wrong_is_left() {
        let told = choices().for_program();
        let get = |k: &str| told.iter().find(|(n, _)| *n == k).map(|(_, v)| v.clone());
        let base = Choices { printer: "Other".into(), copies: 1, pages: None, sides: Sides::One, colour: true, paper: Paper::Legal,
                             orientation: Orientation::Portrait };
        assert_eq!(Choices::from_program(base.clone(), get), choices());
        let nonsense = |k: &str| Some(match k { "n-copies" => "500", "duplex" => "sideways", "printer" => "-evil", "print-pages" => "ranges",
                                                "page-ranges" => "3-1", _ => "?" }.to_string());
        assert_eq!(Choices::from_program(base.clone(), nonsense), base);
        assert_eq!(get("page-ranges").as_deref(), Some("0-2,4"), "programs count from nought");
        assert_eq!(get("duplex").as_deref(), Some("horizontal"));
    }

    #[test]
    fn paper_is_known_by_name_or_size() {
        assert_eq!(Paper::from_program(Some("iso_a4"), None), Some(Paper::A4));
        assert_eq!(Paper::from_program(None, Some((279.4, 215.9))), Some(Paper::Letter));
        assert_eq!(Paper::from_program(None, Some((100.0, 100.0))), None);
        assert_eq!(Paper::usual("en_US.UTF-8"), Paper::Letter);
        assert_eq!(Paper::usual("en_GB.UTF-8"), Paper::A4);
        assert_eq!(Paper::usual("C"), Paper::A4);
    }

    #[test]
    fn the_queue_is_read() {
        let out = "Office-12   john   1024   Sat 18 Oct 2026 10:00:00 AM BST\nnonsense\n-x-3 a 1 b\nUp-stairs-7 jane 20 Sun 19 Oct\nOffice-x john 5 now\n";
        let jobs = parse_jobs(out);
        assert_eq!(jobs.len(), 2);
        assert_eq!(jobs[0], Queued { id: "Office-12".into(), printer: "Office".into(), user: "john".into(), bytes: 1024,
                                     when: "Sat 18 Oct 2026 10:00:00 AM BST".into() });
        assert_eq!(jobs[1].printer, "Up-stairs");
        assert!(is_job_id("Office-12") && !is_job_id("-12") && !is_job_id("--help-1") && !is_job_id("Office-x") && !is_job_id("a b-1"));
    }

    #[test]
    fn printers_and_the_default_are_read() {
        let out = "printer Office is idle.  enabled since Sat\nprinter Old disabled since Sat -\nprinter Busy now printing Busy-4.  enabled since x\nsystem default destination: Office\n";
        let p = parse_printers(out);
        assert_eq!(p.iter().map(|p| p.name.as_str()).collect::<Vec<_>>(), ["Office", "Busy"]);
        assert_eq!(p[1].state, "printing");
        assert_eq!(parse_default(out).as_deref(), Some("Office"));
        assert_eq!(parse_default("no system default destination"), None);
    }

    #[test]
    fn queue_names_and_titles_are_made_safe() {
        assert!(is_queue_name("Office_1") && !is_queue_name("-x") && !is_queue_name("a b") && !is_queue_name("") && !is_queue_name("a/b"));
        assert_eq!(job_title("a\nb\u{7}c"), "abc");
        assert_eq!(job_title("  "), "Document");
        assert_eq!(job_title(&"x".repeat(300)).len(), 100);
    }
}
