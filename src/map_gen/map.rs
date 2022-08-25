use super::biomes::Biomes;

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

    pub fn xy_idx(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }
}
