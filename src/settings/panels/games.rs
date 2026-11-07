//! Games.

mod steam;

use super::pads::Pads;
use crate::common::games::{self, Drive};
use crate::common::run::{admin_error, admin_helper, capture};
use crate::settings::ui::{group, headline, line, on, page, page_split, Kit, Msg, Page, Raw};
use crate::toolkit::t;
use crate::toolkit::widget::{button, remove, switch, Node};

const SOUND: &str = "games:sound";
const SPLIT: &str = "games:split-lock";
const DRIVE: &str = "games:drive";

#[derive(Default)]
pub struct Games {
    pub(super) pads: Pads,
    sound: bool,
    split: Option<bool>,
    drives: Vec<Drive>,
    busy: bool,
    steam: steam::Steam,
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

    fn admin(&mut self, kit: &mut Kit, key: &'static str, args: Vec<String>, success: &'static str, after: fn(&mut Games, &mut Kit)) {
        self.busy = true;
        kit.say(key, t!("settings-games-waiting"), true, true);
        let back = kit.then::<Games, (bool, String)>(move |me, k, (ok, err)| {
            me.busy = false;
            let err = admin_error(&err);
            k.done(key, if ok { None } else { Some(&err) }, success);
            me.look();
            if ok {
                after(me, k);
            }
            me.read_drives(k);
        });
        let helper = admin_helper();
        let mut argv = vec!["pkexec", helper.as_str()];
        argv.extend(args.iter().map(String::as_str));
        capture(&argv, None, move |ok, _, err| back((ok, err)));
    }

    fn set_split(&mut self, kit: &mut Kit, fix: bool) {
        self.split = Some(fix);
        self.admin(kit, SPLIT, vec!["split-lock-slowing".into(), if fix { "off" } else { "on" }.into()],
                   if fix { t!("settings-games-split-done-on") } else { t!("settings-games-split-done-off") }, |_, _| {});
    }

    fn use_drive(&mut self, kit: &mut Kit, uuid: String) {
        self.admin(kit, DRIVE, vec!["games-drive".into(), uuid], t!("settings-games-drive-done"), |_, k| {
            let back = k.settles_quietly::<Games, String, Raw>(DRIVE, t!("settings-games-drive-steam-failed"), |_, _, _: Option<String>| {});
            let argv = games::steam_reach();
            let argv: Vec<&str> = argv.iter().map(String::as_str).collect();
            capture(&argv, None, move |ok, out, err| back(answer(ok, out, err)));
        });
    }

    fn forget_drive(&mut self, kit: &mut Kit) {
        self.admin(kit, DRIVE, vec!["games-drive-forget".into()], t!("settings-games-drive-forgotten"), |_, _| {});
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
                line(title).key(key).sub(t!("settings-games-drive-ready")).control(pick).node(kit)
            }
        }
    }

    fn smooth_card(&self, kit: &Kit) -> Node<Msg> {
        let wait = crate::common::ui::wait_while(self.busy);
        let sound = switch(self.sound, t!("settings-games-sound"), |v| on::<Games>(move |me, k| me.set_sound(k, v)));
        let mut rows = vec![line(t!("settings-games-sound")).sub(t!("settings-games-sound-sub")).key(SOUND).control(sound).node(kit)];
        if let Some(fix) = self.split {
            let split = switch(fix, t!("settings-games-split"), |v| on::<Games>(move |me, k| me.set_split(k, v)))
                .off_because(wait);
            rows.push(line(t!("settings-games-split")).sub(t!("settings-games-split-sub")).key(SPLIT).control(split).node(kit));
        }
        group(t!("settings-games-smooth")).icon("input-gaming-symbolic")
            .about(t!("settings-games-smooth-about"))
            .span(5, 7)
            .rows(rows)
            .node(kit)
    }

    fn drive_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows: Vec<Node<Msg>> = self.drives.iter().enumerate().map(|(i, d)| self.drive_row(kit, d, i == 0)).collect();
        if rows.is_empty() {
            rows.push(line(t!("settings-games-drive-none")).key(DRIVE).sub(t!("settings-games-drive-none-sub")).node(kit));
        }
        if self.drives.iter().any(|d| d.games) {
            rows.push(line(t!("settings-games-drive-steam")).sub(t!("settings-games-drive-steam-sub")).node(kit));
        }
        group(t!("settings-games-drive-title")).icon("drive-harddisk-symbolic")
            .about(t!("settings-games-drive-about"))
            .span(5, 7)
            .rows(rows)
            .node(kit)
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
        self.pads.second(kit)
    }
    fn intercept(&self, input: &crate::toolkit::input::Input) -> Option<Msg> {
        self.pads.intercept(input)
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        if self.pads.current().is_none() {
            let items = vec![headline(kit, t!("settings-games-top").to_string()), self.pads.none_card(kit), self.smooth_card(kit), self.drive_card(kit), self.steam_card(kit)];
            return page(kit, self.title(), self.help(), items);
        }
        let mut right = vec![self.pads.profiles_card(kit), self.pads.picture_card(kit), self.pads.quick_card(kit)];
        right.extend(self.pads.other_card(kit));
        right.extend([self.smooth_card(kit), self.drive_card(kit), self.steam_card(kit)]);
        page_split(kit, self.title(), self.help(), vec![headline(kit, self.pads.tell())], vec![self.pads.buttons_card(kit)], right)
    }
}

#[cfg(test)]
impl Games {
    pub(crate) fn made_up() -> Games {
        let drive = |uuid: &str, fs: &str, label: &str, games| Drive { uuid: uuid.into(), fs: fs.into(), label: label.into(), bytes: 1_000_204_886_016, games };
        Games { pads: Pads::made_up(), sound: false, split: Some(false), busy: false,
                drives: vec![drive("2222-aaaa", "ext4", "Games", true), drive("3333-bbbb", "ntfs", "Windows", false)],
                steam: steam::Steam::default() }
    }
}
