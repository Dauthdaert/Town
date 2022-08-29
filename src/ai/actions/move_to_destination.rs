use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;
use if_chain::if_chain;

use crate::map_gen::map::{tile_xy_world_xy, world_xy_tile_xy, Map};

use super::{components::Destination, shared_pathfinding};

#[derive(Component, Clone, Debug)]
pub struct MoveToDestination {
    pub speed: f32,
    pub path: Option<Vec<TilePos>>,
    pub next: Option<TilePos>,
}

impl MoveToDestination {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            path: None,
            next: None,
        }
    }
}

pub fn move_to_destination(
    time: Res<Time>,
    map: Res<Map>,
    mut query: Query<(&mut Transform, &Destination)>,
    mut actions: Query<(&Actor, &mut ActionState, &mut MoveToDestination)>,
) {
    for (Actor(actor), mut action_state, mut move_to) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                let (actor_transform, actor_destination) =
                    query.get(*actor).expect("Actor has no position or destination.");

                let destination_pos =
                    tile_xy_world_xy(actor_destination.destination.x, actor_destination.destination.y);
                let distance = destination_pos.distance(actor_transform.translation.xy());
                if distance <= super::MAX_ACTION_DISTANCE {
                    trace!("Already here");
                    *action_state = ActionState::Success;
                } else {
                    let actor_tile = world_xy_tile_xy(actor_transform.translation.xy());
                    trace!("Going from {:?} to {:?}", actor_tile, actor_destination);
                    if let Some(mut path) = shared_pathfinding::get_path_passable(
                        &actor_tile,
                        &map,
                        &actor_destination.destination,
                        actor_destination.approximate,
                    ) {
                        path.pop();
                        move_to.path = Some(path);
                        *action_state = ActionState::Executing;
                    } else {
                        error!("Failed to get a path.");
                        *action_state = ActionState::Failure;
                    }
                }
            }
            ActionState::Executing => {
                let (mut actor_transform, actor_destination) =
                    query.get_mut(*actor).expect("Actor has no position or destination.");

                let destination_pos =
                    tile_xy_world_xy(actor_destination.destination.x, actor_destination.destination.y);
                let distance = destination_pos.distance(actor_transform.translation.xy());

                if distance <= super::MAX_ACTION_DISTANCE {
                    trace!("Has arrived");
                    *action_state = ActionState::Success;
                } else if distance <= super::MAX_SIMPLE_PATH_DISTANCE {
                    let actor_pos = actor_transform.translation.xy();
                    actor_transform.translation +=
                        calculate_step(actor_pos, destination_pos, move_to.speed, time.delta_seconds()).extend(0.0);
                } else {
                    let mut next = move_to.next;
                    if next.is_none() {
                        if let Some(path) = &mut move_to.path {
                            next = path.pop();
                        }
                    }

                    if_chain! {
                        if let Some(next) = next;
                        if map.is_passable(next.x, next.y);
                        then {
                            let next_pos = tile_xy_world_xy(next.x, next.y);
                            let actor_pos = actor_transform.translation.xy();
                            actor_transform.translation += calculate_step(actor_pos, next_pos, move_to.speed, time.delta_seconds()).extend(0.0);

                            if actor_transform.translation.xy() != next_pos {
                                move_to.next = Some(next);
                            } else {
                                move_to.next = None;
                            }
                        } else {
                            warn!("Next node in path was impassable.");
                            *action_state = ActionState::Failure;
                        }
                    }
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}

fn calculate_step(start_pos: Vec2, end_pos: Vec2, speed: f32, dt: f32) -> Vec2 {
    let delta = end_pos - start_pos;
    let distance = delta.length();
    let step_size = dt * speed;
    if distance > step_size {
        delta / distance * step_size
    } else {
        delta
    }
}
