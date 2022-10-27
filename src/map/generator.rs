use futures_lite::future;
use iyes_progress::ProgressCounter;

use super::{biomes::Biomes, features::Features, Map, MapPathfinding, MAP_HEIGHT, MAP_WIDTH};
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_turborand::{DelegatedRng, GlobalRng};
use noise::{Exponent, Fbm, NoiseFn, OpenSimplex, ScaleBias, ScalePoint};

#[derive(Component)]
pub struct GenerateMap(Task<(Map, MapPathfinding)>);

pub fn start_generate_map(mut commands: Commands, mut global_rng: ResMut<GlobalRng>) {
    let thread_pool = AsyncComputeTaskPool::get();
    let e_seed = global_rng.u32(u32::MIN..u32::MAX);
    let m_seed = global_rng.u32(u32::MIN..u32::MAX);

    let task = thread_pool.spawn(async move {
        let map = MapGenerator::new(MAP_WIDTH, MAP_HEIGHT, e_seed, m_seed)
            .generate_tiles()
            .generate_features()
            .build();
        let map_pathfinding = MapPathfinding::new(&map);
        (map, map_pathfinding)
    });
    commands.spawn().insert(GenerateMap(task));
}

pub fn handle_generate_map(
    mut commands: Commands,
    mut gen_map_tasks: Query<(Entity, &mut GenerateMap)>,
    progress: Res<ProgressCounter>,
) {
    let (gen_map_entity, mut gen_map_task) = gen_map_tasks.single_mut();
    if let Some((map, map_pathfinding)) = future::block_on(future::poll_once(&mut gen_map_task.0)) {
        commands.insert_resource(map);
        commands.insert_resource(map_pathfinding);
        commands.entity(gen_map_entity).despawn();

        progress.manually_track(true.into());
    } else {
        progress.manually_track(false.into());
    }
}

pub struct MapGenerator {
    elevation_seed: u32,
    moisture_seed: u32,
    map: Map,
}

impl MapGenerator {
    pub fn new(width: u32, height: u32, e_seed: u32, m_seed: u32) -> Self {
        MapGenerator {
            elevation_seed: e_seed,
            moisture_seed: m_seed,
            map: Map::new(height, width),
        }
    }

    pub fn generate_tiles(&mut self) -> &mut Self {
        let mut elevation_starter = Fbm::<OpenSimplex>::new(self.elevation_seed);
        elevation_starter.lacunarity = 2.0;
        let elevation_gen = Exponent::new(ScaleBias::new(elevation_starter).set_scale(1.2)).set_exponent(3.0);

        let moisture_gen = Fbm::<OpenSimplex>::new(self.moisture_seed);

        let x_min = -3.0;
        let x_max = 3.0;

        let y_min = -3.0;
        let y_max = 3.0;

        let x_step = (x_max - x_min) / self.map.width as f64;
        let y_step = (y_max - y_min) / self.map.height as f64;

        for x in 0..self.map.width {
            for y in 0..self.map.height {
                let nx = x_min + x_step * x as f64;
                let ny = y_min + y_step * y as f64;

                let e = scale(elevation_gen.get([nx, ny]));
                let m = scale(moisture_gen.get([nx, ny]));

                let tile = self.biome(e, m);
                let idx = self.map.tile_xy_idx(x, y);
                self.map.tiles[idx] = tile;
            }
        }
        self
    }

    pub fn generate_features(&mut self) -> &mut Self {
        let feature_gen = ScalePoint::new(Fbm::<OpenSimplex>::new(self.elevation_seed)).set_scale(5000.0);
        let mut blue_noise = vec![0.0; (self.map.width * self.map.height) as usize];

        let x_min = -5.0;
        let x_max = 5.0;

        let y_min = -5.0;
        let y_max = 5.0;

        let x_step = (x_max - x_min) / self.map.width as f64;
        let y_step = (y_max - y_min) / self.map.height as f64;

        for x in 0..self.map.width {
            for y in 0..self.map.height {
                let nx = x_min + x_step * x as f64;
                let ny = y_min + y_step * y as f64;

                blue_noise[self.map.tile_xy_idx(x, y)] = scale(feature_gen.get([nx, ny]));
            }
        }

        for x_c in 0..self.map.width as i32 {
            for y_c in 0..self.map.height as i32 {
                let idx = self.map.tile_xy_idx(x_c as u32, y_c as u32);
                let r = match self.map.tiles[idx] {
                    Biomes::Beach => 8,
                    Biomes::Scorched => 9,
                    Biomes::Stone => 0,
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
                    Biomes::Empty | Biomes::Ocean => {
                        continue;
                    }
                };
                let mut max = 0.0;
                for x_n in (x_c - r)..=(x_c + r) {
                    if x_n >= 0 && x_n < self.map.width as i32 {
                        for y_n in (y_c - r)..=(y_c + r) {
                            if y_n >= 0 && y_n < self.map.height as i32 {
                                let e = blue_noise[self.map.tile_xy_idx(x_n as u32, y_n as u32)];
                                max = f64::max(e, max);
                            }
                        }
                    }
                }

                if blue_noise[idx] == max {
                    // TODO!(3, Wayan, 0): Replace checks of max with rng to pick objects.
                    // TODO!(3, Wayan, 0): Replace continues with biome specific features.
                    self.map.features[idx] = match self.map.tiles[idx] {
                        Biomes::Beach => Some(Features::CoconutTree),
                        Biomes::Scorched => continue,
                        Biomes::Stone => Some(Features::StoneWall),
                        Biomes::Snow => continue,
                        Biomes::Taiga | Biomes::Tundra => continue,
                        Biomes::TemperateDesert | Biomes::SubtropicalDesert => Some(Features::Cactus),
                        Biomes::Shrubland | Biomes::Grassland => {
                            if max > 0.8 {
                                Some(Features::BerryBush)
                            } else {
                                Some(Features::Rocks)
                            }
                        }
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
                        Biomes::Empty | Biomes::Ocean => continue,
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

        if e > 0.6 {
            return Biomes::Stone;
        }

        if e > 0.5 {
            if m < 0.33 {
                return Biomes::TemperateDesert;
            }

            return Biomes::Shrubland;
        }

        if e > 0.15 {
            if m < 0.26 {
                return Biomes::TemperateDesert;
            }
            if m < 0.65 {
                return Biomes::Grassland;
            }
            if m < 0.93 {
                return Biomes::TemperateDeciduousForest;
            }

            return Biomes::TemperateRainForest;
        }

        if e > 0.035 {
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

        if e > 0.02 {
            return Biomes::Beach;
        }

        Biomes::Ocean
    }
}

fn scale(initial: f64) -> f64 {
    (initial + 1.0) / 2.0
}
