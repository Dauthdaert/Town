use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use iyes_progress::Progress;

use super::{map::Map, FeatureLayer, FeatureLayerObject, Layer, TileLayer, TileLayerObject, TilemapAssets, TILE_SIZE};

pub fn spawn_tiles(
    mut commands: Commands,
    tilemap_assets: Res<TilemapAssets>,
    array_texture_loader: Res<ArrayTextureLoader>,
    map: Res<Map>,
    mut done: Local<bool>,
) -> Progress {
    if !*done {
        let tilemap_size = TilemapSize {
            x: map.width,
            y: map.height,
        };
        let mut tile_storage = TileStorage::empty(tilemap_size);

        commands
            .spawn()
            .insert_bundle((Name::from("Tile Layer"), TileLayer))
            .with_children(|parent| {
                for (idx, tile_biome) in map.tiles.iter().enumerate() {
                    let tile_pos = map.idx_tile_xy(idx);

                    let mut tile_builder = parent.spawn_bundle(TileBundle {
                        position: tile_pos,
                        tilemap_id: TilemapId(parent.parent_entity()),
                        texture: TileTexture(tile_biome.texture()),
                        ..default()
                    });

                    if tile_biome.is_water_source() {
                        tile_builder.insert(super::components::WaterSource);
                    }

                    if tile_biome.is_obstacle() {
                        tile_builder.insert(super::components::Obstacle);
                    }

                    let tile_entity = tile_builder.insert_bundle((Name::from("Tile"), TileLayerObject)).id();
                    tile_storage.set(&tile_pos, tile_entity);
                }
            })
            .insert_bundle(TilemapBundle {
                grid_size: TILE_SIZE.into(),
                size: tilemap_size,
                storage: tile_storage,
                texture: TilemapTexture::Single(tilemap_assets.tiles.clone()),
                tile_size: TILE_SIZE,
                transform: Transform::from_translation(Vec3::splat(TileLayer::z_index())),
                ..default()
            });

        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(tilemap_assets.tiles.clone()),
            tile_size: TILE_SIZE,
            ..default()
        });
        *done = true;
    }

    true.into()
}

pub fn spawn_features(
    mut commands: Commands,
    tilemap_assets: Res<TilemapAssets>,
    array_texture_loader: Res<ArrayTextureLoader>,
    map: Res<Map>,
    mut done: Local<bool>,
) -> Progress {
    if !*done {
        let feature_map_size = TilemapSize {
            x: map.width,
            y: map.height,
        };
        let mut feature_storage = TileStorage::empty(feature_map_size);

        commands
            .spawn()
            .insert_bundle((Name::from("Feature Layer"), FeatureLayer))
            .with_children(|parent| {
                for (idx, feature) in map.features.iter().enumerate() {
                    if let Some(feature) = feature {
                        let feature_pos = map.idx_tile_xy(idx);
                        let mut feature_builder = parent.spawn_bundle(TileBundle {
                            position: feature_pos,
                            tilemap_id: TilemapId(parent.parent_entity()),
                            texture: TileTexture(feature.texture()),
                            ..default()
                        });

                        if feature.is_obstacle() {
                            feature_builder.insert(super::components::Obstacle);
                        }

                        if feature.is_choppable() {
                            feature_builder.insert(super::components::Choppable);
                        }

                        if feature.is_minable() {
                            feature_builder.insert(super::components::Minable);
                        }

                        let feature_entity = feature_builder
                            .insert_bundle((Name::from("Feature"), FeatureLayerObject))
                            .id();
                        feature_storage.set(&feature_pos, feature_entity);
                    }
                }
            })
            .insert_bundle(TilemapBundle {
                grid_size: TILE_SIZE.into(),
                size: feature_map_size,
                storage: feature_storage,
                texture: TilemapTexture::Single(tilemap_assets.features.clone()),
                tile_size: TILE_SIZE,
                transform: Transform::from_translation(Vec3::splat(FeatureLayer::z_index())),
                ..default()
            });

        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(tilemap_assets.features.clone()),
            tile_size: TILE_SIZE,
            ..default()
        });
        *done = true;
    }

    true.into()
}
