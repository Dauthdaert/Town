use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_turborand::{DelegatedRng, RngComponent};
use big_brain::prelude::*;

use crate::map_gen::components::Obstacle;

use super::components::Destination;

#[derive(Component, Clone, Debug)]
pub struct RandomDestination;

pub fn random_destination(
    mut commands: Commands,
    tiles: Query<&TilePos, Without<Obstacle>>,
    mut query: Query<&mut RngComponent>,
    mut actions: Query<(&Actor, &mut ActionState, &RandomDestination)>,
) {
    for (Actor(actor), mut action_state, _move_to) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let res = query
                    .get_mut(*actor)
                    .expect("Actor has no RNG")
                    .usize(0..tiles.iter().count());
                let destination = tiles.iter().nth(res);
                if let Some(destination) = destination {
                    //TODO: Check that destination is possible.
                    commands.entity(*actor).insert(Destination::new(*destination, false));
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
