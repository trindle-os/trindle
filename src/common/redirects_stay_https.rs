//! Every curl that follows redirects (`-L`) is told to follow only https ones (`--proto-redir =https`), so a redirect cannot drop a download to plain http.

fn follows_without_https_only(call: &str) -> bool {
    let follows = call.split('"').any(|t| t == "--location" || (t.starts_with('-') && !t.starts_with("--") && t.contains('L')));
    call.contains("curl") && follows && !call.contains("--proto-redir")
}

fn calls(text: &str) -> Vec<(usize, String)> {
    let lines: Vec<&str> = text.lines().collect();
    (0..lines.len()).filter(|&i| lines[i].contains("curl")).map(|i| {
        let end = (i..lines.len().min(i + 8)).find(|&j| lines[j].contains(';')).unwrap_or(lines.len().min(i + 8) - 1);
        (i + 1, lines[i..=end].join("\n"))
    }).collect()
}

fn scan(dir: &std::path::Path, found: &mut Vec<String>) {
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan(&path, found);
        } else if path.extension().is_some_and(|e| e == "rs") && path.file_name().is_some_and(|n| n != "redirects_stay_https.rs") {
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            for (line, call) in calls(&text) {
                if follows_without_https_only(&call) {
                    found.push(format!("{}:{}", path.display(), line));
                }
            }
        }
    }
}

#[test]
fn every_curl_that_follows_redirects_follows_only_https() {
    assert!(follows_without_https_only(r#"capture(&["curl", "-fsSL", "--proto", "=https", url])"#));
    assert!(!follows_without_https_only(r#"capture(&["curl", "-fsSL", "--proto", "=https", "--proto-redir", "=https", url])"#));
    assert!(!follows_without_https_only(r#"run(&["mkfs.ext4", "-F", "-L", "trindle-start", boot])"#));
    let split = "let out = Command::new(\"curl\")\n    .args([\"-fsSL\", \"--proto\", \"=https\", url])\n    .output();";
    assert!(calls(split).iter().any(|(_, c)| follows_without_https_only(c)));
    let mut found = Vec::new();
    scan(&std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut found);
    assert!(found.is_empty(), "curl follows redirects to plain http here: {found:?}");
}
