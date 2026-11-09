//! The programs people use to read or hear the screen, which tidying never
//! takes away. The list is `system/protected-packages.txt`, the same file the
//! package's postinst reads, so there is one list.

use std::process::Command;

use super::Outcome;

const LIST: &str = include_str!("../../system/protected-packages.txt");

pub fn names() -> Vec<&'static str> {
    LIST.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).collect()
}

fn regex(name: &str) -> String {
    let body: String = name.chars().map(|c| match c {
        '*' => ".*".to_string(),
        '.' | '+' => format!("\\{c}"),
        c => c.to_string(),
    }).collect();
    format!("^{body}$")
}

pub fn never_autoremove() -> Vec<String> {
    names().into_iter().flat_map(|n| ["-o".to_string(), format!("APT::NeverAutoRemove::={}", regex(n))]).collect()
}

pub fn would_go(simulation: &str) -> Vec<String> {
    simulation.lines().filter_map(|l| l.strip_prefix(concat!("Re", "mv "))).flat_map(|l| l.split_whitespace().take(1))
        .flat_map(|n| n.split(':').take(1)).map(String::from).collect()
}

fn installed() -> Result<Vec<String>, String> {
    installed_among(&names())
}

pub fn installed_among(patterns: &[&str]) -> Result<Vec<String>, String> {
    let mut argv = vec!["dpkg-query".to_string(), "-W".into(), "-f=${db:Status-Abbrev}\t${binary:Package}\n".into()];
    argv.extend(patterns.iter().map(|n| n.to_string()));
    let refs: Vec<&str> = argv.iter().map(String::as_str).collect();

    let said = capture(&refs)?;
    Ok(said.lines().filter_map(|l| l.split_once('\t')).filter(|(s, _)| s.starts_with("ii"))
        .flat_map(|(_, n)| n.split(':').take(1)).map(String::from).collect())
}

pub fn capture(argv: &[&str]) -> Result<String, String> {
    #[cfg(test)]
    if let Some(answer) = super::stand_in::run(argv) {
        return answer.map(|_| String::new());
    }
    let out = Command::new(argv[0]).args(&argv[1..]).output().map_err(|e| e.to_string())?;
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

pub fn mark_chosen() -> Outcome {
    let have = installed()?;
    if have.is_empty() {
        return Ok(());
    }
    let mut argv = vec!["apt-mark", "manual"];
    argv.extend(have.iter().map(String::as_str));
    super::run(&argv, None)
}

pub fn autoremove_argv(simulate: bool) -> Vec<String> {
    let mut argv: Vec<String> = ["apt-get", "-q", "-y"].map(String::from).into();
    if simulate {
        argv.push("-s".into());
    }
    argv.extend(never_autoremove());
    argv.push("autoremove".into());
    argv
}

#[cfg(test)]
pub fn covers(package: &str) -> bool {
    names().iter().any(|n| match n.strip_suffix('*') {
        Some(stem) => package.starts_with(stem),
        None => package == *n,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_command_for_an_unattended_tidy_protects_the_whole_stack() {
        let line = autoremove_argv(false).join(" ");
        let (reader, speech, voice) = (["or", "ca"].concat(), ["speech-", "dispatcher"].concat(), ["e", "speak-ng"].concat());
        for p in [reader.as_str(), speech.as_str(), voice.as_str(), "python3-speechd", "python3-brlapi", "liblouis20",
                  "gir1.2-gtk-3.0", "gir1.2-atspi-2.0", "gir1.2-atk-1.0", "gir1.2-wnck-3.0", "gir1.2-gstreamer-1.0", "xkbset"] {
            assert!(covers(p), "{p} is not protected");
        }
        assert!(line.contains(&format!("APT::NeverAutoRemove::=^{reader}$")) && line.contains("^liblouis.*$"), "{line}");
        assert!(line.ends_with("autoremove") && line.contains("-o"), "an autoremove with no protection: {line}");
        assert!(!line.contains("--purge"));
    }

    #[test]
    fn nothing_else_runs_an_unprotected_autoremove() {
        let src = ["mod.rs", "accounts.rs", "dns.rs", "firewall.rs", "machine.rs", "packages.rs", "sources.rs", "tailscale.rs", "updates.rs"];
        for f in src {
            let text = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/admin").join(f)).unwrap();
            for (n, l) in text.lines().enumerate() {
                let code = l.split("//").next().unwrap_or("");
                assert!(!code.contains("\"autoremove\"") || code.contains("protected"), "{f}:{}: {l}", n + 1);
            }
        }
    }

    #[test]
    fn what_the_simulation_would_remove_is_read() {
        let said = "Remv xkbset [1.0]\nRemv liblouis20:amd64 [3.33]\nInst foo\n";
        assert_eq!(would_go(said), ["xkbset", "liblouis20"]);
    }

    #[test]
    fn a_tidy_marks_the_stack_chosen_then_removes_with_protection() {
        let s = crate::admin::stand_in::StandIn::all_well("protected-tidy");
        crate::admin::packages::tidy().unwrap();
        let ran = s.ran();
        assert!(ran.iter().any(|c| c.starts_with("dpkg-query")), "{ran:?}");
        let auto: Vec<_> = ran.iter().filter(|c| c.contains("autoremove")).collect();
        assert!(!auto.is_empty() && auto.iter().all(|c| c.contains("NeverAutoRemove")), "{ran:?}");
    }
}
