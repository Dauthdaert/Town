use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use big_brain::prelude::*;

use crate::map_gen::{
    components::{Choppable, Growing},
    map::{world_xy_tile_xy, Map},
    FeatureLayer, Features,
};

use super::components::HasJob;

#[derive(Component, Clone, Copy, Debug)]
pub struct DoJob;

pub fn do_job(
    mut commands: Commands,
    mut map: ResMut<Map>,
    time: Res<Time>,
    mut actors: Query<(&Transform, &mut HasJob)>,
    features_query: Query<&TileStorage, With<FeatureLayer>>,
    mut trees: Query<(Entity, &mut TileTexture), With<Choppable>>,
    mut actions: Query<(&Actor, &mut ActionState, &DoJob)>,
) {
    for (Actor(actor), mut action_state, _do_job) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let (actor_transform, mut actor_job) =
                    actors.get_mut(*actor).expect("Actor should have a position and job.");
                let actor_tile = world_xy_tile_xy(actor_transform.translation.xy());
                if crate::map_gen::map::is_neighbor(&actor_tile, &actor_job.job.position) {
                    actor_job.progress += actor_job.job.job_type.speed() * time.delta_seconds();

                    if actor_job.progress >= 100.0 {
                        let feature = features_query
                            .get_single()
                            .expect("Should only be one feature layer.")
                            .get(&actor_job.job.position);
                        if let Some(feature) = feature {
                            match actor_job.job.job_type {
                                crate::jobs::Jobs::Chop => {
                                    //TODO: Have better job duplication logic in an earlier step.
                                    if let Ok(tree) = trees.get_mut(feature) {
                                        do_chop(&actor_job.job.position, &mut map, tree, &mut commands);
                                    }
                                }
                            }
                            commands.entity(*actor).remove::<HasJob>();
                            *action_state = ActionState::Success;
                        }
                    }
                } else {
                    //We're too far away.
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

fn do_chop(feature_pos: &TilePos, map: &mut Map, tree: (Entity, Mut<TileTexture>), commands: &mut Commands) {
    let idx = map.tile_xy_idx(feature_pos.x, feature_pos.y);
    let (entity, mut entity_texture) = tree;

    let current_feature = map.features[idx];
    let next_feature = match current_feature {
        Some(Features::Tree) => Some(Features::Stump),
        Some(Features::CoconutTree) => Some(Features::CoconutStump),
        Some(_) => None,
        None => None,
    };

    map.features[idx] = next_feature;
    if let Some(next_feature) = next_feature {
        entity_texture.0 = next_feature.texture();
        commands.entity(entity).remove::<Choppable>().insert(Growing::new());
    } else {
        commands.entity(entity).despawn_recursive();
    }
}
