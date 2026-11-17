//! Region and language: LANG and LC_* in trindle.env, applied at next login.
//! Missing locales are installed via trindle-admin.

use std::collections::{BTreeSet, HashMap};
use std::path::PathBuf;

use crate::common::files::{read_env, set_setting, setting};
use crate::common::language_names::{self, Tongue};
use crate::common::live_language;
use crate::common::run::{captured, on_thread};
use crate::settings::{session_env, Settings};
use crate::settings::ui::Do;
use crate::settings::ui::{admin_run, confirm, not_saved, dropdown, group, headline, job, line, on, page, Kit, Msg, Page, INSTALL_ICON};
use crate::toolkit::t;
use crate::toolkit::tiles::fill_bar_good;
use crate::toolkit::widget::Node;

mod words;

const SUPPORTED: &str = "/usr/share/i18n/SUPPORTED";
const LOCALES: &str = "/usr/share/i18n/locales";
const FORMATS: &[&str] = &["LC_TIME", "LC_NUMERIC", "LC_MONETARY", "LC_MEASUREMENT", "LC_PAPER"];

fn later() -> &'static str {
    t!("settings-region-later")
}

const ADD: &str = "add-language";

fn normalise(name: &str) -> String {
    let l = name.to_lowercase();
    if l.ends_with(".utf8") || l.ends_with(".utf-8") {
        format!("{}.UTF-8", name.split('.').next().unwrap_or(name))
    } else {
        name.to_string()
    }
}

fn plain_name(code: &str) -> String {
    let base = code.split('.').next().unwrap_or(code);
    let (mut language, mut territory) = (String::new(), String::new());
    if let Ok(bytes) = std::fs::read(PathBuf::from(LOCALES).join(base)) {
        for line in String::from_utf8_lossy(&bytes).lines() {
            let mut parts = line.trim_start().splitn(2, char::is_whitespace);
            let (Some(key), Some(value)) = (parts.next(), parts.next()) else { continue };
            let value = value.trim().trim_matches('"');
            if key == "language" && language.is_empty() {
                language = value.to_string();
            } else if key == "territory" && territory.is_empty() {
                territory = value.to_string();
            }
            if !language.is_empty() && !territory.is_empty() {
                break;
            }
        }
    }
    if language.is_empty() {
        return code.to_string();
    }
    let variant = base.split_once('@').map(|(_, v)| format!(", {v}")).unwrap_or_default();
    if territory.is_empty() {
        format!("{language}{variant}")
    } else {
        format!("{language} ({territory}{variant})")
    }
}

fn supported() -> Vec<String> {
    std::fs::read_to_string(SUPPORTED)
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            (parts.len() == 2 && parts[1] == "UTF-8").then(|| parts[0].to_string())
        })
        .collect()
}

fn choices(installed: &[String]) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = supported()
        .into_iter()
        .filter(|n| !installed.contains(&normalise(n)))
        .map(|n| {
            let label = plain_name(&n);
            (n, label)
        })
        .collect();
    out.sort_by_key(|(_, l)| l.to_lowercase());
    out
}

fn running_lang() -> String {
    normalise(&std::env::var("LANG").unwrap_or_else(|_| "C.UTF-8".to_string()))
}

fn running_fmt() -> String {
    std::env::var("LC_TIME").ok().filter(|s| !s.is_empty()).map(|s| normalise(&s)).unwrap_or_else(running_lang)
}

fn saved() -> (String, String) {
    let env = read_env(&session_env::path());
    let lang = env.get("LANG").filter(|s| !s.is_empty()).cloned().unwrap_or_else(running_lang);
    let fmt = env.get("LC_TIME").filter(|s| !s.is_empty()).cloned().unwrap_or_else(|| lang.clone());
    (lang, fmt)
}

fn money_and_paper(said: &str) -> (String, String) {
    let value = |key: &str| said.lines().find_map(|l| l.strip_prefix(key).and_then(|v| v.strip_prefix('=')))
        .map(|v| v.trim().trim_matches('"').trim().to_string()).unwrap_or_default();
    let sign = value("currency_symbol");
    let code = value("int_curr_symbol");
    let money = if !sign.is_empty() { sign } else if !code.is_empty() { code } else { "\u{2026}".to_string() };
    let paper = match (value("height").as_str(), value("width").as_str()) {
        ("297", "210") => "a4",
        ("279", "216") => "letter",
        _ => "other",
    };
    let measures = if value("measurement") == "2" { "imperial" } else { "metric" };
    (money, t!("settings-region-paper-measures", paper = paper, measures = measures))
}

