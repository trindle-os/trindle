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

pub(super) fn file_icon(path: std::path::PathBuf, size: f32) -> Node<Msg> {
    let img = picture::load(&path).map(Rc::new);
    drawing((size, size), "", move |canvas, _pen| {
        if let Some(img) = &img {
            let side = canvas.width().min(canvas.height());
            picture::draw(canvas, img, 0, 0, side, side);
        }
    })
}

fn shot_file(url: &str) -> std::path::PathBuf {
    let name: String = url.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
    let tail: String = name.chars().rev().take(120).collect::<String>().chars().rev().collect();
    crate::common::files::home().join(".cache/trindle/screenshots").join(tail)
}

pub fn fetch_pictures(s: &mut Store, a: &App) {
    for (url, _) in &a.screenshots {
        fetch_picture(s, url, None);
    }
}

pub(crate) fn fetch_picture(s: &mut Store, url: &str, small: Option<u32>) {
    let _ = std::fs::create_dir_all(shot_file("x").parent().unwrap_or(std::path::Path::new("/tmp")));
    if s.shots.contains_key(url) {
        return;
    }
    s.shots.insert(url.to_string(), None);
    s.shots_missed.remove(url);
    let (file, u, gone) = (shot_file(url), url.to_string(), url.to_string());
    let load = move |s: &mut Store, file: std::path::PathBuf| {
        let back = s.then(move |s: &mut Store, img: Option<image::DynamicImage>| {
            match img {
                Some(img) => { s.shots.insert(u, Some(Rc::new(img))); }
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
    s.shots.remove(url);
    s.shots_missed.insert(url.to_string());
}
