//! The Wallpaper portal.

use std::path::{Path, PathBuf};
use std::rc::Rc;

use zbus::zvariant::OwnedObjectPath;

use super::portal::{answer, DONE, FAILED};
use super::portal_ask::{app_name, ask_code, get, Confirm, Kit, Options, Part, SIZE};
use crate::toolkit::bus::{Asked, Offer, Props};
use crate::toolkit::t;

pub(super) const INTERFACE: &str = "org.freedesktop.impl.portal.Wallpaper";
const XML: &str = r#"<interface name="org.freedesktop.impl.portal.Wallpaper">
    <method name="SetWallpaperURI">
      <arg type="o" name="handle" direction="in"/>
      <arg type="s" name="app_id" direction="in"/>
      <arg type="s" name="parent_window" direction="in"/>
      <arg type="s" name="uri" direction="in"/>
      <arg type="a{sv}" name="options" direction="in"/>
      <arg type="u" name="response" direction="out"/>
    </method>
  </interface>"#;

const PREVIEW: (f32, f32) = (400.0, 225.0);

fn folder() -> PathBuf {
    crate::common::files::data_home().join("trindle").join("wallpapers")
}

fn keep(from: &Path, dir: &Path) -> Option<PathBuf> {
    std::fs::create_dir_all(dir).ok()?;
    let name = from.file_name()?.to_string_lossy().to_string();
    let (stem, dot) = match name.rsplit_once('.') {
        Some((s, e)) if !s.is_empty() => (s.to_string(), format!(".{e}")),
        _ => (name.clone(), String::new()),
    };
    let size = std::fs::metadata(from).ok()?.len();
    for n in 1..1000 {
        let to = match n {
            1 => dir.join(&name),
            _ => dir.join(format!("{stem}-{n}{dot}")),
        };
        if to == from {
            return Some(to);
        }
        match std::fs::metadata(&to) {
            Ok(m) if m.len() == size && std::fs::read(&to).ok() == std::fs::read(from).ok() => return Some(to),
            Ok(_) => continue,
            Err(_) => match crate::common::copies::copy_new(from, &to) {
                Ok(()) => return Some(to),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(_) => return None,
            },
        }
    }
    None
}

fn set(from: &Path) -> bool {
    if crate::session::wallpaper_limits::over(from).any() {
        return false;
    }
    let Some(kept) = keep(from, &folder()) else { return false };
    if crate::common::files::set_setting("wallpaper", &kept.to_string_lossy()).is_err() {
        return false;
    }
    crate::common::run::spawn(&["trindle-wallpaper"]);
    true
}

fn preview(path: &Path) -> Option<tiny_skia::Pixmap> {
    let img = crate::toolkit::picture::reader(path).ok()?.decode().ok()?;
    crate::toolkit::picture::to_pixmap(&img.thumbnail(PREVIEW.0 as u32 * 2, PREVIEW.1 as u32 * 2))
}

fn set_uri(kit: &Kit, asked: Asked) {
    let Ok((handle, app_id, parent, uri, options)) = asked.message().body()
        .deserialize::<(OwnedObjectPath, String, String, String, Options)>() else {
        return asked.fail("org.freedesktop.DBus.Error.InvalidArgs", "SetWallpaperURI takes a handle, an app, a window, an address and options");
    };
    let on = get::<String>(&options, "set-on").unwrap_or_default();
    let path = crate::toolkit::chooser::from_uri(&uri).filter(|p| p.is_file());
    let Some(path) = path.filter(|_| on != "lockscreen") else {
        return asked.reply(&(FAILED,));
    };
    if crate::session::wallpaper_limits::over(&path).any() {
        return asked.reply(&(FAILED,));
    }
    if get::<bool>(&options, "show-preview") == Some(false) {
        return asked.reply(&(if set(&path) { DONE } else { FAILED },));
    }
    let who = app_name(&app_id);
    let heading = match who.is_empty() {
        true => t!("screenshot-wallpaper-ask").to_string(),
        false => t!("screenshot-wallpaper-asks", who = &who),
    };
    let mut parts = vec![Part::Heading(heading.clone())];
    if let Some(px) = preview(&path) {
        parts.push(Part::Picture { px: Rc::new(px), size: PREVIEW, cover: true, tip: t!("screenshot-wallpaper-picture").into() });
    }
    parts.push(Part::Quiet(path.file_name().unwrap_or_default().to_string_lossy().into_owned()));
    let size = (SIZE.0.max(PREVIEW.0 as u32 + 40), SIZE.1);
    ask_code(kit, asked, handle.as_str(), &parent, &heading, size, move |done| {
        Confirm::new(parts, t!("screenshot-portal-cancel"), t!("screenshot-wallpaper-set"), done, move |_| {
            answer(if set(&path) { DONE } else { FAILED }, Default::default())
        })
    });
}

