//! The Help page's content.

use std::sync::OnceLock;

use crate::common::key_enrolment::{self, Lead};
use crate::toolkit::t;

pub enum Block {
    P(&'static str),
    Tip(&'static str),
    Steps(&'static [&'static str]),
    Img(&'static str, &'static str),
}

pub struct Topic {
    pub id: &'static str,
    pub title: &'static str,
    pub keywords: &'static str,
    pub blocks: &'static [Block],
}

fn key_blocks(lead: Lead) -> Vec<Block> {
    let n = key_enrolment::before_restart(lead);
    vec![Block::P(n.why), Block::Steps(leak(n.steps)), Block::Tip(n.missed)]
}

fn leak<T>(v: Vec<T>) -> &'static [T] {
    Box::leak(v.into_boxed_slice())
}

fn getting_started() -> Vec<Topic> {
    vec![
        Topic { id: "ai-programs", title: t!("settings-help-topics-ai-programs-title"), keywords: t!("settings-help-topics-ai-programs-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-ai-programs-p1")),
            Block::P(t!("settings-help-topics-ai-programs-p2")),
            Block::Steps(leak(vec![t!("settings-help-topics-ai-programs-step3-1"), t!("settings-help-topics-ai-programs-step3-2"), t!("settings-help-topics-ai-programs-step3-3"), t!("settings-help-topics-ai-programs-step3-4")])),
            Block::P(t!("settings-help-topics-ai-programs-p4")),
        ]) },
        Topic { id: "welcome", title: t!("settings-help-topics-welcome-title"), keywords: t!("settings-help-topics-welcome-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-welcome-p1")),
            Block::Img("ring-first.png", t!("settings-help-topics-welcome-pic2")),
            Block::P(t!("settings-help-topics-welcome-p3")),
            Block::Steps(leak(vec![t!("settings-help-topics-welcome-step4-1"), t!("settings-help-topics-welcome-step4-2"), t!("settings-help-topics-welcome-step4-3")])),
            Block::P(t!("settings-help-topics-welcome-p5")),
            Block::P(t!("settings-help-topics-welcome-p6")),
        ]) },
        Topic { id: "ring", title: t!("settings-help-topics-ring-title"), keywords: t!("settings-help-topics-ring-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-ring-p1")),
            Block::P(t!("settings-help-topics-ring-p2")),
            Block::P(t!("settings-help-topics-ring-p3")),
            Block::P(t!("settings-help-topics-ring-p4")),
            Block::P(t!("settings-help-topics-ring-p5")),
            Block::Img("ring-all.png", t!("settings-help-topics-ring-pic6")),
            Block::Steps(leak(vec![t!("settings-help-topics-ring-step7-1"), t!("settings-help-topics-ring-step7-2"), t!("settings-help-topics-ring-step7-3"), t!("settings-help-topics-ring-step7-4")])),
            Block::Tip(t!("settings-help-topics-ring-tip8")),
        ]) },
        Topic { id: "awake", title: t!("settings-help-topics-awake-title"), keywords: t!("settings-help-topics-awake-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-awake-p1")),
            Block::Tip(t!("settings-help-topics-awake-tip2")),
        ]) },
        Topic { id: "favourites", title: t!("settings-help-topics-favourites-title"), keywords: t!("settings-help-topics-favourites-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-favourites-p1")),
            Block::P(t!("settings-help-topics-favourites-p2")),
            Block::P(t!("settings-help-topics-favourites-p3")),
            Block::Img("ring-menu.png", t!("settings-help-topics-favourites-pic4")),
            Block::P(t!("settings-help-topics-favourites-p5")),
        ]) },
        Topic { id: "switcher", title: t!("settings-help-topics-switcher-title"), keywords: t!("settings-help-topics-switcher-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-switcher-p1")),
            Block::P(t!("settings-help-topics-switcher-p2")),
            Block::P(t!("settings-help-topics-switcher-p3")),
            Block::P(t!("settings-help-topics-switcher-p4")),
        ]) },
        Topic { id: "notifications", title: t!("settings-help-topics-notifications-title"), keywords: t!("settings-help-topics-notifications-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-notifications-p1")),
            Block::P(t!("settings-help-topics-notifications-p2")),
            Block::P(t!("settings-help-topics-notifications-p3")),
            Block::P(t!("settings-help-topics-notifications-p4")),
            Block::P(t!("settings-help-topics-notifications-p5")),
            Block::Tip(t!("settings-help-topics-notifications-tip6")),
        ]) },
    ]
}

