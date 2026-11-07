//! Tests for copied_guard.rs.

#[cfg(test)]
use std::fmt::Write as _;
use super::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::unix::fs::symlink;

fn pair() -> (OwnedFd, OwnedFd) {
    let mut fds = [0 as RawFd; 2];
    // SAFETY: socketpair writes two descriptors into the array.
    assert_eq!(unsafe { libc::socketpair(libc::AF_UNIX, libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC, 0, fds.as_mut_ptr()) }, 0);
    (owned(fds[0]), owned(fds[1]))
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("trindle-guard-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::canonicalize(&dir).unwrap()
}

fn uris(paths: &[PathBuf]) -> Vec<(String, Vec<u8>)> {
    let text: String = paths.iter().fold(String::new(), |mut s, p| { let _ = write!(s, "file://{}\r\n", p.display()); s });
    vec![(copied::KINDS[1].to_string(), text.into_bytes())]
}

struct Fake {
    home: PathBuf,
    kinds: Mutex<Vec<(String, Vec<u8>)>>,
    copied_at: Mutex<Option<Instant>>,
    allowed: AtomicBool,
    reads: AtomicUsize,
}

fn serve_of(fake: &Arc<Fake>) -> Arc<Serve> {
    let (f1, f2, f3) = (fake.clone(), fake.clone(), fake.clone());
    Arc::new(Serve {
        clipboard: Box::new(move || {
            Some(Held { kinds: f1.kinds.lock().unwrap().clone(), copied_at: *f1.copied_at.lock().unwrap() })
        }),
        allowed: Box::new(move || f2.allowed.load(Ordering::SeqCst)),
        note: Box::new(move || {
            f3.reads.fetch_add(1, Ordering::SeqCst);
        }),
        roots: Roots::new(&fake.home, &[]),
        limits: Limits::default(),
        gate: Mutex::new(Gate::new()),
        pump: Pump::new(),
        dead: AtomicBool::new(false),
    })
}

fn fake(name: &str) -> Arc<Fake> {
    Arc::new(Fake {
        home: scratch(name),
        kinds: Mutex::new(Vec::new()),
        copied_at: Mutex::new(Some(Instant::now())),
        allowed: AtomicBool::new(true),
        reads: AtomicUsize::new(0),
    })
}

struct Answer {
    lists: Vec<(String, Vec<OwnedFd>)>,
    last: String,
}

fn ask(sock: RawFd) -> Answer {
    send_message(sock, b"LIST", &[]).unwrap();
    let mut a = Answer { lists: Vec::new(), last: String::new() };
    loop {
        let (body, fds) = recv_message(sock, 1 << 20).unwrap().expect("the connection stayed");
        let text = String::from_utf8(body).unwrap();
        if text.starts_with("FILES\n") {
            a.lists.push((text, fds));
        } else {
            assert!(fds.is_empty());
            a.last = text;
            return a;
        }
    }
}

fn contents(a: Answer) -> HashMap<String, Vec<u8>> {
    let mut out = HashMap::new();
    for (text, fds) in a.lists {
        let mut fds = fds.into_iter();
        for line in text.lines().skip(1) {
            let mut part = line.splitn(3, '\t');
            let (kind, size, rel) = (part.next().unwrap(), part.next().unwrap(), part.next().unwrap());
            if kind == "F" {
                let mut data = Vec::new();
                let mut f = std::fs::File::from(fds.next().expect("a descriptor for each file"));
                f.read_to_end(&mut data).unwrap();
                assert_eq!(data.len(), size.parse::<usize>().unwrap(), "{rel}");
                out.insert(rel.to_string(), data);
            } else {
                out.insert(rel.to_string(), Vec::new());
            }
        }
        assert!(fds.next().is_none(), "no spare descriptors");
    }
    out
}

#[test]
fn descriptors_cross_a_socket_pair_in_one_message() {
    let (a, b) = pair();
    let mut ends = Vec::new();
    let mut keep = Vec::new();
    for i in 0..3 {
        let (r, w) = stream().unwrap();
        let mut w = std::fs::File::from(w);
        w.write_all(format!("stream {i}").as_bytes()).unwrap();
        ends.push(r.as_raw_fd());
        keep.push(r);
    }
    send_message(a.as_raw_fd(), b"FILES\nhello", &ends).unwrap();
    let (body, fds) = recv_message(b.as_raw_fd(), 100).unwrap().unwrap();
    assert_eq!(body, b"FILES\nhello");
    assert_eq!(fds.len(), 3);
    for (i, fd) in fds.into_iter().enumerate() {
        let mut s = String::new();
        std::fs::File::from(fd).read_to_string(&mut s).unwrap();
        assert_eq!(s, format!("stream {i}"));
    }
    send_message(a.as_raw_fd(), b"DONE", &[]).unwrap();
    let (body, fds) = recv_message(b.as_raw_fd(), 100).unwrap().unwrap();
    assert_eq!((body.as_slice(), fds.len()), (&b"DONE"[..], 0));
    drop(a);
    assert!(recv_message(b.as_raw_fd(), 100).unwrap().is_none());
}

#[test]
fn the_most_a_message_may_carry_is_253() {
    let (a, _b) = pair();
    let keep: Vec<(OwnedFd, OwnedFd)> = (0..254).map(|_| stream().unwrap()).collect();
    let raw: Vec<RawFd> = keep.iter().map(|(r, _)| r.as_raw_fd()).collect();
    assert!(send_message(a.as_raw_fd(), b"x", &raw[..253]).is_ok());
    assert!(send_message(a.as_raw_fd(), b"x", &raw).is_err());
}

#[test]
fn a_message_that_lies_about_its_length_or_is_too_long_is_refused() {
    let (a, b) = pair();
    // SAFETY: plain sends of bytes that live through the calls.
    unsafe {
        let bad = [0u8, 0, 0, 9, b'L', b'I', b'S', b'T'];
        libc::send(a.as_raw_fd(), bad.as_ptr() as *const libc::c_void, bad.len(), 0);
    }
    assert!(recv_message(b.as_raw_fd(), 64).is_err());
    send_message(a.as_raw_fd(), &[b'x'; 200], &[]).unwrap();
    assert!(recv_message(b.as_raw_fd(), 64).is_err());
}

#[test]
fn a_plugin_reads_what_was_copied_through_streams_it_cannot_write_to() {
    let f = fake("serve");
    std::fs::create_dir_all(f.home.join("Folder/sub")).unwrap();
    std::fs::write(f.home.join("a.txt"), b"hello there").unwrap();
    std::fs::write(f.home.join("empty"), b"").unwrap();
    std::fs::write(f.home.join("Folder/one"), b"12").unwrap();
    std::fs::write(f.home.join("Folder/sub/two"), b"123").unwrap();
    *f.kinds.lock().unwrap() = uris(&[f.home.join("a.txt"), f.home.join("empty"), f.home.join("Folder")]);
    let s = serve_of(&f);
    let (mine, theirs) = pair();
    std::thread::scope(|sc| {
        sc.spawn(|| serve_connection(theirs, &s));
        let raw = mine.as_raw_fd();
        let a = ask(raw);
        assert_eq!(a.last, "DONE\tcopy\t4\t16\t0");
        let (_, fds) = &a.lists[0];
        for fd in fds {
            // SAFETY: fstat on a descriptor held open.
            let mut st: libc::stat = unsafe { std::mem::zeroed() };
            // SAFETY: fstat on that descriptor into the stat just made.
            assert_eq!(unsafe { libc::fstat(fd.as_raw_fd(), &mut st) }, 0);
            assert_eq!(st.st_mode & libc::S_IFMT, libc::S_IFIFO, "never a real file");
            // SAFETY: a one-byte write to a descriptor held open.
            let w = unsafe { libc::write(fd.as_raw_fd(), b"x".as_ptr() as *const libc::c_void, 1) };
            assert_eq!(w, -1, "it cannot be written to");
            // SAFETY: lseek on a descriptor held open.
            assert_eq!(unsafe { libc::lseek(fd.as_raw_fd(), 0, libc::SEEK_SET) }, -1);
        }
        let got = contents(a);
        assert_eq!(got["a.txt"], b"hello there");
        assert_eq!(got["empty"], b"");
        assert_eq!(got["Folder"], b"");
        assert_eq!(got["Folder/one"], b"12");
        assert_eq!(got["Folder/sub/two"], b"123");
        assert_eq!(got.len(), 6);
        drop(mine);
    });
    assert_eq!(f.reads.load(Ordering::SeqCst), 1, "the ring is told once for the one read");
    let _ = std::fs::remove_dir_all(&f.home);
}

#[test]
fn a_second_request_within_two_seconds_is_turned_away() {
    let f = fake("rate");
    std::fs::write(f.home.join("a.txt"), b"x").unwrap();
    *f.kinds.lock().unwrap() = uris(&[f.home.join("a.txt")]);
    let s = serve_of(&f);
    let (mine, theirs) = pair();
    std::thread::scope(|sc| {
        sc.spawn(|| serve_connection(theirs, &s));
        let raw = mine.as_raw_fd();
        assert!(ask(raw).last.starts_with("DONE"));
        for _ in 0..5 {
            assert_eq!(ask(raw).last, "ERR\tslow_down");
        }
        drop(mine);
    });
    assert_eq!(f.reads.load(Ordering::SeqCst), 1, "a refused ask opens and shows nothing");
    let _ = std::fs::remove_dir_all(&f.home);
}

#[test]
fn nothing_is_given_for_an_old_copy_or_a_clipboard_that_has_changed() {
    let f = fake("recent");
    std::fs::write(f.home.join("a.txt"), b"x").unwrap();
    *f.kinds.lock().unwrap() = uris(&[f.home.join("a.txt")]);
    let s = serve_of(&f);
    let (mine, theirs) = pair();
    std::thread::scope(|sc| {
        sc.spawn(|| serve_connection(theirs, &s));
        let raw = mine.as_raw_fd();
        *f.copied_at.lock().unwrap() = None;
        assert_eq!(ask(raw).last, "ERR\ttoo_old");
        *s.gate.lock().unwrap() = Gate::new();
        *f.copied_at.lock().unwrap() = Instant::now().checked_sub(Duration::from_secs(121));
        assert_eq!(ask(raw).last, "ERR\ttoo_old");
        *s.gate.lock().unwrap() = Gate::new();
        *f.copied_at.lock().unwrap() = Some(Instant::now());
        *f.kinds.lock().unwrap() = Vec::new();
        assert_eq!(ask(raw).last, "ERR\tno_files");
        *s.gate.lock().unwrap() = Gate::new();
        *f.kinds.lock().unwrap() = vec![("text/plain".into(), format!("file://{}", f.home.join("a.txt").display()).into_bytes())];
        assert_eq!(ask(raw).last, "ERR\tno_files");
        *s.gate.lock().unwrap() = Gate::new();
        *f.kinds.lock().unwrap() = uris(&[f.home.join("a.txt")]);
        assert!(ask(raw).last.starts_with("DONE"));
        drop(mine);
    });
    let _ = std::fs::remove_dir_all(&f.home);
}

#[test]
fn a_clipboard_that_names_other_paths_gives_none_of_them() {
    let f = fake("probe");
    let outside = scratch("probe-out");
    std::fs::write(outside.join("secret"), b"top secret").unwrap();
    std::fs::create_dir_all(f.home.join(".ssh")).unwrap();
    std::fs::write(f.home.join(".ssh/id"), b"key").unwrap();
    symlink(&outside, f.home.join("Link")).unwrap();
    *f.kinds.lock().unwrap() = uris(&[
        PathBuf::from("/etc/passwd"), f.home.join(".ssh/id"), f.home.join("Link/secret"), outside.join("secret"),
    ]);
    let s = serve_of(&f);
    let (mine, theirs) = pair();
    std::thread::scope(|sc| {
        sc.spawn(|| serve_connection(theirs, &s));
        let raw = mine.as_raw_fd();
        let a = ask(raw);
        assert_eq!(a.last, "DONE\tcopy\t0\t0\t4", "{}", a.last);
        assert!(a.lists.is_empty());
        drop(mine);
    });
    let _ = std::fs::remove_dir_all(&f.home);
    let _ = std::fs::remove_dir_all(&outside);
}

#[test]
fn withdrawing_the_permission_ends_the_conversation() {
    let f = fake("withdraw");
    std::fs::write(f.home.join("a.txt"), b"x").unwrap();
    *f.kinds.lock().unwrap() = uris(&[f.home.join("a.txt")]);
    let s = serve_of(&f);
    let (mine, theirs) = pair();
    std::thread::scope(|sc| {
        sc.spawn(|| serve_connection(theirs, &s));
        let raw = mine.as_raw_fd();
        assert!(ask(raw).last.starts_with("DONE"));
        f.allowed.store(false, Ordering::SeqCst);
        *s.gate.lock().unwrap() = Gate::new();
        let a = ask(raw);
        assert_eq!(a.last, "ERR\tnot_allowed");
        assert!(a.lists.is_empty());
        assert!(recv_message(raw, 100).unwrap().is_none());
        drop(mine);
    });
    let _ = std::fs::remove_dir_all(&f.home);
}

#[test]
fn a_request_crossing_the_refusal_still_gets_its_answer() {
    let f = fake("crossing");
    let s = serve_of(&f);
    for extra in [0, 1, 5] {
        let (mine, theirs) = pair();
        f.allowed.store(false, Ordering::SeqCst);
        for _ in 0..=extra {
            send_message(mine.as_raw_fd(), b"LIST", &[]).unwrap();
        }
        let started = Instant::now();
        std::thread::scope(|sc| {
            sc.spawn(|| serve_connection(theirs, &s));
            let (body, fds) = recv_message(mine.as_raw_fd(), 100).expect("no reset").expect("an answer");
            assert_eq!(body, b"ERR\tnot_allowed");
            assert!(fds.is_empty());
            assert!(recv_message(mine.as_raw_fd(), 100).expect("no reset").is_none());
            let _ = send_message(mine.as_raw_fd(), b"LIST", &[]);
            drop(mine);
        });
        assert!(started.elapsed() < PARTING, "the plugin closing its end ends the wait at once");
    }
    let (mine, theirs) = pair();
    let started = Instant::now();
    serve_connection(theirs, &s);
    assert!(started.elapsed() >= PARTING && started.elapsed() < PARTING + SEND_FOR, "{:?}", started.elapsed());
    assert_eq!(recv_message(mine.as_raw_fd(), 100).unwrap().unwrap().0, b"ERR\tnot_allowed");
    let _ = std::fs::remove_dir_all(&f.home);
}

#[test]
fn a_plugin_that_passes_descriptors_or_other_words_is_cut_off() {
    let f = fake("abuse");
    std::fs::write(f.home.join("a.txt"), b"x").unwrap();
    *f.kinds.lock().unwrap() = uris(&[f.home.join("a.txt")]);
    let s = serve_of(&f);
    let (mine, theirs) = pair();
    let (r, w) = stream().unwrap();
    std::thread::scope(|sc| {
        sc.spawn(|| serve_connection(theirs, &s));
        send_message(mine.as_raw_fd(), b"LIST", &[r.as_raw_fd()]).unwrap();
        drop(r);
        let (body, _) = recv_message(mine.as_raw_fd(), 100).unwrap().unwrap();
        assert_eq!(body, b"ERR\tbad_request");
        assert!(recv_message(mine.as_raw_fd(), 100).unwrap().is_none());
        drop(mine);
    });
    let mut w = std::fs::File::from(w);
    assert_eq!(w.write(b"x").unwrap_err().kind(), io::ErrorKind::BrokenPipe, "Trindle kept no copy");
    for body in [&b"list"[..], b"LIST ", b"GET /etc/passwd", b"", &[b'L'; 200][..]] {
        let (mine, theirs) = pair();
        std::thread::scope(|sc| {
            sc.spawn(|| serve_connection(theirs, &s));
            let _ = send_message(mine.as_raw_fd(), body, &[]);
            let got = recv_message(mine.as_raw_fd(), 100);
            assert!(matches!(&got, Ok(Some((b, _))) if b == b"ERR\tbad_request") || matches!(got, Ok(None)), "{body:?}");
            drop(mine);
        });
    }
    let _ = std::fs::remove_dir_all(&f.home);
}

#[test]
fn six_hundred_files_come_in_batches_of_at_most_250() {
    raise_open_files();
    let f = fake("batches");
    std::fs::create_dir_all(f.home.join("Many")).unwrap();
    for i in 0..600 {
        std::fs::write(f.home.join(format!("Many/f{i:03}")), format!("file {i}")).unwrap();
    }
    *f.kinds.lock().unwrap() = uris(&[f.home.join("Many")]);
    let s = serve_of(&f);
    let (mine, theirs) = pair();
    std::thread::scope(|sc| {
        sc.spawn(|| serve_connection(theirs, &s));
        let a = ask(mine.as_raw_fd());
        assert_eq!(a.last, "DONE\tcopy\t600\t4690\t0");
        let sizes: Vec<usize> = a.lists.iter().map(|(t, _)| t.lines().count() - 1).collect();
        assert!(sizes.iter().all(|n| *n <= 250) && sizes.len() >= 3 && sizes.iter().sum::<usize>() == 601, "{sizes:?}");
        for (t, fds) in &a.lists {
            assert_eq!(fds.len(), t.lines().filter(|l| l.starts_with("F\t")).count());
            assert!(fds.len() <= 250);
        }
        let got = contents(a);
        assert_eq!(got.len(), 601);
        assert_eq!(got["Many/f599"], b"file 599");
        drop(mine);
    });
    let _ = std::fs::remove_dir_all(&f.home);
}

#[test]
fn more_big_files_than_may_be_in_hand_are_sent_as_the_plugin_reads_them() {
    raise_open_files();
    let f = fake("busy");
    std::fs::create_dir_all(f.home.join("Big")).unwrap();
    for i in 0..150 {
        std::fs::write(f.home.join(format!("Big/b{i:03}")), vec![(i % 200) as u8; 300_000]).unwrap();
    }
    *f.kinds.lock().unwrap() = uris(&[f.home.join("Big")]);
    let s = serve_of(&f);
    let (mine, theirs) = pair();
    std::thread::scope(|sc| {
        sc.spawn(|| serve_connection(theirs, &s));
        let raw = mine.as_raw_fd();
        send_message(raw, b"LIST", &[]).unwrap();
        let mut seen = 0;
        let started = Instant::now();
        loop {
            let (body, fds) = recv_message(raw, 1 << 20).unwrap().unwrap();
            let text = String::from_utf8(body).unwrap();
            if !text.starts_with("FILES\n") {
                assert_eq!(text, "DONE\tcopy\t150\t45000000\t0");
                break;
            }
            assert!(fds.len() <= BUSY + 1, "{} descriptors", fds.len());
            for (fd, line) in fds.into_iter().zip(text.lines().skip(1).filter(|l| l.starts_with("F\t"))) {
                let mut data = Vec::new();
                std::fs::File::from(fd).read_to_end(&mut data).unwrap();
                assert_eq!(data.len(), 300_000, "{line}");
                seen += 1;
            }
        }
        assert_eq!(seen, 150);
        assert!(started.elapsed() < Duration::from_secs(30));
        drop(mine);
    });
    let _ = std::fs::remove_dir_all(&f.home);
}

#[test]
fn a_big_file_arrives_whole_and_no_longer_than_it_was_listed() {
    let f = fake("big");
    let data: Vec<u8> = (0..5_000_000u32).map(|i| (i % 251) as u8).collect();
    std::fs::write(f.home.join("big.bin"), &data).unwrap();
    *f.kinds.lock().unwrap() = uris(&[f.home.join("big.bin")]);
    let s = serve_of(&f);
    let (mine, theirs) = pair();
    std::thread::scope(|sc| {
        sc.spawn(|| serve_connection(theirs, &s));
        let a = ask(mine.as_raw_fd());
        assert_eq!(a.last, "DONE\tcopy\t1\t5000000\t0");
        let got = contents(a);
        assert!(got["big.bin"] == data);
        drop(mine);
    });
    let (r, w) = stream().unwrap();
    let src = scratch("grow").join("g");
    std::fs::write(&src, vec![7u8; 1000]).unwrap();
    let pump = Pump::new();
    pump.submit(std::fs::File::open(&src).unwrap().into(), w, 100);
    let mut got = Vec::new();
    std::fs::File::from(r).read_to_end(&mut got).unwrap();
    assert_eq!(got.len(), 100);
    let _ = std::fs::remove_dir_all(&f.home);
}

#[test]
fn a_plugin_that_never_reads_holds_up_nothing_for_ever() {
    let pump = Pump::new();
    let src = scratch("gone").join("g");
    std::fs::write(&src, vec![1u8; 1_000_000]).unwrap();
    for _ in 0..5 {
        let (r, w) = stream().unwrap();
        pump.submit(std::fs::File::open(&src).unwrap().into(), w, 1_000_000);
        drop(r);
    }
    let dead = AtomicBool::new(false);
    assert!(pump.wait_below(1, Duration::from_secs(5), &dead), "{} still busy", pump.busy());
    let (r, w) = stream().unwrap();
    pump.submit(std::fs::File::open(&src).unwrap().into(), w, 1_000_000);
    dead.store(true, Ordering::SeqCst);
    assert!(!pump.wait_below(1, Duration::from_secs(5), &dead));
    drop(r);
}

#[test]
fn the_socket_is_made_and_a_stranger_on_it_is_refused_by_name_only() {
    let dir = scratch("socket");
    let path = dir.join("copied");
    let lis = listener(&path).unwrap();
    let client = std::os::unix::net::UnixStream::connect(&path);
    assert!(client.is_err());
    // SAFETY: a socket and a connect on the path made above.
    let raw = unsafe { libc::socket(libc::AF_UNIX, libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC, 0) };
    let me = owned(raw);
    // SAFETY: an all-zero sockaddr_un is a valid empty one.
    let mut addr: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    addr.sun_family = libc::AF_UNIX as _;
    for (i, b) in path.to_string_lossy().bytes().enumerate() {
        addr.sun_path[i] = b as _;
    }
    // SAFETY: connect on the socket made above, with the address just filled in.
    let r = unsafe { libc::connect(me.as_raw_fd(), &addr as *const _ as *const libc::sockaddr, std::mem::size_of::<libc::sockaddr_un>() as u32) };
    assert_eq!(r, 0);
    // SAFETY: accept4 on the socket made above.
    let c = unsafe { libc::accept4(lis.as_raw_fd(), std::ptr::null_mut(), std::ptr::null_mut(), libc::SOCK_CLOEXEC) };
    assert!(c >= 0);
    assert!(from_me(owned(c).as_raw_fd()), "this person's own program is let in");
    let _ = std::fs::remove_dir_all(&dir);
}

fn connect_to(path: &str) -> io::Result<OwnedFd> {
    // SAFETY: socket only makes a descriptor.
    let raw = unsafe { libc::socket(libc::AF_UNIX, libc::SOCK_SEQPACKET | libc::SOCK_CLOEXEC, 0) };
    cvt(raw as isize)?;
    let fd = owned(raw);
    // SAFETY: an all-zero sockaddr_un is a valid empty one.
    let mut addr: libc::sockaddr_un = unsafe { std::mem::zeroed() };
    addr.sun_family = libc::AF_UNIX as _;
    for (i, b) in path.bytes().enumerate().take(addr.sun_path.len() - 1) {
        addr.sun_path[i] = b as _;
    }
    // SAFETY: a filled sockaddr_un of the size given and a socket held here.
    let r = unsafe { libc::connect(fd.as_raw_fd(), &addr as *const _ as *const libc::sockaddr, std::mem::size_of::<libc::sockaddr_un>() as u32) };
    cvt(r as isize)?;
    Ok(fd)
}

#[test]
#[ignore = "not a test of its own: it runs inside a box started by another test"]
fn client_in_the_box() {
    let Ok(home) = std::env::var("HOME") else { return };
    let home = PathBuf::from(home);
    let Ok(mode) = std::fs::read_to_string(home.join("mode")) else { return };
    let out = std::cell::RefCell::new(String::new());
    let say = |l: String| {
        let mut o = out.borrow_mut();
        o.push_str(&l);
        o.push('\n');
    };
    let wait_for = |name: &str, secs: u64| {
        let until = Instant::now() + Duration::from_secs(secs);
        while !home.join(name).exists() && Instant::now() < until {
            std::thread::sleep(Duration::from_millis(200));
        }
        home.join(name).exists()
    };
    let runtime = std::env::var("XDG_RUNTIME_DIR").unwrap_or_default();
    if mode.trim() == "absent" {
        absent_report(&runtime, &say);
        std::fs::write(home.join("result.txt"), out.borrow().as_str()).unwrap();
        return;
    }
    assert!(wait_for("go", 90), "no go");
    let path = std::env::var("TRINDLE_COPIED_SOCKET").expect("the socket is named");
    let sock = connect_to(&path).expect("connects");
    let raw = sock.as_raw_fd();
    say(format!("socket {path}"));
    send_message(raw, b"LIST", &[]).unwrap();
    let mut probed = false;
    loop {
        let (body, fds) = recv_message(raw, 1 << 20).unwrap().expect("an answer");
        let text = String::from_utf8(body).unwrap();
        if !text.starts_with("FILES\n") {
            say(format!("last {}", text.replace('\t', " ")));
            break;
        }
        if !probed && !fds.is_empty() {
            probed = true;
            probe_the_descriptor(fds[0].as_raw_fd(), &say);
        }
        read_the_listed_files(&text, fds, &say);
    }
    send_message(raw, b"LIST", &[]).unwrap();
    let (body, _) = recv_message(raw, 1 << 20).unwrap().expect("an answer");
    say(format!("again {}", String::from_utf8_lossy(&body).replace('\t', " ")));
    std::fs::write(home.join("result.txt"), out.borrow().as_str()).unwrap();
    if wait_for("go2", 60) {
        after_withdrawal(raw, &path, &say);
    }
    std::fs::write(home.join("result.txt"), out.borrow().as_str()).unwrap();
}

fn absent_report(runtime: &str, say: &dyn Fn(String)) {
    say(format!("env {}", std::env::var("TRINDLE_COPIED_SOCKET").unwrap_or_else(|_| "unset".into())));
    let at = format!("{runtime}/trindle-copied");
    say(format!("exists {}", Path::new(&at).exists()));
    say(format!("connect {}", match connect_to(&at) { Ok(_) => "ok".to_string(), Err(e) => e.to_string() }));
    let mut names: Vec<String> = std::fs::read_dir(runtime).into_iter().flatten().flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned()).collect();
    names.sort();
    say(format!("runtime {}", names.join(" ")));
}

