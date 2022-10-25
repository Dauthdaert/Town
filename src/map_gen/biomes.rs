#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Biomes {
    Ocean,
    Beach,
    Scorched,
    Stone,
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
    Empty,
}

impl Biomes {
    pub fn tile_name(&self) -> &str {
        match self {
            Biomes::Ocean => "Ocean",
            Biomes::Beach | Biomes::TemperateDesert | Biomes::SubtropicalDesert => "Desert",
            Biomes::Scorched => "Scorched",
            Biomes::Stone => "Stone",
            Biomes::Snow => "Snow",
            Biomes::Taiga => "Taiga",
            Biomes::Tundra => "Tundra",
            Biomes::Shrubland => "Shrubland",
            Biomes::Grassland => "Grassland",
            Biomes::TemperateDeciduousForest
            | Biomes::TemperateRainForest
            | Biomes::TropicalSeasonalForest
            | Biomes::TropicalRainForest => "Forest",
            Biomes::Empty => "Empty",
        }
    }

    pub fn cost(&self) -> isize {
        match self {
            Biomes::Ocean => -1,
            Biomes::Beach => 140,
            Biomes::Scorched => 140,
            Biomes::Stone => 100,
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
            Biomes::Empty => -1,
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
