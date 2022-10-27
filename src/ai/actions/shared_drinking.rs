use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;

use crate::map::{components::WaterSource, tile_xy_world_xy};

/// A utility function that finds the closest water source to the actor.
pub fn find_closest_water_source(waters: &Query<&TilePos, With<WaterSource>>, actor_position: &Transform) -> TilePos {
    *waters
        .iter()
        .min_by(|a, b| {
            let da = tile_xy_world_xy(a.x, a.y).distance_squared(actor_position.translation.xy());
            let db = tile_xy_world_xy(b.x, b.y).distance_squared(actor_position.translation.xy());
            da.partial_cmp(&db).unwrap()
        })
        .expect("no water sources")
}
