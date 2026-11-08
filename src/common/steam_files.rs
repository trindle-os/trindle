//! Steam's own files for the Games page: the games it has, a game's launch option and Windows layer, and a game's Windows setup folder. Written only while Steam is closed, which then does not overwrite them.

use std::path::{Path, PathBuf};

use super::files;

pub const STEAM: &str = "com.valvesoftware.Steam";

pub const GE: &str = "com.valvesoftware.Steam.CompatibilityTool.Proton-GE";
const PRIME: &str = "DRI_PRIME=1";
const COMMAND: &str = "%command%";

const NOT_GAMES: [&str; 3] = ["Proton", "Steam Linux Runtime", "Steamworks Common Redistributables"];

#[derive(Clone, Debug, PartialEq)]
pub enum Vdf {
    Word(String),
    Keys(Vec<(String, Vdf)>),
}

fn tokens(text: &str) -> Option<Vec<(bool, String)>> {
    let mut out = Vec::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '{' | '}' => out.push((true, c.to_string())),
            '"' => {
                let mut word = String::new();
                loop {
                    match chars.next()? {
                        '"' => break,
                        '\\' => word.push(match chars.next()? {
                            'n' => '\n',
                            't' => '\t',
                            other => other,
                        }),
                        other => word.push(other),
                    }
                }
                out.push((false, word));
            }
            '/' if chars.peek() == Some(&'/') => {
                while chars.next_if(|&c| c != '\n').is_some() {}
            }
            c if c.is_whitespace() => {}
            _ => return None,
        }
    }
    Some(out)
}

fn keys(tokens: &mut std::vec::IntoIter<(bool, String)>, depth: usize) -> Option<Vec<(String, Vdf)>> {
    let mut out = Vec::new();
    while let Some((brace, key)) = tokens.next() {
        if brace {
            return (key == "}" && depth > 0).then_some(out);
        }
        match tokens.next()? {
            (true, b) if b == "{" => {
                if depth > 64 {
                    return None;
                }
                out.push((key, Vdf::Keys(keys(tokens, depth + 1)?)));
            }
            (false, word) => out.push((key, Vdf::Word(word))),
            _ => return None,
        }
    }
    (depth == 0).then_some(out)
}

pub fn parse(text: &str) -> Option<Vec<(String, Vdf)>> {
    keys(&mut tokens(text)?.into_iter(), 0)
}

fn quoted(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\t', "\\t"))
}

fn put(out: &mut String, keys: &[(String, Vdf)], depth: usize) {
    let tabs = "\t".repeat(depth);
    for (k, v) in keys {
        match v {
            Vdf::Word(w) => out.push_str(&format!("{tabs}{}\t\t{}\n", quoted(k), quoted(w))),
            Vdf::Keys(inner) => {
                out.push_str(&format!("{tabs}{}\n{tabs}{{\n", quoted(k)));
                put(out, inner, depth + 1);
                out.push_str(&format!("{tabs}}}\n"));
            }
        }
    }
}

pub fn render(keys: &[(String, Vdf)]) -> String {
    let mut out = String::new();
    put(&mut out, keys, 0);
    out
}

fn inside<'a>(mut keys: &'a mut Vec<(String, Vdf)>, path: &[&str]) -> &'a mut Vec<(String, Vdf)> {
    for name in path {
        let at = match keys.iter().position(|(k, v)| k.eq_ignore_ascii_case(name) && matches!(v, Vdf::Keys(_))) {
            Some(at) => at,
            None => {
                keys.push((name.to_string(), Vdf::Keys(Vec::new())));
                keys.len() - 1
            }
        };
        let Vdf::Keys(inner) = &mut keys[at].1 else { unreachable!() };
        keys = inner;
    }
    keys
}

fn word<'a>(keys: &'a [(String, Vdf)], name: &str) -> Option<&'a str> {
    keys.iter().find_map(|(k, v)| match v {
        Vdf::Word(w) if k.eq_ignore_ascii_case(name) => Some(w.as_str()),
        _ => None,
    })
}

fn set_word(keys: &mut Vec<(String, Vdf)>, name: &str, value: Option<&str>) {
    keys.retain(|(k, v)| !(k.eq_ignore_ascii_case(name) && matches!(v, Vdf::Word(_))));
    if let Some(v) = value {
        keys.push((name.to_string(), Vdf::Word(v.to_string())));
    }
}

