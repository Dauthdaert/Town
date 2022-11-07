use bevy::prelude::Entity;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_tileset::auto::AutoTileId;

use super::tile::AutoTileCategory;

pub struct RemoveAutoTileEvent {
    pub entity: Entity,
    pub pos: TilePos,
    pub auto_id: AutoTileId,
    pub category: AutoTileCategory,
}
