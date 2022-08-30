use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;
use hierarchical_pathfinding::internals::AbstractPath;

use crate::{
    ai::characteristics::Speed,
    map_gen::{
        map::{is_neighbor, tile_xy_world_xy, world_xy_tile_xy, Map},
        neighborhood::EuclideanNeighborhood,
    },
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
    mut query: Query<(&mut Transform, &Destination, &Speed)>,
    mut actions: Query<(&Actor, &mut ActionState, &mut MoveToDestination)>,
) {
    for (Actor(actor), mut action_state, mut move_to) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                let (actor_transform, actor_destination, _actor_speed) =
                    query.get(*actor).expect("Actor has no position or destination.");

                let actor_tile = world_xy_tile_xy(actor_transform.translation.xy());
                if let Some(path) = map.get_path(actor_tile, actor_destination.destination) {
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

                if let Some(next) = move_to.next.or_else(|| {
                    move_to
                        .path
                        .as_mut()
                        .expect("Actor has no path.")
                        .next()
                        .map(|(x, y)| TilePos::new(x as u32, y as u32))
                }) {
                    if map.is_passable(next.x, next.y) {
                        let next_pos = tile_xy_world_xy(next.x, next.y);
                        let actor_pos = actor_transform.translation.xy();
                        actor_transform.translation += calculate_step(
                            actor_pos,
                            next_pos,
                            actor_speed.speed,
                            map.tile_cost(next.x, next.y),
                            time.delta_seconds(),
                        )
                        .extend(0.0);

                        if actor_transform.translation.xy() != next_pos {
                            move_to.next = Some(next);
                        } else {
                            move_to.next = None;
                        }
                    } else {
                        let actor_tile = world_xy_tile_xy(actor_transform.translation.xy());
                        if is_neighbor(&actor_tile, &actor_destination.destination) {
                            //No problem, we've already arrived.
                            *action_state = ActionState::Success;
                        } else {
                            warn!(
                                "Next node in path was impassable. Going from {:?} to {:?}.",
                                actor_tile, next
                            );
                            *action_state = ActionState::Failure;
                        }
                    }
                } else {
                    let actor_tile = world_xy_tile_xy(actor_transform.translation.xy());
                    if is_neighbor(&actor_tile, &actor_destination.destination) {
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
