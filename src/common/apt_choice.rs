//! The names a Debian package may have.

const LONGEST: usize = 128;

pub fn valid_name(name: &str) -> bool {
    (2..=LONGEST).contains(&name.len()) && name.starts_with(|c: char| c.is_ascii_lowercase() || c.is_ascii_digit())
        && name.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"+-.".contains(&b))

        && !name.ends_with('-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_debians_own_names_are_taken() {
        for good in ["claude-code", "libc6", "g++", "libstdc++6", "0ad", "python3.13"] {
            assert!(valid_name(good), "{good}");
        }
        for bad in ["", "a", "-rf", "--purge", "Bash", "a b", "a;rm", "../x", "a/b", "trindle=1.0", "x\n", &"a".repeat(129), "ünï", "bash-", "coreutils-"] {
            assert!(!valid_name(bad), "{bad:?}");
        }
    }
}
