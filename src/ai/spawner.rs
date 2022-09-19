use bevy::prelude::*;
use bevy_turborand::{GlobalRng, RngComponent};
use big_brain::prelude::*;
use iyes_progress::Progress;

use crate::{
    animation::{AnimationTimer, SpriteAssets},
    map_gen::{map::Map, TILE_SIZE},
    SIMULATION_SPEED,
};

const NUM_AI: u32 = 1000;

use super::{
    actions::{
        do_job::DoJob, drink::Drink, job_destination::JobDestination, move_to_destination::MoveToDestination,
        random_destination::RandomDestination, take_job::TakingJob, water_source_destination::WaterSourceDestination,
    },
    characteristics::*,
    pickers::highest_score::HighestScore,
    scorers::{job_available::JobAvailable, thirsty::Thirsty},
};

pub fn spawn_ai(
    mut commands: Commands,
    map: Res<Map>,
    sprite_assets: Res<SpriteAssets>,
    mut rng: ResMut<GlobalRng>,
    mut next_ai_id: Local<u32>,
) -> Progress {
    for i in *next_ai_id..u32::min(*next_ai_id + 100, NUM_AI) {
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
                Speed::new(1. * TILE_SIZE.x * SIMULATION_SPEED),
                JobSeeker,
                build_thinker(),
                Name::from(format!("Villager {}", i)),
                RngComponent::from(&mut rng),
                AnimationTimer(Timer::from_seconds(0.5, true)),
            ));

        *next_ai_id += 1;
    }

    Progress {
        done: *next_ai_id,
        total: NUM_AI,
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
    let take_and_do_jobs = Steps::build()
        .step(TakingJob)
        .step(JobDestination)
        .step(MoveToDestination::default())
        .step(DoJob);
    Thinker::build()
        .picker(HighestScore::new())
        .when(Thirsty, move_and_drink)
        .when(JobAvailable, take_and_do_jobs)
        .when(FixedScore(0.5), meander)
}
