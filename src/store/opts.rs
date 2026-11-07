//! The command line's choices, and what each does to the open Store.

use std::path::PathBuf;

use super::{dropped, came_from, pages, update_page, Store};

#[derive(Default)]
pub(super) struct Opts {
    pub(super) search: Option<String>,
    pub(super) github: Option<String>,
    pub(super) backup: Option<String>,
    pub(super) restore: Option<String>,
    pub(super) app: Option<String>,
    pub(super) page: Option<String>,
    pub(super) updates: Option<String>,
    pub(super) update_now: bool,
    pub(super) renew_steam: bool,
    pub(super) deb: Option<PathBuf>,
    pub(super) appimage: Option<PathBuf>,
}

impl Opts {
    pub(super) fn read(args: &[String]) -> Opts {
        let mut o = Opts::default();
        let mut it = args.iter().skip(1);
        while let Some(a) = it.next() {
            let Some(a) = a.strip_prefix("--") else { continue };
            let (key, inline) = match a.split_once('=') {
                Some((k, v)) => (k, Some(v.to_string())),
                None => (a, None),
            };
            if key == "updates" && inline.is_none() {
                o.updates = Some(String::new());
                continue;
            }
            if key == "update-now" {
                o.update_now = true;
                continue;
            }
            if key == "renew-steam" {
                o.renew_steam = true;
                continue;
            }
            let Some(v) = inline.or_else(|| it.next().cloned()) else { continue };
            match key {
                "search" => o.search = Some(v),
                "github" => o.github = Some(v),
                "backup" => o.backup = Some(v),
                "restore" => o.restore = Some(v),
                "app" => o.app = Some(v),
                "page" => o.page = Some(v),
                "updates" => o.updates = Some(v),
                "deb" => o.deb = Some(PathBuf::from(v)),
                "appimage" => o.appimage = Some(PathBuf::from(v)),
                _ => {}
            }
        }
        o
    }
}

impl Store {
    pub(super) fn apply(&mut self, o: Opts) {
        if let Some(file) = o.deb {
            pages::add_file::open_deb(self, file);
            return;
        }
        if let Some(file) = o.appimage {
            if dropped::kind_of(&file) == Some(dropped::Kind::AppImage) {
                pages::appimage_warning::before(self, move |s| pages::add_file::add_appimage_file(s, file, true, came_from::CameFrom::NotKnown));
            }
        }
        if let Some(id) = o.app {
            self.pending_app = Some(id);
            self.show_pending_app();
        }
        if let Some(page) = o.page {
            self.select_nav(&page);
        }
        if let Some(part) = o.updates {
            if o.update_now {
                update_page::check::ask_start(self);
            }
            self.select_nav("updates");
            self.pending_spot = Some(part).filter(|p| !p.is_empty());
        }
        if let Some(words) = o.github {
            self.pending_github = Some(words);
            self.select_nav("github");
        }
        if let Some(words) = o.search.filter(|w| !w.is_empty()) {
            self.search_for(&words);
        }
    }
}
