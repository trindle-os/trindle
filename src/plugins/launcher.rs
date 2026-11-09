//! A plugin with a ring entry is a regular app: it has a launcher in the person's applications folder, so it is listed and right-clicked like any other.

use std::path::{Path, PathBuf};

use super::{installed, menu_of, Plugin};

const MARK: &str = "X-Trindle-Plugin";

pub fn status_icon(p: &Plugin) -> bool {
    status_icon_in(&super::folders(), p)
}

fn status_icon_in(root: &Path, p: &Plugin) -> bool {
    p.ring.is_some() && !menu_of(&root.join(&p.id).join("app")).is_empty()
}

fn folder() -> PathBuf {
    crate::common::apps::app_dirs().remove(0)
}

fn text(p: &Plugin) -> Option<String> {
    let (icon, _, _) = p.ring.as_ref()?;
    let one = |s: &str| s.chars().filter(|c| !c.is_control()).collect::<String>();
    Some(format!(
        "[Desktop Entry]\nType=Application\nName={}\nComment={}\nIcon={}\nExec=trindle-plugin run {} ring\n\
         Categories=Utility;\nStartupWMClass={}\n{MARK}={}\n",
        one(&p.name), one(&p.summary), one(&icon.to_string_lossy()), p.id, p.id, p.id))
}

pub fn sync() {
    sync_in(&folder(), &super::folders(), &installed());
}

fn sync_in(at: &Path, root: &Path, all: &[Plugin]) {
    let wanted: Vec<(String, String)> = all.iter().filter(|p| !status_icon_in(root, p))
        .filter_map(|p| Some((format!("{}.desktop", p.id), text(p)?))).collect();
    if let Ok(dir) = std::fs::read_dir(at) {
        for e in dir.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            let ours = std::fs::read_to_string(e.path()).is_ok_and(|t| t.lines().any(|l| l.starts_with(MARK)));
            if ours && !wanted.iter().any(|(w, _)| *w == name) {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
    let _ = std::fs::create_dir_all(at);
    for (name, body) in wanted {
        let path = at.join(&name);

        let mine = std::fs::read_to_string(&path).map(|t| (t == body, t.lines().any(|l| l.starts_with(MARK))));
        match mine {
            Ok((true, _)) | Ok((false, false)) => {}
            _ => { let _ = super::notes::write_in_home(&path, &body); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plugin(dir: &Path, id: &str, menu: bool) -> Plugin {
        let app = dir.join(id).join("app");
        std::fs::create_dir_all(app.join("bin")).unwrap();
        std::fs::create_dir_all(app.join("icons")).unwrap();
        std::fs::write(app.join("icons/i.svg"), "<svg xmlns='http://www.w3.org/2000/svg'/>").unwrap();
        std::fs::write(app.join("bin/go"), "#!/bin/sh\n").unwrap();
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(app.join("bin/go"), std::fs::Permissions::from_mode(0o755)).unwrap();
        let more = if menu { "Menu=m\n\n[Menu m]\nLabel=Do it\nRun=bin/go\n" } else { "" };
        std::fs::write(app.join("plugin.ini"), format!(
            "[Plugin]\nInterface=1\nId={id}\nName=Thing\nVersion=1\n\n[Ring]\nIcon=icons/i.svg\nLabel=Thing\nRun=bin/go\n{more}")).unwrap();
        crate::plugins::manifest::check(&app).unwrap()
    }

    #[test]
    fn a_plugin_with_a_ring_entry_is_a_listed_app_unless_it_is_a_status_icon_and_gone_ones_go() {
        let root = std::env::temp_dir().join(format!("trindle-launch-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        let (apps, plugs) = (root.join("applications"), root.join("plugins"));
        let scan = plugin(&plugs, "org.example.Scan", false);
        let flow = plugin(&plugs, "org.example.Flow", true);
        std::fs::create_dir_all(&apps).unwrap();
        std::fs::write(apps.join("theirs.desktop"), "[Desktop Entry]\nName=Theirs\n").unwrap();
        std::fs::write(apps.join("org.example.Scan.desktop"), "[Desktop Entry]\nName=Old\nX-Trindle-Plugin=org.example.Scan\n").unwrap();
        assert!(!status_icon_in(&plugs, &scan) && status_icon_in(&plugs, &flow));
        sync_in(&apps, &plugs, &[scan.clone(), flow.clone()]);
        let got = std::fs::read_to_string(apps.join("org.example.Scan.desktop")).unwrap();
        assert!(got.contains("Exec=trindle-plugin run org.example.Scan ring") && got.contains("Name=Thing"), "{got}");
        assert!(!apps.join("org.example.Flow.desktop").exists(), "a status icon is not an app");
        assert!(apps.join("theirs.desktop").exists(), "another program's launcher is never touched");
        sync_in(&apps, &plugs, &[]);
        assert!(!apps.join("org.example.Scan.desktop").exists(), "removed with the plugin");
        assert!(apps.join("theirs.desktop").exists());
        let _ = std::fs::remove_dir_all(&root);
    }
}
