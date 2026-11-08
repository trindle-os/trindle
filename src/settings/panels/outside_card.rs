//! The Permissions page's two cards for programs outside a box: the
//! notices about the network and the person's files, the programs said to
//! be fine (each can be forgotten), and downloaded files opened in a box,
//! with the file types the person opens as normal.

use std::path::PathBuf;

use super::users::banded_rows;
use crate::common::outside::{self, Fine, Kind};
use crate::settings::ui::{group, line, on, Kit, Msg};
use crate::store::downloaded;
use crate::toolkit::t;
use crate::toolkit::widget::{button, switch, Node};

use super::permissions::Permissions;

#[derive(Default)]
pub struct Outside {
    network: bool,
    files: bool,
    boxed: bool,
    fine: Vec<Fine>,
    seen: Vec<String>,
    as_is: Vec<String>,
    #[cfg(test)]
    pub top: Option<PathBuf>,
}

const NETWORK_KEY: &str = "outside:network";
const FILES_KEY: &str = "outside:files";
const BOXED_KEY: &str = "outside:downloads";

impl Outside {
    #[cfg(test)]
    fn at(&self, name: &str, real: PathBuf) -> PathBuf {
        self.top.as_ref().map_or(real, |top| top.join(name))
    }

    #[cfg(not(test))]
    fn at(&self, _name: &str, real: PathBuf) -> PathBuf {
        real
    }

    fn fine_path(&self) -> PathBuf {
        self.at("outside-fine", outside::fine_path())
    }

    fn seen_path(&self) -> PathBuf {
        self.at("downloads-seen", downloaded::seen_path())
    }

    fn as_is_path(&self) -> PathBuf {
        self.at("downloads-as-is", downloaded::as_is_path())
    }

    pub fn read(&mut self) {
        self.network = outside::network_on();
        self.files = outside::files_on();
        self.boxed = downloaded::on();
        self.fine = outside::fine_in(&self.fine_path());
        self.seen = downloaded::seen_in(&self.seen_path());
        self.as_is = downloaded::as_is_in(&self.as_is_path());
    }

    fn switched(&mut self, kit: &mut Kit, key: &'static str, setting: &str, on: bool) {
        if !crate::toolkit::window::persons_own(crate::common::person_only::SETTINGS_PERMISSIONS) {
            return;
        }
        let err = crate::common::files::set_setting(setting, if on { "yes" } else { "no" }).err();
        match key {
            NETWORK_KEY => self.network = on != err.is_some(),
            FILES_KEY => self.files = on != err.is_some(),
            _ => self.boxed = on != err.is_some(),
        }
        kit.done(key, err.map(|_| t!("settings-permissions-not-changed")), t!("settings-permissions-saved"));
    }

    fn forget(&mut self, kit: &mut Kit, kind: Kind, key: &str) {
        if !crate::toolkit::window::persons_own(crate::common::person_only::SETTINGS_PERMISSIONS) {
            return;
        }
        let err = outside::forget_in(&self.fine_path(), kind, key).err();
        self.fine = outside::fine_in(&self.fine_path());
        kit.done(&format!("outside:fine:{}:{key}", kind.word()), err.map(|_| t!("settings-permissions-not-changed")), t!("settings-permissions-saved"));
    }

    fn open_as_normal(&mut self, kit: &mut Kit, kind: &str, as_is: bool) {
        if !crate::toolkit::window::persons_own(crate::common::person_only::SETTINGS_PERMISSIONS) {
            return;
        }
        let err = downloaded::set_as_is_in(&self.as_is_path(), kind, as_is).err();
        self.as_is = downloaded::as_is_in(&self.as_is_path());
        kit.done(&format!("outside:type:{kind}"), err.map(|_| t!("settings-permissions-not-changed")), t!("settings-permissions-saved"));
    }