pub fn root() -> PathBuf {
    files::home().join(format!(".var/app/{STEAM}/.local/share/Steam"))
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub name: String,

    pub library: PathBuf,
}

pub fn libraries(root: &Path) -> Vec<PathBuf> {
    let own = root.join("steamapps");
    let mut all = vec![own.clone()];
    let listed = files::read_text(own.join("libraryfolders.vdf"));
    for (_, v) in parse(&listed).unwrap_or_default().iter().flat_map(|(_, v)| match v {
        Vdf::Keys(k) => k.iter().collect::<Vec<_>>(),
        _ => vec![],
    }) {
        if let Vdf::Keys(k) = v {
            if let Some(p) = word(k, "path").filter(|p| Path::new(p).is_absolute()) {
                let lib = Path::new(p).join("steamapps");
                if !all.contains(&lib) {
                    all.push(lib);
                }
            }
        }
    }
    all
}

pub fn games(libs: &[PathBuf]) -> Vec<Game> {
    let mut out: Vec<Game> = Vec::new();
    for lib in libs {
        let Ok(dir) = std::fs::read_dir(lib) else { continue };
        for e in dir.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            if !(name.starts_with("appmanifest_") && name.ends_with(".acf")) {
                continue;
            }
            let Some(parsed) = parse(&files::read_text(e.path())) else { continue };
            let Some((_, Vdf::Keys(k))) = parsed.iter().find(|(k, _)| k.eq_ignore_ascii_case("AppState")) else { continue };
            let (Some(id), Some(title)) = (word(k, "appid").and_then(|i| i.parse::<u32>().ok()), word(k, "name")) else { continue };
            if NOT_GAMES.iter().any(|n| title.starts_with(n)) || out.iter().any(|g| g.id == id) {
                continue;
            }
            out.push(Game { id, name: title.to_string(), library: lib.clone() });
        }
    }
    out.sort_by_key(|g| g.name.to_lowercase());
    out
}

pub fn prime_on(opts: &str) -> bool {
    opts.split_whitespace().any(|w| w == PRIME)
}

pub fn with_prime(opts: &str, on: bool) -> String {
    let rest: Vec<&str> = opts.split_whitespace().filter(|w| *w != PRIME).collect();
    if !on {
        return if rest == [COMMAND] { String::new() } else { rest.join(" ") };
    }
    match rest.iter().position(|w| *w == COMMAND) {
        Some(at) => {
            let mut all = rest.clone();
            all.insert(at, PRIME);
            all.join(" ")
        }
        None => format!("{PRIME} {COMMAND} {}", rest.join(" ")).trim_end().to_string(),
    }
}

pub fn local_configs(root: &Path) -> Vec<PathBuf> {
    let Ok(dir) = std::fs::read_dir(root.join("userdata")) else { return vec![] };
    let mut found: Vec<PathBuf> = dir.flatten()
        .filter(|e| e.file_name().to_string_lossy().parse::<u64>().is_ok_and(|n| n > 0))
        .map(|e| e.path().join("config/localconfig.vdf")).filter(|p| p.is_file()).collect();
    found.sort();
    found
}

const APPS: [&str; 4] = ["Software", "Valve", "Steam", "apps"];

fn store(file: &Path, top: &str) -> Option<Vec<(String, Vdf)>> {
    let text = files::read_text(file);
    if text.trim().is_empty() {
        return Some(vec![(top.to_string(), Vdf::Keys(Vec::new()))]);
    }
    parse(&text)
}

fn save(file: &Path, keys: &[(String, Vdf)]) -> Result<(), String> {
    files::write_whole(file, render(keys).as_bytes()).map_err(|e| e.to_string())
}

pub fn launch_options(file: &Path, id: u32) -> String {
    let Some(mut all) = store(file, "UserLocalConfigStore") else { return String::new() };
    let Some((_, Vdf::Keys(root))) = all.iter_mut().find(|(k, _)| k.eq_ignore_ascii_case("UserLocalConfigStore")) else { return String::new() };
    let game = inside(root, &[APPS[0], APPS[1], APPS[2], APPS[3], &id.to_string()]);
    word(game, "LaunchOptions").unwrap_or("").to_string()
}

