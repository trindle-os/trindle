//! The text written out as a web page, with HTML in it shown as text and unsafe links dropped.

use super::{Align, Block, Item};
use super::Span;

const CSS: &str = "\
:root { color-scheme: light dark; --bg: #ffffff; --fg: #1d1d1f; --muted: #5f6368; --link: #1a56b0; --code: #f2f3f5; --edge: #d0d4d9; }
@media (prefers-color-scheme: dark) { :root { --bg: #17181a; --fg: #e8e8ea; --muted: #a0a4a8; --link: #8ab4f8; --code: #26282c; --edge: #3a3d42; } }
body { margin: 0; background: var(--bg); color: var(--fg); font: 16px/1.6 system-ui, sans-serif; }
main { padding: 2rem clamp(1rem, 4vw, 3rem); }
h1, h2 { border-bottom: 1px solid var(--edge); padding-bottom: 0.2em; }
a { color: var(--link); }
code, pre { font-family: ui-monospace, monospace; background: var(--code); border-radius: 4px; }
code { padding: 0.1em 0.3em; }
pre { padding: 0.8em 1em; overflow-x: auto; }
pre code { padding: 0; background: none; }
blockquote { margin: 0 0 1em; padding: 0 1em; border-left: 4px solid var(--edge); color: var(--muted); }
table { border-collapse: collapse; width: 100%; }
th, td { border: 1px solid var(--edge); padding: 0.4em 0.7em; }
th { background: var(--code); }
hr { border: 0; border-top: 1px solid var(--edge); }
li.task { list-style: none; }
img { max-width: 100%; }
";

pub fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn safe_url(url: &str) -> bool {
    let clean: String = url.chars().filter(|c| !c.is_whitespace() && !c.is_control()).collect::<String>().to_lowercase();
    let cut = clean.find(['/', '?', '#']).unwrap_or(clean.len());
    match clean[..cut].split_once(':') {
        None => true,
        Some((scheme, _)) => matches!(scheme, "http" | "https" | "mailto" | "tel" | "ftp"),
    }
}

pub fn to_html(blocks: &[Block], title: &str) -> String {
    let mut body = String::new();
    for b in blocks {
        block_html(b, &mut body);
    }
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
         <meta name=\"generator\" content=\"Trindle Text\">\n<title>{}</title>\n<style>\n{CSS}</style>\n</head>\n<body>\n<main>\n{body}</main>\n</body>\n</html>\n",
        escape(title)
    )
}

fn block_html(b: &Block, out: &mut String) {
    match b {
        Block::Heading(n, s) => out.push_str(&format!("<h{n}>{}</h{n}>\n", spans_html(s))),
        Block::Para(s) => out.push_str(&format!("<p>{}</p>\n", spans_html(s))),
        Block::Quote(inner) => {
            out.push_str("<blockquote>\n");
            for b in inner {
                block_html(b, out);
            }
            out.push_str("</blockquote>\n");
        }
        Block::List(start, items) => list_html(*start, items, out),
        Block::Code(lang, code) => {
            let class = if lang.is_empty() { String::new() } else { format!(" class=\"language-{}\"", escape(lang)) };
            out.push_str(&format!("<pre><code{class}>{}\n</code></pre>\n", escape(code)));
        }
        Block::Rule => out.push_str("<hr>\n"),
        Block::Table(align, head, rows) => table_html(align, head, rows, out),
    }
}

fn list_html(start: Option<u64>, items: &[Item], out: &mut String) {
    let open = match start {
        Some(1) => "<ol>".to_string(),
        Some(n) => format!("<ol start=\"{n}\">"),
        None => "<ul>".to_string(),
    };
    out.push_str(&open);
    out.push('\n');
    for item in items {
        item_html(item, out);
    }
    out.push_str(if start.is_some() { "</ol>\n" } else { "</ul>\n" });
}

fn item_html(item: &Item, out: &mut String) {
    out.push_str(if item.task.is_some() { "<li class=\"task\">" } else { "<li>" });
    match item.task {
        Some(true) => out.push_str("<input type=\"checkbox\" disabled checked> "),
        Some(false) => out.push_str("<input type=\"checkbox\" disabled> "),
        None => {}
    }
    match item.blocks.as_slice() {
        [Block::Para(s)] => out.push_str(&spans_html(s)),
        many => {
            for (k, b) in many.iter().enumerate() {
                match b {
                    Block::Para(s) if k == 0 => {
                        out.push_str(&spans_html(s));
                        out.push('\n');
                    }
                    _ => block_html(b, out),
                }
            }
        }
    }
    out.push_str("</li>\n");
}

fn table_html(align: &[Align], head: &[Vec<Span>], rows: &[Vec<Vec<Span>>], out: &mut String) {
    let cell = |tag: &str, a: Align, s: &[Span]| {
        let style = match a {
            Align::Start => "",
            Align::Left => " style=\"text-align: left\"",
            Align::Centre => " style=\"text-align: center\"",
            Align::Right => " style=\"text-align: right\"",
        };
        format!("<{tag}{style}>{}</{tag}>", spans_html(s))
    };
    out.push_str("<table>\n<thead>\n<tr>");
    for (a, s) in align.iter().zip(head) {
        out.push_str(&cell("th", *a, s));
    }
    out.push_str("</tr>\n</thead>\n<tbody>\n");
    for row in rows {
        out.push_str("<tr>");
        for (a, s) in align.iter().zip(row) {
            out.push_str(&cell("td", *a, s));
        }
        out.push_str("</tr>\n");
    }
    out.push_str("</tbody>\n</table>\n");
}

fn spans_html(spans: &[Span]) -> String {
    let mut s = String::new();
    for span in spans {
        match span {
            Span::Text(t) => s.push_str(&escape(t)),
            Span::Bold(x) => s.push_str(&format!("<strong>{}</strong>", spans_html(x))),
            Span::Italic(x) => s.push_str(&format!("<em>{}</em>", spans_html(x))),
            Span::Strike(x) => s.push_str(&format!("<del>{}</del>", spans_html(x))),
            Span::Code(t) => s.push_str(&format!("<code>{}</code>", escape(t))),
            Span::Link(x, url) if safe_url(url) => s.push_str(&format!("<a href=\"{}\">{}</a>", escape(url), spans_html(x))),
            Span::Link(x, _) => s.push_str(&spans_html(x)),
            Span::Image(alt, url) if safe_url(url) => s.push_str(&format!("<img src=\"{}\" alt=\"{}\">", escape(url), escape(alt))),
            Span::Image(alt, _) => s.push_str(&escape(alt)),
            Span::Break => s.push_str("<br>\n"),
        }
    }
    s
}
