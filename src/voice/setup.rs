//! Adding and removing voice: builds whisper.cpp from a pinned, checksummed
//! release and fetches the model; temporary build tools are removed
//! afterwards. Settings runs it through trindle-admin (pkexec), so the
//! password is asked in the usual box and nothing opens a terminal;
//! `trindle-voice-setup install | remove` does the same by hand, using sudo.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::toolkit::t;
use super::{MODEL, WHISPER};

const WHISPER_VERSION: &str = "1.9.4";
const WHISPER_SHA256: &str = "57e280cee375ab02425b806ad5146b99f6eb9357e3c2b31357c8a6af2e2e44ae";
const MODEL_URL: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin";
const MODEL_SHA256: &str = "a03779c86df3323075f5e796cb2ce5029f00ec8869eee3fdfb897afe36c6d002";
const BUILD_TOOLS: &[&str] = &["cmake", "g++", "make", "curl"];
/// Beside the engine: the build tools brought in for voice that would not go
/// again, one per line. Only names in BUILD_TOOLS are ever acted on.
pub const LEFT_FILE: &str = "/usr/local/lib/trindle-whisper/build-tools-left";

fn run(program: &str, args: &[&str]) -> Result<(), String> {
    let status = crate::common::gentle::program(&mut Command::new(program)).args(args).status().map_err(|e| format!("{program}: {e}"))?;
    if status.success() { Ok(()) } else { Err(format!("{program} {} did not work", args.first().unwrap_or(&""))) }
}

fn run_in(dir: &Path, program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program).args(args).current_dir(dir).status().map_err(|e| format!("{program}: {e}"))?;
    if status.success() { Ok(()) } else { Err(format!("{program} did not work")) }
}

fn sha256(path: &Path) -> String {
    Command::new("sha256sum").arg(path).output().ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).split_whitespace().next().unwrap_or("").to_string())
        .unwrap_or_default()
}

/// As the system: directly when already running as it, through sudo when not.
fn as_root(program: &str, args: &[&str]) -> Result<(), String> {
    // SAFETY: asks the kernel for this process's own effective user id.
    if unsafe { libc::geteuid() } == 0 {
        run(program, args)
    } else {
        let mut all = vec![program];
        all.extend_from_slice(args);
        run("sudo", &all)
    }
}

fn installed_package(p: &str) -> bool {
    Command::new("dpkg").args(["-s", p]).stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null()).status().is_ok_and(|s| s.success())
}

fn temp_dir() -> Result<PathBuf, String> {
    crate::common::files::private_temp("trindle-voice").map_err(|e| e.to_string())
}

fn build_whisper() -> Result<(), String> {
    let lib = Path::new(WHISPER).parent().unwrap_or(Path::new("/"));
    if std::fs::read_to_string(lib.join(".version")).is_ok_and(|v| v.trim() == WHISPER_VERSION) {
        return Ok(());
    }
    let tmp = temp_dir()?;
    let built = build_in(&tmp, lib);
    // Only the building's own leavings, in the system's temporary folder.
    let _ = std::fs::remove_dir_all(&tmp);
    built
}

fn build_in(tmp: &Path, lib: &Path) -> Result<(), String> {
    let tarball = tmp.join("whisper.tar.gz");
    let url = format!("https://github.com/ggml-org/whisper.cpp/archive/refs/tags/v{WHISPER_VERSION}.tar.gz");
    run("curl", &["-fsSL", "--proto", "=https", "--proto-redir", "=https", "--max-filesize", "4g", "-o", &tarball.to_string_lossy(), &url])?;
    if sha256(&tarball) != WHISPER_SHA256 {
        return Err("the whisper.cpp download does not match its fingerprint; not building it".into());
    }
    run("tar", &["-xzf", &tarball.to_string_lossy(), "-C", &tmp.to_string_lossy()])?;
    let src = tmp.join(format!("whisper.cpp-{WHISPER_VERSION}"));
    run_in(&src, "cmake", &["-B", "build", "-DCMAKE_BUILD_TYPE=Release", "-DBUILD_SHARED_LIBS=OFF",
                            "-DWHISPER_BUILD_TESTS=OFF", "-DWHISPER_BUILD_SERVER=OFF"])?;
    let jobs = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(2).to_string();
    run_in(&src, "cmake", &["--build", "build", "-j", &jobs, "--target", "whisper-cli"])?;
    as_root("install", &["-Dm755", &src.join("build/bin/whisper-cli").to_string_lossy(), WHISPER])?;
    let version = tmp.join(".version");
    std::fs::write(&version, format!("{WHISPER_VERSION}\n")).map_err(|e| e.to_string())?;
    as_root("install", &["-m644", &version.to_string_lossy(), &lib.join(".version").to_string_lossy()])?;
    Ok(())
}

fn fetch_model() -> Result<(), String> {
    if sha256(Path::new(MODEL)) == MODEL_SHA256 {
        return Ok(());
    }
    let tmp = temp_dir()?;
    let file = tmp.join("model.bin");
    let got = run("curl", &["-fsSL", "--proto", "=https", "--proto-redir", "=https", "--max-filesize", "4g", "-o", &file.to_string_lossy(), MODEL_URL])
        .and_then(|()| if sha256(&file) == MODEL_SHA256 { Ok(()) } else {
            Err("the speech model does not match its fingerprint; not installing it".to_string())
        })
        .and_then(|()| as_root("install", &["-Dm644", &file.to_string_lossy(), MODEL]));
    // Only the download, in the system's temporary folder.
    let _ = std::fs::remove_dir_all(&tmp);
    got
}

