//! Tests for the Network page.

use super::join::{ACTIVE_JOINED, ACTIVE_LEAVING, ACTIVE_LEFT, AUTH_LOOKS, Join, Joining, REASON_NO_PASSWORD, STATE_NEED_AUTH, join_failed_words, join_step};
use super::refresh::chip_without_driver;
use super::*;

#[test]
fn a_network_keeps_its_place_when_it_is_joined() {
    use crate::settings::ui::Do;
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: Do| {});
    let kit = Kit::new(events.clone(), asks, None, None);
    let all = ["Home", "Home guests", "Cafe on the corner", "Number 14", "Printer setup", "A new one"];
    let lines = |n: &Network| -> Vec<String> {
        n.networks(&kit).said().into_iter().filter(|w| all.contains(&w.as_str())).collect()
    };
    let mut n = Network::made_up();
    let opened = ["Home", "Number 14", "Home guests", "Cafe on the corner", "Printer setup"];
    assert_eq!(lines(&n), opened);

    n.shown.active_ap = "/ap/3".into();
    n.shown.saved.insert("Cafe on the corner".into(), "/saved/3".into());
    n.shown.aps.get_mut("Cafe on the corner").unwrap().strength = 99;
    n.shown.aps.insert("A new one".into(), Ap { path: "/ap/9".into(), strength: 100, secure: true, enterprise: false });
    n.arrange();
    assert_eq!(lines(&n), ["Home", "Number 14", "Home guests", "Cafe on the corner", "Printer setup", "A new one"],
               "no line moved, and the new one is last");

    n.shown.aps.remove("Home guests");
    n.arrange();
    assert_eq!(lines(&n), ["Home", "Number 14", "Cafe on the corner", "Printer setup", "A new one"]);

    n.order.forget();
    n.arrange();
    assert_eq!(lines(&n), ["Cafe on the corner", "Home", "Number 14", "A new one", "Printer setup"]);
}

#[test]
fn the_networks_card_stays_greyed_with_wifi_off() {
    use crate::settings::ui::Do;
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: Do| {});
    let kit = Kit::new(events.clone(), asks, None, None);
    let off = Network::flipped().remove(1);
    let said = off.view(&kit).said();
    assert!(said.contains(&"Networks in range".to_string()));
    assert!(said.contains(&"Switch on Wi-Fi to see the networks in range.".to_string()));
    let mut none = Network::flipped().remove(1);
    none.shown.wifi = false;
    assert!(!none.view(&kit).said().contains(&"Networks in range".to_string()));
}

#[test]
fn the_connection_in_one_sentence() {
    let home = ("Home".to_string(), 80);
    assert_eq!(headline_words(true, Some(&home), false), ("Connected to Home. The signal is strong.".to_string(), Tone::Plain));
    let far = ("Home".to_string(), 20);
    assert_eq!(headline_words(true, Some(&far), false).1, Tone::Warn);
    assert_eq!(headline_words(true, None, true).0, "Connected by cable.");
    assert_eq!(headline_words(false, None, false).1, Tone::Calm);
    assert_eq!(headline_words(true, None, false).1, Tone::Warn);
}

#[test]
fn a_wifi_chip_with_no_driver_is_seen() {
    let at = std::env::temp_dir().join(format!("trindle-pci-{}", std::process::id()));
    let chip = at.join("0000:03:00.0");
    std::fs::create_dir_all(&chip).unwrap();
    std::fs::write(chip.join("class"), "0x028000\n").unwrap();
    assert!(chip_without_driver(&at));
    std::fs::create_dir(chip.join("driver")).unwrap();
    assert!(!chip_without_driver(&at));
    let _ = std::fs::remove_dir_all(&at);
    assert!(!chip_without_driver(&at));
}

#[test]
fn a_join_is_joined_only_once_the_network_says_so() {

    assert_eq!(join_step(Some(1), 50, 0, 0), Join::Waiting);

    assert_eq!(join_step(Some(ACTIVE_JOINED), STATE_ACTIVATED, 0, 0), Join::Joined);

    assert_eq!(join_step(Some(ACTIVE_LEFT), 120, 7, 0), Join::Failed(7));
    assert_eq!(join_step(Some(ACTIVE_LEAVING), 110, 53, 0), Join::Failed(53));
    assert_eq!(join_step(None, STATE_DISCONNECTED, 11, 0), Join::Failed(11));

    assert_eq!(join_step(Some(1), STATE_NEED_AUTH, 0, 1), Join::Waiting);
    assert_eq!(join_step(Some(1), STATE_NEED_AUTH, 0, AUTH_LOOKS), Join::Failed(REASON_NO_PASSWORD));
}

#[test]
fn why_a_join_failed_is_said_in_words_that_say_what_to_do() {
    assert_eq!(join_failed_words(7, Password::Typed), "That password wasn't accepted. Try it again.");
    assert_eq!(join_failed_words(8, Password::Typed), "That password wasn't accepted. Try it again.");
    assert_eq!(join_failed_words(11, Password::Kept),
               "The saved password for this network wasn't accepted. Use the bin button to forget the network, then join it again.");
    assert_eq!(join_failed_words(53, Password::None), "Trindle couldn't find that network. Check its name, and check it is in range.");
    assert_eq!(join_failed_words(17, Password::Typed), "This computer joined the network but didn't get an address. Try again, or restart the router.");

    let honest = "This computer couldn't join the network. Move closer to the router, then try again.";
    assert_eq!(join_failed_words(8, Password::None), honest);
    assert_eq!(join_failed_words(0, Password::Typed), honest);
    assert_eq!(join_failed_words(999, Password::Kept), honest);

    let bad = Failed::new("org.freedesktop.NetworkManager.Settings.Connection.InvalidProperty", "802-11-wireless-security.psk: property is invalid");
    assert_eq!(refused_words(&bad), "That can't be a Wi-Fi password. Check it, then try again.");
    assert_eq!(refused_words(&Failed::new("org.freedesktop.NetworkManager.UnknownDevice", "Device not found")), "Couldn't join the network.");
}