fn programs() -> Vec<Topic> {
    vec![
        Topic { id: "apps", title: t!("settings-help-topics-apps-title"), keywords: t!("settings-help-topics-apps-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-apps-p1")),
            Block::P(t!("settings-help-topics-apps-p2")),
            Block::Img("store-app.png", t!("settings-help-topics-apps-pic3")),
            Block::Steps(leak(vec![t!("settings-help-topics-apps-step4-1"), t!("settings-help-topics-apps-step4-2"), t!("settings-help-topics-apps-step4-3")])),
            Block::Tip(t!("settings-help-topics-apps-tip5")),
            Block::P(t!("settings-help-topics-apps-p6")),
            Block::Tip(t!("settings-help-topics-apps-tip7")),
            Block::P(t!("settings-help-topics-apps-p8")),
            Block::P(t!("settings-help-topics-apps-p9")),
            Block::Tip(t!("settings-help-topics-apps-tip10")),
        ]) },
        Topic { id: "sources", title: t!("settings-help-topics-sources-title"), keywords: t!("settings-help-topics-sources-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-sources-p1")),
            Block::P(t!("settings-help-topics-sources-p2")),
            Block::P(t!("settings-help-topics-sources-p3")),
            Block::Tip(t!("settings-help-topics-sources-tip4")),
        ]) },
        Topic { id: "snapshots", title: t!("settings-help-topics-snapshots-title"), keywords: t!("settings-help-topics-snapshots-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-snapshots-p1")),
            Block::Steps(leak(vec![t!("settings-help-topics-snapshots-step2-1"), t!("settings-help-topics-snapshots-step2-2"), t!("settings-help-topics-snapshots-step2-3")])),
            Block::P(t!("settings-help-topics-snapshots-p3")),
            Block::Tip(t!("settings-help-topics-snapshots-tip4")),
            Block::Tip(t!("settings-help-topics-snapshots-tip5")),
        ]) },
        Topic { id: "moving", title: t!("settings-help-topics-moving-title"), keywords: t!("settings-help-topics-moving-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-moving-p1")),
            Block::P(t!("settings-help-topics-moving-p2")),
            Block::P(t!("settings-help-topics-moving-p3")),
            Block::Tip(t!("settings-help-topics-moving-tip4")),
        ]) },
    ]
}

