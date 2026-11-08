//! Which ordinary programs (outside any box) reach the network or have the
//! person's files open, read from the kernel's own tables with no special
//! rights, and the list of those the person said are fine. It sees what is open when it looks; it
//! tells, and stops nothing.

use std::fmt::Write as _;
use std::collections::{BTreeMap, HashSet};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::path::{Path, PathBuf};

use super::files;

pub const NETWORK: &str = "outside_network";
pub const FILES: &str = "outside_files";

const MOST_FINE: usize = 500;
const LONGEST_KEY: usize = 300;

pub fn network_on() -> bool {
    files::setting(NETWORK) != "no"
}

pub fn files_on() -> bool {
    files::setting(FILES) != "no"
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Network,
    Files,
}

impl Kind {
    pub fn word(self) -> &'static str {
        match self {
            Kind::Network => "net",
            Kind::Files => "files",
        }
    }

    pub fn from_word(w: &str) -> Option<Kind> {
        [Kind::Network, Kind::Files].into_iter().find(|k| k.word() == w)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Seen {

    pub key: String,
    pub name: String,

    pub exe: String,
    pub network: bool,
    pub files: bool,
}

impl Seen {
    pub fn did(&self, kind: Kind) -> bool {
        match kind {
            Kind::Network => self.network,
            Kind::Files => self.files,
        }
    }
}

#[derive(Default)]
pub struct Tables {
    pub tcp: String,
    pub tcp6: String,
    pub udp: String,
    pub udp6: String,
}

impl Tables {
    pub fn read(net: &Path) -> Tables {
        let get = |n: &str| std::fs::read_to_string(net.join(n)).unwrap_or_default();
        Tables { tcp: get("tcp"), tcp6: get("tcp6"), udp: get("udp"), udp6: get("udp6") }
    }
}

fn address(text: &str) -> Option<(IpAddr, u16)> {
    let (ip, port) = text.split_once(':')?;
    let port = u16::from_str_radix(port, 16).ok()?;
    let word = |i: usize| u32::from_str_radix(ip.get(i * 8..i * 8 + 8)?, 16).ok();
    match ip.len() {
        8 => Some((IpAddr::V4(Ipv4Addr::from(word(0)?.to_le_bytes())), port)),
        32 => {
            let mut bytes = [0u8; 16];
            for i in 0..4 {
                bytes[i * 4..i * 4 + 4].copy_from_slice(&word(i)?.to_le_bytes());
            }
            Some((IpAddr::V6(Ipv6Addr::from(bytes)), port))
        }
        _ => None,
    }
}

fn elsewhere(ip: IpAddr) -> bool {
    let ip = match ip {
        IpAddr::V6(v6) => v6.to_ipv4_mapped().map_or(IpAddr::V6(v6), IpAddr::V4),
        v4 => v4,
    };
    !ip.is_loopback() && !ip.is_unspecified()
}

fn rows(text: &str) -> impl Iterator<Item = (&str, u16, Option<(IpAddr, u16)>, u64)> {
    text.lines().skip(1).filter_map(|line| {
        let f: Vec<&str> = line.split_whitespace().collect();
        let local = address(f.get(1)?)?;
        let inode: u64 = f.get(9)?.parse().ok()?;
        Some((*f.get(3)?, local.1, address(f.get(2)?), inode))
    })
}

pub fn reaching(t: &Tables) -> HashSet<u64> {
    let listening: HashSet<u16> = [&t.tcp, &t.tcp6].into_iter().flat_map(|x| rows(x)).filter(|r| r.0 == "0A").map(|r| r.1).collect();
    let mut out = HashSet::new();
    for text in [&t.tcp, &t.tcp6] {
        for (state, port, remote, inode) in rows(text) {
            if matches!(state, "01" | "02") && !listening.contains(&port) && inode != 0 && remote.is_some_and(|r| elsewhere(r.0)) {
                out.insert(inode);
            }
        }
    }
    for text in [&t.udp, &t.udp6] {
        for (state, _, remote, inode) in rows(text) {
            if state == "01" && inode != 0 && remote.is_some_and(|r| elsewhere(r.0)) {
                out.insert(inode);
            }
        }
    }
    out
}

const RUNNERS: &[&str] = &["python", "perl", "ruby", "node", "nodejs", "java", "bash", "sh", "dash", "zsh", "php", "lua", "deno", "bun", "tclsh", "wish"];

fn plain(text: &str) -> String {
    text.chars().map(|c| if c.is_control() { ' ' } else { c }).take(LONGEST_KEY).collect()
}

pub fn identity(exe: &Path, cmdline: &[u8]) -> (String, String, String) {
    let shown = exe.to_string_lossy();
    let path = shown.strip_suffix(" (deleted)").unwrap_or(&shown).to_string();
    let base = Path::new(&path).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| path.clone());
    let runner = base.trim_end_matches(|c: char| c.is_ascii_digit() || c == '.');
    let script = cmdline.split(|b| *b == 0).skip(1).map(String::from_utf8_lossy).find(|a| !a.is_empty() && !a.starts_with('-'));
    match script.filter(|_| RUNNERS.contains(&runner)) {
        Some(script) => {
            let name = Path::new(script.as_ref()).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| script.to_string());
            (plain(&format!("{path} {script}")), plain(&name), base)
        }
        None => (plain(&path), plain(&base), base),
    }
}

