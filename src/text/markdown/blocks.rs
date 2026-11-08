//! Reading the lines of a text into blocks: headings, quotes, lists, code, tables and paragraphs.

use super::marks::inline;
use super::{Align, Block, Item, Refs};
use super::Span;

pub(super) fn reference(line: &str) -> Option<(String, String)> {
    let s = shallow(line)?.strip_prefix('[')?;
    let close = s.find("]:")?;
    let name = &s[..close];
    if name.trim().is_empty() || name.contains('[') || name.contains(']') {
        return None;
    }
    let url = s[close + 2..].split_whitespace().next()?;
    let url = url.strip_prefix('<').and_then(|u| u.strip_suffix('>')).unwrap_or(url);
    Some((label(name), url.to_string()))
}

pub(super) fn label(name: &str) -> String {
    name.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

fn indent(line: &str) -> usize {
    line.len() - line.trim_start_matches(' ').len()
}

fn shallow(line: &str) -> Option<&str> {
    let n = indent(line);
    (n < 4).then(|| &line[n..])
}

fn blank(line: &str) -> bool {
    line.trim().is_empty()
}

fn heading(line: &str) -> Option<(u8, &str)> {
    let s = shallow(line)?;
    let level = s.bytes().take_while(|&b| b == b'#').count();
    if level == 0 || level > 6 {
        return None;
    }
    let rest = &s[level..];
    if !rest.is_empty() && !rest.starts_with(' ') {
        return None;
    }
    let t = rest.trim();
    let bare = t.trim_end_matches('#');
    let t = if bare.is_empty() {
        ""
    } else if bare.len() < t.len() && bare.ends_with(' ') {
        bare.trim_end()
    } else {
        t
    };
    Some((level as u8, t))
}

fn rule(line: &str) -> bool {
    let Some(s) = shallow(line) else { return false };
    let marks: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();
    match marks.first() {
        Some(&c) if matches!(c, '-' | '*' | '_') => marks.len() >= 3 && marks.iter().all(|&m| m == c),
        _ => false,
    }
}

pub(super) fn fence(line: &str) -> Option<(char, usize, &str)> {
    let s = shallow(line)?;
    let c = s.chars().next().filter(|&c| c == '`' || c == '~')?;
    let n = s.chars().take_while(|&x| x == c).count();
    if n < 3 {
        return None;
    }
    let info = s[n..].trim();
    if c == '`' && info.contains('`') {
        return None;
    }
    Some((c, n, info))
}

struct Marker<'a> {
    number: Option<u64>,
    bullet: char,

    width: usize,
    rest: &'a str,
}

fn marker(line: &str) -> Option<Marker<'_>> {
    let lead = indent(line);
    if lead >= 4 {
        return None;
    }
    let s = &line[lead..];
    let (number, bullet, len) = match s.chars().next()? {
        c @ ('-' | '*' | '+') => (None, c, 1),
        _ => {
            let digits = s.bytes().take_while(u8::is_ascii_digit).count();
            if digits == 0 || digits > 9 {
                return None;
            }
            let d = s[digits..].chars().next().filter(|&d| d == '.' || d == ')')?;
            (Some(s[..digits].parse().ok()?), d, digits + 1)
        }
    };
    let after = &s[len..];
    if after.trim().is_empty() {
        return Some(Marker { number, bullet, width: lead + len + 1, rest: "" });
    }
    if !after.starts_with(' ') {
        return None;
    }
    let spaces = match indent(after) {
        n if n > 4 => 1,
        n => n,
    };
    Some(Marker { number, bullet, width: lead + len + spaces, rest: &after[spaces..] })
}

fn starts_block(line: &str) -> bool {
    heading(line).is_some()
        || rule(line)
        || fence(line).is_some()
        || shallow(line).is_some_and(|s| s.starts_with('>'))
        || marker(line).is_some_and(|m| !m.rest.is_empty() && m.number.is_none_or(|n| n == 1))
}