fn devices() -> Vec<Topic> {
    vec![
        Topic { id: "wifi", title: t!("settings-help-topics-wifi-title"), keywords: t!("settings-help-topics-wifi-keywords"), blocks: leak(vec![
            Block::Steps(leak(vec![t!("settings-help-topics-wifi-step1-1"), t!("settings-help-topics-wifi-step1-2"), t!("settings-help-topics-wifi-step1-3"), t!("settings-help-topics-wifi-step1-4")])),
            Block::Img("settings-network.png", t!("settings-help-topics-wifi-pic2")),
            Block::Tip(t!("settings-help-topics-wifi-tip3")),
        ]) },
        Topic { id: "sound", title: t!("settings-help-topics-sound-title"), keywords: t!("settings-help-topics-sound-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-sound-p1")),
            Block::Img("settings-audio.png", t!("settings-help-topics-sound-pic2")),
            Block::Steps(leak(vec![t!("settings-help-topics-sound-step3-1"), t!("settings-help-topics-sound-step3-2"), t!("settings-help-topics-sound-step3-3")])),
            Block::P(t!("settings-help-topics-sound-p4")),
            Block::Tip(t!("settings-help-topics-sound-tip5")),
        ]) },
        Topic { id: "sounds", title: t!("settings-help-topics-sounds-title"), keywords: t!("settings-help-topics-sounds-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-sounds-p1")),
            Block::Steps(leak(vec![t!("settings-help-topics-sounds-step2-1"), t!("settings-help-topics-sounds-step2-2"), t!("settings-help-topics-sounds-step2-3")])),
        ]) },
        Topic { id: "bluetooth", title: t!("settings-help-topics-bluetooth-title"), keywords: t!("settings-help-topics-bluetooth-keywords"), blocks: leak(vec![
            Block::Steps(leak(vec![t!("settings-help-topics-bluetooth-step1-1"), t!("settings-help-topics-bluetooth-step1-2"), t!("settings-help-topics-bluetooth-step1-3"), t!("settings-help-topics-bluetooth-step1-4")])),
            Block::Img("settings-bluetooth.png", t!("settings-help-topics-bluetooth-pic2")),
        ]) },
        Topic { id: "display", title: t!("settings-help-topics-display-title"), keywords: t!("settings-help-topics-display-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-display-p1")),
            Block::Img("settings-display.png", t!("settings-help-topics-display-pic2")),
            Block::P(t!("settings-help-topics-display-p3")),
            Block::P(t!("settings-help-topics-display-p4")),
        ]) },
        Topic { id: "nvidia", title: t!("settings-help-topics-nvidia-title"), keywords: t!("settings-help-topics-nvidia-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-nvidia-p1")),
            Block::P(t!("settings-help-topics-nvidia-p2")),
            Block::Tip(t!("settings-help-topics-nvidia-tip3")),
            Block::P(t!("settings-help-topics-nvidia-p4")),
            Block::Steps(leak(vec![t!("settings-help-topics-nvidia-step5-1"), t!("settings-help-topics-nvidia-step5-2"), t!("settings-help-topics-nvidia-step5-3")])),
            Block::P(t!("settings-help-topics-nvidia-p6")),
        ].into_iter().chain(key_blocks(Lead::Nvidia)).chain([
            Block::P(t!("settings-help-topics-nvidia-p10")),
        ]).collect()) },
        Topic { id: key_enrolment::HELP, title: t!("key-enrol-title"), keywords: t!("key-enrol-keywords"), blocks: leak(key_blocks(Lead::AddOnDriver)) },
    ]
}

fn seeing() -> Vec<Topic> {
    vec![
        Topic { id: "magnifier", title: t!("settings-help-topics-magnifier-title"), keywords: t!("settings-help-topics-magnifier-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-magnifier-p1")),
            Block::Steps(leak(vec![t!("settings-help-topics-magnifier-step2-1"), t!("settings-help-topics-magnifier-step2-2"), t!("settings-help-topics-magnifier-step2-3"), t!("settings-help-topics-magnifier-step2-4")])),
            Block::Tip(t!("settings-help-topics-magnifier-tip3")),
        ]) },
        Topic { id: "eye-dropper", title: t!("settings-help-topics-eye-dropper-title"), keywords: t!("settings-help-topics-eye-dropper-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-eye-dropper-p1")),
            Block::Steps(leak(vec![t!("settings-help-topics-eye-dropper-step2-1"), t!("settings-help-topics-eye-dropper-step2-2"), t!("settings-help-topics-eye-dropper-step2-3"), t!("settings-help-topics-eye-dropper-step2-4")])),
            Block::Tip(t!("settings-help-topics-eye-dropper-tip3")),
        ]) },
        Topic { id: "screenshots", title: t!("settings-help-topics-screenshots-title"), keywords: t!("settings-help-topics-screenshots-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-screenshots-p1")),
            Block::Steps(leak(vec![t!("settings-help-topics-screenshots-step2-1"), t!("settings-help-topics-screenshots-step2-2"), t!("settings-help-topics-screenshots-step2-3")])),
            Block::P(t!("settings-help-topics-screenshots-p3")),
            Block::Tip(t!("settings-help-topics-screenshots-tip4")),
        ]) },
        Topic { id: "screen-reader", title: t!("settings-help-topics-screen-reader-title"), keywords: t!("settings-help-topics-screen-reader-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-screen-reader-p1")),
            Block::P(t!("settings-help-topics-screen-reader-p2")),
        ]) },
        Topic { id: "wallpaper", title: t!("settings-help-topics-wallpaper-title"), keywords: t!("settings-help-topics-wallpaper-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-wallpaper-p1")),
            Block::Steps(leak(vec![t!("settings-help-topics-wallpaper-step2-1"), t!("settings-help-topics-wallpaper-step2-2"), t!("settings-help-topics-wallpaper-step2-3"), t!("settings-help-topics-wallpaper-step2-4")])),
            Block::Tip(t!("settings-help-topics-wallpaper-tip3")),
            Block::Img("settings-wallpaper.png", t!("settings-help-topics-wallpaper-pic4")),
        ]) },
    ]
}

