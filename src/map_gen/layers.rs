use bevy::prelude::*;

pub trait Layer {
    fn z_index() -> f32;
}

#[derive(Component, Clone, Copy, Debug)]
pub struct TileLayer;

impl Layer for TileLayer {
    fn z_index() -> f32 {
        0.0
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct TileLayerObject;

#[derive(Component, Clone, Copy, Debug)]
pub struct FeatureLayer;

impl Layer for FeatureLayer {
    fn z_index() -> f32 {
        1.0
    }
}

#[derive(Component, Clone, Copy, Debug)]
pub struct FeatureLayerObject;
