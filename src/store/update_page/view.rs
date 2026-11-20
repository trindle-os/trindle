//! The Updates page: its cards in order and its main buttons.

use crate::toolkit::t;
use crate::store::look::ring_with_figure;
use crate::store::{go, Msg, Store};
use crate::toolkit::theme;
use crate::toolkit::widget::{button, label, main_action, row, spinner, Length, Node, Tone};
use crate::store::screen::{screen, title_row};
use super::cards::{apps_card, held_card, plugins_card};
use super::check::{check_all};
use super::install::{install_all, restart};
use super::system_cards::{debian_card, firmware_card, security_card, trindle_card};
use super::words::{headline, overall_ring};
use super::{Phase, REMIND_ROW, going_under, phase, rows};

pub fn view(s: &Store) -> Node<Msg> {
    let (_, words) = headline(s);
    let mut fw = firmware_card(s).key(crate::store::spot::SPOT);
    if s.spot.last == crate::store::spot::SPOT {
        fw = fw.lit(s.fade_in.lit);
    }
    let mut tiles = vec![
        apps_card(s),
        trindle_card(s),
        debian_card(s),
        fw,
    ];

    if !s.plugins.installed.is_empty() {
        let card = plugins_card(s).key(crate::store::spot::SPOT_PLUGINS);
        tiles.push(if s.spot.last == crate::store::spot::SPOT_PLUGINS { card.lit(s.fade_in.lit) } else { card });
    }

    if s.upd.held_card {
        tiles.push(held_card(s));
    }

    tiles.push(security_card(s));
    tiles.splice(0..0, rows::red_words(s, REMIND_ROW));
    screen(title_row(t!("store-nav-updates"), actions(s)), words, tiles)
}

fn actions(s: &Store) -> Vec<Node<Msg>> {

    let held = s.other.is_some() || going_under(s, &["pl:"]).is_some();
    let again = || button("view-refresh-symbolic", t!("store-upd-check-again")).on_press(go(check_all)).off_because(crate::common::ui::wait_while(held));
    match phase(s) {
        Phase::Checking => vec![spinner(), main_action("view-refresh-symbolic", t!("store-upd-checking-main")).off(true)],

        Phase::Installing => vec![ring_with_figure(overall_ring(s), theme::sheet().action_height), main_action(crate::settings::ui::UPDATE_ICON, t!("store-upd-installing-main")).off(true)],
        Phase::Found => {
            let n = s.waiting_updates();
            vec![again(), main_action(crate::settings::ui::UPDATE_ICON, t!("store-upd-install-n", n = n))
                .on_press(go(install_all)).off_because(crate::common::ui::wait_while(held))]
        }
        Phase::Restart => vec![again(), remind_later(s), main_action("system-reboot-symbolic", t!("store-upd-restart-now")).on_press(go(|_| restart())).off_because(crate::common::ui::wait_while(held))],
        Phase::Done => vec![main_action("view-refresh-symbolic", t!("store-upd-check-for")).on_press(go(check_all)).off_because(crate::common::ui::wait_while(held))],
    }
}

pub(super) fn remind_choices() -> Vec<String> {
    crate::common::remind::HOURS.iter().map(|&n| t!("store-upd-remind-hours", n = n)).collect()
}

fn remind_later(s: &Store) -> Node<Msg> {
    let name = t!("store-upd-remind-later");
    let pick = crate::toolkit::widget::dropdown(remind_choices(), s.upd.reminded, name, |at| go(move |s| remind_pick(s, at)));
    row(vec![label(name), pick]).gap(8.0).centred().width(Length::Shrink)
}

fn remind_pick(s: &mut Store, at: usize) {
    remind_saved(s, at, crate::common::remind::set(at));
}

pub(super) fn remind_saved(s: &mut Store, at: usize, saved: std::io::Result<()>) {
    match saved {
        Ok(()) => {
            s.upd.reminded = Some(at);
            s.words.said.remove(REMIND_ROW);
        }
        Err(e) => {
            s.words.said.insert(REMIND_ROW.into(), (t!("store-upd-remind-not-saved", why = e.to_string()), Tone::Problem));
        }
    }
}

pub(super) fn checking(s: &Store, key: &str) -> Vec<Node<Msg>> {
    if s.upd.checking.contains(key) { vec![spinner()] } else { Vec::new() }
}
