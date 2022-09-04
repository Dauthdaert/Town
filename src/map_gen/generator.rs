use futures_lite::future;
use iyes_progress::ProgressCounter;

use super::{biomes::Biomes, features::Features, map::Map, MAP_HEIGHT, MAP_WIDTH};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_turborand::{DelegatedRng, GlobalRng};
use noise::{NoiseFn, OpenSimplex};

#[derive(Component)]
pub struct GenerateMap(Task<Map>);

pub fn start_generate_map(mut commands: Commands, mut global_rng: ResMut<GlobalRng>) {
    let thread_pool = AsyncComputeTaskPool::get();
    let e_seed = global_rng.u32(u32::MIN..u32::MAX);
    let m_seed = global_rng.u32(u32::MIN..u32::MAX);

    let task = thread_pool.spawn(async move {
        let mut map = MapGenerator::new(MAP_WIDTH, MAP_HEIGHT, e_seed, m_seed)
            .generate_tiles()
            .generate_features()
            .build();
        map.init_path_cache();
        map
    });
    commands.spawn().insert(GenerateMap(task));
}

pub fn handle_generate_map(
    mut commands: Commands,
    mut gen_map_tasks: Query<(Entity, &mut GenerateMap)>,
    progress: Res<ProgressCounter>,
) {
    let (gen_map_entity, mut gen_map_task) = gen_map_tasks.single_mut();
    if let Some(map) = future::block_on(future::poll_once(&mut gen_map_task.0)) {
        commands.insert_resource(map);
        commands.entity(gen_map_entity).despawn();

        progress.manually_track(true.into());
    } else {
        progress.manually_track(false.into());
    }
}

pub struct MapGenerator {
    elevation_gen: OpenSimplex,
    moisture_gen: OpenSimplex,
    map: Map,
}

impl MapGenerator {
    pub fn new(width: u32, height: u32, e_seed: u32, m_seed: u32) -> Self {
        MapGenerator {
            elevation_gen: OpenSimplex::new(e_seed),
            moisture_gen: OpenSimplex::new(m_seed),
            map: super::map::Map::new(height, width),
        }
    }

    pub fn generate_tiles(&mut self) -> &mut Self {
        for x in 0..self.map.width {
            for y in 0..self.map.height {
                let nx = x as f64 / 400. - 0.5;
                let ny = y as f64 / 400. - 0.5;

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

                let tile = self.biome(e, m);
                let idx = self.map.tile_xy_idx(x, y);
                self.map.tiles[idx] = tile;
            }
        }
        self
    }

    pub fn generate_features(&mut self) -> &mut Self {
        let mut blue_noise = vec![0.0; (self.map.width * self.map.height) as usize];
        for x in 0..self.map.width {
            for y in 0..self.map.height {
                let nx = x as f64 / self.map.width as f64 - 0.5;
                let ny = y as f64 / self.map.height as f64 - 0.5;
                blue_noise[self.map.tile_xy_idx(x, y)] = self.noise_e(5000.0 * nx, 5000.0 * ny);
            }
        }

        for x_c in 0..self.map.width as i32 {
            for y_c in 0..self.map.height as i32 {
                let idx = self.map.tile_xy_idx(x_c as u32, y_c as u32);
                let r = match self.map.tiles[idx] {
                    Biomes::Beach => 8,
                    Biomes::Scorched => 9,
                    Biomes::Bare => 10,
                    Biomes::Snow => 9,
                    Biomes::Taiga => 6,
                    Biomes::Tundra => 7,
                    Biomes::TemperateDesert => 8,
                    Biomes::Shrubland => 5,
                    Biomes::Grassland => 6,
                    Biomes::TemperateDeciduousForest => 1,
                    Biomes::TemperateRainForest => 1,
                    Biomes::SubtropicalDesert => 8,
                    Biomes::TropicalSeasonalForest => 1,
                    Biomes::TropicalRainForest => 1,
                    Biomes::None | Biomes::Ocean => {
                        continue;
                    }
                };
                let mut max = 0.0;
                for x_n in (x_c - r)..=(x_c + r) {
                    for y_n in (y_c - r)..=(y_c + r) {
                        if y_n >= 0 && y_n < self.map.height as i32 && x_n >= 0 && x_n < self.map.width as i32 {
                            let e = blue_noise[self.map.tile_xy_idx(x_n as u32, y_n as u32)];
                            max = f64::max(e, max);
                        }
                    }
                }

                if blue_noise[idx] == max {
                    //TODO: Replace checks of max with rng to pick objects.
                    //TODO: Replace continues with biome specific features.
                    self.map.features[idx] = match self.map.tiles[idx] {
                        Biomes::Beach => Some(Features::CoconutTree),
                        Biomes::Scorched => continue,
                        Biomes::Bare => Some(Features::Rocks),
                        Biomes::Snow => continue,
                        Biomes::Taiga | Biomes::Tundra => continue,
                        Biomes::TemperateDesert | Biomes::SubtropicalDesert => Some(Features::Cactus),
                        Biomes::Shrubland | Biomes::Grassland => Some(Features::Bush),
                        Biomes::TemperateDeciduousForest
                        | Biomes::TemperateRainForest
                        | Biomes::TropicalRainForest
                        | Biomes::TropicalSeasonalForest => {
                            if max > 0.60 {
                                Some(Features::Tree)
                            } else {
                                Some(Features::AppleTree)
                            }
                        }
                        Biomes::None | Biomes::Ocean => continue,
                    };
                }
            }
        }

        self
    }

    pub fn build(&self) -> Map {
        self.map.clone()
    }

    fn biome(&self, e: f64, m: f64) -> Biomes {
        // these thresholds will need tuning to match your generator

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
            if m < 0.60 {
                return Biomes::Grassland;
            }
            if m < 0.93 {
                return Biomes::TemperateDeciduousForest;
            }
            return Biomes::TemperateRainForest;
        }

        if e > 0.15 {
            if m < 0.16 {
                return Biomes::SubtropicalDesert;
            }
            if m < 0.53 {
                return Biomes::Grassland;
            }
            if m < 0.76 {
                return Biomes::TropicalSeasonalForest;
            }

            return Biomes::TropicalRainForest;
        }

        if e > 0.13 {
            return Biomes::Beach;
        }

        Biomes::Ocean
    }

    fn noise_e(&self, n_x: f64, n_y: f64) -> f64 {
        self.elevation_gen.get([n_x, n_y]) / 2.0 + 0.5
    }

    fn noise_m(&self, n_x: f64, n_y: f64) -> f64 {
        self.moisture_gen.get([n_x, n_y]) / 2.0 + 0.5
    }
}
