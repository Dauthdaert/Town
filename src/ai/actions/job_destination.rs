use bevy::prelude::*;
use big_brain::prelude::*;

use crate::ai::characteristics::JobSeeker;

use super::components::{Destination, HasJob};

#[derive(Component, Clone, Debug)]
pub struct JobDestination;

pub fn job_destination(
    mut commands: Commands,
    actors: Query<&HasJob, With<JobSeeker>>,
    mut actions: Query<(&Actor, &mut ActionState, &JobDestination)>,
) {
    for (Actor(actor), mut action_state, _move_to) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let actor_has_job = actors.get(*actor).expect("Actor should have a job.");
                let destination = actor_has_job.job.position;

                // TODO!(3, Wayan, 2): Check that destination is possible.
                commands.entity(*actor).insert(Destination::new(destination, false));
                *action_state = ActionState::Success;
            }
            ActionState::Cancelled => {
                // TODO!(3, Wayan, 2): Cancel job?
                *action_state = ActionState::Failure;
            }
            _ => {}
        }
    }
}