fn font_missing(code: &str, fonts: &[String]) -> Option<&'static crate::toolkit::lang::script::Script> {
    use crate::toolkit::lang::script;
    script::missing_in(script::SCRIPTS, &crate::toolkit::lang::tags_of(code), fonts)
}

struct Found {
    installed: Vec<String>,
    names: HashMap<String, String>,
    add: Vec<(String, String)>,
    tongues: Vec<Tongue>,

    fonts: Vec<String>,
}

fn look() -> Found {
    let all = vec!["locale".to_string(), "-a".to_string()];
    let (_, out, _) = captured(&all, None);
    let installed: Vec<String> = out
        .split_whitespace()
        .filter(|n| {
            let l = n.to_lowercase();
            n.contains('.') && (l.ends_with("utf8") || l.ends_with("utf-8")) && !l.starts_with("c.") && !l.starts_with("posix")
        })
        .map(normalise)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let mut names = HashMap::new();
    for name in &installed {
        let argv: Vec<String> = ["env", &format!("LC_ALL={name}"), "locale", "language", "territory"]
            .iter().map(|s| s.to_string()).collect();
        let (ok, text, _) = captured(&argv, None);
        let lines: Vec<&str> = text.lines().map(str::trim).filter(|s| !s.is_empty()).collect();
        if ok && lines.len() >= 2 {
            names.insert(name.clone(), format!("{} ({})", lines[0], lines[1]));
        }
    }
    let add = choices(&installed);
    let tongues = crate::settings::wordsheet::own::with_own(language_names::found(&crate::toolkit::lang::folders(), &crate::toolkit::lang::english()),
                                                            &crate::settings::wordsheet::base());
    let fonts = crate::toolkit::lang::script::system_families();
    Found { installed, names, add, tongues, fonts }
}

pub struct Region {

    loaded: bool,
    installed: Vec<String>,
    names: HashMap<String, String>,
    add: Vec<(String, String)>,

    tongues: Vec<Tongue>,
    fonts: Vec<String>,

    own: String,

    pick: usize,
    lang: String,
    fmt: String,
    head: String,
    count: usize,
    date: String,
    number: String,

    money: String,
    paper: String,

    generation: u32,

    pending: bool,
    words: words::State,
}

impl Default for Region {
    fn default() -> Self {
        Region {
            loaded: false,
            installed: Vec::new(),
            names: HashMap::new(),
            add: Vec::new(),
            tongues: Vec::new(),
            fonts: Vec::new(),
            own: setting("language"),
            pick: 0,
            lang: String::new(),
            fmt: String::new(),
            head: t!("settings-region-looking").to_string(),
            count: 0,
            date: "\u{2026}".to_string(),
            number: "\u{2026}".to_string(),
            money: "\u{2026}".to_string(),
            paper: String::new(),
            generation: 0,
            pending: false,
            words: words::State::default(),
        }
    }
}

impl Region {
    fn load(&mut self, kit: &mut Kit) {
        self.generation = self.generation.wrapping_add(1);
        let generation = self.generation;
        on_thread(look, kit.then::<Region, Found>(move |me, k, found| {
            if me.generation != generation {
                return;
            }
            me.installed = found.installed;
            me.names = found.names;
            me.add = found.add;
            me.tongues = found.tongues;
            me.fonts = found.fonts;
            me.own = setting("language");
            me.pick = me.pick.min(me.add.len().saturating_sub(1));
            me.loaded = true;
            me.told(k);
            if me.pending {
                me.pending = false;
                k.done(ADD, None, t!("settings-region-installed-done"));
            }
        }));
    }

    fn label(&self, name: &str) -> String {
        self.names.get(name).cloned().unwrap_or_else(|| plain_name(name))
    }

    fn told(&mut self, kit: &mut Kit) {
        let (lang, fmt) = saved();
        self.tell(kit, lang, fmt);
    }