fn trindle_own(exe: &Path) -> bool {
    let named = exe.file_name().is_some_and(|n| n.to_string_lossy().starts_with("trindle"));

    let flatpak = exe == Path::new("/usr/bin/flatpak") || exe.starts_with("/usr/libexec") && exe.file_name().is_some_and(|n| n.to_string_lossy().starts_with("flatpak"));
    flatpak
        || (named && exe.parent().is_some_and(|d| d == Path::new("/usr/bin") || d == Path::new("/usr/sbin")))
        || ["/usr/lib/trindle", "/usr/libexec/trindle", "/usr/share/trindle"].iter().any(|p| exe.starts_with(p))
}

pub fn personal_folders(home: &Path) -> Vec<PathBuf> {
    crate::store::boxed::walls::OWN_FOLDERS.iter().map(|f| home.join(f)).collect()
}

pub fn scan(proc_root: &Path, sockets: &HashSet<u64>, folders: &[PathBuf], mine: &str, want_net: bool, want_files: bool) -> Vec<Seen> {
    let mut found: BTreeMap<String, Seen> = BTreeMap::new();
    let Ok(dir) = std::fs::read_dir(proc_root) else { return Vec::new() };
    for e in dir.flatten() {
        let at = e.path();
        if !e.file_name().to_str().is_some_and(|n| n.bytes().all(|b| b.is_ascii_digit())) {
            continue;
        }
        if std::fs::read_link(at.join("ns/mnt")).ok().as_deref().and_then(Path::to_str) != Some(mine) {
            continue;
        }
        let Ok(exe) = std::fs::read_link(at.join("exe")) else { continue };
        if trindle_own(&exe) {
            continue;
        }
        let Ok(fds) = std::fs::read_dir(at.join("fd")) else { continue };
        let (mut network, mut files) = (false, false);
        for fd in fds.flatten() {
            let Ok(to) = std::fs::read_link(fd.path()) else { continue };
            if want_net && !network {
                network = to.to_str().and_then(|t| t.strip_prefix("socket:[")?.strip_suffix(']')?.parse::<u64>().ok()).is_some_and(|i| sockets.contains(&i));
            }
            if want_files && !files {
                files = folders.iter().any(|f| to.starts_with(f));
            }
            if network == want_net && files == want_files {
                break;
            }
        }
        if !network && !files {
            continue;
        }
        let cmdline = std::fs::read(at.join("cmdline")).unwrap_or_default();
        let (key, name, exe) = identity(&exe, &cmdline);
        let seen = found.entry(key.clone()).or_insert(Seen { key, name, exe, network: false, files: false });
        seen.network |= network;
        seen.files |= files;
    }
    let mut out: Vec<Seen> = found.into_values().collect();
    out.sort_by(|a, b| (a.name.to_lowercase(), &a.key).cmp(&(b.name.to_lowercase(), &b.key)));
    out
}

