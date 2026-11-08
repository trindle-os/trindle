//! Reading the inline marks of a paragraph into spans: emphasis, code, links and entities.

use super::blocks::label;
use super::{Refs, plain};
use super::Span;
use super::blocks::MOST_DEPTH;
use super::blocks::DEPTH;

pub fn inline(text: &str, refs: &Refs) -> Vec<Span> {
    let chars: Vec<char> = text.chars().collect();
    let mut out = Vec::new();
    spans(&chars, refs, &mut out);
    out
}

fn push_text(out: &mut Vec<Span>, s: &str) {
    if s.is_empty() {
        return;
    }
    if let Some(Span::Text(t)) = out.last_mut() {
        t.push_str(s);
    } else {
        out.push(Span::Text(s.to_string()));
    }
}

fn run(c: &[char], i: usize, ch: char) -> usize {
    c[i..].iter().take_while(|&&x| x == ch).count()
}

fn ticks_end(c: &[char], from: usize, n: usize) -> Option<usize> {
    let mut j = from;
    while j < c.len() {
        if c[j] == '`' {
            let m = run(c, j, '`');
            if m == n {
                return Some(j);
            }
            j += m;
        } else {
            j += 1;
        }
    }
    None
}

fn closer(c: &[char], i: usize, n: usize, ch: char) -> Option<usize> {
    let next = c.get(i + n)?;
    if next.is_whitespace() || (ch == '_' && i > 0 && c[i - 1].is_alphanumeric()) {
        return None;
    }
    let mut j = i + n;
    while j < c.len() {
        match c[j] {
            '\\' => j += 2,
            '`' => {
                let m = run(c, j, '`');
                j = ticks_end(c, j + m, m).map(|e| e + m).unwrap_or(j + m);
            }
            x if x == ch => {
                let m = run(c, j, ch);
                let after_ok = ch != '_' || c.get(j + m).is_none_or(|a| !a.is_alphanumeric());
                if m == n && j > i + n && !c[j - 1].is_whitespace() && after_ok {
                    return Some(j);
                }
                j += m;
            }
            _ => j += 1,
        }
    }
    None
}

fn link_at(c: &[char], i: usize, refs: &Refs) -> Option<(usize, String, usize)> {
    let close = bracket_end(c, i)?;
    let words: String = c[i + 1..close].iter().collect();
    match c.get(close + 1) {
        Some('(') => inline_target(c, close + 2).map(|(url, next)| (close, url, next)),
        Some('[') => {
            let end = (close + 2..c.len()).find(|&k| c[k] == ']')?;
            let name: String = c[close + 2..end].iter().collect();
            let name = if name.trim().is_empty() { &words } else { &name };
            refs.get(&label(name)).map(|u| (close, u.clone(), end + 1))
        }
        _ => refs.get(&label(&words)).map(|u| (close, u.clone(), close + 1)),
    }
}

fn bracket_end(c: &[char], i: usize) -> Option<usize> {
    let mut depth = 0;
    let mut j = i;
    loop {
        match c.get(j)? {
            '\\' => j += 1,
            '`' => {
                let m = run(c, j, '`');
                j = ticks_end(c, j + m, m).map(|e| e + m - 1).unwrap_or(j + m - 1);
            }
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(j);
                }
            }
            _ => {}
        }
        j += 1;
    }
}

fn inline_target(c: &[char], k: usize) -> Option<(String, usize)> {
    let (url, k) = link_url(c, skip_space(c, k));
    let mut k = skip_space(c, k);
    if let Some(&q) = c.get(k).filter(|q| matches!(q, '"' | '\'' | '(')) {
        let end = if q == '(' { ')' } else { q };
        k += 1;
        while c.get(k).is_some_and(|&x| x != end) {
            k += 1;
        }
        k = skip_space(c, k + 1);
    }
    (c.get(k) == Some(&')')).then(|| (url, k + 1))
}

fn skip_space(c: &[char], mut k: usize) -> usize {
    while c.get(k).is_some_and(|x| x.is_whitespace()) {
        k += 1;
    }
    k
}

