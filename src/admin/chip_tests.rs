//! Tests: sealing the disk's key to the chip again, with every command stood in.

use super::*;
use crate::admin::stand_in::StandIn;

const UUID: &str = "0b5f6a7e-1c2d-4e3f-8a9b-0c1d2e3f4a5b";
const OTHER: &str = "11111111-2222-4333-8444-555555555555";

fn device() -> String {
    format!("/dev/disk/by-uuid/{UUID}")
}

fn header() -> Vec<u8> {
    header_with(r#"["1"]"#)
}

fn header_with(slots: &str) -> Vec<u8> {
    let protected = base64url(br#"{"alg":"dir","clevis":{"pin":"tpm2","tpm2":{}},"enc":"A256GCM"}"#);
    let json = format!(r#"{{"keyslots":{{"0":{{}},"1":{{}}}},"tokens":{{"0":{{"type":"clevis","keyslots":{slots},"jwe":{{"protected":"{protected}"}}}}}},"segments":{{}}}}"#);
    let size: u64 = 16384;
    let mut disk = vec![0u8; size as usize];
    disk[..6].copy_from_slice(b"LUKS\xba\xbe");
    disk[6..8].copy_from_slice(&2u16.to_be_bytes());
    disk[8..16].copy_from_slice(&size.to_be_bytes());
    disk[168..204].copy_from_slice(UUID.as_bytes());
    disk[4096..4096 + json.len()].copy_from_slice(json.as_bytes());
    disk
}

fn base64url(bytes: &[u8]) -> String {
    const A: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
    bytes.chunks(3).flat_map(|c| {
        let n = c.iter().enumerate().fold(0u32, |n, (i, &b)| n | (u32::from(b) << (16 - 8 * i)));
        (0..=c.len()).map(move |i| A[((n >> (18 - 6 * i)) & 63) as usize] as char)
    }).collect()
}

fn after_a_refusal(stand: &StandIn) {
    stand.write(REFUSED, format!("{UUID}\n").as_bytes());
    stand.write(CRYPTTAB, format!("root_crypt UUID={UUID} none luks,discard\n").as_bytes());
    stand.write(&device(), &header());
    stand.write(&format!("{TPMS}/tpm0/dev"), b"");
    stand.write(CLEVIS, b"");
    stand.prints("cryptsetup open", "Key slot 0 unlocked.\nCommand successful.\n");
}

fn proving_fails_for(slots: &'static [&'static str]) -> impl FnMut(&[&str]) -> Outcome {
    move |argv: &[&str]| if argv[0] == "sh" && slots.contains(argv.last().unwrap()) { Err("no".into()) } else { Ok(()) }
}

#[test]
fn a_new_chip_key_is_added_and_proved_before_the_old_one_goes() {
    let stand = StandIn::new("chip-reseal", proving_fails_for(&["1"]));
    after_a_refusal(&stand);
    assert_eq!(reseal_with(UUID, "right"), Ok(()));
    let ran = stand.ran();
    let bind = ran.iter().position(|c| c.starts_with("clevis luks bind") && c.contains(" -s 2 ")).expect("bound to the free slot");
    let unbind = ran.iter().position(|c| c.starts_with("clevis luks unbind") && c.ends_with("-s 1")).expect("the old chip key goes");
    assert!(bind < unbind, "{ran:?}");
    assert!(ran.iter().any(|c| c.starts_with("sh -c") && c.ends_with(" 2")), "the new key is proved: {ran:?}");
    assert!(!ran.iter().any(|c| c.contains("unbind") && c.ends_with("-s 0")), "the password's slot is never touched");
    assert!(!stand.path(REFUSED).exists(), "the note goes once the chip works");
}

#[test]
fn a_new_key_that_cannot_be_proved_is_taken_away_and_the_old_kept() {
    let stand = StandIn::new("chip-unproved", proving_fails_for(&["1", "2"]));
    after_a_refusal(&stand);
    assert_eq!(reseal_with(UUID, "right"), Err(t!("admin-chip-not-sealed").to_string()));
    let ran = stand.ran();
    assert!(ran.iter().any(|c| c.starts_with("clevis luks unbind") && c.ends_with("-s 2")), "{ran:?}");
    assert!(!ran.iter().any(|c| c.starts_with("clevis luks unbind") && c.ends_with("-s 1")), "{ran:?}");
    assert!(stand.path(REFUSED).exists(), "Settings still offers it");
}

#[test]
fn a_chip_that_opens_the_disk_again_needs_no_new_key() {
    let stand = StandIn::new("chip-works", |_| Ok(()));
    after_a_refusal(&stand);
    assert_eq!(reseal_with(UUID, "right"), Ok(()));
    assert!(!stand.ran().iter().any(|c| c.starts_with("clevis luks bind")), "{:?}", stand.ran());
    assert!(!stand.path(REFUSED).exists());
}

#[test]
fn wrong_passwords_are_counted_and_then_made_to_wait() {
    let stand = StandIn::new("chip-wrong", |argv| if argv[0] == "cryptsetup" { Err("exit 2".into()) } else { Ok(()) });
    after_a_refusal(&stand);
    for _ in 0..FREE_TRIES {
        assert_eq!(reseal_with(UUID, "wrong"), Err(t!("admin-chip-wrong-password").to_string()));
    }
    assert_eq!(reseal_with(UUID, "wrong"), Err(t!("admin-chip-wait").to_string()));
    assert!(!stand.ran().iter().any(|c| c.starts_with("clevis")), "nothing is sealed on a wrong password");
}

#[test]
fn only_the_disk_the_note_names_and_crypttab_lists_is_sealed() {
    let stand = StandIn::new("chip-which", |_| Ok(()));
    assert_eq!(reseal_with(UUID, "right"), Err(t!("admin-chip-not-needed").to_string()), "no note");
    after_a_refusal(&stand);
    assert_eq!(reseal_with(OTHER, "right"), Err(t!("admin-chip-changed").to_string()), "not the noted disk");
    stand.write(CRYPTTAB, format!("root_crypt UUID={OTHER} none luks\n").as_bytes());
    assert_eq!(reseal_with(UUID, "right"), Err(t!("admin-chip-changed").to_string()), "not in crypttab");
    assert!(stand.ran().is_empty(), "{:?}", stand.ran());
}

#[test]
fn no_chip_or_no_clevis_is_said_before_anything_runs() {
    let stand = StandIn::new("chip-none", |_| Ok(()));
    after_a_refusal(&stand);
    std::fs::remove_dir_all(stand.path(TPMS)).unwrap();
    assert_eq!(reseal_with(UUID, "right"), Err(t!("admin-chip-none").to_string()));
    stand.write(&format!("{TPMS}/tpm0/dev"), b"");
    std::fs::remove_file(stand.path(CLEVIS)).unwrap();
    assert_eq!(reseal_with(UUID, "right"), Err(t!("admin-chip-no-clevis").to_string()));
    assert!(stand.ran().is_empty(), "{:?}", stand.ran());
}

#[test]
fn the_password_is_taken_exactly_or_not_at_all() {
    assert_eq!(password(&mut &b"secret"[..]).map(|p| p.to_string()), Ok("secret".into()));
    assert_eq!(password(&mut &b"secret\n"[..]).map(|p| p.to_string()), Ok("secret".into()), "as Settings sends it");
    assert_eq!(password(&mut &b"\n"[..]).map(|p| p.to_string()), Err(t!("admin-password-none").to_string()));
    assert_eq!(password(&mut &b""[..]).map(|p| p.to_string()), Err(t!("admin-password-none").to_string()));
    for bad in [&b"two\nlines"[..], b"two\n\n", b"nul\0", b"\xff\xfe"] {
        assert!(password(&mut &bad[..]).is_err(), "{bad:?}");
    }
    let long = vec![b'a'; MOST_PASSWORD as usize + 1];
    assert!(password(&mut &long[..]).is_err());
}

#[test]
fn the_slot_the_password_opened_is_read_from_cryptsetup() {
    assert_eq!(opened_slot("Key slot 3 unlocked.\nCommand successful.\n"), Some(3));
    assert_eq!(opened_slot("No key available with this passphrase.\n"), None);
}

#[test]
fn the_slot_the_password_opens_stays_even_where_a_chip_token_names_it() {
    let stand = StandIn::new("chip-own-slot", proving_fails_for(&["0", "1"]));
    after_a_refusal(&stand);
    stand.write(&device(), &header_with(r#"["0","1"]"#));
    assert_eq!(reseal_with(UUID, "right"), Ok(()));
    let ran = stand.ran();
    assert!(ran.iter().any(|c| c.starts_with("clevis luks unbind") && c.ends_with("-s 1")), "{ran:?}");
    assert!(!ran.iter().any(|c| c.starts_with("clevis luks unbind") && c.ends_with("-s 0")), "{ran:?}");
}
