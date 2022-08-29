use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Destination {
    pub destination: TilePos,
    pub approximate: bool,
}

impl Destination {
    pub fn new(destination: TilePos, approximate: bool) -> Self {
        Self {
            destination,
            approximate,
        }
    }
}