    fn tell(&mut self, kit: &mut Kit, lang: String, fmt: String) {
        let (lang_name, fmt_name) = (self.label(&lang), self.label(&fmt));
        let mut words = if lang == fmt {
            t!("settings-region-everything-in", lang = lang_name)
        } else {
            t!("settings-region-menus-in", lang = lang_name, fmt = fmt_name)
        };

        let waiting = lang != running_lang() || fmt != running_fmt();
        if waiting {
            words = t!("settings-region-with-wait", words = &words);
        }

        self.head = words;
        let count = self.installed.len();
        self.count = count;
        self.lang = lang;

        let wanted = fmt.clone();
        self.fmt = fmt;
        let lc = format!("LC_ALL={wanted}");
        let date: Vec<String> = ["env", &lc, "date", "+%x"].iter().map(|s| s.to_string()).collect();
        let number: Vec<String> = ["env", &lc, "printf", "%'.2f", "1234567.5"].iter().map(|s| s.to_string()).collect();
        let kept: Vec<String> = ["env", &lc, "locale", "-k", "LC_MONETARY", "LC_PAPER", "LC_MEASUREMENT"]
            .iter().map(|s| s.to_string()).collect();
        type Said = (bool, String, String);
        on_thread(move || (captured(&date, None), captured(&number, None), captured(&kept, None)),
                  kit.then::<Region, _>(move |me, _k, (date, number, kept): (Said, Said, Said)| {
            if me.fmt != wanted {
                return;
            }
            if kept.0 {
                let (money, paper) = money_and_paper(&kept.1);
                me.money = money;
                me.paper = paper;
            }
            if date.0 {
                me.date = date.1.trim().to_string();
            }
            if number.0 {
                me.number = number.1.trim().to_string();
            }
        }));
    }

    fn install(&mut self, kit: &mut Kit) {
        let Some((name, _)) = self.add.get(self.pick).cloned() else { return };
        if kit.working(ADD) {
            return;
        }

        kit.start(ADD, t!("settings-region-installing")).expect(30.0);
        kit.hush(ADD);
        let back = kit.then::<Region, Result<(), String>>(|me, k, done| {
            if let Err(why) = done {
                k.finish(ADD, false, "");
                k.say(ADD, &why, false, false);
                return;
            }

            k.finish(ADD, true, t!("settings-region-installed-done"));
            me.pending = true;
            me.load(k);
        });
        admin_run(&["locale", &name], back);
    }

    fn language(&self, kit: &Kit) -> Node<Msg> {
        let mut options: Vec<(String, String)> = self.installed.iter().map(|n| (n.clone(), self.label(n))).collect();
        options.sort_by_key(|(_, l)| l.to_lowercase());
        let labels: Vec<String> = options.iter().map(|(_, l)| l.clone()).collect();
        let codes: Vec<String> = options.iter().map(|(c, _)| c.clone()).collect();
        let at_lang = codes.iter().position(|c| *c == self.lang);
        let at_fmt = codes.iter().position(|c| *c == self.fmt);
        let lang_codes = codes.clone();
        let lang = dropdown("LANG", labels.clone(), at_lang, t!("settings-region-language"), move |i| {
            let v = lang_codes[i].clone();
            on::<Region>(move |me, k| me.choose_lang(k, &v))
        });
        let fmt = dropdown("formats", labels, at_fmt, t!("settings-region-formats"), move |i| {
            let v = codes[i].clone();
            on::<Region>(move |me, k| {
                let changes: Vec<(&str, Option<&str>)> = FORMATS.iter().map(|f| (*f, Some(v.as_str()))).collect();
                session_env::update(&changes, k, "formats", later());
                me.told(k);
            })
        });
        group(t!("settings-region-language")).span(5, 8).icon("preferences-desktop-locale-symbolic")
            .about([t!("settings-region-language-about"), t!("settings-region-language-sub")].join(" "))
            .rows(vec![
                line(t!("settings-region-language")).key("LANG").control(lang).node(kit),
                line(t!("settings-region-formats")).key("formats").control(fmt).node(kit),

                line(t!("settings-region-date-number")).sub(t!("settings-region-date-and-number", date = &self.date, number = &self.number)).node(kit),
                line(t!("settings-region-money-paper")).sub(self.money_words()).node(kit),
                line(t!("settings-region-in-use")).sub(self.label(&running_lang())).node(kit),
            ])
            .node(kit)
    }

