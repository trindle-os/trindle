//! One design for every list of apps, so that Install is in the same place in each.

use crate::toolkit::t;
use super::catalogue::App;
use super::look::{self, Dot};
use super::pages::{self, image_tile::image_tile, mixed::Tile, pictures::app_icon};
use super::catalogue::open_app;
use super::{go, Msg, Store};
use crate::settings::ui::INSTALL_ICON;
use crate::toolkit::theme;
use crate::toolkit::widget::{button, card, column, even_grid, label, note, row, Length, Node, Tone};

pub const BLANK: &str = "\u{a0}";
const LINE_GAP: f32 = 4.0;
pub const TILE: &str = "apps-tile";
pub const TILE_ACTION: &str = "apps-tile-action";

pub fn line<M>(words: impl Into<String>) -> Node<M> {
    let words = words.into();
    note(if words.trim().is_empty() { BLANK.to_string() } else { words }).cut()
}

pub fn two_lines<M>(words: impl Into<String>) -> Node<M> {
    let words = words.into();
    note(if words.trim().is_empty() { BLANK.to_string() } else { words }).cut_after(2).width(Length::Fill)
}

const ROW_LINES: u8 = 2;

pub fn row_line<M>(words: impl Into<String>) -> Node<M> {
    let words = words.into();
    note(if words.trim().is_empty() { BLANK.to_string() } else { words }).cut_after(ROW_LINES).width(Length::Fill)
}

pub fn listing<M>(picture: Option<Node<M>>, name: impl Into<String>, line: Node<M>, under: Vec<Node<M>>, end: Vec<Node<M>>) -> Node<M> {
    made(false, false, picture, name, line, under, end)
}

pub fn row_of<M>(picture: Option<Node<M>>, name: impl Into<String>, line: Node<M>, under: Vec<Node<M>>, end: Vec<Node<M>>) -> Node<M> {
    made(false, true, picture, name, line, under, end)
}

pub fn row_or_greyed<M>(greyed: bool, picture: Option<Node<M>>, name: impl Into<String>, line: Node<M>, under: Vec<Node<M>>, end: Vec<Node<M>>) -> Node<M> {
    made(greyed, true, picture, name, line, under, end)
}

fn made<M>(greyed: bool, roomy: bool, picture: Option<Node<M>>, name: impl Into<String>, line: Node<M>, under: Vec<Node<M>>, end: Vec<Node<M>>) -> Node<M> {
    let mut words = vec![label(name).cut().off(greyed), line.off(greyed)];
    words.extend(under);
    let shows = if greyed { theme::Theme::for_page("apps").dim(theme::apps_list().held, theme::AIM_EDGES) } else { 1.0 };
    let mut parts: Vec<Node<M>> = picture.map(|p| p.opacity(shows)).into_iter().collect();
    let words = column(words).gap(LINE_GAP).width(Length::Fill);
    parts.push(if roomy {
        let room = column(vec![label(BLANK).cut(), note(BLANK).room_for(ROW_LINES)]).gap(LINE_GAP).width(Length::Fixed(0.0));
        row(vec![room, words]).gap(0.0).width(Length::Fill)
    } else {
        words
    });
    parts.extend(end);
    row(parts).gap(theme::sheet().gap).centred()
}

pub fn rows<M>(rows: Vec<Node<M>>) -> Vec<Node<M>> {
    let room = theme::sheet().line_padding;
    crate::toolkit::widget::banded_list(rows.into_iter().map(|r| r.padding(room)).collect())
}

pub fn dotted(s: &Store, dot: Dot, words: impl Into<String>) -> Node<Msg> {
    row(vec![look::dot(s, dot), line(words)]).gap(DOT_GAP).centred()
}

pub fn dotted_row_line(s: &Store, dot: Dot, words: impl Into<String>) -> Node<Msg> {
    row(vec![look::dot(s, dot), row_line(words)]).gap(DOT_GAP)
}

const DOT_GAP: f32 = 8.0;

pub fn standing(s: &Store, id: &str) -> Option<Dot> {
    if !s.installed.contains_key(id) {
        return None;
    }
    Some(match super::update_page::app_share(s, id) {
        Some(share) => Dot::Filling(share),
        None if s.held.app(id) => Dot::Quiet,
        None if s.updatable.iter().any(|u| u == id) => Dot::Waiting,
        None => Dot::UpToDate,
    })
}