fn looks() -> Vec<Topic> {
    vec![
        Topic { id: "look", title: t!("settings-help-topics-look-title"), keywords: t!("settings-help-topics-look-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-look-p1")),
            Block::P(t!("settings-help-topics-look-p2")),
            Block::Img("settings-appearance.png", t!("settings-help-topics-look-pic3")),
            Block::P(t!("settings-help-topics-look-p4")),
            Block::P(t!("settings-help-topics-look-p5")),
            Block::P(t!("settings-help-topics-look-p6")),
        ]) },
        Topic { id: "themes", title: t!("settings-help-topics-themes-title"), keywords: t!("settings-help-topics-themes-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-themes-p1")),
            Block::P(t!("settings-help-topics-themes-p2")),
            Block::Tip(t!("settings-help-topics-themes-tip3")),
        ]) },
        Topic { id: "bluetooth-connect", title: t!("settings-help-topics-bluetooth-connect-title"), keywords: t!("settings-help-topics-bluetooth-connect-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-bluetooth-connect-p1")),
            Block::P(t!("settings-help-topics-bluetooth-connect-p2")),
            Block::P(t!("settings-help-topics-bluetooth-connect-p3")),
        ]) },
        Topic { id: "bluetooth-codec", title: t!("settings-help-topics-bluetooth-codec-title"), keywords: t!("settings-help-topics-bluetooth-codec-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-bluetooth-codec-p1")),
            Block::P(t!("settings-help-topics-bluetooth-codec-p2")),
        ]) },
        Topic { id: "hardware-no-driver", title: t!("settings-help-topics-hardware-no-driver-title"), keywords: t!("settings-help-topics-hardware-no-driver-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-hardware-no-driver-p1")),
            Block::P(t!("settings-help-topics-hardware-no-driver-p2")),
            Block::P(t!("settings-help-topics-hardware-no-driver-p3")),
        ]) },
        Topic { id: "theme-files", title: t!("settings-help-topics-theme-files-title"), keywords: t!("settings-help-topics-theme-files-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-theme-files-p1")),
            Block::Steps(leak(vec![t!("settings-help-topics-theme-files-step2-1"), t!("settings-help-topics-theme-files-step2-2"), t!("settings-help-topics-theme-files-step2-3")])),
            Block::P(t!("settings-help-topics-theme-files-p3")),
            Block::P(t!("settings-help-topics-theme-files-p4")),
            Block::P(t!("settings-help-topics-theme-files-p5")),
            Block::Tip(t!("settings-help-topics-theme-files-tip6")),
        ]) },
        Topic { id: "theming", title: t!("settings-help-topics-theming-title"), keywords: t!("settings-help-topics-theming-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-theming-p1")),
            Block::Steps(leak(vec![t!("settings-help-topics-theming-step2-1"), t!("settings-help-topics-theming-step2-2"), t!("settings-help-topics-theming-step2-3")])),
            Block::P(t!("settings-help-topics-theming-p3")),
            Block::P(t!("settings-help-topics-theming-p4")),
            Block::P(t!("settings-help-topics-theming-p5")),
            Block::Tip(t!("settings-help-topics-theming-tip6")),
        ]) },
    ]
}

