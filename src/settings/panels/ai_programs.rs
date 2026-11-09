//! AI programs: the fence round Claude Code, Codex and the like, which keeps
//! them away from the person's personal files (store/fence.rs). The page
//! holds the one switch, the list of programs, the folders shared with them,
//! the computers they may reach by ssh, the keys typed for them and the
//! optional limit on the network. Every change here is the person's own
//! press (`person_only::AI_PROGRAMS`).

use std::path::{Path, PathBuf};

use crate::common::files::home;
use crate::settings::ui::{no_chooser, group, headline, line, on, page, Kit, Msg, Page};
use crate::store::{fence, fence_broker};
use crate::toolkit::bus::Listen;
use crate::toolkit::t;
use crate::toolkit::widget::{button, entry, note, remove, switch, Node, Secret};

const FENCE: &str = "ai:fence";
const PROGRAM: &str = "ai:program";
const SHARE: &str = "ai:share";
const HOST: &str = "ai:host";
const KEYS: &str = "ai:keys";
const NET: &str = "ai:net";
const NET_ADD: &str = "ai:net-add";

pub struct AiPrograms {
    root: PathBuf,
    home: PathBuf,
    enabled: bool,
    programs: Vec<(String, bool)>,
    own: Vec<String>,
    shared: Vec<fence::Share>,
    hosts: Vec<String>,
    names: Vec<String>,
    public: Option<String>,
    typed_program: String,
    typed_host: String,
    typed_key: String,
    typed_net: String,
    net_on: bool,
    net_hosts: Vec<String>,
    asking: Option<Listen>,
}

impl Default for AiPrograms {
    fn default() -> Self {
        AiPrograms {
            root: fence::root(), home: home(), enabled: true, programs: Vec::new(), own: Vec::new(), shared: Vec::new(), hosts: Vec::new(),
            names: Vec::new(), public: None, typed_program: String::new(), typed_host: String::new(), typed_key: String::new(), typed_net: String::new(), net_on: false, net_hosts: Vec::new(), asking: None,
        }
    }
}

impl AiPrograms {
    fn look(&mut self) {
        self.enabled = fence::enabled(&self.root);
        self.programs = fence::programs(&self.root);
        self.own = fence::uses_own_settings(&self.root);
        self.shared = fence::shared(&self.root, &self.home);
        self.hosts = fence::ssh_hosts(&self.root);
        self.names = fence::typed_env(&self.root).into_iter().map(|(k, _)| k).collect();
        self.public = fence_broker::public_key(&self.root);
        self.net_on = fence::net_limit(&self.root).is_some();
        self.net_hosts = fence::net_hosts(&self.root);
    }

    fn own_press() -> bool {
        crate::toolkit::window::persons_own(crate::common::person_only::AI_PROGRAMS)
    }

    fn finish(&mut self, kit: &mut Kit, key: &str, result: Result<(), &str>) {
        self.look();
        kit.done(key, result.err(), t!("settings-ai-saved"));
    }

    fn fence(&mut self, kit: &mut Kit, on: bool) {
        if !Self::own_press() {
            return;
        }
        let r = fence::set_enabled(&self.root, on).map_err(|_| t!("settings-ai-not-changed"));
        self.finish(kit, FENCE, r);
    }

    fn set_program(&mut self, kit: &mut Kit, name: &str, on: bool) {
        if !Self::own_press() {
            return;
        }
        let mut list = self.programs.clone();
        if let Some(p) = list.iter_mut().find(|p| p.0 == name) {
            p.1 = on;
        }
        let r = fence::set_programs(&self.root, &list);
        self.finish(kit, PROGRAM, r);
    }

    fn drop_program(&mut self, kit: &mut Kit, name: &str) {
        if !Self::own_press() {
            return;
        }
        let list: Vec<(String, bool)> = self.programs.iter().filter(|p| p.0 != name).cloned().collect();
        let r = fence::set_programs(&self.root, &list);
        self.finish(kit, PROGRAM, r);
    }

    fn add_program(&mut self, kit: &mut Kit) {
        if !Self::own_press() {
            return;
        }
        let mut list = self.programs.clone();
        list.push((self.typed_program.trim().to_string(), true));
        let r = fence::set_programs(&self.root, &list);
        if r.is_ok() {
            self.typed_program.clear();
        }
        self.finish(kit, PROGRAM, r);
    }

    fn own_settings(&mut self, kit: &mut Kit, name: &str, on: bool) {
        if !Self::own_press() {
            return;
        }
        let r = fence::set_uses_own_settings(&self.root, name, on);
        self.finish(kit, PROGRAM, r);
    }

