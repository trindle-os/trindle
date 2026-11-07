//! Trindle.

#[macro_use]
mod said;
mod activity;
mod admin;
mod controller;
mod disks;
mod dns;
mod common;
mod corner;
mod greeter;
mod icons;
mod installer;
mod keyring;
mod mcp;
mod plugins;
mod polkit;
mod ring;
mod screenshot;
mod session;
mod settings;
mod store;
mod text;
mod thunder;
mod usbretry;
mod report;
mod voice;
mod welcome;

use std::process::ExitCode;

use trindle_toolkit as toolkit;

#[used]
#[link_section = ".init_array"]
static TOOLKIT_HOST: extern "C" fn() = common::host::set;

pub const NAMES: &[&str] = &[
    "trindle-ring", "trindle-settings", "trindle-apps", "trindle-greeter",
    "trindle-wallpaper", "trindle-idle", "trindle-wsdd", "trindle-icons", "trindle-scale",
    "trindle-session", "trindle-greeter-session", "trindle-autostart",
    "trindle-admin", "trindle-voice-setup", "trindle-lock", "trindle-clipboard",
    "trindle-installer", "trindle-welcome", "trindle-live-welcome", "trindle-windows-grub", "trindle-starting", "trindle-screensaver",
    "trindle-nightlight", "trindle-screenshot", "trindle-brightness", "trindle-activity",
    "trindle-portal", "trindle-drives", "trindle-eye-dropper", "trindle-keyring", "trindle-mcp", "trindle-boxed",
    "trindle-plugin", "trindle-dns", "trindle-thunder", "trindle-controller", "trindle-usb-retry",
    "trindle-text", "trindle-terminal", "trindle-report",
    "trindle-disks", "trindle-store",
];

fn part_called(args: &mut Vec<String>) -> String {
    let called = args
        .first()
        .map(|a| a.rsplit('/').next().unwrap_or(a).to_string())
        .unwrap_or_default();
    if called == "trindle" && args.len() > 1 {
        let p = args.remove(1);
        format!("trindle-{p}")
    } else {
        called
    }
}

fn started_bare(part: &str, args: &[String]) -> Option<i32> {
    if part == store::fence::INSIDE {
        return Some(store::fence::inside(args));
    }
    if store::fence::STAND_INS.contains(&part) && std::env::var_os(store::fence::BROKER_ENV).is_some() {
        return Some(store::fence_broker::stand_in(part, &args[1..]));
    }
    if !part.starts_with("trindle") && store::fence::programs(&store::fence::root()).iter().any(|(n, _)| *n == part) {
        return Some(store::fence::main_for(part, &args[1..]));
    }
    if part == store::home_net::KEEPER {
        return Some(store::home_net::keeper_main(args));
    }
    if part.strip_prefix("trindle-") == Some(session::moving::DECODING) {
        return Some(session::decoding::main(args.to_vec()));
    }
    None
}

fn main() -> ExitCode {
    common::memory::few_heaps();
    let mut args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("--names") {
        println!("{}", NAMES.join("\n"));
        return ExitCode::SUCCESS;
    }
    let part = part_called(&mut args);
    if let Some(code) = started_bare(&part, &args) {
        return ExitCode::from(code as u8);
    }
    common::renewed::named(&part);
    if !common::renewed::close(&part) {
        eprintln!("{part}: this system would not mark it as closed to other programs");
    }
    common::crash::watch(&part);
    ExitCode::from(run_part(&part, args) as u8)
}

