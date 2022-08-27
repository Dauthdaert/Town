use bevy::prelude::*;
use big_brain::prelude::*;

use crate::{
    animation::{AnimationTimer, SpriteAssets},
    SIMULATION_SPEED,
};

use super::{
    actions::{drink::Drink, meander::Meander, move_to_water_source::MoveToWaterSource},
    behaviors::thirst::Thirst,
    pickers::highest_score::HighestScore,
    scorers::thirsty::Thirsty,
};

pub fn spawn_ai(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    let move_and_drink = Steps::build()
        .step(MoveToWaterSource {
            speed: 32.0 * SIMULATION_SPEED,
        })
        .step(Drink {
            per_second: 10.0 * SIMULATION_SPEED,
        });
    let thinker = Thinker::build()
        .picker(HighestScore::new())
        .when(Thirsty, move_and_drink)
        .when(
            FixedScore::build(0.5),
            Meander {
                speed: 16.0 * SIMULATION_SPEED,
            },
        );

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
        .insert(Thirst::new(0.0, 0.1 * SIMULATION_SPEED))
        .insert(thinker)
        .insert(Name::from("Villager"))
        .insert(AnimationTimer(Timer::from_seconds(0.5, true)));
}
