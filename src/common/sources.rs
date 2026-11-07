//! Third-party apt repositories for software not on Flathub.

use std::path::PathBuf;

pub struct Known {
    pub id: &'static str,
    pub name: &'static str,
    pub what: &'static str,
    pub icon: &'static str,
    pub uri: &'static str,
    pub suite: &'static str,
    pub components: &'static str,
    pub key: &'static str,
    pub archs: &'static [&'static str],
    pub packages: &'static [&'static str],
    pub vendor_files: &'static [&'static str],
    pub page: Option<&'static str>,
}

pub const KNOWN: &[Known] = &[
    Known {
        id: "claude", name: "Claude", what: "Anthropic's Claude app: chat, Cowork and Claude Code",
        icon: "claude-desktop", uri: "https://downloads.claude.ai/claude-desktop/apt/stable", suite: "stable",
        components: "main", key: "https://downloads.claude.ai/claude-desktop/key.asc", archs: &["amd64", "arm64"],
        packages: &["claude-desktop"], vendor_files: &["claude-desktop.list", "claude-desktop.sources"], page: None,
    },
    Known {
        id: "tailscale", name: "Tailscale", what: "A private network between your own devices",
        icon: "tailscale", uri: "https://pkgs.tailscale.com/stable/debian", suite: "trixie", components: "main",
        key: "https://pkgs.tailscale.com/stable/debian/trixie.noarmor.gpg", archs: &["amd64", "arm64"],
        packages: &["tailscale"], vendor_files: &["tailscale.list", "tailscale.sources"], page: None,
    },
    Known {
        id: "syncthing", name: "Syncthing", what: "Keeps folders the same on your computers, with no cloud; Debian's is older",
        icon: "syncthing", uri: "https://apt.syncthing.net/", suite: "syncthing", components: "stable-v2",
        key: "https://syncthing.net/release-key.gpg", archs: &["amd64", "arm64"],
        packages: &["syncthing"], vendor_files: &["syncthing.list", "syncthing.sources"],
        page: Some("http://127.0.0.1:8384"),
    },
    Known {
        id: "jellyfin", name: "Jellyfin", what: "Your own film, music and photo server, for any screen in the house",
        icon: "jellyfin", uri: "https://repo.jellyfin.org/debian", suite: "trixie", components: "main",
        key: "https://repo.jellyfin.org/jellyfin_team.gpg.key", archs: &["amd64", "arm64"],
        packages: &["jellyfin"], vendor_files: &["jellyfin.list", "jellyfin.sources"],
        page: Some("http://127.0.0.1:8096"),
    },
    Known {
        id: "nodejs", name: "Node.js 24", what: "The current long-term Node.js, for developers; Debian's is version 20",
        icon: "nodejs", uri: "https://deb.nodesource.com/node_24.x", suite: "nodistro", components: "main",
        key: "https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key", archs: &["amd64", "arm64"],
        packages: &["nodejs"], vendor_files: &["nodesource.list", "nodesource.sources"], page: None,
    },
    Known {
        id: "github-cli", name: "GitHub CLI", what: "GitHub from the Terminal, for developers; Debian's is older",
        icon: "github", uri: "https://cli.github.com/packages", suite: "stable", components: "main",
        key: "https://cli.github.com/packages/githubcli-archive-keyring.gpg", archs: &["amd64", "arm64"],
        packages: &["gh"], vendor_files: &["github-cli.list", "github-cli.sources"], page: None,
    },
    Known {
        id: "docker", name: "Docker", what: "Runs programs in containers; for developers",
        icon: "docker", uri: "https://download.docker.com/linux/debian", suite: "trixie", components: "stable",
        key: "https://download.docker.com/linux/debian/gpg", archs: &["amd64", "arm64"],
        packages: &["docker-ce", "docker-ce-cli", "containerd.io", "docker-buildx-plugin", "docker-compose-plugin"],
        vendor_files: &["docker.list", "docker.sources"], page: None,
    },
];

pub const LISTS: &str = "/etc/apt/sources.list.d";
pub const KEYRINGS: &str = "/etc/apt/keyrings";

pub fn known(id: &str) -> Option<&'static Known> {
    KNOWN.iter().find(|k| k.id == id)
}

pub fn file_for(id: &str) -> PathBuf {
    PathBuf::from(LISTS).join(format!("trindle-{id}.sources"))
}

pub fn arch() -> String {
    std::process::Command::new("dpkg").arg("--print-architecture").output().ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string()).filter(|s| !s.is_empty())
        .unwrap_or_else(|| "amd64".into())
}

pub fn custom_id(uri: &str, suite: &str) -> String {
    let host = uri.trim_start_matches("https://").split('/').next().unwrap_or("source");
    let raw = format!("custom-{host}-{suite}");
    let mut id: String = raw.chars().map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '-' }).collect();
    while id.contains("--") {
        id = id.replace("--", "-");
    }
    id.trim_matches('-').chars().take(60).collect()
}

pub fn valid_uri(u: &str) -> bool {
    u.starts_with("https://") && u.len() > 8 && u.len() < 300
        && u.chars().all(|c| c.is_ascii_alphanumeric() || "-._~:/?#[]@!$&'()*+,;=%".contains(c))
        && !u.contains(['\'', ';', '$'])
}

pub fn valid_words(s: &str) -> bool {
    !s.trim().is_empty() && s.len() < 120
        && s.split_whitespace().all(|w| w.chars().all(|c| c.is_ascii_alphanumeric() || "._/-".contains(c)) && !w.starts_with('-'))
}

pub fn valid_package(p: &str) -> bool {
    !p.is_empty() && p.len() < 100 && p.chars().next().is_some_and(|c| c.is_ascii_alphanumeric())
        && p.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || "+-.".contains(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_sources_are_well_formed() {
        for k in KNOWN {
            assert!(valid_uri(k.uri) && valid_uri(k.key), "{}", k.id);
            assert!(valid_words(k.suite) && valid_words(k.components), "{}", k.id);
            assert!(k.packages.iter().all(|p| valid_package(p)), "{}", k.id);
        }
    }

    #[test]
    fn custom_sources_are_checked() {
        assert_eq!(custom_id("https://apt.example.org/deb/", "stable"), "custom-apt-example-org-stable");
        assert!(!valid_uri("http://plain.example.org"));
        assert!(!valid_uri("https://x.org/$(rm -rf)"));
        assert!(!valid_words("stable --option"));
        assert!(!valid_package("-rf"));
    }
}
