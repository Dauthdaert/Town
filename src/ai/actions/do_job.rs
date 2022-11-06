use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use big_brain::prelude::*;

use crate::map::{
    components::{Choppable, Growing, Mineable},
    is_neighbor, world_xy_tile_xy, FeatureQuery, Features, Map, MapPathfinding,
};

use super::components::HasJob;

#[derive(Component, Clone, Copy, Debug)]
pub struct DoJob;

pub fn do_job(
    mut commands: Commands,
    mut feature_query: FeatureQuery,
    mut map: ResMut<Map>,
    mut map_pathfinding: ResMut<MapPathfinding>,
    time: Res<Time>,
    mut actors: Query<(&Transform, &mut HasJob)>,
    chop_query: Query<Entity, With<Choppable>>,
    mine_query: Query<Entity, With<Mineable>>,
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
                if is_neighbor(&actor_tile, &actor_job.job.position) {
                    actor_job.progress += actor_job.job.job_type.speed() * time.delta_seconds();

                    if actor_job.progress >= 100.0 {
                        match actor_job.job.job_type {
                            crate::jobs::Jobs::Chop => {
                                // TODO!(3, Wayan, 0): Have better job duplication logic in an earlier step.
                                let feature = feature_query.get_feature(&actor_job.job.position);
                                if let Some(feature_layer) = feature {
                                    if let Ok(chop_target) = chop_query.get(feature_layer) {
                                        do_chop(
                                            &actor_job.job.position,
                                            &mut map,
                                            chop_target,
                                            &mut commands,
                                            &mut feature_query,
                                        );
                                    }
                                }
                            }
                            crate::jobs::Jobs::Mine => {
                                // TODO!(3, Wayan, 0): Have better job duplication logic in an earlier step.
                                let feature = feature_query.get_feature(&actor_job.job.position);
                                if let Some(feature_layer) = feature {
                                    if mine_query.get(feature_layer).is_ok() {
                                        do_mine(
                                            &actor_job.job.position,
                                            &mut map,
                                            &mut map_pathfinding,
                                            &mut feature_query,
                                        );
                                    }
                                }
                            }
                            crate::jobs::Jobs::Build(feature) => {
                                do_build(
                                    &actor_job.job.position,
                                    feature,
                                    &mut map,
                                    &mut map_pathfinding,
                                    &mut feature_query,
                                );
                            }
                            crate::jobs::Jobs::Clear => do_clear(
                                &actor_job.job.position,
                                &mut map,
                                &mut map_pathfinding,
                                &mut feature_query,
                            ),
                        }
                        commands.entity(*actor).remove::<HasJob>();
                        *action_state = ActionState::Success;
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

fn do_chop(
    chop_target_pos: &TilePos,
    map: &mut Map,
    chop_target: Entity,
    commands: &mut Commands,
    feature_query: &mut FeatureQuery,
) {
    let idx = map.tile_xy_idx(chop_target_pos.x, chop_target_pos.y);

    let current_feature = map.features[idx];
    let next_feature = match current_feature {
        Some(Features::Tree) => Some(Features::TreeStump),
        Some(Features::CoconutTree) => Some(Features::CoconutTreeStump),
        Some(_) => None,
        None => None,
    };

    map.features[idx] = next_feature;
    if let Some(next_feature) = next_feature {
        feature_query.change_feature_tile(chop_target, next_feature);
        commands
            .entity(chop_target)
            .remove::<Choppable>()
            .insert(Growing::new());
    } else {
        commands.entity(chop_target).despawn_recursive();
    }
}

fn do_mine(
    mine_target_pos: &TilePos,
    map: &mut Map,
    map_pathfinding: &mut MapPathfinding,
    feature_query: &mut FeatureQuery,
) {
    let idx = map.tile_xy_idx(mine_target_pos.x, mine_target_pos.y);
    map.features[idx] = None;
    map_pathfinding.announce_tile_changed(map, mine_target_pos);
    feature_query.despawn_feature(*mine_target_pos);
}

fn do_build(
    build_pos: &TilePos,
    feature: Features,
    map: &mut Map,
    map_pathfinding: &mut MapPathfinding,
    feature_query: &mut FeatureQuery,
) {
    let idx = map.tile_xy_idx(build_pos.x, build_pos.y);
    map.features[idx] = Some(feature);
    map_pathfinding.announce_tile_changed(map, build_pos);
    feature_query.spawn_feature(*build_pos, feature);
}

fn do_clear(
    build_pos: &TilePos,
    map: &mut Map,
    map_pathfinding: &mut MapPathfinding,
    feature_query: &mut FeatureQuery,
) {
    let idx = map.tile_xy_idx(build_pos.x, build_pos.y);
    map.features[idx] = None;
    map_pathfinding.announce_tile_changed(map, build_pos);
    feature_query.despawn_feature(*build_pos);
}
