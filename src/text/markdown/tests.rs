use super::html::{safe_url, to_html};
use super::layout::{Lay, Piece};
use super::marks::inline;
use super::*;
use crate::toolkit::text::Fonts;

fn text(s: &str) -> Span {
    Span::Text(s.into())
}

fn md(s: &str) -> Vec<Block> {
    parse(s)
}

#[test]
fn headings_paragraphs_and_rules() {
    let b = md("# Title #\n\nSome *words*\nrun on.\n\nSub\n---\n\n***\n");
    assert_eq!(
        b,
        vec![
            Block::Heading(1, vec![text("Title")]),
            Block::Para(vec![text("Some "), Span::Italic(vec![text("words")]), text(" run on.")]),
            Block::Heading(2, vec![text("Sub")]),
            Block::Rule,
        ]
    );
}

#[test]
fn inline_looks() {
    let refs = Refs::new();
    assert_eq!(
        inline("**bold** and _it_ and ~~gone~~ and `a * b`", &refs),
        vec![
            Span::Bold(vec![text("bold")]),
            text(" and "),
            Span::Italic(vec![text("it")]),
            text(" and "),
            Span::Strike(vec![text("gone")]),
            text(" and "),
            Span::Code("a * b".into()),
        ]
    );
    assert_eq!(inline("2 * 3 * 4 and snake_case_name", &refs), vec![text("2 * 3 * 4 and snake_case_name")]);
    assert_eq!(inline("\\*not\\* &amp; line  \nnext", &refs), vec![text("*not* & line"), Span::Break, text("next")]);
    assert_eq!(
        inline("[Trindle](https://trindle.org \"home\") ![a cat](cat.png) <https://example.com>", &refs),
        vec![
            Span::Link(vec![text("Trindle")], "https://trindle.org".into()),
            text(" "),
            Span::Image("a cat".into(), "cat.png".into()),
            text(" "),
            Span::Link(vec![text("https://example.com")], "https://example.com".into()),
        ]
    );
    assert_eq!(
        inline("See https://example.com/a_(b).", &refs),
        vec![text("See "), Span::Link(vec![text("https://example.com/a_(b)")], "https://example.com/a_(b)".into()), text(".")]
    );
}

#[test]
fn named_links() {
    let b = md("Read [the guide][g] or [g].\n\n[g]: https://example.com/guide\n");
    let link = Span::Link(vec![text("the guide")], "https://example.com/guide".into());
    let short = Span::Link(vec![text("g")], "https://example.com/guide".into());
    assert_eq!(b, vec![Block::Para(vec![text("Read "), link, text(" or "), short, text(".")])]);
}

#[test]
fn deep_nesting_ends_in_plain_text_not_a_crash() {
    let quotes = format!("{} deep", ">".repeat(5000));
    assert!(!parse(&quotes).is_empty());
    let emphasis = format!("{}x{}", "*_".repeat(3000), "_*".repeat(3000));
    let _ = parse(&emphasis);
}

#[test]
fn lists_quotes_and_code() {
    let b = md("- one\n- two\n  - deeper\n- [x] done\n\n3. three\n4. four\n\n> quoted\nlazily\n\n```rust\nfn main() {}\n```\n\n    indented\n");
    let item = |s: &str| Item { task: None, blocks: vec![Block::Para(vec![text(s)])] };
    assert_eq!(
        b,
        vec![
            Block::List(
                None,
                vec![
                    item("one"),
                    Item { task: None, blocks: vec![Block::Para(vec![text("two")]), Block::List(None, vec![item("deeper")])] },
                    Item { task: Some(true), blocks: vec![Block::Para(vec![text("done")])] },
                ]
            ),
            Block::List(Some(3), vec![item("three"), item("four")]),
            Block::Quote(vec![Block::Para(vec![text("quoted lazily")])]),
            Block::Code("rust".into(), "fn main() {}".into()),
            Block::Code(String::new(), "indented".into()),
        ]
    );
}

#[test]
fn tables() {
    let b = md("| Name | Size |\n|:-----|-----:|\n| a | 1 |\n| b \\| c |\n");
    assert_eq!(
        b,
        vec![Block::Table(
            vec![Align::Left, Align::Right],
            vec![vec![text("Name")], vec![text("Size")]],
            vec![vec![vec![text("a")], vec![text("1")]], vec![vec![text("b | c")], vec![]]],
        )]
    );
}

#[test]
fn markdown_to_html() {
    let html = to_html(&md("# Hi & bye\n\nA <script>alert(1)</script> [bad](javascript:alert(1)) and [good](https://example.com).\n\n- [ ] task\n"), "Hi & bye");
    assert!(html.starts_with("<!DOCTYPE html>"));
    assert!(html.contains("<title>Hi &amp; bye</title>"));
    assert!(html.contains("<h1>Hi &amp; bye</h1>"));
    assert!(html.contains("A &lt;script&gt;alert(1)&lt;/script&gt; bad and <a href=\"https://example.com\">good</a>."));
    assert!(!html.contains("javascript:"));
    assert!(html.contains("<li class=\"task\"><input type=\"checkbox\" disabled> task</li>"));
    let code = to_html(&md("```\n<b>&\n```"), "x");
    assert!(code.contains("<pre><code>&lt;b&gt;&amp;\n</code></pre>"));
}

#[test]
fn unsafe_addresses() {
    assert!(safe_url("https://example.com") && safe_url("notes/other.md") && safe_url("#part") && safe_url("mailto:john@example.com"));
    assert!(!safe_url("javascript:alert(1)") && !safe_url(" JavaScript:x") && !safe_url("java\tscript:x") && !safe_url("data:text/html,x"));
}

#[test]
fn the_finished_look_wraps_and_keeps_links() {
    let fonts = Fonts::new();
    let blocks = md("# Heading\n\nA paragraph with [a link](https://example.com) that is long enough to wrap over more than one line here.\n\n| a | b |\n|---|---|\n| 1 | 2 |\n");
    let laid = Lay::new(&fonts, 16.0, None).lay(&blocks, 300.0);
    assert_eq!(laid.links, vec!["https://example.com".to_string()]);
    let rows_with_words = laid.rows.iter().filter(|r| r.pieces.iter().any(|p| matches!(p, Piece::Words { .. }))).count();
    assert!(rows_with_words >= 4, "the paragraph wraps: {rows_with_words} rows");
    for row in &laid.rows {
        for p in &row.pieces {
            assert!(p.right() <= 300.5, "nothing runs past the edge");
        }
    }
    assert!((laid.height - laid.rows.iter().map(|r| r.height).sum::<f32>()).abs() < 0.01);
}
