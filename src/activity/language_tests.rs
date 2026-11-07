//! Tests of the activity words in another language.

use crate::toolkit::lang::{Bundle, Catalogue, english, with_catalogue};

use super::reading::{busy_words, running_words};

#[test]
fn the_activity_words_follow_the_language_and_fall_back_word_by_word() {
    let french = "activity-resting = au repos\nactivity-under-a-minute = Moins d'une minute\n";
    with_catalogue(Catalogue::new(Bundle::from_text("fr", french), english()), || {
        assert_eq!(busy_words(0.01), "au repos");
        assert_eq!(running_words(30), "Moins d'une minute");
        assert_eq!(running_words(7300), "2 hours");
    });
}
