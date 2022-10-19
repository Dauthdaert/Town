use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use bevy_mouse_tracking_plugin::MousePosWorld;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    jobs::{job_queue::Job, Jobs},
    map_gen::{components::Choppable, map::world_xy_tile_xy, FeatureLayer},
};

use super::{job_queue::*, JobCreationControls, JobCreationMenuManager, JobSelectionType};

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct SelectionStart(TilePos);

pub fn handle_job_creation_hotkeys(
    mut commands: Commands,
    job_type: Res<JobSelectionType>,
    selection: Option<Res<SelectionStart>>,
    mut job_queue: ResMut<JobQueue>,
    query: Query<&ActionState<JobCreationControls>, With<JobCreationMenuManager>>,
    mouse_pos: Res<MousePosWorld>,
    feature_tiles_query: Query<&TileStorage, With<FeatureLayer>>,
    choppable_tiles_query: Query<Entity, With<Choppable>>,
) {
    let job_creation_menu = query.single();

    if job_creation_menu.just_pressed(JobCreationControls::Select) {
        let world_tile = world_xy_tile_xy(mouse_pos.xy());
        if let Some(selection) = selection {
            // TODO!(3, Wayan, 0): Take the opportunity to add a ring around the tree to indicate the job maybe?
            let feature_tile_storage = feature_tiles_query.single();
            for x in u32::min(selection.x, world_tile.x)..=u32::max(selection.x, world_tile.x) {
                for y in u32::min(selection.y, world_tile.y)..=u32::max(selection.y, world_tile.y) {
                    match job_type.0 {
                        Jobs::Chop => {
                            let entity = feature_tile_storage.get(&TilePos::new(x, y));
                            if let Some(entity) = entity {
                                if choppable_tiles_query.contains(entity) {
                                    job_queue.jobs.push_back(Job::new(Jobs::Chop, TilePos::new(x, y)));
                                }
                            }
                        }
                        Jobs::Build(feature) => {
                            let entity = feature_tile_storage.get(&TilePos::new(x, y));
                            if entity.is_none() {
                                job_queue
                                    .jobs
                                    .push_back(Job::new(Jobs::Build(feature), TilePos::new(x, y)));
                            }
                        }
                    }
                }
            }
            commands.remove_resource::<SelectionStart>();
        } else {
            commands.insert_resource(SelectionStart(world_tile));
        }
    }
}
