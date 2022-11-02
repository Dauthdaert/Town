use std::marker::PhantomData;

use bevy::prelude::*;

use super::{FeatureLayer, Layer};

mod events;
mod systems;
mod tile;

pub use events::RemoveAutoTileEvent;

pub struct AutoTilePlugin;

impl Plugin for AutoTilePlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(CoreStage::PostUpdate, AutoTileAddUpdateStage, SystemStage::parallel());
        app.add_stage_before(AutoTileAddUpdateStage, AutoTileRemoveStage, SystemStage::parallel());

        app.add_event::<events::RemoveAutoTileEvent>()
            .add_plugin(AutoTileLayerPlugin::<FeatureLayer>::default());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
struct AutoTileRemoveStage;

#[derive(Debug, Clone, PartialEq, Eq, Hash, StageLabel)]
struct AutoTileAddUpdateStage;

struct AutoTileLayerPlugin<T: Layer + Component> {
    _phantom: PhantomData<T>,
}

impl<T: Layer + Component> Default for AutoTileLayerPlugin<T> {
    fn default() -> Self {
        Self {
            _phantom: PhantomData::default(),
        }
    }
}

impl<T: Layer + Component> Plugin for AutoTileLayerPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(AutoTileRemoveStage, systems::on_remove_auto_tile::<T>)
            .add_system_to_stage(AutoTileAddUpdateStage, systems::on_change_auto_tile::<T>);
    }
}
