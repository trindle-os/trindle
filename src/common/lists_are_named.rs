//! A screen reader says "list" and nothing more of a list that has no name. Every list or tree Trindle draws is given one with `.called(..)`.

use super::no_writing_in_place::{all_code, code};

fn statement_after(text: &str, from: usize) -> &str {
    let rest = &text[from..];
    let (mut depth, mut end) = (0i32, rest.len());
    for (i, c) in rest.char_indices() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            ';' if depth <= 0 => {
                end = i;
                break;
            }
            _ => {}
        }
        if depth < 0 {
            end = i;
            break;
        }
    }
    &rest[..end]
}

fn unnamed_lists(text: &str) -> Vec<usize> {
    let mut bare = Vec::new();
    for word in ["list(", "tree("] {
        for (at, _) in text.match_indices(word) {
            let before = text[..at].chars().next_back().unwrap_or(' ');
            let call = text[..at].ends_with("fn ") || before.is_alphanumeric() || matches!(before, '_' | '.' | ':');
            if call || text[at + word.len()..].starts_with(')') {
                continue;
            }
            let statement = statement_after(text, at);
            if !statement.contains("Msg::") && !statement.contains('|') {
                continue;
            }
            if !has_a_name(statement) {
                bare.push(text[..at].matches('\n').count() + 1);
            }
        }
    }
    bare
}

fn has_a_name(statement: &str) -> bool {
    statement.match_indices(".called(").any(|(i, m)| {
        let inner = statement[i + m.len()..].trim_start();
        !inner.starts_with(')') && !inner.starts_with("\"\")")
    })
}

#[test]
fn every_list_and_tree_drawn_is_called_something() {
    let mut bare = Vec::new();
    for (name, text) in all_code() {
        if name.starts_with("toolkit/") {
            continue;
        }
        let text = code(&text).join("\n");
        bare.extend(unnamed_lists(&text).into_iter().map(|line| format!("{name}:{line}")));
    }
    assert!(bare.is_empty(), "a list with no name for a screen reader (add `.called(..)`): {bare:?}");
}

#[test]
fn the_audit_sees_unnamed_lists_over_many_lines_and_empty_names() {
    let named = "let a = list(\n    rows,\n    |i| Msg::Pick(i),\n).called(\"Files\");";
    assert!(unnamed_lists(named).is_empty());
    let unnamed = "let a = list(\n    rows,\n    |i| Msg::Pick(i),\n);";
    assert_eq!(unnamed_lists(unnamed), [1]);
    let empty = "let a = list(rows, |i| Msg::Pick(i)).called(\"\");";
    assert_eq!(unnamed_lists(empty), [1]);
    let one_line = "x();\nlet a = tree(rows, |i| Msg::Pick(i));";
    assert_eq!(unnamed_lists(one_line), [2]);
}
