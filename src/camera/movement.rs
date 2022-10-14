use bevy::{math::Vec3, prelude::*};
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Clone, Debug, Copy, PartialEq, Eq)]
pub enum CameraMovement {
    Zoom,
    PanUp,
    PanDown,
    PanLeft,
    PanRight,
}

const CAMERA_MIN_SCALE: f32 = 0.4;
const CAMERA_MAX_SCALE: f32 = 4.5;

// A simple camera system for moving and zooming the camera.
pub fn movement(
    time: Res<Time>,
    mut query: Query<
        (
            &mut Transform,
            &mut OrthographicProjection,
            &ActionState<CameraMovement>,
        ),
        With<Camera2d>,
    >,
) {
    let (mut transform, mut ortho, camera_movement_action) = query.single_mut();
    let mut direction = Vec3::ZERO;

    if camera_movement_action.pressed(CameraMovement::PanLeft) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
    }

    if camera_movement_action.pressed(CameraMovement::PanRight) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if camera_movement_action.pressed(CameraMovement::PanUp) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }

    if camera_movement_action.pressed(CameraMovement::PanDown) {
        direction -= Vec3::new(0.0, 1.0, 0.0);
    }

    let zoom_delta = camera_movement_action.value(CameraMovement::Zoom);
    if zoom_delta != 0.00 {
        ortho.scale += zoom_delta * 0.1;
        ortho.scale = ortho.scale.clamp(CAMERA_MIN_SCALE, CAMERA_MAX_SCALE);
    }

    let z = transform.translation.z;

    // TODO!(1, Wayan, 1): Clamp translation to never show outside of map.
    transform.translation += time.delta_seconds() * direction * 500. * ortho.scale;

    // Important! We need to restore the Z values when moving the camera
    // around. Bevy has a specific camera setup and this can mess
    // with how our layers are shown.
    transform.translation.z = z;
}
