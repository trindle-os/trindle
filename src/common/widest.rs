//! How wide words need a place to be, worked out as the toolkit lays them
//! out, for a place of set width that must hold the longest words of a
//! language. A test holds each to the toolkit's own layout.

use crate::toolkit::text::{Fonts, Style};
use crate::toolkit::theme::sheet;

thread_local! {
    static FONTS: Fonts = Fonts::new();
}

fn width(words: &str, style: &Style) -> f32 {
    FONTS.with(|f| f.layout(words, style, None).widest().ceil())
}

fn plain() -> Style {
    let s = sheet();
    Style { weight: s.text.1, ..Style::new(s.text.0) }
}

pub fn words(words: &str) -> f32 {
    width(words, &plain())
}

pub fn action(words: &str) -> f32 {
    let s = sheet();
    let style = Style { weight: s.action_weight, ..Style::new(s.action_text) };
    2.0 * s.action_padding.round() + s.action_gap.round() + s.icon.round() + width(words, &style)
}

pub fn strip(options: &[String]) -> f32 {
    let s = sheet();
    options.iter().map(|o| words(o) + 2.0 * s.entry_padding.round()).sum::<f32>() + 2.0 * s.tabs_padding
}

const MENU_PADS: f32 = 6.0;

pub fn menu_line(words: &str, keys: &str) -> f32 {
    let s = sheet();
    let keys = if keys.is_empty() { 0.0 } else { width(keys, &Style::new(s.note.0)) + s.menu_padding };
    width(words, &Style::new(s.text.0)) + keys + s.menu_icon + MENU_PADS * s.menu_padding
}

pub fn headline_lines(words: &str, room: f32) -> usize {
    let (size, weight) = sheet().headline;
    FONTS.with(|f| f.layout(words, &Style { weight, ..Style::new(size) }, Some(room)).lines.len())
}

pub fn note_lines(words: &str, room: f32) -> usize {
    let (size, weight) = sheet().note;
    FONTS.with(|f| f.layout(words, &Style { weight, ..Style::new(size) }, Some(room)).lines.len())
}

pub type Stamp = ((String, u64), u64);

pub fn stamp() -> Stamp {
    (crate::common::live_language::words_in_use(), sheet_reading())
}

#[cfg(not(test))]
fn sheet_reading() -> u64 {
    crate::toolkit::theme::reading()
}

#[cfg(test)]
thread_local! {
    static REREAD: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
fn sheet_reading() -> u64 {
    REREAD.with(std::cell::Cell::get)
}

#[cfg(test)]
pub fn reread_here() {
    REREAD.with(|r| r.set(r.get() + 1));
}

pub type Measured = std::cell::RefCell<Option<(Stamp, f32)>>;

pub fn per_language(cache: &'static std::thread::LocalKey<Measured>, of: impl FnOnce() -> f32) -> f32 {
    let now = stamp();
    cache.with(|m| {
        if let Some((had, wide)) = &*m.borrow() {
            if *had == now {
                return *wide;
            }
        }
        let wide = of();
        *m.borrow_mut() = Some((now, wide));
        wide
    })
}

const STRIPS_KEPT: usize = 256;

thread_local! {
    static STRIPS: std::cell::RefCell<(Option<Stamp>, std::collections::HashMap<Vec<String>, f32>)> = Default::default();
}

pub fn strip_kept(options: &[String]) -> f32 {
    let now = stamp();
    STRIPS.with(|kept| {
        let mut kept = kept.borrow_mut();
        if kept.0.as_ref() != Some(&now) {
            *kept = (Some(now), std::collections::HashMap::new());
        }
        if let Some(wide) = kept.1.get(options) {
            return *wide;
        }
        if kept.1.len() >= STRIPS_KEPT {
            kept.1.clear();
        }
        let wide = strip(options);
        kept.1.insert(options.to_vec(), wide);
        wide
    })
}

#[cfg(test)]
pub fn laid<M: Clone + 'static>(items: Vec<crate::toolkit::widget::Node<M>>) -> f32 {
    use crate::toolkit::widget::{column, row, space};
    let keyed: Vec<_> = items.into_iter().enumerate().map(|(i, n)| row(vec![n.key(format!("laid:{i}")), space()])).collect();
    let mut ui = crate::toolkit::ui::Ui::new(column(keyed), crate::toolkit::theme::Theme::chosen());
    ui.lay_at(4000, 4000);
    ui.boxes().iter().filter(|b| b.0.starts_with("laid:")).map(|b| b.1.w.ceil()).fold(0.0, f32::max)
}

#[cfg(test)]
mod tests {
    use crate::toolkit::widget::{choice, label, main_action};

    #[test]
    fn the_reckonings_are_the_toolkits_own_in_every_language() {
        use crate::common::shipped_languages::{in_language, tags};
        for tag in tags() {
            in_language(&tag, || {
                for key in ["store-app-release", "store-deb-reinstall", "settings-keyboard-every-device", "store-nav-installed"] {
                    let w = crate::toolkit::lang::t(key);
                    let a = super::laid::<()>(vec![main_action("view-pin-symbolic", w)]);
                    assert!((super::action(w) - a).abs() <= 1.0, "{tag}: action {w:?} {} against {a}", super::action(w));
                    let l = super::laid::<()>(vec![label(w)]);
                    assert!((super::words(w) - l).abs() <= 1.0, "{tag}: words {w:?} {} against {l}", super::words(w));
                }
                let options: Vec<String> = ["settings-display-place-right", "settings-display-place-left", "settings-display-place-above"]
                    .map(|k| crate::toolkit::lang::t(k).to_string()).to_vec();
                let c = super::laid::<()>(vec![choice(options.clone(), 0, "", |_| ())]);
                assert!((super::strip(&options) - c).abs() <= 1.0, "{tag}: strip {options:?} {} against {c}", super::strip(&options));
            });
        }
    }

    #[test]
    fn a_new_sheet_or_new_words_measure_again() {
        thread_local! {
            static KEPT: super::Measured = const { std::cell::RefCell::new(None) };
        }
        let made = std::cell::Cell::new(0);
        let measure = || super::per_language(&KEPT, || { made.set(made.get() + 1); 1.0 });
        let strips = || super::STRIPS.with(|k| k.borrow().1.len());
        let options = vec!["Left".to_string(), "Right".to_string()];
        measure();
        measure();
        super::strip_kept(&options);
        assert_eq!((made.get(), strips()), (1, 1), "kept while nothing changes");
        super::reread_here();
        measure();
        super::strip_kept(&["Up".to_string()]);
        assert_eq!((made.get(), strips()), (2, 1), "a new style sheet is measured again, and the old strips let go");
        crate::common::live_language::switch(vec!["en".to_string()]);
        measure();
        super::strip_kept(&["Down".to_string()]);
        assert_eq!((made.get(), strips()), (3, 1), "new words are measured again, and the old strips let go");
    }
}
