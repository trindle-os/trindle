//! Ticks only where something is confirmed, and no green state dot: a
//! confirming button carries the tick (CONFIRM_ICON, the one name for it),
//! a marked line a plain dot, and good news is plain.

pub const CONFIRM_ICON: &str = crate::common::ui::TICK_ICON;

pub const MARK_ICON: &str = "media-record-symbolic";

pub const SHOWN_SECONDS: u32 = 2;

#[cfg(test)]
mod tests {

    fn banned() -> Vec<String> {
        vec![
            ["object", "select"].join("-"),
            ["TICK", "ICON"].join("_"),
            ["TICK", "SECONDS"].join("_"),
            ["Tone", "Good"].join("::"),
            ["theme", "good()"].join("::"),
        ]
    }

    const ALLOWED: [(&str, &str); 2] = [
        ("common/app_theme.rs", "solid(theme::good())"),
        ("settings/tests/colours.rs", "let status = [theme::good()"),
    ];

    #[test]
    fn no_tick_glyph_or_green_state_dot_is_drawn() {
        let found: Vec<_> = super::super::save_is_a_word::all_lines().into_iter()
            .filter(|(file, line)| file != "common/no_tick.rs" && !file.starts_with("toolkit/") && banned().iter().any(|b| line.contains(b.as_str()))
                && !ALLOWED.iter().any(|(f, w)| file.ends_with(f) && line.contains(w)))
            .collect();
        assert!(found.is_empty(), "a tick or a green state dot is drawn: {found:#?}");
    }
}
