//! Approving a driver's key. A driver built on this computer (NVIDIA's own,
//! an add-on driver, VirtualBox's, the virtual camera's) is not signed by
//! anyone Secure Boot trusts, so the next start shows a blue screen (shim's
//! MOK manager) that asks for the password chosen when the key was queued
//! with mokutil. It has an American keyboard and gives up after about ten
//! seconds, so people are told the steps first. Every path that leads to
//! that screen calls `before_restart` and shows what it gives; nothing else
//! in the program names mokutil (a test reads the source to be sure).
//! The words are in share/lang/en/key-enrolment.ftl.

use crate::toolkit::t;

pub const HELP: &str = "secure-boot-key";

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Lead {
    Nvidia,
    AddOnDriver,
}

pub struct Notice {
    pub why: &'static str,
    pub steps: Vec<&'static str>,
    pub missed: &'static str,
}

pub fn steps() -> Vec<&'static str> {
    vec![t!("key-enrol-step1"), t!("key-enrol-step2"), t!("key-enrol-step3"), t!("key-enrol-step4"), t!("key-enrol-step5")]
}

pub fn before_restart(lead: Lead) -> Notice {
    let why = match lead {
        Lead::Nvidia => t!("key-enrol-why-nvidia"),
        Lead::AddOnDriver => t!("key-enrol-why-addon"),
    };
    Notice { why, steps: steps(), missed: t!("key-enrol-missed") }
}

#[cfg(test)]
pub const LEADS: [Lead; 2] = [Lead::Nvidia, Lead::AddOnDriver];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_path_gives_the_same_steps_first_in_the_screens_order() {
        for lead in LEADS {
            let n = before_restart(lead);
            assert_eq!(n.steps.len(), 5, "{lead:?}");
            assert!(n.why.contains("before you restart"), "{lead:?}");
            let all = n.steps.join(" ");
            assert!(all.contains("letters and digits only"));
            assert!(all.contains("American keyboard"));
            assert!(all.contains("about 10 seconds"));
            let at = |w: &str| all.find(w).unwrap_or_else(|| panic!("{w}"));
            assert!(at("Enroll MOK") < at("Continue") && at("Continue") < at("Yes") && at("Yes") < at("password you made") && at("password you made") < at("Reboot"));
            assert!(!n.steps[0].to_lowercase().contains("restart"), "the password comes before the restart");
            assert!(n.steps[1].starts_with("Restart"));
        }
        let why: Vec<_> = LEADS.iter().map(|l| before_restart(*l).why).collect();
        assert_ne!(why[0], why[1]);
    }

    #[test]
    fn nothing_but_this_module_and_its_words_names_mokutil() {
        fn scan(dir: &std::path::Path, hits: &mut Vec<String>) {
            for e in std::fs::read_dir(dir).unwrap().flatten() {
                let p = e.path();
                if p.is_dir() {
                    scan(&p, hits);
                } else if matches!(p.extension().and_then(|x| x.to_str()), Some("rs" | "ftl" | "sh")) {
                    let name = p.file_name().unwrap().to_string_lossy().to_string();
                    if name != "key_enrolment.rs" && name != "key-enrolment.ftl" && std::fs::read_to_string(&p).unwrap_or_default().contains("mokutil") {
                        hits.push(p.display().to_string());
                    }
                }
            }
        }
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let mut hits = Vec::new();
        for d in ["src", "share/lang/en", "tools", "system"] {
            scan(&root.join(d), &mut hits);
        }
        assert!(hits.is_empty(), "these lead to a key without the steps first: {hits:?}");
    }
}
