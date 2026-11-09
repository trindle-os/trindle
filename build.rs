//! Builds the list of English word files (share/lang/en/*.ftl) into the
//! program, so a new file needs no line of code anywhere.

use std::path::Path;

fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = Path::new(&root).join("share/lang/en");
    println!("cargo:rerun-if-changed=share/lang/en");
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .map(|r| r.flatten().filter_map(|e| e.file_name().into_string().ok()).filter(|n| n.ends_with(".ftl")).collect())
        .unwrap_or_default();
    names.sort();
    let mut out = String::from("&[\n");
    for n in &names {
        let path = dir.join(n);
        out.push_str(&format!("    ({n:?}, include_str!({:?})),\n", path.display().to_string()));
    }
    out.push_str("]\n");
    let target = Path::new(&std::env::var("OUT_DIR").unwrap()).join("english.rs");
    std::fs::write(target, out).unwrap();
}
