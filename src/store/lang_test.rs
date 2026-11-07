//! Apps' words follow the person's language, one word at a time.

use super::{catalogue, pages, refusal};
use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};

#[test]
fn sizes_counts_and_ratings_follow_the_language() {
    assert_eq!((super::github::size_text(4_200_000), super::github::size_text(1_500_000_000)), ("4 MB".to_string(), "1.5 GB".to_string()));
    let made_up = "kit-number-decimal = ,\nkit-number-group = _\nkit-size-mb = MO { $size }\nkit-size-gb = GO { $size }\n\
                   store-app-rating-from = { $stars }  de { $n ->\n        [one] { $count } notes\n       *[other] { $count } notes\n    }\n";
    with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
        assert_eq!(super::github::size_text(4_200_000), "MO 4");
        assert_eq!(super::github::size_text(1_500_000_000), "GO 1,5");
        assert_eq!(crate::common::thousands(1_234_567), "1_234_567");
        assert_eq!(crate::toolkit::t!("store-app-rating-from", stars = "***", n = 1234, count = crate::common::thousands(1234)), "***  de 1_234 notes");
        assert_eq!(crate::toolkit::lang::format::decimal(4.5, 1), "4,5");
    });
}

#[test]
fn apps_follows_the_language_and_falls_back_word_by_word() {
    let french = "store-nav-discover = Découvrir\n\
                  store-cat-office = Bureautique\n\
                  store-installed-part-apps =\n    { $n ->\n        [one] 1 appli\n       *[other] { $n } applis\n    }\n\
                  store-because-in-the-way = quelque chose est déjà là où une partie de la sauvegarde doit aller\n\
                  store-refusal-nothing = Rien n'a été remis, car { $why }.\n";
    with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
        let open = pages::grid::open_without();
        assert_eq!((open[0], open[1]), ("Installed", "Updates"), "words French lacks are English");
        assert_eq!(pages::links::back_name(&super::View::Home), "Découvrir");
        let office = catalogue::categories().iter().find(|c| c.0 == "Office").map(|c| c.1);
        assert_eq!(office, Some("Bureautique"));
        assert_eq!(catalogue::categories()[0].1, "Sound and video");
        assert_eq!(crate::toolkit::t!("store-installed-part-apps", n = 1), "1 appli");
        assert_eq!(crate::toolkit::t!("store-installed-part-apps", n = 3), "3 applis");
        assert_eq!(refusal::in_the_way_words(), "Rien n'a été remis, car quelque chose est déjà là où une partie de la sauvegarde doit aller.");
    });
    assert_eq!(pages::links::back_name(&super::View::Home), "Discover", "and English again outside");
}
