//! Films are decoded in one process and nowhere else.

use super::no_writing_in_place::{all_code, code};
use super::source;

const HOME: &str = "session/decoding.rs";
const START: &str = "main.rs";

fn library() -> String {
    ["vp", "x"].concat()
}

fn reaching_in() -> String {
    ["decod", "ing::"].concat()
}

fn making() -> String {
    ["Decoder::", "new("].concat()
}

fn said(text: &str) -> Vec<String> {
    let own = code(text).join("\n");
    source::code(Box::leak(own.into_boxed_str())).lines().map(str::to_string).collect()
}

fn faults(files: &[(String, String)]) -> Vec<String> {
    let (library, reaching, making) = (library(), reaching_in(), making());
    let way_in = [&reaching[..], "main("].concat();
    let mut out = Vec::new();
    let mut ways_in = 0;
    for (file, text) in files {
        let lines = said(text);
        if file == HOME {
            let made: Vec<usize> = (0..lines.len()).filter(|&i| lines[i].contains(&making)).collect();
            let process = lines.iter().position(|l| l.starts_with("fn child("));
            let ends = process.and_then(|from| lines[from..].iter().position(|l| l.starts_with('}')).map(|n| from + n));
            let inside = matches!((process, ends, &made[..]), (Some(from), Some(to), [at]) if from < *at && *at < to);
            if !inside {
                out.push(format!("{file}: the decoder is made {} times, and must be made once, inside the process", made.len()));
            }
            for line in lines.iter().filter(|l| l.trim_start().starts_with("pub") && !l.starts_with("pub fn main(")) {
                out.push(format!("{file}: a way in besides main: {}", line.trim()));
            }
            continue;
        }
        for line in &lines {
            if line.to_lowercase().contains(&library) {
                out.push(format!("{file}: names the films' decoder: {}", line.trim()));
            }
            for (at, _) in line.match_indices(&reaching) {
                if file == START && line[at..].starts_with(&way_in) {
                    ways_in += 1;
                } else {
                    out.push(format!("{file}: reaches into the films' decoder: {}", line.trim()));
                }
            }
        }
    }
    if let Some((_, start)) = files.iter().find(|(file, _)| file == START) {
        let lines = said(start);
        let guarded = lines.windows(2).any(|pair| pair[0].contains("moving::DECODING") && pair[0].trim_start().starts_with("if ") && pair[1].contains(&way_in));
        if ways_in != 1 || !guarded {
            out.push(format!("{START}: the decoder is started {ways_in} times, and must be started once, for its own part's name alone"));
        }
    }
    out
}

#[test]
fn films_are_decoded_in_the_decoding_part_and_nowhere_else() {
    let files = all_code();
    assert!(files.iter().any(|(file, _)| file == HOME) && files.iter().any(|(file, _)| file == START), "the source was not found");
    let found = faults(&files);
    assert!(found.is_empty(), "films must be decoded only by the part of Trindle that is their decoder, in its own process:\n{}", found.join("\n"));
}

#[test]
fn each_way_of_decoding_a_film_somewhere_else_is_caught() {
    let (lib, reach, make, upper) = (library(), reaching_in(), making(), library().to_uppercase());
    let (lib, reach, make, upper) = (lib.as_str(), reach.as_str(), make.as_str(), upper.as_str());
    let file = |name: &str, text: &str| (name.to_string(), text.to_string());
    let start = file(START, &["fn main() {\n    if part == session::moving::DECODING {\n        return session::", reach, "main(args);\n    }\n}\n"].concat());
    let home = file(HOME, &["pub fn main() {\n    child()\n}\n\nfn child() {\n    let d = ", make, "codec);\n}\n"].concat());
    assert_eq!(faults(&[start.clone(), home.clone()]), Vec::<String>::new());
    let talk = file("session/moving.rs", &["// ", lib, " is not here\nfn f() {}\n#[cfg(test)]\nmod tests {\n    fn t() { super::super::", reach, "tests::key(); }\n}\n"].concat());
    assert_eq!(faults(&[start.clone(), home.clone(), talk]), Vec::<String>::new());
    let caught = |files: &[(String, String)]| faults(files).len();
    let opened = file("session/backdrop.rs", &["fn f() { dlopen(c\"lib", lib, ".so.9\"); }\n"].concat());
    assert_eq!(caught(&[start.clone(), home.clone(), opened]), 1);
    let called = file("settings/panels/wallpaper.rs", &["fn f() { ", upper, "_decode(); }\n"].concat());
    assert_eq!(caught(&[start.clone(), home.clone(), called]), 1);
    let reached = file("settings/panels/wallpaper.rs", &["fn f() { crate::session::", reach, "main(vec![]); }\n"].concat());
    assert_eq!(caught(&[start.clone(), home.clone(), reached]), 1);
    let always = file(START, &["fn main() {\n    session::", reach, "main(args);\n}\n"].concat());
    assert_eq!(caught(&[always, home.clone()]), 1);
    let twice = file(START, &["fn main() {\n    if part == session::moving::DECODING {\n        return session::", reach, "main(args);\n    }\n    session::",
                              reach, "main(args);\n}\n"].concat());
    assert_eq!(caught(&[twice, home.clone()]), 1);
    let loose = file(HOME, &["pub fn main() {\n    child()\n}\n\nfn child() {\n}\n\nfn other() {\n    let d = ", make, "codec);\n}\n"].concat());
    assert_eq!(caught(&[start.clone(), loose]), 1);
    let door = file(HOME, &["pub fn main() {\n    child()\n}\n\npub(crate) fn child() {\n    let d = ", make, "codec);\n}\n"].concat());
    assert!(caught(&[start, door]) >= 1);
}
