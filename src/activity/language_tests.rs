//! Tests of the activity words in another language.

use crate::toolkit::lang::{Bundle, Catalogue, english, with_catalogue};
use crate::toolkit::t;

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

#[test]
fn the_processor_sentence_never_doubles_a_word_in_any_language() {
    use crate::common::shipped_languages::{in_language, tags};
    for tag in tags() {
        in_language(&tag, || {
            for share in [1.0, 0.01, 0.5] {
                let said = t!("activity-processor-sentence", threads = super::reading::threads_words(share, 16));
                let words: Vec<String> = said.split_whitespace().map(str::to_lowercase).collect();
                for len in 1..=3 {
                    let twice = words.windows(2 * len).find(|w| w[..len] == w[len..]);
                    assert!(twice.is_none(), "{tag}: {said:?} says {twice:?}");
                }
            }
        });
    }
}
