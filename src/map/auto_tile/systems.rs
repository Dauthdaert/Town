use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::{auto::*, prelude::*};

use crate::map::Layer;

use super::{
    events::RemoveAutoTileEvent,
    tile::{AutoTileCategory, TileCoord, TileInfo},
};

pub trait TileQuery {
    fn find_tile(&self, entity: Entity) -> Option<TileInfo>;
    fn count(&self) -> usize;
}

impl<'w, 's> TileQuery for Query<'w, 's, (Entity, &TilePos, &AutoTileId, &AutoTileCategory)> {
    fn find_tile(&self, entity: Entity) -> Option<TileInfo> {
        if let Ok((entity, pos, auto_tile, category)) = self.get(entity) {
            Some(TileInfo::new(entity, pos, auto_tile, category))
        } else {
            None
        }
    }

    fn count(&self) -> usize {
        self.iter().count()
    }
}

struct TilemapCache<'a> {
    pub tile_storage: &'a TileStorage,
    pub tile_query: &'a dyn TileQuery,
}

impl<'a> AutoTilemap for TilemapCache<'a> {
    type Tile = TileInfo;

    fn make_coords(
        &self,
        pos: IVec2,
        _template: &<Self::Tile as AutoTile>::Coords,
    ) -> <Self::Tile as AutoTile>::Coords {
        TileCoord(pos.as_uvec2().into())
    }

    fn get_tile_at(&self, coords: &<Self::Tile as AutoTile>::Coords) -> Option<Self::Tile> {
        let entity = self.tile_storage.checked_get(&coords.0);
        if let Some(entity) = entity {
            self.tile_query.find_tile(entity)
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.tile_query.count()
    }
}

#[allow(clippy::type_complexity)]
pub fn on_change_auto_tile<T: Layer + Component>(
    mut commands: Commands,
    changed_tiles: Query<
        (Entity, &TilePos, &AutoTileId, &AutoTileCategory),
        Or<(Changed<AutoTileId>, Changed<AutoTileCategory>)>,
    >,
    all_tiles: Query<(Entity, &TilePos, &AutoTileId, &AutoTileCategory)>,
    mut working_tiles: Query<(&mut TileTextureIndex, &AutoTileId, Option<&mut AnimatedTile>), With<TilePos>>,
    tilesets: Tilesets,
    tile_storage: Query<&TileStorage, With<T>>,
) {
    if changed_tiles.is_empty() {
        return;
    }

    let tile_storage = tile_storage.single();
    let mut cache = TilemapCache {
        tile_storage,
        tile_query: &all_tiles,
    };
    let mut tiler = AutoTiler::new(&mut cache);
    for (entity, pos, auto_tile, category) in changed_tiles.iter() {
        tiler.add_tile(TileInfo::new(entity, pos, auto_tile, category), true);
    }

    let requests = tiler.finish();

    apply_requests(&requests, &tilesets, &mut working_tiles, &mut commands);
}

pub fn on_remove_auto_tile<T: Layer + Component>(
    mut commands: Commands,
    mut events: EventReader<RemoveAutoTileEvent>,
    all_tiles: Query<(Entity, &TilePos, &AutoTileId, &AutoTileCategory)>,
    mut working_tiles: Query<(&mut TileTextureIndex, &AutoTileId, Option<&mut AnimatedTile>), With<TilePos>>,
    tilesets: Tilesets,
    tile_storage: Query<&TileStorage, With<T>>,
) {
    if events.is_empty() {
        return;
    }

    let tile_storage = tile_storage.single();
    let mut cache = TilemapCache {
        tile_storage,
        tile_query: &all_tiles,
    };
    let mut tiler = AutoTiler::new(&mut cache);

    for ref event in events.iter() {
        let RemoveAutoTileEvent {
            entity,
            pos,
            auto_id,
            category,
        } = event;
        tiler.add_tile(TileInfo::new(*entity, pos, auto_id, category), true);
    }

    let requests = tiler.finish();

    apply_requests(&requests, &tilesets, &mut working_tiles, &mut commands);
}

fn apply_requests(
    requests: &[AutoTileRequest<TileInfo>],
    tilesets: &Tilesets,
    query: &mut Query<(&mut TileTextureIndex, &AutoTileId, Option<&mut AnimatedTile>), With<TilePos>>,
    commands: &mut Commands,
) {
    for request in requests.iter() {
        let rule = request.rule;
        let TileInfo { entity, .. } = request.tile;
        if let Ok((ref mut tile_texture, auto_tile, ref mut anim)) = query.get_mut(entity) {
            if let Some(tileset) = tilesets.get_by_id(&auto_tile.tileset_id) {
                if let Some(tile_name) = tileset.get_tile_name(&auto_tile.group_id) {
                    //Check if variant.
                    let texture_index = tile_texture.0 as usize;
                    if tileset.is_auto_variant(tile_name, &texture_index, &rule) {
                        //The request index is just a variant of the correct state, so we skip.
                        continue;
                    }

                    //Apply rule
                    if let Some(index) = tileset.get_auto_index(tile_name, rule) {
                        match index {
                            TileIndex::Standard(index) => {
                                tile_texture.0 = index as u32;

                                if anim.is_some() {
                                    commands.entity(entity).remove::<AnimatedTile>();
                                }
                            }
                            TileIndex::Animated(start, end, speed) => {
                                tile_texture.0 = start as u32;

                                if let Some(anim) = anim {
                                    anim.start = start as u32;
                                    anim.end = end as u32;
                                    anim.speed = speed;
                                } else {
                                    commands.entity(entity).insert(AnimatedTile {
                                        start: start as u32,
                                        end: end as u32,
                                        speed,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
