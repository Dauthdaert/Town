use bevy::prelude::*;
use bevy_turborand::{GlobalRng, RngComponent};
use big_brain::prelude::*;

use crate::{
    animation::{AnimationTimer, SpriteAssets},
    map_gen::map::Map,
    SIMULATION_SPEED,
};

use super::{
    actions::{
        drink::Drink, move_to_destination::MoveToDestination, random_destination::RandomDestination,
        water_source_destination::WaterSourceDestination,
    },
    characteristics::*,
    pickers::highest_score::HighestScore,
    scorers::thirsty::Thirsty,
};

pub fn spawn_ai(mut commands: Commands, map: Res<Map>, sprite_assets: Res<SpriteAssets>, mut rng: ResMut<GlobalRng>) {
    for i in 1..=1000 {
        let pos_offset = crate::map_gen::map::tile_xy_world_xy(map.width / 2, map.height / 2);
        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: sprite_assets.villager.clone(),
                transform: Transform {
                    translation: pos_offset.extend(2.),
                    ..default()
                },
                ..default()
            })
            .insert_bundle((
                Thirst::new(0.0, 0.1 * SIMULATION_SPEED),
                Speed::new(32.0 * SIMULATION_SPEED),
                build_thinker(),
                Name::from(format!("Villager {}", i)),
                RngComponent::from(&mut rng),
                AnimationTimer(Timer::from_seconds(0.5, true)),
            ));
    }
}

fn build_thinker() -> ThinkerBuilder {
    let move_and_drink = Steps::build()
        .step(WaterSourceDestination)
        .step(MoveToDestination::default())
        .step(Drink {
            per_second: 10.0 * SIMULATION_SPEED,
        });
    let meander = Steps::build()
        .step(RandomDestination)
        .step(MoveToDestination::default());
    Thinker::build()
        .picker(HighestScore::new())
        .when(Thirsty, move_and_drink)
        .when(FixedScore::build(0.5), meander)
}
