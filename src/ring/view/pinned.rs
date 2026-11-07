//! What is pinned to the ring and the looks outside it that find what is there.

use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

use crate::common::{apps, favourites};
use crate::common::run::{on_thread, spawn};
use crate::ring::ctx::{Do, Program};
use crate::toolkit::t;
use crate::toolkit::colour::Colour;

use super::{Face, REFUSED_SECONDS};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Ink {
    Usual,
    Accent,
    Own(Colour),
}

#[derive(Default)]
pub(super) struct Pinned {
    apps: Option<Rc<Vec<Program>>>,
    ids: Vec<String>,
    all: Option<Rc<Vec<Program>>>,
    inks: Vec<(String, Ink)>,
    found: Vec<(std::path::PathBuf, crate::common::looks::Looks)>,
}

struct Found {
    ids: Vec<String>,
    images: Vec<std::path::PathBuf>,
    left_out: Vec<String>,
    there: Vec<(std::path::PathBuf, crate::common::looks::Looks)>,
    stamps: Vec<Option<Option<SystemTime>>>,
    images_there: Vec<std::path::PathBuf>,
}

const LOOK_MOST: Duration = Duration::from_secs(10);

type At = Arc<Mutex<Option<String>>>;

#[derive(Default)]
pub(super) struct Looking {
    turn: u64,
    out: Option<(Instant, At)>,
    held_up: Vec<(u64, String)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Back {
    Ours,
    Late,
}

impl Looking {
    fn begin(&mut self, now: Instant) -> Option<(u64, Vec<String>, At)> {
        if let Some((since, at)) = &self.out {
            if now.saturating_duration_since(*since) < LOOK_MOST {
                return None;
            }
            let path = at.lock().ok().and_then(|at| at.clone())?;
            self.held_up.push((self.turn, path));
        }
        self.turn += 1;
        let at = At::default();
        self.out = Some((now, at.clone()));
        Some((self.turn, self.left_out(), at))
    }

    fn back(&mut self, turn: u64) -> Back {
        if turn == self.turn && self.out.is_some() {
            self.out = None;
            return Back::Ours;
        }
        self.held_up.retain(|(held, _)| *held != turn);
        Back::Late
    }

    fn waits_for(&self, turn: u64) -> bool {
        self.out.is_some() && self.turn == turn
    }

