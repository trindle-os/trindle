//! Things added under a name another has.

use super::{Kit, Msg, line};
use crate::toolkit::t;
use crate::toolkit::widget::{button, Node};

#[derive(Default)]
pub struct Clashes {
    waiting: Vec<(u32, crate::common::copies::Taken)>,
    batch: u32,
    all: bool,
}

impl Clashes {
    pub fn add_batch(&mut self, taken: impl IntoIterator<Item = crate::common::copies::Taken>) {
        self.batch += 1;
        let batch = self.batch;
        self.waiting.extend(taken.into_iter().map(|t| (batch, t)));
    }

    pub fn first(&self) -> Option<&crate::common::copies::Taken> {
        self.waiting.first().map(|(_, t)| t)
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.waiting.is_empty()
    }

    pub fn more(&self) -> usize {
        let Some((batch, _)) = self.waiting.first() else { return 0 };
        self.waiting[1..].iter().filter(|(b, _)| b == batch).count()
    }

    pub fn same(&self) -> bool {
        self.all
    }

    pub fn set_same(&mut self, on: bool) {
        self.all = on && self.more() > 0;
    }

    pub fn answer(&mut self) -> Vec<crate::common::copies::Taken> {
        let Some(&(batch, _)) = self.waiting.first() else { return Vec::new() };
        let all = std::mem::take(&mut self.all);
        let mut covered = vec![self.waiting.remove(0).1];
        if all {
            let (same, later): (Vec<_>, Vec<_>) = self.waiting.drain(..).partition(|(b, _)| *b == batch);
            self.waiting = later;
            covered.extend(same.into_iter().map(|(_, t)| t));
        }
        covered
    }
}

pub fn replace_rows(kit: &Kit, key: &str, clashes: &Clashes, bring: Vec<Node<Msg>>, there: Vec<Node<Msg>>,
                    skip: Msg, replace: Msg, same: impl Fn(bool) -> Msg + 'static) -> Vec<Node<Msg>> {
    use crate::common::volumes::exact_size_words as size_words;
    let Some(taken) = clashes.first() else { return Vec::new() };
    let more = clashes.more();
    let mut head = line(t!("settings-common-already-there", name = &taken.name))
        .keep_sub(if more == 0 { String::new() } else { t!("settings-common-same-for-rest-more", n = more) })
        .key(key);
    if more > 0 {
        head = head.control(crate::toolkit::widget::switch(clashes.same(), t!("settings-common-same-for-rest"), same));
    }
    let skip = button("media-skip-forward-symbolic", t!("settings-common-skip")).on_press(skip);
    let yes = crate::toolkit::widget::word_button(t!("settings-common-replace")).spoken(t!("settings-common-replace-spoken")).on_press(replace);
    vec![
        head.node(kit),
        line(t!("settings-common-being-brought")).sub(size_words(taken.new_size)).controls(bring).node(kit),
        line(t!("settings-common-already-line")).sub(size_words(taken.old_size)).controls(there).node(kit),
        line(t!("settings-common-replace-question")).controls(vec![skip, yes]).node(kit),
    ]
}
