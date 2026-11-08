//! Ring geometry: favorites round the center, and per-category pieces in outer bands.

use std::f64::consts::PI;

pub const CENTRE: f64 = 252.0;
const INNER_MIN: f64 = 150.0;
pub const ICON_SIZES: [i32; 6] = [48, 40, 32, 28, 24, 20];
const PAD: f64 = 12.0;
pub const MARGIN: f64 = 40.0;
const NAME_SPACE: f64 = 24.0;
const NAMES_FROM: i32 = 32;
const BAND_GAP: f64 = 12.0;
const PIECE_GAP: f64 = 12.0;
pub const LABEL_BAND: f64 = 18.0;
const LABEL_ARC: f64 = 96.0;
const MAX_BANDS: usize = 40;
const STEPS: usize = 2880;

#[derive(Clone, Debug)]
pub struct Placed<T> {
    pub item: T,
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug)]
pub struct Piece {
    pub category: String,
    pub start: f64,
    pub end: f64,
    pub inner: f64,
    pub outer: f64,

    pub label_radius: f64,

    pub straight: Option<Straight>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Straight {
    pub x: f64,
    pub y: f64,
    pub turn: f64,
    pub length: f64,
}

pub const LABEL_INSET: f64 = 13.0;

fn straight_label(start: f64, end: f64, outer: f64, room: Room) -> Option<Straight> {
    let mid = (start + end) / 2.0;
    if room.edge(mid) >= outer - 1e-6 {
        return None;
    }
    let (c, s) = (mid.cos(), mid.sin());
    let across = c.abs() < 1e-9 || room.half_h / s.abs() <= room.half_w / c.abs();

    let along = |a: f64, line: f64, horizontal: bool| -> Option<f64> {
        let (ca, sa) = (a.cos(), a.sin());
        let (towards, sideways) = if horizontal { (sa, ca) } else { (ca, sa) };
        if towards * line <= 0.0 {
            return None;
        }
        let limit = if horizontal { room.half_w } else { room.half_h };
        let ring = (outer * outer - line * line).max(0.0).sqrt();
        Some((line * sideways / towards).clamp(-limit.min(ring), limit.min(ring)))
    };
    if across {
        let line = s.signum() * (room.half_h - LABEL_INSET);
        let (a, b) = (along(start, line, true)?, along(end, line, true)?);
        Some(Straight { x: (a + b) / 2.0, y: line, turn: 0.0, length: (a - b).abs() })
    } else {
        let line = c.signum() * (room.half_w - LABEL_INSET);
        let (a, b) = (along(start, line, false)?, along(end, line, false)?);

        let turn = if c > 0.0 { PI / 2.0 } else { -PI / 2.0 };
        Some(Straight { x: line, y: (a + b) / 2.0, turn, length: (a - b).abs() })
    }
}

#[derive(Clone, Debug)]
pub struct Layout<T> {
    pub icon: i32,
    pub pitch: f64,
    pub names: bool,
    pub inner_radius: f64,
    pub disc_radius: f64,
    pub outer_radius: f64,
    pub placed: Vec<Placed<T>>,
    pub pieces: Vec<Piece>,

    pub clip: Option<(f64, f64, f64, f64)>,
}

pub fn pitch_for(icon: i32, names: bool) -> f64 {
    icon as f64 + 2.0 * PAD + if names && icon >= NAMES_FROM { NAME_SPACE } else { 0.0 }
}

pub fn button_size(icon: i32, names: bool) -> (f64, f64) {
    let h = icon as f64 + 10.0 + if names && icon >= NAMES_FROM { NAME_SPACE - 4.0 } else { 0.0 };
    (h, h)
}

fn reach(icon: i32, names: bool) -> f64 {
    button_size(icon, names).1 / 2.0
}

const CLEAR: f64 = 8.0;

fn inner_radius(n: usize, pitch: f64, icon: i32) -> f64 {
    let need = if n > 0 { n as f64 * pitch / (2.0 * PI) } else { 0.0 };
    INNER_MIN.max(need).max(CENTRE / 2.0 + reach(icon, true) + CLEAR + 6.0)
}

fn disc_radius(inner: f64, icon: i32) -> f64 {
    inner + reach(icon, true) + CLEAR
}

fn around<T: Clone>(items: &[T], radius: f64, cx: f64, cy: f64) -> Vec<Placed<T>> {
    let n = items.len() as f64;
    items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let a = -PI / 2.0 + i as f64 * 2.0 * PI / n;
            Placed { item: item.clone(), x: cx + radius * a.cos(), y: cy + radius * a.sin() }
        })
        .collect()
}