fn link_url(c: &[char], mut k: usize) -> (String, usize) {
    let mut url = String::new();
    if c.get(k) == Some(&'<') {
        k += 1;
        while let Some(&x) = c.get(k) {
            k += 1;
            if x == '>' {
                break;
            }
            url.push(x);
        }
    } else {
        let mut parens = 0;
        while let Some(&x) = c.get(k) {
            if x.is_whitespace() || (x == ')' && parens == 0) {
                break;
            }
            if x == '(' {
                parens += 1;
            } else if x == ')' {
                parens -= 1;
            } else if x == '\\' && c.get(k + 1).is_some_and(char::is_ascii_punctuation) {
                k += 1;
                url.push(c[k]);
                k += 1;
                continue;
            }
            url.push(x);
            k += 1;
        }
    }
    (url, k)
}

fn entity(c: &[char], i: usize) -> Option<(String, usize)> {
    let end = (i + 1..c.len().min(i + 12)).find(|&k| c[k] == ';')?;
    let name: String = c[i + 1..end].iter().collect();
    let ch = if let Some(num) = name.strip_prefix('#') {
        let code = match num.strip_prefix(['x', 'X']) {
            Some(hex) => u32::from_str_radix(hex, 16).ok()?,
            None => num.parse().ok()?,
        };
        char::from_u32(code).filter(|&c| c != '\0').unwrap_or('\u{FFFD}')
    } else {
        match name.as_str() {
            "amp" => '&',
            "lt" => '<',
            "gt" => '>',
            "quot" => '"',
            "apos" => '\'',
            "nbsp" => '\u{A0}',
            "copy" => '\u{A9}',
            "reg" => '\u{AE}',
            "trade" => '\u{2122}',
            "hellip" => '\u{2026}',
            "ndash" => '\u{2013}',
            "mdash" => '\u{2014}',
            "pound" => '\u{A3}',
            "euro" => '\u{20AC}',
            _ => return None,
        }
    };
    Some((ch.to_string(), end + 1))
}

fn is_uri(s: &str) -> bool {
    let Some((scheme, rest)) = s.split_once(':') else { return false };
    scheme.len() >= 2
        && scheme.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
        && !rest.is_empty()
        && !s.chars().any(|c| c.is_whitespace() || c == '<')
}

fn is_email(s: &str) -> bool {
    let Some((name, host)) = s.split_once('@') else { return false };
    !name.is_empty() && host.contains('.') && !s.chars().any(|c| c.is_whitespace() || matches!(c, '<' | '>' | ':'))
}

fn spans(c: &[char], refs: &Refs, out: &mut Vec<Span>) {
    DEPTH.with(|d| {
        if d.get() < MOST_DEPTH {
            d.set(d.get() + 1);
            spans_in(c, refs, out);
            d.set(d.get() - 1);
        }
    });
}

fn spans_in(c: &[char], refs: &Refs, out: &mut Vec<Span>) {
    let mut i = 0;
    while i < c.len() {
        let ch = c[i];
        i = match ch {
            '\\' if c.get(i + 1) == Some(&'\n') => {
                out.push(Span::Break);
                i + 2
            }
            '\\' if c.get(i + 1).is_some_and(char::is_ascii_punctuation) => {
                push_text(out, &c[i + 1].to_string());
                i + 2
            }
            '\n' => line_end(out, i),
            '`' => code_span(c, i, out),
            '!' if c.get(i + 1) == Some(&'[') => image(c, i, refs, out),
            '[' => link(c, i, refs, out),
            '<' => angle_link(c, i, out),
            '*' | '_' | '~' => emphasis(c, i, refs, out),
            '&' => match entity(c, i) {
                Some((s, next)) => {
                    push_text(out, &s);
                    next
                }
                None => {
                    push_text(out, "&");
                    i + 1
                }
            },
            'h' if (i == 0 || c[i - 1].is_whitespace() || c[i - 1] == '(') && bare_link(&c[i..]) > 0 => {
                let n = bare_link(&c[i..]);
                let url: String = c[i..i + n].iter().collect();
                out.push(Span::Link(vec![Span::Text(url.clone())], url));
                i + n
            }
            _ => {
                let mut buf = [0; 4];
                push_text(out, ch.encode_utf8(&mut buf));
                i + 1
            }
        };
    }
}

