//! Which notification circles are out, their sizes and what waits to come back.

use super::circles::{cleared, places, size_at, Corner, Face, Held, Sizes, Snooze};

struct Item {
    id: u32,
    pinned: bool,
    face: Face,
    hovering: bool,
    scroll: usize,
    rest: f32,
    grown: f32,
    from: f32,
    to: f32,
    since: u64,
    stay: u64,
    expires: Option<u64>,
    put: Option<(f32, f32)>,
    held: Option<Option<(f32, f32)>>,
}

impl Item {
    fn expiry(&self, now: u64) -> Option<u64> {
        if self.pinned || self.hovering || self.face == Face::Snooze || self.stay == 0 { None } else { Some(now + self.stay) }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Due {
    pub closed: Vec<u32>,
    pub back: Vec<u32>,
}

fn keep_inside(p: (f32, f32), d: f32, (w, h): (f32, f32)) -> (f32, f32) {
    (p.0.clamp(0.0, (w - d).max(0.0)).round(), p.1.clamp(0.0, (h - d).max(0.0)).round())
}

pub struct Stack {
    k: Sizes,
    slide: u64,
    items: Vec<Item>,
    asleep: Vec<(u32, u64, f32)>,
}

impl Stack {
    pub fn new(k: Sizes, slide: u64) -> Stack {
        Stack { k, slide, items: Vec::new(), asleep: Vec::new() }
    }

    pub fn restyle(&mut self, k: Sizes, slide: u64, now: u64) {
        self.k = k;
        self.slide = slide;
        for i in 0..self.items.len() {
            self.items[i].rest = self.floor(self.items[i].rest);
            self.items[i].grown = self.items[i].grown.max(self.items[i].rest);
            self.retarget(i, now);
        }
    }

    pub fn has(&self, id: u32) -> bool {
        self.items.iter().any(|i| i.id == id)
    }

    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty() && self.asleep.is_empty()
    }

    pub fn many(&self) -> bool {
        self.items.len() >= 2
    }

    fn at(&self, id: u32) -> Option<usize> {
        self.items.iter().position(|i| i.id == id)
    }

    #[cfg(test)]
    pub fn add(&mut self, id: u32, seconds: i32, now: u64) {
        self.add_sized(id, seconds, self.k.across, now);
    }

    pub fn add_sized(&mut self, id: u32, seconds: i32, rest: f32, now: u64) {
        self.asleep.retain(|(s, _, _)| *s != id);
        let stay = seconds.max(0) as u64 * 1000;
        if let Some(i) = self.at(id) {
            let item = &mut self.items[i];
            item.stay = stay;
            item.scroll = 0;
            item.expires = item.expiry(now);
            self.set_rest(id, rest, now);
            return;
        }
        let rest = self.floor(rest);
        self.items.push(Item {
            id, pinned: false, face: Face::Note, hovering: false, scroll: 0, rest, grown: rest, from: rest,
            to: rest, since: now, stay, expires: (stay > 0).then_some(now + stay), put: None, held: None,
        });
    }

    fn floor(&self, rest: f32) -> f32 {
        rest.max(self.k.least.min(self.k.across))
    }

    pub fn set_rest(&mut self, id: u32, rest: f32, now: u64) {
        let Some(i) = self.at(id) else { return };
        self.items[i].rest = self.floor(rest);
        self.items[i].grown = self.items[i].grown.max(self.items[i].rest);
        self.retarget(i, now);
    }

    pub fn rest(&self, id: u32) -> f32 {
        self.at(id).map_or(self.k.across, |i| self.items[i].rest)
    }

    pub fn close(&mut self, id: u32) {
        self.items.retain(|i| i.id != id);
        self.asleep.retain(|(s, _, _)| *s != id);
    }

    pub fn pinned(&self, id: u32) -> bool {
        self.at(id).is_some_and(|i| self.items[i].pinned)
    }

    pub fn face(&self, id: u32) -> Face {
        self.at(id).map_or(Face::Note, |i| self.items[i].face)
    }

    pub fn hovering(&self, id: u32) -> bool {
        self.at(id).is_some_and(|i| self.items[i].hovering)
    }

    pub fn scroll(&self, id: u32) -> usize {
        self.at(id).map_or(0, |i| self.items[i].scroll)
    }

    pub fn toggle_pin(&mut self, id: u32, now: u64) -> bool {
        let Some(i) = self.at(id) else { return false };
        let item = &mut self.items[i];
        item.pinned = !item.pinned;
        if !item.pinned {
            item.put = None;
            item.held = None;
        }
        item.expires = item.expiry(now);
        item.pinned
    }

    pub fn set_pin(&mut self, id: u32, pinned: bool, now: u64) -> bool {
        match self.at(id) {
            Some(i) if self.items[i].pinned != pinned => {
                self.toggle_pin(id, now);
                true
            }
            _ => false,
        }
    }

    pub fn point(&mut self, id: u32, on: bool, grown: f32, now: u64) {
        let Some(i) = self.at(id) else { return };
        let item = &mut self.items[i];
        item.grown = grown.max(item.rest);
        if item.hovering != on {
            item.hovering = on;
            item.expires = item.expiry(now);
            if !on {
                item.scroll = 0;
            }
        }
        self.retarget(i, now);
    }

    pub fn open_snooze(&mut self, id: u32, now: u64) {
        let Some(i) = self.at(id) else { return };
        self.items[i].face = Face::Snooze;
        self.items[i].expires = None;
        self.retarget(i, now);
    }

    pub fn back(&mut self, id: u32, now: u64) {
        let Some(i) = self.at(id) else { return };
        let item = &mut self.items[i];
        item.face = Face::Note;
        item.expires = item.expiry(now);
        self.retarget(i, now);
    }

    pub fn snooze(&mut self, id: u32, after: Snooze, clock: (i32, i32, i32), now: u64) {
        let Some(i) = self.at(id) else { return };
        let rest = self.items[i].rest;
        self.close(id);
        let wake = now + after.seconds(clock.0, clock.1, clock.2).max(1) as u64 * 1000;
        self.asleep.push((id, wake, rest));
    }

    pub fn scroll_by(&mut self, id: u32, lines: i32, most: usize) {
        let Some(i) = self.at(id) else { return };
        let to = (self.items[i].scroll as i64 + lines as i64).clamp(0, most as i64);
        self.items[i].scroll = to as usize;
    }

    pub fn clear_all(&mut self) -> Vec<u32> {
        let held: Vec<Held> = self.items.iter().map(|i| Held { id: i.id, pinned: i.pinned, moved: i.put.is_some() }).collect();
        let gone = cleared(&held);
        self.items.retain(|i| !gone.contains(&i.id));
        gone
    }

    fn retarget(&mut self, i: usize, now: u64) {
        let across = self.across_of(i, now);
        let item = &self.items[i];
        let to = if item.hovering && item.face == Face::Note { item.grown } else { item.rest };
        if (to - item.to).abs() < 0.01 {
            return;
        }
        let item = &mut self.items[i];
        item.from = across;
        item.to = to;
        item.since = now;
    }

    fn across_of(&self, i: usize, now: u64) -> f32 {
        let item = &self.items[i];
        if self.slide == 0 {
            return item.to;
        }
        size_at(item.from, item.to, now.saturating_sub(item.since) as f32 / self.slide as f32)
    }

    pub fn across(&self, id: u32, now: u64) -> f32 {
        self.at(id).map_or(self.k.across, |i| self.across_of(i, now))
    }

    pub fn moving(&self, now: u64) -> bool {
        self.items.iter().any(|i| i.from != i.to && now < i.since + self.slide)
    }

    pub fn places(&self, corner: Corner, screen: (f32, f32), now: u64) -> Vec<(u32, f32, f32, f32)> {
        let sizes: Vec<f32> = (0..self.items.len()).map(|i| self.across_of(i, now).round()).collect();
        let column: Vec<f32> = self.items.iter().zip(&sizes).filter(|(i, _)| i.put.is_none()).map(|(_, d)| *d).collect();
        let mut column = places(corner, screen, &column, self.k).into_iter();
        let mut out = Vec::new();
        for (item, d) in self.items.iter().zip(&sizes) {
            match item.put {
                Some(p) => {
                    let (x, y) = keep_inside(p, *d, screen);
                    out.push((item.id, x, y, *d));
                }
                None => {
                    if let Some((x, y, d)) = column.next() {
                        out.push((item.id, x, y, d));
                    }
                }
            }
        }
        out
    }

    pub fn pinned_places(&self) -> Vec<(u32, Option<(f32, f32)>)> {
        self.items.iter().filter(|i| i.pinned).map(|i| (i.id, i.put)).collect()
    }

    pub fn sleepers(&self) -> Vec<(u32, u64)> {
        self.asleep.iter().map(|(id, wake, _)| (*id, *wake)).collect()
    }

    pub fn restore(&mut self, id: u32, rest: f32, put: Option<(f32, f32)>, wake: Option<u64>, now: u64) {
        match wake {
            Some(w) => {
                self.asleep.retain(|(s, _, _)| *s != id);
                self.asleep.push((id, w, self.floor(rest)));
            }
            None => {
                self.add_sized(id, 0, rest, now);
                self.set_pin(id, true, now);
                if let Some(i) = self.at(id) {
                    self.items[i].put = put;
                }
            }
        }
    }

    #[cfg(test)]
    pub fn moved(&self, id: u32) -> bool {
        self.at(id).is_some_and(|i| self.items[i].put.is_some())
    }

    pub fn holding(&self, id: u32) -> bool {
        self.at(id).is_some_and(|i| self.items[i].held.is_some())
    }

    pub fn pick_up(&mut self, id: u32, local: (f32, f32), corner: Corner, screen: (f32, f32), now: u64) -> bool {
        let Some(i) = self.at(id).filter(|i| self.items[*i].pinned && self.items[*i].held.is_none()) else { return false };
        let Some((_, x, y, d)) = self.places(corner, screen, now).into_iter().find(|p| p.0 == id) else { return false };
        let before = self.items[i].put;
        self.items[i].held = Some(before);
        self.items[i].put = Some(keep_inside((x + local.0 - d / 2.0, y + local.1 - d / 2.0), d, screen));
        true
    }

    pub fn drag(&mut self, id: u32, window: (f32, f32), local: (f32, f32), screen: (f32, f32), now: u64) {
        let Some(i) = self.at(id).filter(|i| self.items[*i].held.is_some()) else { return };
        let d = self.across_of(i, now).round();
        self.items[i].put = Some(keep_inside((window.0 + local.0 - d / 2.0, window.1 + local.1 - d / 2.0), d, screen));
    }

    pub fn drop_held(&mut self, id: u32) {
        if let Some(i) = self.at(id) {
            self.items[i].held = None;
        }
    }

    pub fn put_back(&mut self, id: u32) {
        if let Some(i) = self.at(id) {
            if let Some(before) = self.items[i].held.take() {
                self.items[i].put = before;
            }
        }
    }

    pub fn nudge(&mut self, id: u32, by: (f32, f32), corner: Corner, screen: (f32, f32), now: u64) -> bool {
        let Some(i) = self.at(id).filter(|i| self.items[*i].pinned) else { return false };
        let Some((_, x, y, d)) = self.places(corner, screen, now).into_iter().find(|p| p.0 == id) else { return false };
        self.items[i].put = Some(keep_inside((x + by.0, y + by.1), d, screen));
        true
    }

    pub fn next_wake(&self) -> Option<u64> {
        self.items.iter().filter_map(|i| i.expires).chain(self.asleep.iter().map(|(_, t, _)| *t)).min()
    }

    pub fn tick(&mut self, now: u64) -> Due {
        let mut due = Due::default();
        let out: Vec<u32> = self.items.iter().filter(|i| i.expires.is_some_and(|t| t <= now)).map(|i| i.id).collect();
        for id in out {
            self.close(id);
            due.closed.push(id);
        }
        let mut still = Vec::new();
        for (id, wake, rest) in std::mem::take(&mut self.asleep) {
            if wake <= now {
                self.items.push(Item {
                    id, pinned: false, face: Face::Note, hovering: false, scroll: 0, rest, grown: rest, from: rest,
                    to: rest, since: now, stay: 0, expires: None, put: None, held: None,
                });
                due.back.push(id);
            } else {
                still.push((id, wake, rest));
            }
        }
        self.asleep = still;
        due
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::circles;
    use crate::toolkit::text::Fonts;

    fn kit() -> Sizes {
        Sizes { across: 160.0, least: 96.0, gap: 12.0, margin: 24.0, button: 24.0 }
    }

    const SLIDE: u64 = 200;
    const SCREEN: (f32, f32) = (1280.0, 1000.0);

    fn ids(s: &Stack) -> Vec<u32> {
        s.items.iter().map(|i| i.id).collect()
    }

    fn stack() -> Stack {
        Stack::new(kit(), SLIDE)
    }

    #[test]
    fn a_circle_goes_when_its_time_is_up_and_not_before() {
        let mut s = stack();
        s.add(1, 5, 1_000);
        assert_eq!(s.next_wake(), Some(6_000));
        assert_eq!(s.tick(5_999), Due::default());
        assert!(s.has(1));
        assert_eq!(s.tick(6_000).closed, vec![1]);
        assert!(!s.has(1));
        s.add(2, 0, 0);
        assert_eq!(s.next_wake(), None);
        assert_eq!(s.tick(10_000_000), Due::default());
    }

    #[test]
    fn pointing_holds_a_circle_and_leaving_starts_its_whole_time_again() {
        let mut s = stack();
        s.add(1, 5, 0);
        s.point(1, true, 160.0, 4_000);
        assert_eq!(s.tick(60_000), Due::default());
        s.point(1, false, 160.0, 60_000);
        assert_eq!(s.next_wake(), Some(65_000));
        assert_eq!(s.tick(64_999).closed, Vec::<u32>::new());
        assert_eq!(s.tick(65_000).closed, vec![1]);
    }

    #[test]
    fn a_pin_made_in_the_list_is_set_and_not_toggled() {
        let mut s = stack();
        s.add(1, 5, 0);
        assert!(s.set_pin(1, true, 0) && s.pinned(1));
        assert!(!s.set_pin(1, true, 0) && s.pinned(1), "already pinned: nothing changes");
        assert_eq!(s.tick(1_000_000), Due::default(), "it waits to be closed");
        assert!(s.set_pin(1, false, 2_000_000) && !s.pinned(1));
        assert_eq!(s.next_wake(), Some(2_005_000), "let go, it follows how long it stays again");
        assert!(!s.set_pin(9, true, 0), "no such circle");
    }

    #[test]
    fn a_pinned_circle_waits_to_be_closed_and_ignores_clear_all() {
        let mut s = stack();
        for id in 1..=3 {
            s.add(id, 5, 0);
        }
        assert!(s.toggle_pin(2, 0));
        assert!(s.pinned(2));
        assert_eq!(s.clear_all(), vec![1, 3]);
        assert_eq!(ids(&s), vec![2]);
        assert_eq!(s.tick(1_000_000), Due::default());
        assert!(s.has(2));
        assert!(!s.toggle_pin(2, 2_000_000));
        assert_eq!(s.tick(2_004_999).closed, Vec::<u32>::new());
        assert_eq!(s.tick(2_005_000).closed, vec![2]);
        s.add(7, 0, 0);
        s.toggle_pin(7, 0);
        s.close(7);
        assert!(!s.has(7));
    }

    #[test]
    fn clear_all_is_for_two_or_more() {
        let mut s = stack();
        s.add(1, 0, 0);
        assert!(!s.many());
        s.add(2, 0, 0);
        assert!(s.many());
        s.close(2);
        assert!(!s.many());
    }

    #[test]
    fn snoozing_takes_it_away_and_brings_it_back_after_the_time_chosen() {
        for (choice, ms) in [(Snooze::Minutes10, 600_000u64), (Snooze::Hour1, 3_600_000), (Snooze::Hours6, 6 * 3_600_000)] {
            let mut s = stack();
            s.add(1, 5, 0);
            s.add(2, 5, 0);
            s.open_snooze(1, 100);
            assert_eq!(s.face(1), Face::Snooze);
            assert_eq!(s.tick(1_000_000_000).closed, vec![2]);
            s.snooze(1, choice, (12, 0, 0), 1_000);
            assert!(!s.has(1), "out of sight");
            assert!(!s.is_empty(), "but not forgotten");
            assert_eq!(s.next_wake(), Some(1_000 + ms));
            assert_eq!(s.tick(1_000 + ms - 1), Due::default());
            let due = s.tick(1_000 + ms);
            assert_eq!(due.back, vec![1]);
            assert!(s.has(1));
            assert_eq!(s.next_wake(), None);
            assert_eq!(s.tick(u64::MAX / 2), Due::default());
        }
    }

    #[test]
    fn a_replacement_that_stays_until_closed_loses_the_old_timer() {
        let mut s = Stack::new(kit(), SLIDE);
        s.add(1, 5, 0);
        assert_eq!(s.next_wake(), Some(5_000));
        s.add(1, 0, 1_000);
        assert_eq!(s.next_wake(), None, "the old timer closed a circle meant to stay");
        assert!(s.tick(60_000).closed.is_empty());
    }

    #[test]
    fn a_morning_snooze_comes_back_at_eight() {
        let mut s = stack();
        s.add(1, 0, 0);
        s.snooze(1, Snooze::Morning, (22, 0, 0), 0);
        assert_eq!(s.next_wake(), Some(10 * 3_600_000));
        let mut s = stack();
        s.add(1, 0, 0);
        s.snooze(1, Snooze::Morning, (6, 30, 0), 0);
        assert_eq!(s.next_wake(), Some(90 * 60_000));
    }

    #[test]
    fn closing_takes_a_snoozed_circle_for_good_and_clear_all_leaves_it_alone() {
        let mut s = stack();
        s.add(1, 0, 0);
        s.add(2, 0, 0);
        s.snooze(1, Snooze::Hour1, (12, 0, 0), 0);
        assert_eq!(s.clear_all(), vec![2]);
        assert_eq!(s.tick(3_600_000).back, vec![1]);
        let mut s = stack();
        s.add(3, 0, 0);
        s.snooze(3, Snooze::Hour1, (12, 0, 0), 0);
        s.close(3);
        assert!(s.is_empty());
        assert_eq!(s.tick(u64::MAX / 2), Due::default());
    }

    #[test]
    fn the_back_button_puts_the_notification_back_with_its_time_started_again() {
        let mut s = stack();
        s.add(1, 5, 0);
        s.open_snooze(1, 0);
        assert_eq!(s.next_wake(), None);
        s.back(1, 7_000);
        assert_eq!(s.face(1), Face::Note);
        assert_eq!(s.next_wake(), Some(12_000));
    }

    #[test]
    fn growth_holds_to_the_style_sheets_share_of_the_screen_and_a_short_message_does_not_grow() {
        let (fonts, k, tall) = (Fonts::new(), Sizes::sheet(), 800.0f32);
        let cap = (crate::toolkit::theme::sheet().circle_pointed_most * tall).round();
        let rest = |t: &str, b: &str| circles::rest(&fonts, k, tall, t, b);
        let short = circles::grown(&fonts, k, rest("Copied", "3 files."), circles::REST_BUTTONS, "Copied", "3 files.", tall);
        assert_eq!(short, circles::POINTED_LEAST, "a message that fits grows only to the least a pointed circle is");
        let endless = (1..=80).map(|i| format!("Line {i} of a very long message.")).collect::<Vec<_>>().join(" ");
        let long = circles::grown(&fonts, k, rest("Mail from John Smith", &endless), circles::REST_BUTTONS, "Mail from John Smith", &endless, tall);
        assert_eq!(long, cap, "a long one stops at the cap");
        let lunch = "Lunch on Thursday? I have booked the table for one o'clock, and Jane Brown is bringing the folder with the figures.";
        let mid = circles::grown(&fonts, k, rest("Mail from John Smith", lunch), circles::REST_BUTTONS, "Mail from John Smith", lunch, tall);
        assert!(mid > k.across && mid <= cap, "{mid}");
        let mut s = Stack::new(k, SLIDE);
        s.add(1, 0, 0);
        s.point(1, true, long, 0);
        assert_eq!(s.across(1, SLIDE), cap);
        s.point(1, true, 10.0, 0);
        assert_eq!(s.across(1, 10 * SLIDE), k.across);
    }

    #[test]
    fn a_circle_grows_smoothly_and_the_ones_beyond_it_slide_without_touching() {
        for corner in [Corner::BottomRight, Corner::BottomLeft, Corner::TopLeft, Corner::TopRight] {
            let mut s = stack();
            for id in 1..=4 {
                s.add(id, 0, 0);
            }
            let calm = s.places(corner, SCREEN, 0);
            s.point(2, true, 280.0, 1_000);
            let end = s.places(corner, SCREEN, 1_000 + SLIDE);
            let mut last = s.places(corner, SCREEN, 1_000);
            assert_eq!(last, calm, "nothing has moved at the moment it is pointed at");
            let mut grew = 0.0f32;
            for step in 1..=20u64 {
                let now = 1_000 + step * SLIDE / 20;
                let all = s.places(corner, SCREEN, now);
                for (i, (_, _, y, d)) in all.iter().enumerate() {
                    if i > 0 {
                        let (_, _, py, pd) = all[i - 1];
                        let apart = if corner.bottom() { py - (y + d) } else { y - (py + pd) };
                        assert!(apart >= kit().gap - 1.01, "{corner:?} step {step}: {apart} between {i} and the one before");
                    }
                }
                assert!(all[1].3 >= last[1].3);
                grew = all[1].3;
                let moved_from = calm[2].2;
                let now_at = all[2].2;
                assert!(if corner.bottom() { now_at <= moved_from } else { now_at >= moved_from });
                last = all;
            }
            assert_eq!(grew, 280.0);
            assert_eq!(last, end);
            assert_eq!((end[0].1, end[0].2), (calm[0].1, calm[0].2));
            let slid = (end[2].2 - calm[2].2).abs();
            assert!((slid - 120.0).abs() < 1.01, "{corner:?}: slid {slid}");
            s.point(2, false, 280.0, 5_000);
            assert!(!s.moving(5_000 + SLIDE));
            assert_eq!(s.places(corner, SCREEN, 5_000 + SLIDE), calm);
        }
    }

    #[test]
    fn a_size_change_part_way_carries_on_from_where_it_had_got_to() {
        let mut s = stack();
        s.add(1, 0, 0);
        s.point(1, true, 260.0, 0);
        let half = s.across(1, SLIDE / 2);
        assert!(half > 160.0 && half < 260.0);
        s.point(1, false, 260.0, SLIDE / 2);
        assert_eq!(s.across(1, SLIDE / 2), half);
        assert!(s.across(1, SLIDE / 2 + 20) < half);
    }

    #[test]
    fn only_as_many_as_fit_the_screen_are_placed() {
        let mut s = stack();
        for id in 1..=30 {
            s.add(id, 0, 0);
        }
        let all = s.places(Corner::BottomRight, (1280.0, 720.0), 0);
        assert!(all.len() < 30 && !all.is_empty());
        assert_eq!(all[0].0, 1, "the first to come is in the corner");
    }

    #[test]
    fn scrolling_stays_within_the_message() {
        let mut s = stack();
        s.add(1, 0, 0);
        s.scroll_by(1, 3, 5);
        assert_eq!(s.scroll(1), 3);
        s.scroll_by(1, 10, 5);
        assert_eq!(s.scroll(1), 5);
        s.scroll_by(1, -20, 5);
        assert_eq!(s.scroll(1), 0);
        s.scroll_by(1, 2, 5);
        s.point(1, true, 200.0, 0);
        s.point(1, false, 200.0, 10);
        assert_eq!(s.scroll(1), 0, "it starts at the top again");
    }

    #[test]
    fn one_replaced_by_its_program_starts_its_time_again_and_is_not_doubled() {
        let mut s = stack();
        s.add(1, 5, 0);
        s.add(1, 5, 3_000);
        assert_eq!(ids(&s), vec![1]);
        assert_eq!(s.next_wake(), Some(8_000));
        s.toggle_pin(1, 3_000);
        s.add(1, 5, 4_000);
        assert_eq!(s.next_wake(), None, "a pinned one stays pinned");
    }

    const BR: Corner = Corner::BottomRight;

    fn pinned_pair() -> Stack {
        let mut s = stack();
        s.add(1, 5, 0);
        s.add(2, 5, 0);
        s.toggle_pin(1, 0);
        s.toggle_pin(2, 0);
        s
    }

    fn at_of(s: &Stack, id: u32) -> (f32, f32) {
        s.places(BR, SCREEN, 0).into_iter().find(|p| p.0 == id).map(|p| (p.1, p.2)).unwrap()
    }

    #[test]
    fn a_pinned_circle_picked_up_jumps_to_the_pointer_follows_it_and_stays_where_dropped_moved() {
        let mut s = pinned_pair();
        let home = at_of(&s, 1);
        assert!(s.pick_up(1, (10.0, 10.0), BR, SCREEN, 0));
        assert!(s.holding(1) && s.moved(1));
        let now = at_of(&s, 1);
        assert_eq!(now, (home.0 + 10.0 - 80.0, home.1 + 10.0 - 80.0));
        s.drag(1, at_of(&s, 1), (80.0, 80.0), SCREEN, 0);
        assert_eq!(at_of(&s, 1), now);
        s.drag(1, at_of(&s, 1), (110.0, 100.0), SCREEN, 0);
        assert_eq!(at_of(&s, 1), (now.0 + 30.0, now.1 + 20.0));
        let dropped = at_of(&s, 1);
        s.drop_held(1);
        assert!(!s.holding(1) && s.moved(1));
        assert_eq!(at_of(&s, 1), dropped, "it stays where it was dropped");
        assert_eq!(s.tick(10_000_000), Due::default(), "and it is not timed out");
        assert_eq!(at_of(&s, 2), (SCREEN.0 - 24.0 - 160.0, SCREEN.1 - 24.0 - (SCREEN.1 * crate::ring::circles::EDGE_SHARE).round() - 160.0));
        assert!(s.clear_all().is_empty());
        s.close(1);
        assert_eq!(ids(&s), vec![2]);
    }

    #[test]
    fn escape_while_held_puts_a_circle_back_where_it_was_moved() {
        let mut s = pinned_pair();
        let home = at_of(&s, 1);
        s.pick_up(1, (80.0, 80.0), BR, SCREEN, 0);
        s.drag(1, at_of(&s, 1), (150.0, 150.0), SCREEN, 0);
        s.put_back(1);
        assert!(!s.holding(1) && !s.moved(1));
        assert_eq!(at_of(&s, 1), home);
        s.pick_up(1, (80.0, 80.0), BR, SCREEN, 0);
        s.drag(1, at_of(&s, 1), (60.0, 80.0), SCREEN, 0);
        s.drop_held(1);
        let dropped = at_of(&s, 1);
        s.pick_up(1, (80.0, 80.0), BR, SCREEN, 0);
        s.drag(1, at_of(&s, 1), (20.0, 20.0), SCREEN, 0);
        s.put_back(1);
        assert_eq!(at_of(&s, 1), dropped);
    }

    #[test]
    fn a_circle_that_is_not_pinned_never_moves() {
        let mut s = stack();
        s.add(1, 5, 0);
        let home = at_of(&s, 1);
        assert!(!s.pick_up(1, (80.0, 80.0), BR, SCREEN, 0));
        assert!(!s.nudge(1, (24.0, 0.0), BR, SCREEN, 0));
        s.drag(1, at_of(&s, 1), (150.0, 150.0), SCREEN, 0);
        assert!(!s.holding(1) && !s.moved(1));
        assert_eq!(at_of(&s, 1), home);
        s.toggle_pin(1, 0);
        s.pick_up(1, (80.0, 80.0), BR, SCREEN, 0);
        s.toggle_pin(1, 0);
        assert!(!s.holding(1) && !s.moved(1));
        assert_eq!(at_of(&s, 1), home);
    }

    #[test]
    fn a_moved_circle_is_kept_fully_inside_the_screen() {
        let mut s = pinned_pair();
        s.pick_up(1, (80.0, 80.0), BR, SCREEN, 0);
        for local in [(-5000.0, -5000.0), (5000.0, 5000.0), (-5000.0, 5000.0), (5000.0, -5000.0)] {
            s.drag(1, at_of(&s, 1), local, SCREEN, 0);
            let (x, y) = at_of(&s, 1);
            assert!(x >= 0.0 && y >= 0.0 && x + 160.0 <= SCREEN.0 && y + 160.0 <= SCREEN.1, "{x} {y}");
        }
        let small = (300.0, 200.0);
        let (x, y) = s.places(BR, small, 0).into_iter().find(|p| p.0 == 1).map(|p| (p.1, p.2)).unwrap();
        assert!(x >= 0.0 && x + 160.0 <= small.0 && y >= 0.0 && y + 160.0 <= small.1, "{x} {y}");
        assert!(s.nudge(1, (5000.0, 5000.0), BR, SCREEN, 0));
        assert_eq!(at_of(&s, 1), (SCREEN.0 - 160.0, SCREEN.1 - 160.0));
    }

    #[test]
    fn a_pinned_circle_moves_by_the_keyboard_a_step_at_a_time() {
        let mut s = pinned_pair();
        let home = at_of(&s, 1);
        assert!(s.nudge(1, (-24.0, 0.0), BR, SCREEN, 0));
        assert_eq!(at_of(&s, 1), (home.0 - 24.0, home.1));
        assert!(s.nudge(1, (0.0, -24.0), BR, SCREEN, 0));
        assert_eq!(at_of(&s, 1), (home.0 - 24.0, home.1 - 24.0));
        assert!(s.moved(1) && !s.holding(1));
    }

    #[test]
    fn a_lagging_compositor_does_not_make_a_held_circle_run_away_from_the_pointer() {
        for lag in 0..4usize {
            let mut s = pinned_pair();
            s.pick_up(1, (80.0, 80.0), BR, SCREEN, 0);
            let start = at_of(&s, 1);
            let mut pointer = (start.0 + 80.0, start.1 + 80.0);
            let mut asked = vec![start; lag + 1];
            for step in 0..40 {
                pointer = (pointer.0 + 9.0, pointer.1 + 6.0);
                let actual = asked[asked.len() - 1 - lag];
                s.drag(1, actual, (pointer.0 - actual.0, pointer.1 - actual.1), SCREEN, 0);
                asked.push(at_of(&s, 1));
                let want = (pointer.0 - 80.0, pointer.1 - 80.0);
                let got = at_of(&s, 1);
                let want = (want.0.clamp(0.0, SCREEN.0 - 160.0), want.1.clamp(0.0, SCREEN.1 - 160.0));
                assert_eq!(got, want, "lag {lag}, step {step}: the circle is not under the pointer");
            }
        }
    }
    fn circles_of_every_length() -> [(u32, f32); 3] {
        use circles::rest;
        let (fonts, k) = (Fonts::new(), kit());
        let endless = (1..=40).map(|i| format!("Line {i} of a long message.")).collect::<Vec<_>>().join(" ");
        [
            (1, rest(&fonts, k, 800.0, "Copied", "3 files.")),
            (2, rest(&fonts, k, 800.0, "Mail from Jane", "Lunch is at one on Thursday.")),
            (3, rest(&fonts, k, 800.0, "Mail from John Smith", &endless)),
        ]
    }

    #[test]
    fn at_rest_a_short_message_is_a_small_circle_and_a_long_one_the_full_size() {
        let [(_, short), (_, medium), (_, long)] = circles_of_every_length();
        assert!(short >= kit().least && short <= medium, "a short one is small: {short}");
        assert!(short < long && medium < long, "{short} {medium} {long}");
        assert_eq!(long, (crate::toolkit::theme::sheet().circle_most * 800.0).round(), "a long one is the most the sheet allows");
        let (fonts, tight) = (Fonts::new(), Sizes { least: 200.0, ..kit() });
        assert_eq!(circles::rest(&fonts, tight, 800.0, "Copied", "3 files."), tight.across);
    }

    #[test]
    fn a_stack_of_mixed_sizes_sits_flush_each_its_own_size_and_a_small_one_grows_on_pointing() {
        let mut s = stack();
        let all = circles_of_every_length();
        for (id, rest) in all {
            s.add_sized(id, 0, rest, 0);
            assert_eq!(s.rest(id), rest);
            assert_eq!(s.across(id, 0), rest, "worked out once: it is that size from the start");
        }
        let k = kit();
        let at = |s: &Stack, now| s.places(Corner::BottomRight, SCREEN, now);
        let put = at(&s, 0);
        for ((id, rest), (pid, x, _y, d)) in all.iter().zip(&put) {
            assert_eq!((id, rest), (pid, d));
            assert_eq!(*x, SCREEN.0 - k.margin - d, "flush with the side");
        }
        assert_eq!(put[0].2 + put[0].3, SCREEN.1 - k.margin - (SCREEN.1 * crate::ring::circles::EDGE_SHARE).round(), "the first is a fifth of the way up");
        for w in put.windows(2) {
            assert_eq!(w[1].2 + w[1].3 + k.gap, w[0].2, "a gap apart, no more");
        }
        assert_eq!(at(&s, 10_000_000), put);
        let grown = 150.0f32.max(circles::POINTED_LEAST);
        s.point(1, true, grown, 1_000);
        assert_eq!(s.across(1, 1_000 + SLIDE), grown);
        let grown_put = at(&s, 1_000 + SLIDE);
        assert_eq!(grown_put[1].2 + grown_put[1].3 + k.gap, grown_put[0].2, "the next one slid along");
        assert!(grown_put[1].2 < put[1].2);
        s.point(2, true, 10.0, 2_000);
        assert_eq!(s.across(2, 2_000 + SLIDE), all[1].1);
        s.point(1, false, all[0].1, 5_000);
        assert_eq!(s.across(1, 5_000 + SLIDE), all[0].1);
        assert_eq!(at(&s, 5_000 + SLIDE + 1_000)[0].3, all[0].1);
    }

    #[test]
    fn a_small_pinned_circle_keeps_its_size_when_picked_up_and_after_a_snooze() {
        let mut s = stack();
        let [(_, small), ..] = circles_of_every_length();
        s.add_sized(1, 0, small, 0);
        s.toggle_pin(1, 0);
        assert!(s.pick_up(1, (small / 2.0, small / 2.0), Corner::BottomRight, SCREEN, 0));
        s.drag(1, (500.0, 500.0), (small / 2.0, small / 2.0), SCREEN, 0);
        let p = s.places(Corner::BottomRight, SCREEN, 0);
        assert_eq!((p[0].1, p[0].2, p[0].3), (500.0, 500.0, small), "it follows the pointer, still its own size");
        s.drop_held(1);
        s.toggle_pin(1, 0);
        s.snooze(1, Snooze::Hour1, (12, 0, 0), 0);
        assert_eq!(s.tick(3_600_000).back, vec![1]);
        assert_eq!(s.across(1, 3_600_000), small, "back at the size it had");
    }

    #[test]
    fn the_rest_size_is_held_to_the_least_and_a_long_message_may_pass_the_sheets_size() {
        let mut s = stack();
        s.add_sized(1, 0, 250.0, 0);
        s.restyle(Sizes { across: 144.0, ..kit() }, SLIDE, 0);
        assert_eq!(s.rest(1), 250.0);
        s.set_rest(1, 10.0, 0);
        assert_eq!(s.rest(1), 96.0, "never under the least");
    }

    #[test]
    fn a_pinned_circle_keeps_its_place_and_a_snoozed_one_its_time_across_a_restart() {
        let mut s = stack();
        s.restore(1, 120.0, Some((300.0, 200.0)), None, 1_000);
        s.restore(2, 120.0, None, Some(5_000), 1_000);
        assert_eq!(s.pinned_places(), vec![(1, Some((300.0, 200.0)))]);
        assert_eq!(s.sleepers(), vec![(2, 5_000)]);
        let at = s.places(Corner::BottomRight, SCREEN, 1_000);
        assert_eq!((at[0].1, at[0].2), (300.0, 200.0));
        assert!(s.tick(4_000).back.is_empty());
        assert_eq!(s.tick(5_000).back, vec![2]);
        assert!(s.has(1) && s.has(2) && s.sleepers().is_empty());
    }
}