    fn own_codes(&self) -> Vec<String> {
        let mut codes: Vec<String> = vec!["auto".to_string()];
        codes.extend(self.tongues.iter().map(|t| t.code.clone()));
        let own = self.own.trim();
        if !own.is_empty() && own != "auto" && !codes.iter().any(|c| c.eq_ignore_ascii_case(own)) {
            codes.push(own.to_string());
        }
        codes
    }

    fn tongue(&self, kit: &Kit) -> Node<Msg> {
        let codes = self.own_codes();
        let named = |c: &String| self.tongues.iter().find(|t| t.code == *c).map(|t| t.name.clone()).unwrap_or_else(|| language_names::own_name(c));
        let options: Vec<String> = codes.iter().map(|c| if c == "auto" { t!("settings-language-auto").to_string() } else { named(c) }).collect();
        let own = self.own.trim();
        let at = codes.iter().position(|c| if own.is_empty() { c == "auto" } else { c.eq_ignore_ascii_case(own) });
        let pick = dropdown("language", options, at, t!("settings-language-pick-tip"), move |i| {
            let v = codes[i].clone();
            on::<Region>(move |me, k| me.choose_own(k, &v))
        });
        let mut rows = vec![line(t!("settings-language-pick")).key("language").control(pick).node(kit),
                            line(t!("settings-language-how-complete")).node(kit)];
        for tongue in &self.tongues {
            let tip = t!("settings-language-share", done = tongue.done, all = tongue.all);
            let mut one = line(tongue.name.clone()).below(fill_bar_good(Some(tongue.share()), tip));
            if tongue.code != "en" && language_names::VARIANTS.contains(&tongue.code.as_str()) {
                one = one.sub(t!("settings-language-variant"));
            } else if tongue.machine {
                one = one.sub(t!("settings-language-machine"));
            }
            rows.push(one.node(kit));
            if self.loaded {
                if let Some(script) = font_missing(&tongue.code, &self.fonts) {
                    rows.push(line(t!("fonts-missing", language = &tongue.name, package = script.package)).node(kit));
                }
            }
        }
        group(t!("settings-language-heading")).span(5, 8).icon("preferences-desktop-locale-symbolic")
            .about([t!("settings-language-about"), t!("settings-language-next-start")].join(" ")).rows(rows).node(kit)
    }

    fn choose_lang(&mut self, kit: &mut Kit, code: &str) {
        if session_env::update(&[("LANG", Some(code))], kit, "LANG", later()) {
            self.relanguage(kit, None);
        }
        self.told(kit);
    }

    fn choose_own(&mut self, kit: &mut Kit, code: &str) {
        if kit.settle("language", set_setting("language", code), not_saved()).is_some() {
            self.own = code.to_string();
            self.relanguage(kit, None);
            self.told(kit);
        }
    }

    pub(super) fn relanguage(&mut self, kit: &mut Kit, changed: Option<&str>) {
        let tags = live_language::for_choice(&self.own, &saved().0);
        let in_use = changed.is_some_and(|c| tags.iter().any(|t| t.eq_ignore_ascii_case(c)));
        if in_use || live_language::differs(&tags) {
            live_language::switch(tags);
            kit.asks.send(Do::Then(Box::new(Settings::relanguage)));
        }
    }

    fn money_words(&self) -> String {
        if self.paper.is_empty() { t!("settings-region-looking").to_string() } else { t!("settings-region-money-and-paper", money = &self.money, paper = &self.paper) }
    }

    fn remove(&mut self, kit: &mut Kit, name: String) {
        if kit.working(ADD) {
            return;
        }
        kit.start(ADD, t!("settings-region-removing")).expect(30.0);
        kit.hush(ADD);
        let back = kit.then::<Region, Result<(), String>>(|me, k, done| {
            if let Err(why) = done {
                k.finish(ADD, false, "");
                k.say(ADD, &why, false, false);
                return;
            }
            k.finish(ADD, true, t!("settings-region-removed-done"));
            me.pending = true;
            me.load(k);
        });
        admin_run(&["locale-remove", &name], back);
    }

