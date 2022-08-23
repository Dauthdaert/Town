use noise::{NoiseFn, OpenSimplex};

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
}

impl Biomes {
    pub fn texture(self) -> u8 {
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
        }
    }
}

pub struct MapGenerator {
    elevation_gen: OpenSimplex,
    moisture_gen: OpenSimplex,
}

impl MapGenerator {
    pub fn new(e_seed: u32, m_seed: u32) -> Self {
        MapGenerator {
            elevation_gen: OpenSimplex::new(e_seed),
            moisture_gen: OpenSimplex::new(m_seed),
        }
    }

    pub fn generate(&self, x: i32, y: i32) -> Biomes {
        let nx = x as f64 / 100.0 - 0.5;
        let ny = y as f64 / 100.0 - 0.5;

        let mut e = 1.00 * self.noise_e(1.0 * nx, 1.0 * ny)
            + 0.50 * self.noise_e(2.0 * nx + 2.1, 2.0 * ny + 1.5)
            + 0.25 * self.noise_e(4.0 * nx + 6.4, 4.0 * ny + 5.9)
            + 0.13 * self.noise_e(8.0 * nx + 17.6, 8.0 * ny + 25.3)
            + 0.06 * self.noise_e(16.0 * nx + 42.4, 16.0 * ny + 51.6)
            + 0.03 * self.noise_e(32.0 * nx + 98.2, 32.0 * ny + 105.2);

        e /= 1.00 + 0.50 + 0.25 + 0.13 + 0.06 + 0.03;
        e = f64::powi(e * 1.2, 3);

        let mut m = 1.00 * self.noise_m(1.0 * nx, 1.0 * ny)
            + 0.75 * self.noise_m(2.0 * nx + 1.4, 2.0 * ny + 3.2)
            + 0.33 * self.noise_m(4.0 * nx + 5.8, 4.0 * ny + 4.2)
            + 0.33 * self.noise_m(8.0 * nx + 17.6, 8.0 * ny + 29.1)
            + 0.33 * self.noise_m(16.0 * nx + 38.9, 16.0 * ny + 36.8)
            + 0.50 * self.noise_m(32.0 * nx + 114.5, 32.0 * ny + 85.2);
        m /= 1.00 + 0.75 + 0.33 + 0.33 + 0.33 + 0.50;

        self.biome(e, m)
    }

    fn biome(&self, e: f64, m: f64) -> Biomes {
        // these thresholds will need tuning to match your generator
        if e < 0.1 {
            return Biomes::Ocean;
        }
        if e < 0.12 {
            return Biomes::Beach;
        }

        if e > 0.8 {
            if m < 0.1 {
                return Biomes::Scorched;
            }
            if m < 0.2 {
                return Biomes::Bare;
            }
            if m < 0.5 {
                return Biomes::Tundra;
            }
            return Biomes::Snow;
        }

        if e > 0.6 {
            if m < 0.33 {
                return Biomes::TemperateDesert;
            }
            if m < 0.66 {
                return Biomes::Shrubland;
            }
            return Biomes::Taiga;
        }

        if e > 0.3 {
            if m < 0.16 {
                return Biomes::TemperateDesert;
            }
            if m < 0.50 {
                return Biomes::Grassland;
            }
            if m < 0.83 {
                return Biomes::TemperateDeciduousForest;
            }
            return Biomes::TemperateRainForest;
        }

        if m < 0.16 {
            return Biomes::SubtropicalDesert;
        }
        if m < 0.33 {
            return Biomes::Grassland;
        }
        if m < 0.66 {
            return Biomes::TropicalSeasonalForest;
        }
        Biomes::TropicalRainForest
    }

    fn noise_e(&self, n_x: f64, n_y: f64) -> f64 {
        self.elevation_gen.get([n_x, n_y]) / 2.0 + 0.5
    }

    fn noise_m(&self, n_x: f64, n_y: f64) -> f64 {
        self.moisture_gen.get([n_x, n_y]) / 2.0 + 0.5
    }
}
