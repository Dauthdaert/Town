use bevy::prelude::*;
use big_brain::prelude::*;
use hierarchical_pathfinding::prelude::Neighborhood;

use crate::{
    ai::{actions::components::HasJob, characteristics::job_seeker::*},
    jobs::job_queue::JobQueue, map::Map, map::neighborhood::EuclideanNeighborhood
};

#[derive(Component, Clone, Copy, Debug)]
pub struct JobAvailable;

pub fn job_available_scorer(
    map: Res<Map>,
    job_queue: Res<JobQueue>,
    job_seekers: Query<(&JobSeeker, Option<&HasJob>)>,
    mut actors: Query<(&Actor, &mut Score), With<JobAvailable>>,
) {
    let accessible_job = job_queue.any_job_accessible(&map);
    actors.par_for_each_mut(100, |(Actor(actor), mut score)| {
        if let Ok((_job_seeker, has_job)) = job_seekers.get(*actor) {
            if has_job.is_some() || accessible_job {
                score.set(0.8);
            } else {
                score.set(0.0);
            }
        }
    });
}
