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
        //TODO: Way to use alternate sprites
        match self {
            Biomes::Shrubland => 6,
            Biomes::Grassland => 7,
            Biomes::Ocean => 1,
            Biomes::TemperateDeciduousForest => 12,
            Biomes::TemperateRainForest => 12,
            Biomes::TropicalSeasonalForest => 12,
            Biomes::TropicalRainForest => 12,
            Biomes::Bare => 24,
            Biomes::Snow => 11,
            Biomes::Taiga => 9,
            Biomes::Tundra => 10,
            Biomes::Beach => 18,
            Biomes::TemperateDesert => 18,
            Biomes::SubtropicalDesert => 18,
            Biomes::Scorched => 8,
            Biomes::None => 0,
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
