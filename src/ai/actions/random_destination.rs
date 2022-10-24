use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_turborand::{rng::Rng, DelegatedRng, RngComponent};
use big_brain::prelude::*;
use if_chain::if_chain;

use crate::map_gen::{
    components::Obstacle,
    map::{tile_xy_world_xy, Map},
    TileLayerObject, TILE_SIZE,
};

use super::components::Destination;

const MAX_DESTINATION_DISTANCE: f32 = 100.0 * TILE_SIZE.x;

#[derive(Debug)]
pub struct RandomDestinationBuilder;

impl ActionBuilder for RandomDestinationBuilder {
    fn build(&self, cmd: &mut Commands, action: Entity, _actor: Entity) {
        let rng = Rng::new();
        cmd.entity(action).insert(RandomDestination);
        cmd.entity(action).insert(RngComponent::from(&rng));
    }
}

#[derive(Component, Debug)]
pub struct RandomDestination;

pub fn random_destination(
    commands: ParallelCommands,
    tiles: Query<&TilePos, (With<TileLayerObject>, Without<Obstacle>)>,
    map: Res<Map>,
    query: Query<&Transform>,
    mut actions: Query<(&Actor, &mut ActionState, &RandomDestination, &mut RngComponent)>,
) {
    actions.par_for_each_mut(10, |(Actor(actor), mut action_state, _move_to, mut rng)| {
        match *action_state {
            ActionState::Requested => {
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let actor_position =
                    query.get(*actor).expect("Actor should have Transform.");
                let filtered: Vec<&TilePos> = tiles
                    .iter()
                    .filter(|t| {
                        let idx = map.tile_xy_idx(t.x, t.y);
                        if_chain! {
                            if let Some(feature) = map.features[idx];
                            if feature.is_obstacle();
                            then {
                                false
                            } else {
                                tile_xy_world_xy(t.x, t.y).distance(actor_position.translation.xy()) < MAX_DESTINATION_DISTANCE
                            }
                        }
                    })
                    .collect();
                let res = rng.usize(0..filtered.len());
                let destination = filtered.get(res);
                if let Some(destination) = destination {
                    // TODO!(3, Wayan, 2): Check that destination is possible.
                    commands.command_scope(|mut commands| {
                        commands.entity(*actor).insert(Destination::new(**destination, false));
                    });
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
    });
}
