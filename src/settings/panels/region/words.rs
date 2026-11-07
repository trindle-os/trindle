//! "Your own words" in Settings, Language.

use super::Region;
use crate::common::run;
use crate::common::files::set_setting;
use crate::settings::ui::{confirm, dropdown, group, line, no_chooser, not_saved, on, Kit, Msg, INSTALL_ICON};
use crate::settings::wordsheet as sheet;
use crate::toolkit::bus::Listen;
use crate::toolkit::chooser::{self, Ask};
use crate::toolkit::t;
use crate::toolkit::widget::{button, entry, Node};

const EXPORT: &str = "words:export";
const IMPORT: &str = "words:import";
const RESET: &str = "words:reset";
const OFFER: &str = "words:offer";
const START: &str = "words:start";

#[derive(Default)]
pub struct State {
    pub(super) tag: String,
    asking: Option<Listen>,
    name: String,
}

fn start_tag(own: &str, detected: &[String]) -> String {
    let own = own.trim();
    if sheet::tag_ok(own) && own != "auto" {
        return own.to_string();
    }
    detected.iter().find(|t| sheet::tag_ok(t) && !sheet::own::is_own(t)).cloned().unwrap_or_else(|| "en".to_string())
}

impl Region {
    pub(super) fn words_tag(&self) -> String {
        if sheet::tag_ok(&self.words.tag) {
            return self.words.tag.clone();
        }
        start_tag(&self.own, &crate::toolkit::lang::choice())
    }

    pub(super) fn words_for(&mut self, tag: String) {
        self.words.tag = tag;
    }

    fn words_export(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.session_for(EXPORT) else { return };
        let tag = self.words_tag();
        let ask = Ask { title: t!("settings-words-export-title").into(), name: Some(format!("trindle-words-{tag}.txt")), ..Ask::default() };
        let back = kit.then::<Region, Option<std::path::PathBuf>>(move |me, k, path| {
            me.words.asking = None;
            let Some(path) = path else { return k.unanswered(EXPORT) };
            let done = k.then::<Region, std::io::Result<()>>(|_me, k, res| {
                k.settle(EXPORT, res, t!("settings-words-not-saved"));
            });
            run::on_thread(move || {
                let text = sheet::export(&tag, sheet::rows(), &sheet::in_use(&tag, sheet::rows(), &sheet::base()));
                crate::common::files::write_whole(&path, text.as_bytes())
            }, done);
        });
        self.words.asking = chooser::save(&bus, &kit.events, ask, back);
        kit.started(EXPORT, self.words.asking.is_some(), no_chooser());
    }

    fn words_import(&mut self, kit: &mut Kit) {
        let Some(bus) = kit.session_for(IMPORT) else { return };
        let ask = Ask { title: t!("settings-words-import-title").into(), ..Ask::default() };
        let back = kit.then::<Region, Option<std::path::PathBuf>>(|me, k, path| {
            me.words.asking = None;
            let Some(path) = path else { return k.unanswered(IMPORT) };
            let done = k.then::<Region, Result<String, sheet::Why>>(|me, k, res| match res {
                Ok(tag) => {
                    me.words_for(tag);
                    k.say(IMPORT, t!("settings-words-imported"), true, false);
                    me.load(k);
                }
                Err(why) => k.say(IMPORT, &why.words(), false, true),
            });
            run::on_thread(move || {
                sheet::take(&path, sheet::rows(), &sheet::base())
            }, done);
        });
        self.words.asking = chooser::open(&bus, &kit.events, ask, back);
        kit.started(IMPORT, self.words.asking.is_some(), no_chooser());
    }

    fn words_start(&mut self, kit: &mut Kit) {
        let name = self.words.name.clone();
        let done = kit.then::<Region, Result<String, sheet::Why>>(|me, k, res| match res {
            Ok(tag) => {
                me.words.name.clear();
                me.words_for(tag);
                k.say(START, t!("settings-words-started"), true, false);
                me.load(k);
            }
            Err(why) => k.say(START, &why.words(), false, true),
        });
        run::on_thread(move || sheet::own::start(&sheet::base(), &name, &crate::toolkit::lang::folders()), done);
    }