    fn kept(&self, kit: &Kit) -> Node<Msg> {
        let mut list: Vec<&String> = self.installed.iter().collect();
        list.sort_by_key(|n| self.label(n).to_lowercase());
        let rows = list.into_iter().map(|n| {
            let used = *n == self.lang || *n == self.fmt;
            let row = line(self.label(n)).sub(n.clone());
            if used {
                row.sub(t!("settings-region-in-use-stays")).node(kit)
            } else {
                let name = n.clone();
                row.control(confirm(&format!("language-{n}"), "user-trash-symbolic", t!("settings-region-remove-language"),
                                    move |me: &mut Region, k| me.remove(k, name.clone()))).node(kit)
            }
        }).collect();
        group(t!("settings-region-kept")).span(4, 7).icon("preferences-desktop-locale-symbolic")
            .about(t!("settings-region-kept-about")).rows(rows).node(kit)
    }

    fn adding(&self, kit: &Kit) -> Node<Msg> {
        let card = group(t!("settings-region-add")).span(4, 7).icon(INSTALL_ICON)
            .about(t!("settings-region-add-about"));
        if self.add.is_empty() {
            return card.rows(vec![line(t!("settings-region-all-installed")).key(ADD).node(kit),
                                  line(t!("settings-region-installed")).sub(t!("settings-region-count", n = self.count)).node(kit)]).node(kit);
        }
        let labels: Vec<String> = self.add.iter().map(|(_, l)| l.clone()).collect();
        let pick = dropdown("add-pick", labels, Some(self.pick), t!("settings-region-install-another"),
                            |i| on::<Region>(move |me, _k| me.pick = i));
        let go = job(kit, ADD, INSTALL_ICON, t!("settings-region-install-this"),
                     on::<Region>(|me, k| me.install(k)));
        let n = self.add.len();
        card.rows(vec![
            line(t!("settings-region-install-another")).key(ADD).controls(vec![pick, go]).node(kit),
            line(t!("settings-region-ready")).sub(t!("settings-region-more", n = n))
                .node(kit),
            line(t!("settings-region-installed")).sub(t!("settings-region-count", n = self.count)).node(kit),
        ]).node(kit)
    }
}

impl Page for Region {
    fn id(&self) -> &'static str {
        "region"
    }
    fn title(&self) -> &'static str {
        t!("settings-region-title")
    }
    fn icon(&self) -> &'static str {
        "preferences-desktop-locale-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-region-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-region-help")
    }
    fn shown(&mut self, kit: &mut Kit) {
        if self.words.tag.is_empty() {
            self.words_for(self.words_tag());
        }
        self.load(kit);
    }

    fn conf_changed(&mut self, _kit: &mut Kit) {
        self.own = setting("language");
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let mut items = vec![headline(self.head.clone())];
        if self.loaded {
            items.extend([self.language(kit), self.tongue(kit), self.own_words(kit), self.adding(kit), self.kept(kit)]);
        } else {

            let looking = |name: &str| line(name).sub(t!("settings-region-looking")).node(kit);
            items.push(group(t!("settings-region-language")).span(5, 8).icon("preferences-desktop-locale-symbolic")
                .rows(vec![looking(t!("settings-region-language")), looking(t!("settings-region-formats")), looking(t!("settings-region-date-number")),
                           looking(t!("settings-region-money-paper")), looking(t!("settings-region-in-use"))]).node(kit));
            items.push(group(t!("settings-region-add")).span(4, 7).icon(INSTALL_ICON)
                .rows(vec![looking(t!("settings-region-install-another")), looking(t!("settings-region-ready")), looking(t!("settings-region-installed"))]).node(kit));
        }
        page(kit, self.title(), self.help(), items)
    }
}

#[cfg(test)]
impl Region {

    pub(crate) fn made_up() -> Region {
        let names: HashMap<String, String> = [("en_GB.UTF-8", "English (United Kingdom)"), ("fr_FR.UTF-8", "French (France)")]
            .iter().map(|(c, n)| (c.to_string(), n.to_string())).collect();
        Region { loaded: true, installed: vec!["en_GB.UTF-8".into(), "fr_FR.UTF-8".into()], names,
                 add: vec![("de_DE.UTF-8".into(), "German (Germany)".into()), ("it_IT.UTF-8".into(), "Italian (Italy)".into()),
                           ("es_ES.UTF-8".into(), "Spanish (Spain)".into())],
                 lang: "en_GB.UTF-8".into(), fmt: "en_GB.UTF-8".into(),
                 head: "Everything is in English (United Kingdom).".into(), count: 2,
                 date: "10/10/26".into(), number: "1,234,567.50".into(), money: "\u{a3}".into(),
                 paper: "A4 paper, metric measures".into(), ..Region::default() }
    }
}

