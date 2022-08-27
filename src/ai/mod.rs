use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::states::GameStates;

mod spawner;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_exit_system(GameStates::MapGeneration, spawner::spawn_ai);
    }
}
