//! Settings from a terminal or a text console, with no window: the screen reader key and the ways back from a theme or keys that went wrong.

use crate::toolkit::t;

pub(super) fn screen_reader(args: &mut Vec<String>, own_key: bool) -> Option<i32> {
    match crate::common::screen_reader::toggle(own_key) {
        Ok(true) => return Some(0),
        Ok(false) => {}
        Err(e) => {
            eprintln!("{e}");
            return Some(1);
        }
    }
    *args = vec![args[0].clone(), "--panel".into(), "appearance".into()];
    None
}

pub(super) fn way_back(args: &[String]) -> Option<i32> {
    let how = args.get(2).map(String::as_str);
    match args.get(1).map(String::as_str) {
        Some("--themes") => Some(list_themes()),
        Some("--theme") => Some(choose_theme(how)),
        Some("--keys") => Some(put_keys_back(how)),
        _ => None,
    }
}

fn list_themes() -> i32 {
    let chosen = crate::common::themes::chosen();
    for t in crate::common::themes::all() {
        let id = if t.id.is_empty() { "default" } else { &t.id };
        match t.refused {
            Some(why) => println!("{}", t!("settings-window-theme-refused", id = id, name = &t.name, why = why)),
            None => println!("{}{id}  {}", if t.id == chosen { "* " } else { "  " }, t.name),
        }
    }
    0
}

fn choose_theme(named: Option<&str>) -> i32 {
    let id = match named {
        Some("default" | "trindle" | "Trindle") => "",
        Some(id) => id,
        None => {
            eprintln!("{}", t!("settings-window-theme-name"));
            return 2;
        }
    };
    match crate::common::themes::choose(id) {
        Ok(()) => {
            println!("{}", t!("settings-window-theme-now", theme = if id.is_empty() { t!("settings-window-theme-own") } else { id }));
            0
        }
        Err(e) => {
            eprintln!("{}", t!("settings-window-themes-list", error = e.to_string()));
            1
        }
    }
}

fn put_keys_back(how: Option<&str>) -> i32 {
    if how != Some("default") {
        eprintln!("{}", t!("settings-window-keys-how"));
        return 2;
    }
    match crate::common::remap::save(&crate::common::remap::Table::default()) {
        Ok(()) => {
            println!("{}", t!("settings-window-keys-back"));
            0
        }
        Err(e) => {
            eprintln!("{}", t!("settings-window-keys-failed", error = e.to_string()));
            1
        }
    }
}
