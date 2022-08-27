use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::tiles::TilePos;
use big_brain::prelude::*;

use crate::map_gen::components::WaterSource;

#[derive(Component, Clone, Copy, Debug)]
pub struct MoveToWaterSource {
    pub speed: f32,
}

pub fn move_to_water_source(
    time: Res<Time>,
    water_sources: Query<&TilePos, With<WaterSource>>,
    mut positions: Query<&mut Transform, Without<WaterSource>>,
    mut actions: Query<(&Actor, &mut ActionState, &MoveToWaterSource)>,
) {
    for (Actor(actor), mut action_state, move_to) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                //Compute a path?
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let mut actor_transform = positions.get_mut(*actor).expect("Actor has no position.");
                let closest_water_source =
                    super::shared_drinking::find_closest_water_source(&water_sources, &actor_transform);

                let delta = closest_water_source - actor_transform.translation.xy();
                let distance = delta.length();

                if distance > super::MAX_ACTION_DISTANCE {
                    let step_size = time.delta_seconds() * move_to.speed;
                    let step = if distance > step_size {
                        delta / distance * step_size
                    } else {
                        delta
                    };

                    actor_transform.translation += step.extend(0.0);
                } else {
                    //We've arrived.
                    *action_state = ActionState::Success;
                }
            }
            ActionState::Cancelled => {
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