pub fn offer(kit: &Kit) -> Offer {
    let kit = kit.clone();
    Offer {
        name: INTERFACE.into(),
        xml: XML.into(),
        call: Box::new(move |asked: Asked| match asked.member().as_str() {
            "SetWallpaperURI" => set_uri(&kit, asked),
            _ => asked.fail("org.freedesktop.DBus.Error.UnknownMethod", "Not part of Wallpaper"),
        }),
        props: Box::new(Props::new),
        set: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use super::super::portal_ask::testkit::{call, Rig};
    use zbus::zvariant::{ObjectPath, Value};

    #[test]
    fn a_kept_picture_keeps_its_name_unless_another_has_it() {
        let dir = std::env::temp_dir().join(format!("trindle-wallpaper-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        let src = dir.join("src");
        std::fs::create_dir_all(&src).expect("dir");
        let (a, b) = (src.join("sea.png"), src.join("other").join("sea.png"));
        std::fs::create_dir_all(b.parent().expect("parent")).expect("dir");
        std::fs::write(&a, b"one").expect("write");
        std::fs::write(&b, b"two").expect("write");
        let kept = dir.join("kept");
        assert_eq!(keep(&a, &kept), Some(kept.join("sea.png")));
        assert_eq!(keep(&a, &kept), Some(kept.join("sea.png")), "the same picture is not copied twice");
        assert_eq!(keep(&b, &kept), Some(kept.join("sea-2.png")));
        assert_eq!(std::fs::read(kept.join("sea-2.png")).expect("read"), b"two");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn what_cannot_be_shown_or_set_fails() {
        let rig = Rig::new();
        let _w = rig.a.serve(super::super::portal::PATH, offer(&rig.kit())).expect("serve");
        let handle = ObjectPath::try_from("/org/freedesktop/portal/desktop/request/1_1/w1").expect("path");
        let none: HashMap<&str, Value> = HashMap::new();
        let r: (u32,) = call(&rig, INTERFACE, "SetWallpaperURI", &(&handle, "", "", "https://example.com/a.png", &none)).expect("reply");
        assert_eq!(r.0, FAILED, "only files can be set");
        let r: (u32,) = call(&rig, INTERFACE, "SetWallpaperURI", &(&handle, "", "", "file:///etc/hostname", &none)).expect("reply");
        assert_eq!(r.0, FAILED);
        assert_eq!(rig.stay.busy.get(), 0);
    }

    #[test]
    fn a_picture_too_large_is_refused_before_anything_is_asked() {
        let dir = std::env::temp_dir().join(format!("trindle-wallpaper-portal-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("dir");
        let big = dir.join("big.png");
        image::RgbImage::new(3841, 16).save(&big).expect("picture");
        assert!(!set(&big), "a picture larger than 4K was set");
        let rig = Rig::new();
        let _w = rig.a.serve(super::super::portal::PATH, offer(&rig.kit())).expect("serve");
        let handle = ObjectPath::try_from("/org/freedesktop/portal/desktop/request/1_1/w2").expect("path");
        let silent: HashMap<&str, Value> = HashMap::from([("show-preview", Value::from(false))]);
        let uri = format!("file://{}", big.display());
        let r: (u32,) = call(&rig, INTERFACE, "SetWallpaperURI", &(&handle, "", "", uri.as_str(), &silent)).expect("reply");
        assert_eq!(r.0, FAILED);
        assert_eq!(rig.stay.busy.get(), 0, "a question was put for a picture that cannot be used");
        assert!(!folder().join("big.png").exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
