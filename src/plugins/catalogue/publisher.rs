//! Making a catalogue and signing it: for whoever publishes Trindle's
//! plugins, and for the test build. This file is not in the release
//! program (see `mod publisher` in the parent): nobody who has Trindle installed can make, sign or add a
//! plugin or a catalogue. It is built into `trindle-plugin` only with
//! `--features publisher`, or `--features test-catalogue` for a build with
//! its own test key and address.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use ::ring::signature::{Ed25519KeyPair, KeyPair};

use super::*;
use crate::common::copies::copy_new;
use crate::common::files::{read_text, write_text, write_whole};
use crate::common::keyfile::KeyFile;

pub const SUMMARY_MOST: usize = 160;

pub fn stem(name: &str) -> &str {
    ARCHIVES.iter().find_map(|a| name.strip_suffix(a)).unwrap_or(name)
}

pub fn next_number(before: Option<u64>) -> u64 {
    let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
    before.map_or(now, |b| now.max(b.saturating_add(1)))
}

pub fn write(entries: &[Entry], number: u64) -> String {
    use std::fmt::Write as _;
    let mut sorted: Vec<&Entry> = entries.iter().collect();
    sorted.sort_by(|a, b| a.id.cmp(&b.id));
    let mut kf = KeyFile::parse(&format!("# Trindle's plugin catalogue, made by trindle-plugin catalogue.\n\
                                          [Catalogue]\nFormat={FORMAT}\nSerial={number}\n"));
    for e in sorted {
        let g = format!("Plugin {}", e.id);
        kf.set(&g, "Interface", &INTERFACE.to_string());
        kf.set(&g, "Name", &e.name);
        kf.set(&g, "Version", &e.version);
        kf.set(&g, "Summary", &e.summary);
        kf.set(&g, "Permissions", &e.permissions.iter().fold(String::new(), |mut s, p| { let _ = write!(s, "{p};"); s }));
        if let (Some(i), Some(h)) = (&e.icon, &e.icon_sha256) {
            kf.set(&g, "Icon", i);
            kf.set(&g, "IconSHA256", h);
        }
        kf.set(&g, "Archive", &e.archive);
        kf.set(&g, "SHA256", &e.sha256);
    }
    kf.text()
}

pub fn first_sentence(text: &str) -> String {
    let para = text.trim().split("\n\n").next().unwrap_or("").split_whitespace().collect::<Vec<_>>().join(" ");
    let para = para.trim_start_matches('#').trim();
    let line = match para.find(". ") {
        Some(i) => &para[..=i],
        None => para,
    };
    if line.chars().count() <= SUMMARY_MOST {
        return line.to_string();
    }
    let cut: String = line.chars().take(SUMMARY_MOST - 3).collect();
    format!("{}...", cut.rsplit_once(' ').map_or(cut.as_str(), |(a, _)| a).trim_end_matches([',', ';', ':']))
}

fn summary_for(p: &Plugin, beside: &Path, stem: &str, inside: &Path) -> Option<String> {
    if !p.summary.is_empty() {
        return Some(p.summary.clone());
    }
    let mut look = Vec::new();
    for name in [stem, p.id.as_str()] {
        for end in [".txt", ".md", ".description", ".description.txt"] {
            look.push(beside.join(format!("{name}{end}")));
        }
    }
    look.push(beside.join(&p.id).join("description.txt"));
    for name in ["description.txt", "description.md", "DESCRIPTION"] {
        look.push(inside.join(name));
    }
    look.iter().map(read_text).map(|t| first_sentence(&t)).find(|s| !s.is_empty())
}

fn icon_for(p: &Plugin, beside: &Path, stem: &str) -> Option<String> {
    for name in [stem, p.id.as_str()] {
        for end in [".svg", ".png"] {
            if beside.join(format!("{name}{end}")).is_file() {
                return Some(format!("{name}{end}"));
            }
        }
    }
    let (from, _, _) = p.ring.as_ref()?;
    let end = if from.extension().is_some_and(|e| e == "png") { "png" } else { "svg" };
    let name = format!("{}.{end}", p.id);
    copy_new(from, &beside.join(&name)).ok().map(|_| name)
}

pub fn build(folder: &Path) -> Result<(Vec<Entry>, Vec<String>), String> {
    let mut names: Vec<String> = std::fs::read_dir(folder).map_err(|e| format!("{}: {e}", folder.display()))?
        .flatten().filter(|e| e.path().is_file()).filter_map(|e| e.file_name().into_string().ok())
        .filter(|n| is_archive(n) && plain_name(n)).collect();
    names.sort();
    let mut kept: BTreeMap<String, Entry> = BTreeMap::new();
    let mut problems = Vec::new();
    for name in names {
        let archive = folder.join(&name);
        let made = (|| -> Result<Entry, String> {
            let sha256 = sha256_of(&archive).map_err(|e| e.to_string())?;
            let tmp = scratch()?;
            let at = unpack(&archive, &tmp.0)?;
            let p = check(&at).map_err(|problems| problems.join(" "))?;
            let summary = summary_for(&p, folder, stem(&name), &at).ok_or_else(|| {
                format!("it has no summary: give its plugin.ini a Summary= line, or put {}.txt beside it", p.id)
            })?;
            let icon = icon_for(&p, folder, stem(&name));
            let icon_sha256 = icon.as_ref().and_then(|i| sha256_of(&folder.join(i)).ok());
            let icon = icon.filter(|_| icon_sha256.is_some());
            Ok(Entry { id: p.id, name: p.name, version: p.version, summary, permissions: p.asks, icon, icon_sha256, archive: name.clone(), sha256 })
        })();
        match made {
            Ok(e) => {
                if kept.get(&e.id).is_none_or(|old| newer(&e.version, &old.version)) {
                    kept.insert(e.id.clone(), e);
                }
            }
            Err(why) => problems.push(format!("{name} is left out: {why}")),
        }
    }
    Ok((kept.into_values().collect(), problems))
}

