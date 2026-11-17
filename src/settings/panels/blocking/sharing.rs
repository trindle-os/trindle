//! Firewall and blocking: letting other computers connect for shared folders and printers, and the folders shared.

use super::Blocking;
use crate::toolkit::widget::banded_list;
use crate::common::sharing::{self, Problem, Shared};
use crate::settings::ui::{admin_step, group, line, on, Kit, Msg};
use crate::toolkit::t;
use crate::toolkit::widget::{button, switch, Node};

impl Blocking {

    pub(super) fn sharing_card(&self, kit: &Kit) -> Node<Msg> {
        let share = line(t!("settings-network-share-switch"))
            .key(SHARING)
            .control(switch(self.sharing, t!("settings-network-share-switch"), |v| on::<Blocking>(move |me, k| me.set_sharing(k, v))))
            .node(kit);
        let mut rows = vec![share];
        match &self.shared {
            None => rows.push(line(t!("settings-network-share-looking")).node(kit)),
            Some(Err(Problem::Other(w))) => rows.push(line(t!("settings-network-share-unreadable")).sub(w.clone()).node(kit)),
            Some(Ok(list)) if !list.is_empty() => {
                rows.extend(banded_list(list.iter().map(|s| shared_line(kit, s)).collect()));
                if !self.sharing {
                    rows.push(line(t!("settings-network-not-reachable"))
                        .sub(t!("settings-network-not-reachable-sub")).node(kit));
                }
            }
            _ => rows.push(line(t!("settings-network-no-shared")).node(kit)),
        }
        group(t!("settings-network-sharing")).icon("folder-publicshare-symbolic")
            .about(t!("settings-network-sharing-about"))
            .rows(rows)
            .node(kit)
    }
    pub(super) fn read_shared(&mut self, kit: &mut Kit) {
        let back = kit.then::<Blocking, Result<Vec<Shared>, Problem>>(|me, _, got| me.shared = Some(got));
        crate::common::run::on_thread(sharing::list, back);
    }

    fn stop_share(&mut self, kit: &mut Kit, name: String) {
        let key = shared_key(&name);
        kit.say(&key, t!("settings-network-stopping"), true, true);
        let back = kit.then::<Blocking, (Result<(), String>, Result<Vec<Shared>, Problem>)>(move |me, k, (got, list)| {
            match got {
                Ok(()) => k.hush(&key),
                Err(e) => k.say(&key, &e, false, true),
            }
            me.shared = Some(list);
        });
        crate::common::run::on_thread(move || (sharing::remove(&name), sharing::list()), back);
    }

    fn set_sharing(&mut self, kit: &mut Kit, on: bool) {
        self.sharing = on;
        let done = if on { t!("settings-network-sharing-allowed") } else { t!("settings-network-kept-out") };
        admin_step::<Blocking>(kit, SHARING, &["firewall-sharing", if on { "on" } else { "off" }], t!("settings-network-waiting"), done, move |me, _, ok| {
            if !ok {
                me.sharing = !on;
            }
        });
    }
}

const SHARING: &str = "network:sharing";

fn shared_key(name: &str) -> String {
    format!("network:shared:{name}")
}

fn access_words(s: &Shared) -> &'static str {
    match (s.write, s.guests) {
        (false, false) => t!("settings-network-access-open"),
        (true, false) => t!("settings-network-access-change"),
        (false, true) => t!("settings-network-access-guests-open"),
        (true, true) => t!("settings-network-access-guests-change"),
    }
}

fn shared_line(kit: &Kit, s: &Shared) -> Node<Msg> {
    let name = s.name.clone();
    let home = crate::common::files::home();
    let place = match s.path.strip_prefix(&home) {
        Ok(rest) if !rest.as_os_str().is_empty() => format!("~/{}", rest.display()),
        _ => s.path.display().to_string(),
    };
    line(s.name.clone())
        .icon("folder-publicshare-symbolic")
        .sub(t!("settings-network-shared-sub", place = &place, access = access_words(s)))
        .key(shared_key(&s.name))
        .control(button("media-playback-stop-symbolic", t!("settings-network-stop-sharing", name = &s.name))
            .on_press(on::<Blocking>(move |me, k| me.stop_share(k, name.clone()))))
        .node(kit)
}
