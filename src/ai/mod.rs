use bevy::prelude::*;
use big_brain::prelude::*;
use iyes_loopless::prelude::*;

use crate::states::GameStates;

mod actions;
mod behaviors;
mod pickers;
mod scorers;
mod spawner;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BigBrainPlugin);

        app.add_exit_system(GameStates::MapGeneration, spawner::spawn_ai);

        app.add_system(behaviors::thirst::handle_thirst.run_in_state(GameStates::InGame));
        app.add_system_set_to_stage(
            BigBrainStage::Actions,
            ConditionSet::new()
                .run_in_state(GameStates::InGame)
                .with_system(actions::drink::drink)
                .with_system(actions::move_to_water_source::move_to_water_source)
                .with_system(actions::meander::randomly_move)
                .into(),
        );
        app.add_system_set_to_stage(
            BigBrainStage::Scorers,
            ConditionSet::new()
                .run_in_state(GameStates::InGame)
                .with_system(scorers::thirsty::thirsty_scorer)
                .into(),
        );
    }
}
