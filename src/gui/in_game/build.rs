use bevy::prelude::*;
use bevy_ui_navigation::prelude::*;
use if_chain::if_chain;
use iyes_loopless::prelude::*;

use crate::{
    jobs::{JobCreation, JobSelectionType, SelectionStart},
    states::GameStates,
};

#[derive(Component, Clone, Copy)]
pub struct InGameBuildUiRoot;

#[allow(clippy::enum_variant_names)]
#[derive(Component, Clone, Copy)]
pub enum InGameBuildUiElem {
    BuildRoadButton,
    BuildRoomButton,
}

impl InGameBuildUiElem {
    fn name(&self) -> &str {
        match self {
            InGameBuildUiElem::BuildRoadButton => "BuildRoadButton",
            InGameBuildUiElem::BuildRoomButton => "BuildRoomButton",
        }
    }

    pub fn to_button(
        self,
    ) -> (
        Button,
        Focusable,
        Interaction,
        Name,
        InGameBuildUiElem,
        super::InGameUiMenu,
    ) {
        (
            Button,
            Focusable::new(),
            Interaction::None,
            Name::new(self.name().to_string()),
            self,
            super::InGameUiMenu::BuildMenu,
        )
    }
}

pub fn update_build_menu_ui(
    mut events: EventReader<NavEvent>,
    mut commands: Commands,
    elements: Query<&InGameBuildUiElem>,
    current_state: Res<CurrentState<GameStates>>,
) {
    commands.remove_resource::<SelectionStart>();

    let mut requested_state_change = None;
    for button in events.nav_iter().activated_in_query(&elements) {
        match button {
            InGameBuildUiElem::BuildRoadButton => {
                requested_state_change = Some(GameStates::InJobSelection);
                commands.insert_resource(JobSelectionType(JobCreation::Build(crate::map::Features::Road)));
            }
            InGameBuildUiElem::BuildRoomButton => {
                requested_state_change = Some(GameStates::InJobSelection);
                commands.insert_resource(JobSelectionType(JobCreation::BuildRoom));
            }
        }
    }

    if_chain! {
        if let Some(request) = requested_state_change;
        if request != current_state.0;
        then {
            commands.insert_resource(NextState(request));
        }
    }
}
