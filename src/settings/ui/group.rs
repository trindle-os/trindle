//! A card of related settings, built up from its name, icon, words, picture and rows.

use super::{Kit, Msg, PAGE_ICON, info};
use crate::toolkit::t;
use crate::toolkit::widget::{card as boxed, column, note, Length, Node};

pub struct Group {
    name: String,
    icon: Option<String>,
    about: Option<String>,
    vis: Option<Node<Msg>>,
    rows: Vec<Node<Msg>>,
    span: Option<(u8, u8)>,
    waits: Option<String>,
    wide: bool,
    known_as: Option<String>,
}

pub fn group(name: impl Into<String>) -> Group {
    Group { name: name.into(), icon: None, about: None, vis: None, rows: Vec::new(), span: None, waits: None, wide: false, known_as: None }
}

impl Group {
    pub fn icon(mut self, name: impl Into<String>) -> Self {
        self.icon = Some(name.into());
        self
    }

    pub fn about(mut self, words: impl Into<String>) -> Self {
        self.about = Some(words.into());
        self
    }

    pub fn vis(mut self, n: Node<Msg>) -> Self {
        self.vis = Some(n);
        self
    }

    pub fn rows(mut self, rows: Vec<Node<Msg>>) -> Self {
        self.rows = rows;
        self
    }

    pub fn span(mut self, least: u8, most: u8) -> Self {
        self.span = Some((least, most));
        self
    }

    pub fn wide_by_choice(mut self) -> Self {
        self.wide = true;
        self
    }

    pub fn waits_for(mut self, why: Option<&str>) -> Self {
        self.waits = why.map(str::to_string);
        self
    }

    pub fn known_as(mut self, id: impl Into<String>) -> Self {
        self.known_as = Some(id.into());
        self
    }

    pub fn node(self, kit: &Kit) -> Node<Msg> {
        self.build(Some(kit))
    }

    pub(super) fn build(self, kit: Option<&Kit>) -> Node<Msg> {
        let icon = self.icon.unwrap_or_else(|| PAGE_ICON.with(|i| i.get()).to_string());
        let mut end = Vec::new();
        if let (Some(k), Some(about)) = (kit, &self.about) {
            end.push(info(k, &format!("card:{}", self.name), &t!("settings-common-about-card", name = self.name.to_lowercase()), about));
        }
        let key = self.known_as.map(|id| card_key(&id)).or_else(|| (!self.name.is_empty()).then(|| card_key(&self.name)));
        let mut all = vec![crate::toolkit::tiles::tile_head(icon, self.name, end)];
        let grey = |n: Node<Msg>, waits: bool| if waits { n.off(true) } else { n };
        let waits = self.waits.is_some();
        all.extend(self.waits.map(note));
        all.extend(self.vis.map(|v| grey(v, waits)));
        all.extend(self.rows.into_iter().map(|r| grey(r, waits)));
        let b = crate::toolkit::theme::bento();
        let (least, most) = self.span.unwrap_or(b.card);
        let card = boxed(column(all).gap(ROW_GAP)).height(Length::Fill).beside().span(least, most, b.words);
        let card = if self.wide { card.wide_by_choice() } else { card };
        match key {
            Some(k) => card.key(k),
            None => card,
        }
    }
}

pub(crate) fn card_key(name: &str) -> String {
    format!("card:{name}")
}

pub const ROW_GAP: f32 = 16.0;
