//! Tests for the Firewall and blocking page's firewall parts.

use super::*;
use crate::common::sharing::{Problem, Shared};
use crate::settings::ui::Kit;

#[test]
fn the_sharing_card_lists_each_share_with_a_stop_button() {
    use crate::settings::ui::Do;
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: Do| {});
    let kit = Kit::new(events.clone(), asks, None, None);
    let mut n = Blocking::default();
    let said = |n: &Blocking| n.sharing_card(&kit).said();
    assert!(said(&n).iter().any(|w| w.starts_with("Looking")));
    n.shared = Some(Err(Problem::NoSamba));
    assert!(said(&n).contains(&"No folders are shared".to_string()));
    let home = crate::common::files::home();
    n.shared = Some(Ok(vec![
        Shared { name: "photos".into(), path: home.join("Photos"), write: false, guests: false },
        Shared { name: "music".into(), path: "/srv/music".into(), write: true, guests: true },
    ]));
    let words = said(&n);
    assert_eq!(words[0], "Sharing");
    for w in ["photos", "~/Photos: open with your password", "button: Stop sharing photos",
              "music", "/srv/music: anyone nearby can change", "button: Stop sharing music"] {
        assert!(words.contains(&w.to_string()), "{w}: {words:?}");
    }

    assert!(words.contains(&"Not reachable yet".to_string()));
    n.sharing = true;
    assert!(!said(&n).contains(&"Not reachable yet".to_string()));
    assert!(said(&n).iter().all(|w| !w.contains('\u{2014}')));
}

#[test]
fn a_typed_port_opens_tcp_udp_or_both_and_nonsense_is_refused() {
    use super::ports::to_open;
    use crate::common::firewall::parse_own;
    assert_eq!(to_open("37666", 0), Ok(vec![parse_own("tcp:37666").unwrap()]));
    assert_eq!(to_open("37666", 1), Ok(vec![parse_own("udp:37666").unwrap()]));
    assert_eq!(to_open(" 37666", 2), Ok(vec![parse_own("tcp:37666").unwrap(), parse_own("udp:37666").unwrap()]));
    for bad in ["", "abc", "0", "65536", "1.5"] {
        assert!(to_open(bad, 0).is_err(), "{bad:?}");
    }
}

#[test]
#[ignore = "makes the Welcome's picture of the firewall"]
fn shoot_the_firewall_for_the_welcome() {
    use crate::settings::ui::Do;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;
    use crate::toolkit::widget::column;
    let out = std::path::PathBuf::from(std::env::var("TRINDLE_SHOTS").unwrap_or_else(|_| "target/shots".into()));
    std::fs::create_dir_all(&out).unwrap();
    let events = crate::toolkit::event::Loop::new();
    let asks = events.post(|_: Do| {});
    let kit = Kit::new(events.clone(), asks, None, None);
    let b = Blocking::made_up();
    let tree = column(vec![b.firewall_card(&kit), b.ports_card(&kit)]).gap(12.0).padding(12.0);
    let mut ui = Ui::new(tree, Theme::new(false, "#fbc711"));
    let w = 560;

    let h = 460;
    let mut pix = tiny_skia::Pixmap::new(w * 2, h * 2).unwrap();
    ui.paint(&mut pix.as_mut(), 2.0);
    pix.save_png(out.join("firewall.png")).unwrap();
}
