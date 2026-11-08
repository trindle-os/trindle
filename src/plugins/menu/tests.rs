use super::*;
use crate::plugins::fixtures::*;
use crate::plugins::manifest::check;

#[test]
fn a_ring_menu_is_read_in_order_and_older_plugins_have_none() {
    let dir = folder_with("menu", MENU);
    check(&dir).unwrap();
    let items = menu_of(&dir);
    assert_eq!(items.iter().map(|i| i.id.as_str()).collect::<Vec<_>>(), ["connect", "disconnect"]);
    assert_eq!(items[0], MenuItem { id: "connect".into(), label: "Connect".into(), run: vec!["bin/go".into(), "connect".into()] });
    let _ = std::fs::remove_dir_all(&dir);
    let old = folder_with("nomenu", GOOD);
    assert!(check(&old).is_ok() && menu_of(&old).is_empty());
    let _ = std::fs::remove_dir_all(&old);
}
