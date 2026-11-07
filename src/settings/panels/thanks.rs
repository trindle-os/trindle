//! Thanks.

use std::sync::OnceLock;

use crate::common::run::on_thread;
use crate::settings::panels::network::banded;
use crate::settings::ui::{group, headline, line, link, on, page, Kit, Msg, Page};
use crate::toolkit::t;
use crate::toolkit::widget::{button, Node};

fn kinds() -> [(&'static str, &'static str, &'static [&'static str]); 6] {
    [
    (t!("settings-thanks-kind-underneath-it-all"), "computer-symbolic",
     &["Debian", "Linux", "Rust", "systemd", "Mesa", "Xwayland", "greetd", "polkit", "fwupd and the LVFS"]),
    (t!("settings-thanks-kind-the-desktop"), "view-app-grid-symbolic",
     &["Smithay", "wayland-rs", "Alacritty", "libinput", "wvkbd", "xdg-desktop-portal", "zbus",
       "AccessKit", "whisper.cpp"]),
    (t!("settings-thanks-kind-drawing-and-letters"), "applications-graphics-symbolic",
     &["tiny-skia", "rustybuzz", "Skrifa (Fontations)", "resvg", "fontdb", "ab_glyph", "Inter", "Radio Canada",
       "Noto", "Papirus"]),
    (t!("settings-thanks-kind-folders-and-drives"), "folder-symbolic",
     &["GVfs", "UDisks", "ntfs-3g, exfatprogs, dosfstools", "Samba", "libarchive", "snapper", "Btrfs"]),
    (t!("settings-thanks-kind-sound-networks-and-devices"), "audio-speakers-symbolic",
     &["PipeWire", "NetworkManager", "ModemManager", "BlueZ", "UPower", "power-profiles-daemon", "CUPS", "Avahi",
       "HPLIP"]),
    (t!("settings-thanks-kind-apps"), "package-x-generic-symbolic",
     &["Flatpak", "Flathub", "AppImageHub", "ODRS", "HaGeZi's DNS Blocklists", "serde, quick-xml, flate2"]),
]
}

fn shorter() -> [(&'static str, &'static str); 6] {
    [
    ("Skrifa (Fontations)", t!("settings-thanks-short-skrifa-fontations")),
    ("Smithay", t!("settings-thanks-short-smithay")),
    ("Papirus", t!("settings-thanks-short-papirus")),
    ("HaGeZi's DNS Blocklists", t!("settings-thanks-short-hagezi-s-dns-blocklists")),
    ("ab_glyph", t!("settings-thanks-short-ab-glyph")),
    ("ntfs-3g, exfatprogs, dosfstools", t!("settings-thanks-short-ntfs-3g-exfatprogs-dosfstools")),
]
}

