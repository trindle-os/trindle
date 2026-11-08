//! Which graphics cards are in this computer and which driver each is using.

use std::path::Path;

use crate::toolkit::t;

pub struct Card {

    pub driver: String,

    pub vendor: String,
}

impl Card {
    pub fn is_nvidia(&self) -> bool {
        self.vendor == "0x10de" || self.driver == "nouveau" || self.driver == "nvidia"
    }

    pub fn on_open_driver(&self) -> bool {
        self.is_nvidia() && self.driver != "nvidia"
    }
}

pub fn cards() -> Vec<Card> {
    let mut found = Vec::new();
    let Ok(entries) = std::fs::read_dir("/sys/class/drm") else { return found };
    for e in entries.flatten() {
        let name = e.file_name().to_string_lossy().to_string();

        if !name.starts_with("card") || name.contains('-') {
            continue;
        }
        let device = Path::new("/sys/class/drm").join(&name).join("device");
        let uevent = std::fs::read_to_string(device.join("uevent")).unwrap_or_default();
        let driver = uevent.lines().find_map(|l| l.strip_prefix("DRIVER=")).unwrap_or("").to_string();
        let vendor = std::fs::read_to_string(device.join("vendor")).unwrap_or_default().trim().to_string();
        if driver.is_empty() && vendor.is_empty() {
            continue;
        }
        found.push(Card { driver, vendor });
    }
    found
}

pub fn has_graphics_chip() -> bool {
    std::fs::read_dir("/dev/dri")
        .map(|d| d.flatten().any(|e| e.file_name().to_string_lossy().starts_with("renderD")))
        .unwrap_or(false)
}

pub fn drawn_by_processor() -> bool {
    std::env::var_os("TRINDLE_DRAWN_BY_PROCESSOR").is_some() || !has_graphics_chip()
}

pub fn virtual_machine() -> Option<String> {
    let out = std::process::Command::new("systemd-detect-virt").arg("--vm").output().ok()?;
    let id = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if !out.status.success() || id.is_empty() || id == "none" {
        return None;
    }
    Some(match id.as_str() {
        "qemu" | "kvm" => "a QEMU virtual machine (GNOME Boxes, virt-manager and the like)".to_string(),
        "oracle" => "VirtualBox".to_string(),
        "vmware" => "VMware".to_string(),
        "microsoft" => "Hyper-V".to_string(),
        "parallels" => "Parallels".to_string(),
        "apple" => "a Mac's virtual machine".to_string(),
        "xen" => "Xen".to_string(),
        other => format!("a virtual machine ({other})"),
    })
}

pub fn vm_note(machine: &str) -> (&'static str, String) {
    let body = if !drawn_by_processor() {
        t!("graphics-vm-body-fast", machine = machine)
    } else {
        t!("graphics-vm-body-slow", machine = machine)
    };
    (t!("graphics-vm-title"), body)
}

pub const DRAWING: &str = "TRINDLE_DRAWING";

pub fn light_drawing(choice: Option<&str>) -> bool {
    match choice {
        Some("light") => true,
        Some("normal") => false,
        _ => !has_graphics_chip(),
    }
}

pub fn moving() -> bool {
    !light_drawing(std::env::var(DRAWING).ok().as_deref())
}

pub fn nvidia_on_open_driver() -> bool {
    cards().iter().any(Card::on_open_driver)
}

#[cfg(test)]
mod tests {
    use super::Card;

    fn card(driver: &str, vendor: &str) -> Card {
        Card { driver: driver.into(), vendor: vendor.into() }
    }

    #[test]
    fn intel_and_amd_are_left_alone() {
        assert!(!card("i915", "0x8086").is_nvidia());
        assert!(!card("amdgpu", "0x1002").is_nvidia());
        assert!(!card("i915", "0x8086").on_open_driver());
    }

    #[test]
    fn nvidia_on_nouveau_is_worth_mentioning() {
        assert!(card("nouveau", "0x10de").on_open_driver());
    }

    #[test]
    fn a_choice_made_on_the_login_screen_wins() {
        assert!(super::light_drawing(Some("light")));
        assert!(!super::light_drawing(Some("normal")));
    }

    #[test]
    fn nvidia_with_no_driver_is_still_nvidia() {
        assert!(card("", "0x10de").on_open_driver());
    }

    #[test]
    fn nvidias_own_driver_is_not_mentioned() {
        let c = card("nvidia", "0x10de");
        assert!(c.is_nvidia());
        assert!(!c.on_open_driver());
    }
}
