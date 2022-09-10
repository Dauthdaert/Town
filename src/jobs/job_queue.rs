use std::collections::VecDeque;

use super::Jobs;
use bevy_ecs_tilemap::prelude::*;

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
