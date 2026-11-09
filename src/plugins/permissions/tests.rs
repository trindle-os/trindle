use super::*;
use crate::plugins::pin;
use crate::plugins::fixtures::*;
use crate::plugins::fields::field_shown;
use crate::plugins::install::Background;
use crate::plugins::install::put_in;
use crate::plugins::manifest::check;
use crate::plugins::manifest::folders;
use crate::plugins::ports::ports_to_open;

#[test]
fn an_update_keeps_what_was_allowed_and_allows_nothing_new() {
    let root = std::env::temp_dir().join(format!("trindle-told-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let v1 = folder_with("told1", GOOD);
    let first = put_in(&root, &v1).unwrap();
    assert_eq!((first.plugin.version.as_str(), first.background.clone()), ("1.0", Background::None));
    let at = root.join("org.example.Weather");
    assert!(boxed::allowed_at(&at, PERMISSIONS).is_empty(), "new, it is allowed nothing");
    set_allow(&at, &[], "network", true).unwrap();

    let v2 = folder_with("told2", &GOOD.replace("Version=1.0", "Version=1.1"));
    assert_eq!(put_in(&root, &v2).unwrap().plugin.version, "1.1");
    assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["network"]);

    let v3 = folder_with("told3", &GOOD.replace("Version=1.0", "Version=1.2").replace("network;notifications", "network;notifications;microphone"));
    assert_eq!(put_in(&root, &v3).unwrap().plugin.version, "1.2");
    assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["network"], "nothing new was allowed by the update");

    let bad = folder_with("told4", &GOOD.replace("Version=1.0", "Version=1.3").replace("Run=bin/go --show", "Run=/usr/bin/sh"));
    assert!(put_in(&root, &bad).is_err());
    assert_eq!(check(&at.join("app")).unwrap().version, "1.2");
    for d in [v1, v2, v3, bad] {
        let _ = std::fs::remove_dir_all(d);
    }
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_firewall_step_says_what_is_true_of_listening() {
    assert_eq!(firewall_said(true, "No password was given, so nothing changed.", Ok(())),
               "its ports were not opened, so listening is off again: No password was given, so nothing changed.");
    let kept = firewall_said(true, "No password was given.", Err("Permission denied (os error 13)".into()));
    assert_eq!(kept, "its ports were not opened (No password was given), and listening could not be switched off again either \
                      (Permission denied (os error 13)). Nothing is open; switch it off in Settings, Permissions.");
    assert!(!kept.contains("is off again"), "never said when it could not be put off");
    let closing = firewall_said(false, "the firewall could not be told", Ok(()));
    assert_eq!(closing, "its ports could not be closed: the firewall could not be told.");
    assert!(!closing.contains("opened") && !closing.contains("off again"));

    let source = plugin_code();
    let arm = source.split_once("(\"firewall\", id) if").unwrap().1.split_once("(\"close\", id)").unwrap().0;
    assert!(arm.contains("fit_firewall(id)"));
    let step = source.split_once("fn fit_firewall(").unwrap().1.split_once("\n}\n").unwrap().0;
    assert!(step.contains("firewall_said(opening, &e, put_off)") && !step.contains("let _ = allow("));
    assert!(step.find("let opening").unwrap() < step.find("firewall_apply(id)").unwrap());
}

#[test]
fn the_plugins_words_follow_the_language_and_fall_back_word_by_word() {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
    let french = "plugins-allow = Autoriser\n\
                  plugins-needs-none = { $name } a besoin de votre permission : 0 sur { $n } autorisées.\n\
                  plugins-settled =\n    { $n ->\n        [0] Il ne demande rien.\n        [one] La seule chose qu'il demande est autorisée.\n       *[other] Les { $n } choses qu'il demande sont autorisées.\n    }\n";
    with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
        assert_eq!(allow_word(), "Autoriser");
        assert_eq!(needs_words("Météo", 3, 0).unwrap(), "Météo a besoin de votre permission : 0 sur 3 autorisées.");
        assert_eq!(settled_words(0), "Il ne demande rien.");
        assert_eq!(settled_words(4), "Les 4 choses qu'il demande sont autorisées.");
        assert_eq!(needs_words("Météo", 3, 1).unwrap(), "Météo has only some of the permission it asks for: 1 of 3 allowed.");
        assert_eq!(pin::four(), "A PIN is 4 digits.");
    });
}

