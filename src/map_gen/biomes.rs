#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Biomes {
    Ocean,
    Beach,
    Scorched,
    Bare,
    Snow,
    Taiga,
    Tundra,
    TemperateDesert,
    Shrubland,
    Grassland,
    TemperateDeciduousForest,
    TemperateRainForest,
    SubtropicalDesert,
    TropicalSeasonalForest,
    TropicalRainForest,
    None,
}

impl Biomes {
    pub fn texture(&self) -> u32 {
        match self {
            Biomes::Shrubland => 0,
            Biomes::Grassland => 0,
            Biomes::Ocean => 1,
            Biomes::TemperateDeciduousForest => 2,
            Biomes::TemperateRainForest => 2,
            Biomes::TropicalSeasonalForest => 2,
            Biomes::TropicalRainForest => 2,
            Biomes::Bare => 3,
            Biomes::Snow => 5,
            Biomes::Taiga => 5,
            Biomes::Tundra => 5,
            Biomes::Beach => 6,
            Biomes::TemperateDesert => 6,
            Biomes::SubtropicalDesert => 6,
            Biomes::Scorched => 7,
            Biomes::None => 255,
        }
    }

    pub fn cost(&self) -> isize {
        match self {
            Biomes::Ocean => -1,
            Biomes::Beach => 140,
            Biomes::Scorched => 140,
            Biomes::Bare => 100,
            Biomes::Snow => 170,
            Biomes::Taiga => 100,
            Biomes::Tundra => 140,
            Biomes::TemperateDesert => 170,
            Biomes::SubtropicalDesert => 170,
            Biomes::Shrubland => 100,
            Biomes::Grassland => 100,
            Biomes::TemperateDeciduousForest => 150,
            Biomes::TemperateRainForest => 150,
            Biomes::TropicalSeasonalForest => 150,
            Biomes::TropicalRainForest => 150,
            Biomes::None => -1,
        }
    }

    #[allow(dead_code)]
    pub fn is_obstacle(&self) -> bool {
        self.cost() == -1
    }

    pub fn is_water_source(&self) -> bool {
        matches!(self, Biomes::Ocean)
    }
}