fn hardware_and_files() -> Vec<Topic> {
    vec![
        Topic { id: "voice", title: t!("settings-help-topics-voice-title"), keywords: t!("settings-help-topics-voice-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-voice-p1")),
            Block::P(t!("settings-help-topics-voice-p2")),
            Block::Steps(leak(vec![t!("settings-help-topics-voice-step3-1"), t!("settings-help-topics-voice-step3-2"), t!("settings-help-topics-voice-step3-3"), t!("settings-help-topics-voice-step3-4"), t!("settings-help-topics-voice-step3-5"), t!("settings-help-topics-voice-step3-6")])),
            Block::Tip(t!("settings-help-topics-voice-tip4")),
        ]) },
        Topic { id: "files", title: t!("settings-help-topics-files-title"), keywords: t!("settings-help-topics-files-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-files-p1")),
            Block::P(t!("settings-help-topics-files-p2")),
            Block::P(t!("settings-help-topics-files-p3")),
            Block::P(t!("settings-help-topics-files-p4")),
            Block::P(t!("settings-help-topics-files-p5")),
            Block::P(t!("settings-help-topics-files-p6")),
            Block::P(t!("settings-help-topics-files-p7")),
            Block::P(t!("settings-help-topics-files-p8")),
            Block::P(t!("settings-help-topics-files-p9")),
            Block::Img("settings-disks.png", t!("settings-help-topics-files-pic10")),
        ]) },
        Topic { id: "appimages", title: t!("settings-help-topics-appimages-title"), keywords: t!("settings-help-topics-appimages-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-appimages-p1")),
            Block::P(t!("settings-help-topics-appimages-p2")),
        ]) },
        Topic { id: "older-programs", title: t!("settings-help-topics-older-programs-title"), keywords: t!("settings-help-topics-older-programs-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-older-programs-p1")),
            Block::P(t!("settings-help-topics-older-programs-p2")),
            Block::Tip(t!("settings-help-topics-older-programs-tip3")),
        ]) },
        Topic { id: "printers", title: t!("settings-help-topics-printers-title"), keywords: t!("settings-help-topics-printers-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-printers-p1")),
            Block::Img("settings-printers.png", t!("settings-help-topics-printers-pic2")),
        ]) },
        Topic { id: "users", title: t!("settings-help-topics-users-title"), keywords: t!("settings-help-topics-users-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-users-p1")),
            Block::Img("settings-users.png", t!("settings-help-topics-users-pic2")),
            Block::Tip(t!("settings-help-topics-users-tip3")),
        ]) },
    ]
}

fn system() -> Vec<Topic> {
    vec![
        Topic { id: "language", title: t!("settings-help-topics-language-title"), keywords: t!("settings-help-topics-language-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-language-p1")),
            Block::P(t!("settings-help-topics-language-p2")),
            Block::Img("settings-keyboard.png", t!("settings-help-topics-language-pic3")),
            Block::Img("settings-region.png", t!("settings-help-topics-language-pic4")),
        ]) },
        Topic { id: "power", title: t!("settings-help-topics-power-title"), keywords: t!("settings-help-topics-power-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-power-p1")),
            Block::P(t!("settings-help-topics-power-p2")),
            Block::Img("settings-power.png", t!("settings-help-topics-power-pic3")),
            Block::P(t!("settings-help-topics-power-p4")),
            Block::P(t!("settings-help-topics-power-p5")),
        ]) },
        Topic { id: "startup", title: t!("settings-help-topics-startup-title"), keywords: t!("settings-help-topics-startup-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-startup-p1")),
            Block::Steps(leak(vec![t!("settings-help-topics-startup-step2-1"), t!("settings-help-topics-startup-step2-2"), t!("settings-help-topics-startup-step2-3")])),
        ]) },
        Topic { id: "keys", title: t!("settings-help-topics-keys-title"), keywords: t!("settings-help-topics-keys-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-keys-p1")),
            Block::Steps(leak(vec![t!("settings-help-topics-keys-step2-1"), t!("settings-help-topics-keys-step2-2"), t!("settings-help-topics-keys-step2-3"), t!("settings-help-topics-keys-step2-4"), t!("settings-help-topics-keys-step2-5"), t!("settings-help-topics-keys-step2-6"), t!("settings-help-topics-keys-step2-7"), t!("settings-help-topics-keys-step2-8"), t!("settings-help-topics-keys-step2-9"), t!("settings-help-topics-keys-step2-10"), t!("settings-help-topics-keys-step2-11"), t!("settings-help-topics-keys-step2-12"), t!("settings-help-topics-keys-step2-13")])),
        ]) },
        Topic { id: "disks", title: t!("settings-help-topics-disks-title"), keywords: t!("settings-help-topics-disks-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-disks-p1")),
            Block::Steps(leak(vec![t!("settings-help-topics-disks-step2-1"), t!("settings-help-topics-disks-step2-2"), t!("settings-help-topics-disks-step2-3")])),
            Block::P(t!("settings-help-topics-disks-p3")),
            Block::Steps(leak(vec![t!("settings-help-topics-disks-step4-1"), t!("settings-help-topics-disks-step4-2"), t!("settings-help-topics-disks-step4-3"), t!("settings-help-topics-disks-step4-4"), t!("settings-help-topics-disks-step4-5"), t!("settings-help-topics-disks-step4-6")])),
            Block::P(t!("settings-help-topics-disks-p5")),
            Block::Tip(t!("settings-help-topics-disks-tip6")),
        ]) },
        Topic { id: "defaults", title: t!("settings-help-topics-defaults-title"), keywords: t!("settings-help-topics-defaults-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-defaults-p1")),
            Block::Tip(t!("settings-help-topics-defaults-tip2")),
        ]) },
        Topic { id: "updates", title: t!("settings-help-topics-updates-title"), keywords: t!("settings-help-topics-updates-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-updates-p1")),
            Block::Steps(leak(vec![t!("settings-help-topics-updates-step2-1"), t!("settings-help-topics-updates-step2-2"), t!("settings-help-topics-updates-step2-3")])),
            Block::P(t!("settings-help-topics-updates-p3")),
            Block::Tip(t!("settings-help-topics-updates-tip4")),
        ]) },
    ]
}

