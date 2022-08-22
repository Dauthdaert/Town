use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 16.0, y: 16.0 };

mod chunks;

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
            .add_system(chunks::spawn_chunks_around_camera)
            .add_system(chunks::despawn_chunks_outside_camera);
    }
}
