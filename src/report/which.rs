//! The "Which app?" choice in the report window: what a report can be about, and the project each goes to.

use std::sync::OnceLock;

use super::send;
use crate::common::apps::same_name;
use crate::toolkit::lang::{self, Bundle, Catalogue};

pub(super) const NOT_SURE: &str = "";

pub(super) const OTHER: &str = "other";

const OWN: [(&str, &str); 9] = [
    ("trindle-ring", "settings-common-ring-name"),
    ("trindle-settings", "settings-window-title"),
    ("trindle-apps", "store-title"),
    (send::FOLDERS_PROGRAM, "settings-default-apps-kind-folders"),
    ("trindle-screenshot", "screenshot-name"),
    ("trindle-text", "text-title"),
    ("trindle-activity", "activity-title"),
    ("trindle-drives", "disks-title"),
    ("trindle-greeter", "greeter-window-title"),
];

pub(super) struct Pick {

    pub program: String,

    pub name: String,

    key: Option<&'static str>,
}

impl Pick {
    fn keyed(program: &str, key: &'static str) -> Pick {
        Pick::named(program, lang::t(key), key)
    }

    fn named(program: &str, name: &str, key: &'static str) -> Pick {
        Pick { program: program.into(), name: name.into(), key: Some(key) }
    }

    pub fn english(&self) -> String {
        static ENGLISH: OnceLock<Catalogue> = OnceLock::new();
        match self.key {
            Some(key) => ENGLISH.get_or_init(|| Catalogue::new(Bundle::default(), lang::english())).plain(key).into(),
            None => self.name.clone(),
        }
    }
}

pub(super) fn picks(plugins: &[(String, String)], crashed: &str) -> Vec<Pick> {
    let mut all = vec![Pick::named(NOT_SURE, lang::t("report-app-unsure"), "report-app-unsure")];
    all.extend(OWN.iter().map(|(program, key)| Pick::keyed(program, key)));
    all.push(Pick::named("trindle-session", lang::t("report-app-desktop"), "report-app-desktop"));
    for (id, name) in plugins {
        let program = format!("trindle-plugin {id}");
        if send::project_of(&program).is_some() {
            all.push(Pick { program, name: name.clone(), key: None });
        }
    }
    if !crashed.is_empty() && crashed != send::OWN && !all.iter().any(|p| same_name(&p.program, crashed)) {
        all.push(Pick { program: crashed.into(), name: crashed.into(), key: None });
    }
    all.push(Pick::named(OTHER, lang::t("report-app-other"), "report-app-other"));
    all
}

pub(super) fn place(all: &[Pick], program: &str) -> usize {
    all.iter().position(|p| same_name(&p.program, program)).unwrap_or(0)
}

pub(super) fn line(pick: &Pick) -> Option<String> {
    (pick.program != NOT_SURE).then(|| format!("App: {}.\n", pick.english()))
}
