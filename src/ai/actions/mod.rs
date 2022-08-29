mod components;
pub mod drink;
pub mod move_to_destination;
pub mod random_destination;
mod shared_drinking;
mod shared_pathfinding;
pub mod water_source_destination;

pub const MAX_ACTION_DISTANCE: f32 = 0.3;
pub const MAX_SIMPLE_PATH_DISTANCE: f32 = crate::map_gen::TILE_SIZE.x * 1.7;
