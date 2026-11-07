use std::mem;

use crate::utils::{Physical, Rectangle, Size};

const DEFAULT_MIN_TILE_SIDE: i32 = 16;

const MAX_DAMAGE_TO_DAMAGE_BBOX_RATIO: f32 = 0.9;

const TILE_DAMAGE_GAP_FRACTION: i32 = 4;

#[derive(Debug, Default)]
pub struct DamageShaper<const MIN_TILE_SIDE: i32 = DEFAULT_MIN_TILE_SIDE> {
    tiles_cache: Vec<Tile>,
    out_damage: Vec<Rectangle<i32, Physical>>,
}

impl<const MIN_TILE_SIDE: i32> DamageShaper<MIN_TILE_SIDE> {
    #[profiling::function]
    pub fn shape_damage(&mut self, in_damage: &mut Vec<Rectangle<i32, Physical>>) {
        self.out_damage.clear();
        self.tiles_cache.clear();

        self.shape_damage_impl(in_damage, None, false);

        mem::swap(&mut self.out_damage, in_damage);
    }

    #[inline(never)]
    #[profiling::function]
    fn shape_damage_impl(
        &mut self,
        in_damage: &mut [Rectangle<i32, Physical>],
        last_direction: Option<DamageSplitAxis>,
        invert_direction: bool,
    ) {
        if in_damage.is_empty() {
            return;
        } else if in_damage.len() == 1 {
            self.out_damage.push(in_damage[0]);
            return;
        }

        let (x_min, y_min, x_max, y_max, max_damage_area) =
            in_damage
                .iter()
                .fold((i32::MAX, i32::MAX, i32::MIN, i32::MIN, i32::MIN), |acc, rect| {
                    let area = rect.size.w * rect.size.h;
                    (
                        acc.0.min(rect.loc.x),
                        acc.1.min(rect.loc.y),
                        acc.2.max(rect.loc.x + rect.size.w),
                        acc.3.max(rect.loc.y + rect.size.h),
                        acc.4.max(area),
                    )
                });

        let bbox_w = x_max - x_min;
        let bbox_h = y_max - y_min;

        let damage_bbox = Rectangle::<i32, Physical>::new((x_min, y_min).into(), (bbox_w, bbox_h).into());

        if max_damage_area as f32 / (damage_bbox.size.w * damage_bbox.size.h) as f32
            > MAX_DAMAGE_TO_DAMAGE_BBOX_RATIO
        {
            self.out_damage.push(damage_bbox);
            return;
        }

        let mut direction = if bbox_w >= bbox_h {
            DamageSplitAxis::X
        } else {
            DamageSplitAxis::Y
        };

        if invert_direction {
            direction = direction.invert();
        }

        let mut overlap_end = match direction {
            DamageSplitAxis::X => {
                if Some(direction) != last_direction {
                    in_damage.sort_unstable_by(|lhs, rhs| {
                        lhs.loc.x.cmp(&rhs.loc.x).then(rhs.size.w.cmp(&lhs.size.w))
                    });
                }
                in_damage[0].loc.x + in_damage[0].size.w
            }
            DamageSplitAxis::Y => {
                if Some(direction) != last_direction {
                    in_damage.sort_unstable_by(|lhs, rhs| {
                        lhs.loc.y.cmp(&rhs.loc.y).then(rhs.size.h.cmp(&lhs.size.h))
                    });
                }
                in_damage[0].loc.y + in_damage[0].size.h
            }
        };

        let mut overlap_start_idx = 0;
        for idx in overlap_start_idx + 1..in_damage.len() {
            let rect = in_damage[idx];
            let (rect_start, rect_end) = match direction {
                DamageSplitAxis::X => (rect.loc.x, rect.loc.x + rect.size.w),
                DamageSplitAxis::Y => (rect.loc.y, rect.loc.y + rect.size.h),
            };

            if rect_start >= overlap_end {
                self.shape_damage_impl(&mut in_damage[overlap_start_idx..idx], Some(direction), false);

                overlap_start_idx = idx;
                overlap_end = rect_end;
            } else {
                overlap_end = overlap_end.max(rect_end);
            }
        }

        if overlap_start_idx == 0 && invert_direction {
            const NUM_TILES: i32 = 4;
            let (tile_w, tile_h) = match direction.invert() {
                DamageSplitAxis::X => (bbox_w / NUM_TILES, bbox_h / (NUM_TILES * 2)),
                DamageSplitAxis::Y => (bbox_w / (NUM_TILES * 2), bbox_h / NUM_TILES),
            };
            let tile_size = (tile_w.max(MIN_TILE_SIDE), tile_h.max(MIN_TILE_SIDE)).into();

            self.shape_damage_tiled(in_damage, damage_bbox, tile_size);
        } else {
            self.shape_damage_impl(
                &mut in_damage[overlap_start_idx..],
                Some(direction),
                overlap_start_idx == 0,
            );
        }
    }

