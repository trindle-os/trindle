//! The Store's Plugins page.

use crate::toolkit::t;
use std::collections::HashMap;

use super::pages::pictures::icon;
use super::{go, listing, Msg, Store};
use crate::common::access;
use crate::common::filling::Dial;
use crate::common::run;
use crate::common::stopping::Stop;
use crate::common::updating::{self, Who};
use super::update_page;
use crate::plugins::{self, catalogue, Plugin};
use crate::common::held::{Held, Thing};
use crate::settings::ui::{in_words, INSTALL_ICON, UPDATE_ICON};
use crate::toolkit::theme;
use crate::toolkit::widget::{button, card, column, even_grid, label, main_action, note, row, Length, Node};
use super::screen::{screen, title_row, words_card};

pub(super) fn open_settings_tip() -> &'static str {
    t!("store-plugins-open-settings")
}
const REMOVE_ICON: &str = "user-trash-symbolic";
pub(super) const SETTINGS_ICON: &str = "preferences-system-symbolic";
pub(super) const PLAIN_ICON: &str = "application-x-addon-symbolic";

#[derive(Default)]
pub struct PluginPage {
    loaded: bool,
    turn: u64,
    offers: Vec<catalogue::Offer>,
    own_said: Option<String>,
    pub(super) installed: Vec<Plugin>,
    said: HashMap<String, String>,
    standing: HashMap<String, (usize, usize)>,
    glows: HashMap<String, u32>,
}

#[cfg(test)]
pub(super) fn made_up(loaded: bool, installed: Vec<Plugin>, offers: Vec<catalogue::Offer>, standing: &[(&str, (usize, usize))],
                      own_said: Option<&str>, said: &[(&str, &str)]) -> PluginPage {
    PluginPage {
        loaded, installed, offers, own_said: own_said.map(String::from),
        standing: standing.iter().map(|(k, v)| (k.to_string(), *v)).collect(),
        said: said.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        ..PluginPage::default()
    }
}

fn start_again(s: &mut Store) {
    let back = s.then(|s: &mut Store, res: Result<(), String>| match res {
        Ok(()) => opened(s),
        Err(e) => s.plugins.own_said = Some(t!("store-plugins-not-done", why = e)),
    });
    super::updates::admin_watching(&["plugin-catalogue-restart"], |_| {}, back);
}

pub fn opened(s: &mut Store) {
    read(s, true);
}

pub fn opened_by_itself(s: &mut Store) {
    let net = !s.upd.off;
    read(s, net);
}

fn read(s: &mut Store, net: bool) {
    let p = &mut s.plugins;
    p.installed = plugins::installed();
    p.standing = standing_of(&p.installed);
    p.glows.clear();
    p.turn += 1;
    let turn = p.turn;
    let back = s.then(move |s: &mut Store, (offers, own_said): (Vec<catalogue::Offer>, Option<String>)| {
        if s.plugins.turn != turn {
            return;
        }
        s.plugins.loaded = true;
        let before = std::mem::take(&mut s.plugins.offers);
        s.plugins.offers = offers.into_iter().map(|mut o| {
            if !net && o.icon.is_none() {
                o.icon = before.iter().find(|b| b.entry.id == o.entry.id).and_then(|b| b.icon.clone());
            }
            o
        }).collect();
        s.plugins.own_said = own_said;
    });
    if net {
        run::on_thread(catalogue::gather, back);
    } else {
        run::on_thread(|| (catalogue::offers_kept(), None), back);
    }
}

pub(super) fn may_remove() -> bool {
    crate::toolkit::window::persons_own(crate::common::person_only::PLUGIN_REMOVE)
}

fn standing_of(installed: &[Plugin]) -> HashMap<String, (usize, usize)> {
    installed.iter().map(|p| (p.id.clone(), plugins::standing(p))).collect()
}

