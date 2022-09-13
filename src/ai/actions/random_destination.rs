use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_turborand::{DelegatedRng, RngComponent};
use big_brain::prelude::*;
use if_chain::if_chain;

use crate::map_gen::{
    components::Obstacle,
    map::{tile_xy_world_xy, Map},
    TileLayerObject, TILE_SIZE,
};

use super::components::Destination;

const MAX_DESTINATION_DISTANCE: f32 = 100.0 * TILE_SIZE.x;

#[derive(Component, Clone, Debug)]
pub struct RandomDestination;

pub fn random_destination(
    mut commands: Commands,
    tiles: Query<&TilePos, (With<TileLayerObject>, Without<Obstacle>)>,
    map: Res<Map>,
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
