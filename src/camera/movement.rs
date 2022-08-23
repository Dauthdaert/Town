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
    let (mut transform, mut ortho, camera_movement) = query.single_mut();
    let mut direction = Vec3::ZERO;

    if camera_movement.pressed(CameraMovement::PanLeft) {
        direction -= Vec3::new(1.0, 0.0, 0.0);
    }

    if camera_movement.pressed(CameraMovement::PanRight) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }

    if camera_movement.pressed(CameraMovement::PanUp) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }

    if camera_movement.pressed(CameraMovement::PanDown) {
        direction -= Vec3::new(0.0, 1.0, 0.0);
    }

    let zoom_delta = camera_movement.value(CameraMovement::Zoom);
    if zoom_delta != 0.00 {
        ortho.scale += zoom_delta * 0.05;
    }

    ortho.scale = ortho.scale.clamp(0.4, 1.0);

    let z = transform.translation.z;
    transform.translation += time.delta_seconds() * direction * 500.;
    // Important! We need to restore the Z values when moving the camera
    // around. Bevy has a specific camera setup and this can mess
    // with how our layers are shown.
    transform.translation.z = z;
}