fn angle(i: f64) -> f64 {
    -PI / 2.0 + i * 2.0 * PI / STEPS as f64
}

#[derive(Clone, Copy)]
struct Room {
    half_w: f64,
    half_h: f64,
}

impl Room {

    fn edge(&self, a: f64) -> f64 {
        let (c, s) = (a.cos().abs(), a.sin().abs());
        let x = if c > 1e-9 { self.half_w / c } else { f64::INFINITY };
        let y = if s > 1e-9 { self.half_h / s } else { f64::INFINITY };
        x.min(y)
    }
}

struct Band {
    radius: f64,
    fits: Vec<bool>,
    whole: bool,
}

fn band(radius: f64, icon: i32, room: Room) -> Band {
    let (bw, bh) = button_size(icon, true);
    let (lim_x, lim_y) = (room.half_w - LABEL_BAND - bw / 2.0, room.half_h - LABEL_BAND - bh / 2.0);
    let fits: Vec<bool> = (0..STEPS)
        .map(|i| {
            let a = angle(i as f64 + 0.5);
            (radius * a.cos()).abs() <= lim_x && (radius * a.sin()).abs() <= lim_y
        })
        .collect();
    let whole = fits.iter().all(|f| *f);
    Band { radius, fits, whole }
}

fn runs(b: &Band, from: usize, to: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    let mut i = from;
    while i < to {
        if b.fits[i % STEPS] {
            let s = i;
            while i < to && b.fits[i % STEPS] {
                i += 1;
            }
            out.push((s, i - s));
        } else {
            i += 1;
        }
    }
    out
}

fn holds(b: &Band, steps: usize, pitch: f64) -> usize {
    let arc = steps as f64 * 2.0 * PI / STEPS as f64 * b.radius;
    (arc / pitch + 1e-9).floor() as usize
}

fn capacity(b: &Band, from: usize, to: usize, pitch: f64) -> usize {
    runs(b, from, to).iter().map(|(_, n)| holds(b, *n, pitch)).sum()
}

struct Plan {
    icon: i32,
    pitch: f64,
    inner: f64,
    piece_inner: f64,
    bands: Vec<Band>,

    spans: Vec<(usize, usize)>,
}

fn fill(sizes: &[usize], bands: &[Band], pitch: f64, piece_inner: f64, label_r: f64, extra: usize)
        -> Option<Vec<(usize, usize)>> {
    let gap = ((PIECE_GAP / piece_inner) / (2.0 * PI) * STEPS as f64).ceil() as usize;
    let label = ((LABEL_ARC / label_r) / (2.0 * PI) * STEPS as f64).ceil() as usize;
    let mut spans = Vec::new();
    let mut at = gap / 2;
    for &n in sizes {
        let mut end = at;
        let mut done = vec![0usize; bands.len()];
        let mut open = vec![0usize; bands.len()];
        loop {
            if end >= STEPS - gap / 2 {
                return None;
            }
            for (k, b) in bands.iter().enumerate() {
                if b.fits[end] {
                    open[k] += 1;
                } else if open[k] > 0 {
                    done[k] += holds(b, open[k], pitch);
                    open[k] = 0;
                }
            }
            end += 1;
            let have: usize = bands.iter().enumerate().map(|(k, b)| done[k] + holds(b, open[k], pitch)).sum();
            if have >= n && end - at >= label {
                break;
            }
        }

        end += extra;
        if end >= STEPS - gap / 2 {
            return None;
        }
        spans.push((at, end));
        at = end + gap;
    }
    Some(spans)
}