    fn choose_folder(&mut self, kit: &mut Kit) {
        if !Self::own_press() {
            return;
        }
        let Some(bus) = kit.session_for(SHARE) else { return };
        let back = kit.then::<AiPrograms, Option<PathBuf>>(|me, k, path| me.chosen(k, path));
        self.asking = super::wallpaper::pick_folder(&bus, &kit.events, t!("settings-ai-choose-folder"), &self.home, back);
        kit.started(SHARE, self.asking.is_some(), no_chooser());
    }

    fn chosen(&mut self, kit: &mut Kit, path: Option<PathBuf>) {
        self.asking = None;
        let Some(path) = path else {
            kit.unanswered(SHARE);
            return;
        };
        let r = fence::share(&self.root, &self.home, &path, false, true).map(|_| ());
        self.finish(kit, SHARE, r);
    }

    fn read_only(&mut self, kit: &mut Kit, path: &Path, read_only: bool) {
        if !Self::own_press() {
            return;
        }
        let r = fence::share(&self.root, &self.home, path, read_only, true).map(|_| ());
        self.finish(kit, SHARE, r);
    }

    fn unshare(&mut self, kit: &mut Kit, path: &Path) {
        if !Self::own_press() {
            return;
        }
        let r = fence::unshare(&self.root, path).map_err(|_| t!("settings-ai-not-changed"));
        self.finish(kit, SHARE, r);
    }

    fn hosts_set(&mut self, kit: &mut Kit, list: Vec<String>) {
        if !Self::own_press() {
            return;
        }
        let r = fence::set_ssh_hosts(&self.root, &list);
        if r.is_ok() {
            self.typed_host.clear();
        }
        self.finish(kit, HOST, r);
    }

    fn add_host(&mut self, kit: &mut Kit) {
        let mut list = self.hosts.clone();
        list.push(self.typed_host.trim().to_string());
        self.hosts_set(kit, list);
    }

    fn make_key(&mut self, kit: &mut Kit) {
        if !Self::own_press() {
            return;
        }
        let r = fence_broker::make_key(&self.root.join("ssh")).map_err(|_| t!("settings-ai-key-not-made"));
        self.finish(kit, HOST, r);
    }

    fn add_key(&mut self, kit: &mut Kit) {
        if !Self::own_press() {
            return;
        }
        let mut text: Vec<String> = fence::typed_env(&self.root).into_iter().map(|(k, v)| format!("{k}={v}")).collect();
        text.push(self.typed_key.trim().to_string());
        let r = fence::set_typed_env(&self.root, &text.join("\n"));
        if r.is_ok() {
            self.typed_key.clear();
        }
        self.finish(kit, KEYS, r);
    }

    fn drop_key(&mut self, kit: &mut Kit, name: &str) {
        if !Self::own_press() {
            return;
        }
        let text: Vec<String> = fence::typed_env(&self.root).into_iter().filter(|(k, _)| k != name).map(|(k, v)| format!("{k}={v}")).collect();
        let r = fence::set_typed_env(&self.root, &text.join("\n"));
        self.finish(kit, KEYS, r);
    }

    fn net_limit(&mut self, kit: &mut Kit, on: bool) {
        if !Self::own_press() {
            return;
        }
        let r = fence::set_net_limit(&self.root, on).map_err(|_| t!("settings-ai-not-changed"));
        self.finish(kit, NET, r);
    }

    fn net_hosts_set(&mut self, kit: &mut Kit, list: Vec<String>) {
        if !Self::own_press() {
            return;
        }
        let r = fence::set_net_hosts(&self.root, &list);
        if r.is_ok() {
            self.typed_net.clear();
        }
        self.finish(kit, NET_ADD, r);
    }

    fn add_net(&mut self, kit: &mut Kit) {
        let mut list = self.net_hosts.clone();
        list.push(self.typed_net.trim().to_string());
        self.net_hosts_set(kit, list);
    }

    fn net_card(&self, kit: &Kit) -> Node<Msg> {
        let toggle = switch(self.net_on, t!("settings-ai-limit-net"), |v| on::<AiPrograms>(move |me, k| me.net_limit(k, v)));
        let mut rows = vec![line(t!("settings-ai-limit-net"))
            .sub(t!("settings-ai-limit-net-sub"))
            .key(NET).control(toggle).node(kit)];
        for h in &self.net_hosts {
            let name = h.clone();
            let take = remove("list-remove-symbolic", t!("settings-ai-stop-site")).on_press(on::<AiPrograms>(move |me, k| {
                let list = me.net_hosts.iter().filter(|x| **x != name).cloned().collect();
                me.net_hosts_set(k, list);
            }));
            rows.push(line(h.clone()).key(format!("ai:net:{h}")).control(take).node(kit));
        }
        let typed = entry(self.typed_net.clone(), t!("settings-ai-eg-site"), |t| on::<AiPrograms>(move |me, _| me.typed_net = t.clone()))
            .on_submit(on::<AiPrograms>(|me, k| me.add_net(k)));
        let add = button("list-add-symbolic", t!("settings-ai-let-site")).on_press(on::<AiPrograms>(|me, k| me.add_net(k)));
        rows.push(line(t!("settings-ai-add-site")).sub(t!("settings-ai-add-site-sub")).key(NET_ADD).control(add).below(typed).node(kit));
        group(t!("settings-ai-network")).icon("network-wired-symbolic")
            .about(t!("settings-ai-network-about"))
            .span(5, 7).rows(rows).node(kit)
    }

