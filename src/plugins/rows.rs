//! A plugin's settings rows.

use std::collections::HashSet;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use serde::Deserialize;
use trindle_toolkit::lang::{Arg, Bundle, Catalogue};

const FILE_MOST: u64 = 64 * 1024;
const LINE_MOST: usize = 64;
const NAME_MOST: usize = 32;
const SAYS_MOST: usize = 12;
const ROWS_MOST: usize = 24;
const OPTIONS_MOST: usize = 16;
const ARGS_MOST: usize = 4;
const FILES_MOST: usize = 16;
const WAIT: Duration = Duration::from_secs(2);
const RUN_WAIT: Duration = Duration::from_secs(120);

#[derive(Clone, Debug)]
pub struct Roots {
    pub share: PathBuf,
    pub run: PathBuf,
    pub lib: PathBuf,
    pub pkexec: PathBuf,
}

pub fn roots() -> Roots {
    #[cfg(test)]
    return tests::HERE.with(|h| h.borrow().clone()).unwrap_or_else(|| Roots {
        share: "/nonexistent/share".into(), run: "/nonexistent/run".into(), lib: "/nonexistent/lib".into(), pkexec: "/nonexistent/pkexec".into(),
    });
    #[cfg(not(test))]
    Roots { share: "/usr/share/trindle".into(), run: "/run".into(), lib: "/usr/lib".into(), pkexec: "pkexec".into() }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Test {
    Always,
    Is(String),
    Not(String),
}

#[derive(Clone, Debug)]
pub struct Say {
    pub test: Test,
    pub text: String,
    pub name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Run {
    program: PathBuf,
    args: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Opt {
    pub value: String,
    pub text: String,
}

#[derive(Clone, Debug)]
pub enum Kind {
    Switch { field: String, on: String, off: String },
    Choice { field: String, template: String, options: Vec<Opt>, run: Option<Run> },
    Send { line: String },
    Launch { run: Run },
}

#[derive(Clone, Debug)]
pub struct Row {
    pub label: String,
    pub sub: Option<String>,
    pub show: Test,
    pub kind: Kind,
}

#[derive(Clone, Debug)]
pub struct Rows {
    pub id: String,
    pub name: String,
    pub says: Vec<Say>,
    pub rows: Vec<Row>,
    words: String,
    status: String,
    socket: PathBuf,
    dir: String,
    roots: Roots,
}

pub type Status = serde_json::Map<String, serde_json::Value>;

#[derive(Clone, Copy, Debug)]
pub enum Act {
    Switch(bool),
    Pick(usize),
    Press,
}

#[derive(Debug, PartialEq)]
pub enum Done {
    Kept,
    Refused,
    Silent,
    Cancelled,
    Failed(Option<String>),
}

#[derive(Deserialize)]
struct File {
    name: String,
    words: String,
    socket: String,
    status: String,
    #[serde(default)]
    say: Vec<RawSay>,
    #[serde(default)]
    row: Vec<RawRow>,
}

#[derive(Deserialize)]
struct RawSay {
    test: Option<String>,
    text: String,
    name: Option<String>,
}

#[derive(Deserialize)]
struct RawRow {
    kind: String,
    label: String,
    sub: Option<String>,
    show: Option<String>,
    field: Option<String>,
    send_on: Option<String>,
    send_off: Option<String>,
    send: Option<String>,
    run: Option<String>,
    #[serde(default)]
    options: Vec<RawOpt>,
}

#[derive(Deserialize)]
struct RawOpt {
    value: serde_json::Value,
    text: String,
}

fn token(s: &str, most: usize) -> bool {
    !s.is_empty() && s.len() <= most && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
}

fn id_ok(s: &str) -> bool {
    !s.starts_with('-') && s.len() <= NAME_MOST && !s.is_empty() && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

fn key_ok(s: &str) -> bool {
    !s.is_empty() && s.len() <= 64 && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

fn field_ok(s: &str) -> bool {
    !s.is_empty() && s.len() <= 32 && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

fn line_ok(s: &str) -> bool {
    !s.is_empty() && s.len() <= LINE_MOST && s.bytes().all(|b| (0x20..0x7f).contains(&b)) && s.trim() == s
}

fn tidy(s: &str, most: usize) -> String {
    s.chars().filter(|c| !c.is_control()).take(most).collect::<String>().trim().to_string()
}

fn test_of(s: Option<&String>) -> Option<Test> {
    match s.map(String::as_str) {
        None => Some(Test::Always),
        Some(t) => match t.strip_prefix('!') {
            Some(f) => field_ok(f).then(|| Test::Not(f.to_string())),
            None => field_ok(t).then(|| Test::Is(t.to_string())),
        },
    }
}

fn value_of(v: &serde_json::Value) -> Option<String> {
    match v {
        serde_json::Value::Number(n) => n.as_u64().map(|n| n.to_string()),
        serde_json::Value::String(s) => token(s, 32).then(|| s.clone()),
        _ => None,
    }
}

fn run_of(text: &str, dir: &str, roots: &Roots, fills: bool) -> Option<Run> {
    let mut parts = text.split(' ');
    let rest = Path::new(parts.next()?).strip_prefix("/usr/lib").ok()?;
    let mut inside = rest.components();
    if inside.next() != Some(Component::Normal(dir.as_ref())) {
        return None;
    }
    let tail: Vec<Component> = inside.collect();
    if tail.is_empty() || !tail.iter().all(|c| matches!(c, Component::Normal(_))) {
        return None;
    }
    let args: Vec<String> = parts.map(String::from).collect();
    let holes = args.iter().filter(|a| *a == "{}").count();
    let plain = args.iter().all(|a| a == "{}" || token(a, 32));
    (args.len() <= ARGS_MOST && plain && holes <= 1 && (fills || holes == 0)).then(|| Run { program: roots.lib.join(rest), args })
}

fn socket_of(text: &str, dir: &str, roots: &Roots) -> Option<PathBuf> {
    let rest = Path::new(text).strip_prefix("/run").ok()?;
    let mut inside = rest.components();
    if inside.next() != Some(Component::Normal(dir.as_ref())) {
        return None;
    }
    let tail: Vec<Component> = inside.collect();
    (!tail.is_empty() && tail.iter().all(|c| matches!(c, Component::Normal(_)))).then(|| roots.run.join(rest))
}

fn inside(base: &Path, dir: &str, target: &Path) -> Option<PathBuf> {
    let own = base.canonicalize().ok()?.join(dir);
    if own.canonicalize().ok()? != own {
        return None;
    }
    let real = target.canonicalize().ok()?;
    (real.starts_with(&own) && real != own).then_some(real)
}

fn read_capped(path: &Path) -> Option<String> {
    let file = crate::common::files::open_file(path).ok()?;
    let mut text = String::new();
    file.take(FILE_MOST + 1).read_to_string(&mut text).ok()?;
    (text.len() as u64 <= FILE_MOST).then_some(text)
}

pub fn load_all() -> Vec<Rows> {
    load_from(&roots())
}

pub fn load_from(roots: &Roots) -> Vec<Rows> {
    let Ok(read) = std::fs::read_dir(roots.share.join("plugins")) else { return Vec::new() };
    let mut paths: Vec<PathBuf> = read.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|x| x == "toml")).collect();
    paths.sort();
    paths.into_iter().take(FILES_MOST)
        .filter_map(|p| Rows::parse(p.file_stem()?.to_str()?, &read_capped(&p)?, roots)).collect()
}

impl Rows {
    pub fn parse(id: &str, text: &str, roots: &Roots) -> Option<Rows> {
        if !id_ok(id) || text.len() as u64 > FILE_MOST {
            return None;
        }
        let f: File = toml::from_str(text).ok()?;
        let dir = format!("trindle-{id}");
        let name = tidy(&f.name, NAME_MOST);
        if name.is_empty() || !id_ok(&f.words) || !field_ok(&f.status) || f.say.len() > SAYS_MOST || f.row.len() > ROWS_MOST {
            return None;
        }
        let socket = socket_of(&f.socket, &dir, roots)?;
        let says = f.say.iter().map(|s| {
            let name = match &s.name {
                Some(n) => Some(field_ok(n).then(|| n.clone())?),
                None => None,
            };
            Some(Say { test: test_of(s.test.as_ref())?, text: key_ok(&s.text).then(|| s.text.clone())?, name })
        }).collect::<Option<Vec<_>>>()?;
        let rows = f.row.iter().map(|r| row_of(r, &dir, roots)).collect::<Option<Vec<_>>>()?;
        Some(Rows { id: id.to_string(), name, says, rows, words: f.words, status: f.status, socket, dir, roots: roots.clone() })
    }

    fn socket(&self) -> Option<PathBuf> {
        inside(&self.roots.run, &self.dir, &self.socket)
    }

    fn ask(&self, line: &str) -> Option<String> {
        ask_at(&self.socket()?, line)
    }

    pub fn status(&self) -> Option<Status> {
        match serde_json::from_str(&self.ask(&format!("{}\n", self.status))?).ok()? {
            serde_json::Value::Object(m) => Some(m),
            _ => None,
        }
    }

    pub fn words(&self, tags: &[String]) -> Words {
        let own = |tag: &str| {
            let ok = tag.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-');
            let text = ok.then(|| read_capped(&self.roots.share.join("lang").join(tag).join(format!("{}.ftl", self.words)))).flatten();
            Bundle::from_text(tag, text.as_deref().unwrap_or(""))
        };
        let mut chosen = own(tags.first().map_or("en", String::as_str));
        for tag in tags.iter().skip(1) {
            chosen.fill_from(own(tag));
        }
        let english = own("en");
        let known = chosen.keys().chain(english.keys()).map(String::from).collect();
        Words { cat: Catalogue::new(chosen, english), known }
    }

    pub fn sentence(&self, status: &Status, words: &Words) -> Option<String> {
        self.says.iter().filter(|s| holds(&s.test, status)).find_map(|s| {
            words.say(&s.text, s.name.as_ref().map(|f| text_of(status, f)))
        })
    }

    pub fn act(&self, row: usize, what: Act) -> Done {
        let Some(row) = self.rows.get(row) else { return Done::Failed(None) };
        match (&row.kind, what) {
            (Kind::Switch { on, off, .. }, Act::Switch(now)) => self.send(if now { on } else { off }),
            (Kind::Choice { template, options, run, .. }, Act::Pick(i)) => {
                let Some(opt) = options.get(i) else { return Done::Failed(None) };
                match self.send(&template.replace("{}", &opt.value)) {
                    Done::Refused => run.as_ref().map_or(Done::Refused, |r| self.elevated(r, &opt.value)),
                    done => done,
                }
            }
            (Kind::Send { line }, Act::Press) => self.send(line),
            (Kind::Launch { run }, Act::Press) => self.launch(run),
            _ => Done::Failed(None),
        }
    }

    fn send(&self, line: &str) -> Done {
        match self.ask(&format!("{line}\n")).as_deref() {
            Some("ok") => Done::Kept,
            Some(_) => Done::Refused,
            None => Done::Silent,
        }
    }

    fn program(&self, run: &Run, value: Option<&str>) -> Option<(PathBuf, Vec<String>)> {
        let real = inside(&self.roots.lib, &self.dir, &run.program).filter(|p| p.is_file())?;
        let args = run.args.iter().map(|a| if a == "{}" { value.map(String::from) } else { Some(a.clone()) }).collect::<Option<_>>()?;
        Some((real, args))
    }

    fn launch(&self, run: &Run) -> Done {
        let Some((program, args)) = self.program(run, None) else { return Done::Failed(None) };
        match Command::new(program).args(args).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn() {
            Ok(mut child) => {
                std::thread::spawn(move || child.wait());
                Done::Kept
            }
            Err(_) => Done::Failed(None),
        }
    }

    fn elevated(&self, run: &Run, value: &str) -> Done {
        let Some((program, args)) = self.program(run, Some(value)) else { return Done::Failed(None) };
        let Ok(mut child) = Command::new(&self.roots.pkexec).arg(program).args(args)
            .stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn() else { return Done::Failed(None) };
        let first = child.stdout.take().map(first_line);
        let ends = Instant::now() + RUN_WAIT;
        let code = loop {
            match child.try_wait() {
                Ok(Some(status)) => break status.code(),
                Ok(None) if Instant::now() < ends => std::thread::sleep(Duration::from_millis(50)),
                _ => {
                    let _ = child.kill();
                    let _ = child.wait();
                    return Done::Failed(None);
                }
            }
        };
        let said = first.and_then(|line| line.recv_timeout(Duration::from_secs(1)).ok()).unwrap_or_default();
        let said = said.trim();
        match (said, code) {
            ("ok", _) => Done::Kept,
            (_, Some(126)) if said.is_empty() => Done::Cancelled,
            _ => Done::Failed(said.strip_prefix("failed ").map(|why| tidy(why, 80)).filter(|why| !why.is_empty())),
        }
    }
}

fn first_line(mut out: impl Read + Send + 'static) -> std::sync::mpsc::Receiver<String> {
    let (tell, hear) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut said = String::new();
        let _ = BufReader::new((&mut out).take(1024)).read_line(&mut said);
        let _ = tell.send(said);
        let mut buf = [0u8; 4096];
        while out.read(&mut buf).is_ok_and(|n| n > 0) {}
    });
    hear
}

fn row_of(r: &RawRow, dir: &str, roots: &Roots) -> Option<Row> {
    let label = key_ok(&r.label).then(|| r.label.clone())?;
    let sub = match &r.sub {
        Some(s) => Some(key_ok(s).then(|| s.clone())?),
        None => None,
    };
    let show = test_of(r.show.as_ref())?;
    let plain = |s: &Option<String>| s.as_ref().filter(|s| line_ok(s) && !s.contains(['{', '}'])).cloned();
    let field = || r.field.as_ref().filter(|f| field_ok(f)).cloned();
    let kind = match r.kind.as_str() {
        "switch" => Kind::Switch { field: field()?, on: plain(&r.send_on)?, off: plain(&r.send_off)? },
        "choice" => {
            let template = r.send.as_ref().filter(|s| line_ok(s) && s.matches("{}").count() <= 1 && s.replace("{}", "").find(['{', '}']).is_none())?.clone();
            if r.options.is_empty() || r.options.len() > OPTIONS_MOST {
                return None;
            }
            let options = r.options.iter().map(|o| {
                let value = value_of(&o.value)?;
                line_ok(&template.replace("{}", &value)).then_some(())?;
                Some(Opt { value, text: key_ok(&o.text).then(|| o.text.clone())? })
            }).collect::<Option<Vec<_>>>()?;
            let run = match &r.run {
                Some(t) => Some(run_of(t, dir, roots, true)?),
                None => None,
            };
            Kind::Choice { field: field()?, template, options, run }
        }
        "button" => match (&r.run, &r.send) {
            (Some(t), None) => Kind::Launch { run: run_of(t, dir, roots, false)? },
            (None, Some(_)) => Kind::Send { line: plain(&r.send)? },
            _ => return None,
        },
        _ => return None,
    };
    Some(Row { label, sub, show, kind })
}

fn holds(test: &Test, status: &Status) -> bool {
    match test {
        Test::Always => true,
        Test::Is(f) => truth(status, f),
        Test::Not(f) => !truth(status, f),
    }
}

fn truth(status: &Status, field: &str) -> bool {
    status.get(field).and_then(|v| v.as_bool()).unwrap_or(false)
}

pub fn text_of(status: &Status, field: &str) -> String {
    match status.get(field) {
        Some(serde_json::Value::String(s)) => tidy(s, 64),
        Some(serde_json::Value::Number(n)) => n.to_string(),
        _ => String::new(),
    }
}

impl Row {
    pub fn shows(&self, status: &Status) -> bool {
        holds(&self.show, status)
    }
}

impl Kind {
    pub fn is_on(&self, status: &Status) -> bool {
        matches!(self, Kind::Switch { field, .. } if truth(status, field))
    }

    pub fn chosen(&self, status: &Status) -> Option<usize> {
        match self {
            Kind::Choice { field, options, .. } => {
                let now = text_of(status, field);
                options.iter().position(|o| o.value == now)
            }
            _ => None,
        }
    }
}

pub struct Words {
    cat: Catalogue,
    known: HashSet<String>,
}

impl Words {
    pub fn say(&self, key: &str, name: Option<String>) -> Option<String> {
        self.known.contains(key).then(|| match name {
            Some(n) => self.cat.with(key, &[("name", Arg::from(n))]),
            None => self.cat.with(key, &[]),
        })
    }
}

pub fn ask_at(path: &Path, line: &str) -> Option<String> {
    let mut s = std::os::unix::net::UnixStream::connect(path).ok()?;
    s.set_read_timeout(Some(WAIT)).ok()?;
    s.set_write_timeout(Some(WAIT)).ok()?;
    s.write_all(line.as_bytes()).ok()?;
    let mut answer = String::new();
    BufReader::new(s.take(4096)).read_line(&mut answer).ok()?;
    answer.strip_suffix('\n').map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn a_helper_that_writes_a_lot_is_not_left_waiting_and_its_first_line_is_kept() {
        let mut child = Command::new("sh").args(["-c", "echo ok; head -c 1000000 /dev/zero"])
            .stdout(Stdio::piped()).spawn().unwrap();
        let line = first_line(child.stdout.take().unwrap());
        assert!(child.wait().unwrap().success(), "it finished instead of stalling on a full pipe");
        assert_eq!(line.recv_timeout(Duration::from_secs(5)).unwrap().trim(), "ok");
    }

    thread_local! {
        pub static HERE: RefCell<Option<Roots>> = const { RefCell::new(None) };
    }

    const FILE: &str = r#"
name = "Thing"
words = "thing"
socket = "/run/trindle-thing/user.sock"
status = "status"
[[say]]
test = "!on"
text = "t-off"
[[say]]
text = "t-on"
name = "who"
[[row]]
kind = "switch"
label = "t-on-label"
field = "on"
send_on = "on"
send_off = "off"
[[row]]
kind = "choice"
label = "t-delay"
field = "delay"
send = "delay {}"
options = [ { value = 10, text = "t-ten" }, { value = 30, text = "t-thirty" } ]
[[row]]
kind = "choice"
label = "t-more"
field = "more"
send = "more {}"
run = "/usr/lib/trindle-thing/set {}"
options = [ { value = 0, text = "t-never" }, { value = 60, text = "t-minute" } ]
[[row]]
kind = "button"
label = "t-forget"
send = "unpair"
[[row]]
kind = "button"
label = "t-pair"
run = "/usr/lib/trindle-thing/pairing"
"#;

    struct World {
        roots: Roots,
        top: PathBuf,
    }

    fn world() -> World {
        static N: AtomicUsize = AtomicUsize::new(0);
        let base = std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
        let top = base.join(format!("trindle-rows-{}-{}", std::process::id(), N.fetch_add(1, Ordering::Relaxed)));
        let _ = std::fs::remove_dir_all(&top);
        for d in ["share/plugins", "share/lang/en", "run/trindle-thing", "lib/trindle-thing"] {
            std::fs::create_dir_all(top.join(d)).unwrap();
        }
        std::fs::write(top.join("share/lang/en/thing.ftl"), "t-off = Off.\nt-on = Paired with { $name }.\nt-on-label = On\n").unwrap();
        let wrapper = top.join("pkexec");
        std::fs::write(&wrapper, "#!/bin/sh\nexec \"$@\"\n").unwrap();
        std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).unwrap();
        let roots = Roots { share: top.join("share"), run: top.join("run"), lib: top.join("lib"), pkexec: wrapper };
        World { roots, top }
    }

    fn program(w: &World, name: &str, body: &str) {
        let p = w.top.join("lib/trindle-thing").join(name);
        std::fs::write(&p, format!("#!/bin/sh\n{body}\n")).unwrap();
        std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o755)).unwrap();
    }

    fn daemon(w: &World, replies: Vec<&'static str>) -> std::thread::JoinHandle<Vec<String>> {
        let path = w.top.join("run/trindle-thing/user.sock");
        let listener = std::os::unix::net::UnixListener::bind(&path).unwrap();
        std::thread::spawn(move || {
            let mut heard = Vec::new();
            for reply in replies {
                let (mut s, _) = listener.accept().unwrap();
                let mut line = String::new();
                BufReader::new(&s).read_line(&mut line).unwrap();
                heard.push(line);
                s.write_all(reply.as_bytes()).unwrap();
            }
            heard
        })
    }

    fn rows(w: &World) -> Rows {
        Rows::parse("thing", FILE, &w.roots).expect("the made-up file reads")
    }

    #[test]
    fn the_file_is_read_and_its_words_come_only_from_the_words_file() {
        let w = world();
        std::fs::write(w.roots.share.join("plugins/thing.toml"), FILE).unwrap();
        let all = load_from(&w.roots);
        assert_eq!(all.len(), 1);
        let r = &all[0];
        assert_eq!((r.rows.len(), r.says.len()), (5, 2));
        let words = r.words(&[]);
        let status: Status = serde_json::from_str(r#"{"on":true,"who":"John's phone\u0007"}"#).unwrap();
        assert_eq!(r.sentence(&status, &words).as_deref(), Some("Paired with John's phone."));
        let off: Status = serde_json::from_str(r#"{"on":false}"#).unwrap();
        assert_eq!(r.sentence(&off, &words).as_deref(), Some("Off."));
        assert_eq!(words.say("t-ten", None), None);
        assert_eq!(words.say("t-on-label", None).as_deref(), Some("On"));
    }

    #[test]
    fn a_file_of_the_wrong_size_or_shape_shows_nothing() {
        let w = world();
        let big = format!("{FILE}\n# {}", "x".repeat(70_000));
        assert!(Rows::parse("thing", &big, &w.roots).is_none());
        assert!(Rows::parse("Thing", FILE, &w.roots).is_none(), "the id is lower case words");
        assert!(Rows::parse("../thing", FILE, &w.roots).is_none());
        assert!(Rows::parse("thing", "name = ", &w.roots).is_none());
        assert!(Rows::parse("thing", &FILE.replace("kind = \"button\"\nlabel = \"t-pair\"", "kind = \"mystery\"\nlabel = \"t-pair\""), &w.roots).is_none());
    }

    #[test]
    fn a_socket_outside_the_plugins_own_run_folder_is_refused() {
        let w = world();
        for socket in ["/run/dbus/system_bus_socket", "/tmp/x.sock", "/run/trindle-thing/../dbus/s", "/run/trindle-thing", "run/trindle-thing/user.sock",
                       "/run/trindle-thingy/user.sock", "/run/trindle-other/user.sock"] {
            let hostile = FILE.replace("/run/trindle-thing/user.sock", socket);
            assert!(Rows::parse("thing", &hostile, &w.roots).is_none(), "{socket}");
        }
    }

    #[test]
    fn a_link_out_of_the_run_folder_is_never_talked_to() {
        let w = world();
        let outside = w.top.join("outside.sock");
        let listener = std::os::unix::net::UnixListener::bind(&outside).unwrap();
        listener.set_nonblocking(true).unwrap();
        std::os::unix::fs::symlink(&outside, w.top.join("run/trindle-thing/user.sock")).unwrap();
        let r = rows(&w);
        assert!(r.status().is_none() && r.act(0, Act::Switch(true)) == Done::Silent);
        assert!(listener.accept().is_err(), "nothing was ever connected to");
        std::fs::remove_dir_all(w.top.join("run/trindle-thing")).unwrap();
        let away = w.top.join("away");
        std::fs::create_dir(&away).unwrap();
        let _l = std::os::unix::net::UnixListener::bind(away.join("user.sock")).unwrap();
        std::os::unix::fs::symlink(&away, w.top.join("run/trindle-thing")).unwrap();
        assert!(r.socket().is_none());
    }

    #[test]
    fn a_program_outside_its_own_lib_folder_is_refused() {
        let w = world();
        for run in ["/usr/lib/trindle-thing/../evil", "/usr/bin/evil", "/usr/lib/other/evil", "/usr/lib/trindle-thing", "/usr/lib/trindle-thing/x/../../y",
                    "set", "/usr/lib/trindle-thing/set $(id)", "/usr/lib/trindle-thing/set {} {}"] {
            let hostile = FILE.replace("/usr/lib/trindle-thing/set {}", run);
            assert!(Rows::parse("thing", &hostile, &w.roots).is_none(), "{run}");
        }
        std::os::unix::fs::symlink("/bin/sh", w.top.join("lib/trindle-thing/pairing")).unwrap();
        assert_eq!(rows(&w).act(4, Act::Press), Done::Failed(None));
    }

    #[test]
    fn a_newline_or_a_control_in_a_value_or_a_line_refuses_the_file() {
        let w = world();
        for (from, to) in [("send = \"unpair\"", "send = \"unpair\\nreauth 0\""), ("send_on = \"on\"", "send_on = \"on\\r\\nunpair\""),
                           ("{ value = 10,", "{ value = \"10\\nunpair\","), ("{ value = 10,", "{ value = \"1 0\","),
                           ("{ value = 10,", "{ value = -1,"), ("send = \"delay {}\"", "send = \"delay {} {}\""),
                           ("send = \"unpair\"", "send = \"unp\\u00e9\"")] {
            assert!(Rows::parse("thing", &FILE.replace(from, to), &w.roots).is_none(), "{to}");
        }
    }

    #[test]
    fn only_the_lines_the_rows_make_are_sent() {
        let w = world();
        let r = rows(&w);
        let d = daemon(&w, vec!["ok\n"; 5]);
        assert_eq!(r.act(0, Act::Switch(true)), Done::Kept);
        assert_eq!(r.act(0, Act::Switch(false)), Done::Kept);
        assert_eq!(r.act(1, Act::Pick(1)), Done::Kept);
        assert_eq!(r.act(3, Act::Press), Done::Kept);
        assert_eq!(r.act(1, Act::Pick(2)), Done::Failed(None));
        assert_eq!(r.act(9, Act::Press), Done::Failed(None));
        assert_eq!(r.act(1, Act::Switch(true)), Done::Failed(None));
        assert_eq!(r.act(0, Act::Press), Done::Failed(None));
        assert_eq!(r.act(2, Act::Pick(1)), Done::Kept);
        assert_eq!(d.join().unwrap(), ["on\n", "off\n", "delay 30\n", "unpair\n", "more 60\n"]);
    }

    #[test]
    fn status_is_read_and_a_silent_daemon_is_silent() {
        let w = world();
        let r = rows(&w);
        assert!(r.status().is_none() && r.act(3, Act::Press) == Done::Silent, "nothing listens");
        let d = daemon(&w, vec!["{\"on\":true,\"delay\":30,\"who\":\"P\"}\n", "junk\n"]);
        let s = r.status().unwrap();
        assert!(r.rows[0].kind.is_on(&s) && r.rows[1].kind.chosen(&s) == Some(1) && r.rows[2].kind.chosen(&s).is_none());
        assert!(r.status().is_none());
        d.join().unwrap();
    }

    #[test]
    fn a_refused_choice_runs_the_rows_helper_through_pkexec_with_the_value() {
        let w = world();
        let r = rows(&w);
        let out = w.top.join("heard");
        program(&w, "set", &format!("echo \"$@\" > {}; echo ok", out.display()));
        let d = daemon(&w, vec!["none\n"]);
        assert_eq!(r.act(2, Act::Pick(1)), Done::Kept);
        assert_eq!(std::fs::read_to_string(&out).unwrap().trim(), "60");
        assert_eq!(d.join().unwrap(), ["more 60\n"]);
        program(&w, "set", "echo 'failed wrong password'");
        let d = daemon_again(&w, vec!["none\n"]);
        assert_eq!(r.act(2, Act::Pick(0)), Done::Failed(Some("wrong password".into())));
        d.join().unwrap();
        program(&w, "set", "exit 126");
        let d = daemon_again(&w, vec!["none\n"]);
        assert_eq!(r.act(2, Act::Pick(0)), Done::Cancelled);
        d.join().unwrap();
        let d = daemon_again(&w, vec!["none\n"]);
        assert_eq!(r.act(1, Act::Pick(0)), Done::Refused);
        d.join().unwrap();
    }

    fn daemon_again(w: &World, replies: Vec<&'static str>) -> std::thread::JoinHandle<Vec<String>> {
        let _ = std::fs::remove_file(w.top.join("run/trindle-thing/user.sock"));
        daemon(w, replies)
    }

    #[test]
    fn the_rows_own_program_is_started_and_nothing_else() {
        let w = world();
        let r = rows(&w);
        let out = w.top.join("started");
        program(&w, "pairing", &format!("touch {}", out.display()));
        assert_eq!(r.act(4, Act::Press), Done::Kept);
        for _ in 0..100 {
            if out.exists() {
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(out.exists());
    }

    impl Drop for World {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.top);
        }
    }
}