pub fn set_launch_options(file: &Path, id: u32, opts: &str) -> Result<(), String> {
    let mut all = store(file, "UserLocalConfigStore").ok_or("The Steam file could not be read.")?;
    let Some((_, Vdf::Keys(root))) = all.iter_mut().find(|(k, _)| k.eq_ignore_ascii_case("UserLocalConfigStore")) else {
        return Err("The Steam file could not be read.".into());
    };
    let game = inside(root, &[APPS[0], APPS[1], APPS[2], APPS[3], &id.to_string()]);
    set_word(game, "LaunchOptions", (!opts.is_empty()).then_some(opts));
    save(file, &all)
}

pub fn prime_for(root: &Path, id: u32) -> bool {
    local_configs(root).iter().any(|f| prime_on(&launch_options(f, id)))
}

pub fn set_prime(root: &Path, id: u32, on: bool) -> Result<(), String> {
    for f in local_configs(root) {
        let now = launch_options(&f, id);
        if prime_on(&now) != on {
            set_launch_options(&f, id, &with_prime(&now, on))?;
        }
    }
    Ok(())
}

pub fn compat_tool(root: &Path, id: u32) -> Option<String> {
    let mut all = parse(&files::read_text(root.join("config/config.vdf")))?;
    let Some((_, Vdf::Keys(top))) = all.iter_mut().find(|(k, _)| k.eq_ignore_ascii_case("InstallConfigStore")) else { return None };
    let map = inside(top, &["Software", "Valve", "Steam", "CompatToolMapping"]);
    let Some((_, Vdf::Keys(one))) = map.iter().find(|(k, _)| *k == id.to_string()) else { return None };
    word(one, "name").filter(|n| !n.is_empty()).map(String::from)
}

pub fn set_compat_tool(root: &Path, id: u32, tool: Option<&str>) -> Result<(), String> {
    let file = root.join("config/config.vdf");
    let mut all = store(&file, "InstallConfigStore").ok_or("The Steam file could not be read.")?;
    let Some((_, Vdf::Keys(top))) = all.iter_mut().find(|(k, _)| k.eq_ignore_ascii_case("InstallConfigStore")) else {
        return Err("The Steam file could not be read.".into());
    };
    let map = inside(top, &["Software", "Valve", "Steam", "CompatToolMapping"]);
    map.retain(|(k, _)| *k != id.to_string());
    if let Some(name) = tool {
        map.push((id.to_string(), Vdf::Keys(vec![
            ("name".into(), Vdf::Word(name.into())),
            ("config".into(), Vdf::Word(String::new())),
            ("priority".into(), Vdf::Word("250".into())),
        ])));
    }
    save(&file, &all)
}

pub fn ge_name(location: &Path) -> Option<String> {
    let mut found = None;
    let mut dirs = vec![location.to_path_buf()];
    let mut seen = 0;
    while let Some(d) = dirs.pop() {
        seen += 1;
        if seen > 200 {
            break;
        }
        let Ok(read) = std::fs::read_dir(&d) else { continue };
        for e in read.flatten() {
            let p = e.path();
            if p.file_name().is_some_and(|n| n == "compatibilitytool.vdf") {
                let tools = parse(&files::read_text(&p))?;
                let Some((_, Vdf::Keys(top))) = tools.iter().find(|(k, _)| k == "compatibilitytools") else { continue };
                let Some((_, Vdf::Keys(list))) = top.iter().find(|(k, _)| k == "compat_tools") else { continue };
                found = list.iter().find(|(_, v)| matches!(v, Vdf::Keys(_))).map(|(k, _)| k.clone());
            } else if e.file_type().is_ok_and(|t| t.is_dir()) {
                dirs.push(p);
            }
        }
    }
    found
}

pub fn setup_folder(library: &Path, id: u32) -> PathBuf {
    library.join("compatdata").join(id.to_string())
}

pub fn game_running(proc: &Path, id: u32) -> bool {
    crate::common::pads::steam_games_now(proc).contains(&id)
}

