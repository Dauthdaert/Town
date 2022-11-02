use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::tiles::{TilePos, TileStorage};
use bevy_mouse_tracking_plugin::MousePosWorld;
use iyes_loopless::state::NextState;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    jobs::{job_queue::Job, Jobs},
    map::{components::Choppable, world_xy_tile_xy, FeatureLayer, Features},
};

use super::{job_queue::*, JobCreation, JobCreationControls, JobCreationMenuManager, JobSelectionType};

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
            let feature_tile_storage = feature_tiles_query.single();
            match job_type.0 {
                // TODO!(3, Wayan, 0): Add a visual indicator of chopping job. (Ring around tree?)
                JobCreation::Chop => {
                    for x in u32::min(selection.x, world_tile.x)..=u32::max(selection.x, world_tile.x) {
                        for y in u32::min(selection.y, world_tile.y)..=u32::max(selection.y, world_tile.y) {
                            let tile_pos = TilePos::new(x, y);
                            let entity = feature_tile_storage.get(&tile_pos);
                            if let Some(entity) = entity {
                                if choppable_tiles_query.contains(entity) {
                                    job_queue.jobs.push_back(Job::new(Jobs::Chop, tile_pos));
                                }
                            }
                        }
                    }
                }
                // TODO!(3, Wayan, 0): Add a visual indicator of bulding job. (Phantom tile?)
                JobCreation::Build(feature) => {
                    for x in u32::min(selection.x, world_tile.x)..=u32::max(selection.x, world_tile.x) {
                        for y in u32::min(selection.y, world_tile.y)..=u32::max(selection.y, world_tile.y) {
                            let tile_pos = TilePos::new(x, y);
                            let entity = feature_tile_storage.get(&tile_pos);
                            if entity.is_none() {
                                job_queue.jobs.push_back(Job::new(Jobs::Build(feature), tile_pos));
                            }
                        }
                    }
                }
                // TODO!(3, Wayan, 2): Make into interactive build. Add ability to customise while building.
                // Manually specify door, shift size, add furniture
                // All implemented using phantom tiles
                JobCreation::BuildRoom => {
                    let i_max = (u32::max(selection.x, world_tile.x) - u32::min(selection.x, world_tile.x)) as usize;
                    let j_max = (u32::max(selection.y, world_tile.y) - u32::min(selection.y, world_tile.y)) as usize;
                    for (i, x) in
                        (u32::min(selection.x, world_tile.x)..=u32::max(selection.x, world_tile.x)).enumerate()
                    {
                        for (j, y) in
                            (u32::min(selection.y, world_tile.y)..=u32::max(selection.y, world_tile.y)).enumerate()
                        {
                            let tile_pos = TilePos::new(x, y);
                            let entity = feature_tile_storage.get(&tile_pos);
                            if entity.is_none() {
                                // Hack to add a door.
                                if i == 0 && j == 1 {
                                    continue;
                                }

                                let feature = if i == 0 || j == 0 || i == i_max || j == j_max {
                                    Features::Wall
                                } else {
                                    Features::Floor
                                };
                                job_queue.jobs.push_back(Job::new(Jobs::Build(feature), tile_pos));
                            }
                        }
                    }
                }
            }
            commands.remove_resource::<SelectionStart>();
            commands.insert_resource(NextState(crate::states::GameStates::InGame));
        } else {
            commands.insert_resource(SelectionStart(world_tile));
        }
    }
}