fn probe_the_descriptor(n: RawFd, say: &dyn Fn(String)) {
    let c = CString::new(format!("/proc/self/fd/{n}")).unwrap();
    // SAFETY: open of a path made above.
    let w = unsafe { libc::open(c.as_ptr(), libc::O_WRONLY | libc::O_APPEND) };
    say(format!("reopen-for-write {}", if w >= 0 { "opened (the other end of the same pipe, not the file)".to_string() } else { io::Error::last_os_error().to_string() }));
    if w >= 0 {
        // SAFETY: the closing of a descriptor made just above and
        // used nowhere else (held open, it would keep its own
        // stream from ever ending).
        unsafe { libc::close(w) };
    }
    // SAFETY: fchmod on a descriptor held open.
    let m = unsafe { libc::fchmod(n, 0o777) };
    say(format!("fchmod {}", if m == 0 { "WORKED".to_string() } else { io::Error::last_os_error().to_string() }));
    // SAFETY: a one-byte write to a descriptor held open.
    let wr = unsafe { libc::write(n, b"x".as_ptr() as *const libc::c_void, 1) };
    say(format!("write {}", if wr >= 0 { "WORKED".to_string() } else { io::Error::last_os_error().to_string() }));
    // SAFETY: lseek on a descriptor held open.
    let sk = unsafe { libc::lseek(n, 0, libc::SEEK_SET) };
    say(format!("seek {}", if sk >= 0 { "WORKED".to_string() } else { io::Error::last_os_error().to_string() }));
    // SAFETY: flock on a descriptor held open.
    let fl = unsafe { libc::flock(n, libc::LOCK_SH | libc::LOCK_NB) };
    say(format!("flock {}", if fl == 0 { "worked (a pipe takes it, it locks nothing)".to_string() } else { io::Error::last_os_error().to_string() }));
}

