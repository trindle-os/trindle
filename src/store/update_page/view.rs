//! The Updates page.

use crate::toolkit::t;
use crate::store::look::ring_with_figure;
use crate::store::{go, Msg, Store};
use crate::toolkit::theme;
use crate::toolkit::widget::{button, main_action, spinner, Node};
use crate::store::screen::{screen, title_row};
use super::cards::{apps_card, held_card, plugins_card};
use super::check::{check_all};
use super::install::{install_all, restart};
use super::system_cards::{firmware_card, security_card, system_card};
use super::words::{headline, overall};
use super::{Phase, going_under, phase};

pub fn view(s: &Store) -> Node<Msg> {
    let (_, words) = headline(s);
    let mut fw = firmware_card(s).key(crate::store::spot::SPOT);
    if s.spot_last == crate::store::spot::SPOT {
        fw = fw.lit(s.lit);
    }
    let mut tiles = vec![
        apps_card(s),
        system_card(s),
        fw,
    ];
    if !s.plugins.installed.is_empty() {
        let card = plugins_card(s).key(crate::store::spot::SPOT_PLUGINS);
        tiles.push(if s.spot_last == crate::store::spot::SPOT_PLUGINS { card.lit(s.lit) } else { card });
    }
    if s.upd.held_card {
        tiles.push(held_card(s));
    }
    tiles.push(security_card(s));
    screen(title_row(t!("store-nav-updates"), actions(s)), words, tiles)
}

fn actions(s: &Store) -> Vec<Node<Msg>> {
    let held = s.other.is_some() || going_under(s, &["pl:"]).is_some();
    let again = || button("view-refresh-symbolic", t!("store-upd-check-again")).on_press(go(check_all)).off_because(crate::common::ui::wait_while(held));
    match phase(s) {
        Phase::Checking => vec![spinner(), main_action("view-refresh-symbolic", t!("store-upd-checking-main")).off(true)],
        Phase::Installing => vec![ring_with_figure(overall(s), theme::sheet().action_height), main_action(crate::settings::ui::UPDATE_ICON, t!("store-upd-installing-main")).off(true)],
        Phase::Found => {
            let n = s.waiting_updates();
            vec![again(), main_action(crate::settings::ui::UPDATE_ICON, t!("store-upd-install-n", n = n))
                .on_press(go(install_all)).off_because(crate::common::ui::wait_while(held))]
        }
        Phase::Restart => vec![again(), main_action("system-reboot-symbolic", t!("store-upd-restart-now")).on_press(go(|_| restart())).off_because(crate::common::ui::wait_while(held))],
        Phase::Done => vec![main_action("view-refresh-symbolic", t!("store-upd-check-for")).on_press(go(check_all)).off_because(crate::common::ui::wait_while(held))],
    }
}

pub(super) fn checking(s: &Store, key: &str) -> Vec<Node<Msg>> {
    if s.upd.checking.contains(key) { vec![spinner()] } else { Vec::new() }
}
