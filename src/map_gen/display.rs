use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use super::{map::Map, TilemapAssets, TILE_SIZE};

pub fn spawn_tiles(mut commands: Commands, tilemap_assets: Res<TilemapAssets>, map: Res<Map>) {
    let tilemap_entity = commands.spawn().id();
    let tilemap_size = TilemapSize {
        x: map.width,
        y: map.height,
    };
    let mut tile_storage = TileStorage::empty(tilemap_size);

    for x in 0..map.width {
        for y in 0..map.height {
            let tile_biome = map.tiles[map.tile_xy_idx(x, y)];
            let tile_pos = TilePos { x, y };

            let mut tile_builder = commands.spawn_bundle(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture: TileTexture(tile_biome.texture()),
                ..Default::default()
            });

            if tile_biome.is_water_source() {
                tile_builder.insert(super::components::WaterSource);
            }

            let tile_entity = tile_builder.id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    commands.entity(tilemap_entity).insert_bundle(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture(tilemap_assets.tiles.clone()),
        tile_size: TILE_SIZE,
        transform: Transform::from_translation(Vec3::splat(0.0)),
        ..Default::default()
    });
}