pub(super) fn second(s: &mut Store) -> bool {
    let standing = standing_of(&s.plugins.installed);
    let changed = standing != s.plugins.standing;
    s.plugins.standing = standing;
    changed
}

pub(super) fn waiting(s: &Store) -> Vec<(Plugin, catalogue::Offer)> {
    waiting_in(&s.plugins)
}

fn waiting_in(p: &PluginPage) -> Vec<(Plugin, catalogue::Offer)> {
    catalogue::updates_for(&p.installed, &p.offers)
}

pub(super) fn waits_for_you() -> &'static str {
    t!("store-plugins-waits-for-you")
}

pub(super) fn waiting_detail(mine: &Plugin, offer: &catalogue::Offer) -> String {
    let more = catalogue::said_to_ask_more(mine, offer);
    if more.is_empty() {
        t!("store-plugins-waiting", mine = &mine.version, new = &offer.entry.version)
    } else {
        t!("store-plugins-waiting-more", mine = &mine.version, new = &offer.entry.version, more = in_words(&more), waits = waits_for_you())
    }
}

fn headline(p: &PluginPage, other: Option<Who>, kept: &Held) -> String {
    if let Some(who) = other {
        return t!("store-installed-other-busy", who = who.sentence(), back = come_back());
    }
    let n = p.installed.len();
    let offered = p.installed.iter().filter(|i| newer_offer(p, i).is_some()).count();
    let updates = p.installed.iter().filter(|i| newer_offer(p, i).is_some() && !kept.plugin(&i.id)).count();
    let held = offered - updates;
    let held_back = (held > 0).then(|| t!("store-plugins-count-held", n = held));
    let ready = p.offers.iter().filter(|o| !p.installed.iter().any(|i| i.id == o.entry.id)).count();
    if !p.loaded && n == 0 {
        return t!("store-plugins-looking").into();
    }
    let installed = t!("store-plugins-count-installed", n = n);
    match (n, updates, ready) {
        (0, _, 0) => t!("store-plugins-none").into(),
        (0, _, r) => t!("store-plugins-none-ready", n = r),
        (_, 0, _) if offered > 0 => match held_back {
            Some(held) => t!("store-plugins-line-held", installed = installed, held = held),
            None => t!("store-plugins-line-installed", installed = installed),
        },
        (_, 0, _) => t!("store-plugins-line-current", installed = installed),
        (_, u, _) => {
            let have = t!("store-plugins-count-have-update", n = u);
            match held_back {
                Some(held) => t!("store-plugins-line-updates-held", installed = installed, have = have, held = held),
                None => t!("store-plugins-line-updates", installed = installed, have = have),
            }
        }
    }
}

pub(super) fn come_back() -> &'static str {
    t!("store-plugins-come-back")
}

pub(super) fn newer_offer<'a>(p: &'a PluginPage, installed: &Plugin) -> Option<&'a catalogue::Offer> {
    catalogue::update_of(&p.offers, installed)
}

pub(super) fn picture_of(p: &PluginPage, id: &str) -> Option<String> {
    let mine = p.installed.iter().find(|i| i.id == id);
    let offer = p.offers.iter().find(|o| o.entry.id == id);
    offer.and_then(|o| o.icon.clone()).or_else(|| mine.and_then(|i| i.ring.as_ref().map(|r| r.0.clone())))
        .map(|f| f.to_string_lossy().into_owned())
}

pub(super) fn open_settings() {
    run::spawn(&["trindle-settings", "--panel", "plugins"]);
}

pub(super) fn offered(p: &PluginPage, id: &str) -> bool {
    p.offers.iter().any(|o| o.entry.id == id)
}

fn asks_line(asks: &[String]) -> String {
    let names: Vec<String> = asks.iter().map(|a| access::ask(a).name.to_string()).collect();
    if names.is_empty() {
        t!("store-plugins-asks-nothing").into()
    } else {
        t!("store-plugins-asks", list = in_words(&names))
    }
}