    fn fence_card(&self, kit: &Kit) -> Node<Msg> {
        let toggle = switch(self.enabled, t!("settings-ai-keep-inside"), |v| on::<AiPrograms>(move |me, k| me.fence(k, v)));
        let mut sub = t!("settings-ai-hides-home").to_string();
        if !self.enabled {
            sub = t!("settings-ai-fence-off").to_string();
        }
        group(t!("settings-ai-the-fence")).icon("security-high-symbolic")
            .about(t!("settings-ai-fence-about"))
            .span(5, 7)
            .rows(vec![line(t!("settings-ai-keep-inside")).sub(sub).key(FENCE).control(toggle).node(kit)])
            .node(kit)
    }

    fn programs_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows: Vec<Node<Msg>> = Vec::new();
        for (name, fenced) in &self.programs {
            let n = name.clone();
            let toggle = switch(*fenced, t!("settings-ai-fence-name", name = name), move |v| {
                let n = n.clone();
                on::<AiPrograms>(move |me, k| me.set_program(k, &n, v))
            });
            let n = name.clone();
            let take = remove("list-remove-symbolic", t!("settings-ai-take-off", name = name))
                .on_press(on::<AiPrograms>(move |me, k| me.drop_program(k, &n)));
            rows.push(line(name.clone()).key(format!("ai:program:{name}")).controls(vec![toggle, take]).node(kit));
            if !fence::own_settings_of(name).is_empty() {
                let n = name.clone();
                let mine = switch(self.own.contains(name), t!("settings-ai-own-for", name = name), move |v| {
                    let n = n.clone();
                    on::<AiPrograms>(move |me, k| me.own_settings(k, &n, v))
                });
                rows.push(line(t!("settings-ai-own"))
                    .sub(t!("settings-ai-own-sub"))
                    .key(format!("ai:own:{name}")).control(mine).node(kit));
            }
        }
        let typed = entry(self.typed_program.clone(), t!("settings-ai-eg-program"), |t| on::<AiPrograms>(move |me, _| me.typed_program = t.clone()))
            .on_submit(on::<AiPrograms>(|me, k| me.add_program(k)));
        let add = button("list-add-symbolic", t!("settings-ai-add-program")).on_press(on::<AiPrograms>(|me, k| me.add_program(k)));
        rows.push(line(t!("settings-ai-add-program")).sub(t!("settings-ai-add-program-sub")).key(PROGRAM).control(add).below(typed).node(kit));
        group(t!("settings-ai-title")).icon("application-x-executable-symbolic")
            .about(t!("settings-ai-programs-about"))
            .span(5, 7).rows(rows).node(kit)
    }

    fn shared_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows: Vec<Node<Msg>> = Vec::new();
        if self.shared.is_empty() {
            rows.push(line(t!("settings-ai-nothing-shared")).sub(t!("settings-ai-none-seen")).node(kit));
        }
        for s in &self.shared {
            let (p, q) = (s.path.clone(), s.path.clone());
            let ro = switch(s.read_only, t!("settings-ai-only-read"), move |v| {
                let p = p.clone();
                on::<AiPrograms>(move |me, k| me.read_only(k, &p, v))
            });
            let take = remove("list-remove-symbolic", t!("settings-ai-make-private"))
                .on_press(on::<AiPrograms>(move |me, k| me.unshare(k, &q)));
            rows.push(line(s.path.to_string_lossy().into_owned()).sub(t!("settings-ai-only-read-short")).key(format!("ai:share:{}", s.path.display()))
                .controls(vec![ro, take]).node(kit));
        }
        let add = button("folder-new-symbolic", t!("settings-ai-share-folder")).on_press(on::<AiPrograms>(|me, k| me.choose_folder(k)));
        rows.push(line(t!("settings-ai-share-folder")).sub(t!("settings-ai-share-folder-sub")).key(SHARE).control(add).node(kit));
        group(t!("settings-ai-shared")).icon("folder-symbolic")
            .about(t!("settings-ai-shared-about"))
            .span(5, 7).rows(rows).node(kit)
    }

    fn ssh_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows: Vec<Node<Msg>> = Vec::new();
        for h in &self.hosts {
            let name = h.clone();
            let take = remove("list-remove-symbolic", t!("settings-ai-stop-computer")).on_press(on::<AiPrograms>(move |me, k| {
                let list = me.hosts.iter().filter(|x| **x != name).cloned().collect();
                me.hosts_set(k, list);
            }));
            rows.push(line(h.clone()).key(format!("ai:host:{h}")).control(take).node(kit));
        }
        let typed = entry(self.typed_host.clone(), t!("settings-ai-eg-computer"), |t| on::<AiPrograms>(move |me, _| me.typed_host = t.clone()))
            .on_submit(on::<AiPrograms>(|me, k| me.add_host(k)));
        let add = button("list-add-symbolic", t!("settings-ai-let-computer")).on_press(on::<AiPrograms>(|me, k| me.add_host(k)));
        rows.push(line(t!("settings-ai-add-computer")).sub(t!("settings-ai-add-computer-sub")).key(HOST).control(add).below(typed).node(kit));
        match &self.public {
            Some(key) => rows.push(note(t!("settings-ai-put-key", key = key))),
            None => {
                let make = button("dialog-password-symbolic", t!("settings-ai-make-key")).on_press(on::<AiPrograms>(|me, k| me.make_key(k)));
                rows.push(line(t!("settings-ai-make-key-for")).sub(t!("settings-ai-make-key-sub")).control(make).node(kit));
            }
        }
        group(t!("settings-ai-ssh")).icon("network-server-symbolic")
            .about(t!("settings-ai-ssh-about"))
            .span(5, 7).rows(rows).node(kit)
    }

    fn keys_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows: Vec<Node<Msg>> = Vec::new();
        for n in &self.names {
            let name = n.clone();
            let take = remove("list-remove-symbolic", t!("settings-ai-stop-key")).on_press(on::<AiPrograms>(move |me, k| me.drop_key(k, &name)));
            rows.push(line(n.clone()).sub(t!("settings-ai-given")).key(format!("ai:key:{n}")).control(take).node(kit));
        }
        let typed = entry(self.typed_key.clone(), t!("settings-ai-eg-key"), |t| on::<AiPrograms>(move |me, _| me.typed_key = t.clone()))
            .on_submit(on::<AiPrograms>(|me, k| me.add_key(k))).secret_of(Secret::Key).key("ai:key-entry");
        let add = button("list-add-symbolic", t!("settings-ai-give-key")).on_press(on::<AiPrograms>(|me, k| me.add_key(k)));
        rows.push(line(t!("settings-ai-add-key")).sub(t!("settings-ai-add-key-sub")).key(KEYS).control(add).below(typed).node(kit));
        group(t!("settings-ai-keys")).icon("dialog-password-symbolic")
            .about(t!("settings-ai-keys-about"))
            .span(5, 7).rows(rows).node(kit)
    }
}

