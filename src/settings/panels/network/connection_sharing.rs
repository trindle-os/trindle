//! Network.

use super::{Network, banded};
use crate::common::run::{admin_error, admin_helper, capture};
use crate::common::sharing::{self, Problem, Shared};
use crate::settings::ui::{group, line, on, Kit, Msg};
use crate::toolkit::t;
use crate::toolkit::widget::{button, switch, Node};

impl Network {
    pub(super) fn sharing_card(&self, kit: &Kit) -> Node<Msg> {
        let share = line(t!("settings-network-share-switch"))
            .sub(t!("settings-network-share-switch-sub"))
            .key(SHARING)
            .control(switch(self.sharing, t!("settings-network-share-switch"), |v| on::<Network>(move |me, k| me.set_sharing(k, v))))
            .node(kit);
        let mut rows = vec![share];
        match &self.shared {
            None => rows.push(line(t!("settings-network-share-looking")).node(kit)),
            Some(Err(Problem::Other(w))) => rows.push(line(t!("settings-network-share-unreadable")).sub(w.clone()).node(kit)),
            Some(Ok(list)) if !list.is_empty() => {
                rows.extend(banded(list.iter().map(|s| shared_line(kit, s)).collect()));
                if !self.sharing {
                    rows.push(line(t!("settings-network-not-reachable"))
                        .sub(t!("settings-network-not-reachable-sub")).node(kit));
                }
            }
            _ => rows.push(line(t!("settings-network-no-shared"))
                .sub(t!("settings-network-no-shared-sub")).node(kit)),
        }
        group(t!("settings-network-sharing")).icon("folder-publicshare-symbolic")
            .about(t!("settings-network-sharing-about"))
            .rows(rows)
            .node(kit)
    }
    pub(super) fn read_shared(&mut self, kit: &mut Kit) {
        let back = kit.then::<Network, Result<Vec<Shared>, Problem>>(|me, _, got| me.shared = Some(got));
        crate::common::run::on_thread(sharing::list, back);
    }

    fn stop_share(&mut self, kit: &mut Kit, name: String) {
        let key = shared_key(&name);
        kit.say(&key, t!("settings-network-stopping"), true, true);
        let back = kit.then::<Network, (Result<(), String>, Result<Vec<Shared>, Problem>)>(move |me, k, (got, list)| {
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
        kit.say(SHARING, t!("settings-network-waiting"), true, true);
        let back = kit.then::<Network, (bool, String, String)>(move |me, k, (ok, _, err)| {
            let err = admin_error(&err);
            k.done(SHARING, if ok { None } else { Some(&err) }, if on { t!("settings-network-sharing-allowed") } else { t!("settings-network-kept-out") });
            if !ok {
                me.sharing = !on;
            }
        });
        let helper = admin_helper();
        capture(&["pkexec", &helper, "firewall-sharing", if on { "on" } else { "off" }], None,
                move |ok, out, err| back((ok, out, err)));
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
            .on_press(on::<Network>(move |me, k| me.stop_share(k, name.clone()))))
        .node(kit)
}
