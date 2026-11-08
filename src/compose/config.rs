//! What the compositor is told: shortcuts, key repeat, workspaces, gaps and title bar colours.

use std::path::{Path, PathBuf};

use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use smithay::input::keyboard::{xkb, Keysym, ModifiersState};

use crate::common::{files, sheet};
use crate::common::combos::Combo;
pub use crate::common::pointer::Pointer;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Mods {
    pub logo: bool,
    pub alt: bool,
    pub ctrl: bool,
    pub shift: bool,
}

impl Action {

    pub fn is_restart(&self) -> bool {
        matches!(self, Action::RestartParts | Action::RestartAll)
    }
}

impl Mods {
    pub fn held(state: &ModifiersState) -> Mods {
        Mods { logo: state.logo, alt: state.alt, ctrl: state.ctrl, shift: state.shift }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Execute(String),

    Plugin(String),
    Close,
    ToggleMaximize,
    ToggleFullscreen,
    Iconify,
    NextWindow,
    PreviousWindow,
    SnapLeft,
    SnapRight,
    Exit,

    RestartParts,

    RestartAll,

    Nothing,
}

#[derive(Clone, Debug)]
pub struct Keybind {
    pub mods: Mods,
    pub key: Keysym,

    pub on_release: bool,
    pub action: Action,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub keybinds: Vec<Keybind>,
    pub repeat_rate: i32,
    pub repeat_delay: i32,
    pub workspaces: usize,

    pub gap: i32,
    pub corner_radius: i32,

    pub desktop_double_click: Option<String>,
    pub pointer: Pointer,

    pub theme: Theme,

    reading_plugins: bool,

    pub older_programs: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TitleColours {
    pub background: [f32; 4],
    pub text: [f32; 4],
    pub button: [f32; 4],
    pub border: [f32; 4],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub active: TitleColours,
    pub inactive: TitleColours,

    pub edge: [f32; 4],

    pub shadow: [[f32; 4]; sheet::MOST_LAYERS],

    pub shadow_colour: [f32; 4],

    pub button_corner: f32,
}

impl Theme {

    pub(crate) fn of(light: bool) -> Theme {
        let layers = sheet::layers();
        let c = |name: &str| layers.colour(name, light).unwrap_or([0.0, 0.0, 0.0, 1.0]);
        let colours = |away: &str| TitleColours {
            background: c(&format!("trindle-title{away}-background")),
            text: c(&format!("trindle-title{away}-ink")),
            button: c(&format!("trindle-title{away}-ink")),
            border: c(&format!("trindle-title{away}-border")),
        };
        let mut shadow = [[0.0; 4]; sheet::MOST_LAYERS];
        for (to, layer) in shadow.iter_mut().zip(layers
                .each("trindle-float-shadow", light)
                .find_map(|t| sheet::shadow_layers(t, |c| sheet::plain_colour(c).map(|c| c[3])))
                .unwrap_or_default()) {
            *to = layer;
        }
        Theme {
            active: colours(""),
            inactive: colours("-away"),
            edge: c("trindle-window-edge"),
            shadow,
            shadow_colour: c("trindle-shadow"),
            button_corner: layers.number("trindle-icon-button-corner", light, 0.0, 999.0).unwrap_or(8.0),
        }
    }

    fn dark() -> Theme {
        Theme::of(false)
    }

    #[cfg(test)]
    fn light() -> Theme {
        Theme::of(true)
    }

    fn chosen() -> Theme {
        Theme::of(sheet::asked_light())
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::dark()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            keybinds: Vec::new(),
            repeat_rate: 25,
            repeat_delay: 600,
            workspaces: 1,
            gap: 0,
            corner_radius: 8,
            desktop_double_click: None,
            pointer: Pointer::load(),
            theme: Theme::default(),
            reading_plugins: false,
            older_programs: true,
        }
    }
}

fn system_rc() -> PathBuf {
    files::system_desktop("shortcuts.xml")
}

fn plugin_rc() -> PathBuf {
    crate::common::combos::plugin_keys_rc()
}

fn user_rc() -> PathBuf {
    files::trindle_config().join("shortcuts.xml")
}

pub fn sources() -> Vec<PathBuf> {
    let mut all = vec![system_rc(), plugin_rc(), user_rc(), files::trindle_conf(), files::system_desktop("environment"),
                       files::user_environment(), sheet::own_sheet()];
    all.extend(crate::keys_changed::sources());
    all
}

const WORKSPACES_ON: usize = 9;

#[derive(Default)]
struct RcPosition {

