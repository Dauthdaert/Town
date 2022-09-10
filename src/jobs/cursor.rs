use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_mouse_tracking_plugin::MousePosWorld;

use crate::map_gen::map::{tile_xy_world_xy, world_xy_tile_xy};

use super::JobCreationMenuAssets;

#[derive(Component, Clone, Copy, Debug)]
pub struct JobCreationMenuCursor;

pub fn setup_job_creation_menu_cursor(mut commands: Commands, job_creation_menu_assets: Res<JobCreationMenuAssets>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: job_creation_menu_assets.cursor.clone(),
            transform: Transform {
                translation: Vec2::ZERO.extend(2.),
                ..default()
            },
            ..default()
        })
        .insert_bundle((Name::from("Job Creation Menu Cursor"), JobCreationMenuCursor));
}

pub fn job_creation_menu_cursor_follow_mouse(
    mut cursor: Query<&mut Transform, With<JobCreationMenuCursor>>,
    mouse_pos: Res<MousePosWorld>,
) {
    let mut cursor_transform = cursor.single_mut();

    let world_tile = world_xy_tile_xy(mouse_pos.xy());
    let world_pos = tile_xy_world_xy(world_tile.x, world_tile.y);

    cursor_transform.translation = world_pos.extend(10.0);
}