#[test]
fn ports_are_open_only_while_both_listen_and_network_are_allowed() {
    let p = listening("open", &with_listen("network;listen", LISTENING)).unwrap();
    let a = |k: &[&str]| k.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    assert!(open_ports(&p, &[]).is_empty());
    assert!(open_ports(&p, &a(&["network"])).is_empty());
    assert!(open_ports(&p, &a(&["listen"])).is_empty());
    assert_eq!(open_ports(&p, &a(&["listen", "network", "sound"])).len(), 2);
}

#[test]
fn summon_is_allowed_and_withdrawn_like_the_other_permissions_and_is_what_the_compositor_reads() {
    use crate::common::summon::{allowed_in, WORD};
    assert!(PERMISSIONS.contains(&WORD), "a plugin cannot ask for it");
    let all = std::env::temp_dir().join(format!("trindle-summon-allowed-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&all);
    let at = all.join("org.example.steer");
    std::fs::create_dir_all(&at).unwrap();
    let said = || allowed_in(&all, "org.example.steer");

    assert!(!said());
    boxed::allow_at(&at, PERMISSIONS, "control", true).unwrap();
    assert!(!said());
    boxed::allow_at(&at, PERMISSIONS, WORD, true).unwrap();
    assert!(said(), "allowed in the permission window, and the compositor would not know");

    boxed::allow_at(&at, PERMISSIONS, WORD, false).unwrap();
    assert!(!said());
    boxed::allow_at(&at, PERMISSIONS, WORD, true).unwrap();
    boxed::allow_at(&at, PERMISSIONS, "control", false).unwrap();
    assert!(!said(), "summon stood without control");

    assert_eq!(folders(), crate::common::files::data_home().join("trindle/plugins"));

    let v = |a: &[&str]| a.iter().map(|s| s.to_string()).collect::<Vec<_>>();
    assert_eq!(boxed::walls::engine_for(&v(&["control", WORD])), boxed::walls::engine_for(&v(&["control"])));
    assert_eq!(boxed::walls::engine_for(&v(&[WORD])), boxed::walls::engine_for(&v(&[])));

    let ask = crate::common::access::ask(WORD);
    assert_eq!(ask.name, "open the ring over a full-screen window, as if you pressed the key");
    assert_eq!(ask.without, "Its keys and clicks will open the ring on the desktop only, never over a full-screen window.");
    assert_eq!(crate::common::access::warning(WORD),
        Some("It can put the ring, which takes everything you type, over a game or a film without you touching a key. Only allow this for a plugin you trust."));

    assert!(!boxed::ALLOWED.contains(&WORD));
    let _ = std::fs::remove_dir_all(&all);
}

#[test]
fn an_update_that_stops_asking_for_something_takes_its_allowance_away() {
    use crate::common::summon::{allowed_in, WORD};
    let root = std::env::temp_dir().join(format!("trindle-unasked-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let ini = |asks: &str, version: &str| GOOD.replace("network;notifications", asks).replace("Version=1.0", version)
        .replace("org.example.Weather", crate::common::own_hands::SCREEN_READER);
    for dropped in PERMISSIONS.iter().copied().filter(|p| !["control", "listen"].contains(p)) {
        let both = format!("control;{dropped}");
        let v1 = folder_with("unasked1", &ini(&both, "Version=1.0"));
        let p = install_at(&root, &v1).unwrap();
        let at = root.join(&p.id);
        set_allow(&at, &[], "control", true).unwrap();
        set_allow(&at, &[], dropped, true).unwrap();
        std::fs::write(at.join("asked"), format!("control\n{dropped}\n")).unwrap();
        assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["control", dropped]);
        assert_eq!(allowed_in(&root, &p.id), dropped == WORD);

        let same = folder_with("unasked2", &ini(&both, "Version=1.1"));
        install_at(&root, &same).unwrap();
        assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["control", dropped]);
        assert_eq!(std::fs::read_to_string(at.join("asked")).unwrap(), format!("control\n{dropped}\n"));

        let less = folder_with("unasked3", &ini("control", "Version=1.2"));
        install_at(&root, &less).unwrap();
        assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["control"], "{dropped} was still allowed after an update that no longer asks for it");
        assert_eq!(std::fs::read_to_string(at.join("asked")).unwrap(), "control\n");
        assert!(!allowed_in(&root, &p.id));

        let again = folder_with("unasked4", &ini(&both, "Version=1.3"));
        let (old, new) = (check(&at.join("app")).unwrap(), check(&again).unwrap());
        assert_eq!(asks_more(&old, &new.asks, Some(&new.listen)), [crate::common::access::ask(dropped).name.to_string()]);
        install_at(&root, &again).unwrap();
        assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["control"], "{dropped} came back without the person being asked");
        assert!(!allowed_in(&root, &p.id));
        for d in [v1, same, less, again] {
            let _ = std::fs::remove_dir_all(d);
        }
        std::fs::remove_dir_all(&at).unwrap();
    }
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn listen_is_kept_and_withdrawn_like_the_other_permissions() {
    let dir = std::env::temp_dir().join(format!("trindle-listen-allowed-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    assert!(boxed::allowed_at(&dir, PERMISSIONS).is_empty());
    boxed::allow_at(&dir, PERMISSIONS, "listen", true).unwrap();
    boxed::allow_at(&dir, PERMISSIONS, "network", true).unwrap();
    assert_eq!(boxed::allowed_at(&dir, PERMISSIONS), ["listen", "network"]);
    boxed::allow_at(&dir, PERMISSIONS, "listen", false).unwrap();
    assert_eq!(boxed::allowed_at(&dir, PERMISSIONS), ["network"]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn listen_has_words_and_names_its_ports() {
    let a = crate::common::access::ask("listen");
    assert_eq!(a.name, "let other computers on your home network reach it");
    assert!(a.without.ends_with('.'));
    let p = listening("words", &with_listen("network;listen", LISTENING)).unwrap();
    assert_eq!(crate::common::access::listen_detail(&p.listen),
               "Ports TCP 24810 and UDP 24811, from computers on your home network only. It needs the internet allowed too.");
}

#[test]
fn root_opens_only_what_the_person_allowed_and_was_shown() {
    let at = tmp("allowed");
    let me = my_uid();
    let shown = "tcp:24810\nudp:24811\n";
    let m = manifest("udp:24811;tcp:24810");
    keep(&at, &m, "network\nlisten\n", Some(shown));
    assert_eq!(ports_to_open(&at, me).unwrap().len(), 2);

    keep(&at, &m, "", Some(shown));
    assert!(ports_to_open(&at, me).is_err());
    keep(&at, &m, "network\n", Some(shown));
    assert!(ports_to_open(&at, me).is_err());
    keep(&at, &m, "listen\n", Some(shown));
    assert!(ports_to_open(&at, me).is_err());

    keep(&at, &m, "network\nlisten\n", None);
    assert!(ports_to_open(&at, me).unwrap_err().contains("allowed again"));
    keep(&at, &m, "network\nlisten\n", Some("tcp:24810\n"));
    assert!(ports_to_open(&at, me).is_err());

    keep(&at, &manifest("tcp:24810;udp:24811;tcp:30000"), "network\nlisten\n", Some(shown));
    assert!(ports_to_open(&at, me).is_err());
    keep(&at, &manifest("tcp:24810;udp:24812"), "network\nlisten\n", Some(shown));
    assert!(ports_to_open(&at, me).is_err());

    keep(&at, &manifest("tcp:5900"), "network\nlisten\n", Some("tcp:5900\n"));
    assert!(ports_to_open(&at, me).is_err());

    keep(&at, &m, "network\nlisten\n", Some(shown));
    assert!(ports_to_open(&at, me.wrapping_add(1)).is_err());
    let _ = std::fs::remove_dir_all(&at);
}

#[test]
fn root_never_follows_a_link_or_reads_a_big_file() {
    let at = tmp("links");
    let me = my_uid();
    let shown = "tcp:24810\n";
    keep(&at, &manifest("tcp:24810"), "network\nlisten\n", Some(shown));
    assert!(ports_to_open(&at, me).is_ok());

    let elsewhere = at.join("elsewhere");
    for name in ["app/plugin.ini", "allowed", GRANTED] {
        let real = std::fs::read_to_string(at.join(name)).unwrap();
        std::fs::write(&elsewhere, &real).unwrap();
        std::fs::remove_file(at.join(name)).unwrap();
        std::os::unix::fs::symlink(&elsewhere, at.join(name)).unwrap();
        assert!(ports_to_open(&at, me).is_err(), "{name} was followed");
        std::fs::remove_file(at.join(name)).unwrap();
        std::fs::write(at.join(name), real).unwrap();
    }
    assert!(ports_to_open(&at, me).is_ok());

    let big = format!("{}\n# {}\n", manifest("tcp:24810"), "x".repeat(70_000));
    keep(&at, &big, "network\nlisten\n", Some(shown));
    assert!(ports_to_open(&at, me).is_err());
    let _ = std::fs::remove_dir_all(&at);
}

#[test]
fn an_update_that_names_other_ports_asks_again_and_one_that_does_not_keeps_the_choice() {
    let root = std::env::temp_dir().join(format!("trindle-update-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let ini = |listen: &str| GOOD.replace("network;notifications", "network;listen") + &format!("\n[Network]\nListen={listen}\n");
    let v1 = folder_with("upd1", &ini("tcp:24810;udp:24811"));
    let p = install_at(&root, &v1).unwrap();
    let at = root.join(&p.id);
    set_allow(&at, &p.listen, "network", true).unwrap();
    set_allow(&at, &p.listen, "listen", true).unwrap();
    std::fs::write(at.join("asked"), "network\nlisten\n").unwrap();
    assert!(ports_to_open(&at, my_uid()).is_ok());

    let same = folder_with("upd2", &ini("udp:24811;tcp:24810").replace("Version=1.0", "Version=1.1"));
    install_at(&root, &same).unwrap();
    assert!(ports_to_open(&at, my_uid()).is_ok());
    assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["network", "listen"]);

    let other = folder_with("upd3", &ini("tcp:24810;udp:24812").replace("Version=1.0", "Version=1.2"));
    install_at(&root, &other).unwrap();
    assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["network"]);
    assert_eq!(std::fs::read_to_string(at.join("asked")).unwrap(), "network\n");
    assert!(!at.join(GRANTED).exists());
    assert!(ports_to_open(&at, my_uid()).is_err());
    for d in [v1, same, other] {
        let _ = std::fs::remove_dir_all(d);
    }
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_switch_can_be_waited_on_to_be_off() {
    let ini = ROLES.replace("ShowWhen=on=true", "ShowWhen=on=false");
    let dir = folder_with("roles-off", &ini);
    let p = check(&dir).unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    let speed = p.fields.iter().find(|f| f.name == "speed").unwrap();

    assert!(field_shown(&p, speed, &vals(&[]), &[]));
    assert!(field_shown(&p, speed, &vals(&[("on", "nonsense")]), &[]));
    assert!(!field_shown(&p, speed, &vals(&[("on", "true")]), &[]));
}

#[test]
fn a_plugin_short_of_permission_is_said_in_plain_words() {
    assert_eq!(needs_words("Weather", 7, 0).as_deref(), Some("Weather needs your permission to work: 0 of 7 allowed."));
    assert_eq!(needs_words("Weather", 7, 3).as_deref(),
               Some("Weather has only some of the permission it asks for: 3 of 7 allowed."));
    assert_eq!(needs_words("Weather", 7, 7), None);
    assert_eq!(needs_words("Weather", 0, 0), None);

    assert_eq!(needs_words("Weather", 2, 9), None);
    assert_eq!(settled_words(0), "It asks for nothing.");
    assert_eq!(settled_words(1), "The one thing it asks for is allowed.");
    assert_eq!(settled_words(7), "All 7 things it asks for are allowed.");
}

#[test]
fn a_plugin_may_ask_for_the_home_network_and_for_usb_devices() {
    for (ask, asks) in [("home-network", &["home-network"][..]), ("usb-devices", &["usb-devices"]),
                        ("home-network;usb-devices", &["home-network", "usb-devices"]),
                        ("sound;usb-devices;files;home-network", &["sound", "usb-devices", "files", "home-network"])] {
        let p = asking(ask).unwrap_or_else(|e| panic!("{ask}: {e:?}"));
        assert_eq!(p.asks, asks, "{ask}");
    }
    assert!(PERMISSIONS.contains(&HOME_NETWORK) && PERMISSIONS.contains(&USB_DEVICES));
    assert_eq!((HOME_NETWORK, USB_DEVICES), ("home-network", "usb-devices"));
}

#[test]
fn a_malformed_ask_for_either_is_refused_and_says_why() {
    for bad in ["home_network", "Home-Network", "HOME-NETWORK", "homenetwork", "home network", "home-networks", "home-network:all", "home-network=1",
                "usb-device", "usb_devices", "USB-devices", "usb", "usbdevices", "usb-devices/all", "usb-devices=1", "scanner", "devices", "camera"] {
        let e = asking(bad).unwrap_err().join(" ");
        assert!(e.contains(&format!("Permission {} is not one there is", bad.split(';').next().unwrap())), "{bad}: {e}");

        assert!(e.contains("home-network") && e.contains("usb-devices"), "{bad}: {e}");
    }
    for twice in ["home-network;home-network", "usb-devices;sound;usb-devices", "network;network"] {
        let e = asking(twice).unwrap_err().join(" ");
        assert!(e.contains("is named twice"), "{twice}: {e}");
    }
    for both in ["network;home-network", "home-network;network", "home-network;usb-devices;network"] {
        let e = asking(both).unwrap_err().join(" ");
        assert!(e.contains("home-network or network, not both"), "{both}: {e}");
    }

    let e = listening("scan-listen", &with_listen("home-network;listen", LISTENING)).unwrap_err().join(" ");
    assert!(e.contains("[Network] Listen needs the network permission"), "{e}");
}

#[test]
fn both_are_off_until_the_person_allows_them_and_can_be_taken_away() {
    let root = std::env::temp_dir().join(format!("trindle-scan-allowed-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let ini = GOOD.replace("network;notifications", "home-network;usb-devices").replace("Weather", "Scan");
    let v1 = folder_with("scan1", &ini);
    let first = put_in(&root, &v1).unwrap();
    let at = root.join("org.example.Scan");
    assert_eq!(first.plugin.asks, ["home-network", "usb-devices"]);
    assert!(boxed::allowed_at(&at, PERMISSIONS).is_empty(), "new, it is allowed neither");
    set_allow(&at, &[], HOME_NETWORK, true).unwrap();
    assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["home-network"]);
    set_allow(&at, &[], USB_DEVICES, true).unwrap();
    assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["home-network", "usb-devices"]);
    set_allow(&at, &[], HOME_NETWORK, false).unwrap();
    assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["usb-devices"]);

    let v2 = folder_with("scan2", &ini.replace("Version=1.0", "Version=1.1")
        .replace("home-network;usb-devices", "home-network;usb-devices;microphone"));
    put_in(&root, &v2).unwrap();
    assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["usb-devices"], "nothing new was allowed by the update");

    let v3 = folder_with("scan3", &ini.replace("Version=1.0", "Version=1.2")
        .replace("home-network;usb-devices", "home-network"));
    put_in(&root, &v3).unwrap();
    assert!(boxed::allowed_at(&at, PERMISSIONS).is_empty(), "it was allowed {:?} though it no longer asks", boxed::allowed_at(&at, PERMISSIONS));

    let p = check(&at.join("app")).unwrap();
    let record = |list: &[&str]| std::fs::write(at.join("allowed"), list.iter().fold(String::new(), |mut s, l| { let _ = writeln!(s, "{l}"); s })).unwrap();
    record(&["usb-devices", "home-network", "network"]);
    let walled = |allowed: &[String]| {
        let gates = boxed::walls::Gates { usb: vec!["--dev-bind".into(), "/x".into(), "/dev/bus/usb/001/002".into()], home_room: Some((1000, 1000)), ..boxed::walls::Gates::default() };
        boxed::walls::walls(&at, allowed, &gates, &["x".to_string()]).join(" ")
    };
    let box_args = walled(&allowed_in_folder(&at, &p));
    assert!(box_args.contains("--share-net --unshare-user --disable-userns --uid 1000 --gid 1000 --cap-drop ALL"), "home-network was allowed: {box_args}");
    assert!(!box_args.contains("/dev/bus/usb/001/002"), "usb-devices is not asked for any more: {box_args}");
    for d in [v1, v2, v3] {
        let _ = std::fs::remove_dir_all(d);
    }
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_permission_is_switched_on_only_where_the_person_answers() {
    let grants = ["plugins::allow(", "plugins::set_allowed(", "crate::plugins::allow("];
    let mut files: Vec<String> = Vec::new();
    for (name, text) in crate::common::source::every_file() {
        let code = text.split("\n#[cfg(test)]\nmod ").next().unwrap_or("");

        if grants.iter().any(|g| code.contains(g)) && name != "plugins/permissions/tests.rs" {
            files.push(name);
        }
    }
    assert_eq!(files, ["settings/panels/permissions.rs", "store/asking.rs"], "{files:?}");

    let code = plugin_code();
    let mut on = 0;
    for (n, line) in code.lines().enumerate() {
        if (line.contains("set_allow(") || line.contains("allow_at(")) && !line.contains("fn ") && !line.contains("false)") && !line.trim_start().starts_with("//") {
            on += 1;
            assert!(line.contains("what, on") || line.contains("set_allow(&at, &[], w, true)") || line.contains("(&folder(id), &ports, what, on)") || line.contains("boxed::allow_at(at, PERMISSIONS, what, on)"), "line {}: {line}", n + 1);
        }
    }
    assert_eq!(on, 3, "allow() and set_allow() are two; the third is the screen reader's one-time first grant");

    let (asking, page) = (crate::common::source::path_of("store/asking.rs"), crate::common::source::path_of("settings/panels/permissions.rs"));
    let page = std::fs::read_to_string(page).unwrap();
    assert!(page.contains("if !crate::toolkit::window::persons_own(crate::common::person_only::SETTINGS_PERMISSIONS) {"));
    assert!(std::fs::read_to_string(asking).unwrap().contains("name: \"trindle-permissions\".into()"));
    assert!(crate::common::person_only::ASKED_BY_NAME.contains(&crate::common::person_only::PERMISSIONS));
    assert!(crate::common::person_only::ACTS.iter().any(|a| a.0 == crate::common::person_only::SETTINGS_PERMISSIONS));
}

#[test]
fn the_permission_window_says_both_in_plain_words_with_the_risk() {
    for (key, name, risk) in [
        (HOME_NETWORK, "Reach devices on your home network only", "your router included, but not to the internet"),
        (USB_DEVICES, "Use scanners and USB devices", "and no other USB device"),
    ] {
        let lines = crate::store::asking::lines_of(PERMISSIONS, &[key.to_string()], &[], &[], "");
        assert_eq!(lines.len(), 1, "{key}");
        assert_eq!(lines[0].name, name);
        assert!(lines[0].warning.as_deref().is_some_and(|w| w.contains(risk) && w.ends_with("Only allow this for a plugin you trust.")), "{key}: {:?}", lines[0].warning);
        assert!(!lines[0].on, "{key} starts off");
        assert!(lines[0].without.starts_with("It will not be able to"), "{key}");

        assert!(crate::common::access::off_line(&lines[0].without).starts_with("Off: it will not be able to"));
    }
}

#[test]
fn the_catalogue_marked_reader_is_allowed_sound_and_accessibility_once_and_the_person_can_turn_them_off() {
    let reader = crate::common::own_hands::SCREEN_READER;
    let root = std::env::temp_dir().join(format!("trindle-reader-preset-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join(reader)).unwrap();
    std::fs::create_dir_all(root.join("org.example.Weather")).unwrap();
    let sum = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    preset_reader(&root, reader);
    assert!(boxed::allowed_at(&root.join(reader), PERMISSIONS).is_empty(), "no catalogue mark, nothing allowed");
    crate::common::own_hands::mark(&root, reader, sum).unwrap();
    crate::common::own_hands::mark(&root, "org.example.Weather", sum).unwrap();
    preset_reader(&root, "org.example.Weather");
    assert!(boxed::allowed_at(&root.join("org.example.Weather"), PERMISSIONS).is_empty(), "another id is never preset");
    preset_reader(&root, reader);
    assert_eq!(boxed::allowed_at(&root.join(reader), PERMISSIONS), ["sound", "accessibility"]);
    set_allow(&root.join(reader), &[], ACCESSIBILITY, false).unwrap();
    preset_reader(&root, reader);
    assert_eq!(boxed::allowed_at(&root.join(reader), PERMISSIONS), ["sound"], "turned off stays off");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn only_the_screen_reader_may_ask_for_accessibility_and_it_is_off_until_allowed() {
    let ini = |id: &str, asks: &str| GOOD.replace("network;notifications", asks).replace("org.example.Weather", id);
    let reader = crate::common::own_hands::SCREEN_READER;
    let dir = folder_with("reader-asks", &ini(reader, "sound;accessibility"));
    let p = check(&dir).unwrap_or_else(|e| panic!("{e:?}"));
    let _ = std::fs::remove_dir_all(&dir);
    assert_eq!(p.asks, ["sound", "accessibility"]);
    assert!(PERMISSIONS.contains(&ACCESSIBILITY) && ACCESSIBILITY == "accessibility");
    for id in ["org.example.Weather", "org.trindle.screenreader", crate::common::own_hands::FLOW] {
        let dir = folder_with("other-asks", &ini(id, "sound;accessibility"));
        let e = check(&dir).unwrap_err().join(" ");
        let _ = std::fs::remove_dir_all(&dir);
        assert!(e.contains("only for Trindle's screen reader"), "{id}: {e}");
    }
    for bad in ["Accessibility", "accessibilty", "accessibility-bus", "accessible", "a11y", "read-screen", "accessibility=1", "access"] {
        let e = asking(bad).unwrap_err().join(" ");
        assert!(e.contains(&format!("Permission {bad} is not one there is")) && e.contains("accessibility"), "{bad}: {e}");
    }
    let e = asking("accessibility;accessibility").unwrap_err().join(" ");
    assert!(e.contains("is named twice"), "{e}");
    let root = std::env::temp_dir().join(format!("trindle-reader-allowed-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let v1 = folder_with("reader1", &ini(reader, "sound;accessibility"));
    put_in(&root, &v1).unwrap();
    let at = root.join(reader);
    assert!(boxed::allowed_at(&at, PERMISSIONS).is_empty(), "new, it is not allowed");
    set_allow(&at, &[], ACCESSIBILITY, true).unwrap();
    assert_eq!(boxed::allowed_at(&at, PERMISSIONS), ["accessibility"]);
    set_allow(&at, &[], ACCESSIBILITY, false).unwrap();
    assert!(boxed::allowed_at(&at, PERMISSIONS).is_empty());
    let _ = std::fs::remove_dir_all(&v1);
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_permission_window_says_accessibility_in_plain_words_with_the_risk() {
    let lines = crate::store::asking::lines_of(PERMISSIONS, &[ACCESSIBILITY.to_string()], &[], &[], "");
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].name, "Read what every window shows and say it aloud");
    assert!(lines[0].warning.as_deref().is_some_and(|w| w.contains("press buttons and type text in other windows") && w.ends_with("Only allow this for a plugin you trust.")));
    assert!(!lines[0].on && lines[0].without == "It will not be able to read your windows aloud.");
}
