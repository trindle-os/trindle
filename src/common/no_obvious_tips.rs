//! No "i" button says "About ..." on pointing, and the tips judged obvious stay off the parts whose looks already say them.

use super::no_writing_in_place::all_code;

const OBVIOUS: &[&str] = &["store-tabs-show"];

const I_ICON: &str = "button(\"dialog-information-symbolic\"";

fn hushed_near(lines: &[&str], at: usize) -> bool {
    lines[at..lines.len().min(at + 4)].iter().any(|l| l.contains(".hushed()"))
}

#[test]
fn every_i_button_keeps_its_hover_label_away() {
    let mut wrong = Vec::new();
    for (file, text) in all_code() {
        let lines: Vec<&str> = text.lines().collect();
        for (n, line) in lines.iter().enumerate() {
            if line.contains(I_ICON) && !hushed_near(&lines, n) {
                wrong.push(format!("{file}:{}", n + 1));
            }
        }
    }
    assert!(wrong.is_empty(), "add .hushed() to the i button: {wrong:?}");
}

#[test]
fn the_tips_judged_obvious_stay_off() {
    let mut wrong = Vec::new();
    for (file, text) in all_code() {
        let lines: Vec<&str> = text.lines().collect();
        for (n, line) in lines.iter().enumerate() {
            for key in OBVIOUS {
                if line.contains(&format!("t!(\"{key}\")")) && !hushed_near(&lines, n) {
                    wrong.push(format!("{file}:{} {key}", n + 1));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "these parts say again what they are: {wrong:?}");
}

#[test]
fn the_guard_sees_a_tip_that_is_not_hushed() {
    let lines = ["a(button(\"dialog-information-symbolic\", tip)", ".size(3)", ".on(x)", ".go()", ".hushed()"];
    assert!(!hushed_near(&lines, 0));
    assert!(hushed_near(&lines, 1));
}