pub fn view(s: &Store) -> Node<Msg> {
    let p = &s.plugins;
    let mut all = Vec::new();
    all.extend(cards(p, &s.dials, super::update_page::busy(s), &s.held, &s.upd));
    all.push(places(s));
    screen(title_row(t!("store-plugins-title"), Vec::new()), headline(p, s.other, &s.held), all)
}

fn cards(p: &PluginPage, dials: &HashMap<String, Dial>, held: bool, kept: &Held, upd: &update_page::UpdatesPage) -> Option<Node<Msg>> {
    let mut ids: Vec<(String, String)> = p.installed.iter().map(|i| (i.name.to_lowercase(), i.id.clone())).collect();
    for o in &p.offers {
        if !ids.iter().any(|(_, id)| *id == o.entry.id) {
            ids.push((o.entry.name.to_lowercase(), o.entry.id.clone()));
        }
    }
    ids.sort();
    (!ids.is_empty()).then(|| {
        even_grid(theme::apps_list().card_least, ids.iter().map(|(_, id)| card_of(p, dials, held, kept, upd, id)).collect()).gap(theme::bento().page_gap)
    })
}

pub(super) fn problem(s: &Store, id: &str) -> Option<String> {
    s.plugins.said.get(id).cloned()
}

pub(super) fn page_of(words: &str) -> &'static str {
    crate::plugins::help::of_words(words).unwrap_or(super::help::PLUGIN)
}

pub(super) fn say(s: &mut Store, id: &str, words: String) {
    s.plugins.said.insert(id.to_string(), words);
}

pub(super) fn job_in(dials: &HashMap<String, Dial>, held: bool, key: &str, icon: &str, tip: &str, press: impl Fn(&mut Store) + 'static) -> Node<Msg> {
    let b = button(icon, tip).key(key);
    match dials.get(key) {
        Some(d) if d.running() => d.round(b),
        _ => b.on_press(go(press)).off_because(crate::common::ui::wait_while(held)),
    }
}

fn card_of(p: &PluginPage, dials: &HashMap<String, Dial>, held: bool, kept: &Held, upd: &update_page::UpdatesPage, id: &str) -> Node<Msg> {
    let mine = p.installed.iter().find(|i| i.id == id);
    let offer = p.offers.iter().find(|o| o.entry.id == id);
    let name = mine.map(|i| i.name.clone()).or_else(|| offer.map(|o| o.entry.name.clone())).unwrap_or_default();
    let summary = offer.map(|o| o.entry.summary.clone()).filter(|t| !t.is_empty())
        .or_else(|| mine.map(|i| i.summary.clone()).filter(|t| !t.is_empty()))
        .unwrap_or_default();
    let asks = offer.map(|o| o.entry.permissions.clone()).or_else(|| mine.map(|i| i.asks.clone())).unwrap_or_default();
    let picture = picture_of(p, id).unwrap_or_else(|| PLAIN_ICON.into());
    let update = mine.and_then(|i| newer_offer(p, i));
    let entry = Entry { id, mine, offer, update, kept_back: Thing::plugin(id).filter(|t| kept.has(t)) };
    let version = entry.version();
    let key = format!("pl:{id}");
    let mut buttons = entry.buttons(dials, held, &name);

    buttons.splice(0..0, upd.stop_button(&key));
    let mut words = vec![listing::listing(Some(icon(picture, theme::apps_list().tile_icon)), name.clone(), listing::two_lines(summary), Vec::new(), buttons)];
    words.extend(needs_rows(p, mine, id, &asks));
    words.push(note(version));
    if upd.stopped.contains(&key) {
        words.push(note(update_page::words::stopped_row()));
    }
    if let Some(w) = p.said.get(id) {
        words.push(super::pages::links::red_about(w.clone(), page_of(w)));
    }
    let mut c = card(column(words).gap(theme::sheet().gap)).height(Length::Fill).key(format!("plugin:{id}"));
    if let Some(turn) = p.glows.get(id) {
        c = c.glow(*turn);
    }
    c
}