pub fn main(folder: &Path) -> i32 {
    let (entries, problems) = match build(folder) {
        Ok(r) => r,
        Err(e) => {
            println!("{e}");
            return 1;
        }
    };
    for p in &problems {
        println!("{p}");
    }
    let index = folder.join(INDEX);
    let number = next_number(number_of(&read_text(&index)));
    if let Err(e) = write_text(&index, &write(&entries, number)) {
        println!("{} could not be written: {e}", index.display());
        return 1;
    }
    let n = entries.len();
    println!("{} lists {n} plugin{}, as catalogue number {number}.", index.display(), if n == 1 { "" } else { "s" });
    i32::from(!problems.is_empty())
}

pub fn hex_of(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::new(), |mut s, b| { let _ = write!(s, "{b:02x}"); s })
}

pub fn new_key_main(to: &Path) -> i32 {
    use std::io::Write;
    use std::os::unix::fs::OpenOptionsExt;
    let whole = std::path::absolute(to).unwrap_or_else(|_| to.to_path_buf());
    if whole.ancestors().skip(1).any(|dir| dir.join(".git").exists()) {
        println!("{} is inside a repository. A private key is never kept in one: give a path outside it.", to.display());
        return 1;
    }
    let made = Ed25519KeyPair::generate_pkcs8(&::ring::rand::SystemRandom::new()).ok()
        .and_then(|doc| Ed25519KeyPair::from_pkcs8(doc.as_ref()).ok().map(|pair| (doc, pair)));
    let Some((doc, pair)) = made else {
        println!("A key could not be made.");
        return 1;
    };
    let written = std::fs::OpenOptions::new().write(true).create_new(true).mode(0o600).open(to)
        .and_then(|mut f| f.write_all(doc.as_ref()).and_then(|()| f.sync_all()));
    if let Err(e) = written {
        println!("The private key could not be written to {}: {e}", to.display());
        return 1;
    }
    println!("The private key is in {}. Keep it off this computer's repositories and backed up; it is not shown.", to.display());
    println!("Public key, for KEYS in src/plugins/catalogue.rs:");
    println!("{}", hex_of(pair.public_key().as_ref()));
    0
}

pub fn sign_main(catalogue: &Path, key: &Path) -> i32 {
    let Some(bytes) = small_file(catalogue, KEPT_MOST) else {
        println!("{} is not a plain file of at most 1 MB.", catalogue.display());
        return 1;
    };
    let text = String::from_utf8_lossy(&bytes);
    if let Err(why) = parse(&text) {
        println!("{why}");
        return 1;
    }
    let Some(number) = number_of(&text) else {
        println!("{} has no Serial in its [Catalogue] part. Make it again with trindle-plugin catalogue.", catalogue.display());
        return 1;
    };
    let Some(pair) = small_file(key, 4096).and_then(|k| Ed25519KeyPair::from_pkcs8(&k).ok()) else {
        println!("{} is not a private key made by trindle-plugin new-key.", key.display());
        return 1;
    };
    let signature = pair.sign(&bytes);
    let mut name = catalogue.as_os_str().to_owned();
    name.push(SIGNATURE_END);
    let beside = PathBuf::from(name);
    if let Err(e) = write_whole(&beside, signature.as_ref()) {
        println!("{} could not be written: {e}", beside.display());
        return 1;
    }

    let mut public = [0u8; KEY_BYTES];
    public.copy_from_slice(pair.public_key().as_ref());
    let wrote = small_file(&beside, SIGNATURE_BYTES as u64).unwrap_or_default();
    if signed_by(&bytes, &wrote, &[public]).is_err() {
        println!("The signature written to {} does not check. Nothing is to be published.", beside.display());
        return 1;
    }
    println!("{} is signed, as catalogue number {number}: {}.", catalogue.display(), beside.display());
    if built_in_keys().contains(&public) {
        println!("This Trindle accepts that key.");
    } else {
        println!("Note: this Trindle does not accept that key yet. Its public half is {}", hex_of(&public));
    }
    0
}

pub fn check_main(folder: &Path, public: Option<&str>) -> i32 {
    match checked(folder, public) {
        Ok(n) => {
            println!("{} checks: signed, and {n} plugin{} match the SHA-256 in the signed index.", folder.display(), if n == 1 { "" } else { "s" });
            0
        }
        Err(why) => {
            println!("{} is refused: {why}", folder.display());
            1
        }
    }
}

