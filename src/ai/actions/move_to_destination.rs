use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;

use crate::{
    ai::characteristics::Speed,
    map_gen::map::{is_neighbor, tile_xy_world_xy, world_xy_tile_xy, Map},
};

use super::{components::Destination, shared_pathfinding};

#[derive(Component, Clone, Debug, Default)]
pub struct MoveToDestination {
    pub path: Option<Vec<TilePos>>,
    pub next: Option<TilePos>,
}

pub fn move_to_destination(
    time: Res<Time>,
    map: Res<Map>,
    mut query: Query<(&mut Transform, &Destination, &Speed)>,
    mut actions: Query<(&Actor, &mut ActionState, &mut MoveToDestination)>,
) {
    for (Actor(actor), mut action_state, mut move_to) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                let (actor_transform, actor_destination, _actor_speed) =
                    query.get(*actor).expect("Actor has no position or destination.");

                let actor_tile = world_xy_tile_xy(actor_transform.translation.xy());
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
            ActionState::Executing => {
                let (mut actor_transform, actor_destination, actor_speed) =
                    query.get_mut(*actor).expect("Actor has no position or destination.");

                if let Some(next) = move_to
                    .next
                    .or_else(|| move_to.path.as_mut().expect("Actor has no path.").pop())
                {
                    if map.is_passable(next.x, next.y) {
                        let next_pos = tile_xy_world_xy(next.x, next.y);
                        let actor_pos = actor_transform.translation.xy();
                        actor_transform.translation +=
                            calculate_step(actor_pos, next_pos, actor_speed.speed, time.delta_seconds()).extend(0.0);

                        if actor_transform.translation.xy() != next_pos {
                            move_to.next = Some(next);
                        } else {
                            move_to.next = None;
                        }
                    } else {
                        let actor_tile = world_xy_tile_xy(actor_transform.translation.xy());
                        warn!(
                            "Next node in path was impassable. Going from {:?} to {:?}.",
                            actor_tile, next
                        );
                        *action_state = ActionState::Failure;
                    }
                } else {
                    let actor_tile = world_xy_tile_xy(actor_transform.translation.xy());
                    if is_neighbor(&actor_tile, &actor_destination.destination) {
                        *action_state = ActionState::Success;
                    } else {
                        warn!(
                            "Path contains no next node. Going from {:?} to {:?}.",
                            actor_tile, actor_destination.destination
                        );
                        *action_state = ActionState::Failure;
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
