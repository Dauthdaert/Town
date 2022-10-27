use bevy::prelude::*;

use crate::SIMULATION_SPEED;

#[derive(Component, Clone, Copy, Debug)]
pub struct WaterSource;

#[derive(Component, Clone, Copy, Debug)]
pub struct Obstacle;

#[derive(Component, Clone, Copy, Debug)]
pub struct Choppable;

#[derive(Component, Clone, Copy, Debug)]
pub struct Growing {
    pub speed: f32,
    pub progress: f32,
}

impl Growing {
    pub fn new() -> Self {
        Self {
            speed: 0.001 * SIMULATION_SPEED,
            progress: 0.0,
        }
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct Destructable;