    pub fn cards(&self, kit: &Kit) -> Vec<Node<Msg>> {
        let net = switch(self.network, t!("settings-outside-network"), |b| {
            on::<Permissions>(move |me, k| me.outside.switched(k, NETWORK_KEY, outside::NETWORK, b))
        });
        let files = switch(self.files, t!("settings-outside-files"), |b| {
            on::<Permissions>(move |me, k| me.outside.switched(k, FILES_KEY, outside::FILES, b))
        });
        let mut rows = vec![
            line(t!("settings-outside-network")).sub(t!("settings-outside-once")).key(NETWORK_KEY).control(net).node(kit),
            line(t!("settings-outside-files")).sub(t!("settings-outside-once")).key(FILES_KEY).control(files).node(kit),
        ];
        let fine: Vec<Node<Msg>> = self
            .fine
            .iter()
            .map(|f| {
                let (kind, key) = (f.kind, f.key.clone());
                let forget = button("list-remove-symbolic", t!("settings-outside-forget", name = &f.name))
                    .on_press(on::<Permissions>(move |me, k| me.outside.forget(k, kind, &key)));
                let what = match f.kind {
                    Kind::Network => t!("settings-outside-fine-network"),
                    Kind::Files => t!("settings-outside-fine-files"),
                };
                line(f.name.clone()).sub(what).key(format!("outside:fine:{}:{}", f.kind.word(), f.key)).control(forget).node(kit)
            })
            .collect();
        if fine.is_empty() {
            rows.push(line(t!("settings-outside-no-fine")).node(kit));
        } else {
            rows.extend(banded_rows(fine));
        }
        let outside_card = group(t!("settings-outside-heading")).icon("security-high-symbolic").about(t!("settings-outside-about")).rows(rows).node(kit);

        let boxed = switch(self.boxed, t!("settings-downloads-box"), |b| {
            on::<Permissions>(move |me, k| me.outside.switched(k, BOXED_KEY, downloaded::SETTING, b))
        });
        let mut rows = vec![line(t!("settings-downloads-box")).sub(t!("settings-downloads-box-sub")).key(BOXED_KEY).control(boxed).node(kit)];
        let types: Vec<Node<Msg>> = self
            .seen
            .iter()
            .map(|kind| {
                let normal = self.as_is.contains(kind);
                let name = kind.clone();
                let sw = switch(normal, t!("settings-downloads-normal", kind = kind.as_str()), move |b| {
                    let name = name.clone();
                    on::<Permissions>(move |me, k| me.outside.open_as_normal(k, &name, b))
                });
                line(t!("settings-downloads-type", kind = kind.as_str())).sub(t!("settings-downloads-normal-sub")).key(format!("outside:type:{kind}")).control(sw).node(kit)
            })
            .collect();
        if types.is_empty() {
            rows.push(line(t!("settings-downloads-none")).node(kit));
        } else {
            rows.extend(banded_rows(types));
        }
        let downloads = group(t!("settings-downloads-heading")).icon("folder-download-symbolic").about(t!("settings-downloads-about")).rows(rows).node(kit);
        vec![outside_card, downloads]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use crate::settings::ui::Do;
    use crate::toolkit::event::Loop;
    use crate::toolkit::window::{with_named, with_press};

    fn kit() -> Kit {
        let events = Loop::new();
        let asks = events.post(|_: Do| {});
        Kit::new(events.clone(), asks, None, None)
    }

    fn room(tag: &str) -> PathBuf {
        let top = std::env::temp_dir().join(format!("trindle-outside-card-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&top);
        std::fs::create_dir_all(&top).unwrap();
        top
    }

    fn with(top: &Path) -> Outside {
        let mut o = Outside { top: Some(top.to_path_buf()), ..Outside::default() };
        o.fine = outside::fine_in(&o.fine_path());
        o.seen = downloaded::seen_in(&o.seen_path());
        o.as_is = downloaded::as_is_in(&o.as_is_path());
        o
    }

    #[test]
    fn the_cards_list_what_is_fine_and_what_has_been_boxed() {
        let top = room("list");
        outside::make_fine_in(&top.join("outside-fine"), Kind::Network, &[("/usr/bin/vlc".into(), "vlc".into())]).unwrap();
        downloaded::remember_in(&top.join("downloads-seen"), &["pdf".to_string(), "png".to_string()]).unwrap();
        let o = with(&top);
        let said: Vec<String> = o.cards(&kit()).into_iter().flat_map(|c| c.said()).collect();
        assert!(said.contains(&"button: Say it again for vlc".to_string()), "{said:?}");
        for want in ["switch: Say when a program reaches the network", "switch: Say when a program has your files open",
                     "switch: Open downloaded files in a box", "switch: Open pdf files as normal", "switch: Open png files as normal"] {
            assert!(said.contains(&want.to_string()), "{want}: {said:?}");
        }
        std::fs::remove_dir_all(&top).unwrap();
    }

    #[test]
    fn forgetting_and_opening_as_normal_are_kept_for_the_persons_own_press_only() {
        let top = room("press");
        outside::make_fine_in(&top.join("outside-fine"), Kind::Files, &[("/usr/bin/vlc".into(), "vlc".into())]).unwrap();
        let mut o = with(&top);
        let mut kit = kit();
        with_named(true, Some(7), || with_press(Some(8), || {
            o.forget(&mut kit, Kind::Files, "/usr/bin/vlc");
            o.open_as_normal(&mut kit, "pdf", true);
        }));
        assert_eq!((o.fine.len(), o.as_is.len()), (1, 0), "made-up presses changed something");
        o.forget(&mut kit, Kind::Files, "/usr/bin/vlc");
        o.open_as_normal(&mut kit, "pdf", true);
        assert_eq!((o.fine.len(), o.as_is.clone()), (0, vec!["pdf".to_string()]));
        assert!(outside::fine_in(&top.join("outside-fine")).is_empty());
        o.open_as_normal(&mut kit, "pdf", false);
        assert!(o.as_is.is_empty());
        std::fs::remove_dir_all(&top).unwrap();
    }
}
