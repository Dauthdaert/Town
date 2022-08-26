use bevy::prelude::*;

use crate::animation::{AnimationTimer, SpriteAssets};

pub fn spawn_ai(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: sprite_assets.villager.clone(),
            transform: Transform {
                translation: Vec3::new(1000., 1000., 1.),
                ..default()
            },
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.5, true)));
}
