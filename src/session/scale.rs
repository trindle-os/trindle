//! trindle-scale.

use crate::common::outputs::{self, Head, Setting};

pub fn choose(out: &Head) -> Option<f64> {
    out.current_mode()?;
    Some(out.screen().worked_out())
}

pub fn main(_args: Vec<String>) -> i32 {
    let heads = match outputs::heads() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("trindle-scale: {e}");
            return 1;
        }
    };
    let mut changes = Vec::new();
    for out in heads.iter().filter(|o| o.enabled) {
        let Some(scale) = choose(out) else { continue };
        if (scale - out.scale).abs() > 0.01 {
            changes.push(Setting { scale: Some(scale), ..Setting::on(&out.name) });
            println!("trindle-scale: {} at {}%", out.name, (scale * 100.0).round());
        }
    }
    if changes.is_empty() && !outputs::overlapping(&heads) {
        return 0;
    }
    match outputs::apply(&changes) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("trindle-scale: {e}");
            1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::outputs::{Mode, Turn};

    fn head(name: &str, width_mm: i32, width_px: i32) -> Head {
        Head {
            name: name.into(), description: String::new(), make: String::new(), model: String::new(),
            serial: String::new(), physical_size: (width_mm, 0),
            modes: vec![Mode { width: width_px, height: 1080, refresh: 60000, preferred: true }],
            current: Some(0), enabled: true, position: (0, 0), turn: Turn::Normal, scale: 1.0, varies_rate: false,
        }
    }

    #[test]
    fn sizes() {
        assert_eq!(choose(&head("eDP-1", 310, 1920)), Some(1.25));
        assert_eq!(choose(&head("HDMI-A-1", 527, 1920)), Some(1.0));
        assert_eq!(choose(&head("DP-1", 597, 3840)), Some(1.25));
        assert_eq!(choose(&head("eDP-1", 0, 1920)), Some(1.25));
        assert_eq!(choose(&head("HDMI-A-1", 0, 1920)), Some(1.0));
    }
}