fn cells(line: &str) -> Vec<String> {
    let mut t = line.trim();
    t = t.strip_prefix('|').unwrap_or(t);
    if t.ends_with('|') && !t.ends_with("\\|") {
        t = &t[..t.len() - 1];
    }
    let mut out = vec![String::new()];
    let mut chars = t.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' if chars.peek() == Some(&'|') => {
                out.last_mut().unwrap_or(&mut String::new()).push('|');
                chars.next();
            }
            '|' => out.push(String::new()),
            _ => {
                if let Some(cell) = out.last_mut() {
                    cell.push(c);
                }
            }
        }
    }
    out.iter().map(|c| c.trim().to_string()).collect()
}

fn delimiter(line: &str) -> Option<Vec<Align>> {
    if !line.contains('-') {
        return None;
    }
    cells(line)
        .iter()
        .map(|c| {
            let core = c.trim_matches(':');
            if core.is_empty() || !core.bytes().all(|b| b == b'-') {
                return None;
            }
            Some(match (c.starts_with(':'), c.ends_with(':')) {
                (true, true) => Align::Centre,
                (true, false) => Align::Left,
                (false, true) => Align::Right,
                _ => Align::Start,
            })
        })
        .collect()
}

thread_local! {
    pub(super) static DEPTH: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub(super) const MOST_DEPTH: usize = 32;

pub(super) fn blocks(lines: &[&str], refs: &Refs) -> Vec<Block> {
    DEPTH.with(|d| {
        if d.get() >= MOST_DEPTH {
            return vec![Block::Code(String::new(), lines.join("\n"))];
        }
        d.set(d.get() + 1);
        let out = blocks_in(lines, refs);
        d.set(d.get() - 1);
        out
    })
}

fn blocks_in(lines: &[&str], refs: &Refs) -> Vec<Block> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if blank(lines[i]) {
            i += 1;
            continue;
        }
        out.push(block_at(lines, &mut i, refs));
    }
    out
}

fn block_at(lines: &[&str], i: &mut usize, refs: &Refs) -> Block {
    let line = lines[*i];
    if let Some(open) = fence(line) {
        return fenced_code(lines, i, open);
    }
    if indent(line) >= 4 {
        return indented_code(lines, i);
    }
    if let Some((level, t)) = heading(line) {
        *i += 1;
        return Block::Heading(level, inline(t, refs));
    }
    if rule(line) {
        *i += 1;
        return Block::Rule;
    }
    if shallow(line).is_some_and(|s| s.starts_with('>')) {
        return quote(lines, i, refs);
    }
    if let Some(first) = marker(line) {
        return list(lines, i, refs, &first);
    }
    if let Some(table) = table_block(lines, i, refs) {
        return table;
    }
    paragraph(lines, i, refs)
}

fn fenced_code(lines: &[&str], i: &mut usize, (c, n, info): (char, usize, &str)) -> Block {
    let lead = indent(lines[*i]);
    let lang = info.split_whitespace().next().unwrap_or("").to_string();
    let mut body = Vec::new();
    *i += 1;
    while *i < lines.len() {
        let l = lines[*i];
        *i += 1;
        if fence(l).is_some_and(|(c2, n2, info2)| c2 == c && n2 >= n && info2.is_empty()) {
            break;
        }
        body.push(&l[indent(l).min(lead)..]);
    }
    Block::Code(lang, body.join("\n"))
}

fn indented_code(lines: &[&str], i: &mut usize) -> Block {
    let mut body = Vec::new();
    while *i < lines.len() && (indent(lines[*i]) >= 4 || blank(lines[*i])) {
        body.push(lines[*i].get(4..).unwrap_or(""));
        *i += 1;
    }
    while body.last().is_some_and(|l| blank(l)) {
        body.pop();
    }
    Block::Code(String::new(), body.join("\n"))
}

fn quote(lines: &[&str], i: &mut usize, refs: &Refs) -> Block {
    let mut inner: Vec<&str> = Vec::new();
    while *i < lines.len() {
        let l = lines[*i];
        if let Some(s) = shallow(l).and_then(|s| s.strip_prefix('>')) {
            inner.push(s.strip_prefix(' ').unwrap_or(s));
        } else if !blank(l) && inner.last().is_some_and(|p| !blank(p)) && !starts_block(l) {
            inner.push(l);
        } else {
            break;
        }
        *i += 1;
    }
    Block::Quote(blocks(&inner, refs))
}