    #[inline]
    #[profiling::function]
    fn shape_damage_tiled(
        &mut self,
        in_damage: &[Rectangle<i32, Physical>],
        bbox: Rectangle<i32, Physical>,
        tile_size: Size<i32, Physical>,
    ) {
        self.tiles_cache.clear();
        let tile_gap: Size<i32, Physical> = From::from((
            tile_size.w / TILE_DAMAGE_GAP_FRACTION,
            tile_size.h / TILE_DAMAGE_GAP_FRACTION,
        ));

        for x in (bbox.loc.x..bbox.loc.x + bbox.size.w).step_by(tile_size.w as usize) {
            let mut tiles_in_column = 0;
            for y in (bbox.loc.y..bbox.loc.y + bbox.size.h).step_by(tile_size.h as usize) {
                let bbox = Rectangle::<i32, Physical>::new((x, y).into(), tile_size);
                let mut tile = Tile {
                    bbox,
                    damage: None,
                    #[cfg(test)]
                    merged_tiles: 0,
                };

                for damage_rect in in_damage.iter() {
                    if let Some(intersection) = tile.bbox.intersection(*damage_rect) {
                        tile.damage = if let Some(tile_damage) = tile.damage {
                            Some(intersection.merge(tile_damage))
                        } else {
                            Some(intersection)
                        };
                    }
                }

                tiles_in_column += 1;
                self.tiles_cache.push(tile);
            }

            let num_tiles = self.tiles_cache.len();
            for idx in num_tiles - tiles_in_column..num_tiles - 1 {
                let (damage, adjacent_damage) =
                    match (self.tiles_cache[idx].damage, self.tiles_cache[idx + 1].damage) {
                        (Some(damage), Some(adjacent_damage)) => (damage, adjacent_damage),
                        _ => continue,
                    };

                if damage.loc.y + damage.size.h + tile_gap.h >= adjacent_damage.loc.y
                    && (damage.size.w - adjacent_damage.size.w).abs() < tile_gap.w
                {
                    self.tiles_cache[idx].damage = None;
                    self.tiles_cache[idx + 1].damage = Some(damage.merge(adjacent_damage));
                    #[cfg(test)]
                    {
                        self.tiles_cache[idx + 1].merged_tiles = self.tiles_cache[idx].merged_tiles + 1;
                    }
                }
            }
        }

        self.out_damage
            .extend(self.tiles_cache.iter().filter_map(|tile| tile.damage));
    }
}

