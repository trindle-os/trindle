//! Tests of what the broker runs outside the fence.

use super::*;

thread_local! {
    pub static ADMIN: std::cell::RefCell<Option<PathBuf>> = const { std::cell::RefCell::new(None) };
}

fn v(s: &[&str]) -> Vec<String> {
    s.iter().map(|x| x.to_string()).collect()
}

fn scratch(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("trindle-broker-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn request(kind: &str, args: &[&str], root: &Path, approver: &Approver, dir: &Path) -> (i32, String, String) {
    let open = |n: &str| std::fs::File::create(dir.join(n)).unwrap();
    let (out, err) = (dir.join("out"), dir.join("err"));
    let io = (OwnedFd::from(std::fs::File::open("/dev/null").unwrap()), OwnedFd::from(open("out")), Some(OwnedFd::from(open("err"))));
    let code = handle(kind, &v(args), root, dir, approver, io);
    (code, std::fs::read_to_string(out).unwrap(), std::fs::read_to_string(err).unwrap())
}

#[test]
fn the_real_ssh_reaches_a_listed_computer_with_the_programs_own_key_only() {
    let sshd = Path::new("/usr/sbin/sshd");
    if !sshd.exists() {
        eprintln!("no sshd on this computer: the real ssh run is not tried");
        return;
    }
    let dir = scratch("ssh");
    let root = dir.join("root");
    make_key(&root.join("ssh")).unwrap();
    crate::store::fence::set_ssh_hosts(&root, &["localhost".to_string()]).unwrap();
    let host = dir.join("host");
    assert!(std::process::Command::new("ssh-keygen").args(["-q", "-t", "ed25519", "-N", "", "-f"]).arg(&host).status().unwrap().success());
    let other = dir.join("other");
    assert!(std::process::Command::new("ssh-keygen").args(["-q", "-t", "ed25519", "-N", "", "-f"]).arg(&other).status().unwrap().success());
    std::fs::copy(root.join("ssh/id_ed25519.pub"), dir.join("auth")).unwrap();
    let port = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
    std::fs::write(dir.join("sshd_config"), format!(
        "Port {port}\nListenAddress 127.0.0.1\nHostKey {}\nAuthorizedKeysFile {}\nPidFile {}\nUsePAM no\nStrictModes no\nPasswordAuthentication no\n",
        host.display(), dir.join("auth").display(), dir.join("pid").display())).unwrap();
    let mut server = std::process::Command::new(sshd).arg("-D").arg("-f").arg(dir.join("sshd_config")).arg("-E").arg(dir.join("log"))
        .stdin(Stdio::null()).spawn().unwrap();
    for _ in 0..100 {
        if std::net::TcpStream::connect(("127.0.0.1", port)).is_ok() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    let asked = Arc::new(Mutex::new(0));
    let counted = asked.clone();
    let approver: Approver = Arc::new(move |_, _| { *counted.lock().unwrap() += 1; Answer::No });
    let p = port.to_string();
    let (code, out, err) = request("ssh", &["-4", "-p", &p, "localhost", "echo", "from-the-far-side"], &root, &approver, &dir);
    let log = std::fs::read_to_string(dir.join("log")).unwrap_or_default();
    let _ = server.kill();
    let _ = server.wait();
    assert_eq!((code, out.trim()), (0, "from-the-far-side"), "{err} {log}");
    assert_eq!(*asked.lock().unwrap(), 0, "ssh to a listed computer asks nothing");
    assert!(root.join("ssh/known_hosts").exists(), "the programs' own list of known computers was used");
    let (code, out, err) = request("ssh", &["-4", "-p", &p, "127.0.0.1", "echo", "x"], &root, &approver, &dir);
    assert!(code == 255 && out.is_empty() && err.contains("not a computer name"), "{code} {err}");
    let (code, out, err) = request("ssh", &["-p", &p, "elsewhere.example.com", "echo", "x"], &root, &approver, &dir);
    assert!(code == 255 && out.is_empty() && err.contains("is not on the list"), "{code} {err}");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn a_command_as_the_administrator_runs_only_after_the_person_allows_it_and_one_window_at_a_time() {
    let dir = scratch("admin");
    let root = dir.join("root");
    let fake = dir.join("pkexec");
    std::fs::write(&fake, "#!/bin/sh\necho \"$@\" > \"$0.ran\"\npwd >> \"$0.ran\"\nexit 7\n").unwrap();
    std::fs::set_permissions(&fake, std::os::unix::fs::PermissionsExt::from_mode(0o755)).unwrap();
    ADMIN.with(|a| *a.borrow_mut() = Some(fake.clone()));
    let shown = Arc::new(Mutex::new(Vec::new()));
    let seen = shown.clone();
    let no: Approver = Arc::new(move |kind, text| { seen.lock().unwrap().push((kind.to_string(), text.to_string())); Answer::No });
    let (code, _, err) = request("sudo", &["apt", "install", "a b"], &root, &no, &dir);
    assert_eq!(code, 1);
    assert!(err.contains("You did not allow this."));
    assert!(!dir.join("pkexec.ran").exists(), "nothing ran before the person allowed it");
    assert_eq!(*shown.lock().unwrap(), [("sudo".to_string(), "apt install 'a b'".to_string())], "the window shows the command exactly as it runs");
    let yes: Approver = Arc::new(|_, _| Answer::Once(false));
    let (code, _, _) = request("pkexec", &["apt", "install", "a b"], &root, &yes, &dir);
    assert_eq!(code, 7, "the command's own exit code comes back");
    assert_eq!(std::fs::read_to_string(dir.join("pkexec.ran")).unwrap(), "apt install a b\n/\n");
    let (opened_tx, opened_rx) = std::sync::mpsc::channel();
    let (answer_tx, answer_rx) = std::sync::mpsc::channel::<()>();
    let slow: Approver = Arc::new({
        let answer_rx = Mutex::new(answer_rx);
        let opened_tx = Mutex::new(opened_tx);
        move |_, _| { opened_tx.lock().unwrap().send(()).unwrap(); let _ = answer_rx.lock().unwrap().recv(); Answer::No }
    });
    let (root2, dir2, slow2) = (root.clone(), dir.join("first"), slow.clone());
    std::fs::create_dir_all(&dir2).unwrap();
    let first = std::thread::spawn(move || request("sudo", &["id"], &root2, &slow2, &dir2).0);
    opened_rx.recv().unwrap();
    let second_dir = dir.join("second");
    std::fs::create_dir_all(&second_dir).unwrap();
    let (code, _, err) = request("sudo", &["id"], &root, &slow, &second_dir);
    assert_eq!(code, 1);
    assert!(err.contains("Another request is waiting for you"), "{err}");
    answer_tx.send(()).unwrap();
    assert_eq!(first.join().unwrap(), 1);
    ADMIN.with(|a| *a.borrow_mut() = None);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn ssh_goes_only_to_listed_computers_with_plain_options() {
    let hosts = v(&["nas.local", "pi.example.com"]);
    let ok = parse_ssh(&v(&["-p", "2222", "-Tq", "john@NAS.local", "systemctl", "status", "x"]), &hosts).unwrap();
    assert_eq!(ok, v(&["-p", "2222", "-T", "-q", "--", "john@NAS.local", "systemctl", "status", "x"]));
    assert!(parse_ssh(&v(&["-oConnectTimeout=5", "pi.example.com"]), &hosts).is_ok());
    for bad in [
        v(&["other.example.com"]), v(&["-o", "ProxyCommand=sh -c id", "nas.local"]), v(&["-oLocalCommand=id", "nas.local"]),
        v(&["-J", "nas.local", "nas.local"]), v(&["-L", "1:x:2", "nas.local"]), v(&["-i", "/x", "nas.local"]),
        v(&["-F", "/x", "nas.local"]), v(&["-o", "PermitLocalCommand=yes", "nas.local"]), v(&["-oForwardAgent=yes", "nas.local"]),
        v(&["127.0.0.1"]), v(&["-oUser=-x", "nas.local"]), v(&["-l", "-oProxyCommand=x", "nas.local"]), v(&["-W", "x:1", "nas.local"]),
        v(&["--", "-oProxyCommand=id"]), v(&["nas.local@evil.example.com"]), v(&[]), v(&["-p", "x", "nas.local"]),
    ] {
        assert!(parse_ssh(&bad, &hosts).is_err(), "{bad:?}");
    }
    let cmd = parse_ssh(&v(&["nas.local", "-oProxyCommand=id"]), &hosts).unwrap();
    assert_eq!(cmd, v(&["--", "nas.local", "-oProxyCommand=id"]));
}

#[test]
fn sudo_takes_a_command_and_nothing_stranger() {
    assert_eq!(parse_sudo("sudo", &v(&["-n", "--", "apt", "update"])).unwrap(), v(&["apt", "update"]));
    for bad in [v(&["-s"]), v(&["-i"]), v(&["-u", "x", "id"]), v(&["-E", "-e", "f"]), v(&[]), v(&["-n"])] {
        assert!(parse_sudo("sudo", &bad).is_err(), "{bad:?}");
    }
    assert_eq!(approval_text(&v(&["rm", "-rf", "a b", "it's"])), "rm -rf 'a b' 'it'\\''s'");
}

#[test]
fn the_administrator_command_line_is_exactly_the_approved_words() {
    let c = admin_command(v(&["apt", "install", "a b", "--", "-y"]));
    assert_eq!(c.get_program(), "pkexec");
    assert_eq!(c.get_args().collect::<Vec<_>>(), ["apt", "install", "a b", "--", "-y"]);
    assert_eq!(c.get_current_dir(), Some(Path::new("/")));
    assert_eq!(c.get_envs().count(), 0);
}
