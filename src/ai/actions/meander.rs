use bevy::{math::Vec3Swizzles, prelude::*};
use big_brain::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub struct Meander {
    pub speed: f32,
}

pub fn randomly_move(
    time: Res<Time>,
    mut positions: Query<&mut Transform>,
    mut actions: Query<(&Actor, &mut ActionState, &Meander)>,
) {
    for (Actor(actor), mut action_state, move_to) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                //Compute a path?
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let mut actor_transform = positions.get_mut(*actor).expect("Actor has no position.");

                //TODO: Make destination random.
                let destination = Vec2::new(1000., 1000.);
                let delta = destination - actor_transform.translation.xy();
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
