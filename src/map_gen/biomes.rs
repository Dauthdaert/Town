#[derive(Debug, Clone, Copy)]
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

    pub fn distance(&self) -> u32 {
        match self {
            Biomes::Ocean => 100,
            Biomes::Beach => 100,
            Biomes::Scorched => 100,
            Biomes::Bare => 100,
            Biomes::Snow => 100,
            Biomes::Taiga => 100,
            Biomes::Tundra => 100,
            Biomes::TemperateDesert => 100,
            Biomes::Shrubland => 100,
            Biomes::Grassland => 100,
            Biomes::TemperateDeciduousForest => 100,
            Biomes::TemperateRainForest => 100,
            Biomes::SubtropicalDesert => 100,
            Biomes::TropicalSeasonalForest => 100,
            Biomes::TropicalRainForest => 100,
            Biomes::None => u32::MAX,
        }
    }

    #[allow(dead_code)]
    pub fn is_obstacle(&self) -> bool {
        matches!(self, Biomes::Ocean | Biomes::None)
    }

    pub fn is_water_source(&self) -> bool {
        matches!(self, Biomes::Ocean)
    }
}