fn checked(folder: &Path, public: Option<&str>) -> Result<usize, String> {
    let keys = match public {
        Some(hex) => vec![key_of(hex).ok_or("the key is not 64 hex digits")?],
        None => built_in_keys(),
    };
    let bytes = small_file(&folder.join(INDEX), KEPT_MOST).ok_or("catalogue.ini is missing, not a plain file or over 1 MB")?;
    let signature = small_file(&folder.join(format!("{INDEX}{SIGNATURE_END}")), SIGNATURE_BYTES as u64).ok_or("catalogue.ini.sig is missing or not a plain file")?;
    let (text, _) = accepted(&bytes, &signature, &keys, 0).map_err(|r| format!("{r:?}"))?;
    let (entries, left_out) = parse(&text)?;
    if !left_out.is_empty() {
        return Err(format!("the signed index has entries Trindle would leave out: {}", left_out.join(" ")));
    }
    for e in &entries {
        let got = sha256_of(&folder.join(&e.archive)).map_err(|err| format!("{}: {err}", e.archive))?;
        if got != e.sha256 {
            return Err(format!("{} is not the archive the signed index names (SHA-256 differs).", e.archive));
        }
        if let (Some(icon), Some(hash)) = (&e.icon, &e.icon_sha256) {
            if sha256_of(&folder.join(icon)).map_err(|err| format!("{icon}: {err}"))? != *hash {
                return Err(format!("{icon} is not the picture the signed index names (SHA-256 differs)."));
            }
        }
    }
    Ok(entries.len())
}

#[cfg(test)]
mod release_tests {
    use super::*;

    fn fake(top: &Path) -> PathBuf {
        let shelf = top.join("shelf");
        let dir = top.join("work/org.example.Fake");
        std::fs::create_dir_all(dir.join("bin")).unwrap();
        std::fs::create_dir_all(&shelf).unwrap();
        std::fs::write(dir.join("plugin.ini"), "[Plugin]\nInterface=1\nId=org.example.Fake\nName=Fake\nVersion=1.0\nSummary=A made-up plugin.\nPermissions=notifications\n\n[Ring]\nIcon=icon.svg\nLabel=Fake\nRun=bin/go\n").unwrap();
        std::fs::write(dir.join("icon.svg"), "<svg xmlns='http://www.w3.org/2000/svg'/>").unwrap();
        std::fs::write(dir.join("bin/go"), "#!/bin/sh\n").unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(dir.join("bin/go"), std::fs::Permissions::from_mode(0o755)).unwrap();
        crate::common::needs::programs(&["tar", "zstd"]);
        let ok = std::process::Command::new("tar").args(["--zstd", "-cf"]).arg(shelf.join("org.example.Fake.tar.zst")).arg("-C").arg(top.join("work")).arg("org.example.Fake").status().unwrap().success();
        assert!(ok);
        shelf
    }

    #[test]
    fn a_release_made_with_a_throwaway_key_checks_and_every_change_is_refused() {
        let top = PathBuf::from("/tmp").join(format!("trindle-release-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        std::fs::create_dir_all(&top).unwrap();
        let shelf = fake(&top);
        let key = top.join("throwaway.key");
        assert_eq!(new_key_main(&key), 0);
        assert_eq!(main(&shelf), 0);
        assert_eq!(sign_main(&shelf.join(INDEX), &key), 0);
        let pair = Ed25519KeyPair::from_pkcs8(&std::fs::read(&key).unwrap()).unwrap();
        let public = hex_of(pair.public_key().as_ref());
        assert_eq!(checked(&shelf, Some(&public)), Ok(1));
        assert!(checked(&shelf, None).is_err(), "not signed by a key built in");
        assert!(checked(&shelf, Some(&"00".repeat(32))).is_err(), "another key");
        let change = |name: &str, f: &dyn Fn(Vec<u8>) -> Vec<u8>| {
            let at = shelf.join(name);
            let was = std::fs::read(&at).unwrap();
            std::fs::write(&at, f(was.clone())).unwrap();
            let r = checked(&shelf, Some(&public));
            std::fs::write(&at, was).unwrap();
            r
        };
        assert!(change("org.example.Fake.tar.zst", &|mut b| { b.push(0); b }).is_err(), "changed archive");
        assert!(change("org.example.Fake.svg", &|mut b| { b.push(b' '); b }).is_err(), "changed picture");
        assert!(change(INDEX, &|mut b| { b.extend_from_slice(b"\n"); b }).is_err(), "changed catalogue");
        assert!(change(INDEX, &|b| String::from_utf8(b).unwrap().replace("Name=Fake", "Name=Evil").into_bytes()).is_err());
        assert!(change("catalogue.ini.sig", &|mut b| { b[0] ^= 1; b }).is_err(), "changed signature");
        assert!(change("catalogue.ini.sig", &|_| Vec::new()).is_err(), "empty signature");
        assert_eq!(checked(&shelf, Some(&public)), Ok(1), "all put back");
        let _ = std::fs::remove_dir_all(&top);
    }
}