fn about_and_trouble() -> Vec<Topic> {
    vec![
        Topic { id: "about", title: t!("settings-help-topics-about-title"), keywords: t!("settings-help-topics-about-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-about-p1")),
            Block::P(t!("settings-help-topics-about-p2")),
            Block::P(t!("settings-help-topics-about-p3")),
            Block::P(t!("settings-help-topics-about-p4")),
            Block::P(t!("settings-help-topics-about-p5")),
            Block::P(t!("settings-help-topics-about-p6")),
            Block::P(t!("settings-help-topics-about-p7")),
        ]) },
        Topic { id: "activity", title: t!("settings-help-topics-activity-title"), keywords: t!("settings-help-topics-activity-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-activity-p1")),
            Block::P(t!("settings-help-topics-activity-p2")),
            Block::P(t!("settings-help-topics-activity-p3")),
            Block::P(t!("settings-help-topics-activity-p4")),
            Block::Steps(leak(vec![t!("settings-help-topics-activity-step5-1"), t!("settings-help-topics-activity-step5-2"), t!("settings-help-topics-activity-step5-3")])),
            Block::Tip(t!("settings-help-topics-activity-tip6")),
        ]) },
        Topic { id: "virtual-machine", title: t!("settings-help-topics-virtual-machine-title"), keywords: t!("settings-help-topics-virtual-machine-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-virtual-machine-p1")),
            Block::P(t!("settings-help-topics-virtual-machine-p2")),
            Block::Tip(t!("settings-help-topics-virtual-machine-tip3")),
        ]) },
        Topic { id: "games", title: t!("settings-help-topics-games-title"), keywords: t!("settings-help-topics-games-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-games-p1")),
            Block::P(t!("settings-help-topics-games-p2")),
            Block::P(t!("settings-help-topics-games-p3")),
            Block::Steps(leak(vec![t!("settings-help-topics-games-step4-1"), t!("settings-help-topics-games-step4-2"), t!("settings-help-topics-games-step4-3"), t!("settings-help-topics-games-step4-4")])),
            Block::P(t!("settings-help-topics-games-p5")),
            Block::P(t!("settings-help-topics-games-p6")),
            Block::Tip(t!("settings-help-topics-games-tip7")),
        ]) },
        Topic { id: "trouble", title: t!("settings-help-topics-trouble-title"), keywords: t!("settings-help-topics-trouble-keywords"), blocks: leak(vec![
            Block::P(t!("settings-help-topics-trouble-p1")),
            Block::P(t!("settings-help-topics-trouble-p2")),
            Block::Tip(t!("settings-help-topics-trouble-tip3")),
        ]) },
    ]
}

pub fn topics() -> &'static [Topic] {
    static ALL: OnceLock<Vec<Topic>> = OnceLock::new();
    ALL.get_or_init(|| {
        let groups = [getting_started, programs, devices, seeing, looks, hardware_and_files, system, about_and_trouble];
        groups.iter().flat_map(|group| group()).collect()
    })
}
