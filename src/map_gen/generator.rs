use futures_lite::future;
use iyes_progress::ProgressCounter;

use super::{biomes::Biomes, map::Map, MAP_HEIGHT, MAP_WIDTH};
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
        let mut map = super::map::Map::new(MAP_HEIGHT, MAP_WIDTH);

        let generator = MapGenerator::new(e_seed, m_seed);
        for x in 0..map.width {
            for y in 0..map.height {
                let idx = map.xy_idx(x, y);
                map.tiles[idx] = generator.generate(x as i32, y as i32);
            }
        }
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
}

impl MapGenerator {
    pub fn new(e_seed: u32, m_seed: u32) -> Self {
        MapGenerator {
            elevation_gen: OpenSimplex::new(e_seed),
            moisture_gen: OpenSimplex::new(m_seed),
        }
    }

    pub fn generate(&self, x: i32, y: i32) -> Biomes {
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
