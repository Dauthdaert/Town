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

                        if feature.is_destructable() {
                            feature_builder.insert(super::components::Destructable);
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

struct SpawnFeatureCommand {
    pos: TilePos,
    feature: super::Features,
}

impl bevy::ecs::system::Command for SpawnFeatureCommand {
    fn write(self, world: &mut World) {
        let feature_layer = world
            .query_filtered::<Entity, With<FeatureLayer>>()
            .get_single(world)
            .expect("Should only be one FeatureLayer.");

        let mut feature_builder = world.spawn();

        if self.feature.is_obstacle() {
            feature_builder.insert(super::components::Obstacle);
        }

        if self.feature.is_choppable() {
            feature_builder.insert(super::components::Choppable);
        }

        if self.feature.is_destructable() {
            feature_builder.insert(super::components::Destructable);
        }

        let feature_entity = feature_builder
            .insert_bundle((Name::from("Feature"), FeatureLayerObject))
            .insert_bundle(TileBundle {
                position: self.pos,
                tilemap_id: TilemapId(feature_layer),
                texture: TileTexture(self.feature.texture()),
                ..default()
            })
            .id();

        world
            .query_filtered::<&mut TileStorage, With<FeatureLayer>>()
            .get_single_mut(world)
            .expect("Should only be one FeatureLayer.")
            .set(&self.pos, feature_entity);
        world.entity_mut(feature_layer).push_children(&[feature_entity]);
    }
}

pub trait CommandsFeatureExt {
    fn spawn_feature(&mut self, feature_pos: TilePos, feature: super::Features) -> &mut Self;
}

impl CommandsFeatureExt for Commands<'_, '_> {
    fn spawn_feature(&mut self, feature_pos: TilePos, feature: super::Features) -> &mut Self {
        self.add(SpawnFeatureCommand {
            pos: feature_pos,
            feature,
        });
        self
    }
}
