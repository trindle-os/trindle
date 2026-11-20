//! The Wi-Fi panel: power, the networks in range and joining one.

use std::time::Duration;

use zeroize::Zeroizing;

use trindle_common::wifi::{self, Joined, Seen};

use crate::common::run::{capture, on_thread};
use crate::ring::{ctx::Msg, plain};
use crate::settings::ui::{icon, icon_in};
use crate::toolkit::t;
use crate::toolkit::widget::{button, card, column, entry, label, Length, Node, row, space, spinner, switch, Tone};

use super::{banded, Body, line_of, m, Quick, ROW_GAP, ROW_ICON, Well};

const WIFI_EVERY: Duration = Duration::from_secs(4);

pub(super) const WIFI_LEAVE: &str = "for d in $(nmcli -t -f DEVICE,TYPE dev | sed -n 's/:wifi$//p'); do nmcli dev disconnect \"$d\" || exit 1; done";

impl Quick {
    pub(super) fn wifi(&mut self) {
        self.body = Body::Wifi(None, Seen::default());
        let b = self.back();
        on_thread(wifi::rescan, move |()| b.then(|q| q.wifi_draw()));
        self.repeat(WIFI_EVERY, |q| q.wifi_draw());
    }

    pub(super) fn wifi_view(&self, on: Option<bool>, seen: &Seen) -> Vec<Node<Msg>> {
        let top = card(row(vec![
            label(t!("ring-quick-wifi-switch")),
            space(),
            switch(on != Some(false), t!("ring-quick-wifi-switch"), |on| m(move |q| q.wifi_power(on))).hushed(),
        ]).gap(ROW_GAP)).snug().flat();
        let rows: Vec<Node<Msg>> = match on {
            Some(false) => vec![label(t!("ring-quick-wifi-off"))],
            _ if seen.near.is_empty() => vec![label(t!("ring-quick-wifi-looking"))],
            _ => seen.near.iter().map(|n| {
                let joining = self.wifi_joining.as_deref() == Some(n.ssid.as_str());
                let name = if joining { label(t!("ring-quick-wifi-joining", ssid = &n.ssid)) } else { line_of(n.ssid.clone(), false, 400) };
                let mut line = vec![icon(format!("network-wireless-signal-{}-symbolic", wifi::signal_level(n.signal)), ROW_ICON), name];
                if n.secured {
                    line.push(icon_in("changes-prevent-symbolic", ROW_ICON, |t| t.muted));
                }
                let ssid = n.ssid.clone();
                if joining {
                    line.push(spinner());
                    return row(line).gap(ROW_GAP);
                }
                if let Some((_, text, problem)) = self.wifi_typing.as_ref().filter(|(s, _, _)| *s == n.ssid) {
                    return self.password_under(row(line).gap(ROW_GAP), text, problem);
                }
                if n.inuse {

                    line.push(button("network-offline-symbolic", t!("ring-quick-wifi-disconnect")).on(true).on_press(m(move |q| q.wifi_down())).well());
                } else {
                    let ask = seen.asks(n);
                    line.push(button("network-transmit-receive-symbolic", t!("ring-quick-wifi-connect"))
                        .on_press(m(move |q| q.wifi_connect(&ssid, ask))).well());
                }
                row(line).gap(ROW_GAP)
            }).collect(),
        };
        vec![top, self.scrolling(banded(rows))]
    }

    fn password_under(&self, line: Node<Msg>, text: &Zeroizing<String>, problem: &'static str) -> Node<Msg> {
        let typed = entry(text.as_str(), t!("ring-quick-wifi-password-box"), |t| m(move |q| q.password_typed(t.clone())))
            .key("ring:wifi-password").secret().grab(self.turn).width(Length::Fill)
            .on_submit(m(|q| q.password_done(true)));
        let mut items = vec![line, row(vec![
            typed,
            button(super::NO_ICON, t!("ring-quick-cancel")).on_press(m(|q| q.password_done(false))).well(),
            button("network-transmit-receive-symbolic", t!("ring-quick-wifi-join")).on_press(m(|q| q.password_done(true))).well(),
        ]).gap(ROW_GAP)];
        if !problem.is_empty() {
            items.push(label(problem).tone(Tone::Problem));
        }
        column(items).gap(ROW_GAP).on_escape(m(|q| q.password_done(false)))
    }

    pub(super) fn password_typed(&mut self, text: String) {
        if let Some((_, typed, _)) = &mut self.wifi_typing {
            *typed = Zeroizing::new(text);
        }
    }

    pub(super) fn password_done(&mut self, join: bool) {
        let Some((ssid, text, _)) = self.wifi_typing.take() else { return };
        if join && !text.is_empty() {
            self.join(ssid, Some(text));
        }
    }

    fn wifi_draw(&mut self) {
        if self.wifi_busy() {
            return;
        }
        let b = self.back();
        capture(&plain(&["sh", "-c", wifi::LOOK]),
                None, move |_, out, _| b.then(move |q| {
            if !q.wifi_busy() {
                let mut seen = wifi::parse(&out);

                q.order.keep(&mut seen.near, |n| n.ssid.clone());
                q.body = Body::Wifi(Some(seen.on), seen);
            }
        }));
    }

    fn wifi_power(&mut self, on: bool) {
        if let Body::Wifi(state, _) = &mut self.body {
            *state = Some(on);
        }
        let b = self.back();
        on_thread(move || wifi::power(on), move |_| b.then(|q| q.wifi_draw()));
    }

    fn wifi_down(&mut self) {
        let b = self.back();
        capture(&plain(&["sh", "-c", WIFI_LEAVE]), None, move |ok, _, err| b.then(move |q| {
            if !ok {
                eprintln!("trindle-ring: could not leave the network: {}", err.trim());
                q.say(t!("ring-quick-wifi-leave-failed"));
            }
            q.refresh_now();
        }));
    }

    pub(super) fn wifi_busy(&self) -> bool {
        self.wifi_typing.is_some() || self.wifi_joining.is_some()
    }

    pub(super) fn wifi_connect(&mut self, ssid: &str, ask: bool) {
        if ask {
            self.ask_password(ssid, "");
        } else {
            self.join(ssid.to_string(), None);
        }
    }

    fn ask_password(&mut self, ssid: &str, problem: &'static str) {
        self.turn += 1;
        self.wifi_typing = Some((ssid.to_string(), Zeroizing::new(String::new()), problem));
    }

    fn join(&mut self, ssid: String, password: Option<Zeroizing<String>>) {
        self.say("");
        self.wifi_joining = Some(ssid.clone());
        let b = self.back();
        let name = ssid.clone();
        on_thread(move || wifi::join(&ssid, password.as_deref().map(|s| s.as_str())), move |joined| b.then(move |q| {
            q.wifi_joining = None;
            match joined {
                Joined::Yes => q.refresh_now(),
                Joined::WrongPassword => q.ask_password(&name, t!("ring-quick-wifi-wrong-password")),
                Joined::Failed(why) => {
                    eprintln!("trindle-ring: could not join a network: {why}");
                    q.say(t!("ring-quick-wifi-failed"));
                }
            }
        }));
    }
}
