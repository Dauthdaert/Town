use bevy::prelude::*;
use bevy_mouse_tracking_plugin::{prelude::*, MainCamera};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

mod movement;
use movement::CameraMovement;

use crate::{
    condition_set_in_states,
    map::{MAP_HEIGHT, MAP_WIDTH},
    states::GameStates,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<CameraMovement>::default())
            .add_enter_system(GameStates::Splash, setup_camera);

        app.add_system_set(
            condition_set_in_states!(GameStates::InGame | GameStates::InJobSelection)
                .with_system(movement::movement)
                .into(),
        );
    }
}

fn setup_camera(mut commands: Commands, camera_query: Query<Entity, With<Camera2d>>) {
    if camera_query.is_empty() {
        let offset = crate::map::tile_xy_world_xy(MAP_WIDTH / 2, MAP_HEIGHT / 2);
        commands
            .spawn_bundle(Camera2dBundle {
                transform: Transform::from_xyz(offset.x, offset.y, 1000.0)
                    .looking_at(Vec3::new(offset.x, offset.y, 0.0), Vec3::Y),
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
            .add_world_tracking()
            .insert_bundle((Name::from("Main Camera"), MainCamera));
    }
}
