//! Tests: pages.

#[cfg(test)]
use super::*;

#[test]
#[ignore]
fn shoot_every_apps_page() {
    std::fs::create_dir_all(folder()).unwrap();
    every_apps_page();
}

#[test]
fn apps_in_the_dark_look_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::drawn;
    drawn::none(drawn::in_look("Apps", false, 2000, every_apps_page));
}

#[test]
fn apps_in_the_light_look_draws_only_pairs_the_table_holds() {
    use crate::toolkit::theme::drawn;
    drawn::none(drawn::in_look("Apps", true, 2000, every_apps_page));
}
