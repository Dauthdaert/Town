use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{map::Map, TilemapAssets, TILE_SIZE};

pub fn spawn_tiles(mut commands: Commands, tilemap_assets: Res<TilemapAssets>, map: Res<Map>) {
    let tilemap_size = TilemapSize {
        x: map.width,
        y: map.height,
    };
    let mut tile_storage = TileStorage::empty(tilemap_size);

    commands
        .spawn()
        .insert(Name::from("Tilemap"))
        .with_children(|parent| {
            for (idx, tile_biome) in map.tiles.iter().enumerate() {
                let tile_pos = map.idx_tile_xy(idx);

                let mut tile_builder = parent.spawn_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(parent.parent_entity()),
                    texture: TileTexture(tile_biome.texture()),
                    ..Default::default()
                });

                if tile_biome.is_water_source() {
                    tile_builder.insert(super::components::WaterSource);
                }

                if tile_biome.is_obstacle() {
                    tile_builder.insert(super::components::Obstacle);
                }

                let tile_entity = tile_builder.insert(Name::from("Tile")).id();
                tile_storage.set(&tile_pos, Some(tile_entity));
            }
        })
        .insert_bundle(TilemapBundle {
            grid_size: TILE_SIZE.into(),
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(tilemap_assets.tiles.clone()),
            tile_size: TILE_SIZE,
            transform: Transform::from_translation(Vec3::splat(0.0)),
            ..Default::default()
        });
}
