use bevy::prelude::Vec2;
use bevy_ecs_tilemap::tiles::TilePos;
use hierarchical_pathfinding::{internals::AbstractPath, PathCache, PathCacheConfig};
use if_chain::if_chain;

use super::{biomes::Biomes, features::Features, neighborhood::EuclideanNeighborhood, TILE_SIZE};

fn cost_fn(map: &Map) -> impl '_ + Sync + Fn((usize, usize)) -> isize {
    move |(x, y)| {
        let idx = map.tile_xy_idx(x.try_into().unwrap(), y.try_into().unwrap());
        if_chain! {
            if let Some(feature) = map.features[idx];
            if feature.is_obstacle();
            then {
                -1
            } else {
                map.tiles[idx].cost()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    pub tiles: Vec<Biomes>,
    pub features: Vec<Option<Features>>,
    pub path_cache: Option<PathCache<EuclideanNeighborhood>>,
    pub neighborhood: EuclideanNeighborhood,
    pub height: u32,
    pub width: u32,
}

impl Map {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            tiles: vec![Biomes::None; (height * width).try_into().unwrap()],
            features: vec![None; (height * width).try_into().unwrap()],
            height,
            width,
            neighborhood: EuclideanNeighborhood::new(width.try_into().unwrap(), height.try_into().unwrap()),
            path_cache: None,
        }
    }

    pub fn init_path_cache(&mut self) {
        let height = self.height.try_into().unwrap();
        let width = self.width.try_into().unwrap();

        self.path_cache = Some(PathCache::new(
            (width, height),
            cost_fn(self),
            self.neighborhood,
            PathCacheConfig::with_chunk_size(30),
        ));
    }

    pub fn get_path(&self, start_tile: TilePos, goal_tile: TilePos) -> Option<AbstractPath<EuclideanNeighborhood>> {
        self.path_cache
            .as_ref()
            .expect("Path cache should be initialized before calling get_path.")
            .find_path(
                (start_tile.x.try_into().unwrap(), start_tile.y.try_into().unwrap()),
                (goal_tile.x.try_into().unwrap(), goal_tile.y.try_into().unwrap()),
                cost_fn(self),
            )
    }

    #[allow(dead_code)]
    pub fn tile_xy_idx(&self, x: u32, y: u32) -> usize {
        (y * self.width + x).try_into().unwrap()
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
            (idx as u32 % self.width) as f32 * TILE_SIZE.x + TILE_SIZE.x * 0.5,
            (idx as u32 / self.width) as f32 * TILE_SIZE.y + TILE_SIZE.y * 0.5,
        )
    }

    pub fn tile_cost(&self, x: u32, y: u32) -> isize {
        if x >= self.width || y >= self.height {
            return -1;
        }
        self.tiles[self.tile_xy_idx(x, y)].cost()
    }

    pub fn is_passable(&self, x: u32, y: u32) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        !self.tiles[self.tile_xy_idx(x, y)].is_obstacle()
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
        x: x as f32 * TILE_SIZE.x + TILE_SIZE.x * 0.5,
        y: y as f32 * TILE_SIZE.y + TILE_SIZE.y * 0.5,
    }
}

#[allow(dead_code)]
pub fn is_neighbor(pos1: &TilePos, pos2: &TilePos) -> bool {
    pos1.x.abs_diff(pos2.x) <= 1 && pos1.y.abs_diff(pos2.y) <= 1
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

        assert_eq!(map.world_xy_idx(3200.0, 3200.0), map.world_xy_idx(3216.0, 3216.0));
        assert_eq!(map.world_xy_idx(3231.0, 3231.0), map.world_xy_idx(3216.0, 3216.0));

        let idx = map.world_xy_idx(3216.0, 3216.0);
        assert_eq!(idx, 100100);

        let coords = map.idx_world_xy(idx);
        assert_eq!(coords.x, 3216.0);
        assert_eq!(coords.y, 3216.0);
    }

    #[test]
    fn is_neighbor_test() {
        assert!(is_neighbor(&TilePos::new(2, 2), &TilePos::new(2, 2)));

        assert!(is_neighbor(&TilePos::new(2, 2), &TilePos::new(2, 3)));
        assert!(is_neighbor(&TilePos::new(2, 2), &TilePos::new(2, 1)));
        assert!(is_neighbor(&TilePos::new(2, 2), &TilePos::new(3, 2)));
        assert!(is_neighbor(&TilePos::new(2, 2), &TilePos::new(1, 2)));

        assert!(is_neighbor(&TilePos::new(2, 2), &TilePos::new(3, 3)));
        assert!(is_neighbor(&TilePos::new(2, 2), &TilePos::new(1, 1)));
        assert!(is_neighbor(&TilePos::new(2, 2), &TilePos::new(1, 3)));
        assert!(is_neighbor(&TilePos::new(2, 2), &TilePos::new(3, 1)));

        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(0, 0)));
        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(0, 1)));
        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(0, 2)));
        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(0, 3)));
        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(0, 4)));

        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(4, 0)));
        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(4, 1)));
        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(4, 2)));
        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(4, 3)));
        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(4, 4)));

        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(1, 0)));
        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(2, 0)));
        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(3, 0)));

        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(1, 4)));
        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(2, 4)));
        assert!(!is_neighbor(&TilePos::new(2, 2), &TilePos::new(3, 4)));
    }
}
