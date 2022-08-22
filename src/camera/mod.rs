use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

mod movement;
use movement::CameraMovement;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<CameraMovement>::default())
            .add_startup_system(setup_camera)
            .add_system(movement::movement);
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert_bundle(InputManagerBundle::<CameraMovement> {
            action_state: ActionState::default(),
            input_map: InputMap::default()
                .insert(SingleAxis::mouse_wheel_y(), CameraMovement::Zoom)
                .insert(KeyCode::A, CameraMovement::PanLeft)
                .insert(KeyCode::D, CameraMovement::PanRight)
                .insert(KeyCode::W, CameraMovement::PanUp)
                .insert(KeyCode::S, CameraMovement::PanDown)
                .build(),
        });
}