    path: Vec<String>,
    bind: Option<(Mods, Keysym, bool)>,
    action: Option<(String, Vec<(String, String)>)>,

    context: Option<String>,

    mouse_button: Option<String>,
}

impl Config {
    pub fn load() -> Config {
        let mut c = Config::default();
        for path in [system_rc(), plugin_rc(), user_rc()] {
            let text = files::read_text(&path);
            if !text.is_empty() {
                c.reading_plugins = path == plugin_rc();
                c.read_rc(&text);
                c.reading_plugins = false;
            }
        }
        c.add_setting_keys();
        c.theme = Theme::chosen();
        c.workspaces = if files::setting("workspaces") == "on" { WORKSPACES_ON } else { 1 };
        c.older_programs = crate::x11_rooms::allowed(&files::setting(crate::x11_rooms::SETTING));
        c
    }

    pub fn keybind(&self, mods: Mods, key: Keysym) -> Option<&Keybind> {
        let key = lower(key);
        self.keybinds.iter().rev().find(|k| k.mods == mods && k.key == key)
    }

    fn add(&mut self, bind: Keybind) {

        self.keybinds.retain(|k| !(k.mods == bind.mods && k.key == bind.key));
        if bind.action != Action::Nothing || !self.keybinds.is_empty() {
            self.keybinds.push(bind);
        }
    }

    fn add_setting_keys(&mut self) {
        use crate::common::combos::{LIST_COMMAND, LIST_KEYS, REPORT_COMMAND, REPORT_KEYS, SCREENSHOT_COMMAND, SCREENSHOT_KEYS};
        for (setting, action) in [(crate::restart::PARTS_KEYS, Action::RestartParts), (crate::restart::ALL_KEYS, Action::RestartAll),
                                  (SCREENSHOT_KEYS, Action::Execute(SCREENSHOT_COMMAND.into())), (LIST_KEYS, Action::Execute(LIST_COMMAND.into())),
                                  (REPORT_KEYS, Action::Execute(REPORT_COMMAND.into()))] {
            if let Some((mods, key)) = Combo::typed(&files::setting(setting)).ok().and_then(|c| parse_combo(&c.rc())) {
                self.add(Keybind { mods, key, on_release: false, action });
            }
        }
    }

    fn add_defaults(&mut self) {
        for (combo, action) in [
            ("A-Tab", Action::NextWindow),
            ("A-S-Tab", Action::PreviousWindow),
            ("A-F4", Action::Close),
            ("W-a", Action::ToggleMaximize),
            ("W-Left", Action::SnapLeft),
            ("W-Right", Action::SnapRight),
            ("W-Up", Action::ToggleMaximize),
            ("W-Down", Action::Iconify),
            ("F11", Action::ToggleFullscreen),
        ] {
            if let Some((mods, key)) = parse_combo(combo) {
                self.add(Keybind { mods, key, on_release: false, action });
            }
        }
    }

    pub(crate) fn read_rc(&mut self, text: &str) {
        let mut reader = Reader::from_str(text);
        reader.config_mut().trim_text(true);
        let mut at = RcPosition::default();
        loop {
            match reader.read_event() {
                Ok(Event::Eof) | Err(_) => break,
                Ok(Event::Start(e)) => self.rc_start(&mut at, &e, false),
                Ok(Event::Empty(e)) => self.rc_start(&mut at, &e, true),
                Ok(Event::Text(t)) => self.rc_text(&mut at, &t),
                Ok(Event::End(e)) => self.rc_end(&mut at, &e),
                Ok(_) => {}
            }
        }
    }

