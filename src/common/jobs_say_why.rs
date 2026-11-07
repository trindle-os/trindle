//! A control the program will refuse while a job runs is greyed out and says why, through the toolkit's one call (`Node::off_because`, with `ui::wait_while` for the words), so a page never looks frozen.

use super::save_is_a_word::all_lines;

const JOBS: [&str; 12] = ["busy", "working", "running", "going", "pending", "installing", "updating", "changing", "signing", "pairing", "waiting", "checking"];
const PROGRAMS: [&str; 4] = ["store/", "settings/", "files/", "disks/"];

const LEFT: [(&str, &str, &str); 3] = [
    ("settings/panels/permissions.rs", ".off(waiting)", "greyed until Flatpak has answered, which no job of the person's is"),
    ("store/update_page/view.rs", "store-upd-checking-main", "the main pill whose own words say it is checking"),
    ("store/update_page/view.rs", "store-upd-installing-main", "the main pill whose own words say it is installing"),
];

fn plain_off(line: &str) -> bool {
    line.contains(".off(") && !line.contains("off_because(") && JOBS.iter().any(|j| line.contains(j))
}

fn plain_offs(lines: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let Some(at) = line.find(".off(") else { continue };
        let (mut call, mut depth, mut started, mut last) = (String::new(), 0i32, false, i);
        'call: for (j, l) in lines[i..].iter().take(8).enumerate() {
            last = i + j;
            for c in l[if j == 0 { at } else { 0 }..].chars() {
                call.push(c);
                match c {
                    '(' => (depth, started) = (depth + 1, true),
                    ')' => {
                        depth -= 1;
                        if started && depth == 0 {
                            break 'call;
                        }
                    }
                    _ => {}
                }
            }
            call.push(' ');
        }
        let mut chain = format!("{call}{}{}", lines[i], lines[last]);
        for l in lines[last + 1..].iter().take(4).take_while(|l| l.trim_start().starts_with('.')) {
            chain.push_str(l);
        }
        for l in lines[..i].iter().rev().take(4).take_while(|l| l.trim_start().starts_with('.')) {
            chain.push_str(l);
        }
        if JOBS.iter().any(|j| call.contains(j)) && !chain.contains("off_because(") {
            out.push(format!("{} {}", line.trim(), call));
        }
    }
    out
}

fn by_file() -> Vec<(String, Vec<String>)> {
    let mut out: Vec<(String, Vec<String>)> = Vec::new();
    for (file, line) in all_lines() {
        match out.last_mut() {
            Some((f, lines)) if *f == file => lines.push(line),
            _ => out.push((file, vec![line])),
        }
    }
    out
}

#[test]
fn no_control_is_greyed_out_for_a_job_without_saying_why() {
    let mut bad = Vec::new();
    for (file, lines) in by_file().iter().filter(|(file, _)| PROGRAMS.iter().any(|p| file.starts_with(p))) {
        let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
        for found in plain_offs(&refs) {
            if !LEFT.iter().any(|(f, words, _)| file == f && found.contains(words)) {
                bad.push(format!("{file}: {found}"));
            }
        }
    }
    assert!(bad.is_empty(), "grey these out with off_because(ui::wait_while(..)) so the reason shows:\n{}", bad.join("\n"));
}

#[test]
fn every_excuse_still_matches_a_line() {
    let files = by_file();
    for (file, words, why) in LEFT {
        assert!(!why.is_empty());
        let found = files.iter().any(|(f, lines)| f == file && lines.iter().any(|l| l.contains(words)));
        assert!(found, "{file} no longer has a line with {words}: take it out of LEFT");
    }
}

#[test]
fn the_rule_tells_a_plain_off_from_one_that_says_why() {
    assert!(plain_off("    button(a, b).on_press(c).off(self.busy)"));
    assert!(!plain_off("    button(a, b).on_press(c).off_because(wait_while(self.busy))"));
    assert!(!plain_off("    button(a, b).off(true)"));
    assert!(plain_off("    .off(r.busy || r.done)"));
    assert_eq!(plain_offs(&["    button(a, b)", "        .on_press(c)", "        .off(", "            self.busy", "        ),"]).len(), 1);
    assert_eq!(plain_offs(&["    button(a, b)", "        .off(self.busy)", "        .on_press(c),"]).len(), 1);
    assert!(plain_offs(&["    button(a, b)", "        .off(", "            self.busy,", "        )", "        .off_because(ui::wait_while(self.busy)),"]).is_empty());
    assert!(plain_offs(&["    button(a, b)", "        .off_because(ui::wait_while(self.busy))", "        .off(self.busy),"]).is_empty());
    assert!(plain_offs(&["    button(a, b).off(true),", "    let busy = 1;"]).is_empty(), "a word on the next line is not in the call");
}
