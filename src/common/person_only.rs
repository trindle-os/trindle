//! What only the person may press.

pub const PASSWORD: &str = "trindle-polkit";
pub const DEVICE: &str = "trindle-thunder";
pub const PERMISSIONS: &str = "trindle-permissions";
pub const RING: &str = "trindle-ring";
pub const PORTAL: &str = "org.trindle.Portal";
pub const CHOOSER: &str = "org.trindle.FileChooser";
pub const ASSISTANT: &str = "trindle-mcp";
pub const KEYRING: &str = "trindle-keyring";
pub const PIN: &str = "trindle-pin";

pub const ASKED_BY_TRINDLE: [&str; 2] = [PASSWORD, DEVICE];
pub const ASKED_BY_NAME: [&str; 1] = [PERMISSIONS];
pub const PANELS_BY_NAME: [&str; 1] = [RING];
pub const WINDOWS: [&str; 5] = [PORTAL, CHOOSER, ASSISTANT, KEYRING, PIN];

pub fn asks(name: &str) -> bool {
    ASKED_BY_TRINDLE.contains(&name) || ASKED_BY_NAME.contains(&name) || WINDOWS.contains(&name)
}

pub const STOP: &str = "activity-stop";
pub const FORCE_CLOSE: &str = "windows-force-close";
pub const RING_POWER: &str = "ring-power";
pub const SETTINGS_POWER: &str = "settings-power";
pub const SNAPSHOT_RESTART: &str = "settings-snapshots-restart";
pub const APPS_RESTART: &str = "apps-restart";
pub const SETTINGS_PERMISSIONS: &str = "settings-permissions";
pub const AI_PROGRAMS: &str = "settings-ai-programs";
pub const APPS_INSTALL: &str = "apps-install";
pub const APPS_BACKUP_BACK: &str = "apps-backup-back";
pub const APPS_DATA_DELETE: &str = "apps-data-delete";
pub const PLUGIN_OFF: &str = "apps-plugin-off";
pub const PLUGIN_REMOVE: &str = "apps-plugin-remove";
pub const SCREEN_READER: &str = "settings-screen-reader";
pub const KEYBOARD: &str = "settings-keyboard";
pub const CLICKER: &str = "settings-clicking-helper";
pub const INPUT_MADE: &str = "settings-input-made";
pub const RESTART_PARTS: &str = "settings-restart-parts";
pub const PIN_CARD: &str = "pin-card";

pub const ACTS: [(&str, &str, &str); 19] = [
    (STOP, "activity/stop.rs", "stop_step(self.armed, crate::toolkit::window::persons_own(crate::common::person_only::STOP))"),
    (FORCE_CLOSE, "ring/view/window_ring.rs", "let own = kept::own_press();"),
    (RING_POWER, "ring/view/power.rs", "power_step(what, self.armed.as_deref(), kept::own_press())"),
    (SETTINGS_POWER, "settings/panels/power.rs", "if !crate::toolkit::window::persons_own(crate::common::person_only::SETTINGS_POWER) {"),
    (SNAPSHOT_RESTART, "settings/panels/snapshots.rs", "if !crate::toolkit::window::persons_own(crate::common::person_only::SNAPSHOT_RESTART) {"),
    (APPS_RESTART, "store/update_page/install.rs", "if !crate::toolkit::window::persons_own(crate::common::person_only::APPS_RESTART) {"),
    (SETTINGS_PERMISSIONS, "settings/panels/permissions.rs", "if !crate::toolkit::window::persons_own(crate::common::person_only::SETTINGS_PERMISSIONS) {"),
    (AI_PROGRAMS, "settings/panels/ai_programs.rs", "crate::toolkit::window::persons_own(crate::common::person_only::AI_PROGRAMS)"),
    (APPS_INSTALL, "store/pages/install.rs", "if !crate::toolkit::window::persons_own(crate::common::person_only::APPS_INSTALL) {"),
    (APPS_BACKUP_BACK, "store/pages/backup_page.rs", "if !crate::toolkit::window::persons_own(crate::common::person_only::APPS_BACKUP_BACK) {"),
    (APPS_DATA_DELETE, "store/pages/remove_ask.rs", "if wipe && !crate::toolkit::window::persons_own(crate::common::person_only::APPS_DATA_DELETE) {"),
    (PLUGIN_OFF, "settings/panels/plugin_rows.rs", "if matches!(what, Act::Switch(false)) && !crate::toolkit::window::persons_own(crate::common::person_only::PLUGIN_OFF) {"),
    (PLUGIN_REMOVE, "store/plugin_page.rs", "crate::toolkit::window::persons_own(crate::common::person_only::PLUGIN_REMOVE)"),
    (SCREEN_READER, "settings/panels/appearance/reading.rs", "if !crate::toolkit::window::persons_own(crate::common::person_only::SCREEN_READER) {"),
    (KEYBOARD, "settings/panels/keyboard/devices.rs", "if !crate::toolkit::window::persons_own(crate::common::person_only::KEYBOARD) {"),
    (CLICKER, "settings/panels/keyboard/devices.rs", "if !crate::toolkit::window::persons_own(crate::common::person_only::CLICKER) {"),
    (INPUT_MADE, "settings/panels/keyboard/changed_keys.rs", "if !crate::toolkit::window::persons_own(crate::common::person_only::INPUT_MADE) {"),
    (RESTART_PARTS, "settings/panels/keyboard/restart_keys.rs", "if !crate::toolkit::window::persons_own(crate::common::person_only::RESTART_PARTS) {"),
    (PIN_CARD, "plugins/pin.rs", "Msg::Digit(_) | Msg::Other | Msg::Back | Msg::Send if !persons_own(PIN_CARD) => {}"),
];

pub const INSTALL_PRESSED: [(&str, &str); 7] = [
    ("store/app_page.rs", "go(move |s| pages::install::pressed_install(s, |s| pages::install::install(s, &i)))"),
    ("store/listing.rs", "go(move |s| pages::install::pressed_install(s, |s| pages::install::install(s, &app)))"),
    ("store/plugin_page.rs", "move |s| super::pages::install::pressed_install(s, |s| install(s, &o))"),
    ("store/pages/github_page.rs", "move |s| pressed_install(s, |s| install_repo(s, &repo))"),
    ("store/pages/image_tile.rs", "move |s| pressed_install(s, |s| install_repo(s, &repo))"),
    ("store/pages/github_page.rs", "move |s| pressed_install(s, |s| install_file(s, &k, &d, CameFrom::GitHub(repo.clone())))"),
    ("store/deb.rs", "on_press(go(pressed_install_boxed))"),
];

pub const WAITING: [(&str, &str, &str); 0] = [];

pub const GIVEN_IN: [(&str, &str, &str); 9] = [
    (PASSWORD, "polkit/mod.rs", "name: \"trindle-polkit\".into()"),
    (DEVICE, "thunder/ask.rs", "name: \"trindle-thunder\".into()"),
    (PERMISSIONS, "store/asking.rs", "name: \"trindle-permissions\".into()"),
    (RING, "ring/view/panel.rs", "name: \"trindle-ring\".into()"),
    (PORTAL, "screenshot/portal_ask.rs", "const APP_ID: &str = \"org.trindle.Portal\";"),
    (CHOOSER, "screenshot/portal_files.rs", "app_id: \"org.trindle.FileChooser\".into()"),
    (ASSISTANT, "mcp/ask.rs", "app_id: \"trindle-mcp\".into()"),
    (KEYRING, "keyring/prompt.rs", "app_id: \"trindle-keyring\".into()"),
    (PIN, "plugins/pin.rs", "app_id: \"trindle-pin\".into()"),
];
