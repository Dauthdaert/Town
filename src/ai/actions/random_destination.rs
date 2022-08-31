use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_turborand::{DelegatedRng, RngComponent};
use big_brain::prelude::*;

use crate::map_gen::{components::Obstacle, map::tile_xy_world_xy};

use super::components::Destination;

#[derive(Component, Clone, Debug)]
pub struct RandomDestination;

pub fn random_destination(
    mut commands: Commands,
    tiles: Query<&TilePos, Without<Obstacle>>,
    mut query: Query<(&Transform, &mut RngComponent)>,
    mut actions: Query<(&Actor, &mut ActionState, &RandomDestination)>,
) {
    for (Actor(actor), mut action_state, _move_to) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let (actor_position, mut actor_rng) =
                    query.get_mut(*actor).expect("Actor should have RNG and Transform.");
                let filtered: Vec<&TilePos> = tiles
                    .iter()
                    .filter(|t| tile_xy_world_xy(t.x, t.y).distance(actor_position.translation.xy()) < 3000.0)
                    .collect();
                let res = actor_rng.usize(0..filtered.len());
                let destination = filtered.get(res);
                if let Some(destination) = destination {
                    //TODO: Check that destination is possible.
                    commands.entity(*actor).insert(Destination::new(**destination, false));
                    *action_state = ActionState::Success;
                } else {
                    error!("Failed to get a random destination.");
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