    fn rc_start(&mut self, at: &mut RcPosition, e: &BytesStart<'_>, empty: bool) {
        let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
        let attrs: Vec<(String, String)> = e.attributes().flatten()
            .map(|a| (String::from_utf8_lossy(a.key.as_ref()).to_string(),
                      a.normalized_value(quick_xml::XmlVersion::Implicit1_0).map(|v| v.to_string()).unwrap_or_default()))
            .collect();
        let attr = |k: &str| attrs.iter().find(|(n, _)| n == k).map(|(_, v)| v.clone());
        let inside = |p: &str| at.path.iter().any(|x| x == p);
        match name.as_str() {
            "default" if at.path.last().map(String::as_str) == Some("keyboard") => self.add_defaults(),
            "keybind" => {
                at.bind = attr("key").and_then(|k| parse_combo(&k))
                    .map(|(m, k)| (m, k, attr("onRelease").as_deref() == Some("yes")));
            }
            "context" => at.context = attr("name"),
            "mousebind" if at.context.is_some() => {
                at.mouse_button = match (attr("action").as_deref(), attr("button").as_deref()) {
                    (Some("Push"), _) => Some("Push".into()),
                    (Some("DoubleClick"), Some("Left")) => Some("DoubleLeft".into()),
                    (Some("DoubleClick"), _) => None,
                    _ => attr("button"),
                };
            }
            "action" if at.bind.is_some() || (at.context.is_some() && inside("mousebind")) => {
                at.action = Some((attr("name").unwrap_or_default(), attrs.clone()));
            }
            _ => {}
        }
        if !empty {
            at.path.push(name);
        } else if name == "action" {
            let mouse = if at.path.iter().any(|x| x == "mousebind") { mouse_at(&at.context, &at.mouse_button) } else { None };
            self.end_action(&mut at.bind, &mut at.action, mouse);
        }
    }

    fn rc_text(&mut self, at: &mut RcPosition, t: &BytesText<'_>) {
        let raw = t.decode().map(|s| s.to_string()).unwrap_or_default();
        let text = quick_xml::escape::unescape(&raw).map(|s| s.to_string()).unwrap_or(raw);
        let here: Vec<&str> = at.path.iter().map(String::as_str).collect();
        match here.as_slice() {
            [.., "keyboard", "repeatRate"] => {
                if let Ok(n) = text.parse() { self.repeat_rate = n; }
            }
            [.., "keyboard", "repeatDelay"] => {
                if let Ok(n) = text.parse() { self.repeat_delay = n; }
            }
            [.., "core", "gap"] => {
                if let Ok(n) = text.parse() { self.gap = n; }
            }
            [.., "theme", "cornerRadius"] => {
                if let Ok(n) = text.parse() { self.corner_radius = n; }
            }

            [.., "action", "command"] => {
                if let Some((_, attrs)) = at.action.as_mut() {
                    attrs.push(("command".into(), text));
                }
            }
            _ => {}
        }
    }

    fn rc_end(&mut self, at: &mut RcPosition, e: &BytesEnd<'_>) {
        let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
        let mouse = if at.path.iter().any(|x| x == "mousebind") { mouse_at(&at.context, &at.mouse_button) } else { None };
        at.path.pop();
        match name.as_str() {
            "action" => self.end_action(&mut at.bind, &mut at.action, mouse),
            "keybind" => at.bind = None,
            "mousebind" => at.mouse_button = None,
            "context" => at.context = None,
            _ => {}
        }
    }