#[cfg(test)]
mod font_tests {
    use super::*;

    #[test]
    fn a_language_is_flagged_only_when_its_script_font_is_missing() {
        let plain = vec!["DejaVu Sans".to_string()];
        assert_eq!(font_missing("hi", &plain).map(|s| s.package), Some("fonts-noto-core"));
        assert_eq!(font_missing("zh-Hans", &plain).map(|s| s.package), Some("fonts-noto-cjk"));
        assert!(font_missing("hi", &["Noto Sans Devanagari".to_string()]).is_none());
        for code in ["en", "fr", "ru", "sw", "vi"] {
            assert!(font_missing(code, &plain).is_none(), "{code}");
        }
    }

    #[test]
    fn the_missing_font_note_follows_the_language_and_falls_back_to_english() {
        use crate::toolkit::lang::{english, with_catalogue, Bundle, Catalogue};
        let args = || t!("fonts-missing", language = "Hindi", package = "fonts-noto-core");
        assert_eq!(args(), "Hindi needs a font that isn't installed yet. Install the package fonts-noto-core to show it properly.");
        let made_up = "fonts-missing = { $language }: manque { $package }.\n";
        with_catalogue(Catalogue::new(Bundle::from_text("fr", made_up), english()), || {
            assert_eq!(args(), "Hindi: manque fonts-noto-core.");
        });
    }
}

#[cfg(test)]
mod switch_now_tests {
    use super::*;
    use crate::common::live_language::switched;
    use crate::common::shipped_languages::in_language;
    use crate::settings::ui::Page;
    use crate::toolkit::ui::App;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn settings(events: &crate::toolkit::event::Loop) -> (Settings, Rc<RefCell<Vec<Do>>>) {
        let got: Rc<RefCell<Vec<Do>>> = Rc::default();
        let (keep, ev) = (got.clone(), events.clone());
        let asks = events.post(move |d: Do| {
            keep.borrow_mut().push(d);
            ev.quit();
        });
        let pages: Vec<Box<dyn Page>> = vec![Box::new(Region::made_up()), Box::new(crate::settings::panels::thanks::Thanks::default())];
        (Settings { kit: Kit::new(events.clone(), asks, None, None), pages, current: 0, search: String::new(), tall: 800.0 }, got)
    }

    #[test]
    fn choosing_a_language_shows_settings_in_it_at_once() {
        if crate::common::scratch_home::inside().is_none() {
            return crate::common::scratch_home::rerun(module_path!(), "choosing_a_language_shows_settings_in_it_at_once");
        }
        std::env::remove_var("LC_ALL");
        std::env::remove_var("LC_MESSAGES");
        let events = crate::toolkit::event::Loop::new();
        let (mut s, got) = settings(&events);
        type Pick = fn(&mut Region, &mut Kit);
        let own: Pick = |r, k| r.choose_own(k, "fr");
        let system: Pick = |r, k| {
            r.choose_own(k, "auto");
            r.choose_lang(k, "de_DE.UTF-8");
        };
        for (pick, tag, title) in [(own, "fr", "R\u{e9}glages"), (system, "de", "Einstellungen")] {
            let behind: *const dyn Page = &*s.pages[1];
            s.with::<Region>(pick);
            assert_eq!(switched(), Some(vec![tag.to_string()]), "{tag}: the words were not switched");

            for _ in 0..50 {
                if !std::ptr::addr_eq(behind, &*s.pages[1]) {
                    break;
                }
                let ev = events.clone();
                events.after(std::time::Duration::from_millis(200), move || ev.quit());
                events.run();
                let told: Vec<Do> = got.borrow_mut().drain(..).collect();
                for d in told {
                    if let Do::Then(f) = d {
                        f(&mut s);
                    }
                }
            }
            assert!(!std::ptr::addr_eq(behind, &*s.pages[1]), "{tag}: the page behind keeps the old words");
            in_language(tag, || {
                assert_eq!(s.window_title().as_deref(), Some(title), "{tag}");
                assert_eq!(s.window_title(), None, "the title is told once");
            });
        }
        assert_eq!(setting("language"), "auto");
        assert_eq!(session_env::get("LANG").as_deref(), Some("de_DE.UTF-8"));
        s.with::<Region>(|r, k| r.choose_own(k, "tlh"));
        assert_eq!(switched(), None, "no words for it: nothing changes");
    }
}

