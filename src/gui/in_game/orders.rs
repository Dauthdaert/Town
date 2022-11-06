use bevy::prelude::*;
use bevy_ui_navigation::prelude::*;
use if_chain::if_chain;
use iyes_loopless::prelude::*;

use crate::{
    jobs::{JobCreation, JobSelectionType, SelectionStart},
    states::GameStates,
};

#[derive(Component, Clone, Copy)]
pub struct InGameOrdersUiRoot;

#[allow(clippy::enum_variant_names)]
#[derive(Component, Clone, Copy)]
pub enum InGameOrdersUiElem {
    ChopButton,
    MineButton,
    ClearButton,
}

impl InGameOrdersUiElem {
    fn name(&self) -> &str {
        match self {
            InGameOrdersUiElem::ChopButton => "ChopButton",
            InGameOrdersUiElem::MineButton => "MineButton",
            InGameOrdersUiElem::ClearButton => "ClearButton",
        }
    }

    pub fn to_button(
        self,
    ) -> (
        Button,
        Focusable,
        Interaction,
        Name,
        InGameOrdersUiElem,
        super::InGameUiMenu,
    ) {
        (
            Button,
            Focusable::new(),
            Interaction::None,
            Name::new(self.name().to_string()),
            self,
            super::InGameUiMenu::OrdersMenu,
        )
    }
}

pub fn update_orders_menu_ui(
    mut events: EventReader<NavEvent>,
    mut commands: Commands,
    elements: Query<&InGameOrdersUiElem>,
    current_state: Res<CurrentState<GameStates>>,
) {
    commands.remove_resource::<SelectionStart>();

    let mut requested_state_change = None;
    for button in events.nav_iter().activated_in_query(&elements) {
        match button {
            InGameOrdersUiElem::ChopButton => {
                requested_state_change = Some(GameStates::InJobSelection);
                commands.insert_resource(JobSelectionType(JobCreation::Chop));
            }
            InGameOrdersUiElem::MineButton => {
                requested_state_change = Some(GameStates::InJobSelection);
                commands.insert_resource(JobSelectionType(JobCreation::Mine));
            }
            InGameOrdersUiElem::ClearButton => {
                requested_state_change = Some(GameStates::InJobSelection);
                commands.insert_resource(JobSelectionType(JobCreation::Clear));
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