pub fn projects() -> &'static [(&'static str, &'static str, &'static str, Option<&'static str>)] {
    static ALL: OnceLock<Vec<(&'static str, &'static str, &'static str, Option<&'static str>)>> = OnceLock::new();
    ALL.get_or_init(|| vec![
    ("Debian", t!("settings-thanks-project-debian"), "https://www.debian.org", Some("https://www.debian.org/donations")),
    ("Linux", t!("settings-thanks-project-linux"), "https://kernel.org", None),
    ("Rust", t!("settings-thanks-project-rust"), "https://www.rust-lang.org", None),
    ("tiny-skia", t!("settings-thanks-project-tiny-skia"), "https://github.com/linebender/tiny-skia", None),
    ("rustybuzz", t!("settings-thanks-project-rustybuzz"), "https://github.com/harfbuzz/rustybuzz", None),
    ("Skrifa (Fontations)", t!("settings-thanks-project-skrifa-fontations"), "https://github.com/googlefonts/fontations", None),
    ("resvg", t!("settings-thanks-project-resvg"), "https://github.com/linebender/resvg", None),
    ("AccessKit", t!("settings-thanks-project-accesskit"), "https://github.com/AccessKit/accesskit", None),
    ("zbus", t!("settings-thanks-project-zbus"), "https://github.com/dbus2/zbus", None),
    ("fontdb", t!("settings-thanks-project-fontdb"), "https://github.com/RazrFalcon/fontdb", None),
    ("Smithay", t!("settings-thanks-project-smithay"), "https://smithay.github.io", Some("https://opencollective.com/smithay")),
    ("Alacritty", t!("settings-thanks-project-alacritty"), "https://alacritty.org", None),
    ("systemd", t!("settings-thanks-project-systemd"), "https://systemd.io", None),
    ("Mesa", t!("settings-thanks-project-mesa"), "https://mesa3d.org", None),
    ("PipeWire", t!("settings-thanks-project-pipewire"), "https://pipewire.org", None),
    ("NetworkManager", t!("settings-thanks-project-networkmanager"), "https://networkmanager.dev", None),
    ("libinput", t!("settings-thanks-project-libinput"), "https://gitlab.freedesktop.org/libinput/libinput", None),
    ("wayland-rs", t!("settings-thanks-project-wayland-rs"), "https://github.com/Smithay/wayland-rs", None),
    ("Flatpak", t!("settings-thanks-project-flatpak"), "https://flatpak.org", None),
    ("Flathub", t!("settings-thanks-project-flathub"), "https://flathub.org", Some("https://flathub.org/donate")),
    ("Inter", t!("settings-thanks-project-inter"), "https://rsms.me/inter/", None),
    ("Radio Canada", t!("settings-thanks-project-radio-canada"), "https://github.com/cbcrc/radiocanadafonts", None),
    ("Papirus", t!("settings-thanks-project-papirus"), "https://github.com/PapirusDevelopmentTeam/papirus-icon-theme", None),
    ("greetd", t!("settings-thanks-project-greetd"), "https://sr.ht/~kennylevinsen/greetd/", None),
    ("Noto", t!("settings-thanks-project-noto"), "https://notofonts.github.io", None),
    ("Samba", t!("settings-thanks-project-samba"), "https://www.samba.org", None),
    ("libarchive", t!("settings-thanks-project-libarchive"), "https://www.libarchive.org", None),
    ("AppImageHub", t!("settings-thanks-project-appimagehub"), "https://appimage.github.io", None),
    ("ODRS", t!("settings-thanks-project-odrs"), "https://odrs.gnome.org", None),
    ("HaGeZi's DNS Blocklists", t!("settings-thanks-project-hagezi-s-dns-blocklists"), "https://github.com/hagezi/dns-blocklists", None),
    ("xdg-desktop-portal", t!("settings-thanks-project-xdg-desktop-portal"), "https://github.com/flatpak/xdg-desktop-portal", None),
    ("BlueZ", t!("settings-thanks-project-bluez"), "https://github.com/bluez/bluez", None),
    ("UPower", t!("settings-thanks-project-upower"), "https://upower.freedesktop.org", None),
    ("power-profiles-daemon", t!("settings-thanks-project-power-profiles-daemon"), "https://gitlab.freedesktop.org/upower/power-profiles-daemon", None),
    ("UDisks", t!("settings-thanks-project-udisks"), "https://github.com/storaged-project/udisks", None),
    ("CUPS", t!("settings-thanks-project-cups"), "https://openprinting.github.io/cups/", Some("https://openprinting.github.io/donations/")),
    ("polkit", t!("settings-thanks-project-polkit"), "https://github.com/polkit-org/polkit", None),
    ("Xwayland", t!("settings-thanks-project-xwayland"), "https://gitlab.freedesktop.org/xorg/xserver", Some("https://www.x.org/wiki/donations/")),
    ("wvkbd", t!("settings-thanks-project-wvkbd"), "https://github.com/jjsullivan5196/wvkbd", None),
    ("snapper", t!("settings-thanks-project-snapper"), "https://github.com/openSUSE/snapper", None),
    ("Btrfs", t!("settings-thanks-project-btrfs"), "https://btrfs.readthedocs.io", None),
    ("fwupd and the LVFS", t!("settings-thanks-project-fwupd-and-the-lvfs"), "https://fwupd.org", None),
    ("GVfs", t!("settings-thanks-project-gvfs"), "https://gitlab.gnome.org/GNOME/gvfs", None),
    ("ntfs-3g, exfatprogs, dosfstools", t!("settings-thanks-project-ntfs-3g-exfatprogs-dosfstools"), "https://github.com/tuxera/ntfs-3g", None),
    ("ModemManager", t!("settings-thanks-project-modemmanager"), "https://modemmanager.org", None),
    ("Avahi", t!("settings-thanks-project-avahi"), "https://avahi.org", None),
    ("HPLIP", t!("settings-thanks-project-hplip"), "https://developers.hp.com/hp-linux-imaging-and-printing", None),
    ("whisper.cpp", t!("settings-thanks-project-whisper-cpp"), "https://github.com/ggml-org/whisper.cpp", None),
    ("ab_glyph", t!("settings-thanks-project-ab-glyph"), "https://github.com/alexheretic/ab-glyph", None),
    ("serde, quick-xml, flate2", t!("settings-thanks-project-serde-quick-xml-flate2"), "https://serde.rs", None),
])
}

