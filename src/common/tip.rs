//! How long the little name on a button stays, as the person chose in
//! Settings (Appearance, Accessibility). The kinds and the drawing are the
//! toolkit's (its common/tip.rs); Trindle tells it which one through its
//! host (common/host.rs).

pub use crate::toolkit::common::tip::Labels;

use super::files::setting;

pub const KEY: &str = "labels_stay";

pub fn chosen() -> Labels {
    if super::screen_reader::on() {
        Labels::Always
    } else if setting(KEY) == "on" {
        Labels::WhilePointing
    } else {
        Labels::Brief
    }
}
