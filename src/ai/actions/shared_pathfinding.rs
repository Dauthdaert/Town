use bevy_ecs_tilemap::tiles::TilePos;

use crate::map_gen::map::Map;

const HEURISTIC_FACTOR: f32 = 2.0;

/// Returns optimal computed path using astar.
/// Next node is at the end.
pub fn get_path_passable(start: &TilePos, map: &Map, destination: &TilePos, approximate: bool) -> Option<Vec<TilePos>> {
    pathfinding::prelude::astar(
        start,
        |p| map.get_passable_neighbors(p.x, p.y),
        |p| {
            let d_x = p.x.abs_diff(destination.x) as f32;
            let d_y = p.y.abs_diff(destination.y) as f32;
            ((d_x + d_y + (1.45 - 2.0) * f32::min(d_x, d_y)) * 100.0 * HEURISTIC_FACTOR) as u32
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
