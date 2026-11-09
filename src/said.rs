//! Printing that never ends the program: the print macros drop words that cannot be written (the standard ones panic on a closed pipe). `out` and `err` are called by the macros in the programs that bring this file in, so the toolkit's own build never uses them.

pub fn put(out: &mut dyn std::io::Write, words: std::fmt::Arguments, line: bool) {
    let _ = if line { writeln!(out, "{words}") } else { out.write_fmt(words) };
}

#[cfg(not(test))]
#[allow(dead_code, reason = "called by the macros in the programs that bring this file in, not by the toolkit")]
pub fn out(words: std::fmt::Arguments, line: bool) {
    put(&mut std::io::stdout().lock(), words, line);
}

#[cfg(not(test))]
#[allow(dead_code, reason = "called by the macros in the programs that bring this file in, not by the toolkit")]
pub fn err(words: std::fmt::Arguments, line: bool) {
    put(&mut std::io::stderr().lock(), words, line);
}

#[cfg(not(test))]
#[allow(unused_macros)]
macro_rules! println {
    () => { $crate::said::out(format_args!(""), true) };
    ($($words:tt)*) => { $crate::said::out(format_args!($($words)*), true) };
}

#[cfg(not(test))]
#[allow(unused_macros)]
macro_rules! print {
    ($($words:tt)*) => { $crate::said::out(format_args!($($words)*), false) };
}

#[cfg(not(test))]
#[allow(unused_macros)]
macro_rules! eprintln {
    () => { $crate::said::err(format_args!(""), true) };
    ($($words:tt)*) => { $crate::said::err(format_args!($($words)*), true) };
}

#[cfg(not(test))]
#[allow(unused_macros)]
macro_rules! eprint {
    ($($words:tt)*) => { $crate::said::err(format_args!($($words)*), false) };
}

#[cfg(test)]
mod said_tests {
    use std::io::{self, Write};

    struct Shut(usize);

    impl Write for Shut {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            self.0 += 1;
            Err(io::Error::from(io::ErrorKind::BrokenPipe))
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::from(io::ErrorKind::BrokenPipe))
        }
    }

    #[test]
    fn words_that_cannot_be_written_are_let_go() {
        let mut shut = Shut(0);
        super::put(&mut shut, format_args!("{} and {}", "one", 2), true);
        super::put(&mut shut, format_args!(""), true);
        assert!(shut.0 >= 1, "nothing was tried");
        let mut kept = Vec::new();
        super::put(&mut kept, format_args!("{} and {}", "one", 2), true);
        super::put(&mut kept, format_args!("more"), false);
        super::put(&mut kept, format_args!(""), true);
        assert_eq!(kept, b"one and 2\nmore\n");
    }

    fn trindle() -> Option<std::path::PathBuf> {
        let root = std::path::PathBuf::from(std::env::var_os("TRINDLE_SOURCE_ROOT").filter(|v| !v.is_empty())?);
        root.join("src/main.rs").is_file().then_some(root)
    }

    #[test]
    fn every_program_prints_only_this_way() {
        if env!("CARGO_PKG_NAME") != "trindle-toolkit" {
            return;
        }
        let toolkit = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let trindle = trindle();
        if trindle.is_none() {
            eprintln!("NOTE: Trindle's programs were not found (set TRINDLE_SOURCE_ROOT); only the toolkit's lib.rs is checked");
        }
        let read = |at: std::path::PathBuf| std::fs::read_to_string(&at).unwrap_or_else(|e| panic!("{}: {e}", at.display()));
        let mut programs = vec![(toolkit.join("src/lib.rs"), "lib.rs".to_string(), "said.rs".to_string())];
        if let Some(top) = &trindle {
            let to_toolkit = |from: &str| {
                let back = std::path::Path::new(from).parent().map_or(0, |p| p.components().count());
                format!("{}trindle-toolkit/src/said.rs", "../".repeat(back + 1))
            };
            for name in ["src/main.rs", "src/compose/main.rs", "src/unlock/main.rs"] {
                programs.push((top.join(name), name.to_string(), to_toolkit(name)));
            }
        }
        for (file, root, path) in programs {
            let text = read(file);
            let lines: Vec<&str> = text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with("//")).collect();
            let first = lines.iter().position(|l| l.starts_with("mod ") || l.starts_with("pub mod ") || l.starts_with("use "))
                .unwrap_or_else(|| panic!("{root} brings nothing in"));
            assert_eq!(lines[first], "mod said;", "{root}: the first thing brought in is not the printing that cannot panic");
            let before = &lines[..first];
            let placed = path == "said.rs" || before.contains(&format!("#[path = \"{path}\"]").as_str());
            assert!(before.contains(&"#[macro_use]") && placed, "{root}: {before:?}");
        }
        let own = read(toolkit.join("src/said.rs"));
        let code = own.split_once("#[cfg(test)]\nmod said_tests").unwrap().0;
        for name in ["println", "print", "eprintln", "eprint"] {
            assert!(code.contains(&format!("#[cfg(not(test))]\n#[allow(unused_macros)]\nmacro_rules! {name} {{")), "{name} is not replaced");
        }
        let full = [["std::", "println!"].concat(), ["std::", "print!"].concat(), ["std::", "eprintln!"].concat(), ["std::", "eprint!"].concat()];
        let mut found = Vec::new();
        let mut look = vec![toolkit.join("src")];
        look.extend(trindle.map(|t| t.join("src")));
        while let Some(dir) = look.pop() {
            for entry in std::fs::read_dir(&dir).unwrap().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    look.push(path);
                } else if path.extension().is_some_and(|e| e == "rs") {
                    let text = std::fs::read_to_string(&path).unwrap_or_default();
                    if full.iter().any(|f| text.contains(f.as_str())) {
                        found.push(path);
                    }
                }
            }
        }
        assert!(found.is_empty(), "the standard printing is asked for by its full name in {found:?}");
    }
}
