//! XDG user dirs: writes user-dirs.dirs if absent and creates the listed folders,
//! and lists them in the file manager's side bar.

use crate::common::files::{config_home, gently, home, read_text, user_folder_for, user_folders, write_text, START_WAIT};
use std::path::Path;

const FOLDERS: &[(&str, &str)] = &[
    ("DOCUMENTS", "Documents"),
    ("DOWNLOAD", "Downloads"),
    ("MUSIC", "Music"),
    ("PICTURES", "Pictures"),
    ("VIDEOS", "Videos"),
];

pub fn ensure() {
    let dirs = config_home().join("user-dirs.dirs");
    if !dirs.exists() {
        let mut text = String::from("# Where apps put downloads, pictures and the rest (freedesktop user dirs).\n");
        for (key, name) in FOLDERS {
            text.push_str(&format!("XDG_{key}_DIR=\"$HOME/{name}\"\n"));
        }

        for key in ["DESKTOP", "PUBLICSHARE", "TEMPLATES"] {
            text.push_str(&format!("XDG_{key}_DIR=\"$HOME\"\n"));
        }
        let _ = write_text(&dirs, &text);
    }
    let listed = read_text(&dirs);

    for folder in to_make(&listed, &home()) {
        let _ = gently(&folder, START_WAIT, |f| std::fs::create_dir_all(f));
    }
    side_bar(&listed);
}

fn to_make(listed: &str, home: &Path) -> Vec<std::path::PathBuf> {
    use std::path::Component;
    user_folders(listed, home).into_iter()
        .filter(|f| f.strip_prefix(home).is_ok_and(|rest| !rest.as_os_str().is_empty() && rest.components().all(|c| matches!(c, Component::Normal(_)))))
        .collect()
}

fn side_bar(listed: &str) {

    let _ = side_bar_in(&config_home().join("gtk-3.0").join("bookmarks"), listed);
}

pub(crate) fn side_bar_in(marks: &std::path::Path, listed: &str) -> std::io::Result<()> {
    if !crate::common::files::read_small(marks)?.trim().is_empty() {
        return Ok(());
    }
    let home = home();
    let mut text = String::new();
    for (key, name) in FOLDERS {
        let path = user_folder_for(listed, key, &home).unwrap_or_else(|| home.join(name));
        if path == home {
            continue;
        }
        let url = file_uri(&path);
        text.push_str(&format!("{url} {}\n", path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| name.to_string())));
    }
    if !text.is_empty() {
        if let Some(dir) = marks.parent() {
            std::fs::create_dir_all(dir)?;
        }
        write_text(marks, &text)?;
    }
    Ok(())
}

pub fn file_uri(path: &Path) -> String {
    use std::os::unix::ffi::OsStrExt;
    let mut url = String::from("file://");
    for &b in path.as_os_str().as_bytes() {
        if b.is_ascii_alphanumeric() || b"/-._~!$&'()*+,;=:@".contains(&b) {
            url.push(b as char);
        } else {
            url.push_str(&format!("%{b:02X}"));
        }
    }
    url
}

pub fn terminal() {
    let file = config_home().join("alacritty").join("alacritty.toml");
    if file.exists() {
        return;
    }
    if let Some(dir) = file.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = write_text(&file, include_str!("../../config/alacritty/alacritty.toml"));
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    #[test]
    fn a_folder_given_by_its_full_path_is_never_made_under_home() {
        use crate::common::files::{user_folder, user_folder_for, user_folders};
        let home = Path::new("/home/john");
        let listed = "# a note\nXDG_DOCUMENTS_DIR=\"$HOME/Docs\"\nXDG_PICTURES_DIR=\"/home/john/prove/deadpics\"\nXDG_VIDEOS_DIR=\"/data/films\"\n\
                      XDG_DESKTOP_DIR=\"$HOME\"\nXDG_MUSIC_DIR=\"$HOME/../other/Music\"\nXDG_TEMPLATES_DIR=\"Templates\"\nXDG_DOWNLOAD_DIR=\"$HOMEdown\"\n";
        assert_eq!(user_folder_for(listed, "PICTURES", home), Some(PathBuf::from("/home/john/prove/deadpics")));
        assert_eq!(user_folder_for(listed, "DOCUMENTS", home), Some(PathBuf::from("/home/john/Docs")));
        assert_eq!(user_folder_for(listed, "VIDEOS", home), Some(PathBuf::from("/data/films")));
        assert_eq!(user_folder_for(listed, "DESKTOP", home), Some(home.to_path_buf()));

        assert_eq!(user_folder_for(listed, "TEMPLATES", home), None);
        assert_eq!(user_folder_for(listed, "DOWNLOAD", home), None);
        assert_eq!(user_folder("\"$HOME//Music\"", home), Some(PathBuf::from("/home/john/Music")));
        assert_eq!(user_folders(listed, home).len(), 5);

        assert_eq!(super::to_make(listed, home), [PathBuf::from("/home/john/Docs"), PathBuf::from("/home/john/prove/deadpics")]);
        assert!(super::to_make("XDG_PICTURES_DIR=\"/mnt/share/pics\"\n", home).is_empty());

        let top = std::env::temp_dir().join(format!("trindle-user-folders-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        let (home, far) = (top.join("home/john"), top.join("far/pics"));
        std::fs::create_dir_all(&home).unwrap();
        let listed = format!("XDG_PICTURES_DIR=\"{}\"\nXDG_MUSIC_DIR=\"$HOME/Music\"\n", far.display());
        for folder in super::to_make(&listed, &home) {
            std::fs::create_dir_all(folder).unwrap();
        }
        assert!(home.join("Music").is_dir());
        assert!(!far.exists(), "a folder outside home was made at login");
        assert_eq!(std::fs::read_dir(&home).unwrap().count(), 1, "something else was made inside the home folder");
        let _ = std::fs::remove_dir_all(&top);
    }

    #[test]
    fn user_dirs_is_read_by_the_one_rule() {
        use crate::common::no_writing_in_place::{all_code, code};
        let home = ["\"$", "HOME\""].concat();
        let mut found = Vec::new();
        for (name, text) in all_code() {
            if name == "toolkit/common/files.rs" {
                continue;
            }
            for line in code(&text) {
                let l = line.trim_start();
                if !l.starts_with("//") && l.contains(&home) {
                    found.push(format!("{name}: {l}"));
                }
            }
        }
        assert!(found.is_empty(), "user-dirs.dirs is read outside common::files::user_folder: {found:#?}");
    }

    #[test]
    fn a_folder_address_escapes_spaces_and_letters_beyond_ascii() {
        let p = std::path::Path::new("/home/john/My Café");
        assert_eq!(super::file_uri(p), "file:///home/john/My%20Caf%C3%A9");
    }
}
