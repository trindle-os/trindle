//! A disk's LUKS2 header as far as the TPM chip goes, and the note the
//! start-up password screen leaves when the chip did not open the disk.

use std::io::Read;

pub const REFUSED: &str = "/run/trindle-chip-refused";

pub const POLICY: &str = r#"{"pcr_bank":"sha256","pcr_ids":"7"}"#;
pub const TPMS: &str = "/sys/class/tpm";

const MOST_HEADER: u64 = 4 << 20;

const BINARY: usize = 4096;

const SLOTS: u32 = 32;

pub struct Header {
    pub uuid: String,
    pub json: String,
}

pub fn chip_seen() -> bool {
    chip_listed(std::path::Path::new(TPMS))
}

pub fn chip_listed(tpms: &std::path::Path) -> bool {
    std::fs::read_dir(tpms).is_ok_and(|d| d.flatten().any(|e| e.file_name().to_string_lossy().starts_with("tpm")))
}

pub fn valid_uuid(text: &str) -> bool {
    let b = text.as_bytes();
    b.len() == 36 && b.iter().enumerate().all(|(i, c)| match i {
        8 | 13 | 18 | 23 => *c == b'-',
        _ => c.is_ascii_digit() || (b'a'..=b'f').contains(c),
    })
}

pub fn refused_disk(note: &std::path::Path, owner: u32) -> Option<String> {
    use std::os::unix::fs::MetadataExt;
    let meta = std::fs::symlink_metadata(note).ok()?;
    if !meta.file_type().is_file() || meta.uid() != owner || meta.len() > 64 {
        return None;
    }
    let text = std::fs::read_to_string(note).ok()?;
    Some(text.trim().to_string()).filter(|u| valid_uuid(u))
}

pub fn read_header(mut disk: impl Read) -> Option<Header> {
    let mut head = [0u8; BINARY];
    disk.read_exact(&mut head).ok()?;
    if &head[..6] != b"LUKS\xba\xbe" || u16::from_be_bytes([head[6], head[7]]) != 2 {
        return None;
    }
    let size = u64::from_be_bytes(head[8..16].try_into().ok()?);
    if !(BINARY as u64 + 1..=MOST_HEADER).contains(&size) {
        return None;
    }
    let uuid = std::str::from_utf8(&head[168..208]).ok()?.trim_end_matches('\0').to_string();
    let mut json = Vec::new();
    disk.take(size - BINARY as u64).read_to_end(&mut json).ok()?;
    let end = json.iter().position(|&b| b == 0).unwrap_or(json.len());
    json.truncate(end);
    Some(Header { uuid, json: String::from_utf8(json).ok()? })
}

pub fn chip_slots(json: &str) -> Vec<u32> {
    let Ok(header) = serde_json::from_str::<serde_json::Value>(json) else { return Vec::new() };
    let Some(tokens) = header.get("tokens").and_then(|t| t.as_object()) else { return Vec::new() };
    let mut slots: Vec<u32> = tokens.values()
        .filter(|t| t.get("type").and_then(|v| v.as_str()) == Some("clevis"))
        .filter(|t| t.pointer("/jwe/protected").and_then(|v| v.as_str()).and_then(base64url)
            .and_then(|p| serde_json::from_slice::<serde_json::Value>(&p).ok())
            .is_some_and(|p| p.pointer("/clevis/pin").and_then(|v| v.as_str()) == Some("tpm2")))
        .filter_map(|t| t.get("keyslots").and_then(|k| k.as_array()))
        .flatten()
        .filter_map(|s| s.as_str()?.parse().ok())
        .filter(|s| *s < SLOTS)
        .collect();
    slots.sort_unstable();
    slots.dedup();
    slots
}

pub fn has_chip_key(json: &str) -> bool {
    !chip_slots(json).is_empty()
}

pub fn free_slot(json: &str) -> Option<u32> {
    let header = serde_json::from_str::<serde_json::Value>(json).ok()?;
    let used = header.get("keyslots")?.as_object()?;
    (0..SLOTS).find(|s| !used.contains_key(&s.to_string()))
}

fn base64url(text: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(text.len() * 3 / 4);
    let (mut bits, mut have) = (0u32, 0u32);
    for c in text.bytes().filter(|&c| c != b'=') {
        let v = match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'-' => 62,
            b'_' => 63,
            _ => return None,
        };
        bits = (bits << 6) | u32::from(v);
        have += 6;
        if have >= 8 {
            have -= 8;
            out.push((bits >> have) as u8);
            bits &= (1 << have) - 1;
        }
    }
    Some(out)
}

#[cfg(test)]
mod made_up {
    pub const UUID: &str = "0b5f6a7e-1c2d-4e3f-8a9b-0c1d2e3f4a5b";

    fn encode(bytes: &[u8]) -> String {
        const A: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
        let mut s = String::new();
        for chunk in bytes.chunks(3) {
            let n = chunk.iter().enumerate().fold(0u32, |n, (i, &b)| n | (u32::from(b) << (16 - 8 * i)));
            for i in 0..=chunk.len() {
                s.push(A[((n >> (18 - 6 * i)) & 63) as usize] as char);
            }
        }
        s
    }