impl Page for AiPrograms {
    fn id(&self) -> &'static str {
        "ai-programs"
    }
    fn title(&self) -> &'static str {
        t!("settings-ai-title")
    }
    fn icon(&self) -> &'static str {
        "security-high-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-ai-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-ai-help")
    }
    fn shown(&mut self, _kit: &mut Kit) {
        self.look();
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let items = vec![
            headline(kit, t!("settings-ai-note")),
            note(t!("settings-ai-inside")),
            self.fence_card(kit),
            self.programs_card(kit),
            self.shared_card(kit),
            self.ssh_card(kit),
            self.keys_card(kit),
            self.net_card(kit),
        ];
        page(kit, self.title(), self.help(), items)
    }
}

#[cfg(test)]
impl AiPrograms {

    pub(crate) fn made_up() -> AiPrograms {
        AiPrograms {
            root: PathBuf::from("/nonexistent/ai"), home: PathBuf::from("/home/john"), enabled: true,
            programs: [("claude", true), ("codex", false), ("aider", true)].iter().map(|(n, o)| (n.to_string(), *o)).collect(),
            own: vec!["claude".into()],
            shared: vec![fence::Share { path: "/home/john/Projects".into(), read_only: false }, fence::Share { path: "/home/john/Statements".into(), read_only: true }],
            hosts: vec!["nas.example.com".into()], names: vec!["ANTHROPIC_API_KEY".into()], public: Some("ssh-ed25519 AAAA example".into()),
            typed_program: String::new(), typed_host: String::new(), typed_key: String::new(), typed_net: String::new(), net_on: false, net_hosts: vec!["api.anthropic.com".into()], asking: None,
        }
    }
}
