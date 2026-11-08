//! Games, Steam's fiddly fixes: the stronger graphics chip and the community Proton build for chosen games, and a fresh start for one that will not start.

use std::path::Path;

use super::Games;
use crate::common::run::capture;
use crate::common::steam_files::{self as files, Game};
use crate::settings::ui::{confirm, group, line, on, Kit, Msg};
use crate::toolkit::t;
use crate::toolkit::widget::{row, switch, Length, Node};

const ALL: &str = "games:proton-all";
const UP: &str = "games:steam-up";
const PROC: &str = "/proc";
const DRM: &str = "/sys/class/drm";

#[derive(Clone, Debug, Default)]
pub struct Entry {
    pub id: u32,
    pub name: String,
    pub library: std::path::PathBuf,
    pub stronger: bool,
    pub community: bool,
}

#[derive(Default)]
pub struct Steam {
    pub entries: Vec<Entry>,

    pub two_chips: bool,

    pub open: bool,

    pub tool: Option<String>,
    pub all_community: bool,
    pub installing: bool,
}

impl Steam {
    pub fn read(&mut self, root: &Path) {
        let known = self.tool.clone();
        let community = |id: u32| known.is_some() && files::compat_tool(root, id) == known;
        self.entries = files::games(&files::libraries(root)).into_iter()
            .map(|Game { id, name, library }| Entry { id, name, library, stronger: files::prime_for(root, id), community: community(id) }).collect();
        self.all_community = community(0);
        self.two_chips = files::two_chips(Path::new(DRM));
    }
}

impl Games {
    pub(super) fn look_steam(&mut self) {
        self.steam.read(&files::root());
    }

    pub(super) fn read_steam(&mut self, kit: &mut Kit) {
        let up = kit.then::<Games, (bool, String)>(|me, _, (ok, out)| {
            me.steam.open = ok && out.lines().any(|l| l.trim() == files::STEAM);
        });
        capture(&["flatpak", "ps", "--columns=application"], None, move |ok, out, _| up((ok, out)));
        self.read_tool(kit, |_, _| {});
    }

    fn read_tool(&mut self, kit: &mut Kit, after: impl FnOnce(&mut Games, &mut Kit) + 'static) {
        let back = kit.then::<Games, (bool, String)>(move |me, k, (ok, out)| {
            me.steam.tool = ok.then(|| files::ge_name(Path::new(out.trim()))).flatten();
            me.look_steam();
            after(me, k);
        });
        capture(&["flatpak", "info", "--show-location", files::GE], None, move |ok, out, _| back((ok, out)));
    }