pub fn two_chips(drm: &Path) -> bool {
    let Ok(dir) = std::fs::read_dir(drm) else { return false };
    dir.flatten().filter(|e| {
        let n = e.file_name().to_string_lossy().into_owned();
        n.strip_prefix("card").is_some_and(|d| !d.is_empty() && d.bytes().all(|b| b.is_ascii_digit()))
            && files::read_text(e.path().join("device/class")).trim().starts_with("0x03")
    }).count() > 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn folder(tag: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!("trindle-steam-files-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn put(path: &Path, text: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    const LOCAL: &str = "\"UserLocalConfigStore\"\n{\n\t\"Software\"\n\t{\n\t\t\"Valve\"\n\t\t{\n\t\t\t\"Steam\"\n\t\t\t{\n\t\t\t\t\"apps\"\n\t\t\t\t{\n\t\t\t\t\t\"289650\"\n\t\t\t\t\t{\n\t\t\t\t\t\t\"LaunchOptions\"\t\t\"-windowed\"\n\t\t\t\t\t\t\"Playtime\"\t\t\"12\"\n\t\t\t\t\t}\n\t\t\t\t}\n\t\t\t}\n\t\t}\n\t}\n}\n";

    #[test]
    fn a_file_is_read_and_written_back_the_same() {
        let keys = parse(LOCAL).unwrap();
        assert_eq!(render(&keys), LOCAL);
        assert_eq!(parse(&render(&keys)).unwrap(), keys);
        assert_eq!(render(&parse("\"a\" { \"b\" \"say \\\"hi\\\"\\\\\" } // note\n").unwrap()), "\"a\"\n{\n\t\"b\"\t\t\"say \\\"hi\\\"\\\\\"\n}\n");
    }

    #[test]
    fn a_broken_file_is_not_read_and_so_never_written_over() {
        for bad in ["\"a\" {", "\"a\" { \"b\" }", "}", "\"a", "a b"] {
            assert!(parse(bad).is_none(), "{bad}");
        }
        let root = folder("broken");
        let file = root.join("userdata/42/config/localconfig.vdf");
        put(&file, "\"UserLocalConfigStore\" {");
        assert!(set_prime(&root, 7, true).is_err());
        assert_eq!(std::fs::read_to_string(&file).unwrap(), "\"UserLocalConfigStore\" {");
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn the_stronger_graphics_go_in_front_of_the_command_and_keep_what_was_typed() {
        assert_eq!(with_prime("", true), "DRI_PRIME=1 %command%");
        assert_eq!(with_prime("-windowed", true), "DRI_PRIME=1 %command% -windowed");
        assert_eq!(with_prime("MANGOHUD=1 %command% -windowed", true), "MANGOHUD=1 DRI_PRIME=1 %command% -windowed");
        assert_eq!(with_prime("DRI_PRIME=1 %command%", false), "");
        assert_eq!(with_prime("MANGOHUD=1 DRI_PRIME=1 %command% -windowed", false), "MANGOHUD=1 %command% -windowed");
        assert_eq!(with_prime("DRI_PRIME=1 %command%", true), "DRI_PRIME=1 %command%");
        assert!(prime_on("DRI_PRIME=1 %command%") && !prime_on("DRI_PRIME=10 %command%") && !prime_on(""));
    }

    #[test]
    fn the_choice_is_written_for_the_game_and_nothing_else_in_the_file_moves() {
        let root = folder("prime");
        let file = root.join("userdata/42/config/localconfig.vdf");
        put(&file, LOCAL);
        put(&root.join("userdata/0/config/localconfig.vdf"), "\"UserLocalConfigStore\"\n{\n}\n");
        assert!(!prime_for(&root, 289650));
        set_prime(&root, 289650, true).unwrap();
        assert_eq!(launch_options(&file, 289650), "DRI_PRIME=1 %command% -windowed");
        assert!(prime_for(&root, 289650) && !prime_for(&root, 1));
        assert!(std::fs::read_to_string(&file).unwrap().contains("\"Playtime\"\t\t\"12\""));
        assert_eq!(std::fs::read_to_string(root.join("userdata/0/config/localconfig.vdf")).unwrap(), "\"UserLocalConfigStore\"\n{\n}\n");
        set_prime(&root, 289650, false).unwrap();
        assert_eq!(launch_options(&file, 289650), "%command% -windowed");
        set_prime(&root, 5, true).unwrap();
        set_prime(&root, 5, false).unwrap();
        assert!(!prime_for(&root, 5));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn a_game_is_given_the_community_build_or_steam_chooses_again() {
        let root = folder("tool");
        put(&root.join("config/config.vdf"), "\"InstallConfigStore\"\n{\n\t\"Software\"\n\t{\n\t\t\"Valve\"\n\t\t{\n\t\t\t\"Steam\"\n\t\t\t{\n\t\t\t\t\"AutoUpdateWindowStart\"\t\t\"-1\"\n\t\t\t}\n\t\t}\n\t}\n}\n");
        assert_eq!(compat_tool(&root, 289650), None);
        set_compat_tool(&root, 289650, Some("GE-Proton")).unwrap();
        set_compat_tool(&root, 0, Some("GE-Proton")).unwrap();
        assert_eq!(compat_tool(&root, 289650).as_deref(), Some("GE-Proton"));
        assert_eq!(compat_tool(&root, 0).as_deref(), Some("GE-Proton"));
        assert_eq!(compat_tool(&root, 5), None);
        set_compat_tool(&root, 289650, None).unwrap();
        assert_eq!(compat_tool(&root, 289650), None);
        assert_eq!(compat_tool(&root, 0).as_deref(), Some("GE-Proton"));
        assert!(std::fs::read_to_string(root.join("config/config.vdf")).unwrap().contains("AutoUpdateWindowStart"));
        let _ = std::fs::remove_dir_all(&root);
    }

    #[test]
    fn the_add_ons_own_file_gives_the_name_steam_knows_it_by() {
        let loc = folder("ge");
        put(&loc.join("files/GE-Proton/compatibilitytool.vdf"),
            "\"compatibilitytools\"\n{\n\t\"compat_tools\"\n\t{\n\t\t\"GE-Proton9-20\"\n\t\t{\n\t\t\t\"install_path\"\t\t\".\"\n\t\t}\n\t}\n}\n");
        assert_eq!(ge_name(&loc).as_deref(), Some("GE-Proton9-20"));
        assert_eq!(ge_name(&loc.join("nowhere")), None);
        let _ = std::fs::remove_dir_all(&loc);
    }

    #[test]
    fn games_are_found_in_every_library_and_steams_own_pieces_are_not() {
        let root = folder("games");
        let extra = folder("games-extra");
        put(&root.join("steamapps/libraryfolders.vdf"), &format!("\"libraryfolders\"\n{{\n\t\"0\"\n\t{{\n\t\t\"path\"\t\t\"{}\"\n\t}}\n\t\"1\"\n\t{{\n\t\t\"path\"\t\t\"relative\"\n\t}}\n}}\n", extra.display()));
        let acf = |id: u32, name: &str| format!("\"AppState\"\n{{\n\t\"appid\"\t\t\"{id}\"\n\t\"name\"\t\t\"{name}\"\n}}\n");
        put(&root.join("steamapps/appmanifest_289650.acf"), &acf(289650, "Prince of Persia"));
        put(&root.join("steamapps/appmanifest_1493710.acf"), &acf(1493710, "Proton Experimental"));
        put(&root.join("steamapps/appmanifest_1628350.acf"), &acf(1628350, "Steam Linux Runtime 3.0 (sniper)"));
        put(&extra.join("steamapps/appmanifest_400.acf"), &acf(400, "Portal"));
        put(&extra.join("steamapps/appmanifest_9.acf"), "\"AppState\" {");
        let libs = libraries(&root);
        assert_eq!(libs.len(), 2);
        let all = games(&libs);
        assert_eq!(all.iter().map(|g| (g.id, g.name.as_str())).collect::<Vec<_>>(), [(400, "Portal"), (289650, "Prince of Persia")]);
        assert_eq!(all[0].library, extra.join("steamapps"));
        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(&extra);
    }

    #[test]
    fn only_that_games_own_reaper_counts_as_it_running() {
        let proc = folder("proc");
        put(&proc.join("40/cmdline"), "reaper\0SteamLaunch\0AppId=289650\0--\0game.exe\0");
        put(&proc.join("41/cmdline"), "vim\0AppId=77\0");
        assert!(game_running(&proc, 289650));
        assert!(!game_running(&proc, 2896) && !game_running(&proc, 77) && !game_running(&proc, 1));
        let _ = std::fs::remove_dir_all(&proc);
    }

    #[test]
    fn two_graphics_chips_are_seen_by_their_class() {
        let drm = folder("drm");
        put(&drm.join("card0/device/class"), "0x030000\n");
        put(&drm.join("card0-eDP-1/device/class"), "0x030000\n");
        assert!(!two_chips(&drm));
        put(&drm.join("card1/device/class"), "0x038000\n");
        assert!(two_chips(&drm));
        let _ = std::fs::remove_dir_all(&drm);
        let one = folder("drm-one");
        put(&one.join("card0/device/class"), "0x030000\n");
        put(&one.join("card1/device/class"), "0x040300\n");
        assert!(!two_chips(&one));
        let _ = std::fs::remove_dir_all(&one);
    }
}
