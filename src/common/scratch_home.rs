//! Tests that write to the person's folders run again in a child whose home is a scratch folder.

use std::path::PathBuf;
use std::process::Command;

const SET: &str = "TRINDLE_SCRATCH_HOME";

pub fn inside() -> Option<PathBuf> {
    std::env::var_os(SET).map(PathBuf::from)
}

pub fn rerun(module: &str, name: &str) {
    let scratch = std::env::temp_dir().join(format!("trindle-scratch-{}-{name}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    let home = scratch.join("home");
    std::fs::create_dir_all(&home).unwrap();
    let path = format!("{}::{name}", module.split_once("::").map_or(module, |(_, rest)| rest));
    let out = Command::new(std::env::current_exe().unwrap()).args(["--exact", &path, "--nocapture"])
        .env(SET, &home).env("HOME", &home).env("XDG_DATA_HOME", home.join("data"))
        .env("XDG_CONFIG_HOME", home.join("config")).env("XDG_CACHE_HOME", home.join("cache")).output().unwrap();
    let _ = std::fs::remove_dir_all(&scratch);
    let said = format!("{}{}", String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert!(out.status.success() && said.contains("1 passed"), "{said}");
}

pub fn files_under(top: &std::path::Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    for e in std::fs::read_dir(top).into_iter().flatten().flatten() {
        let p = e.path();
        if e.file_type().is_ok_and(|t| t.is_dir()) { found.extend(files_under(&p)) } else { found.push(p) }
    }
    found
}
