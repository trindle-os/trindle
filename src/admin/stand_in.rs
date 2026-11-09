//! For tests only: a folder that stands for "/" and a function that stands
//! for every command, both kept for the one thread a test runs on. A step
//! of the helper can then be run whole, and any part of it made to fail,
//! with nothing on the real computer touched and nothing run as root.

use std::cell::RefCell;
use std::path::{Path, PathBuf};

use super::Outcome;

type Runner = Box<dyn FnMut(&[&str]) -> Outcome>;

thread_local! {
    static ROOT: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
    static RUNNER: RefCell<Option<Runner>> = const { RefCell::new(None) };
    static READS: RefCell<Vec<(String, String)>> = const { RefCell::new(Vec::new()) };
    static RAN: RefCell<Vec<Vec<String>>> = const { RefCell::new(Vec::new()) };
}

const HARMLESS: [&str; 3] = ["true", "false", "sh"];

pub struct StandIn(PathBuf);

impl StandIn {

    pub fn new(tag: &str, runner: impl FnMut(&[&str]) -> Outcome + 'static) -> StandIn {
        let test = std::thread::current().name().unwrap_or_default().replace("::", "-");
        let root = std::env::temp_dir().join(format!("trindle-admin-test-{tag}-{test}-{}", std::process::id()));
        // Left from a run that was stopped: this test's own, by its name.
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("a folder for the test");
        ROOT.with(|r| *r.borrow_mut() = Some(root.clone()));
        RUNNER.with(|r| *r.borrow_mut() = Some(Box::new(runner)));
        RAN.with(|r| r.borrow_mut().clear());
        StandIn(root)
    }

    pub fn all_well(tag: &str) -> StandIn {
        StandIn::new(tag, |_| Ok(()))
    }

    pub fn failing(tag: &str, failing: &'static str, said: &'static str) -> StandIn {
        StandIn::new(tag, move |argv| if argv.join(" ").starts_with(failing) { Err(said.into()) } else { Ok(()) })
    }

    pub fn path(&self, path: &str) -> PathBuf {
        self.0.join(path.trim_start_matches('/'))
    }

    pub fn write(&self, path: &str, bytes: &[u8]) {
        let at = self.path(path);
        std::fs::create_dir_all(at.parent().expect("a file has a folder")).expect("the folders above the file");
        std::fs::write(at, bytes).expect("the file");
    }

    pub fn stuck(&self, path: &str) {
        let at = self.path(path);
        std::fs::create_dir_all(at.join("held")).expect("the folder in the file's place");
    }

    pub fn prints(&self, start: &str, text: &str) {
        READS.with(|r| r.borrow_mut().push((start.to_string(), text.to_string())));
    }

    pub fn ran(&self) -> Vec<String> {
        RAN.with(|r| r.borrow().iter().map(|a| a.join(" ")).collect())
    }
}

impl Drop for StandIn {
    fn drop(&mut self) {
        ROOT.with(|r| *r.borrow_mut() = None);
        RUNNER.with(|r| *r.borrow_mut() = None);
        READS.with(|r| r.borrow_mut().clear());
        // The test's own folder, made by `new`.
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

pub fn at(path: &Path) -> PathBuf {
    let root = ROOT.with(|r| r.borrow().clone())
        .unwrap_or_else(|| panic!("a test reached {} on the real computer: it needs a StandIn", path.display()));
    root.join(path.strip_prefix("/").unwrap_or(path))
}

pub fn run(argv: &[&str]) -> Option<Outcome> {
    let answer = RUNNER.with(|r| r.borrow_mut().as_mut().map(|f| f(argv)));
    if answer.is_some() {
        RAN.with(|r| r.borrow_mut().push(argv.iter().map(|a| a.to_string()).collect()));
        return answer;
    }
    assert!(HARMLESS.contains(&argv[0]), "a test tried to run {argv:?} for real: it needs a StandIn");
    None
}

pub fn said(argv: &[&str]) -> Option<Result<String, String>> {
    let key = argv.join(" ");
    READS.with(|r| r.borrow().iter().find(|(start, _)| key.starts_with(start.as_str())).map(|(_, text)| Ok(text.clone())))
        .or_else(|| ROOT.with(|r| r.borrow().is_some()).then(|| Err(format!("{} said nothing in this test", argv[0]))))
}
