use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

use crate::states::GameStates;

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };
pub const MAP_HEIGHT: u32 = 800;
pub const MAP_WIDTH: u32 = 800;

mod auto_tile;
mod biomes;
pub mod components;
mod display;
mod features;
mod generator;
mod layers;
pub mod neighborhood;
mod structs;

pub use biomes::Biomes;
pub use display::FeatureQuery;
pub use features::Features;
pub use layers::*;
pub use structs::*;

#[derive(AssetCollection)]
pub struct TilemapAssets {
    #[allow(dead_code)]
    #[asset(path = "tilesets/tiles.ron")]
    tiles: Handle<Tileset>,
    #[allow(dead_code)]
    #[asset(path = "tilesets/features.ron")]
    features: Handle<Tileset>,
}

impl TilemapAssets {}

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilesetPlugin::default())
            .add_plugin(auto_tile::AutoTilePlugin::<FeatureLayer>::default());

        app.add_plugin(ProgressPlugin::new(GameStates::MapGeneration).continue_to(GameStates::GameObjectSpawning))
            .add_enter_system(GameStates::MapGeneration, generator::start_generate_map)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameStates::MapGeneration)
                    .with_system(generator::handle_generate_map)
                    .into(),
            );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameStates::GameObjectSpawning)
                .with_system(display::spawn_tiles.track_progress())
                .with_system(display::spawn_features.track_progress())
                .into(),
        );
    }
}
