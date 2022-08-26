use bevy::prelude::*;
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

mod movement;
use movement::CameraMovement;

use crate::states::GameStates;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<CameraMovement>::default())
            .add_enter_system(GameStates::InGame, setup_camera)
            .add_system(movement::movement.run_in_state(GameStates::InGame));
    }
}

fn setup_camera(mut commands: Commands) {
    let offset_x = crate::map_gen::TILE_SIZE.x * (crate::map_gen::MAP_WIDTH / 2) as f32;
    let offset_y = crate::map_gen::TILE_SIZE.y * (crate::map_gen::MAP_HEIGHT / 2) as f32;
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_xyz(offset_x, offset_y, 1000.0)
                .looking_at(Vec3::new(offset_x, offset_y, 0.0), Vec3::Y),
            ..Default::default()
        })
        .insert_bundle(InputManagerBundle::<CameraMovement> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(SingleAxis::mouse_wheel_y(), CameraMovement::Zoom)
                .insert(KeyCode::A, CameraMovement::PanLeft)
                .insert(KeyCode::Left, CameraMovement::PanLeft)
                .insert(KeyCode::D, CameraMovement::PanRight)
                .insert(KeyCode::Right, CameraMovement::PanRight)
                .insert(KeyCode::W, CameraMovement::PanUp)
                .insert(KeyCode::Up, CameraMovement::PanUp)
                .insert(KeyCode::S, CameraMovement::PanDown)
                .insert(KeyCode::Down, CameraMovement::PanDown)
                .build(),
        })
        .insert(Name::from("Main Camera"));
}