fn widest(sizes: &[usize], bands: &[Band], pitch: f64, piece_inner: f64, label_r: f64) -> Option<Vec<(usize, usize)>> {
    let tight = fill(sizes, bands, pitch, piece_inner, label_r, 0)?;
    let (mut low, mut high) = (0usize, STEPS / sizes.len().max(1));
    while low < high {
        let mid = (low + high).div_ceil(2);
        if fill(sizes, bands, pitch, piece_inner, label_r, mid).is_some() {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    let mut spans = if low == 0 { tight } else { fill(sizes, bands, pitch, piece_inner, label_r, low)? };
    let gap = ((PIECE_GAP / piece_inner) / (2.0 * PI) * STEPS as f64).ceil() as usize;
    if let Some(last) = spans.last_mut() {
        last.1 = last.1.max(STEPS - gap / 2 - 1);
    }
    Some(spans)
}

fn try_plan(first: usize, sizes: &[usize], icon: i32, n_bands: usize, room: Room, partial: bool) -> Option<Plan> {
    let pitch = pitch_for(icon, true);
    let inner = inner_radius(first, pitch, icon);
    let piece_inner = disc_radius(inner, icon) + BAND_GAP;
    let bands: Vec<Band> = (0..n_bands).map(|i| band(piece_inner + pitch / 2.0 + i as f64 * pitch, icon, room)).collect();
    if bands.iter().any(|b| !b.fits.iter().any(|f| *f)) || (!partial && bands.iter().any(|b| !b.whole)) {
        return None;
    }
    let outer = piece_inner + n_bands as f64 * pitch + LABEL_BAND;

    let label_r = outer.min(room.half_w.min(room.half_h));
    let spans = widest(sizes, &bands, pitch, piece_inner, label_r)?;
    Some(Plan { icon, pitch, inner, piece_inner, bands, spans })
}

fn plan(first: usize, sizes: &[usize], room: Room) -> Option<Plan> {
    let named: Vec<i32> = ICON_SIZES.iter().copied().filter(|i| *i >= NAMES_FROM).collect();

    for &icon in &named {
        for n in 1..=MAX_BANDS {
            let fits = try_plan(first, sizes, icon, n, room, false);
            if fits.is_some() {
                return fits;
            }

            let pitch = pitch_for(icon, true);
            let piece_inner = disc_radius(inner_radius(first, pitch, icon), icon) + BAND_GAP;
            if !band(piece_inner + pitch / 2.0 + (n - 1) as f64 * pitch, icon, room).whole {
                break;
            }
        }
    }

    let smallest = *named.last().unwrap_or(&NAMES_FROM);
    for &icon in std::iter::once(&smallest).chain(ICON_SIZES.iter().filter(|i| **i < NAMES_FROM)) {
        for n in 1..=MAX_BANDS {
            if let Some(p) = try_plan(first, sizes, icon, n, room, true) {
                return Some(p);
            }
        }
    }
    None
}

fn place_on_band<T: Clone>(items: &[T], b: &Band, from: usize, to: usize, pitch: f64, cx: f64, cy: f64,
                           out: &mut Vec<Placed<T>>) {
    let parts = runs(b, from, to);
    let caps: Vec<usize> = parts.iter().map(|(_, n)| holds(b, *n, pitch)).collect();
    let total: usize = caps.iter().sum();
    if total == 0 || items.is_empty() {
        return;
    }

    let mut takes: Vec<usize> = caps.iter().map(|&cap| (items.len() * cap / total).min(cap)).collect();
    let mut spare = items.len() - takes.iter().sum::<usize>();
    for (t, &cap) in takes.iter_mut().zip(&caps) {
        let add = (cap - *t).min(spare);
        *t += add;
        spare -= add;
    }
    let mut given = 0;
    for (&(s, n), &take) in parts.iter().zip(&takes) {
        let step = n as f64 / take.max(1) as f64;
        for i in 0..take {
            let a = angle(s as f64 + step * (i as f64 + 0.5));
            out.push(Placed { item: items[given + i].clone(), x: cx + b.radius * a.cos(), y: cy + b.radius * a.sin() });
        }
        given += take;
    }
}

fn alone<T: Clone>(first: &[T], icon: i32, width: f64, height: f64) -> Layout<T> {
    let (cx, cy) = (width / 2.0, height / 2.0);
    let pitch = pitch_for(icon, true);
    let inner = inner_radius(first.len(), pitch, icon);
    let disc = disc_radius(inner, icon);
    Layout { icon, pitch, names: true, inner_radius: inner, disc_radius: disc, outer_radius: disc,
             placed: around(first, inner, cx, cy), pieces: Vec::new(), clip: None }
}

pub fn ring_holds(icon: i32, disc_most: f64) -> usize {
    let pitch = pitch_for(icon, true);
    let inner = (disc_most - reach(icon, true) - CLEAR).max(inner_radius(0, pitch, icon));
    ((2.0 * PI * inner / pitch) + 1e-9).floor() as usize
}

pub fn search_ring<T: Clone>(found: &[T], width: f64, height: f64, reserve: f64) -> Layout<T> {
    let disc_most = width.min(height) / 2.0 - MARGIN - reserve;
    let named = ICON_SIZES.iter().copied().filter(|i| *i >= NAMES_FROM);
    let icon = named.clone().find(|&i| found.len() <= ring_holds(i, disc_most)).or(named.clone().next_back()).unwrap_or(NAMES_FROM);
    let take = found.len().min(ring_holds(icon, disc_most));
    alone(&found[..take], icon, width, height)
}

pub fn layout<T: Clone>(first: &[T], groups: &[(String, Vec<T>)], width: f64, height: f64) -> Layout<T> {
    let (cx, cy) = (width / 2.0, height / 2.0);
    let room = Room { half_w: width / 2.0 - MARGIN, half_h: height / 2.0 - MARGIN };

    if groups.is_empty() {
        let icon = ICON_SIZES[0];
        return alone(first, icon, width, height);
    }

    let sizes: Vec<usize> = groups.iter().map(|(_, v)| v.len()).collect();
    let Some(p) = plan(first.len(), &sizes, room) else {

        let icon = ICON_SIZES[ICON_SIZES.len() - 1];
        let pitch = pitch_for(icon, false);
        let inner = inner_radius(first.len(), pitch, icon);
        let disc = disc_radius(inner, icon);
        return Layout { icon, pitch, names: false, inner_radius: inner, disc_radius: disc, outer_radius: disc,
                        placed: around(first, inner, cx, cy), pieces: Vec::new(), clip: None };
    };

    let disc = disc_radius(p.inner, p.icon);
    let outer = p.piece_inner + p.bands.len() as f64 * p.pitch + LABEL_BAND;
    let partial = p.bands.iter().any(|b| !b.whole);
    let mut placed = around(first, p.inner, cx, cy);
    let mut pieces = Vec::new();
    for ((category, items), &(from, to)) in groups.iter().zip(&p.spans) {
        let mut rest: &[T] = items;
        for b in &p.bands {
            if rest.is_empty() {
                break;
            }
            let take = capacity(b, from, to, p.pitch).min(rest.len());
            place_on_band(&rest[..take], b, from, to, p.pitch, cx, cy, &mut placed);
            rest = &rest[take..];
        }
        let (start, end) = (angle(from as f64), angle(to as f64));
        let mid = (start + end) / 2.0;
        let label_radius = outer.min(room.edge(mid));
        let straight = straight_label(start, end, outer, room);
        pieces.push(Piece { category: category.clone(), start, end, inner: p.piece_inner, outer, label_radius, straight });
    }
    let clip = partial.then_some((MARGIN, MARGIN, width - 2.0 * MARGIN, height - 2.0 * MARGIN));
    let outer_radius = if partial { outer.min(room.half_h) } else { outer };
    Layout { icon: p.icon, pitch: p.pitch, names: p.icon >= NAMES_FROM, inner_radius: p.inner, disc_radius: disc,
             outer_radius, placed, pieces, clip }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn overlaps<T>(p: &[Placed<T>], pitch: f64) -> bool {
        for i in 0..p.len() {
            for j in i + 1..p.len() {
                if (p[i].x - p[j].x).hypot(p[i].y - p[j].y) < pitch - 1.0 {
                    return true;
                }
            }
        }
        false
    }

    fn on_screen<T>(lay: &Layout<T>, w: f64, h: f64) -> bool {
        let (bw, bh) = button_size(lay.icon, lay.names);
        lay.placed.iter().all(|p| p.x - bw / 2.0 >= MARGIN - 0.5 && p.x + bw / 2.0 <= w - MARGIN + 0.5
            && p.y - bh / 2.0 >= MARGIN - 0.5 && p.y + bh / 2.0 <= h - MARGIN + 0.5)
    }

    #[test]
    fn buttons_stay_inside_the_disc_and_clear_of_the_middle() {
        for n in [1, 3, 5, 8, 12] {
            let first: Vec<u32> = (0..n).collect();
            let lay = layout(&first, &[], 1920.0, 1080.0);
            let r = reach(lay.icon, true);
            assert!(lay.inner_radius + r + CLEAR <= lay.disc_radius + 0.01);
            assert!(lay.inner_radius - r - CLEAR >= CENTRE / 2.0);
        }
    }

    #[test]
    fn closed_ring_is_only_the_favourites() {
        let lay = layout(&["a", "b", "c"], &[], 1920.0, 1080.0);
        assert_eq!(lay.placed.len(), 3);
        assert!(lay.pieces.is_empty());
    }

    #[test]
    fn everything_placed_once_without_overlap() {
        let office: Vec<String> = (0..20).map(|i| format!("o{i}")).collect();
        let games: Vec<String> = (0..3).map(|i| format!("g{i}")).collect();
        let system: Vec<String> = (0..40).map(|i| format!("s{i}")).collect();
        let lay = layout(&["f".to_string()], &[("office".into(), office), ("games".into(), games), ("system".into(), system)],
                         1920.0, 1080.0);
        assert_eq!(lay.placed.len(), 64);
        assert_eq!(lay.pieces.len(), 3);
        assert!(!overlaps(&lay.placed, lay.pitch));
        assert!(on_screen(&lay, 1920.0, 1080.0));
    }

    #[test]
    fn a_broken_band_still_shows_every_app() {
        let (radius, pitch) = (1000.0, 40.0);
        let one = (pitch / radius / (2.0 * PI) * STEPS as f64).ceil() as usize;
        let mut fits = vec![false; STEPS];
        for run in 0..5 {
            let start = run * (one + 4);
            fits[start..start + one].fill(true);
        }
        let band = Band { radius, fits, whole: false };
        let items: Vec<u32> = vec![1, 2];
        let mut out = Vec::new();
        place_on_band(&items, &band, 0, 5 * (one + 4), pitch, 0.0, 0.0, &mut out);
        assert_eq!(out.len(), items.len());
    }

    #[test]
    fn pieces_do_not_overlap_and_go_round_once() {
        let groups: Vec<(String, Vec<u32>)> = (0..6).map(|c| (format!("c{c}"), (0..(c * 3 + 1)).collect())).collect();
        let lay = layout(&[1u32, 2], &groups, 1920.0, 1080.0);
        for w in lay.pieces.windows(2) {
            assert!(w[0].end < w[1].start, "pieces touch");
        }
        let sweep = lay.pieces.last().unwrap().end - lay.pieces[0].start;
        assert!(sweep < 2.0 * PI);
    }

    #[test]
    fn a_name_cut_by_the_screen_edge_runs_level_along_it() {
        let room = Room { half_w: 628.0, half_h: 388.0 };

        let (start, end) = (0.55, 1.1);
        let s = super::straight_label(start, end, 700.0, room).expect("cut by the bottom");
        assert_eq!(s.turn, 0.0);
        assert!((s.y - (388.0 - LABEL_INSET)).abs() < 1e-9);
        assert!(s.length > 100.0);

        assert!(super::straight_label(start, end, 300.0, room).is_none());

        let side = super::straight_label(-0.3, 0.3, 700.0, room).expect("cut by the right");
        assert!((side.turn - PI / 2.0).abs() < 1e-9);
    }

    #[test]
    fn the_pieces_go_the_whole_way_round_with_no_empty_stretch() {

        let sizes = [5usize, 4, 6, 6, 3, 3, 5, 2];
        let groups: Vec<(String, Vec<u32>)> =
            sizes.iter().enumerate().map(|(c, n)| (format!("c{c}"), (0..*n as u32).collect())).collect();
        for (w, h) in [(1280.0, 800.0), (1920.0, 1080.0), (1366.0, 768.0)] {
            let lay = layout(&[1u32, 2, 3, 4], &groups, w, h);
            let mut gaps: Vec<f64> = lay.pieces.windows(2).map(|p| p[1].start - p[0].end).collect();
            gaps.push(lay.pieces[0].start + 2.0 * PI - lay.pieces.last().unwrap().end);
            let widest = gaps.iter().cloned().fold(0.0, f64::max);
            let narrowest = gaps.iter().cloned().fold(f64::MAX, f64::min);
            assert!(widest < narrowest * 2.0 + 0.01, "{w}x{h}: an empty stretch of {widest} radians");
        }
    }

    #[test]
    fn a_single_app_still_gets_a_piece_wide_enough_for_its_name() {
        let lay = layout(&["f"], &[("games".into(), vec!["g"]), ("system".into(), (0..10).map(|_| "s").collect())],
                         1920.0, 1080.0);
        let games = &lay.pieces[0];
        assert!((games.end - games.start) * games.label_radius >= LABEL_ARC - 1.0);
    }

    #[test]
    fn whole_bands_while_they_fit_and_named_icons_first() {

        let groups: Vec<(String, Vec<u32>)> = (0..5).map(|c| (format!("c{c}"), (0..6).collect())).collect();
        let lay = layout(&[1u32, 2, 3], &groups, 1536.0, 864.0);
        assert!(lay.clip.is_none(), "30 apps fit in whole bands");
        assert!(lay.names);
    }

    #[test]
    fn many_apps_go_into_bands_cut_off_at_the_top_and_bottom() {
        let groups: Vec<(String, Vec<u32>)> = (0..9).map(|c| (format!("c{c}"), (0..8).collect())).collect();
        let lay = layout(&[1u32, 2, 3], &groups, 1536.0, 864.0);
        assert!(lay.clip.is_some(), "72 apps need bands that are only partly there");
        assert!(lay.names, "still big enough for names");
        assert_eq!(lay.placed.len(), 3 + 72);
        assert!(!overlaps(&lay.placed, lay.pitch));
        assert!(on_screen(&lay, 1536.0, 864.0));
    }

    #[test]
    fn hundreds_shrink_and_still_stay_on_the_screen() {
        let many: Vec<u32> = (0..300).collect();
        let lay = layout(&[], &[("system".into(), many)], 1920.0, 1080.0);
        assert_eq!(lay.placed.len(), 300);
        assert!(!overlaps(&lay.placed, lay.pitch));
        assert!(on_screen(&lay, 1920.0, 1080.0));
    }

    const SCREENS: [(f64, f64); 7] = [(1289.0, 859.0), (1920.0, 1080.0), (1536.0, 864.0), (1366.0, 768.0), (1706.0, 1066.0), (1280.0, 800.0), (1920.0, 1080.0)];

    #[test]
    fn search_fills_one_ring_and_leaves_room_for_the_windows_ring() {
        let reserve = 8.0 + 88.0;
        for (w, h) in SCREENS {
            let disc_most = w.min(h) / 2.0 - MARGIN - reserve;
            let smallest = NAMES_FROM;
            for n in [0usize, 1, 3, 8, 15, 40, 400] {
                let found: Vec<u32> = (0..n as u32).collect();
                let lay = search_ring(&found, w, h, reserve);
                let at = format!("{w}x{h}, {n} found");
                assert!(lay.pieces.is_empty() && lay.clip.is_none(), "{at}: more than one ring");
                assert!(lay.names && lay.icon >= NAMES_FROM, "{at}: results carry their names");
                assert!(lay.placed.iter().map(|p| p.item).eq(0..lay.placed.len() as u32), "{at}: not best first");
                assert!(!overlaps(&lay.placed, lay.pitch), "{at}: icons touch");
                assert!(on_screen(&lay, w, h), "{at}: off the screen");
                let nearest = disc_radius(inner_radius(0, lay.pitch, lay.icon), lay.icon);
                assert!(lay.disc_radius <= disc_most.max(nearest) + 1e-6, "{at}: the windows ring would be pushed off");

                if lay.placed.len() < n {
                    assert_eq!(lay.icon, smallest, "{at}: shrank no further");
                    assert_eq!(lay.placed.len(), ring_holds(smallest, disc_most), "{at}: ring not full");
                } else {
                    assert_eq!(lay.placed.len(), n, "{at}");
                }
            }

            let many: Vec<u32> = (0..400).collect();
            let lay = search_ring(&many, w, h, reserve);
            assert!(lay.placed.len() > ring_holds(ICON_SIZES[0], disc_most), "{w}x{h}: no more than the largest icons hold");
            assert!(lay.placed.len() >= 8, "{w}x{h}: too few show");
        }
    }
}
