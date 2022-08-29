use bevy_ecs_tilemap::tiles::TilePos;

use crate::map_gen::map::{tile_xy_world_xy, Map};

const HEURISTIC_FACTOR: f32 = 2.5;

/// Returns optimal computed path using astar.
/// Next node is at the end.
pub fn get_path_passable(start: &TilePos, map: &Map, destination: &TilePos, approximate: bool) -> Option<Vec<TilePos>> {
    pathfinding::prelude::astar(
        start,
        |p| map.get_passable_neighbors(p.x, p.y),
        |p| {
            (tile_xy_world_xy(destination.x, destination.y).distance(tile_xy_world_xy(p.x, p.y)) * HEURISTIC_FACTOR)
                .floor() as u32
        },
        |p| {
            if approximate {
                map.is_neighbor(p, destination)
            } else {
                p == destination
            }
        },
    )
    .map(|(mut path, _total_cost)| {
        path.reverse();
        path
    })
}