#[derive(Debug)]
struct Tile {
    bbox: Rectangle<i32, Physical>,
    damage: Option<Rectangle<i32, Physical>>,
    #[cfg(test)]
    merged_tiles: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DamageSplitAxis {
    X,
    Y,
}

impl DamageSplitAxis {
    fn invert(self) -> Self {
        match self {
            DamageSplitAxis::X => DamageSplitAxis::Y,
            DamageSplitAxis::Y => DamageSplitAxis::X,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shaper() -> DamageShaper {
        DamageShaper::default()
    }

    fn check_tiles(tiles: &[Tile]) {
        let mut xs: Vec<i32> = tiles.iter().map(|tile| tile.bbox.loc.x).collect();
        let rows = xs.iter().filter(|&&x| tiles[0].bbox.loc.x == x).count();
        xs.dedup();
        let cols = xs.len();

        let mut bbox = tiles[0].bbox;
        for col in 0..cols {
            for row in 0..rows {
                let tile = &tiles[col * rows + row];
                assert_eq!(bbox, tile.bbox);
                if let Some(damage) = tile.damage {
                    assert!(tile.bbox.loc.x <= damage.loc.x && damage.size.w <= tile.bbox.size.w);
                    assert!(
                        tile.bbox.loc.y - tile.bbox.size.h * tile.merged_tiles <= damage.loc.y
                            && damage.size.h <= tile.bbox.loc.y + tile.bbox.size.h
                    );
                }
                bbox.loc.y += bbox.size.h;
            }
            bbox.loc.y = tiles[0].bbox.loc.y;
            bbox.loc.x += bbox.size.w;
        }
    }

    fn damage_area(damage: &[Rectangle<i32, Physical>]) -> i32 {
        let mut area = 0;
        for rect in damage {
            area += rect.size.w * rect.size.h;
        }
        area
    }

    #[test]
    fn tile_shaping() {
        let mut damage = vec![
            Rectangle::<i32, Physical>::new((98, 406).into(), (36, 48).into()),
            Rectangle::<i32, Physical>::new((158, 502).into(), (828, 168).into()),
            Rectangle::<i32, Physical>::new((122, 694).into(), (744, 528).into()),
            Rectangle::<i32, Physical>::new((194, 1318).into(), (420, 72).into()),
            Rectangle::<i32, Physical>::new((146, 1414).into(), (312, 48).into()),
            Rectangle::<i32, Physical>::new((32, 406).into(), (108, 1152).into()),
        ];

        let mut shaper = shaper();
        let mut to_shape = damage.clone();
        shaper.shape_damage(&mut to_shape);
        check_tiles(&shaper.tiles_cache);

        let mut to_shape_again = to_shape.clone();
        shaper.shape_damage(&mut to_shape_again);
        to_shape_again.sort_by_key(|rect| rect.loc.x);
        to_shape.sort_by_key(|rect| rect.loc.x);
        assert_eq!(to_shape, to_shape_again);
        assert!(shaper.tiles_cache.is_empty());

        let bbox = Rectangle::<i32, Physical>::from_size((3840, 2160).into());
        damage.push(bbox);
        shaper.shape_damage(&mut damage);
        assert_eq!(damage[0], bbox);
        assert!(shaper.tiles_cache.is_empty());
    }

    #[test]
    fn small_damage() {
        let mut shaper = shaper();
        let mut damage = vec![];
        shaper.shape_damage(&mut damage);
        assert!(damage.is_empty());

        let rect = Rectangle::<i32, Physical>::from_size((5, 5).into());
        damage.push(rect);
        shaper.shape_damage(&mut damage);
        assert!(damage.len() == 1);
        assert_eq!(damage[0], rect);
    }

    #[test]
    fn shape_pixels() {
        let mut shaper = shaper();
        let mut damage = vec![];
        shaper.shape_damage(&mut damage);
        assert!(damage.is_empty());

        let w = 384;
        let h = 216;

        for x in 0..w {
            for y in 0..h {
                let rect = Rectangle::<i32, Physical>::new((x, y).into(), (1, 1).into());
                damage.push(rect);
            }
        }

        let mut to_shape = damage.clone();
        shaper.shape_damage(&mut to_shape);
        assert!(shaper.tiles_cache.is_empty());
        assert_eq!(damage_area(&to_shape), w * h);
        to_shape.sort_by_key(|rect| rect.loc.x);
        assert_eq!(damage, to_shape);

        let w1 = 216;
        let h1 = 144;
        let overlap1 = Rectangle::<i32, Physical>::from_size((w1, h1).into());
        let overlap2 = Rectangle::<i32, Physical>::new((w1, h1).into(), (w - w1, h - h1).into());
        damage.push(overlap1);
        damage.push(overlap2);

        shaper.shape_damage(&mut damage);
        assert!(shaper.tiles_cache.is_empty());
        assert_eq!(damage_area(&damage), w * h);
        assert!(damage.contains(&overlap1));
        assert!(damage.contains(&overlap2));
    }
}