    fn end_action(&mut self, bind: &mut Option<(Mods, Keysym, bool)>,
                  action: &mut Option<(String, Vec<(String, String)>)>, mouse: Option<(&str, &str)>) {
        let Some((name, attrs)) = action.take() else { return };
        let attr = |k: &str| attrs.iter().rev().find(|(n, _)| n == k).map(|(_, v)| v.clone());

        if self.reading_plugins && (name != "Execute" || mouse.is_some()) {
            return;
        }
        let act = match name.as_str() {
            "Execute" => match attr("command") {
                Some(c) if self.reading_plugins => Action::Plugin(c),
                Some(c) => Action::Execute(c),
                None => return,
            },
            "Close" => Action::Close,
            "ToggleMaximize" => Action::ToggleMaximize,
            "ToggleFullscreen" => Action::ToggleFullscreen,
            "Iconify" => Action::Iconify,
            "NextWindow" => Action::NextWindow,
            "PreviousWindow" => Action::PreviousWindow,
            "Exit" => Action::Exit,
            "SnapToEdge" => match attr("direction").as_deref() {
                Some("left") => Action::SnapLeft,
                Some("right") => Action::SnapRight,
                Some("up") => Action::ToggleMaximize,
                Some("down") => Action::Iconify,
                _ => return,
            },
            "None" => Action::Nothing,
            _ => return,
        };
        if let Some(place) = mouse {
            let run = match act {
                Action::Execute(c) => Some(c),
                _ => None,
            };

            if let ("Root", "DoubleLeft") = place { self.desktop_double_click = run }
            return;
        }
        if let Some((mods, key, on_release)) = *bind {
            self.add(Keybind { mods, key, on_release, action: act });
        }
    }
}

pub fn lower(key: Keysym) -> Keysym {
    let name = xkb::keysym_get_name(key);
    if name.chars().count() == 1 && name.chars().all(|c| c.is_ascii_uppercase()) {
        return xkb::keysym_from_name(&name.to_ascii_lowercase(), xkb::KEYSYM_NO_FLAGS);
    }
    key
}

fn parse_combo(text: &str) -> Option<(Mods, Keysym)> {
    let mut mods = Mods::default();
    let parts: Vec<&str> = text.split('-').collect();
    let (name, held) = parts.split_last()?;
    for m in held {
        match *m {
            "W" | "Mod4" => mods.logo = true,
            "A" | "Mod1" => mods.alt = true,
            "C" => mods.ctrl = true,
            "S" => mods.shift = true,
            _ => return None,
        }
    }
    let key = xkb::keysym_from_name(name, xkb::KEYSYM_NO_FLAGS);
    let key = if key.raw() == 0 { xkb::keysym_from_name(name, xkb::KEYSYM_CASE_INSENSITIVE) } else { key };
    (key.raw() != 0).then(|| (mods, lower(key)))
}

pub fn apply_environment() {
    for path in [files::system_desktop("environment"), files::user_environment()] {
        for (k, v) in files::read_env(Path::new(&path)) {
            std::env::set_var(k, v);
        }
    }
}

pub fn xkb_names() -> (String, String, String, Option<String>) {
    let get = |k: &str| std::env::var(k).unwrap_or_default();
    let options = get("XKB_DEFAULT_OPTIONS");
    (get("XKB_DEFAULT_LAYOUT"), get("XKB_DEFAULT_VARIANT"), get("XKB_DEFAULT_MODEL"),
     (!options.is_empty()).then_some(options))
}

fn mouse_at<'a>(context: &'a Option<String>, button: &'a Option<String>) -> Option<(&'a str, &'a str)> {
    Some((context.as_deref()?, button.as_deref()?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::combos::{Combo, BUILT_IN};

    fn trindle() -> Config {
        let mut c = Config { pointer: Pointer::load(), ..Config::default() };
        c.read_rc(&std::fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/desktop/shortcuts.xml")).unwrap());
        c
    }

    #[test]
    fn every_key_kept_in_settings_is_bound_and_clashes_with_no_other_shortcut() {
        let mut c = trindle();
        c.add_defaults();
        let before = c.keybinds.len();
        c.add_setting_keys();
        assert_eq!(c.keybinds.len(), before + crate::common::combos::SETTING_KEYS.len(), "a key kept in Settings took over another shortcut's keys, or one is missing");
        for (keys, command) in [("A-C-S-s", crate::common::combos::SCREENSHOT_COMMAND), ("W-k", crate::common::combos::LIST_COMMAND)] {
            let (m, k) = parse_combo(keys).unwrap();
            assert_eq!(c.keybind(m, k).map(|b| &b.action), Some(&Action::Execute(command.into())), "{keys}");
        }
        for (keys, want) in [("C-A-Escape", Action::RestartParts), ("C-A-S-Escape", Action::RestartAll)] {
            let (m, k) = parse_combo(keys).unwrap();
            assert_eq!(c.keybind(m, k).map(|b| &b.action), Some(&want), "{keys}");
            assert!(want.is_restart());
        }
        assert!(!Action::Close.is_restart());
    }

    #[test]
    fn a_file_of_keybinds_can_not_bind_a_restart() {
        let mut c = Config::default();
        c.read_rc(r#"<trindle_desktop><keyboard><keybind key="C-A-r"><action name="RestartAll" /></keybind><keybind key="C-A-p"><action name="RestartParts" /></keybind></keyboard></trindle_desktop>"#);
        assert!(c.keybinds.iter().all(|k| !k.action.is_restart()));
    }

    #[test]
    fn the_standard_shortcuts_are_the_ones_the_plugins_are_told_are_kept() {
        let mut c = Config::default();
        c.add_defaults();
        let mut standard: Vec<Combo> = c.keybinds.iter().map(|k| {
            Combo { logo: k.mods.logo, alt: k.mods.alt, ctrl: k.mods.ctrl, shift: k.mods.shift,
                    key: xkb::keysym_get_name(k.key) }
        }).collect();
        for told in BUILT_IN {
            let told = Combo::from_rc(told).unwrap();
            assert!(standard.contains(&told), "{told:?} is told as kept but is not a standard shortcut");
            standard.retain(|k| *k != told);
        }
        assert!(standard.is_empty(), "standard shortcuts a plugin is not told are kept: {standard:?}");
    }

    #[test]
    fn a_plugins_shortcut_runs_its_command_and_a_persons_own_wins_over_it() {
        let mut c = Config::default();
        c.read_rc(r#"<trindle_desktop><keyboard><keybind key="A-C-s"><action name="Execute" command="trindle-plugin run org.example.One shortcut switch" /></keybind></keyboard></trindle_desktop>"#);
        let (m, k) = parse_combo("C-A-s").unwrap();

        let mut p = Config { reading_plugins: true, ..Config::default() };
        p.read_rc(r#"<trindle_desktop><keyboard><keybind key="A-C-s"><action name="Execute" command="trindle-plugin run org.example.One shortcut switch" /></keybind><keybind key="A-C-q"><action name="Exit" /></keybind></keyboard></trindle_desktop>"#);
        assert_eq!(p.keybind(m, k).map(|b| &b.action),
                   Some(&Action::Plugin("trindle-plugin run org.example.One shortcut switch".into())));
        let (qm, qk) = parse_combo("C-A-q").unwrap();
        assert!(p.keybind(qm, qk).is_none(), "a plugin's file cannot quit the desktop");
        c.read_rc(r#"<trindle_desktop><keyboard><keybind key="A-C-s"><action name="Execute" command="trindle-plugin run org.example.One shortcut switch" /></keybind></keyboard></trindle_desktop>"#);
        assert_eq!(c.keybind(m, k).map(|b| &b.action),
                   Some(&Action::Execute("trindle-plugin run org.example.One shortcut switch".into())));
        c.read_rc(r#"<trindle_desktop><keyboard><keybind key="C-A-s"><action name="Execute" command="foot" /></keybind></keyboard></trindle_desktop>"#);
        assert_eq!(c.keybind(m, k).map(|b| &b.action), Some(&Action::Execute("foot".into())));
        assert!(sources().contains(&plugin_rc()));
    }

    #[test]
    fn trindles_own_shortcuts_are_read() {
        let c = trindle();
        let (m, k) = parse_combo("W-q").unwrap();
        assert_eq!(c.keybind(m, k).map(|b| &b.action), Some(&Action::Close));
        let (m, k) = parse_combo("Super_L").unwrap();
        let ring = c.keybind(m, k).unwrap();
        assert!(ring.on_release);
        assert_eq!(ring.action, Action::Execute("trindle-ring".into()));
        let (m, k) = parse_combo("W-A-s").unwrap();
        assert_eq!(c.keybind(m, k).map(|b| &b.action), Some(&Action::Execute("trindle-settings --screen-reader".into())));

        let (m, k) = parse_combo("W-S-s").unwrap();
        assert_eq!(c.keybind(m, k).map(|b| &b.action), Some(&Action::Execute("trindle-screenshot".into())));

        let (m, k) = parse_combo("W-Down").unwrap();
        assert_eq!(c.keybind(m, k).map(|b| &b.action), Some(&Action::Iconify));
        let (m, k) = parse_combo("A-Tab").unwrap();
        assert_eq!(c.keybind(m, k).map(|b| &b.action), Some(&Action::NextWindow));
    }

    #[test]
    fn the_rest_of_trindles_settings_are_read() {
        let c = trindle();
        assert_eq!((c.repeat_rate, c.repeat_delay, c.gap, c.corner_radius), (30, 400, 6, 10));
    }

    #[test]
    fn an_older_file_cannot_bring_back_the_desktop_click() {
        let mut c = trindle();
        c.read_rc(r#"<trindle_desktop><mouse><context name="Root">
            <mousebind button="Left" action="Press"><action name="Execute" command="foot" /></mousebind>
            <mousebind button="Middle" action="Press"><action name="Execute" command="foot" /></mousebind>
            </context></mouse></trindle_desktop>"#);
        assert_eq!(c.desktop_double_click.as_deref(), Some("trindle-ring"));
    }

    #[test]
    fn the_desktop_double_click_is_read_and_a_plugin_cannot_bind_it() {
        let c = trindle();
        assert_eq!(c.desktop_double_click.as_deref(), Some("trindle-ring"));
        let text = r#"<trindle_desktop><mouse><context name="Root">
            <mousebind button="Left" action="DoubleClick"><action name="Execute" command="foot" /></mousebind>
            <mousebind button="Right" action="Press"><action name="Execute" command="foot" /></mousebind>
            </context><context name="ScreenEdge">
            <mousebind button="Left" action="Press"><action name="Execute" command="foot" /></mousebind>
            </context></mouse></trindle_desktop>"#;
        let mut p = Config { reading_plugins: true, ..Config::default() };
        p.read_rc(text);
        assert!(p.desktop_double_click.is_none(),
                "a plugin's file bound a click on the desktop");
        let mut q = trindle();
        q.reading_plugins = true;
        q.read_rc(text);
        assert_eq!(q.desktop_double_click.as_deref(), Some("trindle-ring"), "a plugin's file changed the double click");
        let mut other = trindle();
        other.read_rc(r#"<trindle_desktop><mouse><context name="Root">
            <mousebind button="Left" action="Press"><action name="Execute" command="foot" /></mousebind>
            <mousebind button="Middle" action="DoubleClick"><action name="Execute" command="foot" /></mousebind>
            </context></mouse></trindle_desktop>"#);
        assert_eq!(other.desktop_double_click.as_deref(), Some("trindle-ring"), "only a left double click is bound");
    }

    #[test]
    fn light_and_dark_title_bars_differ() {
        assert_ne!(Theme::dark(), Theme::light());
        assert_eq!(Theme::light().active.background, [1.0, 1.0, 1.0, 1.0]);

        let byte = |c: [f32; 4]| c.map(|v| (v * 255.0).round() as u8);
        let dark = Theme::dark();
        assert_eq!(byte(dark.active.background), [0x0e, 0x17, 0x28, 255]);
        assert_eq!(byte(dark.active.text), [0xec, 0xf2, 0xfb, 255]);
        assert_eq!(byte(dark.inactive.background), [0x0a, 0x11, 0x1e, 255]);
        assert_eq!(byte(dark.inactive.text), [0xae, 0xbd, 0xd4, 255]);
        assert_eq!(byte(dark.active.border), [190, 215, 255, 23]);
        assert_eq!(byte(dark.inactive.border), [190, 215, 255, 15]);
        assert_eq!(byte(dark.edge), [190, 215, 255, 26]);
        let light = Theme::light();
        assert_eq!(byte(light.active.border), [0xd6, 0xd6, 0xd6, 255]);
        assert_eq!(byte(light.inactive.background), [0xed, 0xed, 0xed, 255]);
        assert_eq!(light.edge, [0.0, 0.0, 0.0, 0.10]);
    }

    #[test]
    fn windows_cast_the_style_sheets_float_shadow() {

        let dark = Theme::dark();
        assert_eq!(dark.shadow, [[22.0, 54.0, 0.0, 0.55], [3.0, 10.0, 0.0, 0.40], [0.0; 4]]);
        assert_eq!(dark.shadow_colour.map(|v| (v * 255.0).round() as u8), [0x02, 0x06, 0x0e, 255]);
        let light = Theme::light();
        assert_eq!(light.shadow, [[10.0, 28.0, 0.0, 0.18], [0.0; 4], [0.0; 4]]);

        let held = sheet::shadow_layers("0 90px 200px 40px rgba(0, 0, 0, 2)", |c| sheet::plain_colour(c).map(|c| c[3]));
        assert_eq!(held, None, "a colour past solid does not read");
        let held = sheet::shadow_layers("0 90px 200px 40px #000", |c| sheet::plain_colour(c).map(|c| c[3]));
        assert_eq!(held, Some(vec![[32.0, 64.0, 16.0, 1.0]]));
        assert_eq!(sheet::shadow_layers("none", |_| None), Some(Vec::new()));
    }

    #[test]
    fn a_persons_own_file_overrides() {
        let mut c = trindle();
        c.read_rc(r#"<trindle_desktop><keyboard><keybind key="W-q"><action name="Execute" command="foot" /></keybind></keyboard></trindle_desktop>"#);
        let (m, k) = parse_combo("W-q").unwrap();
        assert_eq!(c.keybind(m, k).map(|b| &b.action), Some(&Action::Execute("foot".into())));
    }

    #[test]
    fn the_shipped_shortcuts_bind_every_volume_and_brightness_key() {
        let mut c = Config::default();
        c.read_rc(include_str!("../../config/desktop/shortcuts.xml"));
        for name in ["XF86AudioRaiseVolume", "XF86AudioLowerVolume", "XF86AudioMute", "XF86AudioMicMute",
                     "XF86MonBrightnessUp", "XF86MonBrightnessDown"] {
            let (mods, key) = parse_combo(name).unwrap_or_else(|| panic!("{name} is not a key"));
            assert!(matches!(c.keybind(mods, key).map(|b| &b.action), Some(Action::Execute(_))), "{name} runs nothing");
        }
    }
}