fn notices_path() -> std::path::PathBuf {
    let mut places = vec![std::path::PathBuf::from("/usr/share/doc/trindle/THIRD-PARTY.txt")];
    if let Ok(exe) = std::env::current_exe() {
        for dir in exe.ancestors().skip(1).take(4) {
            places.push(dir.join("share/licences/THIRD-PARTY.txt"));
        }
    }
    places.iter().find(|p| p.is_file()).cloned().unwrap_or_else(|| places[0].clone())
}

fn notices_counts() -> Vec<(String, usize)> {
    library_licences(&std::fs::read_to_string(notices_path()).unwrap_or_default())
}

fn open_notices() -> bool {
    let path = notices_path();
    path.is_file() && crate::common::run::spawn(&["trindle-text", &path.to_string_lossy()])
}

fn included() -> [(&'static str, &'static str, &'static str); 10] {
    [
        (t!("settings-thanks-included-the-compositor-name"), t!("settings-thanks-included-the-compositor"), t!("settings-thanks-where-copyright")),
        (t!("settings-thanks-included-smithay-0-7-0-name"), t!("settings-thanks-included-smithay-0-7-0"), t!("settings-thanks-included-smithay-0-7-0-where")),
        (t!("settings-thanks-included-screen-helpers-name"), t!("settings-thanks-included-screen-helpers"), t!("settings-thanks-where-copyright")),
        (t!("settings-thanks-included-spellbook-name"), t!("settings-thanks-included-spellbook"), t!("settings-thanks-included-spellbook-where")),
        (t!("settings-thanks-included-trindle-sans-name"), t!("settings-thanks-included-trindle-sans"), t!("settings-thanks-included-trindle-sans-where")),
        (t!("settings-thanks-included-emoji-and-spelling-lists-name"), t!("settings-thanks-included-emoji-and-spelling-lists"), t!("settings-thanks-included-emoji-and-spelling-lists-where")),
        (t!("settings-thanks-included-alacritty-name"), t!("settings-thanks-included-alacritty"), t!("settings-thanks-included-alacritty-where")),
        (t!("settings-thanks-included-voice-control-name"), t!("settings-thanks-included-voice-control"), t!("settings-thanks-included-voice-control-where")),
        (t!("settings-thanks-included-papirus-icons-name"), t!("settings-thanks-included-papirus-icons"), t!("settings-thanks-included-papirus-icons-where")),
        (t!("settings-thanks-included-firmware-name"), t!("settings-thanks-included-firmware"), t!("settings-thanks-included-firmware-where")),
    ]
}

fn library_licences(notices: &str) -> Vec<(String, usize)> {
    let mut counts: Vec<(String, usize)> = Vec::new();
    let mut lines = notices.lines().peekable();
    while let Some(line) = lines.next() {
        let Some(licence) = line.strip_prefix("Licence: ") else { continue };
        if lines.peek() == Some(&"Part of Trindle itself.") {
            continue;
        }
        match counts.iter_mut().find(|c| c.0 == licence) {
            Some(c) => c.1 += 1,
            None => counts.push((licence.to_string(), 1)),
        }
    }
    counts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    counts
}

fn rust_words(counts: Option<&[(String, usize)]>) -> String {
    let Some(counts) = counts else { return t!("settings-thanks-rust-reading").to_string() };
    let total: usize = counts.iter().map(|c| c.1).sum();
    match counts.first() {
        Some((licence, n)) if total > 0 => t!("settings-thanks-rust-counts", total = total, n = *n, licence = licence),
        _ => t!("settings-thanks-rust-missing").to_string(),
    }
}

fn licences_card(kit: &Kit, counts: Option<&[(String, usize)]>) -> Node<Msg> {
    let mut rows = vec![
        line("Trindle").sub(t!("settings-thanks-trindle-licence")).key("licences:trindle").node(kit),
    ];
    let mut read = vec![t!("settings-thanks-licences-about").to_string()];
    for (name, what, where_read) in included() {
        rows.push(line(name).sub(what).key(format!("licences:{name}")).node(kit));
        read.push(t!("settings-thanks-where", name = name, where_read = where_read));
    }
    let open = button("text-x-generic-symbolic", t!("settings-thanks-open-list")).on_press(on::<Thanks>(|_, k| {
        on_thread(open_notices, k.then::<Thanks, _>(|_, k, opened: bool| {
            k.started("licences:rust", opened, t!("settings-thanks-open-failed"));
        }));
    }));
    rows.push(line(t!("settings-thanks-rust")).sub(rust_words(counts)).key("licences:rust").control(open).node(kit));
    group(t!("settings-thanks-licences")).icon("text-x-generic-symbolic").about(read.join(" ")).span(6, 8).rows(banded(rows)).node(kit)
}

#[derive(Default)]
pub struct Thanks {
    counts: Option<Vec<(String, usize)>>,
}

impl Thanks {
    fn read_notices(&mut self, kit: &mut Kit) {
        on_thread(notices_counts, kit.then::<Thanks, _>(|me, _, counts: Vec<(String, usize)>| me.counts = Some(counts)));
    }
}

impl Page for Thanks {
    fn shown(&mut self, kit: &mut Kit) {
        self.read_notices(kit);
    }

    fn preload(&mut self, kit: &mut Kit) {
        self.read_notices(kit);
    }

    fn id(&self) -> &'static str {
        "thanks"
    }
    fn title(&self) -> &'static str {
        t!("settings-thanks-title")
    }
    fn icon(&self) -> &'static str {
        "emblem-favorite-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-thanks-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-thanks-help")
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let mut items = vec![
            headline(kit, t!("settings-thanks-headline", n = projects().len())),
        ];
        for (kind, icon, names) in kinds() {
            let mut longer = Vec::new();
            let rows = names.iter().filter_map(|n| projects().iter().find(|p| p.0 == *n)).map(|&(name, what, home, donate)| {
                let mut controls = Vec::new();
                let key = format!("thanks:{name}");
                if let Some(url) = donate {
                    controls.push(button("emblem-favorite-symbolic", t!("settings-thanks-donate", name = name)).leads_to(url)
                        .on_press(link(key.clone(), url)));
                }
                controls.push(crate::toolkit::widget::link(home).on_press(link(key.clone(), home)));
                let shown = match shorter().into_iter().find(|s| s.0 == name) {
                    Some((_, short)) => {
                        longer.push(t!("settings-thanks-longer", name = name, what = what));
                        short
                    }
                    None => what,
                };
                line(name).sub(shown).key(key).controls(controls).node(kit)
            }).collect::<Vec<_>>();
            let rows = banded(rows);
            let mut card = group(kind).icon(icon);
            if !longer.is_empty() {
                card = card.about(longer.join(" "));
            }
            if rows.len() > 8 {
                card = card.span(6, 8);
            }
            items.push(card.rows(rows).node(kit));
        }
        items.push(licences_card(kit, self.counts.as_deref()));
        page(kit, self.title(), self.help(), items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const WEB_START: &str = "<!-- thanks:start (made from src/settings/panels/thanks.rs) -->";
    const WEB_END: &str = "<!-- thanks:end -->";

    #[test]
    fn the_licences_card_counts_libraries_by_licence() {
        let notices = "x 1\nLicence: MIT\n\ny 1\nLicence: MIT\n\nz 1\nLicence: Zlib\n\ntrindle 1\nLicence: GPL-3.0-or-later\nPart of Trindle itself.\n";
        assert_eq!(library_licences(notices), vec![("MIT".to_string(), 2), ("Zlib".to_string(), 1)]);
        let real = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/share/licences/THIRD-PARTY.txt")).unwrap();
        assert!(library_licences(&real).iter().map(|c| c.1).sum::<usize>() > 300);
    }

    fn escape(text: &str) -> String {
        text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
    }

    fn website_cards() -> String {
        let mut out = String::from("<div class=\"grid g3 thanks-grid\">\n");
        for &(name, what, home, donate) in projects() {
            out.push_str(&format!(
                "  <div class=\"panel lift\"><h3><a href=\"{}\" target=\"_blank\" rel=\"noopener\">{}</a></h3><p>{}</p>",
                escape(home), escape(name), escape(what)));
            if let Some(url) = donate {
                out.push_str(&format!(
                    "<p class=\"thanks-give\"><a href=\"{}\" target=\"_blank\" rel=\"noopener\"><svg class=\"ico\" viewBox=\"0 0 24 24\" \
                     fill=\"none\" stroke=\"currentColor\" stroke-width=\"1.7\" stroke-linecap=\"round\" stroke-linejoin=\"round\" \
                     aria-hidden=\"true\" focusable=\"false\"><path d=\"M12 20s-7-4.3-7-9.2A4 4 0 0 1 12 8a4 4 0 0 1 7 2.8C19 15.7 12 20 12 20z\"/></svg>\
                     Donate to {}</a></p>",
                    escape(url), escape(name)));
            }
            out.push_str("</div>\n");
        }
        out.push_str("</div>");
        out
    }

    #[test]
    fn every_project_has_one_card() {
        for p in projects() {
            let found = kinds().iter().filter(|k| k.2.contains(&p.0)).count();
            assert_eq!(found, 1, "{} is in {found} cards", p.0);
        }
        let named: usize = kinds().iter().map(|k| k.2.len()).sum();
        assert_eq!(named, projects().len(), "a card names a project that is not in the list");
        for (name, _) in shorter() {
            assert!(projects().iter().any(|p| p.0 == name), "{name} has a short line but no project");
        }
    }

    #[test]
    fn the_website_thanks_the_same_projects_as_the_app() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../trindle-site/thanks.html");
        let Ok(page) = std::fs::read_to_string(&path) else {
            println!("skipped: ../trindle-site/thanks.html is not here");
            return;
        };
        let (Some(a), Some(b)) = (page.find(WEB_START), page.find(WEB_END)) else {
            panic!("trindle-site/thanks.html has lost its thanks:start and thanks:end marks");
        };
        let now = &page[a + WEB_START.len()..b];
        let made = format!("\n{}\n", website_cards());
        if now != made {
            if std::env::var_os("TRINDLE_WRITE_THANKS").is_some() {
                let fresh = format!("{}{}{}", &page[..a + WEB_START.len()], made, &page[b..]);
                std::fs::write(&path, fresh).expect("write trindle-site/thanks.html");
                return;
            }
            panic!("trindle-site/thanks.html is out of step with the Thanks page: run TRINDLE_WRITE_THANKS=1 cargo test thanks");
        }
    }
}