    pub fn header(slots: &[u32], tokens: &str) -> Vec<u8> {
        let size: u64 = 16384;
        let mut disk = vec![0u8; size as usize];
        disk[..6].copy_from_slice(b"LUKS\xba\xbe");
        disk[6..8].copy_from_slice(&2u16.to_be_bytes());
        disk[8..16].copy_from_slice(&size.to_be_bytes());
        disk[168..204].copy_from_slice(UUID.as_bytes());
        let slots = slots.iter().map(|s| format!(r#""{s}":{{}}"#)).collect::<Vec<_>>().join(",");
        let json = format!(r#"{{"keyslots":{{{slots}}},"tokens":{{{tokens}}},"segments":{{}}}}"#);
        disk[super::BINARY..super::BINARY + json.len()].copy_from_slice(json.as_bytes());
        disk
    }

    pub fn clevis(id: u32, pin: &str, slot: u32) -> String {
        let protected = encode(format!(r#"{{"alg":"dir","clevis":{{"pin":"{pin}","{pin}":{{}}}},"enc":"A256GCM"}}"#).as_bytes());
        format!(r#""{id}":{{"type":"clevis","keyslots":["{slot}"],"jwe":{{"ciphertext":"","protected":"{protected}"}}}}"#)
    }

    pub fn base64(bytes: &[u8]) -> String {
        encode(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::made_up::{base64, clevis, header, UUID};
    use super::*;

    fn slots(disk: &[u8]) -> Vec<u32> {
        read_header(disk).map(|h| chip_slots(&h.json)).unwrap_or_default()
    }

    #[test]
    fn only_a_disk_bound_to_the_chip_counts() {
        assert_eq!(slots(&header(&[0, 1], &clevis(0, "tpm2", 1))), [1]);
        assert!(slots(&header(&[0, 1], &clevis(0, "tang", 1))).is_empty(), "a network key is not the chip");
        assert!(slots(&header(&[0], "")).is_empty(), "no tokens");
        assert!(slots(&header(&[0], r#""0":{"type":"systemd-recovery","keyslots":["1"]}"#)).is_empty());
        assert!(slots(&header(&[0], r#""0":{"type":"clevis","jwe":{"protected":"not base64!"}}"#)).is_empty());
        let odd = clevis(0, "tpm2", 1).replace(r#"["1"]"#, r#"["99","x",3]"#);
        assert!(slots(&header(&[0], &odd)).is_empty(), "a slot LUKS2 cannot have is not one");
    }

    #[test]
    fn anything_but_a_sound_luks2_header_is_no_header() {
        let good = header(&[0, 1], &clevis(0, "tpm2", 1));
        assert_eq!(read_header(&good[..]).map(|h| h.uuid), Some(UUID.to_string()));
        let mut luks1 = good.clone();
        luks1[6..8].copy_from_slice(&1u16.to_be_bytes());
        assert!(read_header(&luks1[..]).is_none());
        let mut huge = good.clone();
        huge[8..16].copy_from_slice(&u64::MAX.to_be_bytes());
        assert!(read_header(&huge[..]).is_none(), "a header claiming to be huge is not read");
        assert!(read_header(&good[..100]).is_none(), "cut short");
        assert!(read_header(&vec![0u8; 8192][..]).is_none(), "not LUKS at all");
    }

    #[test]
    fn the_free_slot_is_the_lowest_unused() {
        let json = |d: Vec<u8>| read_header(&d[..]).unwrap().json;
        assert_eq!(free_slot(&json(header(&[0, 1], ""))), Some(2));
        assert_eq!(free_slot(&json(header(&[1], ""))), Some(0));
        assert_eq!(free_slot(&json(header(&(0..32).collect::<Vec<_>>(), ""))), None);
        assert_eq!(free_slot("not json"), None);
    }

    #[test]
    fn only_a_plain_uuid_is_one() {
        assert!(valid_uuid(UUID));
        for bad in ["", "../../etc/passwd", &UUID.to_uppercase(), &UUID.replace('-', "_"), &format!("{UUID}0"), &UUID[1..]] {
            assert!(!valid_uuid(bad), "{bad}");
        }
    }

    #[test]
    fn a_note_counts_only_as_a_plain_file_holding_a_uuid() {
        let dir = std::env::temp_dir().join(format!("trindle-chip-note-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let note = dir.join("note");
        let me = me();
        assert_eq!(refused_disk(&note, me), None, "none there");
        std::fs::write(&note, format!("{UUID}\n")).unwrap();
        assert_eq!(refused_disk(&note, me).as_deref(), Some(UUID));
        assert_eq!(refused_disk(&note, me + 1), None, "only the owner's note counts");
        std::fs::write(&note, "rubbish").unwrap();
        assert_eq!(refused_disk(&note, me), None);
        std::fs::remove_file(&note).unwrap();
        std::fs::write(dir.join("real"), format!("{UUID}\n")).unwrap();
        std::os::unix::fs::symlink(dir.join("real"), &note).unwrap();
        assert_eq!(refused_disk(&note, me), None, "a link is not followed");
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn me() -> u32 {
        // SAFETY: reads this process's own user id; cannot fail.
        unsafe { libc::geteuid() }
    }

    #[test]
    fn base64url_reads_what_jose_writes() {
        for text in [&b""[..], b"a", b"ab", b"abc", b"{\"clevis\":{\"pin\":\"tpm2\"}}\xff\xfe"] {
            assert_eq!(base64url(&base64(text)).as_deref(), Some(text));
        }
        assert_eq!(base64url("a+b/"), None, "plain base64's letters are not base64url's");
    }
}
