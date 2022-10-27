use std::marker::PhantomData;

use bevy::prelude::*;

use super::Layer;

mod events;
mod systems;
mod tile;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
struct AutoTileRemoveStage;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
struct AutoTileAddUpdateStage;

pub struct AutoTilePlugin<T: Layer + Component> {
    _phantom: PhantomData<T>,
}

impl<T: Layer + Component> Default for AutoTilePlugin<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl<T: Layer + Component> Plugin for AutoTilePlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_stage_before(CoreStage::PostUpdate, AutoTileAddUpdateStage, SystemStage::parallel());
        app.add_stage_before(AutoTileAddUpdateStage, AutoTileRemoveStage, SystemStage::parallel());

        app.add_event::<events::RemoveAutoTileEvent>()
            .add_system_to_stage(AutoTileRemoveStage, systems::on_remove_auto_tile::<T>)
            .add_system_to_stage(AutoTileAddUpdateStage, systems::on_change_auto_tile::<T>);
    }
}
