//! Pictures in Apps: icons drawn from names and files, and screenshots fetched to the cache.

use crate::store::catalogue::App;
use crate::store::{Msg, Store};
use crate::common::apps::generic_icon_for;
use crate::common::run;
use crate::toolkit::picture;
use crate::toolkit::widget::{drawing, Node};
use std::rc::Rc;
use super::Pipe3;

pub fn app_icon(a: &App, size: f32) -> Node<Msg> {
    let name = a.icon.as_ref().map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|| generic_icon_for(&a.id, &a.name));
    icon(name, size)
}

pub fn own_icon<M>(id: String, size: f32) -> Node<M> {
    let plain = generic_icon_for(&id, &id);
    icon_or(id, plain, size)
}

pub fn icon_or<M>(name: String, plain: impl Into<String>, size: f32) -> Node<M> {
    let plain = plain.into();
    drawing((size, size), "", move |canvas, pen| {
        let (side, scale) = crate::settings::ui::icon_px(size, pen.scale);
        if name.is_empty() || pen.icons.draw(canvas, &name, 0, 0, side, scale, pen.theme.fg).is_none() {
            pen.icons.draw(canvas, &plain, 0, 0, side, scale, pen.theme.fg);
        }
    })
}

pub fn icon<M>(name: String, size: f32) -> Node<M> {
    drawing((size, size), "", move |canvas, pen| {
        let (side, whole) = crate::settings::ui::icon_px(size, pen.scale);
        pen.icons.draw(canvas, &name, 0, 0, side, whole, pen.theme.fg);
    })
}

thread_local! {

    static FILES: std::cell::RefCell<std::collections::HashMap<std::path::PathBuf, Option<Rc<image::DynamicImage>>>> = std::cell::RefCell::new(Default::default());
    static NAMED: std::cell::RefCell<std::collections::HashMap<std::path::PathBuf, Option<std::path::PathBuf>>> = std::cell::RefCell::new(Default::default());
}

pub(super) fn forget_files() {
    FILES.with(|f| f.borrow_mut().clear());
    NAMED.with(|f| f.borrow_mut().clear());
}

pub(super) fn appimage_icon(file: &std::path::Path) -> Option<std::path::PathBuf> {
    NAMED.with(|n| n.borrow_mut().entry(file.to_path_buf()).or_insert_with(|| {
        let launcher = crate::icons::appimages::launcher_for(file);
        let named = std::path::PathBuf::from(crate::icons::read_entry(&launcher).get("Icon").cloned().unwrap_or_default());
        named.is_file().then_some(named)
    }).clone())
}

pub(super) fn file_icon(path: std::path::PathBuf, size: f32) -> Node<Msg> {
    let img = FILES.with(|f| f.borrow_mut().entry(path.clone()).or_insert_with(|| picture::load(&path).map(Rc::new)).clone());
    drawing((size, size), "", move |canvas, _pen| {
        if let Some(img) = &img {
            let side = canvas.width().min(canvas.height());
            picture::draw(canvas, img, 0, 0, side, side);
        }
    })
}

const KEPT_DAYS: u64 = 30;

fn shot_folder() -> std::path::PathBuf {
    crate::common::files::home().join(".cache/trindle/screenshots")
}

fn shot_file(url: &str) -> std::path::PathBuf {
    use sha2::{Digest, Sha256};
    let name = Sha256::digest(url.as_bytes()).iter().take(16).fold(String::new(), |a, b| a + &format!("{b:02x}"));
    shot_folder().join(name)
}

fn trim(dir: &std::path::Path) {
    let limit = std::time::Duration::from_secs(KEPT_DAYS * 86_400);
    for e in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let old = e.metadata().and_then(|m| m.modified()).ok().and_then(|t| t.elapsed().ok()).is_some_and(|age| age > limit);
        if old {
            let _ = std::fs::remove_file(e.path());
        }
    }
}

fn ready_folder() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let _ = std::fs::create_dir_all(shot_folder());
        trim(&shot_folder());
    });
}

pub fn fetch_pictures(s: &mut Store, a: &App) {
    for (url, _) in &a.screenshots {
        fetch_picture(s, url, None);
    }
}

pub(crate) fn fetch_picture(s: &mut Store, url: &str, small: Option<u32>) {
    ready_folder();
    if s.heard.shots.contains_key(url) {
        return;
    }
    s.heard.shots.insert(url.to_string(), None);
    s.heard.missed.remove(url);
    let (file, u, gone) = (shot_file(url), url.to_string(), url.to_string());
    let load = move |s: &mut Store, file: std::path::PathBuf| {
        let back = s.then(move |s: &mut Store, img: Option<image::DynamicImage>| {
            match img {
                Some(img) => { s.heard.shots.insert(u, Some(Rc::new(img))); }
                None => missed(s, &u),
            }
        });
        run::on_thread(move || picture::load(&file).map(|i| match small { Some(px) => i.thumbnail(px, px), None => i }), back);
    };
    if file.is_file() {
        load(s, file);
        return;
    }
    let part = file.with_extension("part");
    let p = part.display().to_string();
    run::capture(&["curl", "-fsSL", "--proto", "=https", "--proto-redir", "=https", "--max-time", "30", "--max-filesize", run::MAX_ANSWER, "-o", &p, url],
                 None, s.then(move |s: &mut Store, (ok, _o, _e): (bool, String, String)| {
                     if ok && std::fs::rename(&part, &file).is_ok() {
                         load(s, file);
                     } else {
                         let _ = std::fs::remove_file(&part);
                         missed(s, &gone);
                     }
                 }).pipe3());
}

pub(super) fn missed(s: &mut Store, url: &str) {
    s.heard.shots.remove(url);
    s.heard.missed.insert(url.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_picture_from_a_file_is_decoded_once_until_forgotten() {
        let dir = std::env::temp_dir().join(format!("trindle-row-picture-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join("icon.png");
        image::RgbaImage::new(2, 2).save(&file).unwrap();
        let _ = file_icon(file.clone(), 16.0);
        std::fs::remove_file(&file).unwrap();
        let _ = file_icon(file.clone(), 16.0);
        assert!(FILES.with(|f| f.borrow().get(&file).is_some_and(|i| i.is_some())), "the second draw read the file again");
        forget_files();
        assert!(FILES.with(|f| f.borrow().is_empty()));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn pictures_are_named_by_their_whole_address_and_old_ones_go() {
        let tail = "a".repeat(200);
        let (one, two) = (format!("https://example.org/one/{tail}.png"), format!("https://example.org/two/{tail}.png"));
        assert!(shot_file(&one) != shot_file(&two) && shot_file(&one) == shot_file(&one));
        let name = shot_file(&one).file_name().unwrap().to_string_lossy().into_owned();
        assert!(name.len() == 32 && name.bytes().all(|b| b.is_ascii_hexdigit()));
        let dir = std::env::temp_dir().join(format!("trindle-shots-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let (old, new) = (dir.join("old"), dir.join("new"));
        std::fs::write(&old, "x").unwrap();
        std::fs::write(&new, "x").unwrap();
        let aged = std::time::SystemTime::now() - std::time::Duration::from_secs((KEPT_DAYS + 1) * 86_400);
        std::fs::File::options().write(true).open(&old).unwrap().set_modified(aged).unwrap();
        trim(&dir);
        assert!(!old.exists() && new.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