    fn left_out(&self) -> Vec<String> {
        self.held_up.iter().map(|(_, path)| path.clone()).collect()
    }
}

fn path_kind(id: &str) -> Option<bool> {
    std::fs::metadata(id).ok().map(|m| m.is_dir())
}

const IMAGE_FOLDERS: [&str; 3] = [".appimage", "Applications", "AppImages"];

fn found_outside(home: &std::path::Path, ids: &[String], images: &[std::path::PathBuf], left_out: &[String], at: &Mutex<Option<String>>,
                 looks: impl Fn(&[std::path::PathBuf]) -> Vec<crate::common::looks::Looks>) -> Found {
    let go_to = |id: &str| {
        let wanted = !left_out.iter().any(|left| left == id);
        if let (true, Ok(mut at)) = (wanted, at.lock()) {
            *at = Some(id.to_string());
        }
        wanted
    };
    let mut there = Vec::new();
    for id in ids.iter().filter(|id| go_to(id)) {
        let path = std::path::PathBuf::from(id);
        if path.exists() {
            if let Some(looks) = looks(std::slice::from_ref(&path)).into_iter().next() {
                there.push((path, looks));
            }
        }
    }
    let stamps = IMAGE_FOLDERS.iter().map(|folder| {
        let folder = home.join(folder);
        go_to(&folder.to_string_lossy()).then(|| std::fs::metadata(&folder).and_then(|m| m.modified()).ok())
    }).collect();
    let images_there = images.iter()
        .filter(|image| image.parent().is_none_or(|folder| !left_out.iter().any(|left| folder.as_os_str() == left.as_str())))
        .filter(|image| go_to(&image.to_string_lossy()) && image.is_file())
        .cloned().collect();
    if let Ok(mut at) = at.lock() {
        *at = None;
    }
    Found { ids: ids.to_vec(), images: images.to_vec(), left_out: left_out.to_vec(), there, stamps, images_there }
}

fn stamps_kept(before: Option<&[Option<SystemTime>]>, found: Vec<Option<Option<SystemTime>>>) -> Vec<Option<SystemTime>> {
    found.into_iter().enumerate().map(|(i, stamp)| stamp.unwrap_or_else(|| before.and_then(|b| b.get(i).copied()).flatten())).collect()
}

fn join_found(apps: &[Program], ids: &[String], found: &[(std::path::PathBuf, crate::common::looks::Looks)]) -> (Vec<Program>, Vec<(String, Ink)>) {
    let mut all = apps.to_vec();
    let mut inks = Vec::new();
    for (path, looks) in ids.iter().filter_map(|id| found.iter().find(|(path, _)| path.as_os_str() == id.as_str())) {
        let ink = match (looks.folder, looks.tint) {
            (true, Some(own)) => Ink::Own(own),
            (true, None) => Ink::Accent,
            (false, _) => Ink::Usual,
        };
        inks.push((path.to_string_lossy().into_owned(), ink));
        all.push(favourites::path_entry(path, Some(looks.icon.clone())));
    }
    (all, inks)
}

fn image_of(app: &Program) -> Option<std::path::PathBuf> {
    let image = std::path::PathBuf::from(app.appimage.as_deref()?.split('|').next()?);
    (image.parent() == Some(crate::common::files::home().join(IMAGE_FOLDERS[0]).as_path())).then_some(image)
}

fn begin_opening(list: &mut Vec<(String, Instant)>, id: &str, now: Instant) -> bool {
    list.retain(|(_, since)| now.saturating_duration_since(*since) < LOOK_MOST);
    if list.iter().any(|(o, _)| o == id) {
        return false;
    }
    list.push((id.to_string(), now));
    true
}

impl Face {
    pub(super) fn all(&self) -> Rc<Vec<Program>> {
        let apps = self.ctx.apps();
        let ids = self.pinned_ids();
        if ids.is_empty() {
            return apps;
        }
        let mut kept = self.pinned.borrow_mut();
        if let (Some(from), Some(all)) = (&kept.apps, &kept.all) {
            if Rc::ptr_eq(from, &apps) && kept.ids == ids {
                return all.clone();
            }
        }
        if kept.ids != ids {
            self.look_aside(&ids);
        }
        let (all, inks) = join_found(&apps, &ids, &kept.found);
        let all = Rc::new(all);
        let found = std::mem::take(&mut kept.found);
        *kept = Pinned { apps: Some(apps), ids, all: Some(all.clone()), inks, found };
        all
    }

    pub(super) fn pinned_ids(&self) -> Vec<String> {
        self.favourites.iter().filter(|f| favourites::is_path(f)).cloned().collect()
    }

    pub(super) fn look_aside(&self, ids: &[String]) {
        let Some((turn, left_out, at)) = self.looking.borrow_mut().begin(Instant::now()) else { return };
        let (ids, ctx, looking) = (ids.to_vec(), self.ctx.clone(), self.looking.clone());
        let (images, home) = (self.images(), crate::common::files::home());
        on_thread(move || found_outside(&home, &ids, &images, &left_out, &at, crate::common::looks::looks), move |found| {
            match looking.borrow_mut().back(turn) {
                Back::Ours => ctx.face(move |face| face.looked(found)),
                Back::Late => ctx.face(|face| face.look_afresh()),
            }
        });
        let ctx = self.ctx.clone();
        self.ctx.lp.after(LOOK_MOST + Duration::from_millis(100), move || ctx.face(move |face| face.look_overdue(turn)));
    }

    fn images(&self) -> Vec<std::path::PathBuf> {
        self.ctx.apps().iter().filter_map(image_of).collect()
    }

    fn look_afresh(&mut self) {
        let ids = self.pinned_ids();
        self.look_aside(&ids);
    }

    fn look_overdue(&mut self, turn: u64) {
        if self.looking.borrow().waits_for(turn) {
            self.look_afresh();
        }
    }

