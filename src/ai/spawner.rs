use bevy::prelude::*;

use crate::animation::{AnimationTimer, SpriteAssets};

pub fn spawn_ai(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    let offset_x = crate::map_gen::TILE_SIZE.x * (crate::map_gen::MAP_WIDTH / 2) as f32;
    let offset_y = crate::map_gen::TILE_SIZE.y * (crate::map_gen::MAP_HEIGHT / 2) as f32;
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: sprite_assets.villager.clone(),
            transform: Transform {
                translation: Vec3::new(offset_x, offset_y, 2.),
                ..default()
            },
            ..default()
        })
        .insert(Name::from("Villager"))
        .insert(AnimationTimer(Timer::from_seconds(0.5, true)));
}
