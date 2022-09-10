use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::jobs::job_queue::Job;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Destination {
    pub destination: TilePos,
    pub approximate: bool,
}

impl Destination {
    pub fn new(destination: TilePos, approximate: bool) -> Self {
        Self {
            destination,
            approximate,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct HasJob {
    pub job: Job,
    pub progress: f32,
}

impl HasJob {
    pub fn new(job: Job) -> Self {
        Self { job, progress: 0.0 }
    }
}
