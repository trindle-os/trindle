//! Help.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::rc::Rc;

use image::{DynamicImage, GenericImageView};

use crate::settings::help_topics::{topics, Block, Topic};
use crate::settings::panels::network::banded;
use crate::settings::ui::{group, headline, icon, line, on, page, Kit, Msg, Page};
use crate::toolkit::picture;
use crate::toolkit::t;
use crate::toolkit::widget::{button, card, column, drawing, entry, label, note, row, Length, Node, Tone};

fn areas() -> [(&'static str, &'static str, &'static [&'static str]); 7] {
    [
        (t!("settings-help-area-start"), "go-home-symbolic",
         &["welcome", "ring", "favourites", "switcher", "notifications", "keys", "awake"]),
        (t!("settings-help-area-apps"), "view-app-grid-symbolic", &["apps", "games", "sources", "appimages", "older-programs", "defaults", "startup"]),
        (t!("settings-help-area-screen"), "video-display-symbolic",
         &["display", "nvidia", "secure-boot-key", "wallpaper", "look", "theme-files", "themes", "theming", "screenshots", "eye-dropper"]),
        (t!("settings-help-area-access"), "audio-input-microphone-symbolic", &["magnifier", "screen-reader", "voice"]),
        (t!("settings-help-area-devices"), "audio-speakers-symbolic",
         &["wifi", "sound", "sounds", "bluetooth", "bluetooth-connect", "bluetooth-codec", "printers", "files", "disks", "hardware-no-driver"]),
        (t!("settings-help-area-safe"), "document-revert-symbolic", &["updates", "snapshots", "moving", "trouble", "ai-programs"]),
        (t!("settings-help-area-computer"), "computer-symbolic", &["users", "language", "power", "activity", "about", "virtual-machine"]),
    ]
}

fn area_icon(id: &str) -> &'static str {
    areas().iter().find(|a| a.2.contains(&id)).map_or("help-browser-symbolic", |a| a.1)
}

fn pictures() -> PathBuf {
    let mut places = vec![
        PathBuf::from("/usr/share/trindle/help"),
        PathBuf::from("/usr/lib/trindle/share/help"),
    ];
    if let Ok(exe) = std::env::current_exe() {
        for dir in exe.ancestors().skip(1).take(4) {
            places.push(dir.join("share/help"));
        }
    }
    places.iter().find(|p| p.is_dir()).cloned().unwrap_or_else(|| places[0].clone())
}

fn text_of(blocks: &[Block]) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for b in blocks {
        match b {
            Block::P(t) | Block::Tip(t) => parts.push(t),
            Block::Steps(steps) => parts.extend(steps.iter()),
            Block::Img(_, caption) => parts.push(caption),
        }
    }
    parts.join(" ")
}

fn matches(topic: &Topic, words: &[&str]) -> bool {
    let hay = format!("{} {} {}", topic.title, topic.keywords, text_of(topic.blocks)).to_lowercase();
    words.iter().all(|w| hay.contains(w))
}

fn start() -> &'static str {
    t!("settings-help-start")
}

fn found_words(shown: usize, typed: &str) -> (String, Tone) {
    match shown {
        0 => (t!("settings-help-found-none", typed = typed), Tone::Warn),
        n => (t!("settings-help-found", n = n, typed = typed), Tone::Plain),
    }
}

fn picture_node(img: Rc<DynamicImage>, caption: &str) -> Node<Msg> {
    let (_, ih) = img.dimensions();
    drawing((1.0, ih.min(420) as f32), caption, move |canvas, _| {
        let (iw, ih) = img.dimensions();
        let (cw, ch) = (canvas.width() as f32, canvas.height() as f32);
        let fit = (cw / iw as f32).min(ch / ih as f32);
        let (w, h) = ((iw as f32 * fit) as u32, (ih as f32 * fit) as u32);
        let x = ((cw - w as f32) / 2.0) as i32;
        let y = ((ch - h as f32) / 2.0) as i32;
        picture::draw(canvas, &img, x, y, w, h);
    })
    .width(Length::Fill)
}

#[derive(Default)]
pub struct Help {
    typed: String,
    open: HashSet<&'static str>,
    started: bool,
    back_to: Option<String>,
    wanted: Option<&'static str>,
    turn: u32,
    pictures: RefCell<HashMap<&'static str, Option<Rc<DynamicImage>>>>,
}

impl Help {
    fn picture(&self, file: &'static str) -> Option<Rc<DynamicImage>> {
        self.pictures.borrow_mut().entry(file)
            .or_insert_with(|| picture::load(&pictures().join(file)).map(Rc::new))
            .clone()
    }

    fn body(&self, blocks: &[Block]) -> Node<Msg> {
        let mut b = Vec::new();
        for block in blocks {
            match block {
                Block::P(t) => b.push(label(*t).width(Length::Fill)),
                Block::Tip(t) => b.push(card(row(vec![icon("dialog-information-symbolic", 16.0), label(*t).width(Length::Fill)])
                    .gap(12.0).width(Length::Fill)).width(Length::Fill)),
                Block::Steps(steps) => {
                    let lines = steps.iter().enumerate().map(|(n, step)| {
                        row(vec![label(format!("{}.", n + 1)), label(*step).width(Length::Fill)])
                            .gap(12.0).width(Length::Fill)
                    }).collect();
                    b.push(column(lines).gap(8.0).width(Length::Fill));
                }
                Block::Img(file, caption) => {
                    let Some(img) = self.picture(file) else { continue };
                    b.push(picture_node(img, caption));
                    b.push(column(vec![note(*caption)]).centred().width(Length::Fill));
                }
            }
        }
        column(b).gap(12.0).width(Length::Fill)
    }

