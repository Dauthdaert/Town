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

    pub fn tile_xy_idx(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    #[allow(dead_code)]
    pub fn world_xy_idx(&self, x: f32, y: f32) -> usize {
        self.tile_xy_idx((x / TILE_SIZE.x).floor() as u32, (y / TILE_SIZE.y).floor() as u32)
    }
}
