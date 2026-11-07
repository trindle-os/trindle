//! Default applications, kept in ~/.config/mimeapps.list.

use crate::common::apps::{self, Entry};
use crate::settings::ui::{app_icon, dropdown, group, headline, icon, in_words, line, on, page, Kit, Msg, Page};
use crate::toolkit::widget::{row, Length, Node};
use crate::toolkit::t;

const KINDS: &[(&str, &[&str])] = &[
    ("Web", &["x-scheme-handler/http", "x-scheme-handler/https", "text/html"]),
    ("Mail", &["x-scheme-handler/mailto"]),
    ("Folders", &["inode/directory"]),
    ("Text", &["text/plain"]),
    ("Pictures", &["image/png", "image/jpeg", "image/webp", "image/gif"]),
    ("Music", &["audio/mpeg", "audio/flac", "audio/ogg", "audio/x-wav"]),
    ("Video", &["video/mp4", "video/x-matroska", "video/webm"]),
    ("pdf", &["application/pdf"]),
    ("Calendar", &["text/calendar"]),
];

fn cards() -> [(&'static str, &'static str, &'static [usize], &'static str); 3] {
    [
    (t!("settings-default-apps-card-internet"), "web-browser-symbolic", &[0, 1, 8],
     t!("settings-default-apps-card-internet-about")),
    (t!("settings-default-apps-card-files"), "folder-symbolic", &[2, 3, 7],
     t!("settings-default-apps-card-files-about")),
    (t!("settings-default-apps-card-media"), "image-x-generic-symbolic", &[4, 5, 6],
     t!("settings-default-apps-card-media-about")),
    ]
}

#[derive(Default)]
struct Openers {
    could: Vec<Entry>,
    chosen: Option<Entry>,
}

static NONE: Openers = Openers { could: Vec::new(), chosen: None };

#[derive(Default)]
pub struct DefaultApps {
    kinds: Vec<Openers>,
}

fn kind_name(at: usize) -> &'static str {
    match at {
        0 => t!("settings-default-apps-kind-web"),
        1 => t!("settings-default-apps-kind-mail"),
        2 => t!("settings-default-apps-kind-folders"),
        3 => t!("settings-default-apps-kind-text"),
        4 => t!("settings-default-apps-kind-pictures"),
        5 => t!("settings-default-apps-kind-music"),
        6 => t!("settings-default-apps-kind-video"),
        7 => t!("settings-default-apps-kind-pdf"),
        8 => t!("settings-default-apps-kind-calendar"),
        _ => "",
    }
}

fn kind_in_sentence(at: usize) -> &'static str {
    match at {
        0 => t!("settings-default-apps-kind-web-in-sentence"),
        1 => t!("settings-default-apps-kind-mail-in-sentence"),
        2 => t!("settings-default-apps-kind-folders-in-sentence"),
        3 => t!("settings-default-apps-kind-text-in-sentence"),
        4 => t!("settings-default-apps-kind-pictures-in-sentence"),
        5 => t!("settings-default-apps-kind-music-in-sentence"),
        6 => t!("settings-default-apps-kind-video-in-sentence"),
        7 => t!("settings-default-apps-kind-pdf-in-sentence"),
        8 => t!("settings-default-apps-kind-calendar-in-sentence"),
        _ => "",
    }
}

impl DefaultApps {
    fn read(&mut self) {
        let all = apps::entries();
        self.kinds = KINDS.iter().map(|(_, types)| {
            let mut could: Vec<Entry> = Vec::new();
            for t in *types {
                for e in apps::openers_of(&all, t) {
                    if !could.iter().any(|c| c.id == e.id) {
                        could.push(e);
                    }
                }
            }
            let chosen = apps::chosen_of(&all, types[0]);
            Openers { could, chosen }
        }).collect();
    }

    fn openers(&self, at: usize) -> &Openers {
        self.kinds.get(at).unwrap_or(&NONE)
    }

    fn sum_up(&self) -> String {
        let missing: Vec<String> = self.kinds.iter().enumerate()
            .filter(|(_, o)| !o.could.is_empty() && o.chosen.is_none())
            .map(|(at, _)| kind_in_sentence(at).to_string()).collect();
        let web = self.kinds.first().and_then(|o| o.chosen.as_ref()).map(|e| e.name.clone());
        match (missing.as_slice(), web) {
            ([], Some(web)) => t!("settings-default-apps-all-set-web", web = &web),
            ([], None) => t!("settings-default-apps-all-set").into(),
            ([one], _) => t!("settings-default-apps-one-missing", kind = one),
            (many, _) => t!("settings-default-apps-many-missing", kinds = in_words(many).replace(" and ", " or ")),
        }
    }

