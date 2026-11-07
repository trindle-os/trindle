//! The Email portal.

use std::collections::HashMap;
use std::os::fd::{AsFd, OwnedFd};
use std::os::unix::fs::{DirBuilderExt, FileExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use zbus::zvariant::{OwnedObjectPath, Value};

use super::portal::{answer, DONE};
use super::portal_ask::{failed, get, Kit, Options};
use crate::common::apps::{self, Entry};
use crate::toolkit::bus::{Asked, Offer, Props};
use crate::toolkit::t;

pub(super) const INTERFACE: &str = "org.freedesktop.impl.portal.Email";
const XML: &str = r#"<interface name="org.freedesktop.impl.portal.Email">
    <method name="ComposeEmail">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
      <arg type="a{sv}" name="results" direction="out"/>
    </method>
  </interface>"#;

const MAILTO: &str = "x-scheme-handler/mailto";

const MAX_FILES: usize = 10;
const MAX_FILE: u64 = 25 << 20;
const MAX_ALL: u64 = 50 << 20;
const MAX_ADDRESSES: usize = 50;
const MAX_ADDRESS: usize = 254;
const MAX_SUBJECT: usize = 200;
const MAX_BODY: usize = 20_000;
const KEEP: Duration = Duration::from_secs(2 * 60 * 60);
const STAGE: &str = "trindle-mail";

#[derive(Debug, PartialEq, Clone, Copy)]
enum Why {
    NoApp,
    TooManyFiles,
    BigFile,
    BadFile,
    BadAddress,
    BadSubject,
    BadBody,
    NoPlace,
    NotStarted,
}

impl Why {
    fn words(self) -> String {
        match self {
            Why::NoApp => t!("screenshot-mail-no-app"),
            Why::TooManyFiles => t!("screenshot-mail-too-many"),
            Why::BigFile => t!("screenshot-mail-too-big"),
            Why::BadFile => t!("screenshot-mail-bad-file"),
            Why::BadAddress => t!("screenshot-mail-bad-address"),
            Why::BadSubject => t!("screenshot-mail-bad-subject"),
            Why::BadBody => t!("screenshot-mail-bad-words"),
            Why::NoPlace => t!("screenshot-mail-no-place"),
            Why::NotStarted => t!("screenshot-mail-not-started"),
        }.to_string()
    }
}

#[derive(Default, Debug, PartialEq)]
struct Mail {
    to: Vec<String>,
    cc: Vec<String>,
    bcc: Vec<String>,
    subject: String,
    body: String,
    files: Vec<String>,
}

fn address_ok(a: &str) -> bool {
    let bad = |c: char| c.is_control() || c.is_whitespace() || "<>,;\"\\()[]:'%\u{2028}\u{2029}".contains(c);
    let mut halves = a.split('@');
    let (Some(local), Some(domain), None) = (halves.next(), halves.next(), halves.next()) else { return false };
    a.chars().count() <= MAX_ADDRESS && !local.is_empty() && !domain.is_empty() && !a.chars().any(bad)
}

fn subject_ok(s: &str) -> bool {
    s.chars().count() <= MAX_SUBJECT && !s.chars().any(|c| c.is_control() || c == '\u{2028}' || c == '\u{2029}')
}

fn body_ok(s: &str) -> bool {
    s.chars().count() <= MAX_BODY && !s.chars().any(|c| (c.is_control() && !matches!(c, '\n' | '\r' | '\t')) || c == '\u{2028}' || c == '\u{2029}')
}

impl Mail {
    fn from(options: &Options) -> Result<Mail, Why> {
        let list = |key: &str| -> Result<Vec<String>, Why> {
            match options.get(key) {
                None => Ok(Vec::new()),
                Some(_) => get::<Vec<String>>(options, key).ok_or(Why::BadAddress),
            }
        };
        let mut to: Vec<String> = match options.get("address") {
            None => Vec::new(),
            Some(_) => vec![get::<String>(options, "address").ok_or(Why::BadAddress)?],
        };
        to.extend(list("addresses")?);
        let tidy = |v: Vec<String>| -> Vec<String> { v.into_iter().map(|a| a.trim().to_string()).filter(|a| !a.is_empty()).collect() };
        let (to, cc, bcc) = (tidy(to), tidy(list("cc")?), tidy(list("bcc")?));
        if to.len() + cc.len() + bcc.len() > MAX_ADDRESSES {
            return Err(Why::BadAddress);
        }
        if !to.iter().chain(&cc).chain(&bcc).all(|a| address_ok(a)) {
            return Err(Why::BadAddress);
        }
        let subject = get::<String>(options, "subject").unwrap_or_default();
        let body = get::<String>(options, "body").unwrap_or_default();
        if !subject_ok(&subject) {
            return Err(Why::BadSubject);
        }
        if !body_ok(&body) {
            return Err(Why::BadBody);
        }
        Ok(Mail { to, cc, bcc, subject, body, files: Vec::new() })
    }

    fn mailto(&self) -> String {
        let mut out = format!("mailto:{}", self.to.iter().map(|a| encode(a, true)).collect::<Vec<_>>().join(","));
        let mut parts: Vec<String> = Vec::new();
        for (key, all) in [("cc", &self.cc), ("bcc", &self.bcc)] {
            if !all.is_empty() {
                parts.push(format!("{key}={}", all.iter().map(|a| encode(a, true)).collect::<Vec<_>>().join(",")));
            }
        }
        for (key, words) in [("subject", &self.subject), ("body", &self.body)] {
            if !words.is_empty() {
                parts.push(format!("{key}={}", encode(words, false)));
            }
        }
        parts.extend(self.files.iter().map(|f| format!("attach={}", encode(f, false))));
        if !parts.is_empty() {
            out.push('?');
            out.push_str(&parts.join("&"));
        }
        out
    }

    fn compose(&self) -> String {
        let quoted = |s: &str| format!("'{}'", s.replace('\'', "\u{2019}"));
        let mut parts = Vec::new();
        for (key, all) in [("to", &self.to), ("cc", &self.cc), ("bcc", &self.bcc), ("attachment", &self.files)] {
            if !all.is_empty() {
                parts.push(format!("{key}={}", quoted(&all.join(","))));
            }
        }
        for (key, words) in [("subject", &self.subject), ("body", &self.body)] {
            if !words.is_empty() {
                parts.push(format!("{key}={}", quoted(words)));
            }
        }
        parts.join(",")
    }
}

fn descriptors(options: &Options) -> Result<Vec<OwnedFd>, Why> {
    let Some(value) = options.get("attachment_fds") else { return Ok(Vec::new()) };
    let value: &Value = value;
    let value = match value {
        Value::Value(inner) => &**inner,
        other => other,
    };
    let Value::Array(all) = value else { return Err(Why::BadFile) };
    if all.len() > MAX_FILES {
        return Err(Why::TooManyFiles);
    }
    all.iter()
        .map(|item| match item {
            Value::Fd(fd) => fd.as_fd().try_clone_to_owned().map_err(|_| Why::BadFile),
            _ => Err(Why::BadFile),
        })
        .collect()
}

fn copy_name(fd: &OwnedFd, n: usize) -> String {
    use std::os::fd::AsRawFd;
    let plain = |s: &str| !s.is_empty() && s.len() <= 64 && !s.starts_with('.')
        && s.chars().all(|c| c.is_ascii_alphanumeric() || "-_. ".contains(c));
    std::fs::read_link(format!("/proc/self/fd/{}", fd.as_raw_fd())).ok()
        .and_then(|p| p.file_name().map(|f| f.to_string_lossy().to_string()))
        .filter(|s| plain(s))
        .map(|s| format!("{n}-{}", s.replace(' ', "_")))
        .unwrap_or_else(|| format!("attachment-{n}"))
}

fn stage_root() -> Option<PathBuf> {
    let run = std::env::var_os("XDG_RUNTIME_DIR").map(PathBuf::from).filter(|p| p.is_absolute() && p.is_dir())?;
    let root = run.join(STAGE);
    let mut made = std::fs::DirBuilder::new();
    made.mode(0o700);
    let _ = made.create(&root);
    let meta = std::fs::symlink_metadata(&root).ok()?;
    use std::os::unix::fs::MetadataExt;
    // SAFETY: geteuid only reads this process's own user number.
    let me = unsafe { libc::geteuid() };
    (meta.is_dir() && meta.uid() == me).then_some(root)
}

fn sweep(root: &Path, now: SystemTime) {
    let Ok(all) = std::fs::read_dir(root) else { return };
    for e in all.flatten() {
        let old = e.metadata().and_then(|m| m.modified()).ok()
            .is_some_and(|t| now.duration_since(t).unwrap_or_default() > KEEP);
        if old {
            let _ = std::fs::remove_dir_all(e.path());
        }
    }
}

fn stage(root: &Path, fds: Vec<OwnedFd>) -> Result<(PathBuf, Vec<String>), Why> {
    static COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let n = COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let stamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(0);
    let folder = root.join(format!("{}-{stamp}-{n}", std::process::id()));
    let mut made = std::fs::DirBuilder::new();
    made.mode(0o700);
    made.create(&folder).map_err(|_| Why::NoPlace)?;
    let mut total = 0u64;
    let mut out = Vec::new();
    let result = (|| {
        for (i, fd) in fds.into_iter().enumerate() {
            let name = copy_name(&fd, i + 1);
            let source = std::fs::File::from(fd);
            let meta = source.metadata().map_err(|_| Why::BadFile)?;
            if !meta.is_file() {
                return Err(Why::BadFile);
            }
            if meta.len() > MAX_FILE {
                return Err(Why::BigFile);
            }
            let path = folder.join(name);
            let copy = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open(&path).map_err(|_| Why::NoPlace)?;
            let mut at = 0u64;
            let mut chunk = vec![0u8; 64 * 1024];
            loop {
                let got = source.read_at(&mut chunk, at).map_err(|_| Why::BadFile)?;
                if got == 0 {
                    break;
                }
                at += got as u64;
                total += got as u64;
                if at > MAX_FILE || total > MAX_ALL {
                    return Err(Why::BigFile);
                }
                copy.write_all_at(&chunk[..got], at - got as u64).map_err(|_| Why::NoPlace)?;
            }
            out.push(crate::session::folders::file_uri(&path));
        }
        Ok(())
    })();
    match result {
        Ok(()) => Ok((folder, out)),
        Err(why) => {
            let _ = std::fs::remove_dir_all(&folder);
            Err(why)
        }
    }
}

fn remove_later(folder: PathBuf) {
    let _ = std::thread::Builder::new().name("mail-tidy".into()).spawn(move || {
        std::thread::sleep(KEEP);
        let _ = std::fs::remove_dir_all(folder);
    });
}

fn encode(s: &str, address: bool) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-._~".contains(&b) || (address && b == b'@') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

fn is_thunderbird(app: &Entry) -> bool {
    let words = format!("{} {}", app.id, app.exec).to_lowercase();
    words.contains("thunderbird") || words.contains("betterbird")
}

fn start_thunderbird(app: &Entry, spec: String, token: Option<String>) -> bool {
    let Ok(file) = crate::common::keyfile::KeyFile::load(&app.path) else { return false };
    let Some((mut argv, folder)) = apps::command_of(&file, &app.path) else { return false };
    let at = argv.iter().position(|w| w.starts_with("@@")).unwrap_or(argv.len());
    argv.splice(at..at, ["-compose".to_string(), spec]);
    let mut command = std::process::Command::new(&argv[0]);
    command.args(&argv[1..]).stdin(std::process::Stdio::null());
    if let Some(folder) = folder {
        command.current_dir(folder);
    }
    if let Some(token) = token {
        command.env("XDG_ACTIVATION_TOKEN", token);
    }
    crate::common::run::child(command, |_| {}).is_some()
}

fn start(mail: &Mail, token: Option<String>) -> Result<(), Why> {
    let app = apps::opener_for(MAILTO).ok_or(Why::NoApp)?;
    let started = match is_thunderbird(&app) {
        true => start_thunderbird(&app, mail.compose(), token),
        false => apps::launch_with(&app, &[mail.mailto()], token),
    };
    started.then_some(()).ok_or(Why::NotStarted)
}

fn tell(kit: &Kit, why: Why) {
    let hold = kit.events.hold();
    kit.bus.call("org.freedesktop.Notifications", "/org/freedesktop/Notifications", "org.freedesktop.Notifications")
        .method("Notify", &("Trindle", 0u32, "mail-send-symbolic", t!("screenshot-mail-failed"), why.words(), Vec::<String>::new(),
                            HashMap::<&str, Value>::new(), -1i32),
                false, Some(Duration::from_secs(2)), &kit.events, move |_| drop(hold));
}

fn prepare(mut mail: Mail, fds: Vec<OwnedFd>, root: Option<&Path>) -> Result<(Mail, Option<PathBuf>), Why> {
    if fds.is_empty() {
        return Ok((mail, None));
    }
    let root = root.ok_or(Why::NoPlace)?;
    sweep(root, SystemTime::now());
    let (dir, files) = stage(root, fds)?;
    mail.files = files;
    Ok((mail, Some(dir)))
}

#[cfg(test)]
fn handle(options: &Options, root: Option<&Path>, open: &dyn Fn(&Mail) -> Result<(), Why>) -> Result<Option<PathBuf>, Why> {
    let (mail, folder) = prepare(Mail::from(options)?, descriptors(options)?, root)?;
    match open(&mail) {
        Ok(()) => Ok(folder),
        Err(why) => {
            if let Some(dir) = folder {
                let _ = std::fs::remove_dir_all(dir);
            }
            Err(why)
        }
    }
}

fn compose(kit: &Kit, asked: Asked) {
    let Ok((_handle, _app_id, _parent, options)) = asked.message().body().deserialize::<(OwnedObjectPath, String, String, Options)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "ComposeEmail takes a handle, an app, a window and options");
    };
    let token = get::<String>(&options, "activation_token").filter(|t| !t.is_empty());
    let (checked, fds) = (Mail::from(&options), descriptors(&options));
    let (mail, fds) = match (checked, fds) {
        (Ok(m), Ok(f)) => (m, f),
        (Err(why), _) | (_, Err(why)) => {
            tell(kit, why);
            return asked.reply(&failed());
        }
    };
    kit.hold();
    let kit = kit.clone();
    let root = stage_root();
    crate::common::run::on_thread(move || prepare(mail, fds, root.as_deref()), move |prepared| {
        kit.release();
        let sent = prepared.and_then(|(mail, folder)| match start(&mail, token) {
            Ok(()) => {
                if let Some(dir) = folder {
                    remove_later(dir);
                }
                Ok(())
            }
            Err(why) => {
                if let Some(dir) = folder {
                    let _ = std::fs::remove_dir_all(dir);
                }
                Err(why)
            }
        });
        match sent {
            Ok(()) => asked.reply(&answer(DONE, HashMap::new())),
            Err(why) => {
                tell(&kit, why);
                asked.reply(&failed())
            }
        }
    });
}

