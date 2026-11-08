//! Tests for the welcome page's report choices.

use super::*;
use super::pages::reports_question;

#[test]
fn the_tour_asks_about_crash_reports_once_plainly_and_off_until_answered() {
    let mut w = Welcome::new(None);
    w.moving = false;
    assert!(!w.reports, "off until chosen");
    w.go_to(PAGES.iter().position(|p| *p == Page::Reports).unwrap());
    let words = w.view().said().join("\n");
    assert!(words.contains(reports_question()), "{words}");
    assert!(w.asked.iter().all(|a| !matches!(a, Ask::Reports(_))), "nothing is chosen by looking");
    w.update(Msg::Reports(true));
    assert!(w.reports);
    assert!(w.asked.contains(&Ask::Reports(true)));
    w.update(Msg::Reports(false));
    assert_eq!(w.asked.last(), Some(&Ask::Reports(false)));
}

#[test]
fn the_tour_follows_the_language_and_falls_back_word_by_word() {
    use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
    let french = "welcome-title = Bienvenue dans Trindle\nwelcome-next = Suivant\nwelcome-installed =\n    { $n ->\n        [one] { $names } est dans l'anneau.\n       *[other] { $names } sont dans l'anneau.\n    }\n";
    with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
        let mut w = Welcome::new(None);
        w.moving = false;
        assert_eq!(Page::Hello.title(), "Bienvenue dans Trindle");
        assert_eq!(w.main_words(), "Suivant");
        assert_eq!(Page::Ring.title(), "Open the ring", "a word French lacks is English");
        w.install = Install::Done(vec!["org.videolan.VLC".into()]);
        assert!(w.status().said().contains(&"VLC est dans l'anneau.".to_string()));
        w.install = Install::Done(vec!["org.videolan.VLC".into(), "org.gimp.GIMP".into()]);
        assert!(w.status().said().contains(&"VLC and GIMP sont dans l'anneau.".to_string()));
    });
}
