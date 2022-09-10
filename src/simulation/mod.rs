use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::states::GameStates;

mod growing;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameStates::InGame)
                .with_system(growing::grow)
                .into(),
        );
    }
}