    fn words_remove(&mut self, kit: &mut Kit) {
        let tag = self.words_tag();
        let gone = tag.clone();
        let done = kit.then::<Region, std::io::Result<()>>(move |me, k, res| {
            if k.settle(RESET, res, t!("settings-words-not-saved")).is_none() {
                return;
            }
            if me.own.trim().eq_ignore_ascii_case(&tag) && k.settle("language", set_setting("language", "auto"), not_saved()).is_some() {
                me.own = "auto".to_string();
            }
            me.words.tag.clear();
            me.load(k);
        });
        run::on_thread(move || sheet::own::remove(&sheet::base(), &gone), done);
    }

    fn words_reset(&mut self, kit: &mut Kit) {
        let tag = self.words_tag();
        let done = kit.then::<Region, std::io::Result<()>>(|me, k, res| {
            if k.settle(RESET, res, t!("settings-words-not-saved")).is_some() {
                me.load(k);
            }
        });
        run::on_thread(move || sheet::reset(&sheet::base(), &tag), done);
    }

    pub(super) fn own_words(&self, kit: &Kit) -> Node<Msg> {
        let tag = self.words_tag();
        let tongues: Vec<(String, String)> = self.tongues.iter().filter(|t| sheet::tag_ok(&t.code)).map(|t| (t.code.clone(), t.name.clone())).collect();
        let at = tongues.iter().position(|(c, _)| *c == tag);
        let codes: Vec<String> = tongues.iter().map(|(c, _)| c.clone()).collect();
        let pick = dropdown(kit, "words-language", tongues.iter().map(|(_, n)| n.clone()).collect(), at, t!("settings-words-language-tip"), move |i| {
            let v = codes[i].clone();
            on::<Region>(move |me, _k| me.words_for(v.clone()))
        });
        let export = button("document-send-symbolic", t!("settings-words-export")).on_press(on::<Region>(|me, k| me.words_export(k)));
        let import = button(INSTALL_ICON, t!("settings-words-import")).on_press(on::<Region>(|me, k| me.words_import(k)));
        let own = sheet::own::is_own(&tag);
        let reset = if own {
            confirm(kit, RESET, "edit-undo-symbolic", t!("settings-words-reset"), |me: &mut Region, k| me.words_remove(k))
        } else {
            button("edit-undo-symbolic", t!("settings-words-reset")).on_press(on::<Region>(|me, k| me.words_reset(k)))
        };
        let name = entry(self.words.name.clone(), t!("settings-words-name-tip"), |t| on::<Region>(move |me, _| me.words.name = t.clone()))
            .on_submit(on::<Region>(|me, k| me.words_start(k)));
        let start = button("list-add-symbolic", t!("settings-words-start")).on_press(on::<Region>(|me, k| me.words_start(k)));
        let offer = button("mail-send-symbolic", t!("settings-words-offer")).on_press(crate::settings::ui::link(OFFER, crate::report::send::SHEETS_URL.to_string()));
        let rows = vec![
            line(t!("settings-words-rule")).sub(t!("settings-words-next-start")).node(kit),
            line(t!("settings-words-language")).sub(t!("settings-words-language-sub")).key("words-language").control(pick).node(kit),
            line(t!("settings-words-start")).sub(t!("settings-words-start-sub")).key(START).control(start).below(name).node(kit),
            line(t!("settings-words-export")).sub(t!("settings-words-export-sub")).key(EXPORT).control(export).node(kit),
            line(t!("settings-words-import")).sub(t!("settings-words-import-sub")).key(IMPORT).control(import).node(kit),
            line(t!("settings-words-offer")).sub(t!("settings-words-offer-sub")).key(OFFER).control(offer).node(kit),
            line(t!("settings-words-reset")).sub(if own { t!("settings-words-reset-own-sub") } else { t!("settings-words-reset-sub") }).key(RESET).control(reset).node(kit),
        ];
        group(t!("settings-words-heading")).span(5, 8).icon("preferences-desktop-locale-symbolic")
            .about(t!("settings-words-about")).rows(rows).node(kit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_sheet_starts_on_the_language_the_programs_are_shown_in_when_none_is_chosen() {
        let detected = vec!["fr-CA".to_string(), "fr".to_string()];
        assert_eq!(start_tag("auto", &detected), "fr-CA");
        assert_eq!(start_tag("", &detected), "fr-CA");
        assert_eq!(start_tag("de", &detected), "de", "a language the person chose wins");
        assert_eq!(start_tag("auto", &[]), "en");
        assert_eq!(start_tag("../x", &["../y".to_string()]), "en", "never a tag that is not safe as a folder name");
        assert_eq!(start_tag("auto", &["x-klingon".to_string(), "x".to_string(), "de".to_string()]), "de", "a language of your own is only ever chosen by name");
    }
}