    fn why_not(&self, id: u32) -> Option<&'static str> {
        if self.steam.open {
            Some(t!("settings-games-steam-open"))
        } else if self.steam.installing || self.busy {
            crate::common::ui::wait_while(true)
        } else if id != 0 && files::game_running(Path::new(PROC), id) {
            Some(t!("settings-games-game-running"))
        } else {
            None
        }
    }

    fn set_stronger(&mut self, kit: &mut Kit, id: u32, on: bool) {
        let key = format!("games-game:{id}");
        let result = files::set_prime(&files::root(), id, on);
        self.look_steam();
        kit.done(&key, result.as_ref().err().map(String::as_str), if on { t!("settings-games-stronger-on") } else { t!("settings-games-stronger-off") });
    }

    fn set_community(&mut self, kit: &mut Kit, id: u32, on: bool) {
        let key = if id == 0 { ALL.to_string() } else { format!("games-game:{id}") };
        if !on {
            let result = files::set_compat_tool(&files::root(), id, None);
            self.look_steam();
            kit.done(&key, result.as_ref().err().map(String::as_str), t!("settings-games-proton-off"));
            return;
        }
        if self.steam.tool.is_some() {
            self.use_tool(kit, id);
            return;
        }
        self.steam.installing = true;
        kit.say(&key, t!("settings-games-proton-installing"), true, true);
        let back = kit.then::<Games, (bool, String)>(move |me, k, (ok, err)| {
            me.steam.installing = false;
            if !ok {
                k.done(&key, Some(&err), "");
                return;
            }
            me.read_tool(k, move |me, k| me.use_tool(k, id));
        });
        capture(&crate::common::gentle::argv(&["flatpak", "install", "--user", "--noninteractive", "-y", "flathub", files::GE]), None, move |ok, _, err| back((ok, err)));
    }

    fn use_tool(&mut self, kit: &mut Kit, id: u32) {
        let key = if id == 0 { ALL.to_string() } else { format!("games-game:{id}") };
        let Some(name) = self.steam.tool.clone() else {
            kit.done(&key, Some(t!("settings-games-proton-missing")), "");
            return;
        };
        let result = files::set_compat_tool(&files::root(), id, Some(&name));
        self.look_steam();
        kit.done(&key, result.as_ref().err().map(String::as_str), t!("settings-games-proton-on"));
    }

    fn fresh_start(&mut self, kit: &mut Kit, id: u32) {
        let key = format!("games-game:{id}");
        if let Some(why) = self.why_not(id) {
            kit.done(&key, Some(why), "");
            return;
        }
        let Some(entry) = self.steam.entries.iter().find(|e| e.id == id) else { return };
        let (sent, trouble) = fresh_start(&entry.library, id, None);
        kit.done(&key, trouble.as_deref(), if sent { t!("settings-games-fresh-done") } else { t!("settings-games-fresh-none") });
    }

    fn game_row(&self, kit: &Kit, e: &Entry) -> Node<Msg> {
        let why = self.why_not(e.id);
        let mut controls = Vec::new();
        if self.steam.two_chips {
            let id = e.id;
            controls.push(switch(e.stronger, t!("settings-games-stronger"), move |v| on::<Games>(move |me, k| me.set_stronger(k, id, v))));
        }
        let id = e.id;
        controls.push(switch(e.community, t!("settings-games-proton-one"), move |v| on::<Games>(move |me, k| me.set_community(k, id, v))));
        controls.push(confirm::<Games>(kit, &format!("games-fresh:{id}"), "edit-clear-symbolic", t!("settings-games-fresh"), move |me, k| me.fresh_start(k, id)));
        line(e.name.as_str()).key(format!("games-game:{id}")).sub(t!("settings-games-game-sub"))
            .control(row(controls).gap(8.0).width(Length::Shrink).off_because(why)).node(kit)
    }

    pub(super) fn steam_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows = Vec::new();
        if self.steam.open {
            rows.push(line(t!("settings-games-steam-open")).key(UP).sub(t!("settings-games-steam-open-sub")).node(kit));
        }
        let all = switch(self.steam.all_community, t!("settings-games-proton-all"), |v| on::<Games>(move |me, k| me.set_community(k, 0, v)))
            .off_because(self.why_not(0));
        rows.push(line(t!("settings-games-proton-all")).key(ALL).sub(t!("settings-games-proton-all-sub")).control(all).node(kit));
        if self.steam.entries.is_empty() {
            rows.push(line(t!("settings-games-none")).sub(t!("settings-games-none-sub")).node(kit));
        }
        rows.extend(self.steam.entries.iter().map(|e| self.game_row(kit, e)));
        group(t!("settings-games-steam-title")).icon("input-gaming-symbolic")
            .about(t!("settings-games-steam-about"))
            .span(6, 8)
            .rows(rows)
            .node(kit)
    }
}

pub fn fresh_start(library: &Path, id: u32, bin: Option<crate::toolkit::common::file_work::trash::Bin>) -> (bool, Option<String>) {
    use crate::toolkit::common::file_work::work::{self, Ctl, Kind, Plain, Plan};
    let folder = files::setup_folder(library, id);
    if std::fs::symlink_metadata(&folder).is_err() {
        return (false, None);
    }
    let plan = Plan { kind: Kind::<Plain>::Trash, items: vec![folder], to: std::path::PathBuf::new(), key: None };
    let troubles = work::run(plan, &Ctl { bin, ..Ctl::default() }, &|_| {}).troubles();
    (troubles.is_empty(), troubles.into_iter().next())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_fresh_start_trashes_the_setup_folder_and_leaves_the_game_and_the_rest() {
        let base = std::env::temp_dir().join(format!("trindle-fresh-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&base);
        let lib = base.join("steamapps");
        std::fs::create_dir_all(lib.join("compatdata/289650/pfx")).unwrap();
        std::fs::write(lib.join("compatdata/289650/pfx/user.reg"), "x").unwrap();
        std::fs::create_dir_all(lib.join("compatdata/400")).unwrap();
        std::fs::create_dir_all(lib.join("common/Prince")).unwrap();
        let bin = crate::toolkit::common::file_work::trash::Bin::at(&base.join("trash"), None);
        bin.ready().unwrap();
        assert_eq!(fresh_start(&lib, 289650, Some(bin)), (true, None));
        assert!(!lib.join("compatdata/289650").exists());
        assert!(lib.join("compatdata/400").is_dir() && lib.join("common/Prince").is_dir());
        let kept: Vec<_> = std::fs::read_dir(base.join("trash/files")).unwrap().flatten().collect();
        assert_eq!(kept.len(), 1);
        assert!(kept[0].path().join("pfx/user.reg").is_file(), "the folder is in the Trash, not deleted");
        assert_eq!(fresh_start(&lib, 289650, None), (false, None));
        let _ = std::fs::remove_dir_all(&base);
    }
}