pub fn my_room(proc_root: &Path) -> Option<String> {
    std::fs::read_link(proc_root.join("self/ns/mnt")).ok()?.to_str().map(String::from)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fine {
    pub kind: Kind,
    pub key: String,
    pub name: String,
}

pub fn fine_path() -> PathBuf {
    files::config_path("outside-fine")
}

fn parse_fine(text: &str) -> Vec<Fine> {
    text.lines()
        .filter_map(|l| {
            let mut p = l.splitn(3, '\t');
            let kind = Kind::from_word(p.next()?)?;
            let key = p.next().filter(|k| !k.is_empty())?.to_string();
            Some(Fine { kind, key, name: p.next().unwrap_or("").to_string() })
        })
        .take(MOST_FINE)
        .collect()
}

fn write_fine(at: &Path, list: &[Fine]) -> std::io::Result<()> {
    let text: String = list.iter().fold(String::new(), |mut s, f| { let _ = writeln!(s, "{}\t{}\t{}", f.kind.word(), plain(&f.key), plain(&f.name)); s });
    if let Some(up) = at.parent() {
        std::fs::create_dir_all(up)?;
    }
    files::write_text(at, &text)
}

pub fn fine_in(at: &Path) -> Vec<Fine> {
    files::read_small(at).map(|t| parse_fine(&t)).unwrap_or_default()
}

pub fn fine() -> Vec<Fine> {
    fine_in(&fine_path())
}

pub fn make_fine_in(at: &Path, kind: Kind, programs: &[(String, String)]) -> std::io::Result<()> {
    let mut list = match files::read_small(at) {
        Ok(t) => parse_fine(&t),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(e) => return Err(e),
    };
    for (key, name) in programs {
        if !list.iter().any(|f| f.kind == kind && &f.key == key) && list.len() < MOST_FINE {
            list.push(Fine { kind, key: plain(key), name: plain(name) });
        }
    }
    write_fine(at, &list)
}

pub fn forget_in(at: &Path, kind: Kind, key: &str) -> std::io::Result<()> {
    let mut list = files::read_small(at).map(|t| parse_fine(&t))?;
    list.retain(|f| !(f.kind == kind && f.key == key));
    write_fine(at, &list)
}

pub fn to_say(seen: &[Seen], kind: Kind, fine: &[Fine], said: &HashSet<(Kind, String)>) -> Vec<Seen> {
    seen.iter()
        .filter(|s| s.did(kind) && !fine.iter().any(|f| f.kind == kind && f.key == s.key) && !said.contains(&(kind, s.key.clone())))
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TCP: &str = "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode\n\
   0: 00000000:1F90 00000000:0000 0A 00000000:00000000 00:00000000 00000000  1000        0 1001 1 0 100 0 0 10 0\n\
   1: 0F02A8C0:C350 22D8B85D:01BB 01 00000000:00000000 00:00000000 00000000  1000        0 2002 1 0 100 0 0 10 0\n\
   2: 0100007F:C351 0100007F:1F90 01 00000000:00000000 00:00000000 00000000  1000        0 3003 1 0 100 0 0 10 0\n\
   3: 0F02A8C0:1F90 0A02A8C0:D000 01 00000000:00000000 00:00000000 00000000  1000        0 4004 1 0 100 0 0 10 0\n\
   4: 0F02A8C0:C352 22D8B85D:01BB 06 00000000:00000000 00:00000000 00000000     0        0 0 1 0 100 0 0 10 0\n\
   5: 0F02A8C0:C353 22D8B85D:01BB 02 00000000:00000000 00:00000000 00000000  1000        0 5005 1 0 100 0 0 10 0\n";

    #[test]
    fn flatpak_in_its_own_places_is_not_asked_about_but_a_copy_elsewhere_is() {
        assert!(trindle_own(Path::new("/usr/bin/flatpak")));
        assert!(trindle_own(Path::new("/usr/libexec/flatpak-portal")));
        assert!(!trindle_own(Path::new("/home/john/bin/flatpak")));
        assert!(!trindle_own(Path::new("/usr/bin/curl")));
    }

    #[test]
    fn a_connection_to_another_computer_is_seen_and_nothing_else() {
        let got = reaching(&Tables { tcp: TCP.into(), ..Tables::default() });
        assert!(got.contains(&2002), "an open connection to a website");
        assert!(got.contains(&5005), "one being made");
        assert!(!got.contains(&3003), "this computer talking to itself");
        assert!(!got.contains(&4004), "someone calling in to a port that is listened on");
        assert!(!got.contains(&1001) && !got.contains(&0), "a listener, and a socket that is already closing");
    }

    #[test]
    fn an_address_is_read_the_way_the_kernel_writes_it() {
        assert_eq!(address("0100007F:0035"), Some((IpAddr::V4(Ipv4Addr::LOCALHOST), 53)));
        assert_eq!(address("0F02A8C0:C350").map(|a| a.0), Some(IpAddr::V4(Ipv4Addr::new(192, 168, 2, 15))));
        assert_eq!(address("00000000000000000000000001000000:0050"), Some((IpAddr::V6(Ipv6Addr::LOCALHOST), 80)));
        let mapped = address("0000000000000000FFFF00000F02A8C0:0050").unwrap().0;
        assert_eq!(mapped.to_string(), "::ffff:192.168.2.15");
        assert!(elsewhere(mapped) && !elsewhere(address("0000000000000000FFFF00000100007F:0050").unwrap().0));
        assert!(address("nonsense").is_none() && address("0100007F:zz").is_none() && address("0100:0050").is_none());
    }

    #[test]
    fn udp_counts_only_when_connected_to_another_computer() {
        let udp = "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode ref pointer drops\n\
   0: 0F02A8C0:C350 22D8B85D:01BB 01 00000000:00000000 00:00000000 00000000  1000        0 7001 2 0 0\n\
   1: 0F02A8C0:C351 00000000:0000 07 00000000:00000000 00:00000000 00000000  1000        0 7002 2 0 0\n\
   2: 0100007F:C352 3500007F:0035 01 00000000:00000000 00:00000000 00000000  1000        0 7003 2 0 0\n";
        let got = reaching(&Tables { udp: udp.into(), ..Tables::default() });
        assert_eq!(got, HashSet::from([7001]));
    }

    #[test]
    fn a_script_is_known_by_itself_and_not_by_its_interpreter() {
        let (key, name, exe) = identity(Path::new("/usr/bin/python3.13"), b"python3\0-u\0/home/john/bin/sync.py\0--now\0");
        assert_eq!((key.as_str(), name.as_str(), exe.as_str()), ("/usr/bin/python3.13 /home/john/bin/sync.py", "sync.py", "python3.13"));
        let (key, name, _) = identity(Path::new("/usr/bin/python3"), b"python3\0/home/john/bin/other.py\0");
        assert_ne!(key, identity(Path::new("/usr/bin/python3"), b"python3\0/home/john/bin/sync.py\0").0, "{name}");
        let (key, name, _) = identity(Path::new("/usr/lib/firefox-esr/firefox-esr (deleted)"), b"firefox-esr\0-contentproc\0");
        assert_eq!((key.as_str(), name.as_str()), ("/usr/lib/firefox-esr/firefox-esr", "firefox-esr"));
        let (key, _, _) = identity(Path::new("/usr/bin/bash"), b"bash\0");
        assert_eq!(key, "/usr/bin/bash", "a shell with no script is the shell");
        assert!(!identity(Path::new("/x/a"), b"a\0").0.contains('\t'));
        assert!(identity(Path::new("/usr/bin/node"), format!("node\0{}\0", "a".repeat(900)).as_bytes()).0.len() <= LONGEST_KEY);
    }

    #[test]
    fn only_ordinary_programs_outside_a_box_are_found() {
        let top = std::env::temp_dir().join(format!("trindle-outside-scan-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        let proc = |pid: &str, room: &str, exe: &str, cmd: &[u8], fds: &[&str]| {
            let d = top.join(pid);
            std::fs::create_dir_all(d.join("ns")).unwrap();
            std::fs::create_dir_all(d.join("fd")).unwrap();
            std::os::unix::fs::symlink(room, d.join("ns/mnt")).unwrap();
            std::os::unix::fs::symlink(exe, d.join("exe")).unwrap();
            std::fs::write(d.join("cmdline"), cmd).unwrap();
            for (i, to) in fds.iter().enumerate() {
                std::os::unix::fs::symlink(to, d.join("fd").join(i.to_string())).unwrap();
            }
        };
        proc("10", "mnt:[1]", "/usr/bin/firefox-esr", b"firefox-esr\0", &["socket:[2002]", "/home/john/Documents/a.odt", "pipe:[5]"]);
        proc("11", "mnt:[9]", "/usr/bin/gimp", b"gimp\0", &["socket:[2002]", "/home/john/Pictures/a.png"]);
        proc("12", "mnt:[1]", "/usr/bin/trindle-ring", b"trindle-ring\0", &["socket:[2002]"]);
        proc("13", "mnt:[1]", "/usr/bin/vlc", b"vlc\0", &["/home/john/.config/vlc/x", "socket:[3003]"]);
        proc("14", "mnt:[1]", "/usr/bin/mousepad", b"mousepad\0", &["/home/john/Documents/b.txt"]);
        proc("15", "mnt:[1]", "/home/john/Downloads/trindle-helper", b"trindle-helper\0", &["/home/john/Documents/c.txt"]);
        proc("16", "mnt:[1]", "/usr/lib/trindle/trindle-plugin", b"trindle-plugin\0", &["/home/john/Documents/d.txt"]);
        std::fs::create_dir_all(top.join("net")).unwrap();
        let sockets = HashSet::from([2002]);
        let folders = personal_folders(Path::new("/home/john"));
        let both = scan(&top, &sockets, &folders, "mnt:[1]", true, true);
        let names: Vec<(&str, bool, bool)> = both.iter().map(|s| (s.name.as_str(), s.network, s.files)).collect();
        assert_eq!(names, [("firefox-esr", true, true), ("mousepad", false, true), ("trindle-helper", false, true)]);
        let net = scan(&top, &sockets, &folders, "mnt:[1]", true, false);
        assert_eq!(net.iter().map(|s| (s.name.as_str(), s.files)).collect::<Vec<_>>(), [("firefox-esr", false)]);
        assert!(scan(&top, &sockets, &folders, "mnt:[1]", false, false).is_empty(), "a look that is off is not made");
        let _ = std::fs::remove_dir_all(&top);
    }

    #[test]
    fn what_the_person_said_is_fine_is_kept_once_and_can_be_forgotten() {
        let top = std::env::temp_dir().join(format!("trindle-outside-fine-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        let at = top.join("outside-fine");
        assert!(fine_in(&at).is_empty());
        let one = vec![("/usr/bin/firefox-esr".to_string(), "firefox-esr".to_string())];
        make_fine_in(&at, Kind::Network, &one).unwrap();
        make_fine_in(&at, Kind::Network, &one).unwrap();
        make_fine_in(&at, Kind::Files, &one).unwrap();
        assert_eq!(fine_in(&at).len(), 2, "once for each kind");
        let seen = [Seen { key: one[0].0.clone(), name: "firefox-esr".into(), exe: "firefox-esr".into(), network: true, files: true }];
        let said = HashSet::new();
        assert!(to_say(&seen, Kind::Network, &fine_in(&at), &said).is_empty());
        forget_in(&at, Kind::Network, &one[0].0).unwrap();
        assert_eq!(to_say(&seen, Kind::Network, &fine_in(&at), &said).len(), 1, "forgotten, it is said again");
        assert!(to_say(&seen, Kind::Files, &fine_in(&at), &said).is_empty(), "the other kind is still fine");
        let said = HashSet::from([(Kind::Network, one[0].0.clone())]);
        assert!(to_say(&seen, Kind::Network, &[], &said).is_empty(), "said once in a session");
        std::fs::write(&at, "net\t\tno key\nnonsense\nfiles\t/a\tA\n").unwrap();
        assert_eq!(fine_in(&at), [Fine { kind: Kind::Files, key: "/a".into(), name: "A".into() }], "a bad line is skipped");
        std::fs::remove_dir_all(&top).unwrap();
    }

    #[test]
    fn a_list_that_cannot_be_read_is_not_written_over() {
        let top = std::env::temp_dir().join(format!("trindle-outside-bad-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        std::fs::create_dir_all(&top).unwrap();
        let at = top.join("outside-fine");
        std::fs::write(&at, [0xff, 0xfe, 0x00]).unwrap();
        assert!(make_fine_in(&at, Kind::Network, &[("/a".into(), "a".into())]).is_err());
        assert_eq!(std::fs::read(&at).unwrap(), [0xff, 0xfe, 0x00]);
        std::fs::remove_dir_all(&top).unwrap();
    }
}