fn action(s: &Store, a: &App) -> Node<Msg> {
    let id = a.id.clone();
    if let Some(share) = super::update_page::app_share(s, &a.id) {
        return button(crate::settings::ui::UPDATE_ICON, t!("store-installed-updating", name = &a.name)).working(share, Tone::Accent);
    }
    if let Some(d) = s.dials.get(&a.id) {
        let words = if s.busy.get(&a.id) == Some(&"uninstall") { t!("store-list-removing", name = &a.name) } else { t!("store-list-installing", name = &a.name) };
        return d.round(button(INSTALL_ICON, words));
    }
    if s.installed.contains_key(&a.id) {
        return look::open_button(&t!("store-list-open", name = &a.name)).on_press(go(move |_| open_app(&id)));
    }
    let app = a.clone();
    button(INSTALL_ICON, t!("store-list-install", name = &a.name)).on_press(go(move |s| pages::install::pressed_install(s, |s| pages::install::install(s, &app))))
}

pub fn app_tile(s: &Store, a: &App) -> Node<Msg> {
    let k = theme::apps_list();
    let end = vec![action(s, a).key(format!("{TILE_ACTION}:{}", a.id))];
    let open = a.clone();
    let under = match standing(s, &a.id) {
        Some(dot) => dotted(s, dot, if a.summary.trim().is_empty() { t!("store-list-installed").to_string() } else { a.summary.clone() }),
        None => line(a.summary.clone()),
    };
    card(listing(Some(app_icon(a, k.tile_icon)), a.name.clone(), under, Vec::new(), end))
        .key(format!("{TILE}:{}", a.id)).on_press(go(move |s| s.open_detail(open.clone())))
}

pub(crate) fn app_grid(s: &Store, tiles: &[Tile]) -> Node<Msg> {
    let one = |t: &Tile| match t {
        Tile::Flat(a) => app_tile(s, a),
        Tile::Image(f) => image_tile(s, f),
    };
    even_grid(theme::apps_list().tile_least, tiles.iter().map(one).collect()).gap(theme::bento().page_gap)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toolkit::theme::Theme;
    use crate::toolkit::ui::Ui;

    #[test]
    fn a_row_s_line_takes_two_lines_and_every_row_is_one_height() {
        const SHORT: &str = "From Flathub";
        const LONG: &str = "Held back at version 3.2.1; version 3.3 is waiting";
        const LONGER: &str = "Held back at version 2026.10.1-beta.3; version 2026.11.2-beta.1 is waiting, and a great deal more is said after that than any row has room for";
        let theme = || Theme::new(true, "#fbc711");
        let alone = |words: &str, across: f32| Ui::new(row_line::<Msg>(words), theme()).natural_height(across);
        let one = alone(SHORT, 220.0);
        assert_eq!(alone(LONG, 1200.0), one, "one line where there is room");
        assert_eq!(alone(LONG, 220.0), one * 2.0, "two lines where there is not");
        assert_eq!(alone(LONGER, 220.0), one * 2.0, "and never a third");
        assert_eq!(alone("", 220.0), one, "an empty line keeps its room");
        let a_row = |words: &str, across: f32| {
            let end = vec![button::<Msg>(INSTALL_ICON, "Install")];
            Ui::new(row_of(None, "Notes", row_line(words), Vec::new(), end), theme()).natural_height(across)
        };
        for across in [260.0, 340.0, 600.0, 1200.0] {
            let tall = a_row(SHORT, across);
            for words in ["", LONG, LONGER] {
                assert_eq!(a_row(words, across), tall, "{across}: {words:?}");
            }
            assert_eq!(tall, a_row(SHORT, 1200.0), "{across}: a row is one height at every width");
        }
        let name = Ui::new(label::<Msg>("Notes").cut(), theme()).natural_height(600.0);
        assert!(a_row(SHORT, 600.0) >= name + LINE_GAP + one * 2.0, "the two lines have their room");
        let tile = Ui::new(listing::<Msg>(None, "Notes", line(LONGER), Vec::new(), Vec::new()), theme()).natural_height(260.0);
        assert_eq!(tile, name + LINE_GAP + one);
    }
}
