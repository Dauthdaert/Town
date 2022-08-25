use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

use crate::states::GameStates;

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
pub const MAP_HEIGHT: u32 = 1000;
pub const MAP_WIDTH: u32 = 1000;

mod biomes;
mod chunks;
mod generator;
pub mod map;

#[derive(AssetCollection)]
pub struct TilemapAssets {
    #[asset(path = "tiles.png")]
    tiles: Handle<Image>,
}

impl TilemapAssets {}

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(chunks::ChunkManager::default());

        app.add_plugin(ProgressPlugin::new(GameStates::MapGeneration).continue_to(GameStates::InGame))
            .add_enter_system(GameStates::MapGeneration, generator::start_generate_map)
            .add_system(generator::handle_generate_map.run_in_state(GameStates::MapGeneration));

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameStates::InGame)
                .with_system(chunks::spawn_chunks_around_camera)
                .with_system(chunks::despawn_chunks_outside_camera)
                .into(),
        );
    }
}
