//! Hardware: the computer's devices by kind, and which of them lack a driver or firmware.

use crate::common::no_driver::{self, Device, Inventory, Kind, State};
use crate::common::run::on_thread;
use crate::settings::ui::{admin_step_secret, card, group, headline, help_link, line, loading, on, page, Kit, Msg, Page};
use crate::toolkit::t;
use crate::toolkit::widget::{banded_list, button, entry, row, word_button, Node};
use trindle_shared::luks_chip::{refused_disk, REFUSED};
use zeroize::Zeroizing;

const CHIP_KEY: &str = "hardware:chip";

fn kind_name(k: Kind) -> &'static str {
    match k {
        Kind::Graphics => t!("settings-hardware-kind-graphics"),
        Kind::Sound => t!("settings-hardware-kind-sound"),
        Kind::Network => t!("settings-hardware-kind-network"),
        Kind::Bluetooth => t!("settings-hardware-kind-bluetooth"),
        Kind::Storage => t!("settings-hardware-kind-storage"),
        Kind::Input => t!("settings-hardware-kind-input"),
        Kind::Camera => t!("settings-hardware-kind-camera"),
        Kind::Other => t!("settings-hardware-kind-other"),
    }
}

fn kind_icon(k: Kind) -> &'static str {
    match k {
        Kind::Graphics => "video-display-symbolic",
        Kind::Sound => "audio-card-symbolic",
        Kind::Network => "network-wired-symbolic",
        Kind::Bluetooth => "bluetooth-symbolic",
        Kind::Storage => "drive-harddisk-symbolic",
        Kind::Input => "input-keyboard-symbolic",
        Kind::Camera => "camera-web-symbolic",
        Kind::Other => "computer-symbolic",
    }
}

fn status(d: &Device) -> String {
    match (d.state, &d.driver) {
        (State::Working, Some(name)) => t!("settings-hardware-driver", name = name.as_str()),
        (State::Working, None) => t!("settings-hardware-working").to_string(),
        (State::NoDriver, _) => t!("settings-hardware-no-driver").to_string(),
        (State::NoFirmware, _) => t!("settings-hardware-no-firmware").to_string(),
        (State::Idle, _) => t!("settings-hardware-idle").to_string(),
    }
}

fn device_line(kit: &Kit, d: &Device) -> Node<Msg> {
    let title = d.name.clone().unwrap_or_else(|| t!("settings-hardware-unnamed", id = d.id.as_str()));
    line(title).sub(status(d)).key(format!("hardware:{}", d.id)).node(kit)
}

#[derive(Default)]
pub struct Hardware {
    inventory: Option<Inventory>,

    chip: Option<String>,

    asking: bool,
    password: Zeroizing<String>,
}

impl Hardware {
    fn read(&mut self, kit: &mut Kit) {
        self.chip = refused_disk(std::path::Path::new(REFUSED), 0);
        on_thread(no_driver::inventory, kit.then::<Hardware, _>(|me, _, inv: Inventory| me.inventory = Some(inv)));
    }

    fn close_chip(&mut self) {
        self.asking = false;
        self.password = Zeroizing::default();
    }

    fn use_chip(&mut self, kit: &mut Kit) {
        let Some(uuid) = self.chip.clone() else { return };
        if self.password.is_empty() {
            kit.say(CHIP_KEY, t!("settings-hardware-chip-type"), false, false);
            return;
        }
        let password = std::mem::take(&mut self.password);
        self.close_chip();
        admin_step_secret::<Hardware>(kit, CHIP_KEY, &["chip-reseal", &uuid], password, t!("settings-hardware-chip-waiting"),
                                      t!("settings-hardware-chip-done"), |me, _, worked| {
            if worked {
                me.chip = None;
            }
        });
    }