    fn reading(&self, kit: &Kit, open: &[&'static Topic]) -> Node<Msg> {
        let mut rows = Vec::new();
        for topic in open {
            let id = topic.id;
            let mut close = button("window-close-symbolic", t!("settings-help-close", title = topic.title))
                .on_press(on::<Help>(move |me, _| { me.open.remove(id); }));
            if self.wanted == Some(id) {
                close = close.grab(self.turn);
            }
            rows.push(line(topic.title).icon(area_icon(id)).control(close).below(self.body(topic.blocks)).node(kit));
        }
        if rows.is_empty() {
            rows.push(line(t!("settings-help-nothing-open")).sub(t!("settings-help-nothing-open-sub")).node(kit));
        }
        group(t!("settings-help-reading")).icon("help-browser-symbolic").rows(rows).span(7, 8).node(kit)
    }

    fn area(&self, kit: &Kit, name: &str, icon: &str, topics: &[&'static Topic]) -> Node<Msg> {
        let rows = topics.iter().map(|t| {
            let id = t.id;
            let open = self.open.contains(id);
            let tip = if open { t!("settings-help-close", title = t.title) } else { t!("settings-help-read", title = t.title) };
            let read = button(if open { "pan-down-symbolic" } else { "pan-end-symbolic" }, tip).on(open)
                .on_press(on::<Help>(move |me, _| {
                    if !me.open.remove(id) {
                        me.open.insert(id);
                    }
                }));
            line(t.title).control(read).node(kit)
        }).collect::<Vec<_>>();
        group(name).icon(icon).rows(banded(rows)).span(4, 5).node(kit)
    }

    fn search(&self, kit: &Kit) -> Node<Msg> {
        let mut rows = vec![entry(self.typed.clone(),
                                  t!("settings-help-search-placeholder"),
                                  |t| on::<Help>(move |me, _| {
                                      me.typed = t.clone();
                                      let text = t.to_lowercase();
                                      let words: Vec<&str> = text.split_whitespace().collect();
                                      if !words.is_empty() {
                                          me.open = topics().iter().filter(|tp| matches(tp, &words)).map(|tp| tp.id).collect();
                                      }
                                  }))
            .key("help-search").width(Length::Fill)];
        if let Some(title) = &self.back_to {
            rows.push(line(t!("settings-help-back", title = title))
                .control(button("go-previous-symbolic", t!("settings-help-back", title = title)).on_press(on::<Help>(|_, k| k.go_back())))
                .node(kit));
        }
        rows.push(line(t!("settings-help-program")).key("help-program")
            .control(button("help-browser-symbolic", t!("settings-help-program-open")).on_press(on::<Help>(|_, k| {
                let opened = crate::common::run::spawn(&["trindle-help"]);
                k.started("help-program", opened, t!("settings-help-program-not-opened"));
            })))
            .node(kit));
        group(t!("settings-help-search")).icon("system-search-symbolic").rows(rows).span(3, 4).node(kit)
    }
}

impl Page for Help {
    fn id(&self) -> &'static str {
        "help"
    }
    fn title(&self) -> &'static str {
        t!("settings-help-title")
    }
    fn icon(&self) -> &'static str {
        "help-browser-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-help-keywords")
    }

    fn shown(&mut self, kit: &mut Kit) {
        if !self.started {
            self.started = true;
            self.open.insert("welcome");
        }
        self.back_to = kit.came_from().and_then(|id| {
            crate::settings::panels::all().iter().find(|p| p.id() == id).map(|p| p.title().to_string())
        });
    }

    fn open_topic(&mut self, _kit: &mut Kit, id: &str) {
        let Some(topic) = topics().iter().find(|t| t.id == id) else { return };
        self.typed.clear();
        self.open.clear();
        self.open.insert(topic.id);
        self.wanted = Some(topic.id);
        self.turn += 1;
    }

    fn view(&self, kit: &Kit) -> Node<Msg> {
        let text = self.typed.to_lowercase();
        let typed = text.trim();
        let words: Vec<&str> = text.split_whitespace().collect();
        let found: Vec<&'static Topic> = topics().iter().filter(|t| matches(t, &words)).collect();
        let mut items = vec![match typed {
            "" => headline(kit, start()),
            _ => headline(kit, found_words(found.len(), typed).0),
        }];
        items.push(self.search(kit));
        let open: Vec<&'static Topic> = found.iter().copied().filter(|t| self.open.contains(t.id)).collect();
        items.push(self.reading(kit, &open));

        for (name, icon, ids) in areas() {
            let here: Vec<&'static Topic> = found.iter().copied().filter(|t| ids.contains(&t.id)).collect();
            if !here.is_empty() {
                items.push(self.area(kit, name, icon, &here));
            }
        }
        page(kit, self.title(), self.help(), items)
    }
}

#[cfg(test)]
impl Help {
    pub(crate) fn flipped() -> Vec<Help> {
        let open = |ids: &[&'static str]| Help { open: ids.iter().copied().collect(), started: true, ..Help::default() };
        vec![open(&[]), open(&["welcome"]), open(&["welcome", "ring"]), open(&["ring"])]
    }
}

#[cfg(test)]
mod tests {
    use super::{areas, topics};

    #[test]
    fn every_topic_has_one_card() {
        for t in topics() {
            let found = areas().iter().filter(|a| a.2.contains(&t.id)).count();
            assert_eq!(found, 1, "{} is in {found} cards", t.id);
        }
        let named: usize = areas().iter().map(|a| a.2.len()).sum();
        assert_eq!(named, topics().len(), "a card names a topic that is not there");
    }
}
