use bevy::prelude::Vec2;
use bevy_ecs_tilemap::tiles::TilePos;
use smallvec::SmallVec;

use super::{biomes::Biomes, TILE_SIZE};

pub struct Map {
    pub tiles: Vec<Biomes>,
    pub height: u32,
    pub width: u32,
}

impl Map {
    pub fn new(height: u32, width: u32) -> Self {
        Map {
            tiles: vec![Biomes::None; (height * width) as usize],
            height,
            width,
        }
    }

    #[allow(dead_code)]
    pub fn tile_xy_idx(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    #[allow(dead_code)]
    pub fn idx_tile_xy(&self, idx: usize) -> TilePos {
        TilePos::new(idx as u32 % self.width, idx as u32 / self.width)
    }

    #[allow(dead_code)]
    pub fn world_xy_idx(&self, x: f32, y: f32) -> usize {
        self.tile_xy_idx((x / TILE_SIZE.x).floor() as u32, (y / TILE_SIZE.y).floor() as u32)
    }

    #[allow(dead_code)]
    pub fn idx_world_xy(&self, idx: usize) -> Vec2 {
        Vec2::new(
            (idx as u32 % self.width) as f32 * TILE_SIZE.x,
            (idx as u32 / self.width) as f32 * TILE_SIZE.y,
        )
    }

    pub fn is_passable(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        !self.tiles[self.tile_xy_idx(x, y)].is_obstacle()
    }

    pub fn get_passable_neighbors(&self, x: u32, y: u32) -> SmallVec<[(TilePos, u32); 10]> {
        let mut exits = SmallVec::new();
        let t_t = self.tiles[self.tile_xy_idx(x, y)];

        //Check cardinal directions
        if x > 0 && self.is_passable(x - 1, y) {
            exits.push((TilePos::new(x - 1, y), t_t.distance()));
        }
        if self.is_passable(x + 1, y) {
            exits.push((TilePos::new(x + 1, y), t_t.distance()));
        }
        if y > 0 && self.is_passable(x, y - 1) {
            exits.push((TilePos::new(x, y - 1), t_t.distance()));
        }
        if self.is_passable(x, y + 1) {
            exits.push((TilePos::new(x, y + 1), t_t.distance()));
        }

        //Check diagonal directions
        if x > 0 && y > 0 && self.is_passable(x - 1, y - 1) {
            exits.push((
                TilePos::new(x - 1, y - 1),
                (t_t.distance() as f32 * 1.45).floor() as u32,
            ));
        }
        if y > 0 && self.is_passable(x + 1, y - 1) {
            exits.push((
                TilePos::new(x + 1, y - 1),
                (t_t.distance() as f32 * 1.45).floor() as u32,
            ));
        }
        if x > 0 && self.is_passable(x - 1, y + 1) {
            exits.push((
                TilePos::new(x - 1, y + 1),
                (t_t.distance() as f32 * 1.45).floor() as u32,
            ));
        }
        if self.is_passable(x + 1, y + 1) {
            exits.push((
                TilePos::new(x + 1, y + 1),
                (t_t.distance() as f32 * 1.45).floor() as u32,
            ));
        }

        exits
    }

    pub fn is_neighbor(&self, pos1: &TilePos, pos2: &TilePos) -> bool {
        pos1.x.abs_diff(pos2.x) <= 1 && pos1.y.abs_diff(pos2.y) <= 1
    }
}

#[allow(dead_code)]
pub fn world_xy_tile_xy(position: Vec2) -> TilePos {
    TilePos::new(
        (position.x / TILE_SIZE.x).floor() as u32,
        (position.y / TILE_SIZE.y).floor() as u32,
    )
}

#[allow(dead_code)]
pub fn tile_xy_world_xy(x: u32, y: u32) -> Vec2 {
    Vec2 {
        x: x as f32 * TILE_SIZE.x,
        y: y as f32 * TILE_SIZE.y,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tile_xy_idx_round_trip() {
        let map = Map::new(1000, 1000);

        let idx = map.tile_xy_idx(100, 100);
        assert_eq!(idx, 100100);

        let tile_pos = map.idx_tile_xy(idx);
        assert_eq!(tile_pos.x, 100);
        assert_eq!(tile_pos.y, 100);
    }

    #[test]
    fn world_xy_idx_round_trip() {
        let map = Map::new(1000, 1000);

        let idx = map.world_xy_idx(3200.0, 3200.0);
        assert_eq!(idx, 100100);

        let coords = map.idx_world_xy(idx);
        assert_eq!(coords.x, 3200.0);
        assert_eq!(coords.y, 3200.0);
    }
}
