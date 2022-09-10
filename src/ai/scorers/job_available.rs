use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    ai::{actions::components::HasJob, characteristics::job_seeker::*},
    jobs::job_queue::JobQueue,
};

#[derive(Component, Clone, Copy, Debug)]
pub struct JobAvailable;

pub fn job_available_scorer(
    job_queue: Res<JobQueue>,
    job_seekers: Query<(&JobSeeker, Option<&HasJob>)>,
    mut actors: Query<(&Actor, &mut Score), With<JobAvailable>>,
) {
    actors.par_for_each_mut(100, |(Actor(actor), mut score)| {
        if let Ok((_job_seeker, has_job)) = job_seekers.get(*actor) {
            if has_job.is_some() || job_queue.jobs.front().is_some() {
                score.set(0.8);
            } else {
                score.set(0.0);
            }
        }
    });
}
