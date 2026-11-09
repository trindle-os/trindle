//! The Wi-Fi panel: power, the networks in range and joining one.

use std::collections::HashSet;
use std::time::Duration;

use zeroize::Zeroizing;

use crate::common::run::{capture, captured, captured_secret, on_thread};
use crate::ring::ctx::Msg;
use crate::settings::ui::{icon, icon_in};
use crate::toolkit::t;
use crate::toolkit::widget::{button, card, label, Node, row, space, switch};

use super::{banded, Body, line_of, m, Net, Quick, ROW_GAP, ROW_ICON, Well};

const WIFI_EVERY: Duration = Duration::from_secs(4);

pub(super) fn signal_level(signal: u32) -> &'static str {
    match signal {
        76.. => "excellent",
        51..=75 => "good",
        26..=50 => "ok",
        _ => "weak",
    }
}

pub(super) fn parse_wifi(out: &str) -> (bool, HashSet<String>, Vec<Net>) {
    let on = out.lines().next() == Some("enabled");
    let mut saved = HashSet::new();
    let mut near: Vec<Net> = Vec::new();
    let mut section = "";
    for l in out.lines().skip(1) {
        if let Some(s) = l.strip_prefix('#') {
            section = if s == "saved" { "saved" } else { "near" };
            continue;
        }
        if section == "saved" {
            saved.insert(l.replace("\\:", ":"));
            continue;
        }

        let mut parts = l.splitn(4, ':');
        let (Some(inuse), Some(sig), Some(sec), Some(ssid)) = (parts.next(), parts.next(), parts.next(), parts.next())
            else { continue };
        let ssid = ssid.replace("\\:", ":");
        if ssid.is_empty() {
            continue;
        }
        let net = Net { inuse: inuse == "*", signal: sig.parse().unwrap_or(0), secured: !sec.is_empty() && sec != "--", ssid };
        match near.iter_mut().find(|n| n.ssid == net.ssid) {
            Some(n) => {
                n.inuse |= net.inuse;
                n.signal = n.signal.max(net.signal);
            }
            None => near.push(net),
        }
    }
    near.sort_by_key(|n| (!n.inuse, std::cmp::Reverse(n.signal)));
    (on, saved, near)
}

impl Quick {
    pub(super) fn wifi(&mut self) {
        self.body = Body::Wifi(None, vec![], HashSet::new());
        let b = self.back();
        capture(&["nmcli", "dev", "wifi", "rescan"], None, move |_, _, _| b.then(|q| q.wifi_draw()));
        self.repeat(WIFI_EVERY, |q| q.wifi_draw());
    }

    pub(super) fn wifi_view(&self, on: Option<bool>, nets: &[Net], saved: &HashSet<String>) -> Vec<Node<Msg>> {
        let top = card(row(vec![
            label(t!("ring-quick-wifi-switch")),
            space(),
            switch(on != Some(false), t!("ring-quick-wifi-switch"), |on| m(move |q| q.wifi_power(on))),
        ]).gap(ROW_GAP)).snug().flat();
        let rows: Vec<Node<Msg>> = match on {
            Some(false) => vec![label(t!("ring-quick-wifi-off"))],
            _ if nets.is_empty() => vec![label(t!("ring-quick-wifi-looking"))],
            _ => nets.iter().map(|n| {
                let mut line = vec![icon(format!("network-wireless-signal-{}-symbolic", signal_level(n.signal)), ROW_ICON),
                                    line_of(n.ssid.clone(), false, 400)];
                if n.secured {
                    line.push(icon_in("changes-prevent-symbolic", ROW_ICON, |t| t.muted));
                }
                let ssid = n.ssid.clone();
                if n.inuse {

                    line.push(button("network-offline-symbolic", t!("ring-quick-wifi-disconnect")).on(true).on_press(m(move |q| q.wifi_down(&ssid))).well());
                } else {
                    let ask = n.secured && !saved.contains(&n.ssid);
                    line.push(button("network-transmit-receive-symbolic", t!("ring-quick-wifi-connect"))
                        .on_press(m(move |q| q.wifi_connect(&ssid, ask))).well());
                }
                row(line).gap(ROW_GAP)
            }).collect(),
        };
        vec![top, self.scrolling(banded(rows))]
    }

    fn wifi_draw(&mut self) {
        if self.wifi_asking {
            return;
        }
        let b = self.back();
        capture(&["sh", "-c", "nmcli radio wifi; echo '#saved'; nmcli -t -f NAME con show; echo '#near'; \
                               nmcli -t -f IN-USE,SIGNAL,SECURITY,SSID dev wifi list --rescan no"],
                None, move |_, out, _| b.then(move |q| {
            if !q.wifi_asking {
                let (on, saved, mut nets) = parse_wifi(&out);

                q.order.keep(&mut nets, |n| n.ssid.clone());
                q.body = Body::Wifi(Some(on), nets, saved);
            }
        }));
    }

    fn wifi_power(&mut self, on: bool) {
        if let Body::Wifi(state, _, _) = &mut self.body {
            *state = Some(on);
        }
        let b = self.back();
        capture(&["nmcli", "radio", "wifi", if on { "on" } else { "off" }], None, move |_, _, _| b.then(|q| q.wifi_draw()));
    }

    fn wifi_down(&mut self, ssid: &str) {
        let b = self.back();
        capture(&["nmcli", "con", "down", "id", ssid], None, move |_, _, _| b.then(|q| q.refresh_now()));
    }

    fn wifi_connect(&mut self, ssid: &str, ask: bool) {
        if ask {
            self.wifi_asking = true;
            self.ask_password(ssid, "");
        } else {
            self.join(ssid.to_string(), None);
        }
    }

    fn ask_password(&mut self, ssid: &str, problem: &'static str) {
        let name = ssid.to_string();
        self.ask("changes-prevent-symbolic", name.clone(), problem, None, Some((t!("ring-quick-wifi-password-box"), true)), t!("ring-quick-wifi-join"), t!("ring-quick-cancel"),
                 move |q, typed| match typed {
            Some(pw) => q.join(name, Some(pw)),
            None => q.wifi_asking = false,
        });
    }

    fn join(&mut self, ssid: String, password: Option<Zeroizing<String>>) {
        self.say(&t!("ring-quick-wifi-joining", ssid = &ssid));
        let b = self.back();
        let name = ssid.clone();
        on_thread(move || match password {
            Some(pw) => captured_secret(&["nmcli", "--ask", "dev", "wifi", "connect", &ssid], pw),
            None => captured(&["nmcli".into(), "dev".into(), "wifi".into(), "connect".into(), ssid], None),
        }, move |(ok, _out, err): (bool, String, String)| b.then(move |q| {
            if ok {
                q.wifi_asking = false;
                q.say("");
                q.refresh_now();
            } else if err.contains("Secrets were required") || err.contains("property is invalid") {
                q.say("");
                q.ask_password(&name, t!("ring-quick-wifi-wrong-password"));
            } else {
                q.wifi_asking = false;
                q.say(err.trim().lines().last().unwrap_or(t!("ring-quick-wifi-failed")));
            }
        }));
    }
}