fn read_the_listed_files(text: &str, fds: Vec<OwnedFd>, say: &dyn Fn(String)) {
    let mut fds = fds.into_iter();
    for line in text.lines().skip(1) {
        let mut part = line.splitn(3, '\t');
        let (kind, size, rel) = (part.next().unwrap(), part.next().unwrap(), part.next().unwrap());
        if kind == "F" {
            let mut data = Vec::new();
            std::fs::File::from(fds.next().unwrap()).read_to_end(&mut data).unwrap();
            let shown: String = String::from_utf8_lossy(&data).chars().take(40).collect();
            say(format!("file {rel} listed {size} read {} {:?}", data.len(), shown));
        } else {
            say(format!("folder {rel}"));
        }
    }
}

fn after_withdrawal(raw: RawFd, path: &str, say: &dyn Fn(String)) {
    std::thread::sleep(Duration::from_secs(3));
    let _ = send_message(raw, b"LIST", &[]);
    match recv_message(raw, 1 << 20) {
        Ok(Some((body, _))) => say(format!("after-withdrawal {}", String::from_utf8_lossy(&body).replace('\t', " "))),
        Ok(None) => say("after-withdrawal closed".to_string()),
        Err(e) => say(format!("after-withdrawal error {e}")),
    }
    let conn = connect_to(path);
    say(format!("after-withdrawal connect {}", match conn { Ok(_) => "ok".to_string(), Err(e) => e.to_string() }));
}

#[test]
fn a_long_folder_name_still_gets_its_socket() {
    let dir = scratch(&"long".repeat(40));
    let lis = listener(&dir.join("copied")).unwrap();
    assert!(dir.join("copied").exists());
    drop(lis);
    let _ = std::fs::remove_dir_all(&dir);
}