    fn looked(&mut self, found: Found) {
        let stamps = stamps_kept(self.image_stamps.as_deref(), found.stamps);
        if self.image_stamps.as_ref().is_some_and(|was| *was != stamps) {
            self.ctx.tell(Do::AppsChanged);
        }
        self.image_stamps = Some(stamps);
        self.images_there = found.images_there;
        {
            let mut kept = self.pinned.borrow_mut();
            kept.found = found.there;
            kept.all = None;
        }
        let ids = self.pinned_ids();
        if found.ids != ids || found.images != self.images() || self.looking.borrow().left_out() != found.left_out {
            self.look_aside(&ids);
        }
        self.relayout();
    }

    pub(super) fn ink_of(&self, id: &str) -> Ink {
        if !favourites::is_path(id) {
            return Ink::Usual;
        }
        self.pinned.borrow().inks.iter().find(|(i, _)| i == id).map_or(Ink::Usual, |(_, ink)| *ink)
    }

    pub(super) fn open_path(&mut self, id: &str) {
        let now = Instant::now();
        if !begin_opening(&mut self.opening.borrow_mut(), id, now) {
            return;
        }
        let (ctx, opening) = (self.ctx.clone(), self.opening.clone());
        let (asked, id) = (id.to_string(), id.to_string());
        on_thread(move || path_kind(&asked), move |kind| {
            opening.borrow_mut().retain(|(o, _)| *o != id);
            if now.elapsed() < LOOK_MOST {
                ctx.face(move |face| face.open_looked(&id, kind));
            } else {
                ctx.face(|face| face.say(t!("ring-could-not-open"), REFUSED_SECONDS));
            }
        });
    }