#[test]
fn a_wrong_password_is_said_under_its_network_and_the_box_stays_to_try_again() {
    use crate::settings::ui::stand::Stand;
    let mut s = Stand::new(Box::new(Network::made_up()));
    let key = ap_key("Number 14");
    let joining = |how: Password| Joining { key: ap_key("Number 14"), device: "/device/1".into(), active: "/active/1".into(),
                                           added: Some("/settings/9".into()), password: how, looks: 4, auth: AUTH_LOOKS };
    s.page::<Network, _>(|n, k| {
        n.asking = Some(Asking::Password("Number 14".into()));
        n.pass_typed = "wrong-pass".into();
        k.say(&ap_key("Number 14"), "Joining...", true, true);
        n.join_ended(k, joining(Password::Typed), Join::Failed(REASON_NO_PASSWORD));
    });
    assert_eq!(s.red(&key).as_deref(), Some("That password wasn't accepted. Try it again."));
    assert!(!s.glows(&key));
    assert!(s.page::<Network, _>(|n, _| n.asking == Some(Asking::Password("Number 14".into()))), "the box is still there");

    s.page::<Network, _>(|n, k| n.join_ended(k, joining(Password::Typed), Join::Joined));
    assert_eq!(s.red(&key), None);
    assert!(s.glows(&key));
    assert!(s.page::<Network, _>(|n, _| n.asking.is_none() && n.pass_typed.is_empty()));
}

#[test]
fn the_wifi_switch_never_does_nothing_and_never_stays_wrongly_switched() {
    use crate::settings::ui::stand::Stand;
    let mut s = Stand::new(Box::new(Network::made_up()));

    s.page::<Network, _>(|n, k| n.switch_wifi(k, false));
    assert_eq!(s.red(WIFI).as_deref(), Some(crate::settings::ui::no_system_bus()));
    assert!(s.page::<Network, _>(|n, _| n.enabled));

    s.page::<Network, _>(|_, k| disconnect(k, "/device/1", "network:wired:/device/1"));
    assert_eq!(s.red("network:wired:/device/1").as_deref(), Some(crate::settings::ui::no_system_bus()));
}

#[test]
fn the_headline_follows_the_language_and_falls_back_word_by_word() {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
    let french = "settings-network-top-cable = Connecté par câble.\n";
    with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
        assert_eq!(headline_words(true, None, true).0, "Connecté par câble.");
        assert_eq!(headline_words(false, None, false).0, "Wi-Fi is off, so this computer isn't connected.");
    });
}

#[test]
fn a_password_typed_and_not_sent_is_cleared_when_the_page_is_left() {
    use crate::settings::ui::stand::Stand;
    let mut s = Stand::new(Box::new(Network::made_up()));
    s.page::<Network, _>(|n, k| {
        n.asking = Some(Asking::Password("Number 14".into()));
        n.pass_typed = "half-typed".into();
        n.hidden(k);
        assert!(n.pass_typed.is_empty() && n.asking.is_none());
    });
}

#[test]
fn a_second_wifi_adapter_does_not_run_into_the_firsts_counts() {
    use crate::settings::ui::stand::Stand;
    let mut s = Stand::new(Box::new(Network::made_up()));
    let wifi = || -> crate::toolkit::bus::Props {
        std::iter::once(("DeviceType".to_string(), zbus::zvariant::Value::from(2u32 ).try_into().unwrap())).collect()
    };
    s.page::<Network, _>(|n, k| {
        n.wifi_device = None;
        n.next = Found { waiting: 2, ..Found::default() };
        n.got_device(k, "/device/1", wifi());
        assert_eq!((n.wifi_device.as_deref(), n.next.waiting), (Some("/device/1"), 2), "the first adapter is being read");
        n.got_device(k, "/device/2", wifi());
        assert_eq!((n.wifi_device.as_deref(), n.next.waiting), (Some("/device/1"), 1), "the second adapter took the page's device or was read as well");
    });
}

#[test]
fn a_network_that_needs_a_username_says_so_and_asks_for_no_password() {
    use crate::settings::ui::stand::Stand;
    let mut s = Stand::new(Box::new(Network::made_up()));
    let ap = Ap { path: "/ap/20".into(), strength: 70, secure: true, enterprise: true };
    s.page::<Network, _>(|n, k| {
        n.connect(k, "University", &ap);
        assert!(n.asking.is_none(), "a password box was opened for a network that cannot use one");
    });
    assert_eq!(s.red(&ap_key("University")).as_deref(), Some("This network needs a username as well as a password. Trindle can't join those yet."));
}

#[test]
fn no_line_under_a_network_control_explains_it() {
    use crate::settings::ui::Do;
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: Do| {});
    let kit = Kit::new(events, asks, None, None);
    let mut said = Network::made_up().view(&kit).said().join("|");
    said.push_str(&Network::flipped().remove(1).view(&kit).said().join("|"));
    for old in ["Type the exact name, with capitals.", "Trindle checks again each time you open this page."] {
        assert!(!said.contains(old), "{old} is still on the page");
    }
}