#[cfg(test)]
mod picked_language_tests {
    use crate::toolkit::lang::choice_from;

    #[test]
    fn each_language_picked_is_the_one_a_program_starts_in() {
        let lang = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("share/lang");
        let mut tried = 0;
        for dir in std::fs::read_dir(lang).unwrap().flatten().filter(|d| d.path().is_dir()) {
            let code = dir.file_name().to_string_lossy().into_owned();
            let tags = choice_from(Some(code.clone()), None, None, Some("en_GB.UTF-8".into()));
            assert_eq!(tags.first(), Some(&code), "{code} picked, but programs would start in {tags:?}");
            tried += 1;
        }
        assert!(tried >= 15, "only {tried} languages were tried");
        assert_eq!(choice_from(Some("auto".into()), None, None, Some("de_DE.UTF-8".into())).first().map(String::as_str), Some("de-DE"));
    }
}

#[cfg(test)]
mod show_the_minimum_tests {
    const GONE: &[&str] = &[
        "settings-region-formats-sub", "settings-region-needs-password", "settings-language-how-complete-sub", "settings-words-language-sub",
        "settings-ring-open-click", "settings-ring-open-super", "settings-ring-favourites-about", "settings-ring-notify-show-about",
        "settings-ring-clip-forget-about", "settings-ring-voice-about", "settings-snap-to-have", "settings-snap-reinstall",
        "settings-snap-choose-btrfs", "settings-snap-none-line-sub", "settings-snap-kept-until-you", "settings-snap-free-space",
        "settings-sounds-moment-login-about", "settings-sounds-moment-plug-about", "settings-sounds-moment-unplug-about",
        "settings-sounds-moment-battery-about", "settings-sounds-moment-notification-about", "settings-sounds-add-formats",
        "settings-sounds-none-yet-about", "settings-startup-command-about", "settings-startup-none-about", "settings-startup-app-about",
        "settings-startup-cmd-about", "settings-tailscale-switch-sub-first", "settings-tailscale-switch-sub", "settings-tailscale-no-devices-sub",
        "settings-tailscale-sign-in-sub", "settings-tailscale-connect-sub", "settings-tailscale-sign-out-sub", "settings-users-admin-can",
        "settings-users-new-account-sub", "settings-users-opens-login", "settings-wallpaper-wallpaper-about", "settings-wallpaper-picture-about",
        "settings-wallpaper-fit-about", "settings-wallpaper-colour-shown", "settings-wallpaper-rotate-about", "settings-wallpaper-every-about",
        "settings-wallpaper-order-about", "settings-wallpaper-next-about", "settings-wallpaper-no-pictures-about",
    ];
    const PANELS: &[&str] = &[
        include_str!("region.rs"), include_str!("region/words.rs"), include_str!("ring.rs"), include_str!("snapshots.rs"), include_str!("sounds.rs"),
        include_str!("startup.rs"), include_str!("tailscale.rs"), include_str!("users.rs"), include_str!("wallpaper/cards.rs"),
    ];

    #[test]
    fn no_line_under_a_control_only_explains_it() {
        let lang = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("share/lang");
        for dir in std::fs::read_dir(lang).unwrap().flatten().filter(|d| d.path().is_dir()) {
            for file in std::fs::read_dir(dir.path()).unwrap().flatten() {
                let text = std::fs::read_to_string(file.path()).unwrap_or_default();
                for line in text.lines() {
                    let key = line.split(" =").next().unwrap_or("");
                    assert!(!GONE.contains(&key), "{} still has {key}", file.path().display());
                }
            }
        }
        for code in PANELS {
            let code = code.split("\n#[cfg(test)]").next().unwrap();
            for at in code.match_indices(".sub(t!(\"") {
                let key = code[at.0 + 8..].split('"').next().unwrap();
                let explains = key.ends_with("-about") || key.ends_with("-sub");
                assert!(!explains || key == "settings-tailscale-firewall-sub", "a line under a control explains it: {key}");
            }
        }
    }
}
