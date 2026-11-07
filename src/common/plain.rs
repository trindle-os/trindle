//! One plain line from text a program chose.

pub fn line(text: &str) -> String {
    text.chars().map(|c| if hostile(c) { ' ' } else { c }).collect()
}

fn hostile(c: char) -> bool {
    c.is_control() || matches!(c, '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

#[cfg(test)]
mod tests {
    use super::line;

    #[test]
    fn controls_and_direction_overrides_become_spaces() {
        assert_eq!(line("one\ntwo\u{7}\u{202e}x\u{2066}y"), "one two  x y");
        assert_eq!(line("café"), "café");
    }
}
