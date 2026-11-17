//! The one list of characters that cannot be seen but change what words say.

pub fn unseen(c: char) -> bool {
    (c.is_control() && c != '\n' && c != '\t')
        || matches!(c,
            '\u{00AD}' | '\u{061C}' | '\u{180E}' | '\u{200B}'..='\u{200F}' | '\u{2028}'..='\u{202E}'
            | '\u{2060}'..='\u{206F}' | '\u{FEFF}' | '\u{FFF9}'..='\u{FFFB}' | '\u{E0000}'..='\u{E007F}'
            | '\u{E0100}'..='\u{E01EF}')
}

#[cfg(test)]
mod tests {
    use super::unseen;

    #[test]
    fn every_hidden_kind_is_found_and_plain_text_is_not() {
        for c in ['\u{00AD}', '\u{061C}', '\u{180E}', '\u{200B}', '\u{202E}', '\u{2066}', '\u{206A}', '\u{FEFF}', '\u{E0041}', '\u{E0100}', '\0'] {
            assert!(unseen(c), "{c:?}");
        }
        for c in ['a', ' ', '\n', '\t', 'é', '\u{2014}'] {
            assert!(!unseen(c), "{c:?}");
        }
    }
}