struct Entry<'a> {
    id: &'a str,
    mine: Option<&'a Plugin>,
    offer: Option<&'a catalogue::Offer>,
    update: Option<&'a catalogue::Offer>,
    kept_back: Option<Thing>,
}

impl Entry<'_> {
    fn version(&self) -> String {
        match (self.mine, self.offer) {
            (Some(i), _) if self.kept_back.is_some() =>
                t!("store-plugins-version-held", words = match self.update {
                    Some(u) => super::update_page::words::held_words(Some(&i.version), Some(u.entry.version.as_str())),
                    None => super::update_page::words::held_only(Some(&i.version)),
                }),
            (Some(i), _) if self.update.is_some() => t!("store-plugins-version-ready", mine = &i.version, new = self.update.map_or("", |u| u.entry.version.as_str())),
            (Some(i), _) => t!("store-plugins-version-installed", version = &i.version),
            (None, Some(o)) => t!("store-plugins-version-offered", version = &o.entry.version),
            (None, None) => String::new(),
        }
    }

    fn buttons(&self, dials: &HashMap<String, Dial>, held: bool, name: &str) -> Vec<Node<Msg>> {
        let id = self.id;
        let key = format!("pl:{id}");
        let mut buttons = Vec::new();
        match (self.mine, self.offer) {
            (None, Some(o)) => {
                let o = o.clone();
                buttons.push(job_in(dials, held, &key, INSTALL_ICON, &t!("store-plugins-install", name = name), move |s| super::pages::install::pressed_install(s, |s| install(s, &o))));
            }
            (Some(_), _) => {
                let gone = format!("pl-remove:{id}");
                let busy = dials.get(&key).is_some_and(Dial::running);
                let own = id.to_string();
                let remove = crate::toolkit::widget::remove(REMOVE_ICON, t!("store-app-remove")).key(gone.clone());
                buttons.push(remove.on_press(go(move |s| uninstall(s, &own))).off_because(crate::common::ui::wait_while(busy)));
                buttons.push(button(SETTINGS_ICON, open_settings_tip()).on_press(go(|_| open_settings())));
                if let Some(thing) = self.kept_back.clone() {
                    let row = key.clone();
                    buttons.push(button(super::update_page::HOLD_ICON, t!("store-plugins-release", name = name)).key(super::update_page::rows::hold_key(&key)).on(true)
                        .on_press(go(move |s| super::update_page::hold::set_held(s, &thing, false, &row))).off_because(crate::common::ui::wait_while(held)));
                } else if let Some(u) = self.update.cloned() {
                    buttons.push(job_in(dials, held, &key, UPDATE_ICON, &t!("store-plugins-update-to", version = &u.entry.version), move |s| install(s, &u)));
                }
            }
            (None, None) => {}
        }
        buttons
    }
}

fn needs_rows(p: &PluginPage, mine: Option<&Plugin>, id: &str, asks: &[String]) -> Vec<Node<Msg>> {
    let mut words = Vec::new();
    let (asked, allowed) = mine.and_then(|i| p.standing.get(&i.id).copied()).unwrap_or((0, 0));
    match mine.and_then(|i| plugins::needs_words(&i.name, asked, allowed)) {
        Some(needs) => {
            let id = id.to_string();
            words.push(row(vec![
                label(needs).width(Length::Fill),
                main_action(plugins::ALLOW_ICON, plugins::allow_word()).on_press(go(move |_| {
                    plugins::ask_again(&id);
                })),
            ]).gap(theme::sheet().gap).stacks());
            words.push(note(asks_line(asks)));
        }
        None if mine.is_some() && asked > 0 => words.push(note(plugins::settled_words(asked))),
        None => words.push(note(asks_line(asks))),
    }
    words
}