fn run_part(part: &str, args: Vec<String>) -> i32 {
    match part {
        "trindle-ring" => ring::main(args),
        "trindle-settings" => settings::main(args),
        "trindle-apps" | "trindle-store" => store::main(args),
        "trindle-text" => text::main(args),
        "trindle-activity" => activity::main(args),
        "trindle-drives" | "trindle-disks" => disks::main(args),
        "trindle-eye-dropper" => screenshot::colours::main(args),
        "trindle-report" => report::main(args),
        "trindle-keyring" => keyring::main(args),
        "trindle-mcp" => mcp::main(args),
        "trindle-boxed" => store::boxed::first_apps::main(args),
        "trindle-plugin" => plugins::main(args),
        "trindle-terminal" => if common::apps::start_terminal(None, &args[1..]) { 0 } else { 1 },
        "trindle-greeter" => greeter::main(args),
        "trindle-wallpaper" => session::wallpaper::main(args),
        "trindle-idle" => session::idle::main(args),
        "trindle-wsdd" => session::wsdd::main(args),
        "trindle-nightlight" => session::nightlight::main(args),
        "trindle-brightness" => session::brightness::main(args),
        "trindle-icons" => icons::main(args),
        "trindle-scale" => session::scale::main(args),
        "trindle-session" => session::start::session(args),
        "trindle-starting" => session::starting::main(args),
        "trindle-screensaver" => session::screensaver::main(args),
        "trindle-greeter-session" => session::start::greeter_session(args),
        "trindle-autostart" => session::start::autostart(args),
        "trindle-admin" => admin::main(args),
        "trindle-dns" => dns::main(args),
        "trindle-thunder" => thunder::main(args),
        "trindle-controller" => controller::main(args),
        "trindle-usb-retry" => usbretry::main(args),
        "trindle-voice-setup" => voice::setup::main(args),
        "trindle-lock" => session::idle::lock(args),
        "trindle-clipboard" => common::clipboard::main(args),
        "trindle-screenshot" => screenshot::main(args),
        "trindle-portal" => screenshot::portal::main(args),
        "trindle-installer" => installer::main(args),
        "trindle-welcome" => welcome::main(args),
        "trindle-live-welcome" => installer::welcome::main(args),
        "42_trindle_snapshots" | "trindle-snapshots-grub" => common::snapshots::grub_main(),
        "41_trindle_windows" | "trindle-windows-grub" => common::otheros::grub_main(),
        _ => {
            eprintln!(
                "trindle: call me by one of these names: {}",
                NAMES.join(", ")
            );
            2
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_invocation_name_is_the_name_of_another_trindle_package() {
        for line in include_str!("../packages.txt").lines() {
            let package = line.split('#').next().unwrap_or("").trim();
            if package.starts_with("trindle-") {
                assert!(!super::NAMES.contains(&package), "{package} is another package's program, and also a name this one answers to");
            }
        }
    }

    #[test]
    fn readme_lists_every_package() {
        let names = |text: &str| {
            let mut v: Vec<String> = text.lines().map(|l| l.split('#').next().unwrap_or("").trim().to_string())
                .filter(|l| !l.is_empty()).collect();
            v.sort();
            v
        };
        let want = names(include_str!("../packages.txt"));
        let readme = include_str!("../README.md");
        let table = readme.split("<!-- packages -->").nth(1).and_then(|t| t.split("<!-- /packages -->").next()).unwrap_or("");
        let mut have: Vec<String> = table.lines().filter_map(|l| l.strip_prefix("| `"))
            .filter_map(|l| l.split('`').next()).map(String::from).collect();
        have.sort();
        assert_eq!(want, have, "README.md and packages.txt disagree");
    }

    #[test]
    fn the_readme_and_the_changelog_name_cargos_version() {
        let version = env!("CARGO_PKG_VERSION");
        assert!(include_str!("../README.md").contains(&format!("badge/version-{}-", version.replace('-', "--"))), "README.md's badge is not {version}");
        let newest = include_str!("../CHANGELOG.md").lines().find(|l| l.starts_with("## ")).unwrap_or("");
        assert!(newest.starts_with(&format!("## {version} ")), "CHANGELOG.md's newest heading is not {version}: {newest}");
    }

    #[test]
    fn firmware_scanners_and_printers_come_with_trindle() {
        let list: Vec<String> = include_str!("../system/apt/hardware-packages.txt").lines()
            .map(|l| l.split('#').next().unwrap_or("").trim().to_string()).filter(|l| !l.is_empty()).collect();
        for need in ["firmware-iwlwifi", "firmware-realtek", "firmware-mediatek", "firmware-atheros", "bluez-firmware",
            "firmware-amd-graphics", "firmware-intel-graphics", "firmware-nvidia-graphics", "firmware-sof-signed", "firmware-intel-sound",
            "libsane1", "sane-utils", "sane-airscan", "hplip", "libsane-hpaio", "printer-driver-all", "printer-driver-gutenprint",
            "printer-driver-hpcups", "printer-driver-escpr", "printer-driver-brlaser"] {
            assert!(list.iter().any(|p| p == need), "{need} is missing from hardware-packages.txt");
        }
        assert!(list.iter().all(|p| !p.starts_with("linux-image") && !p.starts_with("linux-headers")), "the kernel is not a hardware package");
        assert!(include_str!("../tools/iso/build.sh").contains("system/apt/hardware-packages.txt"), "the image must install the list");
        assert!(include_str!("../tools/deb/build.sh").contains("system/apt/hardware-packages.txt"), "the package must recommend the list");
        let thanks = include_str!("../share/lang/en/settings-thanks.ftl");
        assert!(thanks.contains("settings-thanks-included-firmware = Debian's firmware. Not free software."), "the Thanks page must say the firmware is not free software");
    }

    #[test]
    fn hardware_lists_use_only_stable_and_named_backports() {
        const FIRMWARE_META: [&str; 3] = ["firmware-linux", "firmware-linux-nonfree", "firmware-linux-free"];
        let code = |text: &str| -> Vec<String> {
            text.lines().map(|l| l.split('#').next().unwrap_or("").trim().to_string()).filter(|l| !l.is_empty()).collect()
        };
        let hardware_text = include_str!("../system/apt/hardware-packages.txt");
        let pin = include_str!("../system/apt/trindle-backports.pref");
        let sources = include_str!("../system/apt/trindle-backports.sources");
        let iso_list = include_str!("../tools/iso/config/package-lists/trindle.list.chroot");
        let iso_build = include_str!("../tools/iso/build.sh");

        let mut named: Vec<String> = Vec::new();
        let mut names: Vec<String> = Vec::new();
        for line in code(pin) {
            if let Some(rest) = line.strip_prefix("Package:") {
                names = rest.split_whitespace().map(String::from).collect();
            } else if line.starts_with("Pin-Priority:") && line.ends_with("800") {
                named.extend(names.iter().cloned());
            }
        }
        assert!(!named.is_empty() && !named.iter().any(|n| n == "*"), "the pin file must name its packages");

        let marked: Vec<String> = hardware_text.lines()
            .filter(|l| l.split('#').nth(1).is_some_and(|c| c.trim_start().starts_with("backports")))
            .map(|l| l.split('#').next().unwrap_or("").trim().to_string()).collect();
        for m in &marked {
            assert!(named.contains(m), "{m} is marked backports but the pin file does not name it");
        }
        for n in &named {
            assert!(!n.starts_with("linux-"), "{n} is a kernel package: the kernel is Debian stable's, never from backports");
            assert!(FIRMWARE_META.contains(&n.as_str()) || marked.contains(n), "{n} is pinned to backports but is not marked backports in hardware-packages.txt");
        }
        let listed: Vec<String> = code(hardware_text).into_iter()
            .chain(code(iso_list).into_iter().flat_map(|l| l.split_whitespace().map(String::from).collect::<Vec<_>>()))
            .collect();
        for n in named.iter().filter(|n| !FIRMWARE_META.contains(&n.as_str())) {
            assert!(listed.contains(n), "{n} is pinned but no list installs it");
        }
        assert!(listed.iter().any(|p| p == "linux-image-amd64"), "the image must install the kernel");
        for need in FIRMWARE_META {
            assert!(named.iter().any(|n| n == need), "{need} must be pinned to backports or apt keeps the firmware back");
        }
        for (what, text) in [("image build", iso_build), ("system files", include_str!("../tools/system-files.sh"))] {
            assert!(text.contains("trindle-backports.pref") && !text.contains("Pin-Priority"), "{what} must copy the one pin file, not define a list");
        }

        for (what, text) in [("sources", sources), ("pin file", pin), ("image build", iso_build)] {
            for line in code(text).into_iter().filter(|l| what != "image build" || l.contains("--distribution")) {
                let lower = line.to_lowercase();
                let suites = lower.split(|c: char| !c.is_alphanumeric()).collect::<Vec<_>>();
                for bad in ["testing", "unstable", "sid", "experimental"] {
                    assert!(!suites.contains(&bad), "{what} names {bad}: {line}");
                }
            }
        }
        assert!(sources.contains("Suites: trixie-backports"), "the backports source is trixie-backports");
        assert!(sources.contains("non-free-firmware"), "the firmware component must be on");
        assert!(!sources.contains("trixie-backports-sloppy"), "no sloppy backports");
        assert!(iso_build.contains("non-free-firmware"), "the image build must have the firmware component");
        assert!(iso_build.contains("--distribution trixie"), "the image is built from stable");
    }
}
