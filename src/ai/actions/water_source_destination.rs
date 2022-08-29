use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;

use crate::map_gen::components::WaterSource;

use super::components::Destination;

#[derive(Component, Clone, Debug)]
pub struct WaterSourceDestination;

pub fn water_source_destination(
    mut commands: Commands,
    water_sources: Query<&TilePos, With<WaterSource>>,
    positions: Query<&Transform, Without<WaterSource>>,
    mut actions: Query<(&Actor, &mut ActionState, &WaterSourceDestination)>,
) {
    for (Actor(actor), mut action_state, _move_to) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let actor_transform = positions.get(*actor).expect("Actor has no position.");
                let destination = Some(super::shared_drinking::find_closest_water_source(
                    &water_sources,
                    actor_transform,
                ));
                if let Some(destination) = destination {
                    trace!("Setting water source destination.");
                    commands.entity(*actor).insert(Destination::new(destination, true));
                    *action_state = ActionState::Success;
                } else {
                    error!("Failed to find a water source destination.");
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