pub fn offer(kit: &Kit) -> Offer {
    if let Some(root) = stage_root() {
        sweep(&root, SystemTime::now());
    }
    let kit = kit.clone();
    Offer {
        name: INTERFACE.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| match asked.member().as_str() {
            "ComposeEmail" => compose(&kit, asked),
            _ => asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "Not part of Email"),
        }),
        props: Box::new(Props::new),
        set: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use std::sync::atomic::{AtomicU32, Ordering};

    fn text(v: &str) -> zbus::zvariant::OwnedValue { Value::from(v).try_into().expect("v") }
    fn texts(v: Vec<&str>) -> zbus::zvariant::OwnedValue { Value::from(v).try_into().expect("v") }

    fn options() -> Options {
        HashMap::from([
            ("address".to_string(), text("john@example.com")),
            ("addresses".to_string(), texts(vec!["jane@example.com"])),
            ("cc".to_string(), texts(vec!["sam@example.com", " "])),
            ("subject".to_string(), text("Tea & cake?")),
            ("body".to_string(), text("It's at 4.\nBring plates.")),
        ])
    }

    fn with(key: &str, v: zbus::zvariant::OwnedValue) -> Options {
        let mut o = options();
        o.insert(key.to_string(), v);
        o
    }

    fn place(name: &str) -> PathBuf {
        static N: AtomicU32 = AtomicU32::new(0);
        let dir = std::env::temp_dir().join(format!("494-{name}-{}-{}", std::process::id(), N.fetch_add(1, Ordering::Relaxed)));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("dir");
        dir
    }

    fn file_with(dir: &Path, name: &str, bytes: &[u8]) -> OwnedFd {
        let path = dir.join(name);
        std::fs::File::create(&path).expect("file").write_all(bytes).expect("write");
        std::fs::File::open(&path).expect("open").into()
    }

    #[test]
    fn a_mail_is_written_as_a_mailto_address() {
        let mail = Mail::from(&options()).expect("fine");
        assert_eq!(mail.to, vec!["john@example.com", "jane@example.com"]);
        assert_eq!(mail.cc, vec!["sam@example.com"], "blank addresses are dropped");
        assert_eq!(mail.mailto(), "mailto:john@example.com,jane@example.com?cc=sam@example.com\
            &subject=Tea%20%26%20cake%3F&body=It%27s%20at%204.%0ABring%20plates.");
        assert_eq!(Mail::default().mailto(), "mailto:");
        let with_file = Mail { files: vec!["file:///run/user/1/trindle-mail/1/A%20note.txt".into()], ..Mail::default() };
        assert_eq!(with_file.mailto(), "mailto:?attach=file%3A%2F%2F%2Frun%2Fuser%2F1%2Ftrindle-mail%2F1%2FA%2520note.txt");
    }

    #[test]
    fn thunderbird_is_told_in_its_own_words() {
        let mut mail = Mail::from(&options()).expect("fine");
        mail.files = vec!["file:///run/x/1-a.txt".into()];
        assert_eq!(mail.compose(), "to='john@example.com,jane@example.com',cc='sam@example.com',\
            attachment='file:///run/x/1-a.txt',subject='Tea & cake?',body='It\u{2019}s at 4.\nBring plates.'");
    }

    #[test]
    fn injected_addresses_are_refused() {
        for bad in ["a@example.com\nBcc: boss@example.com", "a@example.com\rBcc: x@example.com", "a@example.com,b@example.com",
                    "a b@example.com", "a@example.com\0", "<a@example.com>", "\"a\"@example.com", "a@@example.com",
                    "noat", "@example.com", "a@", "a@example.com;b@example.com", "a'@example.com", "a%0ABcc:x@example.com"] {
            assert_eq!(Mail::from(&with("address", text(bad))).err(), Some(Why::BadAddress), "address {bad:?}");
            for key in ["cc", "bcc"] {
                assert_eq!(Mail::from(&with(key, texts(vec![bad]))).err(), Some(Why::BadAddress), "{key} {bad:?}");
            }
            assert_eq!(Mail::from(&with("addresses", texts(vec![bad]))).err(), Some(Why::BadAddress), "{bad:?}");
        }
        for key in ["address", "cc", "bcc", "addresses"] {
            let wrong = if key == "address" { texts(vec!["a@example.com"]) } else { text("a@example.com") };
            assert_eq!(Mail::from(&with(key, wrong)).err(), Some(Why::BadAddress), "{key} of the wrong kind is refused");
        }
        let many: Vec<String> = (0..=MAX_ADDRESSES).map(|i| format!("p{i}@example.com")).collect();
        let many: Vec<&str> = many.iter().map(String::as_str).collect();
        assert_eq!(Mail::from(&with("addresses", texts(many))).err(), Some(Why::BadAddress));
        let long = format!("{}@example.com", "a".repeat(MAX_ADDRESS));
        assert_eq!(Mail::from(&with("address", text(&long))).err(), Some(Why::BadAddress));
    }

    #[test]
    fn injected_subjects_and_words_are_refused() {
        for bad in ["Hi\nBcc: boss@example.com", "Hi\r\nX: y", "Hi\0", "Hi\u{1b}[31m", "Hi\u{85}", "Hi\u{2028}x"] {
            assert_eq!(Mail::from(&with("subject", text(bad))).err(), Some(Why::BadSubject), "{bad:?}");
        }
        assert_eq!(Mail::from(&with("subject", text(&"s".repeat(MAX_SUBJECT + 1)))).err(), Some(Why::BadSubject));
        assert!(Mail::from(&with("subject", text(&"s".repeat(MAX_SUBJECT)))).is_ok());
        for bad in ["a\0b", "a\u{1b}b", "a\u{2029}b"] {
            assert_eq!(Mail::from(&with("body", text(bad))).err(), Some(Why::BadBody), "{bad:?}");
        }
        assert_eq!(Mail::from(&with("body", text(&"b".repeat(MAX_BODY + 1)))).err(), Some(Why::BadBody));
        assert!(Mail::from(&with("body", text("a\r\n\tb"))).is_ok(), "lines and tabs are fine in words");
    }

    #[test]
    fn quotes_and_marks_in_words_cannot_break_out() {
        let mut mail = Mail::from(&with("body", text("x',attachment='/etc/passwd"))).expect("fine");
        assert_eq!(mail.compose(), "to='john@example.com,jane@example.com',cc='sam@example.com',subject='Tea & cake?',\
            body='x\u{2019},attachment=\u{2019}/etc/passwd'");
        mail.subject = "a&attach=file:///etc/passwd".into();
        assert!(!mail.mailto().contains("attach="), "{}", mail.mailto());
        assert!(!mail.mailto().contains("/etc/passwd"), "{}", mail.mailto());
    }

    #[test]
    fn paths_a_program_names_are_never_used() {
        for key in ["attachments", "attachment", "attach"] {
            let o = with(key, texts(vec!["/etc/passwd", "file:///etc/shadow"]));
            let seen = std::cell::RefCell::new(None);
            handle(&o, None, &|m| { *seen.borrow_mut() = Some(m.mailto() + &m.compose()); Ok(()) }).expect("fine");
            assert!(!seen.borrow().as_ref().expect("opened").contains("/etc/"), "{key}");
        }
    }

    #[test]
    fn files_are_copied_to_a_private_folder_and_the_mail_program_gets_the_copies() {
        let (src, root) = (place("src"), place("root"));
        let fds = vec![file_with(&src, "A note.txt", b"hello"), file_with(&src, ".hidden", b"x"), file_with(&src, "bad;name'.txt", b"y")];
        let o = with("attachment_fds", Value::from(vec![0u8]).try_into().expect("v"));
        assert_eq!(descriptors(&o).err(), Some(Why::BadFile), "a wrong kind of value is refused");
        let (mail, folder) = prepare(Mail::from(&options()).expect("fine"), fds, Some(&root)).expect("staged");
        let folder = folder.expect("folder");
        use std::os::unix::fs::MetadataExt;
        assert_eq!(std::fs::metadata(&folder).expect("m").mode() & 0o777, 0o700);
        let names: Vec<_> = std::fs::read_dir(&folder).expect("dir").flatten().map(|e| e.file_name().to_string_lossy().to_string()).collect();
        assert_eq!(names.len(), 3);
        for n in &names {
            assert!(n.starts_with("1-A_note") || n.starts_with("attachment-") , "{n}");
            assert_eq!(std::fs::metadata(folder.join(n)).expect("m").mode() & 0o777, 0o600);
        }
        assert_eq!(mail.files.len(), 3);
        assert!(mail.files.iter().all(|f| f.starts_with("file://") && f.contains(folder.to_str().expect("utf8"))));
        assert!(mail.files.iter().all(|f| !f.contains(src.to_str().expect("utf8"))), "never the original path");
        assert_eq!(std::fs::read(folder.join("1-A_note.txt")).expect("copy"), b"hello");
    }

    #[test]
    fn too_many_too_big_and_odd_files_are_refused_and_leave_nothing() {
        let (src, root) = (place("src"), place("root"));
        let none = |root: &Path| std::fs::read_dir(root).expect("dir").count() == 0;
        let many: Vec<OwnedFd> = (0..=MAX_FILES).map(|i| file_with(&src, &format!("f{i}"), b"x")).collect();
        let fds: Vec<zbus::zvariant::OwnedFd> = many.into_iter().map(Into::into).collect();
        let o = with("attachment_fds", Value::from(zbus::zvariant::Array::from(fds.iter().map(|f| Value::from(zbus::zvariant::Fd::from(f))).collect::<Vec<_>>())).try_into().expect("v"));
        assert_eq!(descriptors(&o).err(), Some(Why::TooManyFiles));

        let big = src.join("big");
        std::fs::File::create(&big).expect("f").set_len(MAX_FILE + 1).expect("len");
        let fd: OwnedFd = std::fs::File::open(&big).expect("o").into();
        assert_eq!(prepare(Mail::default(), vec![fd], Some(&root)).err(), Some(Why::BigFile));
        assert!(none(&root), "nothing is kept after a refusal");

        let many_big: Vec<OwnedFd> = (0..3).map(|_| {
            let f = std::fs::File::open(&big).expect("o");
            f.into()
        }).collect();
        std::fs::File::create(&big).expect("f").set_len(MAX_FILE).expect("len");
        let _ = many_big;
        let three: Vec<OwnedFd> = (0..3).map(|_| std::fs::File::open(&big).expect("o").into()).collect();
        assert_eq!(prepare(Mail::default(), three, Some(&root)).err(), Some(Why::BigFile), "all together are limited too");
        assert!(none(&root));

        let dir: OwnedFd = std::fs::File::open(&src).expect("o").into();
        assert_eq!(prepare(Mail::default(), vec![dir], Some(&root)).err(), Some(Why::BadFile), "a folder is refused");
        let null: OwnedFd = std::fs::File::open("/dev/zero").expect("o").into();
        assert_eq!(prepare(Mail::default(), vec![null], Some(&root)).err(), Some(Why::BadFile), "a device is refused");
        assert!(none(&root));
        let fd = file_with(&src, "a", b"x");
        assert_eq!(prepare(Mail::default(), vec![fd], None).err(), Some(Why::NoPlace), "no private place means no copy");
    }

    #[test]
    fn copies_are_removed_when_no_mail_program_opens_and_old_ones_are_swept() {
        let (src, root) = (place("src"), place("root"));
        let o = with("attachment_fds", Value::from(zbus::zvariant::Array::from(vec![Value::from(zbus::zvariant::Fd::from(&std::fs::File::open(src.clone()).expect("o")))])).try_into().expect("v"));
        let _ = o;
        let fd = file_with(&src, "a.txt", b"x");
        let (mail, folder) = prepare(Mail::default(), vec![fd], Some(&root)).expect("staged");
        let folder = folder.expect("folder");
        assert_eq!(mail.files.len(), 1);
        assert!(folder.is_dir());
        sweep(&root, SystemTime::now());
        assert!(folder.is_dir(), "a new copy stays for the mail program");
        sweep(&root, SystemTime::now() + KEEP + Duration::from_secs(5));
        assert!(!folder.exists(), "an old copy is removed");

        let refused = handle(&options(), Some(&root), &|_| Err(Why::NoApp));
        assert_eq!(refused, Err(Why::NoApp));
        let opened = handle(&options(), Some(&root), &|_| Ok(()));
        assert_eq!(opened, Ok(None));
    }

    #[test]
    fn every_reason_has_words_that_say_what_to_do_or_why() {
        for why in [Why::NoApp, Why::TooManyFiles, Why::BigFile, Why::BadFile, Why::BadAddress, Why::BadSubject, Why::BadBody, Why::NoPlace, Why::NotStarted] {
            assert!(!why.words().is_empty() && !why.words().contains("screenshot-mail"), "{why:?}");
        }
        assert!(Why::NoApp.words().contains("Settings"), "it says where to choose one");
    }
}
