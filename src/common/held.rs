//! Updates held back.

use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Path, PathBuf};

const NAME: &str = "held-updates";
const HEADER: &str = "# Updates held back in Apps, one to a line. Release them in Apps, on its Updates page.";
const MOST_BYTES: u64 = 256 * 1024;
const MOST: usize = 2000;
const LONGEST: usize = 255;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    App,
    Part,
    Firmware,
    Plugin,
}

impl Kind {
    fn word(self) -> &'static str {
        match self {
            Kind::App => "app",
            Kind::Part => "part",
            Kind::Firmware => "firmware",
            Kind::Plugin => "plugin",
        }
    }

    fn from_word(word: &str) -> Option<Kind> {
        [Kind::App, Kind::Part, Kind::Firmware, Kind::Plugin].into_iter().find(|k| k.word() == word)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Thing {
    kind: Kind,
    id: String,
    branch: String,
}

fn fits(word: &str) -> bool {
    !word.is_empty() && word.len() <= LONGEST && !word.starts_with(['-', '.'])
        && word.bytes().all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
}

impl Thing {
    fn new(kind: Kind, id: &str, branch: &str) -> Option<Thing> {
        let branch_fits = if kind == Kind::Part { fits(branch) } else { branch.is_empty() };
        (fits(id) && branch_fits).then(|| Thing { kind, id: id.to_string(), branch: branch.to_string() })
    }

    pub fn app(id: &str) -> Option<Thing> {
        Thing::new(Kind::App, id, "")
    }

    pub fn part(id: &str, branch: &str) -> Option<Thing> {
        Thing::new(Kind::Part, id, branch)
    }

    pub fn firmware(id: &str) -> Option<Thing> {
        Thing::new(Kind::Firmware, id, "")
    }

    pub fn plugin(id: &str) -> Option<Thing> {
        Thing::new(Kind::Plugin, id, "")
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn branch(&self) -> &str {
        &self.branch
    }

    fn line(&self) -> String {
        if self.branch.is_empty() { format!("{} {}", self.kind.word(), self.id) } else { format!("{} {} {}", self.kind.word(), self.id, self.branch) }
    }

    fn from_line(line: &str) -> Option<Thing> {
        let mut words = line.split_whitespace();
        let kind = Kind::from_word(words.next()?)?;
        let id = words.next()?;
        let branch = if kind == Kind::Part { words.next()? } else { "" };
        if words.next().is_some() {
            return None;
        }
        Thing::new(kind, id, branch)
    }

    pub fn mask(&self) -> Option<String> {
        match self.kind {
            Kind::App => Some(self.id.clone()),
            Kind::Part => Some(format!("{}//{}", self.id, self.branch)),
            Kind::Firmware | Kind::Plugin => None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Held {
    things: BTreeSet<Thing>,
}

impl Held {
    #[cfg(test)]
    pub fn of(things: &[Option<Thing>]) -> Held {
        Held { things: things.iter().flatten().cloned().collect() }
    }

    pub fn read() -> Held {
        Held::read_at(&place())
    }

    fn read_at(path: &Path) -> Held {
        let plain = std::fs::metadata(path).is_ok_and(|m| m.is_file() && m.len() <= MOST_BYTES);
        if !plain {
            return Held::default();
        }
        let bytes = std::fs::read(path).unwrap_or_default();
        Held::from_text(&String::from_utf8_lossy(&bytes))
    }

    fn from_text(text: &str) -> Held {
        Held { things: text.lines().filter_map(Thing::from_line).take(MOST).collect() }
    }

    fn text(&self) -> String {
        let mut text = format!("{HEADER}\n");
        for t in &self.things {
            text.push_str(&t.line());
            text.push('\n');
        }
        text
    }

    pub fn has(&self, thing: &Thing) -> bool {
        self.things.contains(thing)
    }

    pub fn app(&self, id: &str) -> bool {
        Thing::app(id).is_some_and(|t| self.has(&t))
    }

    pub fn part(&self, id: &str, branch: &str) -> bool {
        Thing::part(id, branch).is_some_and(|t| self.has(&t))
    }

    pub fn firmware(&self, id: &str) -> bool {
        Thing::firmware(id).is_some_and(|t| self.has(&t))
    }

    pub fn plugin(&self, id: &str) -> bool {
        Thing::plugin(id).is_some_and(|t| self.has(&t))
    }

    pub fn is_empty(&self) -> bool {
        self.things.is_empty()
    }

    pub fn len(&self) -> usize {
        self.things.len()
    }

    pub fn things(&self) -> impl Iterator<Item = &Thing> {
        self.things.iter()
    }

    pub fn any_flatpak(&self) -> bool {
        self.things.iter().any(|t| t.mask().is_some())
    }

    pub fn masks(&self) -> Vec<String> {
        self.things.iter().filter_map(Thing::mask).collect()
    }

    pub fn hold(&mut self, thing: Thing) -> std::io::Result<()> {
        self.change(|things| {
            if things.len() < MOST {
                things.insert(thing);
            }
        })
    }

    pub fn release(&mut self, thing: &Thing) -> std::io::Result<()> {
        self.change(|things| {
            things.remove(thing);
        })
    }

    fn change(&mut self, with: impl FnOnce(&mut BTreeSet<Thing>)) -> std::io::Result<()> {
        let path = place();
        let mut now = Held::read_at(&path);
        with(&mut now.things);
        write_whole(&path, &now.text())?;
        *self = now;
        Ok(())
    }
}

fn write_whole(path: &Path, text: &str) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension(format!("{}-tmp", std::process::id()));
    let written = (|| {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(text.as_bytes())?;
        f.sync_all()?;
        std::fs::rename(&tmp, path)
    })();
    if written.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    written
}

#[cfg(not(test))]
fn place() -> PathBuf {
    super::files::config_path(NAME)
}

#[cfg(test)]
thread_local! {
    static PLACE: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
}

#[cfg(test)]
fn place() -> PathBuf {
    PLACE.with(|p| p.borrow().clone()).unwrap_or_else(|| {
        let thread: String = format!("{:?}", std::thread::current().id()).chars().filter(char::is_ascii_digit).collect();
        std::env::temp_dir().join(format!("trindle-{NAME}-test-{}-{thread}", std::process::id()))
    })
}

#[cfg(test)]
pub fn pretend(path: Option<PathBuf>) {
    PLACE.with(|p| *p.borrow_mut() = path);
    let _ = std::fs::remove_file(place());
}

pub fn changed_at() -> Option<std::time::SystemTime> {
    std::fs::metadata(place()).and_then(|m| m.modified()).ok()
}

pub fn flat_ref(line: &str) -> Option<(bool, &str, &str, &str)> {
    let mut name = line.split('\t').next()?.trim().split('/');
    let (kind, id, arch, branch, None) = (name.next()?, name.next()?, name.next()?, name.next()?, name.next()) else { return None };
    if !(kind == "app" || kind == "runtime") || !fits(id) || !fits(arch) || !fits(branch) {
        return None;
    }
    Some((kind == "app", id, arch, branch))
}

pub fn flatpaks_to_update(out: &str, held: &Held) -> Vec<String> {
    out.lines().filter_map(flat_ref)
        .filter(|(app, id, _, branch)| if *app { !held.app(id) } else { !held.part(id, branch) })
        .map(|(_, id, arch, branch)| format!("{id}/{arch}/{branch}"))
        .collect()
}

#[cfg(test)]
pub fn temp(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-held-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.join(NAME)
}

#[cfg(test)]
pub fn names_for_update(out: &str, held: &Held) -> Option<Vec<String>> {
    if !held.any_flatpak() {
        return Some(Vec::new());
    }
    Some(flatpaks_to_update(out, held)).filter(|names| !names.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_thing_is_held_kept_across_a_restart_and_released() {
        let path = temp("kept");
        pretend(Some(path.clone()));
        let mut held = Held::read();
        assert!(held.is_empty() && !path.exists(), "nothing is held, and reading makes no file");
        let notes = Thing::app("org.example.Notes").unwrap();
        let part = Thing::part("org.example.Platform", "25.08").unwrap();
        held.hold(notes.clone()).unwrap();
        held.hold(part.clone()).unwrap();
        held.hold(Thing::firmware("0a1b2c").unwrap()).unwrap();
        held.hold(Thing::plugin("org.example.Weather").unwrap()).unwrap();
        let again = Held::read();
        assert_eq!(again, held);
        assert!(again.app("org.example.Notes") && again.part("org.example.Platform", "25.08"));
        assert!(again.firmware("0a1b2c") && again.plugin("org.example.Weather"));
        assert!(!again.app("org.example.Paint") && !again.part("org.example.Platform", "26.08"), "only what was held, and only that branch");
        assert!(!again.plugin("org.example.Notes"), "an app held is not a plugin held");
        assert_eq!(again.masks(), ["org.example.Notes", "org.example.Platform//25.08"], "what Flatpak is told: the Flatpaks alone");
        let listed: Vec<(Kind, &str, &str)> = again.things().map(|t| (t.kind(), t.id(), t.branch())).collect();
        assert_eq!(listed, [(Kind::App, "org.example.Notes", ""), (Kind::Part, "org.example.Platform", "25.08"), (Kind::Firmware, "0a1b2c", ""),
                            (Kind::Plugin, "org.example.Weather", "")], "listed for the card of what is held, each kind in turn");
        assert_eq!(again.len(), 4);
        let text = std::fs::read_to_string(&path).unwrap();
        assert_eq!(text, format!("{HEADER}\napp org.example.Notes\npart org.example.Platform 25.08\nfirmware 0a1b2c\nplugin org.example.Weather\n"));
        held.release(&notes).unwrap();
        assert!(!Held::read().app("org.example.Notes") && Held::read().part("org.example.Platform", "25.08"));
        held.hold(part.clone()).unwrap();
        held.release(&notes).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap().lines().count(), 4);
        let beside: Vec<_> = std::fs::read_dir(path.parent().unwrap()).unwrap().flatten().map(|e| e.file_name()).collect();
        assert_eq!(beside.len(), 1, "{beside:?}");
        pretend(None);
    }

    #[test]
    fn a_change_keeps_what_another_window_held_meanwhile() {
        let path = temp("two");
        pretend(Some(path));
        let mut one = Held::read();
        let mut two = Held::read();
        one.hold(Thing::app("org.example.Notes").unwrap()).unwrap();
        two.hold(Thing::app("org.example.Paint").unwrap()).unwrap();
        assert!(two.app("org.example.Notes") && two.app("org.example.Paint"), "the second took the first's change with its own");
        assert_eq!(Held::read(), two);
        pretend(None);
    }

    #[test]
    fn a_bad_file_is_read_for_what_is_good_in_it_and_never_stops_anything() {
        let path = temp("bad");
        pretend(Some(path.clone()));
        std::fs::write(&path, b"\xff\xfe not text at all\napp org.example.Notes\n\napp\npart org.example.Platform\nrubbish org.example.X\n\
                                app org.example.Two words\n  plugin   org.example.Weather  \n\0\0\0\n").unwrap();
        let held = Held::read();
        assert!(held.app("org.example.Notes") && held.plugin("org.example.Weather"), "the good lines are kept");
        assert_eq!(held.things.len(), 2, "{held:?}");
        std::fs::remove_file(&path).unwrap();
        std::fs::create_dir(&path).unwrap();
        assert!(Held::read().is_empty());
        std::fs::remove_dir(&path).unwrap();
        std::fs::write(&path, "app org.example.Notes\n".repeat(20_000)).unwrap();
        assert!(Held::read().is_empty(), "a file too long to be a list is left unread");
        let mut held = Held::read();
        held.hold(Thing::app("org.example.Paint").unwrap()).unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), format!("{HEADER}\napp org.example.Paint\n"));
        pretend(None);
    }

    #[test]
    fn a_hostile_name_is_refused() {
        for id in ["", "*", "org.example.*", "--system", "-y", "../../etc/passwd", "a/b", "a b", "a\nb", "a\tb", "org.example.App;rm", "é", ".hidden",
                   "app/org.example.App/x86_64/stable", &"a".repeat(300)] {
            assert_eq!(Thing::app(id), None, "{id:?}");
            assert_eq!(Thing::part(id, "25.08"), None, "{id:?}");
            assert_eq!(Thing::part("org.example.Platform", id), None, "{id:?} as a branch");
            assert_eq!(Thing::firmware(id), None, "{id:?}");
            assert_eq!(Thing::plugin(id), None, "{id:?}");
            assert!(!Held::default().app(id));
        }
        let held = Held::from_text("app *\napp org.example.*\npart org.example.Platform */*\napp --remove\npart --user 1\napp org.example.Notes extra\n");
        assert!(held.is_empty() && held.masks().is_empty(), "{held:?}");
        assert!(Thing::app("org.example.Notes_2-beta").is_some());
    }

    #[test]
    fn installing_everything_leaves_out_what_is_held() {
        const WAITING: &str = "app/org.example.Notes/x86_64/stable\tflathub\napp/org.example.Paint/x86_64/stable\tflathub\n\
                               runtime/org.example.Platform/x86_64/25.08\tflathub\nruntime/org.example.Platform/x86_64/26.08\tflathub\n\
                               app/--system/x86_64/stable\nnot a ref\n";
        let all = ["org.example.Notes/x86_64/stable", "org.example.Paint/x86_64/stable", "org.example.Platform/x86_64/25.08", "org.example.Platform/x86_64/26.08"];
        let mut held = Held::default();
        assert_eq!(flatpaks_to_update(WAITING, &held), all, "a line that is no ref, or reads as an option, is left");
        assert_eq!(names_for_update(WAITING, &held), Some(Vec::new()), "nothing held: everything, as ever, with no name given");
        held.things.insert(Thing::firmware("abc").unwrap());
        held.things.insert(Thing::plugin("org.example.Notes").unwrap());
        assert_eq!(names_for_update(WAITING, &held), Some(Vec::new()));
        held.things.insert(Thing::app("org.example.Notes").unwrap());
        held.things.insert(Thing::part("org.example.Platform", "25.08").unwrap());
        assert_eq!(flatpaks_to_update(WAITING, &held), [all[1], all[3]]);
        assert_eq!(names_for_update(WAITING, &held).unwrap(), [all[1], all[3]], "each by name, the held ones not among them");
        held.things.insert(Thing::app("org.example.Paint").unwrap());
        held.things.insert(Thing::part("org.example.Platform", "26.08").unwrap());
        assert_eq!(names_for_update(WAITING, &held), None);
        assert_eq!(names_for_update("", &held), None, "and none is named when the list could not be read");
    }
}
