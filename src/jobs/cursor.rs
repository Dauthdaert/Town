use bevy::{math::Vec3Swizzles, prelude::*, sprite::Anchor};
use bevy_mouse_tracking_plugin::MousePosWorld;

use crate::map_gen::{
    map::{tile_xy_world_xy, world_xy_tile_xy},
    TILE_SIZE,
};

use super::{job_creation::SelectionStart, JobCreationMenuAssets};

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
            sprite: Sprite {
                color: Color::BLUE,
                anchor: Anchor::TopLeft,
                ..default()
            },
            ..default()
        })
        .insert_bundle((Name::from("Job Creation Menu Cursor"), JobCreationMenuCursor));
}

pub fn job_creation_menu_cursor_follow_mouse(
    mut cursor: Query<&mut Transform, With<JobCreationMenuCursor>>,
    selection: Option<Res<SelectionStart>>,
    mouse_pos: Res<MousePosWorld>,
) {
    let mut cursor_transform = cursor.single_mut();

    let world_tile = world_xy_tile_xy(mouse_pos.xy());
    let world_pos = tile_xy_world_xy(world_tile.x, world_tile.y);

    let (pos, size) = if let Some(selection_start) = selection {
        let size_x = selection_start.x.abs_diff(world_tile.x) as f32 + 1.;
        let size_y = selection_start.y.abs_diff(world_tile.y) as f32 + 1.;
        let size = Vec2::new(size_x, size_y);

        if world_tile.x >= selection_start.x && world_tile.y <= selection_start.y {
            let selection_pos = tile_xy_world_xy(selection_start.x, selection_start.y);

            (shift_to_anchor(selection_pos), size.extend(1.0))
        } else if world_tile.x <= selection_start.x && world_tile.y <= selection_start.y {
            let selection_pos = tile_xy_world_xy(selection_start.x, selection_start.y);
            let pos = shift_to_anchor(Vec2::new(world_pos.x, selection_pos.y));

            (pos, size.extend(1.0))
        } else if world_tile.x <= selection_start.x && world_tile.y >= selection_start.y {
            (shift_to_anchor(world_pos), size.extend(1.0))
        } else {
            //if world_tile.x >= selection_start.x && world_tile.y >= selection_start.y
            let selection_pos = tile_xy_world_xy(selection_start.x, selection_start.y);
            let pos = shift_to_anchor(Vec2::new(selection_pos.x, world_pos.y));

            (pos, size.extend(1.0))
        }
    } else {
        (shift_to_anchor(world_pos), Vec3::splat(1.0))
    };

    cursor_transform.scale = size;
    cursor_transform.translation = pos.extend(10.0);
}

fn shift_to_anchor(pos: Vec2) -> Vec2 {
    pos - Vec2::new(TILE_SIZE.x / 2.0, -TILE_SIZE.y / 2.0)
}
