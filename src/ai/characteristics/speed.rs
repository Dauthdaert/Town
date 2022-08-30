use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Speed {
    pub speed: f32,
}

impl Speed {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}
