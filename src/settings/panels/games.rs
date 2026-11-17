//! Games: what helps Steam games run well that Steam itself does not set.
//! A larger sound buffer, a second drive for games, and the kernel's
//! slowing of split-lock programs (common/games.rs), and the game controllers
//! (pads.rs), which share the page.

mod steam;

use super::pads::Pads;
use crate::common::games::{self, Drive};
use crate::common::run::capture;
use crate::settings::ui::{admin_step, group, headline, line, on, page, page_split, Kit, Msg, Page, Raw};
use crate::toolkit::t;
use crate::toolkit::widget::{button, remove, switch, Node};

const SOUND: &str = "games:sound";
const SPLIT: &str = "games:split-lock";
const DRIVE: &str = "games:drive";

const STEAM_LOOK_EVERY: u32 = 3;

#[derive(Default)]
pub struct Games {
    pub(super) pads: Pads,
    sound: bool,

    split: Option<bool>,
    drives: Vec<Drive>,
    busy: bool,
    steam: steam::Steam,

    looks: u32,
}

fn answer(ok: bool, out: String, err: String) -> Result<String, Raw> {
    if ok { Ok(out) } else { Err(Raw(err)) }
}

fn size(bytes: u64) -> String {
    crate::common::volumes::size_words(bytes)
}

impl Games {
    fn look(&mut self) {
        self.sound = games::sound_fix_on();
        self.split = games::split_lock_off();
        self.look_steam();
    }

    fn read_drives(&mut self, kit: &mut Kit) {
        let back = kit.settles_quietly::<Games, String, Raw>(DRIVE, t!("settings-games-drive-unread"), |me, _, got| {
            if let Some(out) = got {
                me.drives = games::drives(&out);
            }
        });
        capture(&games::LSBLK, None, move |ok, out, err| back(answer(ok, out, err)));
    }

    fn set_sound(&mut self, kit: &mut Kit, on: bool) {
        if let Err(e) = games::set_sound_fix(on) {
            kit.done(SOUND, Some(&e), "");
            return;
        }
        self.sound = on;
        let back = kit.then::<Games, bool>(move |me, k, now| {
            me.sound = games::sound_fix_on();
            let said = match (on, now) {
                (true, true) => t!("settings-games-sound-done-on"),
                (true, false) => t!("settings-games-sound-next-login"),
                (false, _) => t!("settings-games-sound-done-off"),
            };
            k.done(SOUND, None, said);
        });
        let argv = games::sound_now(on);
        let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
        capture(&argv, None, move |ok, _, _| back(ok));
    }

    fn admin(&mut self, kit: &mut Kit, key: &'static str, args: &[&str], success: &'static str, after: fn(&mut Games, &mut Kit)) {
        self.busy = true;
        admin_step::<Games>(kit, key, args, t!("settings-games-waiting"), success, move |me, k, ok| {
            me.busy = false;
            me.look();
            if ok {
                after(me, k);
            }
            me.read_drives(k);
        });
    }

    fn set_split(&mut self, kit: &mut Kit, fix: bool) {
        self.split = Some(fix);
        self.admin(kit, SPLIT, &["split-lock-slowing", if fix { "off" } else { "on" }],
                   if fix { t!("settings-games-split-done-on") } else { t!("settings-games-split-done-off") }, |_, _| {});
    }

    fn use_drive(&mut self, kit: &mut Kit, uuid: String) {
        self.admin(kit, DRIVE, &["games-drive", &uuid], t!("settings-games-drive-done"), |_, k| {

            let back = k.settles_quietly::<Games, String, Raw>(DRIVE, t!("settings-games-drive-steam-failed"), |_, _, _: Option<String>| {});
            let argv = games::steam_reach();
            let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
            capture(&argv, None, move |ok, out, err| back(answer(ok, out, err)));
        });
    }

    fn forget_drive(&mut self, kit: &mut Kit) {
        self.admin(kit, DRIVE, &["games-drive-forget"], t!("settings-games-drive-forgotten"), |_, _| {});
    }

    fn drive_row(&self, kit: &Kit, d: &Drive, first: bool) -> Node<Msg> {
        let name = if d.label.is_empty() { t!("settings-games-drive-unnamed").to_string() } else { d.label.clone() };
        let key = if first { DRIVE.to_string() } else { format!("games-drive:{}", d.uuid) };
        let title = t!("settings-games-drive-name", name = name, size = size(d.bytes), fs = d.fs.as_str());
        let wait = crate::common::ui::wait_while(self.busy);
        if d.games {
            let stop = remove("list-remove-symbolic", t!("settings-games-drive-stop")).key(format!("games-stop:{}", d.uuid))
                .off_because(wait).on_press(on::<Games>(|me, kit| me.forget_drive(kit)));
            return line(title).key(key).sub(t!("settings-games-drive-in-use", place = games::DRIVE_DIR)).control(stop).node(kit);
        }
        match games::refused(&d.fs) {
            Some(why) => line(title).key(key).sub(why).node(kit),
            None => {
                let uuid = d.uuid.clone();
                let pick = button("drive-harddisk-symbolic", t!("settings-games-drive-use")).key(format!("games-use:{uuid}"))
                    .off_because(wait).on_press(on::<Games>(move |me, kit| me.use_drive(kit, uuid.clone())));
                line(title).key(key).control(pick).node(kit)
            }
        }
    }

