use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;
use hierarchical_pathfinding::{internals::AbstractPath, prelude::Neighborhood};

use crate::{
    ai::characteristics::Speed,
    map::{is_neighbor, neighborhood::EuclideanNeighborhood, tile_xy_world_xy, world_xy_tile_xy, Map, MapPathfinding},
};

use super::components::Destination;

#[derive(Component, Clone, Debug, Default)]
pub struct MoveToDestination {
    pub path: Option<AbstractPath<EuclideanNeighborhood>>,
    pub next: Option<TilePos>,
}

pub fn move_to_destination(
    time: Res<Time>,
    map: Res<Map>,
    map_pathfiding: Res<MapPathfinding>,
    mut query: Query<(&mut Transform, &Destination, &Speed)>,
    mut actions: Query<(&Actor, &mut ActionState, &mut MoveToDestination)>,
) {
    for (Actor(actor), mut action_state, mut move_to) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                let (actor_transform, actor_destination, _actor_speed) =
                    query.get(*actor).expect("Actor has no position or destination.");

                let actor_tile = world_xy_tile_xy(actor_transform.translation.xy());
                let mut actor_tiles = vec![(actor_tile.x as usize, actor_tile.y as usize)];
                map.neighborhood.get_all_neighbors(
                    (actor_tile.x.try_into().unwrap(), actor_tile.y.try_into().unwrap()),
                    &mut actor_tiles,
                );

                let path = if let Some(path) = actor_tiles.iter().find_map(|start| {
                    if !map.is_passable(start.0 as u32, start.1 as u32)
                        || !map.is_passable(actor_destination.destination.x, actor_destination.destination.y)
                    {
                        return None;
                    }

                    map_pathfiding.get_path(
                        &map,
                        TilePos::new(start.0 as u32, start.1 as u32),
                        actor_destination.destination,
                    )
                }) {
                    Some(path)
                } else if actor_destination.approximate {
                    let mut destination_tiles = vec![(
                        actor_destination.destination.x as usize,
                        actor_destination.destination.y as usize,
                    )];
                    map.neighborhood.get_all_neighbors(
                        (
                            actor_destination.destination.x.try_into().unwrap(),
                            actor_destination.destination.y.try_into().unwrap(),
                        ),
                        &mut destination_tiles,
                    );

                    actor_tiles.iter().find_map(|start| {
                        if !map.is_passable(start.0 as u32, start.1 as u32) {
                            return None;
                        }

                        destination_tiles.iter().find_map(|end| {
                            if !map.is_passable(end.0 as u32, end.1 as u32) {
                                return None;
                            }

                            map_pathfiding.get_path(
                                &map,
                                TilePos::new(start.0.try_into().unwrap(), start.1.try_into().unwrap()),
                                TilePos::new(end.0.try_into().unwrap(), end.1.try_into().unwrap()),
                            )
                        })
                    })
                } else {
                    None
                };

                if let Some(path) = path {
                    move_to.path = Some(path);
                    *action_state = ActionState::Executing;
                } else {
                    error!(
                        "Failed to get a path going from {:?} to {:?}.",
                        actor_tile, actor_destination.destination
                    );
                    *action_state = ActionState::Failure;
                }
            }
            ActionState::Executing => {
                let (mut actor_transform, actor_destination, actor_speed) =
                    query.get_mut(*actor).expect("Actor has no position or destination.");

                let actor_tile = world_xy_tile_xy(actor_transform.translation.xy());
                if let Some(next_tile) = move_to.next.or_else(|| {
                    move_to
                        .path
                        .as_mut()
                        .expect("Actor has no path.")
                        .next()
                        .map(|(x, y)| TilePos::new(x.try_into().unwrap(), y.try_into().unwrap()))
                }) {
                    if map.is_passable(next_tile.x, next_tile.y) {
                        let next_pos = tile_xy_world_xy(next_tile.x, next_tile.y);
                        let actor_pos = actor_transform.translation.xy();
                        actor_transform.translation += calculate_step(
                            actor_pos,
                            next_pos,
                            actor_speed.speed,
                            map.tile_cost(actor_tile.x, actor_tile.y),
                            time.delta_seconds(),
                        )
                        .extend(0.0);

                        if actor_transform.translation.xy() != next_pos {
                            move_to.next = Some(next_tile);
                        } else {
                            move_to.next = None;
                        }
                    } else if is_neighbor(&actor_tile, &actor_destination.destination) {
                        //No problem, we've already arrived.
                        *action_state = ActionState::Success;
                    } else {
                        warn!(
                            "Next node in path was impassable. Going from {:?} to {:?}.",
                            actor_tile, next_tile
                        );
                        *action_state = ActionState::Failure;
                    }
                } else if is_neighbor(&actor_tile, &actor_destination.destination) {
                    //No problem, we've already arrived.
                    *action_state = ActionState::Success;
                } else {
                    warn!(
                        "Path contains no next node. Going from {:?} to {:?}.",
                        actor_tile, actor_destination.destination
                    );
                    *action_state = ActionState::Failure;
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

fn calculate_step(start_pos: Vec2, end_pos: Vec2, speed: f32, tile_cost: isize, dt: f32) -> Vec2 {
    let delta = end_pos - start_pos;
    let distance = delta.length() * (tile_cost as f32 / 100.0);
    let step_size = dt * speed;
    if distance > step_size {
        delta / distance * step_size
    } else {
        delta
    }
}
