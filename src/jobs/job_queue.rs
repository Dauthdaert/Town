use std::collections::VecDeque;

use crate::map::Map;

use super::Jobs;
use bevy_ecs_tilemap::prelude::*;
use hierarchical_pathfinding::prelude::Neighborhood;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Job {
    pub job_type: Jobs,
    pub position: TilePos,
}

impl Job {
    pub fn new(job_type: Jobs, position: TilePos) -> Self {
        Self { job_type, position }
    }
}

#[derive(Debug, Clone, Default)]
pub struct JobQueue {
    pub jobs: VecDeque<Job>,
}

impl JobQueue {
    pub fn any_job_accessible(&self, map: &Map) -> bool {
        self.jobs.iter().any(|j| {
            map.is_neighbor_passable(j.position.x, j.position.y)
        })
    }
}
