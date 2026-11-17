//! Firewall and blocking: ports the person opens by hand for a program of theirs, from the home network only.

use super::Blocking;
use crate::common::firewall::{self, Door, Proto};
use crate::settings::ui::{admin_run, dropdown_beside, group, line, on, Kit, Msg};
use crate::toolkit::t;
use crate::toolkit::widget::{button, column, entry, row, word_button, Length, Node};

const ADD: &str = "network:ports:add";

fn port_key(d: &Door) -> String {
    format!("network:ports:{d}")
}

fn protocols(at: usize) -> &'static [Proto] {
    match at {
        0 => &[Proto::Tcp],
        1 => &[Proto::Udp],
        _ => &[Proto::Tcp, Proto::Udp],
    }
}

pub(super) fn to_open(typed: &str, at: usize) -> Result<Vec<Door>, &'static str> {
    let port = firewall::own_port(typed).ok_or(t!("settings-network-port-not-a-port"))?;
    Ok(protocols(at).iter().map(|&proto| Door { proto, port }).collect())
}

impl Blocking {

    pub(super) fn ports_card(&self, kit: &Kit) -> Node<Msg> {
        let mut rows: Vec<Node<Msg>> = self.own_ports.iter().map(|d| {
            let door = *d;
            line(door.words())
                .icon("network-wired-symbolic")
                .key(port_key(&door))
                .control(button("list-remove-symbolic", t!("settings-network-port-close", port = &door.words()))
                    .on_press(on::<Blocking>(move |me, k| me.close_port(k, door))))
                .node(kit)
        }).collect();
        if rows.is_empty() {
            rows.push(line(t!("settings-network-ports-none")).node(kit));
        }
        let open = on::<Blocking>(|me, k| me.open_typed(k));
        let typed = entry(self.port_typed.clone(), t!("settings-network-port-number"), |t| on::<Blocking>(move |me, _| me.port_typed = t.clone()))
            .key("network:ports:number")
            .on_submit(open.clone())
            .width(Length::Fill);
        let kinds = vec![t!("settings-network-port-tcp").to_string(), t!("settings-network-port-udp").to_string(),
                         t!("settings-network-port-both").to_string()];

        let proto = dropdown_beside("network:ports:protocol", kinds, Some(self.port_proto), t!("settings-network-port-protocol"), 0.0,
                                    |i| on::<Blocking>(move |me, _| me.port_proto = i));
        rows.push(line(t!("settings-network-port-add"))
            .key(ADD)
            .below(column(vec![row(vec![typed, word_button(t!("settings-network-port-open")).on_press(open)]).gap(8.0).width(Length::Fill), proto]).gap(8.0))
            .node(kit));
        group(t!("settings-network-ports")).icon("network-wired-symbolic")
            .about(t!("settings-network-ports-about"))
            .rows(rows)
            .node(kit)
    }

    pub(super) fn read_ports(&mut self) {
        self.own_ports = firewall::own_ports();
    }

    fn open_typed(&mut self, kit: &mut Kit) {
        match to_open(&self.port_typed, self.port_proto) {
            Err(why) => kit.say(ADD, why, false, false),
            Ok(doors) if doors.iter().all(|d| self.own_ports.contains(d)) => kit.say(ADD, t!("settings-network-port-already"), false, false),
            Ok(doors) => {
                self.port_typed.clear();
                self.open_all(kit, doors);
            }
        }
    }

    fn open_all(&mut self, kit: &mut Kit, mut doors: Vec<Door>) {
        let Some(door) = doors.pop() else { return };
        kit.say(ADD, t!("settings-network-waiting"), true, true);
        let back = kit.then::<Blocking, Result<(), String>>(move |me, k, done| {
            me.read_ports();
            if let Err(why) = done {
                k.say(ADD, &why, false, false);
            } else if doors.is_empty() {
                k.say(ADD, t!("settings-network-port-opened"), true, false);
            } else {
                me.open_all(k, doors);
            }
        });
        admin_run(&["firewall-port", "open", &door.to_string()], back);
    }

    fn close_port(&mut self, kit: &mut Kit, door: Door) {
        let key = port_key(&door);
        kit.say(&key, t!("settings-network-waiting"), true, true);
        let back = kit.then::<Blocking, Result<(), String>>(move |me, k, done| {
            me.read_ports();
            match done {
                Ok(()) => k.hush(&key),
                Err(why) => k.say(&key, &why, false, false),
            }
        });
        admin_run(&["firewall-port", "close", &door.to_string()], back);
    }
}