fn come_from() -> &'static str {
    t!("store-plugins-come-from")
}

fn places(s: &Store) -> Node<Msg> {
    let mut all = vec![note(come_from())];
    all.extend(s.plugins.own_said.as_ref().map(|why| super::pages::links::red_about(why.clone(), crate::plugins::help::LIST)));
    if s.plugins.own_said.as_deref().is_some_and(catalogue::is_paused) {
        all.push(button(UPDATE_ICON, catalogue::start_again()).on_press(go(start_again)));
    }
    words_card(t!("store-plugins-where"), all)
}

pub(super) fn install(s: &mut Store, offer: &catalogue::Offer) {
    install_as(s, offer, false);
}

pub(super) fn install_unseen(s: &mut Store, offer: &catalogue::Offer) {
    install_as(s, offer, true);
}

fn install_as(s: &mut Store, offer: &catalogue::Offer, unseen: bool) {
    let hold = match updating::begin(Who::Apps) {
        Ok(hold) => hold,
        Err(other) => {
            s.other = Some(other);
            return;
        }
    };
    let id = offer.entry.id.clone();
    let key = format!("pl:{id}");
    let mut d = Dial::default();
    d.expect(4.0);
    s.dials.insert(key.clone(), d);
    s.plugins.said.remove(&id);
    s.working += 1;
    let offer = offer.clone();
    let stop = Stop::default();
    s.upd.stops.insert(key.clone(), stop.clone());
    s.upd.stopped.remove(&key);
    let fetching = stop.clone();
    let mine = s.plugins.installed.iter().find(|p| p.id == id).map(|p| p.version.clone()).unwrap_or_default();
    let back = s.then(move |s: &mut Store, res: Result<plugins::Installed, String>| fetch_ended(s, &id, &key, &stop, res));
    run::on_thread(move || {
        let res = if unseen {
            catalogue::fetch_update_unseen(&offer, &plugins::folders(), &fetching).and_then(|u| match u {
                catalogue::Unseen::In(done) => Ok(*done),
                catalogue::Unseen::AsksMore(more) => Err(t!("store-plugins-waiting-more", mine = &mine, new = &offer.entry.version,
                                                             more = in_words(&more), waits = waits_for_you())),
            })
        } else {
            catalogue::fetch_install(&offer, &plugins::folders(), &fetching)
        };
        drop(hold);
        res
    }, back);
}

fn fetch_ended(s: &mut Store, id: &str, key: &str, stop: &Stop, res: Result<plugins::Installed, String>) {
    s.job_over();
    if update_page::stopping::stopped(s, key, stop, res.is_ok()) {
        return;
    }
    if let Some(d) = s.dials.get_mut(key) {
        d.finish(res.is_ok(), "");
    }
    s.dials.remove(key);
    match res {
        Ok(done) => {
            if let plugins::Background::NotRunning(why) = &done.background {
                s.plugins.said.insert(id.to_string(), not_running(why));
            }
            let p = done.plugin;
            *s.plugins.glows.entry(p.id.clone()).or_insert(0) += 1;
            s.plugins.installed = plugins::installed();
            s.plugins.standing = standing_of(&s.plugins.installed);
            if !p.asks.is_empty() {
                run::spawn(&["trindle-plugin", "ask", &p.id]);
            }
        }
        Err(e) => {
            s.plugins.said.insert(id.to_string(), e);
        }
    }
}

fn not_running(why: &str) -> String {
    t!("store-plugins-not-running", why = why)
}

fn uninstall(s: &mut Store, id: &str) {
    if !may_remove() {
        return;
    }
    let done = plugins::remove(id).map_err(|e| e.to_string());
    removed(s, id, done, true);
}

