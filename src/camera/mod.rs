use bevy::prelude::*;
use bevy_mouse_tracking_plugin::{prelude::*, MainCamera};
use iyes_loopless::prelude::*;
use leafwing_input_manager::prelude::*;

mod movement;
use movement::CameraMovement;

use crate::{
    map_gen::{MAP_HEIGHT, MAP_WIDTH},
    states::GameStates,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<CameraMovement>::default())
            .add_exit_system(GameStates::GameObjectSpawning, setup_camera);

        app.add_system_set(
            ConditionSet::new()
                .run_not_in_state(GameStates::Splash)
                .run_not_in_state(GameStates::MapGeneration)
                .run_not_in_state(GameStates::GameObjectSpawning)
                .with_system(movement::movement)
                .into(),
        );
    }
}

fn setup_camera(mut commands: Commands) {
    let offset = crate::map_gen::map::tile_xy_world_xy(MAP_WIDTH / 2, MAP_HEIGHT / 2);
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
