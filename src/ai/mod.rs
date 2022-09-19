use bevy::prelude::*;
use big_brain::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::ProgressSystem;

use crate::states::GameStates;

mod actions;
mod characteristics;
mod pickers;
mod scorers;
mod spawner;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin);

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameStates::GameObjectSpawning)
                .with_system(spawner::spawn_ai.track_progress())
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameStates::InGame)
                .with_system(characteristics::thirst::handle_thirst)
                .into(),
        );

        app.add_system_set_to_stage(
            BigBrainStage::Actions,
            ConditionSet::new()
                .run_in_state(GameStates::InGame)
                .with_system(actions::take_job::take_job)
                .with_system(actions::water_source_destination::water_source_destination)
                .with_system(actions::random_destination::random_destination)
                .with_system(actions::job_destination::job_destination)
                .with_system(actions::move_to_destination::move_to_destination)
                .with_system(actions::drink::drink)
                .with_system(actions::do_job::do_job)
                .into(),
        );
        app.add_system_set_to_stage(
            BigBrainStage::Scorers,
            ConditionSet::new()
                .run_in_state(GameStates::InGame)
                .with_system(scorers::thirsty::thirsty_scorer)
                .with_system(scorers::job_available::job_available_scorer)
                .into(),
        );
    }
}
