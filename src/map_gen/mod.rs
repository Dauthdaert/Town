use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };

mod chunks;
mod generator;

#[derive(AssetCollection)]
pub struct TilemapAssets {
    #[asset(path = "tiles.png")]
    tiles: Handle<Image>,
}

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<TilemapAssets>()
            .insert_resource(chunks::ChunkManager::default())
            .add_startup_system(setup_generator)
            .add_system(chunks::spawn_chunks_around_camera)
            .add_system(chunks::despawn_chunks_outside_camera);
    }
}

fn setup_generator(mut commands: Commands, mut global_rng: ResMut<GlobalRng>) {
    commands.insert_resource(generator::MapGenerator::new(
        global_rng.u32(u32::MIN..u32::MAX),
        global_rng.u32(u32::MIN..u32::MAX),
    ));
}
