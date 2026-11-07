//! A real .deb built here, put in its own folder and taken off again, in a scratch home.

use super::install::{install_with, remove};
use super::launchers::launchers_of;
use super::*;
use crate::common::files::data_home;
use std::process::Command;

fn make_deb(into: &Path, name: &str, depends: &str, files: &[(&str, &str, u32)]) {
    use std::os::unix::fs::PermissionsExt;
    let root = into.join(format!("{name}-tree"));
    std::fs::create_dir_all(root.join("DEBIAN")).unwrap();
    let mut control = format!("Package: {name}\nVersion: 1.0\nArchitecture: all\nMaintainer: John Smith <john@example.com>\nDescription: a test package\n");
    if !depends.is_empty() {
        control.push_str(&format!("Depends: {depends}\n"));
    }
    std::fs::write(root.join("DEBIAN/control"), control).unwrap();
    for (path, text, mode) in files {
        let file = root.join(path);
        std::fs::create_dir_all(file.parent().unwrap()).unwrap();
        std::fs::write(&file, text).unwrap();
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(*mode)).unwrap();
    }
    let built = Command::new("dpkg-deb").args(["--build", "--root-owner-group"]).arg(&root)
        .arg(into.join(format!("{name}.deb"))).output().unwrap();
    assert!(built.status.success(), "{}", String::from_utf8_lossy(&built.stderr));
}

#[test]
fn every_deb_goes_in_its_own_folder_and_twenty_come_out_leaving_nothing() {
    use crate::common::scratch_home::{files_under, inside, rerun};
    let Some(home) = inside() else { return rerun(module_path!(), "every_deb_goes_in_its_own_folder_and_twenty_come_out_leaving_nothing") };
    use std::os::unix::fs::MetadataExt;
    assert!(data_home().starts_with(&home));
    let debs = home.join("debs");
    std::fs::create_dir_all(&debs).unwrap();
    make_deb(&debs, "trindle-e2e-lib", "", &[("usr/lib/e2e/shared.txt", "shared", 0o644)]);
    let names: Vec<String> = (0..20).map(|n| format!("trindle-e2e-app{n}")).collect();
    for name in &names {
        let desktop = format!("[Desktop Entry]\nName=App\nExec={name} %U\nType=Application\nCategories=Utility;\n");
        make_deb(&debs, name, "trindle-e2e-lib", &[
            (&format!("usr/bin/{name}"), "#!/bin/sh\necho hi\n", 0o755),
            (&format!("usr/share/applications/{name}.desktop"), &desktop, 0o644),
        ]);
    }
    let mut seen = Vec::new();
    for name in &names {
        let done = install_with(&debs.join(format!("{name}.deb")), Some(&debs), |_| {}).unwrap();
        assert_eq!(done.name, *name);
        assert!(done.missing.is_empty(), "{:?}", done.missing);
        assert_eq!(version(name).as_deref(), Some("1.0"));
        let program = folder(name).join("files/usr/bin").join(name);
        assert_eq!(std::fs::read_to_string(&program).unwrap(), "#!/bin/sh\necho hi\n");
        assert!(folder(name).join("home").is_dir(), "a private home");
        let carried: Vec<String> = carried(name).into_iter().map(|c| c.0).collect();
        assert_eq!(carried, ["trindle-e2e-lib", name.as_str()], "its library first, itself last");
        let launchers = launchers_of(name);
        assert_eq!(launchers.len(), 1);
        let text = std::fs::read_to_string(&launchers[0]).unwrap();
        assert!(text.contains(&format!("Exec=trindle-boxed {name} {name} %U")), "{text}");
        seen.push(std::fs::metadata(folder(name).join("files/usr/lib/e2e/shared.txt")).unwrap().ino());
    }
    assert!(seen.iter().all(|i| *i == seen[0]), "the shared library is kept once");
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(installed(), sorted);
    remove(&names[0]).unwrap();
    assert!(!folder(&names[0]).exists() && launchers_of(&names[0]).is_empty());
    assert_eq!(std::fs::read_to_string(folder(&names[1]).join("files/usr/lib/e2e/shared.txt")).unwrap(), "shared");
    for name in &names[1..] {
        remove(name).unwrap();
    }
    let mut left = files_under(&home);
    left.retain(|f| !f.starts_with(&debs));
    assert!(left.is_empty(), "nothing left behind: {left:?}");
    assert!(installed().is_empty());
    make_deb(&debs, "trindle-e2e-driver", "", &[("lib/modules/6.0/kernel/x.ko", "x", 0o644)]);
    assert!(install_with(&debs.join("trindle-e2e-driver.deb"), None, |_| {}).is_err());
    assert!(!folder("trindle-e2e-driver").exists());
    assert!(install_with(&debs.join("not-there.deb"), None, |_| {}).is_err());
}
