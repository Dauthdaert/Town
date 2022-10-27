use bevy::prelude::Entity;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_tileset::auto::AutoTileId;

pub struct RemoveAutoTileEvent {
    pub entity: Entity,
    pub pos: TilePos,
    pub auto_id: AutoTileId,
}
