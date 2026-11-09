//! Where an AppImage comes from, said as a hint and never as a check, and kept beside its box once it is added.

use std::path::{Path, PathBuf};

use super::github::{self, Found};
use super::loose;
use crate::toolkit::t;

#[derive(Clone, Debug, PartialEq)]
pub enum CameFrom {

    GitHub(String),

    Site(String),
    NotKnown,
}

impl CameFrom {
    pub fn of(f: &Found) -> CameFrom {
        if !f.repo.is_empty() {
            CameFrom::GitHub(f.repo.clone())
        } else if !f.site.is_empty() {
            CameFrom::Site(f.site.clone())
        } else {
            CameFrom::NotKnown
        }
    }

    pub fn words(&self, stars: Option<u64>) -> String {
        match self {
            CameFrom::GitHub(repo) => match stars {
                Some(n) => t!("store-from-github-stars", repo = repo.as_str(), stars = crate::common::thousands(n)),
                None => t!("store-from-github", repo = repo.as_str()),
            },
            CameFrom::Site(url) => t!("store-from-site", site = super::origin::host_of(url).unwrap_or_else(|| url.clone())),
            CameFrom::NotKnown => t!("store-from-unknown").into(),
        }
    }

    pub fn link(&self) -> Option<(String, &'static str)> {
        match self {
            CameFrom::GitHub(repo) => Some((format!("https://github.com/{repo}"), t!("store-gh-open"))),
            CameFrom::Site(url) => Some((url.clone(), t!("store-from-site-open"))),
            CameFrom::NotKnown => None,
        }
    }
}

fn file(top: &Path, name: &str) -> PathBuf {
    top.join(format!("appimage-{}", loose::id_for(crate::common::apps::appimage_stem(name)))).join("from")
}

pub fn keep_in(top: &Path, name: &str, from: &CameFrom) -> std::io::Result<()> {
    let at = file(top, name);
    let line = match from {
        CameFrom::GitHub(repo) => format!("github {repo}\n"),
        CameFrom::Site(url) => format!("site {url}\n"),
        CameFrom::NotKnown => return std::fs::remove_file(&at).or_else(|e| if e.kind() == std::io::ErrorKind::NotFound { Ok(()) } else { Err(e) }),
    };
    std::fs::create_dir_all(at.parent().unwrap_or(top))?;
    crate::common::files::write_text(&at, &line)
}

pub fn read_in(top: &Path, name: &str) -> CameFrom {
    let text = std::fs::read_to_string(file(top, name)).unwrap_or_default();
    match text.trim().split_once(' ') {
        Some(("github", repo)) if github::repo_of_url(&format!("https://github.com/{repo}")).as_deref() == Some(repo) => CameFrom::GitHub(repo.into()),
        Some(("site", url)) if url.starts_with("https://") && crate::toolkit::link::is_web(url) => CameFrom::Site(url.into()),
        _ => CameFrom::NotKnown,
    }
}

pub fn keep(name: &str, from: &CameFrom) -> std::io::Result<()> {
    keep_in(&loose::root(), name, from)
}

pub fn read(name: &str) -> CameFrom {
    read_in(&loose::root(), name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn where_an_appimage_came_from_is_kept_and_read_back() {
        let top = std::env::temp_dir().join(format!("trindle-came-from-{}", std::process::id()));
        let name = "Example-1.2-x86_64.AppImage";
        assert_eq!(read_in(&top, name), CameFrom::NotKnown);
        keep_in(&top, name, &CameFrom::GitHub("john-smith/example".into())).unwrap();
        assert_eq!(read_in(&top, "Example-1.3-x86_64.AppImage"), CameFrom::GitHub("john-smith/example".into()), "an update keeps it");
        keep_in(&top, name, &CameFrom::Site("https://example.org/get".into())).unwrap();
        assert_eq!(read_in(&top, name), CameFrom::Site("https://example.org/get".into()));
        keep_in(&top, name, &CameFrom::NotKnown).unwrap();
        assert_eq!(read_in(&top, name), CameFrom::NotKnown, "one added by hand says so");
        keep_in(&top, name, &CameFrom::NotKnown).unwrap();
        for bad in ["github ../x", "site http://example.org", "site javascript:x", "nothing"] {
            std::fs::write(file(&top, name), bad).unwrap();
            assert_eq!(read_in(&top, name), CameFrom::NotKnown, "{bad}");
        }
        let _ = std::fs::remove_dir_all(&top);
        let f = Found { site: "https://example.org/get".into(), ..Default::default() };
        assert_eq!(CameFrom::of(&f).words(None), "Not on GitHub: from a website, example.org");
        assert_eq!(CameFrom::GitHub("a/b".into()).words(Some(1200)), "On GitHub: a/b, 1,200 stars");
        assert_eq!(CameFrom::NotKnown.link(), None);
    }
}