fn list(lines: &[&str], i: &mut usize, refs: &Refs, first: &Marker) -> Block {
    let (ordered, bullet, start) = (first.number.is_some(), first.bullet, first.number);
    let same = |m: &Marker| m.number.is_some() == ordered && m.bullet == bullet;
    let mut items = Vec::new();
    while *i < lines.len() {
        let Some(m) = marker(lines[*i]).filter(|m| same(m)) else { break };
        if rule(lines[*i]) {
            break;
        }
        let mut body = item_lines(lines, i, &m);
        let task = task_mark(&mut body[0]);
        let refs_in: Vec<&str> = body.iter().map(String::as_str).collect();
        items.push(Item { task, blocks: blocks(&refs_in, refs) });

        let mut j = *i;
        while j < lines.len() && blank(lines[j]) {
            j += 1;
        }
        if j > *i && j < lines.len() && marker(lines[j]).is_some_and(|m| same(&m)) {
            *i = j;
        }
    }
    Block::List(start, items)
}

fn item_lines(lines: &[&str], i: &mut usize, m: &Marker) -> Vec<String> {
    let width = m.width;
    let mut body: Vec<String> = vec![m.rest.to_string()];
    *i += 1;
    while *i < lines.len() {
        let l = lines[*i];
        if blank(l) {
            let next = lines[*i + 1..].iter().position(|l| !blank(l)).map(|k| lines[*i + 1 + k]);
            if next.is_some_and(|n| indent(n) >= width) {
                body.push(String::new());
                *i += 1;
                continue;
            }
            break;
        }
        if indent(l) >= width {
            body.push(l[width..].to_string());
        } else if body.last().is_some_and(|p| !blank(p)) && !starts_block(l) && marker(l).is_none() {
            body.push(l.trim_start().to_string());
        } else {
            break;
        }
        *i += 1;
    }
    body
}

fn task_mark(first: &mut String) -> Option<bool> {
    let task = ["[ ]", "[x]", "[X]"]
        .iter()
        .find(|t| first.starts_with(**t) && first[3..].chars().next().is_none_or(|c| c == ' '))
        .map(|t| *t != "[ ]");
    if task.is_some() {
        *first = first[3..].trim_start().to_string();
    }
    task
}

fn table_block(lines: &[&str], i: &mut usize, refs: &Refs) -> Option<Block> {
    let line = lines[*i];
    if !(line.contains('|') && *i + 1 < lines.len()) {
        return None;
    }
    let head = cells(line);
    let align = delimiter(lines[*i + 1]).filter(|a| a.len() == head.len())?;
    *i += 2;
    let mut rows = Vec::new();
    while *i < lines.len() && !blank(lines[*i]) && !starts_block(lines[*i]) {
        let mut row: Vec<Vec<Span>> = cells(lines[*i]).iter().map(|c| inline(c, refs)).collect();
        row.resize(align.len(), Vec::new());
        rows.push(row);
        *i += 1;
    }
    Some(Block::Table(align, head.iter().map(|c| inline(c, refs)).collect(), rows))
}

fn paragraph(lines: &[&str], i: &mut usize, refs: &Refs) -> Block {
    let mut para = vec![lines[*i].trim_start()];
    let mut level = 0;
    *i += 1;
    while *i < lines.len() {
        let l = lines[*i];
        if blank(l) {
            break;
        }
        if let Some(t) = shallow(l).map(str::trim_end).filter(|t| !t.is_empty()) {
            if t.bytes().all(|b| b == b'=') {
                level = 1;
            } else if t.bytes().all(|b| b == b'-') {
                level = 2;
            }
            if level > 0 {
                *i += 1;
                break;
            }
        }
        if starts_block(l) {
            break;
        }
        para.push(l.trim_start());
        *i += 1;
    }
    let text = para.join("\n");
    let spans = inline(text.trim_end(), refs);
    if level > 0 { Block::Heading(level, spans) } else { Block::Para(spans) }
}
