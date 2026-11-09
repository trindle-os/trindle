//! What the installer knows before it writes anything: the disks it found,
//! the lists it offers, and the plan the root half carries out.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::common::run::text;
use crate::toolkit::t;

pub const GIB: u64 = 1024 * 1024 * 1024;

const KIB_PER_GIB: u64 = 1024 * 1024;

pub const LEAST: u64 = 20 * GIB;

pub const KEEP_FREE: u64 = 8 * GIB;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Part {
    pub dev: String,
    pub size: u64,
    pub used: u64,
    pub fstype: String,
    pub label: String,

    pub system: String,

    pub hint: String,
}

impl Part {

    pub fn floor(&self) -> u64 {
        self.used.saturating_add(KEEP_FREE).max(self.size / 8)
    }

    pub fn most_kept(&self) -> u64 {
        self.size.saturating_sub(LEAST)
    }

    pub fn shrinkable(&self) -> bool {
        matches!(self.fstype.as_str(), "ntfs" | "ext4" | "ext3" | "ext2")
            && self.size > self.floor().saturating_add(LEAST)
    }

    pub fn what(&self) -> String {
        let name = if !self.system.is_empty() {
            self.system.clone()
        } else if !self.label.is_empty() {
            self.label.clone()
        } else if self.fstype == "ntfs" {
            "Windows".into()
        } else if self.fstype.is_empty() {
            t!("installer-empty-space").into()
        } else {
            self.fstype.clone()
        };
        t!("installer-named-size", name = &name, size = size(self.size))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Disk {
    pub dev: String,
    pub model: String,
    pub size: u64,
    pub live: bool,
    pub parts: Vec<Part>,
}

impl Disk {
    pub fn what(&self) -> String {
        let model = if self.model.is_empty() { self.dev.clone() } else { self.model.clone() };
        t!("installer-named-size", name = &model, size = size(self.size))
    }

    pub fn holds(&self) -> String {
        let mut names: Vec<&str> = Vec::new();
        for p in &self.parts {
            if !p.system.is_empty() && !names.contains(&p.system.as_str()) {
                names.push(&p.system);
            }
        }
        if names.is_empty() {
            for p in &self.parts {
                for n in p.hint.split(" and ").filter(|n| !n.is_empty()) {
                    if !names.contains(&n) {
                        names.push(n);
                    }
                }
            }
        }
        let locked = self.parts.iter().any(|p| p.fstype == "crypto_LUKS");
        let windows = self.parts.iter().any(|p| p.fstype == "ntfs");
        if names.is_empty() && windows {
            names.push("Windows");
        }
        let list = match names.as_slice() {
            [] => String::new(),
            [one] => one.to_string(),
            [rest @ .., last] => t!("installer-list-and", rest = rest.join(", "), last = *last),
        };
        match (list.is_empty(), locked) {
            (false, false) => t!("installer-disk-has", list = &list),
            (false, true) => t!("installer-disk-has-encrypted", list = &list),
            (true, true) => t!("installer-disk-has-encrypted-system").into(),
            (true, false) if self.parts.iter().all(|p| p.fstype.is_empty()) => t!("installer-disk-empty").into(),
            (true, false) => t!("installer-disk-has-files").into(),
        }
    }
}

pub fn size(bytes: u64) -> String {
    let gb = bytes as f64 / GIB as f64;
    use trindle_toolkit::lang::format::decimal;
    if gb >= 1000.0 {
        t!("kit-size-tb", size = decimal(gb / 1024.0, 1))
    } else if gb >= 10.0 {
        t!("kit-size-gb", size = decimal(gb.round(), 0))
    } else {
        t!("kit-size-gb", size = decimal(gb, 1))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Where {

    Erase { disk: String },

    Beside { part: String, leave: u64 },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub enum Lock {

    #[default]
    None,

    Password,

    Chip,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Plan {
    pub locale: String,
    pub layout: String,
    pub variant: String,
    pub timezone: String,
    pub hostname: String,
    pub fullname: String,
    pub username: String,
    pub password: String,
    pub lock: Lock,
    pub disk_password: String,
    pub target: Option<Where>,

    #[serde(default)]
    pub swap: bool,

    #[serde(default)]
    pub terms: bool,
}

pub fn secure_boot() -> bool {
    std::fs::read("/sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c")
        .ok().and_then(|b| b.last().copied()) == Some(1)
}

pub fn memory_gib() -> u64 {
    memory_kib().unwrap_or(4 * KIB_PER_GIB).div_ceil(KIB_PER_GIB).max(1)
}

fn memory_kib() -> Option<u64> {
    std::fs::read_to_string("/proc/meminfo").ok()
        .and_then(|m| m.lines().find_map(|l| l.strip_prefix("MemTotal:")?.split_whitespace().next()?.parse().ok()))
}

const LEAST_MEMORY_KIB: u64 = 900 * 1024;

pub fn short_of_memory() -> bool {
    short_of(memory_kib())
}

fn short_of(kib: Option<u64>) -> bool {
    kib.is_some_and(|k| k < LEAST_MEMORY_KIB)
}

pub fn uefi() -> bool {
    Path::new("/sys/firmware/efi").exists()
}

impl Drop for Plan {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.password.zeroize();
        self.disk_password.zeroize();
    }
}

impl Plan {

    pub fn with_swap(mut self) -> Plan {
        self.swap = true;
        self
    }

    pub fn disk(&self) -> String {
        match &self.target {
            Some(Where::Erase { disk }) => disk.clone(),
            Some(Where::Beside { part, .. }) => whole_disk(part),
            None => String::new(),
        }
    }
}

pub fn whole_disk(dev: &str) -> String {
    let trimmed = dev.trim_end_matches(|c: char| c.is_ascii_digit());
    if let Some(base) = trimmed.strip_suffix('p') {
        if base.ends_with(|c: char| c.is_ascii_digit()) {
            return base.to_string();
        }
    }

    let name = trimmed.rsplit('/').next().unwrap_or_default();
    if ["nvme", "mmcblk", "loop", "nbd", "md", "sr"].iter().any(|f| name.starts_with(f)) {
        return dev.to_string();
    }
    trimmed.to_string()
}

pub fn part_name(disk: &str, n: u32) -> String {
    if disk.ends_with(|c: char| c.is_ascii_digit()) {
        format!("{disk}p{n}")
    } else {
        format!("{disk}{n}")
    }
}

pub fn other_systems() -> Vec<(String, String)> {
    let mut found = parse_os_prober(&text(&["sudo", "-n", "os-prober"]));
    for (dev, name) in parse_os_prober(&text(&["sudo", "-n", "trindle-installer", "--look"])) {
        if !found.iter().any(|(d, _)| *d == dev) {
            found.push((dev, name));
        }
    }
    found
}

pub fn parse_os_prober(out: &str) -> Vec<(String, String)> {
    out.lines().filter_map(|l| {
        let mut f = l.split(':');
        let dev = f.next()?.trim();
        let name = f.next()?.trim();
        let guess = f.next().map(str::trim) == Some("efi");
        let name = if guess { format!("efi:{name}") } else { name.to_string() };
        (!dev.is_empty() && !name.is_empty()).then(|| (dev.to_string(), name))
    }).collect()
}

pub fn disks() -> Vec<Disk> {
    let json = text(&["lsblk", "-J", "-b", "-o", "PATH,TYPE,SIZE,MODEL,RM,FSTYPE,LABEL,FSUSED,MOUNTPOINT"]);
    let live = live_disk();
    let mut out = parse_disks(&json);
    for d in &mut out {
        d.live = d.dev == live;
    }
    name_systems(&mut out, &other_systems());
    out.retain(|d| d.size >= LEAST && !d.live);
    out
}

pub fn live_disk() -> String {
    for mount in ["/run/live/medium", "/run/live/findiso", "/cdrom"] {
        let src = text(&["findmnt", "-no", "SOURCE", mount]);
        if !src.is_empty() {
            return whole_disk(src.trim());
        }
    }
    String::new()
}

pub fn name_systems(disks: &mut [Disk], systems: &[(String, String)]) {
    for d in disks.iter_mut() {
        for p in d.parts.iter_mut() {
            if let Some((_, name)) = systems.iter().find(|(dev, _)| *dev == p.dev) {
                match name.strip_prefix("efi:") {
                    Some(guess) => p.hint = guess.to_string(),
                    None => p.system = name.clone(),
                }
            }
        }
    }
}

pub fn parse_disks(json: &str) -> Vec<Disk> {
    let v: serde_json::Value = serde_json::from_str(json).unwrap_or(serde_json::Value::Null);

    let mut all = Vec::new();
    fn walk(v: &serde_json::Value, out: &mut Vec<serde_json::Value>) {
        for d in v.as_array().into_iter().flatten() {
            out.push(d.clone());
            walk(&d["children"], out);
        }
    }
    walk(&v["blockdevices"], &mut all);
    let mut out: Vec<Disk> = all.iter().filter(|d| d["type"].as_str() == Some("disk")).filter_map(|d| {
        let dev = d["path"].as_str().unwrap_or_default().to_string();
        if dev.is_empty() || dev.starts_with("/dev/zram") || dev.starts_with("/dev/loop") {
            return None;
        }
        Some(Disk {
            dev,
            model: d["model"].as_str().unwrap_or_default().trim().to_string(),
            size: d["size"].as_u64().unwrap_or(0),
            live: false,
            parts: Vec::new(),
        })
    }).collect();
    for p in all.iter().filter(|p| p["type"].as_str() == Some("part")) {
        let dev = p["path"].as_str().unwrap_or_default().to_string();
        let Some(disk) = out.iter_mut().find(|d| d.dev == whole_disk(&dev)) else { continue };
        disk.parts.push(Part {
            dev,
            size: p["size"].as_u64().unwrap_or(0),
            used: p["fsused"].as_u64().unwrap_or(0),
            fstype: p["fstype"].as_str().unwrap_or_default().to_string(),
            label: p["label"].as_str().unwrap_or_default().to_string(),
            system: String::new(),
            hint: String::new(),
        });
    }
    out
}

pub fn mistyped(one: &str, two: &str) -> bool {
    !two.is_empty() && !one.starts_with(two)
}

pub fn locales() -> Vec<(String, String)> {
    let supported = std::fs::read_to_string("/usr/share/i18n/SUPPORTED").unwrap_or_default();
    parse_locales(&supported)
}

pub fn parse_locales(supported: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = supported.lines()
        .filter_map(|l| l.split_whitespace().next())
        .filter(|l| l.ends_with(".UTF-8") && !l.starts_with("C."))
        .map(|l| {
            let code = l.to_string();
            let name = language_name(&code);
            (code, name)
        })
        .collect();
    out.sort_by(|a, b| a.1.to_lowercase().cmp(&b.1.to_lowercase()).then(a.0.cmp(&b.0)));
    out.dedup_by(|a, b| a.0 == b.0);
    out
}

pub fn language_name(locale: &str) -> String {
    let stem = locale.split('.').next().unwrap_or(locale);
    let file = std::fs::read_to_string(format!("/usr/share/i18n/locales/{stem}")).unwrap_or_default();
    name_from(stem, &file)
}

fn name_from(stem: &str, file: &str) -> String {
    let (lang, country) = stem.split_once('_').unwrap_or((stem, ""));
    let country = country.split('@').next().unwrap_or(country);
    let own = |key: &str| file.lines()
        .find_map(|l| l.trim().strip_prefix(key).filter(|v| v.starts_with(char::is_whitespace)))
        .map(|v| unescape(v.trim().trim_matches('"')))
        .filter(|v| !v.is_empty());
    let name = own("lang_name")
        .or_else(|| LANGUAGES.iter().find(|(c, _)| *c == lang).map(|(_, n)| n.to_string()))
        .unwrap_or_else(|| lang.to_string());
    let mut chars = name.chars();
    let name: String = chars.next().map(|c| char::to_uppercase(c).chain(chars).collect()).unwrap_or_default();
    match (own("country_name"), country.is_empty()) {
        (_, true) => name,
        (Some(place), _) => format!("{name} ({place})"),
        (None, _) => format!("{name} ({country})"),
    }
}

fn unescape(s: &str) -> String {
    let mut out = String::new();
    let mut rest = s;
    while let Some(i) = rest.find("<U") {
        out.push_str(&rest[..i]);
        let tail = &rest[i + 2..];
        match tail.find('>').and_then(|e| u32::from_str_radix(&tail[..e], 16).ok().and_then(char::from_u32).map(|c| (c, e))) {
            Some((c, e)) => { out.push(c); rest = &tail[e + 1..]; }
            None => { out.push_str("<U"); rest = tail; }
        }
    }
    out.push_str(rest);
    out
}

const LANGUAGES: &[(&str, &str)] = &[
    ("af", "Afrikaans"), ("ar", "العربية"), ("be", "Беларуская"), ("bg", "Български"),
    ("bn", "বাংলা"), ("bs", "Bosanski"), ("ca", "Català"), ("cs", "Čeština"), ("cy", "Cymraeg"),
    ("da", "Dansk"), ("de", "Deutsch"), ("el", "Ελληνικά"), ("en", "English"), ("es", "Español"),
    ("et", "Eesti"), ("eu", "Euskara"), ("fa", "فارسی"), ("fi", "Suomi"), ("fr", "Français"),
    ("ga", "Gaeilge"), ("gd", "Gàidhlig"), ("gl", "Galego"), ("he", "עברית"), ("hi", "हिन्दी"),
    ("hr", "Hrvatski"), ("hu", "Magyar"), ("id", "Indonesia"), ("is", "Íslenska"),
    ("it", "Italiano"), ("ja", "日本語"), ("ka", "ქართული"), ("kk", "Қазақ"), ("km", "ខ្មែរ"),
    ("ko", "한국어"), ("lt", "Lietuvių"), ("lv", "Latviešu"), ("mk", "Македонски"),
    ("ml", "മലയാളം"), ("mr", "मराठी"), ("ms", "Melayu"), ("mt", "Malti"), ("nb", "Norsk bokmål"),
    ("ne", "नेपाली"), ("nl", "Nederlands"), ("nn", "Norsk nynorsk"), ("pa", "ਪੰਜਾਬੀ"),
    ("pl", "Polski"), ("pt", "Português"), ("ro", "Română"), ("ru", "Русский"),
    ("si", "සිංහල"), ("sk", "Slovenčina"), ("sl", "Slovenščina"), ("sq", "Shqip"),
    ("sr", "Српски"), ("sv", "Svenska"), ("sw", "Kiswahili"), ("ta", "தமிழ்"), ("te", "తెలుగు"),
    ("th", "ไทย"), ("tr", "Türkçe"), ("uk", "Українська"), ("ur", "اردو"), ("vi", "Tiếng Việt"),
    ("zh", "中文"),
];

pub fn layout_for(locale: &str) -> (String, String) {
    let country = str::to_uppercase(locale.split('.').next().unwrap_or("").split('_').nth(1).unwrap_or(""));
    let lang = locale.split(['_', '.']).next().unwrap_or("");

    if country == "CA" && lang != "fr" {
        return ("us".to_string(), String::new());
    }
    if let Some((_, layout, variant)) = KEYBOARD_BY_COUNTRY.iter().find(|(c, _, _)| *c == country) {
        return (layout.to_string(), variant.to_string());
    }

    let layout = match lang { "ar" => "ara", "zh" => "cn", other => other };
    (layout.to_string(), String::new())
}

const KEYBOARD_BY_COUNTRY: &[(&str, &str, &str)] = &[
    ("GB", "gb", ""), ("US", "us", ""), ("AU", "us", ""), ("NZ", "us", ""), ("IE", "ie", ""),
    ("CA", "ca", ""), ("BR", "br", ""), ("PT", "pt", ""), ("ES", "es", ""), ("MX", "latam", ""),
    ("AR", "latam", ""), ("CL", "latam", ""), ("CO", "latam", ""), ("PE", "latam", ""),
    ("FR", "fr", ""), ("BE", "be", ""), ("CH", "ch", ""), ("AT", "at", ""), ("DE", "de", ""),
    ("NL", "us", ""), ("IT", "it", ""), ("SE", "se", ""), ("NO", "no", ""), ("DK", "dk", ""),
    ("FI", "fi", ""), ("IS", "is", ""), ("PL", "pl", ""), ("CZ", "cz", ""), ("SK", "sk", ""),
    ("HU", "hu", ""), ("RO", "ro", ""), ("BG", "bg", ""), ("GR", "gr", ""), ("TR", "tr", ""),
    ("RU", "ru", ""), ("UA", "ua", ""), ("IL", "il", ""), ("IN", "in", ""), ("PK", "pk", ""),
    ("CN", "cn", ""), ("TW", "tw", ""), ("JP", "jp", ""), ("KR", "kr", ""), ("TH", "th", ""),
    ("VN", "vn", ""), ("ID", "us", ""), ("MY", "us", ""), ("PH", "us", ""), ("SG", "us", ""),
    ("ZA", "za", ""), ("NG", "us", ""), ("KE", "us", ""), ("EG", "ara", ""), ("SA", "ara", ""),
    ("AE", "ara", ""), ("HK", "us", ""), ("SI", "si", ""), ("HR", "hr", ""), ("RS", "rs", ""),
    ("EE", "ee", ""), ("LV", "lv", ""), ("LT", "lt", ""), ("GE", "ge", ""), ("KZ", "kz", ""),
];

pub fn suggest_passphrase() -> String {
    let mut bytes = [0u8; 8];
    if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
        use std::io::Read;
        let _ = f.read_exact(&mut bytes);
    }
    let pick = |n: usize| WORDS[bytes[n] as usize % WORDS.len()];
    format!("{}-{}-{}-{}{}", pick(0), pick(1), pick(2), pick(3), bytes[4] % 10)
}

const WORDS: &[&str] = &[
    "amber", "anchor", "apple", "arrow", "autumn", "basket", "beacon", "birch", "bramble", "breeze",
    "bridge", "bronze", "cabin", "canvas", "cedar", "chimney", "cinder", "clover", "cobalt", "copper",
    "cotton", "crayon", "daisy", "dolphin", "ember", "falcon", "feather", "fennel", "ferry", "flint",
    "forest", "fossil", "garden", "granite", "harbour", "hazel", "heather", "hollow", "indigo", "ivory",
    "jasmine", "kettle", "lantern", "lemon", "lichen", "linen", "lobster", "magnet", "maple", "marble",
    "meadow", "mitten", "monsoon", "moss", "mustard", "nectar", "nutmeg", "oak", "ocean", "olive",
    "orchid", "otter", "paddle", "pebble", "pepper", "pewter", "pine", "plum", "pollen", "poppy",
    "quartz", "quilt", "rabbit", "rafter", "raven", "ribbon", "river", "rosemary", "saffron", "salmon",
    "sapphire", "shell", "silver", "sparrow", "spruce", "stone", "sugar", "sunset", "tandem", "teapot",
    "thistle", "thunder", "timber", "topaz", "tulip", "velvet", "walnut", "willow", "window", "winter",
];

pub fn keyboards() -> Vec<(String, String, String)> {
    let lst = ["/usr/share/X11/xkb/rules/base.lst", "/usr/share/X11/xkb/rules/evdev.lst"].iter()
        .find_map(|p| std::fs::read_to_string(p).ok()).unwrap_or_default();
    parse_keyboards(&lst)
}

pub fn parse_keyboards(lst: &str) -> Vec<(String, String, String)> {
    let mut out = Vec::new();
    let mut section = "";
    for line in lst.lines() {
        if let Some(name) = line.strip_prefix('!') {
            section = name.split_whitespace().next().unwrap_or("");
            continue;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((code, name)) = line.split_once(char::is_whitespace) else { continue };
        let name = name.trim();
        match section {
            "layout" => out.push((code.to_string(), String::new(), name.to_string())),
            "variant" => {
                let Some((layout, rest)) = name.split_once(':') else { continue };
                out.push((layout.trim().to_string(), code.to_string(), rest.trim().to_string()));
            }
            _ => {}
        }
    }
    out.sort_by_key(|k| k.2.to_lowercase());
    out
}

pub fn timezones() -> Vec<String> {
    let mut out: Vec<String> = text(&["timedatectl", "list-timezones"]).lines().map(String::from).collect();
    if out.is_empty() {
        out = std::fs::read_to_string("/usr/share/zoneinfo/zone1970.tab").unwrap_or_default().lines()
            .filter(|l| !l.starts_with('#'))
            .filter_map(|l| l.split('\t').nth(2).map(String::from))
            .collect();
        out.sort();
    }
    out
}

const MAIN_ZONE: [(&str, &str); 29] = [
    ("AU", "Australia/Sydney"), ("BR", "America/Sao_Paulo"), ("CA", "America/Toronto"), ("CH", "Europe/Zurich"),
    ("CL", "America/Santiago"), ("CN", "Asia/Shanghai"), ("DE", "Europe/Berlin"), ("DZ", "Africa/Algiers"),
    ("ES", "Europe/Madrid"), ("FR", "Europe/Paris"), ("GB", "Europe/London"), ("ID", "Asia/Jakarta"),
    ("IN", "Asia/Kolkata"), ("IE", "Europe/Dublin"), ("IT", "Europe/Rome"), ("KZ", "Asia/Almaty"),
    ("MA", "Africa/Casablanca"), ("MX", "America/Mexico_City"), ("NL", "Europe/Amsterdam"), ("NZ", "Pacific/Auckland"),
    ("PT", "Europe/Lisbon"), ("RU", "Europe/Moscow"), ("SA", "Asia/Riyadh"), ("TR", "Europe/Istanbul"),
    ("TZ", "Africa/Dar_es_Salaam"), ("UA", "Europe/Kyiv"), ("US", "America/New_York"), ("VN", "Asia/Ho_Chi_Minh"),
    ("ZA", "Africa/Johannesburg"),
];

pub fn timezone_for(locale: &str, all: &[String]) -> String {
    let country = locale.split('.').next().unwrap_or("").split('_').nth(1).unwrap_or("").to_string();
    if let Some((_, zone)) = MAIN_ZONE.iter().find(|(c, _)| *c == country) {
        if all.iter().any(|z| z == zone) {
            return (*zone).to_string();
        }
    }
    let tab = std::fs::read_to_string("/usr/share/zoneinfo/zone1970.tab").unwrap_or_default();
    let guess = tab.lines().filter(|l| !l.starts_with('#'))
        .find(|l| l.split('\t').next().is_some_and(|cc| cc.split(',').any(|c| c == country)))
        .and_then(|l| l.split('\t').nth(2)).unwrap_or("").to_string();
    if all.contains(&guess) { guess } else { "UTC".to_string() }
}

pub fn password_strength(pw: &str) -> (u8, String) {
    if pw.is_empty() {
        return (0, String::new());
    }
    let lower = pw.chars().any(|c| c.is_ascii_lowercase());
    let upper = pw.chars().any(|c| c.is_ascii_uppercase());
    let digit = pw.chars().any(|c| c.is_ascii_digit());
    let other = pw.chars().any(|c| !c.is_ascii_alphanumeric());
    let mut pool = 0u32;
    if lower { pool += 26 }
    if upper { pool += 26 }
    if digit { pool += 10 }
    if other { pool += 33 }
    let pool = pool.max(2);

    let distinct = {
        let mut seen: Vec<char> = pw.chars().collect();
        seen.sort_unstable();
        seen.dedup();
        seen.len()
    };
    let effective = (pw.chars().count().min(64) as f64) * (distinct as f64 / pw.chars().count().max(1) as f64).max(0.35);
    let bits = effective * (pool as f64).log2();

    let seconds = 2f64.powf(bits - 1.0) / 1e10;
    let how_long = if seconds < 1.0 {
        t!("installer-guess-now").to_string()
    } else if seconds < 3600.0 {
        t!("installer-guess-minutes", n = (seconds / 60.0).max(1.0).round() as u64)
    } else if seconds < 86_400.0 {
        t!("installer-guess-hours", n = (seconds / 3600.0).round() as u64)
    } else if seconds < 31_557_600.0 {
        t!("installer-guess-days", n = (seconds / 86_400.0).round() as u64)
    } else if seconds < 31_557_600.0 * 1000.0 {
        t!("installer-guess-years", n = (seconds / 31_557_600.0).round() as u64)
    } else {
        t!("installer-guess-never").to_string()
    };
    let score = match bits as u32 {
        0..=27 => 1,
        28..=39 => 2,
        40..=55 => 3,
        56..=79 => 4,
        _ => 5,
    };
    (score, how_long)
}

pub fn username_from(fullname: &str) -> String {
    let first = fullname.split_whitespace().next().unwrap_or("");
    let name: String = first.to_lowercase().chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    if name.chars().next().is_some_and(|c| c.is_ascii_digit()) { String::new() } else { name }
}

pub fn username_fault(name: &str) -> Option<&'static str> {
    const TAKEN: &[&str] = &["root", "daemon", "bin", "sys", "nobody", "trindle", "_greetd"];
    if name.is_empty() {
        Some(t!("installer-user-needed"))
    } else if name.len() > 32 {
        Some(t!("installer-name-too-long"))
    } else if !name.chars().next().is_some_and(|c| c.is_ascii_lowercase()) {
        Some(t!("installer-user-start"))
    } else if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_') {
        Some(t!("installer-user-letters"))
    } else if TAKEN.contains(&name) {
        Some(t!("installer-user-taken"))
    } else {
        None
    }
}

pub fn hostname_fault(name: &str) -> Option<&'static str> {
    if name.is_empty() {
        Some(t!("installer-host-needed"))
    } else if name.len() > 63 {
        Some(t!("installer-name-too-long"))
    } else if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') || name.starts_with('-') {
        Some(t!("installer-host-letters"))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn too_little_memory_is_refused_and_unreadable_memory_is_not() {
        assert!(super::short_of(Some(512 * 1024)) && super::short_of(Some(900 * 1024 - 1)));
        assert!(!super::short_of(Some(980 * 1024)) && !super::short_of(Some(8 * 1024 * 1024)) && !super::short_of(None));
    }

    #[test]
    fn the_first_guess_is_where_most_people_of_the_country_live() {
        let all: Vec<String> = ["Australia/Sydney", "Australia/Lord_Howe", "America/Toronto", "America/St_Johns", "America/Sao_Paulo", "America/Noronha",
            "Europe/Moscow", "Europe/Kaliningrad", "Europe/Berlin", "Europe/Zurich", "America/New_York", "UTC"].iter().map(|z| z.to_string()).collect();
        for (locale, zone) in [("en_AU.UTF-8", "Australia/Sydney"), ("en_CA.UTF-8", "America/Toronto"), ("pt_BR.UTF-8", "America/Sao_Paulo"),
                               ("ru_RU.UTF-8", "Europe/Moscow"), ("de_DE.UTF-8", "Europe/Berlin"), ("de_CH.UTF-8", "Europe/Zurich"), ("en_US.UTF-8", "America/New_York")] {
            assert_eq!(timezone_for(locale, &all), zone, "{locale}");
        }
        assert_eq!(timezone_for("xx_ZZ.UTF-8", &all), "UTC");
    }

    #[test]
    fn a_keyboard_is_always_a_layout_that_exists() {
        for (locale, layout) in [("en_CA.UTF-8", "us"), ("fr_CA.UTF-8", "ca"), ("ar_DZ.UTF-8", "ara"), ("ar_MA.UTF-8", "ara"), ("zh_HK.UTF-8", "us"), ("zh_TW.UTF-8", "tw"), ("zh_SG.UTF-8", "us")] {
            assert_eq!(layout_for(locale).0, layout, "{locale}");
        }
    }

    use super::*;

    #[test]
    fn disk_sizes_follow_the_language() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        assert_eq!((size(512_110_190_592), size(3 * GIB), size(2_000_398_934_016)), ("477 GB".to_string(), "3.0 GB".to_string(), "1.8 TB".to_string()));
        let made_up = "kit-number-decimal = ,\nkit-size-gb = { $size } Go\nkit-size-tb = { $size } To\n";
        with_catalogue(Catalogue::new(Bundle::from_text("xx", made_up), english()), || {
            assert_eq!((size(512_110_190_592), size(3 * GIB), size(2_000_398_934_016)), ("477 Go".to_string(), "3,0 Go".to_string(), "1,8 To".to_string()));
        });
    }

    #[test]
    fn disks_are_read() {
        let json = r#"{"blockdevices":[
            {"path":"/dev/nvme0n1","type":"disk","size":512110190592,"model":"WD SN770","rm":false,
             "children":[{"path":"/dev/nvme0n1p1","type":"part","size":536870912,"fstype":"vfat","label":"EFI","fsused":40000000},
                         {"path":"/dev/nvme0n1p2","type":"part","size":511000000000,"fstype":"ntfs","label":"","fsused":90000000000}]},
            {"path":"/dev/sda","type":"disk","size":1000000,"model":"tiny","rm":true}]}"#;
        let all = parse_disks(json);
        assert_eq!(all.len(), 2);
        assert_eq!(all[0].parts.len(), 2);
        assert!(all[0].parts[1].shrinkable());
        assert!(!all[0].parts[0].shrinkable());
        assert_eq!(all[0].what(), "WD SN770, 477 GB");
    }

    #[test]
    fn flat_listings_are_read_too() {
        let json = r#"{"blockdevices":[
            {"path":"/dev/nvme0n1","type":"disk","size":2000398934016,"model":"CT2000","rm":false},
            {"path":"/dev/nvme0n1p1","type":"part","size":1023410176,"fstype":"vfat","fsused":9469952},
            {"path":"/dev/nvme0n1p2","type":"part","size":1982505943040,"fstype":"ext4","fsused":23953653760}]}"#;
        let all = parse_disks(json);
        assert_eq!(all[0].parts.len(), 2);
        assert!(all[0].parts[1].shrinkable());
    }

    #[test]
    fn other_systems_are_named() {
        let mut disks = parse_disks(r#"{"blockdevices":[{"path":"/dev/sda","type":"disk","size":500000000000,
            "children":[{"path":"/dev/sda2","type":"part","size":400000000000,"fstype":"ntfs","fsused":100000000000}]}]}"#);
        let found = parse_os_prober("/dev/sda2:Windows 11:Windows:chain\n");
        name_systems(&mut disks, &found);
        assert!(disks[0].parts[0].what().starts_with("Windows 11,"));
    }

    #[test]
    fn each_disk_says_what_is_on_it() {
        let mut disks = parse_disks(r#"{"blockdevices":[
            {"path":"/dev/nvme0n1","type":"disk","size":500000000000,"children":[
                {"path":"/dev/nvme0n1p1","type":"part","size":600000000,"fstype":"vfat"},
                {"path":"/dev/nvme0n1p2","type":"part","size":1000000000,"fstype":"ext4"},
                {"path":"/dev/nvme0n1p3","type":"part","size":400000000000,"fstype":"btrfs"}]},
            {"path":"/dev/nvme1n1","type":"disk","size":500000000000,"children":[
                {"path":"/dev/nvme1n1p1","type":"part","size":600000000,"fstype":"vfat"},
                {"path":"/dev/nvme1n1p2","type":"part","size":400000000000,"fstype":"crypto_LUKS"}]},
            {"path":"/dev/sda","type":"disk","size":500000000000}]}"#);
        let found = parse_os_prober("/dev/nvme0n1p1:Fedora:efi\n/dev/nvme0n1p3:Bazzite 42\n\
                                     /dev/nvme1n1p1:Ubuntu:efi\n");
        name_systems(&mut disks, &found);
        assert_eq!(disks[0].holds(), "Has Bazzite 42 on it");
        assert_eq!(disks[1].holds(), "Has Ubuntu on it, encrypted");
        assert_eq!(disks[2].holds(), "Empty");
    }

    #[test]
    fn partition_names_follow_the_disk() {
        assert_eq!(whole_disk("/dev/nvme0n1p2"), "/dev/nvme0n1");
        assert_eq!(whole_disk("/dev/sda3"), "/dev/sda");
        assert_eq!(whole_disk("/dev/nvme0n1"), "/dev/nvme0n1");
        assert_eq!(whole_disk("/dev/mmcblk0"), "/dev/mmcblk0");
        assert_eq!(whole_disk("/dev/mmcblk0p1"), "/dev/mmcblk0");
        assert_eq!(whole_disk("/dev/sda"), "/dev/sda");
        assert_eq!(whole_disk("/dev/vdb"), "/dev/vdb");
        assert_eq!(whole_disk("/dev/sr0"), "/dev/sr0");
        assert_eq!(part_name("/dev/nvme0n1", 2), "/dev/nvme0n1p2");
        assert_eq!(part_name("/dev/sda", 2), "/dev/sda2");
    }

    #[test]
    fn languages_and_keyboards_are_listed() {
        let list = parse_locales("en_GB.UTF-8 UTF-8\nfr_FR.UTF-8 UTF-8\nen_US.UTF-8 UTF-8\nC.UTF-8 UTF-8\nen_GB ISO-8859-1\n");
        assert!(list.iter().any(|(c, n)| c == "fr_FR.UTF-8" && n.starts_with("Français (")));
        assert!(list.iter().all(|(c, _)| c.ends_with(".UTF-8") && c != "C.UTF-8"));
        let keys = parse_keyboards("! layout\n  gb   English (UK)\n  fr   French\n! variant\n  dvorak  gb: English (UK, Dvorak)\n");
        assert!(keys.contains(&("gb".into(), "dvorak".into(), "English (UK, Dvorak)".into())));
        assert!(keys.contains(&("fr".into(), String::new(), "French".into())));
    }

    #[test]
    fn a_password_on_its_way_is_not_a_mistake() {
        assert!(!mistyped("horse", ""));
        assert!(!mistyped("horse", "hor"));
        assert!(!mistyped("horse", "horse"));
        assert!(mistyped("horse", "hog"));
        assert!(mistyped("horse", "horses"));
    }

    #[test]
    fn languages_use_their_own_names() {
        assert_eq!(name_from("aa_DJ", "lang_name     \"Qafar\"\ncountry_name  \"Yabuuti\"\n"), "Qafar (Yabuuti)");
        assert_eq!(name_from("syr", "lang_name   \"<U0723><U0718>\"\n"), "\u{723}\u{718}");
        assert_eq!(name_from("fr_CA", ""), "Français (CA)");
        assert_eq!(name_from("sr_RS@latin", "country_name \"Srbija\"\n"), "Српски (Srbija)");
        assert_eq!(name_from("xx_YY", ""), "Xx (YY)");
    }

    #[test]
    fn language_picks_a_keyboard() {
        assert_eq!(layout_for("en_GB.UTF-8"), ("gb".to_string(), String::new()));
        assert_eq!(layout_for("fr_FR.UTF-8"), ("fr".to_string(), String::new()));
        assert_eq!(layout_for("nl_NL.UTF-8"), ("us".to_string(), String::new()));
        assert_eq!(layout_for("hy_AM.UTF-8"), ("hy".to_string(), String::new()));
        let one = suggest_passphrase();
        assert_eq!(one.split('-').count(), 4);
        assert!(one.len() >= 16);
    }

    #[test]
    fn names_are_checked() {
        assert_eq!(username_from("Morgan Smith"), "morgan");
        assert_eq!(username_fault("morgan"), None);
        assert!(username_fault("Morgan").is_some());
        assert!(username_fault("root").is_some());
        assert_eq!(hostname_fault("trindle-laptop"), None);
        assert!(hostname_fault("my laptop").is_some());
    }
}
