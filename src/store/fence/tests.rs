//! Tests of the fence for AI programs. The escape tests start a real
//! bubblewrap fence on a made-up home folder under /var/tmp (never the real
//! home, the real session bus or any network but localhost), run a program
//! inside it that tries to get at personal things, and read what it says.

use super::*;
use crate::store::fence_broker::{ask, start, Answer, Approver};
use std::os::linux::net::SocketAddrExt;
use std::sync::{Arc, Mutex};

const TRY: &str = "TRINDLE_TRY";
const ENTRY: &str = "store::fence::tests::the_fence_entry";

struct Room {
    base: PathBuf,
    home: PathBuf,
    root: PathBuf,
}

impl Drop for Room {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.base);
    }
}

fn put(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, text).unwrap();
}

fn make_room(tag: &str) -> Room {
    let base = PathBuf::from(format!("/var/tmp/trindle-fence-test-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let home = base.join("home");
    for (rel, text) in [
        (".ssh/id_test", "PRIVATE-KEY"), ("Documents/statement.txt", "BANK"), ("Documents/other.txt", "OTHER"),
        ("Documents/Statements/january.txt", "JANUARY"), ("9-Private/secret.txt", "SECRET"), ("Work_Repos/hello.txt", "HELLO"),
        (".local/share/keyrings/login.keyring", "KEYRING"), (".mozilla/firefox/logins.json", "LOGINS"), ("ReadOnly/f.txt", "RO"),
    ] {
        put(&home.join(rel), text);
    }
    let proj = home.join("Work_Repos");
    std::os::unix::fs::symlink(home.join("9-Private/secret.txt"), proj.join("link_abs")).unwrap();
    std::os::unix::fs::symlink("../9-Private/secret.txt", proj.join("link_rel")).unwrap();
    std::os::unix::fs::symlink(home.join(".ssh"), proj.join("link_dir")).unwrap();
    let root = home.join(".local/share/trindle/ai");
    std::fs::create_dir_all(&root).unwrap();
    Room { base, home, root }
}

fn names(dir: &Path) -> String {
    let mut n: Vec<String> = std::fs::read_dir(dir).map(|r| r.flatten().map(|e| e.file_name().to_string_lossy().into_owned()).collect()).unwrap_or_default();
    n.sort();
    n.join(",")
}

#[test]
fn the_fence_entry() {
    let Ok(out) = std::env::var(TRY) else { return };
    let mut lines: Vec<String> = Vec::new();
    if let Err(e) = scope_abstract_sockets() {
        lines.push(format!("scope=failed: {e}"));
    }
    if let Some(sock) = std::env::var_os(PROXY_ENV) {
        start_bridge(Path::new(&sock));
        net_tries(&mut lines);
    } else {
        tries(&mut lines);
    }

    std::fs::write(out, lines.join("\n")).unwrap();
    std::process::exit(0);
}

fn open(r: &mut Vec<String>, name: &str, got: bool) {
    r.push(format!("{name}={}", if got { "open" } else { "blocked" }));
}

fn works(r: &mut Vec<String>, name: &str, got: bool) {
    r.push(format!("{name}={}", if got { "works" } else { "failed" }));
}

fn var(n: &str) -> Option<String> {
    std::env::var(n).ok()
}

fn tries(r: &mut Vec<String>) {
    let home = PathBuf::from(var("HOME").unwrap());
    let proj = home.join("Work_Repos");
    tries_reads(r, &home, &proj);
    tries_outside_processes(r);
    tries_nested_boxes_and_mounts(r);
    tries_a_new_program(r, &home);
    tries_writes(r, &home);

    works(r, "read_system", std::fs::read("/usr/bin/env").is_ok() && std::fs::read("/etc/os-release").is_ok() && std::fs::read_dir("/var/log").is_ok());
    works(r, "read_share", std::fs::read_to_string(proj.join("hello.txt")).is_ok_and(|t| t == "HELLO"));
    works(r, "write_share", std::fs::write(proj.join("new.txt"), "x").is_ok());
    works(r, "write_nested_share", std::fs::write(home.join("Documents/Statements/new.txt"), "x").is_ok());
    works(r, "write_private_home", std::fs::write(home.join("stray.txt"), "x").is_ok());
    works(r, "typed_variable", var("TYPED_KEY").as_deref() == Some("abc"));
    works(r, "language", var("LANG").is_some());
    works(r, "path_has_standins", var("PATH").is_some_and(|p| p.starts_with(TRINDLE_AT)));
    if let Some(port) = var("TRINDLE_TRY_PORT") {
        works(r, "network_localhost", std::net::TcpStream::connect(format!("127.0.0.1:{port}")).is_ok());
    }
}

fn tries_reads(r: &mut Vec<String>, home: &Path, proj: &Path) {
    let reads = |p: PathBuf| std::fs::read(p).is_ok();
    open(r, "ssh_key", reads(home.join(".ssh/id_test")));
    open(r, "documents", reads(home.join("Documents/statement.txt")));
    open(r, "documents_sibling_of_share", reads(home.join("Documents/other.txt")));
    open(r, "sibling_secret", reads(home.join("9-Private/secret.txt")));
    open(r, "dotdot", reads(proj.join("../9-Private/secret.txt")));
    open(r, "dotdot_ssh", reads(proj.join("../.ssh/id_test")));
    open(r, "symlink_abs", reads(proj.join("link_abs")));
    open(r, "symlink_rel", reads(proj.join("link_rel")));
    open(r, "symlink_dir", reads(proj.join("link_dir/id_test")));
    open(r, "keyring", reads(home.join(".local/share/keyrings/login.keyring")));
    open(r, "browser", reads(home.join(".mozilla/firefox/logins.json")));

    r.push(format!("listing_home={}", names(home)));
    r.push(format!("listing_documents={}", names(&home.join("Documents"))));
}

fn tries_outside_processes(r: &mut Vec<String>) {

    if let Some(name) = var("TRINDLE_TRY_ABSTRACT") {
        let addr = std::os::unix::net::SocketAddr::from_abstract_name(name.as_bytes()).unwrap();
        open(r, "abstract_socket", std::os::unix::net::UnixStream::connect_addr(&addr).is_ok());
    }
    open(r, "run_user_has_anything", std::fs::read_dir("/run/user").map(|d| d.count() > 0).unwrap_or(false));
    open(r, "tmp_has_anything", names(Path::new("/tmp")) != ".trindle");
    open(r, "ssh_agent_var", var("SSH_AUTH_SOCK").is_some());
    open(r, "bus_var", var("DBUS_SESSION_BUS_ADDRESS").is_some());
    for k in ["ANTHROPIC_API_KEY", "GITHUB_TOKEN", "AWS_SECRET_ACCESS_KEY"] {
        open(r, &format!("env_{k}"), var(k).is_some());
    }

    if let Some(pid) = var("TRINDLE_TRY_PID").and_then(|p| p.parse::<i32>().ok()) {
        open(r, "other_process_seen", Path::new(&format!("/proc/{pid}")).exists());
        open(r, "other_process_root", std::fs::read_dir(format!("/proc/{pid}/root")).is_ok());
        // SAFETY: PTRACE_ATTACH on a process number; it only asks the kernel.
        let attached = unsafe { libc::ptrace(libc::PTRACE_ATTACH, pid, 0, 0) } == 0;
        open(r, "ptrace_other_process", attached);
    }
}

fn tries_nested_boxes_and_mounts(r: &mut Vec<String>) {

    // SAFETY: unshare with a flag; harmless when refused, and the process ends soon after.
    open(r, "nested_userns", unsafe { libc::unshare(libc::CLONE_NEWUSER) } == 0);
    let tmpfs = std::ffi::CString::new("tmpfs").unwrap();
    let at = std::ffi::CString::new("/tmp").unwrap();
    // SAFETY: mount with valid C strings and no data; refused or not, it touches only this box.
    open(r, "mount_tmpfs", unsafe { libc::mount(tmpfs.as_ptr(), at.as_ptr(), tmpfs.as_ptr(), 0, std::ptr::null()) } == 0);
    let bind = std::ffi::CString::new("/").unwrap();
    // SAFETY: as above, a bind mount.
    open(r, "mount_bind", unsafe { libc::mount(bind.as_ptr(), at.as_ptr(), std::ptr::null(), libc::MS_BIND, std::ptr::null()) } == 0);
    open(r, "sudo_setuid", std::process::Command::new("/usr/bin/sudo").args(["-n", "true"]).status().is_ok_and(|s| s.success()));
}

fn tries_a_new_program(r: &mut Vec<String>, home: &Path) {

    let bin = home.join(".local/bin");
    std::fs::create_dir_all(&bin).unwrap();
    let script = bin.join("newprog");
    let text = format!("#!/bin/sh\ncat {h}/.ssh/id_test {h}/9-Private/secret.txt {h}/Documents/statement.txt 2>/dev/null\n", h = home.display());
    std::fs::write(&script, text).unwrap();
    std::fs::set_permissions(&script, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
    let got = std::process::Command::new(&script).output().map(|o| o.stdout).unwrap_or_default();
    open(r, "installed_script_reads_home", !got.is_empty());
    let copy = bin.join("mycat");
    let copied = std::fs::copy("/usr/bin/cat", &copy).is_ok();
    std::fs::set_permissions(&copy, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
    let got = std::process::Command::new(&copy).arg(home.join("9-Private/secret.txt")).output().map(|o| o.status.success()).unwrap_or(false);
    open(r, "installed_binary_reads_home", copied && got);
}

fn tries_writes(r: &mut Vec<String>, home: &Path) {

    open(r, "write_usr", std::fs::write("/usr/trindle-test", "x").is_ok());
    open(r, "write_readonly_share", std::fs::write(home.join("ReadOnly/new.txt"), "x").is_ok());

    let own = home.join(".local/share/trindle/ai");
    let grant = format!("{}\n", home.join("9-Private").display());
    let wrote = std::fs::create_dir_all(&own).is_ok() && std::fs::write(own.join("shared"), &grant).is_ok()
        && std::fs::write(own.join("shared-once"), &grant).is_ok() && std::fs::write(own.join("fence-off"), "").is_ok();
    r.push(format!("grant_file_written_somewhere={}", if wrote { "works" } else { "failed" }));
}

fn host_env() -> Vec<(String, String)> {
    [("PATH", "/usr/local/bin:/usr/bin:/bin"), ("LANG", "C.UTF-8"), ("ANTHROPIC_API_KEY", "sk-test"), ("GITHUB_TOKEN", "ghp-test"),
     ("SSH_AUTH_SOCK", "/tmp/ssh-test/agent"), ("AWS_SECRET_ACCESS_KEY", "aws-test"), ("DBUS_SESSION_BUS_ADDRESS", "unix:path=/nowhere")]
        .iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
}

fn run_try(room: &Room, extra: Vec<(String, String)>) -> (i32, Vec<String>) {
    let proj = room.home.join("Work_Repos");
    let out = proj.join("try.out");
    let _ = std::fs::remove_file(&out);
    let mut extra_env = vec![(TRY.to_string(), out.to_string_lossy().into_owned())];
    extra_env.extend(extra);
    let code = run(&Start {
        root: &room.root, home: room.home.clone(), host_env: host_env(), cwd: proj.clone(), trindle: std::env::current_exe().unwrap(),
        command: vec!["--exact".into(), ENTRY.into(), "--nocapture".into(), "--test-threads=1".into()],
        program: None, approver: None, extra_env, name: None, run_dir: Some(room.base.join("run")), net_test: None,
    });
    let text = std::fs::read_to_string(&out).unwrap_or_default();
    (code, text.lines().map(String::from).collect())
}

fn got<'a>(lines: &'a [String], name: &str) -> &'a str {
    lines.iter().find_map(|l| l.strip_prefix(&format!("{name}="))).unwrap_or_else(|| panic!("{name} was not reported: {lines:#?}"))
}

#[test]
fn a_fenced_program_cannot_reach_personal_files_or_a_way_out() {
    crate::common::needs::program("bwrap");
    let room = make_room("escape");
    let proj = room.home.join("Work_Repos");

    share(&room.root, &room.home, &proj, false, true).unwrap();
    share(&room.root, &room.home, &room.home.join("Documents/Statements"), false, true).unwrap();
    share(&room.root, &room.home, &room.home.join("ReadOnly"), true, true).unwrap();
    set_typed_env(&room.root, "TYPED_KEY=abc").unwrap();

    let mut outsider = std::process::Command::new("sleep").arg("60").spawn().unwrap();
    let abstract_name = format!("trindle-fence-test-{}", std::process::id());
    let server = std::os::unix::net::UnixListener::bind_addr(&std::os::unix::net::SocketAddr::from_abstract_name(abstract_name.as_bytes()).unwrap()).unwrap();
    let tcp = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = tcp.local_addr().unwrap().port();
    std::thread::spawn(move || for _ in tcp.incoming() {});

    let addr = std::os::unix::net::SocketAddr::from_abstract_name(abstract_name.as_bytes()).unwrap();
    assert!(std::os::unix::net::UnixStream::connect_addr(&addr).is_ok(), "the test's own socket must be reachable from outside");
    let extra = vec![("TRINDLE_TRY_ABSTRACT".to_string(), abstract_name), ("TRINDLE_TRY_PID".to_string(), outsider.id().to_string()),
                     ("TRINDLE_TRY_PORT".to_string(), port.to_string())];
    for round in 1..=2 {
        let (code, lines) = run_try(&room, extra.clone());
        assert_eq!(code, 0, "round {round}: {lines:#?}");
        assert!(!lines.iter().any(|l| l.starts_with("scope=")), "{lines:#?}");
        for l in &lines {
            let Some((name, what)) = l.split_once('=') else { continue };
            match what {
                "open" => panic!("round {round}: {name} got through: {lines:#?}"),
                "failed" if name != "grant_file_written_somewhere" => panic!("round {round}: {name} did not work: {lines:#?}"),
                _ => {}
            }
        }

        let expect = if round == 1 { "Documents,ReadOnly,Work_Repos" } else { ".local,Documents,ReadOnly,Work_Repos,stray.txt" };
        assert_eq!(got(&lines, "listing_home"), expect, "round {round}");
        assert_eq!(got(&lines, "listing_documents"), "Statements");

        assert_eq!(got(&lines, "grant_file_written_somewhere"), "works");
        assert!(shared(&room.root, &room.home).iter().all(|s| !s.path.ends_with("9-Private")), "round {round}: the program granted itself a folder");
        assert!(!lines.iter().any(|l| l.contains("sibling_secret=open")));
        assert!(enabled(&room.root), "round {round}: the program switched the fence off");
        assert!(!room.root.join("shared-once").exists());
    }

    assert_eq!(std::fs::read_to_string(room.home.join("9-Private/secret.txt")).unwrap(), "SECRET");
    assert!(proj.join("new.txt").exists());
    assert!(!room.home.join("ReadOnly/new.txt").exists());
    assert!(room.root.join("home/stray.txt").exists(), "the private home is kept between runs");
    assert!(room.root.join("home/.local/share/trindle/ai/shared").exists());
    assert!(!room.home.join("stray.txt").exists(), "nothing was written to the real home");
    let _ = outsider.kill();
    let _ = outsider.wait();
    drop(server);
}

#[test]
fn the_walls_clear_the_environment_and_hide_the_home() {
    let f = Fence {
        home: PathBuf::from("/home/john"), private: PathBuf::from("/data/ai/home"),
        shared: vec![Share { path: "/home/john/Projects".into(), read_only: false }, Share { path: "/home/john/Docs".into(), read_only: true }],
        trindle: "/usr/bin/trindle".into(), broker: Some("/run/x/broker.sock".into()), program: Some("/home/john/.local/bin/claude".into()),
        cwd: "/home/john/Projects".into(), env: vec![("LANG".into(), "C".into())], seccomp: 5, secrets: Some(6),
        hide_dirs: vec!["/run/user".into()], hide_files: vec!["/run/docker.sock".into()], no_network: false, proxy: None, extra: vec![],
    };
    let a = walls(&f, &["claude".to_string()]);
    let line = a.join(" ");
    let at = |needle: &str| line.find(needle).unwrap_or_else(|| panic!("{needle} missing: {line}"));
    assert!(!line.contains("--share-net") && !line.contains("--unshare-net"), "the network is open by default");
    assert!(at("--clearenv") < at("--setenv LANG") && at("--setenv LANG") < at("--args 6"));
    assert!(at("--tmpfs /home") < at("--bind /data/ai/home /home/john") && at("--bind /data/ai/home /home/john") < at("--bind /home/john/Projects /home/john/Projects"));
    assert!(line.contains("--ro-bind /home/john/Docs /home/john/Docs"), "read only share: {line}");
    assert!(line.contains("--tmpfs /run/user") && line.contains("--ro-bind /dev/null /run/docker.sock"));
    assert!(line.contains("--unshare-pid") && line.contains("--disable-userns") && line.contains("--seccomp 5") && line.contains("--new-session"));
    assert!(line.contains("--ro-bind /home/john/.local/bin/claude /home/john/.local/bin/claude"));
    assert!(!a.iter().any(|w| w == "abc"), "typed values never go on the command line");
    let limited = walls(&Fence { no_network: true, ..f }, &[]);
    assert!(limited.iter().any(|w| w == "--unshare-net"));
}

#[test]
fn the_environment_passes_harmless_names_only() {
    let host: Vec<(String, String)> = [("LANG", "C"), ("LC_ALL", "C"), ("TERM", "xterm"), ("HOME", "/home/x"), ("ANTHROPIC_API_KEY", "k"), ("GITHUB_TOKEN", "t"),
        ("SSH_AUTH_SOCK", "s"), ("MY_SECRET", "s"), ("DBUS_SESSION_BUS_ADDRESS", "u"), ("DISPLAY", ":0"), ("WAYLAND_DISPLAY", "w"), ("LD_PRELOAD", "x"),
        ("OPENAI_API_KEY", "k"), ("GNOME_KEYRING_CONTROL", "k"), ("XDG_RUNTIME_DIR", "/run/user/1000"), ("AWS_ACCESS_KEY_ID", "k")]
        .iter().map(|(a, b)| (a.to_string(), b.to_string())).collect();
    let got: Vec<String> = clean_env(&host).into_iter().map(|(k, _)| k).collect();
    for ok in ["LANG", "LC_ALL", "TERM", "HOME"] {
        assert!(got.contains(&ok.to_string()), "{ok}");
    }
    for bad in ["ANTHROPIC_API_KEY", "GITHUB_TOKEN", "SSH_AUTH_SOCK", "MY_SECRET", "DBUS_SESSION_BUS_ADDRESS", "DISPLAY", "WAYLAND_DISPLAY", "LD_PRELOAD",
                "OPENAI_API_KEY", "GNOME_KEYRING_CONTROL", "XDG_RUNTIME_DIR", "AWS_ACCESS_KEY_ID"] {
        assert!(!got.contains(&bad.to_string()), "{bad} passed");
    }
    let env = env_for(&host, Path::new("/home/x"), Path::new("/home/x/.local/share/trindle/ai/bin"));
    let path = &env.iter().find(|(k, _)| k == "PATH").unwrap().1;
    assert!(path.starts_with(TRINDLE_AT), "{path}");
}

#[test]
fn only_the_person_can_type_a_key_in_and_never_a_reserved_name() {
    let room = make_room("typed");
    set_typed_env(&room.root, "ANTHROPIC_API_KEY=sk-1\nOTHER_TOKEN=x=y").unwrap();
    assert_eq!(typed_env(&room.root), [("ANTHROPIC_API_KEY".to_string(), "sk-1".to_string()), ("OTHER_TOKEN".to_string(), "x=y".to_string())]);
    for bad in ["PATH=/x", "LD_PRELOAD=/x", "HOME=/x", "lower=x", "NO_EQUALS", "NODE_OPTIONS=--require x", "TRINDLE_FENCE_BROKER=/x", "BASH_ENV=/x"] {
        assert!(set_typed_env(&room.root, bad).is_err(), "{bad}");
    }

    std::fs::write(room.root.join("env"), "LD_PRELOAD=/x\nPATH=/y\nGOOD_KEY=1\n").unwrap();
    assert_eq!(typed_env(&room.root), [("GOOD_KEY".to_string(), "1".to_string())]);
    let args = secret_args(&typed_env(&room.root));
    assert_eq!(args, b"--setenv\0GOOD_KEY\x001\0");
}

#[test]
fn a_folder_is_shared_only_when_it_is_plainly_not_personal() {
    let room = make_room("judge");
    let h = &room.home;
    let make = |rel: &str| {
        std::fs::create_dir_all(h.join(rel)).unwrap();
        h.join(rel)
    };
    for (rel, why) in [(".", "The whole home folder stays private."), (".ssh", "Hidden folders always stay private."),
                       (".gnupg", "Hidden folders always stay private."), (".mozilla", "Hidden folders always stay private."),
                       ("Work_Repos/.git", "Hidden folders always stay private."), ("snap", "It keeps passwords or mail, so it stays private."),
                       ("Mail/inbox", "It keeps passwords or mail, so it stays private.")] {
        let p = if rel == "." { h.clone() } else { make(rel) };
        assert_eq!(shareable(&p, h).unwrap_err(), why, "{rel}");
    }
    assert_eq!(shareable(h.parent().unwrap(), h).unwrap_err(), "It holds your home folder, so it stays private.");
    assert_eq!(shareable(Path::new("/usr"), h).unwrap_err(), "Only folders inside your home folder can be shared.");
    assert_eq!(shareable(&h.join("nothing"), h).unwrap_err(), "Choose a folder that is there.");
    assert_eq!(shareable(&h.join("Work_Repos/hello.txt"), h).unwrap_err(), "Choose a folder that is there.");

    assert_eq!(shareable(&h.join("Work_Repos/link_dir"), h).unwrap_err(), "Hidden folders always stay private.");
    assert!(shareable(&h.join("Work_Repos"), h).is_ok());

    let tmp = make("Swap");
    share(&room.root, h, &tmp, false, true).unwrap();
    assert_eq!(shared(&room.root, h).len(), 1);
    std::fs::remove_dir(&tmp).unwrap();
    std::os::unix::fs::symlink(h.join(".ssh"), &tmp).unwrap();
    assert!(shared(&room.root, h).is_empty(), "a link to .ssh must not stay shared");

    std::fs::write(room.root.join("shared"), format!("{}\n{}\nro:{}\n", h.display(), h.join(".ssh").display(), h.join("9-Private/../.ssh").display())).unwrap();
    assert!(shared(&room.root, h).is_empty());
}

#[test]
fn a_share_is_for_the_next_start_only_or_always_and_read_only_when_asked() {
    let room = make_room("share");
    let h = &room.home;
    share(&room.root, h, &h.join("Work_Repos"), true, false).unwrap();
    assert!(shared(&room.root, h).is_empty());
    assert_eq!(take_once(&room.root, h), [Share { path: h.join("Work_Repos"), read_only: true }]);
    assert!(take_once(&room.root, h).is_empty(), "once means once");
    share(&room.root, h, &h.join("ReadOnly"), false, true).unwrap();
    share(&room.root, h, &h.join("ReadOnly"), true, true).unwrap();
    assert_eq!(shared(&room.root, h), [Share { path: h.join("ReadOnly"), read_only: true }], "shared again replaces the old line");
    unshare(&room.root, &h.join("ReadOnly")).unwrap();
    assert!(shared(&room.root, h).is_empty());
}

#[test]
fn the_switches_the_list_and_the_programs_own_settings() {
    let room = make_room("switch");
    let h = &room.home;
    assert!(enabled(&room.root) && fenced(&room.root, "claude") && !fenced(&room.root, "vim"));
    set_programs(&room.root, &[("claude".into(), true), ("codex".into(), false), ("mine".into(), true)]).unwrap();
    assert!(fenced(&room.root, "claude") && !fenced(&room.root, "codex") && fenced(&room.root, "mine"));
    set_enabled(&room.root, false).unwrap();
    assert!(!fenced(&room.root, "claude"), "the one switch turns every program's fence off");
    set_enabled(&room.root, true).unwrap();
    for bad in ["ssh", "sudo", "bash", "a b", "../x", "-x", ""] {
        assert!(set_programs(&room.root, &[(bad.into(), true)]).is_err(), "{bad:?}");
    }
    assert!(wrappers(&room.root).join("claude").is_symlink() && !wrappers(&room.root).join("gemini").exists());

    std::fs::create_dir_all(h.join(".claude")).unwrap();
    assert!(own_settings_binds(&room.root, h, "claude").is_empty(), "off unless the person chose it");
    set_uses_own_settings(&room.root, "claude", true).unwrap();
    assert_eq!(own_settings_binds(&room.root, h, "claude"), [h.join(".claude")]);
    assert!(set_uses_own_settings(&room.root, "mine", true).is_err());
    for private in [".ssh", ".gnupg", ".mozilla", ".local/share/keyrings"] {
        assert!(never_offered(h, &h.join(private)), "{private}");
    }

    for name in ["claude", "codex", "gemini", "cursor-agent", "opencode", "goose", "amp", "aider"] {
        for rel in own_settings_of(name) {
            assert!(!never_offered(h, &h.join(rel)), "{name} {rel}");
        }
    }
}

fn stub(answer: Answer) -> (Approver, Arc<Mutex<Vec<(String, String)>>>) {
    let seen: Arc<Mutex<Vec<(String, String)>>> = Arc::default();
    let s = seen.clone();
    (Arc::new(move |kind, text| {
        s.lock().unwrap().push((kind.to_string(), text.to_string()));
        answer
    }), seen)
}

#[test]
fn a_request_from_inside_changes_nothing_without_an_answer() {
    let room = make_room("broker");
    let h = &room.home;
    std::fs::create_dir_all(h.join("Other")).unwrap();
    let (no, seen_no) = stub(Answer::No);
    let socket = start(&room.base, &room.root, h, Some(no)).unwrap();
    let share_ask = |p: &str| ask(&socket, SHARE_ASK, &[p.to_string()]);
    assert_eq!(share_ask(&h.join("Other").to_string_lossy()), 1);
    assert!(shared(&room.root, h).is_empty() && take_once(&room.root, h).is_empty());
    assert_eq!(seen_no.lock().unwrap().as_slice(), [("share".to_string(), h.join("Other").to_string_lossy().into_owned())]);
    seen_no.lock().unwrap().clear();

    for bad in [h.join(".ssh"), h.clone(), h.join(".gnupg"), h.join("Work_Repos/link_dir"), PathBuf::from("/usr"), PathBuf::from("relative/dir"),
                h.join("Other/../.ssh"), h.join("nothing")] {
        assert_eq!(share_ask(&bad.to_string_lossy()), 1, "{}", bad.display());
    }
    assert!(seen_no.lock().unwrap().is_empty(), "no window for what can never be shared");

    assert_eq!(ask(&socket, "ssh", &["unlisted.example.com".into(), "id".into()]), 255);
    assert_eq!(ask(&socket, "sudo", &["rm".into(), "-rf".into(), "a b".into()]), 1);
    assert_eq!(seen_no.lock().unwrap().as_slice(), [("sudo".to_string(), "rm -rf 'a b'".to_string())]);
    assert_eq!(ask(&socket, "ls", &[]), 1);

    let (once, _) = stub(Answer::Once(true));
    let room2 = make_room("broker2");
    let socket2 = start(&room2.base, &room2.root, &room2.home, Some(once)).unwrap();
    assert_eq!(ask(&socket2, SHARE_ASK, &[room2.home.join("Work_Repos").to_string_lossy().into_owned()]), 0);
    assert!(shared(&room2.root, &room2.home).is_empty());
    assert_eq!(take_once(&room2.root, &room2.home), [Share { path: room2.home.join("Work_Repos"), read_only: true }]);
    let (always, _) = stub(Answer::Always(false));
    let room3 = make_room("broker3");
    let socket3 = start(&room3.base, &room3.root, &room3.home, Some(always)).unwrap();
    assert_eq!(ask(&socket3, SHARE_ASK, &[room3.home.join("Work_Repos").to_string_lossy().into_owned()]), 0);
    assert_eq!(shared(&room3.root, &room3.home), [Share { path: room3.home.join("Work_Repos"), read_only: false }]);
}

fn through_proxy(host: &str, port: u16) -> String {
    use std::io::{Read, Write};
    let Ok(mut c) = std::net::TcpStream::connect(("127.0.0.1", super::super::fence_net::PROXY_PORT)) else { return "no proxy".into() };
    c.set_read_timeout(Some(std::time::Duration::from_secs(5))).unwrap();
    let _ = c.write_all(format!("CONNECT {host}:{port} HTTP/1.1\r\nHost: {host}:{port}\r\n\r\n").as_bytes());
    let mut got = String::new();
    let mut buf = [0u8; 256];
    while let Ok(n) = c.read(&mut buf) {
        if n == 0 {
            break;
        }
        got.push_str(&String::from_utf8_lossy(&buf[..n]));
        if got.contains("HELLO") || (got.contains("\r\n\r\n") && !got.starts_with("HTTP/1.1 200")) {
            break;
        }
    }
    got
}

fn net_tries(r: &mut Vec<String>) {
    use std::net::ToSocketAddrs;
    let port: u16 = std::env::var("TRINDLE_TRY_PORT").unwrap().parse().unwrap();
    let ms = std::time::Duration::from_millis(1500);
    open(r, "direct_to_the_computers_localhost", std::net::TcpStream::connect(("127.0.0.1", port)).is_ok());
    open(r, "direct_to_the_internet", std::net::TcpStream::connect_timeout(&"192.0.2.1:80".parse().unwrap(), ms).is_ok());
    open(r, "name_lookup", ("listed.example.com", 80).to_socket_addrs().is_ok_and(|mut a| a.next().is_some()));
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    open(r, "udp_out", socket.send_to(b"x", "192.0.2.1:53").is_ok());
    let good = through_proxy("listed.example.com", port);
    works(r, "listed_host_through_proxy", good.starts_with("HTTP/1.1 200") && good.contains("HELLO"));
    for (name, host) in [("unlisted_host", "other.example.com"), ("address_number", "127.0.0.1"), ("local_name", "localhost"), ("trick", "listed.example.com@evil.example")] {
        let got = through_proxy(host, port);
        open(r, name, got.starts_with("HTTP/1.1 200"));
    }
    open(r, "other_port", through_proxy("listed.example.com", port + 1).starts_with("HTTP/1.1 200"));
}

#[test]
fn with_the_network_limited_only_listed_hosts_can_be_reached() {
    crate::common::needs::program("bwrap");
    let room = make_room("net");
    let proj = room.home.join("Work_Repos");
    share(&room.root, &room.home, &proj, false, true).unwrap();
    set_net_hosts(&room.root, &["listed.example.com".to_string()]).unwrap();
    assert!(set_net_hosts(&room.root, &["127.0.0.1".to_string()]).is_err());
    set_net_limit(&room.root, true).unwrap();
    let server = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = server.local_addr().unwrap().port();
    std::thread::spawn(move || {
        use std::io::Write;
        for mut c in server.incoming().flatten() {
            let _ = c.write_all(b"HELLO");
        }
    });
    let resolver: super::super::fence_net::Resolver = Arc::new(|h, p| if h == "listed.example.com" || h == "other.example.com" { vec![std::net::SocketAddr::from(([127, 0, 0, 1], p))] } else { Vec::new() });
    let out = proj.join("try.out");
    let code = run(&Start {
        root: &room.root, home: room.home.clone(), host_env: host_env(), cwd: proj.clone(), trindle: std::env::current_exe().unwrap(),
        command: vec!["--exact".into(), ENTRY.into(), "--nocapture".into(), "--test-threads=1".into()],
        program: None, approver: None, extra_env: vec![(TRY.to_string(), out.to_string_lossy().into_owned()), ("TRINDLE_TRY_PORT".into(), port.to_string())],
        name: None, run_dir: Some(room.base.join("run")), net_test: Some((resolver, true, port)),
    });
    let lines: Vec<String> = std::fs::read_to_string(&out).unwrap_or_default().lines().map(String::from).collect();
    assert_eq!(code, 0, "{lines:#?}");
    assert!(lines.len() >= 9, "{lines:#?}");
    for l in &lines {
        assert!(!l.ends_with("=open") && !l.ends_with("=failed"), "{l}: {lines:#?}");
    }

    set_net_limit(&room.root, false).unwrap();
    assert!(net_limit(&room.root).is_none());
}
