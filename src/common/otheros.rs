//! Windows in the boot menu, so a computer with both can start either.
//! os-prober misses a boot partition that is already mounted, which is how
//! Trindle keeps it, so the entry is written from what is on that partition.

use std::path::Path;

use crate::common::run::text;

fn esp() -> Option<(String, String)> {
    let dev = text(&["findmnt", "-no", "SOURCE", "/boot/efi"]);
    if dev.is_empty() {
        return None;
    }
    let uuid = text(&["blkid", "-s", "UUID", "-o", "value", &dev]);
    (!uuid.is_empty()).then_some((dev, uuid))
}

const HIBERNATED: i32 = 14;

pub fn windows_asleep(dev: &str) -> bool {
    std::process::Command::new("ntfs-3g.probe").args(["--readwrite", dev]).output()
        .is_ok_and(|o| o.status.code() == Some(HIBERNATED))
}

pub fn grub_main() -> i32 {
    if !Path::new("/sys/firmware/efi").exists() {
        return 0;
    }
    let Some((_, uuid)) = esp() else { return 0 };
    let loader = "/boot/efi/EFI/Microsoft/Boot/bootmgfw.efi";
    if !Path::new(loader).is_file() {
        return 0;
    }
    println!("menuentry 'Windows' --class windows --class os {{");
    println!("    insmod part_gpt");
    println!("    insmod fat");
    println!("    insmod chain");
    println!("    search --no-floppy --fs-uuid --set=root {uuid}");
    println!("    chainloader /EFI/Microsoft/Boot/bootmgfw.efi");
    println!("}}");
    0
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    fn have(program: &str) -> bool {
        Command::new("sh").args(["-c", &format!("command -v {program}")]).output().is_ok_and(|o| o.status.success())
    }

    #[test]
    fn a_hibernated_windows_volume_is_found_and_a_clean_one_is_not() {
        if !["mkntfs", "ntfscp", "ntfs-3g.probe"].iter().all(|p| have(p)) {
            return;
        }
        let dir = std::env::temp_dir().join(format!("trindle-asleep-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let (image, file) = (dir.join("a.img"), dir.join("hiberfil.sys"));
        let (image_s, file_s) = (image.to_str().unwrap(), file.to_str().unwrap());
        std::fs::File::create(&image).unwrap().set_len(64 << 20).unwrap();
        assert!(Command::new("mkntfs").args(["-F", "-Q", "-q", image_s]).output().unwrap().status.success());
        assert!(!super::windows_asleep(image_s));
        let mut body = b"hibr".to_vec();
        body.resize(8192, 0);
        std::fs::write(&file, body).unwrap();
        assert!(Command::new("ntfscp").args(["-f", image_s, file_s, "hiberfil.sys"]).output().unwrap().status.success());
        assert!(super::windows_asleep(image_s));
        assert!(!super::windows_asleep("/nonexistent"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