pub fn install() -> Result<(), String> {
    std::env::set_var("DEBIAN_FRONTEND", "noninteractive");
    // Install missing build tools only for the duration of the build.
    let added: Vec<&str> = BUILD_TOOLS.iter().copied().filter(|p| !installed_package(p)).collect();
    // Each part as a step, for the ring in Settings that fills as it works.
    let of = if added.is_empty() { 2 } else { 4 };
    let step = |n: usize, words: &str| println!("{}", crate::common::filling::step_line(n, of, words));
    let first = if added.is_empty() { 1 } else { 2 };
    if !added.is_empty() {
        step(1, &t!("voice-installing-tools", names = added.join(" ")));
        let mut args = vec!["install", "-y", "--no-install-recommends"];
        args.extend(&added);
        as_root("apt-get", &args)?;
    }
    step(first, &t!("voice-building", version = WHISPER_VERSION));
    let result = build_whisper().and_then(|_| {
        step(first + 1, t!("voice-fetching-model"));
        fetch_model()
    });
    let mut tools_gone = Ok(());
    if !added.is_empty() {
        step(4, t!("voice-removing-tools"));
        let mut args = vec!["remove", "-y"];
        args.extend(&added);
        tools_gone = as_root("apt-get", &args);
    }
    let left = ended(result, tools_gone, &added);
    let left = left?;
    if left.is_empty() {
        // An old note that cannot be taken away is read again against what is installed.
        let _ = std::fs::remove_file(LEFT_FILE);
    } else if let Err(e) = crate::common::files::write_whole(Path::new(LEFT_FILE), left.join("\n").as_bytes()) {
        // Voice works, so this is no failure: the tools are kept in a note,
        // the person is shown a plain line with a button, and the log has why.
        eprintln!("trindle-voice: the note of the build tools left could not be written: {e}");
    }
    println!("{}", t!("voice-ready"));
    Ok(())
}

/// The build tools noted as left behind by adding voice, only those that
/// are still installed and that voice could have brought in.
pub fn tools_left() -> Vec<String> {
    let noted = std::fs::read_to_string(LEFT_FILE).unwrap_or_default();
    left_from(&noted, installed_package)
}

fn left_from(noted: &str, still: impl Fn(&str) -> bool) -> Vec<String> {
    noted.lines().map(str::trim).filter(|l| BUILD_TOOLS.contains(l) && still(l)).map(String::from).collect()
}

/// Takes the left-behind build tools away. Quiet when there are none.
pub fn remove_tools() -> Result<(), String> {
    let left = tools_left();
    if !left.is_empty() {
        let mut args = vec!["remove", "-y"];
        args.extend(left.iter().map(String::as_str));
        as_root("apt-get", &args)?;
    }
    // The note is read again against what is installed, so one left behind harms nothing.
    let _ = std::fs::remove_file(LEFT_FILE);
    Ok(())
}

/// How adding voice ended. Voice being ready is the whole of what matters:
/// build tools that would not go again harm nothing, so they are logged and
/// returned as the list left, and never make the ending a failure.
fn ended(built: Result<(), String>, tools_gone: Result<(), String>, tools: &[&str]) -> Result<Vec<String>, String> {
    built?;
    match tools_gone {
        Ok(()) => Ok(Vec::new()),
        Err(e) => {
            eprintln!("trindle-voice: the build tools ({}) could not be removed: {e}", tools.join(" "));
            Ok(tools.iter().map(|t| t.to_string()).collect())
        }
    }
}

pub fn remove() -> Result<(), String> {
    // Tried first, since the note goes with the engine; a failure is not said.
    if let Err(e) = remove_tools() {
        eprintln!("trindle-voice: the build tools could not be removed: {e}");
    }
    let lib = Path::new(WHISPER).parent().unwrap_or(Path::new("/nonexistent"));
    let model = Path::new(MODEL).parent().unwrap_or(Path::new("/nonexistent"));
    as_root("rm", &["-rf", &lib.to_string_lossy(), &model.to_string_lossy()])?;
    println!("{}", t!("voice-removed"));
    Ok(())
}

pub fn main(args: Vec<String>) -> i32 {
    let outcome = match args.get(1).map(String::as_str) {
        Some("install") => install(),
        Some("remove") => remove(),
        _ => Err(t!("voice-usage").into()),
    };
    let code = match outcome {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("trindle-voice: {e}");
            1
        }
    };
    // Keep the terminal window open for reading.
    if args.iter().any(|a| a == "--pause") {
        print!("\n{} ", t!("voice-press-enter"));
        // Only the prompt, and the wait for a key that keeps it in view.
        let _ = std::io::Write::flush(&mut std::io::stdout());
        // As above.
        let _ = std::io::stdin().read_line(&mut String::new());
    }
    code
}

#[cfg(test)]
mod tests {
    use super::{ended, left_from};

    #[test]
    fn build_tools_that_would_not_go_are_no_failure() {
        let left = ended(Ok(()), Err("apt-get remove did not work".into()), &["cmake", "g++"]).unwrap();
        assert_eq!(left, vec!["cmake", "g++"]);
        assert_eq!(ended(Ok(()), Ok(()), &[]), Ok(Vec::new()));
        // What stopped voice itself is what is said.
        assert_eq!(ended(Err("no".into()), Err("nor that".into()), &["cmake"]), Err("no".into()));
    }

    #[test]
    fn only_voices_own_build_tools_are_ever_taken() {
        let noted = "cmake\nopenssh-server\n g++ \nmake\n";
        assert_eq!(left_from(noted, |_| true), vec!["cmake", "g++", "make"]);
        assert_eq!(left_from(noted, |t| t == "make"), vec!["make"]);
        assert!(left_from("", |_| true).is_empty());
    }
}
