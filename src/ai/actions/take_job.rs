use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{ai::characteristics::JobSeeker, jobs::job_queue::JobQueue, map::Map};

use super::components::HasJob;

#[derive(Component, Clone, Debug)]
pub struct TakingJob;

pub fn take_job(
    map: Res<Map>,
    mut commands: Commands,
    mut job_queue: ResMut<JobQueue>,
    actors: Query<Option<&HasJob>, With<JobSeeker>>,
    mut actions: Query<(&Actor, &mut ActionState, &TakingJob)>,
) {
    for (Actor(actor), mut action_state, _taking_job) in actions.iter_mut() {
        match *action_state {
            ActionState::Requested => {
                *action_state = ActionState::Executing;
            }
            ActionState::Executing => {
                let actor_has_job = actors.get(*actor).expect("Actor should have JobSeeker.");
                if actor_has_job.is_none() {
                    // TODO!(2, Wayan, 1): Improve job selection to incorporate distance to actor to prioritise jobs.
                    let job = job_queue.jobs.pop_front();
                    if let Some(job) = job {
                        if map.is_neighbor_passable(job.position.x, job.position.y) {
                            commands.entity(*actor).insert(HasJob::new(job));
                            *action_state = ActionState::Success;
                        } else {
                            job_queue.jobs.push_back(job);
                            *action_state = ActionState::Failure;
                        }
                    } else {
                        *action_state = ActionState::Failure;
                    }
                } else {
                    warn!("Actor already has a job.");
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
