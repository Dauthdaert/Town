use bevy::{
    ecs::system::{EntityCommands, SystemParam},
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::prelude::*;
use iyes_progress::Progress;

use super::{map::Map, FeatureLayer, FeatureLayerObject, Features, Layer, TileLayer, TileLayerObject, TILE_SIZE};

pub fn spawn_tiles(
    mut commands: Commands,
    tilesets: Tilesets,
    array_texture_loader: Res<ArrayTextureLoader>,
    map: Res<Map>,
    tile_layer_query: Query<Entity, With<TileLayer>>,
) -> Progress {
    if tile_layer_query.is_empty() {
        let tilemap_size = TilemapSize {
            x: map.width,
            y: map.height,
        };
        let mut tile_storage = TileStorage::empty(tilemap_size);

        let tileset = tilesets.get_by_name("Tiles").expect("Tiles tileset should be loaded.");

        commands
            .spawn()
            .insert_bundle((Name::from("Tile Layer"), TileLayer))
            .with_children(|parent| {
                for (idx, tile_biome) in map.tiles.iter().enumerate() {
                    let tile_pos = map.idx_tile_xy(idx);
                    let tilemap_id = TilemapId(parent.parent_entity());

                    let mut tile_builder = parent.spawn();

                    let (tile_index, ..) = tileset
                        .select_tile(tile_biome.tile_name())
                        .unwrap_or_else(|| panic!("Tile {} should exist.", tile_biome.tile_name()));
                    let texture = match tile_index {
                        TileIndex::Standard(index) => TileTexture(index as u32),
                        TileIndex::Animated(start, end, speed) => {
                            tile_builder.insert(AnimatedTile {
                                start: start as u32,
                                end: end as u32,
                                speed,
                            });

                            TileTexture(start as u32)
                        }
                    };

                    tile_builder.insert_bundle(TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture,
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
                texture: TilemapTexture::Single(tileset.texture().clone()),
                tile_size: TILE_SIZE,
                transform: Transform::from_translation(Vec3::splat(TileLayer::z_index())),
                ..default()
            });

        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(tileset.texture().clone()),
            tile_size: TILE_SIZE,
            ..default()
        });
    }

    true.into()
}

pub fn spawn_features(
    mut commands: Commands,
    tilesets: Tilesets,
    array_texture_loader: Res<ArrayTextureLoader>,
    map: Res<Map>,
    feature_layer_query: Query<Entity, With<FeatureLayer>>,
) -> Progress {
    if feature_layer_query.is_empty() {
        let feature_map_size = TilemapSize {
            x: map.width,
            y: map.height,
        };
        let mut feature_storage = TileStorage::empty(feature_map_size);
        let tileset = tilesets
            .get_by_name("Features")
            .expect("Features tileset should be loaded.");

        commands
            .spawn()
            .insert_bundle((Name::from("Feature Layer"), FeatureLayer))
            .with_children(|child_builder| {
                for (idx, feature) in map.features.iter().enumerate() {
                    if let Some(feature) = feature {
                        let feature_pos = map.idx_tile_xy(idx);
                        let parent = child_builder.parent_entity();
                        let tile_index = get_feature_tile(tileset, feature);
                        let feature_entity =
                            fill_feature(&mut child_builder.spawn(), parent, tile_index, feature, feature_pos);
                        feature_storage.set(&feature_pos, feature_entity);
                    }
                }
            })
            .insert_bundle(TilemapBundle {
                grid_size: TILE_SIZE.into(),
                size: feature_map_size,
                storage: feature_storage,
                texture: TilemapTexture::Single(tileset.texture().clone()),
                tile_size: TILE_SIZE,
                transform: Transform::from_translation(Vec3::splat(FeatureLayer::z_index())),
                ..default()
            });

        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(tileset.texture().clone()),
            tile_size: TILE_SIZE,
            ..default()
        });
    }

    true.into()
}

#[derive(SystemParam)]
pub struct FeatureQuery<'w, 's> {
    commands: Commands<'w, 's>,
    parent_query: Query<'w, 's, Entity, With<FeatureLayer>>,
    feature_query: Query<'w, 's, (Entity, &'static mut TileTexture), With<FeatureLayerObject>>,
    feature_storage: Query<'w, 's, &'static mut TileStorage, With<FeatureLayer>>,
    tilesets: Tilesets<'w, 's>,
}

impl<'w, 's> FeatureQuery<'w, 's> {
    pub fn spawn_feature(&mut self, feature_pos: TilePos, feature: Features) {
        let parent = self.parent_query.get_single().expect("FeatureLayer should exist.");
        let tileset = self.get_tileset();
        let tile_index = get_feature_tile(tileset, &feature);

        let mut feature_builder = self.commands.spawn();
        let feature_entity = fill_feature(&mut feature_builder, parent, tile_index, &feature, feature_pos);
        self.feature_storage.single_mut().set(&feature_pos, feature_entity);
    }

    pub fn change_feature_tile(&mut self, feature: Entity, new_feature: Features) {
        let tileset = self.get_tileset();
        let tile_index = get_feature_tile(tileset, &new_feature);

        let (_, mut entity_texture) = self
            .feature_query
            .get_mut(feature)
            .expect("Feature entity should exist.");
        entity_texture.0 = match tile_index {
            TileIndex::Standard(index) => index as u32,
            TileIndex::Animated(..) => {
                panic!("Should not switch to an animated feature.")
            }
        };
    }

    pub fn get_feature(&self, feature_pos: &TilePos) -> Option<Entity> {
        self.feature_storage
            .get_single()
            .expect("FeatureLayer should exist.")
            .get(feature_pos)
    }

    fn get_tileset(&self) -> &Tileset {
        self.tilesets
            .get_by_name("Features")
            .expect("Features tileset should be loaded.")
    }
}

fn get_feature_tile(tileset: &Tileset, feature: &Features) -> TileIndex {
    let (tile_index, ..) = tileset
        .select_tile(feature.tile_name())
        .unwrap_or_else(|| panic!("Feature {} should exist.", feature.tile_name()));
    tile_index
}

fn fill_feature(
    feature_builder: &mut EntityCommands,
    parent: Entity,
    tile_index: TileIndex,
    feature: &Features,
    feature_pos: TilePos,
) -> Entity {
    let texture = match tile_index {
        TileIndex::Standard(index) => TileTexture(index as u32),
        TileIndex::Animated(start, end, speed) => {
            feature_builder.insert(AnimatedTile {
                start: start as u32,
                end: end as u32,
                speed,
            });

            TileTexture(start as u32)
        }
    };

    feature_builder.insert_bundle(TileBundle {
        position: feature_pos,
        tilemap_id: TilemapId(parent),
        texture,
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

    feature_builder
        .insert_bundle((Name::from("Feature"), FeatureLayerObject))
        .id()
}