pub(super) fn removed(s: &mut Store, id: &str, done: Result<(), String>, read: bool) {
    match done {
        Ok(()) => {
            s.plugins.said.remove(id);
            s.plugins.glows.remove(id);
            s.plugins.installed.retain(|p| p.id != id);
            if read {
                s.plugins.installed = plugins::installed();
            }
            if let Some(thing) = Thing::plugin(id).filter(|t| s.held.has(t)) {
                let _ = s.held.release(&thing);
            }
        }
        Err(e) => {
            s.plugins.said.insert(id.to_string(), t!("store-plugins-not-removed", why = e));
        }
    }
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    pub fn offer(id: &str, version: &str) -> catalogue::Offer {
        catalogue::Offer {
            entry: catalogue::Entry {
                id: id.into(), name: id.into(), version: version.into(), summary: String::new(), permissions: Vec::new(),
                icon: None, icon_sha256: None, archive: "a.tar.zst".into(), sha256: "0".repeat(64),
            },
            icon: None,
        }
    }

    pub fn page_with_an_update() -> PluginPage {
        PluginPage { loaded: true, installed: vec![mine("org.example.One", "1.0")],
                     offers: vec![offer("org.example.One", "1.1"), offer("org.example.Two", "1.0")], ..PluginPage::default() }
    }

    pub fn first_offer(p: &PluginPage) -> catalogue::Offer {
        p.offers[0].clone()
    }

    pub fn mine(id: &str, version: &str) -> Plugin {
        Plugin { id: id.into(), name: id.into(), version: version.into(), summary: String::new(), asks: Vec::new(),
                 ring: None, fields: Vec::new(), background: None, listen: Vec::new(), shortcuts: Vec::new() }
    }

    #[test]
    fn the_where_plugins_come_from_card_is_only_the_information() {
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: crate::store::Do| {});
        let mut s = Store::new(&events, asks, None);
        let said = places(&s).said();
        assert_eq!(said, ["Where plugins come from", come_from()]);
        assert!(come_from().contains("submit") && come_from().contains("tested") && !come_from().contains('\u{2014}'));
        assert!(places(&s).buttons().is_empty(), "nothing to press");
        s.plugins.own_said = Some("Refused.".into());
        assert_eq!(places(&s).said(), ["Where plugins come from", come_from(), "Refused.", &format!("button: {}", t!("common-more"))]);
        let pressable = |s: &Store| places(s).buttons().iter().map(|b| b.0.clone()).collect::<Vec<_>>();
        assert_eq!(pressable(&s), [t!("common-more")]);
        s.plugins.own_said = Some(catalogue::paused().into());
        assert_eq!(pressable(&s), [t!("common-more"), catalogue::start_again()]);
        let source = crate::common::source::code(include_str!("plugin_page.rs"));
        let page = source.split("#[cfg(test)]").next().unwrap_or_default();
        assert!(!page.contains(&["folder", "-symbolic"].concat()) && !page.contains("list_card(\"") , "the card has an icon or a row");
    }

    #[test]
    fn the_updates_page_lists_each_plugin_with_a_newer_version() {
        let mut p = PluginPage { installed: vec![mine("a.b", "1.0.0"), mine("c.d", "2.0.0"), mine("e.f", "3.0.0")], ..PluginPage::default() };
        p.offers = vec![offer("a.b", "1.1.0"), offer("c.d", "2.0.0"), offer("e.f", "2.9.0"), offer("g.h", "1.0.0")];
        let waiting: Vec<String> = waiting_in(&p).into_iter().map(|(m, o)| format!("{} {}", m.id, o.entry.version)).collect();
        assert_eq!(waiting, ["a.b 1.1.0"], "only a newer version counts, and only for an installed plugin");
        p.offers.clear();
        assert!(waiting_in(&p).is_empty());
    }

    #[test]
    fn a_row_says_when_the_new_version_asks_for_more() {
        let mut have = mine("a.b", "1.0");
        have.asks = vec!["network".into()];
        let mut same = offer("a.b", "1.1");
        same.entry.permissions = vec!["network".into()];
        assert_eq!(waiting_detail(&have, &same), "Version 1.0 now, 1.1 is ready");
        let mut more = offer("a.b", "1.1");
        more.entry.permissions = vec!["network".into(), "microphone".into(), "files".into()];
        assert_eq!(waiting_detail(&have, &more), "Version 1.0 now, 1.1 is ready. It asks for more than before: your microphone and \
                                                 your Documents, Downloads, Pictures, Music, Videos and Desktop folders. This update needs a new permission. It is waiting for you.");
        let mut p = PluginPage::default();
        (p.installed, p.offers) = (vec![have.clone()], vec![same.clone()]);
        let card = card_of(&p, &HashMap::new(), false, &Held::default(), &update_page::UpdatesPage::default(), "a.b").said();
        assert!(card.contains(&"Version 1.0 installed; 1.1 is ready.".to_string()), "{card:?}");
        assert_eq!(not_running("it did not start within 15 seconds."),
                   "It is installed, but its background part is not running: it did not start within 15 seconds.");
    }

    #[test]
    fn a_plugin_s_fetch_can_be_stopped_and_the_card_says_it_was() {
        let (p, mut upd) = (page_with_an_update(), update_page::UpdatesPage::default());
        let tips = |upd: &update_page::UpdatesPage| card_of(&p, &HashMap::new(), false, &Held::default(), upd, "org.example.One")
            .buttons().into_iter().map(|(tip, _)| tip).collect::<Vec<_>>();
        assert!(!tips(&upd).contains(&"Stop".to_string()));
        let mut command = std::process::Command::new("sleep");
        command.arg("300");
        let (mut child, proc) = run::start_group(&mut command).unwrap();
        let stop = Stop::default();
        stop.hold(proc.clone());
        upd.stops.insert("pl:org.example.One".into(), stop.clone());
        let during = tips(&upd);
        assert_eq!(during.first().map(String::as_str), Some("Stop"), "{during:?}");
        assert_eq!(during.len(), tips(&update_page::UpdatesPage::default()).len() + 1, "and the rest stay as they were");
        assert!(stop.press());
        run::collect(&mut child, &proc);
        let events = crate::toolkit::event::Loop::new();
        let asks = events.post(|_: super::super::Do| {});
        let mut s = Store::new(&events, asks, None);
        s.upd = upd;
        assert!(update_page::stopping::stopped(&mut s, "pl:org.example.One", &stop, false));
        let after = card_of(&p, &HashMap::new(), false, &Held::default(), &s.upd, "org.example.One").said();
        assert!(after.contains(&"Stopped. Press Update to carry on.".to_string()), "{after:?}");
        assert!(!after.iter().any(|w| w.contains("download") || w.contains("failed")), "no red words: {after:?}");
    }

    #[test]
    fn the_headline_says_how_things_stand() {
        let mut p = PluginPage::default();
        assert!(headline(&p, None, &Held::default()).starts_with("Looking"));
        p.loaded = true;
        assert_eq!(headline(&p, None, &Held::default()), "No plugins are installed, and none are on offer yet.");
        p.offers = vec![offer("a.b", "1.0"), offer("c.d", "2.0")];
        assert_eq!(headline(&p, None, &Held::default()), "No plugins are installed yet; 2 are ready to install.");
        p.installed = vec![mine("a.b", "1.0")];
        assert_eq!(headline(&p, None, &Held::default()), "1 plugin is installed, all up to date.");
        p.installed.push(mine("c.d", "1.5"));
        assert_eq!(headline(&p, None, &Held::default()), "2 plugins are installed, and 1 has an update.");
    }

    #[test]
    fn a_card_builds_in_every_permission_state() {
        use crate::toolkit::theme::Theme;
        use crate::toolkit::ui::Ui;
        let mut asking = mine("a.b", "1.0");
        asking.asks = vec!["network".into(), "notifications".into()];
        for allowed in [0, 1, 2] {
            let mut p = PluginPage { loaded: true, installed: vec![asking.clone(), mine("c.d", "1.0")], ..PluginPage::default() };
            p.offers = vec![offer("e.f", "1.0")];
            p.standing.insert("a.b".into(), (2, allowed));
            p.standing.insert("c.d".into(), (0, 0));
            assert_eq!(plugins::needs_words("a.b", 2, allowed).is_some(), allowed < 2);
            let cards: Vec<Node<Msg>> = ["a.b", "c.d", "e.f"].iter().map(|id| card_of(&p, &HashMap::new(), false, &Held::default(), &update_page::UpdatesPage::default(), id)).collect();
            let mut ui = Ui::new(column(cards), Theme::new(false, "#fbc711"));
            let mut pix = tiny_skia::Pixmap::new(420, 900).unwrap();
            ui.paint(&mut pix.as_mut(), 1.0);
        }
    }

    #[test]
    fn what_a_plugin_does_wraps_onto_a_second_line() {
        use crate::toolkit::theme::Theme;
        use crate::toolkit::ui::Ui;
        let tall = |words: &str| Ui::new(listing::two_lines::<Msg>(words), Theme::new(true, "#fbc711")).natural_height(240.0);
        let one = tall("A kitchen timer.");
        let two = tall("Use one computer's mouse and keyboard on another beside it.");
        assert!((two - 2.0 * one).abs() <= 2.0, "two lines, not one cut short: {two} against {one}");
        let many = tall(&"Use one computer's mouse and keyboard on another beside it. ".repeat(6));
        assert_eq!(many, two, "and never a third");
        assert_eq!(tall(""), one, "an empty line keeps its room");
    }

    #[test]
    fn the_cards_of_a_row_are_the_same_height() {
        use crate::toolkit::theme::Theme;
        use crate::toolkit::ui::Ui;
        let mut asking = mine("a.b", "1.0");
        asking.asks = vec!["network".into(), "notifications".into()];
        let mut long = offer("e.f", "1.0");
        long.entry.summary = "Use one computer's mouse and keyboard on another beside it, over your own network.".into();
        let mut short = offer("g.h", "2.0");
        short.entry.summary = "A kitchen timer.".into();
        let mut p = PluginPage { loaded: true, installed: vec![asking, mine("c.d", "1.0")], offers: vec![long, short, offer("i.j", "1.0")],
                                 ..PluginPage::default() };
        p.standing.insert("a.b".into(), (2, 0));
        p.said.insert("g.h".into(), "It could not be fetched.".into());
        let ids = ["a.b", "c.d", "e.f", "g.h", "i.j"];
        let mut seen_rows = 0;
        for width in [1000u32, 700, 400] {
            let grid = cards(&p, &HashMap::new(), false, &Held::default(), &update_page::UpdatesPage::default()).expect("there are plugins");
            let mut ui = Ui::new(grid, Theme::new(true, "#fbc711"));
            ui.lay_at(width, 2000);
            let boxes = ui.boxes();
            let at: Vec<_> = ids.iter().map(|id| boxes.iter().find(|b| b.0.ends_with(&format!("plugin:{id}"))).expect("its card").1).collect();
            let wide = at[0].w;
            assert!(at.iter().all(|r| r.w == wide), "equal columns at {width}: {at:?}");
            for a in &at {
                for b in at.iter().filter(|b| b.y == a.y) {
                    assert_eq!(a.h, b.h, "cards in one row at {width}: {at:?}");
                    if b.x != a.x {
                        seen_rows += 1;
                    }
                }
            }
        }
        assert!(seen_rows > 0, "some rows held more than one card");
    }

    #[test]
    fn what_it_asks_for_is_in_plain_words() {
        assert_eq!(asks_line(&[]), "It asks for nothing.");
        assert_eq!(asks_line(&["network".into(), "notifications".into()]), "What it asks for: the internet and show notifications.");
    }
}
