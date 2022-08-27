use std::ops::Sub;

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;

use crate::map_gen::{components::WaterSource, TILE_SIZE};

/// A utility function that finds the closest water source to the actor.
pub fn find_closest_water_source(waters: &Query<&TilePos, With<WaterSource>>, actor_position: &Transform) -> Vec2 {
    waters
        .iter()
        .map(|a| Vec2::new(a.x as f32 * TILE_SIZE.x, a.y as f32 * TILE_SIZE.y))
        .min_by(|a, b| {
            let da = (a.sub(actor_position.translation.xy())).length_squared();
            let db = (b.sub(actor_position.translation.xy())).length_squared();
            da.partial_cmp(&db).unwrap()
        })
        .expect("no water sources")
}