    fn chip_card(&self, kit: &Kit) -> Option<Node<Msg>> {
        self.chip.as_ref()?;
        let mut chip = line(t!("settings-hardware-chip-refused")).key(CHIP_KEY);
        if self.asking {
            let go = || on::<Hardware>(|me, k| me.use_chip(k));
            chip = chip.below(row(vec![
                entry(self.password.to_string(), t!("settings-hardware-chip-password"),
                      |t| on::<Hardware>(move |me, _| me.password = Zeroizing::new(t.clone())))
                    .secret().key("hardware:chip:password").on_submit(go()),
                word_button(t!("settings-hardware-chip-again")).on_press(go()),
                button("window-close-symbolic", t!("settings-hardware-chip-cancel")).on_press(on::<Hardware>(|me, _| me.close_chip())),
            ]).gap(8.0));
        } else {
            chip = chip.control(word_button(t!("settings-hardware-chip-again")).on_press(on::<Hardware>(|me, _| me.asking = true)));
        }
        Some(group(t!("settings-hardware-chip")).icon("security-high-symbolic").rows(vec![chip.node(kit)]).node(kit))
    }

    #[cfg(test)]
    pub(crate) fn made_up() -> Hardware {
        let device = |kind, id: &str, name: Option<&str>, driver: Option<&str>, state, problem: Option<&str>| Device {
            kind, id: id.into(), name: name.map(Into::into), driver: driver.map(Into::into), state, problem: problem.map(Into::into),
        };
        let devices = vec![
            device(Kind::Graphics, "8086:9a49", Some("Intel Corporation Iris Xe Graphics"), Some("i915"), State::Working, None),
            device(Kind::Sound, "8086:51c8", Some("Intel Corporation Sound controller"), Some("snd_hda_intel"), State::Working, None),
            device(Kind::Network, "14c3:7902", Some("MediaTek MT7902 Wi-Fi"), None, State::NoDriver,
                   Some("MediaTek MT7902 Wi-Fi doesn't have a driver here yet. Linux 7.1 is the first version to include one.")),
            device(Kind::Camera, "0c45:6a10", None, Some("uvcvideo"), State::Working, None),
        ];
        Hardware::with(Inventory { devices, loose: Vec::new() })
    }

    #[cfg(test)]
    pub(crate) fn chip_refused(mut self, uuid: &str) -> Hardware {
        self.chip = Some(uuid.into());
        self
    }

    #[cfg(test)]
    pub(crate) fn chip_asking(mut self) -> Hardware {
        self.asking = true;
        self
    }

    #[cfg(test)]
    pub(crate) fn with(inventory: Inventory) -> Hardware {
        Hardware { inventory: Some(inventory), ..Hardware::default() }
    }
}

impl Page for Hardware {
    fn shown(&mut self, kit: &mut Kit) {
        self.read(kit);
    }

    fn preload(&mut self, kit: &mut Kit) {
        self.read(kit);
    }

    fn id(&self) -> &'static str {
        "hardware"
    }
    fn title(&self) -> &'static str {
        t!("settings-hardware-title")
    }
    fn icon(&self) -> &'static str {
        "cpu-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-hardware-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-hardware-help")
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let Some(inv) = &self.inventory else { return page(kit, self.title(), self.help(), vec![loading()]) };
        let problems = inv.problems();
        let mut items = vec![headline(match problems.len() {
            0 => t!("settings-hardware-all-well", n = inv.devices.len()),
            n => t!("settings-hardware-trouble", n = n),
        })];
        items.extend(self.chip_card(kit));
        if !problems.is_empty() {
            let rows = problems.into_iter()
                .map(|l| line(l).control(help_link(t!("common-no-driver-more"), no_driver::HELP)).node(kit)).collect();
            items.push(card(None, rows));
        }
        for kind in no_driver::KINDS {
            let mut devices = inv.of_kind(kind);

            devices.sort_by_key(|d| d.state == State::Working);
            if devices.is_empty() {
                continue;
            }
            let rows = devices.into_iter().map(|d| device_line(kit, d)).collect::<Vec<_>>();
            let wide = rows.len() > 8;
            let mut card = group(kind_name(kind)).icon(kind_icon(kind));
            if wide {
                card = card.span(6, 8);
            }
            items.push(card.rows(banded_list(rows)).node(kit));
        }
        page(kit, self.title(), self.help(), items)
    }
}