    fn smooth_card(&self, kit: &Kit) -> Node<Msg> {
        let wait = crate::common::ui::wait_while(self.busy);
        let sound = switch(self.sound, t!("settings-games-sound"), |v| on::<Games>(move |me, k| me.set_sound(k, v)));
        let mut rows = vec![line(t!("settings-games-sound")).key(SOUND).control(sound).node(kit)];
        if let Some(fix) = self.split {
            let split = switch(fix, t!("settings-games-split"), |v| on::<Games>(move |me, k| me.set_split(k, v)))
                .off_because(wait);
            rows.push(line(t!("settings-games-split")).key(SPLIT).control(split).node(kit));
        }
        group(t!("settings-games-smooth")).icon("input-gaming-symbolic")
            .about(format!("{} {} {}", t!("settings-games-smooth-about"), t!("settings-games-sound-sub"), t!("settings-games-split-sub")))
            .span(5, 7)
            .rows(rows)
            .node(kit)
    }

    fn drive_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows: Vec<Node<Msg>> = self.drives.iter().enumerate().map(|(i, d)| self.drive_row(kit, d, i == 0)).collect();
        if rows.is_empty() {
            rows.push(line(t!("settings-games-drive-none")).key(DRIVE).node(kit));
        }
        group(t!("settings-games-drive-title")).icon("drive-harddisk-symbolic")
            .about(format!("{} {} {}", t!("settings-games-drive-about"), t!("settings-games-drive-steam"), t!("settings-games-drive-steam-sub")))
            .span(5, 7)
            .rows(rows)
            .grows()
            .node(kit)
    }
}

impl Games {

    fn steam_look_due(&self) -> bool {
        self.looks % STEAM_LOOK_EVERY == 0 && self.pads.has_steam_games()
    }
}

impl Page for Games {
    fn id(&self) -> &'static str {
        "games"
    }
    fn title(&self) -> &'static str {
        t!("settings-games-title")
    }
    fn icon(&self) -> &'static str {
        "input-gaming-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-games-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-games-help")
    }
    fn shown(&mut self, kit: &mut Kit) {
        self.look();
        self.read_drives(kit);
        self.read_steam(kit);
        self.pads.shown(kit);
    }
    fn hidden(&mut self, kit: &mut Kit) {
        self.pads.hidden(kit);
    }
    fn second(&mut self, kit: &mut Kit) -> bool {
        self.looks = self.looks.wrapping_add(1);
        let mut news = false;
        if self.steam_look_due() {
            self.read_open(kit);
            news = self.pads.look_running();
        }
        self.pads.second(kit) | news
    }
    fn intercept(&self, input: &crate::toolkit::input::Input) -> Option<Msg> {
        self.pads.intercept(input)
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        if self.pads.current().is_none() {
            let items = vec![headline(t!("settings-games-top").to_string()), self.pads.none_card(kit), self.smooth_card(kit), self.drive_card(kit), self.steam_card(kit)];
            return page(kit, self.title(), self.help(), items);
        }
        let mut right = vec![self.pads.profiles_card(kit), self.pads.picture_card(kit), self.pads.quick_card(kit)];
        right.extend(self.pads.other_card(kit));
        right.extend([self.smooth_card(kit), self.drive_card(kit), self.steam_card(kit)]);
        page_split(kit, self.title(), self.help(), vec![headline(self.pads.tell())], vec![self.pads.buttons_card(kit)], right)
    }
}

#[cfg(test)]
impl Games {

    pub(crate) fn made_up() -> Games {
        let drive = |uuid: &str, fs: &str, label: &str, games| Drive { uuid: uuid.into(), fs: fs.into(), label: label.into(), bytes: 1_000_204_886_016, games };
        Games { pads: Pads::made_up(), sound: false, split: Some(false), busy: false,
                drives: vec![drive("2222-aaaa", "ext4", "Games", true), drive("3333-bbbb", "ntfs", "Windows", false)],
                steam: steam::Steam::default(), looks: 0 }
    }
}

#[cfg(test)]
mod idle {
    use super::*;

    #[test]
    fn with_no_steam_game_the_page_never_looks_for_steam() {
        let mut page = Games { pads: Pads::default(), ..Games::made_up() };
        let due = (0..STEAM_LOOK_EVERY * 5).filter(|n| {
            page.looks = *n;
            page.steam_look_due()
        }).count();
        assert_eq!(due, 0);
        let mut page = Games::made_up();
        page.looks = STEAM_LOOK_EVERY;
        assert!(page.steam_look_due(), "a computer with Steam games still follows Steam");
    }
}

#[cfg(test)]
mod show_the_minimum {
    const KEPT: [&str; 12] = [
        "settings-ai-only-read-short", "settings-audio-bt-silent-about", "settings-audio-failed-about", "settings-bluetooth-hard-line-sub",
        "settings-default-apps-none-installed", "settings-display-nvidia-about", "settings-display-relative", "settings-games-drive-in-use",
        "settings-games-steam-open-sub", "settings-games-game-sub", "settings-network-not-reachable-sub", "settings-network-shared-sub",
    ];

    #[test]
    fn no_line_under_a_control_restates_it() {
        let pages = [
            include_str!("about.rs"), include_str!("ai_programs.rs"), include_str!("appearance/colours.rs"), include_str!("appearance/fonts_icons.rs"),
            include_str!("appearance/looks.rs"), include_str!("appearance/pointers.rs"), include_str!("appearance/reading.rs"), include_str!("audio.rs"),
            include_str!("blocking.rs"), include_str!("blocking/ports.rs"), include_str!("blocking/sharing.rs"), include_str!("bluetooth.rs"),
            include_str!("datetime.rs"), include_str!("default_apps.rs"), include_str!("display.rs"), include_str!("games.rs"), include_str!("games/steam.rs"),
        ];
        let mut extra = Vec::new();
        for src in pages {
            for part in src.split(".sub(t!(\"").skip(1) {
                let key = part.split('"').next().unwrap_or("");
                if !KEPT.contains(&key) {
                    extra.push(key.to_string());
                }
            }
        }
        assert!(extra.is_empty(), "lines under a control that are not state, a reason or a warning: {extra:?}");
    }
}
