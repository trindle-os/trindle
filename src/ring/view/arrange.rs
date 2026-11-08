//! Which apps stand where: the middle ring, the pieces of All apps and the matches for what is typed, and the change between the two levels.

use std::rc::Rc;

use crate::common::apps::{self, CATEGORIES};
use crate::ring::open_windows;
use crate::ring::ctx::Program;
use crate::ring::layout::layout;
use crate::toolkit::motion::{Ease, tween};

use super::{ALL, Face, FADE_MS, FAVOURITES};

const TYPED_MOST: usize = 200;

fn in_centre(id: &str) -> bool {
    apps::own(id).is_some_and(|o| matches!(o.key, "settings" | "store" | "activity" | "drives"))
}

pub(super) fn less_word(typed: &str) -> String {
    let kept = typed.trim_end();
    let cut = kept.rfind(char::is_whitespace).map_or(0, |i| i + kept[i..].chars().next().map_or(1, char::len_utf8));
    typed[..cut].to_string()
}

pub(super) fn pieces_of(all: &[Program], favourites: &[String], first: &[usize], every: bool) -> Vec<(String, Vec<usize>)> {
    if !every {
        return Vec::new();
    }
    let mut groups = Vec::new();
    for c in CATEGORIES {
        let mut these: Vec<usize> = (0..all.len())
            .filter(|&i| all[i].category == c && !first.contains(&i) && !favourites.contains(&all[i].id) && !has_own_button(&all[i].id))
            .collect();
        these.sort_by_cached_key(|&i| all[i].name.to_lowercase());
        if !these.is_empty() {
            groups.push((c.to_string(), these));
        }
    }
    groups
}

fn has_own_button(id: &str) -> bool {
    apps::own(id).is_some_and(|o| o.key == "drives")
}

pub(super) fn found(all: &[Program], typed: &str) -> Vec<usize> {
    let names: Vec<&str> = all.iter().map(|p| p.name.as_str()).collect();
    rank_found(typed, &names)
}

pub(super) fn quick_side(disc_radius: f64) -> f64 {
    disc_radius * 2.0 - 16.0
}

pub(super) fn rank_found(typed: &str, apps: &[impl AsRef<str>]) -> Vec<usize> {
    let want = typed.trim().to_lowercase();
    if want.is_empty() {
        return Vec::new();
    }
    let mut hits: Vec<(usize, String, usize)> = apps
        .iter()
        .enumerate()
        .map(|(i, name)| (name.as_ref().to_lowercase(), i))
        .filter(|(name, _)| name.starts_with(&want))
        .map(|(name, i)| (name.split_whitespace().count(), name, i))
        .collect();
    hits.sort();
    hits.into_iter().map(|(_, _, i)| i).collect()
}

impl Face {

    fn rings(&self) -> (Vec<usize>, Vec<(String, Vec<usize>)>) {
        let all = self.all();
        let first: Vec<usize> = self
            .favourites
            .iter()
            .filter(|id| !in_centre(id))
            .filter_map(|id| all.iter().position(|a| &a.id == id)
                .or_else(|| all.iter().position(|a| apps::same_name(&a.id, id))))
            .collect();
        let groups = pieces_of(&all, &self.favourites, &first, self.level == ALL && !self.searching());
        (first, groups)
    }

    pub(super) fn searching(&self) -> bool {
        self.level == ALL && !self.typed.trim().is_empty()
    }

    fn windows_reserve(&self) -> f64 {
        let k = open_windows::Sizes::of(crate::toolkit::theme::sheet());
        k.gap + k.width
    }

    pub(super) fn relayout(&mut self) {
        if self.searching() {
            let found = found(&self.all(), &self.typed);
            self.best_match = found.first().copied();
            if found.is_empty() {

                let (first, groups) = self.rings();
                self.first_len = first.len();
                self.lay = Rc::new(layout(&first, &groups, self.size.0, self.size.1));
            } else {
                self.first_len = 0;
                self.lay = Rc::new(crate::ring::layout::search_ring(&found, self.size.0, self.size.1, self.windows_reserve()));
            }
        } else {
            let (first, groups) = self.rings();
            self.first_len = first.len();
            self.best_match = None;
            self.lay = Rc::new(layout(&first, &groups, self.size.0, self.size.1));
        }
        let q = quick_side(self.lay.disc_radius) as i32;
        self.quick.set_size(q);
    }

    pub(super) fn toggle_level(&mut self) {
        self.set_level(if self.level == ALL { FAVOURITES } else { ALL });
    }

    pub(super) fn set_level(&mut self, level: u8) {
        if level != self.level {
            if let Some(t) = self.level_tween.take() {
                t.stop();
            }
            self.level_turn += 1;
            self.level_k = 0.0;
            let ctx = self.ctx.clone();
            let end = self.ctx.clone();
            self.level_tween = Some(tween(&self.ctx.lp, FADE_MS, Ease::Out, move |k| ctx.face(move |f| f.level_k = k),
                                          move || end.face(|f| {
                                              f.level_k = 1.0;
                                              f.level_tween = None;
                                          })));
        }
        self.level = level;
        self.typed.clear();
        self.relayout();
    }

    pub(super) fn set_typed(&mut self, text: String) {
        if text != self.typed {
            self.typed = text;
            self.relayout();
        }
    }

    pub(super) fn type_more(&mut self, more: &str) {
        let t: String = self.typed.chars().chain(more.chars()).take(TYPED_MOST).collect();
        self.set_typed(t);
    }
}