fn line_end(out: &mut Vec<Span>, i: usize) -> usize {
    let hard = matches!(out.last(), Some(Span::Text(t)) if t.ends_with("  "));
    if let Some(Span::Text(t)) = out.last_mut() {
        let kept = t.trim_end_matches(' ').len();
        t.truncate(kept);
    }
    if hard {
        out.push(Span::Break);
    } else {
        push_text(out, " ");
    }
    i + 1
}

fn code_span(c: &[char], i: usize, out: &mut Vec<Span>) -> usize {
    let n = run(c, i, '`');
    if let Some(end) = ticks_end(c, i + n, n) {
        let inner: String = c[i + n..end].iter().collect::<String>().replace('\n', " ");
        let trim = inner.len() > 2 && inner.starts_with(' ') && inner.ends_with(' ') && !inner.trim().is_empty();
        out.push(Span::Code(if trim { inner[1..inner.len() - 1].to_string() } else { inner }));
        end + n
    } else {
        push_text(out, &"`".repeat(n));
        i + n
    }
}

fn image(c: &[char], i: usize, refs: &Refs, out: &mut Vec<Span>) -> usize {
    match link_at(c, i + 1, refs) {
        Some((close, url, next)) => {
            let mut alt = Vec::new();
            spans(&c[i + 2..close], refs, &mut alt);
            out.push(Span::Image(plain(&alt), url));
            next
        }
        None => {
            push_text(out, "!");
            i + 1
        }
    }
}

fn link(c: &[char], i: usize, refs: &Refs, out: &mut Vec<Span>) -> usize {
    match link_at(c, i, refs) {
        Some((close, url, next)) => {
            let mut words = Vec::new();
            spans(&c[i + 1..close], refs, &mut words);
            out.push(Span::Link(words, url));
            next
        }
        None => {
            push_text(out, "[");
            i + 1
        }
    }
}

fn angle_link(c: &[char], i: usize, out: &mut Vec<Span>) -> usize {
    let end = (i + 1..c.len()).take_while(|&k| c[k] != '<' && c[k] != '\n').find(|&k| c[k] == '>');
    let inside: Option<String> = end.map(|e| c[i + 1..e].iter().collect());
    match (end, inside) {
        (Some(e), Some(s)) if is_uri(&s) => {
            out.push(Span::Link(vec![Span::Text(s.clone())], s));
            e + 1
        }
        (Some(e), Some(s)) if is_email(&s) => {
            out.push(Span::Link(vec![Span::Text(s.clone())], format!("mailto:{s}")));
            e + 1
        }
        _ => {
            push_text(out, "<");
            i + 1
        }
    }
}

fn emphasis(c: &[char], i: usize, refs: &Refs, out: &mut Vec<Span>) -> usize {
    let ch = c[i];
    let n = run(c, i, ch);
    let fits = if ch == '~' { n == 2 } else { n <= 3 };
    match closer(c, i, n, ch).filter(|_| fits) {
        Some(end) => {
            let mut inner = Vec::new();
            spans(&c[i + n..end], refs, &mut inner);
            out.push(match (ch, n) {
                ('~', _) => Span::Strike(inner),
                (_, 1) => Span::Italic(inner),
                (_, 2) => Span::Bold(inner),
                _ => Span::Bold(vec![Span::Italic(inner)]),
            });
            end + n
        }
        None => {
            push_text(out, &ch.to_string().repeat(n));
            i + n
        }
    }
}

fn bare_link(c: &[char]) -> usize {
    let head: String = c.iter().take(8).collect();
    if !head.starts_with("http://") && !head.starts_with("https://") {
        return 0;
    }
    let mut n = c.iter().take_while(|x| !x.is_whitespace() && **x != '<').count();
    loop {
        let Some(&last) = c[..n].last() else { return 0 };
        let open = c[..n].iter().filter(|&&x| x == '(').count();
        let shut = c[..n].iter().filter(|&&x| x == ')').count();
        if matches!(last, '.' | ',' | ':' | ';' | '!' | '?' | '"' | '\'' | '*' | '_' | '~') || (last == ')' && shut > open) {
            n -= 1;
        } else {
            break;
        }
    }
    if n <= head.find("//").unwrap_or(0) + 2 {
        0
    } else {
        n
    }
}
