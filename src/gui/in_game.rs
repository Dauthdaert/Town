use bevy::prelude::*;
use bevy_ui_build_macros::*;
use bevy_ui_navigation::prelude::*;
use if_chain::if_chain;
use iyes_loopless::prelude::*;

use crate::{
    condition_set_in_states,
    jobs::{JobCreation, JobSelectionType, SelectionStart},
    states::GameStates,
};

use super::UiAssets;

#[derive(Component, Clone, Copy)]
struct InGameUiRoot;

#[allow(clippy::enum_variant_names)]
#[derive(Component, Clone, Copy)]
enum InGameUiElem {
    ChopButton,
    BuildWallButton,
    BuildFloorButton,
    BuildRoomButton,
}

pub struct InGameGuiPlugin;

impl Plugin for InGameGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameStates::InGame, setup_in_game_ui)
            .add_system_set(
                condition_set_in_states!(GameStates::InGame | GameStates::InJobSelection)
                    .run_on_event::<NavEvent>()
                    .with_system(update_in_game_ui)
                    .into(),
            );
    }
}

fn setup_in_game_ui(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    mut in_game_ui_query: Query<&mut Visibility, With<InGameUiRoot>>,
) {
    if let Ok(mut root) = in_game_ui_query.get_single_mut() {
        root.is_visible = true;
    } else {
        let text_bundle = |content: &str, font_size: f32| ui_assets.text_bundle(content, font_size);
        let focusable = Focusable::default();
        let node = NodeBundle {
            color: Color::NONE.into(),
            style: style! {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
            },
            ..default()
        };
        let button = (Button, Interaction::None);

        build_ui! {
            #[cmd(commands)]
            node {
                min_size: size!(100 pct, 100 pct),
                justify_content: JustifyContent::Center
            }[; Name::new("InGameRootNode"), InGameUiRoot](
                node {
                    position_type: PositionType::Absolute,
                    position: rect!(15 px, auto, auto, 15 px,),
                    flex_direction: FlexDirection::Row
                }[;](
                    node{
                        padding: rect!(20 px),
                        margin: rect!(10 px)
                    }[button; focusable, Name::new("ChopButton"), InGameUiElem::ChopButton](
                        node[text_bundle("Chop", 20.0);]
                    ),
                    node{
                        padding: rect!(20 px),
                        margin: rect!(10 px)
                    }[button; focusable, Name::new("BuildWallButton"), InGameUiElem::BuildWallButton](
                        node[text_bundle("Build Wall", 20.0);]
                    ),
                    node{
                        padding: rect!(20 px),
                        margin: rect!(10 px)
                    }[button; focusable, Name::new("BuildFloorButton"), InGameUiElem::BuildFloorButton](
                        node[text_bundle("Build Floor", 20.0);]
                    ),
                    node{
                        padding: rect!(20 px),
                        margin: rect!(10 px)
                    }[button; focusable, Name::new("BuildRoomButton"), InGameUiElem::BuildRoomButton](
                        node[text_bundle("Build Room", 20.0);]
                    )
                )
            )
        };
    }
}

fn update_in_game_ui(
    mut events: EventReader<NavEvent>,
    mut commands: Commands,
    elements: Query<&InGameUiElem>,
    current_state: Res<CurrentState<GameStates>>,
) {
    use NavRequest::Action;

    commands.remove_resource::<SelectionStart>();

    let mut requested_state_change = None;
    for (event_type, from) in events.nav_iter().types() {
        match (event_type, elements.get(from)) {
            (NavEvent::NoChanges { request: Action, .. }, Ok(InGameUiElem::ChopButton)) => {
                requested_state_change = Some(GameStates::InJobSelection);
                commands.insert_resource(JobSelectionType(JobCreation::Chop));
            }
            (NavEvent::NoChanges { request: Action, .. }, Ok(InGameUiElem::BuildWallButton)) => {
                requested_state_change = Some(GameStates::InJobSelection);
                commands.insert_resource(JobSelectionType(JobCreation::Build(crate::map::Features::Wall)));
            }
            (NavEvent::NoChanges { request: Action, .. }, Ok(InGameUiElem::BuildFloorButton)) => {
                requested_state_change = Some(GameStates::InJobSelection);
                commands.insert_resource(JobSelectionType(JobCreation::Build(crate::map::Features::Floor)));
            }
            (NavEvent::NoChanges { request: Action, .. }, Ok(InGameUiElem::BuildRoomButton)) => {
                requested_state_change = Some(GameStates::InJobSelection);
                commands.insert_resource(JobSelectionType(JobCreation::BuildRoom));
            }
            (_, Err(err)) => error!("Error in in_game_ui update: {err:?}"),
            _ => {}
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