    fn kind(&self, kit: &Kit, at: usize) -> Node<Msg> {
        let title = kind_name(at);
        let o = self.openers(at);
        let key = format!("default:{}", KINDS[at].0);
        if o.could.is_empty() {
            return line(title).icon("action-unavailable-symbolic").sub(t!("settings-default-apps-none-installed"))
                .key(key).node(kit);
        }
        let mut options: Vec<String> = o.could.iter().map(|e| e.name.clone()).collect();
        let mut chosen = o.chosen.as_ref().and_then(|c| o.could.iter().position(|e| e.id == c.id));
        let offset = usize::from(chosen.is_none());
        if chosen.is_none() {
            options.insert(0, t!("settings-default-apps-not-chosen").into());
            chosen = Some(0);
        }
        let ids: Vec<String> = o.could.iter().map(|e| e.id.clone()).collect();
        let pick = dropdown(kit, &key, options, chosen, title, move |i| {
            let id = i.checked_sub(offset).and_then(|n| ids.get(n)).cloned();
            on::<DefaultApps>(move |me, k| if let Some(id) = &id { me.choose(k, at, id) })
        });
        let n = o.could.len();
        let sub = t!("settings-default-apps-can-open", n = n);
        let pic = match &o.chosen {
            Some(e) => app_icon(e.icons(), 32.0),
            None => icon(crate::common::apps::GENERIC_ICONS[0], 32.0),
        };
        row(vec![pic, line(title).sub(sub).key(key).control(pick).node(kit).width(Length::Fill)])
            .gap(12.0).width(Length::Fill)
    }

    fn card(&self, kit: &Kit, (name, icon, kinds, about): &(&str, &str, &[usize], &str)) -> Node<Msg> {
        group(*name).icon(*icon).about(*about)
            .rows(kinds.iter().map(|&k| self.kind(kit, k)).collect())
            .node(kit)
    }

    fn choose(&mut self, kit: &mut Kit, at: usize, id: &str) {
        let key = format!("default:{}", KINDS[at].0);
        match apps::choose_opener(id, KINDS[at].1) {
            Ok(()) => {
                if let Some(o) = self.kinds.get_mut(at) {
                    o.chosen = o.could.iter().find(|e| e.id == id).cloned();
                }
                kit.say(&key, t!("settings-default-apps-saved"), true, false);
            }
            Err(e) => kit.say(&key, &e, false, false),
        }
    }
}

impl Page for DefaultApps {
    fn id(&self) -> &'static str {
        "default-apps"
    }
    fn title(&self) -> &'static str {
        t!("settings-default-apps-title")
    }
    fn icon(&self) -> &'static str {
        "starred-symbolic"
    }
    fn keywords(&self) -> &'static str {
        t!("settings-default-apps-keywords")
    }
    fn help(&self) -> &'static str {
        t!("settings-default-apps-help")
    }
    fn shown(&mut self, _kit: &mut Kit) {
        self.read();
    }
    fn view(&self, kit: &Kit) -> Node<Msg> {
        let mut items = vec![headline(kit, self.sum_up())];
        items.extend(cards().iter().map(|c| self.card(kit, c)));
        page(kit, self.title(), self.help(), items)
    }
}

#[cfg(test)]
impl DefaultApps {
    pub(crate) fn made_up() -> DefaultApps {
        let app = |name: &str| Entry {
            id: format!("org.example.{name}.desktop"), name: name.into(), icon: None, exec: String::new(), try_exec: None,
            wm_class: String::new(), program: String::new(), kinds: Vec::new(), path: Default::default(), category: "",
            keywords: Vec::new(), flatpak: None, appimage: None, terminal: false,
        };
        let names = ["Browser", "Mail", "Folders", "Text", "Viewer", "Player", "Player", "Reader", "Calendar"];
        let kinds = names.iter().enumerate().map(|(i, n)| Openers {
            could: vec![app(n), app("Another")],
            chosen: (i != 6).then(|| app(n)),
        }).collect();
        DefaultApps { kinds }
    }
}
