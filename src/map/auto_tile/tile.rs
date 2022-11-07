use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_tileset::{
    auto::{AutoTile, AutoTileId},
    tileset::coords::TileCoords,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileCoord(pub TilePos);

impl TileCoords for TileCoord {
    fn pos(&self) -> IVec2 {
        let pos: UVec2 = self.0.into();
        pos.as_ivec2()
    }
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum AutoTileCategory {
    None,
    Wall,
}

#[derive(Clone, Copy)]
pub struct TileInfo {
    pub pos: TileCoord,
    pub entity: Entity,
    pub auto_tile: AutoTileId,
    pub category: AutoTileCategory,
}

impl TileInfo {
    pub fn new(entity: Entity, pos: &TilePos, auto_tile: &AutoTileId, category: &AutoTileCategory) -> Self {
        Self {
            pos: TileCoord(*pos),
            entity,
            auto_tile: *auto_tile,
            category: *category,
        }
    }
}

impl AutoTile for TileInfo {
    type Coords = TileCoord;

    fn coords(&self) -> Self::Coords {
        self.pos
    }

    fn auto_id(&self) -> AutoTileId {
        self.auto_tile
    }

    fn can_match(&self, other: &Self) -> bool {
        self.auto_tile == other.auto_tile
            || (self.category != AutoTileCategory::None && self.category == other.category)
    }
}
