//! Nothing a stranger's file names is followed. A picture, a document or
//! an archive a person was sent can name other things: another picture,
//! a file on this computer, a device, an address. Whatever reads it reads
//! its own bytes and no more. One that followed what it was told would
//! read whatever the stranger chose: a device with no end, a pipe with
//! no writer, a private file drawn into a thumbnail.

use super::no_writing_in_place::{all_code, code};

struct Reader {
    words: Vec<String>,
    only: &'static [&'static str],

    under: &'static str,
    rule: &'static str,
}

fn readers() -> Vec<Reader> {
    let join = |parts: &[&str]| parts.concat();
    vec![
        Reader {
            words: vec![join(&["usvg::", "Options"]), join(&["usvg::", "Tree"]), join(&["Tree::", "from_data"]), join(&["Tree::", "from_str"]), join(&["ImageHref", "Resolver"]),
                        join(&["use resvg::", "usvg"])],
            only: &["toolkit/picture.rs"],
            under: "",
            rule: "an SVG is read by toolkit::picture::svg_tree, which follows nothing it names",
        },
        Reader {
            words: vec![join(&["picture::", "svg("]), join(&["svg_", "tree("]), join(&["resvg::", "render("])],
            only: &["files/walled.rs"],
            under: "files/",
            rule: "in Folders an SVG is drawn only in the box, by files::walled::svg",
        },
        Reader {
            words: vec![join(&["(\"ff", "mpeg\")"]), join(&["(\"ff", "probe\")"]), join(&["(\"pdfto", "ppm\")"])],
            only: &["files/walled.rs"],
            under: "",
            rule: "a program that reads a stranger's film or PDF is started only in the box, by files::walled",
        },
        Reader {
            words: vec![join(&["libarchive::", "Reader"]), join(&["Reader::", "open("]), join(&["archive_read_", "open"])],
            only: &["files/unarchive.rs", "files/libarchive.rs"],
            under: "files/",
            rule: "libarchive reads a stranger's archive only in the boxed child, files::unarchive",
        },
    ]
}

fn breaks(file: &str, text: &str, reader: &Reader) -> Vec<String> {
    if !file.starts_with(reader.under) || reader.only.contains(&file) {
        return Vec::new();
    }
    code(text).iter().enumerate()
        .filter(|(_, l)| !l.trim_start().starts_with("//") && reader.words.iter().any(|w| l.contains(w.as_str())))
        .map(|(n, l)| format!("src/{file}:{}: {} ({})", n + 1, l.trim(), reader.rule))
        .collect()
}

#[test]
fn a_reader_that_could_follow_a_name_is_seen_wherever_it_is_started() {
    let all = readers();
    let [svg, drawn, programs, archives] = &all[..] else { panic!("four kinds of reader") };
    let (options, render, film, c) = (&svg.words[0], &drawn.words[2], &programs.words[0], &archives.words[0]);
    let text = format!("let opt = resvg::{options} {{ ..Default::default() }};\n// resvg::{options}\n{render}&tree, t, &mut px);\nlet f = program{film};\nuse super::{c};\n\
                        #[cfg(test)]\nmod tests {{\n    let opt = resvg::{options}::default();\n}}\n");
    assert_eq!(breaks("files/viewer.rs", &text, svg).len(), 1);
    assert_eq!(breaks("screenshot/portal_ask.rs", &text, svg).len(), 1, "the rule for an SVG's options holds everywhere");
    assert!(breaks("toolkit/picture.rs", &text, svg).is_empty());
    assert_eq!(breaks("files/viewer.rs", &text, drawn).len(), 1);
    assert!(breaks("toolkit/icon.rs", &text, drawn).is_empty(), "outside Folders an icon is drawn in its own program, from the one reader");
    assert!(breaks("files/walled.rs", &text, drawn).is_empty());
    assert_eq!(breaks("files/thumbs.rs", &text, programs).len(), 1);
    assert_eq!(breaks("files/archive.rs", &text, archives).len(), 1);
    assert!(breaks("files/unarchive.rs", &text, archives).is_empty());
}

#[test]
fn an_svg_is_read_in_one_place_that_follows_nothing_it_names() {
    let found: Vec<String> = all_code().iter().flat_map(|(file, text)| readers().iter().flat_map(|r| breaks(file, text, r)).collect::<Vec<_>>()).collect();
    assert!(found.is_empty(), "a reader that could follow what a stranger's file names is started outside its one place:\n{}", found.join("\n"));

    let picture = all_code().into_iter().find(|(file, _)| file == "toolkit/picture.rs").expect("toolkit/picture.rs").1;
    let lines = code(&picture);
    assert!(lines.iter().any(|l| l.contains(&["resolve_string: Box::new(|_, _| ", "None)"].concat())), "the one reader of an SVG follows a name again");
    assert!(lines.iter().any(|l| l.contains(&["resources_dir: ", "None"].concat())), "the one reader of an SVG is given a folder to look names up in");
}
