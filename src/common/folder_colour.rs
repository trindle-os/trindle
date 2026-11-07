//! A folder shown as a thing has its icon in the folder colour, by the one rule, `Theme::folder_ink`.

use std::path::Path;

const FOLDER_ICONS: [&str; 7] = ["folder-symbolic", "user-home-symbolic", "folder-documents-symbolic", "folder-download-symbolic",
                                 "folder-music-symbolic", "folder-pictures-symbolic", "folder-videos-symbolic"];

fn closes(text: &[u8], open: usize) -> usize {
    let (mut depth, mut i, mut quoted) = (1usize, open, false);
    while i < text.len() && depth > 0 {
        match text[i] {
            b'\\' if quoted => i += 1,
            b'\'' if !quoted && text.get(i + 2) == Some(&b'\'') => i += 2,
            b'\'' if !quoted && text.get(i + 1) == Some(&b'\\') && text.get(i + 3) == Some(&b'\'') => i += 3,
            b'"' => quoted = !quoted,
            b'(' if !quoted => depth += 1,
            b')' if !quoted => depth -= 1,
            _ => {}
        }
        i += 1;
    }
    i
}

fn rows(text: &str) -> Vec<(usize, &str)> {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(found) = text[from..].find("item(") {
        let start = from + found;
        let mut end = closes(bytes, start + "item(".len());
        loop {
            let rest = &text[end..];
            let after = rest.trim_start();
            let name = after.strip_prefix('.').map(|a| a.chars().take_while(|c| c.is_alphanumeric() || *c == '_').count()).unwrap_or(0);
            let call = end + (rest.len() - after.len()) + 1 + name;
            if name == 0 || bytes.get(call) != Some(&b'(') {
                break;
            }
            end = closes(bytes, call + 1);
        }
        let whole = start == 0 || !(bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_');
        if whole {
            out.push((start, &text[start..end]));
        }
        from = start + "item(".len();
    }
    out
}

fn forgets(row: &str) -> bool {
    !row.contains(".folder()") && FOLDER_ICONS.iter().any(|icon| row.contains(&format!(".icon(\"{icon}\")")))
}

fn scan(dir: &Path, found: &mut Vec<String>) {
    let Ok(list) = std::fs::read_dir(dir) else { return };
    for entry in list.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan(&path, found);
        } else if path.extension().is_some_and(|e| e == "rs") {
            let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{} could not be read: {e}", path.display()));
            found.extend(rows(&text).into_iter().filter(|(_, row)| forgets(row))
                .map(|(at, _)| format!("{}:{}", path.display(), text[..at].matches('\n').count() + 1)));
        }
    }
}

#[test]
fn every_folder_shown_as_a_thing_takes_the_folder_colour() {
    let row = |end: &str| format!("item(name).icon(\"{}\"){end}", FOLDER_ICONS[0]);
    assert!(forgets(&row("")) && !forgets(&row(".folder()")) && !forgets("item(name).icon(\"user-trash-symbolic\")"));
    let icon = FOLDER_ICONS[0];
    let found_in = |text: &str| rows(text).into_iter().filter(|(_, r)| forgets(r)).count();
    assert_eq!(found_in(&format!("let a = item(name);\nlet b = item(name)\n    .icon(\"{icon}\")\n    .detail(short(path, &home()));\n")), 1);
    assert_eq!(found_in(&format!("item(name)\n    .icon(\"{icon}\")\n    .detail(short(path, \")\"))\n    .folder()")), 0);
    assert_eq!(found_in(&format!("item(format!(\"{{}} ({{}})\", a, b))\n\n  .folder()\n  .icon(\"{icon}\");")), 0);
    assert_eq!(found_in(&format!("vec![item(a).icon(\"{icon}\"), item(b).icon(\"{icon}\").folder()]")), 1);
    assert_eq!(found_in(&format!("if disk {{ item(a).icon(\"drive\") }} else {{ item(b).icon(\"{icon}\").folder() }}")), 0);
    assert_eq!(found_in(&format!("item(a);\nbutton(\"{icon}\", tip).icon(\"{icon}\")")), 0);
    assert_eq!(found_in(&format!("menu_item(a).icon(\"{icon}\")")), 0);
    assert_eq!(found_in(&format!("item(name.trim_matches('(').trim_matches('\"').trim_matches('\\\\'))\n.icon(\"{icon}\")")), 1);
    assert_eq!(rows("x\ny\nitem(a)\n.icon(\"z\")").first().map(|(at, row)| (*at, *row)), Some((4, "item(a)\n.icon(\"z\")")));
    let mut found = Vec::new();
    for (src, _) in crate::common::source::trees() {
        scan(&src, &mut found);
    }
    assert!(found.is_empty(), "a folder's row is not marked a folder (add .folder()) in: {found:?}");
}