    fn open_looked(&mut self, id: &str, kind: Option<bool>) {
        let Some(folder) = kind else {
            return self.say(t!("ring-gone"), REFUSED_SECONDS);
        };
        let folders = self.ctx.apps().iter().find(|a| a.id == "trindle-folders.desktop").cloned();
        let opened = match folders.filter(|_| folder) {
            Some(app) => apps::launch(&app, &[id.to_string()]),
            None => spawn(&["xdg-open", id]),
        };
        if opened {
            self.close();
        } else {
            self.say(t!("ring-could-not-open"), REFUSED_SECONDS);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_click_on_a_path_being_looked_at_is_ignored_only_until_the_look_is_overdue() {
        use {begin_opening, LOOK_MOST};
        use std::time::{Duration, Instant};
        let now = Instant::now();
        let mut list = Vec::new();
        assert!(begin_opening(&mut list, "/mnt/share/a", now));
        assert!(!begin_opening(&mut list, "/mnt/share/a", now + Duration::from_millis(10)));
        assert!(begin_opening(&mut list, "/mnt/share/b", now), "another path is not held up");
        assert!(begin_opening(&mut list, "/mnt/share/a", now + LOOK_MOST + Duration::from_secs(1)), "a look that never came back blocked the path for good");
    }

    #[test]
    fn a_look_names_the_appimage_folders_and_files_and_the_menu_looks_at_nothing() {
        use {found_outside, image_of, stamps_kept, IMAGE_FOLDERS};
        use crate::common::removal::{how, How, Real};
        use crate::common::favourites;
        use std::sync::Mutex;
        use std::time::{Duration, SystemTime};
        let home = std::env::temp_dir().join(format!("trindle-ring-images-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        let folders: Vec<String> = IMAGE_FOLDERS.iter().map(|f| home.join(f).to_string_lossy().into_owned()).collect();
        std::fs::create_dir_all(&folders[0]).unwrap();
        let (kept, gone) = (home.join(IMAGE_FOLDERS[0]).join("kept.AppImage"), home.join(IMAGE_FOLDERS[0]).join("gone.AppImage"));
        std::fs::write(&kept, b"").unwrap();
        let far = std::path::PathBuf::from("/mnt/share/far.AppImage");
        let at = Mutex::new(None);
        let images = [kept.clone(), gone.clone()];
        let found = found_outside(&home, &[], &images, &[], &at, made_up_looks);
        assert_eq!(found.stamps.iter().map(|s| s.map(|s| s.is_some())).collect::<Vec<_>>(), [Some(true), Some(false), Some(false)]);
        assert_eq!((found.images_there, found.images.len()), (vec![kept.clone()], 2));
        assert_eq!(*at.lock().unwrap(), None);
        let found = found_outside(&home, &[], &images, &folders[..1], &at, made_up_looks);
        assert_eq!(found.stamps.iter().map(|s| s.is_some()).collect::<Vec<_>>(), [false, true, true]);
        assert!(found.images_there.is_empty(), "an AppImage was looked at in a folder the look was to leave alone");
        let found = found_outside(&home, &[], &images, &[gone.to_string_lossy().into_owned()], &at, made_up_looks);
        assert_eq!(found.images_there, std::slice::from_ref(&kept));
        let _ = std::fs::remove_dir_all(&home);
        let then = SystemTime::UNIX_EPOCH + Duration::from_secs(5);
        assert_eq!(stamps_kept(Some(&[Some(then), None, None]), vec![None, Some(None), Some(Some(then))]), [Some(then), None, Some(then)]);
        assert_eq!(stamps_kept(None, vec![None, Some(Some(then))]), [None, Some(then)]);
        let code = crate::ring::view::tests::view_code();
        let look = &code[code.find("\nfn found_outside(").unwrap()..];
        let look = &look[..look.find("\n}\n").unwrap()];
        assert!(look.contains("go_to(&folder.to_string_lossy()).then(|| std::fs::metadata(&folder)"), "an AppImage folder is looked at with no path named");
        assert!(look.contains("go_to(&image.to_string_lossy()) && image.is_file()"), "an AppImage's file is looked at with no path named");
        assert_eq!(look.matches("std::fs::").count() + look.matches(".exists()").count() + look.matches(".is_file()").count() + look.matches(".is_dir()").count(), 3,
                   "the look goes somewhere it does not name");
        let gone = crate::common::files::home().join(IMAGE_FOLDERS[0]).join("gone.AppImage");
        let mut app = favourites::path_entry(std::path::Path::new("/usr/share/applications/far.desktop"), None);
        app.id = "far.desktop".into();
        app.appimage = Some(format!("{}|1", gone.display()));
        assert_eq!(image_of(&app), Some(gone.clone()));
        assert!(how(&app, &Real, &[]).is_none(), "an AppImage not known to be there was offered for uninstalling");
        assert_eq!(how(&app, &Real, std::slice::from_ref(&gone)), Some(How::Image(gone.clone())));
        app.appimage = Some(far.to_string_lossy().into_owned());
        assert!(image_of(&app).is_none() && how(&app, &Real, &[far]).is_some_and(|h| h.only_hides()));
    }

    #[test]
    fn a_look_that_does_not_come_back_is_given_up_on_and_the_next_leaves_out_what_held_it_up() {
        use {Back, Looking, LOOK_MOST};
        use std::time::{Duration, Instant};
        let mut looking = Looking::default();
        let t0 = Instant::now();
        let (first, left_out, at) = looking.begin(t0).expect("the first look goes out");
        assert!(left_out.is_empty());
        assert!(looking.begin(t0 + LOOK_MOST - Duration::from_millis(1)).is_none());
        assert!(looking.waits_for(first));
        *at.lock().unwrap() = Some("/mnt/share/Photos".to_string());
        let (second, left_out, at2) = looking.begin(t0 + LOOK_MOST).expect("a look was waited for for good");
        assert_eq!(left_out, ["/mnt/share/Photos"], "the next look would wait where the last one did");
        assert!(second != first && looking.waits_for(second) && !looking.waits_for(first));
        assert_eq!(looking.back(second), Back::Ours);
        assert!(!looking.waits_for(second));
        assert_eq!(looking.left_out(), ["/mnt/share/Photos"]);
        let (third, left_out, _) = looking.begin(t0 + LOOK_MOST * 2).unwrap();
        assert_eq!(left_out, ["/mnt/share/Photos"]);
        assert_eq!(looking.back(first), Back::Late, "a look given up on was taken up when it came back late");
        assert!(looking.left_out().is_empty());
        assert!(looking.waits_for(third), "a late look ended the wait for the one that is out");
        assert_eq!(looking.back(third), Back::Ours);
        let (fourth, ..) = looking.begin(t0 + LOOK_MOST * 3).unwrap();
        assert!(looking.begin(t0 + LOOK_MOST * 9).is_none());
        assert_eq!(looking.back(fourth), Back::Ours);
        drop(at2);
    }

    #[test]
    fn a_look_names_the_path_it_is_at_and_leaves_alone_the_ones_given_up_on() {
        use found_outside;
        use std::sync::Mutex;
        let (dir, ids) = pinned_things("at");
        let at = Mutex::new(None);
        let seen = std::cell::RefCell::new(Vec::new());
        let looks = |paths: &[std::path::PathBuf]| {
            seen.borrow_mut().push((paths.to_vec(), at.lock().unwrap().clone()));
            made_up_looks(paths)
        };
        let found = found_outside(&dir, &ids, &[], &ids[1..2], &at, looks);
        assert_eq!(found.left_out, ids[1..2]);
        assert_eq!(found.there.iter().map(|(p, _)| p.to_string_lossy().into_owned()).collect::<Vec<_>>(), [ids[0].clone(), ids[2].clone()]);
        for (paths, named) in seen.borrow().iter() {
            assert_eq!(paths.len(), 1);
            assert_eq!(named.as_deref(), paths[0].to_str());
        }
        assert_eq!(seen.borrow().len(), 2);
        assert_eq!(*at.lock().unwrap(), None);
        assert_eq!((path_kind(&ids[0]), path_kind(&ids[2]), path_kind(&ids[3])), (Some(true), Some(false), None));
        let _ = std::fs::remove_dir_all(dir);
    }

    fn pinned_things(tag: &str) -> (std::path::PathBuf, Vec<String>) {
        let dir = std::env::temp_dir().join(format!("trindle-ring-pinned-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("Photos")).unwrap();
        std::fs::create_dir_all(dir.join("Recipes")).unwrap();
        std::fs::write(dir.join("letter to John Smith.odt"), b"made up").unwrap();
        let ids = ["Photos", "Recipes", "letter to John Smith.odt", "taken out with its drive"].iter()
            .map(|n| dir.join(n).to_string_lossy().into_owned()).collect();
        (dir, ids)
    }

    fn join_pinned(apps: &[Program], ids: &[String], looks: impl Fn(&[std::path::PathBuf]) -> Vec<crate::common::looks::Looks>) -> (Vec<Program>, Vec<(String, Ink)>) {
        let found = found_outside(std::path::Path::new("/trindle-test-no-home"), ids, &[], &[], &std::sync::Mutex::new(None), looks);
        join_found(apps, ids, &found.there)
    }

    fn made_up_looks(paths: &[std::path::PathBuf]) -> Vec<crate::common::looks::Looks> {
        paths.iter().map(|p| crate::common::looks::Looks {
            icon: if p.is_dir() { "folder-symbolic" } else { "x-office-document-symbolic" }.into(),
            tint: p.ends_with("Photos").then(|| crate::toolkit::colour::Colour::parse("#019587")).flatten(),
            folder: p.is_dir(),
        }).collect()
    }

    #[test]
    fn folders_and_files_pinned_to_the_ring_join_the_apps_after_them() {
        use Ink;
        let how_to_uninstall = |app: &Program| crate::common::removal::how(app, &crate::common::removal::Real, &[]);
        use crate::common::favourites;
        let (dir, ids) = pinned_things("join");
        let mut app = favourites::path_entry(std::path::Path::new("/usr/share/applications/firefox-esr.desktop"), Some("firefox-esr".into()));
        app.id = "firefox-esr.desktop".into();
        let (all, inks) = join_pinned(&[app.clone()], &ids, made_up_looks);
        assert_eq!(all.len(), 4);
        assert_eq!(all[0], app);
        assert_eq!(all.iter().skip(1).map(|a| a.name.as_str()).collect::<Vec<_>>(), ["Photos", "Recipes", "letter to John Smith.odt"]);
        assert!(all.iter().skip(1).all(|a| favourites::is_path(&a.id) && a.category.is_empty()));
        assert_eq!(all.iter().position(|a| a.id == ids[1]), Some(2));
        assert!(!all.iter().any(|a| a.id == ids[3]));
        assert!(matches!(inks[0], (_, Ink::Own(_))));
        assert_eq!((inks[1].1, inks[2].1), (Ink::Accent, Ink::Usual));
        assert_eq!(all[1].icon.as_deref(), Some("folder-symbolic"));
        assert!(all.iter().skip(1).all(|a| how_to_uninstall(a).is_none()));
        let _ = std::fs::remove_dir_all(dir);
    }
}
